# Work order 316: a login's admission hold before each call that starts model work (DEC-095, DEC-099, DEC-120, DEC-123, the usage guard's third spec order)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the five cases this order adds, all expected failures, and the one case it changes. It
needs no network and changes no Rust source; if a crate is missing from the cache, fetch nothing
and report it. Run one cargo command at a time, and not `tools/gate.py`. Never print a whole
large file, case, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Run no `conduit` outside a test, no `codex` or `claude`,
and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write
is public: no absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.**
[DEC-095](../decisions/log.md#dec-095-2026-11-23-a-guarded-hold-reports-unavailable-at-the-login-hold),
"Accept (Recommended)": the engine holds each call that starts model work on an Agent of a
guarded login before dispatch, at PND-005's login hold, reporting unavailable and never queued.
[DEC-099](../decisions/log.md#dec-099-2026-11-23-a-guarded-hold-tells-the-caller-everything-but-amounts),
"Accept (Recommended)": the hold names the login, the lever and its value, the window, its used
percentage, the threshold, the reset, how it lifts and which report, in text and a structured
field.
[DEC-120](../decisions/log.md#dec-120-2026-11-23-the-guard-checks-before-each-turn-and-a-started-turn-runs-to-its-end),
"I don't want to modify how the models operate just police their usage", then "Yes, check before
turns only (Recommended)": the engine checks the login's report before every call that starts
model work, and never stops a dispatched one.
[DEC-123](../decisions/log.md#dec-123-2026-11-23-a-call-waits-in-the-request-for-a-fresh-usage-report),
"Wait in the request (Recommended)": with no current report, the call waits for the login's one
ask, bounded by an implementation-defined limit, as an admission webhook does.
[DEC-124](../decisions/log.md#dec-124-2026-11-23-the-levers-come-on-new-type-versions),
"New Type versions (Recommended)": logins on the old Types have no check, and their status says
so. DEC-094 (the engine trusts the package's report) and DEC-098 (one trial, a single-use
report) stand behind it. Order 314 wrote what a login reports and how the engine keeps it; this
order writes the hold that acts on it: PND-016, `admission-hold.v1`, DL21, PND-005's clause and
the login's status, and ends the sentences of IFC-054 and CTR-015 that say no call is held yet.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 1, the
hold decides on what the login's package reported, never on an Agent's reasoning, and nothing of
it reaches a turn; 2, the ask grants nothing, goes through no Conduit, a caller with no access
starts none, and a call whose Conduit closes while it waits is refused when its wait ends; 6, the
ask starts no model work, a held call is never queued or resumed, and a waiting call is
dispatched at most once; 7, each hold names the login, its report and its ask, and the ask's
records name its caller; at risk, 4, no vendor assumed: nothing under `spec/` names a vendor
(DEC-105), and the cases' window IDs and methods are neutral.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a Kubernetes API request waits for its validating admission webhooks, each
bounded by `timeoutSeconds` (default 10, at most 30), and under `failurePolicy: Fail` an error or
a timeout rejects it; a cache lets one request fill a missing entry while the others wait on it
(nginx's `proxy_cache_lock`); a resolver may keep a server failure for a while so that retries
make no new queries (RFC 2308, section 7); a refusal says when to try again (`Retry-After`,
RFC 9110). Not taken: queueing or resuming a held call (DEC-095); stopping or cancelling a
dispatched call, or any lever, key or field of the guard reaching a turn (DEC-120); falling back
on an older report; a minimum validity; the ask's records naming their cause, the request whose
wait started it, for which `record.v1` has no field and whose description order 305 changes,
while the ask's caller names who asked and each hold and the status name the ask (preservation
test 7); the warning key and the refusal of an Agent's Conduit listing the control, which the
next spec order writes.

**When it runs.** It stacks on orders 307 and 314 and runs on 314's build (`main` with the
decision log through DEC-157, and 307's and 314's builds on top) until they merge, then on
`main`. Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at
once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what
must hold; how is yours. The words, the cases' names and titles and these choices are the
coordinator's, open to the owner's strike (DEC-029): `admission-hold.v1`'s fields and words,
with `dispatched` defined now for the warnings' order; the hold's field in the diagnostic's
`properties.admission`; `admission-status.v1` as a schema of its own, its words and `checked`
false for "not checked"; the status written into IFC-054 by widening its `when`; DL21 counted
once per ask from its start, the ask ending unknown at its end or at DL10's time for a call,
whichever comes first; DEC-123's amendment of DL3 and DL4 read as the first build's rows for
them in `engine/CONFORMANCE.md`, which the ask's engine order writes, the spec's DL3 gaining
only the ask's exception and its DL4 unchanged; the 60 seconds fixed in PND-016 and applying
only after an ask that gave no report; a single-use report given to the earliest request still
waiting on its ask (rather than to the request that started the ask, which a cancel may have
ended), and with none waiting to the next request decided while it is current; a report that
IFC-054 keeps already ended, the levers having changed while its ask ran, deciding no waiting
request and holding each with `noReport` `levers-changed`, `lifts` `at-time` and `retryAt` the
ask's end, with no 60 seconds after it; a request cancelled while it waits reporting
unavailable under PND-016, not failed, since OPS-005 gives a request that is not dispatched no
outcome; that diagnostic and a waiting repeat's carrying no `properties.admission`; at a
restart, REC-007's unavailable for a waiting request and IMP-007's unknown for its ask; a
repeat of a waiting request reporting conflict; when a wait ends, the access decision made anew
and recorded (ACC-036, REC-001), then PND-005's and PND-011's holds, unlike a Kubernetes
request's authorization, which stands while its webhooks run, since IMP-001 and ACC-039 let
model work start only through Conduits open when it is dispatched; the ask's caller the caller
of the request whose wait started it, its records holding no `serving` and naming no cause; a
login's invocations null in an Agent's status for an inspector who may not inspect the login;
and the five cases, the report case's two asserted callers, the single-use case's closed
Conduit, PND-016 among the report case's promises and IFC-054 among the wait case's.

**Shared files.** Order 299 takes PND-015 and DL20, which this order leaves free, and changes
`spec/promises/pending.yaml`, `spec/implementation-defined.yaml`, `spec/README.md`,
`spec/definitions.md` and `spec/CHANGES.md`; order 309 takes PND-017 and DL22 in the same files
and in the core's; order 305 changes REC-002, `record.v1`, `spec/core/results.yaml` and the
definitions; order 304 changes CTR-013's observable in `spec/promises/contracts.yaml`, where this
order changes CTR-015's; orders 303, 304 and 308 change some of the same files; and order 315,
on 307, removes the `requests-limits-signals` entry of `conformance/expected-failures.yaml` and
regenerates `engine/STANDING.md`. Whichever merges later rebases, recounts and reruns
`promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. In the YAML, indentation and flow style are the file's,
and a `#` line stands for what it names. No text under `spec/` names a vendor or client product.
"A" → "B" replaces A wherever it wraps; "A" +"B" adds B after A.

1. **PND-016** (`spec/promises/pending.yaml`, after PND-005; convention, must). When: "The engine
   would dispatch a request, or a new attempt of one (OPS-012, OPS-024), for an operation whose
   contract is marked `startsModelWork: true` to an Agent whose login place refers to a Resource
   whose Type states `admission` (CTR-015), its login: OPS-023's reasons and the holds of
   PND-005 and PND-011 have been decided, and none applies." Then: "Before dispatch the request
   is decided on the login's admission report (IFC-054). Where one is current, the request is
   dispatched if the report's `admit` is true and held if it is false, and nothing is asked.
   Where none is current, the request waits, not dispatched, for the login's ask: the engine's
   call of `admission-hook.v1` on the login, at most one in progress per login, which the engine
   starts where none is and which every request so waiting on that login shares. When the ask
   ends, as it does at the latest when DL21 ends, each request that waited on it is decided
   again before the report: on its access, decided anew (ACC-036), then on the holds of PND-005
   and PND-011; where none of these applies, it is decided on the report the ask gave as on a
   current one, except that a single-use report admits one request alone, the earliest still
   waiting on its ask, and holds every other. An ask that gives no report, or whose report
   IFC-054 keeps already ended, holds every request that waited on it, with no older report
   used; after one that gives no report, until 60 seconds after its end on the engine's own
   clock, no ask starts on the login and every request this promise decides there is held
   without waiting. A held request is not dispatched and reports unavailable, with an error
   diagnostic whose `ruleId` is PND-016 and whose `properties.admission` has the form of the
   core schema `admission-hold.v1`; nothing is queued, resumed or dispatched after it ends."
   Observable, in your words and citing no decision:
   - every such request is checked, whatever its operation and whoever sends it, an Agent's
     onward request among them, and a caller that may not make it starts no ask, since OPS-023's
     reasons come first;
   - the ask is no request and no part: the engine dispatches it as its own invocation of the
     login, through no Conduit and with no filter, for the caller of the request whose wait
     started it; its records name that caller, hold no `serving`, since the ask serves no
     dispatched invocation, and name no cause; its input is the one `admission-hook.v1`
     describes; it starts no model work, and no inspection starts one (OBS-006);
   - at DL21's end the engine stops waiting for the ask, which ends unknown as OPS-007 has a call
     that gives no result, and a later answer gives no report (OPS-013); an ask gives no report
     where it fails, ends unknown (an answer that fails `admission-report.v1` among them), passes
     DL21 or cannot be dispatched;
   - when its wait ends, a request's access decision is made anew from the Conduits as they
     stand then, as ACC-036 has every decision made, and recorded as any is (REC-001): it is
     dispatched only where each Conduit of its chain is open and lists the operation, none has a
     retired consumer (ACC-039), and the invocation an onward request serves is still in
     progress (ACC-019, IFC-028), so that model work starts only through an open Conduit
     (IMP-001); otherwise it is not dispatched and reports refused, carrying ACC-056's diagnostic
     alone, or unavailable under ACC-037 where that decision cannot be made, as OPS-023 puts
     them first; then a hold of PND-005 or PND-011 that began meanwhile holds it under that
     promise;
   - a report that IFC-054 keeps already ended, as the levers changed while its ask ran, decides
     no request: each that waited on the ask is held with `noReport` `levers-changed`, and no 60
     seconds follow, so a request sent again starts an ask under the new levers;
   - a single-use report on whose ask no request waits admits the next request this promise
     decides while it is current (IFC-054);
   - while a request waits, the engine's ID for it is the one its invocation takes if it is
     dispatched, and a cancel names that ID; a request cancelled while it waits, by the means
     OPS-027 names, is not dispatched and reports unavailable under this promise, its diagnostic
     saying that it was cancelled while it waited and carrying no `properties.admission`, and its
     record says so where the protocol has the engine send no response; the ask goes on for the
     others; at a restart, a request that waits ends unavailable under REC-007, as any request
     decided and not dispatched does, and an ask in progress ends unknown under IMP-007 and gives
     no report; a request that matches a waiting one, as OPS-009 describes a match, is not
     dispatched and reports conflict, its diagnostic naming this promise, saying that the earlier
     request waits for its login's report and carrying no `properties.admission`;
   - the diagnostic's text names the login by kind, name and ID; says that its admission report
     says model work may not start on it now, or that it has no report that admits this call;
     for a report, says why in its reason's words, naming the lever and its value (for
     `threshold`, the threshold), the window with its used percentage, its time and its reset,
     and the paid usage read, each "not reported" where null; for no report, the ask and why it
     gave none, or that the levers changed while it ran; for a single-use report, the call it
     admitted; then how the hold lifts, by `lifts`, with `retryAt`; and, unless `lifts` is
     `lever-change`, that a change of the levers by the Type's control, sent by an Interface,
     also ends the report; `admission-hold.v1` gives the same in its fields, and neither names
     an amount, a balance or a currency;
   - who is named: an Agent's call only where the caller may inspect that Agent, otherwise as a
     call of an Agent of the login; the ask only where the caller may inspect the login; the
     login always, as PND-005 names it;
   - `lifts` and `retryAt`: a refusal's `lifts` is its report's, and `retryAt` its keep time
     plus its `validForSeconds`, null for `lever-change`; after an ask that gave no report,
     `at-time` and that ask's end plus 60 seconds; after one whose report was kept already
     ended, `at-time` and that ask's end; for a request held because a single-use report
     admitted another, `after-call` and the ask's end plus 60 seconds, the latest time to retry,
     since the admitted call may end sooner; a request sent again is decided afresh, and nothing
     lifts a hold by itself;
   - a dispatched request runs to its end: the engine never stops or cancels it under this
     promise, whatever a later report says; it is sent exactly as with no `admission` stated, so
     nothing of this promise reaches the Agent's implementation; the requests one report admits
     run in parallel, as PND-005 lets calls on one login;
   - a login whose Type states no `admission` is not checked: its Agents' requests are
     dispatched as without this promise, and its status says so (IFC-054).
2. **`admission-hold.v1`** (`spec/core/results.yaml`, after `admission-report.v1`; in the header,
   `admission-hold.v1 PND-016 (why a call that starts model work on a guarded login was held)`):

   ```yaml
   spec:
     type: object
     properties:
       login:   # object.v1 by $ref, its kind const: Resource
       dispatched: {type: boolean}
       report:  # admission-report.v1 or null, by anyOf
       reportInvocation:  # id.v1 or null, by anyOf
       noReport: {enum: [failed, unknown, deadline, not-dispatched, levers-changed, null]}
       singleUse:  # id.v1 or null, by anyOf
       lifts: {enum: [at-time, newer-report, lever-change, after-call, null]}
       retryAt: {type: [string, 'null'], format: date-time}
       levers:  # object: type and control (qualified-reference.v1) and pointer (pointer.v1), all required, closed
     required: [login, dispatched, report, reportInvocation, noReport, singleUse, lifts, retryAt,
                levers]
     additionalProperties: false
     allOf:  # six if/then rules:
     # dispatched true: report an object whose admit is true, and noReport, singleUse, lifts and retryAt null; else lifts a string
     # noReport a string: report and singleUse null, and lifts at-time
     # lifts after-call: report and noReport null
     # singleUse a string: lifts after-call
     # lifts lever-change: retryAt null; lifts at-time, newer-report or after-call: retryAt a string
     # dispatched false, noReport null and lifts other than after-call: report an object whose admit is false
   ```

   Its description: "Why a call that starts model work on an Agent of a guarded login was held,
   or, with `dispatched` true, the report that admitted such a call with `warning` or
   `cannot-tell` (PND-016). `login` names the login. `report` is the report that decided, null
   where no report of the ask decided and where a single-use report admitted another call.
   `reportInvocation` is the ask that gave the report, or that gave none, null where the caller
   may not inspect the login or no ask was dispatched. `noReport` says why no report of the ask
   decided: it `failed`, ended `unknown`, passed DL21 (`deadline`) or was `not-dispatched`, or
   the report it gave was kept already ended as the levers changed while it ran
   (`levers-changed`). `singleUse` is the call a single-use report admitted, null where the
   caller may not inspect its Agent. `lifts` says how the hold ends: as the report's, `at-time`
   where no report of the ask decided, or `after-call` once the call a single-use report admitted
   ends; it is null where the call was dispatched. `retryAt` is when the hold's cause lapses of
   itself: as the report's validity ends, as 60 seconds pass after an ask that gave no report,
   or at the ask's end for `levers-changed`; for `after-call` it is the latest time to retry,
   the ask's end plus 60 seconds, since the admitted call may end sooner. It is null for
   `lever-change` and where the call was dispatched. `levers` gives the login's Type, its levers
   place and its control (CTR-015). It holds no amount."
3. **`admission-status.v1`** (`spec/core/results.yaml`, after `admission-hold.v1`; in the header,
   `admission-status.v1 IFC-054 (a login's admission state in a status, or that its calls are not checked)`):

   ```yaml
   spec:
     type: object
     properties:
       checked: {type: boolean}
       report:  # admission-report.v1 or null, by anyOf
       invocation:  # id.v1 or null, by anyOf
       keptAt: {type: [string, 'null'], format: date-time}
       validUntil: {type: [string, 'null'], format: date-time}
       state: {enum: [current, ended, none]}
       ended: {enum: [valid-for-ended, single-use, newer-limits, levers-changed, null]}
       asking:  # id.v1 or null, by anyOf
       noReport:  # [object, 'null']: invocation (id.v1 or null), cause ([failed, unknown, deadline, not-dispatched]) and until (date-time), all required, closed
     required: [checked]
     additionalProperties: false
     if:
       properties:
         checked: {const: true}
     then:
       required: [report, invocation, keptAt, validUntil, state, ended, asking, noReport]
       allOf:  # three if/then rules:
       # state none: report, invocation, keptAt, validUntil and ended null
       # state current or ended: report an object and keptAt a string
       # state ended: ended a string; else ended null
     else:
       maxProperties: 1
   ```

   Its description: "A login's admission state, as the status of a Resource whose Type states
   `signIn` shows it, and as that of an Agent whose login place refers to one shows its login's
   (IFC-054). `checked` false says that the login's Type states no `admission`, so calls on it
   are not checked, and nothing else is given. Otherwise `report` is the newest report the engine
   kept, spent or ended ones included, or null where it kept none; `invocation` the call that
   gave it; `keptAt` its keep time; `validUntil` the end of its validity, null for a single-use
   report; `state` whether it is `current` or `ended`, or `none` where no report was kept;
   `ended` which end ended it: `valid-for-ended`, `single-use`, `newer-limits` or
   `levers-changed`; `asking` the ask in progress (PND-016); and `noReport`, while PND-016 holds
   every call after an ask that gave no report, that ask, null where it was not dispatched, why
   it gave none and when that hold ends. In an Agent's status each invocation is null where the
   inspecting actor may not inspect the login. It holds no amount." Export both new schemas in
   `spec/core/conduit.yaml` after `admission-report.v1`.
4. **IFC-054** (`spec/interface/implementation.yaml`). Its `when` becomes: "The engine's call of
   `admission-hook.v1` on a login Resource whose Type states `admission` (CTR-015) ends; or a
   Resource whose Type states `signIn` (CTR-013), or an Agent whose login place refers to one, is
   inspected." In its `then`, "Where it succeeded, the record of its end holds" → "Where the
   call succeeded, the record of its end holds", and "or a control stores a value at or under
   the Type's levers place (REV-007)." +" A report given by a call that was in progress when such
   a value was stored is kept already ended, and decides no call (PND-016). The status of a
   Resource whose Type states `signIn`, and of an Agent whose login place refers to one, holds
   `admission`, in the form of the core schema `admission-status.v1`, the Agent's being its
   login's. Where the login's Type states `admission`, it gives `checked` true; the newest
   report the engine kept, with the call that gave it, its keep time and the end of its
   validity; whether it is current, or which of the four ends above ended it; the ask in
   progress (PND-016); and, while PND-016 holds every call after an ask that gave no report,
   that ask, why it gave none and when that hold ends. Where the Type states no `admission`, it
   gives `checked` false alone." In its `observable`, delete its first two sentences, "This
   version keeps the report and decides nothing from it: whether the engine asks for one before
   a call that starts model work, and holds or refuses that call, is not yet specified. A login's
   levers are declared and its reports kept, but no call is yet held or refused on them
   (CTR-015).", which PND-016 makes untrue; and "the hook only to decide a call that starts model
   work on an Agent of the login, and no inspection calls it (OBS-006)." → "the hook only to
   decide a call that starts model work on an Agent of the login, under PND-016, and no
   inspection calls it (OBS-006).", followed by, in your words: the status is the engine's own
   state, not an observation (OBS-002), and an inspection starts no call to fill it and changes
   nothing of it (OBS-006); in an Agent's status each invocation of its login is null where the
   inspecting actor may not inspect the login; a login on a Type that states no `admission`, as
   an earlier version of a Type whose later version states it, keeps working unchecked, and its
   status says so.
5. **DL21** (`spec/implementation-defined.yaml`, after DL19, or after DL20 where order 299 has
   merged). Subject: "A login's ask for an admission report". Document: "How long the ask that
   PND-016 starts on a login, the engine's call of `admission-hook.v1`, is waited for, counted on
   the engine's own clock from the time the engine starts it; no request waiting on it waits
   longer. The wait is finite: at its end an ask that has not ended ends unknown, a later answer
   gives no report, and every request that waited on it is held under PND-016. DL10's time
   allowed for a call bounds the ask as well: where it comes first, the ask ends unknown then, as
   at DL21's end. It is distinct from DL3's wait for an actor's reply and DL18's for a person's
   completion, and the 60 seconds after an ask that gave no report are PND-016's, not this
   item's. Vectors supply clock input rather than relying on elapsed wall time." In DL3's
   document, "No promise limits how long an invocation is in progress:" → "No promise limits how
   long an invocation is in progress, PND-016's ask apart, which DL21 bounds:". DL4 stays as it
   is: the rest of DEC-123's amendment of DL3 and DL4 is the first build's rows for them in
   `engine/CONFORMANCE.md`, which the ask's engine order writes.
6. **The sentences this needs.**
   - PND-005's observable: "Otherwise calls on one login run in parallel." → "Otherwise calls on
     one login run in parallel, subject to PND-016."
   - PND-011's observable: "never as a part and never on an implementation notification
     (IFC-054)." → "never as a part and never on an implementation notification (IFC-054), for
     the caller of the request whose wait started it (PND-016)."
   - CTR-015's observable (`spec/promises/contracts.yaml`): "In this version, a login's levers
     are declared and its reports kept, but no call is yet held or refused on them. The following
     lever meanings describe the login's report; acting on that report before model work is not
     yet specified (IFC-054)." → "The engine decides each call that starts model work on an Agent
     of the login on the login's report, under PND-016; the following lever meanings describe
     that report (IFC-054)."
   - `inspect.v1` (`spec/core/operations.yaml`): its status gains, after `limits`,
     `admission` (`$ref` `github.com/shrav-k/conduit/core#admission-status.v1`, in the file's
     block style); its description, "of each usage signal those calls reported." +" `admission`
     holds, for a Resource whose Type states `signIn` and for an Agent whose login place refers
     to one, its login's admission state in `admission-status.v1` form, or `checked` false where
     that login's Type states no `admission` (IFC-054)."
   - `diagnostic.v1`'s description, "every reason names its deciding rule (IFC-050)." +" A hold
     under PND-016 carries `properties.admission` in the form `admission-hold.v1`."; in the
     header of `results.yaml`, under `diagnostic.v1`'s entry, a line "PND-016 (a hold on a
     guarded login carries `properties.admission`)".
   - `spec/definitions.md`: in the `login Resource` row, "and reported by its
     `admission-hook.v1` (CTR-015, IFC-054)." → "and reported by its `admission-hook.v1`
     (CTR-015, IFC-054), and enforced before each call that starts model work on an Agent of it
     (PND-016)."; in the `caller` row, "it belongs to (TPL-017, TPL-018, PND-011), and the
     engine's read" → "it belongs to (TPL-017, TPL-018, PND-011), a login's `admission-hook.v1`
     for that of the request whose wait started it (PND-016), and the engine's read".
   - `spec/core/README.md`: the hooks' table row of `admission-hook.v1` ends "through no
     Conduit, for a call that waits under PND-016 | CTR-015, IFC-054, PND-016 |"; its "Every
     export" row reads "CTR-015, IFC-054, PND-016: answer whether model work may start on a
     login"; after `admission-report.v1`'s "Every export" row, a row for `admission-hold.v1`,
     kind Schema, "PND-016: why a call that starts model work on a guarded login was held", and
     one for `admission-status.v1`, kind Schema, "IFC-054: a login's admission state in a
     status, or that its calls are not checked"; the `results.yaml` row, "15 Schema contracts" →
     "17 Schema contracts" and "a login's admission report, an inventory" → "a login's admission
     report, why a call on a login was held, a login's admission state, an inventory"; and "158
     documents, 80 of them Schema contracts" → "160 documents, 82 of them Schema contracts".
   - `spec/README.md`: the `core/` row's "158 documents exported" → "160 documents exported";
     the `core/results.yaml` row, "`admission-report.v1`, `inventory.v1`" → "`admission-report.v1`,
     `admission-hold.v1`, `admission-status.v1`, `inventory.v1`", "a login's admission report, an
     inventory" → as in the core README, and "Draft; 15 contracts" → "Draft; 17 contracts"; the
     `promises/pending.yaml` row's 14 promises → 15; the `implementation-defined.yaml` row's 17
     items → 18; and the totals → "525 in all: 227 of the layer `language`, 247 of the layer
     `convention` and 51 of the layer `interface`", which `check_schemas.py` checks, reporting
     any other count a recount shows off.

   PND-005's and PND-011's account rows are `left to MC9`, with no digest: they stay. CTR-015 and
   IFC-054, of the layers `language` and `interface`, have no row.
7. **The account** (`experiments/012-model-checking/account.yaml`): PND-016's row after
   PND-005's, in the file's order, `class: uncatalogued` and nothing else, as ACC-058's is; the
   safety catalogue lists no PND-016.
8. **CHANGES.** In `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

   ```text
   - Added: PND-016, each call that starts model work on an Agent of a login whose Type states `admission` is decided on a current admission report or waits, bounded by DL21, for the login's one ask, its access decided again when the wait ends; a refusal, an ask that gives no report or whose report a change of the levers ended, and a single-use report given to another call hold it unavailable, never queued, and no ask starts for 60 seconds after one that gave no report (DEC-095, DEC-099, DEC-120, DEC-123).
   - Added: `admission-hold.v1`, the structured field of a PND-016 hold, and `admission-status.v1`, a login's admission state in its status and its Agents', or that calls on it are not checked (DEC-099, DEC-124).
   - Added: DL21, how long a call waits for its login's ask (DEC-123).
   - Changed: IFC-054 and CTR-015, the engine acts on the login's report under PND-016, a status shows the login's admission state, and a report from a call in progress when a control stored the levers is kept already ended (DEC-123, DEC-124).
   - Changed: PND-005, PND-011, DL3, `inspect.v1`, `diagnostic.v1` and the definitions' `login Resource` and `caller`, for PND-016 and the ask's caller (DEC-095, DEC-123).
   ```

9. **Five cases,** in `conformance/conventions/requests/`, each in that family's `index.yaml` in
   `id` order, profile `first-build`, from `{"recipe":"empty"}`, with neutral names and the titles
   given. This order names a case's steps by their index in its `steps`, counted from 0; the
   conformance runner counts them from 1, so it prints the step at index 15 as `step 16`. *The
   guarded project* is order 314's, in `conformance/language/contracts/admission-shape/`, shared
   by relative path as `requests-admission-report-kept` (the report case) shares it, and *the
   setup* is the report case's steps 0 to 14, copied with their captures: first start,
   registration, the setups of `a` and `b`, `call-a` and `call-b` opened, and `login` set up with
   its account check answering `example-account`. Times are 2026-11-21 UTC; T is 13:01:00.
   Derive the five cases' steps with one script of your own, run once and committed nowhere, and
   report what it did; commit only case files. Each case's ask is dispatched as the report case's
   steps 15 and 16 dispatch it: the hook's discovery, then its call, whose `arguments` hold
   `levers` as stored, `limits` the login's as inspection shows them and `previous` the newest
   report or null, and whose `_meta` holds `io.github.shrav-k.conduit/caller` root's ID,
   `00000000-0000-4000-8000-000000000005`, and no `io.github.shrav-k.conduit/conduit`. A held
   request's reply is `isError` true, `_meta` with `reason` unavailable and no `pendingAction`,
   and a first diagnostic with `ruleId` PND-016 and `properties.admission` the hold given; where
   a step ends a waiting request whose reply the runner does not give, assert its `request-ended`
   record (`reason` and `ruleId`, not dispatched) and report the form used. The reports, as
   `structuredContent.admission`:
   R1 is the report case's (`admit` true, `within-levers`, `validForSeconds` 3600); R2 is
   `{"admit":false,"reason":"paid-in-use","lever":{"pointer":"/admission/paidUsage","value":"refuse"},"window":null,"paidUsage":{"possible":true,"inUse":true,"resetsAt":"2026-11-21T15:00:00Z"},"cannotTell":null,"lifts":"newer-report","observedAt":"2026-11-21T13:00:30Z","source":{"client":null,"method":"example-read"},"validForSeconds":600,"singleUse":false}`;
   R3 is `{"admit":true,"reason":"cannot-tell","lever":{"pointer":"/admission/whenUnknown","value":"one-trial"},"window":null,"paidUsage":{"possible":null,"inUse":null,"resetsAt":null},"cannotTell":"never-observed","lifts":null,"observedAt":null,"source":{"client":null,"method":"example-read"},"validForSeconds":0,"singleUse":true}`.
   Every hold has `login` `{"kind":"Resource","name":"login","id":"$login"}`, `dispatched` false
   and `levers` `{"type":<login-holder.v1>,"pointer":"/admission","control":<set-admission.v1>}`,
   each `<…>` the qualified reference of the project's contract
   (`example.org/conformance/shared-login#…`) in the form the runner gives one; below, each
   hold gives only its other five fields. Every status has `checked` true unless stated.
   - `requests-admission-hold` (`admission-hold/`; layers `convention` and `interface`; PND-016
     and IFC-054; title "A call on a guarded login waits for its login's ask, is held on its
     refusal, and the kept refusal holds the next call with no ask"): the setup; (i) at T, a
     `turn.v1` on `a`: nothing reaches `a`, and the ask's discovery is dispatched to `login`;
     (ii) its call, `previous` null and `limits` `{"state":"unknown","windows":[]}`; (iii) at T
     it succeeds with R2: its end record holds R2 in `admission`, nothing reaches `a`, and `a`'s
     request is held with `report` R2, `reportInvocation` the ask, `noReport` and `singleUse`
     null, `lifts` `newer-report`, `retryAt` 13:11:00; (iv) at 13:02:00 a `turn.v1` on `b`: no
     intent at all, and it is held with the same hold; (v) at 13:02:00 `logins.inspect.v1` on
     `login`: its status's `admission` is `report` R2, `invocation` the ask, `keptAt` T,
     `validUntil` 13:11:00, `state` `current`, `ended`, `asking` and `noReport` null.
   - `requests-admission-no-report` (`admission-no-report/`; layers `convention` and
     `interface`; PND-016 and IFC-054; title "An ask that gives no report holds its calls, and no
     ask starts on the login for 60 seconds"): the setup; (i) and (ii) as above; (iii) at T the
     ask ends failed: its end record holds no `admission`, and `a` is held with `report` null,
     `reportInvocation` the ask, `noReport` `failed`, `singleUse` null, `lifts` `at-time`,
     `retryAt` 13:02:00; (iv) at 13:01:30 a `turn.v1` on `b`: no intent, held with the same
     hold; (v) at 13:01:30 `logins.inspect.v1` on `login`: `report`, `invocation`, `keptAt`,
     `validUntil`, `ended` and `asking` null, `state` `none`, `noReport`
     `{"invocation":<the ask>,"cause":"failed","until":"2026-11-21T13:02:00Z"}`; (vi) at
     13:02:01 a `turn.v1` on `a`: a new ask's discovery; (vii) its call, `previous` null; (viii)
     at 13:02:01 it succeeds with R1 but `validForSeconds` 0, which `admission-report.v1` rejects:
     it ends unknown, its end record holds no `admission`, and `a` is held with `report` null,
     `reportInvocation` the second ask, `noReport` `unknown`, `singleUse` null, `lifts`
     `at-time`, `retryAt` 13:03:01.
   - `requests-admission-single-use` (`admission-single-use/`; layers `convention` and
     `interface`; PND-016 and IFC-054; title "A single-use report admits only the earliest call
     waiting on its ask, the next call asks again, and a call whose Conduit closed while it
     waited is refused"): its own `valid-documents.json`, the guarded project's with
     `login-holder`'s `config` holding `whenUnknown: one-trial`, and the shared manifest and
     fixture by relative path; the setup on it, any expectation that shows that value changed
     with it; (i) as above; (ii) at T a `turn.v1` on `b`: no intent at all, its reply pending on
     the same ask; (iii) the ask's call, `levers` with `whenUnknown` `one-trial`, `previous`
     null; (iv) at T it succeeds with R3: `a`'s call is dispatched to `a`, nothing reaches `b`,
     and `b` is held with `report` null, `reportInvocation` the ask, `noReport` null,
     `singleUse` `a`'s invocation, `lifts` `after-call`, `retryAt` 13:02:00; (v) at 13:01:05
     `workers.inspect.v1` on `a`: its status's `admission` is its login's, `report` R3,
     `invocation` the ask, `keptAt` T, `validUntil` null, `state` `ended`, `ended` `single-use`,
     `asking` and `noReport` null; (vi) at 13:01:10 a `turn.v1` on `b`: a new ask's discovery,
     as no report is current; (vii) its call, `previous` R3; (viii) at 13:01:10 root's
     `access.close.v1` of `call-b` succeeds; (ix) at 13:01:10 the ask succeeds with R1: nothing
     reaches `b`, and `b`'s request is not dispatched and reports refused, its diagnostic's
     `ruleId` ACC-001 and no `properties.admission`.
   - `requests-admission-wait-ends` (`admission-wait-ends/`; layers `convention` and
     `interface`; PND-016 and IFC-054; title "A call cancelled while it waits is not dispatched
     while the ask goes on, and a restart ends a waiting call and its ask"): the setup; (i) as
     above; (ii) at T a `turn.v1` on `b`: no intent, its reply pending on the same ask; (iii) at
     T a `cancel` naming the engine's ID for `a`'s waiting request, captured from its pending
     reply as the report case's step 15 captures `callA`, with no `confirmation`: no intent, no
     cancellation reaches `login`, and `a`'s request ends unavailable under PND-016, not
     dispatched, with no `properties.admission` in its diagnostic (assert it absent); (iv) the
     ask's call, its caller still root's; (v) at T it succeeds with R1: `b`'s call is dispatched,
     nothing reaches `a`; (vi) at 13:02:00 `b`'s call succeeds with the report case's limits
     member (its step 20); (vii) at 13:03:00 a `turn.v1` on `a`: a new ask's discovery; (viii)
     its call, `previous` R1 and `limits` the login's status, as the report case's step 22;
     (ix) at 13:03:00 `logins.inspect.v1` on `login`: `report` R1, `invocation` the first ask,
     `keptAt` T, `validUntil` 14:01:00, `state` `ended`, `ended` `newer-limits`, `asking` the
     second ask, `noReport` null; (x) `restore-state`, its evidence at 13:03:30: `a`'s waiting
     request ends unavailable under REC-007, the ask ends unknown with no `admission` in its end
     record, and no intent; (xi) at 13:04:00 a `turn.v1` on `b`: no intent, held with `report`
     null, `reportInvocation` the second ask, `noReport` `unknown`, `singleUse` null, `lifts`
     `at-time`, `retryAt` 13:04:30.
   - `requests-admission-not-checked` (`admission-not-checked/`; layers `convention` and
     `interface`; IFC-054 and PND-016; title "A login whose Type states no admission shows that
     calls on it are not checked, and its Agents' calls are dispatched unchecked"):
     `requests-login-resource-one-at-a-time`'s files by relative path, as
     `requests-limits-signals` shares them, and its steps 0 to 9, copied; then at T (i)
     `logins.inspect.v1` on `login`: its status's `admission` is `{"checked":false}`; (ii)
     `workers.inspect.v1` on `a`: the same; (iii) a `turn.v1` on `a`: dispatched to `a` with no
     ask, as that case's step 10 is.

   List the five alone in `conformance/expected-failures.yaml`, in `id` order:
   `requests-admission-not-checked` with the reason "IFC-054, order 316: the first build's
   status holds no admission; the guard's engine order for keeping makes it pass", and the other
   four with "PND-016, order 316: the first build dispatches a call on a guarded login with no
   ask and holds none; the guard's engine orders for the ask and the hold make it pass". Each
   must first fail where the unchanged engine lacks what it tests: the four guarded cases at step
   15, the first `turn.v1`, which that engine dispatches to `a` with no ask, and
   `requests-admission-not-checked` at step 10, its first inspection, whose status holds no
   `admission`; the runner prints these as `step 16` and `step 11`. A case that first fails at
   any other step is wrong: fix the case.
10. **The report case** (`requests-admission-report-kept`): PND-016 joins IFC-054 in its
    `promises` and in its index entry, as it asserts PND-016's wait, and the `_meta` of its two
    ask calls, in steps 16 and 22, gains `"io.github.shrav-k.conduit/caller":
    "00000000-0000-4000-8000-000000000005"`. Change nothing else of it; it still first fails at
    step 15, which the runner prints as `step 16`.

Declare all of it in `docs/work-orders/316-declared.yaml`, in 307's form, the report case's
changes as `metadata` (`/promises/*`) and `expectation` (the two steps' `external_intents`), with
the paths `vector_delta.py` names for them; then run `python3 tools/promise_standing.py
--write`. No file under `spec/schemas/` changes: this order changes no document's shape.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
for d in 095 099 120 123 124; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1
cat spec/core/*.yaml spec/promises/*.yaml spec/conventions/*.yaml spec/interface/*.yaml spec/schemas/*.yaml spec/implementation-defined.yaml spec/*.md spec/core/README.md spec/schemas/README.md | grep -cE 'PND-016|DL21|admission-(hold|status)'  # 0
cat spec/promises/contracts.yaml spec/interface/implementation.yaml | grep -cE '^- id: (CTR-015|IFC-054)$'  # 2: order 314's build is the base
cat spec/promises/pending.yaml spec/implementation-defined.yaml | grep -cE '^- id: (PND-015|DL20)$'  # 0; 2 once order 299 has merged
k spec/promises/pending.yaml 'Otherwise calls on one login run in parallel.' 'never as a part and never on an implementation notification (IFC-054). Setup checks'  # 1 1
k spec/interface/implementation.yaml 'call of `admission-hook.v1` on a login Resource whose Type states `admission` (CTR-015) ends.' "or a control stores a value at or under the Type's levers place (REV-007)." 'hook only to decide a call that starts model work on an Agent of the login, and no inspection calls it (OBS-006).'  # 1 1 1
k spec/interface/implementation.yaml 'This version keeps the report and decides nothing from it: whether the engine asks for one before a call that starts model work, and holds or refuses that call, is not yet specified.' "A login's levers are declared and its reports kept, but no call is yet held or refused on them (CTR-015)." 'Where it succeeded, the record of its end holds'  # 1 1 1
k spec/promises/contracts.yaml "In this version, a login's levers are declared and its reports kept, but no call is yet held or refused on them." "The following lever meanings describe the login's report; acting on that report before model work is not yet specified (IFC-054)."  # 1 1
k spec/implementation-defined.yaml 'No promise limits how long an invocation is in progress: until it ends'  # 1
k spec/definitions.md 'and reported by its `admission-hook.v1` (CTR-015, IFC-054).' 'or sign-in.v1 invocation it belongs to (TPL-017, TPL-018, PND-011), and the engine'  # 1 1
k spec/core/operations.yaml 'where there is no report, in `limit-status.v1` form (IFC-053).'  # 1
k spec/core/results.yaml 'refusal.v1 (ACC-056); every reason names its deciding rule (IFC-050).' "# admission-report.v1 IFC-054 (a login's answer to whether model work may start on it now)"  # 1 1
cat spec/core/*.yaml spec/schemas/*.yaml | grep -E '^kind: ' | awk '{n++} /Schema/ {s++} END {print n, s}'  # 158 80
k spec/README.md '524 in all: 227 of the layer `language`, 246 of the layer `convention` and 51 of the layer `interface`' 'Draft; 14 promises; no vectors yet' 'Draft; 17 items' '158 documents exported' '`admission-report.v1`, `inventory.v1`' 'Draft; 15 contracts'  # 1 1 1 1 1 1
k spec/core/README.md '| 15 Schema contracts:' 'exports 158 documents, 80 of them Schema contracts' "a login's admission report, an inventory" 'login Resource, through no Conduit | CTR-015, IFC-054 |' '| CTR-015, IFC-054: answer whether model work may start on a login |'  # 1 1 1 1 1
grep -cE '^\| (IFC-054 \| interface \| must \| 1 \| 1|PND-005 \| convention \| must \| 10 \| 0) \|' engine/STANDING.md  # 2
grep -cE '^- id: PND-0(05|16)$' experiments/012-model-checking/account.yaml  # 1
grep -c 'io.github.shrav-k.conduit/caller' conformance/conventions/requests/admission-report-kept/case.yaml  # 0
ls conformance/conventions/requests docs/work-orders | grep -cE '^admission-|^316-'  # 1
```

Where a line moved, work from where it stands and list it under `own_choices`.

## Scope

Change, as the decisions say: PND-005, PND-011 and PND-016 in `spec/promises/pending.yaml`;
CTR-015's observable in `spec/promises/contracts.yaml`; IFC-054 in
`spec/interface/implementation.yaml`; DL3's sentence and DL21 in
`spec/implementation-defined.yaml`; in `spec/core/`, `results.yaml`, `operations.yaml`,
`conduit.yaml` and `README.md`; `spec/definitions.md`'s two rows; `spec/README.md`'s counts and
its two `core/` rows; `spec/CHANGES.md`; PND-016's row of
`experiments/012-model-checking/account.yaml`; the five cases, their index entries and
expected-failures entries; the report case and its index entry, for decision 10;
`docs/work-orders/316-declared.yaml`; `engine/STANDING.md` by the tool alone. A shape check of a
document that the conformance runner cannot assert goes to `spec/schemas/examples/` (`valid/`,
`invalid/` or `invalid-by-rule/`), with its line in
`engine/crates/loader/tests/expected/invalid-verdicts.txt`, both in Scope; none is expected. Not
the decision log (each decision stays not done), `engine/CONFORMANCE.md` (its DL3, DL4 and DL21
rows are the engine order's), any other promise, `engine/` sources, `packages/` or `tools/`.
Where a test fails because the engine reads the changed spec, report it and change no engine
file.

## Done when

Each line prints otherwise before this order and passes after, with `j` and `k` as above:

```sh
grep -cE '^- id: PND-016$' spec/promises/pending.yaml  # 1; 0 before
grep -cE '^- id: DL21$' spec/implementation-defined.yaml  # 1; 0 before
grep -cE '^  name: admission-(hold|status)\.v1$' spec/core/results.yaml  # 2; 0 before
grep -cE '^  - admission-(hold|status)\.v1$' spec/core/conduit.yaml  # 2; 0 before
sed -n '/^  name: admission-hold.v1$/,/^---$/p' spec/core/results.yaml | grep -oE 'at-time|newer-report|lever-change|levers-changed|after-call|failed|unknown|deadline|not-dispatched' | sort -u | paste -sd' ' -  # after-call at-time deadline failed lever-change levers-changed newer-report not-dispatched unknown; nothing before
sed -n '/^  name: admission-status.v1$/,/^---$/p' spec/core/results.yaml | grep -oE 'valid-for-ended|single-use|newer-limits|levers-changed|current|none' | sort -u | paste -sd' ' -  # current levers-changed newer-limits none single-use valid-for-ended; nothing before
sed -n '/^  name: inspect.v1$/,/^---$/p' spec/core/operations.yaml | grep -c 'core#admission-status.v1'  # 1; 0 before
k spec/promises/pending.yaml 'Otherwise calls on one login run in parallel, subject to PND-016.' 'notification (IFC-054), for the caller of the request whose wait started it (PND-016). Setup checks'  # 1 1; 0 0 before
k spec/interface/implementation.yaml 'or an Agent whose login place refers to one, is inspected.' 'in the form of the core schema `admission-status.v1`' 'is kept already ended, and decides no call (PND-016).' 'on an Agent of the login, under PND-016, and no inspection calls it (OBS-006).' 'Where the call succeeded, the record of its end holds'  # 1 1 1 1 1; 0 0 0 0 0 before
k spec/interface/implementation.yaml 'This version keeps the report and decides nothing from it' 'but no call is yet held or refused on them (CTR-015).' 'Where it succeeded'  # 0 0 0; 1 1 1 before
k spec/promises/contracts.yaml "The engine decides each call that starts model work on an Agent of the login on the login's report, under PND-016;" "In this version, a login's levers are declared" 'acting on that report before model work is not yet specified (IFC-054).'  # 1 0 0; 0 1 1 before
k spec/implementation-defined.yaml "No promise limits how long an invocation is in progress, PND-016's ask apart, which DL21 bounds:" "DL10's time allowed for a call bounds the ask as well"  # 1 1; 0 0 before
k spec/definitions.md 'and enforced before each call that starts model work on an Agent of it (PND-016).' '`admission-hook.v1` for that of the request whose wait started it (PND-016), and the engine'  # 1 1; 0 0 before
k spec/core/results.yaml 'A hold under PND-016 carries `properties.admission` in the form `admission-hold.v1`.' '# admission-hold.v1 PND-016 (why a call' '# admission-status.v1 IFC-054 (a login'  # 1 1 1; 0 0 0 before
grep -cE '^\| `admission-(hold|status)\.v1` \| Schema \|' spec/core/README.md  # 2; 0 before
k spec/core/README.md '| 17 Schema contracts:' 'exports 160 documents, 82 of them Schema contracts' "a login's admission report, why a call on a login was held, a login's admission state, an inventory" 'through no Conduit, for a call that waits under PND-016 | CTR-015, IFC-054, PND-016 |' '| CTR-015, IFC-054, PND-016: answer whether model work may start on a login |'  # 1 1 1 1 1; 0 0 0 0 0 before
cat spec/core/*.yaml spec/schemas/*.yaml | grep -E '^kind: ' | awk '{n++} /Schema/ {s++} END {print n, s}'  # 160 82; 158 80 before
k spec/README.md '525 in all: 227 of the layer `language`, 247 of the layer `convention` and 51 of the layer `interface`' 'Draft; 15 promises; no vectors yet' 'Draft; 18 items' '160 documents exported' '`admission-report.v1`, `admission-hold.v1`, `admission-status.v1`, `inventory.v1`' 'Draft; 17 contracts'  # 1 1 1 1 1 1; 0 0 0 0 0 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- '  # 5; 0 before
grep -c '^- id: PND-016$' experiments/012-model-checking/account.yaml  # 1; 0 before
cat conformance/expected-failures.yaml conformance/conventions/requests/index.yaml | grep -cE '"id": "requests-admission-(hold|no-report|single-use|wait-ends|not-checked)"'  # 10; 0 before
grep -A6 '"id": "requests-admission-report-kept"' conformance/conventions/requests/index.yaml | grep -c '"PND-016"'  # 1; 0 before
grep -c '"io.github.shrav-k.conduit/caller": "00000000-0000-4000-8000-000000000005"' conformance/conventions/requests/admission-report-kept/case.yaml  # 2; 0 before
grep -cE '^\| (PND-016 \| convention \| must \| 6 \| 6|IFC-054 \| interface \| must \| 6 \| 6) \|' engine/STANDING.md  # 2; 0 before
for c in hold no-report single-use wait-ends not-checked; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-$c 2>&1 | grep -A1 "^EXPECTED FAILURE requests-admission-$c " | cut -c1-200); done  # per case its EXPECTED FAILURE line, then "  step 16: ..." ("  step 11: ..." for not-checked); nothing before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
n="conformance/conventions/requests/admission-hold conformance/conventions/requests/admission-no-report conformance/conventions/requests/admission-single-use conformance/conventions/requests/admission-wait-ends conformance/conventions/requests/admission-not-checked docs/work-orders/316-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # base and head equal; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/316-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
v='claude|codex|anthropic|openai|chatgpt'; a=$(git grep -ohiE "$v" HEAD -- spec | wc -l); b=$(grep -rohiE "$v" spec | wc -l); [ $a -eq $b ] && echo same  # same: no vendor named (DEC-105)
git diff --check  # prints nothing
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-report-kept 2>&1 | grep -A1 "^EXPECTED FAILURE requests-admission-report-kept " | cut -c1-200)  # its EXPECTED FAILURE line, then "  step 16: ...", as before
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/core
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case's first failing step as the runner prints it, the script
that derived the cases and every check's last line.

## What follows

The warnings' spec order: IFC-054's warning key on a call admitted on `warning` or a
`cannot-tell` yes, carrying `admission-hold.v1` with `dispatched` true, and the record's warning
naming PND-016; the refusal of an Agent's Conduit listing a Type's `admission.control`; and the
hook and the control in ACC-058's table. The engine orders: keeping (CTR-015's checks, IFC-054's
keeping and the status), which removes the checks case's entry and `requests-admission-not-checked`'s;
the ask and the wait (one ask per login, DL21, the 60 seconds, the access decided again when a
wait ends, a report kept already ended, a cancel and a restart while waiting), whose order amends
`engine/CONFORMANCE.md`'s DL3 and DL4 rows for this wait alone and adds a DL21 row at 30
seconds, revisited once the probe has timed a read; and the hold (the text, `admission-hold.v1`,
the names by inspection), after `hold_work`; the last two remove the report case's entry and the
other four of this order's. Then the packages' new Types (DEC-124). No team-seat Agent takes a
turn until the guard merges (DEC-102).

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, stacked on orders 307, 314 and 315.
The usage guard's spec order S2a (DEC-166). Written, built, read and fixed in stage 3 of
experiment 028; it is no order of stage 3.

**Worker-2's turn** (18:23 to 18:43) wrote PND-016, `admission-hold.v1`, `admission-status.v1`,
DL21 and the five cases, each failing first where the order said.

**Reads and fixes.** The Codex verifier, `root-verifier-2.turn.v1`, passed it; the Claude reader
(`reader-1`) had three shoulds and a nit, applied in one fix turn. The owner then struck three of
the coordinator's choices (DEC-168 to DEC-170): a report ask's records name its cause; a lever
change makes waiting calls wait for a fresh ask; a waiting call's access is checked once at
dispatch. Worker-2 applied them; a confined re-read found two inconsistencies left by them, fixed
in one more turn. The gate's run on the squash is the coordinator's, at the merge.
