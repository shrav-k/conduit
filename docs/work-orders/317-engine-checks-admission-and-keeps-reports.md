# Work order 317: the engine checks a login's admission and keeps its reports (DEC-094, DEC-116, DEC-124, the usage guard's second engine order)

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

**The owner's words.** [DEC-094](../decisions/log.md#dec-094-2026-11-23-usage-is-self-reported-by-packages-and-the-engine-trusts-the-report):
"Conduit specifies what must be reported and what contracts must be upheld. The engine then
trusts this reporting".
[DEC-116](../decisions/log.md#dec-116-2026-11-23-a-model-account-is-its-login-resources-limits-are-enforced-there-not-in-fleets),
"Login Resource, as now (Recommended)": a model account is its login Resources, and its limits
are declared and enforced there.
[DEC-124](../decisions/log.md#dec-124-2026-11-23-the-levers-come-on-new-type-versions), "New
Type versions (Recommended)": logins on the old Types have no check, and their status says so.
[DEC-123](../decisions/log.md#dec-123-2026-11-23-a-call-waits-in-the-request-for-a-fresh-usage-report),
"Wait in the request (Recommended)", and
[DEC-120](../decisions/log.md#dec-120-2026-11-23-the-guard-checks-before-each-turn-and-a-started-turn-runs-to-its-end),
"Yes, check before turns only (Recommended)", stand behind the ask and the hold, which the next
engine orders build. [Order 314](314-admission-report-and-hook.md) wrote the spec this order
builds: CTR-015 (a login Type's levers place and control and an Agent Type's
`admission.loginTypes`, checked at install, submission, registration and replacement), IFC-054
(a report kept from its hook's end record until its validity, its single use, a newer limits
member or a lever change ends it), `admission-report.v1` and `admission-hook.v1`; it listed
`registration-admission-checks` and `requests-admission-report-kept` as expected failures. This
order makes CTR-015's checks and IFC-054's keeping and ends, removes the first entry, and shows
each login's admission state in `inspect.v1`, or that calls on it are not checked. It asks for
no report and holds no call.

**Today** (`main` at `1e135e61` with the builds of orders 307, 314 and
[315](315-engine-keeps-new-limit-fields.md) on top), no engine source names CTR-015 or IFC-054.
`Catalog::resolve_at` (`packages.rs`), which install, submission, `resolve-project` and
`replace.v1`'s trial all reach, checks a Type's `signIn`, `loginResource` and `toolsDirectory`
under CTR-013 and CTR-014 and reads no `admission`; the loader already rejects, by
`type-spec.v1` and under SUB-005, the six shapes of order 314's invalid examples
(`spec/schemas/examples/invalid/CTR-015-*`). `registration::prepare` resolves each instance
through `resolve_instance`, which rejects a configuration failing its Type's `schema`, `if` and
`then` included, under INS-014 and leaves it out of `resolved`, and then checks each resolved
instance's login place under CTR-013; `registration::replacement` makes the new instance of
`register-replacement.v1` through the same `resolve_instance`. `requests::complete` keeps a
result's usage and limits members on its end record (`usage::record_end`,
`limits::Report::record_end`), `records::execute` puts `limits::status` in the status
`inspect.v1` shows, and `State` holds `limit_observations`. So `registration-admission-checks`
fails at its step 1, which accepts a `default` under `/admission`, and
`requests-admission-report-kept` at its step 16, which dispatches `a`'s call and no hook.
Control dispatch stores no settings: `engine/CONFORMANCE.md` says so in order 288's paragraph
("Control dispatch stores no settings under REV-007"). No package states `admission`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, a
check here only refuses: a CTR-015 fault rejects an install, a registration, a replacement or a
control, and nothing is granted; 6, no inspection starts a call to fill the status, and no call
is dispatched, held or queued on a report; 7, each report is in its hook's end record, and the
status names the invocation that gave it; 1, the engine checks a report's shape and keeps it as
received, deciding nothing from its words (DEC-094). None at risk: an inspection's status gains
a member, and an invalid project, replacement or control is refused that is accepted today.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
CTR-015 and IFC-054 are musts since order 314, and DEC-102 keeps the team seats' Agents from
turns until the guard is built. By existing practice, a Kubernetes API server validates a
`ValidatingWebhookConfiguration` when it is created, before any request reaches the webhook; and
an HTTP cache counts a stored response's age from when it received it and invalidates it when an
unsafe request to its URI succeeds (RFC 9111, sections 4.2.3 and 4.4), as a lever change ends a
report. Not taken: asking for a report or holding a call here (DEC-123's and DEC-095's orders);
a minimum or clamp on validity; the engine reading or summing limits to decide (DEC-094);
rebuilding a report from records; checking a report's words beyond its schema; a second check of
what `type-spec.v1` rejects.

**When it runs.** It stacks on orders 307, 314 and 315 and runs on 315's build (`main` with the
decision log through DEC-157, and the builds of 307, 314 and 315 on top) until they merge, then
on `main`. Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at
once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what
must hold; how is yours. It is large for one turn, so it may be dispatched in two halves from the
start, each with its own turn, reads, fix turn and gate: 317a, CTR-015's checks (decisions 1 to
3, their three tests, the expected-failures entry and the new section of decision 9 without its
last sentence), and 317b, on 317a's build, keeping, ends, status and store (decisions 4 to 8,
their four tests, the row of decision 9 and the section's last sentence). *In two halves*, after
the Done when, gives each half's Start check and Done when. The coordinator's, open to the
owner's strike (DEC-029): the status built now, ahead of order 316's promise, in the form its
`admission-status.v1` gives; an Agent's status showing its login's report whole, its `lever`
included, to an actor that may inspect the Agent and not the login, as order 316's status text
gives it, with only `invocation` null, reading CTR-015's "Nothing of `admission` reaches an
Agent's implementation" as about the calls an Agent's implementation receives (IFC-006), not
an inspection that access allows; the report kept as engine state beside `limit_observations`,
keyed by the login, that only a newer report replaces; the keeping entered only for an entry
that names no Conduit, and the tests' helper for that entry until the ask exists;
`levers-changed` at the success of a control sent a value at the levers place, as this build
stores no control's values; a report ending `valid-for-ended` at its keep time plus
`validForSeconds`, and the first end kept; the levers value's pair checked in
`resolve_instance`, in INS-014's place, with its instance kept among the resolved; the
control's pair check in this order, before the keyed-repeat conflict, and under CTR-015, as this
build makes no REV-015 check; a replaced instance's current Agents read as those whose login
place refers to it; one diagnostic per place an author wrote, named by the document that writes
the place; the test names, the store format and the rows' words.

**Shared files.** Order 316, the spec of the hold and the status, not yet in the tree, adds
`admission-hold.v1`, `admission-status.v1`, IFC-054's status and five expected failures,
`requests-admission-not-checked` among them, and changes `requests-admission-report-kept`'s
promises; order 305 changes REC-002 and `record.v1`; DEC-149's orders 311 to 313 change core
`requests.rs` and `records.rs`, the shell's store and `engine/CONFORMANCE.md`, and 311 drops
`held_target` from an ended entry, so read the hook's held Type from the entry as `complete` holds
it before `complete_call`, as `limits::Report::take` reads it. Whichever merges later rebases and
reruns `promise_standing.py --write`. Where order 316 merged first (the Start check's fourth
printed line gives 4, and IFC-054's row in its seventh reads `6 | 6`), build decision 7 to its
schema, check each status there against it with `first_core_failure`, remove
`requests-admission-not-checked`'s entry too, and list it under `own_choices`; the Done when
gives that path's values beside today's.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first CTR-004, CTR-006, CTR-013, CTR-015, IFC-052, IFC-053, IFC-054, OPS-007, OPS-010,
OPS-023, REV-006, REV-007, REV-009, REV-015, REC-002, SUB-005, SUB-009, LIF-009, INS-014,
INS-020, OBS-002 and OBS-006, and DL2; the schemas `admission-report.v1` and `limit-status.v1`
(`spec/core/results.yaml`), `admission-hook.v1` and `inspect.v1` (`spec/core/operations.yaml`)
and `type-spec.v1`'s `admission` (`spec/schemas/contracts.yaml`); order 314's decisions and its
Record; and the rows `DL2: storage` and `DL8: records` of `engine/CONFORMANCE.md`, with order
288's paragraph. In the core, `Catalog::resolve_at` and `pointers_overlap` in `packages.rs`;
`resolve_instance`, `prepare`, `replacement`, `diagnostic`, `instance_diagnostic`,
`authored_fault`, `ReportedFault` and `reject` in `registration.rs`; `invoke_inner`, `complete`,
`finish` and `dispatch_hook` in `requests.rs`; `limits.rs` whole; the status in
`records::execute`; `State` in `lib.rs`; the helpers of `login_resource_tests.rs`,
`limits_tests.rs` and `registration_value_replacement_tests.rs`. In the shell, `limits_tests.rs`,
`Store::write` in `store.rs`, and `established`, `open`, `actor` and `assert_memory_equals_disk`
in `tests.rs`. *The guarded project* is the one `contracts-admission-shape` and `requests-admission-report-kept`
register (`conformance/language/contracts/admission-shape/`): its levers place is `/admission`.

1. **A Type's `admission`** (CTR-015, SUB-005, CTR-006). The shapes `type-spec.v1` gives
   `admission`, which order 314's six invalid examples break, stay the schema's: the loader
   rejects a document breaking one under SUB-005 at install and file submission, and
   `register.v1`'s wire rejects it by its input schema, before the core sees it. The core adds no
   second check of them and reads `admission` only from documents that passed. Beside CTR-013's
   and CTR-014's checks of a Type in `Catalog::resolve_at`, it checks what the schema cannot:
   - on a Type stating `signIn` and `admission`: the levers place and every member on its path
     are required by the Type's `schema`; no `default` stands at or under the place; the schema
     there is an object requiring `billing`, `paidUsage` and `whenUnknown`, each confined by
     `enum` to some of its words (`included` and `paid-from-first-token`; `allow`, `refuse` and
     `refuse-if-possible`; `refuse`, `one-trial` and `admit`); `admission.control` resolves to a
     control the Type lists under `controls`; that control's `changes` is exactly
     `{levers: <the levers pointer>}`; and no other listed control has a pointer in its
     `changes` equal to the levers pointer, inside it or over it, decoded and compared as
     `pointers_overlap` compares them for CTR-006;
   - on an Agent Type stating `admission`: each selection under `loginTypes` resolves to a Type
     stating `signIn` and `admission`.

   Read a schema at a pointer as the CTR-013 and CTR-014 checks read one (`binding_parts`), its
   references followed. Each fault is one error diagnostic, `ruleId` CTR-015, whose text names
   the Type and the cause (for `loginTypes`, the Type listed too), located at its document as
   those checks' are, and the install or submission is rejected as it is for them. Each of the
   seven `resolve-project` steps of `registration-admission-checks` breaks one rule and expects
   exactly one diagnostic. The guarded project is accepted.
2. **An Agent's login Type and a login's levers value** (CTR-015, INS-014, LIF-009, INS-020).
   For `register.v1`, the register-project entry and `replace.v1`'s trial alike, registration
   also checks:
   - (a), where `registration::prepare` checks each resolved instance's login place under
     CTR-013: an Agent whose login place resolves to a Resource whose Type states `admission`
     lists that Type under its own Type's `admission.loginTypes`, compared as qualified
     references, each resolved in its own Type's package, or else a fault, at the login place,
     names CTR-015, the Agent's Type and the login's Type;
   - (b), in `resolve_instance`, on the configuration it fills and before its Type-schema check
     (`failure_at`), for `prepare` and `replacement` alike: a Resource whose Type states
     `admission` has at its levers place no `whenUnknown` `admit` beside `paidUsage` `refuse` or
     `refuse-if-possible`, or else a fault, at the levers place, names CTR-015, the Type and both
     pointers (`/admission/whenUnknown` and `/admission/paidUsage` in the guarded project).

   (b) runs there because the guarded `login-holder.v1` states the same rule as `if` and `then`
   in its `schema`, which `spec/schema-profile.md` allows and `failure_at` evaluates: a check
   after resolution never sees the pair, as INS-014 rejects the login first and leaves it out of
   `resolved`, and its Agents then also get CTR-013's "does not resolve to a login Resource".
   The pair is reported as the one CTR-015 fault, in place of INS-014's for that value; INS-014's
   observable asks only for a diagnostic naming the Type and what fails, which this one gives. A
   login whose configuration fails only by the pair still counts as resolved for the CTR-013
   loop and for (a), so that its Agents get no CTR-013 fault: one way is that a pair fault does
   not drop its instance from `resolved`. Another fault of the same configuration is reported as
   today. List the way under `own_choices`.

   Report each fault through `instance_diagnostic` with the fault at its configuration pointer,
   as the login-place faults are, but build it with the name of the document that writes the
   place: the template's (the source's `template_name`) where the template writes the value, and
   the instance's where its own declaration does, as an argument, as `authored_fault` names its
   place's document. `diagnostic` writes the text as `{name}: {text}`, and `reject`'s dedup
   compares whole `ReportedFault`s, so faults built with the names `a` and `b` would stay two.
   Built so, the instances one template makes give one diagnostic at the place the template
   writes, as the case's register steps expect: exactly one in step 9, for `a` and `b` from
   `worker`, and exactly one in step 10, for `login` and `gone` from `login-holder`. List the way
   under `own_choices`.

   In `registration::replacement`, at the preflight and the dispatch of
   `register-replacement.v1` alike, the new Resource of a Pool's entry is checked under (b),
   through its `resolve_instance`, so its reply names CTR-015, not INS-014, and, where its Type
   states `admission`, under (a) for every Agent, not retired, whose login place refers to the
   instance it replaces: an Agent keeps the ID it resolved, so these are "a current Agent of
   it", the nearest sound reading. No Agent is replaced in this build: `register-replacement.v1`
   makes none (ACC-057), and `replace.v1` changes no instance (INS-020). A login whose Type
   states no `admission`, and its Agents, are checked as today.
3. **A control that would store the pair** (CTR-015, REV-006, REV-015, OPS-023). A request for
   a control on a Resource whose Type states `admission`, where the values it was sent, applied
   at their `changes` pointers to the stored configuration (a place whose input was not supplied
   left as stored, REV-006), would leave at the levers place `whenUnknown` `admit` beside
   `paidUsage` `refuse` or `refuse-if-possible`, is not dispatched and reports invalid, with one
   error diagnostic, `ruleId` CTR-015, naming both pointers. Decide it in `invoke_inner` after
   access and before the keyed-repeat conflict (OPS-010, OPS-032), where the first
   `sign_in::preflight` and `answer.v1`'s invalid check stand: OPS-023 puts invalid before
   conflict, so a request that reuses a key with other arguments and holds the pair reports
   invalid. This build makes no REV-015 check: no engine source names it, and
   `engine/STANDING.md` gives it no case and no reason. CTR-015's observable says that, where the
   package's schema states the rule as `if` and `then`, as the guarded one does, REV-015 rejects
   the pair first; until REV-015 is built, CTR-015's check reports it, whatever the schema
   states. List that under `own_choices`. Nothing else about controls changes: REV-007's storing
   stays as this build has it, which stores nothing.
4. **Keeping** (IFC-054, REC-002, OPS-007, DEC-121). The engine's call of `admission-hook.v1` on
   a login is an invocation entry of that core operation that names no Conduit, whose target's
   Type, held at dispatch, states `admission`. When it ends succeeded, the record of its end holds
   its output `admission`, unchanged, in `data.admission`, whatever the Type's `records` states,
   set on the new record and its stored copy as `Report::record_end` sets `limits`; and the
   engine keeps that report as the login's newest, in a new `State` member, `admission_reports`,
   keyed by the login's ID, so each login of an account, one per machine, keeps its own. An entry
   holds the report, the invocation, `keptAt`, that record's `time`, and the end that ended it,
   if any; a newer report takes its place, ended or not. A call that failed or ended unknown, an
   output off `admission-report.v1` among them, which OPS-007 already ends unknown, keeps
   nothing: its record holds no `admission`, and the earlier report stays as it was. A call of
   the operation through a Conduit keeps nothing; none can list it, as it is in no capability. No
   transition starts the engine's call yet: the ask is DEC-123's order's, and the tests put its
   entry in place (decision 10). No purge, `expire-outputs`, retention or retirement changes a
   kept report. Every other record, reply and retained result is as today.
5. **Validity and single use** (IFC-054). A kept report that is not single use is current from
   `keptAt` until `validUntil`, `keptAt` plus `validForSeconds` seconds on the engine's own
   clock, the transition's evidence time, and has ended `valid-for-ended` from that instant on.
   Compare instants as `limits.rs` does (`instant`: `chrono`, then the digits past the ninth
   fractional one), never as strings, and never from `observedAt`. A single-use report
   (`singleUse` true, `validForSeconds` 0) has no `validUntil` and stays current until it admits
   its one call or ends early; no call is admitted on a report yet, so no transition here ends
   one `single-use`, though an entry can hold that end. The first end ends a report for good: an
   early end that comes at or after `validUntil` leaves `valid-for-ended`.
6. **The two early ends** (IFC-054, IFC-052, REV-007). A current report ends:
   - `newer-limits`, when the engine keeps a limits member under IFC-052 in the end record of a
     call to the login, or to an Agent whose login place, held at dispatch, refers to it, other
     than the hook's call that gave the report: in `limits::Report::record_end`, for each login
     among the IDs it keeps the member for, the call's target where that is a login Resource or
     the Agent's login. A member IFC-052 drops ends nothing.
   - `levers-changed`, when a control on the login succeeds, other than by reconciliation, that
     was sent a value for an input whose `changes` pointer is at or under the levers place: an
     input its invocation's `control_inputs` names, as that holds the inputs of `changes` the
     request supplied. REV-007 says that such a success stores that value; this build stores
     none, so the success is the nearest sound reading. A control sent no such value, as
     `set-admission.v1` sent `revision` alone, stores nothing at the place (REV-006) and ends
     nothing, and nor does a control that fails or ends unknown.

   An end adds no record and changes no reply, and the ended report stays the newest until a
   newer one is kept.
7. **The status** (IFC-054, DEC-124, OBS-002, OBS-006). Ahead of order 316's promise, and in the
   form its `admission-status.v1` gives, `inspect.v1`'s status gains `admission`, built in
   `records::execute` beside `limits::status`, for an instance, not a template, as `limits.rs`'s
   `eligible` reads one: for a Resource whose Type states `signIn` (as
   `inventory::instance_type` reads it), its own; for an Agent whose login place refers to an
   existing one, its login's. Where the login's Type states no `admission`, it is
   `{"checked":false}` alone (DEC-124). Otherwise it holds `checked` true; `report`, the newest
   kept, ended or not; `invocation`, the call that gave it; `keptAt`; `validUntil`, null for a
   single-use report; `state`, `current` or `ended`, or `none` where none is kept, with
   `report`, `invocation`, `keptAt`, `validUntil` and `ended` null; `ended`, the end that ended
   it, null unless `state` is `ended`; and `asking` and `noReport`, null until the ask's order
   fills them. In an Agent's status `invocation` is null where the inspecting actor may not
   inspect the login (`access::may_inspect`), and `report` is shown whole, its `lever` included,
   as order 316's status text gives it: this is the coordinator's reading of CTR-015's "Nothing
   of `admission` reaches an Agent's implementation" (When it runs), and the two schemas give no
   other form, as `admission-report.v1` makes `lever` an object for every reason but
   `within-levers` and order 316's `admission-status.v1` makes `report` an object while `state`
   is `current`. After a report R kept by invocation I at 13:01:00 with `validForSeconds` 3600,
   inspected before 14:01:00, it is:

   ```text
   {"checked":true,"report":R,"invocation":I,"keptAt":"2026-11-21T13:01:00Z",
    "validUntil":"2026-11-21T14:01:00Z","state":"current","ended":null,"asking":null,
    "noReport":null}
   ```

   `validUntil` is written in UTC with `Z`, in whole seconds where `keptAt` is. No other object's
   status holds `admission`, and `limits` and the rest of the status are as today. An inspection
   starts no call and changes no report (OBS-006); `current` and `ended` are read at its
   evidence time.
8. **The store's format** (DL2). `admission_reports` is a `BTreeMap` in `State` with
   `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]`, as `limit_observations` is: a
   store an earlier engine wrote reads as one that keeps no report, and a state that keeps none is
   written byte for byte as today, so memory equals disk and the format version stays 1. No
   package's Type states `admission` (the Start check's fifteenth printed line), so no store but
   one a test made holds a report.
9. **The case and the rows.** Remove `registration-admission-checks`'s entry from
   `conformance/expected-failures.yaml`; `requests-admission-report-kept`'s stays, as it waits for
   the ask; change no case. In `engine/CONFORMANCE.md`, wrapping at the file's width:
   - In the row `DL8: records`, on its one line, after "no access, dispatch or state depends on a
     signal (DEC-094)." add: "From order 317, the end record of the engine's call of a login's
     `admission-hook.v1` that succeeded holds its report in `admission`, and the engine keeps that
     report, as its own state that no purge shortens, as the login's newest, with the record's
     `time` as its keep time, which is current until its validity passes, a later call to the
     login or to one of its Agents keeps a limits member, or a control sent a value at its levers
     place succeeds, and stays the newest, ended, until a newer one is kept; `inspect.v1` shows
     it, for the login and its Agents, in `admission`, or `checked` false where the login's Type
     states no `admission` (IFC-054, DEC-124)." In its evidence cell, after
     "`limits_signals_read_from_an_earlier_engines_store` in" and its link and comma, add:
     "`admission_report_kept_with_its_validity`, `admission_report_ended_early` and
     `admission_status_shows_the_login_or_not_checked` in [admission_tests.rs] (link to
     crates/core/src/admission_tests.rs), `admission_reports_read_from_an_earlier_engines_store`
     in [admission_tests.rs] (link to crates/shell/src/admission_tests.rs),".
   - At the end of the file, a section `## A login's levers and admission reports (order 317)`
     whose one paragraph begins with "From order 317," and reads: "From order 317, install,
     submission and `replace.v1` check a Type's `admission` beyond the shape `type-spec.v1` gives
     it, which the loader rejects under SUB-005: a login Type's levers place, required on its
     path, with no `default` at or under it, requiring the three fixed levers, each confined by
     `enum`; its control, listed, changing exactly that place, with no other listed control
     overlapping it; and an Agent Type's `loginTypes`, each a Type stating `signIn` and
     `admission`. Registration and `register-replacement.v1` check that an Agent's Type lists its
     login's Type and that no levers value holds `whenUnknown` `admit` beside a refusing
     `paidUsage`, which they report under CTR-015 in INS-014's place, and a control that would
     store one is not dispatched, under CTR-015 as this build makes no REV-015 check (CTR-015,
     `registration-admission-checks`). The engine starts no call of `admission-hook.v1` yet and
     holds no call on a report: the ask, the hold and the warnings are later orders' (DEC-123,
     DEC-095, DEC-099). As control dispatch stores no settings (above), a control sent a value at
     the levers place ends the login's report when it succeeds, and the stored levers stay as
     registered."

   Write each "[text] (link to target)" as a real Markdown link: the text in square brackets,
   then at once the target, relative to `engine/`, in parentheses. Then run
   `python3 tools/promise_standing.py --check`, and `--write` where it fails.
10. **Tests.** Write them first, run them on the otherwise unchanged code and report each line.
    Core tests go in a new `engine/crates/core/src/admission_tests.rs`, declared in `lib.rs` as
    `limits_tests` is, with the helpers of `login_resource_tests.rs` (`context`, `project`,
    `run_case`, `invoke`, `complete`, `subject`). *The report state* is
    `requests-admission-report-kept`'s through its step 15 (`run_case` with the guarded project
    and 15), in which `a`, `b` and `login` are set up and `call-a` and `call-b` opened. *Ask* is
    a helper of yours there that puts in place an in-progress `admission-hook.v1` entry on
    `login` as the ask's order will dispatch it: the engine's own, naming no Conduit, serving or
    filter, its `held_target`, `contract` and `implementation` taken as `dispatch_hook` takes a
    hook's, its discovery settled; complete it with `Input::Complete`. R1 is the report of that
    case's step 18 with `observedAt` `2026-11-21T12:00:00Z`; times are on 2026-11-21 in UTC.
    `invoke` and `complete` build their evidence with `evidence`, whose time is always
    13:00:00: build each timed ask, call, completion and inspection below as `Input::Invoke` or
    `Input::Complete` with an `Evidence` of its own at its time.
    - `admission_types_checked_at_install_and_submission` (CTR-015; decision 1):
      `Input::ResolveProject` of each of the seven `resolve-project` fixtures of
      `registration-admission-checks` reports invalid with exactly one diagnostic, `ruleId`
      CTR-015 and level error, whose text names the Type at fault, `login-holder.v1`, or
      `shared-agent.v1` and the Type it lists for `agent-login-type-`, and the cause; `valid-` is
      accepted. A fixture on which another rule also fires is a step needing a change outside
      Scope: stop and report its line. A package archive built in the test with `tar` and
      `flate2`, as the conformance crate's `client_archive` builds one, whose contracts are the
      guarded project's with `default-`'s `login-holder.v1`, reports invalid at `install.v1` with
      one CTR-015 diagnostic and leaves `installed_packages` as it was; with the guarded
      project's own, it installs. Today each is accepted.
    - `admission_checked_at_registration_and_replacement` (CTR-015; decision 2): the case's
      first start (step 8), then its steps 9 to 11 through `Input::RegisterProject`:
      `missing-agent-admission-` reports invalid with exactly one CTR-015 diagnostic naming
      `shared-agent.v1` and `login-holder.v1`, `invalid-pair-` with exactly one naming
      `/admission/whenUnknown` and `/admission/paidUsage`, and no INS-014 or CTR-013 diagnostic,
      and `valid-` is accepted. Then, in a project of yours made from the guarded one, keeping
      `login-holder.v1`'s `if` and `then`, so that the replacement's pair takes the path the
      registration's takes, with a Pool whose `entries` hold `seat` from a template under
      `login-holder.v1` and an Agent of `shared-agent.v1` whose login place refers to `seat`'s
      Resource, once that Resource is retired, as `retire_nested` in
      `registration_value_replacement_tests.rs` retires one: after `replace.v1` gives `seat`'s
      template a Type of yours, a copy of `login-holder.v1` under a name `shared-agent.v1` does
      not list, `register-replacement.v1` for `seat` reports invalid with one CTR-015 diagnostic
      naming both Types; after `replace.v1` gives it `login-holder.v1` again with `whenUnknown`
      `admit` beside `paidUsage` `refuse`, one naming both pointers, and no INS-014; and after
      `replace.v1` gives it the guarded levers again, it succeeds. A refused replacement makes
      nothing, so the retired Resource stays the instance replaced, and the Agent, which still
      refers to it, its current Agent. Today `missing-agent-admission-` and the replacement under
      the unlisted Type are accepted, and `invalid-pair-` and the replacement with the pair
      report INS-014.
    - `admission_control_pair_refused_before_dispatch` (CTR-015, REV-006, OPS-023; decision 3):
      in *the control state*, the report state of a project of yours, the guarded one with
      `login-holder.v1`'s `if` and `then` taken out of its `schema`, so that the diagnostic the
      test pins stays CTR-015's once REV-015 is built, and with a Conduit from root to `login`
      listing `#set-admission.v1`, its Type offering a capability that lists it, as
      `limits_tests.rs`'s `with_read` adds `read-login` (`run_case` with that project and 15):
      `set-admission.v1` with `levers` `{billing: included, paidUsage: refuse, whenUnknown:
      admit}` and `login`'s revision is not dispatched (no external intent), reports invalid
      with one CTR-015 diagnostic naming both pointers, and leaves `state.invocations` as it was;
      with `paidUsage` `allow` beside `whenUnknown` `admit`, sent with a key, it is dispatched;
      and the first's arguments sent with that key report invalid with the same diagnostic, not
      conflict. Today the first is dispatched, and the keyed one reports conflict under OPS-010.
    - `admission_report_kept_with_its_validity` (IFC-054, OPS-007; decisions 4, 5 and 8): in the
      report state, an ask at 13:01:00 completed with R1 (outcome succeeded, `structuredContent`
      `{admission: R1}`): its end record and that record's stored copy hold R1 in
      `data.admission`, though `login-holder.v1` states no `records`; root's
      `logins.inspect.v1` on `login` at 14:00:59 shows the status of decision 7's example, with R1
      and the ask, and at 14:01:00 the same with `state` `ended` and `ended` `valid-for-ended`,
      though R1's `observedAt` is 12:00:00. An ask at 14:02:00 whose `admission` lacks
      `singleUse` ends unknown, and one at 14:03:00 fails: neither end record holds `admission`,
      and the status is as at 14:01:00. An ask at 14:05:00 completed with R2, single use (`admit`
      true, `reason` `cannot-tell`, `cannotTell` `never-observed`, `lever`
      `{pointer: /admission/whenUnknown, value: one-trial}`, `validForSeconds` 0, `singleUse`
      true; the engine checks the shape alone): R2 is the `report`, `validUntil` is null and
      `state` is `current` at `2026-11-22T14:05:00Z`. A `purge.v1` that removes every ask's end
      record, as `limits_status_outlasts_a_purge_and_takes_the_later_of_equal_times` makes one,
      and a serde round trip of the state leave the status as it was. Today no end record holds
      `admission` and no status holds one.
    - `admission_report_ended_early` (IFC-054, IFC-052, REV-006, REV-007; decision 6): in the
      control state, R1 kept by an ask at 13:01:00: (i) a call on `a` with no limits member, and
      one on `b` whose member IFC-052 drops (its `paidUsage` holds an `amount`, as order 315's M4
      does), leave `state` `current`; (ii) a call on `b` at 13:02:00 whose member is kept,
      `usageAllowed` true and `example/all/300m` at 20 as the case's step 21 gives it, leaves
      `state` `ended`, `ended` `newer-limits` and R1 the `report`; (iii) an ask at 13:03:00 whose
      result gives R1 with `observedAt` 13:03:00 and a limits member that is kept: `current`;
      (iv) `set-admission.v1` with valid levers that fails, then one that ends unknown, then one
      sent `revision` alone, with no `levers`, that succeeds: `current`; one sent valid levers
      that succeeds at 13:04:00: `ended` `levers-changed`; (v) an ask at 13:05:00 giving R1 with
      `validForSeconds` 60, then at 13:07:00 a call on `b` whose member is kept: `ended`
      `valid-for-ended`, the first end. Today no status holds `admission`.
    - `admission_status_shows_the_login_or_not_checked` (IFC-054, DEC-124, OBS-006; decision 7):
      in the report state before any ask, root's `logins.inspect.v1` on `login` and
      `workers.inspect.v1` on `a` each show `admission` with `checked` true, `state` `none` and
      its seven other members null; after R1 at 13:01:00, both show R1 and the ask, and, once
      root has opened `observer-scope` with `access.open.v1` (the report state opens only
      `call-a` and `call-b`), `observer`'s inspection of `a` through it shows the same with
      `invocation` null and R1 whole; no inspection dispatches anything or changes
      `admission_reports`. In `requests-login-resource-one-at-a-time`'s state through its step
      10, whose `login-holder.v1` states no `admission`, `login`'s and `a`'s `admission` are
      `{"checked":false}`. A Resource whose Type states no `signIn`, a template, and an Agent
      that names no login place show no `admission`. Today no status holds it.
    - In a new `engine/crates/shell/src/admission_tests.rs`, declared in `tests.rs` as
      `limits_tests` is, `admission_reports_read_from_an_earlier_engines_store` (DL2; decision 8):
      in the store `established` leaves (s1's steps 1 to 13), in a temporary directory,
      `state.json` holds no `admission_reports` member, and still none after a further commit.
      Close the shell; read `state.json` as JSON, add an `admission_reports` member holding one
      entry, keyed by an ID of yours, with R1, an invocation ID of yours, a `keptAt` and no end;
      write it with `Store::write`, as `limits_signals_read_from_an_earlier_engines_store` does,
      and open the store twice: each time it opens, the entry reads back equal in memory and in
      `state.json`, and memory equals disk (`assert_memory_equals_disk`). Write it against JSON,
      so that it compiles today and fails, as today's `State` refuses the unknown member.
    - An existing test changes only where it compares a whole status of a Resource whose Type
      states `signIn` or of an Agent that names a login place, which now gains `admission`; none
      is expected to. Report each.

## Start check

From the repository root:

```sh
k() { f=$1; shift; for p in "$@"; do grep -cF -- "$p" "$f"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 094 116 120 123 124; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1 1
cat spec/promises/contracts.yaml spec/interface/implementation.yaml | grep -cE '^- id: (CTR-015|IFC-054)$'  # 2
cat spec/core/results.yaml spec/core/operations.yaml | grep -cE '^  name: admission-(report|hook|hold|status)\.v1$'  # 2; 4 once order 316 has merged
grep -cE '"id": "(registration-admission-checks|requests-admission-report-kept|requests-limits-signals)"' conformance/expected-failures.yaml  # 2
grep -rlE 'CTR-015|IFC-054|admission-hook|admission_report' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '  # 0
grep -cE '^\| (CTR-015 \| language \| must \| 2 \| 1 \| outside \| none \| case passes|IFC-054 \| interface \| must \| (1 \| 1|6 \| 6) \| outside \| none \| expected failure) \|$' engine/STANDING.md  # 2
k engine/CONFORMANCE.md 'Control dispatch stores no settings under REV-007' 'and no access, dispatch or state depends on a signal (DEC-094). |' 'From order 317,'  # 1 1 0
k engine/crates/core/src/packages.rs 'pub(crate) fn resolve_at(' 'let login = d["spec"]["loginResource"]["place"].as_str();'  # 1 1
k engine/crates/core/src/registration.rs 'let Some(place) = object["login_resource_place"].as_str() else {' 'pub(crate) fn replacement(' 'format!("configuration fails Type {q}: {failure}"),' '"message":{"text":format!("{name}: {text}")}'  # 1 1 1 1
k engine/crates/core/src/requests.rs 'fn invoke_inner(' 'sign_in::preflight(&t.state, &route, request, &a.evidence.time, c)?' 'crate::limits::Report::take(&mut completion, &v["held_target"], c),' '// OPS-023: an unsupported answer is invalid even if the key conflicts.' 'invocation["control_inputs"] = json!('  # 1 2 1 1 1
k engine/crates/core/src/limits.rs 'keep_latest(&mut observations.newest, newest.clone());' 'fn eligible(instance: &Value, c: &Context) -> bool {'  # 1 1
k engine/crates/core/src/records.rs 'if let Some(limits) = crate::limits::status(&t.state, &o, &a.evidence.time, c) {'  # 1
k engine/crates/core/src/lib.rs 'pub limit_observations: BTreeMap<String, limits::Observations>,' 'mod limits_tests;'  # 1 1
grep -rlE 'admission-hook|loginTypes' packages | wc -l | tr -d ' '  # 0
for f in engine/crates/core/src/admission.rs engine/crates/core/src/admission_tests.rs engine/crates/shell/src/admission_tests.rs; do test -e "$f"; echo $?; done | paste -sd' ' -  # 1 1 1
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it, but for the fourth, whose 4
means order 316 merged first: then follow the Shared files paragraph. The seventh gives 2 on
either path, as IFC-054's row reads `1 | 1` today and `6 | 6` once order 316 has merged. A
third printed line other than `2` means order 314's
spec is not in the tree; a fifth other than `2`, that 314's or 315's expected failures moved; a
sixth other than `0`, that an engine source already names the guard; a ninth to fourteenth
other than stated, that the code moved since this order was written; a fifteenth other than
`0`, that a package states `admission`, and decision 8 no longer holds. Where it is dispatched
in two halves, 317b's worker runs this check on 317a's build as *In two halves* says.

## Scope

Change, for decisions 1 to 8 and their tests, in `engine/crates/core/src/`: `admission.rs` (new,
the keeping, the ends and the status), `admission_tests.rs` (new), `lib.rs` (the `State` member
and the two `mod` lines), `packages.rs`, `registration.rs`, `requests.rs`, `limits.rs`,
`records.rs`, and `explain.rs` only where a CTR-015 reason needs its text; in
`engine/crates/shell/src/`, `tests.rs` (its `mod` line) and `admission_tests.rs` (new); the one
entry of `conformance/expected-failures.yaml`; `engine/CONFORMANCE.md`'s row `DL8: records` and
its new section; and `engine/STANDING.md` by the tool alone. Change an existing test only under
decision 10. Change nothing else: not `spec/`, a case, `docs/` (DEC-094, DEC-116, DEC-120,
DEC-123 and DEC-124 stay not done), the loader, the conformance crate, the shell's other sources
(`store.rs` and `install.rs` among them), `packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`.
No promise changes, so `spec/CHANGES.md` gains no line. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `k` as in the Start check; the
second, guards, passes today and after. Run each existing test you changed by name, once, too.
Run the core's tests and then the whole vectors test, once each, last; never the whole engine
suite.

```sh
ok='^test result: ok\. 1 passed'
for t in admission_types_checked_at_install_and_submission admission_checked_at_registration_and_replacement admission_control_pair_refused_before_dispatch admission_report_kept_with_its_validity admission_report_ended_early admission_status_shows_the_login_or_not_checked; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 six times; 0 today
(cd engine && cargo test --offline --release -p conduit-shell --lib admission_reports_read_from_an_earlier_engines_store 2>&1 | grep -c "$ok")  # 1; 0 today
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-admission-checks 2>&1 | grep -c "^PASS registration-admission-checks ")  # 1; 0 today
grep -c '"id": "registration-admission-checks"' conformance/expected-failures.yaml  # 0; 1 today
k engine/CONFORMANCE.md 'From order 317,' '`registration-admission-checks`' '(crates/core/src/admission_tests.rs),'  # 2 1 1; 0 0 0 today
grep -cE '^\| (CTR-015 \| language \| must \| 2 \| 0 \| outside \| core(, shell)? \| case passes|IFC-054 \| interface \| must \| (1 \| 1 \| outside \| core(, shell)? \| expected failure|6 \| 5 \| outside \| core(, shell)? \| case passes)) \|$' engine/STANDING.md  # 2; 0 today. IFC-054's row reads 1 | 1, or, where order 316 merged first, 6 | 5 and case passes
grep -c '"id": "requests-admission-not-checked"' conformance/expected-failures.yaml  # 0; 1 today where order 316 merged first, else 0 before and after
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-not-checked 2>&1 | grep -c "^PASS requests-admission-not-checked ")  # 1 where order 316 merged first, 0 today; else 0 before and after, as the case is not in the tree
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s conduit-core login_resource_ 10  # 1
s conduit-core limits_ 8  # 1
s conduit-shell limits_signals_read_from_an_earlier_engines_store 1  # 1
s conduit-shell memory_equals_disk_after 5  # 1
(cd engine && cargo test --offline --release -p conduit-loader --test examples 2>&1 | grep -c '^test result: ok\.')  # 1: type-spec.v1 still rejects order 314's six shapes
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-admission-report-kept 2>&1 | grep -A1 '^EXPECTED FAILURE requests-admission-report-kept ' | sed -n 2p | cut -c1-10)  # "  step 16:", as today: it waits for the ask; so too where order 316 merged first, as its decision 10 keeps that step
for p in contracts-admission-shape contracts-login-resource- contracts-sign-in- registration-login-place requests-login-resource- requests-limits- requests-usage-kept requests-reconcile- change-unknown-setting records-purge- secret-reads-; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", N in order: 1 1 2 1 2 4 1 8 1 2 11
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
n="engine/crates/core/src/admission.rs engine/crates/core/src/admission_tests.rs engine/crates/shell/src/admission_tests.rs"; git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, twice
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(admission|admission_tests|lib|packages|registration|requests|limits|records|explain)\.rs|engine/crates/shell/src/(tests|admission_tests)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/expected-failures\.yaml)$' | wc -l | tr -d ' '  # 0
git reset -q -- $n  # stage nothing
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Of order 314's expected failures, `registration-admission-checks` flips to pass and its entry
goes; `requests-admission-report-kept` stays an expected failure, first failing at step 16 as
today, until the ask's and the hold's orders. Where order 316 merged first,
`requests-admission-not-checked` flips too.

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code, then its passing line; `registration-admission-checks`'s line before and after,
and `requests-admission-report-kept`'s first failing step after; each existing test you changed,
and why; your own choices; and every check's last line. Stage nothing.

**In two halves.** Where the order is dispatched as 317a and 317b, each half changes only the
files of its own decisions within Scope and runs this Start check and Done when with these
differences.

- 317a: decisions 1 to 3, the first three tests of decision 10 in a new `admission_tests.rs`
  declared in `lib.rs`, the expected-failures entry of decision 9 and its new section without
  the last sentence ("As control dispatch stores no settings …"); no `admission.rs`, no shell
  test, and no source naming IFC-054. In its Done when, these lines replace the first loop, the
  shell test's line, the CONFORMANCE and STANDING lines and `n`; the lines for
  `requests-admission-not-checked` are 317b's. `git add -N` adds nothing where a path it names
  does not exist, so its `n` names one file.

  ```sh
  for t in admission_types_checked_at_install_and_submission admission_checked_at_registration_and_replacement admission_control_pair_refused_before_dispatch; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; 0 today
  k engine/CONFORMANCE.md 'From order 317,' '`registration-admission-checks`' '(crates/core/src/admission_tests.rs),'  # 1 1 0; 0 0 0 today
  grep -cE '^\| CTR-015 \| language \| must \| 2 \| 0 \| outside \| core \| case passes \|$' engine/STANDING.md  # 1; 0 today
  n="engine/crates/core/src/admission_tests.rs"; git add -N -- $n
  ```

- 317b, on 317a's build: decisions 4 to 8, the other three core tests in 317a's
  `admission_tests.rs`, the shell test, the row `DL8: records` and the section's last sentence,
  and, where order 316 merged first, the removal of `requests-admission-not-checked`'s entry.
  Its Start check's fifth to eighth and sixteenth lines, and its ninth to fourteenth where 317a
  changed those files, print what 317a left (about: `1`; more than `0`; `1`, CTR-015's row
  reading `2 | 0 | outside | core | case passes`; `1 1 1`; `1 0 1`): report them as they print
  and go on; every other line holds as stated. Its Done when's first loop runs its three core
  tests (1 three times; 0 before), its CONFORMANCE line gives `2 1 1` (`1 1 0` before) and its
  STANDING line `2` (`1` before), and the two lines for `registration-admission-checks` are
  guards that print `1` and `0` before and after.

## What follows

- **The ask and the wait** (DEC-123), the next engine order: it dispatches the engine's call of
  `admission-hook.v1` that decision 4 keeps, in place of the tests' helper, with `levers`,
  `limits` and `previous`; one ask per login, DL21, the 60 seconds after an ask that gave no
  report, and the single-use hand-out, which sets the `single-use` end; `asking` and `noReport`
  in the status; a report from an ask in progress when the levers change, kept already ended
  (order 316); and `engine/CONFORMANCE.md`'s DL3 and DL4 rows for this wait. With the hold's
  order, it removes `requests-admission-report-kept`'s entry.
- **The hold and the warnings** (DEC-095, DEC-099, DEC-119): the hold after `hold_work` with
  `admission-hold.v1`; a warned call's result key; and the refusal of an Agent's Conduit listing
  the control.
- **Found: a control stores nothing.** This build stores no control's values (REV-007; order
  288's paragraph of `engine/CONFORMANCE.md`), so a `set-admission.v1` sent levers that succeeds
  ends the report here but leaves the stored levers, which the ask will send the hook, as
  registered. Tuning a login from an Interface's client (DEC-119) needs REV-007's storing at the
  levers place before the sitting (DEC-124): the coordinator orders it or asks the owner.
- **Found: no REV-015 check.** This build checks no control's values against the Type's schema
  before dispatch (REV-015, no case and no reason in `engine/STANDING.md`), so CTR-015's check
  reports the control's pair here. Once REV-015's check is built, it rejects the pair first
  where the package's schema states the rule as `if` and `then`, as CTR-015's observable says;
  the control test, on a schema without them, stays as it is.
- **The packages** (DEC-124): `codex-login.v2`, `account-home.v2` and the Agent Types' next
  versions state `admission`, once the guard's engine orders merge. The CLI then shows a login's
  `admission` beside its limits.
- DEC-094, DEC-116, DEC-120, DEC-123 and DEC-124 stay not done until the guard's orders merge,
  and no team-seat Agent takes a turn before then (DEC-102).

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, stacked on orders 307, 314 and 315.
The usage guard's engine order E1a (DEC-166), dispatched in two halves as *In two halves* says.
Written, built, read and fixed in stage 3 of experiment 028; it is no order of stage 3.

**Worker-1's turns.** 317a (18:56 to 19:19) built CTR-015's checks; 317b (20:50 to 21:15), on
317a's build, kept each report with its ends, the status and the store member. The owner confirmed
the status form (DEC-167) and that a lever change holds no call (DEC-169).

**Two blind reads**, of both halves together. The Codex verifier, `root-verifier-1.turn.v1`, had
two musts (a pair fault attributed to the template, a filter hiding unrelated schema failures);
the Claude reader (`reader-1`) had no must, a should (name the lever's words in the refusal) and
three nits. One fix turn by worker-1 applied them, each must with a test first. The gate's run on
the squash is the coordinator's, at the merge.

**At the merge.** On its rebase onto a `main` holding order 278, the core did not compile: 278
writes the state through a mirror whose exhaustive pattern names every field, so that a new field
is never silently left off disk, and this order's `admission_reports` was not in it. The
coordinator added the field to the pattern, the mirror and its construction, in the state's own
order (ruling 87); clippy and the core's admission tests passed before the gate.
