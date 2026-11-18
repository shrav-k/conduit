# 027 — A5, before stage 1's first step, 2026-11-17: what claude auth logout and codex logout revoke

Every result of this experiment is obtained under one operating-system user, so none shows Agents
kept apart (ruling 75 B3). This read ran nothing. A Codex worker read Claude Code's public
documentation, Codex's public source at the tag `rust-v0.167.1` (commit `0a94398fa60c`) with the
documentation it holds, and OpenAI's pages on Codex that Sources lists, on 2026-11-17, under work
order 050g, for item 8 of the targeted review's lists for `claude-code.v2` and `codex.v2`
([section 6](../../docs/proposals/review-signin-2026-11-16/README.md#6-must-state-lists)). Each
statement below is a page's or the source's, quoted with where it is. None is an observation of
a client: a page can differ from the client it documents, and source read at a tag can differ
from the binary installed from it. Beyond its two outcome lines, this file decides nothing and
proposes nothing for order 050. It records no URL of a login, no login literal, no claim's value
and no account value.

## Sources

The two preliminary status-only requests returned HTTP 200. CX1 was cloned at the tag alone with
depth 1; rev-parse matched the required commit, and git log supplied its date. No fetched code was
built or run. Each of CD1 to CD3 and CC1 to CC3 was fetched once with curl -sSL, without cookies or
browser headers. Each returned HTTP 200. No further page was fetched.

A short Python standard-library HTMLParser helper, run with python3 -I, skipped script and style and
wrapped text to 200 characters. Pages, text and helpers stayed in the ignored target/wo050g
directory. The login, logout and credential-storage portions were read with grep -n and sed -n, in
portions of about 80 lines or fewer. Repository files were read from the working tree root. Page
citations identify their section; source citations give lines at CX1. Ellipses mark cuts, including
barred literals. Claude page dates below are their lastModified metadata; Codex page dates were not
stated in the portions read.

| ID | Source | Where | Read on | Its own date | Read |
| --- | --- | --- | --- | --- | --- |
| CX1 | Codex source and documentation, tag `rust-v0.167.1`, commit `0a94398fa60c` | https://github.com/openai/codex/tree/rust-v0.167.1 | 2026-11-17 | 2026-11-10 | in part: the files cited; no file on C3's path could not be fetched |
| CD1 | Authentication | https://learn.chatgpt.com/docs/auth (named https://developers.openai.com/codex/auth) | 2026-11-17 | not stated | in part: authentication, sign-out, login caching and credential storage; HTTP 200 |
| CD2 | Command-line reference | https://learn.chatgpt.com/docs/developer-commands?surface=cli (named https://developers.openai.com/codex/cli/reference) | 2026-11-17 | not stated | in part: login, logout, common flags and command summaries; HTTP 200 |
| CD3 | Configuration reference | https://learn.chatgpt.com/docs/config-file/config-reference (named https://developers.openai.com/codex/config-reference) | 2026-11-17 | not stated | in part: credential-store setting and search matches on authentication; HTTP 200 |
| CC1 | Authentication | https://code.claude.com/docs/en/authentication | 2026-11-17 | 2026-11-17 | in part: account types, login and logout, keyless Console sign-in, credential management and profiles; HTTP 200 |
| CC2 | CLI reference | https://code.claude.com/docs/en/cli-reference | 2026-11-17 | 2026-11-16 | in part: authentication commands and authentication-related search matches; HTTP 200 |
| CC3 | Identity and access management; served content titled Authentication | https://code.claude.com/docs/en/iam | 2026-11-17 | 2026-11-17 | in part: the same authentication, logout, storage and profile portions as CC1; HTTP 200 |

## The outcome under item 8

`claude auth logout`: pass, a revocation scoped to that home's token alone, quoted under L2.

`codex logout`: pass, a revocation scoped to that home's token alone, quoted under C3.

## What the sources say

### L1. What claude auth logout does

“Log out from your Anthropic account” (CC2, CLI commands, claude auth logout row)

In this reading, that row gives claude auth logout as both the command and its example, with no
arguments listed. It states no store removal. Under decision 2, its account wording alone states no
vendor revocation or scope.

“To log out and re-authenticate, type /logout at the Claude Code prompt. Logging out also resets
your first-launch setup state, so the next time you run claude it walks you through login and setup
again.” (CC1 and CC3, authentication account types)

“How to undo it: run /logout, which removes and revokes the credential this sign-in wrote” (CC1 and
CC3, Sign in without an API key)

“For a keyless sign-in, they run claude auth logout in their shell.” (CC1 and CC3, Stop offering the
keyless sign-in on managed machines)

“Claude Code manages .credentials.json through /login and /logout.” (CC1 and CC3, Credential
management)

In this reading, those pages connect the shell command and /logout for the keyless Console sign-in.
They say removal of that sign-in's credential and management of .credentials.json, but do not
enumerate every file or macOS Keychain item deleted by the command. Their named storage locations
are quoted under L3.

### L2. Whether it revokes anything, and the scope

“Sign in with your Console account, labeled (recommended): Claude Code keeps the OAuth token from
that sign-in and stores it as an Anthropic profile. It creates no API key” (CC1 and CC3, Sign in
without an API key)

“How to undo it: run /logout, which removes and revokes the credential this sign-in wrote” (CC1 and
CC3, Sign in without an API key)

“For a keyless sign-in, they run claude auth logout in their shell.” (CC1 and CC3, Stop offering the
keyless sign-in on managed machines)

In this reading, the explicit revocation is of the OAuth credential from this sign-in, not an
inference from the account wording in CC2. Under decision 2's rule that a stated scope of that
sign-in alone counts as that home's token, this supplies the scoped outcome. The statement concerns
the keyless Console sign-in; it does not describe revocation for each other authentication type. The
pages give no request implementation or request payload. In this reading, the credential that
statement names is the keyless Console sign-in's profile, which the pages quoted under L3 place
outside the configuration directory and say separate directories do not keep apart; for a claude.ai
login the pages read state no revocation.

Not found: a statement that this logout revokes every login, every device, an account-wide
credential set, or anything beyond the credential this sign-in wrote; or a statement that no
revocation is made for that sign-in. Searches in CC1, CC2 and CC3: logout, log out, sign out,
revoke, revocation, invalidate, session, token, another home, other home, every device, all devices,
every login and all logins. CC1 and CC3 give the same revocation wording. CC2's shorter account
wording is silent on revocation under decision 2 and does not contradict it.

### L3. One login for each home

“To stay signed in to multiple accounts at once, such as work and personal accounts, give each
account its own configuration directory. When you start claude, set the CLAUDE_CONFIG_DIR
environment variable to the directory for the account you want to use. Each directory has its own
settings, session history, and claude.ai login or API key.” (CC1 and CC3, Log in with multiple
accounts)

“Separate directories don’t keep two Claude Console sign-ins without an API key apart, because
Claude Code stores that kind of sign-in outside the configuration directory.” (CC1 and CC3, Log in
with multiple accounts)

“On macOS, credentials are stored in the encrypted macOS Keychain.” (CC1 and CC3, Credential
management)

“When the Keychain rejects the write, such as when it’s locked in an SSH session, Claude Code stores
your login in ... with file mode 0600 instead, the same storage it uses on Linux. A Console login
that creates an API key fails until the Keychain is writable.” (CC1 and CC3, Credential management)

“On Linux, credentials are stored in ... with file mode 0600.” (CC1 and CC3, Credential management)

“On Windows, credentials are stored in ... and inherit the access controls of your user profile
directory, which restricts the file to your user account by default.” (CC1 and CC3, Credential
management)

“If you’ve set the CLAUDE_CONFIG_DIR environment variable, Claude Code keeps the .credentials.json
file under that directory instead, including the file the macOS fallback writes, and keys the macOS
Keychain entry to that directory too, so a session with a different CLAUDE_CONFIG_DIR reads a
different entry.” (CC1 and CC3, Credential management)

“A profile is a named credential configuration file in your Anthropic configuration directory, by
default ... A profile’s auth mode is oidc_federation when you set it up for Workload Identity
Federation (WIF) or user_oauth when ant auth login wrote it or you signed in to a Console account
without an API key.” (CC1 and CC3, Anthropic profiles and federation credentials)

“Which profile it writes: Claude Code writes the profile named by ANTHROPIC_PROFILE, or your active
profile, or default. If that profile is a federation profile, Claude Code refuses the sign-in
instead of overwriting it” (CC1 and CC3, Sign in without an API key)

“What it signs you out of: Claude Code signs you out of any claude.ai login stored on the machine”
(CC1 and CC3, Sign in without an API key)

“Active profile: run /logout for a user_oauth profile whose current credential you wrote by signing
in to a Console account without an API key, run ant auth logout for one whose current credential ant
auth login wrote, or delete the profile’s file from configs/ in your configuration directory for
either auth mode” (CC1 and CC3, Anthropic profiles and federation credentials)

In this reading, the directory-scoped .credentials.json and Keychain statement does not say that the
separately described Anthropic profiles are scoped by CLAUDE_CONFIG_DIR. The multiple-accounts
section explicitly permits simultaneous sign-ins in separate directories and places keyless Console
sign-ins outside those directories. The machine sign-out sentence describes a new sign-in, not a
logout request or the scope of revocation.

Not found: an explicit statement that logout in one CLAUDE_CONFIG_DIR home touches or leaves another
home's login untouched, including when both select a profile, or whether two homes select the same
Anthropic profile. Searches in CC1, CC2 and CC3: logout, log out, sign out, revoke,
revocation, invalidate, session, token, CLAUDE_CONFIG_DIR, profile, another home, other home,
simultaneous and multiple.

### C1. How codex logout is reached

(CX1, codex-rs/cli/src/main.rs:159-163; codex-rs/cli/src/main.rs:566-569;
codex-rs/cli/src/main.rs:1685-1695)

```rust
    /// Manage login.
    Login(LoginCommand),

    /// Remove stored authentication credentials.
    Logout(LogoutCommand),
...
struct LogoutCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,
}
...
        Some(Subcommand::Logout(mut logout_cli)) => {
            reject_remote_mode_for_subcommand(
                root_remote.as_deref(),
                root_remote_auth_token_env.as_deref(),
                "logout",
            )?;
            prepend_config_flags(
                &mut logout_cli.config_overrides,
                root_config_overrides.clone(),
            );
            run_logout(logout_cli.config_overrides).await;
```

(CX1, codex-rs/utils/cli/src/config_override.rs:29-35)

```rust
    #[arg(
        short = 'c',
        long = "config",
        value_name = "key=value",
        action = ArgAction::Append,
        global = true,
    )]
```

In this reading, logout has no subcommand-specific options. The root config_overrides is flattened
at codex-rs/cli/src/main.rs:129-130. Login also has a skipped config_overrides field at
main.rs:517-520. Both receive root overrides; logout passes them to run_logout. Thus it accepts the
global -c configuration overrides that login accepts. The CLI bootstrap before subcommand dispatch
is already quoted in [order 197's
R4](results-2026-11-17-account-read-source.md#r4-what-answering-reads-writes-refreshes-or-calls); it
is not quoted again here.

(CX1, codex-rs/cli/src/login.rs:566-568; codex-rs/cli/src/login.rs:610-616;
codex-rs/cli/src/login.rs:674-679; codex-rs/cli/src/login.rs:682-684)

```rust
pub async fn run_logout(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    let auth_route_config = config.auth_route_config();
...
    let logged_out = match logout_with_revoke(
        &config.codex_home,
        config.cli_auth_credentials_store_mode,
        config.auth_keyring_backend_kind(),
        &auth_route_config,
    )
    .await
...
    let cli_overrides = match cli_config_overrides.parse_overrides() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing -c overrides: {e}");
            std::process::exit(1);
        }
...
    match Config::load_with_cli_overrides(cli_overrides).await {
        Ok(config) => match config.auth_config().validate() {
            Ok(()) => config,
```

(CX1, codex-rs/core/src/config/mod.rs:2077-2083; codex-rs/core/src/config/mod.rs:1589-1592;
codex-rs/core/src/config/mod.rs:4584-4590)

```rust
    pub async fn load_with_cli_overrides(
        cli_overrides: Vec<(String, TomlValue)>,
    ) -> std::io::Result<Self> {
        ConfigBuilder::default()
            .cli_overrides(cli_overrides)
            .build()
            .await
...
        let codex_home = match codex_home {
            Some(codex_home) => AbsolutePathBuf::from_absolute_path(codex_home)?,
            None => find_codex_home()?,
        };
...
            cli_auth_credentials_store_mode: match cli_auth_credentials_store {
                Some(required) => required.value,
                None => resolve_cli_auth_credentials_store_mode(
                    cfg.cli_auth_credentials_store.unwrap_or_default(),
                    env!("CARGO_PKG_VERSION"),
                ),
            },
```

In this reading, find_codex_home delegates to codex_utils_home_dir::find_codex_home at
codex-rs/core/src/config/mod.rs:5247-5248. Its implementation reads CODEX_HOME, requires an existing
directory and canonicalizes it; without it, dirs::home_dir supplies the operating-system home and
the client's default directory is appended (codex-rs/utils/home-dir/src/lib.rs:13-60). The default
credential-store mode is File (codex-rs/config/src/types.rs:138-147); an explicit required value
takes precedence in the configuration construction quoted above.
resolve_cli_auth_credentials_store_mode preserves the configured mode except for the local
development build case (core/src/config/mod.rs:288-299).

(CX1, codex-rs/core/src/config/auth_keyring.rs:14-17;
codex-rs/core/src/config/auth_keyring.rs:113-118)

```rust
    pub fn auth_keyring_backend_kind(&self) -> AuthKeyringBackendKind {
        auth_keyring_backend_kind_from_secret_auth_storage(
            self.features.enabled(Feature::SecretAuthStorage),
        )
...
    if secret_auth_storage_enabled {
        AuthKeyringBackendKind::Secrets
    } else {
        AuthKeyringBackendKind::Direct
    }
}
```

In this reading, Feature::SecretAuthStorage selects Secrets instead of Direct for keyring-backed
storage. These are further backend choices within keyring and auto, not extra values of
cli_auth_credentials_store.

In this reading, the free function logout_with_revoke is defined at
codex-rs/login/src/auth/manager.rs:1062-1087. Its other production caller is
codex-rs/cli/src/login.rs:133, in clear_existing_auth_before_login. Its direct test callers are
codex-rs/login/tests/suite/logout.rs:60,119,162. The AuthManager method of the same name is defined
at manager.rs:3482-3505 and called by
codex-rs/app-server/src/request_processors/account_processor.rs:1608 and
login/tests/suite/logout.rs:220. These other callers are named, not followed. The full-tree search
was logout_with_revoke( under CX1's codex-rs, restricted to Rust files.

### C2. What it removes in the home and beyond it

(CX1, codex-rs/login/src/auth/manager.rs:1054-1059; codex-rs/login/src/auth/manager.rs:1551-1569)

```rust
        codex_home.to_path_buf(),
        auth_credentials_store_mode,
        keyring_backend_kind,
    );
    let _commit = storage.lock_for_write()?;
    storage.delete()
...
    if auth_credentials_store_mode == AuthCredentialsStoreMode::Ephemeral {
        return logout(
            codex_home,
            AuthCredentialsStoreMode::Ephemeral,
            AuthKeyringBackendKind::default(),
        );
    }
    let removed_ephemeral = logout(
        codex_home,
        AuthCredentialsStoreMode::Ephemeral,
        AuthKeyringBackendKind::default(),
    )?;
    let removed_managed = logout(
        codex_home,
        auth_credentials_store_mode,
        keyring_backend_kind,
    )?;
    Ok(removed_ephemeral || removed_managed)
}
```

In this reading, logout_with_revoke loads the selected store, attempts the request in C3, then calls
logout_all_stores. For file, keyring and auto, that removes the home-keyed ephemeral entry and then
the selected persistent store; ephemeral removes only the process-memory entry. An error in
ephemeral removal stops persistent removal. The operations below concern local removal, separately
from vendor revocation.

(CX1, codex-rs/login/src/auth/storage.rs:715-728; codex-rs/login/src/auth/storage.rs:171-175;
codex-rs/login/src/auth/storage.rs:297-298; codex-rs/login/src/auth/storage.rs:696-697)

```rust
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
...
pub(super) fn delete_file_if_exists(codex_home: &Path) -> std::io::Result<bool> {
    let auth_file = get_auth_file(codex_home);
    match std::fs::remove_file(&auth_file) {
        Ok(()) => Ok(true),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
...
    fn delete(&self) -> std::io::Result<bool> {
        delete_file_if_exists(&self.codex_home)
...
        self.with_store(|store, key| Ok(store.remove(&key).is_some()))
    }
```

In this reading, file removes the home's auth.json and does not delete a keyring item. The
missing-home File case returns false before creating a directory (manager.rs:1050-1051). For
persistent stores, locking can create auth.lock and a separate commit lock, retained on disk:
storage.rs:195-210 and storage_commit.rs:18-35. Ephemeral changes only the process-global map entry
indexed by compute_store_key, with no persistent auth file or OS credential deletion
(storage.rs:671-679,696-697).

(CX1, codex-rs/login/src/auth/storage.rs:310-320; codex-rs/login/src/auth/storage.rs:403-412;
codex-rs/login/src/auth/storage.rs:645-652)

```rust
fn compute_store_key(codex_home: &Path) -> std::io::Result<String> {
    let canonical = codex_home
        .canonicalize()
        .unwrap_or_else(|_| codex_home.to_path_buf());
    let path_str = canonical.to_string_lossy();
    let mut hasher = Sha256::new();
    hasher.update(path_str.as_bytes());
    let digest = hasher.finalize();
    let hex = format!("{digest:x}");
    let truncated = hex.get(..16).unwrap_or(&hex);
    Ok(format!("cli|{truncated}"))
...
        let key = compute_store_key(&self.codex_home)?;
        let keyring_removed = self
            .keyring_store
            .delete(KEYRING_SERVICE, &key)
            .map_err(std::io::Error::from)
            .map_err(|error| {
                storage_error::with_context("failed to delete auth from keyring", error)
            })?;
        let file_removed = delete_file_if_exists(&self.codex_home)?;
        Ok(keyring_removed || file_removed)
...
        // Keyring storage will delete from disk as well
        let mut telemetry = storage_telemetry::telemetry(
            AuthCredentialsStoreMode::Auto,
            self.keyring_backend_kind,
            Operation::Delete,
        );
        let result = self.keyring_storage.delete();
        telemetry.record_delete_attempt(Store::Multiple, &result);
```

In this reading, keyring with Direct deletes the item whose service identifier is KEYRING_SERVICE
(definition at storage.rs:307) and whose account key is cli| followed by the first 16 hexadecimal
SHA-256 characters of the canonical home path, falling back to its supplied path. It then removes
the home's auth.json. Auto delegates deletion to its keyring backend, including that file cleanup;
unlike its load fallback, it does not ignore keyring deletion errors (storage.rs:644-653). A keyring
error can therefore leave the file in place.

(CX1, codex-rs/login/src/auth/storage.rs:440-447; codex-rs/login/src/auth/storage.rs:509-520)

```rust
        let secrets_manager = SecretsManager::new_with_keyring_store_and_namespace(
            codex_home.clone(),
            SecretsBackendKind::Local,
            keyring_store,
            LocalSecretsNamespace::CodexAuth,
        );
        Self {
            codex_home,
...
        let keyring_removed = self
            .secrets_manager
            .delete(&SecretScope::Global, &CODEX_AUTH_SECRET_NAME)
            .map_err(|error| {
                storage_error::with_context(
                    "failed to delete auth from encrypted auth storage",
                    error,
                )
            })?;
        let file_removed = delete_file_if_exists(&self.codex_home)?;
        let direct_removed = self.direct_storage.delete()?;
        Ok(keyring_removed || file_removed || direct_removed)
```

(CX1, codex-rs/secrets/src/local.rs:144-150; codex-rs/secrets/src/local.rs:255-266)

```rust
    pub fn delete(&self, scope: &SecretScope, name: &SecretName) -> Result<bool> {
        let canonical_key = scope.canonical_key(name);
        let mut file = self.load_file()?;
        let removed = file.secrets.remove(&canonical_key).is_some();
        if removed {
            self.save_file(&file)?;
        }
...
        let path = self.secrets_path();
        write_file_atomically(&path, &ciphertext)?;
        let mut cache = self
            .decrypted_cache()
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if cache.as_ref().is_some_and(|cached| cached.path == path) {
            *cache = None;
        }
        Ok(())
    }

```

In this reading, keyring with Secrets removes CODEX_AUTH_SECRET_NAME in SecretScope::Global from the
LocalSecretsNamespace::CodexAuth encrypted file, rewrites that file if an entry was removed, removes
the home's auth.json, and then deletes the Direct item too. The encrypted file is
secrets/codex_auth.age under the home, selected by CODEX_AUTH_SECRETS_FILENAME
(secrets/src/local.rs:42,172-184). The passphrase keyring item is read and may be created by
save_file's load_or_create_passphrase; it is not deleted here (local.rs:265,271-292). Its service
identifier is KEYRING_SERVICE in secrets/src/lib.rs:23,206-208, and its account key is secrets| plus
the first 16 hexadecimal SHA-256 characters of the canonical home path (lib.rs:184-203). That key is
shared with ManagedSecrets and McpOAuth within the same home, not derived from an account. A storage
error can stop later cleanup steps.

In this reading, path-derived keys separate different canonical home paths for file, Direct, Secrets
and ephemeral. Two directory names that canonicalize to the same home name the same credentials. No
unconditional account-derived primary-login item shared across different canonical homes appears in
this path.

(CX1, codex-rs/cli/src/login.rs:569-579; codex-rs/cli/src/login.rs:600-607;
codex-rs/cli/src/login.rs:627-633)

```rust
    let config_server_logout = if config
        .config_layer_stack
        .requirements_toml()
        .requires_remote_config()
    {
        logout_config_server(&config).await
    } else {
        Ok(false)
    };

    // Cleanup is independent of the current feature gate. Retain the grant lock
...
    .await;
    let cleanup_failed = match &enterprise_guard {
        Ok(Some(guard)) => guard.delete_tokens().is_err(),
        Ok(None) => false,
        Err(_) => true,
    };
    if cleanup_failed {
        eprintln!("Warning: failed to remove enterprise authorization; continuing account logout");
...
        if let Some(paths) = ConfigEditsBuilder::bedrock_provider_config_paths_to_clear(&config) {
            let edits = paths
                .into_iter()
                .map(|segments| ConfigEdit::ClearPath { segments });
            if let Err(err) = ConfigEditsBuilder::for_config(&config)
                .with_edits(edits)
                .apply()
```

In this reading, three additional cleanups are independent of the primary store mode. For required
remote configuration, logout_config_server creates the configured OAuthSessionManager and calls its
logout (cli/src/login.rs:556-563; cloud-config/src/bundle_loader.rs:17-25 and
config_server.rs:47-55,86-100). OAuthSessionManager removes its persisted grant and in-memory copy
without contacting the issuer (login/src/oauth_session_login.rs:135-148). Its credential_id hashes
the home, authorization and token endpoints, client identifier, resource and scopes
(login/src/oauth_session.rs:403-422). File storage rewrites secrets/gateway_oauth.json after
removing that entry; Keyring storage removes its Global secret from the GatewayOAuth namespace,
rewriting secrets/gateway_oauth.age, with an independent home-derived passphrase key ending
|gateway-oauth, not deleting that passphrase (oauth_session_storage.rs:65-80,110-124;
secrets/src/local.rs:44,181; secrets/src/lib.rs:184-203). It also creates or opens
secrets/gateway_oauth.lock (oauth_session_storage.rs:25-36). An error here is retained, while
primary cleanup still runs, then produces exit 1 (cli/src/login.rs:644-650).

In this reading, when mcp_enterprise_managed_auth is configured and ema_auth_scope finds a user and
workspace, EnterpriseOAuthCredentialGuard acquires the lock, invalidates pending logins, and deletes
the grant (cli/src/login.rs:582-605; codex-mcp/src/ema_auth.rs:51-54;
rmcp-client/src/enterprise_oauth_login.rs:46-100). credential_name hashes the user and workspace
identifiers, issuer and client identifier, with the ema-idp prefix (config/src/mcp_ema.rs:130-146).
Although that name is account-derived, the keyring deletion key additionally includes the canonical
home (rmcp-client/src/oauth.rs:1056-1085). The service identifier is KEYRING_SERVICE in
oauth.rs:100; Direct deletes that item, Secrets deletes it and the hashed Global secret in the
home's secrets/mcp_oauth.age (oauth.rs:698-750,1092-1099). The home's fallback .credentials.json is
rewritten without that entry, or removed if empty (oauth.rs:993-1013,1101-1103,1151-1175).

(CX1, codex-rs/rmcp-client/src/oauth.rs:1068-1078)

```rust
        // The OS keyring is shared across homes. Keep enterprise sessions
        // isolated by Codex profile as well as authenticated user and workspace.
        let codex_home = find_codex_home()?;
        fs::create_dir_all(&codex_home)?;
        payload.insert(
            "codex_home".to_string(),
            serde_json::to_value(codex_home.as_path().canonicalize()?)?,
        );
        // Different binaries can enable different serde_json ordering features.
        serde_json::to_value(payload.into_iter().collect::<BTreeMap<_, _>>())?
    } else {
```

In this reading, enterprise cleanup also creates or opens a hashed .lock file under mcp-oauth-locks
and writes hashed .enterprise-generation and .enterprise-credential-version files there
(rmcp-client/src/oauth/refresh_lock.rs:35-66; oauth/enterprise_generation.rs:33-46,83-120). It also
creates or opens file-store.lock in that directory each time it deletes from the fallback file,
before reading it and whether or not it rewrites it (rmcp-client/src/oauth.rs:691,993-998), and
secrets-store.lock there when the Secrets backend is selected (oauth.rs:729-734);
oauth/store_lock.rs:32-35 names them, :114 creates them and :206-207 places them under the home. The
key is home-scoped as quoted above. Cleanup errors warn and do not stop account removal. Ordinary
MCP grants outside this enterprise cleanup are not enumerated for deletion by run_logout. No extra
vendor revocation occurs in these local enterprise helpers.

In this reading, a selected Bedrock provider can make logout clear model_provider,
model_providers.<provider>.aws and model in the active user configuration, if that layer selects the
same provider (core/src/config/edit/bedrock.rs:8-35). ConfigEditsBuilder::for_config selects the
user configuration file, falling back to the home's config.toml, and apply writes it
(core/src/config/edit.rs:806-812,996-1000). This is a configuration edit, not another credential
revocation. It runs after primary credential removal; failure exits 1 (cli/src/login.rs:626-642).

(CX1, codex-rs/login/src/auth/storage_commit.rs:31-32;
codex-rs/login/src/auth/storage_commit.rs:39-46)

```rust
        // Keyring commits can unlink an intermediate symlink used by another
        // home, so resolvable chains must share the target's commit lock.
...
        .with_extension("json.lock");
        std::fs::create_dir_all(
            path.parent()
                .ok_or_else(|| std::io::Error::other("credential lock has no parent"))?,
        )?;
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
```

In this reading, no removal above is keyed only by account across distinct canonical homes. Primary
and enterprise Direct items use home-derived keys; encrypted files and fallback files are under the
selected home. The source explicitly recognizes sharing through credential symlinks: keyring cleanup
can unlink an intermediate symlink that another home uses. File removal also unlinks the selected
home's login entry (storage.rs:171-175); another home whose credential link traverses that entry can
lose access to the shared login. The commit lock follows resolvable credential links. Explicitly
sharing an actual home or a selected configuration file also shares that state; no stronger
filesystem isolation is shown by this read.

In this reading, the OS deletion boundary is keyring::Entry::new(service, account) followed by
entry.delete_credential (keyring-store/src/lib.rs:113-128), with keyring 3.6.3 pinned at
codex-rs/Cargo.lock:9971-9972. Encrypted-file writes call age through secrets/src/local.rs; age
0.12.1 is pinned at Cargo.lock:77-78. Hashing calls sha2 (0.10.9, Cargo.lock:14367-14368), and
default-home resolution calls dirs (6.0.0, Cargo.lock:6820-6821). No dependency source was read.

### C3. Whether it revokes at the vendor, and the scope

(CX1, codex-rs/login/src/auth/manager.rs:1068-1087)

```rust
    let auth_dot_json = match load_auth_dot_json(
        codex_home,
        auth_credentials_store_mode,
        keyring_backend_kind,
    ) {
        Ok(auth_dot_json) => auth_dot_json,
        Err(err) => {
            tracing::warn!("failed to load stored auth during logout: {err}");
            None
        }
    };
    if let Err(err) = revoke_auth_tokens(auth_dot_json.as_ref(), auth_route_config).await {
        tracing::warn!("failed to revoke auth tokens during logout: {err}");
    }
    logout_all_stores(
        codex_home,
        auth_credentials_store_mode,
        keyring_backend_kind,
    )
}
```

(CX1, codex-rs/login/src/auth/revoke.rs:1-5; codex-rs/login/src/auth/revoke.rs:59-65;
codex-rs/login/src/auth/revoke.rs:69-75; codex-rs/login/src/auth/revoke.rs:79-82;
codex-rs/login/src/auth/revoke.rs:87-94)

```rust
//! Best-effort OAuth token revocation used during logout.
//!
//! Managed ChatGPT auth stores OAuth tokens locally. Logout attempts to revoke the
//! refresh token, falling back to the access token when no refresh token is
//! available, and callers still remove local auth if the revoke request fails.
...
    let Some((token, kind)) = auth_dot_json.and_then(revocable_token) else {
        return Ok(());
    };

    let endpoint = revoke_token_endpoint();
    let client = create_default_auth_client(&endpoint, auth_route_config)?;
    revoke_oauth_token(&client, endpoint.as_str(), token, kind, REVOKE_HTTP_TIMEOUT).await
...
    let tokens = managed_chatgpt_tokens(auth_dot_json)?;
    if !tokens.refresh_token.is_empty() {
        Some((tokens.refresh_token.as_str(), RevokeTokenKind::Refresh))
    } else if !tokens.access_token.is_empty() {
        Some((tokens.access_token.as_str(), RevokeTokenKind::Access))
    } else {
        None
...
fn managed_chatgpt_tokens(auth_dot_json: &AuthDotJson) -> Option<&TokenData> {
    if resolved_auth_mode(auth_dot_json) == AuthMode::Chatgpt {
        auth_dot_json.tokens.as_ref()
    } else {
...
fn resolved_auth_mode(auth_dot_json: &AuthDotJson) -> AuthMode {
    if let Some(mode) = auth_dot_json.auth_mode {
        return mode;
    }
    if auth_dot_json.openai_api_key.is_some() {
        return AuthMode::ApiKey;
    }
    AuthMode::Chatgpt
```

In this reading, only stored managed Chatgpt tokens are eligible: the selected home's refresh token
if nonempty, otherwise its nonempty access token. The ID token is not sent. ApiKey,
ChatgptAuthTokens, AgentIdentity, PersonalAccessToken and Bedrock modes yield no revocable managed
token. Where auth_mode is missing, an API-key field resolves to ApiKey and otherwise to Chatgpt.
Missing or unreadable stored auth skips revocation, with a warning on read error. The command uses
stored auth even if an access-token environment variable is present.

(CX1, codex-rs/login/src/auth/revoke.rs:39-44; codex-rs/login/src/auth/revoke.rs:104-118;
codex-rs/login/src/auth/revoke.rs:135-136; codex-rs/login/src/auth/revoke.rs:140-145;
codex-rs/login/src/auth/revoke.rs:149-152)

```rust
    fn client_id(self) -> Option<String> {
        match self {
            Self::Access => None,
            Self::Refresh => Some(oauth_client_id()),
        }
    }
...
    let request = RevokeTokenRequest {
        token,
        token_type_hint: kind.as_str(),
        client_id: kind.client_id(),
    };

    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .timeout(timeout)
        .json(&request)
        .send()
        .await
        .map_err(std::io::Error::other)?;

...
    if let Ok(endpoint) = std::env::var(REVOKE_TOKEN_URL_OVERRIDE_ENV_VAR) {
        return endpoint;
...
        && let Some(endpoint) = derive_revoke_token_endpoint(&refresh_endpoint)
    {
        return endpoint;
    }

    REVOKE_TOKEN_URL.to_string()
...
    let mut url = url::Url::parse(refresh_endpoint).ok()?;
    url.set_path("...");
    url.set_query(None);
    Some(url.to_string())
```

In this reading, RevokeTokenRequest sends token and token_type_hint; refresh-token revocation also
sends oauth_client_id(), while access-token revocation omits it. oauth_client_id reads
CLIENT_ID_OVERRIDE_ENV_VAR, otherwise CLIENT_ID (manager.rs:1865-1870). The endpoint comes from
REVOKE_TOKEN_URL_OVERRIDE_ENV_VAR, otherwise a derived endpoint from
REFRESH_TOKEN_URL_OVERRIDE_ENV_VAR, otherwise REVOKE_TOKEN_URL. The derived path literal is cut
above. The request uses create_default_auth_client and a ten-second timeout (revoke.rs:23,63-65). A
non-success status is an error (revoke.rs:120-131), but the manager warns and continues local
removal, as quoted above. A storage-deletion error can still fail logout afterward.

In this reading, the request names only the stored home's token and the OAuth client, not every
login, device or account. This satisfies decision 2's token-scoped reading where no read source
states broader vendor effects. It does not establish what the vendor actually invalidates along with
that token.

Not found: a statement that the revoke request invalidates other homes, every login, every device,
or an account-wide set. Searches: logout, log out, sign out, revoke, revocation, invalidate, session
and token in codex-rs/login/src and codex-rs/cli/src; C4 records the documentation search. The
revocation comment quoted above describes token selection and best-effort removal, not broader
vendor effects.

In this reading, the enterprise-cleanup auth load before this request can itself send non-revocation
requests. This branch requires mcp_enterprise_managed_auth, then calls
config.auth_config().load_auth(false) (cli/src/login.rs:582-589). AuthConfig passes cached_auth
None, even though false disables only the API-key environment shortcut
(login/src/auth/manager.rs:1298-1331,1587-1593). Its AgentIdentity access-token environment branch
remains possible (manager.rs:1628-1659). The following requests are part of that load, not
additional revocations:

(CX1, codex-rs/login/src/auth/personal_access_token.rs:97-102;
codex-rs/login/src/auth/personal_access_token.rs:59-66;
codex-rs/login/src/auth/personal_access_token.rs:128-130)

```rust
    let response = client
        .get(endpoint)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|err| {
...
            .map(|base_url| base_url.trim().trim_end_matches('/').to_string())
            .filter(|base_url| !base_url.is_empty())
            .unwrap_or_else(|| PROD_AUTHAPI_BASE_URL.to_string());
        let endpoint = whoami_endpoint(&authapi_base_url);
        let client = create_default_auth_client(&endpoint, auth_route_config)?;
        hydrate_personal_access_token(&client, &endpoint, access_token).await
    }

...
fn whoami_endpoint(authapi_base_url: &str) -> String {
    format!("{}{WHOAMI_PATH}", authapi_base_url.trim_end_matches('/'))
}
```

In this reading, PersonalAccessTokenAuth::load_with_cached reaches this GET because that caller
supplies no cached auth. It sends the personal access token as bearer authorization, from the stored
login or the access-token environment branch. CODEX_AUTHAPI_BASE_URL_ENV_VAR selects the base,
otherwise PROD_AUTHAPI_BASE_URL; WHOAMI_PATH supplies its path. Those identifiers, not their values,
name the address here. A request or decoding failure bubbles into the enterprise_guard result,
warns, and does not stop primary removal (manager.rs:396-409,1633-1647;
personal_access_token.rs:40-66,92-125; cli/src/login.rs:600-608).

(CX1, codex-rs/agent-identity/src/lib.rs:251-255; codex-rs/agent-identity/src/lib.rs:325-332)

```rust
    let response = client
        .get(agent_identity_jwks_url(agent_identity_jwt_base_url))
        .timeout(AGENT_IDENTITY_JWKS_TIMEOUT)
        .send()
        .await
...
    let url = agent_task_registration_url(agent_identity_authapi_base_url, key.agent_runtime_id);

    let response = client
        .post(url)
        .timeout(AGENT_TASK_REGISTRATION_TIMEOUT)
        .json(&request)
        .send()
        .await
```

In this reading, AgentIdentityAuth::from_jwt validates a JWT with a JWKS GET, then loads a record;
from_record registers a task if task_id is absent or empty
(login/src/auth/agent_identity.rs:133-163,253-283). The JWKS GET sends no login token. The
task-registration POST sends a timestamp and a signature formed with the record's agent key, not a
home's access, refresh or ID token (agent-identity/src/lib.rs:315-332). The registration may retry
up to three attempts (login/src/auth/agent_identity.rs:318-355). No agent registration request in
register_managed_chatgpt_agent_identity is called by this load.

In this reading, agent_identity_jwks_url forms the GET address from the selected ChatGPT base, or
CODEX_AGENT_IDENTITY_JWKS_BASE_URL_ENV_VAR, and one of its two path literals.
ChatGptEnvironment::chatgpt_base_url supplies a default production or staging base when none is
configured (agent-identity/src/lib.rs:76-81,483-489; login/src/auth/manager.rs:369-379,531-535;
auth/agent_identity.rs:253-271). agent_task_registration_url forms the POST address from the
agent-auth base, the record's runtime identifier and its path template
(agent-identity/src/lib.rs:473-480,492-495). CODEX_AGENT_IDENTITY_AUTHAPI_BASE_URL_ENV_VAR can
choose that base; otherwise PROD_AGENT_IDENTITY_AUTHAPI_BASE_URL or
STAGING_AGENT_IDENTITY_AUTHAPI_BASE_URL does (login/src/auth/agent_identity.rs:43-68;
agent-identity/src/lib.rs:83-88). A failure again warns through enterprise_guard and leaves primary
cleanup running. These calls obtain metadata or register a task; none asks the vendor to revoke or
end a login.

In this reading, the CLI config load supplies a default CloudConfigBundleLoader whose future returns
Ok(None), and a NoopThreadConfigLoader, so it does not fetch a remote bundle on this path
(core/src/config/mod.rs:1503,1589-1615,2077-2083; config/src/cloud_config_bundle.rs:520-523;
config/src/loader/mod.rs:213-215). Ordinary Chatgpt auth construction builds a client without
refreshing or sending a request (login/src/auth/manager.rs:443-473). create_default_auth_client at
auth/default_client.rs:449-459 constructs the route-aware client; that call sends no request and
removes nothing, not followed further here. The config-server logout explicitly avoids contacting
its issuer (oauth_session_login.rs:135-148). Enterprise deletion and Bedrock configuration edits in
C2 are local.

In this reading, the request wrapper dispatches direct or routed execution at
http-client/src/request_builder.rs:181-187; its direct execution calls reqwest's execute at
http-client/src/client.rs:170-176. reqwest 0.12.28 is selected by the workspace dependency
(codex-rs/Cargo.toml:469; http-client/Cargo.toml:17; Cargo.lock:13099-13100). URL parsing and path
editing call url::Url::parse and set_path (revoke.rs:148-152), with url 2.5.8 pinned at
Cargo.lock:16591-16592. No dependency implementation or vendor behavior was read or run.

### C4. Codex's documentation

“Remove stored authentication credentials.” (CD2, command summary, codex logout)

“Remove saved credentials for both API key and ChatGPT authentication. This command has no flags.”
(CD2, codex logout reference)

“Run codex login status to see the active authentication method. For stored authentication, run
codex logout to clear the current credentials. When the process selects workload identity, Codex
rejects codex login and codex logout because the process environment controls authentication.” (CD1,
Check authentication or sign out)

“The CLI and extension share the same cached login details. If you log out from either one, you’ll
need to sign in again the next time you start the CLI or extension.” (CD1, Login caching)

“Use cli_auth_credentials_store to control where the Codex CLI stores cached credentials:” (CD1,
Credential storage)

“keyring stores credentials in your operating system credential store and fails if it is
unavailable.” (CD1, Credential storage)

“auto uses the OS credential store when available, otherwise falls back to ...” (CD1, Credential
storage)

“ephemeral keeps credentials in memory only for the current process.” (CD1, Credential storage)

“Control where the CLI stores cached credentials.” (CD3, cli_auth_credentials_store; values file,
keyring, auto and ephemeral)

“To end the ChatGPT web session in that browser, select Log out.” (CD1, Check authentication or sign
out)

“Sign out of Codex.” (CD2, /logout)

In this reading, CD1's browser-session wording and CD2's sign-out wording state no vendor request or
revocation scope under decision 2. CD1's cache-sharing statement concerns the CLI and extension, not
distinct configuration homes. CD2's no-flags wording describes fewer configuration details than C1's
global -c handling; it states no contrary revocation or scope. CX1's docs/authentication.md:3 links
CD1 and states no logout behavior.

“Logout clears the cached routing, and results from earlier authentication owners are discarded.”
(CX1, codex-rs/app-server/README.md:562)

“`test-logout` initializes the app-server, sends an `account/logout` request, and waits for the
resulting `account/updated` notification. It uses the active `CODEX_HOME`, so point it at an
isolated directory when testing credential cleanup.” (CX1,
codex-rs/app-server-test-client/README.md:52-54)

In this reading, the README revocation wording at app-server/README.md:279 concerns
user-verification key deletion, and lines 669-672 concern active-request policy, not codex logout or
vendor login revocation. Other invalidate matches concern audio queues or source receipts, not
authentication. No README statement read contradicts C3's revocation or scope.

Not found: a statement that codex logout makes no vendor revocation, a statement of its vendor
revocation's scope, or a statement of what logout in one home does to another home. Searches in
CX1's docs/ and every README, and in CD1, CD2 and CD3: logout, log out, sign out, revoke,
revocation, invalidate, session and token; the pages were also searched for another home, other
home, every device, all devices, every login and all logins. CD1's revocation-guidance link concerns
enterprise access tokens; no linked page was fetched. The local-removal descriptions say less than
C3 and do not state that no request is made, so they do not differ under decision 2's terms.

## Not shown

- Anything observed: this read ran no client.
- What the vendor's service revokes when it receives a request, beyond what the pages and the source
  state.
- Whether Claude Code's client does what its pages state: Claude Code has no public source.
- What a library that Codex calls does inside.
- Where a Claude Code login lands for the dedicated macOS user, a keychain item or a file (A22).
- Whether a new sign-in rotates or revokes the tokens of another home on the same account (the
  review's G01).
- Codex on the Windows computer or under WSL, any version of Codex but 0.167.1, and any version of
  Claude Code: its pages are as served on the day they were read.
- The logins' literal origins, paths, client IDs and ports.
- What order 050's hooks do with the outcome, and the wording of item 8's fail branch.
