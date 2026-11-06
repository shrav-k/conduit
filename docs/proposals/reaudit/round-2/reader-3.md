# Re-audit round 2, reader 3: undetermined behavior

I read every file under `spec/` in full (README, definitions, schema profile, names, the implementation-defined list, all promise, convention and interface files, `spec/core/`, `spec/schemas/` and every example) and this round's README; I saw nothing else, and I did not report the points listed there under "Known before the round".

Each finding gives the scenario and step, the question, what I consulted, the behaviors two careful engineers could each build, the layer or the list item that covers it, and a one-line fix. Material used throughout: the example project `spec/schemas/examples/valid/project/`, the package `files-package/`, and the core package.

Counts: 7 UNSAFE, 17 DIVERGENT, 16 MINOR.

## UNSAFE

### 1. A filter or policy never reaches the steps of a graph that implements an operation

- Scenario: S3, a call through a Conduit that carries a filter, to an operation whose implementation is a graph (the manifest item `listing` in `files-package/conduit.yaml`, if it stated `supports`).
- Question: when the engine walks a graph that implements an operation, do the restrictions of the invocation go to the steps that invoke the instance's own implementation?
- Consulted: `implementation-item.v1` lets a `graph` item state `supports`; CON-022 ("the implementation its target has selected ... does not state that filter's requirement in `supports`"); IFC-018 (the list holds "the requirements of the request's own Conduit ..., every policy that applies, and the restrictions carried from the served invocation under ACC-017"); IFC-006 ("`conduit` is absent for a step that invokes the instance's own implementation"); ACC-018 (such a step "is not a request through any Conduit"); IFC-019 binds "an implementation [that] receives a request".
- Behaviors: (a) the engine passes the outer invocation's restrictions to each inner step; (b) by the letter of IFC-018 and ACC-018 the inner step has no Conduit and serves no invocation, so its `restrictions` is empty and nothing applies the filter. Under (b) a Conduit instantiated because the graph item states `supports` gives unfiltered access. The same question arises for a policy on the target (ACC-015).
- Layer: convention and interface. Not on the implementation-defined list (DL1 covers ordering only).
- Severity: UNSAFE.
- Fix: say that a step without `conduit` carries the restrictions of the invocation its graph is serving, or forbid `supports` on a `graph` item.

### 2. A step re-dispatched as a "new attempt" carries no operation identity, so a deduplicating implementation cannot recognize it

- Scenario: S2 and S7, setup runs again after the step `establish` (`directory.establish.v1`, `deduplicates: true`) ended unknown.
- Question: what operation identity does a step's dispatch carry?
- Consulted: OPS-021 ("not dispatched again, except as a new attempt of the same invocation where OPS-012 dispatches one"); OPS-012 ("A new attempt carries the same operation identity as the first"); OPS-009 (a repeat has "the same caller, target, operation identity and contents"); IFC-006 (`operationIdentity` is sent "when the caller supplied an operation identity"); OPS-011; DL1; DL2.
- Behaviors: (a) the engine derives a stable identity for each step and sends it, so the implementation deduplicates; (b) a step has no identity, so OPS-012 never applies and the step waits for reconciliation; (c) the step is dispatched again "as a new attempt" with no `operationIdentity` key, because the actor that asked for setup supplied none, and the effect is applied twice. All three satisfy the text.
- Layer: convention and interface. DL1 says only "what a re-run ... does with steps that already ran", and OPS-021 takes the unknown step out of it.
- Severity: UNSAFE (repeats an effect).
- Fix: state the operation identity the engine gives a step, and that it is sent in `io.github.shrav-k.conduit/operationIdentity`.

### 3. A retained result is scoped to the caller alone, so an instance serving that caller can retrieve the caller's other results

- Scenario: S3 and S4. Agent `reviewer-1` calls target S directly with identity `k1`; later it calls Resource R; R's implementation calls `retrieve-result.v1 {operationIdentity: k1, target: S}` naming the invocation it serves.
- Question: is "the original caller" the caller only, or the instance that sent the request as well; and whose access decides whether content is delivered?
- Consulted: IFC-024 ("with which the instance retrieves the caller's own results"; "a call to it needs no Conduit"); OPS-008 ("scoped to that caller and that target"); the definition of caller ("For an onward request it is the caller of the invocation being served"); IFC-021; OPS-015 ("An *actor* asks ..."); OPS-017 ("its access at that moment").
- Behaviors: (a) R's call has the caller `reviewer-1`, the invocation `(reviewer-1, S, k1)` exists, the caller's access to S permits the content, so R receives outputs from a target to which R has no Conduit; (b) the engine also requires that the retrieving instance sent the original request. The same scoping makes a direct request by the Agent and an onward request by R to one target with one identity the same invocation (OPS-009) or a conflict (OPS-010), and lets two Resources serving one caller collide. Nothing requires an identity to be unguessable.
- Layer: convention and interface.
- Severity: UNSAFE (content delivered to an instance that was never permitted it).
- Fix: scope an operation identity, and the retrieval, to the caller and the instance that sent the request.

### 4. "The caller's access at that moment permits the content" has no test

- Scenario: S4. A Conduit is closed while a call is in flight (content withheld); a replacement Conduit with a stricter filter is registered and opened; the caller retrieves the earlier result before the target is unregistered.
- Question: which access permits a retained result: the Conduit the invocation was dispatched through, or any open Conduit from the caller to the target that lists the operation; and how do the filters and policies in force now apply to content produced earlier?
- Consulted: ACC-034 ("delivered only if the caller's access at that moment permits it"); OPS-017; the definition of permitted; ACC-019 (for later work, "A new Conduit to the same target does not bring the work back"); ACC-036.
- Behaviors: (a) only the original Conduit, so the content stays withheld for good; (b) any open Conduit that lists the operation, so the content produced under the closed Conduit is delivered under a Conduit whose filter would have narrowed or forbidden it, with no implementation applying that filter; (c) unavailable, because the engine cannot evaluate a third-party restriction against stored content (ACC-014). For an onward invocation the "caller" usually has no Conduit to the target at all, so (b) withholds from the instance that made the request.
- Layer: convention.
- Severity: UNSAFE under (b).
- Fix: bind a retained result to the Conduit it was dispatched through, as ACC-019 binds later work.

### 5. The steps of a setup asked for by an onward request are "made by the actor that asked", which is the caller and not the instance

- Scenario: S2 and S5. A Resource R is the consumer of a Conduit to `management` that lists `set-up.v1`. While serving an invocation whose caller is `root`, R asks to set up `reviewer-1`.
- Question: whose Conduit to `access` (ACC-005) and to the Pool (POOL-015) authorizes the nested steps?
- Consulted: ACC-005 ("a request to open that Conduit made by the actor that asked for the setup ... If that actor is not the consumer of an open Conduit to `access` whose bound covers ..."); POOL-015 (same wording); ACC-016 ("permitted only under the instance's own open Conduit to that target"); the definition of actor (a Resource is not one).
- Behaviors: (a) "the actor" is the caller, `root`, so the declared Conduits open under `root`'s right to open, which has no bound, although `root` asked only for some unrelated operation of R; (b) the nested steps are onward requests of R and need R's own Conduit to `access` and to the Pool.
- Layer: convention.
- Severity: UNSAFE under (a) (a delegate acts beyond its bound through a caller's right).
- Fix: say that a nested step of a setup is authorized exactly as the setup request was: for an onward request, by the instance's own Conduits.

### 6. Closing a Conduit stops onward requests one level down only

- Scenario: S4, a Conduit is closed while a call is in flight, and the Resource it reached had already made an onward request that is itself in progress.
- Question: after the caller's Conduit closes, is an onward request made by the *second* instance (naming its own invocation) dispatched?
- Consulted: ACC-034 ("An onward request it makes after the close is not dispatched"; "No onward request that names the invocation in `serving` is dispatched after the close"); ACC-017 (restrictions carried are those "that applied to that invocation"); ACC-011 ("All the access it carried ends").
- Behaviors: (a) only requests that name the invocation dispatched through the closed Conduit are refused, so work two levels down keeps reaching further targets on the caller's behalf; (b) the engine follows the chain and refuses every request that descends from it.
- Layer: convention.
- Severity: UNSAFE under (a).
- Fix: state that the rule holds for every invocation that serves, directly or through others, one dispatched through the closed Conduit.

### 7. Which Conduit authorizes reconciling a management verb, and what reconciling a setup or an open does to state

- Scenario: S7, the engine restarts while `open.v1` or `set-up.v1` is in progress (unknown under IMP-007); S2, a setup ends unknown.
- Questions: (i) "the instance the invocation was dispatched to": the target (`access`, `management`, a Pool) or the object being managed? (ii) does recording that an unknown setup or open "occurred" make the instance ready or the Conduit open?
- Consulted: OPS-014 and ACC-033 ("the same target as setting up and retiring the instance the invocation was dispatched to"); LIF-018 (for a retirement, "the same target as retiring the instance"); `access-control.v1` (does not include `reconcile.v1`); LIF-003 ("An instance's setup succeeds. Its state is ready"); REV-010 and LIF-018 (the only two effects of a reconciliation that are stated).
- Behaviors: for (i), an unknown `open.v1` is reconciled through a Conduit to the Pool `system` (owner of `access`), or through `access` (impossible: no Conduit can list the operation), or through `management`. For (ii), engine A makes the Conduit open or the instance ready on `occurred: true`; engine B changes only the recorded outcome. A with the first reading of (i) opens a Conduit for an actor that holds no right to open and no bound that covers it.
- Layer: convention.
- Severity: UNSAFE in that combination; otherwise DIVERGENT.
- Fix: name the authorizing target for each management verb, and state the effect of reconciling `set-up.v1`, `open.v1` and `close.v1`.

## DIVERGENT

### 8. The first-start objects are named, not identified, and may be renamed or instantiated again

- Scenario: S1 and S6 (a rename).
- Question: is "the `access` Resource" (ACC-006, ACC-030, CON-028), "`management`" (ACC-033), "the root Interface" (ACC-024, ACC-032, ACC-035) the object made at first start, whatever bears the name, or any instance of the type?
- Consulted: FST-001; LIF-022 (bars retiring, unregistering and declaring lost, not renaming); `rename.v1` ("a template or an instance"); OBJ-002 ("The names of the first-start objects are taken in this way"); NAM-020 (the core templates resolve for anyone); no promise forbids a second instance of `core#access`, `core#management` or `core#root`.
- Behaviors: after `rename.v1` on `access`, a by-name engine finds no Conduit "to `access`" and nobody, `root` included, can open anything again; a by-ID engine is unaffected. A second instance of `core#access` in the Pool `team` is, on one engine, a second `access` whose Conduits fall under ACC-030, and on another an ordinary Resource whose Conduit may not carry `bound.v1` (CON-028).
- Layer: language and convention.
- Fix: say the first-start objects are identified by their IDs, refuse `rename.v1` on them, and refuse a second instance of those four templates.

### 9. Whether a policy on `access` or `management` "would apply to the root Interface"

- Scenario: S5, a policy is applied with `targets: [{kind: Resource, name: access}]`.
- Consulted: ACC-035; ACC-009 (a request is limited "by any policy on the target"); ACC-015 (a policy the target's implementation does not support makes each request unavailable); `core/conduit.yaml` (the engine supports only `bound.v1`).
- Behaviors: (a) refused, because it limits `root`'s requests; (b) applied, because `root` is not named. Under (b) every request to `access` then reports unavailable, `remove-policy.v1` included, and nothing can undo it.
- Layer: convention.
- Fix: say whether ACC-035 covers a policy on a target of one of `root`'s Conduits, and exempt `remove-policy.v1` or the two Resources.

### 10. A composed requirement: what `supports` must state and what goes on the wire

- Scenario: S3, a Conduit applies `example.org/files#review-limit.v1`; `files-server` states `supports: ['#rate-limit.v1']`.
- Consulted: CTR-010 (bindings are resolved "when the composition is applied"); CON-022; IFC-018 (each item holds `requirement` and `arguments`); ACC-038.
- Behaviors: (a) the Conduit is not instantiated, because the implementation does not state `review-limit.v1`; (b) the composition is expanded, the Conduit is instantiated, and `restrictions` holds `rate-limit.v1` with `limit` filled; (c) instantiated, and the composition itself is sent.
- Layer: language (CON-022) and interface.
- Fix: state that a composition is replaced by its constituents before CON-022 and IFC-018 apply.

### 11. The rules on files have no place on the wire

- Scenario: S1, "a developer submits the example project directory".
- Question: which component reads the directory? `register.v1` takes `documents`, already parsed, and `manifest`.
- Consulted: `register.v1`; MAN-001, MAN-002, MAN-011, SUB-005 (which files are read); SCH-012, SCH-017, SCH-018 (anchors, duplicate keys, numbers with no JSON form); MAN-006 (a relative `fetch` "is resolved against the place of the manifest"); MAN-010 (a relative program "is taken from the root of the package's archive"); README, Conformance ("documents in, resolved objects or diagnostics out").
- Behaviors: engine A ships a loader that enforces the file rules and treats `register.v1` as internal; engine B accepts `register.v1` from any client, so a client whose YAML parser expands an alias or keeps the last duplicate key registers what A rejects. A relative `fetch`, and the `command` of an implementation listed in the manifest of a project that is not published (it has no archive), have no base on either.
- Layer: language. No list item (DL7 covers fetching only).
- Fix: define the entry point that takes files, or give `register.v1` the files' bytes and the manifest's place.

### 12. What a tool presented to a consumer holds beyond its name

- Scenario: S3, the tool list of `reviewer-1`.
- Consulted: IFC-012 (names only); IFC-001; NAM-023 ("The contract as the target holds it governs"); SCH-015 (a `$ref` may be `example.org/files#read-limit.v1` or `#name`).
- Question: the tool's input schema, output schema and description; whether `$ref`s to Schema contracts are written out in full (no Model Context Protocol client can follow them); whether an input with a `default` is listed as required; how a handle is typed; which version of the contract is shown when consumer, Conduit and target hold different ones.
- Layer: interface. No list item.
- Fix: one promise that derives the tool definition from the Operation contract as the target holds it.

### 13. What the answer to a consumer carries beyond `_meta` and `structuredContent`

- Scenario: S7 and S10.
- Questions: (i) is the implementation's `content` passed to the consumer, and is it dropped where content is withheld (IFC-025 says only "no `structuredContent`")? (ii) does the answer to a repeat of a succeeded invocation carry the outputs again? (iii) is a result's content kept for `retrieve-result.v1` when the type's `records.outputs` is false, and for how long? (iv) does a repeat sent after its Conduit was closed report refused (ACC-011) or the invocation's state (OPS-009)?
- Consulted: IFC-001, IFC-025, OPS-009, OPS-016, OPS-017, REC-002, REC-004, CTR-004, ACC-011, ACC-034; DL10 covers sizes and times, not retention.
- Layer: interface and convention.
- Fix: state each of the four.

### 14. Work that starts later has no wire

- Scenario: S3, a call to an operation that starts model work and arranges further starts.
- Consulted: IMP-006 ("each start is ... permitted only if that Conduit is open"; "is not dispatched and reports refused"); ACC-019 ("the work's requests report refused when it runs"); IFC-028 (an invocation "that is no longer in progress" cannot be named in `serving`).
- Question: how a later start, or a request made by later work, tells the engine which arrangement and which Conduit it belongs to. The arranging invocation has ended, so it cannot be named.
- Layer: convention and interface. No list item.
- Fix: give arranged work a key, or say that each start is an invocation the engine itself dispatches.

### 15. `install.v1`: three cases without a stated ending, and dependencies

- Scenario: S8.
- Questions: (i) the fetched archive's digest differs from the input `digest` and nothing was installed before: invalid, or unavailable? PKG-021 speaks of a manifest item, PKG-025 of a package already installed, and PKG-023's list of causes does not include it. (ii) the archive's manifest states another `version` than the request (PKG-017 names only the address). (iii) the same address, version and digest is installed again: succeeded with no change, or conflict? (iv) the package's own manifest lists packages that are not installed: rejected, accepted with a diagnostic as PKG-006 does for a template, or fetched in turn? CTR-004, CTR-007, PKG-014 and SCH-015 all need the other package to decide.
- Layer: convention (i to iii), language (iv). DL7 does not cover them.
- Fix: one promise each.

### 16. `replace.v1` and a template that refers to its project's own contracts

- Scenario: S6 and S8, the template `reviewer` (whose input uses `$ref: '#model-name.v1'`) is replaced.
- Consulted: `replace.v1` (inputs `template`, `revision`, `manifest`; no contracts); SUB-008; NAM-009 ("submitting the project again with other contracts, does not change them"); NAM-022.
- Behaviors: `#model-name.v1` resolves against the unpublished package the template already holds; or is unresolved because the request carries no contract; and a changed contract cannot be supplied at all.
- Layer: language.
- Fix: say which unpublished package a replacement resolves against, or give `replace.v1` the project's contracts.

### 17. A Conduit closed because one of its ends was retired never has a confirmed shutdown

- Scenario: S4, after the target Resource is retired; S9, after a location is declared lost.
- Consulted: LIF-024 (confirmed "when the retire operation succeeded"; unknown for an instance "retired because its location was declared lost"; "pending otherwise"); CON-008; LIF-017; LIF-023; ACC-024.
- Behaviors: no close was dispatched for such a Conduit, so by LIF-024 its shutdown is pending for good and it cannot be unregistered; an engine that sees nothing to shut down reports confirmed. A further `close.v1` could confirm it, except where the consumer is `root` (ACC-024 refuses), so `root`'s dead Conduits keep their names for ever on the first engine.
- Layer: convention.
- Fix: state that a Conduit closed under CON-008 has its shutdown confirmed.

### 18. Does a rename advance the revision?

- Scenario: S6, a rename; two renames that name the same revision.
- Consulted: `rename.v1` ("Its ID, its definition and its revision are unchanged"); REV-004 (no advance unless definition or configuration changes; a name is neither); REV-012 ("Of several requests to change one object that name the same revision, at most one is stored: once it is, the revision has advanced"); REV-011 (lists `rename.v1` as a change).
- Behaviors: no advance, so both renames are stored; or advance, against REV-004 and the contract.
- Layer: convention.
- Fix: choose, and correct REV-012 or `rename.v1`.

### 19. Reconciling a control after another change was stored

- Scenario: S6, a control ends unknown, a second control then succeeds, and the first is reconciled as having taken effect.
- Consulted: REV-012 (an unknown control has ended, so the second is dispatched); REV-010 ("The stored configuration equals the configuration before the control with the stated settings applied").
- Behaviors: the stated settings are applied to the configuration as it stood before the first control, which discards the second control's change; or to the current configuration.
- Layer: convention.
- Fix: say "the current configuration".

### 20. A value reference whose source is absent, with no pointer

- Scenario: S3, a graph forwards an optional input that the request left out and that has no default.
- Consulted: GRA-020; GRA-026 (covers only a `pointer` that selects nothing); OPS-026 (defaults only); TPL-007 (a runtime the instance does not hold).
- Behaviors: the step reports invalid; or the argument is left out, so the inner operation's own default or optional applies. The same holds for an optional output of a step and for `source: runtime` when the runtime is not held.
- Layer: convention. This is not the known point, which concerns a pointer in `outputs`.
- Fix: one sentence in GRA-026.

### 21. "An operation that manages an object", and a control on a target that is not ready

- Scenario: S9, `set-facts.v1` on the second location before it is set up.
- Consulted: CON-024 ("for an operation other than one that manages an object"); ACC-033 (its list includes "a control").
- Behaviors: unavailable, because a control is an ordinary operation of the target; or dispatched, because ACC-033 lists it with the management verbs.
- Layer: convention.
- Fix: list the operations CON-024 exempts.

### 22. A replacement for a component Conduit: its name, and who holds its nested instantiation

- Scenario: S4, `register-replacement.v1` names the closed Conduit `review-read`.
- Consulted: `register-replacement.v1` (names "a component of a composite Conduit"); LIF-009 ("from the nested instantiation as the instance it belongs to holds it"); CON-003 ("no instance represents the composition itself"; the name is `review` then `-read`); LIF-007 (the retired Conduit keeps its name); IFC-012 (the tool name begins with the Conduit's own name).
- Question: the rule gives the replacement the name the retired Conduit still has; no instance holds the component; and the consumer's tool names change with whatever name is chosen.
- Layer: language and interface.
- Fix: state the name, and that the retired Conduit's own resolved definition is used.

### 23. A consumer is never told the engine's ID for its invocation

- Scenario: S4 and S6, any reconciliation.
- Consulted: `reconcile.v1` (names the invocation "by the ID the engine gave it"); IFC-025 (the result's keys: `outcome`, `reason`, `restriction`, `state`, `withheld`); REC-002; `inspect.v1` (`records` items are `{type: object}`); `core/README.md` lists the shape of an inspection result as not settled.
- New consequence: a client cannot call `reconcile.v1` on two engines in the same way, because the ID is found only in a record whose shape each engine chooses.
- Layer: interface.
- Fix: return the ID in the result's `_meta`.

### 24. A request to a target on a location that cannot be reached

- Scenario: S9.
- Consulted: CON-007, LIF-010, OBS-003; the meaning of unavailable in the definitions; DL5 ("How it is detected ..., and what happens when it returns").
- Behaviors: not dispatched and unavailable, which may be asked again; or dispatched, with no answer, so unknown, which needs a reconciliation.
- Layer: convention. DL5 does not say what a request reports meanwhile.
- Fix: state the reason, or add it to DL5.

## MINOR

25. The facts of `local` at first start are "the engine's to supply" (`core/README.md`), and decide whether the example project resolves (PLC-007 against `local-paths.v1`) and whether a shared-host Conduit ever opens. The implementation-defined list has no item for it; DL6 covers only what triggers first start. Fix: add the item, or require `host`.
26. The suffix of a generated name (OBJ-003, CON-003) is not on the list, and `register.v1`'s `objects` does not say whether templates and nested instances are included or which entry or declared Conduit each generated name belongs to. Fix: add a list item and a member naming the nested instantiation.
27. ACC-030 defines "within" by what a bound names. A Conduit to `access` that applies no `bound.v1` names nothing, so the definition alone passes it; only the observable ("covers nothing that the bound of the actor that opened it does not cover") excludes it. Fix: say in the obligation that no bound is within a bound only for an actor that has none.
28. A control that a template's `controls` map leaves out is "not available" (TPL-010), yet a Conduit may list it (CON-017). What a request for it reports is not stated. Fix: name the reason.
29. Three endings with no stated reason: a call to a tool name that was never presented (a protocol error, refused or invalid); `retrieve-result.v1` for an identity that names no invocation, or one whose request ended with a reason (OPS-025); `open.v1` on a Conduit that is already open (CON-004 covers only the setup step).
30. A plain server that returns `content` and no `structuredContent` for an operation with a required output: succeeded under IFC-010, or unknown under OPS-007 ("a declared output ... fails validation"). The `tools` map renames the tool only, so the server's argument names must equal the contract's.
31. A runtime declared by both a Type and a template (the example declares `watcher` in both): whether a step may name one the type alone declares, what happens when the two differ, and what "required" means for a runtime (OBJ-015 lists it, and no promise uses it).
32. `retire-with-resources.v1`: whether the Conduit to the Pool must also list `retire.v1` for each Resource, as POOL-015 requires `set-up.v1` for each entry.
33. `apply-policy.v1` with neither `actors` nor `targets` (both optional): a policy that applies to nothing, or invalid. A bound covers it vacuously (ACC-031).
34. A control's `settings` that holds a key outside its `changes`; and which settings a control "addressed" (REV-009) when only some inputs were supplied, since a contract does not tie an input to a pointer.
35. References in configuration are rewritten to IDs (OBJ-021), but REV-008 and REV-013 test the stored configuration against the type's schema, where an ID need not satisfy a reference schema; and `inspect.v1`'s `declared` then shows IDs that no request may use (LIF-007).
36. A bound's `operations` are qualified references. Whether a re-exported operation (PKG-013) matches by the string written or by the export it resolves to is not stated; an operation of an unpublished package cannot be named in a bound at all.
37. PLC-016 says the default-location setting "names a location that exists or nothing"; LIF-019 forbids changing an object that refers to an unregistered one. After the default location is unregistered the two disagree.
38. An entry's placement that came from the default rules: whether it is stored in the Pool's resolved definition, so that a replacement under LIF-009 goes to the same location, or is chosen again. Also when PLC-007 is tested again after `set-facts.v1` removes a fact that placed instances need.
39. LIF-026 has the level `may` for something an engine must do: it is the only way from a pending shutdown, after a failed retirement, to a confirmed one.
40. S10, values I had to guess: the type of `operationIdentity` on a request (a string, by `retrieve-result.v1`); whether `isError` is set on the answer `inProgress`; where a refusal under ACC-006 "names what is outside the bound"; whether `serving` is passed on to the onward target (I assumed not); whether two equal items of `restrictions`, one a filter and one a policy, can be told apart in `restriction`; and what `content` holds in every case.

## Traced with no guess

- S1: the five first-start objects, their names, owners, states and the placement of `management` and `access` (PLC-002); `root`'s two open Conduits, the operations each lists, and that `root-access` has no bound; `root`'s tool names. For the example project: template against instance, `from`, the precedence of argument, default, template and type default (INS-019), the entry reference and the reference to a Pool's entry, the names `review-read` and `review-settings`, the checks of CON-021, CON-022, CON-028 and IFC-014, that nothing is registered if one document is rejected, and that `reviewer-1` is rejected until both packages are installed (PKG-007).
- S2: which Conduit authorizes the setup of the Pool, of each entry's Resource and of the Agent, and the opening of each declared Conduit (ACC-033, POOL-015, ACC-005, for a request an actor makes itself); that either order works (CON-023, CON-024); states after success, failure, an unknown outcome and a setup that is not dispatched (LIF-003, LIF-004, LIF-027); that an unknown step waits for reconciliation where its operation does not deduplicate.
- S3: every tool name, including `review-read.read.directory.read.v1`; a plain call and its outcome table (IFC-007, IFC-010); a filter as an item of `restrictions`; the keys of an onward request and that the caller and restrictions are taken from the served invocation (IFC-020, ACC-016, ACC-017); IMP-008.
- S4: a close during a call, one level deep (ACC-034); the setup step that reports conflict for a closed declared Conduit (LIF-008); a replacement in the state registered, opened by a separate request; retired from the moment of dispatch; unknown shutdown, reconciliation, then unregistration with the revision (LIF-017, LIF-018).
- S5: opening inside and outside the bound; passing the right on narrower and wider; a further right for oneself; applying and removing a policy inside and outside the bound; that closing the second Interface's right closes nothing it opened (ACC-006, ACC-029 to ACC-031).
- S6: a stale revision, no revision, a partial input, an unknown control and a reconciliation that fails the schema (REV-002, REV-006, REV-007, REV-009, REV-011, REV-013); `set-facts.v1` with a shared-host Conduit open (PLC-013); a template replaced while instances exist (INS-020).
- S7: a repeat while in progress, after failure, after unknown with and without `deduplicates`, with other contents, by another caller, after a reason, after a purge; a restart during an invocation (OPS-009 to OPS-012, OPS-024, OPS-025, REC-004, IMP-007).
- S8: an archive that cannot be fetched; another digest already installed; a second version beside the first; two versions joined by a Conduit (NAM-017, NAM-023, NAM-026); a Schema referred to as `#name`; the digest of an unpublished package; the core package with no manifest item; an unknown `apiVersion`.
- S9: what registering a location needs (two Conduits, each listing `register.v1`); placement on a registered location and the unavailable setup (PLC-018, PLC-019); the shared-host requirement at opening; declaring lost and the order of unregistration (PLC-017, PLC-011, LIF-017).
- S10: the request keys of IFC-006 with their values (`v0`, IDs, the qualified reference, the list of restrictions), their absence for a plain implementation, and the result keys `outcome`, `reason`, `restriction`, `state` and `withheld`.

## Judgement

The language layer is close: from this text I could build resolving, instantiation, naming, packages and the static checks so that a second team would match on the example project, and nearly every finding against it is at an edge (finding 8, 10, 11, 15, 16, 22). The three biggest obstacles are elsewhere. First, the access convention is stated for a request an actor makes directly through one Conduit, and does not carry through composition: graphs that implement operations (1, 2), onward requests that manage objects (5), chains of onward requests (6), and results kept after the Conduit that earned them is gone (3, 4). Second, the wire fixes `_meta` and tool names but not the rest of what a consumer sees or sends: tool definitions, `content`, the invocation's ID, later work, and where files become documents (11 to 14, 23). Third, the core package's identity and reconciliation are assumed rather than stated: what "the `access` Resource" is once it can be renamed or instantiated again, and what a reconciliation authorizes and changes (7, 8, 9). This round is not yet clean: seven of the findings let a conforming engine apply an effect twice or deliver access or content that was closed, but each has a fix of one or two sentences, and the remaining divergences are narrow, so one more pass on the access and interface layers should bring it close.
