# Work order 314: a login's admission report and hook, and the levers a login Type declares (DEC-094, DEC-116, the usage guard's second spec order)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the three cases this order adds, one passing and two expected failures. It needs no network
and changes no Rust source; if a crate is missing from the cache, fetch nothing and report it.
Run one cargo command at a time, and not `tools/gate.py`. Never print a whole large file, case,
diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in
nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public:
no absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** DEC-093: "we should have an explicit flag for this, and flags for allowing
for usage credits etc".
[DEC-094](../decisions/log.md#dec-094-2026-11-23-usage-is-self-reported-by-packages-and-the-engine-trusts-the-report):
"Conduit specifies what must be reported and what contracts must be upheld. The engine then
trusts this reporting". DEC-097 and DEC-098, "Accept (Recommended)" and "Per account kind
(Recommended)", with DEC-093 and DEC-096 for paid usage: three fixed levers, `billing`,
`paidUsage` and `whenUnknown`.
[DEC-116](../decisions/log.md#dec-116-2026-11-23-a-model-account-is-its-login-resources-limits-are-enforced-there-not-in-fleets),
"Login Resource, as now (Recommended)": a model account is its login Resources, one per account
per machine, and the schemas are core because the engine checks them. DEC-120, "Yes, check before
turns only (Recommended)": no lever reaches a turn. DEC-123, "Wait in the request
(Recommended)", and DEC-124, "New Type versions (Recommended)". This order writes what a login
reports and declares: the report, the hook that gives it, its keeping and its end. The hold that
acts on it is the next spec order's.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 1, the
report decides from what a client reported, never from an Agent's reasoning, and nothing of it
reaches a turn; 3, an account is owned once, as its login Resources in a Pool (DEC-116); 6, the
hook starts no model work and changes nothing of the login; 7, each report is in its hook's end
record; at risk, 4, no vendor assumed: the levers and reasons are neutral words, and nothing under
`spec/` names a vendor (DEC-105).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a Kubernetes admission webhook answers allowed or not with a reason, and its
default `failurePolicy` is `Fail`; an HTTP cache trusts a response for the lifetime its origin
sets (RFC 9111's `max-age`). Not taken: amounts or budgets (DEC-099); the engine measuring usage
(DEC-094); a lever, key or stop reaching a turn, or a models lever (DEC-120); a minimum or clamp
on validity.

**When it runs.** It stacks on order 307 and runs on 307's build (`main` with the decision log
through DEC-157, and 307's build on top) until 307 merges, then on `main`. Through Conduit: one
Codex worker turn, the blind Codex verifier and Claude reader at once, one fix turn, the gate
(DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold; how is yours. The
words, the cases' names and titles and these choices are the coordinator's, open to the owner's
strike (DEC-029): the name and meaning of `admission.loginTypes`; the levers place's test; the
control's one input `levers`; the keep time as the end record's `time`, on the engine's own clock
so that no node's clock skew counts; the 7-day cap on `validForSeconds` (604800); the hook's
having no `effect`; the report kept in that record whatever `records` states; IFC-052's and
REC-002's clauses; the hook's exception worded without its caller and cause, which the hold's
order writes; a passing shape case; and the report case asserting DEC-095's and DEC-123's wait
ahead of their promise, its hook call's `caller` left unasserted.

**Shared files.** Orders 302 to 305, 308 and 309 change some of the same files (304 CTR-013's
observable, 305 REC-002 and `record.v1`'s description, 308 and 309 the core's files and counts),
and order 315, also on 307, removes the `requests-limits-signals` entry of
`conformance/expected-failures.yaml` (Start check line 3 prints 0 once 315 has merged) and
regenerates `engine/STANDING.md`: whichever merges later rebases, recounts and reruns
`promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. In the YAML, indentation and flow style are the file's,
and a `#` line stands for what it names. No text under `spec/` names a vendor or client product.

1. **`admission-report.v1`** (`spec/core/results.yaml`, after `limit-status.v1`; in the header,
   `admission-report.v1 IFC-054 (a login's answer to whether model work may start on it now)`):

   ```yaml
   spec:
     $defs:
       when: {type: [string, 'null'], format: date-time}
     type: object
     properties:
       admit: {type: boolean}
       reason: {enum: [within-levers, warning, cannot-tell, included-spent, paid-in-use,
                       paid-possible, paid-from-first-token, threshold]}
       lever:   # [object, 'null']: pointer (pointer.v1) and value ([string, number, boolean]), both required, closed
       window:  # [object, 'null']: id (limit-report.v1's pattern), usedPercent ([number, 'null'], minimum 0), resetsAt (when), all required, closed
       paidUsage:  # limit-report.v1's block
       cannotTell: {enum: [never-observed, not-reported, stale, read-failed, read-timed-out, null]}
       lifts: {enum: [at-time, newer-report, lever-change, null]}
       observedAt: {$ref: '#/$defs/when'}
       source:  # object: client (limit-report.v1's block, its type [object, 'null']) and method (as limit-report.v1's), both required, closed
       validForSeconds: {type: integer, minimum: 0, maximum: 604800}
       singleUse: {type: boolean}
     required: [admit, reason, lever, window, paidUsage, cannotTell, lifts, observedAt, source,
                validForSeconds, singleUse]
     additionalProperties: false
     allOf:  # five if/then rules:
     # admit true: reason within-levers, warning or cannot-tell, and lifts null; else neither of the first two, and lifts a string
     # reason within-levers: lever null; else lever an object
     # reason threshold or warning: window an object
     # reason cannot-tell: cannotTell a string; else null
     # singleUse true: admit true and validForSeconds 0; else validForSeconds at least 1
   ```

   Its description: "A login's answer to whether model work may start on it now, which its
   `admission-hook.v1` gives and the engine keeps (IFC-054). `admit` gives the answer and
   `reason` why. `lever` names the lever that decided, by its pointer at or under the Type's levers
   place and its value, and is null for `within-levers`; `window` the window that decided, by its
   package's fixed ID, used percentage and reset; `paidUsage` the paid usage read, each value null
   where no field gave it; `cannotTell` why it cannot tell, for `cannot-tell` alone. `lifts` says
   how a refusal ends: `at-time`, when its validity ends; `newer-report`, once a newer observation
   says otherwise; `lever-change`, only once the levers change. `observedAt` and `source` give the
   observation decided on and the client's call or event read. `validForSeconds` says how long the
   engine trusts it from the time it keeps it, and `singleUse` that it admits one call alone. It
   holds no amount."
2. **`admission-hook.v1`** (`spec/core/operations.yaml`, after `sign-out-hook.v1`): `inputSchema`
   with required `levers` (`type: object`), `limits` (`limit-status.v1`) and `previous`
   (`admission-report.v1` or null, by `anyOf`); `outputSchema` with required `admission`
   (`admission-report.v1`); no `effect`. Description: "Answer whether model work may start on this
   login now, under the levers given (CTR-015, IFC-054). A Type stating `signIn` and `admission`
   requires it. The engine alone invokes it, as its own invocation of the login Resource, through
   no Conduit and with no filter, never as a part, only to decide a call that starts model work on
   an Agent of the login; it is in no capability, so no Conduit lists it, and no inspection calls
   it (OBS-006). It starts no model work and changes nothing of the login. `levers` is the value at
   the Type's levers place, `limits` the login's limits as inspection shows them (IFC-053), and
   `previous` the newest report the hook gave, a single-use one included, or null." Export both new
   contracts in `spec/core/conduit.yaml`, each beside its neighbours. In `spec/core/README.md`: a
   row in the hooks' table (no capability, required by a Type stating `signIn` and `admission`; the
   engine alone, as its own invocation of a login Resource, through no Conduit; CTR-015, IFC-054);
   two rows of "Every export"; "The 41 Operation contracts" and "15 Schema contracts"; the total,
   which stays "158 documents, 80 of them Schema contracts" and is now true (the base states 158
   and 80 where its files hold 156 and 79), reporting any other count a recount shows off. In the
   same README, the `operations.yaml` row's "the setup, retire and sign-in hooks" becomes "the
   setup, retire and sign-in hooks and the admission hook"; the `results.yaml` row's "an
   instance's limits," gains after it " a login's admission report,"; and the "Every export" row of
   `type-spec.v1` ends its last cell with "; `admission`, a login's levers and control or an
   Agent's login Types, CTR-015".
3. **Keeping** (IFC-054's record). In `record.v1`'s `data`, after `limitsDropped`,
   `admission: {$ref: github.com/shrav-k/conduit/core#admission-report.v1}`, and at the end of its
   description: "The end record of a login's `admission-hook.v1` call that succeeded holds its
   report in `admission` (IFC-054)." In the header of `results.yaml`, a line under `record.v1`'s
   entry: "IFC-054 (the end record of a login's admission hook holds its report)". In REC-002's
   observable (`spec/conventions/records.yaml`), after "under IFC-051 and IFC-052, whatever
   `records` states.": "The end record of a login's `admission-hook.v1` call that succeeded also
   holds its report in `admission`, under IFC-054, whatever `records` states." Its account row is
   `static`, with no digest: it stays.
4. **CTR-015** (`spec/promises/contracts.yaml`, after CTR-014; language, must). When: "A Type
   contract states `admission`." Then: "On a Resource Type stating `signIn`, it holds exactly
   `levers`, a pointer into configuration, the levers place, and `control`, a selection of a
   control the Type lists under `controls`, and the Type requires the core operation
   `admission-hook.v1`. The levers place and every member on its path are required by the Type's
   `schema`, no `default` stands at or under the place, and its schema is an object that requires
   `billing`, `paidUsage` and `whenUnknown`, each confined by `enum` to some of its words:
   `included` and `paid-from-first-token`; `allow`, `refuse` and `refuse-if-possible`; `refuse`,
   `one-trial` and `admit`. The control's `changes` is exactly `{levers: <the levers pointer>}`,
   and no other control the Type lists has a pointer in its `changes` equal to it, inside it or
   over it, compared as CTR-006 compares pointers. On an Agent Type stating `loginResource`, it
   holds exactly `loginTypes`, a non-empty list of selections of Types that each state `signIn`
   and `admission`. No other Type states it. A contract breaking these rules is rejected at
   install or submission, with a diagnostic naming CTR-015, the Type and the cause. Registering or
   replacing an Agent whose login place refers to a Resource whose Type states `admission` reports
   invalid where the Agent's Type does not list that Type under `admission.loginTypes`, and so does
   replacing such a Resource with one whose Type a current Agent of it does not list, each with a
   diagnostic naming CTR-015, the Agent's Type and the login's Type. A levers value holding
   `whenUnknown` `admit` beside `paidUsage` `refuse` or `refuse-if-possible` reports invalid at
   registration and at replacement, and a control that would store one is not dispatched and
   reports invalid, each with a diagnostic naming CTR-015 and both pointers." Observable, in your
   words and citing no decision: what each fixed lever means (DEC-097: `billing` declares whether
   model work is included or paid from the first token, and a paid-from-first-token login whose
   `paidUsage` refuses admits nothing; DEC-093 and DEC-096: `paidUsage`; DEC-098: `whenUnknown`,
   `one-trial` being a single-use admission); a package adds its own levers beside them; no lever
   is an amount of money or credits, a balance or a currency, percentages and minutes are not, and
   a Type or README defining one does not conform; a package's schema states the pair rule as `if`
   and `then`, so REV-015 rejects it first; listing a Type under `loginTypes` says the Agent Type's
   results of calls that start model work carry the limits members (IFC-052) that the listed
   Type's package documents as its hook's input, and the engine checks the listing, never the
   members; the levers are the login's configuration, which reaches only the login's own calls
   (IFC-006) and its `admission-hook.v1` as the input `levers`, and nothing of `admission` reaches
   an Agent's implementation.
5. **IFC-054** (`spec/interface/implementation.yaml`, after IFC-053; interface, must). When: "The
   engine's call of `admission-hook.v1` on a login Resource whose Type states `admission`
   (CTR-015) ends." Then: "Where it succeeded, the record of its end holds its output `admission`,
   unchanged, in `admission`, whatever the Type's `records` states, and the engine keeps that
   report as the login's newest, with that record's `time` as its keep time. A call that failed or
   ended unknown, one whose output fails `admission-report.v1` among them (OPS-007), gives no
   report, and its record holds no `admission`. A kept report is current from its keep time until
   the first of these ends it for good: for a report that is not single use, `validForSeconds`
   seconds pass on the engine's own clock; a single-use report admits its one call; the engine
   keeps a limits member (IFC-052) in the end record of a call to the login, or to an Agent whose
   login place refers to it, other than the hook's call that gave the report; or a control stores
   a value at or under the Type's levers place (REV-007)." Observable, in your words: validity
   counts from the keep time, never from `observedAt`, with no minimum or bound beyond the
   schema's; the engine checks only the output's shape and keeps the report as received, and the
   implementation answers for what it says; v2's text rules: each value maps a field its client
   gave, by a table in its package's documentation, null where none does, nothing inferred; no
   string names an account, plan, amount, balance, limit or currency; a window `id` is the
   package's fixed word, as under IFC-052; `lever.pointer` lies at or under the levers place, and
   a `cannot-tell` report names the `whenUnknown` lever; where the client gives an included-usage
   signal, a null value of it is no answer, and the report is `cannot-tell`; what each `lifts`
   word means; `previous` is the newest report the hook gave, single use or ended included, or
   null; a member IFC-052 drops ends no report; the engine calls the hook only to decide a call
   that starts model work on an Agent of the login, and no inspection calls it (OBS-006).
6. **The sentences this needs.** IFC-053: after "hold or lifecycle state depends on it" insert
   ", apart from ending a login's admission report under IFC-054". IFC-052: "records and every
   decision are as without the key." becomes "records and every decision are as without the key,
   but that keeping it may end a login's admission report under IFC-054." CTR-013's observable,
   after "and exposed through no consumer tool.": "`admission-hook.v1`, which CTR-015 requires of
   a Type stating `signIn` and `admission`, is the one sign-in Type's hook invoked otherwise: the
   engine invokes it as its own invocation of the login Resource, through no Conduit and with no
   filter, never as a part (IFC-054), and it too is exposed through no consumer tool." Rewrap
   CTR-013 only as far as the line ending "What an Agent's client writes in that Resource's
   directory is", leaving a short line there if need be, and change no line after it. PND-011's
   observable, after "never on an implementation notification.": "`admission-hook.v1` is the one
   exception (CTR-013, CTR-015): the engine invokes it as its own invocation of the login
   Resource, with no filter, never as a part and never on an implementation notification
   (IFC-054)." CTR-004's "A Type may also state `signIn`, its account pointer, logins and three
   required hooks under CTR-013, and `toolsDirectory`, the own tools declaration of CTR-014."
   becomes "A Type may also state `signIn`, its account pointer, logins and three required hooks
   under CTR-013, `toolsDirectory`, the own tools declaration of CTR-014, and `admission`, a
   login's levers and control or an Agent's login Types, under CTR-015." PND-011's account row is
   `left to MC9`, with no digest: it stays.
7. **`type-spec.v1`** (`spec/schemas/contracts.yaml`): an `admission` property, closed, holding
   `levers` (`pointer.v1`) and `control` (`selection.v1`), or `loginTypes` (a non-empty list of
   unique `selection.v1`), and no mix; and an `admission` entry of `dependentSchemas`: either
   `family` is `const: Resource`, the Type states `signIn`, `admission` holds `levers` and
   `control`, and `requires` contains `github.com/shrav-k/conduit/core#admission-hook.v1`, or it
   states `loginResource` and `admission` holds `loginTypes`. Add the rule to its description and
   to the file's header, CTR-015's other checks to the header's "Not expressed here", and to
   `spec/schemas/README.md` a paragraph after the `loginResource` one and a row in the table of
   what a schema cannot express, each saying which CTR-015 checks the engine makes and that
   `check_schemas.py` makes none. In that README's Files row for `contracts.yaml`, "the latter
   only for Agent and never beside `signIn` (CTR-013)," becomes "the latter only for Agent and
   never beside `signIn` (CTR-013), and `admission` (CTR-015),".
8. **Definitions.** The `login Resource` row of `spec/definitions.md` (DEC-116's wording) gains:
   "A model account is represented by its login Resources, owned by a Pool, one per account per
   machine; its limits are declared there, by the levers its Type states, and reported by its
   `admission-hook.v1` (CTR-015, IFC-054). Neither a Fleet nor a Pool is an account, and a Fleet
   owns Agents and nothing else (FLEET-001, FLEET-004)."
9. **CHANGES and counts.** In `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

   ```text
   - Added: CTR-015, a login Type's `admission` (its levers place with `billing`, `paidUsage` and `whenUnknown`, its control and the required `admission-hook.v1`) and an Agent Type's `admission.loginTypes`, checked at install, submission, registration and replacement (DEC-093, DEC-094, DEC-096, DEC-097, DEC-098, DEC-120, DEC-124).
   - Added: IFC-054, `admission-hook.v1` and `admission-report.v1`, a login's admission report kept from its hook's end record until its validity, its single use, a newer limits member or a lever change ends it (DEC-094, DEC-096, DEC-123).
   - Changed: CTR-004, CTR-013, PND-011, REC-002, IFC-052, IFC-053, `type-spec.v1` and `record.v1`, for the admission hook and its report (DEC-094, DEC-123).
   - Changed: `login Resource` in the definitions, a model account is its login Resources, one per account per machine, and neither a Fleet nor a Pool is one (DEC-116).
   ```

   In `spec/README.md`, the rows of `contracts.yaml` (13) and `implementation.yaml` (49), and the
   totals (524: 227 `language`, 246 `convention`, 51 `interface`), which `check_schemas.py`
   checks; and three rows it does not: `core/operations.yaml`, "the setup, retire and sign-in
   hooks" becoming "the setup, retire and sign-in hooks and the admission hook" and 41 contracts;
   `core/results.yaml`, `admission-report.v1` after `limit-status.v1`, "a login's admission
   report" after "an instance's limits" and 15 contracts; `core/`, whose "158 documents exported"
   stays and is now true.
10. **Three cases,** each in its family's `index.yaml` in `id` order, profile `first-build`, from
    `{"recipe":"empty"}`, with neutral names and the titles given. *The guarded project* is the
    files of `requests-login-resource-one-at-a-time` with: `login-holder.v1` stating `admission:
    {levers: /admission, control: '#set-admission.v1'}`, its schema requiring `/admission`, an
    object requiring the three fixed levers, each with all its words as `enum` and no `default`;
    listing under `controls` `#set-admission.v1`, an Operation whose `changes` is `{levers:
    /admission}` and whose `inputSchema` holds `levers` (an object) and a required `revision`;
    `admission-hook.v1` under `requires`; both operations in its `implementation`, each to
    `#resource` beside the three sign-in hooks (TPL-004 rejects a listed control with no
    implementation), and in the `resource` item's `implements`; the template's `config` holding
    `admission: {billing: included, paidUsage: refuse, whenUnknown: refuse}`; and
    `shared-agent.v1` stating `admission: {loginTypes: ['#login-holder.v1']}`. The cases may share
    its files by relative path, as `requests-limits-signals` shares its sibling's, or copy them.
    Each variant below is one change to it: derive them mechanically, and commit only case files.
    - `contracts-admission-shape` (`conformance/language/contracts/admission-shape/`; CTR-015;
      title "A login Type's admission holds its levers and control, an Agent Type's its login
      Types, and no other shape"), resolve-project steps as in
      `contracts-login-resource-misplaced`: the guarded project is accepted; each of six changes
      to it reports invalid with a `ruleId` SUB-005 diagnostic, state unchanged: a further
      Resource Type with no `signIn` stating `admission` as `login-holder.v1` does; a further
      Agent Type stating `signIn`, `requires` and `admission` as `login-holder.v1` does;
      `login-holder.v1` without `admission-hook.v1` under `requires`; `shared-agent.v1`'s
      `admission` also holding `levers`; an empty `loginTypes`; a third member of a login's
      `admission`. Write it first and report its failing line; after decisions 2 and 7 it passes
      on the unchanged engine, which reads the schemas from `spec/`. A step that still fails needs
      a change outside Scope: stop and report its line.
    - `registration-admission-checks` (`conformance/language/registration/admission-checks/`;
      CTR-015; title "A login's levers place and control, an Agent's login Types and a levers
      value are checked"). Resolve steps, each one change to the guarded project reporting invalid
      with a `ruleId` CTR-015 diagnostic, state unchanged: a `default` under `/admission`;
      `whenUnknown` not required; `paidUsage` a string with no `enum`; the control's `changes`
      `{flags: /admission}`, its input renamed; a second listed control whose `changes` is
      `{only: /admission/whenUnknown}`, with the input `only` and a required `revision`, otherwise
      as `set-admission.v1` and mapped to `#resource` as it is; `admission.control` naming a
      control `controls` does not list; `loginTypes` naming `#shared-agent.v1`. Then first start
      and register-project steps as in `registration-login-place`: with `shared-agent.v1` stating
      no `admission`, invalid, CTR-015; with the levers `whenUnknown: admit` beside `paidUsage:
      refuse`, invalid, CTR-015; the guarded project, accepted.
    - `requests-admission-report-kept` (`conformance/conventions/requests/admission-report-kept/`;
      layers `convention` and `interface`; IFC-054; title "A login's admission report is asked
      before model work, kept from its end record and ended by a newer limits report"): the
      guarded project with `requests-login-resource-lapse`'s steps through the login's setup:
      `a` and `b` set up, `call-a` and `call-b` opened, and `login` set up by its run, its
      discovery and its account check, which here answers `example-account`, the expected
      account, so that no sign-in action is raised; then (i) a `turn.v1` call on `a`: nothing
      reaches `a`, and the engine dispatches `admission-hook.v1` to `login` in the form of that
      case's account-hook parts (the reply pending with the hook's invocation in `hook`, then the
      hook's discovery and its call), with `levers` as stored, `limits`
      `{"state":"unknown","windows":[]}` and `previous` null, its `caller` unasserted; (ii) it
      succeeds with R1 (`admit` true, `within-levers`, `lever`, `window`, `cannotTell` and `lifts`
      null, `paidUsage` all null, `observedAt` the step's time, `source` `{client: null, method:
      example-read}`, `validForSeconds` 3600, `singleUse` false): its end record holds R1 in
      `admission`, and `a`'s call is dispatched; (iii) `a`'s call succeeds with no limits member;
      (iv) a call on `b` is dispatched with no hook call; (v) it succeeds with a limits member,
      `usageAllowed` true and `example/all/300m` at 20, kept under IFC-052; (vi) a call on `a`:
      nothing reaches `a`, and the hook is dispatched again with `previous` R1 and `limits` the
      login's status.

    List the last two alone in `conformance/expected-failures.yaml`, in `id` order, with the
    reasons "CTR-015, order 314: the first build makes none of CTR-015's checks; the guard's
    engine order for keeping makes it pass" and "IFC-054, order 314: the first build never calls
    admission-hook.v1 and keeps no admission report; the guard's engine orders for keeping and
    the ask make it pass". `required_promises` names only what the unchanged engine cites. Each
    must first fail where the unchanged engine lacks what it tests: `registration-admission-checks`
    at its first CTR-015 resolve step (the `default` under `/admission`), which that engine
    accepts, and `requests-admission-report-kept` at step (i), where it dispatches `a`'s call and
    no hook. A case that first fails at any other step is wrong: fix the case. Declare all three
    in `docs/work-orders/314-declared.yaml`, in 307's form, then run
    `python3 tools/promise_standing.py --write`.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
for d in 094 116 120 123 124; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1
cat spec/core/*.yaml spec/schemas/*.yaml spec/promises/*.yaml spec/conventions/*.yaml spec/interface/*.yaml spec/*.md spec/core/README.md spec/schemas/README.md | grep -cE 'admission-(hook|report)|loginTypes|CTR-015|IFC-054'  # 0
grep -c '"id": "requests-limits-signals"' conformance/expected-failures.yaml  # 1; 0 once order 315 has merged
k spec/interface/implementation.yaml 'records and every decision are as without the key.' 'No outcome, access, dispatch, hold or lifecycle state depends on it, and an inspection'  # 1 1
k spec/promises/contracts.yaml 'and filters (PND-011), and exposed through no consumer tool.' 'and `toolsDirectory`, the own tools declaration of CTR-014.' "What an Agent's client writes in that Resource's directory is"  # 1 1 1
k spec/promises/pending.yaml 'through no Conduit, never on an implementation notification.'  # 1
k spec/conventions/records.yaml 'under IFC-051 and IFC-052, whatever `records` states.'  # 1
k spec/definitions.md 'each keeping its own state directory and context (CTR-013, PRV-029, PND-005). |'  # 1
k spec/core/results.yaml 'record.v1 REC-002, LIF-013, SEC-015, SEC-018, PRV-024 (no record'  # 1
k spec/core/README.md 'The 40 Operation contracts' '| 14 Schema contracts:' 'exports 158 documents, 80 of them Schema contracts' 'the setup, retire and sign-in hooks, the operations' "an instance's limits, an inventory" 'never beside `signIn`, CTR-013 |'  # 1 1 1 1 1 1
cat spec/core/*.yaml spec/schemas/*.yaml | grep -E '^kind: ' | awk '{n++} /Schema/ {s++} END {print n, s}'  # 156 79
k spec/README.md '522 in all: 226 of the layer `language`, 246 of the layer `convention` and 50 of the layer `interface`' '158 documents exported' 'the setup, retire and sign-in hooks, the operations' 'Draft; 40 contracts' '`limit-status.v1`, `inventory.v1`' 'Draft; 14 contracts'  # 1 1 1 1 1 1
k spec/schemas/README.md 'the latter only for Agent and never beside `signIn` (CTR-013), and states'  # 1
grep -c '^| no case, no reason | 22 | 64 | 19 | 105 |$' engine/STANDING.md  # 1
grep -c '"tool": "logins.set-up.v1"' conformance/conventions/requests/login-resource-lapse/case.yaml  # 1
ls conformance/language/contracts conformance/language/registration conformance/conventions/requests docs/work-orders | grep -cE 'admission|^314-'  # 0
```

Where a line moved, work from where it stands and list it under `own_choices`.

## Scope

Change, as the decisions say: in `spec/core/`, `results.yaml`, `operations.yaml`, `conduit.yaml`
and `README.md`; CTR-004, CTR-013 and CTR-015, PND-011, REC-002, and IFC-052 to IFC-054;
`spec/schemas/contracts.yaml` and its README; `spec/definitions.md`'s one row; `spec/README.md`'s
counts and three `core/` rows; `spec/CHANGES.md`; the three cases, their index entries and the
two expected-failures entries; `docs/work-orders/314-declared.yaml`; `engine/STANDING.md` by the
tool alone. Not the decision log (each decision stays not done), `experiments/`, ACC-058's table,
`engine/` sources, `packages/` or `tools/`. Where a test fails because the engine reads the
changed spec, report it and change no engine file.

## Done when

Each line prints otherwise before this order and passes after, with `j` and `k` as above:

```sh
cat spec/core/operations.yaml spec/core/results.yaml | grep -cE '^  name: admission-(hook|report)\.v1$'  # 2; 0 before
grep -cE '^  - admission-(hook|report)\.v1$' spec/core/conduit.yaml  # 2; 0 before
sed -n '/^  name: admission-report.v1$/,/^---$/p' spec/core/results.yaml | grep -oE 'within-levers|warning|cannot-tell|included-spent|paid-in-use|paid-possible|paid-from-first-token|threshold|model-not-included' | sort -u | paste -sd' ' -  # cannot-tell included-spent paid-from-first-token paid-in-use paid-possible threshold warning within-levers; nothing before
grep -cF 'admission: {$ref: github.com/shrav-k/conduit/core#admission-report.v1}' spec/core/results.yaml  # 1; 0 before
k spec/core/results.yaml "IFC-054 (the end record of a login's admission hook holds its report)"  # 1; 0 before
cat spec/promises/contracts.yaml spec/interface/implementation.yaml | grep -cE '^- id: (CTR-015|IFC-054)$'  # 2; 0 before
k spec/promises/contracts.yaml 'On a Resource Type stating `signIn`' "which reaches only the login's own calls (IFC-006)"  # 1 1; 0 0 before
k spec/interface/implementation.yaml "apart from ending a login's admission report under IFC-054" "but that keeping it may end a login's admission report under IFC-054"  # 1 1; 0 0 before
k spec/promises/contracts.yaml "is the one sign-in Type's hook invoked otherwise" "a login's levers and control or an Agent's login Types, under CTR-015."  # 1 1; 0 0 before
k spec/promises/pending.yaml '`admission-hook.v1` is the one exception (CTR-013, CTR-015)'  # 1; 0 before
k spec/conventions/records.yaml 'call that succeeded also holds its report in `admission`, under IFC-054'  # 1; 0 before
sed -n '/^  name: type-spec.v1$/,/^---$/p' spec/schemas/contracts.yaml | grep -cE '^    admission:$'  # 2; 0 before
k spec/schemas/README.md 'never beside `signIn` (CTR-013), and `admission` (CTR-015)'  # 1; 0 before
k spec/definitions.md 'A model account is represented by its login Resources' 'and reported by its `admission-hook.v1` (CTR-015, IFC-054)'  # 1 1; 0 0 before
grep -cE '^\| `admission-(hook|report)\.v1` \|' spec/core/README.md  # 3; 0 before
k spec/core/README.md 'The 41 Operation contracts' '| 15 Schema contracts:' 'exports 158 documents, 80 of them Schema contracts' 'sign-in hooks and the admission hook' "an instance's limits, a login's admission report" "an Agent's login Types, CTR-015 |"  # 1 1 1 1 1 1; 0 0 1 0 0 0 before
cat spec/core/*.yaml spec/schemas/*.yaml | grep -E '^kind: ' | awk '{n++} /Schema/ {s++} END {print n, s}'  # 158 80; 156 79 before
k spec/README.md '524 in all: 227 of the layer `language`' '158 documents exported' 'sign-in hooks and the admission hook' 'Draft; 41 contracts' '`limit-status.v1`, `admission-report.v1`,' 'Draft; 15 contracts'  # 1 1 1 1 1 1; 0 1 0 0 0 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- '  # 4; 0 before
cat conformance/expected-failures.yaml conformance/language/contracts/index.yaml conformance/language/registration/index.yaml conformance/conventions/requests/index.yaml | grep -cE '"id": "(contracts-admission-shape|registration-admission-checks|requests-admission-report-kept)"'  # 5; 0 before
grep -cE '^\| (CTR-015 \| language \| must \| 2 \| 1|IFC-054 \| interface \| must \| 1 \| 1) \|' engine/STANDING.md  # 2; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. contracts-admission-shape 2>&1 | grep -c "^PASS contracts-admission-shape ")  # 1; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-admission-checks 2>&1 | grep -A1 "^EXPECTED FAILURE registration-admission-checks " | cut -c1-200)  # its EXPECTED FAILURE line, then "  step <n>: ..." for its first CTR-015 resolve step; nothing before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-report-kept 2>&1 | grep -A1 "^EXPECTED FAILURE requests-admission-report-kept " | cut -c1-200)  # its EXPECTED FAILURE line, then "  step <n>: ..." for step (i); nothing before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
n="conformance/language/contracts/admission-shape conformance/language/registration/admission-checks conformance/conventions/requests/admission-report-kept docs/work-orders/314-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # base and head equal; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/314-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
v='claude|codex|anthropic|openai|chatgpt'; a=$(git grep -ohiE "$v" HEAD -- spec | wc -l); b=$(grep -rohiE "$v" spec | wc -l); [ $a -eq $b ] && echo same  # same: no vendor named (DEC-105)
git diff --check  # prints nothing
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/core and spec/schemas
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case line asked for above, each expected failure's first
failing step and every check's last line.

## What follows

The hold's spec order: PND-016 (every call that starts model work on a guarded login checked; the
wait for the one ask, bounded by DL21; the single-use hand-out; the 60-second rule; `lifts`),
`admission-hold.v1`, PND-005's clause, IFC-054's status and "not checked", the ask's caller and
cause, PND-016 among the report case's `promises`, and the definitions row's "and enforced before
each call that starts model work (PND-016)". The warnings' spec order: IFC-054's warning key, the
refusal of an Agent's Conduit listing the control, and the hook and the control in ACC-058's
table. The engine's keeping order removes the checks case's entry; its ask and hold orders remove
the report case's. Then the packages' new Types (DEC-124). No team-seat Agent takes a turn until
the guard merges (DEC-102).

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, stacked on order 307. The usage
guard's spec order S1b (DEC-166 puts the guard's orders first). Written, built, read and fixed in
stage 3 of experiment 028; it is no order of stage 3.

**Worker-1's turns** (16:51 to 17:23). The first stopped rightly: decision 10's invalid variants
needed the conformance runner to turn a schema-invalid submission into a reply, an engine change.
By the coordinator's ruling, open to the owner's strike, those shape checks are schema examples
under `spec/schemas/examples/`, checked by the loader, and the case keeps the accepted project;
decision 10's wording is superseded so. The second stopped rightly on the loader's
invalid-verdicts fixture, which the coordinator added to Scope. The third finished the order.

**Two blind reads** (17:25 to 17:45). The Codex verifier, `root-verifier-1.turn.v1`, had one
should (an example that left out two things at once); the Claude reader (`reader-1`) raised
decision 10 as a must, settled by the ruling above, and two shoulds (what each reason word means;
that levers are declared, not yet held on). One fix turn by worker-1 applied the shoulds. The
gate's run on the squash is the coordinator's, at the merge.
