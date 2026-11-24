# Work order 310: the usage guard's probe, which asks each team seat's client for its usage and opens no file of the login (A90, DEC-135)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. It needs no network; if a crate is missing from the
cache, fetch nothing and report it. Run one cargo command at a time, and not `tools/gate.py`.
Never print a whole large file, case, diff or list: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Run no `conduit` outside a test,
no `codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

You write the probe and never run it on anything real: `usage-probe` runs only inside the crate's
tests, against the crate's own fake clients and scratch homes. Outside your working tree, read
nothing under your home directory but what cargo and rustup read for themselves.

## What and why

**The owner's words.** [DEC-135](../decisions/log.md#dec-135-2026-11-23-the-usage-guards-probe-runs-at-a-sitting-on-2026-11-24),
"Tomorrow morning (Recommended)": the probe, A0 of the usage guard's design, is written on
2026-11-23 and run with the owner present on the morning of 2026-11-24; it reads each team seat's
usage and does no model work unless the owner says so. DEC-102: no team seat's Agent takes a turn
until the guard is built. DEC-120: "I don't want to modify how the models operate just police
their usage". DEC-123: a call waits for the login's one ask, up to a limit no timed read has set.
DEC-124: the hooks of `codex-login.v2` and `account-home.v2` will read usage this way.

**What is known.** `agent-codex` reads `account/rateLimits/read` through `codex app-server` with
refresh blocked by `CODEX_REFRESH_TOKEN_URL_OVERRIDE` (`read_app_server`,
`packages/agent-codex/src/lib.rs`), after a usage-limit failure alone; order 208 found that
`account/read` under the override wrote no auth.json of a fabricated login
([results](../../experiments/027-a5-sign-in/results-2026-11-19-refresh-probe.md)). No read has been
timed or run on a team seat. `agent-claude-code` passes `--disable-slash-commands`; the Agent SDK's
references document `/usage`, sent as a prompt, returning a `usage_report`, and local commands
"such as `/voice` or `/usage`" that "bypass the query loop". No run has confirmed either.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing starts model work implicitly: the probe sends no app-server method but `initialize`,
`initialized` and the read and no prompt but `/usage`, refuses any other before it is sent, and
stops a client at the first sign of model work it sees; whether `/usage` reaches a model is L1's
question, so the Claude read runs only on the owner's word (What follows); 3, a shared thing is
never copied: it reads the login where it is and opens no file in its home; 4, no vendor assumed:
the experiment, never `spec/`, names the clients (DEC-105).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a probe pre-registered and tested on fakes before its live run, as orders
205b and 208 did, on each vendor's own route: Codex's protocol gives `excludeResetCreditDetails`
"for background usage polls", and the SDK gives `/usage`. Not taken: a turn on a team seat
(DEC-102); reading the login file or the usage endpoint directly (ruling 82, point 1: ask the
client); any value but percentages, times, booleans and an error's code.

**When it runs.** Written on the decision log's branch (DEC-132 to DEC-135 on top of `main`), or
on `main` once it merges, through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). Run by the coordinator
at the sitting (What follows). By ruling 89, each decision says what must hold; how is yours. The
words, names and these choices are the coordinator's, open to the owner's strike (DEC-029): A90
and experiment 033; three Codex reads and a pair, one Claude read; the hold and the deadline;
printing percentages, times and booleans, wider than the design's "presence only", since DEC-135
reads each seat's usage; the required `--model`; the stop at the first sign of model work; the
Codex stop after a change to the login file; the fakes' marker.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

TODAY is the date you start, as `date +%F` prints it.

1. **The register, first** (ROADMAP's "Proposing an experiment"), before any of the probe:
   - In `ROADMAP.md`'s register, after row A85:

     ```markdown
     | A90 | The usage guard's probe (A0 of its design): on each team seat's login, does a refresh-blocked `account/rateLimits/read` through Codex's app-server leave the login file as it was, how long does a read take, which fields does the seat fill, and do two reads at once both answer and leave the login file as it was; and does `/usage`, sent to a stream-json Claude Code client in the seat's home, start no model work and return a `usage_report`? | No engine; the owner present at a sitting (DEC-135); each team seat's login home on the owner's machine; no model work unless the owner says so (DEC-102) | The guard's login packages, `codex-login.v2`'s hook and `account-home.v2`'s `usageRead`, and the limit on a call's wait for a fresh report (DEC-123) | Dated results in [experiment 033] (a link, from the row's own file, to `experiments/033-usage-guard-probe/README.md`), against the questions, predictions and stop rules pre-registered there before its probe was written | next: pre-registered on TODAY by work order 310, with its probe; its run is the coordinator's, at the sitting with the owner | [DEC-135] (a link, from the row's own file, to `docs/decisions/log.md#dec-135-2026-11-23-the-usage-guards-probe-runs-at-a-sitting-on-2026-11-24`) |
     ```

   - The order of the work's "In parallel" row: Items ends `A89, A90`; its Work ends "the
     rulings consolidation (A89); and since DEC-135, the usage guard's probe (A90)."
   - In `experiments/README.md`, after 032's row:

     ```markdown
     | [033 — A90: the usage guard's probe] (a link, from the row's own file, to `033-usage-guard-probe/README.md`) | A90: each team seat's usage, asked of Codex's app-server with refresh blocked and of Claude Code's `/usage`, the probe opening no file of the login and printing only field names, types, percentages, times and booleans. | [Pre-registered] (a link, from the row's own file, to `033-usage-guard-probe/README.md`) on TODAY by work order 310, with its probe; not yet run. |
     ```

   The two table rows above are quoted with their links named, not linked, so the docs guard does not read them from this file; in ROADMAP.md and experiments/README.md write each as a real Markdown link to the target named.

2. **The pre-registration**, `experiments/033-usage-guard-probe/README.md`, wrapped at 100
   characters, written before the crate, with seven sections in this order:
   - `## The question`. C1: does a Codex read change the home's `auth.json` (name, size, inode or
     times) or leave a lock file, and what else changes? C2: per read, the milliseconds from the
     app-server's start to its `initialize` answer and to the read's answer. C3: which fields
     the seat's answer fills, by name and type. C4: two app-servers on one home, each sent the
     read before either answers: do both answer, with `auth.json` unchanged? L1: does `/usage`
     start model work, by decision 6's signs? L2: does it return a `usage_report`, with which
     fields, and does its reply carry "Showing last-known usage", and where? L3: what in the
     home changes?
   - `## The probe`: decisions 3 to 6, with their lists of names.
   - `## The sitting`: on the owner's machine, the owner present (DEC-135), after order 310
     merges: the Codex command on each Codex team seat's login home; the Claude command on each
     Claude team seat's login home with a model the owner names as included in that seat's plan,
     only on the owner's word, given after being told that if the installed version's `/usage`
     does not bypass the query loop, one request reaches that model before the probe can see it,
     and that on a seat whose included usage is spent and whose extra usage is on, it may bill
     credits; stop rule 2 applies after it. The output kept under the crate's `target/` and
     quoted in the results only as printed.
   - `## Predictions`, the coordinator's from the vendors' documentation and Codex's source, not
     results: C1, `auth.json` unchanged and no lock file left, as order 208 found for
     `account/read`, while the app-server's own state files may change; C2, every read answers
     within the 30 seconds the guard's design assumes for the wait (DEC-123); C3, the windows and
     `ordinaryUsageAllowed` filled, and no source says whether a team seat fills `credits`,
     `individualLimit` or `spendControlReached`; C4, both answer, `auth.json` unchanged; L1, no
     model work; L2, a `usage_report` whose `rate_limits.limits` rows no source says a team seat
     fills; L3, the client writes its own state, such as `.claude.json`, and may refresh the
     login.
   - `## What each outcome changes`: C1 and C4 as predicted, `codex-login.v2`'s hook may read
     through a refresh-blocked app-server, beside the login's other clients; otherwise the
     outcome goes to the owner (by stop rule 3 where the login file changed) before that hook is
     written. C2: the slowest read's `ms_to_answer` sets the limit
     on a call's wait (DEC-123), 30 seconds until a read is timed. C3: the fields each seat fills
     are those its hook may decide from. L1 and L2: no model work and a `usage_report` let
     `account-home.v2` offer `usageRead: when-stale`; otherwise `usageRead` is `never`. The
     last-known note: found, the package can tell a last-known report from a fresh one; not
     found, a `/usage` report counts as observed 60 minutes before the read. L3: a change to the
     login goes to the owner before `usageRead` is offered.
   - `## Stop rules`: (1) the probe refuses, or stops on a survivor: nothing more runs on that
     seat until the cause is read; (2) a Claude read shows model work: no Claude read runs again,
     and the owner is told; (3) a Codex read changes `auth.json` or leaves a lock file (the probe
     then starts no more clients): no Codex read runs again on any seat, and the outcome goes to
     the owner as it stands, as ruling 86, point 5, asks of a write under the override; (4) no
     read is repeated to change an outcome.
   - `## Not shown`: one machine, one morning, the clients as installed. A refresh the Claude
     client writes outside the home, such as to a keychain item, is not seen. No turn runs, so
     whether turns emit `rate_limit_event` at `allowed`, whether `/usage` rows and the stream's
     events name matching windows, and whether a read during a turn leaves its refresh alone stay
     open: each needs model work, which waits for the owner's word (DEC-102, DEC-135).

3. **The crate**, `experiments/033-usage-guard-probe/probe/`: package `usage-guard-probe` 0.1.0,
   edition 2024, `publish = false`, `[lib] doctest = false`; binaries `usage-probe` and, for the
   tests alone, `fake-codex` and `fake-claude` under `src/bin/`, as
   `experiments/027-a5-sign-in/refresh-probe/` keeps its fake app-server; dependencies exactly
   `serde_json = "=1.0.154"` and `libc = "=0.2.191"`, as that probe pins them, starting from its
   `Cargo.lock`; `rust-toolchain.toml` as its. Unix only: elsewhere a command says so and exits 1.
   - **Commands.** `usage-probe codex --home DIR --codex PATH --expect-version V` and
     `usage-probe claude --home DIR --claude PATH --expect-version V --model NAME`, each with
     `--deadline-seconds N` (60 by default, 1 to 600; per client, from its spawn to its answer
     or its `result`) and `--hold-seconds H` (10 by default, 0 to 60). `--home`, the client's
     path, `--expect-version` and `--model` are required, with no default. Each refuses, with one
     line on standard output naming the check and exit 1, where a required flag is missing or
     empty, the client's path is not absolute, `DIR` is not a directory, or a Codex `DIR` holds
     no `auth.json` (by `stat` alone). It runs `PATH --version` as a client (below), under the
     deadline, with `HOME` and the home variable set to a new empty directory, takes the first
     token shaped `N.N.N` with any suffix, and prints `codex.version_matches` (or `claude.`);
     where false it runs nothing more, exit 1. Then the reads; exit 0 once each ran.
   - **The client.** Its environment is cleared, then given `HOME`, `LOGNAME`, `PATH`, `SHELL`,
     `TERM` and `USER` where the probe's has them (the packages' `BASE`), the home variable
     (`CODEX_HOME` or `CLAUDE_CONFIG_DIR`) set to `DIR` made absolute, and for Codex alone
     `CODEX_REFRESH_TOKEN_URL_OVERRIDE` set to the package's `REFRESH_BLOCKED`; nothing else. The
     probe reads those six and `TMPDIR` (for the system's temporary directory), no other
     variable, and prints none. The client runs in a new empty directory under the system's
     temporary directory, its standard error discarded, in its own process group; a stop sends
     TERM to the group, KILL after 2 seconds (`STOP_GRACE`), reaps the client, and only then
     prints `group_stopped` from `killpg` with signal 0. Scratch directories are removed before
     exit. On SIGINT, SIGTERM or SIGHUP the probe stops every live group as a stop does, reaps
     its clients, removes its scratch directories, prints `interrupted: true` and exits 1.
   - **Codex's read.** `codex app-server --stdio -c cli_auth_credentials_store="file"` (the
     package's `account_arguments`): `initialize` with `clientInfo` `usage-guard-probe` 0.1.0 and
     `experimentalApi` false, `initialized`, then `account/rateLimits/read` with exactly
     `{"excludeResetCreditDetails": true}`; lines carrying a `method` are skipped, as the
     package's `ask` skips them. One function writes every message and refuses, before a byte is
     written, any method but those three and any other read params: never `supportsLunaReserve`,
     `account/read`, a thread or turn method, `account/rateLimitResetCredit/consume`,
     `account/sendAddCreditsNudgeEmail`, or a login or logout method. Three reads in turn
     (`codex.read.1` to `.3`), each its own app-server, then `codex.pair`: two app-servers, both
     initialized, both sent the read before either answer is read. Each app-server is held H
     seconds after its answer, as order 208 held its, then stopped; the home is listed before it
     starts and after its group is gone (for the pair, around both). Each read prints `answered`,
     `error_answer` (a boolean) and, where true, `error_code` (the error's number),
     `ms_to_initialized` and `ms_to_answer` from the spawn, `ms_request_to_answer` and
     `group_stopped`; read 1 prints its answer's fields, reads 2 and 3 `same_fields` (read 1's
     paths and types); the pair `both_answered`, each one's times and `overlapped`: both reads
     were written before either answer line arrived, by the times a reader per client stamps on
     each line. After each read's after-listing, and the pair's, where `home.login_file_unchanged`
     or `group_stopped` is false, the probe starts no more clients, prints
     `codex.stopped_on_login_change: true` or `codex.stopped_on_survivor: true`, removes its
     scratch directories and exits 1.

4. **What it prints**: on standard output only lines `KEY: VALUE`, each checked against
   `^[A-Za-z0-9_.?\[\] :+-]{1,200}$`; a line that fails is dropped, and the command prints
   `unprintable_lines: true` and exits 1. On standard error only lines that pass the same check: a
   panic hook prints the one fixed line `panicked: true`, and no error text carries a path or a
   client's bytes. An answer prints one line per field, `PREFIX.field PATH: TYPE`, then ` VALUE`
   where allowed:
   - PATH joins keys with `.`; a key not among the names below prints as `?`. Array elements,
     and the entries of `rateLimitsByLimitId`, keyed by data, print as `[1]`, `[2]`, … in the
     order received; their keys never print. TYPE is `object`, `array`, `string`, `number`,
     `boolean` or `null`.
   - VALUE: every boolean's; a number's only under a percentage name (`usedPercent`,
     `remainingPercent`, `percent`, `utilization`) or a time name (`resetsAt`, `resets_at`,
     `overageResetsAt`, `windowDurationMins`); a string's only under a time name and in RFC
     3339's shape; under a word name, `word:true` or `word:false`, whether the string is one of
     the name's documented words, never the word.
   - Names, Codex (the read's answer, as `packages/agent-codex/src/usage.rs` and Codex's protocol
     name it): `ordinaryUsageAllowed`, `rateLimits`, `rateLimitsByLimitId`,
     `rateLimitResetCredits`, `accountId`, `rateLimitUpsell`, `limitId`, `limitName`,
     `normalModelSlug`, `primary`, `secondary`, `usedPercent`, `windowDurationMins`, `resetsAt`,
     `credits`, `hasCredits`, `unlimited`, `balance`, `individualLimit`, `limit`, `used`,
     `remainingPercent`, `spendControlReached`, `planType`, `rateLimitReachedType`. Claude (the
     SDK's `SDKUsageReport` and rate-limit event, and raw fields users report): `message`,
     `usage_report`, `rate_limits`, `limits`, `kind`, `group`, `percent`, `resets_at`, `scope`,
     `model`, `surface`, `severity`, `is_active`, `extra_usage`, `is_enabled`, `monthly_limit`,
     `used_credits`, `utilization`, `currency`, `rate_limit_info`, `status`, `resetsAt`,
     `errorCode`, `canUserPurchaseCredits`, `hasChargeableSavedPaymentMethod`, `overageEnabled`,
     `rateLimitType`, `overageStatus`, `overageResetsAt`, `overageDisabledReason`,
     `isUsingOverage`, `unifiedWindows`, `five_hour`, `seven_day`.
   - Word names: `kind` (`session`, `weekly_all`, `weekly_scoped`); `severity` (`normal`,
     `warning`, `critical`); `status` (`allowed`, `allowed_warning`, `rejected`); `rateLimitType`
     (`five_hour`, `seven_day`, `seven_day_opus`, `seven_day_sonnet`, `overage`);
     `rateLimitReachedType` (`rate_limit_reached`, `workspace_owner_credits_depleted`,
     `workspace_member_credits_depleted`, `workspace_owner_usage_limit_reached`,
     `workspace_member_usage_limit_reached`).

   So it never prints, on either stream, a token, header, cookie, account ID, email, amount,
   plan, model name, path, the client's text or standard error, or anything of a file in the
   home.

5. **The listing.** Every entry under the home by `symlink_metadata`, never opening a file or
   following a link to a directory: its path relative to the home, kind, size, inode, and
   modification and change times to the nanosecond the system gives, and for a link its target's
   size and times, never the target's path. It compares all of these before and after in memory
   and prints `home.login_file_unchanged` (Codex alone: `auth.json` unchanged, and no top-level
   name beginning `auth` or `.auth` added or removed), `home.unchanged`, and, per top-level entry
   under which anything was added, removed or changed, `home.changed NAME: added`, `removed` or
   `changed`, NAME the entry's name where it matches `^[A-Za-z0-9._-]{1,64}$` and holds no run of
   8 or more hex digits (a UUID's first group is one), else `?`. No deeper name prints.

6. **Claude's read**, one, `claude.read`: `claude --print --input-format stream-json
   --output-format stream-json --verbose --include-partial-messages --no-session-persistence
   --setting-sources "" --settings S --strict-mcp-config --tools "" --model NAME`, S a
   `settings.json` in its working directory holding `{"disableAllHooks": true}`, and without the
   packages' `--disable-slash-commands`. It writes one input line, the package's `run_turn` frame
   with content `/usage`, and its writing function refuses other content. It reads events to the
   `result`, closes the input, waits up to H seconds for an exit, then stops the group. The home
   is listed around it, without the login-file line.
   - **Signs of model work:** an event of type `stream_event`; a `system` event of subtype
     `api_retry`; an `assistant` event whose `message.usage` gives a token count above 0; a result
     whose `usage` gives one, whose `modelUsage` has an entry, or whose `total_cost_usd` or
     `duration_api_ms` is above 0. A sign seen only in the result comes after the work. At the
     first sign it stops the group at once and prints `model_work: true` and
     `stopped_on_model_work: true`; else `model_work: false`.
   - It prints `answered` (a result came), `ms_to_report` (to the first event holding
     `usage_report` at any depth), `ms_to_result`, `ms_api` (the result's `duration_api_ms`),
     `terminal_reason_absent`, `usage_report_present`, `rate_limits_null`, `last_known_note`
     (some string value of an event up to the result holds "last-known", case ignored, the
     string never printed), `last_known_where` (the first such event's kind, as `assistant`,
     `result`, `system`, `user` or `other`) and `group_stopped`; then the fields of
     `usage_report` and of each `rate_limit_event`'s `rate_limit_info`, by decision 4.

7. **The tests.** `cargo test --offline --release` runs on the Mac and CI's Ubuntu against the
   fakes alone. The tests are the 19 that Done when's `T` names, each a top-level `#[test]`
   function in `tests/*.rs` with literal expectations. Each runs `usage-probe` with a cleared
   environment, `HOME` and `TMPDIR` set to scratch directories under a test root in the crate's
   `target/`, and `--hold-seconds` of 0 to 3 unless it needs more; the test writes the marker
   file `fake-root` in that root. A fake acts only where a directory above both its `HOME` and
   its home variable holds that marker; else it exits 1, touching nothing. It reads its script
   from `fake-script.json` there, answers `--version`, logs there, one file per process, its
   arguments, the messages it received and its variables' names (never values; for the
   override, whether it is the blocked address), and does what the script says: answer late or
   never, rewrite `auth.json` in place or by rename, change only its time, restore its time after
   a rename, leave `auth.lock`, remove or add a file, write 3 seconds after answering, start a
   descendant that ignores TERM, write barred values to standard error, emit model-work events or
   the last-known note. Build every token-shaped value at run time from parts, so that no
   literal looks like a secret to CI's scanner. Among the tests: `refuses_any_other_method`
   (each barred method and other read params; nothing reaches the child),
   `codex_environment_exact` (a fake `OPENAI_API_KEY`, `ANTHROPIC_API_KEY` and
   `CODEX_REVOKE_TOKEN_URL_OVERRIDE` in the probe's environment reach no fake, the version run's
   included), `codex_stops_after_login_change` (the fake rewrites `auth.json` on read 1:
   `codex.stopped_on_login_change: true`, exit 1, and the logs show no second app-server),
   `home_without_login_file_refused` (also a missing `--home`, client path or
   `--expect-version`, and a relative client path: each refused, no fake started),
   `fields_printed_by_rule` (every name filled, an unknown key, two `rateLimitsByLimitId`
   entries), `barred_values_never_printed` (an email at `example.invalid`, an account ID, a
   balance, a limit, a JWT-shaped string, a cookie and a header line, in answers and standard
   error: none, nor a scratch path, on the probe's standard output or standard error),
   `listing_detects_each_change` (each change above, a deeper addition, none; a top-level name
   with 8 hex digits printed as `?`; a mode 000 file listed, never opened), `durations_measured`
   (answers 300 and 500 ms late), `deadline_stops_the_group` (`--deadline-seconds 2`, the
   descendant gone; and SIGTERM to the probe in a hold: `interrupted: true`, exit 1, every group
   gone, the scratch directories removed), `claude_model_work_stops_the_client` (a
   `stream_event`, then silence: stopped within 3 seconds; a result whose only sign is
   `total_cost_usd` above 0: `model_work: true`), `claude_last_known_note` (the note in an
   assistant event, then in the result's `result` string: `last_known_where` `assistant`, then
   `result`) and `output_lines_checked` (a line with an at sign, a slash or over 200 characters
   is dropped, on both streams).

   Then show a lost refusal is caught: copy the file holding decision 3's method check to
   `probe/target/send.rs.fixed`, make the check accept every method, run
   `cargo test --offline --release refuses_any_other_method`, which must fail, restore it with
   `cp`, rerun, which passes, and `cmp` the two, which prints nothing.

## Start check

```sh
grep -c '^## DEC-135, 2026-11-23, ' docs/decisions/log.md  # 1
cat ROADMAP.md docs/AUDIT.md | grep -c '^| A90 |'  # 0
ls experiments | grep -c '^03[3-9]-'  # 0
grep -c '| A70, A71, A4, A5, A6, A7, A8, A10, A22, A74, A77, A80, A85, A86, A87, A89 |' ROADMAP.md  # 1
grep -c 'since ruling 92, the rulings consolidation (A89)\. |' ROADMAP.md  # 1
grep -c '^| A85 | The gateway probe' ROADMAP.md  # 1
grep -c '^| \[032 — A85: the gateway probe\](032-gateway-probe/README.md) |' experiments/README.md  # 1
grep -c 'pub const REFRESH_BLOCKED: &str = "https://refresh.invalid/oauth/token";' packages/agent-codex/src/lib.rs  # 1
grep -c '"account/rateLimits/read",' packages/agent-codex/src/lib.rs  # 1
grep -c '"--disable-slash-commands",' packages/agent-claude-code/src/lib.rs  # 1
grep -cE '^(serde_json = "=1\.0\.154"|libc = "=0\.2\.191")$' experiments/027-a5-sign-in/refresh-probe/Cargo.toml  # 2
grep -c '^channel = "1.97.1"$' experiments/027-a5-sign-in/refresh-probe/rust-toolchain.toml  # 1
```

## Scope

Create `experiments/033-usage-guard-probe/`: `README.md`, and `probe/` with `Cargo.toml`,
`Cargo.lock`, `rust-toolchain.toml`, `src/` and `tests/`. Change `ROADMAP.md` (row A90 and the
"In parallel" row) and `experiments/README.md` (one row). Nothing else: not `spec/`,
`conformance/`, `engine/`, `packages/`, `tools/`, `docs/` or `.github/` (the gate and CI find the
crate by its `Cargo.toml`). Build output stays in `probe/target/`, which Git ignores.

## Done when

Write decisions 1 and 2 first, then the crate. Each line prints otherwise before this order and
passes after:

```sh
E=experiments/033-usage-guard-probe; P=$E/probe
grep -c '^| A90 | The usage guard' ROADMAP.md  # 1; 0 before
grep -c 'A85, A86, A87, A89, A90 |' ROADMAP.md  # 1; 0 before
grep -c '^| \[033 — A90: the usage guard' experiments/README.md  # 1; 0 before
grep -s -c '^## ' "$E/README.md"  # 7; nothing before
(cd "$P" && cargo fmt --check >/dev/null 2>&1); echo "fmt $?"  # fmt 0; fmt 1 before
(cd "$P" && cargo clippy --offline --all-targets -- -D warnings >/dev/null 2>&1); echo "clippy $?"  # clippy 0; clippy 1 before
(cd "$P" && cargo test --offline --release >/dev/null 2>&1); echo "test $?"  # test 0; test 1 before
T='codex_sends_only_the_three_methods|refuses_any_other_method|codex_environment_exact|codex_stops_after_login_change|fields_printed_by_rule|barred_values_never_printed|listing_detects_each_change|late_write_in_the_hold_detected|durations_measured|deadline_stops_the_group|pair_overlaps|version_mismatch_runs_nothing|home_without_login_file_refused|claude_arguments_input_and_environment|claude_usage_report_read|claude_model_work_stops_the_client|claude_last_known_note|claude_requires_model|output_lines_checked'
(cd "$P" && cargo test --offline --release -- --list 2>/dev/null) | sed -n 's/: test$//p' | grep -cxE "$T"  # 19; 0 before
```

Guards, which pass before and must still pass:

```sh
export PYTHONDONTWRITEBYTECODE=1
E=experiments/033-usage-guard-probe
git add -N "$E"
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_roadmap.py | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE "^(ROADMAP\.md|experiments/README\.md|$E/(README\.md|probe/(Cargo\.(toml|lock)|rust-toolchain\.toml|(src|tests)/.+\.rs)))$" | wc -l | tr -d ' '  # 0
git reset -q -- "$E"  # stage nothing
```

Report the README's section headings, each test's result, the lost-refusal check's two runs,
every check's last line, and each choice you made yourself.

## What follows

- **The run is the coordinator's**, at the sitting of 2026-11-24, with the owner present and told
  first what runs (DEC-135), after this order merges: on the owner's machine,
  `cargo build --offline --release` in the crate; `usage-probe codex` on each Codex team seat's
  login home and `usage-probe claude` on each Claude team seat's, with a model the owner names as
  included in that seat's plan, each with the client version installed there; the output kept
  under the crate's `target/`; the README's stop rules applied. Before any Claude read, the owner
  is told that if `/usage` does not bypass the query loop in that version, one request reaches the
  named model before the probe can see it, and that on a seat whose included usage is spent and
  whose extra usage is on, it may bill credits; the Claude read runs only on the owner's word at
  the sitting, and stop rule 2 applies after it. No other model work runs unless the owner says
  so (DEC-102). A model-work step (a turn, for `rate_limit_event` at `allowed` and whether
  `/usage` rows and the stream's events name matching windows; a read during a turn) is not in
  this probe: if the owner asks for it at the sitting, it comes as its own order, behind its own
  flag.
- **The results**, the coordinator's: `results-2026-11-24.md` in the experiment, with A90's State
  and 033's row, in the change that records them.
- **They feed** the limit on the guard's wait in its engine order (30 seconds in the design until
  a read is timed); the package orders for `codex-login.v2`'s hook and for `account-home.v2`'s
  `usageRead` and its `/usage` time rule; and the log entry on which children get the refresh
  override, which the hook's read joins.

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on `main` with the decision log
through DEC-135. Written, built, read and fixed in stage 3 of experiment 028; it is no order of
stage 3. The live run is the coordinator's, at the sitting of 2026-11-24 with the owner present.

**Worker-1's turns** (14:35 to 15:01). The first stopped rightly on the docs guard, which read two
links quoted in this order's text; the coordinator named them. The second built the probe and its
19 fake-client tests.

**Reads and fixes.** Both blind reads ran at 15:05: the Codex verifier, `root-verifier-1.turn.v1`,
had four safety musts (scratch placement, a symlink race, a panic, a discarded reply) and a should;
the Claude reader (`reader-1`) found no way to print a secret and had four shoulds. One fix turn by
worker-1 applied all nine (30 tests). Under ruling 87 the coordinator then fixed what later reads
found, test first each time: a re-verification's two musts (scratch overlap by directory identity;
output failures that panicked), a third read's must (the wait after Claude's answer unwatched) and
seven shoulds, and a final read's four musts (a remount with another device, a detached
descendant, Codex notifications unchecked, cleanup through a replaced directory). A check confined
to those four found each fixed and no path into a login home. The probe has 45 fake-client tests.
The gate's run on the squash is the coordinator's, at the merge.
