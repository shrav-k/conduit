# L07-objects-naming: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The slice is clearest where it adopts a standard whole (YAML 1.2.2 core, two I-JSON rules, JSON Schema 2020-12 with asserted formats, RFC 6901, SemVer, RFC 8785) and in the object rules OBJ-001 to OBJ-011: fixed names, one namespace per kind, references pinned by ID, closed keys. A builder can follow those without asking.

It is not minimal. Three areas carry most of the text and most of the faults: defaulting (SCH-008 to SCH-010 and SCH-019, repeated in OPS-026, NAM-019 and NAM-024); the contracts of a submitted project (SUB-008, SUB-009, NAM-009, NAM-022's exception, with MAN-015, MAN-016 and PKG-029); and two routes for handing documents over (SUB-005 and SUB-010 beside `register.v1`). The second and third exist because a project has no identity after registration and contracts are not objects. The owner already holds that question under findings 4-01 and 4-11, and I would settle it before polishing these promises.

The systematic weakness is form: observables carry rejections and whole rules that when and then do not state, so "one obligation per promise" does not hold (F9). Layering is mostly right, but NAM-012 and SUB-010 put engine behavior (fetching, taking a directory, naming an actor) in the language layer.

I would cut or merge the records no engine can break: OBJ-024, SCH-014 (its observable names no one who checks), NAM-017, NAM-025 and SUB-007, which repeats OBJ-002. Two smaller points I did not raise as findings: OBJ-007 never says which reference places in `config` expect an instance, and NAM-026 reports unavailable for a mismatch that is permanent.

The weakest point is that rules precise for files stop at the file. Which keywords carry a schema to a place, schemas that never terminate, numbers and duplicate keys on the wire, the stored form of a template and the core package's version across an engine upgrade are each left to the builder. The first build has already filled several of these with choices the text does not state (F3, F4, F5).

One finding is unsafe: SUB-010 with DL7 lets an engine attribute a directory registration to an actor without that actor's proof (F1).

**What the reader read.** Commit 2eb8f258 (2026-11-12), read-only.

Read in full, line by line:
- spec/promises/objects.yaml (465 lines), naming.yaml (525), schemas.yaml (366), submissions.yaml (300)
- spec/definitions.md (all 467 lines, including "Words the promises do not use")
- spec/README.md (all 678 lines)
- spec/schema-profile.md
- docs/guides/full-review.md, and AGENTS.md "What holds"

Read for the not-new check:
- docs/AUDIT.md "Known to be open" and "Who decides what"
- docs/guides/cold-readers.md, lines 39-173
- IN_FORCE.md, only the lines marked Open or Spec pending (by search), not the whole file
- docs/proposals/first-build/README.md rows 1-01 to 1-18 and 2-02; log-increment-4.md rows 4-01, 4-03 and 4-11
- round 6's "Left for a decision"; docs/LANGUAGE_DEFECTS.md rows L16 to L19 and L118; ruling 52's record and the ledger rows for rulings 37, 40, 46 and 71

Cited text opened:
- Promises: CTR-001, CTR-006, FLEET-006, INS-002, INS-006, INS-013, INS-020, INS-024, INS-025, LIF-009, LIF-012, LIF-031, LIF-040, LIF-041, MAN-006, MAN-010, MAN-015, MAN-016, OPS-008 to OPS-010, OPS-023, OPS-026, OPS-033, PKG-005, PKG-006, PKG-020, PKG-023, PKG-027, PKG-029, PKG-031, REV-001 to REV-004, SEC-001, SEC-002, SCT-006, SCT-013, IFC-001, IFC-021, IFC-022 (when and then), IFC-032, ACC-057, and the PRV-007 passage on relative programs
- Implementation-defined items DL1 to DL12 and DL15
- Core package: spec/core/operations.yaml lines 1-290 (`register.v1`, `replace.v1`, `unregister.v1`), references.yaml lines 1-60, conduit.yaml head, core/README.md lines 36-52
- spec/schemas/values.yaml patterns

Not binding, opened only to see which reading the build took: tools/check_schemas.py (lines 178-192 and 1250-1290), parts of engine/crates/core/src (packages.rs, archives.rs, registration.rs, schemas.rs, records.rs), engine/CONFORMANCE.md's DL7 rows, and conformance/manifest.yaml.

Not read: the other promise, convention and interface files beyond the promises listed; spec/schemas/*.yaml beyond searches; the rest of spec/core; the conformance vectors themselves; ruling records other than 52. Nothing was built or run.

Recount for the slice: 84 promises. Objects 24 (22 language, 2 convention), naming 28 (24 and 4), schemas 21 (all language), submissions 11 (10 and 1), which matches the Contents table. Vector counts do not agree across sources:

| Source | Objects | Naming | Schemas | Submissions |
| --- | --- | --- | --- | --- |
| spec/README.md Contents | 13 | 17 | 10 | 10 |
| Manifest case tags | 15 | 17 | 10 | 10 |
| Manifest `s1_coverage` | 10 | 16 | 10 | 10 |

The promise records' own `vectors` field is filled in 1 of the 84 (SCH-009). I did not raise this as a finding because coverage is already on the known-open list.


## FR-080

**A directory registration is attributed to an actor the engine names, with no promise that the one handing it over proves to be that actor**

L07/F1 · gap · unsafe · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-122.

**Promises.** SUB-010, IFC-022, IFC-032, ACC-033, ACC-057

**Where.** spec/promises/submissions.yaml:270

**Quote.** "In an engine that keeps the conventions the registration is a request like any other, made for an actor that DL7 has the engine name, and is authorized under ACC-033."

**Situation.** SUB-010 makes every engine take a directory, and DL7 (spec/implementation-defined.yaml:62) leaves to the engine both how it is handed over, 'such as by a command or a call', and 'for which actor it then registers'. IFC-022 and IFC-032 establish a sender only for a request received 'on a server it presents to a consumer under IFC-012' (spec/interface/implementation.yaml:594, 859), so they do not reach this route. Steps to the harm: (1) an engine offers the hand-over on a local command or socket; (2) it names root as the actor, which ACC-057 forces for any project that holds an Agent; (3) any party that can reach the command, not only one holding root's credential, registers templates, instances and implementation items (programs, remote URLs) as root, and ACC-033 passes because root has the Conduits. Every promise is kept. The first build chose 'for an established actor' (engine/CONFORMANCE.md:25); the text does not require it.

**Counterargument.** DL7 says a choice must keep every promise, the hand-over is normally a local command run by the operator, and the stated limits already put every program of the engine's operating-system user inside the trust domain, so the usual deployment loses nothing.

**Least change.** Add to SUB-010's then: 'The actor for which it registers is established from what the hand-over presents, as IFC-022 establishes the sender of a request; a hand-over that establishes none is not dispatched and reports refused, as under IFC-032.' In DL7, replace 'and for which actor it then registers' with 'and how the one that hands it over presents its credential'.

**Precedent.** Kubernetes: `kubectl apply -f &lt;dir>` reads the directory on the client and sends authenticated API requests; the API server authenticates every request and never takes a caller's identity from the tool.

**Who decides.** coordinator. A gap whose fix extends an existing rule (IFC-022) to a second entry point; small, so ruling 71 point 4 with ruling 46 point 4, logged for the owner to strike.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/submissions.yaml:270-272. spec/implementation-defined.yaml:62-63: "Also how the engine is handed a directory under SUB-010, such as by a command or a call, and for which actor it then registers." The sender rules are scoped to the presented server: spec/interface/implementation.yaml:594 "The engine receives a request on a server it presents to a consumer under IFC-012"; :859-861 (IFC-032) the same; spec/promises/providers.yaml:550 (PRV-018) "The engine establishes which instance sent a request, under IFC-022". No promise in spec/conventions or spec/interface mentions a directory hand-over (grep for 'handed a directory', 'handed over'). ACC-057 (spec/conventions/access.yaml:899) forces root for any Agent. Harm worked through: an engine offers the hand-over by 'a call' (DL7), names root (DL7), a party with no root credential uses it, ACC-033 passes because root has the Conduits, and the record names root as caller (REC-002). Every promise is kept. Not recorded: no hit for the actor question in docs/AUDIT.md, cold-readers.md, IN_FORCE.md, the first-build logs, G5.md or the ledger; ruling 38 point 8 (docs/proposals/ruling-38/README.md:23) ruled only that a hand-over never installs. Nearest older record is L34 (docs/LANGUAGE_DEFECTS.md:181), on first start under DL6, which does not cover it. The build chose 'for an established actor' (engine/CONFORMANCE.md:25).

**Verifier's corrections.** Class and unsafe mark stand by the brief's test (access, and a wrong caller in the record). Exposure is narrow: a command run as the engine's operating-system user is already inside the stated trust domain (spec/README.md:424, :500), so harm needs a hand-over reachable from outside it. The least change should not go in SUB-010's `then`: SUB-010 is layer language and a Language-level engine has no actors. Put it in DL7 and a convention or interface promise beside IFC-022 and IFC-032. Coordinator under ruling 71 point 4 is defensible, logged for the owner to strike.


## FR-081

**No promise says what objects that keep an earlier core package version resolve through once the engine's built-in core package has another version**

L07/F2 · gap · verifier: confirmed, medium · decides: owner · new

Same fault, or overlapping: FR-073, FR-130.

**Promises.** NAM-009, NAM-010, NAM-011, NAM-020, NAM-023, PKG-005

**Where.** spec/promises/naming.yaml:145

**Quote.** "For the core package, which is built into the engine and so has no archive and no digest, it keeps the address and the version alone."

**Situation.** Every template and instance, the first-start objects among them, keeps `{address: core, version: 0.1.0}` (spec/core/conduit.yaml:21). An engine upgrade ships another core version. NAM-010 (naming.yaml:168) resolves a template's references 'through the packages the template holds'; NAM-011 (naming.yaml:185) says a version no longer installed is reported 'as not installed, and nothing is substituted'; NAM-020 (naming.yaml:328) resolves a core reference 'to the core package built into the engine'; NAM-023 has the contract 'as the target holds it' govern, and `management` holds the old version. Reading A: the built-in core replaces the kept one, so inspection lists a version that no longer governs. Reading B: nothing is substituted, so after an upgrade no existing template can be instantiated and requests to `management` are checked against contracts the engine no longer has. The draft already edits core contracts in place under 0.1.0 (spec/core/README.md:42; IN_FORCE.md:224).

**Counterargument.** The core package is unreleased and stays 0.1.0 during the draft, so no engine has yet met a second core version; the question can wait for the first release.

**Least change.** Add to NAM-020: 'A template or an instance that keeps an earlier version of the core package resolves its references to the core package through the one built into the engine as it now stands; the version it keeps says under which it was made. A release of the core package keeps NAM-024.'

**Precedent.** Go's standard library, which the head of naming.yaml already cites: a module records a `go` version and always uses the toolchain's library. Kubernetes serves built-in API types at the API server's version and converts stored objects.

**Who decides.** owner. It decides what an upgrade does to every existing object, which is behavior; rulings 37 and 46 point 4 leave a fix that adds behavior to the owner.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/naming.yaml:145-146. spec/core/conduit.yaml:21 "version: 0.1.0". naming.yaml:168-169 (NAM-010) "resolve through the packages the template holds"; :328-329 (NAM-020) "to the core package built into the engine"; :404 (NAM-023) "The contract as the target holds it governs." spec/promises/packages.yaml:74 keeps "the core package by its address and version alone". No text in spec/ says what a kept core version means once the built-in one differs (grep for 'built into', 'upgrade'; OBJ-024 at objects.yaml:440-447 is about specification versions and names only). The in-place edits are real: spec/core/README.md:42 "The unreleased core remains version 0.1.0", IN_FORCE.md:224. Not in AUDIT known-open, cold-readers, the limits, the logs or G5.

**Verifier's corrections.** Reading B should not lean on NAM-011: its `when` (naming.yaml:179) is about an installed package, and the core is never installed (NAM-021). The tension is between NAM-010 and NAM-023 on one side and NAM-020 on the other. A narrower older record exists: L41 (docs/LANGUAGE_DEFECTS.md:188) left 'repair ... after a runtime upgrade' to DL6; it does not cover ordinary templates and instances. Not reachable until a second core version ships, hence medium. Owner decides, as stated.


## FR-082

**Which schema keywords carry a reference schema, or a default, to a place is stated only for properties, items and anyOf/oneOf; the checker and the engine each chose differently**

L07/F3 · ambiguity · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-077, FR-125.

**Promises.** OBJ-021, SEC-001, SCH-008, SCH-009

**Where.** spec/promises/objects.yaml:370

**Quote.** "A reference schema governs a place where it is the schema of that place directly, where it is the `items` of a list that holds the value, or where it is a branch of `anyOf` or `oneOf` that the value satisfies"

**Situation.** The profile also admits `additionalProperties`, `patternProperties`, `prefixItems` and `contains` (spec/schema-profile.md:69). A Type with `mounts: {type: object, additionalProperties: {$ref: core#resource-reference.v1}}` and `mounts: {data: scratch}`: reading A, 'directly' covers any keyword that applies a schema to that member, so `scratch` is resolved, checked to exist and stored as `{kind, id}`; reading B, 'directly' means `properties` only (why else name `items`), so it is data, kept as a name and never pinned. For `secret-reference.v1` SEC-001 rejects 'any other applicator' (secrets.yaml:50), so A accepts the schema and B rejects the package. The same hole is in defaults: SCH-008 (schemas.yaml:102) lets the holder be 'written directly, reached through `$ref`, or be the schema that `items` gives'. The build shows the divergence: tools/check_schemas.py:186 allows the secret schema under `additionalProperties` but not `patternProperties`; the engine resolves references under `patternProperties`, `additionalProperties`, `prefixItems` and `contains` (engine/crates/core/src/registration.rs:892-933) and fills defaults through the first three (schemas.rs:626-656).

**Counterargument.** Author schemas that put references or defaults under these keywords are rare, and docs/LANGUAGE_DEFECTS.md:167 (L18) records the default question as ruled by SCH-008 to SCH-010.

**Least change.** In OBJ-021: 'A reference schema governs a place where it is the schema that `properties`, `patternProperties`, `additionalProperties`, `prefixItems` or `items` gives that place, or a branch of `anyOf` or `oneOf` there that the value satisfies'. In SCH-008 and SCH-009 name the same five keywords for the map that holds a defaulted property, and say `contains` gives neither. Make SEC-001's list of rejected applicators match.

**Precedent.** Kubernetes structural schemas, which SCH-008 already follows: defaulting and pruning descend through `properties`, `additionalProperties` and `items`, and nothing else.

**Who decides.** coordinator. An ambiguity whose fix states what the text leaves open and adds no feature: rulings 37 and 46 point 4.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/objects.yaml:370-373. spec/schema-profile.md:69-71 admits `prefixItems`, `contains`, `patternProperties`, `additionalProperties`. spec/promises/secrets.yaml:50-51 "a schema that puts it under any other applicator, or beside any other branch, is rejected", while :64-66 lists only `allOf`, `not`, `if`, `then`, `else`, `dependentSchemas` and `contains`. spec/promises/schemas.yaml:103-104 "may be written directly, reached through `$ref`, or be the schema that `items` gives". The two builds differ: tools/check_schemas.py:186 `SECRET_PLACES = ("properties", "additionalProperties", "items", "$defs")`; engine/crates/core/src/registration.rs:892-933 resolves references under `patternProperties`, `additionalProperties`, `prefixItems` and `contains`; engine/crates/core/src/schemas.rs:625-662 fills defaults through `patternProperties`, `additionalProperties` and `prefixItems`. Not settled: 1-04 and 1-17 (docs/proposals/first-build/README.md:60, :73) are about the regex dialect; L18 (docs/LANGUAGE_DEFECTS.md:167) says 'ruled' but SCH-008 to SCH-010 name none of these keywords.

**Verifier's corrections.** Add `contains` to the stated divergence: OBJ-021 is silent, SEC-001 rejects it for a secret reference, and the engine resolves ordinary references under it. The fix should also decide two unconditional defaults that reach one place through `properties` and `patternProperties`, or through two patterns: SCH-009's admission check (schemas.yaml:157-158) names only `$ref` siblings and `allOf`. Coordinator may settle; checker, engine and a vector must then agree.


## FR-083

**NAM-024's install-time rejection does not say which differences it tests; the text reads as a full compatibility diff and the first build compares defaults only**

L07/F4 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** NAM-024, NAM-025, PKG-023

**Where.** spec/promises/naming.yaml:436

**Quote.** "When installing a release while another at the same address is installed, a same-named contract that breaks this rule is rejected: installation reports invalid under PKG-023, with a NAM-024 diagnostic naming the contract and changed pointer."

**Situation.** 'This rule' is the whole then: a later contract 'may add optional things, such as' three examples, and 'does not remove or change anything the earlier one held' (naming.yaml:421-425). The list is open ('such as'), so no builder can compute it. A release 1.1.0 that removes an operation from a capability, tightens an input's `maximum`, adds `effect` to an operation, or only rewords `metadata.description`: reading A, the engine diffs the whole contract and rejects the install; reading B, only added, removed or changed defaults of a Type's schema and of a Requirement's inputs are compared, which is what the engine does (engine/crates/core/src/packages.rs:277-330, archives.rs:322-340) and all ten NAM-024 vectors test. The same archive installs on one engine and not on another. The check also depends on which releases happen to be installed, so two engines with different install histories differ too.

**Counterargument.** The must binds package authors in full; the last sentence can be read as naming only the engine's part, and the vectors show which part was meant.

**Least change.** Replace the last sentence: 'Where a release is installed while another at the same address is installed, a Type or Requirement contract of one name in both, in which a `default` is added, removed or changed at any place, is rejected: installation reports invalid under PKG-023, with a NAM-024 diagnostic naming the contract and the pointer. Installation checks nothing else of this promise, which binds the package's author.'

**Precedent.** Go's import compatibility rule binds module authors and is checked by publisher-side tools (gorelease, apidiff), not by `go get`; Protobuf uses `buf breaking` the same way.

**Who decides.** coordinator. Ambiguity; narrowing the sentence to what the vectors already test adds no behavior (rulings 37 and 46 point 4). A full diff at installation would be a feature, and the owner's.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/naming.yaml:436-439. The `then` at :421-425 has the open list "such as" and "does not remove or change anything the earlier one held". The build compares defaults only: engine/crates/core/src/packages.rs:277 "NAM-024/SCH-009: compare published defaults at resolved schema use sites", for Type schemas (:299) and Requirement inputs (:310). All ten vectors under NAM-024 are default cases (conformance/manifest.yaml:6543-6556: release-adds-default-rejected ... release-reorders-default-rejected). No other spec text states the install check (grep NAM-024 in spec/). No record in the first-build logs, AUDIT or the review list; ruling 55 A5 concerned defaults only (IN_FORCE.md:96).

**Verifier's corrections.** "'This rule' is the whole then" overstates: the sentence just before it (naming.yaml:435-436, "A default requires a new version segment") is a nearer antecedent and supports the narrow reading the build took. Both readings are grounded in the text, so the class ambiguity holds. Coordinator.


## FR-084

**No promise rejects a schema whose evaluation or default filling never ends**

L07/F5 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** SCH-001, SCH-010, SCH-015, SCH-019, OBJ-021, SUB-011

**Where.** spec/promises/schemas.yaml:171

**Quote.** "The defaults of that map's own properties then apply to it, as they would to a map that a document supplied."

**Situation.** Two documents the profile accepts. (1) `$defs: {a: {$ref: '#/$defs/b'}, b: {$ref: '#/$defs/a'}}`, or two Schema contracts whose `spec` is a `$ref` to the other: SCH-015 allows each `$ref`, SCH-001 defers to JSON Schema 2020-12, whose core text calls this undefined behavior, and OBJ-021 follows 'any number of `$ref`s' (objects.yaml:373). (2) `{type: object, properties: {child: {$ref: '#', default: {}}}}`: SCH-008 accepts the default, and SCH-010 with SCH-019 then fill `{}` with its own `child` default without end. Any actor that may register or install can submit either. One engine hangs or overflows, another accepts, and the first build rejects under SUB-011 as 'schema reference nesting 128' (engine/crates/core/src/registration.rs:651; schemas.rs:604), although SUB-011 says of its own limits that 'another engine that states larger limits accepts it', which no engine can do here.

**Counterargument.** Such schemas are author mistakes, every engine needs a recursion guard anyway, and DL10's stated limits give a deterministic answer per engine.

**Least change.** Add to SCH-015: 'A schema in which `$ref`s alone, with no keyword between them that applies to a part of the value, lead back to a schema already passed is rejected when its package is installed or it is submitted.' Add to SCH-019: 'A `default` whose filling under SCH-010 reaches that same `default` again is rejected in the same way.'

**Precedent.** JSON Schema 2020-12 core, 'Guarding Against Infinite Recursion': such schemas should not be used and their behavior is undefined. Kubernetes structural schemas exclude recursion by forbidding `$ref`; GraphQL rejects input-object cycles that cannot be satisfied.

**Who decides.** coordinator. A gap closed by two admission rules of the kind SCH-016 and SCH-019 already are; a small addition under ruling 71 point 4, logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/schemas.yaml:171-172. SCH-019 at :331-333 "A `default` that is a map is checked after the defaults of its own properties are filled in, as SCH-010 fills them", so `{properties: {child: {$ref: '#', default: {}}}}` never finishes filling. objects.yaml:373-374 "through any number of `$ref`s". No spec text on reference cycles (grep recurs, cycle, terminat, infinite across spec/promises, schema-profile.md, spec/schemas). The build rejects with a rule that does not fit: engine/crates/core/src/registration.rs:651 `fault("SUB-011", "schema reference nesting 128")`, schemas.rs:605; SUB-011's `when` (submissions.yaml:283-286) covers a document, file, submission or archive, and :297-298 says "another engine that states larger limits accepts it". Round 6's 6-1-04 (docs/proposals/reaudit/round-6/DECISIONS.md:89) settled recursion only for the bundle IFC-033 presents.

**Verifier's corrections.** None of substance. Not unsafe: the harm is a hang or a per-engine verdict on an author's mistake. Coordinator under ruling 71 point 4; the rule adds a rejection and cannot move S1's counts.


## FR-085

**The value model stops at files and documents: a duplicate key or an out-of-range number in a request's arguments is decided by no promise**

L07/F6 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** SUB-005, SCH-006, SCH-007, SCH-012, SCH-020, SCH-021, OPS-009, OPS-010, OPS-033

**Where.** spec/promises/submissions.yaml:84

**Quote.** "Where a registration hands the documents over as values, as the input `documents` of `register.v1` does, no file is in front of the engine and those rules have nothing to check; every other rule applies to the values."

**Situation.** A request is JSON text too. (1) The text of a `tools/call` gives one map the same key twice, in a document of `register.v1` or in any operation's arguments: SCH-012 is declared to have 'nothing to check', RFC 8259 leaves the result to the parser, and engines keep the first, keep the last or reject. (2) An argument of a control or of any operation that is not a document holds 9007199254740993, or a fraction that binary64 cannot hold: SCH-006, SCH-020 and SCH-021 each begin 'A document contains a number' (schemas.yaml:73, 340, 357) and no promise of the conventions or the interface cites them. REV-007 then stores 'exactly the values it was sent', which a binary64 parser cannot do, and DL2's suggested digest, 'the SHA-256 of the RFC 8785 form' (implementation-defined.yaml:23), gives 9007199254740993 and 9007199254740992 one digest, so the second request is answered as a repeat under OPS-009 where SCH-007 and OPS-010 require conflict.

**Counterargument.** The Model Context Protocol and JSON-RPC set no such rule, so restricting requests narrows the adopted wire; SCH-007's 'numbers by value' already decides equality for an engine that parses exactly.

**Least change.** Add one promise beside OPS-033: 'A request whose JSON text gives one map the same key twice, or whose arguments hold a number outside the ranges of SCH-006 and SCH-021, cannot be read and reports invalid. A number with a fractional part in a request is read as SCH-020 reads one.' Strike 'those rules have nothing to check' for SCH-012 in SUB-005.

**Precedent.** I-JSON, RFC 7493, written for protocol messages and already the profile's value model; RFC 8785 requires I-JSON input.

**Who decides.** coordinator. A gap closed by applying to the wire the two I-JSON rules the profile already applies to files; small, under ruling 71 point 4, logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/submissions.yaml:84-87. SCH-006, SCH-020 and SCH-021 each begin "A document contains a number" (spec/promises/schemas.yaml:73, :340, :357); no file under spec/conventions, spec/interface or spec/core cites SCH-006, SCH-012, SCH-020 or SCH-021 (grep returns nothing). spec/schema-profile.md:22 states the value model for files. OPS-009 and OPS-010 compare arguments "under SCH-007" (spec/conventions/operations.yaml:207, :251). spec/implementation-defined.yaml:23-24 suggests "The SHA-256 of the RFC 8785 form". No record in the first-build register (1-08, 1-10, 1-12 at docs/proposals/first-build/README.md:64-68 concern files), AUDIT or the logs.

**Verifier's corrections.** The number half is partly decided: SCH-007's equality is by value, so 9007199254740993 and 9007199254740992 are unequal and OPS-010 gives conflict; what is missing is how a request's numbers are read and their range, and DL2's suggested digest is then unsound for them. SUB-005's "every other rule applies to the values" already carries SCH-006, SCH-020 and SCH-021 to documents in `register.v1`. Who decides: rejecting out-of-range numbers in any request narrows what an existing MCP server's tool can be sent (64-bit IDs as numbers), which is behavior and the owner's. The duplicate-key half is the coordinator's.


## FR-086

**'Equal to the template as stored' has no defined stored form, so whether a replace advances the revision differs between builders**

L07/F7 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** SUB-009, OBJ-018, INS-024, REV-003, REV-004

**Where.** spec/promises/submissions.yaml:193

**Quote.** "A request that leaves the template as it was, because its document is equal to the template as stored and the template would keep under NAM-009 exactly what it keeps, changes nothing, does not advance the revision and has the outcome succeeded"

**Situation.** Case 1: `replace.v1` with a document that differs from the registered one only in `metadata.description`. Reading A: it is not equal under SCH-007, so it is a change and REV-003 advances the revision. Reading B: OBJ-018 says such documents 'resolve to the same object' (objects.yaml:310), and REV-004 forbids an advance unless the 'stored definition or configuration' changes, a term that spec/definitions.md:383 defines without metadata and without `inputs`; so nothing changes. Case 2: the template was registered with no `config` and the replacing document writes `config: {}`. INS-024 says both give 'the same template', but an engine that stores the document as written sees unequal values. After either case a second editor's replace that names the old revision succeeds on one engine and reports conflict under REV-002 on the other. The manifest notes that no vector varies description or annotations (conformance/manifest.yaml:5802).

**Counterargument.** The difference is a revision value and one spurious conflict or missed conflict; no access changes, and a vector would settle it.

**Least change.** Add to SUB-009: 'The template as stored is the framed document it was registered or last replaced with, read as INS-024 and OBJ-014 read what is left out. `metadata.description` and `annotations` are part of it: a document that differs only there is a change, and the revision advances.' Restate OBJ-018's observable as 'mean the same' in place of 'resolve to the same object'.

**Precedent.** HTTP entity tags (RFC 9110), which REV-001 cites: a strong validator changes whenever the stored representation changes. Kubernetes `metadata.generation`, which moves on `spec` alone, is the other existing choice.

**Who decides.** coordinator. Ambiguity between SUB-009, OBJ-018 and REV-004 whose fix picks one existing reading and adds no feature: rulings 37 and 46 point 4.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/submissions.yaml:193-196; :198-199 adds "equal, under SCH-007, to the template as stored". No text defines the stored form (grep 'template as stored' finds only submissions.yaml:194, :199 and spec/core/operations.yaml:207). spec/promises/objects.yaml:310-311 "Two documents that differ only in `description` or `annotations` resolve to the same object"; spec/promises/instantiation.yaml:343-344 "gives the same template"; spec/conventions/revisions.yaml:50 (REV-004) "An object's stored definition and configuration do not change"; spec/definitions.md:383 lists type, `implementation` map, controls, nested instantiations and a Conduit's capability, operations and requirements. conformance/manifest.yaml:5802 "no portable variant changes description/annotations". No settlement in the first-build logs, AUDIT or the review list.

**Verifier's corrections.** On one branch this is a contradiction, not only an ambiguity: see the extra. The fix must also align the term 'definition' (or REV-003 and REV-004) with what SUB-009 counts as a change. Coordinator if it picks an existing reading.


## FR-087

**A generated name need only differ from existing objects, not from tombstones or from names in the same submission**

L07/F8 · gap · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-059.

**Promises.** OBJ-003, LIF-041, SUB-004, SUB-001, NAM-027

**Where.** spec/promises/objects.yaml:52

**Quote.** "The template's part is shortened where the whole name would otherwise be longer than 63 characters, and the name is one no existing object of that kind has."

**Situation.** DL15 leaves the suffix to the engine and requires only OBJ-001 and OBJ-002 (implementation-defined.yaml:162); it does not require randomness. (1) An unnamed instance `worker-1` is retired and removed; its tombstone keeps the name (LIF-040). The next unnamed instance may again be named `worker-1`, since no existing object has it, and LIF-041 then reports conflict ('Another instance matches the kind and name the tombstone holds', lifecycle.yaml:864) for a name the author never wrote and cannot change, until someone lifts the tombstone. (2) One submission holds an unnamed instance of `worker` and a document named `worker-1`: the generated name may equal it, SUB-004 rejects the submission, and whether it does depends on the order of processing, against SUB-001. NAM-027 excludes unnamed instances from its check (naming.yaml:490).

**Counterargument.** With the five random characters DL15 suggests a collision is improbable, and a sensible engine draws again; docs/LANGUAGE_DEFECTS.md L118 treats generated-name collisions as ruled by OBJ-003.

**Least change.** In OBJ-003's then: 'and the name is one that no existing object of that kind has, that no tombstone holds for that kind under LIF-040, and that no other document of the same submission gives.'

**Precedent.** Kubernetes `generateName`: the API server generates the name and a collision is retried or reported, never left to the author.

**Who decides.** coordinator. A gap whose fix widens one clause and adds no feature: rulings 37 and 46 point 4.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/objects.yaml:51-53. spec/implementation-defined.yaml:166-167 "Whatever is chosen, the name satisfies OBJ-001 and OBJ-002"; objects.yaml:39-40 "A name held by no existing object may still be held by a tombstone; LIF-041 then gives conflict." spec/conventions/lifecycle.yaml:864 "Another instance matches the kind and name the tombstone holds." spec/promises/naming.yaml:490 "An instance document that gives no name has none to share." The build retries only on collision (engine/CONFORMANCE.md:42 "retried on collision"). L118 (docs/LANGUAGE_DEFECTS.md:237) predates tombstones. Not in the limits (spec/README.md:392-394 speaks of stable names only).

**Verifier's corrections.** Small. With the random suffix DL15 suggests it is improbable; it is certain only for a deterministic generator, which DL15 permits. The SUB-001 argument is weak, since generated suffixes already differ between runs. The tombstone half matters most for a Pool entry's Resource replaced under LIF-009, where `register-replacement.v1` lifts nothing. Coordinator.


## FR-088

**Rules and rejections stand only in observables, and several promises hold many obligations, against the form the specification sets for itself**

L07/F9 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** SCH-009, SUB-005, NAM-012, NAM-022, NAM-024, OBJ-009, SUB-010

**Where.** spec/promises/schemas.yaml:155

**Quote.** "Independently of whether a supplied value needs a default, a document is rejected under SCH-009 when installed or submitted if schemas that apply to one place unconditionally, through a `$ref` and its siblings or through `allOf`, give different values of `default` there."

**Situation.** spec/README.md:291 calls `then` 'The single obligation' and :297 says 'One obligation per promise'. SCH-009's when is 'A value is supplied'; this admission rule, which has two vectors, is outside it and exists only in the observable. SUB-005's when is a submission holding 'something other than' documents, and its observable alone carries the attribution of every schema rejection, the UTF-8 rule, which files of a directory are read and that the wire takes framed documents only. NAM-012, layer language and level must, states six things in its then, among them what an engine fetches and a 'may'. NAM-022's resolution rules, NAM-024's install rejection, OBJ-009's rejection of an authored resolved form and SUB-010's rejection of an unreadable entry are observables too. A builder who implements when and then misses these; a count of 519 obligations, or of safety promises, understates; and a Language-level claim ('documents in, resolved objects or diagnostics out') cannot be judged on NAM-012's fetching.

**Counterargument.** A careful builder reads observables as binding, the vectors test several of them, and splitting would add IDs and churn without changing behavior.

**Least change.** Give each rule that only an observable states a promise of its own with its own when (the conflicting-defaults check, SUB-005's file rules, NAM-024's install check), keep every existing ID, and move NAM-012's fetching and its 'may' to the convention promises on installation (PKG-023, PRV-017).

**Who decides.** coordinator. A defect of form whose fix adds no behavior: rulings 37 and 46 point 4.

**Verifier.** weakened, low. Quote verbatim at spec/promises/schemas.yaml:155-158; spec/README.md:291 "The single obligation" and :297 "One obligation per promise". The instances check out: SCH-009's `when` (schemas.yaml:125) is "A value is supplied"; NAM-024's install rejection is observable-only (naming.yaml:436-439); OBJ-009's rejection of an authored resolved form (objects.yaml:136-137) sits under the `when` "A template is instantiated"; NAM-012 (naming.yaml:191-203), layer language, states engine fetching and a 'may'. But the general point is on record: docs/proposals/reaudit/round-6/design-critique.md:595 "Treat `observable` as part of the obligation review: several important decisions live there rather than in `then`", in D21, accepted by ruling 44 (docs/proposals/reaudit/round-6/DECISIONS.md:125). spec/README.md:292 makes the observable "What can be checked to tell whether the promise was kept", and docs/implementation/SAFETY_CATALOGUE.md (e.g. :39-61) classes promises by quoting observables.

**Verifier's corrections.** Smaller than stated: a builder is not free to skip observables, since they are the test of the promise, so 'a builder who implements when and then misses these' does not follow. What is new is narrower: rules whose situation lies outside their promise's `when` (SCH-009's admission check, NAM-024's install check, OBJ-009's authored resolved form) and NAM-012's engine behavior under layer language. Defect of form stands for those; coordinator.


## FR-089

**Every schema rejection carries ruleId SUB-005, while the object promises say the diagnostic names the rule broken**

L07/F10 · contradiction · verifier: known, low · decides: coordinator · known-new-situation

**Promises.** SUB-005, OBJ-001, OBJ-012, OBJ-018, OBJ-020, IFC-050

**Where.** spec/promises/submissions.yaml:67

**Quote.** "A document, or the manifest, that a Schema contract rejects is rejected under this promise, whichever promise that contract encodes: the diagnostic names the place in the document by a JSON Pointer, in a logical location, and its message names the Schema contract."

**Situation.** OBJ-001's observable says resolving returns 'a diagnostic naming the document and the rule its name breaks' (objects.yaml:24). A document named `Reviewer` is rejected by the pattern of `name.v1`, so under SUB-005 the diagnostic has `ruleId: SUB-005`, a pointer, the schema's name and a keyword location; nothing names OBJ-001 or says what a name may be, and SUB-005's own when ('something other than framed templates, instances, contracts') describes another fault. The attribution was settled in the first build (docs/proposals/first-build/README.md:57 and :70, 1-01 and 1-14). The new situation: ruling 52 point 2 asks that a diagnostic 'say what happened and why, in full', the callers being agents, and an agent author given SUB-005 for nearly every shape fault must decode a keyword location to learn the rule.

**Counterargument.** 1-14 chose this on purpose: attribution to the encoded promise 'varies across engines', and the keyword location with the schema's name does identify the fault exactly.

**Least change.** Either restate the observables of OBJ-001, OBJ-012, OBJ-017, OBJ-018, OBJ-020 and OBJ-022 as 'rejected under SUB-005, with a diagnostic naming the place, the Schema contract and the failing keyword', or add to SUB-005: 'its message also states the rule in words, taken from the `description` of the subschema that failed'.

**Precedent.** SARIF 2.1.0 pairs `ruleId` with rule metadata and a message; JSON Schema validators surface a subschema's `title` and `description`, and Kubernetes CRD validation rules carry a `message`.

**Who decides.** coordinator. A contradiction between observables; the first fix adds no behavior (rulings 37 and 46 point 4) and the second is a small addition under ruling 71 point 4.

**Verifier.** known, low. Quote verbatim at spec/promises/submissions.yaml:67-70. Settled in the first build: docs/proposals/first-build/README.md:57 (1-01) "SUB-005 attributes every schema rejection, completed by 1-14" and :70 (1-14) "Schema-encoded promise attribution varies across engines". The claimed new situation is decided by spec/interface/implementation.yaml:1339-1341 (IFC-050): "It says what happened and why in full: the operation, what the request named, the reason or outcome, the deciding promise's ID and the cause in words." No object promise fixes a ruleId: objects.yaml:24 says "a diagnostic naming the document and the rule its name breaks", and others already say 'under SUB-005' (objects.yaml:255, :340; schemas.yaml:28, :42, :244).

**Verifier's corrections.** Not a contradiction. The residue is wording: OBJ-001's "the rule its name breaks" could say 'rejected under SUB-005, naming the place and the Schema contract'. IFC-050 binds only results given to a consumer or an Interface, so a Language-level engine's diagnostics have no 'cause in words' rule; that is minor. Coordinator, if touched at all.


## FR-090

**The exception that makes `#name` as `from` mean the bare name in a submitted project does not deliver what it is for**

L07/F11 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** NAM-022, INS-025, NAM-028, OBJ-008

**Where.** spec/promises/naming.yaml:366

**Quote.** "There is one exception: in a submitted project, `#name` written as `from` resolves as the bare name does under INS-025, among the templates that exist, of which the templates that the same submission registers are part."

**Situation.** INS-025 gives the purpose: a set of documents 'reads alike as a project and as a package' (instantiation.yaml:367). It does not. In a package, a declared Conduit's `from: '#read'` reaches that package's template at the pinned version, and a name that is missing is rejected under NAM-028. In a submitted project the same line is a second spelling of `from: read`: it reaches any existing Conduit template of that name, one from another submission among them, is resolved again each time the holding template is instantiated, so a later replace of `read` changes what new instances get, and a misspelt name is only reported under OBJ-008 (naming.yaml:519-522). Publishing a project therefore changes its meaning from late-bound to pinned, and testing it as a project does not test the package. The exception costs text in NAM-022, INS-025, NAM-028, SUB-009 and two rows of definitions.md, and a special case in every engine.

**Counterargument.** A project's templates are registered objects and a project has no identity after registration, so resolving by name is the only rule the engine can keep at later instantiations; without the exception, package documents could not be submitted unchanged.

**Least change.** In NAM-022: 'In a submitted project, `#name` written as `from` names a template that the same submission registers; where none does, the document is rejected under NAM-028. A template that an earlier submission registered is named by its bare name.' Later instantiations stay as INS-025 has them.

**Precedent.** Go: a name local to a module never resolves outside it, and anything outside is named by its import path. Helm resolves a chart's own template names within the chart.

**Who decides.** owner. It changes what an accepted notation means and removes a behavior; a fix that adds or removes a feature is the owner's (rulings 37 and 46 point 4). It belongs with findings 4-01 and 4-11, which the owner holds.

**Verifier.** weakened, low. Quote verbatim at spec/promises/naming.yaml:366-368. The differences the reviewer lists are in the text: naming.yaml:384-386 "resolves in the same way whenever the template that holds it is instantiated"; :519-522 "where none has the name, OBJ-008 registers a template whose nested instantiation writes it"; spec/promises/instantiation.yaml:367-368 "therefore reads alike as a project and as a package". But restricting it was considered and advised against: docs/proposals/reaudit/round-6/design-critique.md:601-603 "Restricting submitted-project `from: '#name'` is a possible low-priority synonym removal, but would reduce source portability into packages; do not change it merely to reduce spelling count", in D21, accepted by ruling 44 (docs/proposals/reaudit/round-6/DECISIONS.md:125).

**Verifier's corrections.** Class: a defect in INS-025's observable, which overclaims (late-bound against pinned; a missing name reported under OBJ-008 against rejected under NAM-028), not needless complexity already shown. The proposed least change breaks replacement: `replace.v1` hands over contracts only (submissions.yaml:223-225), so a replacing document's `#name` as `from` would name nothing in 'the same submission' and be rejected, against naming.yaml:384-388. The coordinator may correct the overclaiming sentence; any change to what the notation resolves to is the owner's, with 4-01 and 4-11.


## Further points the verifier noticed

Not verified by a second reader.

- **SUB-009 advances the revision for changes that REV-004 and the term 'definition' say must not advance it** (spec/promises/submissions.yaml:204-207; spec/conventions/revisions.yaml:46-52; spec/definitions.md:383). Contradiction, not unsafe, medium. SUB-009: "An equal document handed over with a project whose digest differs ... is a change: the template as replaced keeps the new ones, and the revision advances." REV-004 forbids an advance where "An object's stored definition and configuration do not change", and definitions.md:386 says the revision "advances exactly when the stored definition or configuration changes". The term 'definition' (definitions.md:383) lists the type, the `implementation` map, controls, nested instantiations and a Conduit's capability, operations and requirements. It lists neither a template's `inputs` nor what the template keeps under NAM-009. So a replace that changes only an input's default, or only a package pin, must advance under SUB-009 and must not under REV-004. Least change: widen the term to everything a template's `spec` declares and what it keeps under NAM-009, or restate REV-003 and REV-004 in those words. Coordinator (no behavior added). Found while checking F7.

## The verifier's notes

Checked at commit 2eb8f258 (2026-11-12), read-only; nothing was built or run. All eleven quotes are verbatim at the cited lines, and every path:line the reviewer gave for supporting text checked out.

Read in full: the four slice files, spec/implementation-defined.yaml, the brief, and the 'form of a promise' and 'Limits of this version' sections of spec/README.md. Read in part, by the cited promise: IFC-001, IFC-021, IFC-022, IFC-032, IFC-036, IFC-050, ACC-001, ACC-033, ACC-056, ACC-057, PRV-018, SEC-001, SEC-008, PKG-005, INS-024, INS-025, FST-005, LIF-040, LIF-041, REV-003, REV-004, REV-007, REC-002, OPS-008 to OPS-010, OPS-026, OPS-033, and the definitions rows they use. Engine and checker code was read only at the lines the reviewer cited.

Newness was checked by search, not by a full read, of docs/AUDIT.md (known to be open, read in full), docs/guides/cold-readers.md (known-open list, read in full), IN_FORCE.md, the first-build register and increment 4 log, the review list, G5.md, the ledger, docs/LANGUAGE_DEFECTS.md and round 6's decisions and design critique. Two findings (F9, F11) are weakened by round 6's D21, which the reviewer did not cite and which sits outside the lists the brief names; it was accepted by ruling 44.

Totals: 8 confirmed (F1, F3 high; F2, F4, F5, F6, F7 medium; F8 low), 2 weakened (F9, F11), 1 known (F10), none refuted. One extra, found while checking F7.
