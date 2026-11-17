# 027 — A5 stage 0b, part A, 2026-11-16: Codex 0.167.1's browser login and status

Every result of this experiment is obtained under one operating-system user, so none shows Agents
kept apart (ruling 75 B3). Part A ran nothing. A Codex worker read Codex's public source at the tag
`rust-v0.167.1` (commit `0a94398fa60c`) and Codex's public documentation on 2026-11-16, under work
order 195, as [stage 0b](README.md#stage-0b) pre-registers. Each statement below is the source's or
the documentation's, quoted with where it is. None is an observation of a client, and source read at
a tag can differ from the binary installed from it. This file decides nothing and proposes nothing
for order 050. It records no URL of a login, no login literal and no account value ([what stage 0b
records](README.md#what-stage-0b-records)).

## Sources

The initial tag-page request returned HTTP 200 and discarded its body. The README's three
additions preceded the source clone and every documentation fetch. CX1 was cloned at the tag,
with depth one, and its commit was checked before any source was read. Each documentation page
was fetched once with `curl -sSL`, without cookies or browser headers. Each returned HTTP 200
after redirecting to the address below. A local Python standard-library `html.parser` helper
skipped scripts and styles and wrapped text to at most 200 characters per line. HTML was never
printed. No further documentation pages were fetched.

| ID | Source | Where | Read on | Its own date | Read |
| --- | --- | --- | --- | --- | --- |
| CX1 | Codex public source, tag `rust-v0.167.1` | https://github.com/openai/codex/tree/rust-v0.167.1 | 2026-11-16 | 2026-11-10, commit date | in part: the files cited |
| CD1 | Authentication | https://learn.chatgpt.com/docs/auth (redirected from https://developers.openai.com/codex/auth) | 2026-11-16 | not stated | in part: sign-in, authentication status, credential storage and headless-device sections; HTTP 200 |
| CD2 | Command-line reference | https://learn.chatgpt.com/docs/developer-commands?surface=cli (redirected from https://developers.openai.com/codex/cli/reference) | 2026-11-16 | not stated | in part: login and status entries, and MCP login for the scope of its browser flag; HTTP 200 |
| CD3 | Configuration reference | https://learn.chatgpt.com/docs/config-file/config-reference (redirected from https://developers.openai.com/codex/config-reference) | 2026-11-16 | not stated | in part: browser and authentication search matches, including login-method and workspace restrictions; HTTP 200 |

CX1's `docs/authentication.md:3` links CD1. The documentation pages do not state that they describe
this tag. No date was found in their extracted text or date metadata.

## What the sources say

### Q1. How the browser login opens a browser

The source defines the command, then dispatches a login with no selected alternative to the
browser-login function. These are excerpts of the definitions and dispatch, with cuts marked.

(CX1, codex-rs/cli/src/main.rs:159-160, 1641-1655, 1679-1682)

```rust
    /// Manage login.
    Login(LoginCommand),
...
        Some(Subcommand::Login(mut login_cli)) => {
            reject_remote_mode_for_subcommand(
                root_remote.as_deref(),
                root_remote_auth_token_env.as_deref(),
                "login",
            )?;
            prepend_config_flags(
                &mut login_cli.config_overrides,
                root_config_overrides.clone(),
            );
            match login_cli.action {
                Some(LoginSubcommand::Status) => {
                    run_login_status(login_cli.config_overrides).await;
                }
...
                    } else {
                        run_login_with_browser(login_cli.config_overrides).await;
                    }
```

There is a configuration-dependent branch before the ChatGPT login: the interactive wrapper
loads configuration and invokes that login when remote configuration is not required. Otherwise
it signs in to the configuration server and finishes that result. In this reading, this other
branch does not constitute a control that preserves the ChatGPT loopback login in Q2.

(CX1, codex-rs/cli/src/login.rs:208-229)

```rust
async fn run_interactive_login<F>(
    cli_config_overrides: CliConfigOverrides,
    chatgpt_login: impl FnOnce(Config) -> F,
) -> !
where
    F: std::future::Future<Output = std::io::Result<()>>,
{
    let mut config = load_config_or_exit(cli_config_overrides.clone()).await;
    if !config
        .config_layer_stack
        .requirements_toml()
        .requires_remote_config()
    {
        let _login_log_guard = init_login_file_logging(&config);
        finish_login(chatgpt_login(config).await);
    }

    let result =
        sign_in_to_config_server_and_reload_config(&mut config, &cli_config_overrides).await;
    let _login_log_guard = init_login_file_logging(&config);
    finish_login(result)
```

In summary of the lines below: when the requirements name a configuration server, `codex login`
with no argument runs that server's OAuth login rather than the ChatGPT login, and that path also
calls `webbrowser::open`. It runs that login only where the server's manager reports that a login
is required. The opener prints its authorization URL, cut below, then calls `webbrowser::open`
with no condition around the call. Between the quoted lines, `login_with_browser` passes the
opener to `authorize_with_browser` (`codex-rs/login/src/oauth_session_login.rs:319-322, 354`),
which calls it once it has built the authorization URL
(`codex-rs/login/src/oauth_session.rs:439-461`). A search of `codex-rs/cloud-config/src/` for the
terms of the search below, made when this file was revised after review, found no match.

(CX1, codex-rs/config/src/config_requirements.rs:1661-1663; codex-rs/cli/src/login.rs:187;
codex-rs/cloud-config/src/bundle_loader.rs:29-47;
codex-rs/login/src/oauth_session_login.rs:130-131, 411-416)

```rust
    pub fn requires_remote_config(&self) -> bool {
        self.config_server.is_some()
    }
...
    codex_cloud_config::ensure_config_server_login(bootstrap_auth, requirements).await?;
...
pub async fn ensure_config_server_login(
    auth: AuthConfig,
    requirements: &ConfigRequirementsToml,
) -> std::io::Result<()> {
    let Some(manager) = managed_config_auth_manager(auth, requirements)? else {
        return Ok(());
    };
    if let Err(error) = manager.authorization_id().await {
        if error
            .get_ref()
            .and_then(|source| source.downcast_ref::<codex_login::OAuthSessionError>())
            != Some(&codex_login::OAuthSessionError::LoginRequired)
        {
            return Err(error);
        }
        manager.login().await?;
    }
    Ok(())
}
...
    pub async fn login(&self) -> io::Result<()> {
        self.login_with_browser(std::future::pending(), open_browser)
...
pub(super) fn open_browser(url: &Url) {
    eprintln!("Authorize the model provider by opening this URL:...");
    if webbrowser::open(url.as_str()).is_err() {
        eprintln!("Browser launch failed; open the URL above manually.");
    }
}
```

The browser-login function checks whether ChatGPT login is allowed. Disallowing it exits; it does
not leave this login printing its URL and waiting. The allowed path calls `login_with_chatgpt`.

(CX1, codex-rs/cli/src/login.rs:232-254)

```rust
pub async fn run_login_with_browser(cli_config_overrides: CliConfigOverrides) -> ! {
    run_interactive_login(cli_config_overrides, |config| async move {
        tracing::info!("starting browser login flow");

        if !config
            .auth_config()
            .is_login_method_allowed(ForcedLoginMethod::Chatgpt)
        {
            eprintln!("{CHATGPT_LOGIN_DISABLED_MESSAGE}");
            std::process::exit(1);
        }

        let effective_chatgpt_workspaces = config.auth_config().effective_chatgpt_workspaces();
        login_with_chatgpt(
            config.codex_home.to_path_buf(),
            effective_chatgpt_workspaces,
            config.cli_auth_credentials_store_mode,
            config.auth_keyring_backend_kind(),
            config.auth_route_config(),
        )
        .await
    })
```

That function constructs options without changing their browser field, starts the login server,
prints the URL and awaits completion. Its client-ID argument is the identifier `CLIENT_ID`.

(CX1, codex-rs/cli/src/login.rs:160-172)

```rust
    let opts = ServerOptions::new(
        codex_home,
        CLIENT_ID.to_string(),
        forced_chatgpt_workspace_id,
        cli_auth_credentials_store_mode,
        auth_keyring_backend_kind,
        auth_route_config,
    );
    let server = run_login_server(opts)?;

    print_login_server_start(server.actual_port, &server.auth_url);

    server.block_until_done().await
```

In summary of the lines below: the constructor sets the browser field to true; the server binds
and builds its authorization URL before reaching the condition below; on that successful path the
browser call is attempted, with no terminal, headless-environment or browser-variable condition
around it; and the return value of the browser call is discarded.

(CX1, codex-rs/login/src/server.rs:118-120, 185, 208-210)

```rust
            issuer: DEFAULT_ISSUER.to_string(),
            port: DEFAULT_PORT,
            open_browser: true,
...
    let server = bind_server(opts.port)?;
...
    if opts.open_browser {
        let _ = webbrowser::open(&auth_url);
    }
```

(CX1, codex-rs/Cargo.lock:16996-16997)

```toml
name = "webbrowser"
version = "1.2.4"
```

Thus the source shows a call to `webbrowser` 1.2.4. What that library does inside was not read;
this reading cannot say how it reaches the operating system or why the installed client passed
stage 0's shim. It does not show any variable that only the library reads.

Not found on this command path: Codex code reading `BROWSER`, executing its value, or directly
running `open` or `xdg-open`, by a name on `PATH` or a full path. Searches covered
`codex-rs/cli/src/`, `codex-rs/login/src/`, `codex-rs/core/src/config/`, `codex-rs/config/` and
`docs/`, for `BROWSER`, `webbrowser`, `open_browser`, `no.browser`, `Command::new`,
`std::process::Command`, `process::Command`, `open(` and `xdg-open`, including a pattern for
`Command::new` with either a bare program name or a path ending in that name. Matches for
launching the desktop app with `open` (`codex-rs/cli/src/desktop_app/mac.rs:109`) and for MCP
login (`codex-rs/cli/src/mcp_cmd.rs:219, 617`) are outside this command path. The
`webbrowser::open` match at `codex-rs/login/src/oauth_session_login.rs:413`, with the opener's
other matches in that file and at `codex-rs/login/src/oauth_session.rs:461`, is on the
configuration-server branch above. The same manager's token-resolution functions also pass that
opener (`codex-rs/login/src/oauth_session.rs:186-290`;
`codex-rs/login/src/oauth_session_request.rs:47`); whether that branch's reload of configuration
reaches them was not followed.

The documentation says: "When you sign in with ChatGPT from the ChatGPT desktop app, Codex CLI,
or IDE extension, the sign-in flow opens a browser window. After you sign in, the browser returns
your credentials to Codex." (CD1)

"Run codex login, then complete the browser flow. This is the default authentication path when
no valid session is available." (CD1)

"Authenticate the CLI with a ChatGPT account, API key, or access token. With no flags, Codex
opens a browser for the ChatGPT OAuth flow." (CD2)

Not found in CD1 to CD3: a statement of which operating-system mechanism opens that browser.
Searches of their extracted text used `browser`, `BROWSER`, `open_browser`, `headless`,
`no.browser`, `remote` and `login status`.

### Q2. What stops it while the URL is still printed

Not found: an argument, configuration setting or environment variable read by Codex on the
browser-login path that suppresses the browser while preserving the printed authorization URL
and the wait for its loopback redirect. The constructor and call in Q1 show why: this path
creates `open_browser: true` and passes the options on without changing that field.

The source's login-argument definition was read in full at
`codex-rs/cli/src/main.rs:517-557`. The arguments `--device-auth`, `--with-api-key` and
`--with-access-token` switch login; they were not examined further. The deprecated `--api-key`
exits with guidance rather than starting this login (`codex-rs/cli/src/main.rs:1668-1672`).
The hidden `--experimental_issuer` and `--experimental_client-id` fields are forwarded only in
the alternative-login branch, not to the default browser call
(`codex-rs/cli/src/main.rs:543-553, 1661-1667, 1679-1680`).

The configuration override is global in the argument definition, so its definition permits it
after `login` as well as before it. No configuration key read on this path was found to change
`open_browser`. This describes the argument definition; no parser or client was run.

(CX1, codex-rs/cli/src/main.rs:128-129, 517-519;
codex-rs/utils/cli/src/config_override.rs:29-36)

```rust
    #[clap(flatten)]
    pub config_overrides: CliConfigOverrides,
...
struct LoginCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,
...
    #[arg(
        short = 'c',
        long = "config",
        value_name = "key=value",
        action = ArgAction::Append,
        global = true,
    )]
    pub raw_overrides: Vec<String>,
```

Searches for `open_browser`, `no.browser`, `headless`, `BROWSER`, `SSH_CONNECTION`, `SSH_TTY`,
`DISPLAY` and `WAYLAND_DISPLAY` covered `codex-rs/cli/src/`, `codex-rs/login/src/`,
`codex-rs/core/src/config/`, `codex-rs/config/` and `docs/`. Searches for `std::env`, `env::var`
and `env::var_os` also covered the CLI and login source. An assignment of false to `open_browser`
was found in `run_login_with_device_code_fallback_to_browser`
(`codex-rs/cli/src/login.rs:415, 451`), outside the default branch: the `login` dispatch
(`codex-rs/cli/src/main.rs:1655-1681`) does not call it, and a search of the `.rs` files under
`codex-rs/` for its name, made when this file was revised after review, found only it and its
re-export (`codex-rs/cli/src/lib.rs:23`). That route was not examined further. The MCP
`no_browser` flag (`codex-rs/cli/src/mcp_cmd.rs:219, 617`) is also outside this browser login.
No headless-environment test was found on the path traced in Q1.

The URL-printing line follows the server call irrespective of that call's discarded browser
result. Below, the URL and the rest of the long string are cut.

(CX1, codex-rs/cli/src/login.rs:121-125, excerpt of line 123)

```rust
fn print_login_server_start(actual_port: u16, auth_url: &str) {
    eprintln!(
        "Starting local login server on ...
...If your browser did not open, navigate to this URL to authenticate:\n\n...\n\n...
```

The await in `codex-rs/cli/src/login.rs:172` is Q1's quoted line. It waits for the callback
server task; the server's request loop also has an explicit shutdown branch.

(CX1, codex-rs/login/src/server.rs:141-153, 236-244)

```rust
impl LoginServer {
    /// Waits for the login callback loop to finish.
    pub async fn block_until_done(self) -> io::Result<()> {
        self.block_until_done_with_callback_result()
            .await
            .map(|_| ())
    }
...
    pub async fn block_until_done_with_callback_result(self) -> io::Result<LoginCallbackResult> {
        self.server_handle
            .await
...
            let result = loop {
                tokio::select! {
                    _ = shutdown_notify.notified() => {
                        break Err(io::Error::other("Login was not completed"));
                    }
                    maybe_req = rx.recv() => {
                        let Some(req) = maybe_req else {
                            break Err(io::Error::other("Login was not completed"));
                        };
```

The documentation's headless section says: "If you are signing in to ChatGPT with the Codex CLI,
there are some situations where the browser-based login UI may not work:" followed by "You’re
running the CLI in a remote or headless environment." and "Your local networking configuration
blocks the localhost callback Codex uses to return the OAuth token to the CLI after you sign in."
(CD1)

"In these situations, prefer device code authentication (beta). In the interactive login UI,
choose Sign in with Device Code, or run codex login --device-auth directly. If device code
authentication doesn’t work in your environment, use one of the fallback methods." (CD1)

For that alternative, CD2 says: "Use OAuth device code flow instead of launching a browser
window." (CD2) In this reading, that is a switch of login, not a control preserving this
loopback login.

CD1 also describes two fallbacks. "If you can complete the login flow on a machine with a
browser, you can copy your cached credentials to the headless machine." (CD1) Its other fallback
says: "If you can forward ports between your local machine and the remote host, you can use the
standard browser-based flow by tunneling Codex’s local callback server ..." and "In that SSH
session, run codex login and follow the printed address on your local machine." (CD1)

CD2's MCP entry says: "Sign in to a streamable HTTP server that supports OAuth. With
--no-browser, open the printed URL in a browser and paste the full redirect URL into the CLI."
(CD2) That entry is for `codex mcp login`, not the command traced here.

CD3 describes `forced_login_method` as "Restrict Codex to a specific authentication method."
It describes `file_opener` as "URI scheme used to open citations from Codex output (default:
vscode)." (CD3) In this reading, neither description supplies a browser-login suppression
control.

Not found in CD1 to CD3: a control that suppresses the default login's browser while keeping its
printed authorization URL and loopback wait. Searches used `browser`, `headless`, `remote`,
`no.browser`, `open_browser`, `BROWSER`, and browser text with `disable`, `prevent` or `suppress`.
Their descriptions of the default browser flow agree with the source; they do not describe the
library's internal behavior.

### Q3. Whether the status can name the account

The source defines status as a unit subcommand, without its own arguments. The shared global
configuration override is quoted in Q2; dispatch passes only those overrides into status, as
quoted in Q1. There is no status-specific account or JSON flag in this definition.

(CX1, codex-rs/cli/src/main.rs:552-563)

```rust
    #[command(subcommand)]
    action: Option<LoginSubcommand>,
}

#[derive(Debug, clap::Subcommand)]
enum LoginSubcommand {
    /// Show login status.
    Status,
}
```

The following excerpts include every output template in the status function, with its branch.
The function first loads configuration; this reading stops at the status code and follows
neither login-file reading nor ID-token reading. No credential or account value was read.

(CX1, codex-rs/cli/src/login.rs:488-553)

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
```

Configuration-load failure messages, before the status branches, are also templates with error
interpolation; the same loading function is used by browser login.

(CX1, codex-rs/cli/src/login.rs:672-692, excerpts)

```rust
async fn load_config_or_exit(cli_config_overrides: CliConfigOverrides) -> Config {
...
            eprintln!("Error parsing -c overrides: {e}");
...
                eprintln!("Error loading configuration: {e}");
...
            eprintln!("Error loading configuration: {e}");
```

In summary of these source lines, each successful status names an authentication mode, not an
account. None has a field for an email address or account, user or workspace ID. The API-key
branch interpolates `safe_format_key(&api_key)`, not an account; no value is quoted here. Error
branches interpolate errors, whose contents this reading does not establish. Not found: a
status argument, setting or login mode in these definitions that reports the signed-in account.
Searches used `Status`, `account`, `email`, `workspace`, `json`, `eprintln!` and `println!` in
`codex-rs/cli/src/main.rs` and `codex-rs/cli/src/login.rs`, followed by reading the complete
login-argument definition and status function. No login-file or ID-token implementation was
followed.

The documentation says: "Run codex login status to see the active authentication method."
(CD1) "Print the active authentication mode and exit with 0 when logged in." (CD2)
"codex login status exits with 0 when credentials are present, which is helpful in automation
scripts." (CD2)

Not found in CD1 to CD3: a statement that CLI login status names an account, or a flag that makes
it do so. Searches used `login status`, `account`, `email`, `workspace` and `json`; the account
and workspace descriptions in CD1's profile-menu entries refer to other surfaces. The CLI
status descriptions agree with the source's mode-only successful messages.

## For part B

None: part A found no control, so part B does not run.

## Not shown

- Anything observed: part A ran no client, so it shows nothing of what the installed binary
  does.
- What a library that Codex calls does inside, and any variable that only it reads.
- Claude Code.
- Codex on the Windows computer or under WSL, observed, and any version of Codex but 0.167.1.
- The device login, a completed sign-in, the account check, and the ID-token route.
- The logins' literal origins, paths, client IDs and ports. Order 050 reads those from the
  clients.
- What order 050 does with any of this.
