# 027 — A5 stage 1, before its first step, 2026-11-17: Codex 0.167.1's account/read

Every result of this experiment is obtained under one operating-system user, so none shows Agents
kept apart (ruling 75 B3). This read ran nothing. A Codex worker read Codex's public source at the
tag `rust-v0.167.1` (commit `0a94398fa60c`) on 2026-11-17, under work order 197, before
[stage 1's first step](README.md#stage-1-step-1-the-account-check), for the app-server's
`account/read`. Each statement below is the source's, quoted with where it is. None is an
observation of a client, and source read at a tag can differ from the binary installed from it.
This file decides nothing and proposes nothing for order 050. It records no URL of a login, no
login literal, no claim's value and no account value.

## Sources

The tag page returned HTTP 200 before the README was amended. After that amendment, the tag alone
was cloned with depth 1 into the ignored `target/wo197/codex`. The clone’s HEAD matched the required
commit; its commit date came from `git log -1 --format=%cs`. Files were read from the working tree’s
root with `grep -n` and `sed -n`, in portions of about 80 lines or fewer. No fetched code was built
or run. Citations below are paths and lines at CX1; `...` marks cuts, including barred literals and
long lines.

| ID | Source | Where | Read on | Its own date | Read |
| --- | --- | --- | --- | --- | --- |
| CX1 | Codex source, tag `rust-v0.167.1`, commit `0a94398fa60c` | https://github.com/openai/codex/tree/rust-v0.167.1 | 2026-11-17 | 2026-11-10 | in part: the files cited |

## The method at the tag

Found at the tag: the app-server offers `account/read`, `account/rateLimits/read`, `accountSession/list`, `accountSession/switch`, quoted under R1.

## What the source says

### R1. Whether the app-server offers account/read, and how it is reached

In this reading, `account/read` is the account-reading request. The wire declaration, request and
response types, and dispatch are:

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:1533-1537)

```rust
    GetAccount => "account/read" {
        params: v2::GetAccountParams,
        serialization: global("account-auth"),
        response: v2::GetAccountResponse,
    },
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:571-592)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct GetAccountParams {
    /// When `true`, requests a proactive token refresh before returning.
    ///
    /// In managed auth mode this triggers the normal refresh-token flow. In
    /// external auth mode this flag is ignored. Clients should refresh tokens
    /// themselves and call `account/login/start` with `chatgptAuthTokens`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub refresh_token: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS, ExperimentalApi)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct GetAccountResponse {
    pub account: Option<Account>,
    pub requires_openai_auth: bool,
    #[experimental("account/read.workspaceRouting")]
    pub workspace_routing: Option<WorkspaceRouting>,
}
```

(CX1, codex-rs/app-server/src/message_processor.rs:1893-1895)

```rust
            ClientRequest::GetAccount { params, .. } => {
                self.account_processor.get_account(params).await
            }
```

In this reading, `account/rateLimits/read` also returns account identity: its `account_id` is
optional and belongs to the authenticated usage snapshot, supplied by the backend. Its request
parameters describe usage-read capabilities, and an omitted request uses their defaults. R2 to R6
follow `account/read`; this additional method's wire declaration, types and dispatch are:

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:1393-1398)

```rust
    GetAccountRateLimits => "account/rateLimits/read" {
        params: ... v2::NullableGetAccountRateLimitsParams,
        serialization: None,
        response: v2::GetAccountRateLimitsResponse,
    },

```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:341-356)

```rust
/// Usage-read capabilities of the requesting client, never inferred from its experiment arm.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct GetAccountRateLimitsParams {
    /// The client supports automatic Luna Reserve fallback. For eligible ChatGPT CLI users,
    /// allow the backend to record experiment exposure after ordinary usage is blocked.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub supports_luna_reserve: bool,
    /// Skip the separate reset-credit detail lookup for background usage polls. The usage
    /// response still includes the available count; omitted/false preserves detailed reads.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub exclude_reset_credit_details: bool,
}

pub type NullableGetAccountRateLimitsParams = Option<GetAccountRateLimitsParams>;
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:358-375)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct GetAccountRateLimitsResponse {
    /// Backend permission for ordinary included usage, validated against the active account.
    /// Null means unavailable; clients must not infer recovery from percentages or reset times.
    pub ordinary_usage_allowed: Option<bool>,
    /// Backward-compatible single-bucket view; mirrors the historical payload.
    pub rate_limits: RateLimitSnapshot,
    /// Multi-bucket view keyed by metered `limit_id` (for example, `codex`).
    pub rate_limits_by_limit_id: Option<HashMap<String, RateLimitSnapshot>>,
    pub rate_limit_reset_credits: Option<RateLimitResetCreditsSummary>,
    /// Account associated with this usage snapshot, when supplied by the backend.
    pub account_id: Option<String>,
    /// Optional backend-owned banner from the same usage read. Its nested keys retain the
    /// backend's snake_case contract; an absent banner leaves the client's existing UI unchanged.
    pub rate_limit_upsell: Option<serde_json::Value>,
}
```

(CX1, codex-rs/app-server/src/message_processor.rs:1899-1901)

```rust
            ClientRequest::GetAccountRateLimits { params, .. } => {
                self.account_processor.get_account_rate_limits(params).await
            }
```

(CX1, codex-rs/app-server/src/request_processors/account_processor.rs:205-213)

```rust
    pub(crate) async fn get_account_rate_limits(
        &self,
        params: Option<GetAccountRateLimitsParams>,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        self.get_account_rate_limits_response(params.unwrap_or_default())
            .await
            .map(|response| Some(response.into()))
    }

```

In this reading, the handler requires account authentication that uses the Codex backend, builds
its backend client from that authentication, and returns the backend response's `account_id`.
These lines show that association and the response construction; this handler is outside the
`account/read` call path and is not followed further.

(CX1, codex-rs/app-server/src/request_processors/account_processor.rs:1175-1194)

```rust
        let Some((auth, http_client_factory)) =
            self.auth_manager.auth_with_http_client_factory().await
        else {
            return Err(invalid_request(
                "codex account authentication required to read rate limits",
            ));
        };

        if !auth.uses_codex_backend() {
            return Err(invalid_request(
                "chatgpt authentication required to read rate limits",
            ));
        }

        let client = BackendClient::from_auth(
            self.config.chatgpt_base_url.clone(),
            &auth,
            http_client_factory,
        );

```

(CX1, codex-rs/app-server/src/request_processors/account_processor.rs:1263-1277)

```rust
        Ok(GetAccountRateLimitsResponse {
            ordinary_usage_allowed: response
                .ordinary_usage_allowed
                .filter(|_| matches_active_account),
            rate_limits: rate_limits.into(),
            rate_limits_by_limit_id: Some(
                rate_limits_by_limit_id
                    .into_iter()
                    .map(|(limit_id, snapshot)| (limit_id, snapshot.into()))
                    .collect(),
            ),
            rate_limit_reset_credits,
            account_id: response.account_id,
            rate_limit_upsell,
        })
```

In this reading, two further methods return account-session identity: `accountSession/list` and the
successful branch of `accountSession/switch`. The latter changes selection; it is included because
its response also reports sessions. R2 to R6 follow `account/read`; these are the additional
declarations, types and dispatches:

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:1371-1385)

```rust
    #[experimental("accountSession/list")]
    AccountSessionsList => "accountSession/list" {
        params: v2::AccountSessionsListParams,
        serialization: global("account-auth"),
        manual_payload_conversion: manual,
        response: v2::AccountSessionsResponse,
    },

    #[experimental("accountSession/switch")]
    AccountSessionsSwitch => "accountSession/switch" {
        params: v2::AccountSessionsSwitchParams,
        serialization: global("account-auth"),
        manual_payload_conversion: manual,
        response: v2::AccountSessionsSwitchResponse,
    },
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:211-217)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AccountSessionsListParams {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub refresh_workspace_metadata: bool,
}
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:227-259)

```rust
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AccountSessionsSwitchParams {
    /// Opaque active-login identifier returned by accountSession/list.
    pub session_id: String,
    /// Workspace to select within that login; this is not a user identifier.
    pub account_id: String,
    /// Allow Core to start browser sign-in if this session needs SSO for the workspace.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[ts(optional, as = "Option<bool>")]
    pub allow_browser_login: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(untagged, rename_all_fields = "camelCase")]
#[ts(untagged, rename_all_fields = "camelCase", export_to = "v2/")]
pub enum AccountSessionsSwitchResponse {
    Switched {
        #[serde(flatten)]
        sessions: AccountSessionsResponse,
    },
    /// Open the URL and await account/login/completed for this login ID.
    BrowserLogin { login_id: String, auth_url: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AccountSessionsResponse {
    pub active_session_id: Option<String>,
    pub sessions: Vec<AccountSession>,
}

```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:261-284)

```rust
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AccountSession {
    pub session_id: String,
    pub email: Option<String>,
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub image_url: Option<String>,
    pub last_used_at: i64,
    pub is_active: bool,
    pub selected_workspace_account_id: Option<String>,
    pub workspaces: Vec<AccountSessionWorkspace>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AccountSessionWorkspace {
    pub account_id: String,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub kind: Option<AccountSessionWorkspaceKind>,
}

```

(CX1, codex-rs/app-server/src/message_processor.rs:1875-1884)

```rust
            ClientRequest::AccountSessionsList { params, .. } => self
                .account_processor
                .list_account_sessions(params)
                .await
                .map(|response| Some(ClientResponsePayload::AccountSessionsList(response))),
            ClientRequest::AccountSessionsSwitch { params, .. } => self
                .account_processor
                .switch_account_session(params, &self.thread_processor, &self.turn_admission)
                .await
                .map(|response| Some(ClientResponsePayload::AccountSessionsSwitch(response))),
```

In this reading, the list handler reports the active managed login, or an empty list; its optional
metadata refresh uses `auth()` rather than `auth_cached()`. The handlers for switching and listing
are outside the `account/read` call path and are not followed further.

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_switching.rs:13-31)

```rust
    pub(crate) async fn list_account_sessions(
        &self,
        params: AccountSessionsListParams,
    ) -> Result<AccountSessionsResponse, JSONRPCErrorError> {
        let auth = if params.refresh_workspace_metadata {
            self.auth_manager.auth().await
        } else {
            self.auth_manager.auth_cached()
        }
        .filter(|_| !self.auth_manager.has_external_auth());
        let session_id = auth
            .as_ref()
            .and_then(CodexAuth::managed_account_session_id);
        let (Some(auth), Some(session_id)) = (auth, session_id) else {
            return Ok(AccountSessionsResponse {
                active_session_id: None,
                sessions: vec![],
            });
        };
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_switching.rs:78-92)

```rust
        }
        Ok(AccountSessionsResponse {
            active_session_id: Some(session_id.clone()),
            sessions: vec![AccountSession {
                session_id,
                email: auth.get_account_email(),
                user_id: auth.get_chatgpt_user_id(),
                display_name: None,
                image_url: None,
                last_used_at: chrono::Utc::now().timestamp(),
                is_active: true,
                selected_workspace_account_id: account_id,
                workspaces,
            }],
        })
```

In this reading, the deprecated `getAuthStatus` reports auth method and optionally a token, with no
account identity field. It is distinct from the identity-bearing responses above:

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:1550-1555)

```rust
    /// DEPRECATED in favor of GetAccount
    GetAuthStatus => "getAuthStatus" {
        params: v1::GetAuthStatusParams,
        serialization: global("account-auth"),
        response: v1::GetAuthStatusResponse,
    },
```

(CX1, codex-rs/app-server-protocol/src/protocol/v1.rs:192-197)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthStatusParams {
    pub include_token: Option<bool>,
    pub refresh_token: Option<bool>,
}
```

(CX1, codex-rs/app-server-protocol/src/protocol/v1.rs:208-214)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthStatusResponse {
    pub auth_method: Option<AuthMode>,
    pub auth_token: Option<String>,
    pub requires_openai_auth: Option<bool>,
}
```

In this reading, the launch command is `codex app-server`; omitting its tooling subcommand runs the
server. Its command arguments include flattened startup settings and code-mode host settings,
`--cloud-skills-snapshot`, `--strict-config`, `--listen`, `--stdio`, hidden `--remote-control` and
`--managed-daemon`, `--analytics-default-enabled`, and flattened WebSocket auth settings. The
definitions and launch are:

(CX1, codex-rs/cli/src/main.rs:171-172)

```rust
    /// [experimental] Run the app server or related tooling.
    AppServer(AppServerCommand),
```

(CX1, codex-rs/cli/src/main.rs:572-604)

```rust
struct AppServerCommand {
    #[command(flatten)]
    startup_config: codex_app_server::StartupConfigArgs,

    /// Load frozen cloud skills from a JSON snapshot instead of a live skill service.
    #[arg(long, value_name = "PATH")]
    cloud_skills_snapshot: Option<PathBuf>,
    /// Omit to run the app server; specify a subcommand for tooling.
    #[command(subcommand)]
    subcommand: Option<AppServerSubcommand>,

    #[command(flatten)]
    code_mode_host: codex_app_server::AppServerCodeModeHostArgs,

    /// Error out when config.toml contains fields that are not recognized by this version of Codex.
    #[arg(long = "strict-config", default_value_t = false)]
    strict_config: bool,

    /// Transport endpoint URL. Supported values: `...` (default),
    /// `...`, `...`, `...`, `off`.
    #[arg(
        long = "listen",
        value_name = "URL",
        default_value = codex_app_server::AppServerTransport::DEFAULT_LISTEN_URL
    )]
    listen: codex_app_server::AppServerTransport,

    /// Use stdio as the transport (equivalent to `--listen ...`).
    #[arg(long = "stdio", conflicts_with = "listen")]
    stdio: bool,

    /// Enable remote control for this app-server process without changing persistence.
    #[arg(long = "remote-control", hide = true)]
```

(CX1, codex-rs/cli/src/main.rs:605-615)

```rust
    remote_control: bool,

    /// Save loaded threads during managed daemon shutdown.
    #[arg(long, hide = true)]
    managed_daemon: bool,

    /// Controls whether analytics are enabled by default.
    ///
    /// Analytics are disabled by default for app-server. Users have to explicitly opt in
    /// via the `analytics` section in the config.toml file.
    ///
```

(CX1, codex-rs/cli/src/main.rs:626-632)

```rust
    #[arg(long = "analytics-default-enabled")]
    analytics_default_enabled: bool,

    #[command(flatten)]
    auth: codex_websocket_auth::WebsocketAuthArgs,
}

```

In this reading, the flattened startup arguments are --startup-sign-out, --print-startup-config and
--startup-client-info, with the dependencies below; the code-mode argument is --code-mode-host.
WebSocket transport auth has the further arguments quoted below. Their values were not obtained.

(CX1, codex-rs/app-server/src/startup_config.rs:29-41)

```rust
pub struct StartupConfigArgs {
    /// Sign out using the managed auth path before loading fresh startup requirements.
    #[arg(long, requires = "print_startup_config")]
    pub startup_sign_out: bool,

    /// Validate configuration and print requirements as JSON without starting the app server.
    #[arg(long, conflicts_with_all = ["listen", "stdio", "remote_control", "managed_daemon"])]
    pub print_startup_config: bool,

    /// ClientInfo JSON used for configuration/auth request identity.
    #[arg(long, requires = "print_startup_config", value_parser = |value: &str| serde_json::from...
    pub startup_client_info: Option<ClientInfo>,
}
```

(CX1, codex-rs/app-server/src/code_mode_host.rs:8-18)

```rust
/// Selects the code-mode host for a single app-server process.
#[derive(Args, Debug, Clone, Default, PartialEq, Eq)]
pub struct AppServerCodeModeHostArgs {
    /// Connect to a remote code-mode host instead of starting a local host.
    #[arg(
        long = "code-mode-host",
        value_name = "URL",
        value_parser = RedactedHostUrlParser
    )]
    pub code_mode_host: Option<Url>,
}
```

(CX1, codex-rs/websocket-auth/src/lib.rs:31-58)

```rust
pub struct WebsocketAuthArgs {
    /// Authentication mode for incoming WebSocket connections.
    #[arg(long = "ws-auth", value_name = "MODE", value_enum)]
    pub ws_auth: Option<WebsocketAuthCliMode>,

    /// Absolute path to the capability-token file.
    #[arg(long = "ws-token-file", value_name = "PATH")]
    pub ws_token_file: Option<PathBuf>,

    /// Hex-encoded SHA-256 digest of the capability token.
    #[arg(long = "ws-token-sha256", value_name = "HEX")]
    pub ws_token_sha256: Option<String>,

    /// Absolute path to the shared secret file for signed JWT bearer tokens.
    #[arg(long = "ws-shared-secret-file", value_name = "PATH")]
    pub ws_shared_secret_file: Option<PathBuf>,

    /// Expected issuer for signed JWT bearer tokens.
    #[arg(long = "ws-issuer", value_name = "ISSUER")]
    pub ws_issuer: Option<String>,

    /// Expected audience for signed JWT bearer tokens.
    #[arg(long = "ws-audience", value_name = "AUDIENCE")]
    pub ws_audience: Option<String>,

    /// Maximum clock skew when validating signed JWT bearer tokens.
    #[arg(long = "ws-max-clock-skew-seconds", value_name = "SECONDS")]
    pub ws_max_clock_skew_seconds: Option<u64>,
```

(CX1, codex-rs/cli/src/main.rs:1273-1287)

```rust
                None => {
                    let transport = if stdio {
                        codex_app_server::AppServerTransport::Stdio
                    } else {
                        listen
                    };
                    let auth = auth.try_into_settings()?;
                    if managed_daemon
                        && matches!(
                            &transport,
                            codex_app_server::AppServerTransport::UnixSocket { .. }
                        )
                    {
                        let _ = codex_app_server_daemon::maintain_stderr_log();
                    }
```

(CX1, codex-rs/cli/src/main.rs:1306-1317)

```rust
                    let exit = codex_app_server::run_main_with_transport_options(
                        arg0_paths.clone(),
                        root_config_overrides,
                        LoaderOverrides::default(),
                        strict_config,
                        analytics_default_enabled,
                        transport,
                        codex_protocol::protocol::SessionSource::VSCode,
                        auth,
                        runtime_options,
                    )
                    .await?;
```

In this reading, transports are Stdio, UnixSocket and WebSocket; Off disables the configured
transport. Stdio is the default; the `--stdio` flag also selects it. Transport URL spellings are cut
to meet this read’s no-URL rule.

(CX1, codex-rs/app-server-transport/src/transport/mod.rs:84-89)

```rust
pub enum AppServerTransport {
    Stdio,
    UnixSocket { socket_path: AbsolutePathBuf },
    WebSocket { bind_address: SocketAddr },
    Off,
}
```

(CX1, codex-rs/app-server-transport/src/transport/mod.rs:125-128)

```rust
    pub fn from_listen_url(listen_url: &str) -> Result<Self, AppServerTransportParseError> {
        if listen_url == Self::DEFAULT_LISTEN_URL {
            return Ok(Self::Stdio);
        }
```

(CX1, codex-rs/app-server-transport/src/transport/mod.rs:155-164)

```rust
        if listen_url == "off" {
            return Ok(Self::Off);
        }

        if let Some(socket_addr) = listen_url.strip_prefix("...") {
            let bind_address = socket_addr.parse::<SocketAddr>().map_err(|_| {
                AppServerTransportParseError::InvalidWebSocketListenUrl(listen_url.to_string())
            })?;
            return Ok(Self::WebSocket { bind_address });
        }
```

In this reading, `initialize` requires `clientInfo`: `name` and `version` are strings and `title` is
optional. `capabilities` is optional; its booleans default false, notification opt-outs and
extensions are optional. The request establishes initialized state before the account call; a
repeated initialization is refused. `clientInfo.name` must be a valid HTTP header value.

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:503-507)

```rust
    Initialize => "initialize" {
        params: v1::InitializeParams,
        serialization: None,
        response: v1::InitializeResponse,
    },
```

(CX1, codex-rs/app-server-protocol/src/protocol/v1.rs:27-42)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub client_info: ClientInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<InitializeCapabilities>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,
    pub title: Option<String>,
    pub version: String,
}

```

(CX1, codex-rs/app-server-protocol/src/protocol/v1.rs:44-74)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct InitializeCapabilities {
    /// Present `humanCommand` approvals to a person and return only a one-time decision.
    /// Requires `experimentalApi`. Pending requests can be replayed to compatible clients.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub required_human_approval: bool,
    /// Use explicit gateway OAuth login instead of automatic browser authorization.
    /// Applies to this app-server's gateway runtime; later connections cannot undo it.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub explicit_gateway_oauth: bool,
    /// Opt into receiving experimental API methods and fields.
    #[serde(default)]
    pub experimental_api: bool,
    /// Opt into `attestation/generate` requests for upstream `x-oai-attestation`.
    #[serde(default)]
    pub request_attestation: bool,
    /// Legacy opt-in for the `openai/form` MCP extension.
    ///
    /// New clients should declare `openai/form` in [`Self::extensions`].
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub mcp_server_openai_form_elicitation: bool,
    /// Exact notification method names that should be suppressed for this
    /// connection (for example `thread/started`).
    #[ts(optional = nullable)]
    pub opt_out_notification_methods: Option<Vec<String>>,
    /// MCP extension settings declared by the app-server client.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional = nullable)]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:68-80)

```rust
        if session.initialized() {
            return Err(invalid_request("Already initialized"));
        }

        // TODO(maxj): Revisit capability scoping for `experimental_api_enabled`.
        // Current behavior is per-connection. Reviewer feedback notes this can
        // create odd cross-client behavior (for example dynamic tool calls on a
        // shared thread when another connected client did not opt into
        // experimental API). Proposed direction is instance-global first-write-wins
        // with initialize-time mismatch rejection.
        let analytics_initialize_params = params.clone();
        let capabilities = params.capabilities.unwrap_or_default();
        let experimental_api_enabled = capabilities.experimental_api;
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:93-102)

```rust
            version,
        } = params.client_info;
        // Validate before committing; set_default_originator validates while
        // mutating process-global metadata.
        if HeaderValue::from_str(&name).is_err() {
            return Err(invalid_request(format!(
                "Invalid clientInfo.name: '{name}'. Must be a valid HTTP header value."
            )));
        }
        // Activate only the embedded TUI and local desktop host. Client-supplied
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:131-143)

```rust
        if session
            .initialize(InitializedConnectionSessionState {
                experimental_api_enabled,
                opted_out_notification_methods: opt_out_notification_methods.into_iter().collect(),
                app_server_client_name: name.clone(),
                client_version: version,
                request_attestation,
                client_mcp_extensions,
            })
            .is_err()
        {
            return Err(invalid_request("Already initialized"));
        }
```

(CX1, codex-rs/app-server/src/message_processor.rs:1012-1020)

```rust
        if !session.initialized() {
            return Err(invalid_request("Not initialized"));
        }

        if let Some(reason) = codex_request.experimental_reason()
            && !session.experimental_api_enabled()
        {
            return Err(invalid_request(experimental_required_message(reason)));
        }
```

In this reading, the protocol also defines a fieldless `initialized` notification, but this
implementation only logs it. It is not the state transition that admits the account request:

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:2156-2158)

```rust
client_notification_definitions! {
    Initialized,
}
```

(CX1, codex-rs/app-server/src/message_processor.rs:770-781)

```rust
    pub(crate) async fn process_notification(&self, notification: JSONRPCNotification) {
        // Currently, we do not expect to receive any notifications from the
        // client, so we just log them.
        tracing::info!("<- notification: {:?}", notification);
    }

    /// Handles typed notifications from in-process clients.
    pub(crate) async fn process_client_notification(&self, notification: ClientNotification) {
        // Currently, we do not expect to receive any typed notifications from
        // in-process clients, so we just log them.
        tracing::info!("<- typed notification: {:?}", notification);
    }
```

In this reading, the home is found through `CODEX_HOME`; when supplied it must be an existing
directory and is canonicalized. Otherwise the helper uses `dirs::home_dir` and appends the client’s
default directory name. No actual home was inspected in this read.

(CX1, codex-rs/app-server/src/lib.rs:549-563)

```rust
    // components do not need to work with raw TOML values.
    let cli_kv_overrides = cli_config_overrides.parse_overrides().map_err(|e| {
        std::io::Error::new(
            ErrorKind::InvalidInput,
            format!("error parsing -c overrides: {e}"),
        )
    })?;
    let codex_home = find_codex_home()?;
    let startup_cwd = std::env::current_dir().ok();
    let local_runtime_paths = ExecServerRuntimeOptions::from_optional_paths(
        arg0_paths.codex_self_exe.clone(),
        arg0_paths.codex_linux_sandbox_exe.clone(),
    )?;
    let ignore_user_config = loader_overrides.ignore_user_config;
    let config_manager = ConfigManager::new(
```

(CX1, codex-rs/utils/home-dir/src/lib.rs:13-26)

```rust
pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let codex_home_env = std::env::var("CODEX_HOME")
        .ok()
        .filter(|val| !val.is_empty());
    find_codex_home_from_env(codex_home_env.as_deref())
}

fn find_codex_home_from_env(codex_home_env: Option<&str>) -> std::io::Result<AbsolutePathBuf> {
    // Honor the `CODEX_HOME` environment variable when it is set to allow users
    // (and tests) to override the default location.
    match codex_home_env {
        Some(val) => {
            let path = PathBuf::from(val);
            let metadata = std::fs::metadata(&path).map_err(|err| match err.kind() {
```

(CX1, codex-rs/utils/home-dir/src/lib.rs:37-63)

```rust
            if !metadata.is_dir() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("CODEX_HOME points to {val:?}, but that path is not a directory"),
                ))
            } else {
                let canonical = path.canonicalize().map_err(|err| {
                    std::io::Error::new(
                        err.kind(),
                        format!("failed to canonicalize CODEX_HOME {val:?}: {err}"),
                    )
                })?;
                AbsolutePathBuf::from_absolute_path(canonical)
            }
        }
        None => {
            let mut p = home_dir().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find home directory",
                )
            })?;
            p.push(".codex");
            AbsolutePathBuf::from_absolute_path(p)
        }
    }
}
```

In this reading, `cli_auth_credentials_store` chooses File, Keyring, Auto or Ephemeral. File is the
default. Managed requirements may supply the setting; local-development builds resolve configured
Keyring or Auto to File. The file named by File is the home’s auth.json.

(CX1, codex-rs/config/src/config_toml.rs:290-295)

```rust
    /// Preferred backend for storing CLI auth credentials.
    /// file (default): Use a file in the Codex home directory.
    /// keyring: Use an OS-specific keyring service.
    /// auto: Use the keyring if available, otherwise use a file.
    #[serde(default)]
    pub cli_auth_credentials_store: Option<AuthCredentialsStoreMode>,
```

(CX1, codex-rs/config/src/types.rs:135-148)

```rust
/// Determine where Codex should store CLI auth credentials.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AuthCredentialsStoreMode {
    #[default]
    ...
    File,
    /// Persist credentials in the keyring. Fail if unavailable.
    Keyring,
    /// Use keyring when available; otherwise, fall back to a file in CODEX_HOME.
    Auto,
    /// Store credentials in memory only for the current process.
    Ephemeral,
}
```

(CX1, codex-rs/core/src/config/mod.rs:4584-4591)

```rust
            cli_auth_credentials_store_mode: match cli_auth_credentials_store {
                Some(required) => required.value,
                None => resolve_cli_auth_credentials_store_mode(
                    cfg.cli_auth_credentials_store.unwrap_or_default(),
                    env!("CARGO_PKG_VERSION"),
                ),
            },
            mcp_servers,
```

(CX1, codex-rs/core/src/config/mod.rs:288-299)

```rust
fn resolve_cli_auth_credentials_store_mode(
    configured: AuthCredentialsStoreMode,
    package_version: &str,
) -> AuthCredentialsStoreMode {
    match (package_version, configured) {
        (
            LOCAL_DEV_BUILD_VERSION,
            AuthCredentialsStoreMode::Keyring | AuthCredentialsStoreMode::Auto,
        ) => AuthCredentialsStoreMode::File,
        (_, mode) => mode,
    }
}
```

In this reading, `-c` / `--config` appends global `key=value` overrides. Both commands receive them:
the app-server passes root overrides to its entry point; login prepends them to its own overrides
before dispatch. This is configuration selection, not a separate home argument.

(CX1, codex-rs/utils/cli/src/config_override.rs:29-36)

```rust
    #[arg(
        short = 'c',
        long = "config",
        value_name = "key=value",
        action = ArgAction::Append,
        global = true,
    )]
    pub raw_overrides: Vec<String>,
```

(CX1, codex-rs/cli/src/main.rs:127-129)

```rust
struct MultitoolCli {
    #[clap(flatten)]
    pub config_overrides: CliConfigOverrides,
```

(CX1, codex-rs/cli/src/main.rs:517-519)

```rust
struct LoginCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,
```

(CX1, codex-rs/cli/src/main.rs:1647-1654)

```rust
            prepend_config_flags(
                &mut login_cli.config_overrides,
                root_config_overrides.clone(),
            );
            match login_cli.action {
                Some(LoginSubcommand::Status) => {
                    run_login_status(login_cli.config_overrides).await;
                }
```

### R2. Its answer for each login

In this reading, `refreshToken: bool` is the request’s only parameter and defaults false; true
requests proactive refresh in managed auth. Its comment says external auth ignores it. The
declaration is quoted in R1. The response fields are `account: Option<Account>`,
`requiresOpenaiAuth: bool`, and `workspaceRouting: Option<WorkspaceRouting>`, serialized in
camelCase. The handler maps account-state errors to invalid-request errors and routing errors to
internal errors:

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:258-275)

```rust
    pub(crate) async fn get_account(
        &self,
        params: GetAccountParams,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        self.refresh_token_if_requested(params.refresh_token).await;
        let read = self
            .read_account(/*request*/ None)
            .await
            .map_err(|error| match error {
                AccountReadError::InvalidAccount(error) => invalid_request(error.to_string()),
                AccountReadError::Routing(error) => internal_error(error.to_string()),
            })?;
        let mut account = read.account_state.account.map(Account::from);
        if let Some(Account::Chatgpt { workspace_name, .. }) = &mut account {
            *workspace_name = read.workspace_name;
        }
        Ok(Some(
            GetAccountResponse {
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:275-291)

```rust
            GetAccountResponse {
                account,
                requires_openai_auth: read.account_state.requires_openai_auth,
                workspace_routing: read.workspace_routing.map(|routing| {
                    codex_app_server_protocol::WorkspaceRouting {
                        chatgpt_account_id: routing.chatgpt_account_id,
                        backend_origin: routing.backend_origin,
                        account_routing_override: match routing.account_routing_override.as_str() {
                            "NO_CONSTRAINT" => AccountRoutingOverride::NoConstraint,
                            "us" => AccountRoutingOverride::Us,
                            "us_cr" => AccountRoutingOverride::UsCr,
                            _ => unreachable!("routing discovery validates the override"),
                        },
                    }
                }),
            }
            .into(),
```

In this reading, a ChatGPT account variant has `email: Option<String>`, `planType: PlanType`, and
`workspaceName: Option<String>`. The API-key variant has no fields beyond its discriminator. Other
account variants are AmazonBedrock and Gcp. Workspace routing has `chatgptAccountId: String`,
`backendOrigin: String`, and `accountRoutingOverride: AccountRoutingOverride`; no value is
reproduced here.

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:18-51)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(tag = "type")]
#[ts(export_to = "v2/")]
pub enum Account {
    #[serde(rename = "apiKey", rename_all = "camelCase")]
    #[ts(rename = "apiKey", rename_all = "camelCase")]
    ApiKey {},

    #[serde(rename = "chatgpt", rename_all = "camelCase")]
    #[ts(rename = "chatgpt", rename_all = "camelCase")]
    Chatgpt {
        #[schemars(
            required,
            schema_with = "crate::protocol::serde_helpers::nullable_string_schema"
        )]
        email: Option<String>,
        plan_type: PlanType,
        /// Display name of the selected ChatGPT workspace, when available.
        workspace_name: Option<String>,
    },

    #[serde(rename = "amazonBedrock", rename_all = "camelCase")]
    #[ts(rename = "amazonBedrock", rename_all = "camelCase")]
    AmazonBedrock {
        #[serde(default)]
        uses_codex_managed_credentials: bool,
    },

    /// Google Cloud Vertex AI with externally managed credentials.
    #[serde(rename = "gcp", rename_all = "camelCase")]
    #[ts(rename = "gcp", rename_all = "camelCase")]
    Gcp {},
}
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:53-71)

```rust
impl From<ProviderAccount> for Account {
    fn from(account: ProviderAccount) -> Self {
        match account {
            ProviderAccount::ApiKey => Self::ApiKey {},
            ProviderAccount::Chatgpt { email, plan_type } => Self::Chatgpt {
                email,
                plan_type,
                workspace_name: None,
            },
            ProviderAccount::AmazonBedrock {
                uses_codex_managed_credentials,
            } => Self::AmazonBedrock {
                uses_codex_managed_credentials,
            },
            ProviderAccount::Gcp => Self::Gcp {},
        }
    }
}

```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:594-601)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct WorkspaceRouting {
    pub chatgpt_account_id: String,
    pub backend_origin: String,
    pub account_routing_override: AccountRoutingOverride,
}
```

In this reading, the active provider determines `requiresOpenaiAuth`; an account is constructed only
when that provider requires OpenAI auth. For a cached API-key login it constructs
ProviderAccount::ApiKey. For ChatGPT and ChatgptAuthTokens it uses email and plan details. Missing
cached auth, a remembered refresh failure, or header auth yields no account; a provider that does
not require OpenAI auth also yields no account. Thus an empty home gives an absent account, rather
than an empty email string, on this path.

(CX1, codex-rs/model-provider/src/provider.rs:690-731)

```rust
    fn account_state(&self) -> ProviderAccountResult {
        let account = if self.info.requires_openai_auth {
            self.auth_manager
                .as_ref()
                .and_then(|auth_manager| {
                    let auth = auth_manager.auth_cached()?;
                    if auth_manager.refresh_failure_for_auth(&auth).is_some() {
                        return None;
                    }
                    if matches!(auth, CodexAuth::Headers(_)) {
                        return None;
                    }
                    Some(auth)
                })
                .map(|auth| match &auth {
                    CodexAuth::ApiKey(_) => Ok(ProviderAccount::ApiKey),
                    CodexAuth::BedrockApiKey(_) | CodexAuth::BedrockAccessKeys(_) => {
                        Err(ProviderAccountError::UnsupportedBedrockApiKeyAuth)
                    }
                    CodexAuth::Chatgpt(_)
                    | CodexAuth::ChatgptAuthTokens(_)
                    | CodexAuth::Headers(_)
                    | CodexAuth::AgentIdentity(_)
                    | CodexAuth::PersonalAccessToken(_) => {
                        let email = auth.get_account_email();
                        let plan_type = auth.account_plan_type();

                        plan_type
                            .map(|plan_type| ProviderAccount::Chatgpt { email, plan_type })
                            .ok_or(ProviderAccountError::MissingChatgptAccountDetails)
                    }
                })
                .transpose()?
        } else {
            None
        };

        Ok(ProviderAccountState {
            account,
            requires_openai_auth: self.info.requires_openai_auth,
        })
    }
```

In this reading, workspace discovery runs only for cached ChatGPT auth with an account ID and the
account-state condition below; without it, routing is absent. API-only and signed-out accounts
therefore have absent routing here. A selected workspace may make the entire read fail instead of
producing an account response; R4 follows that path.

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:343-356)

```rust
            let mut workspace_routing = if let Some((auth, account_id)) = auth
                .as_ref()
                .filter(|auth| {
                    auth.is_chatgpt_auth()
                        && (request.is_some()
                            || matches!(
                                account_state.account,
                                Some(ProviderAccount::Chatgpt { .. })
                            ))
                })
                .and_then(|auth| auth.get_account_id().map(|account_id| (auth, account_id)))
            {
                if account_id.is_empty() {
                    return Err(WorkspaceRoutingError::MissingAccountId.into());
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:507-510)

```rust
                Some(routing)
            } else {
                *self.workspace_routing.lock().await = None;
                None
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:535-542)

```rust
            let (workspace_routing, workspace_name) = workspace_routing
                .map(|workspace| (Some(workspace.routing), workspace.name))
                .unwrap_or_default();
            Ok(AccountRead {
                account_state,
                workspace_routing,
                workspace_name,
            })
```

In this reading, other auth-mode names in the provider match are BedrockApiKey, BedrockAccessKeys,
Headers, AgentIdentity and PersonalAccessToken. They are names, not observed logins. The account
state above is for the default OpenAI-auth provider; Gcp and AmazonBedrock have provider-owned
account states.

"API-only and signed-out accounts return `null` and do not need `accounts/check`." (CX1,
codex-rs/app-server/README.md:560)

### R3. Where the email comes from

In this reading, a managed ChatGPT login’s email is read from the parsed `id_token.email`.
`get_current_token_data` clones the in-memory auth data and applies selected-workspace token
handling; it does not deserialize the home’s auth.json on each call. The plan similarly comes from
`id_token.chatgpt_plan_type`, with Unknown used when absent; the account ID comes from
`TokenData.account_id`. Other auth variants have the explicitly separate accessors below.

(CX1, codex-rs/login/src/auth/manager.rs:660-672)

```rust
    pub fn get_account_id(&self) -> Option<String> {
        match self {
            Self::Headers(headers) => headers
                .headers()
                .get(CHATGPT_ACCOUNT_ID_HEADER)
                .and_then(|value| value.to_str().ok())
                .filter(|account_id| !account_id.is_empty() && account_id.trim() == *account_id)
                .map(ToOwned::to_owned),
            Self::AgentIdentity(auth) => Some(auth.account_id().to_string()),
            Self::PersonalAccessToken(auth) => Some(auth.account_id().to_string()),
            _ => self.get_current_token_data().and_then(|t| t.account_id),
        }
    }
```

(CX1, codex-rs/login/src/auth/manager.rs:687-695)

```rust
    /// Returns `None` if Codex backend auth does not expose an account email.
    pub fn get_account_email(&self) -> Option<String> {
        match self {
            Self::Headers(_) => None,
            Self::AgentIdentity(auth) => auth.email().map(str::to_string),
            Self::PersonalAccessToken(auth) => auth.email().map(str::to_string),
            _ => self.get_current_token_data().and_then(|t| t.id_token.email),
        }
    }
```

(CX1, codex-rs/login/src/auth/manager.rs:725-742)

```rust
    pub fn account_plan_type(&self) -> Option<AccountPlanType> {
        if matches!(self, Self::Headers(_)) {
            return None;
        }
        if let Self::AgentIdentity(auth) = self {
            return Some(auth.plan_type());
        }
        if let Self::PersonalAccessToken(auth) = self {
            return Some(auth.plan_type());
        }

        self.get_current_token_data().map(|t| {
            t.id_token
                .chatgpt_plan_type
                .map(AccountPlanType::from)
                .unwrap_or(AccountPlanType::Unknown)
        })
    }
```

(CX1, codex-rs/login/src/auth/manager.rs:748-769)

```rust

    /// Returns `None` if token-backed ChatGPT auth is unavailable.
    fn get_current_auth_json(&self) -> Option<AuthDotJson> {
        let state = match self {
            Self::Chatgpt(auth) => &auth.state,
            Self::ChatgptAuthTokens(auth) => &auth.state,
            Self::ApiKey(_)
            | Self::Headers(_)
            | Self::AgentIdentity(_)
            | Self::PersonalAccessToken(_)
            | Self::BedrockApiKey(_)
            | Self::BedrockAccessKeys(_) => return None,
        };
        #[expect(clippy::unwrap_used)]
        state.auth_dot_json.lock().unwrap().clone()
    }

    /// Returns `None` if token-backed ChatGPT auth is unavailable.
    fn get_current_token_data(&self) -> Option<TokenData> {
        self.get_current_auth_json()
            .and_then(|t| t.tokens)
            .map(workspace_switching::selected_workspace_tokens)
```

In this reading, `TokenData` stores `id_token: IdTokenInfo`, raw access and refresh token strings,
and an optional account ID. `IdTokenInfo` stores the parsed email, plan, user ID and account ID; raw
token contents are not quoted.

(CX1, codex-rs/login/src/token_data.rs:10-22)

```rust
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
pub struct TokenData {
    ...
    #[serde(
        deserialize_with = "deserialize_id_token",
        serialize_with = "serialize_id_token"
    )]
    pub id_token: IdTokenInfo,

    /// This is a JWT.
    pub access_token: String,

    pub refresh_token: String,
```

(CX1, codex-rs/login/src/token_data.rs:33)

```rust
    pub account_id: Option<String>,
```

(CX1, codex-rs/login/src/token_data.rs:44-45)

```rust
pub struct IdTokenInfo {
    pub email: Option<String>,
```

(CX1, codex-rs/login/src/token_data.rs:49-56)

```rust
    pub chatgpt_plan_type: Option<PlanType>,
    /// ChatGPT user identifier associated with the token, if present.
    pub chatgpt_user_id: Option<String>,
    /// Organization/workspace identifier associated with the token, if present.
    pub chatgpt_account_id: Option<String>,
    /// Whether the selected ChatGPT workspace must route through the FedRAMP edge.
    pub chatgpt_account_is_fedramp: bool,
    pub raw_jwt: String,
```

In this reading, `parse_chatgpt_jwt_claims` parses a top-level `email` claim or falls back to
`profile.email`; the URL-named claims are cut and identified by their fields `profile` and `auth`.
Under `auth` it parses `chatgpt_plan_type`, `chatgpt_user_id` or `user_id`, and
`chatgpt_account_id`. `decode_jwt_payload` splits the JWT, decodes its payload with base64, and
deserializes JSON with serde_json. The code quoted here does not validate a signature.

(CX1, codex-rs/login/src/token_data.rs:86-114)

```rust
#[derive(Deserialize)]
struct IdClaims {
    #[serde(default)]
    email: Option<String>,
    #[serde(rename = "...", default)]
    profile: Option<ProfileClaims>,
    #[serde(rename = "...", default)]
    auth: Option<AuthClaims>,
}

#[derive(Deserialize)]
struct ProfileClaims {
    #[serde(default)]
    email: Option<String>,
}

#[derive(Deserialize)]
struct AuthClaims {
    #[serde(default)]
    chatgpt_plan_type: Option<PlanType>,
    #[serde(default)]
    chatgpt_user_id: Option<String>,
    #[serde(default)]
    user_id: Option<String>,
    #[serde(default)]
    chatgpt_account_id: Option<String>,
    #[serde(default)]
    chatgpt_account_is_fedramp: bool,
}
```

(CX1, codex-rs/login/src/token_data.rs:135-155)

```rust
pub enum IdTokenInfoError {
    #[error("invalid ID token format")]
    InvalidFormat,
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub(crate) fn decode_jwt_payload<T: DeserializeOwned>(jwt: &str) -> Result<T, IdTokenInfoError> {
    // JWT format: header.payload.signature
    let mut parts = jwt.split('.');
    let (_header_b64, payload_b64, _sig_b64) = match (parts.next(), parts.next(), parts.next()) {
        (Some(h), Some(p), Some(s)) if !h.is_empty() && !p.is_empty() && !s.is_empty() => (h, p, s),
        _ => return Err(IdTokenInfoError::InvalidFormat),
    };

    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload_b64)?;
    let claims = serde_json::from_slice(&payload_bytes)?;
    Ok(claims)
}
```

(CX1, codex-rs/login/src/token_data.rs:189-212)

```rust
pub fn parse_chatgpt_jwt_claims(jwt: &str) -> Result<IdTokenInfo, IdTokenInfoError> {
    let claims: IdClaims = decode_jwt_payload(jwt)?;
    let email = claims
        .email
        .or_else(|| claims.profile.and_then(|profile| profile.email));

    match claims.auth {
        Some(auth) => Ok(IdTokenInfo {
            email,
            raw_jwt: jwt.to_string(),
            chatgpt_plan_type: auth.chatgpt_plan_type,
            chatgpt_user_id: auth.chatgpt_user_id.or(auth.user_id),
            chatgpt_account_id: auth.chatgpt_account_id,
            chatgpt_account_is_fedramp: auth.chatgpt_account_is_fedramp,
        }),
        None => Ok(IdTokenInfo {
            email,
            raw_jwt: jwt.to_string(),
            chatgpt_plan_type: None,
            chatgpt_user_id: None,
            chatgpt_account_id: None,
            chatgpt_account_is_fedramp: false,
        }),
    }
```

(CX1, codex-rs/login/src/token_data.rs:215-221)

```rust
fn deserialize_id_token<'de, D>(deserializer: D) -> Result<IdTokenInfo, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_chatgpt_jwt_claims(&s).map_err(serde::de::Error::custom)
}
```

In this reading, sign-in parses the exchanged ID token before storing TokenData. It separately takes
`chatgpt_account_id` from `jwt_auth_claims(&id_token)` into `TokenData.account_id`. Loading stored
JSON invokes `deserialize_id_token`; refreshing parses a replacement ID token if supplied. The
parsed auth document is held in ChatgptAuthState’s mutex.

(CX1, codex-rs/login/src/server.rs:844-847)

```rust
    let id_token = exchanged.id_token.clone();
    let access_token = exchanged.access_token.clone();
    let refresh_token = exchanged.refresh_token.clone();
    let workspace_login = opts.workspace_login.clone();
```

(CX1, codex-rs/login/src/server.rs:856-871)

```rust
    let persist_task = tokio::task::spawn_blocking(move || {
        originator.sync_scope(|| {
            let mut tokens = TokenData {
                id_token: parse_chatgpt_jwt_claims(&id_token).map_err(io::Error::other)?,
                access_token,
                refresh_token,
                oauth_provenance: Some(provenance),
                login_owner_id: Some(generate_state()),
                account_id: None,
            };
            if let Some(acc) = jwt_auth_claims(&id_token)
                .get("chatgpt_account_id")
                .and_then(|v| v.as_str())
            {
                tokens.account_id = Some(acc.to_string());
            }
```

(CX1, codex-rs/login/src/success_page.rs:106-127)

```rust
pub(crate) fn jwt_auth_claims(jwt: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut parts = jwt.split('.');
    let (_header, payload, _signature) = match (parts.next(), parts.next(), parts.next()) {
        (Some(header), Some(payload), Some(signature))
            if !header.is_empty() && !payload.is_empty() && !signature.is_empty() =>
        {
            (header, payload, signature)
        }
        _ => {
            eprintln!("Invalid JWT format while extracting claims");
            return serde_json::Map::new();
        }
    };
    match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload) {
        Ok(bytes) => match serde_json::from_slice::<serde_json::Value>(&bytes) {
            Ok(mut value) => {
                if let Some(claims) = value
                    .get_mut("...")
                    .and_then(JsonValue::as_object_mut)
                {
                    return claims.clone();
                }
```

(CX1, codex-rs/login/src/auth/storage.rs:247-254)

```rust
    pub(super) fn try_read_auth_json(&self, auth_file: &Path) -> std::io::Result<AuthDotJson> {
        let mut file = File::open(auth_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let auth_dot_json: AuthDotJson = serde_json::from_str(&contents).map_err(|err| {
            storage_error::with_context("failed to deserialize CLI auth from file", err)
        })?;

```

(CX1, codex-rs/login/src/auth/manager.rs:455-468)

```rust
        let state = ChatgptAuthState {
            auth_dot_json: Arc::new(Mutex::new(Some(auth_dot_json))),
            client,
        };

        match auth_mode {
            AuthMode::Chatgpt => {
                let storage = create_auth_storage(
                    codex_home.to_path_buf(),
                    storage_mode,
                    keyring_backend_kind,
                );
                Ok(Self::Chatgpt(ChatgptAuth { state, storage }))
            }
```

(CX1, codex-rs/login/src/auth/manager.rs:1756-1768)

```rust
    tokens.oauth_provenance = provenance
        .filter(|_| refresh_token.is_some() || tokens.refresh_token == expected.refresh_token);
    if let Some(id_token) = id_token {
        tokens.id_token = parse_chatgpt_jwt_claims(&id_token).map_err(std::io::Error::other)?;
    }
    if let Some(access_token) = access_token {
        tokens.access_token = access_token;
    }
    if let Some(refresh_token) = refresh_token {
        tokens.refresh_token = refresh_token;
    }
    auth_dot_json.last_refresh = Some(Utc::now());
    storage.compare_and_save(&original, &auth_dot_json)?;
```

In this reading, the external ChatgptAuthTokens mode instead parses the supplied access token using
the same parser and stores that parsed information in the `id_token` slot. Its account ID is
explicitly supplied; a supplied plan can override the parsed plan. This is a separate source for
email in that mode, not a claim that managed browser login uses the access token for email.

(CX1, codex-rs/login/src/auth/manager.rs:1198-1215)

```rust
pub fn login_with_chatgpt_auth_tokens(
    codex_home: &Path,
    access_token: &str,
    chatgpt_account_id: &str,
    chatgpt_plan_type: Option<&str>,
) -> std::io::Result<()> {
    let auth_dot_json = AuthDotJson::from_external_access_token(
        access_token,
        chatgpt_account_id,
        chatgpt_plan_type,
    )?;
    save_auth(
        codex_home,
        &auth_dot_json,
        AuthCredentialsStoreMode::Ephemeral,
        AuthKeyringBackendKind::default(),
    )
}
```

(CX1, codex-rs/login/src/auth/manager.rs:1880-1903)

```rust
impl AuthDotJson {
    fn from_external_access_token(
        access_token: &str,
        chatgpt_account_id: &str,
        chatgpt_plan_type: Option<&str>,
    ) -> std::io::Result<Self> {
        let mut token_info =
            parse_chatgpt_jwt_claims(access_token).map_err(std::io::Error::other)?;
        token_info.chatgpt_account_id = Some(chatgpt_account_id.to_string());
        token_info.chatgpt_plan_type = chatgpt_plan_type
            .map(InternalPlanType::from_raw_value)
            .or(token_info.chatgpt_plan_type)
            .or(...);
        let tokens = TokenData {
            id_token: token_info,
            access_token: access_token.to_string(),
            refresh_token: String::new(),
            oauth_provenance: None,
            login_owner_id: None,
            account_id: Some(chatgpt_account_id.to_string()),
        };

        Ok(Self {
            auth_mode: Some(AuthMode::ChatgptAuthTokens),
```

In this reading, selected-workspace handling may parse the access token on each token-data access
when the retained ID token describes a different workspace. If workspace and user match, it replaces
account ID, plan and routing hint in the cloned ID-token information. It does not replace email.

(CX1, codex-rs/login/src/auth/manager/workspace_switching.rs:354-366)

```rust
/// The retained ID token can still describe the previous workspace until refresh.
pub(super) fn selected_workspace_tokens(mut tokens: TokenData) -> TokenData {
    if tokens.account_id != tokens.id_token.chatgpt_account_id
        && let Ok(claims) = parse_chatgpt_jwt_claims(&tokens.access_token)
        && claims.chatgpt_account_id == tokens.account_id
        && claims.chatgpt_user_id == tokens.id_token.chatgpt_user_id
    {
        tokens.id_token.chatgpt_account_id = claims.chatgpt_account_id;
        tokens.id_token.chatgpt_plan_type = claims.chatgpt_plan_type;
        tokens.id_token.chatgpt_account_is_fedramp = claims.chatgpt_account_is_fedramp;
    }
    tokens
}
```

In this reading, `workspaceRouting.chatgptAccountId` comes from the selected account ID and the
matched account entry returned by `get_accounts_check`; `workspaceName` comes from that entry’s
name. The response’s plan is the account classification above, not a plan fetched by
`get_accounts_check`. These identifier and name values are not reproduced.

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:362-367)

```rust
                    .clone();
                let key = WorkspaceRoutingKey {
                    auth_generation: auth_state.generation,
                    chatgpt_account_id: account_id.clone(),
                    effective_chatgpt_base_url: config.chatgpt_base_url.clone(),
                    required_chatgpt_base_url: required_chatgpt_base_url.clone(),
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:456-465)

```rust
                    let mut accounts = response
                        .accounts
                        .into_iter()
                        .filter(|account| account.id == account_id);
                    let mut entry = accounts
                        .next()
                        .ok_or(WorkspaceRoutingError::MissingWorkspace)?;
                    if accounts.next().is_some() {
                        return Err(WorkspaceRoutingError::DuplicateWorkspace.into());
                    }
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:479-485)

```rust
                        return Err(WorkspaceRoutingError::ConfigurationChanged.into());
                    }
                    let name = entry.name.take();
                    let routing = resolve_routing(
                        entry,
                        required_chatgpt_base_url.as_deref(),
                        &config.chatgpt_base_url,
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:567-603)

```rust
) -> Result<WorkspaceRouting, AccountReadError> {
    let backend = entry
        .workspace_backend_origin
        .ok_or(WorkspaceRoutingError::MissingBackendOrigin)?;
    let account_routing_override = match entry.account_routing_override.as_deref() {
        Some(value ...) => value.to_owned(),
        _ => {
            return Err(WorkspaceRoutingError::InvalidRoutingOverride.into());
        }
    };
    let required = required_chatgpt_base_url
        .map(parse_backend_url)
        .transpose()?;
    let discovered = if backend == "NO_CONSTRAINT" {
        None
    } else {
        let url = parse_backend_url(&backend)?;
        if url.path() != "/" || url.query().is_some() || url.fragment().is_some() {
            return Err(WorkspaceRoutingError::BackendIsNotOrigin.into());
        }
        Some(url)
    };
    let origin = match (required, discovered) {
        (Some(required), Some(discovered)) => {
            if required.origin() != discovered.origin() {
                return Err(WorkspaceRoutingError::BackendConflict.into());
            }
            required.origin()
        }
        (Some(url), None) | (None, Some(url)) => url.origin(),
        (None, None) => parse_backend_url(effective_base_url)?.origin(),
    };
    Ok(WorkspaceRouting {
        chatgpt_account_id: entry.id,
        backend_origin: origin.ascii_serialization(),
        account_routing_override,
    })
```

In this reading, parsing stops at the base64 and serde_json library calls. The workspace selects
base64 0.22.1; CX1’s lock pins base64 0.22.1 and serde_json 1.0.149. Their internals were not read.

(CX1, codex-rs/Cargo.toml:360)

```toml
base64 = "0.22.1"
```

(CX1, codex-rs/Cargo.lock:1196-1197)

```toml
name = "base64"
version = "0.22.1"
```

(CX1, codex-rs/Cargo.lock:14195-14196)

```toml
name = "serde_json"
version = "1.0.149"
```

### R4. What answering reads, writes, refreshes or calls

In this reading, the account fields use cached auth, but the complete path is not only a cached
read. Before the app-server's entry point runs, the CLI's bootstrap may read the home's .env,
resolve the install context and create helper aliases inside the home, the first and the last quoted
below. Startup loads managed configuration and auth; the method reloads configuration and may
discover workspace routing. Refresh can occur by request, during configuration’s auth acquisition,
or during unauthorized routing recovery. The conditions and I/O are quoted below; no such operation
was executed by this read.

In this reading, the CLI's bootstrap runs before the app-server's entry point. The `codex` binary's
`main` runs every subcommand, `app-server` included, inside `arg0_dispatch_or_else`, which calls
`arg0_dispatch()` before it starts the runtime. Unless the binary was invoked as one of its helper
aliases or with a helper's first argument (codex-rs/arg0/src/lib.rs:72-130, not followed),
`arg0_dispatch()` first calls `load_dotenv()`: where `find_codex_home()` (quoted under R1) answers
and the home holds a file named `.env`, it reads that file through dotenvy's `from_path_iter` and
sets each variable it names in the process's environment, except a name that begins `CODEX_`,
compared in upper case. It then calls `InstallContext::current()` (codex-rs/arg0/src/lib.rs:137),
which resolves the home through `find_codex_home()`, canonicalizes the executable's path and the
home's, checks for a package metadata file and directories beside the executable, reading that file
on Windows only, and checks directories under the home (codex-rs/install-context/src/lib.rs:85-97,
120-139, 233-279, 310-337); this reading does not follow it. It then prepares the helper aliases: it
creates the home if it is missing, and in it a directory `tmp/arg0-` followed by the operating
system's name, set to mode 0700 on Unix; `janitor_cleanup` renames each stale per-session directory
there whose lock it can take, and removes it; then it makes a temporary directory there through
tempfile, opens in it a lock file, created if missing, and holds its lock, and, on Unix, links the
alias names to the running executable. A failure of any alias step prints a WARNING line on stderr,
and the start goes on without the aliases. In a release build, where the home's path, as
`find_codex_home()` returns it after canonicalizing `CODEX_HOME` (quoted under R1), begins with the
path that `std::env::temp_dir()` returns, compared with `starts_with`, the alias preparation refuses
with an InvalidInput error before it creates anything, so that WARNING line is printed; a debug
build does not make this check. Stage 1's first step, as the README pre-registers it, puts its
scratch root, and with it the instance's home, under the system temporary directory; whether that
home's canonical path begins with what `std::env::temp_dir()` returns is a library's answer that
this read does not show. None of these lines reads or writes the home's login file, refreshes a
token or calls the network. This reading stops at dotenvy and tempfile; CX1's lock pins dotenvy
0.15.7, quoted here, and tempfile 3.27.0, quoted with the other libraries below.

(CX1, codex-rs/cli/src/main.rs:1037-1040)

```rust
fn main() -> anyhow::Result<()> {
    codex_build_info::initialize!();
    let remote_control_disabled = codex_app_server::take_remote_control_disabled_env();
    arg0_dispatch_or_else(move |arg0_paths: Arg0DispatchPaths| async move {
```

(CX1, codex-rs/arg0/src/lib.rs:202)

```rust
    let path_entry_guard = arg0_dispatch();
```

(CX1, codex-rs/arg0/src/lib.rs:132-140)

```rust
    // This modifies the environment, which is not thread-safe, so do this
    // before creating any threads/the Tokio runtime.
    load_dotenv();

    let (path_entry_guard, updated_path_env_var) = prepare_path_env_var_with_aliases(
        InstallContext::current(),
        std::env::var_os("PATH"),
        prepare_path_entry_for_codex_aliases,
    );
```

(CX1, codex-rs/arg0/src/lib.rs:276-284)

```rust
fn load_dotenv() {
    if let Ok(codex_home) = find_codex_home()
        && let dotenv_path = codex_home.join(".env")
        && dotenv_path.is_file()
        && let Ok(iter) = dotenvy::from_path_iter(dotenv_path)
    {
        set_filtered(iter);
    }
}
```

(CX1, codex-rs/arg0/src/lib.rs:298-302)

```rust
        if !key.to_ascii_uppercase().starts_with(ILLEGAL_ENV_VAR_PREFIX) {
            // It is safe to call set_var() because our process is
            // single-threaded at this point in its execution.
            unsafe { std::env::set_var(&key, &value) };
        }
```

(CX1, codex-rs/arg0/src/lib.rs:159-167)

```rust
    match prepare_aliases(path_for_aliases) {
        Ok((path_entry, updated_path_env_var)) => (Some(path_entry), Some(updated_path_env_var)),
        Err(err) => {
            // It is possible that Codex will proceed successfully even if
            // creating helper aliases fails, so warn the user and move on.
            eprintln!("WARNING: proceeding, even though we could not create PATH aliases: {err}");
            (None, package_path)
        }
    }
```

(CX1, codex-rs/arg0/src/lib.rs:322-337)

```rust
    let codex_home = find_codex_home()?;
    #[cfg(not(debug_assertions))]
    {
        // Guard against placing helpers in system temp directories outside debug builds.
        let temp_root = std::env::temp_dir();
        if codex_home.starts_with(&temp_root) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Refusing to create helper binaries under temporary dir ..."
                ),
            ));
        }
    }

    prepare_aliases_in_home(&codex_home, existing_path)
```

(CX1, codex-rs/arg0/src/lib.rs:345-364)

```rust
    std::fs::create_dir_all(codex_home)?;
    // Windows and WSL can share CODEX_HOME without sharing a lock namespace.
    // Keep each OS outside the legacy `tmp/arg0` root so older clients cannot
    // delete these helpers while their owning process is still running.
    let temp_root = codex_home
        .join("tmp")
        .join(format!("arg0-{}", std::env::consts::OS));
    std::fs::create_dir_all(&temp_root)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // Ensure only the current user can access the temp directory.
        std::fs::set_permissions(&temp_root, std::fs::Permissions::from_mode(0o700))?;
    }

    // Best-effort cleanup of stale per-session dirs. Ignore failures so startup proceeds.
    if let Err(err) = janitor_cleanup(&temp_root) {
        eprintln!("WARNING: failed to clean up stale arg0 temp dirs: {err}");
    }
```

(CX1, codex-rs/arg0/src/lib.rs:366-378)

```rust
    let temp_dir = tempfile::Builder::new()
        .prefix("codex-arg0")
        .tempdir_in(&temp_root)?;
    let path = temp_dir.path();

    let lock_path = path.join(LOCK_FILENAME);
    let lock_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)?;
    lock_file.try_lock()?;
```

(CX1, codex-rs/arg0/src/lib.rs:386-392)

```rust
        let exe = std::env::current_exe()?;

        #[cfg(unix)]
        {
            let link = path.join(filename);
            symlink(&exe, &link)?;
        }
```

(CX1, codex-rs/arg0/src/lib.rs:489-503)

```rust
        #[cfg(unix)]
        let removal = {
            // Rename while locked so initializers cannot publish at the old path,
            // then close the lock before deletion to avoid NFS silly rename.
            let retired_dir = TempDir::new_in(temp_root)?;
            match std::fs::rename(&path, retired_dir.path()) {
                Ok(()) => {
                    drop(_lock_file);
                    retired_dir.close()
                }
                Err(err) => Err(err),
            }
        };
        #[cfg(not(unix))]
        let removal = std::fs::remove_dir_all(&path);
```

(CX1, codex-rs/Cargo.lock:6939-6940)

```toml
name = "dotenvy"
version = "0.15.7"
```

In this reading, after that bootstrap, `dispatch_cli` parses the arguments and canonicalizes the
executable's path (codex-rs/cli/src/main.rs:1049-1065) before the app-server branch at main.rs:1236;
this reading does not follow it further. The app-server's startup then first loads configuration,
creates an AuthManager, installs a managed configuration loader, then reloads configuration.
`new_from_auth_config` converts a failed managed-auth load into absent auth with `.ok().flatten()`.
It keeps the resulting auth in a cache.

(CX1, codex-rs/app-server/src/lib.rs:571-574)

```rust
    );
    let startup = config_manager
        .load_managed_startup_config(startup_cwd.clone())
        .await?;
```

(CX1, codex-rs/app-server/src/config_manager.rs:419-430)

```rust
    pub(crate) async fn load_managed_startup_config(
        &self,
        cwd: Option<PathBuf>,
    ) -> std::io::Result<CloudStartupConfig> {
        let bootstrap = self.load_startup_config(cwd.clone()).await?.config;
        let auth =
            AuthManager::shared_from_config(&bootstrap, /*enable_codex_api_key_env*/ false)
                .await
                .map_err(std::io::Error::other)?;
        self.update_managed_config_loader(auth, &bootstrap);
        let config = self.load_latest_config(cwd).await;
        Ok(CloudStartupConfig { bootstrap, config })
```

(CX1, codex-rs/app-server/src/lib.rs:598-607)

```rust
    config.validate_windows_mxc_requirement()?;
    config.auth_config().validate()?;
    let auth_manager =
        AuthManager::shared_from_config(&config, /*enable_codex_api_key_env*/ false)
            .await
            .map_err(std::io::Error::other)?;
    config_manager.update_managed_config_loader(auth_manager.clone(), &config);
    config_manager
        .sync_default_client_residency_requirement()
        .await;
```

(CX1, codex-rs/login/src/auth/manager.rs:2341-2370)

```rust
    async fn new_from_auth_config(
        auth_config: AuthConfig,
        enable_codex_api_key_env: bool,
        allowed_auth_modes: Option<Vec<AuthMode>>,
    ) -> Self {
        let managed_auth = auth_config
            .load_auth_with_modes(enable_codex_api_key_env, allowed_auth_modes.as_deref())
            .await
            .ok()
            .flatten();
        let AuthConfig {
            codex_home,
            auth_credentials_store_mode,
            keyring_backend_kind,
            forced_login_method,
            chatgpt_base_url,
            forced_chatgpt_workspace_id,
            managed_auth_policy,
            auth_route_config,
        } = auth_config;
        let agent_identity_authapi_base_url =
            agent_identity_authapi_base_url(chatgpt_base_url.as_deref()).ok();
        let (auth_change_tx, _auth_change_rx) = watch::channel(0);
        Self {
            codex_home,
            inner: RwLock::new(CachedAuth {
                auth: managed_auth,
                permanent_refresh_failure: None,
            }),
            auth_change_tx,
```

In this reading, managed auth checks the ephemeral store first and then the configured persistent
store. Loading uses storage.load; File opens and parses the home’s auth.json; a missing file yields
no auth. This path loads and parses login data before the request; ordinary accessors use the cache.

(CX1, codex-rs/login/src/auth/manager.rs:1595-1607)

```rust
    // External ChatGPT auth tokens live in the in-memory (ephemeral) store. Always check this
    // first so external auth takes precedence over any persisted credentials.
    let ephemeral_storage = create_auth_storage(
        codex_home.to_path_buf(),
        AuthCredentialsStoreMode::Ephemeral,
        AuthKeyringBackendKind::default(),
    );
    if let Some(auth_dot_json) = load_stored_auth(ephemeral_storage).await?
        && auth_mode_is_allowed(allowed_login_methods, auth_dot_json.resolved_mode())
    {
        if !mode_allowed(auth_dot_json.resolved_mode()) {
            return Ok(None);
        }
```

(CX1, codex-rs/login/src/auth/manager.rs:1668-1682)

```rust
    // Fall back to the configured persistent store (file/keyring/auto) for managed auth.
    let storage = create_auth_storage(
        codex_home.to_path_buf(),
        auth_credentials_store_mode,
        keyring_backend_kind,
    );
    let auth_dot_json = match load_stored_auth(storage).await? {
        Some(auth) => auth,
        None => return Ok(None),
    };
    if !mode_allowed(auth_dot_json.resolved_mode())
        || !auth_mode_is_allowed(allowed_login_methods, auth_dot_json.resolved_mode())
    {
        return Ok(None);
    }
```

(CX1, codex-rs/login/src/auth/manager.rs:1704-1712)

```rust
// Blocking keyring access must not prevent an async caller's deadline from firing.
async fn load_stored_auth(
    storage: Arc<dyn AuthStorageBackend>,
) -> std::io::Result<Option<AuthDotJson>> {
    let originator = codex_otel::auth_storage::AuthStorageOriginator::current();
    tokio::task::spawn_blocking(move || originator.sync_scope(|| storage.load()))
        .await
        .map_err(|_| std::io::Error::other("credential storage read failed"))?
}
```

(CX1, codex-rs/login/src/auth/storage.rs:247-255)

```rust
    pub(super) fn try_read_auth_json(&self, auth_file: &Path) -> std::io::Result<AuthDotJson> {
        let mut file = File::open(auth_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let auth_dot_json: AuthDotJson = serde_json::from_str(&contents).map_err(|err| {
            storage_error::with_context("failed to deserialize CLI auth from file", err)
        })?;

        Ok(auth_dot_json)
```

(CX1, codex-rs/login/src/auth/storage.rs:264-272)

```rust
    fn load(&self) -> std::io::Result<Option<AuthDotJson>> {
        let auth_file = get_auth_file(&self.codex_home);
        let auth_dot_json = match self.try_read_auth_json(&auth_file) {
            Ok(auth) => auth,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err),
        };
        Ok(Some(auth_dot_json))
    }
```

(CX1, codex-rs/login/src/auth/manager.rs:2545-2552)

```rust
    /// Current cached auth (clone) without attempting a refresh.
    pub fn auth_cached(&self) -> Option<CodexAuth> {
        self.inner
            .read()
            .ok()
            .and_then(|cached| cached.auth.clone())
    }

```

In this reading, Keyring uses either a Direct backend or a Secrets backend, while Auto tries keyring
and falls back to File on absence or error; Ephemeral is memory-only. The setting’s default and
development-build exception are quoted in R1. Direct reads and writes a keyring item named by
KEYRING_SERVICE and the computed home key. A successful keyring save attempts removal of the file
fallback.

(CX1, codex-rs/login/src/auth/storage.rs:700-728)

```rust
pub(super) fn create_auth_storage(
    codex_home: PathBuf,
    mode: AuthCredentialsStoreMode,
    keyring_backend_kind: AuthKeyringBackendKind,
) -> Arc<dyn AuthStorageBackend> {
    let keyring_store: Arc<dyn KeyringStore> = Arc::new(DefaultKeyringStore);
    create_auth_storage_with_store(codex_home, mode, keyring_store, keyring_backend_kind)
}

fn create_auth_storage_with_store(
    codex_home: PathBuf,
    mode: AuthCredentialsStoreMode,
    keyring_store: Arc<dyn KeyringStore>,
    keyring_backend_kind: AuthKeyringBackendKind,
) -> Arc<dyn AuthStorageBackend> {
    let commit_home = codex_home.clone();
    let storage: Arc<dyn AuthStorageBackend> = match mode {
        AuthCredentialsStoreMode::File => Arc::new(FileAuthStorage::new(codex_home)),
        AuthCredentialsStoreMode::Keyring => {
            create_keyring_auth_storage(codex_home, keyring_store, keyring_backend_kind, mode)
        }
        AuthCredentialsStoreMode::Auto => Arc::new(AutoAuthStorage::new(
            codex_home,
            keyring_store,
            keyring_backend_kind,
        )),
        AuthCredentialsStoreMode::Ephemeral => Arc::new(EphemeralAuthStorage::new(codex_home)),
    };
    storage_commit::wrap(
```

(CX1, codex-rs/login/src/auth/storage.rs:734-753)

```rust

fn create_keyring_auth_storage(
    codex_home: PathBuf,
    keyring_store: Arc<dyn KeyringStore>,
    keyring_backend_kind: AuthKeyringBackendKind,
    mode: AuthCredentialsStoreMode,
) -> Arc<dyn AuthStorageBackend> {
    match keyring_backend_kind {
        AuthKeyringBackendKind::Direct => Arc::new(DirectKeyringAuthStorage::new(
            codex_home,
            keyring_store,
            mode,
        )),
        AuthKeyringBackendKind::Secrets => Arc::new(SecretsKeyringAuthStorage::new(
            codex_home,
            keyring_store,
            mode,
        )),
    }
}
```

(CX1, codex-rs/login/src/auth/storage.rs:343-370)

```rust
    fn load_from_keyring(&self, key: &str) -> std::io::Result<Option<AuthDotJson>> {
        match self
            .keyring_store
            .load(KEYRING_SERVICE, key)
            .map_err(std::io::Error::from)
            .map_err(|error| {
                storage_error::with_context("failed to load CLI auth from keyring", error)
            })? {
            Some(serialized) => serde_json::from_str(&serialized).map(Some).map_err(|err| {
                storage_error::with_context("failed to deserialize CLI auth from keyring", err)
            }),
            None => Ok(None),
        }
    }

    fn save_to_keyring(&self, key: &str, value: &str) -> std::io::Result<()> {
        self.keyring_store
            .save(KEYRING_SERVICE, key, value)
            .map_err(std::io::Error::from)
            .map_err(|error| {
                let error =
                    storage_error::with_context("failed to write OAuth tokens to keyring", error);
                if self.mode == AuthCredentialsStoreMode::Keyring {
                    warn!("{error}");
                }
                error
            })
    }
```

(CX1, codex-rs/login/src/auth/storage.rs:378-395)

```rust
    fn load(&self) -> std::io::Result<Option<AuthDotJson>> {
        let key = compute_store_key(&self.codex_home)?;
        self.load_from_keyring(&key)
    }

    fn save(&self, auth: &AuthDotJson) -> std::io::Result<()> {
        let key = compute_store_key(&self.codex_home)?;
        // Simpler error mapping per style: prefer method reference over closure
        let serialized = serde_json::to_string(auth).map_err(std::io::Error::other)?;
        self.save_to_keyring(&key, &serialized)?;
        let mut telemetry = storage_telemetry::telemetry(
            self.mode,
            AuthKeyringBackendKind::Direct,
            Operation::Cleanup,
        );
        let result = delete_file_if_exists(&self.codex_home);
        telemetry.record_delete_attempt(Store::File, &result);
        if let Err(err) = &result {
```

(CX1, codex-rs/login/src/auth/storage.rs:599-617)

```rust
        let result = self.keyring_storage.load();
        telemetry.record_load_attempt(
            storage_telemetry::keyring_store(self.keyring_backend_kind),
            &result,
        );
        match result {
            Ok(Some(auth)) => Ok(Some(auth)),
            Ok(None) => {
                let result = self.file_storage.load();
                telemetry.record_load_attempt(Store::File, &result);
                result
            }
            Err(err) => {
                warn!("failed to load CLI auth from keyring, falling back to file storage: {err}");
                telemetry.record_secure_error(&err);
                let result = self.file_storage.load();
                telemetry.record_load_attempt(Store::File, &result);
                result
            }
```

(CX1, codex-rs/login/src/auth/storage.rs:627-639)

```rust
        let result = self.keyring_storage.save(auth);
        telemetry.record_save_attempt(
            storage_telemetry::keyring_store(self.keyring_backend_kind),
            &result,
        );
        match result {
            Ok(()) => Ok(()),
            Err(err) => {
                warn!("failed to save auth to keyring, falling back to file storage: {err}");
                telemetry.record_secure_error(&err);
                let result = self.file_storage.save(auth);
                telemetry.record_save_attempt(Store::File, &result);
                result
```

In this reading, the Secrets backend uses a Local SecretsManager under
LocalSecretsNamespace::CodexAuth. Loading and saving use its global secret; this adds an encrypted
secrets file beyond the home’s auth.json. Loading an existing encrypted file obtains a passphrase
from keyring, creating and storing one if missing; saving encrypts and writes the secrets file.
Direct keyring calls eventually enter the keyring library; this read stops there.

(CX1, codex-rs/login/src/auth/storage.rs:439-445)

```rust
            DirectKeyringAuthStorage::new(codex_home.clone(), Arc::clone(&keyring_store), mode);
        let secrets_manager = SecretsManager::new_with_keyring_store_and_namespace(
            codex_home.clone(),
            SecretsBackendKind::Local,
            keyring_store,
            LocalSecretsNamespace::CodexAuth,
        );
```

(CX1, codex-rs/login/src/auth/storage.rs:460-479)

```rust
    fn load(&self) -> std::io::Result<Option<AuthDotJson>> {
        match self
            .secrets_manager
            .get(&SecretScope::Global, &CODEX_AUTH_SECRET_NAME)
            .map_err(|error| {
                storage_error::with_context(
                    "failed to load CLI auth from encrypted auth storage",
                    error,
                )
            })? {
            Some(serialized) => serde_json::from_str(&serialized).map(Some).map_err(|err| {
                storage_error::with_context(
                    "failed to deserialize CLI auth from encrypted auth storage",
                    err,
                )
            }),
            None => Ok(None),
        }
    }

```

(CX1, codex-rs/login/src/auth/storage.rs:480-500)

```rust
    fn save(&self, auth: &AuthDotJson) -> std::io::Result<()> {
        let serialized = serde_json::to_string(auth).map_err(std::io::Error::other)?;
        self.secrets_manager
            .set(&SecretScope::Global, &CODEX_AUTH_SECRET_NAME, &serialized)
            .map_err(|error| {
                let error = storage_error::with_context(
                    "failed to write OAuth tokens to encrypted auth storage",
                    error,
                );
                if self.mode == AuthCredentialsStoreMode::Keyring {
                    warn!("{error}");
                }
                error
            })?;
        let mut telemetry = storage_telemetry::telemetry(
            self.mode,
            AuthKeyringBackendKind::Secrets,
            Operation::Cleanup,
        );
        let result = delete_file_if_exists(&self.codex_home);
        telemetry.record_delete_attempt(Store::File, &result);
```

(CX1, codex-rs/secrets/src/local.rs:197-207)

```rust
    fn load_file(&self) -> Result<SecretsFile> {
        let path = self.secrets_path();
        if !path.exists() {
            return Ok(SecretsFile::new_empty());
        }

        let ciphertext = fs::read(&path)
            .with_context(|| format!("failed to read secrets file at {}", path.display()))?;
        let passphrase = self.load_or_create_passphrase()?;
        let ciphertext_hash: [u8; 32] = Sha256::digest(&ciphertext).into();
        let passphrase_hash: [u8; 32] =
```

(CX1, codex-rs/secrets/src/local.rs:247-256)

```rust
    fn save_file(&self, file: &SecretsFile) -> Result<()> {
        let dir = self.secrets_dir();
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create secrets dir {}", dir.display()))?;

        let passphrase = self.load_or_create_passphrase()?;
        let plaintext = serde_json::to_vec(file).context("failed to serialize secrets file")?;
        let ciphertext = encrypt_with_passphrase(&plaintext, &passphrase)?;
        let path = self.secrets_path();
        write_file_atomically(&path, &ciphertext)?;
```

(CX1, codex-rs/secrets/src/local.rs:266-286)

```rust

    fn load_or_create_passphrase(&self) -> Result<SecretString> {
        let account = compute_keyring_account(&self.codex_home, self.namespace);
        let loaded = self
            .keyring_store
            .load(keyring_service(), &account)
            .map_err(std::io::Error::from)
            .with_context(|| format!("failed to load secrets key from keyring for {account}"))?;
        match loaded {
            Some(existing) => Ok(SecretString::from(existing)),
            None => {
                // Generate a high-entropy key and persist it in the OS keyring.
                // This keeps secrets out of plaintext config while remaining
                // fully local/offline for the MVP.
                let generated = generate_passphrase()?;
                self.keyring_store
                    .save(keyring_service(), &account, generated.expose_secret())
                    .map_err(std::io::Error::from)
                    .context("failed to persist secrets key in keyring")?;
                Ok(generated)
            }
```

(CX1, codex-rs/keyring-store/src/lib.rs:78-85)

```rust
    fn load(&self, service: &str, account: &str) -> Result<Option<String>, CredentialStoreError> {
        trace!("keyring.load start");
        let entry = Entry::new(service, account).map_err(CredentialStoreError::new)?;
        match entry.get_password() {
            Ok(password) => {
                trace!("keyring.load success");
                Ok(Some(password))
            }
```

(CX1, codex-rs/keyring-store/src/lib.rs:97-103)

```rust
    fn save(&self, service: &str, account: &str, value: &str) -> Result<(), CredentialStoreError> {
        trace!("keyring.save start");
        let entry = Entry::new(service, account).map_err(CredentialStoreError::new)?;
        match entry.set_password(value) {
            Ok(()) => {
                trace!("keyring.save success");
                Ok(())
```

(CX1, codex-rs/Cargo.lock:9971-9972)

```toml
name = "keyring"
version = "3.6.3"
```

In this reading, configuration reload is on every `account/read` path. It uses non-project
configuration for this method’s None fallback. The loader reads config TOML and requirements via
ExecutorFileSystem::read_file_text. These are reads beyond the login file, both at startup and on
reload; project discovery is excluded by this method’s non-project configuration builder.

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:303-335)

```rust
            let load_config = async || {
                match request.and_then(|request| request.session.as_deref()) {
                    Some(session) => {
                        self.config_manager
                            .load_retained_session_config(&session.config_layer_stack, &session.cwd)
                            .await
                    }
                    None => {
                        self.config_manager
                            .load_latest_config(/*fallback_cwd*/ None)
                            .await
                    }
                }
            };
            let config = match load_config().await {
                Ok(config) => config,
                Err(_)
                    if !self
                        .auth_manager
                        .auth_cached()
                        .as_ref()
                        .is_some_and(CodexAuth::is_chatgpt_auth) =>
                {
                    self.config.as_ref().clone()
                ...
            let auth = self.auth_manager.auth_cached();
```

(CX1, codex-rs/app-server/src/config_manager.rs:247-282)

```rust
    pub(crate) async fn load_latest_config(
        &self,
        fallback_cwd: Option<PathBuf>,
    ) -> std::io::Result<Config> {
        let Some(fallback_cwd) = fallback_cwd else {
            return self.load_non_project_config().await;
        };
        self.load_with_cli_overrides(
            &self.current_cli_overrides(),
            /*request_overrides*/ None,
            ConfigOverrides::default(),
            Some(fallback_cwd),
        )
        .await
    }

    /// Loads system, user, and runtime settings without discovering a project
    /// from the app-server process's working directory.
    pub(crate) async fn load_non_project_config(&self) -> std::io::Result<Config> {
        let mcp_ema_cancellation = self.capture_mcp_ema_cancellation();
        let policy_load = self.refresh_application_network_policy().await?;
        let mut config = ConfigBuilder::default()
            .codex_home(self.codex_home.clone())
            .cli_overrides(self.current_cli_overrides())
            .loader_overrides(self.loader_overrides.clone())
            .strict_config(self.strict_config)
            .fallback_cwd(Some(self.codex_home.clone()))
            .cloud_config_bundle(policy_load.cloud_config.clone())
            .thread_config_loader(Arc::clone(&self.thread_config_loader))
            .without_project_context()
            .build()
            .await?;
        self.check_application_policy_load(&policy_load)?;
        self.apply_network_policy(&mut config);
        self.apply_runtime_feature_enablement(&mut config);
        self.apply_arg0_paths(&mut config);
```

(CX1, codex-rs/config/src/loader/mod.rs:596-605)

```rust
    let toml_file_uri = PathUri::from_abs_path(toml_file);
    let toml_value = match fs
        .read_file_text(&toml_file_uri, Default::default(), /*sandbox*/ None)
        .await
    {
        Ok(contents) => {
            let config: TomlValue = toml::from_str(&contents).map_err(|err| {
                let config_error =
                    config_error_from_toml(toml_file.as_path(), &contents, err.clone());
                io_error_from_config_error(io::ErrorKind::InvalidData, config_error, Some(err))
```

(CX1, codex-rs/config/src/loader/mod.rs:684-697)

```rust
pub async fn load_requirements_toml(
    fs: &dyn ExecutorFileSystem,
    requirements_toml_file: &AbsolutePathBuf,
) -> io::Result<Option<RequirementsLayerEntry>> {
    let requirements_toml_file_uri = PathUri::from_abs_path(requirements_toml_file);
    match fs
        .read_file_text(
            &requirements_toml_file_uri,
            Default::default(),
            /*sandbox*/ None,
        )
        .await
    {
        Ok(contents) => {
```

In this reading, managed configuration can also obtain a cloud snapshot. The normal loader uses a
ChatGPT config-bundle service, or a configured config-server service. The latter call is named below
and not followed into its separate authentication system. The ChatGPT service uses auth(), filters
for eligible workspace plan classes, reads a valid identity-matched cache if available, otherwise
fetches and caches a bundle. Its source can recover unauthorized auth. Thus `refreshToken` false
does not exclude refresh or network use elsewhere in this path.

(CX1, codex-rs/app-server/src/application_network.rs:76-87)

```rust
        let revision = self.network_policy.policy().revision();
        let local_sources = self.load_local_network_policy(revision).await?;
        let cloud_config = self.current_managed_config_loader();
        let result = async {
            let cloud_snapshot = cloud_config.get_snapshot().await;
            let cloud = cloud_snapshot.bundle.clone().map_err(io::Error::other)?;
            let application = local_sources.compose(
                cloud
                    .as_ref()
                    .map(|bundle| bundle.requirements_toml.clone())
                    .unwrap_or_default(),
            )?;
```

(CX1, codex-rs/cloud-config/src/bundle_loader.rs:49-65)

```rust
pub fn managed_config_bundle_loader(
    auth_manager: Arc<AuthManager>,
    chatgpt_base_url: String,
    codex_home: PathBuf,
    factory: HttpClientFactory,
    requirements: &ConfigRequirementsToml,
) -> CloudConfigBundleLoader {
    if let Some(service_config) = requirements.config_server.as_ref() {
        let factory = config_server_http_client_factory(
            service_config,
            &codex_login::AuthRouteConfig::from_http_client_factory(factory),
        );
        return crate::config_server_bundle_loader(service_config.clone(), codex_home, factory);
    }
    let endpoint = codex_backend_client::Client::new(chatgpt_base_url.clone(), factory.clone())
        .config_bundle_url();
    let policy = factory
```

(CX1, codex-rs/cloud-config/src/bundle_loader.rs:132-137)

```rust
    let service = Arc::new(service);
    let background_service = Arc::clone(&service);
    let refresh_task = tokio::spawn(async move {
        let _ = background_service.get_latest().await;
        background_service.refresh_cache_in_background().await;
    });
```

(CX1, codex-rs/cloud-config/src/chatgpt.rs:33-46)

```rust
    async fn auth(&self) -> Result<Option<CodexAuth>, CloudConfigBundleLoadError> {
        Ok(self.auth_manager.auth().await.filter(|auth| {
            auth.uses_codex_backend()
                && auth.account_plan_type().is_some_and(|plan| {
                    plan.is_business_like()
                        || plan.is_education_like()
                        || matches!(
                            plan,
                            PlanType::Enterprise | PlanType::Sci | PlanType::SciPlus
                        )
                })
        }))
    }

```

(CX1, codex-rs/cloud-config/src/service.rs:186-207)

```rust
    async fn load_startup_bundle(
        &self,
    ) -> Result<Option<CloudConfigBundle>, CloudConfigBundleLoadError> {
        let Some(auth) = self.source.auth().await? else {
            return Ok(None);
        };

        if self.cache_enabled {
            // Startup prefers a valid, identity-matched cache entry. The backend is
            // only consulted on cache miss or invalid cache contents.
            let (chatgpt_user_id, account_id) = self.source.identity(&auth);
            match self
                .load_valid_cached_bundle(chatgpt_user_id.as_deref(), account_id.as_deref())
                .await
            {
                CachedBundleLookup::Hit(bundle) => return Ok(bundle),
                CachedBundleLookup::Miss => {}
            }
        }

        self.fetch_remote_bundle_and_update_cache_with_retries(auth, "startup")
            .await
```

(CX1, codex-rs/cloud-config/src/service.rs:343-355)

```rust
        loop {
            let result = self.source.get_bundle(auth, context).await;
            let Err(BundleRequestError::Unauthorized { status_code, .. }) = &result else {
                return result;
            };
            emit_fetch_attempt_metric(trigger, attempt, "unauthorized", *status_code);
            if !self
                .source
                .recover_auth(auth, context, *status_code, recovery_attempt)
                .await?
            {
                return result;
            }
```

(CX1, codex-rs/cloud-config/src/backend.rs:84-102)

```rust
    async fn get_bundle(&self, auth: &CodexAuth) -> Result<CloudConfigBundle, BundleRequestError> {
        let client = BackendClient::from_auth(
            self.base_url.clone(),
            auth,
            self.http_client_factory.clone(),
        );

        fetch_bundle(&client).await
    }
}

pub(crate) async fn fetch_bundle(
    client: &BackendClient,
) -> Result<CloudConfigBundle, BundleRequestError> {
    client
        .get_config_bundle()
        .await
        .map(bundle_from_response)
        .map_err(bundle_request_error)
```

(CX1, codex-rs/backend-client/src/client.rs:553-560)

```rust
    pub async fn get_config_bundle(
        &self,
    ) -> std::result::Result<ConfigBundleResponse, RequestError> {
        let url = self.config_bundle_url();
        let (body, ct) = self.exec_bootstrap_get(&url).await?;
        self.decode_json::<ConfigBundleResponse>(&url, &ct, &body)
            .map_err(RequestError::from)
    }
```

(CX1, codex-rs/cloud-config/src/cache.rs:64-79)

```rust
        let (Some(chatgpt_user_id), Some(account_id)) = (chatgpt_user_id, account_id) else {
            return Err(CacheLoadStatus::AuthIdentityIncomplete);
        };

        let bytes = match fs::read(&self.path).await {
            Ok(bytes) => bytes,
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    return Err(CacheLoadStatus::CacheReadFailed(err.to_string()));
                }
                return Err(CacheLoadStatus::CacheFileNotFound);
            }
        };

        let cache_file: CloudConfigBundleCacheFile = match serde_json::from_slice(&bytes) {
            Ok(cache_file) => cache_file,
```

(CX1, codex-rs/cloud-config/src/service.rs:381-394)

```rust
        let (chatgpt_user_id, account_id) = self.source.identity(auth);
        if self.cache_enabled
            && let Some(revision) = revision
        {
            let result = match self
                .cache
                .prepare(chatgpt_user_id, account_id, bundle.clone())
                .await
            {
                Ok(staged) => staged.publish_if_current(revision).await,
                Err(err) => Err(err),
            };
            if let Err(err) = result {
                tracing::warn!(error = %err, "Failed to write cloud config bundle cache");
```

(CX1, codex-rs/cloud-config/src/cache.rs:167-179)

```rust
        let destination = self.path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let parent = destination.parent().ok_or(CloudConfigBundleCacheError)?;
            std::fs::create_dir_all(parent).map_err(|_| CloudConfigBundleCacheError)?;
            let mut temporary =
                NamedTempFile::new_in(parent).map_err(|_| CloudConfigBundleCacheError)?;
            temporary
                .write_all(&serialized)
                .map_err(|_| CloudConfigBundleCacheError)?;
            Ok(StagedCloudConfigBundleCache {
                temporary,
                destination,
            })
```

(CX1, codex-rs/cloud-config/src/cache.rs:196-202)

```rust
        tokio::task::spawn_blocking(move || {
            revision.commit_if_current(|| {
                self.temporary
                    .persist(self.destination)
                    .map(|_| ())
                    .map_err(|_| CloudConfigBundleCacheError)
            })
```

In this reading, auth() may proactively refresh managed ChatGPT auth when its access token
approaches expiry, or when the fallback refresh interval has elapsed. External auth instead reloads.
The code’s thresholds are named by identifiers here, not login literals.

(CX1, codex-rs/login/src/auth/manager.rs:2599-2618)

```rust
    /// Current cached auth (clone). May be `None` if not logged in or load failed.
    /// For managed ChatGPT auth that needs a proactive refresh, first performs
    /// a guarded reload and then refreshes only if the on-disk auth is unchanged.
    #[instrument(level = "trace", skip_all)]
    pub async fn auth(&self) -> Option<CodexAuth> {
        if self.has_external_auth() {
            self.reload().await;
            return self.auth_cached();
        }

        let auth = self.auth_cached()?;
        if let Some(reason) = Self::proactive_refresh_reason(&auth)
            && let Err(err) = self.refresh_token_with_reason(reason, Some(&auth)).await
        {
            tracing::error!("Failed to refresh token: {}", err);
            return Some(auth);
        }
        self.auth_cached()
    }

```

(CX1, codex-rs/login/src/auth/manager.rs:3528-3546)

```rust
    fn proactive_refresh_reason(auth: &CodexAuth) -> Option<RefreshReason> {
        let chatgpt_auth = match auth {
            CodexAuth::Chatgpt(chatgpt_auth) => chatgpt_auth,
            _ => return None,
        };

        let auth_dot_json = chatgpt_auth.current_auth_json()?;
        if let Some(tokens) = auth_dot_json.tokens.as_ref()
            && let Ok(Some(expires_at)) = parse_jwt_expiration(&tokens.access_token)
        {
            return (expires_at
                <= Utc::now()
                    + chrono::Duration::minutes(CHATGPT_ACCESS_TOKEN_REFRESH_WINDOW_MINUTES))
            .then_some(RefreshReason::ProactiveExpiry);
        }
        let last_refresh = auth_dot_json.last_refresh?;
        (last_refresh < Utc::now() - chrono::Duration::days(TOKEN_REFRESH_INTERVAL))
            .then_some(RefreshReason::Scheduled)
    }
```

In this reading, request-specific `refreshToken` calls `refresh_token_if_requested` before reading.
It skips external ChatGPT auth; a requested managed refresh’s failure is classified and logged, and
get_account does not propagate that helper’s returned classification. The subsequent account/routing
read decides its own result.

(CX1, codex-rs/app-server/src/request_processors/account_processor.rs:1082-1094)

```rust
    async fn refresh_token_if_requested(&self, do_refresh: bool) -> RefreshTokenRequestOutcome {
        if self.auth_manager.is_external_chatgpt_auth_active() {
            return RefreshTokenRequestOutcome::NotAttemptedOrSucceeded;
        }
        if do_refresh && let Err(err) = self.auth_manager.refresh_token().await {
            let failed_reason = err.failed_reason();
            if failed_reason.is_none() {
                tracing::warn!("failed to refresh token while getting account: {err}");
                return RefreshTokenRequestOutcome::FailedTransiently;
            }
            return RefreshTokenRequestOutcome::FailedPermanently;
        }
        RefreshTokenRequestOutcome::NotAttemptedOrSucceeded
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:258-270)

```rust
    pub(crate) async fn get_account(
        &self,
        params: GetAccountParams,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        self.refresh_token_if_requested(params.refresh_token).await;
        let read = self
            .read_account(/*request*/ None)
            .await
            .map_err(|error| match error {
                AccountReadError::InvalidAccount(error) => invalid_request(error.to_string()),
                AccountReadError::Routing(error) => internal_error(error.to_string()),
            })?;
        let mut account = read.account_state.account.map(Account::from);
```

In this reading, saved ChatGPT routing is also discovered in the background at processor creation
and after initialization. The same read_account code is used. Discovery for a selected workspace
uses cached auth and the selected account ID, cached routing when its key matches, and otherwise
BackendClient::from_auth followed by get_accounts_check. An unauthorized response may reload
matching credentials and refresh before retrying; other routing failures become the read’s internal
error.

(CX1, codex-rs/app-server/src/request_processors/account_processor.rs:159-169)

```rust
        let resolver: Arc<dyn codex_login::WorkspaceRoutingResolver> = processor.clone();
        processor
            .auth_manager
            .set_workspace_routing_resolver(Arc::downgrade(&resolver));
        let startup = processor.clone();
        tokio::spawn(async move {
            let _ = startup
                .read_account_for_background_workspace_routing()
                .await;
        });
        processor
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:215-228)

```rust
    pub(super) async fn read_account_for_background_workspace_routing(
        &self,
    ) -> Result<Option<AccountRead>, AccountReadError> {
        if !self
            .auth_manager
            .auth_cached()
            .as_ref()
            .is_some_and(CodexAuth::is_chatgpt_auth)
        {
            return Ok(None);
        }

        self.read_account(/*request*/ None).await.map(Some)
    }
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:230-246)

```rust
    pub(crate) fn notify_workspace_routing_to_connection(&self, connection_id: ConnectionId) {
        let processor = self.clone();
        let auth_changes = self.auth_manager.auth_change_state_receiver();
        let owner_generation = auth_changes.borrow().owner_generation;
        tokio::spawn(async move {
            if auth_changes.borrow().owner_generation != owner_generation {
                return;
            }
            if let Ok(Some(response)) = processor
                .read_account_for_background_workspace_routing()
                .await
                && response.workspace_routing.is_some()
                && auth_changes.borrow().owner_generation == owner_generation
            {
                let notification = processor.current_account_updated_notification();
                processor
                    .outgoing
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:336-367)

```rust
            let provider = create_model_provider(
                config.model_provider.clone(),
                Some(self.auth_manager.clone()),
            );
            let account_state = provider
                .account_state()
                .map_err(AccountReadError::InvalidAccount)?;
            let mut workspace_routing = if let Some((auth, account_id)) = auth
                .as_ref()
                .filter(|auth| {
                    auth.is_chatgpt_auth()
                        && (request.is_some()
                            || matches!(
                                account_state.account,
                                Some(ProviderAccount::Chatgpt { .. })
                            ))
                })
                .and_then(|auth| auth.get_account_id().map(|account_id| (auth, account_id)))
            {
                if account_id.is_empty() {
                    return Err(WorkspaceRoutingError::MissingAccountId.into());
                }
                ...
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:392-416)

```rust
                        .workspace_routing
                        .lock()
                        .await
                        .as_ref()
                        .filter(|cached| cached.key == key)
                        .cloned(),
                };
                let routing = if let Some(cached) = cached {
                    cached
                } else {
                    let mut recovery = self.auth_manager.unauthorized_recovery();
                    let mut discovery_auth = auth.clone();
                    let mut discovery_generation = auth_state.generation;
                    let response = loop {
                        let client = BackendClient::from_auth(
                            &config.chatgpt_base_url,
                            &discovery_auth,
                            config.http_client_factory(),
                        );
                        match client.get_accounts_check().await {
                            Ok(response) => break response,
                            Err(error) => {
                                let unauthorized = error.is_unauthorized();
                                if unauthorized && recovery.has_next() {
                                    let result = recovery.next().await;
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:417-455)

```rust
                                    let refreshed = *current_auth_changes.borrow();
                                    if refreshed.owner_generation != auth_state.owner_generation {
                                        return Err(WorkspaceRoutingError::AccountChanged.into());
                                    }
                                    if let Err(error) = result
                                        && refreshed.generation == discovery_generation
                                    {
                                        let cached_failure = self
                                            .auth_manager
                                            .refresh_failure_for_auth(&discovery_auth);
                                        if let RefreshTokenError::Permanent(failed) = error
                                            && !self.auth_manager.has_external_auth()
                                            && cached_failure.as_ref() == Some(&failed)
                                        {
                                            return Err(
                                                WorkspaceRoutingError::RefreshTokenRejected(failed)
                                                    .into(),
                                            );
                                        }
                                        return Err(
                                            WorkspaceRoutingError::DiscoveryUnauthorized.into()
                                        );
                                    }
                                    discovery_generation = refreshed.generation;
                                    discovery_auth = self
                                        .auth_manager
                                        .auth_cached()
                                        .ok_or(WorkspaceRoutingError::AccountChanged)?;
                                    continue;
                                ...
                                return Err(if unauthorized {
                                    WorkspaceRoutingError::DiscoveryUnauthorized
                                } else {
                                    WorkspaceRoutingError::DiscoveryFailed
                                ...
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:466-478)

```rust
                    let latest_config = load_config().await.map_err(|error| {
                        WorkspaceRoutingError::RequirementsReload(workspace_config_load_reason(
                            &error,
                        ))
                    })?;
                    if latest_config.chatgpt_base_url != config.chatgpt_base_url
                        || latest_config.model_provider != config.model_provider
                        || latest_config
                            .config_layer_stack
                            .requirements_toml()
                            .chatgpt_base_url
                            != required_chatgpt_base_url
                    {
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:486-510)

```rust
                    )?;
                    let mut cached = self.workspace_routing.lock().await;
                    ...
                    // Same-owner refresh can complete a newer discovery first.
                    if current_auth_changes.borrow().generation == discovery_generation {
                        *cached = Some(routing.clone());
                    }
                    ...
                *fetch.routing.lock().await = Some(routing.clone());
                Some(routing)
            } else {
                *self.workspace_routing.lock().await = None;
                None
```

In this reading, the backend request is get_accounts_check, routed by PathStyle and self.base_url.
Endpoint literals are cut. It uses exec_bootstrap_get, whose request is GET, and req.send through
Codex’s HTTP wrapper. A successful discovery also supplies the workspace display name; routing is
validated before caching.

(CX1, codex-rs/backend-client/src/client.rs:424-433)

```rust
    pub async fn get_accounts_check(
        &self,
    ) -> std::result::Result<AccountsCheckResponse, RequestError> {
        let url = match self.path_style {
            PathStyle::CodexApi => format!("...", self.base_url),
            PathStyle::ChatGptApi => format!("...", self.base_url),
        };
        let (body, _) = self.exec_bootstrap_get(&url).await?;
        serde_json::from_str(&body)
            .map_err(|_| RequestError::Other(anyhow::anyhow!("Invalid accounts response.")))
```

(CX1, codex-rs/backend-client/src/client.rs:367-380)

```rust
    async fn exec_bootstrap_get(
        &self,
        url: &str,
    ) -> std::result::Result<(String, String), RequestError> {
        let request = self.request(Method::GET, url).headers(self.headers()?);
        if !self.http.allows_system_proxy_fallback() {
            return self.exec_request_detailed(request, "GET", url).await;
        }

        // Bound the complete GET, including its body, to leave time for proxy discovery and retry
        // within the cloud loader's startup budget. GETs are safe to retry after a response tim...
        match tokio::time::timeout(
            Duration::from_secs(5),
            self.exec_request_detailed(request, "GET", url),
```

(CX1, codex-rs/backend-client/src/client.rs:340-354)

```rust
    async fn exec_request_detailed(
        &self,
        req: RequestBuilder,
        method: &str,
        url: &str,
    ) -> std::result::Result<(String, String), RequestError> {
        let res = req.send().await?;
        let status = res.status();
        let content_type = res
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let body = res.text().await?;
```

In this reading, unauthorized recovery first reloads only the matching account and then can refresh
from authority. Refresh obtains an endpoint through refresh_token_endpoint and client ID through
stored provenance or oauth_client_id; no literal is quoted. The managed refresh posts through
OAuthClient, then persists returned token data and reloads the cache. Account ownership and
configuration changes can reject a pending discovery; no answer is inferred from an earlier owner’s
cached routing.

(CX1, codex-rs/login/src/auth/manager.rs:2141-2157)

```rust
            UnauthorizedRecoveryStep::Reload => {
                match self
                    .manager
                    .reload_if_account_id_matches(self.expected_account_id.as_deref())
                    .await?
                {
                    ReloadOutcome::ReloadedChanged => {
                        self.step = UnauthorizedRecoveryStep::RefreshToken;
                        return Ok(UnauthorizedRecoveryStepResult {
                            auth_state_changed: Some(true),
                        });
                    }
                    ReloadOutcome::ReloadedNoChange => {
                        self.step = UnauthorizedRecoveryStep::RefreshToken;
                        return Ok(UnauthorizedRecoveryStepResult {
                            auth_state_changed: Some(false),
                        });
```

(CX1, codex-rs/login/src/auth/manager.rs:2168-2183)

```rust
            UnauthorizedRecoveryStep::RefreshToken => {
                self.manager
                    .refresh_token_from_authority_with_reason(RefreshReason::UnauthorizedRecovery)
                    .await?;
                self.step = UnauthorizedRecoveryStep::Done;
                return Ok(UnauthorizedRecoveryStepResult {
                    auth_state_changed: Some(true),
                });
            }
            UnauthorizedRecoveryStep::ExternalRefresh => {
                self.manager
                    .refresh_token_from_authority_with_reason(RefreshReason::UnauthorizedRecovery)
                    .await?;
                self.step = UnauthorizedRecoveryStep::Done;
                return Ok(UnauthorizedRecoveryStepResult {
                    auth_state_changed: Some(true),
```

(CX1, codex-rs/login/src/auth/manager.rs:3359-3382)

```rust
        }
        if self.has_external_auth() {
            return self.refresh_token_from_authority_impl(reason).await;
        }
        if auth_before_reload
            .as_ref()
            .is_some_and(|auth| auth.is_api_key_auth() || auth.is_personal_access_token_auth())
        {
            return Ok(());
        }
        let expected_account_id = auth_before_reload
            .as_ref()
            .and_then(CodexAuth::get_account_id);

        match self
            .reload_if_account_id_matches(expected_account_id.as_deref())
            .await?
        {
            ReloadOutcome::ReloadedChanged => {
                tracing::info!("Skipping token refresh because auth changed after guarded reload.");
                Ok(())
            }
            ReloadOutcome::ReloadedNoChange => self.refresh_token_from_authority_impl(reason).await,
            ReloadOutcome::Skipped => {
```

(CX1, codex-rs/login/src/auth/manager.rs:3426-3453)

```rust
        let result = if self.has_external_auth() {
            self.refresh_external_auth(ExternalAuthRefreshReason::Unauthorized)
                .await
        } else {
            match attempted_auth.as_ref() {
                Some(CodexAuth::Chatgpt(chatgpt_auth)) => {
                    let token_data = chatgpt_auth.current_token_data().ok_or_else(|| {
                        RefreshTokenError::Transient(std::io::Error::other(
                            "Token data is not available.",
                        ))
                    })?;
                    Box::pin(self.refresh_and_persist_chatgpt_token(
                        chatgpt_auth,
                        token_data,
                        reason,
                    ))
                    .await
                }
                Some(
                    CodexAuth::ApiKey(_)
                    | CodexAuth::ChatgptAuthTokens(_)
                    | CodexAuth::Headers(_)
                    | CodexAuth::AgentIdentity(_)
                    | CodexAuth::PersonalAccessToken(_)
                    | CodexAuth::BedrockApiKey(_)
                    | CodexAuth::BedrockAccessKeys(_),
                )
                | None => Ok(()),
```

(CX1, codex-rs/login/src/auth/manager.rs:3618-3637)

```rust
        let endpoint = refresh_token_endpoint(tokens.oauth_provenance.as_ref());
        let client_id = tokens
            .oauth_provenance
            .as_ref()
            .map(|provenance| provenance.client_id.clone())
            .unwrap_or_else(oauth_client_id);
        let previous_unsettled = self
            .unsettled_chatgpt_refresh
            .lock()
            .map_err(|_| std::io::Error::other("Refresh progress is unavailable."))?
            .replace(tokens.clone());
        let result: Result<(), RefreshTokenError> = async {
            let refresh_response = request_chatgpt_token_refresh(
                tokens.refresh_token.clone(),
                &endpoint,
                &client_id,
                auth.client(),
                reason,
            )
            .await?;
```

(CX1, codex-rs/login/src/auth/manager.rs:3645-3659)

```rust
            let storage = Arc::clone(auth.storage());
            let originator = codex_otel::auth_storage::AuthStorageOriginator::current();
            tokio::task::spawn_blocking(move || {
                originator.sync_scope(|| {
                    persist_tokens(
                        &storage,
                        &tokens,
                        provenance,
                        refresh_response.id_token,
                        refresh_response.access_token,
                        refresh_response.refresh_token,
                    )
                })
            })
            .await
```

(CX1, codex-rs/login/src/auth/manager.rs:3681-3685)

```rust
        result?;
        self.reload().await;

        Ok(())
    }
```

(CX1, codex-rs/login/src/auth/manager.rs:1781-1798)

```rust
    let oauth = OAuthClient::new(
        client,
        TokenEndpoint {
            url: endpoint,
            client_id,
            encoding: TokenEncoding::Json,
            timeout: Some(Duration::from_secs(45)),
            error_body_limit: ErrorBodyLimit::Unlimited,
        },
    );
    match oauth
        .refresh(RefreshTokenGrant {
            refresh_token: &refresh_token,
            resource: None,
            refresh_reason: Some(refresh_reason),
        })
        .await
    {
```

(CX1, codex-rs/login/src/oauth/client.rs:116-122)

```rust
        &self,
        parameters: &[(&str, &str)],
        secrets: &[&str],
    ) -> Result<T, OAuthError> {
        let mut request = self.http_client.post(self.endpoint.url);
        if let Some(timeout) = self.endpoint.timeout {
            request = request.timeout(timeout);
```

(CX1, codex-rs/login/src/oauth/client.rs:132-140)

```rust
            TokenEncoding::Json => request
                .header("Content-Type", "application/json")
                .json(&parameters.iter().copied().collect::<BTreeMap<_, _>>()),
        };
        let response = request
            .send()
            .await
            .map_err(|error| OAuthError::Transport(redact_error_url(error)))?;
        if !response.status().is_success() {
```

(CX1, codex-rs/login/src/auth/manager.rs:1734-1740)

```rust
    let _commit = storage.lock_for_write()?;
    let stored = storage.load()?;
    let mut auth_dot_json = stored.ok_or(std::io::Error::other("Token data is not available."))?;
    let original = auth_dot_json.clone();
    if auth_dot_json.resolved_mode() != AuthMode::Chatgpt {
        return Err(std::io::Error::other(
            "Account credentials changed during refresh.",
```

(CX1, codex-rs/login/src/auth/manager.rs:1756-1768)

```rust
    tokens.oauth_provenance = provenance
        .filter(|_| refresh_token.is_some() || tokens.refresh_token == expected.refresh_token);
    if let Some(id_token) = id_token {
        tokens.id_token = parse_chatgpt_jwt_claims(&id_token).map_err(std::io::Error::other)?;
    }
    if let Some(access_token) = access_token {
        tokens.access_token = access_token;
    }
    if let Some(refresh_token) = refresh_token {
        tokens.refresh_token = refresh_token;
    }
    auth_dot_json.last_refresh = Some(Utc::now());
    storage.compare_and_save(&original, &auth_dot_json)?;
```

In this reading, refreshed File credentials replace the home’s auth.json through a temporary file
and rename; keyring and Auto saves use the backends already quoted. Persistence additionally creates
and locks auth.lock and a separate commit lock before the write. Loads delegate without taking that
commit lock. These are file effects beyond reading the login file.

(CX1, codex-rs/login/src/auth/storage.rs:195-208)

```rust
    fn lock_for_write(
        &self,
    ) -> std::io::Result<(std::sync::MutexGuard<'static, ()>, Option<File>)> {
        let guard = auth_storage_commit_guard()?;
        let file = if let Some(home) = self.persistent_home() {
            std::fs::create_dir_all(home)?;
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(home.join("auth.lock"))?;
            file.lock()?;
            Some(file)
```

(CX1, codex-rs/login/src/auth/storage_commit.rs:29-52)

```rust
    fn lock(&self) -> std::io::Result<File> {
        let auth_file = get_auth_file(&self.home);
        // Keyring commits can unlink an intermediate symlink used by another
        // home, so resolvable chains must share the target's commit lock.
        let path = match storage_file::resolve_auth_file(auth_file.clone()) {
            Ok(path) => path,
            // Unresolvable fallback links can still be removed by keyring commits.
            Err(_) if self.mode == AuthCredentialsStoreMode::Keyring => auth_file,
            Err(error) => return Err(error),
        }
        .with_extension("json.lock");
        std::fs::create_dir_all(
            path.parent()
                .ok_or_else(|| std::io::Error::other("credential lock has no parent"))?,
        )?;
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let file = options.open(path)?;
        let deadline = Instant::now() + Duration::from_secs(5);
```

(CX1, codex-rs/login/src/auth/storage_commit.rs:76-88)

```rust
    fn load(&self) -> std::io::Result<Option<AuthDotJson>> {
        self.inner.load()
    }
    fn load_account_grant(&self) -> std::io::Result<Option<AuthDotJson>> {
        self.inner.load_account_grant()
    }
    fn save(&self, auth: &AuthDotJson) -> std::io::Result<()> {
        let _guard = self.lock()?;
        self.inner.save(auth)
    }
    fn compare_and_save(&self, expected: &AuthDotJson, auth: &AuthDotJson) -> std::io::Result<()> {
        let _guard = self.lock()?;
        self.inner.compare_and_save(expected, auth)
```

(CX1, codex-rs/login/src/auth/storage.rs:274-293)

```rust
    fn save(&self, auth_dot_json: &AuthDotJson) -> std::io::Result<()> {
        let auth_file = storage_file::resolve_auth_file(get_auth_file(&self.codex_home))?;
        let parent = auth_file
            .parent()
            .ok_or_else(|| std::io::Error::other("auth file has no parent"))?;
        std::fs::create_dir_all(parent)?;
        let json_data = serde_json::to_string_pretty(auth_dot_json)?;
        storage_file::check_replaceable(&auth_file)?;
        // tempfile creates a private (0600 on Unix) file in the same filesystem.
        // Readers see the complete old or new document, never a truncated ....
        let mut replacement = storage_file::replacement(&auth_file)?;
        replacement.write_all(json_data.as_bytes())?;
        replacement.as_file().sync_all()?;
        // Clear Windows' temporary attribute, then restore cleanup protection.
        // std::fs::rename can replace a file held open by a reader on Windows;
        // tempfile::persist only uses MoveFileExW, which rejects that case.
        let (_file, path) = replacement.keep().map_err(|error| error.error)?;
        let mut path = tempfile::TempPath::try_from_path(path)?;
        std::fs::rename(&path, auth_file)?;
        path.disable_cleanup(/*disable_cleanup*/ true);
```

In this reading, starting the app-server also initializes SQLite state and logging and resolves an
installation ID, independent of the identity fields the account response needs. SQLite setup
opens/migrates state databases and can back up damaged files; installation-ID resolution creates,
reads and may rewrite a file. These are additional startup effects, not operations performed by this
source read.

(CX1, codex-rs/app-server/src/lib.rs:663-673)

```rust
    codex_core::otel_init::install_sqlite_telemetry(otel.as_ref(), OTEL_SERVICE_NAME);
    let unix_socket_startup_lock = match &transport {
        AppServerTransport::UnixSocket { .. } => {
            let startup_lock_path = app_server_startup_lock_path(&codex_home)?;
            let startup_lock = acquire_app_server_startup_lock(startup_lock_path).await?;
            Some(startup_lock)
        }
        _ => None,
    };
    let state_db_init = match init_sqlite_state_db_with_fresh_start_on_corruption(&config).await {
        Ok(state_db_init) => state_db_init,
```

(CX1, codex-rs/app-server/src/lib.rs:1428-1440)

```rust
        let mut attempted_backups = HashSet::new();
        loop {
            let err = match rollout_state_db::try_init(config).await {
                Ok(state_db) => return Ok(state_db),
                Err(err) => err,
            };
            let database_path = codex_state::runtime_db_path_for_corruption_error(&err)
                .unwrap_or_else(|| config.sqlite_config().state_db_path());
            if !codex_state::is_sqlite_corruption_error(&err)
                && !sqlite_home_is_blocking_file(database_path.as_path())
            {
                return Err(err);
            }
```

(CX1, codex-rs/app-server/src/lib.rs:1451-1456)

```rust
                database_path.display()
            ));
            let backups = codex_state::backup_runtime_db_for_fresh_start(database_path.as_path())
                .await
                .map_err(|backup_err| {
                    anyhow::anyhow!(
```

(CX1, codex-rs/state/src/runtime.rs:116-124)

```rust
    /// Initialize the state runtime using the provided SQLite configuration and default provider.
    ///
    /// This opens (and migrates) the SQLite databases under the configured
    /// `sqlite_home`.
    /// Logs and paginated thread history live in dedicated files to reduce
    /// lock contention with the rest of the state store.
    pub async fn init(sqlite: SqliteConfig, default_provider: String) -> anyhow::Result<Arc<Self>> {
        Self::init_inner(sqlite, default_provider, /*telemetry_override*/ None).await
    }
```

(CX1, codex-rs/state/src/runtime.rs:135-145)

```rust
    async fn init_inner(
        sqlite: SqliteConfig,
        default_provider: String,
        telemetry_override: Option<&dyn DbTelemetry>,
    ) -> anyhow::Result<Arc<Self>> {
        tokio::fs::create_dir_all(sqlite.home()).await?;
        let state_migrator = runtime_state_migrator();
        let logs_migrator = runtime_logs_migrator();
        let goals_migrator = runtime_goals_migrator();
        let memories_migrator = runtime_memories_migrator();
        let queue_migrator = runtime_queue_migrator();
```

(CX1, codex-rs/state/src/runtime.rs:150-175)

```rust
        let queue_path = sqlite.queue_db_path();
        let pool = match sqlite
            .open_state_db(&state_migrator, telemetry_override)
            .await
        {
            Ok(db) => Arc::new(db),
            Err(err) => {
                warn!("failed to open state db at {}: {err}", state_path.display());
                return Err(err);
            }
        };
        let has_memories_v2 = tokio::fs::try_exists(sqlite.memories_v2_db_path()).await?;
        let logs_pool = match sqlite
            .open_logs_db(&logs_migrator, telemetry_override)
            .await
        {
            Ok(db) => Arc::new(db),
            Err(err) => {
                warn!("failed to open logs db at {}: {err}", logs_path.display());
                close_sqlite_pools(&[pool.as_ref()]).await;
                return Err(err);
            }
        };
        let goals_pool = match sqlite
            .open_goals_db(&goals_migrator, telemetry_override)
            .await
```

(CX1, codex-rs/state/src/runtime.rs:185-205)

```rust
            .open_memories_db(&memories_migrator, telemetry_override)
            .await
        {
            Ok(db) => Arc::new(db),
            Err(err) => {
                warn!(
                    "failed to open memories db at {}: {err}",
                    memories_path.display()
                );
                close_sqlite_pools(&[pool.as_ref(), logs_pool.as_ref(), goals_pool.as_ref()]).await;
                return Err(err);
            }
        };
        let queue_pool = match sqlite
            .open_queue_db(&queue_migrator, telemetry_override)
            .await
        {
            Ok(db) => Arc::new(db),
            Err(err) => {
                warn!("failed to open queue db at {}: {err}", queue_path.display());
                close_sqlite_pools(&[
```

(CX1, codex-rs/app-server/src/lib.rs:752-758)

```rust
    let feedback_metadata_layer = feedback.metadata_layer();
    let log_db = state_db
        .clone()
        .map(|state_db| log_db::start(state_db, log_write_warning.clone()));
    let log_db_layer = log_db
        .clone()
        .map(|layer| layer.with_filter(log_db::default_filter()));
```

(CX1, codex-rs/app-server/src/lib.rs:799)

```rust
    let installation_id = resolve_installation_id(&config.codex_home).await?;
```

(CX1, codex-rs/core/src/installation_id.rs:19-32)

```rust
pub async fn resolve_installation_id(codex_home: &AbsolutePathBuf) -> Result<String> {
    let path = codex_home.join(INSTALLATION_ID_FILENAME);
    fs::create_dir_all(codex_home).await?;
    tokio::task::spawn_blocking(move || {
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true);

        #[cfg(unix)]
        {
            options.mode(0o644);
        }

        let mut file = options.open(&path)?;
        file.lock()?;
```

(CX1, codex-rs/core/src/installation_id.rs:44-59)

```rust

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let trimmed = contents.trim();
        if !trimmed.is_empty()
            && let Ok(existing) = Uuid::parse_str(trimmed)
        {
            return Ok(existing.to_string());
        }

        let installation_id = Uuid::new_v4().to_string();
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(installation_id.as_bytes())?;
        file.flush()?;
        file.sync_all()?;
```

In this reading, initialization records analytics and, for specific experimental desktop hosts, may
probe a verification device; on Windows a particular desktop client may register its installation.
These calls do not supply the account response and are not followed. Initialization also changes
gateway-login policy. It sends InitializeResponse before marking outbound readiness; no initialized
notification performs I/O beyond logging in the quoted notification handler.

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:103-113)

```rust
        // extensions cannot opt other hosts into verification.
        let user_verification_enabled = experimental_api_enabled
            && matches!(
                (session.origin, name.as_str()),
                (ConnectionOrigin::InProcess, "codex-tui")
                    | (ConnectionOrigin::Stdio, "Codex Desktop")
            )
            && tokio::task::spawn_blocking(self.user_verification.device_supported)
                .await
                .unwrap_or(false);
        if user_verification_enabled {
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:154-165)

```rust

        if capabilities.explicit_gateway_oauth || mutates_global_identity {
            // Only the first originating client may restore legacy automatic login.
            // Any explicit opt-in is sticky across subsequent connections.
            self.gateway_login_initialized.get_or_init(|| {
                if !capabilities.explicit_gateway_oauth {
                    self.gateway_login_control.allow_automatic_login();
                }
            });
            if capabilities.explicit_gateway_oauth {
                self.gateway_login_control.require_explicit_login();
            }
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:186-191)

```rust
        }
        self.analytics_events_client.track_initialize(
            connection_id.0,
            analytics_initialize_params,
            originator,
            self.rpc_transport,
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:197-208)

```rust

        #[cfg(windows)]
        if matches!(session.origin, ConnectionOrigin::Stdio) && name == "Codex Desktop" {
            // Uninstall ownership must not depend on account sign-in or sandbox setup.
            // Keep this bounded attempt ahead of the response; background registration can race...
            let home = codex_home.clone();
            if !matches!(
                tokio::task::spawn_blocking(move || {
                    codex_windows_sandbox::register_desktop_installation(&home)
                })
                .await,
                Ok(Ok(()))
```

(CX1, codex-rs/app-server/src/request_processors/initialize_processor.rs:214-231)

```rust
        let user_agent = get_codex_user_agent();
        let response = InitializeResponse {
            user_agent,
            codex_home,
            platform_family: std::env::consts::FAMILY.to_string(),
            platform_os: std::env::consts::OS.to_string(),
        };

        self.outgoing
            .send_response(connection_request_id, response)
            .await;

        if let Some(outbound_initialized) = outbound_initialized {
            outbound_initialized.store(true, Ordering::Release);
            return Ok(true);
        }

        Ok(false)
```

In this reading, selecting authenticated WebSocket transport can additionally read the configured
capability-token file or shared-secret file. These are transport credentials, separate from the
account being reported; their contents were not read. The configuration and file-read lines are:

(CX1, codex-rs/websocket-auth/src/lib.rs:236-258)

```rust
/// Loads credentials once; changes to secret files do not update an existing policy.
pub fn policy_from_settings(settings: &WebsocketAuthSettings) -> io::Result<WebsocketAuthPolicy> {
    let mode = match settings.config.as_ref() {
        Some(WebsocketAuthConfig::CapabilityToken { source }) => match source {
            WebsocketCapabilityTokenSource::TokenFile { token_file } => {
                let token = read_trimmed_secret(token_file.as_ref())?;
                Some(WebsocketAuthMode::CapabilityToken {
                    token_sha256: sha256_digest(token.as_bytes()),
                })
            }
            WebsocketCapabilityTokenSource::TokenSha256 { token_sha256 } => {
                Some(WebsocketAuthMode::CapabilityToken {
                    token_sha256: *token_sha256,
                })
            }
        },
        Some(WebsocketAuthConfig::SignedBearerToken {
            shared_secret_file,
            issuer,
            audience,
            max_clock_skew_seconds,
        }) => {
            let shared_secret = read_trimmed_secret(shared_secret_file.as_ref())?.into_bytes();
```

(CX1, codex-rs/websocket-auth/src/lib.rs:416-425)

```rust
fn read_trimmed_secret(path: &std::path::Path) -> io::Result<String> {
    let raw = std::fs::read_to_string(path).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!(
                "failed to read websocket auth secret {}: {err}",
                path.display()
            ),
        )
    })?;
```

In this reading, other startup calls are outside what this answer needs and are not followed:
FrozenCloudSkillProvider::from_file when a snapshot is supplied (app-server/src/lib.rs:520-528);
GrpcCodeModeSessionProvider::with_http_client_factory for a configured host (619-630);
EnvironmentManager::from_env/from_codex_home (636-647); otel_init::build_provider and
record_process_start (650-663); check_execpolicy_for_warnings (692-694);
start_control_socket_acceptor/start_websocket_acceptor for those transports (831-858);
start_remote_control (886-899); codex_home_metrics::spawn and otel_reloader::spawn (929-941); and
thread-store, queue, thread-manager and plugin services constructed in message_processor.rs:317-365.
Paths in this sentence have the `codex-rs/` prefix. Each is a boundary of this reading, not a claim
of no further I/O. The CLI’s managed-daemon-only maintain_stderr_log call
(cli/src/main.rs:1281-1287) is also not followed.

In this reading, the HTTP calls above stop at Codex’s HttpClient abstraction and its reqwest
library; the keyring calls stop at keyring::Entry; JSON and base64 stop at their library calls. The
selected workspace dependencies are reqwest 0.12, base64 0.22.1, serde_json 1 and keyring 3.6. CX1’s
lock pins reqwest 0.12.28, base64 0.22.1, serde_json 1.0.149, keyring 3.6.3, dirs 6.0.0, tempfile
3.27.0 and sqlx 0.9.0. Library internals were not read; lock entries for the first parsing libraries
and keyring are quoted above.

(CX1, codex-rs/Cargo.toml:469)

```toml
reqwest = { version = "0.12", features = ["cookies"] }
```

(CX1, codex-rs/http-client/src/request_builder.rs:181-188)

```rust
    pub async fn send(self) -> Result<HttpResponse, HttpError> {
        match self.backend {
            HttpClientBackend::Routed(pool) => pool.send(self.request?).await,
            HttpClientBackend::Direct(client) => {
                Ok(client.execute(self.request?.build(&client)?).await?.into())
            }
        }
    }
```

(CX1, codex-rs/Cargo.lock:13099-13100)

```toml
name = "reqwest"
version = "0.12.28"
```

(CX1, codex-rs/Cargo.lock:6820-6821)

```toml
name = "dirs"
version = "6.0.0"
```

(CX1, codex-rs/Cargo.lock:15385-15386)

```toml
name = "tempfile"
version = "3.27.0"
```

(CX1, codex-rs/Cargo.lock:14670-14671)

```toml
name = "sqlx"
version = "0.9.0"
```

### R5. Whether it is marked experimental

In this reading, the CLI labels the app-server command experimental. The account/read request
declaration and its GetAccountParams have no experimental marker and require no experimental
capability to call the method. The response field workspaceRouting is explicitly experimental. The
additional accountSession/list and accountSession/switch methods are experimental and pass through
the initialized-session experimental gate. The external ChatgptAuthTokens auth mode is marked
unstable.

(CX1, codex-rs/cli/src/main.rs:171-172)

```rust
    /// [experimental] Run the app server or related tooling.
    AppServer(AppServerCommand),
```

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:1533-1537)

```rust
    GetAccount => "account/read" {
        params: v2::GetAccountParams,
        serialization: global("account-auth"),
        response: v2::GetAccountResponse,
    },
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:571-592)

```rust
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct GetAccountParams {
    /// When `true`, requests a proactive token refresh before returning.
    ///
    /// In managed auth mode this triggers the normal refresh-token flow. In
    /// external auth mode this flag is ignored. Clients should refresh tokens
    /// themselves and call `account/login/start` with `chatgptAuthTokens`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub refresh_token: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS, ExperimentalApi)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct GetAccountResponse {
    pub account: Option<Account>,
    pub requires_openai_auth: bool,
    #[experimental("account/read.workspaceRouting")]
    pub workspace_routing: Option<WorkspaceRouting>,
}
```

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:1371-1385)

```rust
    #[experimental("accountSession/list")]
    AccountSessionsList => "accountSession/list" {
        params: v2::AccountSessionsListParams,
        serialization: global("account-auth"),
        manual_payload_conversion: manual,
        response: v2::AccountSessionsResponse,
    },

    #[experimental("accountSession/switch")]
    AccountSessionsSwitch => "accountSession/switch" {
        params: v2::AccountSessionsSwitchParams,
        serialization: global("account-auth"),
        manual_payload_conversion: manual,
        response: v2::AccountSessionsSwitchResponse,
    },
```

(CX1, codex-rs/app-server/src/message_processor.rs:1012-1020)

```rust
        if !session.initialized() {
            return Err(invalid_request("Not initialized"));
        }

        if let Some(reason) = codex_request.experimental_reason()
            && !session.experimental_api_enabled()
        {
            return Err(invalid_request(experimental_required_message(reason)));
        }
```

(CX1, codex-rs/app-server-protocol/src/protocol/common.rs:29-36)

```rust
    /// [UNSTABLE] FOR OPENAI INTERNAL USE ONLY - DO NOT USE.
    ///
    /// ChatGPT auth tokens are supplied by an external host app and are only
    /// stored in memory. Token refresh must be handled by the external host app.
    #[serde(rename = "chatgptAuthTokens")]
    #[ts(rename = "chatgptAuthTokens")]
    #[strum(serialize = "chatgptAuthTokens")]
    ChatgptAuthTokens,
```

"The experimental `account/read.workspaceRouting` response field returns the selected ChatGPT
workspace's `chatgptAccountId`, resolved HTTPS `backendOrigin`, and backend-provided
`accountRoutingOverride`." (CX1, codex-rs/app-server/README.md:560)

In this reading, the README’s experimental description is of that field. The handler quoted under R2
assembles it without checking an experimental capability, and the request declaration does not gate
the entire account/read method. The marking and the request admission rule are distinct source
facts. Searches for `experimental` and `unstable` covered app-server/README.md,
app-server/src/message_processor.rs, app-server-protocol/src/protocol/common.rs and
protocol/v2/account.rs; the relevant markings found are quoted here. No general stability guarantee
is inferred.

### R6. What it returns when the claim is absent

In this reading, absent email at both the top level and under profile remains None.
Account::Chatgpt.email is optional and schema-required nullable, so the account can be returned with
a null email. Missing plan in an otherwise parsed managed TokenData becomes
AccountPlanType::Unknown, rather than a parser error. If no token data exists, the plan accessor
returns None and the default provider raises MissingChatgptAccountDetails; the handler maps that to
an invalid-request error. A missing account ID skips routing; a present empty account ID is a
routing error, not an inferred workspace.

(CX1, codex-rs/login/src/token_data.rs:87-105)

```rust
struct IdClaims {
    #[serde(default)]
    email: Option<String>,
    #[serde(rename = "...", default)]
    profile: Option<ProfileClaims>,
    #[serde(rename = "...", default)]
    auth: Option<AuthClaims>,
}

#[derive(Deserialize)]
struct ProfileClaims {
    #[serde(default)]
    email: Option<String>,
}

#[derive(Deserialize)]
struct AuthClaims {
    #[serde(default)]
    chatgpt_plan_type: Option<PlanType>,
```

(CX1, codex-rs/login/src/token_data.rs:189-212)

```rust
pub fn parse_chatgpt_jwt_claims(jwt: &str) -> Result<IdTokenInfo, IdTokenInfoError> {
    let claims: IdClaims = decode_jwt_payload(jwt)?;
    let email = claims
        .email
        .or_else(|| claims.profile.and_then(|profile| profile.email));

    match claims.auth {
        Some(auth) => Ok(IdTokenInfo {
            email,
            raw_jwt: jwt.to_string(),
            chatgpt_plan_type: auth.chatgpt_plan_type,
            chatgpt_user_id: auth.chatgpt_user_id.or(auth.user_id),
            chatgpt_account_id: auth.chatgpt_account_id,
            chatgpt_account_is_fedramp: auth.chatgpt_account_is_fedramp,
        }),
        None => Ok(IdTokenInfo {
            email,
            raw_jwt: jwt.to_string(),
            chatgpt_plan_type: None,
            chatgpt_user_id: None,
            chatgpt_account_id: None,
            chatgpt_account_is_fedramp: false,
        }),
    }
```

(CX1, codex-rs/app-server-protocol/src/protocol/v2/account.rs:29-38)

```rust
    Chatgpt {
        #[schemars(
            required,
            schema_with = "crate::protocol::serde_helpers::nullable_string_schema"
        )]
        email: Option<String>,
        plan_type: PlanType,
        /// Display name of the selected ChatGPT workspace, when available.
        workspace_name: Option<String>,
    },
```

(CX1, codex-rs/login/src/auth/manager.rs:725-742)

```rust
    pub fn account_plan_type(&self) -> Option<AccountPlanType> {
        if matches!(self, Self::Headers(_)) {
            return None;
        }
        if let Self::AgentIdentity(auth) = self {
            return Some(auth.plan_type());
        }
        if let Self::PersonalAccessToken(auth) = self {
            return Some(auth.plan_type());
        }

        self.get_current_token_data().map(|t| {
            t.id_token
                .chatgpt_plan_type
                .map(AccountPlanType::from)
                .unwrap_or(AccountPlanType::Unknown)
        })
    }
```

(CX1, codex-rs/model-provider/src/provider.rs:718-724)

```rust
                            .map(|plan_type| ProviderAccount::Chatgpt { email, plan_type })
                            .ok_or(ProviderAccountError::MissingChatgptAccountDetails)
                    }
                })
                .transpose()?
        } else {
            None
```

(CX1, codex-rs/model-provider/src/provider.rs:74-80)

```rust
impl fmt::Display for ProviderAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingChatgptAccountDetails => {
                write!(f, "plan type is required for chatgpt authentication")
            }
            Self::UnsupportedBedrockApiKeyAuth => {
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:263-270)

```rust
        let read = self
            .read_account(/*request*/ None)
            .await
            .map_err(|error| match error {
                AccountReadError::InvalidAccount(error) => invalid_request(error.to_string()),
                AccountReadError::Routing(error) => internal_error(error.to_string()),
            })?;
        let mut account = read.account_state.account.map(Account::from);
```

(CX1, codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs:353-357)

```rust
                .and_then(|auth| auth.get_account_id().map(|account_id| (auth, account_id)))
            {
                if account_id.is_empty() {
                    return Err(WorkspaceRoutingError::MissingAccountId.into());
                }
```

In this reading, malformed JWT envelope, base64 or JSON causes IdTokenInfoError.
deserialize_id_token maps it into a serde error; file loading wraps JSON failure as a storage
deserialization error. At app-server startup new_from_auth_config swallows failed managed-auth
loading into absent auth. Thus a malformed persisted token does not itself become a successfully
parsed account with missing email; startup can expose no cached account. During refresh, parse
errors propagate from persist_tokens, and routing or later provider errors can instead prevent an
account response. The source does not define one unconditional wire outcome for every parse failure
at every point.

(CX1, codex-rs/login/src/token_data.rs:135-155)

```rust
pub enum IdTokenInfoError {
    #[error("invalid ID token format")]
    InvalidFormat,
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub(crate) fn decode_jwt_payload<T: DeserializeOwned>(jwt: &str) -> Result<T, IdTokenInfoError> {
    // JWT format: header.payload.signature
    let mut parts = jwt.split('.');
    let (_header_b64, payload_b64, _sig_b64) = match (parts.next(), parts.next(), parts.next()) {
        (Some(h), Some(p), Some(s)) if !h.is_empty() && !p.is_empty() && !s.is_empty() => (h, p, s),
        _ => return Err(IdTokenInfoError::InvalidFormat),
    };

    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload_b64)?;
    let claims = serde_json::from_slice(&payload_bytes)?;
    Ok(claims)
}
```

(CX1, codex-rs/login/src/token_data.rs:215-221)

```rust
fn deserialize_id_token<'de, D>(deserializer: D) -> Result<IdTokenInfo, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_chatgpt_jwt_claims(&s).map_err(serde::de::Error::custom)
}
```

(CX1, codex-rs/login/src/auth/storage.rs:250-254)

```rust
        file.read_to_string(&mut contents)?;
        let auth_dot_json: AuthDotJson = serde_json::from_str(&contents).map_err(|err| {
            storage_error::with_context("failed to deserialize CLI auth from file", err)
        })?;

```

(CX1, codex-rs/login/src/auth/manager.rs:2346-2350)

```rust
        let managed_auth = auth_config
            .load_auth_with_modes(enable_codex_api_key_env, allowed_auth_modes.as_deref())
            .await
            .ok()
            .flatten();
```

(CX1, codex-rs/login/src/auth/manager.rs:1758-1760)

```rust
    if let Some(id_token) = id_token {
        tokens.id_token = parse_chatgpt_jwt_claims(&id_token).map_err(std::io::Error::other)?;
    }
```

In this reading, external-token parsing fails with an io error when the supplied access token cannot
be parsed; if parsing succeeds without plan, that construction supplies an Unknown plan
classification. Its email stays optional. Selected-workspace parsing uses `if let Ok`, retaining the
original parsed ID-token data if access-token parsing fails. Both constructions are quoted under R3;
neither supplies an email value absent from the parsed claims.

### R7. Whether codex login status names an account

In this reading, `codex login status` is the fieldless Status login subcommand. The parent takes
configuration overrides, key/access-token login flags, device-auth and experimental issuer/client-ID
options; Status dispatch uses only the configuration overrides. No status argument requests an email
or account ID. The parent flags’ source is quoted below without their unrelated help examples.

(CX1, codex-rs/cli/src/main.rs:159-160)

```rust
    /// Manage login.
    Login(LoginCommand),
```

(CX1, codex-rs/cli/src/main.rs:517-519)

```rust
struct LoginCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,
```

(CX1, codex-rs/cli/src/main.rs:521-531)

```rust
    #[arg(
        long = "with-api-key",
        ...
    )]
    with_api_key: bool,

    #[arg(
        long = "with-access-token",
        ...
    )]
    with_access_token: bool,
```

(CX1, codex-rs/cli/src/main.rs:542-556)

```rust

    #[arg(long = "device-auth")]
    use_device_code: bool,

    /// EXPERIMENTAL: Use custom OAuth issuer base URL (advanced)
    /// Override the OAuth issuer base URL (advanced)
    #[arg(long = "experimental_issuer", value_name = "URL", hide = true)]
    issuer_base_url: Option<String>,

    /// EXPERIMENTAL: Use custom OAuth client ID (advanced)
    #[arg(long = "experimental_client-id", value_name = "CLIENT_ID", hide = true)]
    client_id: Option<String>,

    #[command(subcommand)]
    action: Option<LoginSubcommand>,
```

(CX1, codex-rs/cli/src/main.rs:559-563)

```rust
#[derive(Debug, clap::Subcommand)]
enum LoginSubcommand {
    /// Show login status.
    Status,
}
```

(CX1, codex-rs/cli/src/main.rs:1647-1654)

```rust
            prepend_config_flags(
                &mut login_cli.config_overrides,
                root_config_overrides.clone(),
            );
            match login_cli.action {
                Some(LoginSubcommand::Status) => {
                    run_login_status(login_cli.config_overrides).await;
                }
```

In this reading, the fixed successful ChatGPT/ChatgptAuthTokens status message names only the mode.
Workload identity, API key, AgentIdentity, PersonalAccessToken, BedrockApiKey and BedrockAccessKeys
have the messages below. None has a field for an email, account, user or workspace ID. API-key
status prints a formatted key and a notice; it is not an account identity. Error messages
interpolate errors; this read makes no claim that arbitrary error text cannot contain identity
information.

(CX1, codex-rs/cli/src/login.rs:488-554)

```rust
pub async fn run_login_status(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    if is_workload_identity_selected() {
        match AuthManager::shared_from_config(&config, /*enable_codex_api_key_env*/ false).await {
            Ok(_) => {
                eprintln!("Logged in using workload identity");
                ...
            Err(err) => {
                eprintln!("Error checking login status: {err}");
                ...
    let auth_config = config.auth_config();
    match auth_config
        .load_auth(/*enable_codex_api_key_env*/ false)
        .await
    {
        Ok(Some(auth)) => match auth.auth_mode() {
            AuthMode::ApiKey => match auth.get_token() {
                Ok(api_key) => {
                    eprintln!("API key configured - {}", safe_format_key(&api_key));
                    eprintln!("{API_KEY_AUTHENTICATION_NOTICE}");
                    ...
                Err(e) => {
                    eprintln!("Unexpected error retrieving API key: {e}");
                    ...
            AuthMode::Chatgpt | AuthMode::ChatgptAuthTokens => {
                eprintln!("Logged in using ChatGPT");
                ...
            AuthMode::Headers => {
                unreachable!("header auth cannot be loaded from auth storage")
            ...
            AuthMode::AgentIdentity => {
                eprintln!("Logged in using access token");
                ...
            AuthMode::PersonalAccessToken => {
                eprintln!("Logged in using personal access token");
                ...
            AuthMode::BedrockApiKey => {
                eprintln!("Logged in using Amazon Bedrock API key");
                ...
            AuthMode::BedrockAccessKeys => {
                eprintln!("Logged in using Amazon Bedrock AWS access keys");
                ...
        Ok(None) => {
            eprintln!("Not logged in");
            ...
        Err(err) => {
            eprintln!("Error checking login status: {err}");
            ...
```

In this reading, configuration/override errors can exit before status loads auth; their printing
lines and the key-format function are:

(CX1, codex-rs/cli/src/login.rs:673-703)

```rust
async fn load_config_or_exit(cli_config_overrides: CliConfigOverrides) -> Config {
    let cli_overrides = match cli_config_overrides.parse_overrides() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing -c overrides: {e}");
            std::process::exit(1);
        }
    };

    match Config::load_with_cli_overrides(cli_overrides).await {
        Ok(config) => match config.auth_config().validate() {
            Ok(()) => config,
            Err(e) => {
                eprintln!("Error loading configuration: {e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error loading configuration: {e}");
            std::process::exit(1);
        }
    }
}

fn safe_format_key(key: &str) -> String {
    if key.len() <= 13 {
        return "***".to_string();
    }
    let prefix = &key[..8];
    let suffix = &key[key.len() - 5..];
    format!("{prefix}***{suffix}")
```

In this reading, API_KEY_AUTHENTICATION_NOTICE is referenced by identifier above; its constant
definition is not quoted, under this order’s rule against quoting constant definitions. No key,
account value, login literal or client output was obtained.

## Not shown

- Anything observed: this read ran no client, so it shows nothing of what the installed binary does.

- Whether the token the service issues for a given account holds the claim: only stage 1’s first
  step shows that, for the account it signs in.

- What a library that Codex calls does inside.

- Claude Code.

- Codex on the Windows computer or under WSL, and any version of Codex but 0.167.1.

- The logins’ literal origins, paths, client IDs and ports. Order 050 reads those from the clients.

- What order 050 does with any of this, and whether it confirms ruling 82’s route, which is the
  coordinator’s reading.
