# Work order 167: A5's probe: signals and the shim tested, and a signal never cuts a logout

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only because the
sandbox refuses a loopback bind without it, and the probe's tests bind one on 127.0.0.1. Use it for
nothing else: Cargo runs offline and fetches no crate, and you reach no other machine. Do not run
`claude` or `codex`, and run `client-facts` only through its tests, against the crate's
`fake-client`. Outside your working tree, read nothing under your home directory except Cargo's and
rustup's own directories. Run one cargo command at a time, and never print a whole large file or
log. End every process that a test of yours starts. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token, a
password or any credential.

## What and why

[Order 159](159-a5-stage-0-client-facts.md), merged as pull request 280, built `client-facts`, the
probe that A5's stage 0 runs against the real clients, in `experiments/027-a5-sign-in/client-facts/`.
[Its Record](159-a5-stage-0-client-facts.md#record) lists the fixes made after the first Opus
reader's six shoulds. Among them: on SIGINT, SIGTERM or SIGHUP the probe kills the client it
started and removes its scratch root and its output file, and it faults at step 0 when
`browser-shim` is missing. A second fresh Opus reader of those fixes (ruling 79) rated the probe
safe to run and ready to merge, and logged five shoulds. Each is still open on `main`:

1. No test sends a signal or removes the shim. With the signal handlers and the shim check both
   deleted, every end-to-end test still passed.
2. A signal during step 5 cuts the safety logout or keeps it from starting. The probe then says
   only `step 5 NAME: fault`: it names neither the login it found nor the logout, and it removes the
   report. A scratch home that was signed in can stay signed in, its credential perhaps in the
   keychain outside the scratch root, and the admin is not told.
3. An interrupted run says `fault`, as a client's fault does. That falls short of
   [ruling 52](../proposals/ruling-52/README.md)'s rule that a diagnostic says what happened. And a
   signal that comes after the last check, such as one at the end of Codex's step 6, is ignored:
   the run exits 0 and keeps its report, though the README says a signal always removes it.
4. A signal during a step in the owner's own home (steps 1, 2 and 6) KILLs that client at once.
   Before order 159's fix, such a short command finished on its own. A KILL partway through a
   status that rewrites its login or configuration could damage the everyday login.
5. The shim catches only a client that runs `$BROWSER` or finds `open` on `PATH`. A client that
   opens a browser through the macOS API opens the owner's real default browser at the real
   authorization URL, and a session there could complete the sign-in before the cancel. The README
   does not say this, nor that the safety logout is then the only guard.

Since that Record was written, the admin has run the probe once against the real clients; that run
is not yet merged. It runs again, with the owner watching, once this order merges. So its steps,
its JSON and its Markdown table stay as they are. Only what a signal does, the messages that name
it, the tests and the README change.

## The decisions it applies

Line numbers are those of `main` at `632881df`, in `experiments/027-a5-sign-in/client-facts/`
unless another file is named. If a line has moved, find it by its words. Without a signal, the
probe runs as it does now. The only new output of such a run is decision 2's notice.

1. **What a signal cuts, and what it lets finish** (shoulds 2 and 4). Today every command checks
   the signal flag (`interrupted`, `src/run.rs:28-34`) in `start` (:94), in `command`'s loop (:144)
   and in `finish_polled`'s two loops (:359, :377), and `Running`'s drop KILLs it at once
   (:394-401). After this order, once SIGINT, SIGTERM or SIGHUP has come:
   - No command starts, except step 5's, as below. A refused start stops the probe at that step.
   - Step 3's status and step 4's login, in the scratch home, are cut at once, as now. `login`
     keeps its checks (:181, :197).
   - A command of step 1, 2 or 6, in the owner's own home, runs to its end: the client's exit, or
     its 30 s limit and the KILL a timeout gets now, then `finish`'s wait for its group (:349-392).
     No check of the flag cuts it. The probe stops as soon as the command has ended.
   - Once step 3 has reported no login, step 5 runs before the probe stops, wherever the signal
     came after that. Step 4 is refused or cut at once. Step 5's status, and its logout where the
     status reports a login, start even after the signal and run to their end, as steps 1, 2 and 6
     do. Step 6 then does not run. A fault in step 4 that is not a signal still ends the client at
     once and stops the probe with `step 4 NAME: fault`, without step 5, as now.

   How `start`, `command` and `finish_polled` learn which kind a command is, a parameter or methods
   of their own, is your choice. Report it.
2. **A scratch login is named before its logout runs** (should 2). In `client` (`src/run.rs:454-460`),
   as soon as step 5's status reports `login` `yes`, push `step 5 NAME: scratch login found` to the
   list that `main` reports (`logouts`, which you may rename), before the logout starts. Once the
   logout command has returned, push `step 5 NAME: logout`, as now. NAME is `claude-code` or
   `codex`. `main` (:536-541) reports the list as now: its entries joined by "; ", then "; " and the
   error, if there is one. So a run that finds both scratch homes signed in, with no signal, exits 1
   with exactly
   `step 5 claude-code: scratch login found; step 5 claude-code: logout; step 5 codex: scratch login found; step 5 codex: logout`.
   `logged_out` is set as now.
3. **A signal is named `signal`** (should 3; ruling 52, point 2). A client that stops for a signal
   stops with `step N NAME: signal`, never `fault`, after any notices of decision 2. N is the step
   that the signal cut or kept from starting; where it cut none, N is the step during which it
   came, or the last step to end before the probe saw it. Where the signal cut step 4 or kept it
   from starting, N is 4, though step 5 ran after it. A step that fails for any other reason keeps
   `step N NAME: fault`. A fault of a step 5 that runs after a signal stops the probe with
   `step 5 NAME: fault`. After the client loop (:577-605), and before the report is written (:607),
   the probe checks for a signal once more. If one came, it stops with `step 6 codex: signal`, so
   `main` removes the report (:533-535) and the exit is 1. A signal that comes after that check,
   while the report is written or the table printed, is not honoured: the steps are done and the
   report is complete.
4. **The fake client** (`src/bin/fake-client.rs`). Its answers and modes stay. It gains what the
   tests need:
   - Each login appends its own PID, not `yes`, as its line of `login-started` (:73).
   - `--mode=complete-and-wait`: the login writes `signed-in` into its scratch home before it
     appends to `login-started`, then prints its URL and waits, as the default mode does.
   - With a file `slow-status` in `HOME`, a status run with no home variable appends a line to
     `status-started`, sleeps 2 s, answers, then appends a line to `status-done`.
   - With a file `slow-logout` in `HOME`, a logout sleeps 2 s after it appends to `logout-started`
     and before it removes `signed-in`. Every logout appends a line to `logout-done` once it has
     finished.
5. **Six tests** in `tests/end_to_end.rs` (should 1, and one for each of decisions 1 to 3). Each
   run of the probe has a root of its own, set up by `probe` (:7-35), against `fake-client`, with
   standard output and standard error captured; `probe` may take the program to run as a
   parameter, for the shim test. Each of the five signal tests waits until a file in `everyday/`
   holds a given number of lines, polling every 20 ms for at most 60 s. It then sends the signal to
   the probe's PID with `libc::kill`, and waits for the probe. Every test asserts:
   - the probe exits with status 1;
   - its standard error is exactly the line given below, then a newline;
   - its standard output is empty;
   - no `facts.json` exists and `temporary/` is empty;
   - for each PID in `login-started`, `libc::kill(-pid, 0)` fails with `ESRCH`.

   A value whose drop KILLs the probe, if it has not been waited for, and the group of each PID in
   `login-started` not yet seen gone, leaves no process behind when an assertion fails. The tests:
   - `signal_during_login_cleans_up`: in default mode, three runs, sending SIGINT, SIGTERM and then
     SIGHUP once `login-started` has 1 line. Then one run with `--mode=ignore-term`, sending SIGTERM
     3.5 s after `login-started` has 1 line. That lands in the cancel's 5 s wait after TERM, which
     begins about 2 s after the URL. Standard error: `step 4 claude-code: signal`. Also:
     `login-started` has 1 line, and no `logout-started` exists.
   - `signal_during_login_still_checks_the_scratch_home`: `--mode=complete-and-wait`, SIGTERM once
     `login-started` has 1 line. Standard error:
     `step 5 claude-code: scratch login found; step 5 claude-code: logout; step 4 claude-code: signal`.
     Also: `logout-done` and `login-started` have 1 line each.
   - `signal_never_cuts_the_safety_logout`: `--mode=complete` and `slow-logout`, SIGTERM once
     `logout-started` has 1 line. Standard error:
     `step 5 claude-code: scratch login found; step 5 claude-code: logout; step 5 claude-code: signal`.
     Also: `logout-done` and `login-started` have 1 line each.
   - `signal_lets_an_everyday_status_finish`: default mode and `slow-status`, SIGTERM once
     `status-started` has 1 line (Claude Code's step 2). Standard error:
     `step 2 claude-code: signal`. Also: `status-done` has 1 line, and no `login-started` exists.
   - `signal_during_the_last_step_removes_the_report`: default mode and `slow-status`, SIGTERM once
     `status-started` has 4 lines (Codex's step 6). Standard error: `step 6 codex: signal`. Also:
     `status-done` has 4 lines and `login-started` 2.
   - `shim_missing_or_not_executable_faults`: copy the `client-facts` binary into a directory of
     the test, and run that copy twice: first with no `browser-shim` beside it, then beside a
     `browser-shim` that is a regular file of mode 0600. No signal is sent. Standard error:
     `step 0 claude-code: shim fault`. Also: no `login-started` exists.

   `safety_logout_scratch_only` (:153-173) asserts its standard error exactly, as decision 2 gives
   it, then a newline. Every other existing test passes unchanged.
6. **The README**, `experiments/027-a5-sign-in/README.md` (should 5, and decisions 1 to 3). Wrap
   as each section does. The steps, what is recorded, the pass conditions and the outcomes do not
   change. Line 9 stays: the browser check is part of telling the owner first, which it already
   names.
   - After line 5, add a paragraph, with the date you write it in place of DATE: "Amended on DATE by
     work order 167: how a signal stops the probe, the notice of a scratch login found, what the
     shim cannot catch, and the owner's browser check before the run. The steps, what is recorded,
     the pass conditions and the outcomes do not change."
   - In Safety, the bullet "It starts no model turn and completes no sign-in." (line 148) becomes:
     "It starts no model turn, and it cancels every login it starts, but it cannot keep a client's
     browser from completing one. The shim catches a client that runs `$BROWSER`, or `open` or
     `xdg-open` found on `PATH`. A client that opens a browser through the macOS API, or runs
     `/usr/bin/open` by its full path, opens the owner's default browser at the real authorization
     URL. If that browser holds a session with the client's service, the sign-in could complete
     before the probe cancels the login. So `shim_called` false does not show that no browser
     opened: only the owner's answer does. If a sign-in completes, step 5's logout in the scratch
     home, below, is the only guard left."
   - In Safety, the logout bullet (lines 152-155) becomes: "Where step 5 reports a login in a home
     that step 3 found empty, the probe names it at once (`step 5 NAME: scratch login found`, NAME
     being `claude-code` or `codex`). It then runs `claude auth logout`, or `codex logout`, in that
     scratch home only, and names the logout once it has run (`step 5 NAME: logout`). A signal cuts
     neither that status nor that logout. The probe records `logged_out`, and exits non-zero. It
     runs no logout in any other case, and never in a default home."
   - In Running stage 0, after "and any such window is closed without signing in." (line 207),
     add: "The coordinator also asks the owner to make the default browser, for the run, one that
     holds no Anthropic or OpenAI session (claude.ai, the Anthropic Console, ChatGPT or the OpenAI
     platform), for example a browser profile with no sign-ins. A client may open that browser past
     the shim (Safety)." There, "Safety" is a Markdown link to the README's own section Safety.
   - In Running stage 0, the sentence that begins "On SIGINT, SIGTERM or SIGHUP the probe kills"
     (lines 219-221) becomes: "On SIGINT, SIGTERM or SIGHUP the probe starts no further command
     except step 5's, as below. Step 3's status and step 4's login, in the scratch home, have their
     process group killed at once. Steps 1, 2 and 6, in the owner's own home, run to their end or to
     their 30 s limit, so that no client is killed partway through its everyday home. If step 3 had
     reported no login, step 5's status and any logout still run to their end, so that a signal
     never leaves a signed-in scratch home unchecked or its logout cut. The probe then removes its
     scratch root and its output file, and exits non-zero with `step N NAME: signal` after any
     step-5 notices. N is the step the signal cut or kept from starting, or else the step it came
     during; it is 4 where the signal cut the login or kept it from starting, though step 5 then
     ran. A signal that comes once the probe has begun to write its report changes nothing: the
     report is complete and kept." The next sentence, "KILL leaves it no time to do any of that.",
     stays.
   - In the paragraph on exits, "A probe fault or the safety logout at step 5 exits non-zero,
     naming the step and client; every safety logout is named, even when a later step faults."
     (lines 250-251) becomes: "A probe fault, a signal or the safety logout at step 5 exits
     non-zero, naming the step and client: `fault`, `signal`, or step 5's `scratch login found` and
     `logout`. Every scratch login found and every safety logout is named, even when a later step
     faults or a signal comes." In the same paragraph, "A fault removes a report not yet written in
     full" becomes "A fault or a signal removes a report not yet written in full".

## Scope

Change only these files:

- in `experiments/027-a5-sign-in/client-facts/`: `src/run.rs`, `src/bin/fake-client.rs` and
  `tests/end_to_end.rs`;
- `experiments/027-a5-sign-in/README.md`.

Add no dependency: `libc` and `tempfile` are already the crate's. `src/lib.rs`, the parsers, the
shim, `Cargo.toml` and `Cargo.lock` do not change. Write no results file, and run neither client.

## Done when

First, show that each new test, and `safety_logout_scratch_only`, fails with its fix removed. For
each, make a scratch edit to `src/run.rs` that removes the fix, run
`cargo test --release <test name>` in the crate, report the assertion that failed (its line in
`tests/end_to_end.rs`), and revert the edit:

- `signal_during_login_cleans_up`: the loop in `main` that installs the handlers (:515-521). Then,
  in a second edit, decision 3's `signal` name.
- `shim_missing_or_not_executable_faults`: the shim check (:563-567).
- `signal_during_login_still_checks_the_scratch_home`: step 5 after a signal in step 4. The client
  returns at once, as now.
- `signal_never_cuts_the_safety_logout`: the logout honours the signal at once, as `command` does
  now.
- `signal_lets_an_everyday_status_finish`: steps 1, 2 and 6 honour the signal at once, as now.
- `signal_during_the_last_step_removes_the_report`: both checks that honour a signal once a command
  in the owner's home has ended, the one right after it and the one after the client loop.
- `safety_logout_scratch_only`: decision 2's push of `scratch login found` before the logout.

Then, with every edit reverted, run these in `experiments/027-a5-sign-in/client-facts`, one at a
time. Each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release signal_
cargo test --release shim_
cargo test --release end_to_end
cargo test --release
```

The end-to-end binary reports 5 passed for the third command, 1 for the fourth, and 4 for the
fifth, as on `main`. For the last command it reports 12 passed, and the library 4.

From the repository root, each of these passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each of these prints 1. They read the README with its line breaks as spaces, so a rewrap does not
hide a phrase:

```bash
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -c 'step 5 NAME: scratch login found'
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -c 'is the only guard left'
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -c 'holds no Anthropic or OpenAI session'
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -c 'starts no further command except step 5'
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -c 'A fault or a signal removes a report not yet written in full'
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -c 'Amended on 20[0-9-]* by work order 167'
```

Each of these prints nothing:

```bash
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -o 'completes no sign-in'
tr -s ' \n' '  ' < experiments/027-a5-sign-in/README.md | grep -o 'SIGHUP the probe kills the process group'
grep -rnE '/(Users|home)/[A-Za-z0-9._-]+' experiments/027-a5-sign-in --exclude-dir=target
```

`git status --short` lists only the four files in scope. Do not stage anything. Report each choice
you make as your own, and every check's last line.

A fresh Opus reader then reads the result before it merges (ruling 79). After the merge, the admin
runs the probe against the real clients with the owner watching, as the README says.
