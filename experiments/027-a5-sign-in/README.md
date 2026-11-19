# 027 — A5: sign-in and isolation

Every result of this experiment is obtained under one operating-system user. Under one user, an Agent that runs code can read, use or replace another Agent's login and act as root's Interface. So no result here shows Agents kept apart. No dedicated macOS user is planned: isolation is to come later from a sandbox method, Docker first, and whether a result obtained inside one shows Agents kept apart is that design's to state ([ruling 84](../../docs/proposals/ruling-84/README.md), point 1). Stage 0 runs no Agent at all.

Stage 0 pre-registered on 2026-11-15 by work order 159; run twice, on 2026-11-15 and 2026-11-16: [results](results-2026-11-16.md).

Amended on 2026-11-15 by work order 167: how a signal stops the probe, the notice of a scratch login found, what the shim cannot catch, and the owner's browser check before the run. The steps, what is recorded, the pass conditions and the outcomes do not change.

Stage 0b pre-registered on 2026-11-16 by work order 195, [below](#stage-0b): part A, a read of Codex's public source and documentation, done on 2026-11-16, [results](results-2026-11-16-0b-source.md); part B, a watched run, not yet run.

Stage 1's first step pre-registered on 2026-11-17 by work order 197, [below](#stage-1-step-1-the-account-check): the live account check of ruling 82, point 1, not yet run; before it, a read of Codex's public source for `account/read`, done on 2026-11-17, [results](results-2026-11-17-account-read-source.md). The same order replaced, in the stages below, Codex's ID-token route and A5's "gates nothing" with ruling 82's route and gate.

The refresh-blocked probe pre-registered on 2026-11-19 by work order 208, [below](#stage-1-before-step-1-the-refresh-blocked-probe), before order 050m relies on its route: whether Codex 0.167.1's `account/read`, with refresh blocked for the check's process alone, ever writes the home's auth.json; run under the same order, on the Linux node, with a fabricated login, [results](results-2026-11-19-refresh-probe.md).

Amended on 2026-11-19 by the coordinator's change for [ruling 84](../../docs/proposals/ruling-84/README.md), point 1: the opening's last clause on isolation, which read "before or after the owner's dedicated macOS user (ruling 75 B3)", now says that no dedicated user is planned and that a sandbox method comes later. No step, record, pass condition or outcome changes, and each dated result keeps its opening as written.

Amended on 2026-11-19, before stage 1's first step ran, by the coordinator's change for [ruling 89](../../docs/proposals/ruling-89/README.md), [below](#amendment-of-2026-11-19-the-live-check-as-it-runs): by the coordinator's application of the readiness review's findings (ruling 89, point 1), open to the owner's strike, the check's Type keeps the name `codex.v2`, its client is `conduit sign-in`, a second browser tab is expected, and the engine runs with a call timeout of 1200 seconds; and, for ruling 89, point 5, a Claude scratch-home sign-in follows in the same sitting (through `conduit sign-in` and outside A5's gate by the coordinator's reading in ruling 89, open to the owner's strike).

## Four stages

- **0:** Before order 050 is written: two facts about the real clients, primary accounts only, no engine, nothing signed in; no owner time unless a window opens. The coordinator tells the owner first.
- **0b:** A follow-up to stage 0, before order 050 is written: how Codex's browser login opens a browser past the shim, what stops it, and whether Codex's status can name the account. Part A reads Codex's public source and documentation and runs no client; part B is a watched start of Codex's login in a scratch home, cancelled before any sign-in, at a time the owner picks. [Pre-registered below](#stage-0b).
- **1:** On the first build with sign-in, as the S1 rerun's rehearsal: one checked sign-in on a primary account; the package's account hook asks Codex itself, through its app-server's `account/read`, in its own home, never opening the login file, and never the coordinator. Its first step, after order 048b-join merges and before 0.2.0 merges, is the live check of [ruling 82](../../docs/proposals/ruling-82/README.md), point 1, [pre-registered below](#stage-1-step-1-the-account-check); the rest of stage 1 is pre-registered before it runs. About five owner minutes.
- **2:** After the rerun's pin: the second accounts' first sign-ins, four Agents and four homes, wrong then right accounts, checks after sign-in, undo and retirement, forced lapse, shared directory and history, reach and robustness cases, retirement and fresh sign-in. Needs the Windows computer on; about 30 to 40 owner minutes. If a scenario's owner step comes first, its first second-account sign-ins run under P1 to P7.
- **3:** After stage 2: a 48-hour soak of small scripted turns; no owner time planned.

Stages 1 to 3 use [the full review's P1 to P8](../../docs/proposals/full-review/experiments.md#a5-stages-and-pass-conditions).
Each later stage is pre-registered in this README before it runs. A5 gates one thing: `codex.v2`
states `signIn` only if stage 1's first step passes (ruling 82, point 1). It gates nothing else,
and no Agent doing Conduit's own orders names a second account until the owner says so.

## Stage 0: questions

(a) Does each client's everyday status command report a login and name the account it is signed
in to? [CTR-013](../../spec/promises/contracts.yaml) permits signIn only where the account hook
learns an account the service attests. Status is the first route; Codex's ID token is stage 1's.

(b) Does each client's browser login, started by a hook with no terminal, give a URL that
[IFC-049](../../spec/interface/implementation.yaml) can relay without opening a browser itself,
and then wait? For Claude, does it still give a listening loopback redirect, or fall back to a
code page that a login with carried: false cannot complete?

The [FR-427 verifier's correction](../../docs/proposals/full-review/findings/E04-signin-isolation.md#fr-427)
governs: the old closeout recorded methods, not proof of account naming or no-terminal login.
[FR-490 as corrected](../../docs/proposals/full-review/findings/G3-native-probes.md#fr-490),
D18 and D80 put these two facts alone before order 050. This is A5 stage 0, not a completed sign-in.

## Steps

The commands are those [the sign-in proposal](../../docs/proposals/sign-in/README.md#what-the-clients-offer-read-on-2026-11-10) records.

- For each client, Claude Code first and then Codex, the probe runs these steps in order:
  1. version: `claude --version`, or `codex --version`;
  2. everyday status: `claude auth status`, or `codex login status`, in the client's own
     default home;
  3. empty-home status: the same command, with `CLAUDE_CONFIG_DIR`, or `CODEX_HOME`, set to a
     new empty directory with mode 0700 inside the scratch root;
  4. login start: `claude auth login`, or `codex login`, followed by that client's
     `--login-arg` arguments, in that same home. The probe waits up to 30 s for an
     authorization URL to appear in the client's output or in the shim's log, then waits 2 s
     more. It checks each loopback redirect's listener, then cancels the login ([Safety](#safety));
  5. status after the cancel: step 3's command again, in that home;
  6. everyday status again: step 2's command again.
- Steps 4 and 5 run only where step 3 reported `login` as `no`. Otherwise both are recorded as
  skipped. A home that is not shown to be separate could hold the everyday login, and a login
  started or undone there could touch it.
- Each client starts with an empty environment, plus exactly these variables:
  - `HOME`, `USER` and `LOGNAME`, copied from the probe's own environment. The Agent packages
    keep these (`BASE` in `packages/agent-codex/src/lib.rs`), and a client may need them to
    find its everyday login. Step 3 then tests what P1 asks: a fresh home reports no login
    while the everyday logins stay signed in.
  - `PATH`: first the scratch root's `bin/`, which holds `browser-shim` and links to it named
    `open` and `xdg-open`; then the directory of the client's path; then each `--extra-path`;
    then `/usr/bin:/bin:/usr/sbin:/sbin`.
  - `BROWSER`, set to the shim.
  - `A5_SHIM_LOG`, a file in the scratch root for that client.
  - In steps 3 to 5, the client's home variable.
- Each command starts in a new session (`setsid`), so it has no controlling terminal and is
  the leader of its own process group. Standard input is null. Standard output and standard
  error are captured.
- Every command except the login has a limit of 30 s. When a command passes it, the probe kills
  the command's process group and records the step as timed out.

## What is recorded

- Every value the probe writes is one of these: a boolean, a word from the lists below, a
  count, an exit status, the version, a field name under the rule below, or the `--login-arg`
  arguments. Nothing else that a client printed is ever written.
- For each client the probe records:
  - `client`: `claude-code` or `codex`.
  - `version`: the first token of step 1's output that matches
    `^\d+\.\d+\.\d+([-+.]{1}[0-9A-Za-z.-]+)?$`, or `unparsed` if no token does.
  - For steps 2, 3, 5 and 6: `skipped`, `exit`, `timed_out`, `login`, `account_named` and
    `account_field`.
  - For step 4: `skipped`, `login_args`, `urls_seen` (a count), `shim_called`,
    `waiting_at_cancel`, `exited_before_cancel` (its exit status, or null), `ended_on`
    (`term`, `kill` or `exited`), `group_gone` and `logged_out`.
  - For step 4, one entry per distinct authorization URL. Each entry holds:
    - `source`: `printed`, `shim` or `both`;
    - `return` and `redirect_host`;
    - `state_length`, a count;
    - each check below, as a boolean;
    - `ifc_049`: the names of the checks that fail for its return, empty when none fails;
    - `listens`: `yes`, `no` or `not checked`.
- **The status rules.** If standard output, trimmed, parses as a JSON object, the JSON rules
  below apply to it. Otherwise the text rules apply, to both streams, with ANSI escapes
  removed.
  - `login`:
    - `no` where the text, lowercased, holds "not logged in", "not signed in" or
      "logged out", or where a JSON boolean is false and its key, lowercased with everything
      but letters removed, is `loggedin` or `signedin`;
    - `yes` where such a boolean is true, or where the text holds "logged in" or "signed in"
      and none of the phrases for `no`;
    - `unclear` otherwise.
  - `account_named`:
    - `yes` where a JSON string value, or a whitespace-separated token of the text, is shaped
      as an email address (one `@`, something before it, and a dot after it);
    - `yes` also where a JSON key, in its letters-only lowercase form, is `email`,
      `accountemail`, `useremail`, `accountid`, `accountuuid` or `userid`, and holds a
      non-empty string;
    - `no` otherwise.
  - `account_field`, where `account_named` is `yes`:
    - For JSON, the keys that lead to the value, joined by dots. It is written only if each
      key is an array index or matches `^[A-Za-z_]{1}[A-Za-z0-9_]{0,31}$`.
    - For text, the label before a colon on the value's line, lowercased with everything but
      letters removed. It is written only if it is one of `email`, `emailaddress`, `account`,
      `accountemail`, `user`, `useremail`, `login`, `loggedinas` or `signedinas`. A label can
      hold a person's name, so it is never written as printed.
    - In every other case, a line with no colon included, it is `unnamed`. It is null where
      `account_named` is `no`.
- **The URL rules.**
  - An authorization URL is any `https://` URL, in either stream or in the shim's log, whose
    query holds `client_id`, `response_type` or `redirect_uri`. A URL ends at whitespace or at
    any of `"'<>)]`.
  - The checks: `https`; `no_userinfo`; `no_fragment`; `no_repeated_parameter`; `client_id`
    (present and not empty); `response_type_code`; `s256` (`code_challenge_method` is
    `S256`); `challenge_43` (`code_challenge` is 43 base64url characters); and `state_ok`
    (`state` is at least 22 RFC 3986 unreserved characters).
  - `return` is one of these:
    - `redirect`, where `redirect_uri` is `http://` with the host `localhost`, `127.0.0.1` or
      `[::1]`. That host is recorded as `redirect_host`, as one of those three words. Two
      further checks apply: `redirect_port` (a port is given) and `redirect_no_query`.
    - `page`, where `redirect_uri` is `https://`.
    - `none` otherwise, which fails.
  - `listens`: for each `redirect` URL that gives a port, while the login waits, the probe
    opens a TCP connection to the URL's host and port, with a limit of 2 s, and closes it at
    once without sending anything. For `localhost` it tries 127.0.0.1, then [::1]. The value
    is `yes` if a connection opens and `no` if none does. A `redirect` with no port, and any
    other return, gets `not checked`. A redirect URL first seen after the client exited, or
    after the cancel, also gets `not checked`; it cannot meet (b).

## Safety

- The probe reads no environment variable except `HOME`, `USER`, `LOGNAME` and `TMPDIR`. It
  passes the clients none of its own beyond those in [Steps](#steps).
- It never opens, lists, reads or copies a file in any client home, the everyday ones
  included. The one exception is removing the scratch root at the end. It never runs
  `security`, and never reads a keychain item or the home's auth.json. The only file it reads is
  the shim's log, in its own scratch root.
- A client's output is held in memory, at most 64 KiB per stream, then parsed and dropped. It
  is never printed, logged or written. The probe's own messages name a step and a client only.
- It starts no model turn, and it cancels every login it starts, but it cannot keep a
  client's browser from completing one. The shim catches a client that runs `$BROWSER`, or
  `open` or `xdg-open` found on `PATH`. A client that opens a browser through the macOS
  API, or runs `/usr/bin/open` by its full path, opens the owner's default browser at the
  real authorization URL. If that browser holds a session with the client's service, the
  sign-in could complete before the probe cancels the login. So `shim_called` false does
  not show that no browser opened: only the owner's answer does. If a sign-in completes,
  step 5's logout in the scratch home, below, is the only guard left.
- To cancel a login, the probe sends TERM to its process group, then KILL after 5 s.
  `group_gone` is true only once the probe has reaped the client and sees that the group is
  gone.
- Where step 5 reports a login in a home that step 3 found empty, the probe names it at
  once (`step 5 NAME: scratch login found`, NAME being `claude-code` or `codex`). It then
  runs `claude auth logout`, or `codex logout`, in that scratch home only, and names the
  logout once it has run (`step 5 NAME: logout`). A signal cuts neither that status nor
  that logout. The probe records `logged_out`, and exits non-zero. It runs no logout in any
  other case, and never in a default home.
- The scratch root is a new directory, with mode 0700, under the system temporary directory.
  It is removed at the end, after a fault or a SIGINT, SIGTERM or SIGHUP too.
- Login-file references use only "the home's auth.json", with no directory before it. No
  home-directory path is recorded.

## Pass

For each client, both of these hold:

- (a) The everyday status reports a login and names an account, at step 2 and still at
  step 6. The empty home reports no login, at step 3 and again at step 5, after the
  cancelled login.
- (b) The browser login gives at least one URL whose return is `redirect` and that fails no
  IFC-049 check. Its listener answers, the client is still waiting when cancelled, and the
  owner saw no browser window open. A `page` URL alone fails (b). The code its page shows
  reaches the client only if carried back, and a Claude login is never carried (CTR-013,
  PND-013, ruling 75 C3). For Codex, 038c's handoff plans a redirect.

## What each outcome changes

- Codex's status names no account: order 050 writes Codex's account hook as O3 has it. The
  hook reads the status first and, if that names no account, the email claim of the ID token
  in the home's auth.json
  ([refusals and identity](../../docs/proposals/spec-batch-1/areas/refusals-and-identity.md), D2 and
  O3). Only the package's hook tries that second route, in its own signed-in home at stage 1,
  never the coordinator. If both routes fail, the owner chooses among O3's three answers.
- Claude's status names no account: this goes to the owner under ruling 75 C7.
- The everyday status reports no login, or is unclear, at step 2: the run shows nothing about
  that client. The client is in use and signed in, so the probe's environment is at fault.
  The admin records the fault and runs again.
- The everyday status changes between steps 2 and 6: this goes to the owner at once.
- An empty home reports a login, or steps 4 and 5 were skipped: this goes to the owner before
  050 is written. The Agents' homes would not be separate, and P1 would fail.
- Claude gives only a code page: this goes to the owner under ruling 49 and ruling 75 C3,
  before 050's Claude Type is written.
- Any other failure of (b): order 050 changes before it is written, naming the step.
- A call to the shim is recorded, not failed: 050's hook then sets a browser that opens
  nothing.

## Not shown

- A completed sign-in, and the account check.
- The ID-token route, which is stage 1's.
- Codex's device login. Its code goes to the engine's panel (order 051c; ruling 78, point 9).
- The logins' literal origins, paths, client IDs and ports. Order 050 reads those from the
  clients ([038c's handoff](../../docs/work-orders/038c-staged/return.md#agent-packages-order-050)).
- The Windows computer, the second accounts, refresh, lapse and sign-out.

## Running stage 0

The coordinator tells the owner before the run (D80): a browser, keychain or sign-in window may
appear, and any such window is closed without signing in. The coordinator also asks the
owner to make the default browser, for the run, one that holds no Anthropic or OpenAI
session (claude.ai, the Anthropic Console, ChatGPT or the OpenAI platform), for example a
browser profile with no sign-ins. A client may open that browser past the shim
([Safety](#safety)). Afterwards the coordinator asks the owner whether such a window
appeared, and records the answer as **yes** or **no** beside the probe's results. The probe
cannot supply the owner's observation.

The admin builds and runs from [client-facts/](client-facts/):

```sh
cargo build --release
target/release/client-facts --claude "$(command -v claude)" --codex "$(command -v codex)" --out ../results-DATE.json
```

The admin runs the probe in the foreground, never in the background, with a timeout of at least
10 minutes, so that the probe's own limits, not the timeout, end a run. On SIGINT, SIGTERM
or SIGHUP the probe starts no further command except step 5's, as below. Step 3's status
and step 4's login, in the scratch home, have their process group killed at once. Steps 1,
2 and 6, in the owner's own home, run to their end or to their 30 s limit, so that no
client is killed partway through its everyday home. If step 3 had reported no login, step
5's status and any logout still run to their end, so that a signal never leaves a signed-in
scratch home unchecked or its logout cut. The probe then removes its scratch root and its
output file, and exits non-zero with `step N NAME: signal` after any step-5 notices. N is
the step the signal cut or kept from starting, or else the step it came during; it is 4
where the signal cut the login or kept it from starting, though step 5 then ran. A signal
that comes once the probe has begun to write its report changes nothing: the report is
complete and kept. KILL leaves it no time to do any of that. After every run, completed or
not, each of these must print nothing:

```sh
pgrep -f 'auth login|codex login'
find "${TMPDIR:-/tmp}" -maxdepth 1 -name 'a5-*'
```

If either prints anything, the admin ends those processes, removes that directory and records the
run as a fault.

If a client is a script whose interpreter lives in another directory, the admin adds
`--extra-path` for that directory; the flag may repeat. Optional repeated
`--login-arg claude=<arg>` or `--login-arg codex=<arg>` values follow that client's login command.
Every run is recorded. A run with a `--login-arg` is recorded beside the run without one, never
in its place. The probe refuses an output file that already exists.

JSON has a `clients` array; each client has `client`, `version` and `steps`, keyed `1` to `6`.
Step 1 also records `exit` and `timed_out`. Step 4's `urls` array holds the distinct URL facts,
without the URLs themselves. `redirect_host` is null except for a loopback redirect.
`redirect_port` and `redirect_no_query` apply only to redirect; an unclassified return adds
`return` to `ifc_049`. Signal exits are negative signal numbers. A skipped step writes `skipped`
true and null in every field it would have observed: for a status, `exit`, `timed_out`, `login`,
`account_named` and `account_field`; for the login, `urls_seen`, `shim_called`,
`waiting_at_cancel`, `exited_before_cancel`, `ended_on` and `group_gone`. A skipped login keeps
its `login_args`, `logged_out` false and an empty `urls` list.

The probe writes the same facts as a Markdown table, one row per fact and one column per
client, headed by client and version. Exit 0 means every step ran or was skipped under the
steps above, whatever the clients answered. A probe fault, a signal or the safety logout at
step 5 exits non-zero, naming the step and client: `fault`, `signal`, or step 5's `scratch
login found` and `logout`. Every scratch login found and every safety logout is named, even
when a later step faults or a signal comes. If `browser-shim` is missing beside the probe,
or is not executable, the probe faults at step 0 before it starts a client. A fault or a
signal removes a report not yet written in full; a report written in full is kept, as after
a safety logout.
`logged_out` records that the scratch-home logout command ran, not a verified undo. The admin
records faults and reruns rather than treating them as client evidence.

The admin records every run's JSON and a dated Markdown result, opening with this experiment's
limit and giving yes or no by step, client and version, with the owner's window answer. Under
D78, the result merges with A5's State change and the experiments index's Current evidence change
in the same pull request. The admin stages and reruns the private-path check; that check and the
pull request's secret scan must pass. The worker runs the probe only through synthetic tests,
with Cargo offline; the admin alone runs it against real clients.

## Stage 0b

Pre-registered on 2026-11-16 by work order 195, before any of it ran; the owner approved parts A and
B on 2026-11-16. In stage 0, Codex 0.167.1 failed (a), its status naming no account, and (b):
with the owner watching, a browser tab opened at OpenAI's login page, from Codex, while the
probe recorded `shim_called` false ([results](results-2026-11-16.md)). Stage 0 did not show how
Codex opened that tab, or what would stop it. Stage 0b asks that, and whether Codex's status can
name the account, of Codex 0.167.1 alone, the version stage 0 recorded in both runs. It
completes no sign-in, starts no model turn and runs no Agent. Its results are facts for order
050 and for those who read the sign-in surface before 050 is written. They settle nothing of
050's design.

### Questions

- **Q1.** How does `codex login`, the browser login that stage 0's step 4 started, open a
  browser by itself, past the shim on `BROWSER` and `PATH`?
- **Q2.** Which setting, flag or variable, if any, keeps that login from opening a browser
  while it still prints its authorization URL and waits for its loopback redirect?
- **Q3.** Can `codex login status` name the account it is signed in to, under any flag,
  setting or login mode? Stage 0 found that the everyday status names none. The ID-token route
  stays stage 1's.

### Part A: the source and the documentation

Part A runs no client. A Codex worker reads Codex's public source at the tag of the version
stage 0 recorded, `rust-v0.167.1` of github.com/openai/codex, and Codex's public documentation.
It writes a dated results file in this directory, `results-2026-11-16-0b-source.md`. Each finding is
quoted with its source: the repository path and line at the tag, or the page's URL and the date
it was read. A question the sources do not answer is marked not found, with the searches made.

Part A reads nothing behind a sign-in, no copy of the source or of a page kept by a third party,
and no source of a library that Codex depends on. Where Codex calls a library, part A records the
call and the version that Codex's `Cargo.lock` pins at the tag; what the library does inside is
not shown. Claude Code has no public source, and part A reads nothing of it. Source read at a tag
can differ from the binary installed from it: part A shows what the source and the documentation
say, and only part B observes the client.

### What stage 0b records

- Part A records quotations of Codex's source and documentation, each with where it is, and "not
  found" where a search finds nothing. Part B records what stage 0's probe records
  ([What is recorded](#what-is-recorded)), unchanged, and the owner's answer for each run.
- Neither part ever records, in a file, a report or a message: a `state` value; a PKCE challenge
  or verifier; an authorization code or a device code; a token, or any claim of one; a URL of a
  login, its authorization or its redirect, whole or in part, so never a full URL with its query;
  any account value, such as an email address, an account, user or workspace ID, or a person's
  name; or a login literal, such as a client ID, an authorization origin, a port or a path.
  Where a quoted line holds one, the quotation cuts it with an ellipsis, and a constant is named
  by its identifier, never by its value. Order 050, not this stage, reads those literals from the
  clients ([038c's handoff](../../docs/work-orders/038c-staged/return.md#agent-packages-order-050)).

### Part B: a watched start of Codex's login

- **Who and when.** The coordinator runs part B on the Mac, at a time the owner picks, with the
  owner watching the screen. It runs only once part A's results have merged, and only with Codex
  at the version part A read. Before part B the coordinator runs `codex --version`; if it gives
  another version, part B waits until part A is read again at that version, by a later
  amendment here.
- **The controls.** Part B tests only the controls that part A's results list, under "For part
  B", as arguments that follow `codex login`. The probe as it stands can pass no other kind: a
  variable, a setting with no such argument, or an argument that must come before `login`. Part
  B does not run those, and its results say so; running one needs a later order that changes the
  probe, with tests, and a later amendment here, before it runs. An argument that switches to
  another login (the device login, an API key or an access token) is not a control here, and
  part B never runs it. If part A lists no control that the probe can pass, part B does not run
  and takes no owner time.
- **The runs.** Each run is one run of stage 0's probe, unchanged, built and run from
  [client-facts/](client-facts/) as [Running stage 0](#running-stage-0) gives the commands, with
  `--out ../results-DATE-0b-runN.json`, DATE being the day of part B and N counting from 0. Run 0
  passes no `--login-arg`: it is the sitting's own run without a control, beside which each
  control's run is recorded, as stage 0 asks. Then each control has one run, which passes each
  of its arguments as its own `--login-arg codex=<arg>`. If run 0 shows no window from Codex, no
  control runs in that sitting. Each run also takes Claude Code through its six steps; those
  facts are recorded as the probe writes them, and answer nothing for stage 0b.
- **The owner's answer.** After each run the coordinator also asks the owner whose login page
  any window or tab showed, Anthropic's or OpenAI's, as the owner names it, never its address.
  That is recorded with the owner's **yes** or **no**, beside the run's results.

### Part B's guards

Stage 0's guards hold for every run of part B, in stage 0's words. The browser, from
[Running stage 0](#running-stage-0):

> The coordinator tells the owner before the run (D80): a browser, keychain or sign-in window
> may appear, and any such window is closed without signing in. The coordinator also asks the
> owner to make the default browser, for the run, one that holds no Anthropic or OpenAI session
> (claude.ai, the Anthropic Console, ChatGPT or the OpenAI platform), for example a browser
> profile with no sign-ins. A client may open that browser past the shim ([Safety](#safety)).
> Afterwards the coordinator asks the owner whether such a window appeared, and records the
> answer as **yes** or **no** beside the probe's results. The probe cannot supply the owner's
> observation.

The checks after each run, from [Running stage 0](#running-stage-0):

> After every run, completed or not, each of these must print nothing:
>
> ```sh
> pgrep -f 'auth login|codex login'
> find "${TMPDIR:-/tmp}" -maxdepth 1 -name 'a5-*'
> ```
>
> If either prints anything, the admin ends those processes, removes that directory and records
> the run as a fault.

A scratch home with no login, from [Steps](#steps):

> Steps 4 and 5 run only where step 3 reported `login` as `no`. Otherwise both are recorded as
> skipped. A home that is not shown to be separate could hold the everyday login, and a login
> started or undone there could touch it.

The safety logout, from [Safety](#safety):

> Where step 5 reports a login in a home that step 3 found empty, the probe names it at once
> (`step 5 NAME: scratch login found`, NAME being `claude-code` or `codex`). It then runs
> `claude auth logout`, or `codex logout`, in that scratch home only, and names the logout once
> it has run (`step 5 NAME: logout`). A signal cuts neither that status nor that logout. The
> probe records `logged_out`, and exits non-zero. It runs no logout in any other case, and never
> in a default home.

The rest of [Running stage 0](#running-stage-0) holds too: the run in the foreground with a
timeout of at least 10 minutes, what a signal does, and the record of every run. Part B is
cancelled before any sign-in: the probe cancels each login it starts, and the owner closes any
window without signing in.

### What part B's outcomes show

- A control holds where, in its run, the owner saw no OpenAI page open; Codex's step 4 was not
  skipped, recorded at least one `redirect` URL with `source` `printed` or `both`, an empty
  `ifc_049` and `listens` `yes`, and was waiting at the cancel; and run 0 of the same sitting
  showed OpenAI's page. `shim_called` is recorded, and does not fail a control. Otherwise the
  control does not hold, and the result names the conditions that failed.
- Either way the result is a fact, and it decides nothing: what order 050 does about Codex's
  browser is not settled by this stage.
- Run 0 shows no window from Codex: the sitting shows nothing about any control. This is
  recorded, and the coordinator asks the owner whether to run part B again at another time.
- Step 5 names a scratch login found, or the safety logout runs: a sign-in completed outside
  consent. This goes to the owner at once, and part B stops.
- The everyday status changes between steps 2 and 6: this goes to the owner at once, as in
  stage 0, and part B stops.
- A window from Claude Code: it is recorded beside the run and named to the owner. It answers
  nothing for stage 0b.
- A fault, a signal, or a check after the run that prints anything: the run is recorded as a
  fault, not a result. Once the fault is cleared, the coordinator may run it again in the same
  sitting.

The coordinator records every run's JSON and a dated Markdown result,
`results-DATE-0b-watched.md`, which opens with this experiment's limit and gives, for each run,
its control, Codex's facts at step 4, the checks after it and the owner's answer. Under D78 it
merges with A5's State change and the experiments index's Current evidence change in the same
pull request. As for stage 0, the coordinator stages it and reruns the private-path check,
which must pass.

Part C, the same watched run on the Windows computer, is not pre-registered here. If it is
wanted, a later amendment pre-registers it before it runs.

### Not shown in stage 0b

- Agents kept apart: part B runs under one operating-system user, and part A runs nothing.
- A completed sign-in, the account check, and the ID-token route, which is stage 1's.
- What a library that Codex calls does inside, beyond what part B observes.
- Claude Code, which has no public source and of which stage 0b asks nothing.
- Codex on the Windows computer or under WSL (part C), and any version of Codex but 0.167.1.
- Codex's device login.
- The logins' literal origins, paths, client IDs and ports.
- Order 050's design.

## Stage 1, step 1: the account check

Pre-registered on 2026-11-17 by work order 197, before any of it ran, under
[ruling 82](../../docs/proposals/ruling-82/README.md), point 1, the owner's answer of 2026-11-16
("Ask Codex itself"). Stage 0 found that Codex's everyday status names no account
([results](results-2026-11-16.md)), and stage 0b's source read found no status message that names
one ([results](results-2026-11-16-0b-source.md)). CTR-013 lets a Type state `signIn` only where its
account hook reports an account the service attests, and NAM-024 forbids removing `signIn` once a
release states it. This step asks once, before any such release is installed, whether `codex.v2`'s
account hook reports the account that a home is signed in to. It is stage 1's first step; the
rest of stage 1 is pre-registered here before it runs.

### The route and the gate

- **The route.** `codex.v2`'s account hook learns the signed-in ChatGPT account by asking Codex
  itself, through its app-server's `account/read`, in the Agent's own home. It never opens the
  login file, the home's auth.json, and the coordinator never takes its place. Where Codex gives
  no answer, an error or no account, the hook reports that, with a diagnostic naming the step and
  the cause, and never reports an account that Codex did not give.
- **What it replaces.** This route replaces the one that spec batch 1's area draft planned and
  that this README stated before ruling 82: the status first and, if that names no account, the
  email claim of the ID token in the home's auth.json. Stage 0's and stage 0b's text keeps its
  words, as pre-registered. Where that text names that route or calls the ID-token route stage
  1's, in stage 0's questions, its [outcomes](#what-each-outcome-changes) and its
  [Not shown](#not-shown), and in stage 0b's Q3 and its [Not shown](#not-shown-in-stage-0b), the
  route is now this one, tried only in stage 1, by the package.
- **The gate.** `codex.v2` states `signIn` only if this step passes. No archive whose `codex.v2`
  states `signIn` is installed anywhere before this step's outcome is recorded here. Nothing
  switches a Codex Agent to an API key: a key is used only where a template sets `apiKey`
  explicitly (ruling 82, point 2).
- **The engine.** CTR-013 and PND-011 are unchanged. The engine reads only what the account hook
  reports, and compares it with the expected account, ignoring ASCII case.

### Before it: the source read

Before this step, work order 197 reads Codex's public source at the tag `rust-v0.167.1`, the tag
stage 0b read, for `account/read`, with no client run, and writes
[its results](results-2026-11-17-account-read-source.md) in this directory. It asks whether the
app-server offers the method and how it is reached; its answer for a ChatGPT login, an API-key
login and no login; where the email it returns comes from; whether answering reads or writes
anything beyond the home's login file, refreshes a token or calls the network; whether it is
marked experimental; what it returns when the claim is absent; and whether `codex login status`
names an account at that tag. The read runs no client and decides nothing for order 050. If it
finds no such method at that tag, or the coordinator does not read it as confirming this route,
the coordinator brings that to the owner before order 050's Codex half is written (ruling 82,
point 1), and this step runs only as a later amendment here says.

### When, and under which Type

- **When.** After order 048b-join merges, onto an engine that runs the account check, and before
  the Agent packages 0.2.0 merge. The coordinator runs it on the Mac, at a time the owner picks,
  and tells the owner first: a scratch home is signed in once through Conduit's sign-in flow, the
  owner signs in there with the account the check expects, a browser window opens for that, and
  the home is signed out and removed afterwards. It runs only once order 050's report records
  its read of what `codex logout` revokes
  ([section 6](../../docs/proposals/review-signin-2026-11-16/README.md#050-codexv2), item 8).
- **The client.** Before the step the coordinator runs `codex --version`. The step runs only with
  Codex at a version that a source read for `account/read` covers: 0.167.1, by work order 197. At
  another version it waits until a later amendment here records a read at that version.
- **The engine.** Built from `main` at a commit that holds order 048b-join, and run with a new
  state directory inside the scratch root, never the dogfood engine's state.
- **The Type.** The scratch home is signed in under the Type `codex-account-check.v1`, not under
  an installed `codex.v2`. The coordinator builds the package that exports it from order 050's
  branch, at the address `example.org/conduit/a5-stage-1-check`. It differs from that branch's
  Codex Agent package, `packages/agent-codex`, only in its address and in that Type's name: its
  contract is `codex.v2`'s in the shape that section 6's 050 `codex.v2` item 1 gives for the
  account fact's pass (`signIn` with `/account`, as `claude-code.v2`'s item 1), as order 050's
  branch states that shape, and where that branch states `codex.v2` with no `signIn`, the step
  waits for a later amendment here that gives the shape; and its items are `codex.v2`'s, the
  account hook included. It is installed only in the scratch state
  directory and removed with it. No archive whose `codex.v2` states `signIn` is installed for
  this step.
- **The template.** One Agent template of that Type. Its `account` is the expected account, the
  primary ChatGPT account that the owner uses with Codex, written only in the scratch state
  directory. Its home is the one that the package derives inside the instance's state
  directory, which the engine keeps in the scratch state directory (D53: the package refuses a
  client home outside it).
- **The scratch root.** A new directory, mode 0700, under the system temporary directory, whose
  name begins `a5-`, so that stage 0's check after a run finds it if it is left.

### The step

1. Version: `codex --version`.
2. Everyday status: `codex login status` in the client's own default home.
3. Registration and setup: the coordinator registers the template and runs its setup. Setup's
   account check finds no account and raises the instance's sign-in action (PND-011, PND-005).
4. Empty-home status: step 2's command, with `CODEX_HOME` set to the instance's home and with
   the configuration overrides, if any, that the package passes to every launch of Codex. It must
   report no login. Otherwise the step stops here and is recorded as a fault.
5. The sign-in: the coordinator, as root's Interface, then calls `sign-in.v1` (PND-010). The
   Interface relays the browser login's URL; the owner opens it and signs in once, with the
   expected account. Each retry polls the account hook (PND-010, PND-011) until `sign-in.v1`
   ends.
6. The outcome: the coordinator reads, through a filter that prints only these fields,
   `sign-in.v1`'s outcome and reason, whether the action answered signed-in, and the promise IDs
   its diagnostics name; it never prints the result's `account` or a diagnostic's text, which on
   another account names both accounts (PND-011). It then asks the owner whether the owner
   signed in with the expected account, and whether any other window appeared.
7. Sign-out: the coordinator stops the engine; where 050's item 8 read passed, it runs
   `codex logout` with `CODEX_HOME` set to the instance's home and step 4's overrides, and in no
   other home; where it failed, the home's credential is removed as item 8's fail says, by
   removing that home, without the client command; then step 4's status again, which must
   report no login.
8. Everyday status again: step 2's command again.
9. The checks after: the coordinator removes the scratch root, and each command under the guards'
   checks after, below, must print nothing.

### Stage 0's guards, as they apply

The owner signs in once, on purpose, in this step; stage 0 signed in nowhere. Its guards hold
otherwise, in its sections' words where they apply:

- **The browser** ([Running stage 0](#running-stage-0)). The coordinator tells the owner before
  the run (D80), and asks the owner to make the default browser, for the run, one that holds no
  OpenAI session, so that the account signed in is the one the owner enters. The owner signs in
  only at the URL that the Interface relays, opened by the owner; a window that Codex opens by
  itself is closed without signing in.
- **A scratch home with no login** ([Steps](#steps)): step 4. A home that is not shown to be
  separate could hold the everyday login, and a sign-in or sign-out there could touch it.
- **The everyday login** ([What each outcome changes](#what-each-outcome-changes)): steps 2 and
  8 must both report a login; a change between them goes to the owner at once.
- **No file read** ([Safety](#safety)). No one opens, lists, reads or copies a file in the
  scratch home or in the everyday home, the home's auth.json and any keychain item included. The
  exceptions are removing the instance's home under step 7, where it says so, and removing the
  scratch root. No sign-out or logout runs in a default home.
- **The checks after** ([Running stage 0](#running-stage-0)). Before step 1 the coordinator
  keeps, in a shell variable `BEFORE`, what `pgrep -f 'codex app-server'` prints then. After
  the step, completed or not, each of these must print nothing:

  ```sh
  pgrep -f 'auth login|codex login'
  comm -13 <(printf '%s\n' "$BEFORE" | sort) <(pgrep -f 'codex app-server' | sort)
  find "${TMPDIR:-/tmp}" -maxdepth 1 -name 'a5-*'
  ```

  If any prints anything, the coordinator ends only the processes it lists, removes that
  directory and records the run as a fault.

### What the step records

- The date; Codex's version; the commits of the engine and of the package built for the check;
  and the Type, `codex-account-check.v1`.
- For steps 2, 4, 7 and 8: `login` and `account_named`, under stage 0's
  [status rules](#what-is-recorded).
- The result: **pass** or **fail**, or a fault; and, where it is not a pass, the step that
  failed, by its number and name above, with its cause as one of: no answer from Codex, an
  error, no account, another account, or a guard's check.
- `sign-in.v1`'s outcome and reason, whether the action answered signed-in, and the promise IDs
  that its diagnostics name.
- The owner's two answers, each **yes** or **no**: whether the owner signed in with the expected
  account, and whether any other window appeared.
- Each check after the step, as printed nothing or not; and the owner's minutes.

### What the step never records

In a file, a report or a message: any account value, the expected account and the reported one
included, such as an email address, an account, user or workspace ID, or a person's name; a
token, or any claim of one; an authorization code or a device code; a `state` value, or a PKCE
challenge or verifier; a URL of a login, its authorization or its redirect, whole or in part; a
login literal, such as a client ID, an authorization origin, a port or a path; the app-server's
answer; the text of a diagnostic beyond its step and its cause; or anything else that a client
printed.

Two things pass, and only where this says: the expected account, which the owner enters in the
template in the scratch state directory, never through the coordinator's messages, and which is
removed with that directory; and the login's URL, which the Interface shows the owner on the
Mac's screen in step 5, and which the coordinator's session never prints or copies. Steps 2, 4,
7 and 8 are read through a filter that prints only `login` and `account_named` under stage 0's
status rules, as step 6's prints only its fields.

### What the outcomes change

- **Pass.** `sign-in.v1` succeeded and the action answered signed-in, so PND-011 found the account
  that the hook reported equal to the expected one, ignoring ASCII case; the owner answered
  **yes**, signed in with the expected account; and every guard held.
  `codex.v2` then states `signIn`, as order 050 writes it, and the outcome is recorded here
  before any archive whose `codex.v2` states `signIn` is installed.
- **Fail.** The guards held, the owner answered **yes**, and the account hook gave no answer, an
  error or no account, or the engine found another account. `codex.v2` then does not state
  `signIn`, Codex Agents do not sign in yet, nothing switches a Codex Agent to an API key, and
  the question returns to the owner, with FR-118.
- **A fault, not a result.** The owner answered **no**, or did not sign in; step 4 found a login;
  the engine, the package build or the scratch root failed for a cause that is not the account
  hook's; a signal ended the run; or a check after the step printed anything. Once the fault is
  cleared, the coordinator may run the step again at a time the owner picks.
- **The everyday status changes** between steps 2 and 8, or step 7 cannot leave the scratch home
  with no login: this goes to the owner at once, and the step stops.

The owner's answer that another window appeared is recorded and, alone, changes no outcome.

The coordinator records a dated Markdown result, `results-DATE-1-step-1.md`, DATE being the day
of the step, which opens with this experiment's limit. Under D78 it merges with A5's State change
and the experiments index's Current evidence change in the same pull request. As for stage 0, the
coordinator stages it and reruns the private-path check, which must pass.

### Not shown in stage 1's first step

- Agents kept apart: the step runs under one operating-system user.
- P1 to P8 as a whole. The step's pass is its own, above. It observes P1 only as step 4 and
  step 6, P2 only as step 6, and P7 only for the everyday status at steps 2 and 8. P3 to P6 and
  P8 wait for the rest of stage 1.
- The rest of stage 1, and stages 2 and 3: a wrong account refused and undone, the second
  accounts, refresh, lapse, and one Agent's sign-in or retirement leaving the others working.
- Claude Code, whose status names the account, and of which this step asks nothing.
- Codex's device login, Codex on the Windows computer or under WSL, and any version of Codex but
  the one the step records.
- The logins' literal origins, paths, client IDs and ports. Order 050 reads those from the
  clients.

### Amendment of 2026-11-19: the live check as it runs

Amended on 2026-11-19, before any of this step ran, by the coordinator's change for
[ruling 89](../../docs/proposals/ruling-89/README.md), as ruling 45 asks: each run pre-registered
before it runs. Items 1 to 5 are the coordinator's application of the readiness review's findings
(ruling 89, point 1), each open to the owner's strike; item 6 is for ruling 89, point 5, and its
client, its place outside the gate and its steps are the coordinator's, open to the owner's
strike. Its grounds are the readiness review on Claude Fable that the owner
asked for, whose reports are the coordinator's notes (private): its B3 (the Type's name), B2 (the
engine's session), D1-08 (the client and a second tab) and M1 (Claude's sign-in never run live).
The route, the gate's aim, what the step records and never records, and the Codex step's pass,
fail and fault conditions do not change, except where this says.

1. **The Type keeps the name `codex.v2`.** The package built for the check keeps its Type's name,
   `codex.v2`, and differs from the Codex lane's `packages/agent-codex` only in its address,
   `example.org/conduit/a5-stage-1-check`. The name `codex-account-check.v1`, in "The Type" and
   "What the step records" above, is not used: the Codex sign-in hook finds its stated logins in
   the contract whose Type is named exactly `codex.v2`, so under another name it fails at its
   start, before any login, and the step would end a fault. The gate's "No archive whose
   `codex.v2` states `signIn` is installed anywhere before this step's outcome is recorded here",
   and "The Type"'s "No archive whose `codex.v2` states `signIn` is installed for this step", are
   narrowed to the scratch store: no such archive is installed anywhere but the check's package, at
   its scratch address, in the scratch state directory, and it is removed with that directory. The
   step records the Type as `codex.v2`, with the package's address. Both changes amend the
   coordinator's reading of ruling 82, point 1, that answer 5 keeps Q3 option a's two terms, as
   ruling 89 records, open to the owner's strike.
2. **The client is `conduit sign-in`** (order 051b-2). The step runs after order 051b-2 merges as
   well as 048b-join, and the engine is built from `main` at a commit that holds both. Step 5
   reads: the owner, as root's Interface, runs `conduit sign-in --store <the scratch state
   directory> Agent/<the instance>` in a terminal of the owner's own on the Mac, outside any
   client's session, from a line the coordinator prepares with every path filled in. Its standard
   output, which holds the result and its `account`, goes to a file in the scratch root; the
   engine's message, the URL and the prompts go to the owner's screen alone. The owner reads the
   message, types `y` to open the URL, signs in, and presses Enter for each retry until
   `sign-in.v1` ends. The coordinator never runs that line, so that the URL never reaches its
   session, and step 6 reads the file through its filter. That file is a third thing that passes,
   beside the two that "What the step never records" names, and only in the scratch root: only
   step 6's filter reads it, it is never copied or printed, and step 9 removes it with the scratch
   root. The owner writes the expected account into the template, in the scratch state directory,
   by a line that does not echo it.
3. **A second tab is expected.** Codex's own login opens the default browser past any shim, and
   `conduit sign-in` opens the relayed URL once the owner types `y`, so two tabs of one login may
   open. A tab that opens before the owner types `y` is Codex's own: the owner closes it without
   signing in there, as stage 0's browser guard has it, and signs in only in the tab opened after
   `y`. The owner's answer that another window appeared is expected, and alone changes no outcome,
   as above.
4. **The engine's call timeout.** `conduit serve` for the scratch state directory runs with
   `--call-timeout-seconds 1200`. The serving engine's session idle limit is the call timeout plus
   60 seconds, 180 seconds by default, and a running call does not reset it
   ([engine/CONFORMANCE.md](../../engine/CONFORMANCE.md), DL10), while `conduit sign-in` holds one
   session through the owner's login; under the default the session could end while the owner
   signs in, and the step end on a cause that is not the account hook's. `serve`'s standard output
   and standard error go to a file in the scratch root that no one reads.
5. **Codex's warning.** A release Codex may refuse its helper aliases for a home under the system's
   temporary directory and print a warning on standard error, then go on (order 197's read, R4;
   order 050c). That warning is expected and, as client output, not recorded.
6. **A Claude scratch-home sign-in follows** ([ruling 89](../../docs/proposals/ruling-89/README.md),
   point 5). After step 8, in the same sitting, unless the step stopped because the everyday status
   changed, the owner signs a scratch Claude home in once through `conduit sign-in`. By the
   coordinator's reading in ruling 89, it is not part of this step's gate or of A5's: it decides
   nothing for `codex.v2`, for `claude-code.v2` or for order 050n's join. It runs in a new state
   directory inside the same scratch root:
   - C1, version: `claude --version`, recorded.
   - C2, everyday status: `claude auth status`, with `CLAUDE_CONFIG_DIR` unset, through a filter
     that prints only `login` and `account_named` under stage 0's status rules.
   - C3, registration and setup: one Agent template of a check package built from the Claude
     lane's `packages/agent-claude-code`, differing from it only in its address,
     `example.org/conduit/a5-stage-1-claude-check`, its Type keeping the name `claude-code.v2`. Its
     `account` is the owner's primary Claude account, written as item 2 has it. Setup's account
     check finds no account and raises the instance's sign-in action.
   - C4, empty-home status: C2's command with `CLAUDE_CONFIG_DIR` set to the instance's home. It
     must report no login; otherwise the sign-in stops here and is recorded as a fault.
   - C5, the sign-in: as item 2 has it, for this instance, its standard output's file passing
     only as item 2 says, with `serve` run as item 4 has it.
   - C6, the outcome: as step 6, with the owner's two answers and a third, whether a macOS
     Keychain dialog appeared, each **yes** or **no**.
   - C7, sign-out: the coordinator stops the engine and runs `claude auth logout` with
     `CLAUDE_CONFIG_DIR` set to the instance's home, and in no other home, since item 8's read
     passed for it ([results](results-2026-11-17-logout-source.md#the-outcome-under-item-8)); then
     C4's status again, which must report no login.
   - C8, everyday status again: C2's command.

   Step 9 then runs once, for both. Stage 0's guards hold as for the Codex step: the default
   browser holds no claude.ai session; no file or Keychain item of either home is opened, listed,
   read or copied; and C2 and C8 must both report a login, a change between them going to the
   owner at once. It records what the Codex step records, with the Type `claude-code.v2`, the
   package's address and the third answer, and never records what the Codex step never records.
   **Pass**: `sign-in.v1` succeeded and the action answered signed-in, so PND-011 found the
   account that `claude-code.v2`'s account hook reported equal to the expected one, ignoring ASCII
   case; the owner answered **yes**, signed in with the expected account; and every guard held.
   **Fail** and **fault** as for the Codex step, naming the failing step among C1 to C8; by the
   coordinator's reading in ruling 89, either goes to the owner before any Claude dogfood slot
   signs in. Its result goes in the same `results-DATE-1-step-1.md`, under a heading of its own.

Before the sitting is booked, the coordinator runs a dry run of the Codex step with the same
builds and the Codex package's fixture client in place of Codex, pausing past 180 seconds before
a retry; it is preparation, signs nothing in, and records nothing here.

Not shown by the Claude sign-in, beside what the step does not show: Claude Code's other logins,
Claude Code on another machine, and any version of Claude Code but the one recorded. The line of
"Not shown in stage 1's first step" on Claude Code still holds for the Codex step; the Claude
sign-in is pre-registered here, apart from it.

## Stage 1, before step 1: the refresh-blocked probe

Pre-registered on 2026-11-19 by work order 208, before the probe was written and before any of it
ran, under ruling 86 of 2026-11-18. Order 050l's account hook asks `codex app-server` for
`account/read` with `refreshToken` false, in the Agent's own home. That asks for no refresh, but
the app-server can still refresh a ChatGPT login and rewrite the home's auth.json
([R4](results-2026-11-17-account-read-source.md#r4-what-answering-reads-writes-refreshes-or-calls)),
while `account-hook.v1` declares no effect (CTR-002). The owner, asked on 2026-11-18 how to treat
that, answered "Find a different route". Asked then "Codex account check: block refresh for the
check (a probe confirms no write first). When an expired access token makes the check fail, what
then?", the owner answered "(b) Proceed to sign-in (Recommended)". The coordinator reads that
answer as accepting the route the question named, a reading open to the owner's strike: the
check's app-server alone gets Codex's variable `CODEX_REFRESH_TOKEN_URL_OVERRIDE`, set to an
address under the reserved `.invalid` domain (RFC 6761, section 6.4), and every other child of
the package has it stripped. This probe is the "probe confirms no write first". Order 050m relies
on the route only once the probe's results are recorded here.

### Why a write is possible, and why the variable should stop it

In the coordinator's reading of Codex's source at the tag `rust-v0.167.1` (commit
`0a94398fa60c`), paths under `codex-rs/`:

- **T1, proactive refresh.** `auth()` refreshes a ChatGPT login whose access token expires within
  five minutes (`login/src/auth/manager.rs:222`, `:2599-2615`, `:3528-3546`). The app-server
  reaches `auth()` while it starts and answers, through its configuration's cloud bundle
  (`cloud-config/src/chatgpt.rs:33-46`), whatever the request asks.
- **T2, recovery after a 401.** `account/read` runs workspace discovery for a ChatGPT login with
  an account ID, and a 401 from it starts unauthorized recovery, which refreshes
  (`app-server/src/request_processors/account_processor/workspace_routing.rs:381-450`).
- **The write.** Either refresh saves the new tokens only after the token endpoint answers with
  success (`login/src/auth/manager.rs:3612-3685`), under the lock file `auth.lock` in the home
  (`login/src/auth/storage.rs:195-212`, taken at `login/src/auth/manager.rs:1734`), then a lock
  file beside the home's auth.json (`login/src/auth/storage_commit.rs:29-66`), and through an
  atomic rename (`login/src/auth/storage.rs:274-296`).
- **The variable.** Where it is set, the token endpoint is its value, ahead of the login's own
  issuer (`login/src/auth/manager.rs:1872-1878`); it is read when the login is loaded (`:452`),
  and again at each refresh (`:3618`). It is undocumented. Outside that file,
  `login/src/auth/revoke.rs:139` derives the logout's revoke endpoint from it, and
  `login/src/auth_env_telemetry.rs:41` records only whether it is set; Codex's tests also use
  it. An address under `.invalid` never resolves, so no refresh can complete and nothing is
  saved. That is a reading of the source; this probe observes the binary.

### The question

- **Q.** Codex 0.167.1's `codex app-server`, asked `account/read` with `refreshToken` false as
  order 050l's hook asks it, in a home whose login is due a refresh by T1, by T2 or by both, and
  with `CODEX_REFRESH_TOKEN_URL_OVERRIDE` set to `https://refresh.invalid/oauth/token` on its
  process alone: does it write the home's auth.json, or begin to save it? And what answer does
  the hook get?

### Predictions

The coordinator's, from the source, written before the run. They are not results:

- No run with the variable changes the home's auth.json or creates a lock file, and the stub's
  token endpoint gets no request in such a run.
- Every control run changes the home's auth.json.
- With the variable, case P1 answers with the fabricated account, and cases P2 and P3 answer with
  an error whose message is "workspace routing discovery unauthorized (401)".

### The machine and the client

- **The Linux node** (WSL Ubuntu 24.04, x86_64), as
  [the second-machine guide](../../docs/guides/second-machine.md) describes it, never the Mac,
  which holds the owner's logins; [experiment 032](../032-gateway-probe/README.md) runs there for
  the same reason. The node's own Codex is signed in in its user's real home
  ([the second-machine guide](../../docs/guides/second-machine.md), step 5). So every run
  happens in a mount namespace in which an empty `tmpfs` covers the real home, `/run`, `/tmp`,
  `/var/tmp`, `/dev/shm`, `/mnt` and `/etc/codex` where it exists, as
  [experiment 032](../032-gateway-probe/README.md) masks them, and only `refresh-probe/` is
  bound back under the home's mask. Nothing under the real home but `refresh-probe/` is
  reachable inside.
- **Codex 0.167.1**, the version [stage 0](results-2026-11-16.md) recorded and order 197 read,
  installed as experiment 032 installs its clients: with npm, into `refresh-probe/target/codex/`,
  which Git ignores, as the package `codex` of the npm scope `openai` at 0.167.1, under `env -i`,
  with `--ignore-scripts`, and with `HOME` and npm's cache under `refresh-probe/target/`. If the
  binary then gives no version, it is installed once more with its script, and the results say
  so. The probe runs the native binary that the package carries, not npm's launcher, and only if
  its `--version` names 0.167.1. Installing reaches the network, as do the clone of the tag that
  the worker reads and any crate cargo lacks; all three come before the probe runs.

### The probe

A Rust crate, [refresh-probe/](refresh-probe/), with its tests. Its one command,
`refresh-probe run --codex PATH --out PATH`, with `--form variable` for a second sitting, reads
no environment variable but `HOME`, which it uses only for the masks above and the self-test's
check, and refuses an output file that exists. Its first process, outside the namespace, writes
that file. It resolves `--codex` and `--out` to absolute paths before any mask, changes its
working directory to `refresh-probe/` once the masks are applied, and starts each app-server
with the run's directory as its working directory.

- **The namespace.** The probe runs itself again under
  `unshare --user --map-root-user --net --mount --pid --fork --mount-proc`, applies the masks
  above, then brings loopback up there with `ip link set lo up`, then runs itself once more
  through `unshare --user` with `--map-user` and `--map-group` set to the user's own IDs, so
  that Codex runs with no capability. Every run happens inside: no interface but loopback, and
  no process in `/proc` but the namespace's own, since another process's root, working
  directory or environment there would reach past the masks.
- **The self-test**, inside, before any run; nothing runs unless all of it holds: the namespace
  lists `lo` alone; a TCP connection to `192.0.2.1:443` fails within 2 s; a lookup of
  `refresh.invalid` fails; a TCP listener on `127.0.0.1` accepts a connection; the process runs
  with the user's own user and group IDs; the scratch root is not under `/mnt`; the real home's
  `.codex` directory and `/etc/codex` are absent inside, each checked with `stat` alone; `/mnt`
  and `/run` hold nothing; the working directory is `refresh-probe/`; `/proc` lists no process
  but the namespace's own; and a search of `/run`, `/tmp`, `/var/tmp` and `/dev/shm` inside
  finds no socket, each count printed alone.
- **The stubs.** For each run, one plain-http server on `127.0.0.1`, on a port the system picks,
  as Codex's own tests serve these endpoints at the tag
  (`app-server/tests/common/auth_fixtures.rs`,
  `app-server/tests/suite/v2/account_enterprise_tests.rs`). Workspace discovery, the accounts
  check under `/backend-api`, answers each case's way below, with the fabricated account in the
  shape the tag's fixture gives, but with the backend origin `https://backend.invalid` and the
  routing override `NO_CONSTRAINT`: Codex takes only an https origin there
  (`app-server/src/request_processors/account_processor/workspace_routing.rs:606-617`), and one
  under `.invalid` puts no real host into an answer. The token endpoint, `POST /oauth/token`,
  answers 200 with fabricated tokens in every run, so a run whose refresh reached it would write.
  Any other request answers 404. Each request is logged by its method, its path, the status
  served and which fabricated access token it bore, as `original`, `issued`, `other` or `none`;
  never by a header's or a body's value.
- **The fabricated login.** For each run, a new home of mode 0700, in a scratch root under
  `refresh-probe/target/scratch/`, holding only the home's auth.json, of mode 0600, which the
  probe writes in the shape that Codex's `AuthDotJson` takes at the tag: a ChatGPT login; an ID
  token and an access token, each an unsigned JWT the probe builds; a refresh token; the account
  ID `probe-208-account`; the last refresh, now; and, as the login's own issuer, the stub's
  origin. The ID token's claims are the email `work-orders@conduit.invalid`, the plan `plus`, the
  user ID `probe-208-user` and the account ID `probe-208-account`. Nothing in it comes from a real
  login.
- **The client's start.** `CODEX app-server --stdio -c cli_auth_credentials_store="file" -c
  chatgpt_base_url="http://127.0.0.1:PORT/backend-api"`: the package's arguments, and the stub's
  base URL. Its environment is cleared, as `env -i` clears it, then set to exactly `HOME`, a
  directory of its own beside the home, `CODEX_HOME`, the home, and `PATH`, `/usr/bin:/bin`; and,
  in a blocked run, the variable. It starts in a process group of its own, with its standard error
  kept under `refresh-probe/target/runs/` and never committed.
- **The exchange,** as order 050l's hook sends it
  ([its order](../../docs/work-orders/050l-codex-account-hook.md); the hook's code is on a lane
  branch that is not on the remote), within 30 s: `initialize`, with `clientInfo` `agent-codex`
  0.2.0 and `experimentalApi` false; the `initialized` notification; then `account/read` with
  `refreshToken` false. Notifications and the server's requests are skipped,
  and their methods recorded. After the answer the probe keeps the app-server running for 10 s,
  longer than the hook does, so that a refresh begun at its start has time to finish; then it
  sends TERM to the app-server's group, and KILL after 5 s.

### The cases

| Case | Trigger | The access token expires | Discovery answers |
| --- | --- | --- | --- |
| P1 | T1 | 180 s after the login is written | 200 to every request |
| P2 | T2 | one day after | 401 to the original access token, 200 to any other |
| P3 | T1 and T2 | 600 s before | 401 to the original access token, 200 to any other |

Each case runs in two arms, each in a new home with new stubs:

- **blocked**: the variable set to `https://refresh.invalid/oauth/token`;
- **control**: no variable, so a refresh goes to the login's own issuer, the stub, which answers
  200.

One sitting is eighteen runs, in this order three times over: P1 blocked, P1 control, P2 blocked,
P2 control, P3 blocked, P3 control. If the tag's source refuses a loopback `http` issuer, or a
control run of the sitting sends its stub no token request, a second sitting runs all eighteen in
the second form: no issuer in the login, and each control run's variable set to the stub's token
endpoint. It is recorded beside the first. In that form a blocked run that ignored the variable
would send its refresh to Codex's default endpoint, which the namespace keeps out of reach, and
would write nothing either: its blocked runs do not show that the variable was honored, and the
results say so.

### What each run records

- The case, the arm, the repetition and the form; the answer's kind (`result`, `error`, `eof` or
  `timeout`), the answer's line as Codex wrote it, and an error's message; the methods of the
  notifications and requests skipped; and how the app-server ended.
- The home's auth.json, before the start and after the stop: whether it is there; whether its
  bytes, its modification time and its inode are each unchanged; and its size. Whether its lock
  files, `auth.lock` and `auth.json.lock`, appeared. The home's files and directories before and
  after, by their paths relative to the home, with what was added and what was removed.
- Each request the stub logged, as above, and the count of the standard error's lines.
- A run **wrote** where the home's auth.json changed in its bytes, its modification time or its
  inode, or went away. A run **began a save** where either lock file appeared.

### What is never recorded

A token, fabricated or not, or any claim but those named above; the stub's port; an absolute
path, which is replaced by `<run>` for the run's directory wherever Codex's answer holds one;
the standard error's text; a user, machine or host name; and any environment variable's value but
the variable's fixed value above. The probe writes `results-DATE-refresh-probe.json`, and a
second sitting `results-DATE-refresh-probe-2.json`; the dated Markdown result,
`results-DATE-refresh-probe.md`, is written from them.

### What the outcomes mean

- **No write under the variable.** In one sitting, every control run wrote, in all three
  repetitions of each case, and no blocked run wrote, began a save or sent its stub a token
  request. Order 050m may then rely on the route, with the variable on the account check's child
  alone, and re-read at every bump of Codex's pin.
- **A write under the variable.** A blocked run, in any sitting, wrote, began a save or sent its
  stub a token request. This goes to the owner, as it stands, and is not worked around; order
  050m does not rely on the route, and the choice of the check's route returns to the owner, as
  ruling 86, point 5, states.
- **Not settled.** No blocked run wrote, but some case's control runs did not all write, in
  either sitting, or a run of that case was a fault: that case shows nothing about a write under
  the variable, and the result names the case and the runs. The coordinator reads the source
  again, and either amends this section, dated, before a rerun, or brings it to the owner.
- The answers are recorded, and alone change no outcome. They go to order 050m, which reads them
  against PND-010 as ruling 86 amends it, and against the hook's diagnostic.

### Stop rules

1. A check of [the machine](#the-machine-and-the-client) fails, the binary does not name 0.167.1,
   or the self-test fails: no run happens. The result records which, and nothing else.
2. The probe faults in a run, or a run ends by `timeout` before `initialize` answers: the run is
   recorded as a fault, not a result, and the probe goes on with the next. A third fault in one
   sitting ends it.
3. After every sitting, completed or not, `pgrep -f 'codex[ ]app-server'` must list no process
   that it did not list before the sitting, and the scratch root must hold nothing. Otherwise the
   sitting is recorded as a fault.

### Not shown by the refresh-blocked probe

- Agents kept apart: the probe runs under one operating-system user.
- The Mac, its binary of Codex, and its system proxy, which Codex's HTTP client tries first on
  macOS; and Codex on Windows outside WSL. Stage 1's first step runs on the Mac.
- Any version of Codex but 0.167.1. The variable is undocumented: a bump of the pin needs the
  source read again and this probe run again.
- OpenAI's service: how long a real access token lives, whether a real refresh token is
  single-use, what real discovery answers, and how often a real idle home's check meets T2.
- A lookup of `refresh.invalid` outside the namespace: what a machine's own resolver or proxy
  does with it.
- Whether a real check's telemetry tells OpenAI that the variable is set
  (`login/src/auth_env_telemetry.rs:41`): the namespace lets nothing out.
- A write by another process in the same home, such as a turn that runs while the check does, and
  any write after the hold.
- The keyring stores: the probe uses the file store alone, as the package does.
- Whether order 050m sets the variable on the check's child alone and strips it elsewhere: 050m's
  tests show that.
- What the engine does with the answers: PND-010 as ruling 86 amends it, and IFC-010.
