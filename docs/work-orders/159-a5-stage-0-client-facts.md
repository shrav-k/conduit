# Work order 159: the two client facts order 050 needs (A5's stage 0)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only because the
sandbox refuses a loopback bind without it, and the probe's tests bind one on 127.0.0.1. Use it for
nothing else: fetch no crate, and reach no other machine. Do not run `claude` or `codex`, and run
`client-facts` only through its tests. Outside your working tree, read nothing under your home
directory except Cargo's and rustup's own directories. The admin runs the probe against the real
clients. Run one cargo command at a time, and never print a whole large file or log. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
print a key, a token, a password or any credential.

## What and why

Order 050 writes each Agent Type's logins as literal values and decides whether its Codex Type may
state `signIn`. Two facts about the real clients decide both:

- (a) whether each client's status command names the account it is signed in to.
  [CTR-013](../../spec/promises/contracts.yaml) lets a Type state `signIn` only where its account
  hook learns an account that the service attests. The status is the first route to it. Codex's
  second route, the ID token, is tried at stage 1.
- (b) whether each client's browser login, started by a hook with no terminal, gives a URL that
  [IFC-049](../../spec/interface/implementation.yaml) can relay, without opening a browser itself,
  and then waits.

The full review of 2026-11-14 (order 7 of [its orders](../proposals/full-review/orders.md)) found
these facts unsettled ([FR-427](../proposals/full-review/findings/E04-signin-isolation.md#fr-427)).
Its verifier's corrections govern. The increment-3 closeout of 2026-11-10 records what both status
commands printed: a claude.ai login, and for Codex a ChatGPT login, which is a method, not an
account. It records neither whether a status names the account nor anything about a login started
with no terminal. The larger unknown is Claude's: with no terminal, does its login still give a
loopback redirect that listens, or does it fall back to the page that shows a code, which a login
with `carried: false` cannot complete?

The review placed these two facts, and no other native probe, before 050 is written
([FR-490](../proposals/full-review/findings/G3-native-probes.md#fr-490), as corrected). The same
placement is in decisions D18 and D80 of [the review](../proposals/full-review/decisions.md), which
[ruling 78](../proposals/ruling-78/README.md), point 1, part 2, keeps by keeping sign-in before the
pin. This order builds the probe and pre-registers A5's stage 0. The admin runs it after telling
the owner.

## The decisions it applies

1. **The experiment and its row** (D80; the review's
   [changes to the experiments](../proposals/full-review/experiments.md), "Numbers, assigned
   once"; the roadmap's rule that whoever starts an item writes `in progress`).
   - Make the new directory `experiments/027-a5-sign-in/`.
   - In `ROADMAP.md`, replace A5's row with this line:

   ```markdown
   | A5 | The sign-in test that ruling 49's record makes A5: is each Agent signed in, through the Interface, to the account its configuration names; is a wrong account refused and undone; does nothing of a login cross Conduit; and does one Agent's sign-in, undo or retirement leave the others working? In four stages: 0, two facts about the real clients, with nothing signed in; 1, one checked sign-in on a primary account, as the S1 rerun's rehearsal; 2, the second accounts' first sign-ins; 3, a 48-hour soak of small scripted turns | Stage 0: no engine, the primary accounts, and no owner time unless a window opens. Stage 1: the first build with sign-in. Stage 2: the S1 rerun's pin, which comes after the sign-in chain (048a, 048b, 050, 051b); the second accounts, signed in only through Conduit's Interface ([ruling 72's later answer](docs/proposals/ruling-72/README.md#later-answers)); the Windows computer on. Stage 3: 48 hours | Stage 0: order 050's logins, and whether its Codex Type may state signIn. Stage 1: the S1 rerun. A failed account check goes to the owner. A5 gates nothing, and no Agent doing Conduit's own orders names a second account until the owner says so | Each stage's dated results in [experiment 027](experiments/027-a5-sign-in/README.md), pre-registered there before the stage runs; stages 1 to 3 against P1 to P8 of [the full review's A5 section](docs/proposals/full-review/experiments.md#a5-stages-and-pass-conditions). Every result opens with what one operating-system user cannot show | in progress: stage 0, on branch `wo/159-a5-stage-0-client-facts` (work order 159) | Probe E2 of the plan audit; ruling 49; the full review's D80 |
   ```

   - In "Proposing an experiment", replace the numbering line with this text:

   ```markdown
   - A directory under `experiments/` takes the next unused number, counting the numbers this page
     holds (006 for A74) and those [the full review](docs/proposals/full-review/experiments.md)
     assigned (024 to 031). No number is reused.
   ```

   - In `experiments/README.md`, add this row after 023's, with the date you write it in place of
     DATE:

   ```markdown
   | [027 — A5: sign-in and isolation](027-a5-sign-in/README.md) | A5: whether each Agent is signed in to the account it names, through the Interface, in four stages. Stage 0 probes two facts about the real clients that order 050 needs, with nothing signed in. | Stage 0 pre-registered on DATE by work order 159; not run. |
   ```

2. **`experiments/027-a5-sign-in/README.md` pre-registers stage 0 before anything runs** (ruling
   45; D60; FR-427 as corrected; the review's A5 section). Write its links relative to its own
   directory.
   - Its title is `# 027 — A5: sign-in and isolation`.
   - It opens with this limit, in these words: "Every result of this experiment is obtained under
     one operating-system user. Under one user, an Agent that runs code can read, use or replace
     another Agent's login and act as root's Interface. So no result here shows Agents kept apart,
     before or after the owner's dedicated macOS user (ruling 75 B3). Stage 0 runs no Agent at
     all."
   - It gives the four stages one line each, as the review's A5 section does, and links that
     section for P1 to P8. Each later stage is pre-registered in this README before it runs.
   - It then states stage 0 in full: the two questions (a) and (b); the steps (decision 4); what
     is recorded, and how (decision 5); the safety rules (decision 6); and how the admin runs it,
     from `client-facts/`. The admin builds with `cargo build --release`, then runs
     `target/release/client-facts --claude "$(command -v claude)" --codex "$(command -v codex)" --out ../results-DATE.json`.
     If a client is a script whose interpreter lives in another directory, the admin adds
     `--extra-path` for that directory. Every run is recorded. A run with a `--login-arg` is
     recorded beside the run without one, never in its place.
   - It says that the coordinator tells the owner before the run (D80). The owner is told that a
     browser, keychain or sign-in window may appear, and that any such window is closed without
     signing in. Afterwards the coordinator asks the owner whether such a window appeared, and
     records the answer as yes or no beside the probe's results.
   - **Pass.** For each client, both of these hold:
     - (a) The everyday status reports a login and names an account, at step 2 and still at
       step 6. The empty home reports no login, at step 3 and again at step 5, after the
       cancelled login.
     - (b) The browser login gives at least one URL whose return is `redirect` and that fails no
       IFC-049 check. Its listener answers, the client is still waiting when cancelled, and the
       owner saw no browser window open. A `page` URL alone fails (b). The code its page shows
       reaches the client only if carried back, and a Claude login is never carried (CTR-013,
       PND-013, ruling 75 C3). For Codex, 038c's handoff plans a redirect.
   - **What each outcome changes.**
     - Codex's status names no account: order 050 writes Codex's account hook as O3 has it. The
       hook reads the status first and, if that names no account, the email claim of the ID token
       in the home's auth.json
       ([refusals and identity](../proposals/spec-batch-1/areas/refusals-and-identity.md), D2 and
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
   - **Not shown.**
     - A completed sign-in, and the account check.
     - The ID-token route, which is stage 1's.
     - Codex's device login. Its code goes to the engine's panel (order 051c; ruling 78, point 9).
     - The logins' literal origins, paths, client IDs and ports. Order 050 reads those from the
       clients ([038c's handoff](038c-staged/return.md#agent-packages-order-050)).
     - The Windows computer, the second accounts, refresh, lapse and sign-out.

3. **The probe** (FR-427's verifier: a small Rust probe under an empty environment with scratch
   homes).
   - The crate is `experiments/027-a5-sign-in/client-facts/`, edition 2024. It has its own
     `Cargo.lock`, and a `rust-toolchain.toml` like experiment 022's: channel 1.97.1, the engine's,
     with components `rustfmt` and `clippy` and profile `minimal`. Its `.gitignore` is `/target/`.
   - Its dependencies are `serde_json`, `url`, `libc` and `tempfile` only, resolved offline from
     Cargo's cache. The engine's `Cargo.lock` holds each of them.
   - It has three binaries:
     - `client-facts`, the probe.
     - `browser-shim`, which appends each argument it is given, one per line, to the file named
       by `A5_SHIM_LOG`, and exits 0.
     - `fake-client`, which only the tests use (decision 7).
   - The probe's command line is
     `client-facts --claude <path> --codex <path> --out <file> [--extra-path <dir>]... [--login-arg claude=<arg>|codex=<arg>]...`.
     It refuses an `--out` file that already exists.
   - It writes decision 5's JSON to `--out`, and prints the same facts as a Markdown table: one
     row per fact, and one column per client, headed with the client's name and version.
   - It exits 0 when every step ran or was skipped under decision 4, whatever the clients
     answered. It exits non-zero only for a fault of its own, or after decision 6's logout, and
     names the fault or the logout by step and client.

4. **The steps** (FR-427, steps 1 to 3; the commands as
   [the sign-in proposal](../proposals/sign-in/README.md#what-the-clients-offer-read-on-2026-11-10)
   records them).
   - For each client, Claude Code first and then Codex, the probe runs these steps in order:
     1. version: `claude --version`, or `codex --version`;
     2. everyday status: `claude auth status`, or `codex login status`, in the client's own
        default home;
     3. empty-home status: the same command, with `CLAUDE_CONFIG_DIR`, or `CODEX_HOME`, set to a
        new empty directory with mode 0700 inside the scratch root;
     4. login start: `claude auth login`, or `codex login`, followed by that client's
        `--login-arg` arguments, in that same home. The probe waits up to 30 s for an
        authorization URL to appear in the client's output or in the shim's log, then waits 2 s
        more. It checks each loopback redirect's listener, then cancels the login (decision 6);
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

5. **What it records** (IFC-049's checks; FR-427: yes or no, never a value).
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
       other return, gets `not checked`.

6. **Safety, in the code** (AGENTS.md; FR-422's hazard that an undo signs the owner out; FR-427:
   nothing signed in, no turn).
   - The probe reads no environment variable except `HOME`, `USER`, `LOGNAME` and `TMPDIR`. It
     passes the clients none of its own beyond decision 4's.
   - It never opens, lists, reads or copies a file in any client home, the everyday ones
     included. The one exception is removing the scratch root at the end. It never runs
     `security`, and never reads a keychain item or a home's auth.json. The only file it reads is
     the shim's log, in its own scratch root.
   - A client's output is held in memory, at most 64 KiB per stream, then parsed and dropped. It
     is never printed, logged or written. The probe's own messages name a step and a client only.
   - It starts no model turn and completes no sign-in.
   - To cancel a login, the probe sends TERM to its process group, then KILL after 5 s.
     `group_gone` is true only once the probe has reaped the client and sees that the group is
     gone.
   - Where step 5 reports a login in a home that step 3 found empty, the probe runs
     `claude auth logout`, or `codex logout`, in that scratch home only. It records `logged_out`,
     and exits non-zero, naming it. It runs no logout in any other case, and never in a default
     home.
   - The scratch root is a new directory, with mode 0700, under the system temporary directory.
     It is removed at the end, after a fault too.
   - In every file you write, name a client's login file only as "the home's auth.json", never
     with a directory before it: the path check flags that form. Write no home-directory path.

7. **Tests** (the Rust checks; order 151's secret scan).
   - Unit tests run the parsers on fixtures that you invent and never copy from a real client:
     - a JSON status that is logged in and names an email;
     - a text status with a labelled email, and one whose label is not on decision 5's list;
     - a text status that is logged in and names no account, as Codex's is reported to;
     - a status that is not logged in;
     - output with ANSI colours;
     - a URL that passes as `redirect`, one that is `page`, and one that fails each check once.
   - The end-to-end tests run the probe against `fake-client` as both clients, with `HOME` and
     `TMPDIR` set to temporary directories of the test. Name them `end_to_end_print`,
     `end_to_end_browser`, `end_to_end_ignore_term` and `end_to_end_shared_home`.
     - `fake-client` answers `--version`.
     - It answers both status commands: logged in with a planted email when no home variable is
       set, and not logged in otherwise. When a file named `shared-home` exists in `HOME`, it
       reports logged in with any home.
     - It answers both login commands, and appends a line to a file in `HOME` each time a login
       starts. It binds 127.0.0.1 on a free port and gives a planted authorization URL whose
       redirect is that port.
     - It prints the URL by default. With `--mode=browser`, passed as a `--login-arg`, it instead
       runs `$BROWSER` with the URL. With `--mode=ignore-term`, it prints the URL and ignores
       TERM.
   - The first three tests check:
     - the URL is seen and passes;
     - `listens` is `yes`, and `waiting_at_cancel` holds;
     - `ended_on` is `term`, or `kill` under `ignore-term`;
     - `group_gone` holds;
     - `shim_called` holds in browser mode;
     - step 5 reports no login, step 6 reports a login, and `TMPDIR` is left empty;
     - the JSON written holds no `@`, no `://` and none of the planted values.
   - `end_to_end_shared_home` checks that steps 4 and 5 are recorded as skipped for both clients,
     that no login was started, and that no logout ran.
   - Build the planted values in the tests from low-entropy repeats, such as `"s".repeat(32)` for
     a state. Use addresses at `example.org`. That way order 151's secret scan finds nothing, and
     you add no allowlist entry.

## Scope

Change only these files:

- `experiments/027-a5-sign-in/README.md` and the new crate `experiments/027-a5-sign-in/client-facts/`;
- in `ROADMAP.md`, A5's row and the numbering line of "Proposing an experiment";
- in `experiments/README.md`, the new row.

Write no results file and run neither client: the admin does both.

## Done when

In `experiments/027-a5-sign-in/client-facts`, run these one at a time. Each passes, and the last
reports 4 passed:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo test --release end_to_end
```

From the repository root, each of these passes:

```bash
python3 tools/check_roadmap.py
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

`check_private_paths.py` reads only the files Git tracks, so it does not see your new files. Also
run this; it must print nothing:

```bash
grep -rnE '/(Users|home)/[A-Za-z0-9._-]+' experiments/027-a5-sign-in --exclude-dir=target
```

Do not stage anything. Report each choice you make as your own, and every check's last line.

The admin then does the following:

1. It stages the work, and runs `check_private_paths.py` again.
2. It tells the owner that a browser, keychain or sign-in window may appear, and that any such
   window is closed without signing in. Then it runs the probe on the Mac from its own session,
   against the clients signed in to the primary accounts.
3. It asks the owner whether a browser, keychain or sign-in window appeared.
4. In the same pull request it adds:
   - the probe's JSON, as `experiments/027-a5-sign-in/results-DATE.json`;
   - `results-DATE.md`, which opens with decision 2's limit and gives yes or no for each step,
     client and version, with the owner's answer;
   - A5's State, changed to stage 0's outcome, and the "Current evidence" cell of 027's row in
     `experiments/README.md`. Under D78's rule, a probe's result merges with its row's change.
5. `check_private_paths.py` and the pull request's secret scan pass on it.
