# GL4-second-core-resource: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The text forces one reading for all three requests. `management` and `access` in ACC-033 mean the first-start objects (FST-001, with OBJ-002 taking the names), so every management or access operation asked through a Conduit to a second Resource of `access.v1` or `management.v1`, or to a Resource of a project's own Type that offers their capabilities, reports refused under ACC-033. The first build agrees: it identifies the two objects by the core template they were made from and refuses at engine/crates/core/src/access.rs:721-724.

Nothing rejects such a template, Type or capability, though. The language admits instances whose tools look real and can never be served, and only two sentences in `observable` fields, with no vector, stand between that and unbounded access (F1). The language already closes the same hole for `node.v1` (PLC-028), Fleet Types (CTR-004), reserved operations (CTR-003) and provider operations (PRV-004). One more rule of that shape is the least fix, and it is the coordinator's because it removes nothing that works. FST-006's `when` also misses a Pool's entry made from the core templates, which the build rejects anyway (F2).

For the `system` Pool and a second Interface of root's Type I found nothing to report. `pool.v1` and `interface.v1` are the basic Types meant for every project, every promise names `system` and root by name, and a Conduit made from `root-management` for another Interface is never opened (ACC-032).

Two build notes, as evidence only. The engine's routing sends any core operation without an arm to the Conduit's own target (today `answer.v1` and `sign-in.v1`; `make.v1` when batch 2 adds it), so the ID guard is per verb and orders 047 and 048a need arms for rows 16 and 17. PLC-028 itself is unbuilt, which the first pass knows (P07), and the build recognizes a node by Type (engine/crates/core/src/registration.rs:1269; engine/crates/core/src/inventory.rs:23).

**What the reader read.** Commit 2eb8f258, read only; nothing was built or run.

Read in full:
- docs/guides/full-review.md, and AGENTS.md "What holds".
- spec/promises/first-start.yaml (FST-001 to FST-007).
- spec/conventions/access.yaml, all 1,004 lines, including ACC-029, ACC-030, ACC-033, ACC-054, ACC-055 and ACC-056.
- CON-017 to CON-031, CON-028 among them (spec/promises/conduits.yaml:213-410).
- TPL-001 to TPL-005, CTR-001 to CTR-005 and CTR-009.
- OPS-019, OPS-020, OPS-036, PLC-008, PLC-028, PRV-001, PRV-004, SEC-022, OBJ-002, OBJ-014, INS-012, INS-016, INS-025, MAN-012, LIF-028 and IFC-012.
- spec/core: types.yaml, templates.yaml, conduit.yaml and requirements.yaml.

Read in part:
- OPS-014 (lines 305-355), IFC-033 (its `when` and `then`), POOL-012 and POOL-013.
- spec/core/capabilities.yaml:1-130; spec/core/README.md:1-150, 508-536 and 692-717; in spec/core/operations.yaml only `open.v1`'s description and the "Authorized against" lines.
- spec/definitions.md, spec/README.md (lines 331-338 and 452-488), IN_FORCE.md (topics 6 and 8, line 220, the Open lines), docs/AUDIT.md (lines 120-150 and 228-240), docs/proposals/projects/brief.md:2247-2257. docs/guides/cold-readers.md and docs/implementation/HANDOVER.md by grep only.
- Engine, as evidence only: access.rs:221-346 and 440-1060; registration.rs:415-500, 590-680, 940-1045, 1255-1280, 1360-1420 and 2340-2385; lib.rs:596-625; requests.rs:1030-1075; inventory.rs:10-40.
- First pass: language/INDEX.txt by grep; X06's assessment, coverage, F11 and verifier's notes; the critic's entry for this area; P07 lines 270-284 and 392-402.

Conformance:
- One-off scripts parsed 1,576 JSON and YAML files under conformance/ (six loader fixtures do not parse). They looked for objects and templates of `access.v1` or `management.v1`, Type contracts offering the two capabilities, capabilities outside the core listing management verbs, and Pool entries made from the five core templates.
- A like scan of Type and Capability contracts ran over packages/, experiments/, spec/schemas/examples and conformance/fixtures.
- I read the structure of the three FST-006 cases, the CON-028 case and the requirement-bound case, and searched the titles of the 494 cases.

Not read: spec/core/references.yaml and results.yaml, the rest of spec/core/operations.yaml, spec/schemas beyond three invalid-by-rule examples, the other promise files beyond the promises named, the sectioned short form, the staged 049a patch, the engine's shell and loader, and the rulings' records.


## FR-564

**Nothing rejects a template outside the core whose management or access operations the engine implements: it registers, presents tools, and every call is refused**

GL4/F1 · gap · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** TPL-004, TPL-003, FST-006, PLC-028, CTR-003, CTR-004, CON-021, CON-028, IFC-012, ACC-033, FST-001, OPS-019

**Where.** spec/promises/templates.yaml:84

**Quote.** "Where the core package's implementation `engine` lists the operation, under MAN-012, the implementation is the engine. Otherwise it is the default the type states for that operation, and where the type states none, the template is rejected."

**Situation.** Three routes, none rejected.
(a) A project's own Resource template states `type: github.com/shrav-k/conduit/core#access.v1` or `#management.v1`. FST-006 covers only instances made from the core templates (spec/promises/first-start.yaml:81-82) and PLC-028 only `node.v1` (spec/promises/placement.yaml:448-451).
(b) A project's own Type lists `access-control.v1` or `object-management.v1` under `offers`. CTR-004 rejects only a Pool Type without `pool-management.v1` and a Fleet Type outside the core (spec/promises/contracts.yaml:109-110, 121-123). So a Resource, Agent or Pool Type may offer either, and a Type that is no Pool may offer `pool-management.v1`.
(c) A project's own Capability lists `open.v1`, `register.v1` or `install.v1`. CTR-003 rejects only reserved operations (contracts.yaml:81-83).
In each, TPL-004 makes the engine the implementation and TPL-003 forbids any other, CON-021 instantiates a Conduit to the instance (spec/promises/conduits.yaml:267-269), and IFC-012 presents its tools.

The reading the text forces is by ID. FST-001: "where a promise names `system`, `local`, `management`, `access` or `root` ... it means the object made at first start" (first-start.yaml:22-24), and OBJ-002 takes those names.
- Second Resource of `access.v1`: ACC-033 row 10 names the first-start `access` (spec/conventions/access.yaml:617) and "a Conduit to the object itself or another target permits nothing" (access.yaml:639-640). Open, close, lift and reconcile all report refused.
- Second Resource of `management.v1`: every row names `management`, an owning Pool or Fleet, or a container, never that Resource. All 13 operations report refused.
- A project's own Type: the same for those operations; only its other capabilities and controls work. On a Pool Type that also offers `object-management.v1`, the operations shared with `pool-management.v1` work as the Pool's own and the rest refuse.
A Conduit to such an instance can carry no bound (CON-028) and no filter (CON-022), and ACC-030 does not apply to opening it.

So the Type's contract "promises what its instances offer" (templates.yaml:98) and they can never serve it. Registration says nothing, and each consumer gets up to 4 or 13 tools defined exactly as the real ones that always refuse.

The trap. Every other role in spec/ is held by capability (a provider under PRV-001, a secret Resource under SEC-022, a Pool under CTR-004). TPL-004 says the engine implements these operations "as Kubernetes's API server serves its built-in resources", and CON-018 applies a bound "to each request through that Conduit". An engine that tests the target by Type, as the first build tests a node (engine/crates/core/src/registration.rs:1269), would serve them. Steps: a project registers `desk-management` of `management.v1` in Pool `team`; a delegate whose bound names `team` and the operations of `pool-manage` opens a Conduit to it listing `inspect.v1`, `set-up.v1` and `list.v1` (ACC-006 is met through the Pool); it then inspects and sets up every Interface, Pool and Fleet. With `access.v1` and a bound that names `open.v1`, the Conduit has no bound, which ACC-029 calls unlimited. Only the two sentences quoted above prevent this, and both are in `observable` fields.

Vectors: none. A scan of the 1,576 JSON and YAML files that parse under conformance/ finds that every object of the two Types is the first-start `management` or `access`, no Type contract offers either capability, and no capability outside the core lists a management verb. The three FST-006 cases submit only instance documents made from the core templates (conformance/language/first-start/protected/project/access.yaml:7). CON-028's one case tests the double bound (conformance/conventions/access/duplicate-bound/case.yaml).

First build, as evidence of a reading: it takes the ID reading, through the core template of origin. `management()` finds the object whose `from` is the core template (engine/crates/core/src/access.rs:267-271); open, close and lift route to it (access.rs:571); a request whose Conduit has another target reports refused under ACC-033 (access.rs:721-724). It registers any template whose Type needs engine operations (registration.rs:1023-1026). The guard is per verb: an operation with no arm routes to the Conduit's own target (access.rs:593-595), which today catches `answer.v1` and `sign-in.v1`, stubbed as unavailable (engine/crates/core/src/requests.rs:1046-1059).

**Counterargument.** It fails closed, as X06 said, and the first build refuses. docs/AUDIT.md:236 already records that nothing prohibits another core management or access Resource. ACC-056's cause for ACC-033 describes the fitting row, so the caller is told that `access` is needed. No document under packages/, experiments/, the examples or the fixtures does any of this. A rejection is one more rule in a catalogue the owner wants shorter. It would also foreclose a second access point per project (the idea at docs/proposals/projects/brief.md:2257) until the owner rules on one.

**Least change.** One rule in the shape of PLC-028, checked when the template is registered (a template's `type` is never an input's destination, INS-012): a template that is not the core package's own is rejected where TPL-004 would make the engine the implementation of an operation, except the operations of `pool-management.v1` on a Pool template. The diagnostic names the template, the Type and the operation. This closes all three routes.

Stated at the contracts instead, it takes three sentences: CTR-004 rejects a Type outside the core that offers `object-management.v1` or `access-control.v1`, or `pool-management.v1` when it is not a Pool; CTR-003 rejects a capability outside the core that lists any operation `engine` lists, not only a reserved one; a template of `management.v1` or `access.v1` other than the core's own is rejected.

With either: one file under spec/schemas/examples/invalid-by-rule and one vector per route. Also move FST-001's sentence on IDs and ACC-033's "another target permits nothing" into their `then` clauses, with X06/F11's sentence stated for every row. If the rejection is not taken, add at least one vector that pins the refusal: register a second Resource of each Type, open a Conduit to it, call each tool and expect refused under ACC-033, and expect CON-028 for a bound on that Conduit.

**Precedent.** PLC-028 (a template of `node.v1` other than the core's is rejected); CTR-004 (a Fleet Type outside the core is rejected); CTR-003 (a capability outside the core may not list a reserved operation); PRV-004 (keyed on the operation "whatever capability lists it"). Kubernetes, which TPL-004 cites: the API server alone serves the built-in groups, and a CustomResourceDefinition in a protected group (*.k8s.io) is refused without an approval annotation.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the fix adds no behavior an actor can use, since every request to such an instance is already refused. If it is counted as a small feature, ruling 71 point 4 applies; it cannot move S1's counts, because a scan of Type and Capability contracts under packages/, experiments/, spec/schemas/examples and conformance/fixtures finds no such template, Type or capability. Logged for the owner to strike. Making such a Resource a working second access point would be new behavior and the owner's.

**Verifier.** weakened, low. Quotes are verbatim: spec/promises/templates.yaml:84-86 (TPL-004 `then`), :98 ("A type's contract promises what its instances offer"); spec/promises/first-start.yaml:81-82 (FST-006 `when` names instance documents from the five core templates); spec/promises/placement.yaml:447-451 (PLC-028, `node.v1` only); spec/promises/contracts.yaml:81-83 (CTR-003, reserved operations only), :109-110 and :121-123 (CTR-004, Pool without pool-management.v1 and non-core Fleet only). A search of spec/ (`git grep -E "offers?.{0,80}(access-control|object-management|pool-management|fleet-management)"`) finds only descriptions and no exclusion: definitions.md:204, spec/conventions/operations.yaml:415-416, contracts.yaml:110 and :122. So all three routes register. That much stands. The engine evidence matches as cited: access.rs:267-271 finds by `from`, :571 routes open, close and lift to `access`, :593-595 is the default arm, and :721-724 refuses under ACC-033. registration.rs:1023-1026 is TPL-004. No vector, package, experiment or example has a non-core Type offering either capability (`git grep -E "(access-control|object-management)\.v1"` over conformance, packages, experiments and spec/schemas/examples finds only core references).

KNOWN for route (a). docs/AUDIT.md:236 lists under "Known to be open": "no prohibition on registering another core management/access Resource". docs/proposals/projects/brief.md:2252 says: "Registering another Resource of type core#management.v1 or core#access.v1 from a user project: no promise I found forbids it. FST-001 and ACC-033 give meaning only to the first-start objects, so such a Resource would authorize nothing. Not tested." The first pass's X06 preamble agrees: "still unforbidden but fails closed, because ACC-033 names the first-start objects." Routes (b), a project's own Type offering access-control.v1 or object-management.v1 (or a non-Pool offering pool-management.v1), and (c), a project's own Capability listing open.v1, register.v1 or install.v1, appear in none of those records, so known-new-situation is right for them.

NOT A GAP. The situation is decided: each such request reports refused. The ACC-033 rows name `management` and `access` in its `then` (spec/conventions/access.yaml:595-635). ACC-006's `then` binds every open to that object whatever Conduit carries the request: "The request is an operation of the `access` Resource ... Being the consumer of an open Conduit to the target does not permit it" (access.yaml:107-110). CON-028's `then` says "A Conduit that applies it and whose target is not `access` is not instantiated" (spec/promises/conduits.yaml:378-379).

THE 'TRAP' READING HAS NO TEXT. The finding says "only the two sentences quoted above prevent this, and both are in `observable` fields". That is wrong. Reading by name gives the same object as reading by ID, through four `then` clauses: OBJ-002 rejects a second object of the same kind and name (spec/promises/objects.yaml:31-35, observable :38-39 "The names of the first-start objects are taken in this way from first start"); OBJ-011 fixes names (objects.yaml:163-165); LIF-022 never retires or unregisters them (spec/conventions/lifecycle.yaml:463-468). definitions.md:147-148 define `management` and `access` as "The Resource that ...". No text names them by Type. A builder who served a second `access.v1` Resource as `access` would break ACC-006's `then` and CON-028's `then`. Unsafe: false is right.

**Verifier's corrections.** Class: needless-complexity, not gap. The language admits a template whose management or access tools can only refuse. ACC-033 decides the outcome, and ACC-056:957 gives an explicit cause, "The request needs the target described by its first fitting row."

Known for route (a): docs/AUDIT.md:236 and docs/proposals/projects/brief.md:2252. New only for routes (b) and (c).

Strike from the situation "Only the two sentences quoted above prevent this, and both are in `observable` fields" and the harm steps built on it. The by-Type reading is excluded by the `then` clauses of OBJ-002, OBJ-011, LIF-022, ACC-006 and CON-028, and by definitions.md:147-148. Moving FST-001's ID sentence into its `then` is therefore optional tidying, not a safety fix. Also, ACC-029's "unlimited" sentence does not reach a Conduit to a second `access.v1` Resource: its `when` is "A Conduit to `access` carries a bound" (access.yaml:476).

The counterargument's brief.md:2257 is not an idea for a second access point. It is "a Conduit to a Project as a new authorizing target".

Severity low: it fails closed with an explicit refusal, and no document in the tree uses any route.

The least change stands as the better fix, and the coordinator decides it under rulings 37 and 46 point 4, logged for the owner to strike. It is one PLC-028-shaped rule at template registration: reject a non-core template where TPL-004 would make `engine` the implementation, except pool-management.v1 on a Pool type. It does not touch brokers, whose operations are their own (definitions.md:187). If the owner prefers no new rule, the fallback is one vector that pins refused under ACC-033 for each route.


## FR-565

**FST-006's `when` names only an instance document, so a Pool's entry made from the core templates `access`, `management` or `node` falls outside it**

GL4/F2 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** FST-006, PLC-028, PLC-008, OBJ-014, INS-025, POOL-012

**Where.** spec/promises/first-start.yaml:81

**Quote.** "An instance document's `from` resolves to the core package's template `system`, `node`, `management`, `access` or `root`."

**Situation.** A project's Pool template writes `entries: {acl: {from: github.com/shrav-k/conduit/core#access}}`, or `#management`, or `#node`. An entry is a nested instantiation, "with no `kind`, `metadata` or `spec`" (spec/definitions.md:87), and INS-025 keeps the two apart: "The `from` of an instance document, or of a nested instantiation" (spec/promises/instantiation.yaml:353). So FST-006's `when` does not fire, and no Pool promise limits an entry's template.

Its `then` still says the first-start instances and admitted nodes "are the only ones: the engine alone makes nodes" (first-start.yaml:84-86). Every restatement says "instance document" (spec/promises/placement.yaml:126; spec/schemas/README.md:195; spec/core/README.md:531).

Reading 1, by the `when`: the Pool is instantiated with a second Resource made from the core's own `access` template, or with a Resource of `node.v1` that an actor made. The second is a node by definition (definitions.md:124) and so a valid `placement`.
Reading 2, by the `then`: the entry is rejected, but the promise gives no diagnostic and no moment for an entry.

PLC-028's observable says "A Pool's entry is made from a template too, so no entry makes a node either" (placement.yaml:456-457). That does not follow for an entry made from the core's own `node`, which PLC-028 exempts.

The first build takes reading 2, because documents and entries pass through one function (engine/crates/core/src/registration.rs:480-493, reached for entries from line 621). Its diagnostic reads "protected core template" and names neither the template nor the entry. The build also finds `access` and `management` by the template they were made from (engine/crates/core/src/access.rs:267-271 and 903; registration.rs:2365), so its ID reading in F1 depends on FST-006 covering entries.

No vector: no file under conformance/ holds an entry whose `from` is one of the five templates.

**Counterargument.** The `then` states the invariant plainly, so a careful builder rejects the entry, as the build does. `system` and `root` are not Resource templates, so only three of the five are reachable. An entry made from `access` or `management` would be inert under ACC-033.

**Least change.** FST-006's `when`: "An instance document's `from`, or the `from` of a Pool's entry, resolves to ...". Its `then`: the document, or the Pool template that holds the entry, is rejected when it is registered, with a diagnostic naming the template and the entry. An entry's `from` is never an input's destination (INS-012), so it is known at registration. Correct PLC-028's sentence to cite FST-006 for the core's own template. Add one invalid-by-rule example and one vector.

**Precedent.** INS-025, which already names both "an instance document" and "a nested instantiation" where a rule on `from` covers both.

**Who decides.** coordinator. Ruling 37: a defect whose fix adds no behavior. The `then` already requires the rejection and the build performs it.

**Verifier.** weakened, low. The quote is verbatim at spec/promises/first-start.yaml:81-82. An entry is a nested instantiation "with no `kind`, `metadata` or `spec`" (spec/definitions.md:87). INS-025 names the two apart (spec/promises/instantiation.yaml:353). POOL-0xx at spec/promises/pools.yaml:152 limits an entry only to "a nested instantiation of a Resource template". So `system` and `root` are unreachable as entries, and `node`, `management` and `access` are reachable. The restatements say "instance document": placement.yaml:126, spec/core/templates.yaml:25, definitions.md:145. No vector holds an entry made from any of the five; the three conformance files with both entries and these templates hold first-start snapshots only. Build: registration.rs:480-493 is reached for entries through :621. Its label comes from `metadata.name` (registration.rs:452), which an entry lacks, so the diagnostic names "unnamed". The spec's own checker takes the narrow reading: tools/check_schemas.py:1632 gathers only top-level documents with `from`, and :1700-1703 applies first-start-from to them alone. So two artifacts built from the text already diverge, as evidence of the readings.

The outcome is forced, though, which removes reading 1. PLC-008's `when` is "A node exists", and its `then` is "It is a Resource of the core type node.v1, owned by system ... The engine makes local at first start and every other node on admission" (placement.yaml:122-126). POOL-002's `then`: "One Resource is instantiated for each of its entries, owned by that Pool. If any of them cannot be instantiated, the Pool is not instantiated" (pools.yaml:33-34). So a Resource from core `node` as an entry of a project Pool cannot exist, and the Pool is not instantiated. For `access` and `management`, FST-006's `then` is universal: "are the only ones" (first-start.yaml:84-86). With POOL-002, the Pool again is not instantiated. Not unsafe. It is not in the first pass, AUDIT.md, cold-readers.md, IN_FORCE.md or the logs.

**Verifier's corrections.** Class: defect of wording and diagnostic. FST-006's `when` is narrower than its `then`. It is not two sound readings. Reading 1 ("a valid `placement`") contradicts PLC-008's `then` (placement.yaml:122-126) for `node`, and FST-006's own `then` for `access` and `management`.

The finding says the text gives "no moment". That is wrong: POOL-002 (pools.yaml:33-34) gives one. The Pool is not instantiated when it is instantiated.

What is missing is that FST-006 does not reach entries in its `when`, and no diagnostic naming the template and entry is promised for that case. Add this evidence: tools/check_schemas.py:1632 and :1700-1703 check only top-level instance documents. With the build's rejection, that shows the narrow `when` already misleads one artifact.

The least change stands: widen the `when` to "or the `from` of a Pool's entry", have the diagnostic name the template and the entry, and correct PLC-028's sentence at placement.yaml:456-457 to cite FST-006 and PLC-008. Moving the check to template registration is a refinement, not required. The checker's rule and definitions.md:145 / spec/core/templates.yaml:25 follow the same change.

Severity low. Coordinator decides under ruling 37.


## The verifier's notes

Both findings were checked at 2eb8f258 against spec/promises/first-start.yaml, templates.yaml, contracts.yaml, conduits.yaml, objects.yaml (OBJ-002, OBJ-011), pools.yaml (POOL-002 and the entry shape), placement.yaml (PLC-001, PLC-008, PLC-028), spec/conventions/access.yaml (ACC-001, ACC-006, ACC-029, ACC-030, ACC-033, the ACC-056 row), lifecycle.yaml (LIF-022) and operations.yaml (OPS-019, OPS-036). Also read: spec/core/templates.yaml:1-100, capabilities.yaml (the operation lists), types.yaml:88-104, conduit.yaml:180-201, definitions.md rows 34, 87, 124, 145-148, 186-187 and 204, and tools/check_schemas.py:1632 and 1690-1715.

Build evidence was read only at the lines cited: access.rs:262-275, 560-598 and 712-728, and registration.rs:430-496, 612-626, 1018-1030 and 1264-1272. Novelty was checked against the first-pass INDEX and X06, docs/AUDIT.md "Known to be open" (line 236), docs/proposals/projects/brief.md:2247-2257, and git grep of docs/, IN_FORCE.md, spec/ and the G5 file for FST-006, PLC-028 and second management or access Resources.

I did not rescan all of conformance/; I spot-checked it with git grep. Main outcome: F1 is real but known for its first route and fails closed. The by-Type reading behind its 'trap' is excluded by several `then` clauses, not only by two observables. F2's moment is given by POOL-002, and PLC-008 excludes reading 1 for `node`. The remaining defect is wording and the diagnostic, with the spec's own checker taking the narrow reading.
