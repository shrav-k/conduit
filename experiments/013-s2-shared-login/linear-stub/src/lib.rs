//! Version 1 of S2's synthetic, loopback-only Linear API fixture.
mod barrier;
mod graphql;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, io,
    net::{Ipv4Addr, SocketAddr},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use async_graphql::{EmptySubscription, Schema};
use axum::{
    Json, Router,
    extract::{Request, State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{
    net::TcpListener,
    sync::{Mutex, oneshot},
    task::JoinHandle,
};
use uuid::Uuid;

pub const INTERFACE_VERSION: u32 = 1;

/// Only generated test tokens are accepted. Debug output never reveals them.
#[derive(Clone)]
pub struct SyntheticToken(String);

impl SyntheticToken {
    pub fn generate() -> Self {
        Self(format!("s2-stub-{}", Uuid::new_v4()))
    }

    /// For HTTP headers or a private connection file; do not log the result.
    pub fn expose(&self) -> &str {
        &self.0
    }

    fn parse(value: String) -> Result<Self, ApiError> {
        if value
            .strip_prefix("s2-stub-")
            .is_some_and(|v| Uuid::parse_str(v).is_ok())
        {
            Ok(Self(value))
        } else {
            Err(ApiError(
                StatusCode::BAD_REQUEST,
                "expected a synthetic token",
            ))
        }
    }
}

impl fmt::Debug for SyntheticToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SyntheticToken([redacted])")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IssueFixture {
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub updated_at: DateTime<Utc>,
}

impl IssueFixture {
    pub fn new(identifier: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            title: title.into(),
            description: None,
            state: "Todo".into(),
            updated_at: DateTime::from_timestamp(1_767_225_600, 0).expect("fixed timestamp"),
        }
    }
}

fn synthetic_identifier(value: &str) -> bool {
    value
        .strip_prefix("SYN-")
        .is_some_and(|suffix| !suffix.is_empty() && suffix.bytes().all(|b| b.is_ascii_digit()))
}

#[derive(Debug)]
pub struct StubConfig {
    pub issues: Vec<IssueFixture>,
    pub api_key: SyntheticToken,
    pub control_token: SyntheticToken,
    /// Must be loopback; port zero chooses an available port.
    pub bind: SocketAddr,
}

impl StubConfig {
    pub fn new(issues: Vec<IssueFixture>) -> Self {
        Self {
            issues,
            api_key: SyntheticToken::generate(),
            control_token: SyntheticToken::generate(),
            bind: (Ipv4Addr::LOCALHOST, 0).into(),
        }
    }
}

pub(crate) struct ServiceData {
    key: SyntheticToken,
    visible: BTreeSet<String>,
    issues: BTreeMap<String, IssueFixture>,
}

pub(crate) struct Shared {
    data: Mutex<ServiceData>,
    control_token: SyntheticToken,
    count: AtomicU64,
    next_barrier: AtomicU64,
    barrier: Mutex<Option<Arc<barrier::Barrier>>>,
}

/// Owns the listener task. Dropping it aborts the task; `shutdown` drains it.
pub struct Stub {
    address: SocketAddr,
    stop: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<io::Result<()>>>,
}

impl Stub {
    pub async fn start(config: StubConfig) -> io::Result<Self> {
        if !config.bind.ip().is_loopback() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "loopback address required",
            ));
        }
        if config.api_key.expose() == config.control_token.expose() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "distinct tokens required",
            ));
        }
        let mut issues = BTreeMap::new();
        for issue in config.issues {
            if !synthetic_identifier(&issue.identifier)
                || issues.insert(issue.identifier.clone(), issue).is_some()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "unique SYN identifiers required",
                ));
            }
        }
        let shared = Arc::new(Shared {
            data: Mutex::new(ServiceData {
                key: config.api_key,
                visible: issues.keys().cloned().collect(),
                issues,
            }),
            control_token: config.control_token,
            count: AtomicU64::new(0),
            next_barrier: AtomicU64::new(1),
            barrier: Mutex::new(None),
        });
        let schema = Schema::build(graphql::Query, graphql::Mutation, EmptySubscription)
            .limit_depth(16)
            .limit_complexity(1000)
            .finish();
        let api = Router::new()
            .route("/graphql", post(graphql_handler))
            .route_layer(middleware::from_fn_with_state(shared.clone(), count_api))
            .layer(axum::Extension(schema));
        let control = Router::new()
            .route("/control/v1/count", get(count))
            .route(
                "/control/v1/service-authorization",
                post(service_authorization),
            )
            .route("/control/v1/rotate-key", post(rotate_key))
            .route("/control/v1/barrier/arm", post(barrier::arm))
            .route("/control/v1/barrier/wait", post(barrier::wait))
            .route("/control/v1/barrier/release", post(barrier::release))
            .route_layer(middleware::from_fn_with_state(shared.clone(), control_auth));
        let app = api.merge(control).with_state(shared);
        let listener = TcpListener::bind(config.bind).await?;
        let address = listener.local_addr()?;
        let (stop, stopped) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = stopped.await;
                })
                .await
        });
        Ok(Self {
            address,
            stop: Some(stop),
            task: Some(task),
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn graphql_url(&self) -> String {
        format!("http://{}/graphql", self.address)
    }

    pub fn control_url(&self) -> String {
        format!("http://{}/control/v1", self.address)
    }

    pub async fn shutdown(mut self) -> io::Result<()> {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(task) = self.task.take() {
            task.await.map_err(io::Error::other)??;
        }
        Ok(())
    }
}

impl Drop for Stub {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

pub(crate) struct ApiError(StatusCode, &'static str);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({"error": self.1}))).into_response()
    }
}

fn authorized(headers: &HeaderMap, token: &SyntheticToken) -> bool {
    headers.get("authorization").and_then(|v| v.to_str().ok()) == Some(token.expose())
}

async fn count_api(State(shared): State<Arc<Shared>>, request: Request, next: Next) -> Response {
    if request.method() == axum::http::Method::POST {
        shared.count.fetch_add(1, Ordering::SeqCst);
    }
    next.run(request).await
}

async fn control_auth(State(shared): State<Arc<Shared>>, request: Request, next: Next) -> Response {
    if !authorized(request.headers(), &shared.control_token) {
        return ApiError(StatusCode::UNAUTHORIZED, "control authentication required")
            .into_response();
    }
    next.run(request).await
}

async fn graphql_handler(
    State(shared): State<Arc<Shared>>,
    axum::Extension(schema): axum::Extension<graphql::ApiSchema>,
    headers: HeaderMap,
    body: Result<Json<async_graphql::Request>, JsonRejection>,
) -> Response {
    if !authorized(&headers, &shared.data.lock().await.key) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"errors": [{
                "message": "Authentication required",
                "extensions": {"code": "AUTHENTICATION_ERROR"}
            }]})),
        )
            .into_response();
    }
    let Ok(Json(request)) = body else {
        return ApiError(StatusCode::BAD_REQUEST, "invalid GraphQL JSON request").into_response();
    };
    let context = Arc::new(graphql::RequestContext::new(shared));
    Json(schema.execute(request.data(context)).await).into_response()
}

async fn count(State(shared): State<Arc<Shared>>) -> Json<Value> {
    Json(json!({"api_requests": shared.count.load(Ordering::SeqCst)}))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorizationInput {
    key: String,
    issues: BTreeSet<String>,
}

// This is the SERVICE'S authorization, never Conduit's issue-scope filter.
async fn service_authorization(
    State(shared): State<Arc<Shared>>,
    Json(input): Json<AuthorizationInput>,
) -> Result<Json<Value>, ApiError> {
    let mut data = shared.data.lock().await;
    if input.key != data.key.expose() || input.issues.iter().any(|v| !synthetic_identifier(v)) {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "invalid service authorization",
        ));
    }
    data.visible = input.issues;
    Ok(Json(json!({"ok": true})))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RotateInput {
    key: String,
}

async fn rotate_key(
    State(shared): State<Arc<Shared>>,
    Json(input): Json<RotateInput>,
) -> Result<Json<Value>, ApiError> {
    let key = SyntheticToken::parse(input.key)?;
    if key.expose() == shared.control_token.expose() {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "distinct tokens required",
        ));
    }
    shared.data.lock().await.key = key;
    Ok(Json(json!({"ok": true})))
}
