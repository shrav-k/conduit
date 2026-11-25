# Work order 318: a warned call's admission key, and no Agent's Conduit listing a login's admission control (DEC-099, DEC-119, DEC-120, the usage guard's fourth spec order)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the three cases this order adds, all expected failures; no case changes. It needs no
network and changes no Rust source; if a crate is missing from the cache, fetch nothing and
report it. Run one cargo command at a time, and not `tools/gate.py`. Never print a whole large
file, case, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Run no `conduit` outside a test, no `codex` or `claude`,
and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write
is public: no absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.**
[DEC-099](../decisions/log.md#dec-099-2026-11-23-a-guarded-hold-tells-the-caller-everything-but-amounts),
"Accept (Recommended)": the hold names the login, the lever and its value, the window, its used
percentage, the threshold, the reset, how it lifts and which report, in text and a structured
field, amending DEC-077's "nothing relayed to the model" for this diagnostic only.
[DEC-119](../decisions/log.md#dec-119-2026-11-23-no-agent-tunes-a-logins-usage-levers),
"Not Agents, ever (Recommended)": no Conduit whose consumer is an Agent lists a login Type's
admission control, and such an open reports refused; an Interface's client tunes.
[DEC-120](../decisions/log.md#dec-120-2026-11-23-the-guard-checks-before-each-turn-and-a-started-turn-runs-to-its-end),
"I don't want to modify how the models operate just police their usage", then "Yes, check before
turns only (Recommended)": no lever reaches a turn, and turns stay as they are.
[DEC-098](../decisions/log.md#dec-098-2026-11-23-an-unknown-usage-report-is-decided-per-account-kind),
"Per account kind (Recommended)": where a login cannot tell, one account kind allows one warned
trial. [DEC-079](../decisions/log.md#dec-079-2026-11-22-access-requests-health-and-history-now),
"All three now": the refusal list holds every operation no Conduit with an Agent consumer may
list. [Order 316](316-admission-hold-before-a-turn.md) wrote the hold and `admission-hold.v1`,
with `dispatched` true defined for this order, and
[order 317](317-engine-checks-admission-and-keeps-reports.md) built CTR-015's checks and
IFC-054's keeping. This order writes the warning: IFC-054's key on the result of a call
admitted on a report whose `reason` is `warning` or `cannot-tell`, and `record.v1`'s
`admissionWarning`; DEC-119's refusal: CTR-015 at open, its place in ACC-006 and ACC-056,
and, as DEC-079's refusal list, ACC-058's rows for the admission hook and a login's admission
control; and CTR-015's sentence that nothing of `admission` reaches a call dispatched to an
Agent's implementation, a calling Agent learning a hold or a warning from its own request's
result. The owner's strikes of order 316's choices (DEC-168 to DEC-170) are in its spec as the
tree holds it; none bears on this order.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 1, the
warning decides nothing and holds no later call, and nothing of it reaches the called Agent's
turn or implementation: the caller learns it from its own request's result, as DEC-099 has a
caller learn a hold; 2, the refusal only narrows: no Conduit is opened, and nothing is granted;
6, the warning starts no call and no model work, and the refusal dispatches nothing; 7, the
warning is in the record of the call's dispatch and names the report and its ask, and the
refusal is recorded as any is. At risk, 5, every management action available to an agent under a grant: no
Agent may hold a login's admission control, as none may hold `answer.v1` or `sign-in.v1`
(PND-009, PND-010); this is DEC-119's accepted cost, that root cannot give a coordinating Agent
the levers. And 4, no vendor assumed: nothing under `spec/` names a vendor (DEC-105), and the
cases' window ID and methods are neutral.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a Kubernetes validating admission webhook may allow a request and return
`warnings`, which the API server gives the client in `Warning` response headers (code 299) while
the request goes on unchanged, as a warned call is dispatched as any other and its caller learns
the warning from the result; and an explicit deny that no grant overrides, as a service control
policy's `Deny` holds for every principal of a member account whatever its own policies allow, is
how Conduit already keeps `answer.v1` and `sign-in.v1` off a Conduit whose consumer is not an
Interface (PND-009, PND-010). Not taken: the warning in a turn's request, its `_meta` or any
argument (DEC-120); the warning in a result's `content` or `structuredContent`, which would change
a plain result's content (IFC-025); the warning deciding, delaying or holding any call; refusing
the Conduit at registration, which PND-009 and PND-010 do not do either; refusing a Conduit whose
consumer is an Interface or a Resource, as DEC-119 names Agents alone; matching ACC-058's control
row on the ask's `target`, which would make its result depend on the target, against ACC-058's
rule that a missing, a hidden and a replaced `target` give the same result; and refusing a request
through an Agent's Conduit opened before this rule, as none can exist: no package's Type states
`admission` yet (order 317's fifteenth Start check line), and no project the owner runs does.

**When it runs.** It stacks on orders 307, 314, 315, 316 and 317 and runs on 317's build (`main`
with the decision log through DEC-157, and the builds of 307, 314, 315, 316 and 317 on top) until
they merge, then on `main`. Order 319, the ask and the wait, depends on 316 and 317, not on this
order, and may merge first: its build removes `requests-admission-report-kept`'s entry from
`conformance/expected-failures.yaml`, so the Start check's line for that entry gives 0 there.
On that build, decision 9 and the Done when give that path's first failing step and reason
beside today's. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). It is one worker turn:
it adds no promise and no schema, and its three cases copy the single-use case's setup. By ruling
89, each decision says what must hold; how is yours. The words, the cases' names and titles and
these choices are the coordinator's, open to the owner's strike (DEC-029): the warning written
into IFC-054 by widening its `when`, as order 316 put the status there; the key
`io.github.shrav-k.conduit/admission` holding one diagnostic of level `warning` whose
`properties.admission` is `admission-hold.v1` with `dispatched` true, so that a warning gives
text and a structured field as DEC-099 has a hold give them; that diagnostic kept in the record
of each warned dispatch, in a new `record.v1` member `admissionWarning`, not in its end record,
since the dispatch is what the report decided; each dispatch, of a request or of a new attempt of
one, warned on its own report, and the key on the result that ends that attempt, whatever its
outcome or reason, and on a repeat that OPS-009 answers with that result, and on no other part
of the result; `admission-hold.v1` with `dispatched` true allowing only `warning` and
`cannot-tell`; a single-use report's warning giving `singleUse` null, as the schema has it for a
dispatched call, the report's own `singleUse` saying it; reading CTR-015's "Nothing of
`admission` reaches an Agent's implementation" as about the calls dispatched to an Agent's
implementation (IFC-006), as order 317 read it, and saying so in CTR-015, so that a calling
Agent, whose own request's result reaches its implementation, learns a hold or a warning there;
the refusal written in CTR-015, a promise of the `language` layer, though it is decided when an
open arrives, as order 314 wrote there the refusal of a control that would store `whenUnknown`
`admit` beside a refusing `paidUsage`, and decided under it, in ACC-056's sixth group as
PND-009's and PND-010's are, at every open, a setup's part and root's own among them, with the
Conduit left as it was; the refusal's words; a pointer in ACC-006 as it has one for SEC-008;
ACC-058's two rows and their messages, and a login's admission control matched on the operation
alone, by any Type the engine holds; that match written as an exception in ACC-058's sentence on
what a result depends on and in ACC-056's tier-1 rule, rather than as a rule that a Type's
naming of its admission control is hidden from no caller, since tier 1 then tells an Agent only
that an operation it named itself is some Type's admission control, as DEC-079's refusal list
and DEC-119 have it told, and names no Type, login or target; the sentences in the definitions;
and the three cases, R4 and its lever, the Agent's Conduit registered by a request, and the
single-use case's documents shared by relative path.

**Shared files.** Orders 299 and 309 change `spec/promises/pending.yaml`, `spec/definitions.md`
and `spec/CHANGES.md`; order 305 changes REC-002, `record.v1`, `spec/core/results.yaml` and the
definitions; order 304 changes CTR-013's observable in `spec/promises/contracts.yaml`, where this
order changes CTR-015's text; orders 303, 304 and 308 change some of the same files; and the
guard's engine orders remove entries of `conformance/expected-failures.yaml` and regenerate
`engine/STANDING.md`. Whichever merges later rebases, recounts and reruns
`promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. In the YAML, indentation and flow style are the file's,
and a `#` line stands for what it names. No text under `spec/` names a vendor or client product.
"A" → "B" replaces A wherever it wraps; "A" +"B" adds B after A.

1. **CTR-015** (`spec/promises/contracts.yaml`). In its `then`, "each with a diagnostic naming
   CTR-015 and both pointers." +" A Conduit whose consumer is an Agent and that lists an
   operation that its target's Type names as `admission.control`, compared as qualified
   references, is not opened: a request to open it, whoever sends it, root among them, whether
   on its own or as a part of a setup (CON-004), is not dispatched and reports refused under
   this promise." In its `observable`, "Nothing of `admission` reaches an Agent's
   implementation." → "Nothing of `admission` reaches a call dispatched to an Agent's
   implementation (IFC-006); a calling Agent learns a hold or a warning only from its own
   request's result (PND-016, IFC-054)."; after it, in your words and citing no decision:
   - no Agent tunes a login's levers: the control reaches a login only through a Conduit to it
     whose consumer is not an Agent, an Interface's among them, and is authorized there as any
     control is (ACC-033); a Conduit whose consumer is an Agent may still list the login's other
     operations;
   - the refused open's diagnostic, all of it in tier 1, as the bound of the caller's Conduit to
     `access` covers the target (ACC-056), names the Conduit as the request named it and the
     operation by its qualified reference; says that the operation is the admission control of
     the target's Type, named by its qualified reference, and changes the login's usage levers,
     and that no Conduit whose consumer is an Agent may be opened listing it, whoever sends the
     open; says that nothing was opened, not even the Conduit's other operations, and that the
     Conduit stays as it was; and says that an Interface's client tunes the levers, through a
     Conduit of its own to the login;
   - the Agent's calls that start model work on the login are decided on its report as before
     (PND-016), whatever Conduits it holds.
2. **ACC-006, ACC-056 and ACC-058** (`spec/conventions/access.yaml`).
   - ACC-006's `then`: "Where the new Conduit lists `get-secret.v1`, SEC-008 states a further
     condition." +" Where its consumer is an Agent and it lists its target's admission control,
     CTR-015 refuses it."
   - ACC-056's `then`: "and ACC-032, ACC-058, PND-009 and PND-010 on a Conduit to be opened that
     lists an operation they restrict" → "and ACC-032, ACC-058, CTR-015, PND-009 and PND-010 on a
     Conduit to be opened that lists an operation they restrict"; "but an asking Conduit's
     missing bound covers none (ACC-058)." +" The one exception is ACC-058's row for a login's
     admission control, which also depends on whether a Type of an installed package or of a
     submitted project names the asked operation as its `admission.control` (CTR-015)." In its
     `observable`, "except where the `then` lets tier 1 depend on a named object within the
     caller's bound" → "except where the `then` lets tier 1 depend on a named object within the
     caller's bound or, for ACC-058's row for a login's admission control, on the Types the
     engine holds"; in its table, after the row of CTR-013, the row "| CTR-015 | No Conduit whose
     consumer is an Agent may be opened listing a login's admission control; an Interface's
     client tunes the levers. |", on one line as the others.
   - ACC-058's `then`: "This is decided on the operation's qualified reference in the core
     package alone, as PRV-004 matches, and on the `target` as named." +" A login's admission
     control, the table's last row, is matched instead on the operation alone: an operation that
     a Type of an installed package or of a submitted project names as its `admission.control`
     (CTR-015) is one, whatever `target` the ask names."; "Apart from the bound, a result depends
     only on those names" → "Apart from the bound and, for the last row, the Types the engine
     holds, a result depends only on those names". Tier 1 may carry that fact, as ACC-056 then
     says: the message tells an Agent only that the operation it named itself is some Type's
     admission control, as DEC-079's refusal list and DEC-119 have it told, and names no Type,
     login or target; a missing, a hidden and a replaced `target` still give the same result.
     In its `observable`, "Names in the table are the core package's qualified references, .v1
     unless versioned explicitly." +" The last row names no core operation but any login's
     admission control, matched as the `then` says."
     In its table, each on one line as the others, after the row of `sign-in-hook, account-hook,
     sign-out-hook`, the row "| admission-hook | the engine alone calls it, through no Conduit, to
     decide a call that starts model work on an Agent of a login (PND-016). | There is nothing to
     ask for: the engine asks the login itself before each such call. |"; and after the row of
     `install`, as the last, the row "| a login's admission control | it changes a login's usage
     levers, which only an Interface's client tunes. No Conduit whose consumer is an Agent may
     list it (CTR-015). | If a login's levers hold your work, say so in your reply; an Interface
     tunes them. |".
3. **IFC-054** (`spec/interface/implementation.yaml`). In its `when`, "(CTR-015) ends; or a
   Resource whose Type states" → "(CTR-015) ends; or the engine dispatches a request, or a new
   attempt of one, under PND-016 on a report whose `reason` is `warning` or `cannot-tell`, or
   that attempt ends; or a Resource whose Type states". In its `then`, "under the stored levers,
   as PND-016 specifies." +" Each dispatch of a request, or of a new attempt of one (OPS-012,
   OPS-024), that PND-016 decides on a report whose `reason` is `warning` or `cannot-tell` is
   warned: the record of that dispatch holds in `admissionWarning`, and the result that ends that
   attempt, whatever its outcome or reason, holds in its `_meta` under
   `io.github.shrav-k.conduit/admission`, one diagnostic whose `level` is `warning`, whose
   `ruleId` is PND-016 and whose `properties.admission` has the form of the core schema
   `admission-hold.v1`, with `dispatched` true and that report as its `report`. A repeat that
   OPS-009 answers with a warned attempt's result carries that warning too. A dispatch decided on
   a report whose `reason` is `within-levers`, or one to an Agent of a login whose Type states no
   `admission`, is not warned: its record and the result that ends its attempt carry no warning,
   whatever an earlier attempt carried." In its `observable`, "keeps working unchecked, and its
   status says so." +, in your words and citing no decision:
   - the warning reaches the caller alone, once the attempt has ended: the request is sent to
     the called Agent's implementation exactly as on a report whose `reason` is `within-levers`
     (PND-016), and the key changes no other part of the result, its `isError`, `content` and
     `structuredContent` among them, so a plain result's content stays unchanged (IFC-025);
   - the warning decides nothing: the call is dispatched and runs as it would with no warning,
     no later call is held for it, and the record keeps it whatever the Type's `records` states;
     each attempt is warned, or not, on the report that decided its own dispatch;
   - its text names the login by kind, name and ID; says that the call was dispatched and started
     model work on a report that warns; for `warning`, names the lever and its value and the
     window with its used percentage and reset; for `cannot-tell`, says why the login cannot
     tell, in `cannotTell`'s word, and names the `whenUnknown` lever and its value, and, for a
     single-use report, says that the report admitted this call alone and is spent, so that the
     login's next such call waits for a new ask; gives the paid usage read, each value "not
     reported" where null; says that a later call on the login may be held; and names the ask
     only where the caller may inspect the login, as `reportInvocation` is null otherwise;
     neither the text nor `admission-hold.v1` names an amount, a balance or a currency.
4. **PND-016** (`spec/promises/pending.yaml`). In its `observable`, "the requests one report
   admits run in parallel, as PND-005 lets calls on one login." +" Each dispatch, of a request or
   of a new attempt of one, on a report whose `reason` is `warning` or `cannot-tell` is warned
   under IFC-054: its caller learns it from the result that ends that attempt, and the record of
   that dispatch holds it."
5. **REC-002** (`spec/conventions/records.yaml`). In its `observable`, "The end record of a
   login's `admission-hook.v1` call that succeeded also holds its report in `admission`, under
   IFC-054, whatever `records` states." +" The record of each dispatch that PND-016 decided on a
   report that warns, of a request or of a new attempt of one, holds the warning in
   `admissionWarning`, under IFC-054, whatever `records` states."
6. **The core schemas** (`spec/core/results.yaml`).
   - `admission-hold.v1`: in its description, "With `dispatched` true, which no result in this
     version carries, it describes the" → "With `dispatched` true, as the warning of IFC-054
     carries it, it describes the"; in its first `allOf` rule, whose `if` is `dispatched` true,
     `report`'s `properties` gain, after `admit: {const: true}`,
     `reason: {enum: [warning, cannot-tell]}`; in the header, its entry "admission-hold.v1
     PND-016 (why a call that starts model work on a guarded login was held)" → "admission-hold.v1
     PND-016, IFC-054 (why a call that starts model work on a guarded login was held, or what
     warned it)".
   - `diagnostic.v1`: in its description, "A hold under PND-016 on a report or for want of one
     carries `properties.admission` in the form `admission-hold.v1`." +" So does the warning of
     IFC-054, of an attempt dispatched on a report that warns, whose `level` is `warning` and
     whose `properties.admission` has `dispatched` true."; in the header, under its entry, after its
     PND-016 lines, a line "IFC-054 (the warning of a warned call carries
     `properties.admission`)".
   - `record.v1`: `data`'s `properties` gain, after `admission`,
     `admissionWarning: {$ref: github.com/shrav-k/conduit/core#diagnostic.v1}`; in its
     description, "it is distinct from `serving`." +" The record of each dispatch that PND-016
     decided on a report that warns holds its warning in `admissionWarning` (IFC-054).";
     in the header, under its entry, after its PND-016 line, a line "IFC-054 (the dispatch record
     of a warned call holds its warning in `admissionWarning`)".
7. **The sentences this needs.**
   - `spec/definitions.md`, the `login Resource` row: "and enforced before each call that starts
     model work on an Agent of it (PND-016)." → "and enforced before each call that starts model
     work on an Agent of it (PND-016); the admission control of its Type tunes its levers, and no
     Conduit whose consumer is an Agent is opened listing it (CTR-015)."
   - `spec/definitions.md`, the `Conduit's keys` row: "shown in the status of IFC-053 and never
     relayed (IFC-052)." +" A result that the engine returns to a consumer may also carry
     `admission`, the warning of an attempt that PND-016 dispatched on a report that warns,
     relayed to the caller once that attempt ends (IFC-054)."
   - `spec/core/README.md`: the row "| `admission-hold.v1` | Schema | PND-016: why a call that
     starts model work on a guarded login was held |" → "| `admission-hold.v1` | Schema |
     PND-016, IFC-054: why a call that starts model work on a guarded login was held, or what
     warned it |".

   No promise, schema, contract or implementation-defined item is added, so no count in
   `spec/README.md` or `spec/core/README.md` changes. No row this order changes in
   `experiments/012-model-checking/account.yaml` holds a digest, and CTR-015 and IFC-054, of the
   layers `language` and `interface`, have none: the account does not change.
8. **CHANGES.** In `spec/CHANGES.md`, under `## [Unreleased]`, after its last item, each wrapped
   at the file's width as its other items are:

   ```text
   - Added: IFC-054's warning: each attempt that PND-016 dispatches on a report whose reason is `warning` or `cannot-tell` carries to its caller, once it ends, a warning diagnostic in `io.github.shrav-k.conduit/admission`, with `admission-hold.v1` and `dispatched` true, and the record of its dispatch holds it in `admissionWarning`; nothing of it reaches the called Agent's implementation (DEC-098, DEC-099, DEC-120).
   - Added: CTR-015 refuses the open of a Conduit whose consumer is an Agent and that lists its target's admission control, whoever opens it, and says that nothing of `admission` reaches a call dispatched to an Agent's implementation (DEC-119, DEC-120).
   - Added: ACC-058's rows for `admission-hook.v1` and a login's admission control: an Agent's ask for either is refused, raising nothing; the second is matched on the Types the engine holds, which its result and ACC-056's tier 1 may then depend on (DEC-079, DEC-119).
   - Changed: PND-016, ACC-006, ACC-056, REC-002, `admission-hold.v1` (`dispatched` true for `warning` and `cannot-tell` alone), `diagnostic.v1`, `record.v1` and the definitions' `login Resource` and `Conduit's keys`, for the warning and the refusal (DEC-099, DEC-119).
   ```

9. **Three cases,** each in its family's `index.yaml` in `id` order, profile `first-build`,
   from `{"recipe":"empty"}`, with neutral names and the titles given. This order names a case's
   steps by their index in its `steps`, counted from 0; the conformance runner counts them from
   1, so it prints the step at index 15 as `step 16`. Each case's `files` are
   `requests-admission-single-use`'s by relative path: the guarded project's manifest and
   fixture (`conformance/language/contracts/admission-shape/`) and the single-use case's own
   `valid-documents.json`, whose `login-holder.v1` config holds `whenUnknown` `one-trial` and
   which holds the control's Conduit template `zz-admission-control-template` and root's
   Conduit `zz-admission-control` to `login`. *The setup* is that case's steps 0 to 14, copied
   with their captures (`login`, `agent_a`, `agent_b` among them). Times are 2026-11-21 UTC; T is
   13:01:00. Derive the three cases' steps with one script of your own, run once and committed
   nowhere, and report what it did; commit only case files. Each new step has an evidence of its
   own in the setup's form, its IDs unique in the case. Each ask is dispatched as the single-use
   case's steps 15 and 17 dispatch it: the hook's discovery, then its call, whose `arguments`
   hold `levers` as stored, `{"billing":"included","paidUsage":"refuse","whenUnknown":"one-trial"}`,
   `limits` the login's as inspection shows them and `previous` the newest report or null, and
   whose `_meta` holds `io.github.shrav-k.conduit/caller` root's ID,
   `00000000-0000-4000-8000-000000000005`, and no `io.github.shrav-k.conduit/conduit`. Where a
   step ends a request whose reply the runner does not give, assert its record and report the
   form used. The reports, as `structuredContent.admission`: R1 is the report case's
   (`requests-admission-report-kept`, its step 17: `admit` true, `within-levers`,
   `validForSeconds` 3600); R3 is the single-use case's (its step 18:
   `{"admit":true,"reason":"cannot-tell","lever":{"pointer":"/admission/whenUnknown","value":"one-trial"},"window":null,"paidUsage":{"possible":null,"inUse":null,"resetsAt":null},"cannotTell":"never-observed","lifts":null,"observedAt":null,"source":{"client":null,"method":"example-read"},"validForSeconds":0,"singleUse":true}`);
   R4 is
   `{"admit":true,"reason":"warning","lever":{"pointer":"/admission/warnAt","value":80},"window":{"id":"example/all/300m","usedPercent":85,"resetsAt":"2026-11-21T14:00:00Z"},"paidUsage":{"possible":false,"inUse":false,"resetsAt":null},"cannotTell":null,"lifts":null,"observedAt":"2026-11-21T13:01:30Z","source":{"client":null,"method":"example-read"},"validForSeconds":600,"singleUse":false}`.
   R4's lever, `/admission/warnAt`, is one a package may add beside the three (CTR-015) and that
   the fixture's Type does not declare; the engine keeps R4 as received, as it checks only the
   report's shape (IFC-054), so the case needs no lever of the login's to match it.
   A warning W is `{"level":"warning","ruleId":"PND-016","properties":{"admission":H}}`, asserted
   without its `message`, whose words are the engine's; its hold H has `login`
   `{"kind":"Resource","name":"login","id":"$login"}`, `dispatched` true, `report` the report
   that admitted the call, `reportInvocation` the ask that gave it, `noReport`, `singleUse`,
   `lifts` and `retryAt` null, and `levers`
   `{"type":"example.org/conformance/shared-login#login-holder.v1","pointer":"/admission","control":"example.org/conformance/shared-login#set-admission.v1"}`.
   W3 is the warning with R3 and the first ask, W4 with R4 and the second.
   - `requests-admission-warning` (`conformance/conventions/requests/admission-warning/`; layers
     `convention` and `interface`; IFC-054 and PND-016; title "A call dispatched on a report that
     warns carries the warning to its caller and in its dispatch record, and nothing of it to the
     Agent; a report within the levers warns of nothing"): the setup; (i) at T a `turn.v1` on
     `a`: nothing reaches `a`, and the ask's discovery is dispatched to `login`, captured as the
     single-use case's step 15 captures it (`admission1`, and `a`'s request as `callA`); (ii) its
     call, `previous` null and `limits`
     `{"state":"unknown","windows":[]}`; (iii) at T it succeeds with R3: `a`'s call is dispatched
     to `a` as the single-use case's step 18 dispatches it, with no
     `io.github.shrav-k.conduit/admission` in its `_meta` (assert it absent), and the record of
     its dispatch (`invocation-dispatched`, `invocation` `a`'s) holds W3 in `admissionWarning`;
     (iv) at 13:02:00 `a`'s call succeeds: its reply is `isError` false, with `_meta` holding
     `io.github.shrav-k.conduit/outcome` succeeded and `io.github.shrav-k.conduit/admission` W3;
     (v) at 13:02:00 a `turn.v1` on `b`: a new ask's discovery, as R3 is spent, captured as
     `admission2`, and `b`'s request as `callB`; (vi) its call, `previous` R3; (vii) at 13:02:00
     it succeeds with R4: `b`'s call is dispatched to `b`, with no key in its `_meta`, and the
     record of its dispatch holds W4 in `admissionWarning`; (viii) at 13:03:00 `b`'s call succeeds
     with the report case's limits member (its step 20): its reply holds W4 under the key, and
     its end record holds the member, which ends R4 `newer-limits`; (ix) at 13:04:00 a `turn.v1`
     on `a`: a new ask's discovery, captured as `admission3`, and `a`'s request as `callA2`; (x)
     its call, `previous` R4 and `limits` the login's status as
     the report case's step 22 has it from the same member; (xi) at 13:04:00 it succeeds with R1:
     `a`'s call is dispatched, and the step's records hold no `admissionWarning` (assert it
     absent); (xii) at 13:05:00 `a`'s call succeeds: its reply holds no
     `io.github.shrav-k.conduit/admission` (assert it absent).
   - `access-admission-control-open` (`conformance/conventions/access/admission-control-open/`;
     layers `language`, `convention` and `interface`; CTR-015 and ACC-056; title "A Conduit whose
     consumer is an Agent and that lists a login's admission control is refused when opened, even
     by root, and stays registered; root's own is opened"): the setup; (i) at T root's
     `management.register.v1` of one Conduit, `agent-admission-control`, from
     `zz-admission-control-template`, consumer `{"kind":"Agent","name":"a"}`, target
     `{"kind":"Resource","name":"login"}`: succeeded, as `access-ask-refused`'s step 13 registers
     `ask-a`; (ii) at T root's `access.open.v1` of `agent-admission-control`: `isError` true,
     `_meta` with `reason` refused, a first diagnostic with `ruleId` CTR-015, `level` error and
     `properties.refusal.tier` 2, as root gets tier 2; no external intent; and its
     `access-decision` record with `dispatched` false, `reason` refused and `ruleId` CTR-015, as
     the access family's refused cases assert one; (iii) at T root's `management.inspect.v1` of
     `{"kind":"Conduit","name":"agent-admission-control"}`: it is not open, its lifecycle state
     `registered` as inspection shows a Conduit's (report the pointer asserted); (iv) at T root's
     `access.open.v1` of `zz-admission-control`: succeeded, as the single-use case's step 25 is.
   - `access-ask-admission-refused` (`conformance/conventions/access/ask-admission-refused/`;
     layers `convention` and `interface`; ACC-058 and ACC-056; title "An Agent's ask for a login's
     admission control or for the admission hook is refused with its row's message and raises
     nothing"): the setup; (i) at T root's `management.register.v1` of `ask-a`, from
     `github.com/shrav-k/conduit/core#access-request`, consumer `{"kind":"Agent","name":"a"}`,
     target `{"kind":"Resource","name":"access"}`, `as` `ask`, as `access-ask-refused`'s step 13;
     (ii) at T root's `access.open.v1` of `ask-a`, as its step 14; (iii) at T `a`, its identity
     `{"kind":"instance","id":"$agent_a"}`, sends `ask.request-access.v1` with `operation`
     `example.org/conformance/shared-login#set-admission.v1` and `target`
     `{"kind":"Resource","name":"login"}`: refused as `access-ask-refused`'s step 22 is, its one
     diagnostic `ruleId` ACC-058, `level` error and `properties.refusal.tier` 1, its message
     "example.org/conformance/shared-login#set-admission.v1 cannot be asked for (ACC-058): it
     changes a login's usage levers, which only an Interface's client tunes. No Conduit whose
     consumer is an Agent may list it (CTR-015). Nothing was raised. If a login's levers hold
     your work, say so in your reply; an Interface tunes them.", the first text block "refused: "
     and that message, no external intent, and no `io.github.shrav-k.conduit/pendingAction`
     (assert it absent); (iv) the same with `operation`
     `github.com/shrav-k/conduit/core#admission-hook.v1`, its message
     "github.com/shrav-k/conduit/core#admission-hook.v1 cannot be asked for (ACC-058): the engine
     alone calls it, through no Conduit, to decide a call that starts model work on an Agent of a
     login (PND-016). Nothing was raised. There is nothing to ask for: the engine asks the login
     itself before each such call."

   List the three alone in `conformance/expected-failures.yaml`, in `id` order:
   `access-admission-control-open` with the reason "CTR-015, order 318: the first build opens a
   Conduit whose consumer is an Agent and that lists a login's admission control; the guard's
   engine order for the warnings and the open makes it pass"; `access-ask-admission-refused` with
   "ACC-058, order 318: the first build does not offer request-access.v1; DEC-064's engine orders
   and the guard's engine order for the warnings and the open make it pass"; and
   `requests-admission-warning` with "IFC-054, order 318: the first build dispatches a call on a
   guarded login with no ask and carries no warning; the guard's engine orders for the ask, the
   hold and the warnings make it pass". Where order 319 merged first, `requests-admission-warning`
   takes instead "IFC-054, order 318: the first build asks for a login's report and dispatches on
   it, but carries no warning; the guard's engine orders for the hold and the warnings make it
   pass". Each must first fail where the unchanged engine lacks what it tests:
   `requests-admission-warning` at step 15, the first `turn.v1`, which that engine dispatches to
   `a` with no ask, or, where order 319 merged first, at step 17, where that engine asks, dispatches
   `a`'s call on R3 and keeps no `admissionWarning` in the record of its dispatch;
   `access-admission-control-open` at step 16, the refused open, which that engine opens; and
   `access-ask-admission-refused` at step 17, its first ask, as that engine offers no
   `request-access.v1`, or at step 15 or 16 where it takes no asking Conduit, as
   `access-ask-refused` would fail there too (report which). The runner prints these as `step 16`
   (`step 18` where order 319 merged first), `step 17` and `step 18`, `16` or `17`. A case that
   first fails at any other step is wrong: fix the case.

Declare all of it in `docs/work-orders/318-declared.yaml`, in 316's form, each case `added`, with
the paths `vector_delta.py` names for them; then run `python3 tools/promise_standing.py --write`.
No file under `spec/schemas/` changes: this order changes no document's shape.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
for d in 079 098 099 119 120; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1
cat spec/promises/pending.yaml spec/promises/contracts.yaml spec/interface/implementation.yaml spec/conventions/access.yaml | grep -cE '^- id: (PND-016|CTR-015|IFC-054|ACC-058)$'  # 4: order 316's spec is the base
cat spec/core/*.yaml spec/promises/*.yaml spec/conventions/*.yaml spec/interface/*.yaml spec/schemas/*.yaml spec/implementation-defined.yaml spec/*.md spec/core/README.md spec/schemas/README.md | grep -cE 'admissionWarning|shrav-k\.conduit/admission'  # 0
k spec/promises/contracts.yaml 'each with a diagnostic naming CTR-015 and both pointers.' "Nothing of \`admission\` reaches an Agent's implementation."  # 1 1
k spec/conventions/access.yaml 'Where the new Conduit lists `get-secret.v1`, SEC-008 states a further condition.' 'and ACC-032, ACC-058, PND-009 and PND-010 on a Conduit to be opened that lists an operation they restrict' "| CTR-013 | Only root may confirm a shared client ID in install.v1's confirmClientIds. |" "This is decided on the operation's qualified reference in the core package alone, as PRV-004 matches, and on the \`target\` as named." '| sign-in-hook, account-hook, sign-out-hook |' '| install | root gives it only in its own documents. |' "Names in the table are the core package's qualified references, .v1 unless versioned explicitly." "Apart from the bound, a result depends only on those names and the caller's own Conduits and asks" "but an asking Conduit's missing bound covers none (ACC-058)." "except where the \`then\` lets tier 1 depend on a named object within the caller's bound."  # 1 1 1 1 1 1 1 1 1 1
k spec/interface/implementation.yaml '(CTR-015) ends; or a Resource whose Type states' 'under the stored levers, as PND-016 specifies.' 'keeps working unchecked, and its status says so.'  # 1 1 1
k spec/promises/pending.yaml 'the requests one report admits run in parallel, as PND-005 lets calls on one login.'  # 1
k spec/conventions/records.yaml "The end record of a login's \`admission-hook.v1\` call that succeeded also holds its report in \`admission\`, under IFC-054, whatever \`records\` states."  # 1
k spec/core/results.yaml 'With `dispatched` true, which no result in this version carries, it describes the' 'A hold under PND-016 on a report or for want of one carries `properties.admission` in the form `admission-hold.v1`.' 'it is distinct from `serving`.' '# admission-hold.v1 PND-016 (why a call that starts model work on a guarded login was held)' '# PND-016 (an admission ask names its triggering request in `causedBy`)'  # 1 1 1 1 1
awk '/^  name: admission-hold.v1$/,/^---$/' spec/core/results.yaml | grep -cE '^            admit: \{const: true\}$'  # 1: the first allOf rule's
k spec/definitions.md 'and enforced before each call that starts model work on an Agent of it (PND-016).' 'shown in the status of IFC-053 and never relayed (IFC-052).'  # 1 1
grep -cF '| `admission-hold.v1` | Schema | PND-016: why a call that starts model work on a guarded login was held |' spec/core/README.md  # 1
grep -cE '^\| (PND-016 \| convention \| must \| 6 \| [456]|IFC-054 \| interface \| must \| 6 \| [456]|CTR-015 \| language \| must \| 2 \| 0|ACC-058 \| convention \| must \| 4 \| 4|ACC-056 \| convention \| must \| 7 \| 1) \|' engine/STANDING.md  # 5; PND-016's and IFC-054's rows read `6 | 6` on 317's build, `6 | 5` or `6 | 4` where an engine order removed an entry of theirs: note which
grep -cE '"id": "(requests-admission-single-use|access-ask-refused)"' conformance/expected-failures.yaml  # 2: the cases whose steps this order copies fail as expected
grep -c '"id": "requests-admission-report-kept"' conformance/expected-failures.yaml  # 1: order 319 has not merged; 0 where it has (When it runs)
grep -c '"name": "zz-admission-control-template"' conformance/conventions/requests/admission-single-use/valid-documents.json  # 1
for p in ACC-006 ACC-056 ACC-058 PND-016 REC-002; do sed -n "/^- id: $p\$/,/^- id: /p" experiments/012-model-checking/account.yaml | grep -c '^  digest:'; done | paste -sd' ' -  # 0 0 0 0 0
ls conformance/conventions/requests conformance/conventions/access docs/work-orders | grep -cE '^(admission-warning|admission-control-open|ask-admission-refused|318-.*)$'  # 0
```

Where a line moved, work from where it stands and list it under `own_choices`.

## Scope

Change, as the decisions say: CTR-015 in `spec/promises/contracts.yaml`; ACC-006, ACC-056 and
ACC-058 in `spec/conventions/access.yaml`; IFC-054 in `spec/interface/implementation.yaml`;
PND-016's observable in `spec/promises/pending.yaml`; REC-002's observable in
`spec/conventions/records.yaml`; `admission-hold.v1`, `diagnostic.v1`, `record.v1` and the
header of `spec/core/results.yaml`; `spec/core/README.md`'s one row; `spec/definitions.md`'s two
rows; `spec/CHANGES.md`; the three cases, their index entries and expected-failures entries;
`docs/work-orders/318-declared.yaml`; `engine/STANDING.md` by the tool alone. A shape check of a
document that the conformance runner cannot assert goes to `spec/schemas/examples/` (`valid/`,
`invalid/` or `invalid-by-rule/`), with its line in
`engine/crates/loader/tests/expected/invalid-verdicts.txt`, both in Scope; none is expected. Not
the decision log (each decision stays not done), `engine/CONFORMANCE.md`, any other promise, an
existing case, `experiments/`, `engine/` sources, `packages/` or `tools/`. Where a test fails
because the engine reads the changed spec, report it and change no engine file.

## Done when

Each line prints otherwise before this order and passes after, with `j` and `k` as above:

```sh
k spec/promises/contracts.yaml 'Type names as `admission.control`, compared as qualified references, is not opened' 'as a part of a setup (CON-004), is not dispatched and reports refused under this promise.' "Nothing of \`admission\` reaches a call dispatched to an Agent's implementation (IFC-006); a calling Agent learns a hold or a warning only from its own request's result" "Nothing of \`admission\` reaches an Agent's implementation."  # 1 1 1 0; 0 0 0 1 before
k spec/conventions/access.yaml 'admission control, CTR-015 refuses it.' 'and ACC-032, ACC-058, CTR-015, PND-009 and PND-010 on a Conduit to be opened' '| CTR-015 | No Conduit whose consumer is an Agent may be opened listing a' "The one exception is ACC-058's row for a login's admission control, which also depends on" "or, for ACC-058's row for a login's admission control, on the Types the engine holds"  # 1 1 1 1 1; 0 0 0 0 0 before
k spec/conventions/access.yaml 'is matched instead on the operation alone' 'a Type of an installed package or of a submitted project names as its' 'Apart from the bound and, for the last row, the Types the engine holds, a result depends only' '| admission-hook | the engine alone calls it, through no Conduit, to decide a call' 'No Conduit whose consumer is an Agent may list it (CTR-015). | If a' 'The last row names no core operation'  # 1 1 1 1 1 1; 0 0 0 0 0 0 before
k spec/interface/implementation.yaml 'or the engine dispatches a request, or a new attempt of one, under PND-016 on a report whose `reason` is `warning` or `cannot-tell`, or that attempt ends;' 'under `io.github.shrav-k.conduit/admission`, one diagnostic whose `level` is `warning`' "A repeat that OPS-009 answers with a warned attempt's result carries that warning too."  # 1 1 1; 0 0 0 before
k spec/promises/pending.yaml 'is warned under IFC-054: its caller learns it from the result that ends that attempt'  # 1; 0 before
k spec/conventions/records.yaml 'holds the warning in `admissionWarning`, under IFC-054'  # 1; 0 before
k spec/core/results.yaml 'With `dispatched` true, as the warning of IFC-054 carries it, it describes the' 'So does the warning of IFC-054, of an attempt dispatched on a report that warns' 'holds its warning in `admissionWarning` (IFC-054).' '# admission-hold.v1 PND-016, IFC-054 (why a call' '# IFC-054 (the warning of a warned call carries' '# IFC-054 (the dispatch record of a warned call holds'  # 1 1 1 1 1 1; 0 0 0 0 0 0 before
k spec/core/results.yaml 'With `dispatched` true, which no result in this version carries'  # 0; 1 before
awk '/^  name: admission-hold.v1$/,/^---$/' spec/core/results.yaml | grep -cE '^            reason: \{enum: \[warning, cannot-tell\]\}$'  # 1; 0 before
awk '/^  name: record.v1$/,/^---$/' spec/core/results.yaml | grep -cE '^        admissionWarning: \{\$ref: github\.com/shrav-k/conduit/core#diagnostic\.v1\}$'  # 1; 0 before
k spec/definitions.md 'the admission control of its Type tunes its levers, and no Conduit whose consumer is an Agent is opened listing it (CTR-015).' 'may also carry `admission`, the warning of an attempt that PND-016 dispatched'  # 1 1; 0 0 before
grep -cF '| `admission-hold.v1` | Schema | PND-016, IFC-054: why a call that starts model work on a guarded login was held, or what warned it |' spec/core/README.md  # 1; 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- '  # 4; 0 before
cat conformance/expected-failures.yaml conformance/conventions/requests/index.yaml conformance/conventions/access/index.yaml | grep -cE '"id": "(requests-admission-warning|access-admission-control-open|access-ask-admission-refused)"'  # 6; 0 before
grep -cE '^\| (PND-016 \| convention \| must \| 7 \| [567]|IFC-054 \| interface \| must \| 7 \| [567]|CTR-015 \| language \| must \| 3 \| 1|ACC-058 \| convention \| must \| 5 \| 5|ACC-056 \| convention \| must \| 9 \| 3) \|' engine/STANDING.md  # 5; 0 before; PND-016's and IFC-054's second number each one more than the Start check's line showed, `7 | 7` on 317's build
for c in requests-admission-warning access-admission-control-open; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. $c 2>&1 | grep -A1 "^EXPECTED FAILURE $c " | cut -c1-200); done  # per case its EXPECTED FAILURE line, then "  step 16: ..." for the warning case ("  step 18: ..." where order 319 merged first) and "  step 17: ..." for the open case; nothing before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. access-ask-admission-refused 2>&1 | grep -A1 "^EXPECTED FAILURE access-ask-admission-refused " | cut -c1-200)  # its EXPECTED FAILURE line, then "  step 18: ...", or "  step 16: ..." or "  step 17: ..." as the decisions say; nothing before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
n="conformance/conventions/requests/admission-warning conformance/conventions/access/admission-control-open conformance/conventions/access/ask-admission-refused docs/work-orders/318-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # base and head equal; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/318-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
v='claude|codex|anthropic|openai|chatgpt'; a=$(git grep -ohiE "$v" HEAD -- spec | wc -l); b=$(grep -rohiE "$v" spec | wc -l); [ $a -eq $b ] && echo same  # same: no vendor named (DEC-105)
git diff --check  # prints nothing
for c in requests-admission-single-use access-ask-refused; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. $c 2>&1 | grep -A1 "^EXPECTED FAILURE $c " | cut -c1-200); done  # each its EXPECTED FAILURE line and first failing step, the same before and after
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/core
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case's first failing step as the runner prints it, which
build the Start check found (order 319 merged or not, and PND-016's and IFC-054's rows), the
script that derived the cases and every check's last line.

## What follows

The guard's engine orders: the ask and the wait (order 319); the hold, after `hold_work`; and
the warnings and the open, which builds this order's warning (the key on the result, a repeat's
among them, and `admissionWarning` in the dispatch record), CTR-015's refusal at open with its
words, ACC-058's two rows once DEC-064's engine orders build ACC-058, and a test that a call
reaches the Agent's implementation the same with the guard as without (DEC-120). That order
removes `access-admission-control-open`'s entry and, with the ask's and the hold's,
`requests-admission-warning`'s; `access-ask-admission-refused`'s goes with DEC-064's. Then the
packages' new Types (DEC-124) and the CLI's showing of a held or warned call's admission. No
team-seat Agent takes a turn until the guard merges (DEC-102).

**Found: an Agent may reach the control through another instance.** DEC-119's words refuse a
Conduit whose consumer is an Agent. A Resource whose Conduit to a login lists the control, and
that an Agent calls, can send the control by an onward request whose caller is that Agent
(IFC-020, IFC-021), and nothing here refuses it. Refusing every request for a login's admission
control whose caller is an Agent, whatever Conduit carries it, would close that path; it changes
behavior beyond DEC-119's words, so the coordinator asks the owner, with that as the
recommendation.

## Record

Run on 2026-11-23 and 2026-11-24 through Conduit itself, in Pacific time, stacked on orders 307,
314, 315, 316 and 317 and the decision log's DEC-158 to DEC-171. The usage guard's spec order S2b
(DEC-166). Written, built, read and fixed in stage 3 of experiment 028; it is no order of stage 3.

**Worker-1's turns** (23:15 to 23:43). The first stopped rightly on a stale expected-failure entry,
the stack's artifact (order 316's not-checked case passes with 317's engine); by the coordinator's
leave the second removed it and finished.

**Two blind reads.** The Codex verifier, `root-verifier-1.turn.v1`, passed it with no finding. The
Claude reader's first read ended unknown, because the coordinator had stopped reader-1's live
client by mistake; after `serve` restarted, `reader-1` read it again: no must, two shoulds (one
settled by the leave above) and three nits. One fix turn by worker-1 applied the rest. The gate's
run on the squash is the coordinator's, at the merge.

**At the merge.** This order's `_meta` key `admission` made `tools/set_names.py --check` read the
package address `example.org/admission` in CTR-015's two valid examples (order 314) as a `_meta`
key with another prefix: eleven errors, which the gate's names row refuses. The coordinator
renamed the examples' package to `example.org/acme/admission`, as the spec's own
`example.org/acme/files` is written, and added one CHANGES item (ruling 87); the check gives 0.
