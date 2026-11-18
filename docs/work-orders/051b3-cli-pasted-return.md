# Work order 051b3 (C1b-3): `conduit sign-in` reads a pasted return with echo off, and `conduit call` carries none

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: this order adds no case, so `vectors` is an empty list, and its
tests go under `checks`. This order has network access only because the shell's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate and no
crate feature, and reach no other machine. Run one cargo command at a time. Never print a whole
large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential. Do not run `claude`, `codex` or any client, sign
in nowhere, and open no URL: the URLs below are test values that no program fetches.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s batch 1 gives "051b: the pending-action
commands `pending`, `sign-in` and `answer`. M. After 047 and 048b-join." The coordinator cuts 051b
into three, each after the one before, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264); the coordinator's
split, open to the owner's strike): 051b, `conduit pending`, `conduit answer` and the unanswered
hint, after 047c alone; 051b-2, `conduit sign-in`, which sends `sign-in.v1` declaring URL
elicitation, shows the engine's message and the full URL, asks consent, opens the browser and
carries the retries to the end, after 048b-join; and this order, 051b-3, the carried return from
the command line, after 051b-2, and so after 048a-8, whose redirect return it carries. A slot's
sign-in under [ruling 82](../proposals/ruling-82/README.md#the-decisions), point 3, follows this
order, and so does 050o.

**What it builds.** One behavior: the command line carries a sign-in's return only from what the
person types at `conduit sign-in`'s prompt, read with echo off. The full review's
[D53](../proposals/full-review/decisions.md) asks that "051b says that the command line offers the
paste when it runs on the node's machine and the browser is elsewhere", which is
[ruling 75](../proposals/ruling-75/README.md#the-decisions)'s C4: "`conduit` runs on the Mac in
v0. From the Windows computer or a phone, the person uses SSH and pastes", and A5's case 2
([FR-428](../proposals/full-review/findings/E04-signin-isolation.md#fr-428)): "The owner runs
`conduit` on the Mac over SSH from the Windows desktop, opens the URL in that desktop's browser and
pastes the final URL with echo off". PND-013's `observable`: "An Interface carries a return only
from a loopback listener it opened for that one login, or what the person typed, never model
output. It opens a listener only where it knows it does not run on the machine of the node
signInLogin names [...] Where it cannot tell, or cannot bind the port, it opens none and asks the
person to paste." IFC-049: `signInReturn` "holds {value, via} in an Interface's retry _meta, via
being listener or paste", and `signInLogin`'s `acceptsReturn` "is true exactly where PND-013's
first check passes". Order 117's line says only "051b-3, the carried return from the command line
(D53), after 051b-2"; the coordinator's split, open to the owner's strike, gives it three parts,
which are one behavior here: the paste is read only where `acceptsReturn` is true and the person
is at a terminal; the command line
opens no listener, since in this build it cannot tell which machine a node is, which PND-013 answers
with the paste; and `conduit call`, whose arguments a script or a model may write, refuses to carry
a return at all. No promise changes.

**The targeted review's sentence, which this order carries.**
[Section 6](../proposals/review-signin-2026-11-16/README.md#051b) of the targeted review says:
"Nothing from this review changes 051b: dec-34's command-line paste is the `carried: true` route
under PND-013 and serves no Claude Agent's sign-in, whose on-node delivery passes through no Conduit
process". Decision 2 builds it, since a Claude login states `carried: false`, so its
`acceptsReturn` is false and no paste is asked; decision 10 writes it in `engine/README.md`.
[Ruling 83](../proposals/ruling-83/README.md) touches only the Agent packages and their `apiKey`,
and nothing here reads, sets or names `apiKey`.

**What the orders before it bring, which this order assumes.** Orders 048a-5, 048a-6,
048b-relay1, 048b-join and 051b are written, and so are 048a-5b to 048a-5h, 048a-7, 048a-8,
048b-relay2 and 051b-2, beside this one; the coordinator read their texts, and each assumption
below is taken from them and named, so that the coordinator checks it again against their Records
and amends this order before dispatch where one differs. Where you find at the start that one
merged otherwise, apply the nearest sound reading of each decision below to what merged, and say
so; where an order's Record names a function, a file, a prompt or a test otherwise than below, use
the Record's name and report it. `META/x` below is the key `x` under the prefix that
`spec/names.yaml` sets as `metaPrefix`.

- **A1, from 048b-relay1.** The server relays a sign-in's input request as an
  `InputRequiredResult` whose one input request has the key `url`, whose `requestState` the shell
  issues and checks, and whose `_meta` is the core's output `meta`; an accepted retry reaches the
  core as the `respond` input whose `meta` is the retry's whole request `_meta`, so a
  `META/signInReturn` the retry carries reaches the core with no shell change.
- **A2, from 048a-5e and 048a-7.** Each `sign-in.v1` `InputRequiredResult`, and its final result,
  holds in its `_meta` `META/signInLogin` `{name, return, node, acceptsReturn}`, `node` naming the
  instance's node as `object.v1` names an object; after a retry that carried a return it also holds
  `returned` (`delivered`, `undelivered` or `refused`) and, where PND-013 gives them, `reason` or
  `error`; a refused return's `reason` is one of `not-accepted`, `already-returned`, `malformed`
  (048a-7), `address-mismatch`, `state-mismatch` and `issuer-mismatch` (048a-8). After a refused
  return polling goes on, so the result is another `InputRequired` (048a-7).
- **A3, from 048a-7 and 048a-8.** A retry carrying `META/signInReturn` `{"value": V, "via":
  "paste"}` with `accept` reaches PND-013's checks; a redirect value that passes is rebuilt and
  passed to `sign-in-hook.v1` in its retry as `{"value": V'}`, and for `R` below the rebuilt value
  equals `R`.
- **A4, from 048b-join.** `server_tests.rs` holds `sign_in_relays_through_the_interface_and_carries_the_return`,
  its executor `SignInScript`, its URL `U` for the login `browser` and its return
  `R` = `http://127.0.0.1:8400/callback?code=join-code-0001&state=join-state-0123456789abcdef`,
  whose rebuilt value equals `R`. `SignInScript` keeps every `dispatch` intent it receives and one
  flag `returned`, false at first; it answers `account-hook.v1` with `account` `null` while
  `returned` is false and `primary-account` once it is true, `sign-in-hook.v1` with no
  `requestState` with an `InputRequiredResult` holding `U` and `requestState` `hook-state-1`, and
  `sign-in-hook.v1` with a `requestState` by setting `returned` true (its decision 6). The login
  `browser` of its store has `acceptsReturn` `true`.
- **A5, from 051b-2.** The new file `engine/crates/shell/src/sign_in.rs` holds
  `pub trait Person { fn ask(&mut self, prompt: &str) -> Result<Option<String>>; fn tell(&mut self, line: &str); fn open(&mut self, url: &str) -> Result<()>; }`,
  its implementation `pub struct Terminal;` (`tell` and the prompt on standard error, `ask` a line
  of standard input, `open` the platform's opener), `connect`, and the sign-in loop
  `async fn rounds(service: &RunningService<RoleClient, ClientConfig>, tool: &str, arguments: Value, meta: Option<Value>, label: &str, person: &mut dyn Person) -> Result<Value>`
  (its decision 4), called by
  `pub async fn sign_in(directory: &Path, object: &Value, login: Option<&str>, prefix: &str, person: &mut dyn Person) -> Result<Value>`,
  called in turn by `pub async fn run(directory: &Path, args: &[String], login: Option<&str>, repository: &Path, person: &mut dyn Person) -> Result<()>`,
  which reads the prefix from `spec/names.yaml`. `main.rs` calls
  `conduit_shell::sign_in::run(&store, &positional, login.as_deref(), &repository, &mut person)`
  with `Terminal` (its decision 1). `connect`'s `ClientConfig` declares URL elicitation and the
  extension `format!("{prefix}implementation-interface")` with `{"revision": "v0"}` (its decision
  3). For each `InputRequired` result, `rounds` reads the action with `url_action`, tells the
  message and the URL, tells
  `conduit sign-in: this login's return can be carried back through the Interface, but this command does not carry one (order 051b-3): ...`
  where `signInLogin`'s `acceptsReturn` is `true`, and decides the answer `A`: where the URL is not
  yet opened, it asks through `person.ask`
  `"Open this URL in your browser? Type y to open it, or n to decline: "` until `consent` gives
  other than `Again`, and opens the URL on `Open`; then, and wherever the URL was already opened,
  `A` is `waited` of `person.ask("Press Enter once you have signed in, or type n to stop: ")`. The
  retry is `{"name": tool, "arguments": arguments, "requestState": action.state,
  "inputResponses": {action.key: {"action": A}}}`, with no `_meta` of its own. In
  `server_tests.rs`, 051b-2 adds
  `struct Scripted { answers: VecDeque<Option<String>>, prompts: Vec<String>, told: Vec<String>, opened: Vec<String> }`,
  implementing `Person` and opening nothing, the executor `PollScript`, and two tests run
  in-process with `Scripted` in the library's target: "T2" below,
  `sign_in_command_opens_the_url_and_polls_to_signed_in`, and
  `sign_in_command_declines_cancels_and_asks_for_a_login`. Its help description (in `help.rs`)
  ends "Carries no return from another machine.", and its sentence of `engine/README.md` "It
  carries no return from another machine (order 051b-3)." No test of 051b-2 runs the built
  `conduit sign-in`, and `Terminal::open` has no switch that keeps the browser shut, so no test of
  this order runs the built command either.
- **A6, from 051b.** `main.rs`'s `main` prints an error as `conduit {command}: {error}` and exits
  1; `author.rs` has a `mod tests`.

**What the shell already does, so that you do not redo it.** Line numbers are those of `main` at
`3c761dd9`, before every order named above; find each place by the text quoted with it.

- `main.rs`'s `run`, for `call` and `tools`, parses the second positional argument as the
  arguments JSON (`let arguments = positional`, ending `.unwrap_or(json!({}));`, near `:143`) and
  then sends it through `conduit_shell::server::client`.
- `server.rs`'s `client` (`:811`) builds the request from `{"name": tool, "arguments": arguments}`
  alone, so `conduit call` sends no `_meta` of its own; a key named `signInReturn` inside the
  arguments is an argument, which no promise reads as a return.
- `boundary::value` (`boundary.rs:13`) reads one YAML document as JSON, and `boundary::context`
  reads `metaPrefix` from `spec/names.yaml` with it.
- The shell's crate depends on `libc` 0.2 already (`engine/crates/shell/Cargo.toml`), and
  `runner.rs` calls it in `unsafe` blocks.
- No shell source calls `tcgetattr` or `tcsetattr`, and none reads a line from a terminal with
  echo off.

**When this order starts.** Orders 051b-2 and 048b-join, and every order before each, must be
merged to `main`. Under the owner's approval of stacked dispatch, the coordinator may instead start
this order from 051b-2's verified branch, saying so in a dispatch note, which relaxes only the
first three checks below, the Records of 051b, 051b-2 and 048b-join, and each of them only where
the branch the note names holds that order: 051b-2's own dispatch may have been stacked on 051b's
and 048b-join's branches in the same way. The code checks after them still guard what each order
brings. Before you change anything, check from the repository root that you find what they give,
and stop, change nothing and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/051b-cli-pending-and-answer.md
cat docs/work-orders/051b2-*.md | grep -c '^## Record'
grep -c '^## Record' docs/work-orders/048b-join-build-sign-in-through-the-interface.md
grep -c '"sign-in"' engine/crates/shell/src/main.rs
grep -c '"pending"' engine/crates/shell/src/main.rs
grep -rl 'acceptsReturn' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"delivered"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"state-mismatch"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'fn sign_in_relays_through_the_interface_and_carries_the_return' engine/crates/shell/src/server_tests.rs
grep -c 'struct SignInScript' engine/crates/shell/src/server_tests.rs
test -e engine/crates/shell/src/sign_in.rs; echo "file $?"
grep -c 'pub trait Person' engine/crates/shell/src/sign_in.rs
grep -c 'async fn rounds(' engine/crates/shell/src/sign_in.rs
grep -c 'implementation-interface' engine/crates/shell/src/sign_in.rs
tr -s ' \n' '  ' < engine/crates/shell/src/sign_in.rs | grep -c 'this command does not carry one'
grep -c 'struct Scripted' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_command_opens_the_url_and_polls_to_signed_in' engine/crates/shell/src/server_tests.rs
grep -c 'signInReturn' engine/crates/shell/src/sign_in.rs
grep -c 'signInReturn' engine/crates/shell/src/author.rs
grep -c 'signInReturn' engine/crates/shell/src/main.rs
grep -rl 'tcsetattr' engine/crates/shell/src | wc -l | tr -d ' '
grep -c 'fn refuse_sign_in_return' engine/crates/shell/src/author.rs
grep -o 'const COMMANDS: \[(&str, &str, &str); [0-9]*\]' engine/crates/shell/src/help.rs
```

They print `1` (051b's Record), `1` (051b-2's Record) and `1` (048b-join's Record), the three
that a dispatch note may relax; a number other than `0` (051b-2's command, A5), a number other than `0`
(051b's command, A6), a number other than `0` (048a-5e's `acceptsReturn`, A2), a number other than
`0` (048a-7's `returned`, A2), a number other than `0` (048a-8's redirect check, A3), `1` and `1`
(048b-join's test and script, A4); `file 0`, `1`, `1`, a number other than `0`, `1`, `1` and `1`
(051b-2's file, `Person`, `rounds`, the extension `connect` declares, its not-carry line, `Scripted`
and T2, A5); `0`, `0`, `0`, `0` and `0`; and `const COMMANDS: [(&str, &str, &str); 15]`
(051b-2's help, A5), or 15 plus the commands an order merged beside added (report which). Note the
value it prints: the last check of "Done when" prints it again.

Then, from `engine/`, run T2 with
`cargo test --release -p conduit-shell --lib sign_in_command_opens_the_url_and_polls_to_signed_in`:
it must report 1 passed and 0 failed, or stop and report it. Then run
`cargo test --release -p conduit-shell --lib author::tests` and note its count of passed tests, `N`
below. Report the line of `connect` that declares the extension, and the line of `rounds` where it
sends a retry. Report as well, as a count only and not as a reason to stop, what
`tr -s ' \n' '  ' < engine/README.md | grep -c 'conduit sign-in --store'` prints (decision 10).
Orders that land beside this one (050o, 051c and others): whichever lands second keeps both
changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-013 decides that an Interface carries a return only from its own
listener or what the person typed, never model output, and opens a listener only where it knows it
does not run on the node's machine, asking the person to paste where it cannot tell; IFC-049
decides `signInReturn`'s form, its `via`, and `signInLogin`'s `acceptsReturn`; PND-014 decides
that the value is kept nowhere by the engine; D53 and ruling 75's C4 decide the paste's case; and
the targeted review's section 6 decides that the paste is the `carried: true` route and serves no
Claude Agent's sign-in. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the cut of 051b into three and this part's place
after 051b-2; the three parts of 051b-3 in that split, read as one behavior; asking only where
`acceptsReturn` is true and standard input is a terminal, and reading a line from a pipe as not
what the person typed; asking through a method of 051b-2's `Person`, after the Enter prompt and
before the accepted retry; asking once, and again only after a refused return that a later one may
still pass, and telling the carried-return line only on those rounds; passing the prefix to
`rounds`; opening no listener in this build; refusing in
`conduit call` any arguments that name `signInReturn`, before anything is sent; the functions,
their names and signatures; every text the command line prints, and the three texts of 051b-2 that
this order replaces; the five unit tests, the two other tests and the negative control, and skipping the two
pseudo-terminal tests where the sandbox refuses one; and the
sentence and the paragraph of `engine/README.md`. Apply them as written and report them under `decisions` as the
coordinator's.

**Read first:**

1. In `spec/`: PND-010, PND-013 and PND-014 (`spec/promises/pending.yaml`); IFC-036 and IFC-049
   (`spec/interface/implementation.yaml`); `sign-in.v1` and `sign-in-hook.v1` in
   `spec/core/operations.yaml`; the row "node" of `spec/definitions.md`.
2. Orders 051b and 051b-2 whole, with their Records; order 048b-join, its decision 6 and Record;
   the Records of 048a-5e, 048a-7 and 048a-8.
3. In `engine/crates/shell/src/`: `main.rs` (`run`'s handling of `call`), `author.rs`,
   `server.rs` (`client`), `boundary.rs` (`value`), `sign_in.rs` whole, `help.rs`, and in
   `server_tests.rs` `Scripted`, `SignInScript`, T2 and the helpers T2 uses.

**Size: S-M**: about 0.3 to 0.4k changed lines of Rust, about half of them tests, one sentence of
`help.rs`, and one sentence and one paragraph of `engine/README.md`.

## The decisions it applies

1. **The prefix, and the extension the retry needs.** `rounds` gains one parameter,
   `prefix: &str`, right after `label`, and `sign_in` passes its own `prefix` to it; nothing else
   in either signature changes, and `PREFIX` below is that value. A retry that carries
   `META/signInReturn` carries a key under the prefix, which the shell's HTTP layer refuses with
   `-32021` under IFC-036 unless the client declares the extension
   `format!("{PREFIX}implementation-interface")`. 051b-2's `connect` declares it with
   `{"revision": "v0"}` (A5), so this is a check only: change nothing in `connect`, and report the
   line that declares it.

2. **When the paste is asked.** `pub fn accepts_return(prefix: &str, meta: &Value) -> bool`, in
   `author.rs`, is true exactly where `meta`'s member named `format!("{prefix}signInLogin")` is an
   object whose member `acceptsReturn` is the JSON value `true`.

   In `sign_in.rs`, `Person` gains one method,
   `fn paste(&mut self, node: &str) -> Result<Option<String>>`: it asks the person for the address
   the browser's page ended on, for the login of node `node`, and gives the line typed, or `None`
   where nothing was read from the person.

   `rounds` keeps one `bool`, `asking`, true at the start, which also decides whether decision
   10's told line is told on a round (decision 10). Where 051b-2's answer `A` is
   `"accept"`, after the Enter prompt that gave it and right before that retry is sent, with
   `meta` the `_meta` of the `InputRequired` result it answers, read from that result's serialized
   value (`Value::Null` where absent):
   - where `accepts_return(PREFIX, &meta)` is false, or `asking` is false: ask nothing, and send
     the retry as 051b-2 does;
   - where both are true: set `asking` false, call `person.paste(NODE)?`, and send the retry as
     decision 4 says, `typed` being the line it gave, or the empty string where it gave `None`.

   The paste prompt so comes after the Enter prompt, not in place of it: consent, then Enter, then
   the paste. `NODE` is the member `name` of that object's member `node` where it is a string, and
   `the engine named` otherwise. A retry whose `A` is `"decline"` or `"cancel"`, and a result that
   is not `InputRequired`, ask nothing. A Claude login states `carried: false`, so its
   `acceptsReturn` is false and the command line never reads its code or return (the targeted
   review's section 6).

   `Terminal`'s `paste` is the one place that reads the process's own standard input for a
   return, so that `rounds` itself calls neither `isatty` nor `ask_paste` and a test's person never
   touches the real terminal: where `libc::isatty(libc::STDIN_FILENO) == 1`, it gives
   `Ok(Some(ask_paste(node)?))`; otherwise it writes to standard error the one line
   `conduit sign-in: standard input is not a terminal, so no return is read here; PND-013 takes only what the person types. The login completes only where its program on node NODE takes the return itself`,
   `NODE` being `node`, and gives `Ok(None)`.

   In `server_tests.rs`, `Scripted` gains two fields, `pastes: VecDeque<Option<String>>` and
   `pasted: Vec<String>`, and implements `paste`: it pushes `node` onto `pasted` and gives the
   front of `pastes`, popped, or `None` once none is left; it reads no terminal. Each place in
   051b-2's tests that builds a `Scripted` gives it `pastes: VecDeque::new()` and
   `pasted: Vec::new()` (where 051b-2 built it through a constructor or `Default`, extend that
   instead), and nothing else in those tests changes: their person pastes nothing, so their
   retries are 051b-2's and their prompts are unchanged.

3. **The line, read with echo off.** In `author.rs`:
   - `pub struct EchoOff { fd: std::os::fd::RawFd, saved: libc::termios }`, and
     `pub fn echo_off(fd: std::os::fd::RawFd) -> std::io::Result<Option<EchoOff>>`: where
     `libc::isatty(fd)` is not 1, it gives `Ok(None)` and changes nothing; otherwise it reads the
     terminal's settings with `libc::tcgetattr`, keeps them as `saved`, clears echo with the one
     line `settings.c_lflag &= !libc::ECHO;`, written exactly so, applies them with
     `libc::tcsetattr(fd, libc::TCSANOW, &settings)`, and gives `Ok(Some(..))`; a failing call gives
     `Err(std::io::Error::last_os_error())`. `impl Drop for EchoOff` applies `saved` with
     `libc::tcsetattr(self.fd, libc::TCSANOW, &self.saved)` and ignores its result.
   - `pub fn ask_paste_on(fd: std::os::fd::RawFd, input: &mut dyn std::io::BufRead, output: &mut dyn std::io::Write, node: &str) -> Result<String>`:
     in this order, `let guard = echo_off(fd)`, an error mapped to a `String` that holds no input;
     then write to `output`, with no line break after it and flushed,
     `conduit sign-in: if the browser you signed in with runs on another machine than node NODE, paste the full address its page ended on and press Enter (the input is hidden); otherwise press Enter alone: `,
     `NODE` being `node`; then read one line from `input` with `read_line`; then drop `guard`,
     which restores echo; then write one line break to `output`. It gives the line with ASCII
     whitespace trimmed at both ends (`trim_matches(|c: char| c.is_ascii_whitespace())`), and the
     empty string at end of input. Echo is cleared before the prompt is written, so that nothing
     the person pastes after reading it is echoed.
   - `pub fn ask_paste(node: &str) -> Result<String>`: `ask_paste_on` with
     `libc::STDIN_FILENO`, `std::io::stdin().lock()`, `std::io::stderr()` and `node`.

4. **The retry's `_meta`, and the value's handling.**
   `pub fn paste_meta(prefix: &str, meta: &Value, typed: &str) -> Option<Value>` gives
   `{format!("{prefix}signInReturn"): {"value": typed, "via": "paste"}}` where
   `accepts_return(prefix, meta)` is true and `typed` is not empty, and `None` otherwise. `typed`
   is passed as `paste` gave it, with no other change: PND-013's checks are the engine's. Where it
   gives `Some(M)`, `rounds` sends the retry with `M` as the retry's request `_meta` (051b-2's
   retry carries no `_meta` of its own), and with `{"url": {"action": "accept"}}` and the
   `requestState` as before; where it gives `None`, the retry is 051b-2's, unchanged. The typed
   line is held in one local `String` and in `M`, both dropped once that retry's result is read.
   It is never formatted with `{:?}` or `Display`, and no new type this order adds that holds it
   derives `Debug`; it goes into no message, error, log line, `tell` or standard output. The
   command line reads a return from nowhere else: no argument, option, environment variable or
   file.

5. **What the person is told.** `pub fn returned_line(prefix: &str, meta: &Value) -> Option<String>`,
   with `L` = `meta[format!("{prefix}signInLogin")]`, gives, by `L["returned"]`:
   - `"delivered"`: `conduit sign-in: the pasted return was delivered to the login's program`;
   - `"undelivered"`: `conduit sign-in: the pasted return was passed on, but the login's program did not take it (undelivered); the result's diagnostics say why`;
   - `"refused"` with `L["reason"]` `already-returned` or `not-accepted`:
     `conduit sign-in: the pasted return was refused (REASON); nothing of it was used, and this sign-in takes no other`;
   - `"refused"` otherwise:
     `conduit sign-in: the pasted return was refused (REASON); nothing of it was used, and you may paste again`,
     `REASON` being `L["reason"]` where it is a string and `no reason given` otherwise;
   - anything else, or absent: `None`.

   After the result of a retry that carried `M`, read its `_meta` as JSON, whether the result is
   `InputRequired` or final, and `tell` the person `returned_line`'s text where it gives one
   (`Terminal` writes it as one line on standard error). Where `L["returned"]` is `"refused"`,
   `L["reason"]` is neither `already-returned` nor `not-accepted`, and the result is
   `InputRequired`, set `asking` true again, so that decision 2 asks before that result's accepted
   retry. After any other result, `asking` stays false. In this command's own flow the first
   return it sends is the invocation's first, so `already-returned` and `not-accepted` are not
   expected; their text and the end of asking are there so that nothing tells the person to paste
   again where no return can pass.

6. **No listener.** The command line opens no loopback listener in this build: it reaches the
   engine through `serve`, and nothing it holds tells it which machine the node that
   `signInLogin` names is, so under PND-013 it cannot tell and asks the person to paste. Add no
   socket, port binding or HTTP server to the command line: not in `author.rs`, `sign_in.rs` or
   `main.rs`.

7. **`conduit call` carries no return.**
   `pub fn refuse_sign_in_return(arguments: &Value) -> Result<()>`, in `author.rs`, gives an
   error where `arguments` is an object with a member whose name is `signInReturn` or ends with
   `/signInReturn`, or with a member `_meta` that is an object holding such a member; it gives
   `Ok(())` otherwise, and reads no value. The error is
   `this command carries no sign-in return: a return travels only in conduit sign-in's retry, from what the person types at its prompt (PND-013); nothing was sent`.
   In `main.rs`'s `run`, right after the statement `let arguments = positional` that ends
   `.unwrap_or(json!({}));`, add the one line
   `conduit_shell::author::refuse_sign_in_return(&arguments)?;`, so that the refusal comes before
   `server::client` is called and nothing is sent. `main` prints it as `conduit call: ` and the
   error (A6).

8. **Promise IDs in the shell's sources.** The new texts name `PND-013`: decision 7's error in
   `author.rs`, and decision 2's line in `sign_in.rs`. Run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks, and report PND-013's row of
   `engine/STANDING.md` before and after, as you find them. This order claims no change to that
   row: 048b-join's and 048a-7's tests already name PND-013 in the shell's sources, so its Engine
   source column may hold `shell` at the start.

9. **The unit tests**, in `author.rs`'s `mod tests`, the prefix written as `example.invalid/`,
   since the code under `engine/` writes none of the names of `spec/names.yaml`:
   - `paste_meta_only_where_the_return_is_accepted`: with `A` =
     `{"example.invalid/signInLogin": {"name": "browser", "return": "redirect", "node": {"kind": "Resource", "name": "local"}, "acceptsReturn": true}}`,
     `paste_meta("example.invalid/", &A, "http://127.0.0.1:8400/callback?code=c&state=s")` gives
     `Some({"example.invalid/signInReturn": {"value": "http://127.0.0.1:8400/callback?code=c&state=s", "via": "paste"}})`;
     it gives `None` for `typed` `""`, for `A` with `acceptsReturn` `false`, with
     `acceptsReturn` `"true"`, and with no `acceptsReturn`, for `{}`, and for `A` read with the
     prefix `other.invalid/`.
   - `returned_line_names_each_outcome`: for `L` holding `"returned": "delivered"`, the text
     contains `was delivered`; for `"returned": "refused", "reason": "state-mismatch"`, it contains
     `refused (state-mismatch)` and `you may paste again`; for `"returned": "refused"` alone,
     `refused (no reason given)` and `you may paste again`; for `"returned": "refused", "reason":
     "already-returned"`, `refused (already-returned)` and `takes no other`, and not `paste again`;
     for `"returned": "undelivered"`, `(undelivered)`; for `L` with no `returned`, and for `{}`,
     `None`.
   - `call_arguments_carry_no_sign_in_return`: `refuse_sign_in_return` gives an error for
     `{"signInReturn": {"value": "call-code-0001"}}`, for
     `{"_meta": {"example.invalid/signInReturn": {"value": "call-code-0001"}}}` and for
     `{"example.invalid/signInReturn": 1}`, each containing `nothing was sent` and not
     `call-code-0001`; and `Ok(())` for `{"instance": {"kind": "Agent", "name": "signed"}}`, for
     `{"_meta": {}}`, for `{"note": "signInReturn"}` and for `[]`.
   - A helper `open_pty()` in `mod tests` opens a pseudo-terminal with
     `libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY)`, `libc::grantpt`, `libc::unlockpt` and
     `libc::ptsname`, opens its slave with `libc::open(path, libc::O_RDWR | libc::O_NOCTTY)`, and
     gives the master and the slave as `std::os::fd::OwnedFd`, so that each is closed when
     dropped.
   - `echo_off_hides_input_on_a_terminal_and_restores_it`: with `open_pty()`'s slave, its `ECHO`
     flag, read with `tcgetattr`, is set; inside `echo_off(slave)`, which gives `Some`, it is
     clear; after the guard is dropped it is set again. `echo_off` on the read end of a
     `libc::pipe` gives `Ok(None)`. Close every descriptor it opened.
   - `ask_paste_on_a_terminal_echoes_nothing_typed`: with `R` =
     `http://127.0.0.1:8400/callback?code=join-code-0001&state=join-state-0123456789abcdef` and
     `open_pty()`'s master and slave. In a spawned thread, call
     `ask_paste_on(fd, &mut input, &mut output, "local")`, `input` a `std::io::BufReader` over a
     `std::fs::File` made from `slave.try_clone()`, `fd` that file's raw descriptor, and `output`
     a `std::fs::File` made from another `slave.try_clone()`, and give back its result. In the
     test's own thread, read the master with `libc::read` into a buffer, each read after
     `libc::poll` on the master finds input within 100 ms, until the buffer holds
     `paste the full address` (for at most 10 s); then write `R` and `\n` to the master; join the
     thread; then read the master the same way until one poll finds nothing for 500 ms. The
     thread gave `Ok(R)`; the buffer holds `paste the full address` and does not hold
     `join-code-0001`, so the terminal echoed nothing typed; and the slave's `ECHO` flag, read
     with `tcgetattr`, is set again. Every assertion's message carries the buffer's text less any
     line holding `join-code-0001`. Close every descriptor it opened.

10. **The texts that said the command carries no return, and `engine/README.md`.** 051b-2's three
    texts that say this command carries no return hold only until this order merges (051b-2's
    decision 11); replace each, and change nothing else in their files:
    - in `sign_in.rs`, the line `rounds` tells where `acceptsReturn` is `true`, beginning
      `conduit sign-in: this login's return can be carried back through the Interface, but this command does not carry one (order 051b-3)`,
      becomes
      `conduit sign-in: this login's return can be carried back through the Interface: after you press Enter, this command asks for the address the browser's page ended on, reads it with echo off, and sends it only where you paste one.`,
      and `rounds` tells it only where `asking` (decision 2) is true as well, so that it is told
      exactly on the rounds whose accepted retry decision 2 asks before: on the first, and on one
      after a refused return that a later one may still pass, never on a later polling round;
    - in `help.rs`, the last sentence of `sign-in`'s description, `Carries no return from another machine.`,
      becomes
      `Where the login takes a carried return, asks for the address the browser's page ended on, read with echo off: once, and again after a refused return that a later one may pass.`;
      `COMMANDS` and `DESCRIPTIONS` keep their count as the start's last check found it (15 where
      051b-2 left 15), and `tests/cli.rs`'s list `verbs` is unchanged;
    - in `engine/README.md`, the sentence `It carries no return from another machine (order 051b-3).`
      becomes `Where the login takes a carried return, it asks for it, as the next paragraph says.`

    Then add this paragraph to `engine/README.md` right after the last paragraph that contains
    `conduit sign-in --store`, read with line breaks as spaces; where none does, right after the
    paragraph that begins "`conduit pending --store DIR` lists" (order 051b). Change nothing else
    in the file.

    ```text
    Where a sign-in's login takes a carried return (`acceptsReturn` in its `signInLogin`),
    `conduit sign-in` asks once, on standard error and after you press Enter, for the address
    the browser's page ended on, and reads the line from the terminal with echo off: ruling 75
    C4's case, `conduit` run on the node's machine over SSH while the browser runs on another.
    An empty line sends no return, and a refused return is asked again where a later one may
    still pass. It reads a return only from a terminal and opens no loopback listener, since in
    this build it cannot tell which machine a node is (PND-013). This paste is the
    `carried: true` route of PND-013 and serves no Claude Agent's sign-in: a Claude login
    states `carried: false`, and its delivery on the node passes through no Conduit process.
    `conduit call` carries no sign-in return: it refuses arguments that name `signInReturn` and
    sends nothing.
    ```

11. **The tests of the command line.**

    *`call_refuses_a_sign_in_return`*, in `engine/crates/shell/tests/cli.rs`: initialize a store
    with the file's `command(&store, "init", &["--url", &url])`, `url` being
    `http://127.0.0.1:{port}/mcp` and `port` taken from a `std::net::TcpListener` bound to
    `127.0.0.1:0` and then dropped, as `serve_management_commands_and_fixed_audience` in that file
    does, so that no run reaches an engine that may serve the default URL on the machine; then run
    the `conduit` binary (`env!("CARGO_BIN_EXE_conduit")`) as
    `call --store STORE group.sign-in.v1 ARGS`, with `ARGS`
    `{"instance": {"kind": "Agent", "name": "signed"}, "_meta": {"example.invalid/signInReturn": {"value": "http://127.0.0.1:8400/callback?code=call-code-0001&state=call-state-0123456789abcdef", "via": "paste"}}}`.
    It exits non-zero; its standard error contains `carries no sign-in return` and
    `nothing was sent`, and contains neither `call-code-0001` nor `engine session initialization`;
    its standard output does not contain `call-code-0001`. Run it again with `ARGS`
    `{"instance": {"kind": "Agent", "name": "signed"}, "signInReturn": {"value": "call-code-0001"}}`,
    with the same assertions. Run it a third time with `ARGS`
    `{"instance": {"kind": "Agent", "name": "signed"}}`: it exits non-zero, since no engine serves
    the store, and its standard error contains `engine session initialization`, so that this run
    is shown to reach the send that the first two refused before, and does not contain
    `carries no sign-in return`. Every assertion's message carries the command's standard error.
    Add `libc` to no manifest: the test does not use it.

    *`sign_in_carries_a_pasted_return_through_the_engine`*, in `server_tests.rs` beside T2, a
    `#[tokio::test(flavor = "multi_thread", worker_threads = 4)]` in the library's target, on a
    store of its own built as T2 builds its own (`fixed_case_shell` of
    `conformance/conventions/requests/sign-in-hold` through step 9, root issued a client token
    with `issue_client_token`, `prefix` being `shell.context.meta_prefix`), with one difference:
    its executor is 048b-join's `SignInScript` (A4), wrapped in `ServingExecutor::new` and served
    with `server::serve`, in place of `PollScript`, so that `account-hook.v1` finds no account
    until the hook's retry carrying a `requestState` arrives, and `primary-account` right after.
    The person is a `Scripted` with the answers `[Some("y"), Some("")]` and the pastes
    `[Some(R)]`, `R` as in A4. It runs in-process and opens nothing.
    1. `crate::sign_in::sign_in(temp, &json!({"kind": "Agent", "name": "signed"}), None, &prefix, &mut person).await.unwrap()`
       gives `F`: `F`'s `isError` is not `true`, its `structuredContent.account` is
       `primary-account`, its `_meta` `META/outcome` is `succeeded`, and its `_meta`
       `META/signInLogin`'s `returned` is `delivered`.
    2. `person.prompts` is exactly the consent prompt, then the Enter prompt (A5); `person.opened`
       is exactly `[U]`; `person.pasted` holds exactly one entry, the `name` of `F`'s
       `META/signInLogin`'s `node` where it is a string, and `the engine named` otherwise.
    3. `person.told` holds `conduit sign-in: the pasted return was delivered to the login's program`
       once and decision 10's line for `sign_in.rs` once, and no line of it holds
       `join-code-0001`.
    4. The script's intents, by `call.params.name`, are `account-hook.v1`, `sign-in-hook.v1`
       with no `requestState`, `sign-in-hook.v1` whose `call.params.requestState` is
       `hook-state-1`, whose `call.params.inputResponses` holds `login` with `action` `accept` and
       whose `call.params._meta` `META/signInReturn`'s `value` is `R` (A3), and then
       `account-hook.v1`, and no other.
    5. `sign_in_holds` has no entry for `signed`'s ID.

    Where T2's fixture states the login otherwise than A4, build `R` from its relayed
    `redirect_uri` and state in the same form, keep `join-code-0001` as its code, and report it.
    Before running it, check that no other test's name in `engine/crates/shell/` begins
    `sign_in_carries_a_pasted`. If it fails because a reply differs from what it states, stop and
    report the step and the reply, less any part holding `join-code-0001`: do not change the
    expectations to pass it. Every assertion's message carries `person.told` and `person.prompts`.
    That the paste is hidden on a real terminal is decision 9's last test, on a pseudo-terminal:
    this test drives the same loop through `Scripted`, which reads no terminal.

12. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line, ruling's
    record, case, fixture, core source, `server.rs` or relay source changes; no listener
    (decision 6); no return read from a pipe, an argument or a file; no test runs the built
    `conduit sign-in`; no device code, no panel (051c), and no connection Conduit manages (050o).
    Report as findings, for the coordinator: the line where `rounds` sends the retry that carries
    `M`; the line of `connect` that declares the extension (decision 1); that a signal that ends
    the command while echo is off leaves the terminal without echo, since `Drop` does not run
    then; that in ruling 75 C4's case consent opens a browser on the node's machine, since 051b-2's
    consent prompt is the only route to the paste (`n` declines and ends the sign-in), and a
    browser opened there with a session at the provider could complete a login through the
    program's own listener with whatever account it holds: say what it did in your run, if
    anything, since no test of this order runs `Terminal::open`; the terminal's canonical line
    limit (`MAX_CANON`, or what `fpathconf(fd, _PC_MAX_CANON)` gives) on each platform the tests
    ran on, and whether a pasted value longer than it would be cut, since decision 3 clears only
    `ECHO` and leaves `ICANON` set, while PND-013's bound is 8192 bytes (do not change decision 3
    for it); and any place where PND-013, PND-014 or IFC-049 left you a choice this order does
    not settle.

## Scope

Change only `engine/crates/shell/src/author.rs` (decisions 2 to 5, 7 and 9),
`engine/crates/shell/src/sign_in.rs` (decisions 1, 2, 4, 5 and 10: `prefix` in `rounds`,
`Person::paste` and `Terminal`'s, the paste at the accepted retry, the told line and its
replacement), `engine/crates/shell/src/main.rs` (decision 7's one line),
`engine/crates/shell/src/help.rs` (decision 10's one sentence), `engine/crates/shell/tests/cli.rs`
(decision 11's first test), `engine/crates/shell/src/server_tests.rs` (`Scripted`'s `paste` and
fields, decision 2, and decision 11's second test) and `engine/README.md` (decision 10's sentence
and paragraph). `engine/STANDING.md` changes only as decision 8 says, written by the tool. Not
`spec/`, `IN_FORCE.md`, `tools/`, `packages/`, `conformance/`, the core, the loader, `server.rs`,
`server/relay.rs`, `runner.rs`, `lib.rs`, `engine/CONFORMANCE.md`, any `Cargo.toml`, the
experiments, `docs/implementation/` or `docs/proposals/`: report any defect as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --lib author::tests
cargo test --release -p conduit-shell --test cli call_refuses_a_sign_in_return
cargo test --release -p conduit-shell --lib sign_in_carries_a_pasted_return_through_the_engine
cargo test --release -p conduit-shell --lib sign_in_command_
cargo test --release -p conduit-shell --lib sign_in_relays_through_the_interface
cargo test --release -p conduit-shell
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports `N` plus 5 passed and 0 failed, `N` as the start found it: decision 9's five.
The fourth and fifth each report 1 passed; the sixth reports 2 passed (T2 and 051b-2's second
test), and the seventh 1 passed. Report the counts of the last two. If any other shell test fails,
stop and report it: do not change it. The threads are 4 because at 24 the runner ran out of memory
(order 071's Record).

Where the sandbox refuses a pseudo-terminal, so that `open_pty()` fails at `posix_openpt` in the
third command, do not stop and do not change either test: report the error, then run the third,
eighth and ninth commands again with
`-- --skip echo_off_hides_input_on_a_terminal_and_restores_it --skip ask_paste_on_a_terminal_echoes_nothing_typed`
after them (the third then reports `N` plus 3 passed and 0 failed), skip the negative control
below, run every other check of this order, and report decision 9's two pseudo-terminal tests and
the negative control as the admin's to run. A failure of either test for any other reason is a
failure: stop and report it.

Next, show that the tests fail where echo stays on. From `engine/`, one at a time:

```bash
cp crates/shell/src/author.rs target/author.rs.fixed
sed 's/settings.c_lflag &= !libc::ECHO;/settings.c_lflag \&= !0;/' target/author.rs.fixed > crates/shell/src/author.rs
grep -c 'settings.c_lflag &= !0;' crates/shell/src/author.rs
cargo test --release -p conduit-shell --lib author::tests::echo_off_hides_input_on_a_terminal_and_restores_it
cargo test --release -p conduit-shell --lib author::tests::ask_paste_on_a_terminal_echoes_nothing_typed
cp target/author.rs.fixed crates/shell/src/author.rs
cargo test --release -p conduit-shell --lib author::tests::echo_off_hides_input_on_a_terminal_and_restores_it
cargo test --release -p conduit-shell --lib author::tests::ask_paste_on_a_terminal_echoes_nothing_typed
```

The `grep` prints 1. The first two test commands must each fail: the first at its assertion that
`ECHO` is clear inside the guard, and the second at its assertion that the buffer does not hold
`join-code-0001`, since the terminal echoes what is typed; a failure for any other reason does not
count. After the restore the last two each report 1 passed. Restore with `cp`, not `mv`: a moved
file keeps its older modification time, so cargo would not rebuild the crate. Git ignores
`target/`. Where the sandbox refused a pseudo-terminal above, skip these commands, as that
paragraph says: the admin runs them.

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
git status --short -- spec IN_FORCE.md tools packages conformance engine/crates/core engine/crates/loader engine/crates/shell/src/server.rs engine/crates/shell/src/server engine/crates/shell/src/runner.rs engine/crates/shell/src/lib.rs engine/CONFORMANCE.md experiments docs/implementation docs/proposals
git diff --check
```

`promise_standing.py --check` ends `Errors: 0`. The `git status` command prints nothing.

Each of these prints 1. The `tr` check reads the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
grep -c 'pub fn accepts_return(prefix: &str, meta: &Value) -> bool' engine/crates/shell/src/author.rs
grep -c 'pub fn paste_meta(prefix: &str, meta: &Value, typed: &str) -> Option<Value>' engine/crates/shell/src/author.rs
grep -c 'pub fn returned_line(prefix: &str, meta: &Value) -> Option<String>' engine/crates/shell/src/author.rs
grep -c 'pub fn refuse_sign_in_return(arguments: &Value) -> Result<()>' engine/crates/shell/src/author.rs
grep -c 'pub fn ask_paste(node: &str) -> Result<String>' engine/crates/shell/src/author.rs
grep -c 'settings.c_lflag &= !libc::ECHO;' engine/crates/shell/src/author.rs
grep -c 'conduit_shell::author::refuse_sign_in_return(&arguments)?;' engine/crates/shell/src/main.rs
grep -c 'fn call_refuses_a_sign_in_return' engine/crates/shell/tests/cli.rs
grep -c 'fn sign_in_carries_a_pasted_return_through_the_engine' engine/crates/shell/src/server_tests.rs
grep -c 'fn paste(&mut self, node: &str) -> Result<Option<String>>' engine/crates/shell/src/server_tests.rs
tr -s ' \n' '  ' < engine/README.md | grep -c 'This paste is the `carried: true` route of PND-013 and serves no Claude Agent.s sign-in'
```

This prints 2, the trait's declaration and `Terminal`'s implementation, and this a number other
than 0, this order's own naming of PND-013 (decision 8):

```bash
grep -c 'fn paste(&mut self, node: &str) -> Result<Option<String>>' engine/crates/shell/src/sign_in.rs
grep -c 'PND-013' engine/crates/shell/src/author.rs
```

Each of these prints 0: no listener is added, the shell's sources outside `author.rs`,
`sign_in.rs` and `server_tests.rs` name no return value, and 051b-2's texts that said the command
carries no return are gone:

```bash
grep -lE 'TcpListener|bind\(' engine/crates/shell/src/author.rs engine/crates/shell/src/sign_in.rs engine/crates/shell/src/main.rs | wc -l | tr -d ' '
grep -c 'signInReturn' engine/crates/shell/src/main.rs
tr -s ' \n' '  ' < engine/crates/shell/src/sign_in.rs | grep -c 'this command does not carry one'
tr -s ' \n' '  ' < engine/crates/shell/src/help.rs | grep -c 'Carries no return from another machine'
tr -s ' \n' '  ' < engine/README.md | grep -c 'carries no return from another machine'
git diff HEAD --stat -- spec IN_FORCE.md engine/CONFORMANCE.md | wc -l | tr -d ' '
```

This prints the value the same command printed at the start, `const COMMANDS: [(&str, &str, &str); 15]`
where 051b-2 left 15 and no order beside it added a command:

```bash
grep -o 'const COMMANDS: \[(&str, &str, &str); [0-9]*\]' engine/crates/shell/src/help.rs
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, what the start found of 051b-2's names (A5) and any name its Record gives
otherwise, the signatures of decisions 2 to 5 and 7's functions and of `rounds` after decision 1,
PND-013's row before and after, the text the master gave in decision 9's
`ask_paste_on_a_terminal_echoes_nothing_typed` less any line holding `join-code-0001`, and the
findings of decision 12.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and the
merge does not wait for it. A fresh Codex verifier and a fresh read-only Opus reader read the
result before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes
this order's Record at the foot of this file when it merges.
