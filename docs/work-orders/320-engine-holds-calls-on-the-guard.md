# Work order 320: the engine holds a call that starts model work where its login's report does not admit it (DEC-095, DEC-099, DEC-120, DEC-169, DEC-170, the usage guard's fourth engine order)

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
[DEC-095](../decisions/log.md#dec-095-2026-11-23-a-guarded-hold-reports-unavailable-at-the-login-hold),
"Accept (Recommended)": the engine holds each call that starts model work on an Agent of a
guarded login before dispatch, at PND-005's login hold, reporting unavailable and never queued.
[DEC-099](../decisions/log.md#dec-099-2026-11-23-a-guarded-hold-tells-the-caller-everything-but-amounts),
"Accept (Recommended)": the hold names the login, the lever and its value, the window, its used
percentage, the threshold, the reset, how it lifts and which report, in text and a structured
field, amending DEC-077's "nothing relayed to the model" for this diagnostic only.
[DEC-120](../decisions/log.md#dec-120-2026-11-23-the-guard-checks-before-each-turn-and-a-started-turn-runs-to-its-end),
"I don't want to modify how the models operate just police their usage", then "Yes, check before
turns only (Recommended)": the engine holds the call where the levers refuse it, and never stops
a dispatched turn. Three of the owner's strikes of order 316 name this order for their end:
[DEC-167](../decisions/log.md#dec-167-2026-11-23-an-agents-status-shows-its-logins-guard-report),
"Show it on the Agent (Recommended)": an Agent's status shows why its next call that starts model
work would run or be held, to whoever may inspect the Agent;
[DEC-169](../decisions/log.md#dec-169-2026-11-23-a-lever-change-makes-waiting-calls-wait-for-a-fresh-ask),
"Change 'levers-changed' hold": the report a lever change ended holds no call, and the calls
waiting on it wait for a fresh ask, within DL21;
[DEC-170](../decisions/log.md#dec-170-2026-11-23-a-waiting-calls-access-is-checked-once-at-dispatch),
"Dispatch-time check instead": a waiting call's access is checked once at dispatch, with no
second access-decision record. DEC-094 (the engine trusts the package's report), DEC-096 (a
report may carry paid-usage state) and DEC-102 (no team-seat Agent takes a turn until the guard
is built) stand behind it. [Order 316](316-admission-hold-before-a-turn.md) wrote PND-016,
`admission-hold.v1` and `admission-status.v1`, with the three strikes applied;
[order 317](317-engine-checks-admission-and-keeps-reports.md) built IFC-054's keeping and the
status; [order 319](319-engine-asks-and-waits-for-a-report.md) built the ask, the wait, the 60
seconds after an ask that gave no report, the check once at dispatch and DL21, and left the hold
to this order at one place. This order builds PND-016's hold: a call held on a report whose
`admit` is false, on a single-use report another call spent, or for want of a report, never
queued, with `admission-hold.v1` in its diagnostic and words that tell the caller everything but
amounts; and it makes `requests-admission-hold`, `requests-admission-no-report` and
`requests-admission-wait-ends` pass. It writes no warning: a call admitted on `warning` or
`cannot-tell` is dispatched as today, and its key is the warnings' order's (E2b).

**Today** (`main` with the decision log through DEC-157, the builds of orders 307 and 314 to 319
on top, and the decision log's DEC-158 to DEC-171), no engine source names `admission-hold.v1`
or a decision `hold-login-admission`. In `admission_ask.rs`, `wait` returns false for a call that
is not guarded; where order 319's period after an ask that gave no report runs (`no_report`) or a
report is current (`admission::current`), it asks `decide_report`, and dispatches where that
answers true; otherwise it starts or joins the login's ask. `decide_report`, marked "PND-016's
report/no-report hold is the next engine order", spends a current single-use report that admits
and always answers true. `settle_ended` decides each call still waiting when an ask ends, in
order: `requests::hold_before_dispatch` (PND-005 and PND-011); the fresh ask after a lever change
(`levers_changed`, DEC-169); `dispatch_condition`, whose failure ends the call through
`requests::reject_waiting` (DEC-170); then `decide_report`, and `requests::dispatch_waiting`, or
`requests::record_fault_unavailable` where the ask ended under a record fault. `reject_waiting`
ends a waiting call unavailable under PND-016 with one diagnostic and its reply in
`request_replies`. PND-005's and PND-011's hold, `login_resource::hold_work`, writes its text as
"`{tool} reports unavailable under {rule}: …`", names an Agent's call only where the caller may
inspect it (`access::may_inspect`), and holds the decision `hold-login-resource-work` citing
PND-005, PND-011, OPS-023 and IFC-025. `admission::status` gives `noReport` from
`admission_no_reports`. So `requests-admission-hold` and `requests-admission-no-report` first
fail at step 18, where the ask's refusal or failure leaves `a`'s call dispatched;
`requests-admission-wait-ends` at step 26, its call in the 60 seconds after the ask a restart
ended; `requests-admission-single-use` at step 19, where `b`, which a single-use report admitted
another call before, is dispatched; and `requests-admission-warning` at step 18, where `a`'s
dispatch record holds no `admissionWarning`. `engine/STANDING.md` gives PND-016 and IFC-054 each
`7 | 5`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6, a
held call is never queued, resumed or dispatched later, the hold starts no model work and no ask
of its own, and the engine never stops or cancels a dispatched call for the guard (DEC-120); 1,
the hold decides only on what the login's package reported, `admit` alone, and nothing of it
reaches an Agent's implementation: a dispatched call is sent exactly as with no `admission`
stated; 2, the hold only refuses, grants nothing, and a waiting call's access is checked once at
dispatch, not decided again (DEC-170); 7, each hold names the login, its report and the ask that
gave it, or why it gave none, with a record and a decision, and names an invocation only to a
caller who may inspect it. At risk, none in use: no package states `admission` (the Start check's
eighteenth printed line), so no store but a test's holds a guarded login, and DEC-102 keeps the
team seats from turns until the guard's orders merge.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
PND-016's hold is a must since order 316, and without it a refusing report or an ask that gave
none still lets a turn run, which is what DEC-102 waits on. The usage guard's probe of
2026-11-24, which order 319's Record names, found a team seat's login able to go on in paid
usage, so `paidUsage: refuse` keeps its turns off paid usage only if the engine holds every call
on a report whose `admit` is false, whatever its reason (DEC-096). By existing practice, a Kubernetes
validating admission webhook's denial rejects the request with the webhook's message and nothing
is retried for it, and under `failurePolicy: Fail` an error or a timeout rejects it too; a
server that cannot serve now says so with `503` and `Retry-After` (RFC 9110, sections 15.6.4 and
10.2.3), as `retryAt` does; a circuit breaker, once open, fails calls at once for a cool-down
without calling the dependency, as the 60 seconds after an ask that gave no report do; and the
engine's own PND-005 hold reports unavailable before dispatch with its cause and its lift
(`hold_work`). Not taken: queueing or resuming a held call (DEC-095); stopping or cancelling a
dispatched call (DEC-120); an amount, a balance or a currency in a hold (DEC-099); falling back
on an older report; a hold on a report a lever change ended (DEC-169); a second access decision
when a wait ends (DEC-170); a configurable 60 seconds; a retry by the engine; the warnings and
the refused open (the warnings' order).

**When it runs.** It stacks on orders 307 and 314 to 319 and runs on 319's build (`main` with the
decision log through DEC-157, the builds of 307 and 314 to 319 on top, and the decision log's
DEC-158 to DEC-171) until they merge, then on `main`. Through Conduit: one Codex worker turn, the
blind Codex verifier and Claude reader at once, one fix turn, the gate (DEC-046, DEC-071,
DEC-033). By ruling 89, each decision says what must hold; how is yours. It fits one turn: one
decision point, one builder of the hold, its words, six core tests and four changed. Where it
is too large, it splits in two halves from the start, each with its own turn, reads, fix turn and
gate: 320a, the hold on a report (decisions 1, 2 and 4 to 7, four core tests, one changed test,
the `requests-admission-hold` entry, the single-use reason and the sentence of order 317's
section), and 320b, on 320a's build, the hold for want of a report and the status (decisions 3
and 8, the record-fault path of decision 1, two core tests, three changed tests, the other entries
and reason, and the rest of decision 9). *In two halves*, after the Done when, gives each half's
Start check and Done when. The coordinator's, open to the owner's strike (DEC-029): the hold
decided where `decide_report` stands, and nowhere else; in the 60 seconds after an ask that gave
no report, the period deciding before any current report; a waiting call's dispatch conditions
checked before the hold, so that a call whose Conduit closed while it waited ends with decision 4
of order 319's diagnostic, with no `properties.admission`, whatever its report says; a held
call's reply and record naming the engine's ID for it, as `reject_waiting`'s do, whether it was
held at once or when its wait ended; the text beginning as `hold_work`'s does; the decision
`hold-login-admission` citing PND-016, IFC-054, OPS-023 and IFC-025, with facts `instance` and
`invocation`; under a record fault, a waiting call ending as order 319 built it, held or not; no
change to the status's form; the DL21 row saying that the probe kept 30 seconds; the
expected-failure reasons; and the test names.

**Shared files.** The warnings' engine order (E2b) builds `admission-hold.v1` with `dispatched`
true from the same report and changes `admission_ask.rs` and `requests.rs`: build the hold's
form in one function that takes the report, the ask and the cause, so that E2b adds its case
there. Order 305 changes REC-002 and `record.v1`; DEC-149's orders 311 to 313 change core
`requests.rs` and `records.rs`, the shell's store and `engine/CONFORMANCE.md`, and 311 drops
`held_target` from an ended entry. Whichever merges later rebases and reruns
`promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first PND-016, IFC-054, CTR-015, PND-005, PND-011, OPS-005, OPS-009, OPS-023, OPS-027,
IFC-025, REC-001, REC-002, REC-007, IMP-001, IMP-007, ACC-019, ACC-039, OBS-002 and OBS-006, and
DL21; the schemas `admission-hold.v1`, `admission-report.v1` and `admission-status.v1`
(`spec/core/results.yaml`) and `diagnostic.v1`; orders 316, 317 and 319 whole, with their
Records, and order 318's decision 9; the four cases `requests-admission-hold`,
`requests-admission-no-report`, `requests-admission-single-use` and
`requests-admission-wait-ends` (`conformance/conventions/requests/admission-*/`); and in
`engine/CONFORMANCE.md` the rows `DL4` and `DL21` and the sections of orders 317 and 319. In the
core, `admission_ask.rs` whole; `admission.rs` (`Kept`, `valid_until`, `current`, `end`,
`status`); `hold_before_dispatch`, `invoke_inner` from its call of `admission_ask::wait`,
`reject_diagnostics`, `reject_waiting`, `end_waiting`, `dispatch_waiting`, `fault_reply` and
`record_fault_unavailable` in `requests.rs`; `hold_work` and `label` in `login_resource.rs`;
`access::may_inspect`; `settle_record_fault`'s callers in `record_fault.rs`; and order 317's and
319's helpers in `admission_tests.rs` (`GuardScenario` with `call`, `finish`, `keep`, `inspect`,
`login_status`, `admission_call`, `discover_ask`, `admission_input`; `waiting`,
`dispatched_ids`, `report_one`, `limits_member`, `control_project`, `result`). *A guarded call*
and *its login* are order 319's. *A hold* is a guarded call ended under PND-016 on a report or
for want of one; its *form* is the `admission-hold.v1` it carries. A case's steps are named as
the runner prints them, counted from 1.

1. **Where the hold decides** (PND-016, DEC-095, PND-005, PND-011, OPS-023). Decide the hold where
   `decide_report` stands, the one place order 319 left for it, and remove its marking comment.
   It decides a guarded call once OPS-023's reasons and the holds of PND-005 and PND-011 have
   been decided and none applies, both where a call arrives (`wait`) and where its wait ends
   (`settle_ended`), in this order:
   - (a) while the login's period after an ask that gave no report runs at the step's time
     (`admission_ask::no_report`), the call is held for want of a report (decision 3), without
     waiting and whatever report is current;
   - (b) otherwise, where a report decides (the login's current report for an arriving call, the
     ask's kept report for a waiting one), a report whose `admit` is false holds it (decision 2);
     one whose `admit` is true dispatches it as today, a single-use one spending itself on the
     first so dispatched;
   - (c) for a call still waiting on an ask whose single-use report another call waiting on it
     spent in the same transition, the call is held, `after-call` (decision 2);
   - (d) for a call waiting on an ask that gave no report, the call is held for want of a report
     (decision 3).

   `settle_ended`'s order before the hold stays: PND-005's and PND-011's holds, then the fresh ask
   after a lever change (DEC-169: a report a lever change ended decides no call and holds none,
   and no hold names `levers-changed`), then the dispatch conditions (DEC-170), then the hold.
   An arriving call with no current report and no period still waits for the login's ask, as
   order 319 built it. Under a record fault (`settle_record_fault`), a call still waiting ends
   as `record_fault_unavailable` ends it today, held or not: the hold's record cannot be written
   there, and REC-007 governs. A call the hold does not hold is dispatched exactly as today.
2. **A hold on a report** (PND-016, IFC-054, DEC-095). A call held under (b) carries the form
   with `report` the report that decided, `reportInvocation` the ask that gave it, `noReport` and
   `singleUse` null, `lifts` the report's `lifts`, and `retryAt` the report's `validUntil`, its
   keep time plus `validForSeconds`, written as `admission-status.v1`'s `validUntil` is, null where
   `lifts` is `lever-change`. A call held under (c) carries `report` and `noReport` null,
   `reportInvocation` the ask, `singleUse` the ID of the call the report admitted, `lifts`
   `after-call`, and `retryAt` the ask's end, the time of the step that ended it, plus 60
   seconds. Of the calls waiting on one ask, the first by when it began to wait whose dispatch
   conditions pass takes a single-use report; a call whose conditions fail, or that PND-005 or
   PND-011 holds, spends nothing. A single-use report on whose ask no call waits still admits the
   next arriving call while it is current, as order 319 built it, and a call arriving after it is
   spent finds no current report and waits for a fresh ask, which gets it as `previous`.
3. **A hold for want of a report** (PND-016, IFC-054, OPS-007, DL21, REC-007, IMP-007). A call held
   under (a) or (d) carries `report` and `singleUse` null, `reportInvocation` the ask that gave no
   report, null where it was not dispatched, `noReport` its cause as order 319's period keeps it
   (`failed`, `unknown`, `deadline` or `not-dispatched`; an ask a restart ended is `unknown`),
   `lifts` `at-time`, and `retryAt` the period's end, the ask's end plus 60 seconds, the instant
   the login's status gives in `noReport.until`. The calls that waited on the ask are held in the
   transition that ends it, a deadline's (`admission-ask-deadline`) among them, whose one intent
   is then the ask's `cancel`; at a restart, the waiting calls end under REC-007 as order 319
   built it, and calls arriving in the period that follows are held. At the period's end a call
   starts a new ask, as order 319 built it.
4. **What a held call reports** (PND-016, OPS-005, OPS-023, IFC-025, REC-001). A held call is not
   dispatched: no intent, no `invocation-dispatched` record, no ask of its own. It reports
   unavailable: `isError` true, `_meta` with `io.github.shrav-k.conduit/reason` `unavailable` and
   the engine's ID for it, and no `io.github.shrav-k.conduit/pendingAction`; one error diagnostic,
   `ruleId` PND-016, whose `properties.admission` is its form, with `login`
   `{kind: Resource, name, id}` of its login, `dispatched` false and `levers` `{type, pointer,
   control}`: the login's Type and its `admission.control`, as qualified references, and its
   levers place. Each form validates against `admission-hold.v1`. The reply of a call held at once
   is the transition's reply; that of a call whose wait ended is in `request_replies`, as
   `reject_waiting` gives one. Its `request-ended` record names PND-016, reason unavailable, not
   dispatched, and the transition holds one decision `hold-login-admission` citing PND-016,
   IFC-054, OPS-023 and IFC-025, with facts `instance`, the login's ID, and `invocation`, the
   held call's; no `access-decision` record is made for a call whose wait ended (DEC-170). The
   call ends finally: no entry of it stays in progress, and nothing of it is queued, resumed or
   dispatched later, whatever report is kept after; a request sent again is a new request,
   decided afresh, as a call PND-005 held is.
5. **The words** (PND-016, DEC-099). The diagnostic's text begins
   "`{tool} reports unavailable under PND-016: `", as `hold_work`'s does, and then, in this
   order:
   - names the login by kind, name and ID (`login_resource::label`);
   - says that its admission report says model work may not start on it now, or that it has no
     report that admits this call;
   - says that the call was not dispatched and started no model work, that nothing of it is
     queued, and that a call sent again, at `retryAt` or after the lift, is decided afresh;
   - for a report, says why in its reason's words as `admission-report.v1` defines them, naming
     the lever's pointer and value (for `threshold`, the threshold), the window with its used
     percentage, `observedAt` and its reset, and the paid usage read (`possible`, `inUse`,
     `resetsAt`), each "not reported" where null; for no report, the ask and why it gave none
     (it failed, ended unknown, passed DL21, could not be dispatched, or ended at a restart); for
     a single-use report, the call it admitted, and that the caller need not wait for that call
     to end before sending again;
   - says how the hold lifts, by `lifts`, with `retryAt`: for `lever-change`, the sentence
     "Waiting alone does not lift this; a change of `<pointer>` by the control `<control>` on
     `<login>`, sent by an Interface, does.", with the levers place, the control's qualified
     reference and the login's label; otherwise also that a change of the levers by that
     control, sent by an Interface, ends the report.

   The text and the form name no amount, balance, credit or currency, whatever the report holds.
   Where `explain.rs` holds the words of a rule's diagnostics, put these there.
6. **Who is named** (PND-016, DEC-099, OBS-002). The login is named always, as PND-005 names it.
   The ask, in `reportInvocation` and the text, only where the held call's caller may inspect the
   login (`access::may_inspect`), else null and "the login's ask"; the call a single-use report
   admitted, in `singleUse` and the text, only where the caller may inspect that call's Agent,
   else null and "a call of an Agent of the login". The report itself is given whole, its
   `lever` included, as the status gives it (DEC-167).
7. **A dispatched call runs to its end** (PND-016, DEC-120). No transition sends a `cancel` or any
   other intent for a dispatched call because of a report, a hold or a period: the hold acts
   only before dispatch. A dispatched call is sent exactly as with no `admission` stated, and its
   result carries nothing of the guard (the warning is the warnings' order's). The calls one
   report admits run in parallel, as PND-005 lets calls on one login.
8. **The status** (IFC-054, DEC-167, OBS-006). `admission-status.v1`'s form, as order 317 built
   it and order 319 filled it, stays: it now shows what holds. While a call on the login would be
   held on a report, `state` is `current` and `report` is that report, its `admit` false; after a
   single-use report is spent, `state` is `ended` and `ended` `single-use`; while the period
   runs, `noReport` gives the ask, the cause and `until`, equal to a hold's `reportInvocation`,
   `noReport` and `retryAt` there. An Agent's status shows its login's to whoever may inspect the
   Agent, with each invocation null where the inspecting actor may not inspect the login. No
   inspection starts an ask, makes a hold or changes anything (OBS-006).
9. **The expected failures and the rows.** In `conformance/expected-failures.yaml`, remove
   `requests-admission-hold`'s, `requests-admission-no-report`'s and
   `requests-admission-wait-ends`'s entries, which this order makes pass; give
   `requests-admission-single-use` the reason "PND-016, order 316: the first build stores no
   control's levers (REV-007), so the fresh ask after a lever change sends the levers as
   registered; REV-007's storing at the levers place makes it pass", and
   `requests-admission-warning` "IFC-054, order 318: the first build asks for a login's report,
   waits and holds, but carries no warning; the guard's engine order for the warnings makes it
   pass". Change no case. In `engine/CONFORMANCE.md`, wrapping at the file's width:
   - In order 317's section, "The engine holds no call on a report: the hold and the warnings are
     later orders' (DEC-095, DEC-099), and order 319 writes the ask (below)." → "The engine warns
     of no call yet: the warnings are a later order's (DEC-099), and orders 319 and 320 write the
     ask and the hold (below)."
   - In order 319's section, "This build holds no call on a report or for want of one: a report
     whose `admit` is false, a single-use report another call spent, an ask that gave no report
     and the 60 seconds after one, in which no ask starts on the login and which the status gives
     in `noReport`, each leave the call dispatched as with no `admission` stated, until the hold's
     order (DEC-095, DEC-099)." → "From order 320, a report whose `admit` is false, a single-use
     report another call spent, an ask that gave no report and the 60 seconds after one, in which
     no ask starts on the login and which the status gives in `noReport`, each hold the call
     (below)."
   - In the row `DL21`, on its one line, "and it is revisited once the usage guard's probe has
     timed a hook's read." → "and the usage guard's probe of 2026-11-24, whose reads of a login
     took under two seconds, kept it."
   - At the end of the file, a section `## A login's admission hold (order 320)` whose one
     paragraph reads: "From order 320, a call that starts model work on an Agent of a login whose
     Type states `admission` is held, not dispatched, reporting unavailable under PND-016 and
     never queued, where the report that decides it has `admit` false, where its ask's
     single-use report admitted an earlier call waiting on that ask, and, without waiting, for 60
     seconds after an ask that gave no report because it failed, ended unknown, passed DL21,
     could not be dispatched or was ended by a restart (PND-016, DEC-095). Its one error
     diagnostic names the login; says why, in the report's words with the lever and its value,
     the window and the paid usage read, or which ask gave no report and why, or which call a
     single-use report admitted; and says how the hold lifts, with `retryAt`; it carries the same
     in `properties.admission`, in `admission-hold.v1` form, with no amount (DEC-099). An Agent's
     call is named only where the caller may inspect that Agent, and the ask only where it may
     inspect the login. The call's `request-ended` record names PND-016, and its transition holds
     a decision `hold-login-admission`. A report that a lever change ended while its ask ran
     holds no call: the calls waiting on it wait for a fresh ask (DEC-169). A waiting call's
     dispatch conditions are checked before the hold, and one that fails ends as row DL4 says
     (DEC-170). Where the ask ends in a transition whose records fail, its waiting calls end
     unavailable under REC-007. The engine never stops or cancels a dispatched call under
     PND-016, whatever a later report says, and sends it as with no `admission` stated (DEC-120).
     A call admitted on a report whose `reason` is `warning` or `cannot-tell` is dispatched with no
     warning yet: the warnings are a later order's (DEC-099)."

   Then run `PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check`, and `--write` where it fails.
10. **Tests.** Write them first, run them on the otherwise unchanged code and report each line.
    Core tests go in `engine/crates/core/src/admission_tests.rs`, beside orders 317's and 319's,
    with their helpers and helpers of yours. *The report state* is order 317's. *A* and *B* are
    root's `call-a.turn.v1` and `call-b.turn.v1`, each built as an `Input::Invoke` with an
    `Evidence` of its own at its time; *K* is the ask the first waiting call starts. R1 is
    `report_one()`; R2 is `requests-admission-hold`'s step 18 report and R3
    `requests-admission-single-use`'s step 19 report, each read from its case as
    `admission_single_use_report_spent_once` reads R3. *H(…)* is a form with `login` `login`,
    `dispatched` false and `levers` `{type: example.org/conformance/shared-login#login-holder.v1,
    pointer: /admission, control: example.org/conformance/shared-login#set-admission.v1}`, and
    the five fields given. Check each form against `admission-hold.v1` as order 317 checks a
    status against its schema. Times are on 2026-11-21 in UTC.
    - `admission_held_on_a_refusing_report` (PND-016, DEC-095, DEC-099; decisions 1, 2, 4 and 5):
      in the report state, A at 13:01:00 waits on K; K, its discovery completed, completed with
      R2 at 13:01:00: no intent; A's reply, in `request_replies`, is as decision 4 gives it, with
      H(report R2, reportInvocation K, noReport null, singleUse null, lifts `newer-report`,
      retryAt `2026-11-21T13:11:00Z`); A's `request-ended` record names PND-016, unavailable, not
      dispatched; the transition holds one `hold-login-admission` decision and no
      `access-decision` record; no entry of A is in progress. The text contains the login's
      label, `/admission/paidUsage`, `refuse`, "not reported", `2026-11-21T13:11:00Z`, K's ID,
      "not dispatched" and "Interface", and none of "amount", "balance", "credit" or
      "currency". B at 13:02:00 is held at once: the transition's reply gives the same form, with
      no intent and no ask started (`admission_asks` empty). R1 kept by `keep` at 13:03:00
      dispatches nothing of A or B, and A at 13:03:10 is dispatched on it. In a fresh report
      state, R2 with `lifts` `lever-change`, kept by `keep` at 13:01:00, holds A at 13:01:30 with
      H(report that report, reportInvocation its ask, noReport null, singleUse null, lifts
      `lever-change`, retryAt null), and the text holds decision 5's sentence with
      `/admission`, the control's reference and the login's label. In a fresh report state, R6,
      a threshold refusal, kept by `keep` at 13:01:00, holds A at 13:01:30 with H(report R6,
      reportInvocation its ask, noReport null, singleUse null, lifts `at-time`, retryAt
      `2026-11-21T13:11:00Z`), and the text contains `/admission/refuseAt`, `95`,
      `example/all/300m`, `97`, `2026-11-21T13:00:30Z` and `2026-11-21T14:00:00Z`; R6 is
      `{"admit":false,"reason":"threshold","lever":{"pointer":"/admission/refuseAt","value":95},"window":{"id":"example/all/300m","usedPercent":97,"resetsAt":"2026-11-21T14:00:00Z"},"paidUsage":{"possible":false,"inUse":false,"resetsAt":null},"cannotTell":null,"lifts":"at-time","observedAt":"2026-11-21T13:00:30Z","source":{"client":null,"method":"example-read"},"validForSeconds":600,"singleUse":false}`,
      whose lever, as order 318's R4's, is one a package may add and the engine keeps as
      received (IFC-054). Today A and B are dispatched, and each A after them.
    - `admission_held_for_want_of_a_report` (PND-016, IFC-054, OPS-007, DL21, REC-007, IMP-007;
      decisions 1 (a) and (d), 3 and 4): in the report state, A waits on K at 13:01:00; K fails
      at 13:01:00: A is held with H(report null, reportInvocation K, noReport `failed`, singleUse
      null, lifts `at-time`, retryAt `2026-11-21T13:02:00Z`), no intent; B at 13:01:30 is held at
      once with the same fields and no ask; R1 kept by `keep` at 13:01:40 does not lift the
      period: A at 13:01:45 is held as B was; at 13:02:00, the period over, A is dispatched on
      R1, which is current until 14:01:40, with no ask. In fresh report states: K fails at
      13:01:00, and at 13:02:00 A starts an ask K2, which, answered succeeded with R1 lacking
      `singleUse`, holds A with `reportInvocation` K2, `noReport` `unknown` and retryAt
      `2026-11-21T13:03:00Z`; K ended by `admission-ask-deadline` at
      13:01:30 leaves one intent, K's `cancel`, and holds A with `noReport` `deadline` and
      retryAt `2026-11-21T13:02:30Z`; K's discovery answered with no extension, as
      `admission_ask_without_a_report_starts_none_for_60_seconds` answers it, holds A with
      `reportInvocation` null and `noReport` `not-dispatched`; and `Input::Recover` at 13:01:30
      ends A under REC-007, as order 319 built it, and B at 13:02:00 is held with
      `reportInvocation` K, `noReport` `unknown` and retryAt `2026-11-21T13:02:30Z`. Each text
      says why the ask gave no report and gives `retryAt`. Today each is dispatched.
    - `admission_single_use_holds_the_others_after_call` (PND-016, IFC-054, DEC-170; decisions
      1 (c), 2 and 4): in the report state, A, then B, then a second B (B2) wait on K at
      13:01:00; root's `access.close.v1` of `call-a` succeeds; K completed with R3 at 13:01:00:
      A ends under decision 4 of order 319, with no `properties.admission`; B is dispatched and
      spends R3; B2 is held with H(report null, reportInvocation K, noReport null, singleUse B's
      ID, lifts `after-call`, retryAt `2026-11-21T13:02:00Z`), and its text names B and says the
      caller need not wait for B to end. B2 sent again at 13:01:10 waits on a fresh ask whose
      `previous` is R3. Today B2 is dispatched.
    - `admission_hold_names_only_what_the_caller_may_inspect` (PND-016, DEC-099; decision 6):
      in the report state of a project of yours, made from the guarded one as
      `control_project()` is, with a Conduit from `observer` to `b` listing `turn.v1`, opened,
      and `observer-scope` left closed; first assert, with `access::may_inspect`, that `observer`
      may inspect neither `login` nor `a` (where it may, change the project until it may not,
      and report how). Root's A, then `observer`'s call on `b`, wait on K; K completed with R3:
      A is dispatched; `observer`'s call is held with `reportInvocation` and `singleUse` null and
      `login` named, and its text names neither K nor A, saying "a call of an Agent of the
      login". In a fresh state of that project, `observer`'s call held on R2 has
      `reportInvocation` null and its text no ID of K. Today both are dispatched.
    - `admission_hold_never_stops_a_dispatched_call` (PND-016, DEC-120; decision 7): in the
      report state, A and B wait on K at 13:01:00; K completed with R1: both are dispatched; B's
      call ends at 13:02:00 with `limits_member()`, which ends R1 `newer-limits`; B at 13:03:00
      starts an ask K2, completed with R2 at 13:03:00: B is held; no transition from A's dispatch
      on holds an intent naming A, and A stays in progress; A's call completed succeeded at
      13:04:00 ends succeeded, its reply with no PND-016 diagnostic and no
      `io.github.shrav-k.conduit/admission` in `_meta`. Today B is dispatched at 13:03:00.
    - `admission_status_shows_what_holds` (IFC-054, DEC-167, OBS-006; decision 8): at A's hold
      at 13:01:00 in each of the first two tests' first scenarios, root's `logins.inspect.v1` on
      `login` and `workers.inspect.v1` on `a` give `admission` agreeing with the hold as decision
      8 says, and `observer`'s inspection of `a` through `observer-scope`, opened as order 317's
      status test opens it, the same with each invocation null. Run each inspection with `step`
      and keep its transition, not through `GuardScenario::inspect`, which discards it: the
      transition has no external intent, and its state's `admission_reports`, `admission_asks`,
      `admission_no_reports`, `invocations` and the objects of `login`, `a` and the Conduits
      equal theirs before the inspection; its access-decision record and the IDs it allocates
      are allowed. Today no hold is made to agree with.
    - An existing test changes only where it asserts the dispatch of a call this order holds.
      Four are expected: in `admission_ask_started_once_for_waiting_calls`, the call on `b` at
      13:01:59, in the period that the REC-007-ended ask started, is held with
      `reportInvocation` K, `noReport` `unknown` and retryAt `2026-11-21T13:02:00Z`, with no
      intent and no ask, its other assertions, the REC-007 ones among them, unchanged; in
      `admission_ask_without_a_report_starts_none_for_60_seconds`, A after
      K fails and after the ask that could not be dispatched, and B at 13:01:30, are held, not
      dispatched; in `admission_single_use_report_spent_once`'s fresh report state, B is held
      `after-call` and only A is dispatched; in `admission_ask_deadline_ends_it_unknown`, the
      deadline's intents are K's `cancel` alone and A is held with `noReport` `deadline`. Report
      each test you change, and any other a hold makes fail, before you change it.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 095 099 120 167 169 170; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1 1
cat spec/promises/pending.yaml spec/implementation-defined.yaml | grep -cE '^- id: (PND-016|DL21)$'  # 2
grep -cE '^    noReport: \{enum: \[failed, unknown, deadline, not-dispatched, null\]\}$|^    lifts: \{enum: \[at-time, newer-report, lever-change, after-call, null\]\}$' spec/core/results.yaml  # 2
k spec/promises/pending.yaml 'A request held on a report or for want of one is not dispatched and reports unavailable' 'the earliest still waiting on its ask whose dispatch-time check passes, and holds every other request still waiting' 'for a single-use report, the call it admitted;' "Who is named: an Agent's call only where the caller may inspect that Agent"  # 1 1 1 1
grep -cE '"id": "requests-admission-(hold|no-report|single-use|wait-ends|warning)"' conformance/expected-failures.yaml  # 5
grep -cE '"id": "requests-admission-(report-kept|not-checked)"' conformance/expected-failures.yaml  # 0
k conformance/expected-failures.yaml "PND-016, order 316: the first build holds no call on a login" 'IFC-054, order 318: the first build dispatches a call on a guarded login with no ask and carries no warning'  # 1 1
grep -cE '^\| (PND-016 \| convention|IFC-054 \| interface) \| must \| 7 \| 5 \| outside \| core \| case passes \|$' engine/STANDING.md  # 2
grep -rlE 'admission-hold|hold-login-admission|admission_hold' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '  # 0
k engine/crates/core/src/admission_ask.rs 'fn decide_report(' "// PND-016's report/no-report hold is the next engine order." 'return decide_report(t, a, c, i, login_id).map(|dispatch| !dispatch);' 'if decide_report(t, a, c, i, &login)? {' 'requests::dispatch_waiting(t, a, c, i)?;' 'requests::record_fault_unavailable(' "pub(crate) fn no_report<'a>("  # 1 1 1 1 1 1 1
k engine/crates/core/src/requests.rs 'pub(crate) fn hold_before_dispatch(' 'pub(crate) fn reject_waiting(' 'if crate::admission_ask::wait(t, a, c, i, &route)? {' 'fn reject_diagnostics('  # 1 1 1 1
k engine/crates/core/src/login_resource.rs '"hold-login-resource-work",' 'pub(crate) fn label(object: &Value) -> String {' '"{tool} reports unavailable under {rule}: {}",'  # 1 1 1
k engine/crates/core/src/admission.rs "pub(crate) fn current<'a>(" 'fn valid_until(&self) -> Option<String> {' 'pub(crate) fn status('  # 1 1 1
k engine/crates/core/src/lib.rs 'pub admission_asks: BTreeMap<String, admission_ask::Ask>,' 'pub admission_no_reports: BTreeMap<String, admission_ask::NoReport>,' 'mod admission_tests;'  # 1 1 1
k engine/crates/core/src/admission_tests.rs 'assert_eq!(dispatched_ids(&t), vec![json!(a), json!(b)]);' 'assert_eq!(dispatched_ids(&t), vec![json!(k), json!(a)]);' 'assert_eq!(dispatched_ids(&t), vec![json!(a)]);' 'fn admission_ask_without_a_report_starts_none_for_60_seconds() {' 'fn admission_single_use_report_spent_once() {' 'fn admission_ask_deadline_ends_it_unknown() {' 'let t = s.admission_call("2026-11-21T13:01:59Z", "call-b.turn.v1");'  # 2 1 4 1 1 1 1
k engine/CONFORMANCE.md "The engine holds no call on a report: the hold and the warnings are later orders' (DEC-095, DEC-099), and order 319 writes the ask (below)." 'This build holds no call on a report or for want of one: a report whose `admit` is false,' "and it is revisited once the usage guard's probe has timed a hook's read." 'From order 320,' "## A login's admission hold (order 320)"  # 1 1 1 0 0
grep -rlE 'admission-hook|loginTypes' packages | wc -l | tr -d ' '  # 0
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it. A third to fifth printed line
other than stated means order 316's spec, with the owner's three strikes, is not in the tree; a
sixth to ninth other than stated, that the guarded cases' entries, their reasons or the standing
rows moved (the seventh's 0 is order 319's removal of two entries); a tenth other than `0`, that
an engine source already names the hold; an eleventh to seventeenth other than stated, that the
code moved since this order was written; an eighteenth other than `0`, that a package states
`admission`, and the Preservation tests' "At risk" sentence no longer holds. Where it is
dispatched in two halves, 320b's worker runs this check on 320a's build as *In two halves* says.

The first line, and the Done when's line that counts files outside Scope, count the tree as it is
dispatched, with this order committed to its base. On a review copy where this order's file is
still untracked, each prints 1 more, for that file alone, and other uncommitted files there add
theirs. The coordinator commits the final order to the dispatch base and reruns both lines there
before dispatch; a worker who sees anything but 0 stops as above.

## Scope

Change, for decisions 1 to 8 and their tests, in `engine/crates/core/src/`: `admission_ask.rs`
(the hold's decision and its form), `admission.rs` only where the hold reads a report's validity
or its single use, `admission_tests.rs` (the six tests and the four changed), `requests.rs`
(ending a call held at once, and the held reply), `explain.rs` only where a PND-016 diagnostic
needs its text, `login_resource.rs` only to share `label` or `hold_work`'s text form, and `lib.rs`
only for a `State` member, with `#[serde(default, skip_serializing_if = …)]` so that a state
with none is written byte for byte as today (DL2), where your build keeps one (none is
expected); the five `requests-admission-*` entries of `conformance/expected-failures.yaml`;
`engine/CONFORMANCE.md`'s two sentences, its row DL21's one clause and the new section; and
`engine/STANDING.md` by the tool alone. Change an existing test only under decision 10. Change
nothing else: not `spec/`, a case, `docs/` (DEC-094, DEC-095, DEC-099, DEC-120, DEC-167, DEC-169
and DEC-170 stay not done here; the coordinator marks each at the merge that ends it), the
shell, the loader, the conformance crate, `packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`.
No promise changes, so `spec/CHANGES.md` gains no line. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `j` and `k` as in the Start check;
the second, guards, passes today and after. Run each existing test you changed by name, once,
too. Run the core's tests and then the whole vectors test, once each, last; never the whole
engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in admission_held_on_a_refusing_report admission_held_for_want_of_a_report admission_single_use_holds_the_others_after_call admission_hold_names_only_what_the_caller_may_inspect admission_hold_never_stops_a_dispatched_call admission_status_shows_what_holds; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 six times; 0 today
for c in hold no-report wait-ends; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -c "^PASS requests-admission-$c "); done  # 1 three times; 0 three times today
grep -cE '"id": "requests-admission-(hold|no-report|wait-ends)"' conformance/expected-failures.yaml  # 0; 3 today
for c in requests-admission-single-use requests-admission-warning; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. $c 2>&1 | grep -A1 "^EXPECTED FAILURE $c " | sed -n 2p | cut -c1-10); done  # "  step 35:", "  missing ", one per line; "  step 19:", "  missing " today
k conformance/expected-failures.yaml 'so the fresh ask after a lever change sends the levers as registered' 'waits and holds, but carries no warning'  # 1 1; 0 0 today
k engine/CONFORMANCE.md 'From order 320,' "## A login's admission hold (order 320)" 'orders 319 and 320 write the ask and the hold (below).' 'took under two seconds, kept it.' 'This build holds no call on a report' "and it is revisited once the usage guard's probe"  # 2 1 1 1 0 0; 0 0 0 0 1 1 today
grep -cE '^\| (PND-016 \| convention|IFC-054 \| interface) \| must \| 7 \| 2 \| outside \| core \| case passes \|$' engine/STANDING.md  # 2; 0 today
grep -rlE 'hold-login-admission' engine/crates/core/src | wc -l | tr -d ' '  # more than 0; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-core login_resource_ 10  # 1
s conduit-core limits_ 8  # 1
for t in admission_types_checked_at_install_and_submission admission_checked_at_registration_and_replacement admission_control_pair_refused_before_dispatch admission_report_kept_with_its_validity admission_report_ended_early admission_status_shows_the_login_or_not_checked admission_wait_checks_dispatch_once admission_levers_changed_while_asking admission_wait_cancelled_restarted_or_repeated admission_waiter_cannot_authorize_onward_request admission_wait_decisions_name_each_call_and_fresh_ask admission_dispatch_diagnostic_distinguishes_failed_and_unreadable_conditions; do s conduit-core "$t" 1; done  # 1 twelve times
for t in admission_reports_read_from_an_earlier_engines_store admission_asks_kept_across_a_reopen admission_ask_ends_at_its_deadline; do s conduit-shell "$t" 1; done  # 1 three times
s conduit-shell memory_equals_disk_after 5  # 1
for c in report-kept not-checked; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -c "^PASS requests-admission-$c "); done  # 1 twice
for p in contracts-admission-shape registration-admission-checks contracts-login-resource- contracts-sign-in- registration-login-place requests-login-resource- requests-limits- requests-usage-kept requests-reconcile- requests-sign-in- requests-cancellation requests-restart-interrupted requests-key-scope change-unknown-setting records-purge- secret-reads-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", N in order: 1 1 1 2 1 2 4 1 8 22 1 1 1 1 2 11, as order 319's Done when gave them; where one prints otherwise before your change, report it and compare after with before
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_private_paths; do PYTHONDONTWRITEBYTECODE=1 python3 tools/$t.py | tail -1; done  # Errors: 0, three times
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed; echo "exit $?"  # Vector delta: the step-33 edit's changes, each declared in docs/work-orders/320-declared.yaml; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(admission_ask|admission|admission_tests|requests|explain|login_resource|lib)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/expected-failures\.yaml|conformance/conventions/requests/admission-single-use/case\.yaml|docs/work-orders/320-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Of order 316's expected failures, `requests-admission-hold`, `requests-admission-no-report` and
`requests-admission-wait-ends` flip to pass and their entries go. `requests-admission-single-use`
then first fails at step 35, where the fresh ask after a lever change is sent the levers as
registered, as this build stores no control's values (REV-007); and order 318's
`requests-admission-warning` still first fails at step 18, where `a`'s dispatch record holds no
`admissionWarning`. Neither case changes. Order 318's `access-admission-control-open` and
`access-ask-admission-refused` are untouched.

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code, then its passing line; each case's line before and after; each existing test you
changed, and why; your own choices; and every check's last line. Stage nothing.

**In two halves.** Where the order is dispatched as 320a and 320b, each half changes only the
files of its own decisions within Scope and runs this Start check and Done when with these
differences.

- 320a: decision 1 but (a), (d) and its record-fault sentence; decisions 2 and 4 to 7, for holds
  on a report; of decision 9, `requests-admission-hold`'s entry removed, the single-use reason,
  and the sentence of order 317's section; of decision 10,
  `admission_held_on_a_refusing_report`, `admission_single_use_holds_the_others_after_call`,
  `admission_hold_names_only_what_the_caller_may_inspect`,
  `admission_hold_never_stops_a_dispatched_call` and the change to
  `admission_single_use_report_spent_once`. Until 320b, a call in the period or waiting on an ask
  that gave no report is dispatched as order 319 built it; no test 320a runs reaches that, and
  `admission_ask_started_once_for_waiting_calls`,
  `admission_ask_without_a_report_starts_none_for_60_seconds` and
  `admission_ask_deadline_ends_it_unknown` stay guards, run by name, 1 each. In its Done when, these lines replace the
  first group's first five and its CONFORMANCE line:

  ```sh
  for t in admission_held_on_a_refusing_report admission_single_use_holds_the_others_after_call admission_hold_names_only_what_the_caller_may_inspect admission_hold_never_stops_a_dispatched_call; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 four times; 0 today
  (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-hold 2>&1 | grep -c "^PASS requests-admission-hold ")  # 1; 0 today
  grep -cE '"id": "requests-admission-(hold|no-report|wait-ends)"' conformance/expected-failures.yaml  # 2; 3 today
  for c in no-report wait-ends single-use warning; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -A1 "^EXPECTED FAILURE requests-admission-$c " | sed -n 2p | cut -c1-10); done  # "  missing ", "  step 26:", "  step 35:", "  missing "; "  missing ", "  step 26:", "  step 19:", "  missing " today
  k conformance/expected-failures.yaml 'so the fresh ask after a lever change sends the levers as registered' 'waits and holds, but carries no warning'  # 1 0; 0 0 today
  k engine/CONFORMANCE.md 'From order 320,' "## A login's admission hold (order 320)" 'orders 319 and 320 write the ask and the hold (below).' 'took under two seconds, kept it.' 'This build holds no call on a report' "and it is revisited once the usage guard's probe"  # 0 0 1 0 1 1; 0 0 0 0 1 1 today
  ```

  and its STANDING line reads `7 | 4` for both rows (`7 | 5` today), as 320a removes one entry.
- 320b, on 320a's build: decision 1 (a), (d) and its record-fault sentence; decisions 3 and 8;
  the rest of decision 9; of decision 10, `admission_held_for_want_of_a_report`,
  `admission_status_shows_what_holds` and the changes to
  `admission_ask_started_once_for_waiting_calls`,
  `admission_ask_without_a_report_starts_none_for_60_seconds` and
  `admission_ask_deadline_ends_it_unknown`. Its Start check's sixth, eighth, ninth, tenth and
  sixteenth lines, and its eleventh to fifteenth and seventeenth where 320a changed those files,
  print what 320a left (about: `4`; `0 1`; `0`, as both rows read `7 | 4`; more than `0`;
  `1 1 4 1 1 1 1`; `0 1 1 0 0` for the seventeenth): report them as they print and go on;
  every other line holds as stated. Its Done when's first loop runs its two core tests (1
  twice; 0 before), its case loop `no-report` and `wait-ends` (1 twice; 0 twice before), its
  entries line gives 0 (2 before), its single-use and warning loop is a guard printing
  `  step 35:` and `  missing ` before and after, its reasons line `1 1` (`1 0` before), its
  CONFORMANCE line `2 1 1 1 0 0` (`0 0 1 0 1 1` before), its STANDING line `7 | 2` (`7 | 4`
  before), and the first-half tests and `requests-admission-hold` are guards.

## What follows

- **The warnings and the open** (DEC-099, DEC-119, DEC-120), the guard's next engine order (E2b):
  a call admitted on `warning` or `cannot-tell` carries `admission-hold.v1` with `dispatched`
  true under `io.github.shrav-k.conduit/admission` and in its dispatch record's
  `admissionWarning`, built where this order builds the hold's form; the refusal of an Agent's
  Conduit listing a login Type's admission control; and a test that a call reaches the Agent's
  implementation the same with the guard as without. `requests-admission-warning` and
  `access-admission-control-open` then pass; `access-ask-admission-refused`'s entry goes with
  DEC-064's orders.
- **Found: the single-use case needs the levers stored**, as order 319's What follows says:
  `requests-admission-single-use` stays an expected failure, first failing at step 35, until
  REV-007's storing at the levers place is built; the coordinator orders it or asks the owner.
- **Found: order 318's warning reason was stale on order 319's build.** Its entry still said the
  first build dispatches "with no ask"; order 319 did not touch it. This order rewrites it
  (decision 9).
- **The packages' new Types** (DEC-124): the login Types' next versions state `admission`, with
  their hooks and `set-admission.v1`. For a team seat whose login can go on in paid usage, its
  template's `paidUsage` is `refuse`, which holds its calls once paid usage is in use, or
  `refuse-if-possible`, which holds every call while paid usage is possible at all: the owner
  chooses before the sitting (DEC-102, DEC-124).
- **DL21** stays 30 seconds: the probe kept it, and decision 9 says so in its row.
- DEC-094, DEC-095, DEC-099, DEC-120, DEC-123, DEC-124, DEC-167, DEC-169 and DEC-170 stay not done
  until the guard's orders merge, and no team-seat Agent takes a turn before then (DEC-102).

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on the stack of orders 307 and 314 to
319 at its start. Drafted, built and read on the team seats (DEC-179, DEC-180).

**The draft.** The drafter, `root-drafter.turn.v1`, wrote it on the Team seat; verifier-3 checked
it adversarially (four musts), and the drafter revised it.

**Worker-3's turns**, on the Business seat, built 320a, then 320b. 320b stopped at a scope
blocker: the single-use case of order 318, at step 33, forbade every request-ended text under the
records, while the ask itself is a request and ends with its own request-ended record (REC-001,
REC-002). The coordinator ruled the case wrong, not the engine, and allowed one edit there: the
step's exact records now include the ask's own end, which still shows that the held Agent call
did not end; the edit is declared in `docs/work-orders/320-declared.yaml`. A third turn finished
every check.

**Two blind reads.** The Codex verifier, `root-verifier-3.turn.v1`, verified the engine's
behaviour and the ruled edit, with one must in the order's own text: the warning case's check
expected a step prefix the runner never prints. The Claude reader, `root-reader-a.turn.v1`, found
the build sound, with three shoulds and several nits. The coordinator fixed the order's check
lines (ruling 87); one fix turn by worker-3 answered the rest. The gate's run on the squash is the
coordinator's, at the merge.
