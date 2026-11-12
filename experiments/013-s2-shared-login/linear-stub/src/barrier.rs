use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};

use async_graphql::{Error, ErrorExtensions};
use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{
    sync::watch,
    time::{Instant, timeout},
};

use crate::{ApiError, Shared};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Matcher {
    field: String,
    identifier: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Armed,
    Reached,
    Released,
    TimedOut,
}

pub(crate) struct Barrier {
    id: u64,
    matcher: Matcher,
    phase: watch::Sender<Phase>,
    deadline: Instant,
}

impl Barrier {
    pub fn matches(&self, field: &str, identifier: Option<&str>) -> bool {
        (self.matcher.field == "any" || self.matcher.field == field)
            && self
                .matcher
                .identifier
                .as_deref()
                .is_none_or(|v| Some(v) == identifier)
    }

    fn expire(&self) {
        self.phase.send_if_modified(|phase| {
            if matches!(phase, Phase::Armed | Phase::Reached) && Instant::now() >= self.deadline {
                *phase = Phase::TimedOut;
                true
            } else {
                false
            }
        });
    }

    pub async fn arrive(&self) -> async_graphql::Result<()> {
        self.expire();
        // Exactly one request claims an armed barrier. Later matches run normally.
        let claimed = self.phase.send_if_modified(|phase| {
            if *phase == Phase::Armed {
                *phase = Phase::Reached;
                true
            } else {
                false
            }
        });
        if !claimed {
            return Ok(());
        }
        let mut receiver = self.phase.subscribe();
        let waiting = receiver.wait_for(|phase| matches!(phase, Phase::Released | Phase::TimedOut));
        let _ = tokio::time::timeout_at(self.deadline, waiting).await;
        self.expire();
        if *self.phase.borrow() == Phase::Released {
            Ok(())
        } else {
            Err(
                Error::new("Stub barrier timed out; resolver performed no work")
                    .extend_with(|_, extensions| extensions.set("code", "STUB_BARRIER_TIMEOUT")),
            )
        }
    }

    fn answer(&self) -> Json<Value> {
        Json(json!({"id": self.id, "state": *self.phase.borrow()}))
    }

    async fn reached(&self, timeout_ms: u64) -> Result<(), ApiError> {
        let duration = bounded_timeout(timeout_ms)?;
        self.expire();
        let mut receiver = self.phase.subscribe();
        let timed_out = timeout(duration, receiver.wait_for(|phase| *phase != Phase::Armed))
            .await
            .is_err();
        if timed_out {
            return Err(ApiError(StatusCode::REQUEST_TIMEOUT, "wait timed out"));
        }
        self.expire();
        if *self.phase.borrow() == Phase::TimedOut {
            Err(ApiError(StatusCode::CONFLICT, "barrier expired"))
        } else {
            Ok(())
        }
    }
}

fn bounded_timeout(ms: u64) -> Result<Duration, ApiError> {
    if (1..=60_000).contains(&ms) {
        Ok(Duration::from_millis(ms))
    } else {
        Err(ApiError(
            StatusCode::BAD_REQUEST,
            "timeout_ms must be between 1 and 60000",
        ))
    }
}

fn invalid_input(_: JsonRejection) -> ApiError {
    ApiError(StatusCode::BAD_REQUEST, "invalid barrier input")
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArmInput {
    r#match: Matcher,
    timeout_ms: u64,
}

pub(crate) async fn arm(
    State(shared): State<Arc<Shared>>,
    input: Result<Json<ArmInput>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let Json(input) = input.map_err(invalid_input)?;
    let duration = bounded_timeout(input.timeout_ms)?;
    if !matches!(
        input.r#match.field.as_str(),
        "any" | "issue" | "issues" | "issueUpdate"
    ) || input
        .r#match
        .identifier
        .as_deref()
        .is_some_and(|id| !crate::synthetic_identifier(id))
        || (input.r#match.field == "issues" && input.r#match.identifier.is_some())
    {
        return Err(ApiError(StatusCode::BAD_REQUEST, "invalid barrier matcher"));
    }
    let mut slot = shared.barrier.lock().await;
    if let Some(current) = slot.as_ref() {
        current.expire();
        if matches!(*current.phase.borrow(), Phase::Armed | Phase::Reached) {
            return Err(ApiError(StatusCode::CONFLICT, "barrier already active"));
        }
    }
    let barrier = Arc::new(Barrier {
        id: shared.next_barrier.fetch_add(1, Ordering::SeqCst),
        matcher: input.r#match,
        phase: watch::channel(Phase::Armed).0,
        deadline: Instant::now() + duration,
    });
    let answer = barrier.answer();
    *slot = Some(barrier.clone());
    // Expire an unreached barrier too, without requiring another HTTP request.
    tokio::spawn(async move {
        tokio::time::sleep_until(barrier.deadline).await;
        barrier.expire();
    });
    Ok(answer)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WaitInput {
    id: u64,
    timeout_ms: u64,
}

async fn lookup(shared: &Shared, id: u64) -> Result<Arc<Barrier>, ApiError> {
    shared
        .barrier
        .lock()
        .await
        .as_ref()
        .filter(|barrier| barrier.id == id)
        .cloned()
        .ok_or(ApiError(StatusCode::NOT_FOUND, "barrier not found"))
}

pub(crate) async fn wait(
    State(shared): State<Arc<Shared>>,
    input: Result<Json<WaitInput>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let Json(input) = input.map_err(invalid_input)?;
    let barrier = lookup(&shared, input.id).await?;
    barrier.reached(input.timeout_ms).await?;
    Ok(barrier.answer())
}

pub(crate) async fn release(
    State(shared): State<Arc<Shared>>,
    input: Result<Json<WaitInput>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let Json(input) = input.map_err(invalid_input)?;
    let barrier = lookup(&shared, input.id).await?;
    barrier.reached(input.timeout_ms).await?;
    barrier.expire();
    barrier.phase.send_if_modified(|phase| {
        if *phase == Phase::Reached {
            *phase = Phase::Released;
            true
        } else {
            false
        }
    });
    if *barrier.phase.borrow() == Phase::TimedOut {
        return Err(ApiError(StatusCode::CONFLICT, "barrier expired"));
    }
    Ok(barrier.answer())
}
