# 027 — A5: sign-in and isolation

Every result of this experiment is obtained under one operating-system user. Under one user, an Agent that runs code can read, use or replace another Agent's login and act as root's Interface. So no result here shows Agents kept apart, before or after the owner's dedicated macOS user (ruling 75 B3). Stage 0 runs no Agent at all.

Stage 0 pre-registered on 2026-11-15 by work order 159; not run. No real-client result is claimed.

Amended on 2026-11-15 by work order 167: how a signal stops the probe, the notice of a scratch login found, what the shim cannot catch, and the owner's browser check before the run. The steps, what is recorded, the pass conditions and the outcomes do not change.

## Four stages

- **0:** Before order 050 is written: two facts about the real clients, primary accounts only, no engine, nothing signed in; no owner time unless a window opens. The coordinator tells the owner first.
- **1:** On the first build with sign-in, as the S1 rerun's rehearsal: one checked sign-in on a primary account; the package's hook tries Codex's ID-token route in its own home, never the coordinator. About five owner minutes.
- **2:** After the rerun's pin: the second accounts' first sign-ins, four Agents and four homes, wrong then right accounts, checks after sign-in, undo and retirement, forced lapse, shared directory and history, reach and robustness cases, retirement and fresh sign-in. Needs the Windows computer on; about 30 to 40 owner minutes. If a scenario's owner step comes first, its first second-account sign-ins run under P1 to P7.
- **3:** After stage 2: a 48-hour soak of small scripted turns; no owner time planned.

Stages 1 to 3 use [the full review's P1 to P8](../../docs/proposals/full-review/experiments.md#a5-stages-and-pass-conditions).
Each later stage is pre-registered in this README before it runs. A5 gates nothing;
no Agent doing Conduit's own orders names a second account until the owner says so.

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
