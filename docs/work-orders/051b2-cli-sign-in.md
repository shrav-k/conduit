# Work order 051b2 (C1b-2): `conduit sign-in` asks the person, opens the browser and retries to the end

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: this order adds no case, so `vectors` is an empty list, and its
tests go under `checks`. This order has network access only because the shell's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate and no
crate feature, and reach no other machine. Run one cargo command at a time. Never print a whole
large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential. Test code may read the client token it issued in
its own temporary store, as `server::client` does, and nothing prints it. Do not run `claude`,
`codex` or any client, sign in nowhere, open no browser and no URL, and never run the built
`conduit sign-in` command yourself: only the tests below drive the new code, through a scripted
person whose browser is a list.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s batch 1 gives "051b: the pending-action
commands `pending`, `sign-in` and `answer`. M. After 047 and 048b-join." Spec batch 1's plan adds
the client's URL elicitation: "show the URL and the message, ask consent, open the browser". The
coordinator cut 051b into three, each holding one behavior
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264); the coordinator's
split, open to the owner's strike), as order 051b's "Where it sits" lists them: 051b, `conduit
pending`, `conduit answer` and the unanswered hint; **051b-2, this order**, `conduit sign-in`;
and 051b-3, the carried return from the command line (D53, ruling 75 C4's case), after this one.
This order comes after 048b-join, and so after every part of 048a-5, 048a-6 and 048b-relay, and
after 051b, whose command line it extends.

**Why it is needed now.** `server.rs`'s `client`, behind `conduit call`, uses rmcp's `call_tool`
with no elicitation handler, so it cannot answer an `input_required` result (order 048b-join's
decision 8). [Ruling 82](../proposals/ruling-82/README.md#the-decisions), point 1, has the owner
sign a scratch home in "through Conduit's sign-in flow", and point 3 has each dogfood slot's
sign-in follow 051b. The coordinator's plan makes this order's command the client of A5's live
check ([experiment 027](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check)),
which runs after 048b-join merges and before 0.2.0 merges, at the time the owner picks, told
first. This order runs no live check and signs nothing in.

**What it builds.** One behavior: a person signs an instance in from the command line. The
command sends `sign-in.v1` through root's Conduit to the instance's owner, declaring URL
elicitation; shows the engine's message and the full URL; asks the person's consent; opens the
browser only on consent; and carries each retry, accept, decline or cancel, until the engine gives
a final result. IFC-049: "For sign-in.v1 the request must declare {"elicitation": {"url": {}}} in
_meta.io.modelcontextprotocol/clientCapabilities"; "With that capability tools/call may return
InputRequiredResult with resultType input_required and exactly one inputRequests entry:
elicitation/create with mode url, message and url"; "The client echoes it exactly"; "For
sign-in, accept means consent, not completion: retries poll under PND-010, blocking or returning
another InputRequiredResult until the account appears or the wait ends." PND-010: "Decline or
cancel received on a retry ends failed." `sign-in.v1`'s description: "The optional request _meta
signInLogin asks for one stated login". No promise changes: the engine already does all of this
once 048b-join merges; the command line only asks it, shows what it answers and carries the
person's answer back.

**Ruling 83, which this order follows.** The owner decided on 2026-11-18
([ruling 83](../proposals/ruling-83/README.md)) that the Agent packages 0.2.0 are
subscription-only, with no `apiKey` in `.v2`. Nothing in this order reads, sets or names `apiKey`,
and the command offers no key route.

**What the orders before it bring, which this order assumes.** Orders 051b, 048b-join, 048a-5,
048a-6 and 048b-relay1 are written; 048a-5b to 048a-5h, 048a-7, 048a-8 and 048b-relay2 are written
beside this one or planned. Each assumption below is named, so that the coordinator checks it
against their texts and Records before dispatch and amends this order where one differs. Where you
find at the start that one merged otherwise, apply the nearest sound reading of each decision to
what merged, and say so; where an order's Record names a function, a member or a test, use that
name. Below, `META/x` is the key `x` under the prefix that `spec/names.yaml` sets as `metaPrefix`
(in the shell's tests `shell.context.meta_prefix`), and `C` is the request `_meta` key
`io.modelcontextprotocol/clientCapabilities`.

- **B1, from 051b.** In `engine/crates/shell/src/author.rs`, as on `main` and kept by 051b:
  `fn route(directory: &Path, object: &Value, operation: &str) -> Result<String>`, `fn
  named(argument: &str) -> Result<Value>` and `pub fn ensure_success`; added by 051b:
  `pub fn note_unanswered(reply: &Value)` with
  `pub fn unanswered_hint() -> Option<String>`; `main.rs`'s `main` writes the hint on standard
  error after `run` returns, for every command but `pending` and `__run-guard`. `help.rs`'s
  `COMMANDS` and `DESCRIPTIONS` hold 14 entries, `pending` and `answer` right after `inspect`.
  `tests/cli.rs`'s list `verbs` holds `"pending"` and `"answer"` after `"inspect"`. In
  `engine/README.md`, 051b's paragraph that begins "`conduit pending --store DIR` lists" ends
  with the sentence "The sign-in command is not built yet (order 051b-2)."
- **B2, from 048a-5.** A `sign-in.v1` request whose `C` does not declare `elicitation.url` gets
  the protocol error `-32021` under IFC-049, before any part. The shell forwards `C` and
  `META/signInLogin` to the core (`sign_in_members` in `server.rs`); a `signInLogin` naming no
  stated login reports invalid with one diagnostic naming PND-010 and the stated names.
- **B3, from 048b-relay1.** An input-required output reaches the Interface as
  `CallToolResponse::InputRequired`, whose `inputRequests` holds exactly one entry, of key `url`,
  `{"method": "elicitation/create", "params": {"mode": "url", "message": M, "url": U,
  "elicitationId": ""}}`, with a `requestState` of 43 characters and the core's `_meta`. A retry
  carries both `requestState` and `inputResponses`; a refused retry is answered with
  `ErrorData::invalid_params` (`-32602`) and one of relay1's texts, the expired one being
  `requestState expired with its URL action's wait (DL18)` and the unknown one beginning
  `requestState is not one this engine issued`. The retry's `arguments` are not read.
- **B4, from 048a-5e.** The engine writes `M` (instance, Type and package, node, the URL's host,
  the expected account, what the person brings back) and puts `signInLogin`
  `{name, return, node, acceptsReturn}` in the output's `_meta`.
- **B5, from 048a-5f and 048a-5h.** An accepted retry polls `account-hook.v1`. The expected
  account ends the invocation succeeded with output `account`, answering the sign-in action and
  lifting PND-005's hold. No account yet answers another input-required output with a fresh
  state, dispatching no other part and **repeating the first output's URL elicitation, with the
  same `url`**. Decline or cancel ends it failed and dispatches no part. At DL18 the invocation
  ends unknown, and a later retry is refused as expired (B3). Where 048a-5f's Record says that a
  further output holds another URL or dispatches `sign-in-hook.v1` again, stop and report it
  before you change anything: decision 8's first test rests on both.
- **B6, from 048a-5 and 048a-6.** A first `sign-in.v1` request that carries `META/signInReturn`
  reports invalid under PND-013 (048a-5's refusal); 048a-6 forwards that key to the core and
  refuses it on a request whose operation is not `sign-in.v1`. This order never sends that key
  (051b-3's).
- **B7, from 048b-join.** In `engine/crates/shell/src/server_tests.rs`: `established_case(path,
  case, through)`, `fixed_case_shell(path, case, through)`, the executor `SignInScript` and its
  dispatch answers, and the URL `U` for the login `browser` that its script answers; the two tests
  `sign_in_relays_through_the_interface_and_carries_the_return` and
  `sign_in_decline_through_the_interface_ends_failed`. Their store, from
  `conformance/conventions/requests/sign-in-hold` through step 9, has the Agent `signed` set up
  with the account `primary-account`, its sign-in action and hold standing, and root reaching it
  through the tool `group.sign-in.v1`; root is issued a client token with `issue_client_token`.

**What the shell already does, so that you do not redo it.** Line numbers are those of `main` at
`3c761dd9`, before every order named above; find each place by the text quoted with it.

- `server.rs`'s `client` (`pub async fn client(`, `:811`) finds root's ID among `state.json`'s
  objects (an Interface named `root` with no `spec`), reads its token from the store's
  `client-tokens` directory with `crate::store::private_read`, connects
  `StreamableHttpClientTransport` with `auth_header(token)` and `reinit_on_expired_session(false)`
  to `crate::Config::for_store(directory)?.engine_url`, and serves `()`, which declares no
  elicitation. It is unchanged here.
- In `server_tests.rs`, `modern` (`async fn modern(`) connects with
  `serve_with_lifecycle(..., ClientLifecycleMode::Discover { preferred_versions:
  vec![ProtocolVersion::V_2026_07_28] })`; an `InputRequiredResult` reaches only a peer on that
  revision or later. On such a client, rmcp 3.9.0 writes the handler's own capabilities into `C`
  of every outbound request (048b-join's finding), and it implements `ClientHandler` for
  `ClientConfig` (`rmcp::model::ClientConfig`, an alias of `InitializeRequestParams`), whose
  `capabilities` has `elicitation: Option<ElicitationCapability>` and `extensions:
  Option<ExtensionCapabilities>` (a `BTreeMap<String, JsonObject>`).
  `ElicitationCapability::new().with_url(UrlElicitationCapability::new())` declares URL
  elicitation. `call_tool_once(CallToolRequestParams) -> Result<CallToolResponse, ServiceError>`
  sends one round without driving the next; `CallToolRequestParams` has `meta`, `name`,
  `arguments`, `input_responses` and `request_state`, serialized as `_meta`, `name`,
  `arguments`, `inputResponses` and `requestState`.
- The shell's HTTP layer refuses with `-32021` under IFC-036 a request that carries a key under
  the prefix without declaring the extension `META/implementation-interface` with
  `{"revision": "v0"}` in `C` (`server.rs:621` to `:634`, from `let carries = meta`).
- `boundary::value(path)` reads one YAML document as JSON; `spec/names.yaml` holds `metaPrefix`.
- `main.rs`'s `run` builds a `tokio::runtime::Runtime::new()`, the multi-thread runtime, for the
  commands it hands to `author::run`.

**When this order starts.** Orders 048b-join and 051b must be merged to `main`, and so every order
before each. Under the owner's approval of stacked dispatch, the coordinator may instead start
this order from a verified branch that holds both, saying so in a dispatch note, which relaxes
only the first two checks below. Before you change anything, check from the repository root that
you find what they give, and stop, change nothing and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048b-join-build-sign-in-through-the-interface.md
grep -c '^## Record' docs/work-orders/051b-cli-pending-and-answer.md
grep -c 'fn sign_in_relays_through_the_interface_and_carries_the_return' engine/crates/shell/src/server_tests.rs
grep -c 'fn established_case' engine/crates/shell/src/server_tests.rs
grep -c 'fn fixed_case_shell' engine/crates/shell/src/server_tests.rs
grep -c 'struct SignInScript' engine/crates/shell/src/server_tests.rs
grep -c 'pub fn note_unanswered(reply: &Value)' engine/crates/shell/src/author.rs
grep -c '^fn route(directory: &Path, object: &Value, operation: &str) -> Result<String> {$' engine/crates/shell/src/author.rs
grep -c '^fn named(argument: &str) -> Result<Value> {$' engine/crates/shell/src/author.rs
grep -o 'const COMMANDS: \[(&str, &str, &str); [0-9]*\]' engine/crates/shell/src/help.rs
grep -c '"sign-in"' engine/crates/shell/src/main.rs
test -e engine/crates/shell/src/sign_in.rs; echo "file $?"
grep -c 'requestState expired with its URL action' engine/crates/shell/src/server/relay.rs
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < engine/README.md | grep -c 'The sign-in command is not built yet (order 051b-2).'
```

They print `1` and `1` (each order's Record: the merged check, which a dispatch note may relax);
`1`, `1`, `1` and `1` (048b-join's tests and helpers, B7); `1` (051b's hint, B1); `1` and `1`
(the private `route` and `named`, B1); `const COMMANDS: [(&str, &str, &str); 14]` (051b's
help, B1), or 14 plus the commands an order merged beside added (report which); `0` (no command
yet); `file 1`; `1` (relay1's expired text, B3); `0` (C11's stub removed, 048a-5b); and `1`
(051b's sentence, B1).
Then, from `engine/`, run 048b-join's two tests, one at a time:

```bash
cargo test --release -p conduit-shell sign_in_relays_through_the_interface
cargo test --release -p conduit-shell sign_in_decline_through_the_interface
```

Each must report 1 passed and 0 failed, or stop and report it: this order builds on that path.
Orders that land beside this one (051a, 051c and others) touch the command line too; whichever
lands second keeps both changes. Where 051a or S0c has merged first and moved the commands into a
table, add `sign-in` to that table in the places this order names, and say so.

**Who decided what.** IFC-049, PND-010, PND-013 and DL18 decide what the request must declare,
what the engine relays, what a retry means and when the wait ends; ACC-033 and PND-010 decide
through which target `sign-in.v1` goes. Spec batch 1's plan gives 051b "Client URL elicitation:
show the URL and the message, ask consent, open the browser." Everything else here is the
coordinator's, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):
the split of 051b and this part's place after 048b-join and 051b; that this command is the client
of A5's live check; the command's name, its argument `KIND/NAME` and its option `--login NAME`;
routing through 051b's `route`; a client of its own beside `server::client`, connected as root on
revision `2026-07-28`; sending no `expectedId`; showing the message and the URL on standard error
with control characters escaped, and the result on standard output as one JSON line; consent read
as `y` or `yes`, decline as `n` or `no`, the end of input as cancel, and any other line asked
again; opening the browser only on consent, by the platform's own opener with the URL as one
argument and no shell; waiting for the person's Enter before an accepted retry, rather than
retrying at once or on a timer; not opening a URL already opened; the retry carrying no `_meta`
of its own; the texts; refusing to open anything but one https URL elicitation; the tests, their
script and every value in them; and the paragraph of `engine/README.md`. Apply them as written and
report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-049 and IFC-036 (`spec/interface/implementation.yaml`); PND-010 and PND-013
   (`spec/promises/pending.yaml`); DL18 (`spec/implementation-defined.yaml`); `sign-in.v1` in
   `spec/core/operations.yaml`.
2. Orders 051b and 048b-join, whole, with their Records; in order 048b-relay1, decision 4's
   refusal texts.
3. `engine/crates/shell/src/author.rs` and `main.rs` whole; `server.rs` from `pub async fn
   client`; `help.rs`; in `server_tests.rs`, `modern`, 048b-join's two tests and their helpers.
4. rmcp 3.9.0 in cargo's registry, read only: `src/model/mrtr.rs` (`CallToolResponse`,
   `InputRequiredResult`), `src/model/capabilities.rs` (`ClientCapabilities`,
   `ElicitationCapability`), and in `src/model.rs` `CallToolRequestParams` and `ClientConfig`.

**Size: M**: about 0.4 to 0.5k changed lines of Rust, about half of them tests, in one new file
and six changed ones, and one sentence of `engine/README.md`.

## The decisions it applies

1. **The command.** `conduit sign-in --store DIR KIND/NAME [--login NAME]`. In `main.rs`: add
   `sign-in|` after `answer|` in the usage string; add `"sign-in"` to the list of commands that
   collect positional arguments and to the list of known commands; add a variable
   `let mut login = None;` beside `as_instance`, and the arm
   `"--login" if command == "sign-in" => login = Some(args.next().ok_or("--login needs NAME")?),`
   right after the `"--as"` arm. Right after the block that hands commands to `author::run`, add:

   ```rust
   if command == "sign-in" {
       let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
       let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
       let mut person = conduit_shell::sign_in::Terminal;
       return runtime.block_on(conduit_shell::sign_in::run(
           &store,
           &positional,
           login.as_deref(),
           &repository,
           &mut person,
       ));
   }
   ```

   The multi-thread runtime keeps the client's tasks running while the terminal is read. In
   `engine/crates/shell/src/lib.rs`, add `pub mod sign_in;` right after `pub mod server;`. In
   `author.rs`, make `route` and `named` `pub(crate)`, and change nothing else there.

2. **The new file `engine/crates/shell/src/sign_in.rs`**, beginning with the line
   `//! conduit sign-in: one sign-in.v1 through serve, its URL action shown, consented to and retried (order 051b-2).`
   It holds:
   - `pub trait Person { fn ask(&mut self, prompt: &str) -> Result<Option<String>>; fn tell(&mut self, line: &str); fn open(&mut self, url: &str) -> Result<()>; }`:
     `ask` shows `prompt` and reads one line, `None` at the end of input; `tell` shows one line;
     `open` opens `url` in the person's browser.
   - `pub struct Terminal;` implementing `Person`: `tell` is `eprintln!("{line}")`; `ask` writes
     `prompt` with `eprint!`, flushes standard error, reads one line from standard input with
     `read_line`, gives `None` where it read 0 bytes, and otherwise the line without its trailing
     `\n` or `\r\n`; a read error gives `Err(format!("could not read the terminal: {error}"))`.
     `open` runs `browser_command(url)` with standard input, output and error set to
     `Stdio::null()` and waits for its `status()`; a spawn error gives
     `Err(error.to_string())`, and an unsuccessful status `Err(format!("{program} exited with {status}"))`,
     `program` the command's program as `get_program()` gives it, lossily.
   - `fn browser_command(url: &str) -> std::process::Command`: on `target_os = "macos"`,
     `open` with the one argument `url`; on `windows`, `rundll32` with the two arguments
     `url.dll,FileProtocolHandler` and `url`; on any other target, `xdg-open` with the one
     argument `url`. Each by `#[cfg(...)]`; no shell, and the URL is always one argument.
   - `pub(crate) fn shown(text: &str) -> String`: each character for which `char::is_control` is
     true written as `c.escape_unicode()`, every other character as it is, so that nothing the
     engine sends can move the terminal's cursor or change its colours.
   - `pub(crate) struct UrlAction { pub key: String, pub message: String, pub url: String, pub state: String }`
     and `pub(crate) fn url_action(result: &Value) -> Result<UrlAction>`, `result` an
     `InputRequiredResult` serialized with `serde_json::to_value`. It gives the action where
     `inputRequests` is an object of exactly one entry, of key `key`, whose `method` is
     `elicitation/create` and whose `params.mode` is `url`, `params.message` a string and
     `params.url` a string that `url::Url::parse` reads with scheme `https`, and where
     `requestState` is a nonempty string. Other members of `params`, such as rmcp's empty
     `elicitationId` (B3), are ignored. Otherwise it gives the error
     `the engine's input request is not one URL elicitation with an https URL and a requestState; nothing was opened and nothing more was sent, so this sign-in waits until its URL action's wait ends (DL18)`.
   - `#[derive(Debug, PartialEq, Eq)] pub(crate) enum Consent { Open, Decline, Cancel, Again }`,
     `pub(crate) fn consent(line: Option<&str>) -> Consent` and
     `pub(crate) fn waited(line: Option<&str>) -> &'static str`. Each reads the line trimmed and
     compared ignoring ASCII case. `consent`: `y` or `yes` is `Open`; `n` or `no` is `Decline`;
     `None` is `Cancel`; any other line, the empty one included, is `Again`. `waited`: `None`, `n`
     or `no` is `"cancel"`; any other line, the empty one included, is `"accept"`.
   - `pub(crate) fn subject(args: &[String]) -> Result<Value>`: where `args` holds other than
     one item, the error
     `sign-in: expected 1 positional argument(s), received N; run conduit sign-in --help for usage and an example`;
     otherwise `crate::author::named(&args[0])?`, and where its `kind` is neither `Agent` nor
     `Resource`, the error
     `sign-in: KIND/NAME is a KIND, and sign-in.v1 signs in only an Agent or a Resource; nothing was sent`,
     `KIND/NAME` being the argument and `KIND` its kind.
   - `connect`, `rounds`, `sign_in` and `run`, decisions 3 to 5.

3. **The client, `async fn connect(directory: &Path, prefix: &str) -> Result<RunningService<RoleClient, ClientConfig>>`.**
   It finds root's ID and token as `server::client` does, with the same error texts, and the
   engine URL from `crate::Config::for_store(directory)?`. It builds the handler with exactly
   these lines, so that decision 8's control finds the second:

   ```rust
   let url_elicitation = ElicitationCapability::new().with_url(UrlElicitationCapability::new());
   handler.capabilities.elicitation = Some(url_elicitation);
   ```

   `handler` being `let mut handler = ClientConfig::default();`, and its
   `capabilities.extensions` the one entry `format!("{prefix}implementation-interface")` with the
   object `{"revision": "v0"}`. It connects one `StreamableHttpClientTransport`, built as
   `server::client` builds it, through `handler.serve_with_lifecycle(transport,
   ClientLifecycleMode::Discover { preferred_versions: vec![ProtocolVersion::V_2026_07_28] })`
   under a timeout of 10 seconds, once, with no retry. A failure or the timeout gives
   `conduit sign-in could not open a session with the engine at URL: E; run conduit serve --store DIR and check its diagnostics`,
   `URL` the engine URL, `E` the error or `timed out`, and `DIR` the store's path. The tests do
   not set `C` by hand: rmcp writes it from the handler.

4. **The rounds, `async fn rounds(service: &RunningService<RoleClient, ClientConfig>, tool: &str, arguments: Value, meta: Option<Value>, label: &str, person: &mut dyn Person) -> Result<Value>`.**
   `label` is `KIND/NAME`. Keep `opened: Option<String>`, none at first. The first request is
   `{"name": tool, "arguments": arguments}`, with `"_meta": meta` only where `meta` is `Some`,
   read into `CallToolRequestParams` with `serde_json::from_value`. Then loop, sending each
   request with `service.call_tool_once`:
   - **`Err(ServiceError::McpError(e))`**: print `{"error": E}` as one JSON line on standard
     output, `E` being `e` as `serde_json::to_value` gives it, and give an error: where
     `e.message` begins `requestState expired` or `requestState is not one this engine issued`
     (relay1's two texts, B3; after DL18 a retry may get either, since relay1 drops an expired
     entry at any other output's issue or take),
     `the engine no longer holds this sign-in's state: its URL action's wait ended (DL18) or the engine restarted, so nothing more was sent; run conduit sign-in again`;
     otherwise
     `sign-in.v1 through TOOL was answered with error CODE: MESSAGE; nothing more was sent`.
     Any other `Err` gives `sign-in.v1 through TOOL failed: E; nothing more was sent`.
   - **`Ok(CallToolResponse::Complete(r))`**: give `serde_json::to_value(r)`, an error mapped to
     its text.
   - **`Ok(CallToolResponse::Task(_))`** or any other variant: give the error
     `sign-in.v1 through TOOL was answered with a task, which conduit sign-in does not read; nothing more was sent`.
   - **`Ok(CallToolResponse::InputRequired(i))`**: serialize `i` with `serde_json::to_value`,
     an error mapped to its text, and read it with `url_action`; an error of either ends the
     loop, sending nothing. Where `opened` equals `action.url`, `tell`
     `conduit sign-in: the engine has not yet found the expected account signed in on LABEL; the URL below is the one already opened.`;
     otherwise `tell` `conduit sign-in: the engine asks you to open a URL to sign in LABEL.`.
     Then `tell`, in this order, `Message: ` followed by `shown(&action.message)` and `URL: `
     followed by `shown(&action.url)`. Where the result's `_meta` has a key ending in
     `/signInLogin` (found as `author.rs`'s `outcome` finds its key) whose `name` is a string,
     `tell` `Login: ` followed by `shown` of it; where that key's `acceptsReturn` is `true`, `tell`
     `conduit sign-in: this login's return can be carried back through the Interface, but this command does not carry one (order 051b-3): where the browser is not on the machine where LABEL runs, the login cannot complete through this command.`
     Then decide the answer `A`:
     - where `opened` equals `action.url`, `A` is `waited` of
       `person.ask("Press Enter once you have signed in, or type n to stop: ")?`;
     - otherwise ask `"Open this URL in your browser? Type y to open it, or n to decline: "` until
       `consent` gives other than `Again`. `Decline` makes `A` `"decline"`, `Cancel` makes it
       `"cancel"`, and `Open` calls `person.open(&action.url)`: where it fails, `tell`
       `conduit sign-in: the browser could not be opened (E); open the URL above yourself.`; in
       both cases set `opened` to the URL, then `A` is `waited` of the Enter prompt above.

     The next request is `{"name": tool, "arguments": arguments, "requestState": action.state,
     "inputResponses": {action.key: {"action": A}}}`, with no `_meta` of its own, and the loop
     goes on.

   `LABEL`, `TOOL`, `CODE`, `MESSAGE` and `E` are replaced by their values. No round is limited
   in number: each accepted retry waits for the person. Nothing here prints the message or the URL
   on standard output, and nothing writes them to the store.

5. **`pub async fn sign_in(directory: &Path, object: &Value, login: Option<&str>, prefix: &str, person: &mut dyn Person) -> Result<Value>`
   and `pub async fn run(directory: &Path, args: &[String], login: Option<&str>, repository: &Path, person: &mut dyn Person) -> Result<()>`.**
   `sign_in`: `let tool = crate::author::route(directory, object, "sign-in.v1")?;`, written as
   exactly that one line; the meta `{format!("{prefix}signInLogin"): {"name": login}}` where
   `login` is `Some`, and none otherwise; `connect`; `rounds` with the arguments
   `{"instance": {"kind": K, "name": N}}` of `object`, no `expectedId`, and the label `K/N`; then
   `service.cancel()`, keeping the rounds' error where both fail, as `server::client` does; and
   it gives the final result. `run`: `let object = subject(args)?;`, the prefix as the string
   `metaPrefix` of `crate::boundary::value(&repository.join("spec/names.yaml"))?`, then
   `sign_in`, where a missing or non-string `metaPrefix` gives the error `missing metadata prefix`;
   it prints the result as one JSON line on standard output, calls
   `crate::author::note_unanswered` on it, and gives
   `crate::author::ensure_success(&result, &format!("sign-in.v1 of {label} through serve"))`,
   where `run` builds `label` as `KIND/NAME` from `subject`'s object, as `sign_in` builds its own.
   So the command exits 0 only where the sign-in succeeded, and 051b's hint follows its last
   result.

6. **Help.** In `help.rs`, `COMMANDS` and `DESCRIPTIONS` become one entry more each than the start
   check found (15 where it found 14), the new one right after `answer`'s:
   `("sign-in", "KIND/NAME [--login NAME]", "sign-in --store /tmp/conduit Agent/a")`,
   described
   `Sign in an Agent or Resource through serve and root's Conduit to its owner: shows the engine's message and the full URL, opens the browser only after you type y, and retries each time you press Enter until the engine answers. --login asks for one of the Type's stated logins. Carries no return from another machine.`
   In `tests/cli.rs`, add `"sign-in"` to the list `verbs` of
   `help_describes_every_command_without_store_access`, right after `"answer"`, and change
   nothing else in that file.

7. **The unit tests**, in `sign_in.rs`'s `mod tests`:
   - `url_action_takes_one_https_url_elicitation`: for `V` =
     `{"resultType": "input_required", "inputRequests": {"url": {"method": "elicitation/create", "params": {"mode": "url", "message": "Sign in", "url": "https://example.org/login", "elicitationId": ""}}}, "requestState": "s"}`,
     `url_action` gives the key `url`, the message `Sign in`, the URL
     `https://example.org/login` and the state `s`. It gives an error for `V` with a second entry
     `other` equal to the first; with `mode` `form`; with `url` `http://example.org/login`; with
     `method` `sampling/createMessage`; and without `requestState`.
   - `answers_are_read_strictly`: `consent` gives `Open` for `Some("y")` and `Some(" YES ")`,
     `Decline` for `Some("n")` and `Some("No")`, `Cancel` for `None`, and `Again` for `Some("")`
     and `Some("maybe")`; `waited` gives `"accept"` for `Some("")` and `Some("done")`, and
     `"cancel"` for `None`, `Some("n")` and `Some("NO")`.
   - `shown_escapes_control_characters`: `shown("a\u{1b}[31mb\nc")` equals
     `"a\\u{1b}[31mb\\u{a}c"`, and `shown("https://example.org/a?b=c")` is unchanged.
   - `subject_takes_one_agent_or_resource`: `subject` of `["Agent/a"]` gives
     `{"kind": "Agent", "name": "a"}`; of `["Fleet/f"]` an error containing
     `signs in only an Agent or a Resource`; of `[]` an error containing
     `expected 1 positional argument(s), received 0`.
   - `browser_command_passes_the_url_as_one_argument`: for
     `https://example.org/a?b=c&d=e`, `browser_command`'s `get_program()` and `get_args()` are,
     on macOS, `open` and the URL alone; on Windows, `rundll32`,
     `url.dll,FileProtocolHandler` and the URL; elsewhere, `xdg-open` and the URL alone, each by
     `#[cfg(...)]`. The test never spawns it.

8. **The tests through the engine's MCP server**, in `server_tests.rs`, each
   `#[tokio::test(flavor = "multi_thread", worker_threads = 4)]`, on a store of its own opened as
   048b-join's tests open theirs: `fixed_case_shell(temp, <the repository>/conformance/conventions/requests/sign-in-hold, 9)`,
   root issued a client token with `issue_client_token`, and the executor below wrapped in
   `ServingExecutor::new` and served with `server::serve`. `prefix` is
   `shell.context.meta_prefix`.

   *The person.* Add `struct Scripted { answers: VecDeque<Option<String>>, prompts: Vec<String>, told: Vec<String>, opened: Vec<String> }`
   implementing `crate::sign_in::Person`: `ask` pushes the prompt and gives the front answer,
   popped, or `None` once none is left; `tell` pushes the line; `open` pushes the URL and gives
   `Ok(())`. It opens nothing.

   *The script.* Add an executor `PollScript`, built as 048b-join's `SignInScript` is and
   answering as it does, with one difference: in place of the flag `returned`, a queue of account
   values. `account-hook.v1` answers SignInScript's account result with the queue's front value,
   popped, or `null` where the queue is empty; `sign-in-hook.v1` with no `requestState` answers
   SignInScript's `InputRequiredResult` with `U`; any other call answers SignInScript's plain
   success. It keeps every `dispatch` intent it receives.

   *The first test*, `sign_in_command_opens_the_url_and_polls_to_signed_in`, the queue
   `[null, null, "primary-account"]` and the answers `[Some("y"), Some(""), Some("")]`:
   1. `crate::author::route(temp, &json!({"kind": "Agent", "name": "signed"}), "sign-in.v1")`
      gives `group.sign-in.v1`.
   2. `crate::sign_in::sign_in(temp, &json!({"kind": "Agent", "name": "signed"}), None, &prefix, &mut person).await.unwrap()`
      gives `F`, so that a failure's panic message holds `rounds`' text
      `was answered with error -32021: ...`: `F`'s `isError` is not `true`, its
      `structuredContent.account` is `primary-account`, and its `_meta` `META/outcome` is `succeeded`.
   3. `person.opened` is exactly `[U]`: the URL was opened once, after consent.
   4. `person.prompts` is exactly the consent prompt, then the Enter prompt twice; `person.told`
      holds `URL: U` twice, `conduit sign-in: the engine asks you to open a URL to sign in Agent/signed.`
      once, the not-yet line once, and two lines beginning `Message: ` that contain
      `example.org`.
   5. The script's intents, by `call.params.name`, are `account-hook.v1`, `sign-in-hook.v1`,
      `account-hook.v1` and `account-hook.v1`, and no other.
   6. `sign_in_holds` has no entry for `signed`'s ID.

   *The second test*, `sign_in_command_declines_cancels_and_asks_for_a_login`, the queue empty
   (every account check finds none), three sign-ins on the one store, each with a fresh
   `Scripted`:
   1. No login, the answers `[Some("maybe"), Some("n")]`: `Ok(F1)` with `F1`'s `isError` `true`
      and `META/outcome` `failed` (PND-010); `prompts` is the consent prompt twice; `opened` is
      empty.
   2. The login `Some("phone")`, no answers: `Ok(F2)` with `F2`'s `isError` `true`,
      `META/reason` `invalid` and `structuredContent.diagnostics` holding one diagnostic whose
      `ruleId` is `PND-010` and whose text contains `"browser"`; `prompts`, `told` and `opened`
      are empty.
   3. No login, no answers, through the command's own function:
      `crate::sign_in::run(temp, &["Agent/signed".into()], None, &repository(), &mut person)`,
      `repository()` being the helper of `tests.rs` that `server_tests.rs` reaches through
      `use super::*`. The end of input at the consent prompt cancels, so the sign-in ends failed
      (PND-010) and `run` gives an `Err` containing `did not succeed`; `prompts` is the consent
      prompt once; `opened` is empty. This step checks that `run` reads the prefix, prints and
      fails where the sign-in did not succeed.
   4. The script holds four intents: `account-hook.v1` and `sign-in-hook.v1` for each of the
      first and third sign-ins, and none for the second. `sign_in_holds` still has an entry for
      `signed`'s ID.

   Before running them, check that no other test's name in `engine/crates/shell/` begins
   `sign_in_command_`. If a test fails because a reply differs from what it states, stop and
   report the step and the reply: do not change the expectations to pass it. Neither test
   names `Terminal`.

9. **`engine/README.md`.** In 051b's paragraph, replace the sentence "The sign-in command is not
   built yet (order 051b-2)." with this text, and change nothing else in the file:

   ```text
   `conduit sign-in --store DIR KIND/NAME [--login NAME]` signs an Agent or a Resource in
   through `serve` and root's Conduit to its owner, declaring URL elicitation and, with
   `--login`, asking for one of its Type's stated logins. It shows the engine's message and the
   full URL on standard error, opens the browser only after you type `y`, and retries each time
   you press Enter, until the engine answers: signed in, failed where you type `n` or input
   ends, or ended once the URL action's wait ends (DL18). It prints the result as one JSON line
   and exits 0 only where the sign-in succeeded. It carries no return from another machine
   (order 051b-3). Interrupting it leaves the sign-in waiting until that wait ends.
   ```

10. **Promise IDs in the shell's sources.** The new text names no promise ID other than
    `PND-010` (the second test), which 048b-join's tests already name in `server_tests.rs`, and
    `IFC-049` nowhere in code. Where `python3 tools/promise_standing.py --check` still fails, run
    it with `--write`, as AGENTS.md asks, and report the rows it changes.

11. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line, ruling's
    record, case, fixture, core source, `server.rs`, `server/relay.rs` or `runner.rs` changes.
    No return is carried, no paste is read and no listener is opened (051b-3); `conduit call` is
    unchanged; no `expectedId` is sent; no client token for another Interface (DL13). Report as
    findings, for the coordinator: whether 048a-5f's further output repeated the first URL (B5)
    in your run; that an interrupted command leaves the invocation waiting until DL18, with the
    sign-in conflict of PND-010 refusing a new `sign-in.v1` of that instance until then; that
    `connect` repeats `server::client`'s reading of root's token; that no test runs the built
    command, and `Terminal::open` has no switch that keeps the browser shut; and any place where
    IFC-049, PND-010 or the batch plan left you a choice this order does not settle. The three
    texts that say this command carries no return (decision 4's `acceptsReturn` line, decision 6's
    help sentence "Carries no return from another machine." and decision 9's README sentence "It
    carries no return from another machine (order 051b-3).") hold only until 051b-3 merges; 051b-3
    replaces them.

## Scope

Change only the new file `engine/crates/shell/src/sign_in.rs` (decisions 2 to 5 and 7),
`engine/crates/shell/src/lib.rs` (one line, decision 1), `engine/crates/shell/src/main.rs`
(decision 1), `engine/crates/shell/src/author.rs` (`route` and `named` made `pub(crate)`,
decision 1), `engine/crates/shell/src/help.rs` (decision 6),
`engine/crates/shell/tests/cli.rs` (one name, decision 6),
`engine/crates/shell/src/server_tests.rs` (decision 8) and `engine/README.md` (decision 9).
`engine/STANDING.md` changes only as decision 10 says, written only by the tool. Not `spec/`,
`IN_FORCE.md`, `tools/`, `packages/`, `conformance/`, `engine/crates/core/`,
`engine/crates/loader/`, `engine/crates/conformance/`, `server.rs`, `server/relay.rs`,
`runner.rs`, `engine/CONFORMANCE.md`, the shell's `Cargo.toml`, the experiments,
`docs/implementation/` or `docs/proposals/`: report any defect as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --lib sign_in::tests
cargo test --release -p conduit-shell sign_in_command_
cargo test --release -p conduit-shell sign_in_relays_through_the_interface
cargo test --release -p conduit-shell --test cli help_describes_every_command
cargo test --release -p conduit-shell
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 5 passed and 0 failed: decision 7's five. In the fourth, the library's target
reports 2 passed and 0 failed and every other target 0 passed. In the fifth, the library's
target reports 1 passed, and the sixth reports 1 passed. Report the counts of the last two, and that
of the tree you started from. If any other test fails, stop and report it: do not change it. The
threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the first test fails where the command does not declare URL elicitation. From
`engine/`, one at a time:

```bash
cp crates/shell/src/sign_in.rs target/sign_in.rs.declared
sed 's/handler.capabilities.elicitation = Some(url_elicitation);/handler.capabilities.elicitation = None;/' target/sign_in.rs.declared > crates/shell/src/sign_in.rs
grep -c 'handler.capabilities.elicitation = None;' crates/shell/src/sign_in.rs
cargo test --release -p conduit-shell sign_in_command_opens_the_url
cp target/sign_in.rs.declared crates/shell/src/sign_in.rs
cargo test --release -p conduit-shell sign_in_command_opens_the_url
```

The `grep` prints 1. The first test command must fail at step 2, its error naming `-32021`: the
engine refuses a `sign-in.v1` that does not declare URL elicitation, before any part (IFC-049); a
failure for any other reason does not count. Report its message. After the restore the second
reports 1 passed. A warning that `url_elicitation` is unused in the first build is expected.
Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild the crate. Git ignores `target/`.

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
git status --short -- spec IN_FORCE.md tools packages conformance engine/crates/core engine/crates/loader engine/crates/conformance engine/crates/shell/src/server.rs engine/crates/shell/src/server/relay.rs engine/crates/shell/src/runner.rs engine/crates/shell/Cargo.toml engine/CONFORMANCE.md experiments docs/implementation docs/proposals
git diff --check
```

`promise_standing.py --check` ends `Errors: 0`. The `git status` command prints nothing.

Each of these prints 1. The `tr` check reads the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
grep -c '^pub mod sign_in;$' engine/crates/shell/src/lib.rs
tr -s ' \n' '  ' < engine/crates/shell/src/sign_in.rs | grep -c 'pub async fn run( directory: &Path, args: &\[String\], login: Option<&str>, repository: &Path, person: &mut dyn Person, ) -> Result<()>'
grep -c 'let tool = crate::author::route(directory, object, "sign-in.v1")?;' engine/crates/shell/src/sign_in.rs
grep -c 'handler.capabilities.elicitation = Some(url_elicitation);' engine/crates/shell/src/sign_in.rs
grep -c '"--login" if command == "sign-in"' engine/crates/shell/src/main.rs
grep -c 'fn sign_in_command_opens_the_url_and_polls_to_signed_in' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_command_declines_cancels_and_asks_for_a_login' engine/crates/shell/src/server_tests.rs
tr -s ' \n' '  ' < engine/README.md | grep -c 'signs an Agent or a Resource in through `serve` and root.s Conduit to its owner'
```

The second reads `run`'s signature as `rustfmt` wraps it, one parameter a line with a trailing
comma.

This prints the count the start check found plus one, `const COMMANDS: [(&str, &str, &str); 15]`
where it found 14:

```bash
grep -o 'const COMMANDS: \[(&str, &str, &str); [0-9]*\]' engine/crates/shell/src/help.rs
```

Each of these prints 0:

```bash
tr -s ' \n' '  ' < engine/README.md | grep -c 'The sign-in command is not built yet'
grep -c 'Terminal' engine/crates/shell/src/server_tests.rs
grep -c 'apiKey' engine/crates/shell/src/sign_in.rs
grep -c 'signInReturn' engine/crates/shell/src/sign_in.rs
git diff HEAD --stat -- engine/crates/core conformance spec | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the signatures of decisions 2 to 5's functions, the negative control's failure
message, and the findings of decision 11.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new file and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and the
merge does not wait for it. A fresh Codex verifier and a fresh read-only Opus reader read the
result before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes
this order's Record at the foot of this file when it merges.
