# Work order 321: a call dispatched on a report that warns carries the warning, and no Agent's Conduit opens listing a login's admission control (DEC-098, DEC-099, DEC-119, DEC-120, the usage guard's fifth engine order)

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
list or sweep no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.**
[DEC-099](../decisions/log.md#dec-099-2026-11-23-a-guarded-hold-tells-the-caller-everything-but-amounts),
"Accept (Recommended)": the caller is told the login, the lever and its value, the window, its
used percentage, the threshold, the reset, how it lifts and which report, in text and a
structured field, with no amount.
[DEC-098](../decisions/log.md#dec-098-2026-11-23-an-unknown-usage-report-is-decided-per-account-kind),
"Per account kind (Recommended)": where a login cannot tell, one account kind allows one warned
trial.
[DEC-119](../decisions/log.md#dec-119-2026-11-23-no-agent-tunes-a-logins-usage-levers),
"Not Agents, ever (Recommended)": no Conduit whose consumer is an Agent lists a login Type's
admission control, and such an open reports refused; an Interface's client tunes.
[DEC-120](../decisions/log.md#dec-120-2026-11-23-the-guard-checks-before-each-turn-and-a-started-turn-runs-to-its-end),
"I don't want to modify how the models operate just police their usage", then "Yes, check before
turns only (Recommended)": no lever reaches a turn, and turns stay as they are. DEC-094 (the
engine trusts the package's report) and
[DEC-124](../decisions/log.md#dec-124-2026-11-23-the-levers-come-on-new-type-versions) (the
levers come on new Type versions) stand behind it.
[Order 318](318-admission-warnings-and-refused-open.md) wrote IFC-054's warning, `record.v1`'s
`admissionWarning`, CTR-015's refused open and three cases;
[order 320](320-engine-holds-calls-on-the-guard.md) built the hold and left the warning to this
order, in `hold_form`. This order builds both: a call admitted on a report whose `reason` is
`warning` or `cannot-tell` is dispatched as any other, and its caller learns the warning from the
result; and an open of an Agent's Conduit that lists its login's admission control is refused.

**Today** (order 320's build: `main` with the decision log through DEC-157, the builds of orders
307 and 314 to 320 on top, and the decision log's DEC-158 to DEC-171), no engine source names
`admissionWarning`. In `admission_ask.rs`, `decide_report` answers "dispatch" for every report
whose `admit` is true, whatever its `reason`, spending a single-use one, and `hold_form`, marked
"the warning order adds its dispatched case here", builds every form with `dispatched` false. A
call's `invocation-dispatched` record is written in `invoke_inner` for a plain call that arrives,
in `dispatch_waiting` for one that waited, and in `complete_discovery` for one that is not plain.
`finish` builds the result that ends an attempt, adding `pendingAction` and `signInLogin` to its
`_meta`, but an IFC-026 reason end, in `complete` and `record_fault_reason`, builds its reply
after `restore_or_remove_invocation` and without `finish`; `invoke_inner` answers an OPS-009
repeat with `ended_reply`. `hold_form` errs where the login's Type or its control is absent.
In `access.rs`, `evaluate_path`'s block for a Conduit to be opened refuses under ACC-054,
SEC-008 and ACC-032, and nothing refuses one that lists a login's admission control;
`explain.rs`'s `REFUSAL_CAUSES` has no CTR-015 row. So `requests-admission-warning` first fails
at step 18, where `a`'s dispatch record holds no `admissionWarning`, and
`access-admission-control-open` at step 17, where root's open of `agent-admission-control`
succeeds. `access-ask-admission-refused` fails as no engine offers `request-access.v1`
(ACC-058). `engine/STANDING.md` gives PND-016 and IFC-054 `7 | 2`, CTR-015
`3 | 1` and ACC-056 `9 | 3`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 1, the
warning decides nothing and reaches no Agent's implementation: a warned call is sent exactly as a
call within the levers (DEC-120), on the report the package gave (DEC-094); 2, the refusal only
narrows: nothing is opened or granted, and an Interface's client still tunes; 6, the warning
starts no call, no ask and no model work, and the refused open dispatches nothing; 7, the warning
is kept in the dispatch record and names the report and its ask, and the refusal is recorded as
any is. At risk, 5, every management action available to an agent under a grant: no Agent may
hold a login's admission control, as none may hold `answer.v1` or `sign-in.v1` (PND-009,
PND-010), which is DEC-119's accepted cost. In use, none: no package states `admission` yet (the
Start check's `packages` line), so no store but a test's holds a guarded login. The team seats
already take turns under DEC-179, a later log entry on `main`, unguarded until the packages' new
Types and the sitting (DEC-124).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
IFC-054's warning and CTR-015's refused open are musts since order 318, and DEC-098's one warned
trial reaches its caller only through the warning. By existing practice, a Kubernetes validating
admission webhook may allow a request and return `warnings`, which the API server sends to the
client as `Warning` headers with code 299 while the request goes on unchanged; a service control
policy's explicit `Deny` holds for every principal, whatever its own policies allow; and the
engine already refuses an open in the same place under SEC-008, as PND-009 and PND-010 keep
`answer.v1` and `sign-in.v1` off a Conduit whose consumer is not an Interface. Not taken: the
warning in a turn's request or its `_meta` (DEC-120); in `content` or `structuredContent`
(IFC-025); the warning deciding, delaying or holding any call; the warning in an end record;
refusing such a Conduit at registration; refusing one whose consumer is an Interface or a
Resource (DEC-119 names Agents alone); refusing the control on an Agent's onward request through
another instance, beyond DEC-119's words; and ACC-058's two rows, which need `request-access.v1`
(DEC-064's engine orders).

**When it runs.** It stacks on orders 307 and 314 to 320 and runs on 320's build, commit
`484de85925a89b4ea98e0ce03744877ca0c0e7b4`, until they merge, then on `main`. It is dispatched
in two halves from the start, each through Conduit with its own Codex worker turn, the blind
Codex verifier and Claude reader at once, one fix turn and the gate (DEC-046, DEC-071, DEC-033):
321a, the warning (decisions 1 to 5 and 8's warning parts, tests 1 to 5), and 321b, the open
(decisions 6 and 7 and 8's open parts, test 6). Order 320, of a like size, took three worker
turns (its Record), and this order has three record sites, `finish`, two ends outside it, the
repeat, the open check, two texts and six tests with sub-cases. *In two halves*, after the Done
when, gives each half's differences; where the coordinator dispatches the order whole, the Done
when holds as written. By ruling 89, each decision says what must hold; how is yours. The
coordinator's, open to the owner's strike (DEC-029): the two halves; the warning decided where
`decide_report` admits, built by `hold_form`, and kept on the attempt's entry; written in the
dispatch record wherever that record is written, plain or after discovery; the key on a result
exactly where its attempt's dispatch record was written, in `finish`, in the two IFC-026 ends
and on an OPS-009 repeat; a restored entry keeping the earlier attempt's warning; an attempt
that discovery ends unsent, or that is cancelled while its discovery is pending, carrying none;
the refusal decided in `evaluate_path`'s block for a Conduit to be opened, after SEC-008 and
ACC-032; one function that resolves a login Type's control, and gives none where there is no
control; both texts; `access-ask-admission-refused`'s new reason; and the test names.

**Shared files.** On `main`, order 305 has changed REC-002 and `record.v1`, and order 311 drops
`held_target` and `implementation` from an ended entry and keeps every other member, the warning
among them; the guard's attribution engine order (DEC-117) changes the end record that `finish`
writes. DEC-064's engine orders build ACC-058 in `access.rs` and `explain.rs` and take this
order's control resolution for ACC-058's last row. DEC-190's order turns
`conformance/expected-failures.yaml` into one fragment per order. Where it merges before this
order is dispatched, the coordinator first rewrites, for the fragments, the Start check's eighth
and ninth lines, decision 8, the Done when's entries and reasons lines and file list, and the
Scope; a worker who meets the file split stops at the Start check. Whichever merges later
rebases and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first PND-016, IFC-054, CTR-015, ACC-006, ACC-056, ACC-058, IFC-006, IFC-025, IFC-037,
OPS-007, OPS-009, OPS-012, OPS-024, OPS-027, IMP-007, REC-001, REC-002, CON-004 and OBS-002; the
schemas `admission-hold.v1`, `admission-report.v1`, `diagnostic.v1` and `record.v1`
(`spec/core/results.yaml`); orders 318 and 320 whole, with their Records; the cases
`requests-admission-warning` and `access-admission-control-open`; and in `engine/CONFORMANCE.md`
the sections of orders 317, 319 and 320 and "Refusals and reasons (order 046a)". In the core,
`admission_ask.rs` whole; `invoke_inner` from its repeat answer to its dispatch record,
`invoke_part`, `dispatch_waiting`, `complete_discovery`, `complete`'s validated-reason branch,
`record_fault_reason`, `restore_or_remove_invocation`, `cancel`, `finish`, `ended_reply` and
`record_content` in `requests.rs`; `REFUSAL_CAUSES`, `diagnostic`, `tier` and `admission_hold`
in `explain.rs`; `evaluate_path` and `may_inspect` in `access.rs`; `admission::status`; and
orders 317's to 320's helpers in `admission_tests.rs` (`GuardScenario`, `report_one`,
`single_use_report`, `control_project`, `waiting`, `dispatched_ids`, `result`, `hold_reply`).
*A warned call* is an
attempt (OPS-012, OPS-024) that PND-016 dispatches on a report whose `admit` is true and whose
`reason` is `warning` or `cannot-tell`; *its warning* is the diagnostic of decision 2. A case's
steps are named as the runner prints them, counted from 1.

1. **Where the warning is decided** (PND-016, IFC-054, DEC-098). Decide it where `decide_report`
   admits a call, on the report that decided: the login's current report for an arriving call,
   the ask's kept report for a waiting one, a single-use report as it spends itself. The attempt
   is warned where that report's `reason` is `warning` or `cannot-tell`, and not where it is
   `within-levers`. Build the warning there, once, before dispatch, and keep it on the attempt's
   invocation entry. An entry that is not warned holds nothing new, so a state with none is
   written byte for byte as today (DL2). The ask is never warned, a held call is not warned, and
   a call on a login whose Type states no `admission` is not checked or warned. A new attempt is
   decided afresh and carries its own warning or none, never an earlier attempt's.
2. **The form** (IFC-054, DEC-099). `hold_form` builds it, with a cause for a dispatched call:
   `login`, `levers` and `reportInvocation` as for a hold on a report (the ask that gave the
   report, null where the caller may not inspect the login, `access::may_inspect`), `dispatched`
   true, `report` the report whole, and `noReport`, `singleUse`, `lifts` and `retryAt` null. The
   warning is `{"level":"warning","ruleId":"PND-016","message":{"text":…},"properties":{"admission":<form>}}`.
   Each form validates against `admission-hold.v1`.
3. **Where it goes** (IFC-054, REC-002, IFC-025, OPS-009).
   - The attempt's `invocation-dispatched` record holds the warning in `admissionWarning`,
     whatever the Type's `records` states: at once for a plain call, whether it arrived or
     waited, and when discovery completes for one that is not plain. No other record holds it,
     the ask's records and the call's end records among them.
   - The result that ends the attempt holds the warning in `_meta` under
     `io.github.shrav-k.conduit/admission`, whatever its outcome or reason. A result carries it
     exactly where that attempt's `invocation-dispatched` record was written, in the
     transition's reply and in `request_replies` alike. `finish` builds most such results,
     beside `pendingAction`: a succeeded, failed or unknown end, a cancel (OPS-027) and a
     restart's end (IMP-007) each carry it. An IFC-026 reason end does not pass through
     `finish`: `complete`'s validated-reason branch and `record_fault_reason` build its reply
     after `restore_or_remove_invocation`, so read the warning from the entry as it was before
     that, and carry it there too. `finish` also ends attempts never dispatched, which carry
     none: one cancelled while its discovery is pending, and a hook that `complete_discovery`
     drops (`drop-hook-of-ended-parent`). A repeat that OPS-009 answers from that attempt
     (`ended_reply` in `invoke_inner`) carries it too.
   - Where an attempt ends held, unsent or with an IFC-026 reason and its entry is restored to
     the earlier attempt (`restore_or_remove_invocation`), the entry keeps the earlier attempt's
     warning, or none where it had none, never the new attempt's. Under one contract no reply
     shows this, since OPS-009 redispatches every restored entry of a guarded call (failed, or
     unknown, not plain and deduplicated; PND-016 guards only `startsModelWork` operations, never
     read-only ones); keep it so that the entry stays true.
   - Nothing else of a result changes: `isError`, `content`, `structuredContent`, the first text
     block and the other `_meta` keys are as with no warning, and a pending reply carries none.
     An attempt that discovery ends unsent (IFC-037), or that is cancelled while its discovery
     is pending, was not dispatched, and carries none.
4. **The warning's words** (IFC-054, DEC-099). Its text begins with the tool's name, bare, as
   the hold's text does, then " was dispatched under PND-016 on a report that warns: ", and then,
   in this order:
   - names the login by kind, name and ID (`login_resource::label`);
   - says that the call was dispatched, started model work and runs to its end;
   - for `warning`, names the lever's pointer and value, and the window with its used percentage
     and reset; for `cannot-tell`, says why in `cannotTell`'s word, with the words
     `admission_hold` gives that word, and names the lever, `whenUnknown`'s pointer, and its
     value;
   - for a single-use report, says that the report admitted this call alone and is spent, so
     that the login's next such call waits for a new ask;
   - gives the paid usage read (`possible`, `inUse`, `resetsAt`), each "not reported" where null;
   - says that a later call on the login may be held;
   - names the ask as "ask `<ID>`" only where `reportInvocation` names it, else "the login's ask".

   Neither the text nor the form names an amount, a balance, a credit or a currency. Put the
   words in `explain.rs` beside `admission_hold`, sharing its reading of values.
5. **A warned call is sent as without the guard** (DEC-120, PND-016, CTR-015, IFC-006). A warned
   call's intents, its discovery and its call, with their `params`, `arguments` and `_meta`, are
   those the same call gets on a report within the levers and on a login whose Type states no
   `admission`, IDs apart. No intent holds the key, a lever or any member of the guard. The
   engine never stops or cancels a warned call, and the warning holds no later call: each is
   decided on its own report.
6. **The refused open** (CTR-015, ACC-056, ACC-006, DEC-119). In `evaluate_path`'s block for a
   Conduit to be opened, after SEC-008's and ACC-032's checks, an `open.v1` of a Conduit whose
   consumer is an Agent and that lists an operation equal, as a qualified reference, to its
   target's Type's `admission.control` reports refused under CTR-015. This holds whoever sends
   the open, root among them, on its own or as a setup's part (CON-004): name in your report the
   line by which a part (`invoke_part`) reaches the check. Read the target's Type from its
   publication (`registration::publication_object`) and resolve its control to a qualified
   reference as `hold_form` resolves it, in one function both use. On a refused open nothing is
   dispatched; the Conduit stays as it was, registered and not open; the refusal is recorded as
   any is, its `access-decision` and `request-ended` records with `dispatched` false, reason
   `refused` and `ruleId` CTR-015. A Conduit to such a login whose consumer is an Interface or a
   Resource, or whose consumer is an Agent and that lists only other operations, opens as
   before, and so does every Conduit to a login whose Type states no `admission`. Registration
   refuses none of them. The check runs on every open of a Conduit whose consumer is an Agent,
   so the function gives none, never an error, where the target is not a Resource whose Type
   states `admission.control`: a Pool, a Fleet, an Agent (whose Type's `admission` holds only
   `loginTypes`), `access`, or a Resource whose Type states no `admission`. Such an open goes on
   as today; `hold_form` keeps its error where it gets none.
7. **The refusal's words** (CTR-015, ACC-056). `REFUSAL_CAUSES` gains CTR-015's row of ACC-056's
   table, word for word: "No Conduit whose consumer is an Agent may be opened listing a login's
   admission control; an Interface's client tunes the levers."
   (`refusal_causes_match_spec_table_verbatim` checks it). The refusal's cause, all in tier 1,
   is that sentence and then: "The Conduit `<as named>` lists `<operation>`, the admission
   control of `<Type>`, its target's Type, which changes that login's usage levers; no Conduit
   whose consumer is an Agent may be opened listing it, whoever sends the open. Nothing was
   opened, not even the Conduit's other operations, and the Conduit stays as it was. A Conduit
   whose consumer is an Agent may still list the login's other operations: replace or register
   this one again without the admission control, then open it. An Interface's client tunes the
   levers through a Conduit of its own to the login." The Conduit is named as the request named
   it, the operation and the Type as qualified references. Root's diagnostic is tier 2, as
   ACC-056 gives root. Carry these facts in the route, as `rootLoginNotice` carries PND-009's.
8. **The expected failures and the rows.** In `conformance/expected-failures.yaml`, remove
   `requests-admission-warning`'s and `access-admission-control-open`'s entries, which this
   order makes pass, and give `access-ask-admission-refused` the reason "ACC-058, order 318: the
   first build does not offer request-access.v1; DEC-064's engine orders make it pass, with
   ACC-058's rows for the admission hook and a login's admission control". Change no case. In
   `engine/CONFORMANCE.md`, wrapping at the file's width:
   - in order 317's section, "The engine warns of no call yet: the warnings are a later order's
     (DEC-099), and orders 319 and 320 write the ask and the hold (below)." → "Orders 319, 320
     and 321 write the ask, the hold and the warnings (below).";
   - in order 320's section, "A call admitted on a report whose `reason` is `warning` or
     `cannot-tell` is dispatched with no warning yet: the warnings are a later order's
     (DEC-099)." → "From order 321, a call admitted on a report whose `reason` is `warning` or
     `cannot-tell` is dispatched warned (below).";
   - at the end of the file, a section `## A login's admission warning and its control (order 321)`
     of two paragraphs. The first: "From order 321, a call that PND-016 dispatches on a report
     whose `reason` is `warning` or `cannot-tell` is warned: the record of its dispatch holds in
     `admissionWarning`, and the result that ends that attempt, whatever its outcome, and a
     repeat that OPS-009 answers with it, hold in `_meta` under
     `io.github.shrav-k.conduit/admission`, one warning diagnostic citing PND-016 whose
     `properties.admission` is `admission-hold.v1` with `dispatched` true (IFC-054, DEC-099). Its
     text names the login, the lever and the window, or why the login cannot tell, and the paid
     usage read, says that a later call may be held, and names the ask only where the caller may
     inspect the login, with no amount. The call is sent exactly as on a report within the
     levers, and nothing else of its result changes (DEC-120). A dispatch on a report within the
     levers is not warned, and each new attempt is warned, or not, on its own report." The
     second: "From order 321, an open of a Conduit whose consumer is an Agent and that lists its
     target's Type's `admission.control` reports refused under CTR-015, whoever sends it, root
     and a setup's part among them, and the Conduit stays as it was; one whose consumer is an
     Interface or a Resource opens as before (DEC-119). ACC-058's rows for the admission hook and
     a login's admission control wait for DEC-064's engine orders, which build
     `request-access.v1`."

   Then run `PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check`, and `--write`
   where it fails.
9. **Tests.** Write them first, run them on the otherwise unchanged code and report each line.
   Core tests go in `engine/crates/core/src/admission_tests.rs`, beside orders 317's to 320's,
   with their helpers and helpers of yours. *The report state*, *A*, *B* and *K* are order 320's;
   *the discovered project* is `guarded()` with `shared-agent.v1`'s implementation made
   extension-aware, as `admission_hold_restores_keyed_attempt_without_wait_fields` makes it. R1
   is `report_one()`, R3 `single_use_report()`, and R4 `requests-admission-warning`'s step 22
   report, read from its case as `single_use_report` reads R3. *W(R, K)* is a warning whose form
   has `login` `login`, `dispatched` true, `report` R, `reportInvocation` K, `levers` as order
   320's H, and `noReport`, `singleUse`, `lifts` and `retryAt` null. Check each form against
   `admission-hold.v1` as order 317 checks a status. Times are on 2026-11-21 in UTC.
   - `admission_warning_on_a_report_that_warns` (IFC-054, PND-016, DEC-099; decisions 1 to 4):
     in the report state, R4 kept by `keep` at 13:02:00 as K; A at 13:02:10 is dispatched at
     once, its `invocation-dispatched` record holding W(R4, K) in `admissionWarning`, its intent
     no key; A completed succeeded at 13:03:00: the reply holds W(R4, K) under the key, and with
     the key removed equals the reply of the same completion of a call dispatched on R1 in a
     fresh report state, the invocation ID apart; its end record holds no `admissionWarning`. The
     text contains the login's label, `/admission/warnAt`, `80`, `example/all/300m`, `85`,
     `2026-11-21T14:00:00Z`, "not reported", K's ID and "may be held", and none of "amount",
     "balance", "credit", "currency" or "unavailable". A second call on `a` at 13:03:10 is
     dispatched warned as well, and completed failed at 13:03:30 its reply, `isError` true, holds
     the key, its first text block unchanged by it. Today no record or reply holds a warning.
   - `admission_warning_on_a_single_use_report` (IFC-054, PND-016, DEC-098; decisions 1 to 4): in
     the report state, A and then B wait on K at 13:01:00; K completed with R3: A is dispatched
     with W(R3, K) in its dispatch record, and B is held `after-call`, as order 320 built it,
     with no warning. A's text contains `never-observed`, "`limits` is `unknown`",
     `/admission/whenUnknown` and `one-trial`, and says that the report admitted this call alone
     and is spent. In the discovered project, the same: the transition that dispatches A's
     discovery holds no dispatch record of A; its discovery completed with
     `advertised_extension` writes the record, holding W(R3, K); A completed succeeded carries
     the key. Today neither holds a warning.
   - `admission_warning_on_every_end_and_repeat` (IFC-054, OPS-009, OPS-012, OPS-027, IMP-007;
     decisions 1 and 3): in the report state with R4 kept at 13:02:00, a keyed A
     (`keyed_admission_call`) at 13:02:10, completed succeeded at 13:02:20, then sent again at
     13:02:30: the repeat, with no intent, holds the same key as the end; B at 13:03:00 cancelled
     at 13:03:10 (`{"kind":"cancel","invocation":…}`), and A2, a call on `a` at 13:04:00, ended
     by `Input::Recover` at 13:04:10: the result that ends each holds the key. In the discovered
     project with R4 kept at 13:02:00, a keyed A dispatched warned and completed failed at
     13:02:20; R1 kept at 13:03:00; the same keyed request at 13:03:10 is a new attempt, whose
     dispatch record holds no `admissionWarning` and whose result holds no key. In a fresh
     discovered project with R4 kept at 13:02:00, A at 13:02:10 cancelled at 13:02:20 while its
     discovery is pending ends with no key in its result and no dispatch record of A (decision
     3's unsent end). Today none holds a warning.
   - `admission_warning_names_the_ask_only_to_who_may_inspect_it` (IFC-054, DEC-099; decision 4):
     in the project `admission_hold_names_only_what_the_caller_may_inspect` builds (you may move
     its making into a helper both tests use, that test's assertions unchanged), R4 kept as K:
     `observer`'s call on `b` is dispatched warned, its form's `reportInvocation` null and its
     text with "the login's ask" and no ID of K; root's A has `reportInvocation` K. Today
     neither is warned.
   - `admission_warned_call_is_sent_as_without_the_guard` (DEC-120, PND-016; decision 5): A's
     call dispatched on R4, on R3 and on R1 in fresh report states, and in a state whose login's
     Type states no `admission` (`guarded()` with `admission` taken from both Types, or
     `requests-admission-not-checked`'s documents; report which): each dispatch intent, the
     invocation IDs replaced by one placeholder, equals the others; in the discovered project
     the discovery intents and the call intents do likewise. No intent holds the key or the name
     of a lever. This passes today; it guards decision 5.
   - `admission_control_open_refused_for_an_agent_consumer` (CTR-015, ACC-056, ACC-006, DEC-119;
     decisions 6 and 7): in a project of yours made from `control_project()`, with Conduits to
     `login` from `#tuning` whose consumers are Agent `a` (`agent-tune`) and a Resource of yours
     (`resource-tune`), and one from `a` listing an operation of yours that the login's Type
     offers beside its control (`agent-other`): root's `access.open.v1` of `agent-tune` is
     `isError` true, reason refused, with one diagnostic, `ruleId` CTR-015 and refusal tier 2,
     whose text contains `agent-tune`, `example.org/conformance/shared-login#set-admission.v1`,
     `example.org/conformance/shared-login#login-holder.v1`, "Nothing was opened" and
     "Interface"; no intent; its `access-decision` record has `dispatched` false, reason refused
     and `ruleId` CTR-015; and root's `management.inspect.v1` of `agent-tune` shows it
     `registered`, as before the open. Root's opens of `tune-login`, `resource-tune` and
     `agent-other` succeed, and so does root's open of a Conduit of yours from `#turns` whose
     consumer is Agent `a` and whose target is Agent `b` (`agent-b`), whose Type states only
     `admission.loginTypes`, as on the unchanged code. As a setup's part (CON-004): in a project
     of yours where Agent `a` declares a Conduit to `login` listing the control, not optional,
     root's `set-up.v1` of `a` (through a Conduit of yours where the project has none) gives that
     part reason refused under CTR-015, the setup does not succeed (OPS-022), no run is asked
     (TPL-012), and that Conduit stays `registered`. Today `agent-tune` opens.
   - An existing test changes only where it asserts the exact reply, record or intent of a call
     this order now warns, or where test 4 moves its project's making into a helper, the
     assertions unchanged. None else is expected; report each before you change it.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 098 099 119 120 124; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1
k spec/interface/implementation.yaml 'that PND-016 decides on a report whose `reason` is `warning` or `cannot-tell` is warned:' "A repeat that OPS-009 answers with a warned attempt's result carries that warning too." "it reaches a calling Agent's implementation in the result's \`_meta\` alone."  # 1 1 1
k spec/promises/contracts.yaml 'Type names as `admission.control`, compared as qualified references, is not opened:' 'so the Conduit can be replaced or registered again without the admission control and then opened;'  # 1 1
grep -cF "| CTR-015 | No Conduit whose consumer is an Agent may be opened listing a login's admission control; an Interface's client tunes the levers. |" spec/conventions/access.yaml  # 1
awk '/^  name: record.v1$/,/^---$/' spec/core/results.yaml | grep -c 'admissionWarning: {\$ref: github.com/shrav-k/conduit/core#diagnostic.v1}'  # 1
awk '/^  name: admission-hold.v1$/,/^---$/' spec/core/results.yaml | grep -cE '^            reason: \{enum: \[warning, cannot-tell\]\}$'  # 1
grep -cE '"id": "(requests-admission-warning|access-admission-control-open|access-ask-admission-refused|requests-admission-single-use)"' conformance/expected-failures.yaml  # 4
k conformance/expected-failures.yaml "waits and holds, but carries no warning; the guard's engine order for the warnings makes it pass" "the guard's engine order for the warnings and the open makes it pass" "DEC-064's engine orders and the guard's engine order for the warnings and the open make it pass"  # 1 1 1
grep -cE '^\| (PND-016 \| convention \| must \| 7 \| 2|IFC-054 \| interface \| must \| 7 \| 2|CTR-015 \| language \| must \| 3 \| 1|ACC-056 \| convention \| must \| 9 \| 3) \|' engine/STANDING.md  # 4
grep -rlE 'admissionWarning|Verdict::reason\("refused", "CTR-015"\)' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '  # 0
k engine/crates/core/src/admission_ask.rs "/// One builder for the hold's form; the warning order adds its dispatched case here." 'pub(crate) fn hold_form(' "pub(crate) enum HoldCause<'a> {" 'fn decide_report(' '"dispatched":false,' 'requests::dispatch_waiting(t, a, c, i)?;' '.ok_or("admission control absent")?;'  # 1 1 1 1 1 1 1
k engine/crates/core/src/requests.rs 'pub(crate) fn finish(' 't.reply = ended_reply(c, old, &contract, outcome);' 'pub(crate) fn dispatch_waiting(' 'fn complete_discovery(' 'if crate::admission_ask::wait(t, a, c, i, &route)? {' 't.validated_reason = Some((reason.into(), restriction.clone()));' 'pub(crate) fn record_fault_reason(' 'fn restore_or_remove_invocation(' '"drop-hook-of-ended-parent",' 'invoke_inner(t, a, request, key, c, parent, None)'  # 1 1 1 1 1 1 1 1 1 1
k engine/crates/core/src/access.rs 'return Ok(Verdict::reason("refused", "SEC-008"));' 'return Ok(Verdict::reason("refused", "ACC-032"));' 'let opening = op == context.qualified("open.v1");'  # 1 2 1
k engine/crates/core/src/explain.rs 'pub(crate) fn admission_hold(tool: &str, form: &Value) -> String {' 'pub const REFUSAL_CAUSES: &[(&str, &str)] = &[' 'route["rootLoginNotice"] == true'  # 1 1 1
k engine/crates/core/src/admission_tests.rs 'fn single_use_report() -> Value {' 'fn control_project() -> packages::Project {' 'fn keyed_admission_call(&mut self, time: &str) -> Transition {' 'fn admission_hold_names_only_what_the_caller_may_inspect() {' 'fn admission_hold_restores_keyed_attempt_without_wait_fields() {'  # 1 1 1 1 1
k engine/CONFORMANCE.md "The engine warns of no call yet: the warnings are a later order's (DEC-099), and orders 319 and 320 write the ask and the hold (below)." 'dispatched with no warning yet: the warnings are a later order'"'"'s (DEC-099).' 'From order 321,' "## A login's admission warning and its control (order 321)"  # 1 1 0 0
grep -rlE 'admission-hook|loginTypes' packages | wc -l | tr -d ' '  # 0
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it. A second printed line other
than stated means a decision moved; a third to seventh, that order 318's spec is not in the
tree; an eighth to tenth, that the guarded cases' entries, their reasons or the standing rows
moved; an eleventh other than `0`, that an engine source already names the warning or the
refusal; a twelfth to seventeenth, that the code or the rows moved since this order was
written; and an eighteenth other than `0`, that a package states `admission`, and the
Preservation tests' "In use" sentence no longer holds.

The first line, and the Done when's line that counts files outside Scope, count the tree as it is
dispatched, with this order committed to its base. On a review copy where this order's file is
still untracked, each prints 1 more, for that file alone, and other uncommitted files there add
theirs. The coordinator commits the final order to the dispatch base and reruns both lines there
before dispatch; a worker who sees anything but 0 stops as above.

## Scope

Change, for decisions 1 to 7 and their tests, in `engine/crates/core/src/`: `admission_ask.rs`
(the warning's decision and form), `admission.rs` only for the control's resolution if it lives
there, `requests.rs` (the entry's warning, the three dispatch records, `finish` and the repeat),
`explain.rs` (the warning's words, the CTR-015 row and cause), `access.rs` (the open's check),
`admission_tests.rs` (the six tests), and `lib.rs` only for a `State` member, with
`#[serde(default, skip_serializing_if = …)]` so that a state with none is written byte for byte
as today (DL2), where your build keeps one (none is expected); the three entries of
`conformance/expected-failures.yaml`; `engine/CONFORMANCE.md`'s two sentences and new section;
and `engine/STANDING.md` by the tool alone. Change an existing test only under decision 9. Change
nothing else: not `spec/`, a case, `docs/` (DEC-098, DEC-099, DEC-119 and DEC-120 stay not done
here; the coordinator marks each at the merge that ends it), the shell, the loader, the
conformance crate, `packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`. No promise changes, so
`spec/CHANGES.md` gains no line. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `j` and `k` as in the Start check;
the second, guards, passes today and after, decision 5's test from when it is written. Run each
existing test you changed by name, once, too. Run the core's tests and then the whole vectors
test, once each, last; never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in admission_warning_on_a_report_that_warns admission_warning_on_a_single_use_report admission_warning_on_every_end_and_repeat admission_warning_names_the_ask_only_to_who_may_inspect_it admission_control_open_refused_for_an_agent_consumer; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 five times; 0 today
for c in requests-admission-warning access-admission-control-open; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. $c 2>&1 | grep -c "^PASS $c "); done  # 1 twice; 0 twice today
for p in requests-admission- access-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. $p 2>&1 | tail -1); done  # "6 cases passed; 1 expected failures", "22 cases passed; 4 expected failures"; "5 cases passed; 2 expected failures", "21 cases passed; 5 expected failures" today
grep -cE '"id": "(requests-admission-warning|access-admission-control-open)"' conformance/expected-failures.yaml  # 0; 2 today
k conformance/expected-failures.yaml "DEC-064's engine orders make it pass, with ACC-058's rows for the admission hook and a login's admission control" "the guard's engine order for the warnings"  # 1 0; 0 3 today
k engine/CONFORMANCE.md 'From order 321,' "## A login's admission warning and its control (order 321)" 'Orders 319, 320 and 321 write the ask, the hold and the warnings (below).' 'The engine warns of no call yet' 'dispatched with no warning yet'  # 3 1 1 0 0; 0 0 0 1 1 today
grep -cE '^\| (PND-016 \| convention|IFC-054 \| interface) \| must \| 7 \| 1 \| outside \| core \| case passes \|$' engine/STANDING.md  # 2; 0 today
grep -cE '^\| (CTR-015 \| language \| must \| 3 \| 0|ACC-056 \| convention \| must \| 9 \| 2) \| outside \| core \| case passes \|$' engine/STANDING.md  # 2; 0 today
grep -lE 'admissionWarning' engine/crates/core/src/*.rs | grep -vc '_tests\.rs$'  # more than 0; 0 today, the tests apart
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-core admission_held_ 2  # 1: order 320's holds
s conduit-core admission_hold_ 4  # 1
s conduit-core admission_single_use_ 2  # 1
s conduit-core admission_status_shows_ 2  # 1
s conduit-core login_resource_ 10  # 1
s conduit-core admission_warned_call_is_sent_as_without_the_guard 1  # 1: decision 5, on the unchanged code too
s conduit-conformance refusal_causes_match_spec_table_verbatim 1  # 1: the CTR-015 row is ACC-056's, word for word
for t in admission_reports_read_from_an_earlier_engines_store admission_asks_kept_across_a_reopen admission_ask_ends_at_its_deadline; do s conduit-shell "$t" 1; done  # 1 three times
s conduit-shell memory_equals_disk_after 5  # 1
for c in requests-admission-single-use access-ask-admission-refused access-ask-refused; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. $c 2>&1 | grep -A1 "^EXPECTED FAILURE $c " | sed -n 2p | cut -c1-10); done  # the same three lines before and after, the first "  step 35:"
for p in contracts-admission-shape registration-admission-checks root-consumer- secret-reads- requests-sign-in- requests-cancellation requests-restart-interrupted requests-key-scope requests-repeat-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", N in order: 1 1 5 11 22 1 1 1; then "1 cases passed; 2 expected failures"; where one prints otherwise before your change, report it and compare after with before
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(admission_ask|admission|admission_tests|requests|explain|access|lib)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/expected-failures\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Your report gives, per decision, what changed and where; each new test's line on the unchanged
code, then its passing line; each case's line before and after; the line by which a setup's
part reaches the open's check; each existing test you changed, and why; your own choices; and
every check's last line. Stage nothing.

**In two halves.** Where the order is dispatched as 321a and 321b, each half changes only the
files of its own decisions within Scope and runs this Start check and Done when with these
differences. 321b needs nothing of 321a and may run on 320's build beside it; the later to merge
rebases.

- 321a: decisions 1 to 5; of decision 8, `requests-admission-warning`'s entry and the
  CONFORMANCE section's first paragraph and two sentences; of decision 9, the first five tests.
  In its Done when, the first loop runs four tests, the case lines `requests-admission-warning`
  alone, the prefix line gives `6 cases passed; 1 expected failures` and `21 cases passed; 5
  expected failures`, the entries line `1`, the reasons line `0 2`, the CONFORMANCE line
  `2 1 1 0 0`, and of the STANDING lines only the first changes; the second prints `0`.
- 321b: decisions 6 and 7; of decision 8, `access-admission-control-open`'s entry, the new
  reason and the section's second paragraph (with its heading, where 321a has not merged); of
  decision 9, the sixth test. In its Done when, the first loop runs that test alone and the
  guards' line for decision 5's test is not run, the case lines `access-admission-control-open`
  alone, the prefix line gives `5 cases passed; 2 expected failures` and `22 cases passed; 4
  expected failures`, the entries line `1`, the reasons line `1 1`, the CONFORMANCE line
  `1 1 0 1 1`, and of the STANDING lines only the second changes; the first prints `0`, and
  `admissionWarning` stays at `0`.

Where one half has merged before the other runs, its lines are that half's after values; report
them as they print and go on.

## What follows

- **The login packages' new Types** (DEC-124, DEC-094), the design's P1 and P3: `codex-login.v2`
  in `packages/agent-codex` and `account-home.v2` in `packages/agent-claude-code`, each with the
  three fixed levers and the pair rule as `if` and `then`, `windows` and `staleAfterMinutes` (and
  `usageRead` for the second), its `admission-hook.v1` and `set-admission.v1`, and README tables
  for its field map, windows and decision order. A Claude login's `whenUnknown` `one-trial` then
  reaches its caller as this order's warning (DEC-098). The Agent Types listing them, `codex.v5`
  and `claude-code.v6`, follow.
- **The CLI's view** (C1): `conduit inspect` tables a login's admission, and a held or warned
  call prints its `admission`.
- **ACC-058's two rows** come with DEC-064's engine orders, which build `request-access.v1`;
  `access-ask-admission-refused` passes then.
- **Still open, from order 320: the single-use case needs the levers stored.**
  `requests-admission-single-use` stays an expected failure, first failing at step 35, until
  REV-007's storing at the levers place is built; the coordinator orders it or asks the owner.
- **Still open, from order 318: an Agent may reach the control through another instance.** A
  Resource whose Conduit to a login lists the control, called by an Agent, can send it on an
  onward request. Refusing it goes beyond DEC-119's words, so the coordinator asks the owner.
- DEC-094, DEC-098, DEC-099, DEC-119, DEC-120 and DEC-124 stay not done until the guard's orders
  and the sitting are done.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on the usage guard's stack (orders 307
and 314 to 320) at its start. Drafted and checked by the coordinator's Workflow; built and read on
the team seats (DEC-179, DEC-180).

**Worker-4's turns**, on the Business seat, built 321a and 321b. 321a first stopped on
`tools/set_names.py`'s eleven errors; they were the stack's own (order 318's new `_meta` key read
CTR-015's example address), the coordinator ruled them out of scope and fixed them at order 318's
merge.

**Two blind reads.** The Codex verifier, `root-verifier-4.turn.v1`, passed both halves with no
finding. The Claude reader, `root-reader-a.turn.v1`, on the Team seat, approved it with nits only.
One fix turn by worker-4 answered them. The gate's run on the squash is the coordinator's, at the
merge.
