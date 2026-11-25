# Work order 319: the engine asks a login for its admission report and a call waits for it (DEC-123, DEC-168, DEC-169, DEC-170, the usage guard's third engine order)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network, and every crate it builds is
cached: run cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one
cargo command at a time, and not `tools/gate.py` or the whole engine suite. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere: every implementation a test meets is scripted. Open,
copy or read no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.**
[DEC-123](../decisions/log.md#dec-123-2026-11-23-a-call-waits-in-the-request-for-a-fresh-usage-report),
"Wait in the request (Recommended)": with no current report, a call that starts model work waits
for the login's one ask, bounded by an implementation-defined limit, and is then dispatched or
held on its answer, as an admission webhook does; DL3 and DL4 are amended for this wait.
[DEC-120](../decisions/log.md#dec-120-2026-11-23-the-guard-checks-before-each-turn-and-a-started-turn-runs-to-its-end),
"I don't want to modify how the models operate just police their usage", then "Yes, check before
turns only (Recommended)". The owner struck three of the coordinator's choices in order 316:
DEC-168, "Record the ask's cause": the records of a login's report ask name the invocation whose
wait caused it; DEC-169, "Change 'levers-changed' hold": when a control changes a login's levers,
the report it ended holds no call, and the calls waiting on it wait for a fresh ask, within DL21;
DEC-170, "Dispatch-time check instead": when a call's wait ends, its access is checked once at
dispatch against the Conduits as they then stand, and a call that no longer has access ends
unavailable naming why, with no second access-decision record. DEC-168 to DEC-170 are later
decision log entries, quoted here as the tree this order runs on may not hold them yet.
[DEC-094](../decisions/log.md#dec-094-2026-11-23-usage-is-self-reported-by-packages-and-the-engine-trusts-the-report)
(the engine trusts the package's report) and
[DEC-121](../decisions/log.md#dec-121-2026-11-23-one-login-resource-per-machine-each-with-its-own-levers)
(one login per account per machine, each with its own report) stand behind it.
[Order 316](316-admission-hold-before-a-turn.md) wrote PND-016 and DL21, with the three strikes
applied; [order 317](317-engine-checks-admission-and-keeps-reports.md) built CTR-015's checks,
IFC-054's keeping and the status. This order builds PND-016's ask and wait: the engine's own call
of `admission-hook.v1`, one per login, for which a call waits, bounded by DL21; the 60 seconds
after an ask that gave no report; a cancel, a restart and a repeat while waiting; the ask's cause
in its records; the check once at dispatch; and the first build's rows for DL3, DL4 and DL21. It
holds no call on a report or for want of one: that is the hold's order (E2a,
[DEC-095](../decisions/log.md#dec-095-2026-11-23-a-guarded-hold-reports-unavailable-at-the-login-hold)).

**Today** (`main` with the decision log through DEC-157, and the builds of orders 307, 314, 315,
316 and 317 on top), no engine source names PND-016, DL21 or `causedBy`. `invoke_inner`
(`requests.rs`) decides access and records it, then the sign-in return, CTR-015's control pair,
conflicts and the keyed repeat (OPS-032 for one in progress), the preflights, availability, and,
for an operation whose contract states `startsModelWork: true`, PND-005's and PND-011's holds
(`login_resource::hold_work`, then `sign_in::request_is_held` and
`request_is_held_for_sign_in_undone`); then it makes the invocation, keeps the access record's ID
as `login_request_record` for a call on an Agent with a login place, and dispatches it, or its
discovery, at once. `admission.rs` keeps a report from an `admission-hook.v1` entry that names no
Conduit (`Report::take`, `Report::record_end`), ends it (`end`, `control_succeeded`), and builds
the status with `asking` and `noReport` null; no transition makes such an entry, and order 317's
tests put one in place with `GuardScenario::ask`. `dispatch_hook` makes a part: its `setup_parent`
and `serving` are its parent's ID, and it takes the parent's caller, sender and filters.
`record_content` copies an invocation's caller, sender, Conduit and `serving` into its records.
`recover` ends an entry whose discovery is pending and that is no hook unavailable under REC-007
(`reject_unsent`), and every other in progress unknown under IMP-007; `cancel` forwards a
cancellation and ends the call under OPS-027. In the shell, the one timer that ends a wait is
`end_sign_in_wait_at` (`server.rs`), which submits `sign-in-wait-ended` at DL18's expiry while
`serve` holds the store; `Config`'s `call_timeout` (DL10, 120 seconds) is set by
`--call-timeout-seconds`. In `engine/CONFORMANCE.md`, DL3's row says "There is no autonomous clock
or actor-reply timeout in the core", DL4's "no waiting queue", and order 317's section that the
engine starts no call of `admission-hook.v1` yet. So `requests-admission-report-kept` and order
316's four guarded cases first fail at step 16, where the engine dispatches `a`'s turn with no
ask; and `requests-admission-not-checked`, which order 317's status was built to pass (its Shared
files paragraph), is still listed in `conformance/expected-failures.yaml`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6, the
ask starts no model work and no inspection starts one, a waiting call is dispatched at most once,
and nothing is queued or resumed: a cancel, a closed Conduit or a restart ends it undispatched; 2,
the ask grants nothing and goes through no Conduit, access is decided once, before any ask, and a
call whose Conduit closed while it waited is not dispatched, so model work starts only through an
open Conduit (IMP-001); 7, the ask's records name its caller and the request whose wait caused it,
and the status names the ask; 1, the engine keeps a report as received and decides nothing from
its words here beyond IFC-054's validity and single use, and nothing of the guard reaches an Agent's
implementation. At risk, none in use:
this build holds no call, so a refusing report or an ask that gave none still lets the call run,
but no package states `admission` (the Start check's twenty-second printed line), so no store but
a test's holds a guarded login, and DEC-102 keeps the team seats from turns until the hold merges.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
PND-016 and DL21 are musts since order 316, and the hold acts on what this order asks. By
existing practice, a Kubernetes API request waits for its validating admission webhooks, each
bounded by `timeoutSeconds` (default 10, at most 30); a cache lets one request fill a missing
entry while the others wait on it, for a bounded time (nginx's `proxy_cache_lock` and
`proxy_cache_lock_timeout`); a resolver keeps a server failure for a while so that retries make no
new queries (RFC 2308, section 7); and the engine's own sign-in wait is ended by the shell's timer
submitting an input, not by a clock in the core (DL18). Not taken: holding a call (the hold's
order); a second access decision (DEC-170); queueing or resuming a call (DEC-095); falling back on
an older report; a clock in the core; a configurable 60 seconds, which PND-016 fixes; storing a
control's levers (REV-007, outside this chain so far); retrying an ask that gave no report.

**When it runs.** It stacks on orders 307, 314, 315, 316 and 317 and runs on 317's build (`main`
with the decision log through DEC-157, and the builds of 307, 314, 315, 316 and 317 on top) until
they merge, then on `main`. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each
decision says what must hold; how is yours. It is large for one turn, so it may be dispatched in two
halves from the start, each with its own turn, reads, fix turn and gate: 319a, the ask and the wait
in the core (decisions 1 to 4, the status's `asking`, three core tests, the store test, the expected
failures and the row DL4), and 319b, on 319a's build, what changes, ends or bounds a wait (decisions
5 to 8, the status's `noReport`, four core tests, the timer's test, the option, the README sentence
and the other rows and section). *In two halves*, after the Done when, gives each half's Start check
and Done when. The coordinator's, open to the owner's strike (DEC-029): this build holding no call,
a request PND-016 would hold being dispatched as with no `admission` stated until the hold's order;
DEC-169's and DEC-170's waits built here, though their log entries name the hold's engine order
after order 316, as the wait is this order's and they change it, the hold's order keeping them; a
waiting request as an invocation entry in progress and not dispatched, its reply pending, naming it
and the ask in `hook`; the ask's state in a new `State` member, `admission_asks`; the ask with no
key and no `consumer`, its arguments taken when it starts; waiting requests decided earliest first;
the status giving the 60 seconds in `noReport` though nothing is held in them; a waiting request
counted as no call in progress for PND-005; no cancellation sent for a request that reached nothing;
a repeat of a waiting request answered with OPS-032's invocation ID and PND-016's diagnostic; DL21
at 30 seconds by default, set by `Config` and an option, ended by the serving shell's timer through
a core input, `admission-ask-deadline`, at which the core sends a cancellation of the ask's call,
the timer started for each newly started ask that the shell's commit path hands its executor; the
diagnostics' words, the test names, the store format and the rows' words.

**Shared files.** Order 318, the warnings' spec, changes PND-016, IFC-054 and
`admission-hold.v1`, adds cases and expected failures and regenerates `engine/STANDING.md`; order
305 changes REC-002 and `record.v1`; DEC-149's orders 311 to 313 change core `requests.rs` and
`records.rs`, the shell's store and `engine/CONFORMANCE.md`, and 311 drops `held_target` from an
ended entry, so read an ask's held Type from the entry as `complete` holds it, as
`admission::Report::take` reads it. Whichever merges later rebases and reruns
`promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs. PND-016's
conflict for a request that matches a waiting one narrows OPS-032, whose diagnostic otherwise names
OPS-032 (decision 7): that is no contradiction to stop on; apply it.

## The decisions

Read first PND-016, IFC-054, CTR-015, PND-005, PND-011, OPS-007, OPS-009, OPS-010, OPS-013,
OPS-023, OPS-027, OPS-032, REC-001, REC-002, REC-007, IMP-001, IMP-007, ACC-019, ACC-036,
ACC-037, ACC-039, IFC-028 and OBS-006, and DL3, DL4, DL10, DL18 and DL21; the schemas `record.v1`
(its `causedBy`), `admission-report.v1` and `admission-status.v1` (`spec/core/results.yaml`) and
`admission-hook.v1` (`spec/core/operations.yaml`); orders 316 and 317 whole, with their Records;
the six cases `requests-admission-*` (`conformance/conventions/requests/admission-*/`); and the rows
`DL3: waiting, implemented portion`, `DL4`, `DL10: process and call deadlines` and `DL18` of
`engine/CONFORMANCE.md`, with order 317's section and order 288's paragraph. In the core,
`invoke_inner`, `dispatch_hook`, `discovery_intent`, `dispatch_intent`, `complete`,
`complete_discovery`, `finish`, `reject_unsent`, `recover`, `cancel`, `record_content` and
`link_access` in `requests.rs`; `hold_work` in `login_resource.rs`; `admission.rs` whole;
`limits::status`; `access::evaluate` and `access::may_inspect`; `sign_in::sign_in_wait_ended`;
`Input`, `State` and `Transition` in `lib.rs`; `covers` and `from_trial` in `record_fault.rs`;
order 317's `GuardScenario`, `report_one`, `limits_member` and `control_project` in
`admission_tests.rs`. In the shell, `Config`, `Executor` with its defaulted `tools_changed`,
`Shell::open` (an open recovers), `commit_input`, `begin`, `drive_committed` and the input kinds
that need evidence in `lib.rs`, `--call-timeout-seconds` in `main.rs` and `help.rs`, the runner's
completions through `Shell::complete` in `runner.rs`, `end_sign_in_wait_at` and `ServingExecutor`
(its `changes` and its `tools_changed`) in `server.rs`, `relayed_sign_in_starts_the_expiry_timer`,
`established_case`, `fixed_case_shell` and `fixed_endpoint_shell` in `server_tests.rs`,
`Deferred`, `established`, `open` and `assert_memory_equals_disk` in `tests.rs`,
`init_writes_the_flow_skeleton_unless_a_manifest_is_there` in `tests/context.rs`, and the sentence
of runner settings in `engine/README.md`. *A guarded call*
is a request, or a new attempt of one (OPS-012, OPS-024), for an operation whose contract states
`startsModelWork: true` to an Agent whose login place refers to a Resource whose Type states
`admission`, its *login*. A case's steps are named as the runner prints them, counted from 1.

1. **The wait** (PND-016, OPS-023, PND-005, PND-011). In `invoke_inner`, once OPS-023's reasons
   are decided and PND-005's and PND-011's holds apply none, a guarded call is decided on its
   login's newest kept report. Where that report is current at the step's time (order 317's
   `Kept` whose `end_at` at the step's time gives no end), the call is dispatched as today and
   nothing is asked; a single-use report
   so dispatched on ends `single-use`, its first end. Where none is current, the call waits: its
   invocation entry, with the ID it keeps if it is dispatched, stays in progress, not dispatched,
   with no `invocation-dispatched` record and no intent; its `access-decision` is recorded once,
   as today; and its reply is pending, naming it in `invocation` and its login's ask in `hook`, as
   the cases capture them. Every guarded call is checked, whoever sends it, an Agent's onward
   request among them; a call on a login whose Type states no `admission` is dispatched as today.
   A waiting call is no call in progress where PND-005 counts one (`hold_work`'s `running`). A
   dispatched call is sent exactly as with no `admission` stated.
2. **The ask** (PND-016, IFC-054, CTR-015, DEC-168, DEC-121). Where a call begins to wait and no ask
   is in progress on its login, the engine starts one: its own invocation of the login's
   `admission-hook.v1`, at most one in progress per login, each login of an account keeping its own,
   which every call waiting on that login shares. It is no request and no part: no
   `access-decision`, no key, no `conduit`, `consumer`, `serving` or `setup_parent`, `filters` `[]`,
   its `held_target`, `contract` and `implementation` taken as `dispatch_hook` takes a hook's; its
   caller is the caller of the call whose wait started it, and its cause, `causedBy` in each of its
   records, that call's ID, which stays when that call is cancelled. Its arguments, taken when it
   starts, are `levers`, the login's configuration at its levers place; `limits`, the login's limits
   as `inspect.v1` shows them (`limits::status`); and `previous`, the newest report kept, ended or
   spent included, or null. It is dispatched as a hook that is not plain, its discovery and then its
   call, whose `_meta` holds that caller and no Conduit; its `invocation-dispatched` and
   `invocation-ended` records name `caller` and `causedBy` and hold no `serving` (`record_content`
   is one place for `causedBy`). Order 317's `Report::take` and `record_end` keep its report, or
   none, unchanged. It starts no model work, and no inspection starts one (OBS-006). A new `State`
   member, `admission_asks`, keyed by the login's ID, holds the ask in progress, with its start
   time, and the period of decision 6, with
   `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]` as `admission_reports` has it,
   so that a store an earlier engine wrote reads as one with no ask and a state with none is
   written byte for byte as today, memory equals disk and the format version stays 1 (DL2). The
   member names an ask only while it is in progress: whatever ends it, a restart included,
   removes it there. Where the waiting calls are kept is yours. Only the ask `admission_asks`
   holds is an ask in this order: it alone decides waiting calls when it ends (decision 3) and
   starts decision 6's period. An `admission-hook.v1` entry that order 317's tests put in place
   with `GuardScenario::ask`, and so `keep`, which `admission_asks` does not hold, has its report
   kept or not as order 317 built, and nothing more, so that
   `admission_report_kept_with_its_validity` passes unchanged.
3. **When the ask ends** (PND-016, IFC-054, PND-005, PND-011). When the ask ends, however it ends
   but at a restart (decision 7), each call still waiting on it is decided in that one transition,
   earliest first by when it began to wait:
   - (a) a hold of PND-005 or PND-011 that began meanwhile holds it under that promise, as
     `hold_work` and the sign-in holds hold a call today, its `login_request_record` standing for
     its access record;
   - (b) where the ask's report was kept already ended (decision 5), it keeps waiting;
   - (c) otherwise its dispatch conditions are checked (decision 4), and where they hold it is
     dispatched as it would have been with no wait, the first so dispatched on a single-use report
     spending it, which ends `single-use`; a call whose check fails spends nothing;
   - (d) where PND-016 would hold it, on a report whose `admit` is false, on a single-use report
     another call spent, or for want of a report after an ask that gave none, this build
     dispatches it as in (c): the hold is the hold's order's. Decide what PND-016 would hold on in
     one place, so that the hold's order changes only what that place does.

   A call ended there without dispatch has its reply in `request_replies`; a dispatched one's comes
   when its call ends, as today. The transition makes no `access-decision` record (DEC-170).
4. **The check once at dispatch** (PND-016, DEC-170, IMP-001, ACC-019, ACC-039, IFC-028).
   Immediately before a waiting call would be dispatched, the engine checks once, from the Conduits
   as they then stand, that each Conduit of its chain is open and lists the operation, none has a
   retired consumer, and the invocation an onward request serves is still in progress. This is a
   condition of dispatch for the same permitted request: no access decision is made or recorded
   again. Where a condition fails, or its state cannot be established, the call is not dispatched
   and ends unavailable under PND-016 with one error diagnostic, `ruleId` PND-016, that says which
   condition failed or could not be checked; names a Conduit or invocation only where the caller
   may inspect it (`access::may_inspect`); says that the call was not dispatched, started no model
   work and is not queued, and that a call sent again is decided afresh; and carries no
   `properties.admission`. Its `request-ended` record names PND-016, reason unavailable, not
   dispatched. How the conditions are read is yours (one way is the availability pass of
   `access::evaluate` on the request as first received, its rule mapped to the condition): list
   it under `own_choices`.
5. **A report from an ask in progress when the levers change** (IFC-054, PND-016, DEC-169,
   REV-007). Where, while an ask on a login is in progress, a control on the login succeeds that
   order 317's `control_succeeded` reads as sent a value at or under the levers place, the report
   that ask then gives is kept already ended, `levers-changed`, with its end record's `time` as its
   keep time, and decides no call. Each call still waiting on that ask keeps its ID and its place
   and waits for the login's fresh ask, which the earliest still waiting starts as decision 2 says,
   caused by it and for its caller, with its own DL21 bound; none ends because the report ended,
   and no period of decision 6 follows. Where no call still waits, no fresh ask starts. As this
   build stores no control's values (REV-007; order 288's paragraph), the fresh ask's `levers` are
   the levers as registered: PND-016's "under the stored levers" holds once REV-007's storing is
   built (What follows). A control sent no value at the levers place, or one that fails or ends
   unknown, changes nothing of the ask.
6. **After an ask that gave no report** (PND-016, IFC-054, OPS-007). An ask gives no report where
   it fails, ends unknown (an answer off `admission-report.v1` among them, which OPS-007 already
   ends unknown), passes DL21 (decision 8) or cannot be dispatched. Then, until 60 seconds after
   its end on the engine's own clock, the time of the step that ended it, no ask starts on the
   login, and each guarded call on it is decided without waiting: this build dispatches it, as in
   decision 3 (d). At that instant the period is over, as a report's validity is at `validUntil`.
   The 60 seconds are PND-016's and fixed. Where the engine cannot dispatch an ask, it gave no
   report: the calls are decided as after an ask that gave no report, and the period's cause is
   `not-dispatched` and its `invocation` null, as `admission-status.v1` has it. One such path is
   likely in the build already: `complete_discovery` ends a discovery whose result lacks Conduit's
   extension unavailable under IFC-037 (`hook-without-extension`) and sends no call; an ask ended
   there was not dispatched, and its record there still names `causedBy`. No case reaches this:
   say under `own_choices` how your build treats that path and any other where an ask cannot be
   dispatched. An ask whose report is kept already ended gave a report, and no period follows it.
   Only the ask `admission_asks` holds starts a period (decision 2).
7. **A cancel, a restart and a repeat while waiting** (PND-016, OPS-027, OPS-009, OPS-032,
   REC-007, IMP-007).
   - `Input::Cancel` naming a waiting call, with or without a confirmation, ends it unavailable
     under PND-016, not dispatched: no intent at all, as it reached nothing; a reply that reports
     unavailable with one error diagnostic, `ruleId` PND-016, saying that the call was cancelled
     while it waited for its login's admission report, so it was not dispatched and started no
     model work, with no `properties.admission` and no `pendingAction`; and a `request-ended`
     record naming PND-016, reason unavailable, not dispatched, which also says that the request
     was cancelled while it waited, as PND-016 asks for where the protocol has the engine send no
     response (a closed response stream, which `server.rs` turns into a cancel). `record.v1` lets
     an engine add members: add one to that record, or otherwise make it say so, and list its form
     under `own_choices`; the transition also holds a decision naming the invocation, citing
     PND-016 and OPS-027. A `request-ended` record of decision 4's check is not so marked. The
     ask goes on for the others, its caller and `causedBy` unchanged; where none waits any more,
     it runs to its end and its report is kept.
   - At a restart (`Input::Recover`), a waiting call ends unavailable under REC-007, not
     dispatched, as recovery ends a call whose discovery is pending (`reject_unsent`), and an ask
     in progress ends unknown under IMP-007 and gives no report, so decision 6's period counts from
     the recovery's time. The waiting call's end is recorded before its ask's, as
     `requests-admission-wait-ends`'s step 25 asserts.
   - A request that matches a waiting call as OPS-009 describes a match (the same key, sender,
     caller, target and digest) is a repeat of an invocation not yet ended, which OPS-032 covers
     and PND-016 narrows for a waiting call: nothing is dispatched, it reports conflict, and its
     result's `_meta` holds OPS-032's `io.github.shrav-k.conduit/invocation`, the waiting call's
     ID, as `requests.rs` gives it for OPS-032 today; but its one error diagnostic has the
     `ruleId` PND-016, in place of OPS-032's, and says that the earlier call waits for its login's
     report, with no `properties.admission`. One with other arguments reports OPS-010's conflict,
     as today.
8. **DL21** (PND-016, DL21, DL10, OPS-013). The first build waits for an ask at most 30 seconds by
   default, counted from the time of the step that started it. `Config` gains
   `admission_ask_timeout`, 30 seconds, which `--admission-ask-timeout-seconds N` sets for `init`,
   `register` and `serve` as `--call-timeout-seconds` sets `call_timeout`, with this help line
   after that option's, indented as it is:
   `--admission-ask-timeout-seconds N: wait for a login's admission ask (default 30).`
   The value stands until the usage guard's probe has timed a hook's read. The core keeps no
   clock: a new input,
   `{"kind":"admission-ask-deadline","evidence":…,"invocation":ID,"attempt":N}`, ends the named
   ask where it is still in progress at that attempt: it ends unknown, decision 6's
   period starts with cause `deadline`, a `cancel` intent for its call is sent as `requests::cancel`
   sends one, a later answer to it is ignored (OPS-013) and keeps nothing, and the calls that
   waited on it are decided as decision 3 says. An ask whose discovery is still pending at the
   deadline ends the same way, with cause `deadline`: the `cancel` intent names it as
   `requests::cancel` names any invocation in progress, and a later discovery answer is ignored
   as a later answer is. Otherwise the input changes nothing and replies
   `{"askEnded":false}`. Either way the transition holds a decision `admission-ask-deadline`,
   citing PND-016, with DL21 among its implementation choices and facts `invocation`, `attempt` and
   `ended`, as `sign-in-wait-ended`'s has. List the input where `sign-in-wait-ended` is listed:
   `Input`, the evidence match and the dispatch in the core's `lib.rs`, `covers` and `from_trial`
   in `record_fault.rs`, and the shell's kinds that need evidence. While `serve` holds the store,
   a timer is started for each ask the core starts, whichever input starts it: an `Invoke`
   (decision 2), which the server's handler commits, or a `Complete`, which the runner commits
   through `Shell::complete` and not the handler, as for decision 5's fresh ask, started when the
   ask before it ends. Read the ask from the committed state or transition, never from an intent
   (the cases compare intents exactly): the shell's commit path (`commit_input`, `begin` and
   `drive_committed` in `lib.rs`) hands the executor each ask newly in `admission_asks`, with its
   ID, attempt and start, after each commit, the nested commits of `drive_committed` included,
   through a new `Executor` method whose default does nothing, as `tools_changed`'s does; and
   `ServingExecutor` overrides it to start the timer, which submits that input at the ask's start
   plus the limit, as `end_sign_in_wait_at` submits `sign-in-wait-ended`. How the override
   reaches the shell, the shutdown token and the runtime is yours (`ServingExecutor`'s `changes`
   channel, which its `tools_changed` feeds and `serve` reads, is one model); `runner.rs` needs no
   change, as `serve` gives the runner the serving executor for its completions. The timers end
   with the engine, and a restart ends the ask under decision 7. DL10's call deadline bounds the
   ask too: where it comes first, the runner reports the call missing, and the ask ends unknown
   with cause `unknown`. In `engine/README.md`, the sentence of runner settings becomes, wrapped
   at the file's width: "Runner settings are `initialize_timeout` (30 seconds), `call_timeout`
   (120 seconds), `admission_ask_timeout` (30 seconds) and `pass_environment` (empty); CLI
   equivalents are `--initialize-timeout-seconds`, `--call-timeout-seconds`,
   `--admission-ask-timeout-seconds` and repeated `--pass-environment NAME`."
9. **The status** (IFC-054, OBS-006; order 317's decision 7). `admission` gains what order 317
   left null: `asking`, the ask in progress on the login, or null; and `noReport`, while decision
   6's period runs at the inspection's time, `{invocation, cause, until}`: the ask that gave no
   report, null where none was dispatched; `failed`, `unknown`, `deadline` or `not-dispatched`; and
   the period's end, written as `validUntil` is; otherwise null. In an Agent's status, `asking` and
   `noReport.invocation` are null where the inspecting actor may not inspect the login, as
   `invocation` is. A report kept already ended shows `state` `ended` and `ended`
   `levers-changed`. The status gives the period though this build holds no call in it; the hold's
   order makes it a hold. An inspection starts no ask and changes nothing (OBS-006).
10. **The expected failures and the rows.** In `conformance/expected-failures.yaml`, remove
    `requests-admission-report-kept`'s entry, which this order makes pass, and
    `requests-admission-not-checked`'s, where the Start check's seventh line gave 1, as order
    317's Shared files paragraph directs; give `requests-admission-hold`,
    `requests-admission-no-report` and `requests-admission-wait-ends` the reason "PND-016, order
    316: the first build asks for a login's report and waits, but holds no call on a report or
    for want of one; the guard's engine order for the hold makes it pass", and
    `requests-admission-single-use` "PND-016, order 316: the first build holds no call on a
    login's report and stores no control's levers (REV-007); the guard's engine order for the
    hold and REV-007's storing at the levers place make it pass". Change no case. In
    `engine/CONFORMANCE.md`, wrapping at the file's width:
    - In the row `DL3: waiting, implemented portion`, on its one line, after "There is no
      autonomous clock or actor-reply timeout in the core." add: "From order 319, besides DL18's
      sign-in wait, the core bounds PND-016's ask for a login's admission report, which a call
      that starts model work on an Agent of a login whose Type states `admission` waits for, not
      dispatched: the core ends the ask unknown when the serving shell submits its DL21 end (row
      DL21), or when the runner reports its call missing at DL10's deadline, and keeps no clock of
      its own." In its
      evidence cell, after "`requests-cancellation`," add: "`requests-admission-report-kept`,
      `admission_ask_deadline_ends_it_unknown` in [admission_tests.rs] (link to
      crates/core/src/admission_tests.rs),".
    - In the row `DL4`, on its one line, after "no waiting queue." add: "From order 319, access is
      decided once, before a call that starts model work on an Agent of a login whose Type states
      `admission` waits for its login's ask (PND-016), and no access decision waits: when the wait
      ends, the call's
      dispatch conditions are checked once, from the Conduits as they then stand, and one that
      fails or cannot be established ends it unavailable under PND-016 at once, with no second
      access-decision record (DEC-170)." At the end of its evidence cell add: ",
      `admission_wait_checks_dispatch_once` in [admission_tests.rs] (link to
      crates/core/src/admission_tests.rs)".
    - After the row `DL15`, the table's last, a row: "| DL21 | 30 seconds by default, counted
      from the time of the step that starts the ask; `Config`'s `admission_ask_timeout` and
      `--admission-ask-timeout-seconds N`, for `init`, `register` and `serve` as
      `--call-timeout-seconds` is, set it, and it is revisited once the usage guard's probe has
      timed a hook's read. While `serve` holds the store, a timer started for each ask the core
      starts, by a call or by a completion, submits `admission-ask-deadline`, naming the ask and
      its attempt, at the limit; where
      the ask is still in progress at that attempt, the core ends it unknown, with `deadline` as
      the cause the login's status gives, sends a cancellation of its call and keeps nothing of a
      later answer (OPS-013). DL10's call deadline bounds the ask as well: where it comes first,
      the runner reports the call missing and the ask ends unknown. The shell's timers end with
      the engine, and a restart ends an ask in progress unknown (IMP-007). The 60 seconds after an
      ask that gave no report are PND-016's, fixed. | `admission_ask_deadline_ends_it_unknown` in
      [admission_tests.rs] (link to crates/core/src/admission_tests.rs),
      `admission_ask_ends_at_its_deadline` in [server_tests.rs] (link to
      crates/shell/src/server_tests.rs) |".
    - In order 317's section, "The engine starts no call of `admission-hook.v1` yet and holds no
      call on a report: the ask, the hold and the warnings are later orders' (DEC-123, DEC-095,
      DEC-099)." → "The engine holds no call on a report: the hold and the warnings are later
      orders' (DEC-095, DEC-099), and order 319 writes the ask (below)."
    - At the end of the file, a section `## A login's admission ask (order 319)` whose one
      paragraph reads: "From order 319, a call that starts model work on an Agent of a login whose
      Type states `admission`, once its access, validity, conflicts and the holds of PND-005 and
      PND-011 are decided, is dispatched where the login's newest report is current, and
      otherwise waits, not dispatched, its reply pending, for the login's ask: the engine's own
      call of `admission-hook.v1` on the login, at most one in progress per login, through no
      Conduit and with no filters, sent the levers as stored, the login's limits as `inspect.v1`
      shows them and the newest report, whose records name the caller of the call whose wait
      started it and that call in `causedBy`, and hold no `serving` (PND-016, DEC-168). When the
      ask ends, each call still waiting, earliest first, is held by a hold of PND-005 or PND-011
      that began meanwhile; keeps waiting for a fresh ask, which the earliest starts, where the
      report was kept already ended because a control sent a value at the levers place succeeded
      while the ask ran (IFC-054, DEC-169); and otherwise has its dispatch conditions checked once
      (row DL4) and is dispatched, the first so dispatched spending a single-use report. This
      build holds no call on a report or for want of one: a report whose `admit` is false, a
      single-use report another call spent, an ask that gave no report and the 60 seconds after
      one, in which no ask starts on the login and which the status gives in `noReport`, each
      leave the call dispatched as with no `admission` stated, until the hold's order (DEC-095,
      DEC-099). A call cancelled while it waits is not dispatched and reports unavailable under
      PND-016, a restart ends one unavailable under REC-007 and its ask unknown under IMP-007, and
      a request matching a waiting call reports conflict under PND-016. A waiting call is no call
      in progress where PND-005 counts one. As control dispatch stores no settings (above), a
      fresh ask after a lever change sends the levers as registered. DL21 bounds each ask (row
      DL21)."

    Write each "[text] (link to target)" as a real Markdown link: the text in square brackets,
    then at once the target, relative to `engine/`, in parentheses. Then run
    `python3 tools/promise_standing.py --check`, and `--write` where it fails.
11. **Tests.** Write them first, run them on the otherwise unchanged code and report each line. The
    two shell tests share the shell's lib test target (`server_tests.rs` is a module of `tests.rs`),
    and `admission_ask_ends_at_its_deadline` does not compile until `Config` has its member: in one
    turn, write and run `admission_asks_kept_across_a_reopen` on the unchanged code before you write
    the other. Core tests go in `engine/crates/core/src/admission_tests.rs`, beside order 317's,
    with its `GuardScenario` (`call`, `finish`, `keep`, `inspect`, `login_status`) and helpers of
    yours. *The report state* and *the control state* are order 317's:
    `requests-admission-report-kept`'s state through its step 15, on the guarded project or on
    `control_project()` with `tune-login` opened. *A* and *B* are root's `call-a.turn.v1` and
    `call-b.turn.v1`, each built as an `Input::Invoke` with an `Evidence` of its own at its time;
    *K* is the ask the first waiting call starts; complete an ask's discovery with
    `requests::advertised_extension`. R1 is `report_one()`, R3 the report of
    `requests-admission-single-use`'s step 19. Times are on 2026-11-21 in UTC. Build each input this
    order adds from JSON, so that each test compiles today and fails.
    - `admission_ask_started_once_for_waiting_calls` (PND-016, IFC-054, DEC-168; decisions 1, 2,
      3 and 9): in the report state, A at 13:01:00 is not dispatched: its reply is pending, naming
      A in `invocation` and K in `hook`; the transition's one intent is K's discovery to `login`;
      its records hold A's `access-decision` and no `invocation-dispatched` for A. B at 13:01:00
      is pending on K with no intent. At 13:01:00, root's inspections of `login` and of `a` give
      `asking` K and `state` `none`, and `observer`'s of `a` through `observer-scope`, opened as
      order 317's status test opens it, `asking` null; no inspection dispatches anything. Once K's
      discovery is completed, the one intent is K's call, its `arguments`
      `{levers: {billing: included, paidUsage: refuse, whenUnknown: refuse}, limits: {state:
      unknown, windows: []}, previous: null}` and its `_meta` naming root's ID as caller, `filters`
      `[]` and no Conduit; K's `invocation-dispatched` record names root as `caller` and A in
      `causedBy`, and holds no `serving`, `conduit` or `consumer`. K completed with R1 at 13:01:00:
      its end record holds R1 and `causedBy` A; the intents are A's dispatch, then B's, each as
      with no wait; the transition holds no `access-decision` record; `asking` is null and `state`
      `current`. A at 13:02:00 is dispatched at once, with no ask. Today A is dispatched at
      13:01:00 and nothing is asked.
    - `admission_ask_without_a_report_starts_none_for_60_seconds` (PND-016, IFC-054, OPS-007;
      decisions 3 (d), 6 and 9): in the report state, A waits on K at 13:01:00; K fails at
      13:01:00: its end record holds no `admission`, A is dispatched, and `login`'s status gives
      `state` `none`, `asking` null and `noReport`
      `{invocation: K, cause: failed, until: 2026-11-21T13:02:00Z}`. B at 13:01:30 is dispatched
      at once with no ask, and the status is as before. At 13:02:00 `noReport` is null, and A
      starts an ask K2, which, answered succeeded at 13:02:00 with R1 lacking `singleUse`, ends
      unknown: `noReport` gives K2, `unknown` and 13:03:00. Today nothing is asked and no status
      gives `noReport`.
    - `admission_single_use_report_spent_once` (PND-016, IFC-054; decisions 1, 3 (c) and 4): in
      the report state, A and B wait on K at 13:01:00; root's `access.close.v1` of `call-a`
      succeeds; K completed with R3: A is not dispatched and ends unavailable under PND-016
      (decision 4), and B is dispatched and spends R3, the status giving `ended` `single-use`. R3
      kept again by `keep` at 13:02:00, on which no call waits, is spent by B at 13:02:10,
      dispatched with no ask, and B at 13:02:20 starts an ask whose `previous` is R3. In a fresh
      report state, where A and B both pass, A is dispatched first and spends R3, and B is
      dispatched after it (decision 3 (d)). Today nothing is asked or spent.
    - `admission_wait_checks_dispatch_once` (PND-016, DEC-170, IMP-001, PND-005; decisions 3 (a)
      and 4): in the report state, A and B wait on K; root's `access.close.v1` of `call-b`
      succeeds at 13:01:05; K completed with R1 at 13:01:10: A is dispatched; B is not, and its
      reply, in the transition's `request_replies`, reports unavailable with one diagnostic,
      `ruleId` PND-016, naming `call-b` as closed and the words decision 4 gives, with no
      `properties.admission`; its `request-ended` record names PND-016, unavailable, not
      dispatched; the transition holds no `access-decision` record. In a fresh report state, A
      waits on K; a sign-in hold on `login` is put in place in `sign_in_holds`, as a raise leaves
      one; K completed with R1: A is held under PND-005, as `hold_work` holds a call, and not
      dispatched. Today nothing waits.
    - `admission_levers_changed_while_asking` (IFC-054, PND-016, DEC-169; decisions 5 and 9): in
      the control state, A waits on K at 13:01:00 and K's call is dispatched;
      `tune-login.set-admission.v1` sent valid levers succeeds at 13:01:02; K completed with R1 at
      13:01:04: the status gives R1 with `invocation` K, `keptAt` 13:01:04, `state` `ended`,
      `ended` `levers-changed`, `asking` a fresh ask K2 and `noReport` null; A has no
      `request-ended` record and waits under its ID; the one intent is K2's discovery, and K2's
      dispatch record names A in `causedBy` and root as caller; K2's call has `levers` as
      registered and `previous` R1; K2 completed with R1 at 13:01:06: A is dispatched and the
      status `current`. After A's call ends at 13:02:00 with `limits_member()`, ending that report,
      B at 13:02:10 starts an ask K3; a control sent `revision` alone that succeeds while K3 runs
      leaves K3's report, R1, current when K3 ends, and B is dispatched. In a fresh control state,
      where A is cancelled while K runs and the levers then change, K's report is kept ended and no
      fresh ask starts. Today nothing is asked.
    - `admission_wait_cancelled_restarted_or_repeated` (PND-016, OPS-027, OPS-009, REC-007,
      IMP-007; decision 7): in the report state, A and B wait on K at 13:01:00; `Input::Cancel`
      naming A, with no confirmation, gives no intent and the reply, record and decision that
      decision 7 gives, the record saying that A was cancelled while it waited; B still waits. K
      completed with R1: B is dispatched and K's end record names A in `causedBy`. After B's call
      ends at 13:02:00 with `limits_member()`, which is kept and ends R1, A sent with a key at
      13:03:00 waits on an ask K2, and the same request again reports conflict with nothing
      dispatched, A's ID in `_meta`'s `io.github.shrav-k.conduit/invocation`, and one diagnostic,
      `ruleId` PND-016, with the words decision 7 gives. `Input::Recover` at
      13:03:30 records A's `request-ended` (unavailable, REC-007, not dispatched) before K2's
      `invocation-ended` (unknown, `causedBy` A, no `admission`), sends nothing, and leaves the
      status's `noReport` `{invocation: K2, cause: unknown, until: 2026-11-21T13:04:30Z}`. Today
      nothing waits.
    - `admission_ask_deadline_ends_it_unknown` (PND-016, DL21, OPS-013; decision 8): in the report
      state, A waits on K at 13:01:00 and K's call is dispatched; `admission-ask-deadline` naming K
      at attempt 1 at 13:01:30 ends K unknown with no `admission` in its end record; the
      transition's intents are K's `cancel` and A's dispatch; the status gives `noReport`
      `{invocation: K, cause: deadline, until: 2026-11-21T13:02:30Z}`; K's answer with R1 after it
      is ignored (OPS-013) and keeps no report; the same input again replies `{"askEnded":false}`
      and changes nothing; the decision `admission-ask-deadline` gives `ended` true, then false.
      Today the input does not parse.
    - `admission_asks_kept_across_a_reopen` (DL2; decisions 2 and 7), in
      `engine/crates/shell/src/server_tests.rs`: in the store `established` leaves (s1's steps 1
      to 13), `state.json` holds no `admission_asks` member, and still none after a further commit.
      In a store `established_case` sets up as the report case's first 15 steps, root's
      `call-a.turn.v1` begun with `tests.rs`'s `Deferred`, which answers discovery and defers every
      call, so that its ask stays in progress (with `MissingExecutor` the ask's discovery would
      end at once), leaves `state.json` holding `admission_asks` with an entry for `login` that
      names an ask in progress, its format version 1, and memory equal to disk
      (`assert_memory_equals_disk`). Close the shell and open the store: as an open recovers
      (`Shell::open`), the ask is no longer in progress and no entry of the member names one, and
      memory equals disk; open it again: the member reads back as the first open left it, and
      memory equals disk. Write it against JSON. Today the member is never written.
    - `admission_ask_ends_at_its_deadline` (DL21; decision 8), in `server_tests.rs`: a store
      `fixed_case_shell` sets up as the report case's first 15 steps, opened with
      `admission_ask_timeout` one second through a variant of `fixed_endpoint_shell` that takes a
      `Config`, served with `ServingExecutor` over `tests.rs`'s `Deferred`, which answers discovery
      and defers every call; root's `call-a.turn.v1` through the client, in a task of its own, as
      its reply waits: within ten seconds the trace holds the decision `admission-ask-deadline`
      with `ended` true, as `relayed_sign_in_starts_the_expiry_timer` finds its decision, the ask's
      entry is `unknown`, and root's inspection of `login` gives `noReport` with cause `deadline`;
      `Config::default()` gives 30 seconds. Then the hand-off for an ask a completion starts: in
      a store `established_case` sets up as `requests-admission-single-use`'s first 32 steps (no
      reopen, so its ask `admission3` is still in progress and its control's success came while
      it ran), that case's step 33 completion, sent with `Shell::complete` for the ask in progress
      on `login`, through an executor of the test's that defers every call and records what the
      new `Executor` method is handed, hands it exactly one ask: decision 5's fresh ask, started
      inside that completion, the one then in progress on `login`. Today it does not compile, as
      `Config` lacks the member: report that error as its line.
    - In `tests/context.rs`, `init_writes_the_flow_skeleton_unless_a_manifest_is_there` gains
      `("--admission-ask-timeout-seconds", "2")` among the options `init` refuses without
      `--store`. Today it fails, as the option is unknown.
    - An existing test changes only where it makes a guarded call with no current report. One is
      expected: in `admission_report_ended_early`, part (v)'s call on `b`, today dispatched at
      13:07:00, after its report's validity ended at 13:06:00, is dispatched at 13:05:30, while the
      report is current, and completed at 13:07:00 with its kept member, so that the first end,
      `valid-for-ended`, is still what it tests. Report each test you change.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 094 095 120 123 124; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1
cat spec/promises/pending.yaml spec/implementation-defined.yaml | grep -cE '^- id: (PND-016|DL21)$'  # 2
grep -cE '^  name: admission-(report|hold|status)\.v1$|^        causedBy: \{\$ref: github\.com/shrav-k/conduit/core#id\.v1\}$' spec/core/results.yaml  # 4
k spec/promises/pending.yaml 'the earliest still waiting on its ask whose dispatch-time check passes' 'they keep waiting for a fresh ask under the stored levers' 'Ending its wait makes no second access decision.'  # 1 1 1
grep -cE '"id": "requests-admission-(hold|no-report|report-kept|single-use|wait-ends)"' conformance/expected-failures.yaml  # 5
grep -c '"id": "requests-admission-not-checked"' conformance/expected-failures.yaml  # 1
grep -rlE 'PND-016|DL21|caused_?[bB]y|admission[-_]ask' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '  # 0
grep -cE '^\| (PND-016 \| convention \| must \| 6 \| [56] \| outside \| none|IFC-054 \| interface \| must \| 6 \| [56] \| outside \| core) \| (expected failure|case passes) \|$' engine/STANDING.md  # 2
k engine/CONFORMANCE.md 'There is no autonomous clock or actor-reply timeout in the core.' 'yields unavailable immediately; no waiting queue. |' 'The engine starts no call of `admission-hook.v1` yet and holds no call on a report:' '| DL21 |' 'From order 319,'  # 1 1 1 0 0
k engine/crates/core/src/requests.rs 'crate::login_resource::hold_work(t, a, c, &target, &route, &access_record_id)?' 'invocation["login_request_record"] = json!(access_record_id);' 'if v["discovery"] == "pending" && v["hook"] != true {' '("serving", "serving"),' 'pub(crate) fn cancel(' 'fn dispatch_hook(' 'if rule == "OPS-010" { None } else { Some(id) },' '"hook-without-extension"'  # 1 1 1 1 1 1 1 1
k engine/crates/core/src/admission.rs 'pub(crate) struct Report {' 'pub(crate) fn control_succeeded(' '"ended":ended,"asking":null,"noReport":null}),'  # 1 1 1
k engine/crates/core/src/lib.rs 'pub enum Input {' '| Input::SignInWaitEnded { evidence, .. }' 'pub admission_reports: BTreeMap<String, admission::Kept>,' 'mod admission_tests;'  # 1 1 1 1
k engine/crates/core/src/record_fault.rs '| "sign-in-wait-ended"' '| Input::SignInWaitEnded {'  # 1 1
k engine/crates/core/src/admission_tests.rs 'fn ask(&mut self, time: &str) -> String {' 'let id = s.dispatch("2026-11-21T13:07:00Z", "call-b.turn.v1", json!({}));'  # 1 1
k engine/crates/shell/src/lib.rs 'call_timeout: Duration::from_secs(120),' '| "sign-in-wait-ended"' 'fn tools_changed(&self, _notifications: &[Value]) -> Result<()> {' 'fn drive_committed('  # 1 1 1 1
k engine/crates/shell/src/server.rs 'pub(crate) fn end_sign_in_wait_at(' 'impl Executor for ServingExecutor {' 'changes: broadcast::Sender<String>,'  # 1 1 1
k engine/crates/shell/src/main.rs '"--call-timeout-seconds" => {' '| "--call-timeout-seconds"'  # 1 1
echo "$(k engine/crates/shell/src/help.rs '--call-timeout-seconds N: provider call wait (default 120).') $(k engine/crates/shell/tests/context.rs '("--call-timeout-seconds", "2"),') $(k engine/README.md 'Runner settings are `initialize_timeout` (30 seconds), `call_timeout` (120 seconds) and `pass_environment` (empty); CLI equivalents are `--initialize-timeout-seconds`, `--call-timeout-seconds` and repeated `--pass-environment NAME`.')"  # 1 1 1
k engine/crates/shell/src/server_tests.rs 'fn established_case(path: &Path, case: &Path, through: usize) -> Shell {' 'fn fixed_case_shell(path: &Path, case: &Path, through: usize) -> Shell {'  # 1 1
k engine/crates/shell/src/tests.rs 'struct Deferred;'  # 1
grep -rlE 'admission-hook|loginTypes' packages | wc -l | tr -d ' '  # 0
test -e engine/crates/core/src/admission_ask.rs; echo $?  # 1
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it, but for the seventh, whose 0
means the stack has removed `requests-admission-not-checked`'s entry: then leave it removed and say
so. The ninth gives 2 on either path, as the rows read `6 | 6` with the entry and `6 | 5` without
it. A third to fifth printed line other than stated means order 316's spec, with the owner's three
strikes, is not in the tree; a sixth other than `5`, that the guarded cases' entries moved; an
eighth other than `0`, that an engine source already names the ask; an eleventh to twenty-first
other than stated, that the code moved since this order was written (the nineteenth names
`engine/README.md`'s sentence of runner settings); a twenty-second other than `0`, that a package
states `admission`, and the Preservation tests' "At risk" sentence no longer holds. Where it is
dispatched in two halves, 319b's worker runs this check on 319a's build as *In two halves* says.

## Scope

Change, for decisions 1 to 9 and their tests, in `engine/crates/core/src/`: `admission_ask.rs`
(new: the ask, the wait and their ends), `admission.rs` (a report kept already ended, the
single-use end, `asking` and `noReport`), `admission_tests.rs` (the seven tests and the one
changed), `lib.rs` (the `State` member, the input and the `mod` line), `requests.rs`,
`record_fault.rs` (the input's kind), `login_resource.rs` only so that a waiting call is not
counted as running, `access.rs` only for the dispatch conditions, `records.rs` only where the
status needs it, and `explain.rs` only where a PND-016 diagnostic needs its text; in
`engine/crates/shell/src/`: `lib.rs` (`Config`, the input's kind, and decision 8's hand-off: the
new defaulted `Executor` method and the commit path, `commit_input`, `begin` and
`drive_committed`, that calls it), `main.rs` and `help.rs` (the option; in `main.rs` also, and only,
what `serve` must give `ServingExecutor` for the timer), `server.rs` (`ServingExecutor`'s override
and the timer) and `server_tests.rs` (the two shell tests, their recording executor and
the `Config` variant of `fixed_endpoint_shell`); `engine/crates/shell/tests/context.rs` (the
option); `engine/README.md`'s one sentence of runner settings (decision 8); the six
`requests-admission-*` entries of `conformance/expected-failures.yaml`; `engine/CONFORMANCE.md`'s
rows DL3 and DL4, its new row DL21, the one sentence of order 317's section and the new section;
and `engine/STANDING.md` by the tool alone. A shape check of a document that the conformance
runner cannot assert goes to `spec/schemas/examples/` (`valid/`, `invalid/` or
`invalid-by-rule/`), with its line in `engine/crates/loader/tests/expected/invalid-verdicts.txt`,
both in Scope; none is expected. Such an example is a change under `spec/`: add its path to `n`,
so that `git add -N` adds it, and, as `check_decisions.py`'s coupling rule asks a new `Unreleased`
line for any change under `spec/`, one line under `Unreleased` in `spec/CHANGES.md` of the form
that file's head gives (`Added`, the schema it shows, DEC-123), the one line of that file in Scope.
Change an existing test only under decision 11. Change nothing else: not `spec/` beyond such an
example and its line, a case, `docs/` (DEC-094, DEC-095, DEC-120, DEC-123 and DEC-124 stay not
done), the loader, the conformance crate, the shell's other sources (`store.rs` and `runner.rs`
among them), `packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`. No promise changes, so
`spec/CHANGES.md` gains no other line. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `j` and `k` as in the Start check;
the second, guards, passes today and after. Run each existing test you changed by name, once,
too. Run the core's tests, the shell's unit tests and then the whole vectors test, once each,
last; never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in admission_ask_started_once_for_waiting_calls admission_ask_without_a_report_starts_none_for_60_seconds admission_single_use_report_spent_once admission_wait_checks_dispatch_once admission_levers_changed_while_asking admission_wait_cancelled_restarted_or_repeated admission_ask_deadline_ends_it_unknown; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 seven times; 0 today
for t in admission_asks_kept_across_a_reopen admission_ask_ends_at_its_deadline; do (cd engine && cargo test --offline --release -p conduit-shell --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; 0 today
(cd engine && cargo test --offline --release -p conduit-shell --test context init_writes_the_flow_skeleton_unless_a_manifest_is_there 2>&1 | grep -c "$ok")  # 1; 0 today
k engine/README.md '`admission_ask_timeout` (30 seconds)' '`--admission-ask-timeout-seconds`'  # 1 1; 0 0 today
for c in report-kept not-checked; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -c "^PASS requests-admission-$c "); done  # 1 twice; 0 twice today, or 0 then 1 where the stack removed not-checked's entry
grep -cE '"id": "requests-admission-(report-kept|not-checked)"' conformance/expected-failures.yaml  # 0; 2 today, or 1 where the stack removed not-checked's entry
for c in hold no-report single-use wait-ends; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -A1 "^EXPECTED FAILURE requests-admission-$c " | sed -n 2p | cut -c1-10); done  # "  step 18:", "  step 18:", "  step 19:", "  step 26:", one per line; "  step 16:" four times today
k engine/CONFORMANCE.md 'From order 319,' '| DL21 |' 'sign-in wait, the core bounds' 'and no access decision waits' '(crates/shell/src/server_tests.rs)' 'The engine starts no call of'  # 3 1 1 1 1 0; 0 0 0 0 0 1 today
grep -cE '^\| (PND-016 \| convention|IFC-054 \| interface) \| must \| 6 \| 4 \| outside \| core(, shell)? \| case passes \|$' engine/STANDING.md  # 2; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-core login_resource_ 10  # 1
s conduit-core limits_ 8  # 1
for t in admission_types_checked_at_install_and_submission admission_checked_at_registration_and_replacement admission_control_pair_refused_before_dispatch admission_report_kept_with_its_validity admission_report_ended_early admission_status_shows_the_login_or_not_checked; do s conduit-core "$t" 1; done  # 1 six times; admission_report_ended_early as decision 11 changes it
for t in admission_reports_read_from_an_earlier_engines_store sign_in_wait_end_is_submitted_at_expiry relayed_sign_in_starts_the_expiry_timer; do s conduit-shell "$t" 1; done  # 1 three times
s conduit-shell memory_equals_disk_after 5  # 1
(cd engine && cargo test --offline --release -p conduit-loader --test examples 2>&1 | grep -c '^test result: ok\.')  # 1
for p in contracts-admission-shape registration-admission-checks contracts-login-resource- contracts-sign-in- registration-login-place requests-login-resource- requests-limits- requests-usage-kept requests-reconcile- requests-sign-in- requests-cancellation requests-restart-interrupted requests-key-scope change-unknown-setting records-purge- secret-reads-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", N in order: 1 1 1 2 1 2 4 1 8 22 1 1 1 1 2 11
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
n="engine/crates/core/src/admission_ask.rs"; git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(admission_ask|admission|admission_tests|lib|requests|record_fault|login_resource|access|records|explain)\.rs|engine/crates/shell/src/(lib|main|help|server|server_tests)\.rs|engine/crates/shell/tests/context\.rs|engine/(CONFORMANCE|STANDING|README)\.md|conformance/expected-failures\.yaml|spec/schemas/examples/(valid|invalid|invalid-by-rule)/[^/]+|spec/CHANGES\.md|engine/crates/loader/tests/expected/invalid-verdicts\.txt)$' | wc -l | tr -d ' '  # 0
git reset -q -- $n  # stage nothing
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-shell --lib)  # the shell's unit tests, once
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Of order 316's expected failures, `requests-admission-report-kept` flips to pass and its entry
goes, and `requests-admission-not-checked`, which order 317's status passes, loses its entry too.
`requests-admission-hold` and `requests-admission-no-report` then first fail at step 18 and
`requests-admission-single-use` at step 19, each where the hold would hold `a`'s or `b`'s call and
this build dispatches it, and `requests-admission-wait-ends` at step 26, its hold in the 60
seconds after the ask a restart ended. The hold's order makes the first two and the last pass;
the single-use case also needs REV-007's storing (What follows).

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code, then its passing line; each case's line before and after; each existing test you
changed, and why; your own choices; and every check's last line. Stage nothing.

**In two halves.** Where the order is dispatched as 319a and 319b, each half changes only the
files of its own decisions within Scope and runs this Start check and Done when with these
differences.

- 319a: decisions 1 to 4, but decision 3 (b); of decision 9, `asking` alone (`noReport` stays
  null); of decision 10, the six entries of `conformance/expected-failures.yaml` and the row DL4;
  of decision 11, `admission_ask_started_once_for_waiting_calls`,
  `admission_single_use_report_spent_once`, `admission_wait_checks_dispatch_once`,
  `admission_asks_kept_across_a_reopen` and the change to `admission_report_ended_early`. No
  input, `Config` member, option, timer, hand-off or README sentence, and no other row or section
  of `engine/CONFORMANCE.md`. Until 319b, an ask is bounded by DL10's call deadline alone; a
  report from an ask during which the levers changed is kept as order 317 keeps any; no period
  follows an ask that gave no report, so the next guarded call starts a new ask; and a cancel, a
  restart or a repeat meets a waiting call as today's code meets an invocation in progress,
  except that nothing is dispatched at a restart (decision 3) and an ended call no longer waits.
  No test or case step 319a runs reaches these. In its Done when, these lines replace the first
  two loops, the lines for `tests/context.rs` and `engine/README.md`, the loop for the four
  expected failures and the CONFORMANCE line:

  ```sh
  for t in admission_ask_started_once_for_waiting_calls admission_single_use_report_spent_once admission_wait_checks_dispatch_once; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; 0 today
  (cd engine && cargo test --offline --release -p conduit-shell --lib admission_asks_kept_across_a_reopen 2>&1 | grep -c "$ok")  # 1; 0 today
  for c in hold no-report single-use wait-ends; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -A1 "^EXPECTED FAILURE requests-admission-$c " | sed -n 2p | cut -c1-10); done  # "  step 18:", "  step 18:", "  step 19:", "  step 18:", one per line; "  step 16:" four times today
  k engine/CONFORMANCE.md 'From order 319,' '| DL21 |' 'sign-in wait, the core bounds' 'and no access decision waits' '(crates/shell/src/server_tests.rs)' 'The engine starts no call of'  # 1 0 0 1 0 1; 0 0 0 0 0 1 today
  ```

  `requests-admission-wait-ends` first fails at step 18 here, its cancel of a waiting call.
- 319b, on 319a's build: decisions 5 to 8 and decision 3 (b); of decision 9, `noReport` and a
  report kept already ended; of decision 10, the row DL3, the row DL21, the sentence of order
  317's section and the new section; the README sentence; of decision 11,
  `admission_ask_without_a_report_starts_none_for_60_seconds`,
  `admission_levers_changed_while_asking`, `admission_wait_cancelled_restarted_or_repeated`,
  `admission_ask_deadline_ends_it_unknown`, `admission_ask_ends_at_its_deadline` and the change
  to `tests/context.rs`. Its Start check's sixth to tenth lines and its twenty-third, and its
  eleventh to twenty-first where 319a changed those files, print what 319a left (about: `4`; `0`;
  more than `0`; `0`, as both rows read `6 | 4 | outside | core | case passes`; `1 0 1 0 1`; and
  `0` for the twenty-third, as 319a made `admission_ask.rs`): report them as they print and go
  on; every other line holds as stated. Its Done when's first loop runs those four core tests
  (1 four times; 0 before), its shell loop `admission_ask_ends_at_its_deadline` alone (1; a
  compile error before), its line for `tests/context.rs` gives 1 (0 before) and its line for
  `engine/README.md` `1 1` (`0 0` before), its loop for the four expected failures prints step
  18, 18, 19 and 26 (18, 18, 19 and 18 before), its CONFORMANCE line `3 1 1 1 1 0` (`1 0 0 1 0 1`
  before), and its lines for `requests-admission-report-kept` and `requests-admission-not-checked`,
  the entries and STANDING are guards that print the same before and after.

## What follows

- **The hold** (DEC-095, DEC-099), the next engine order: at decision 3 (d)'s place and in
  decision 6's period it holds the call unavailable under PND-016, after `hold_work`, with
  `admission-hold.v1` in `properties.admission`, the text by reason and `lifts`, the names by
  inspection and the record's decision; `requests-admission-hold`, `requests-admission-no-report`
  and `requests-admission-wait-ends` then pass and their entries go.
- **Found: the single-use case needs the levers stored.** Its step 35 expects the fresh ask after
  a lever change to send the levers the control was sent. This build stores no control's values
  (REV-007; order 288's paragraph), so `requests-admission-single-use` stays an expected failure
  after the hold's order until REV-007's storing at the levers place is built, which order 317's
  What follows already leaves to the coordinator to order or to ask the owner about.
- **DL21's value**: 30 seconds until the usage guard's probe has timed a hook's read; then the row
  and the default change by a dated amendment.
- **`not-dispatched`**: no case reaches an ask that cannot be dispatched, the IFC-037 path of
  decision 6 among them; the hold's order or a later one adds a case where the engine has such a
  path.
- **Found: OPS-032 names no exception.** OPS-032 gives its own `ruleId` to every repeat of an
  invocation not yet ended, and PND-016 its own to a repeat of a waiting request; this order
  applies PND-016's as the narrower promise (decision 7) and keeps OPS-032's invocation ID. A
  dated amendment of OPS-032 naming the exception is the coordinator's to order or to ask about.
- **The warnings** (DEC-099, DEC-119): order 318's spec and its engine order, the result key on a
  call admitted on `warning` or on a `cannot-tell` yes, and the refusal of an Agent's Conduit
  listing a login Type's control.
- DEC-094, DEC-095, DEC-120, DEC-123 and DEC-124 stay not done until the guard's orders merge,
  and no team-seat Agent takes a turn before then (DEC-102).

## Record

Run on 2026-11-23 and 2026-11-24 through Conduit itself, in Pacific time, stacked on orders 307,
314, 315, 316 and 317 and the decision log's DEC-158 to DEC-171. The usage guard's engine order E1b
(DEC-166), dispatched in two halves as *In two halves* says.

**Worker-2's turns.** 319a (23:16 to 23:41) stopped rightly: a record fault (REC-007) could end an
ask while leaving it active. By the coordinator's ruling, open to the owner's strike, 319a's scope
widened to `RecordFault::step`, and such an ask settles as an ask that gave no report (PND-016);
319a then finished (23:41 to 00:03). 319b (00:30 to 01:01) built what changes, ends or bounds a
wait.

**Two blind reads**, of both halves together, after `serve` restarted. The Codex verifier,
`root-verifier-2.turn.v1`, had two musts (an undispatched waiting invocation could authorize an
onward request; a Done-when loop that did not match the runner's output); the Claude reader
(`reader-1`) had four shoulds and four nits. One fix turn by worker-2 (08:49 to 09:12) applied them,
each must with a test first. The probe's results (2026-11-24) keep DL21's default of 30 s. The
gate's run on the squash is the coordinator's, at the merge.

**At the merge.** On its rebase onto a `main` holding orders 278, 311 and 317, two things needed the
coordinator (ruling 87). In `requests.rs` the restart's recovery now recovers interrupted asks
first and then compacts the invocations that have ended (order 311's loop). And the core did not
compile: 278 writes the state through a mirror whose exhaustive pattern names every field, and
this order's `admission_asks` and `admission_no_reports` were not in it; the coordinator added
both, in the state's own order. Clippy and the core's admission tests passed before the gate.
