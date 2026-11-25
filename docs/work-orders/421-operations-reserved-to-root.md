# Work order 421: an operation a package reserves to root (ACC-032, CTR-009, DEC-018; the rule order 413 needs)

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
[DEC-018](../decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2):
enforced -> "Yes; in v0, after S2 (Rec.)": what an Agent reaches is enforced by the engine in
v0, not left to documentation.
[DEC-133](../decisions/log.md#dec-133-2026-11-23-model-driven-programs-keep-root-until-their-interface-lands):
the transition DEC-128 leaves -> "Confirm (Recommended)": the coordinator keeps acting as root
through the CLI until model-driven programs have an Interface of their own; an Agent that Conduit
runs, a worker's turn among them, is not root. DEC-212 (2026-11-24, in decision 212's change,
not yet in the log at this order's base, so not linked here): what the second GitHub token
holds -> "Contents + PR write, this repo (Recommended)", "held as a secret Resource that only
root's write Conduit reaches". Order 413, drafted and held, gives the `git` package `commit.v1`
and `push.v1`, which only root's Conduit may ever hold. Today that could only be documentation:
the spec lets only the core package mark an operation reserved to root (CTR-009), so `git` cannot
say it, and the engine would keep it only in part if it did (Today). This order lets any package
mark its own operation `reservedToRoot: true` and makes the engine refuse it, as ACC-032 refuses
the core's five, to every Conduit whose consumer is not root, at registration and at open, and
every request that is not root's own.

**Today** (`main` at `46c740a9`). ACC-032 reserves an operation whose contract is marked
`reservedToRoot: true`, and says "Under CTR-009 only an operation of the core package is so
marked"; it refuses a request and an open, and says nothing of registration, so a Conduit that
lists a reserved operation for another consumer can be registered and is then never opened.
CTR-009 rejects a marked Operation contract outside the core package at install and submission;
CTR-003 rejects a capability outside the core that lists a reserved operation; ACC-033's row 9
authorizes "an operation reserved to root under ACC-032" against `management`; ACC-058 decides an
ask "on the operation's qualified reference in the core package alone", its table's first row
naming the core's five; PRV-004 says "SEC-008, SEC-009 and ACC-032 are keyed on the operation in
the same way", by the core's qualified reference. In the engine no source names CTR-009 or
CTR-003, and `operation-spec.v1` accepts the key, so the loader takes a marked contract in any
package; only `tools/check_schemas.py`'s rule `reserved-mark` rejects one, in the spec's own
examples (`spec/schemas/examples/invalid-by-rule/CTR-009-…`, which the shell's
`by_rule_examples_through_public_routes` observes as accepted). `routing` in
`engine/crates/core/src/access.rs` sends an operation outside the core to its Conduit's target. In
`evaluate_path` the request check reads the requested operation's contract as the target resolves
it (`schemas::operation`), so a request for a package's marked operation that is not root's own,
an onward one included (`request.serving`), reports refused under ACC-032 already:
`operation_identification_names_the_package_operation_on_a_name_collision` (`explain.rs`) shows
it for an Agent. The open check a few lines below scans only `context.core_environment`'s
contracts, by the core's qualified names, so root opens for an Agent, and an actor with an
unbounded right to open opens for itself, a Conduit listing a package's marked operation: the
Agent is shown a tool that every call of refuses. Registration (`registration.rs`) checks no
Conduit for a reserved operation, the core's included; it refuses a root-consumed Conduit from
anyone but root under ACC-054 (`root_consumed`, `refuse_root_registration`).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, the
mark only refuses: it grants nothing, and each refusal is an access decision that can be
inspected; 9, the restriction is opted into through the language, on the contract, and enforced
by the engine, not mediated; 7, every refused registration, open and request leaves its records,
as ACC-032's and ACC-054's do today. At risk, 5, "every management action is available to an agent
under a grant": a reserved operation is available to no Agent under any grant. It holds for
management actions, which keep their marks as they are; a package's mark is its author's statement
about its own operation, and ACC-032's five already make this exception for the core's.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice and by the core's own precedents (ACC-032 at open and request, ACC-054 at
registration). A program that ships a polkit action declares, in its own `.policy` file, that the
action needs an administrator (`<allow_any>no</allow_any>`, `auth_admin`), and polkitd, not the
program, enforces it. An Android app declares its own permission with
`android:protectionLevel="signature"`, and the system grants it only to apps signed with the same
certificate, whatever another app's manifest asks. Kubernetes refuses, when it is created, a
RoleBinding that would grant permissions its creator does not hold, rather than letting it stand
unused. In each the owner of an action marks it, and the central authority keeps the mark from
the first write. Not taken: a mark on a Type, a capability, a template or a Conduit (the
operation's identity is its contract, and the core marks there); an allow list in root's
configuration; reserving to any Interface but root (PND-009's shape, a separate rule); refusing a
template, rather than each Conduit made from it; closing a Conduit that is open when its
operation's contract is marked (each request is decided anew, ACC-036); hiding a tool from
presentation; building CTR-003's check in the engine here.

**When it runs.** On `main`, now; order 413 waits on 421b's merge. Through Conduit: one Codex
worker turn, the blind Codex verifier and Claude reader at once, one fix turn, the gate (DEC-046,
DEC-071, DEC-033). By ruling 89, each decision says what must hold; how is yours. It may be
dispatched as one turn; if one turn cannot finish it, it runs in two halves from the start, each
with its own turn, reads, fix turn and gate: 421a, the spec (decisions 1 to 7), whose case is an
expected failure, and 421b, on 421a's build, the engine (decisions 8 to 11), which makes it pass
and removes the entry. *In two halves*, after the Done when, gives each half's differences. Run as
one turn, the case is never listed as an expected failure, and the report lists the one case under
`vectors`. The opening rules are order 317's word for word: 421a's report lists its one added case
under `vectors`, an exception to their "leave `vectors` empty", and 421b's leaves `vectors`
empty. The coordinator's choices, open to the owner's strike
([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): the mark on the
Operation contract alone, as the brief's "or the contract that declares it" reads; CTR-009
rewritten in place to accept the mark anywhere, and CTR-003 narrowed to a reserved operation of
another package; the registration refusal written into ACC-032 for every reserved operation, the
core's five included, applied to each Conduit a registration would make (a Conduit document, an
instance's declared Conduit, a nested replacement's) and not to a template, whoever sends it;
the open refusal kept; ACC-033's row 9 narrowed to the core's reserved operations, a package's
being authorized through its own target; ACC-056's ACC-032 cause for refused registrations,
requests and opens, and ACC-058's first row widened for asks, as the brief's "the refusal
ACC-058's table gives" reads; an onward request named in ACC-032's `then`; `check_schemas.py`'s
rule moved from CTR-009 to CTR-003, with its example replaced; the case's name, steps and three
promises; the test names and the sentences' words.

**Shared files.** Order 413 changes `packages/git/` and marks `commit.v1` and `push.v1`; it runs
after 421b. DEC-064's engine orders build ACC-058 and change `access.rs`; order 317 changes
`registration.rs`, `requests.rs` and the expected failures; decision 212's change adds DEC-212 to
the log. Whichever merges later rebases and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first ACC-022, ACC-029, ACC-032, ACC-033, ACC-036, ACC-054, ACC-056, ACC-057, ACC-058,
CTR-002, CTR-003, CTR-009, CON-004, CON-016, LIF-002, LIF-009, PRV-004, SEC-008 and OPS-023;
`operation-spec.v1` and its header (`spec/schemas/contracts.yaml`); the row for CTR-009 and CTR-003
in `spec/schemas/README.md`; `reserved-mark` in `tools/check_schemas.py`;
`by_rule_examples_through_public_routes` (`engine/crates/shell/tests/by_rule.rs`) and its
`tests/expected/by-rule-outcomes.txt`; `routing`, `evaluate_path` and `decide` in `access.rs`;
`root_consumed`, `refuse_root_registration`, `restricted_submission`, `prepare` and `replacement`
in `registration.rs`; and the tests
`operation_identification_names_the_package_operation_on_a_name_collision`,
`reserved_operation_precedes_routing` and `named_uses_sender_on_onward_calls_and_keeps_object_order`
in `explain.rs`. Wrap each insertion at its file's width. "A" → "B" replaces A wherever it wraps;
"A" +"B" adds B after A. No text under `spec/` names a vendor or a package of this repository.

1. **CTR-009** (`spec/promises/contracts.yaml`), rewritten in place under its ID. When: "An
   Operation contract is marked `reservedToRoot: true`." Then: "The operation is reserved to the
   root Interface, under ACC-032, whichever package or project holds the contract. A package that
   holds such a contract is installed, and a project that holds one is submitted, as one that
   holds none would be." Observable: "The core package marks five: declare-lost.v1, purge.v1,
   enroll-machine.v1, admit-machine.v1 and refuse-machine.v1 (ENR-001, ENR-006, ENR-008). Another
   package marks an operation of its own whose effect only the root Interface is to cause, such as
   one that publishes to a shared place: a project's `directory.erase.v1` marked `reservedToRoot:
   true` is accepted, and only the root Interface's own Conduits list it (ACC-032). The mark is the
   contract's, so the operation is reserved wherever a capability lists it, and no Type,
   capability, template or Conduit marks or unmarks it. A capability lists a reserved operation
   only of its own package or project (CTR-003)."
2. **CTR-003** (same file). Then: "A capability that is not a contract of the core package and
   that lists an operation reserved to the root Interface is rejected" → "A capability that lists
   an operation reserved to the root Interface whose contract is not of the capability's own
   package or project is rejected". Observable, from "An operation is reserved" to "lists the two
   that do.": "An operation is reserved to the root Interface where its contract is marked
   `reservedToRoot: true` (ACC-032, CTR-009). `object-management.v1` lists two of the core
   package's; a package's capability may list its own reserved operations, and no capability lists
   another package's, the core's among them." CTR-002 is unchanged: it already sends the reader to
   CTR-009 for where the mark is written.
3. **ACC-032** (`spec/conventions/access.yaml`). When: "or asks to open a Conduit that lists such
   an operation and whose consumer is not the root Interface." → "or asks to open or to register a
   Conduit that lists such an operation and whose consumer is not the root Interface." Then:
   "dispatched only if the actor is the root Interface." → "dispatched only if the actor is the root
   Interface acting on its own behalf: an onward request, which serves another invocation, never
   is, whoever sends it."; "a request to open one that does is not dispatched." → "a request to
   open one that does is not dispatched, and a request that would register one, whether as a
   Conduit document, as an instance's declared Conduit or as a nested replacement's, is not
   dispatched and registers nothing."; "for either cause" → "for any of these causes". Observable,
   from "An operation is reserved" to its end: "An operation is reserved where its contract is
   marked `reservedToRoot: true`, in the core package or in any other (CTR-009). The core's five
   are declare-lost.v1, purge.v1, enroll-machine.v1, admit-machine.v1 and refuse-machine.v1
   (ENR-001, ENR-006, ENR-008). Whether an operation is reserved is read from the contract the
   target resolves for it when each registration, open or request is decided (ACC-036), so an
   operation of another package that shares a core export name is reserved only if its own
   contract is marked. A Conduit that a registration would make is checked as it would be
   instantiated (LIF-002), one whose `operations` an input fills (CON-016) included, and a refused
   registration makes nothing, as ACC-054's does; a template that declares such a Conduit is
   registered, and each registration of an instance made from it is refused. One registered before
   its operation was marked is opened only by `open.v1` or a setup's part (CON-004), each refused.
   The root Interface's own Conduits list reserved operations, and its own requests through them
   are dispatched as for any other operation." The refusal's cause stays ACC-056's row for ACC-032.
   In ACC-056's sixth tier: "and ACC-032, ACC-058, PND-009 and PND-010 on a Conduit to be opened
   that lists an operation they restrict" → "and ACC-032, ACC-058, PND-009 and PND-010 on a Conduit
   to be opened, and ACC-032 on one to be registered, that lists an operation they restrict". In
   `spec/core/operations.yaml`, the descriptions of `register.v1` and `register-replacement.v1`:
   "Only root registers root-consumed Conduits (ACC-054)." +" No Conduit that lists an operation
   reserved to root is registered for another consumer (ACC-032)." and "or register a root-consumed
   Conduit (ACC-057, ACC-054)." +" No Conduit that lists an operation reserved to root is registered
   for another consumer (ACC-032)."
4. **ACC-033** (same file). When: "perform an operation reserved to root;" → "perform an operation
   of the core package reserved to root;". Row 9: "(9) Installing a package or an operation reserved
   to root under ACC-032: `management`." → "(9) Installing a package, or an operation of the core
   package reserved to root under ACC-032: `management`." Observable: "Except for a control, a
   Conduit to the object itself or another target permits nothing." +" An operation that another
   package reserves to root (CTR-009) is in no row: it is authorized as any operation of its target
   is, by an open Conduit to that target listing it (ACC-001), and ACC-032 lets only the root
   Interface's own such Conduit be registered or opened." Restatements: `object-management.v1`'s
   description and the header comment in `spec/core/capabilities.yaml` ("the operations reserved to
   the root Interface" → "the core package's operations reserved to the root Interface"), and
   `spec/definitions.md`'s row `management` ("and operations reserved to root (ACC-033)" → "and the
   core package's operations reserved to root (ACC-033)"). `routing` already sends a package's
   operation to its target, so the engine changes nothing for this decision.
5. **ACC-058 and PRV-004.** In ACC-058's `then`: "This is decided on the operation's qualified
   reference in the core package alone, as PRV-004 matches," +" but for the table's first row,
   which also holds every other operation whose contract the engine holds under that qualified
   reference, in an installed package or a registered project, marked `reservedToRoot: true`
   (ACC-032, CTR-009);"; "a result depends only on those names and the caller's own Conduits and
   asks" → "a result depends only on those names, that mark and the caller's own Conduits and
   asks". In its table, the first row's Operations cell → "declare-lost, purge, enroll-machine,
   admit-machine, refuse-machine, and any other operation reserved to root (CTR-009)", its WHY and
   TODO unchanged; "Names in the table are the core package's qualified references, .v1 unless
   versioned explicitly." → "Names in the table are the core package's qualified references, .v1
   unless versioned explicitly; the first row's last item names none." In PRV-004
   (`spec/promises/providers.yaml`): "SEC-008, SEC-009 and ACC-032 are keyed on the operation in
   the same way." → "SEC-008 and SEC-009 are keyed on the operation in the same way; ACC-032 is
   keyed on the mark of the operation's own contract, which another package may write (CTR-009)."
6. **The restatements, the tool and its fixtures.** Each sentence that says only the core marks,
   or that no capability outside the core lists a reserved operation, says what decisions 1 and 2
   say: `spec/schemas/contracts.yaml` (the header's paragraph on `reservedToRoot`, its "Not
   expressed here" paragraph, `operation-spec.v1`'s `description` and the property
   `reservedToRoot`'s `description`); `spec/core/README.md`'s bullet "Reserved to `root`"; the
   comment in `spec/core/operations.yaml`; `spec/core/templates.yaml`'s `root-management` comment
   ("it lists the core package's operations reserved to the root Interface");
   `spec/definitions.md`'s row "reserved to the root Interface", which reads: "Said of an operation
   dispatched only when the root Interface asks on its own behalf, and listed only by its own
   Conduits, no other being registered or opened (ACC-032). Its Operation contract carries
   reservedToRoot: true (CTR-002), in the core package or any other (CTR-009), and only a
   capability of its own package or project may list it (CTR-003). The core's five are
   declare-lost.v1, purge.v1, enroll-machine.v1, admit-machine.v1 and refuse-machine.v1."; and
   `spec/schemas/README.md`'s row, which becomes "That no capability lists an operation reserved to
   the root Interface of another package or project | CTR-003 | A schema sees one document and not
   the package it belongs to | For an operation of the core package". In `tools/check_schemas.py`,
   `reserved-mark` (CTR-009) goes, and `reserved-listed` (CTR-003) replaces it: outside the core
   package, a Capability that lists an operation of the core package whose core contract is marked
   is rejected, naming both. Remove `CTR-009-reserved-mark-outside-the-core-package.yaml` and add
   `spec/schemas/examples/invalid-by-rule/CTR-003-capability-lists-a-reserved-core-operation.yaml`,
   one project's Capability that lists `github.com/shrav-k/conduit/core#purge.v1`, in the old
   file's form. In `engine/crates/shell/tests/expected/by-rule-outcomes.txt`, the line
   `CTR-009-reserved-mark-outside-the-core-package.yaml accepted succeeded -` goes and the new
   example's line, as `by_rule_examples_through_public_routes` observes it (its `--nocapture`
   prints it; `accepted succeeded -` is expected, since no engine source checks CTR-003), takes its
   place in name order. Add a line to `engine/crates/loader/tests/expected/invalid-verdicts.txt`
   only if the loader's examples test asks for one.
7. **The case** `access-reserved-package-operation`
   (`conformance/conventions/access/reserved-package-operation/`; layers `language` and
   `convention`; promises ACC-032, ACC-033 and CTR-009), modelled on `requests-usage-kept`'s
   `register-project`, open and dispatch steps, its IDs and times your own. Steps, with whatever
   setup the model needs between them:
   - root registers a project with Operation contracts `commit.v1`, marked `reservedToRoot: true`,
     and `status.v1`, unmarked; a Capability `repo-access.v1` listing both; a Resource Type
     `repo.v1` offering it, with a scripted implementation; a Resource `repo`; an Interface
     `actor`; and Conduits `root-repo` (consumer root, both operations), `actor-status`
     (`status.v1`) and `actor-access` (to `access`, listing `open.v1`, no bound): accepted
     (CTR-009);
   - root registers a Conduit `actor-commit` (consumer `actor`, target `repo`, `commit.v1`):
     refused, one error diagnostic, `ruleId` ACC-032, with `properties.refusal` and located at the
     Conduit's document as ACC-054's refusal locates one, its access-decision and request-ended
     records, and nothing registered (ACC-032);
   - root registers an Interface template `committer` whose declared Conduit `commit` to `repo`
     lists `commit.v1`, and an Interface `committer-1` from it: refused under ACC-032, nothing
     registered; the same template with no instance is accepted;
   - root registers `actor-commit-status`, a Conduit like `actor-commit` listing `status.v1`
     instead: accepted (the refusal is about the operation);
   - root opens `root-repo`, `actor-status` and `actor-access`: each succeeds;
   - root's `root-repo.commit.v1` is dispatched, its one external intent the call of `commit.v1`
     on `repo`, authorized by `root-repo` and not by `management` (ACC-033), and completes
     succeeded;
   - `actor`'s `actor-status.status.v1` is dispatched;
   - `actor`'s call of `commit.v1` on `repo` reports refused under ACC-001, which ACC-056 puts
     first, as no open Conduit of its lists it.

   Its index entry in `conformance/conventions/access/index.yaml`; in
   `conformance/manifest.yaml`, the case added to ACC-032's vectors, CTR-009's row given it as its
   vector in place of its reason, and CTR-003's reason saying that no case rejects a capability
   listing another package's reserved operation; its entry in `conformance/expected-failures.yaml`,
   "ACC-032: the first build checks no registered Conduit for a reserved operation; order 421b makes
   it pass"; `docs/work-orders/421-declared.yaml`, in 316's form, `kind: added`. Today it fails
   first at the step in which root registers `actor-commit`, which is accepted. Where
   `promise_standing.py --check` refuses an expected-failure case as an S1 vector, keep CTR-009's
   reason until 421b and list it under `own_choices`. Two lines in `spec/CHANGES.md` under
   `Unreleased`: "- Changed: CTR-009, CTR-003 and ACC-032, any package may reserve its own operation
   to root, and no Conduit listing a reserved operation is registered or opened for another
   consumer (DEC-018)." and "- Changed: ACC-033, ACC-056, ACC-058 and PRV-004, a package's reserved
   operation is authorized through its own target, and an ask for one is refused as the core's are
   (DEC-018)."; where the Start check's third line gives 1, cite DEC-212 beside DEC-018 in both. In
   `docs/implementation/SAFETY_CATALOGUE.md`, requote CTR-003's row from its new `then` and add,
   after order 286's line, "Work order 421 requoted the row of CTR-003, whose quoted text it
   changed."; CTR-009 stays `form`, and the quotes of ACC-032 and ACC-033 still stand in the new
   text. Then run `python3 tools/promise_standing.py --write`.
8. **Registration** (ACC-032, ACC-054, ACC-056, OPS-023, LIF-002, CON-016; 421b). For
   `register.v1`, the register-project entry and `register-replacement.v1` alike, a request that
   would make a Conduit whose consumer is not root and that lists a reserved operation, whoever
   sends it, root included, is refused under ACC-032 and makes nothing: a Conduit document, each
   declared Conduit of an instance the request makes, and a nested replacement's Conduit. Decide it
   as ACC-054's refusal is decided, before validity and conflict (OPS-023), on the operations each
   such Conduit lists as its document or its Conduit template writes them, each resolved to its
   contract in the target's package or the submission's own contracts; a reference that does not
   resolve is left to the checks that decide it today. Where an input fills `operations`
   (CON-016), decide it once the Conduit is instantiated, as PRV-004 decides one, and refuse there.
   The reply is ACC-054's form: `accepted` false, `reason` refused, one diagnostic from
   `explain::diagnostic` with `ruleId` ACC-032, located at the Conduit's document, or at the
   template's declared Conduit for an instance's, by `packages::object_location`; one decision
   naming ACC-032 and OPS-023, and the request ended refused. ACC-054 and ACC-057 come first where
   they apply. A Conduit whose consumer is root (ACC-054 lets root alone register one), one listing
   no reserved operation, and a template alone are registered as today.
9. **The open check** (ACC-032, ACC-056, CON-004; 421b). In `evaluate_path`'s check of a Conduit
   to be opened, decide whether each operation it lists is reserved from that operation's contract
   as the Conduit's target resolves it, as the head of `evaluate_path` resolves the requested one
   through `schemas::operation`, in place of the scan of `context.core_environment`'s contracts by
   the core's qualified names. An operation whose contract does not resolve there is not reserved by
   this check. Its place after ACC-054 and SEC-008 (ACC-056's sixth tier), its verdict and the
   refusal's diagnostic stay as they are. A setup's part that opens a declared Conduit is a request
   for `open.v1` by the setup's sender (CON-004, ACC-005): it reaches the same check; where it
   does not, route it there. A Conduit to `management` listing a core reserved operation is decided
   as today, since `management` resolves the core's contracts.
10. **Requests** (ACC-032, ACC-054). The request check stays as it is: a request for a reserved
    operation, of any package, whose sender is not root, or that serves an invocation, root's
    included, reports refused under ACC-032. The tests pin both halves of that condition.
11. **The tests, written first** (421b), in a new `engine/crates/core/src/reserved_tests.rs`
    declared in `lib.rs` as `limits_tests` is, on a state built as
    `operation_identification_names_the_package_operation_on_a_name_collision` builds one, with a
    project scope whose `commit.v1` is marked and `status.v1` and `purge.v1` are not:
    - `reserved_conduit_refused_at_registration`: root's `register.v1` of a Conduit whose consumer
      is an Interface, listing `commit.v1`, reports refused, `ruleId` ACC-032, located at the
      Conduit, with nothing registered; so do one listing the core's `purge.v1` to `management`,
      an Interface template declaring a Conduit listing `commit.v1` together with an instance from
      it (located at the declared Conduit), and a Conduit whose template's input fills
      `operations` with `commit.v1`; a request with both a refused Conduit and an invalid document
      reports refused (OPS-023); the template alone, a Conduit listing `status.v1` or the
      project's unmarked `purge.v1`, and one whose consumer is root listing `commit.v1`, are
      accepted; `register-replacement.v1` of a nested Conduit listing `commit.v1` for an instance
      is refused under ACC-032. Today each is accepted.
    - `reserved_package_operation_refused_at_open`: with Conduits put in the state registered, as
      the existing test puts its own, root's `open.v1` of one from an Agent listing `commit.v1`
      reports refused, `ruleId` ACC-032, and the Conduit stays unopened; one listing `status.v1`,
      and one listing the project's unmarked `purge.v1`, open; an Interface with an open,
      unbounded Conduit to `access` listing `open.v1` that opens one for itself listing
      `commit.v1` is refused under ACC-032; root's open of one whose consumer is root, listing
      `commit.v1`, succeeds. Today the first and the Interface's open succeed.
    - `reserved_package_operation_refused_at_a_setups_open`: root's `set-up.v1` of an Agent whose
      declared Conduit, put in the state registered, lists `commit.v1` reports that part refused
      under ACC-032 and leaves the Conduit registered. Today the part succeeds. Where no core
      helper reaches a setup's parts, write it on the helpers of the core test that does, and say
      which.
    - `reserved_package_operation_refused_but_to_roots_own_requests`: with Conduits listing
      `commit.v1` open in the state, an Agent's request and an onward request of the Agent's
      instance, each through its own Conduit, report refused under ACC-032; root's own request
      through its open Conduit `root-repo` is not refused under ACC-032; and the same request of
      root sent as an onward request, its `serving` an invocation in progress whose target is
      root and whose Conduit is open, crafted in the state as
      `named_uses_sender_on_onward_calls_and_keeps_object_order` names its `root-call`, so that
      IFC-028, ACC-019 and ACC-034 pass, reports refused under ACC-032. Removing
      `request.serving.is_some()` from the request check fails this test. It passes today and pins
      decision 10.

    No existing test or case is expected to change: no case registers a Conduit for another
    consumer that lists a core reserved operation (the core's reserved operations in cases are on
    root's first-start Conduit, root-consumed). Report any that does change, and why.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 018 133; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1
grep -c '^## DEC-212, ' docs/decisions/log.md  # 0; 1 once decision 212 has merged
k spec/conventions/access.yaml 'Under CTR-009 only an operation of the core package is so marked.' "This is decided on the operation's qualified reference in the core package alone, as PRV-004 matches," '| declare-lost, purge, enroll-machine, admit-machine, refuse-machine | it is reserved to root acting on its own behalf.' "Names in the table are the core package's qualified references" '(9) Installing a package or an operation reserved to root under ACC-032: `management`.' 'perform an operation reserved to root;' 'ACC-032, ACC-058, PND-009 and PND-010 on a Conduit to be opened that lists an operation they restrict' 'or asks to open a Conduit that lists such an operation and whose consumer is not the root Interface.'  # 1 1 1 1 1 1 1 1
k spec/promises/contracts.yaml 'An Operation contract that does not belong to the core package is marked' 'A capability that is not a contract of the core package and that lists an operation reserved to the root Interface is rejected' 'and only an Operation contract of the core package carries the mark, under CTR-009'  # 1 1 1
k spec/promises/providers.yaml 'SEC-008, SEC-009 and ACC-032 are keyed on the operation in the same way.'  # 1
cat spec/schemas/contracts.yaml spec/core/README.md spec/core/operations.yaml spec/definitions.md spec/promises/contracts.yaml | tr -s ' \n#' '   ' | grep -oiE 'only (an Operation contract of the core package|core Operation contracts)' | wc -l | tr -d ' '  # 8
grep -rl 'reservedToRoot' packages | wc -l | tr -d ' '  # 0
k tools/check_schemas.py '"reserved-mark": "CTR-009",'  # 1
for f in spec/schemas/examples/invalid-by-rule/CTR-009-reserved-mark-outside-the-core-package.yaml conformance/conventions/access/reserved-package-operation docs/work-orders/421-declared.yaml engine/crates/core/src/reserved_tests.rs; do test -e "$f"; echo $?; done | paste -sd' ' -  # 0 1 1 1
grep -cE '^\| (ACC-032 \| convention \| must \| 3 \| 0 \| cases \| core \| case passes|ACC-033 \| convention \| must \| 45 \| 1 \| cases \| core, shell \| case passes|CTR-003 \| language \| must \| 0 \| 0 \| reason \| none \| reason, no case|CTR-009 \| language \| must \| 0 \| 0 \| reason \| none \| reason, no case) \|$' engine/STANDING.md  # 4
grep -c '"id"' conformance/expected-failures.yaml  # 20
k engine/crates/core/src/access.rs 'if contract["spec"]["reservedToRoot"] == true' '.core_environment' 'd["spec"]["reservedToRoot"] == true' 'return Ok(Verdict::reason("refused", "ACC-032"));'  # 1 2 1 2
k docs/implementation/SAFETY_CATALOGUE.md '| CTR-003 | language | must | access | `then`: “A capability that is not a contract of the core package' '| CTR-009 | form |'  # 1 1
k engine/crates/shell/tests/expected/by-rule-outcomes.txt 'CTR-009-reserved-mark-outside-the-core-package.yaml accepted succeeded -' 'CTR-003-'  # 1 0
k engine/crates/core/src/registration.rs 'pub(crate) fn root_consumed(' '"refuse-root-consumer-registration",' 'ACC-032'  # 1 1 0
k spec/core/operations.yaml 'Only root registers root-consumed Conduits (ACC-054).' 'or register a root-consumed Conduit (ACC-057, ACC-054).'  # 1 1
k spec/definitions.md 'and operations reserved to root (ACC-033)'  # 1
```

Where a line prints otherwise, stop, change nothing and report it, but for the third, whose 1
means decision 212 merged first (decision 7 then cites it), and the twelfth, which counts other
orders' entries, and the eleventh, where another order moved ACC-033's counts: report it and go
on. A fourth to seventh, seventeenth or eighteenth other than stated means the spec moved since
this order was written; an eighth other than `0`, that a package marks an operation already (order
413 merged first: report it, as it then needs this order's spec); a thirteenth or sixteenth other
than stated, that the core's checks moved; a fifteenth, that the shell's fixture moved. Where it is dispatched in two halves, 421b's
worker runs this check on 421a's build as *In two halves* says.

## Scope

421a, for decisions 1 to 7: `spec/promises/contracts.yaml` (CTR-003, CTR-009),
`spec/conventions/access.yaml` (ACC-032, ACC-033, ACC-056's tier, ACC-058),
`spec/promises/providers.yaml` (PRV-004), `spec/schemas/contracts.yaml` (comments and descriptions
alone), `spec/schemas/README.md`'s row, `spec/core/README.md`, `spec/core/operations.yaml` (its
comment and the descriptions of `register.v1` and `register-replacement.v1`),
`spec/core/capabilities.yaml` (its comment and `object-management.v1`'s description), the comment
of `spec/core/templates.yaml`, `spec/definitions.md`'s two rows, `spec/CHANGES.md`;
`tools/check_schemas.py`'s rule and its two example files;
`engine/crates/shell/tests/expected/by-rule-outcomes.txt`'s one line, and
`engine/crates/loader/tests/expected/invalid-verdicts.txt` only as decision 6 says; the case, its
index entry, its manifest rows and CTR-003's reason, its expected-failures entry;
`docs/work-orders/421-declared.yaml`; `docs/implementation/SAFETY_CATALOGUE.md`'s row and line;
`experiments/012-model-checking/account.yaml` only where `check_model_account.py` asks for a row of
ACC-032, ACC-033, ACC-056 or ACC-058 to change; and `engine/STANDING.md` by the tool alone. No
schema's shape changes; no promise is added or removed, so `spec/README.md`'s counts stay.

421b, for decisions 8 to 11: in `engine/crates/core/src/`, `registration.rs`, `access.rs`,
`packages.rs` only where decision 8 needs a contract resolved, `lifecycle.rs` only where decision
9's setup part needs routing, `lib.rs` (its `mod` line), `reserved_tests.rs` (new) and `explain.rs`
only where a refusal's text or location needs it; the case's entry in
`conformance/expected-failures.yaml`; `engine/STANDING.md` by the tool alone.

Change nothing else: not the decision log (DEC-018 and DEC-133 stay not done), `packages/`
(order 413's), `engine/CONFORMANCE.md`, the loader's sources, the conformance crate, the shell's
sources and its other fixtures, another case, a `Cargo.toml` or `Cargo.lock`. Report anything else
that needs changing.

## Done when

The first group of each half prints otherwise before and passes after, with `j` and `k` as in the
Start check. The guards pass before and after, but for the line marked *after only*. Run each
existing test you changed by name, once. Run the core's tests and then the whole vectors test, once
each, last; never the whole engine suite.

421a, first group:

```sh
k spec/promises/contracts.yaml 'whichever package or project holds the contract' "whose contract is not of the capability's own package or project is rejected" 'An Operation contract that does not belong to the core package is marked' 'A capability that is not a contract of the core package and that lists'  # 1 1 0 0; 0 0 1 1 before
k spec/conventions/access.yaml 'in the core package or in any other (CTR-009)' 'the actor is the root Interface acting on its own behalf: an onward request' 'refuse-machine, and any other operation reserved to root (CTR-009) |' "that mark and the caller's own Conduits and asks" 'Under CTR-009 only an operation of the core package is so marked.'  # 1 1 1 1 0; 0 0 0 0 1 before
k spec/conventions/access.yaml 'or asks to open or to register a Conduit that lists such an operation' 'as an instance'"'"'s declared Conduit or as a nested replacement'"'"'s, is not dispatched and registers nothing' 'and ACC-032 on one to be registered, that lists an operation they restrict' '(9) Installing a package, or an operation of the core package reserved to root under ACC-032' 'is in no row: it is authorized as any operation of its target'  # 1 1 1 1 1; 0 0 0 0 0 before
k spec/core/operations.yaml 'No Conduit that lists an operation reserved to root is registered for another consumer (ACC-032).'  # 2; 0 before
k spec/promises/providers.yaml "ACC-032 is keyed on the mark of the operation's own contract"  # 1; 0 before
k spec/definitions.md 'in the core package or any other (CTR-009)' "and the core package's operations reserved to root (ACC-033)"  # 1 1; 0 0 before
cat spec/schemas/contracts.yaml spec/core/README.md spec/core/operations.yaml spec/definitions.md spec/promises/contracts.yaml | tr -s ' \n#' '   ' | grep -oiE 'only (an Operation contract of the core package|core Operation contracts)' | wc -l | tr -d ' '  # 0; 8 before
k tools/check_schemas.py '"reserved-listed": "CTR-003",' '"reserved-mark"'  # 1 0; 0 2 before
for f in spec/schemas/examples/invalid-by-rule/CTR-003-capability-lists-a-reserved-core-operation.yaml spec/schemas/examples/invalid-by-rule/CTR-009-reserved-mark-outside-the-core-package.yaml; do test -e "$f"; echo $?; done | paste -sd' ' -  # 0 1; 1 0 before
k engine/crates/shell/tests/expected/by-rule-outcomes.txt 'CTR-009-reserved-mark-outside-the-core-package.yaml' 'CTR-003-capability-lists-a-reserved-core-operation.yaml '  # 0 1; 1 0 before
cat conformance/expected-failures.yaml conformance/conventions/access/index.yaml | grep -c '"id": "access-reserved-package-operation"'  # 2; 0 before
for p in ACC-032 CTR-009; do grep -A4 "\"promise\": \"$p\"" conformance/manifest.yaml | grep -c '"access-reserved-package-operation"'; done | paste -sd' ' -  # 1 1; 0 0 before
grep -cE '^\| (ACC-032 \| convention \| must \| 4 \| 1 \| cases \| core \| case passes|ACC-033 \| convention \| must \| 46 \| 2 \| cases \| core, shell \| case passes|CTR-009 \| language \| must \| 1 \| 1 \| cases \| none \| expected failure) \|$' engine/STANDING.md  # 3; 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- '  # 2; 0 before
k docs/implementation/SAFETY_CATALOGUE.md 'Work order 421 requoted the row of CTR-003' '`then`: “A capability that lists an operation reserved to the root Interface whose contract'  # 1 1; 0 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. access-reserved-package-operation 2>&1 | grep -A1 '^EXPECTED FAILURE access-reserved-package-operation ' | cut -c1-200)  # its EXPECTED FAILURE line, then the step in which root registers actor-commit; nothing before
```

421b, first group:

```sh
ok='^test result: ok\. 1 passed'
for t in reserved_conduit_refused_at_registration reserved_package_operation_refused_at_open reserved_package_operation_refused_at_a_setups_open reserved_package_operation_refused_but_to_roots_own_requests; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done | paste -sd' ' -  # 1 1 1 1; 0 0 0 1 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. access-reserved-package-operation 2>&1 | grep -c '^PASS access-reserved-package-operation ')  # 1; 0 before
grep -c '"id": "access-reserved-package-operation"' conformance/expected-failures.yaml  # 0; 1 before: the entry's removal, which vector_delta does not count
k engine/crates/core/src/access.rs '.core_environment'  # 1; 2 before
k engine/crates/core/src/registration.rs '"ACC-032"'  # at least 1; 0 before
grep -cE '^\| (ACC-032 \| convention \| must \| 4 \| 0 \| cases \| core \| case passes|ACC-033 \| convention \| must \| 46 \| 1 \| cases \| core, shell \| case passes|CTR-009 \| language \| must \| 1 \| 0 \| cases \| (none|core) \| case passes) \|$' engine/STANDING.md  # 3; 0 before
```

Guards, for both halves. `n` names the files the half adds (421a: the case's directory, the new
example and `docs/work-orders/421-declared.yaml`; 421b: `engine/crates/core/src/reserved_tests.rs`):

```sh
export PYTHONDONTWRITEBYTECODE=1
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/421-declared.yaml; echo "exit $?"  # 421a, after only (the declaration it reads is 421a's): 0 undeclared; 0 errors, the one added case declared; then exit 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 421b, before and after: Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
v='claude|codex|anthropic|openai|chatgpt'; a=$(git grep -ohiE "$v" HEAD -- spec | wc -l); b=$(grep -rohiE "$v" spec | wc -l); [ $a -eq $b ] && echo same  # same
for c in access-routing access-root-local-lost records-purge-nonroot access-value-resource-root-only; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$c" 2>&1 | tail -1); done  # "1 cases passed", four times
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. access-ask-refused 2>&1 | grep -c '^EXPECTED FAILURE access-ask-refused ')  # 1, as before: ACC-058 is DEC-064's
for t in operation_identification_names_the_package_operation_on_a_name_collision reserved_operation_precedes_routing; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c '^test result: ok\. 1 passed'); done | paste -sd' ' -  # 1 1
(cd engine && cargo test --offline --release -p conduit-shell --test by_rule by_rule_examples_through_public_routes 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1
(cd engine && cargo test --offline --release -p conduit-loader --test examples 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)  # 421b
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)  # 421b
git diff --check  # prints nothing
git reset -q -- $n  # stage nothing
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Your report gives, per decision, what changed and where; each changed sentence; the new example's
observed line in `by-rule-outcomes.txt`; the case's first failing step as the runner prints it
after 421a and its PASS line after 421b; each new test's line on the unchanged code, then after;
each existing test or case you changed, and why; your own choices; and every check's last line.
Stage nothing.

**In two halves.** 421a changes only 421a's Scope and runs the Start check, its own Done when group
and the guards, the 421b `vector_delta` line apart; its report lists the case under `vectors`.
421b, on 421a's build, changes only 421b's Scope and runs the guards, the 421a `vector_delta` line
apart. Its Start check's fourth to seventh, ninth to twelfth, fourteenth, fifteenth and
eighteenth lines print what 421a left (`0 1 0 1 0 0 0 0`; `0 0 0`; `0`; `0`; `0`; `1 0 0 1`; `1`,
CTR-003's row alone; `21`; `0 1`; `0 1`; `0`): report them as they print and go on;
every other line holds as stated. Run as one turn, the expected-failures entry is never written:
421a's lines for it print `1`, the `EXPECTED FAILURE` line is the `PASS` line, the STANDING rows
are 421b's, and only the `vector_delta` line with `--declared` runs, after the work.

## What follows

- **Order 413**, on 421b's merge: `git`'s `commit.v1` and `push.v1` marked `reservedToRoot: true`,
  listed by a capability of its own package, and root's write Conduit to the copy, the only one
  that may be registered and opened, with DEC-212's token in a secret Resource that only root's
  Conduit reaches (SEC-008).
- **ACC-058's engine orders** (DEC-064) build the first row's widening and add to
  `access-ask-refused` an ask for a package's reserved operation, refused with that row's message.
- **CTR-003 in the engine.** No engine source checks it, before or after; a coverage order may
  build the check and give it a case.
- **A template that declares a reserved Conduit** is registered and can make no instance (decision
  8). Refusing the template itself would be a further rule; the coordinator asks the owner if it
  is wanted.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start. Drafted by the
drafter, `root-drafter.turn.v1`, on the Team seat, and checked adversarially by verifier-1 (four
musts), then revised; built and read through Conduit. Order 413 (commit and push) needs it.

**Worker-1's turns**, on the primary ChatGPT account: 421a (the spec), then 421b (the engine's
refusals at registration, at a Conduit's open and on every request that is not root's own).

**Two blind reads.** The Codex verifier, `root-verifier-1.turn.v1`, asked for one must: the
registration preview could not resolve a sibling Pool-entry target written as `{entry: name}`,
so such a declared Conduit listing a reserved operation slipped past the refusal. The Claude
reader, `root-reader-a.turn.v1`, on the Team seat, found decisions 1 to 11 applied, with three
shoulds and two nits on what the refusals say. One fix turn by worker-1 answered all six, the must
with a test that failed first. The gate's run on the squash is the coordinator's, at the merge.
