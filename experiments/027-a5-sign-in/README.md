# 027 — A5: sign-in and isolation

Every result of this experiment is obtained under one operating-system user. Under one user, an Agent that runs code can read, use or replace another Agent's login and act as root's Interface. So no result here shows Agents kept apart, before or after the owner's dedicated macOS user (ruling 75 B3). Stage 0 runs no Agent at all.

Stage 0 pre-registered on 2026-11-15 by work order 159; run twice, on 2026-11-15 and 2026-11-16: [results](results-2026-11-16.md).

Amended on 2026-11-15 by work order 167: how a signal stops the probe, the notice of a scratch login found, what the shim cannot catch, and the owner's browser check before the run. The steps, what is recorded, the pass conditions and the outcomes do not change.

Stage 0b pre-registered on 2026-11-16 by work order 195, [below](#stage-0b): part A, a read of Codex's public source and documentation, done on 2026-11-16, [results](results-2026-11-16-0b-source.md); part B, a watched run, not yet run.

Stage 1's first step pre-registered on 2026-11-17 by work order 197, [below](#stage-1-step-1-the-account-check): the live account check of ruling 82, point 1, not yet run; before it, a read of Codex's public source for `account/read`, done on 2026-11-17, [results](results-2026-11-17-account-read-source.md). The same order replaced, in the stages below, Codex's ID-token route and A5's "gates nothing" with ruling 82's route and gate.

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
