# L11-core-rest: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The core package is mechanically sound at 2eb8f258: its 147 exports match its 147 documents, `engine` implements every operation of the four management capabilities, and the operation lists of `root-pool` and `root-fleet` match FLEET-005. Its weakest point is implementation-defined.yaml: DL13 and DL17 hold safety obligations (at most one part of the engine acts on an invocation; decisions read the state as it stands) that LD5 puts in promises, and DL7 lets an engine name the actor of a directory hand-over without establishing it. The second weak point is results.yaml, the shapes of what Conduit claims as its own: record members and types are each engine's, a refusal's `cause` may be empty, and a diagnostic that does not reject has no fixed level; ruling 52 argues for pinning these before batch 1's refusals are built. The slice is not minimal in its prose: the same rule is stated in a promise, a definitions row, a contract description, a head comment and the core README, and the restatements have drifted (F11); I would cut them to citations. I would also cut members that carry no information in this version (inventory.v1's `directoryNamed` and `enforcement: neither`, tombstone.v1's `within`), and the core package's own `version` unless F10 gives it a meaning. Layering is otherwise right: bound.v1 and shared-host.v1 take their meaning from promises and not from their descriptions, and the provider capabilities are plain contracts. Smaller points I did not report: provenance.v1's \"ID order\" has no defined comparison for opaque IDs, and refusal.v1 and inventory.v1 name one Conduit state in two vocabularies (open and closed against ready and retired). Situations a deployment meets that this slice leaves undecided are an engine upgrade over existing state (F10), a state directory under another runtime provider (F3), and a change of the engine's address or the loss of a signing key while runs hold credentials that never lapse (DL13), which I did not examine in depth.

**What the reader read.** Commit 2eb8f258 (2026-11-12), read-only. Read in full, line by line: spec/core/capabilities.yaml, spec/core/conduit.yaml, spec/core/references.yaml, spec/core/requirements.yaml, spec/core/results.yaml, spec/core/templates.yaml, spec/core/types.yaml, spec/implementation-defined.yaml, spec/names.yaml, spec/definitions.md (all 467 lines), spec/README.md (all 678 lines), docs/guides/full-review.md, and the \"What holds\" section of AGENTS.md. Read in part: spec/core/README.md (lines 1-80, 303-395, 482-536, 692-717); docs/AUDIT.md (known to be open, layered claim, layer decisions, row A39); docs/guides/cold-readers.md (what is known to be open); IN_FORCE.md (lines marked spec pending, and lines on DL items). Cited promises opened in full: ACC-001, ACC-029, ACC-030, ACC-033, ACC-044, ACC-056, CON-006, CON-021, CON-022, CON-024, CON-025, CTR-003, CTR-004, CTR-014, ENR-006, ENR-007, ENR-010, FLEET-001, FLEET-005, FST-001, FST-002, FST-005, FST-006, FST-007, IFC-012, IFC-021, IFC-022, IFC-025, IFC-032, IFC-050, INS-006, LIF-013, LIF-040, LIF-041, MAN-012, MAN-018, NAM-008, NAM-009, NAM-020, NAM-021, OBJ-006, OBJ-008, OBJ-009, OBJ-021, OBJ-023, OBJ-024, OPS-005, OPS-007, OPS-008, OPS-019, OPS-023, PKG-005, PLC-013, PLC-026, PND-001 to PND-004, PND-008, PND-012, POOL-014, PRV-029, REC-001, REC-002, SUB-010, TPL-004; and parts of IFC-033, IFC-049 and PND-010. Counts recomputed with /usr/bin/python3 and PyYAML: exports against documents, capabilities against `engine`'s implementations, and each Type's `requires` against its offered capabilities. Not read: spec/core/operations.yaml (searched only), spec/schemas/, schema-profile.md beyond a search, and the rest of the promise files. Not a standard for any finding, and read only as evidence that two readings exist: engine/crates/core/src/access.rs lines 70-135, engine/crates/core/src/registration.rs lines 818-930, engine/CONFORMANCE.md rows DL7, DL9 and DL17, and packages/provider-ssh-runtime/contracts.yaml lines 36-46. Nothing was built, run or changed.


## FR-121

**A Pool named in a bound's `targets`: what it covers, and whether a bound naming it in `pools` is within it**

L11/F1 · ambiguity · unsafe · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-001.

**Promises.** ACC-029, ACC-030, CON-028

**Where.** spec/conventions/access.yaml:521

**Quote.** "A new bound is within an old one when every Pool and every individual target it names is covered by the old one, as ACC-029 states, and every operation it names is one the old one names."

**Situation.** bound.v1 takes a Pool in two places: `pools` (spec/core/requirements.yaml:62) and `targets`, whose items are target-reference.v1 and so accept `{kind: Pool}` (requirements.yaml:67). ACC-029 and bound.v1 say "Naming a Pool covers the Pool itself and every Resource it owns" and settle only a Fleet in `targets` ("covers that Fleet alone"). Reading A: a Pool in `targets` is an individual target and covers the Pool alone (the first build: engine/crates/core/src/access.rs:88). Reading B: naming a Pool anywhere covers its Resources, so an author who delegates the Pool alone delegates every Resource in it. Under A, ACC-030's `then` as written admits a widening. Steps: X's right to open has the bound targets: [Pool shared, Resource r1], operations: [files#read.v1]. X asks open.v1 for a Conduit to `access` for itself with the bound pools: [shared], operations: [files#read.v1]. "Every Pool it names is covered by the old one" holds, since the old bound covers `shared` as a target, so the request is dispatched. X then opens read Conduits to every Resource of `shared`. ACC-030's observable ("its bound covers nothing that the bound of the actor that opened it does not cover") forbids that result, so `then` and observable disagree.

**Counterargument.** ACC-029 calls the items of `targets` individual targets, so reading A is the natural one, and ACC-030's observable lets a careful builder reject the widening. The first build takes reading A and requires each new Pool to be in the old `pools`.

**Least change.** ACC-029 and bound.v1: "A Pool named in `targets` covers that Pool alone, as a Fleet does; only an item of `pools` covers the Resources a Pool owns." ACC-030: "...when every Pool it names in `pools` is one the old one names in `pools`, every individual target it names is covered by the old one..."

**Precedent.** Kubernetes RBAC escalation prevention: a role may be granted only if each of its rules is covered by a rule the grantor holds, compared rule by rule with `resources` and `resourceNames` kept apart.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix writes the reading that ACC-030's observable and the first build already take, and adds no behavior.

**Verifier.** confirmed, high. Quote verbatim at spec/conventions/access.yaml:521-523. ACC-029 never says what a Pool written in `targets` covers: access.yaml:479-482 "It names Pools, individual targets and operations. Naming a Pool covers the Pool itself and every Resource that Pool owns ... An individual target is covered when the bound names it."; only the Fleet case is settled, access.yaml:509-510 "A Fleet named in `targets` covers that Fleet alone and none of its Agents." `targets` accepts a Pool: spec/core/requirements.yaml:67-70 with spec/core/references.yaml:325-331. ACC-030's own observable is as loose as its `then`: access.yaml:534-535 "may name a Pool only if the old one names that Pool" (the old one does name it, in `targets`); only access.yaml:536-537 "its bound covers nothing that the bound of the actor that opened it does not cover" forbids the widening. definitions.md:186 repeats the loose rule ("when every Pool and every target it names is covered by the other"). The first build takes reading A and the stricter check: engine/crates/core/src/access.rs:88-94 (`covers`) and :99-100 (new `pools` must be in old `pools`). No vector writes a Pool in a bound's `targets`. Not recorded: no hit in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md, first-build logs, G5.md, the ledger, or docs/proposals/spec-batch-1/areas/fleet.md (which holds only the Fleet sentence, lines 71 and 159).

**Verifier's corrections.** Class holds, but it is two faults in one: an ambiguity (what a Pool in `targets` covers) and, under reading A, a defect in ACC-030 whose `then` and whose Pool sentence in the observable both admit a widening that its last observable sentence forbids. Unsafe mark holds, with this qualification: an engine that keeps the whole observable cannot widen; the harm comes from a builder who follows `then` literally, or from reading B, under which an author who names a Pool in `targets` to delegate the Pool alone delegates every Resource in it. The least change must also correct definitions.md:186 and the Pool sentence at access.yaml:534-535 ("names that Pool in `pools`"). Coordinator may fix: the reading written is the one the first build and access.yaml:536-537 already take.


## FR-122

**The actor for whom a handed-over directory is registered is the engine's choice, with no rule that it be established**

L11/F2 · ambiguity · unsafe · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-080.

**Promises.** SUB-010, DL7, IFC-022, IFC-032, ACC-033, ACC-057

**Where.** spec/implementation-defined.yaml:62

**Quote.** "Also how the engine is handed a directory under SUB-010, such as by a command or a call, and for which actor it then registers."

**Situation.** SUB-010 says the registration "is a request like any other, made for an actor that DL7 has the engine name" (spec/promises/submissions.yaml:271). IFC-022 and IFC-032, which have the engine establish the sender from what a request presents, bind only "a request on a server it presents to a consumer under IFC-012" (spec/interface/implementation.yaml:594, 859). Reading A: the hand-over's actor is established as any request's is (the first build: "The runner supplies directory documents for an established actor", engine/CONFORMANCE.md:25). Reading B: the engine documents which actor it uses, such as root for every hand-over. Steps under B: an engine documents under DL7 that a directory is handed over by a local or network call and registered for root. Whoever can make that call registers any object, Agents included (ACC-057 lets root), without presenting root's credential. ACC-033 is checked against root's Conduits and passes. No promise is broken.

**Counterargument.** A choice under a DL item must keep every promise, SUB-010 calls the registration a request like any other, and a process of the engine's operating-system user can already read root's token (a stated limit), so a local command adds no reach for that user.

**Least change.** SUB-010: "...made for the actor that the engine establishes for the hand-over as IFC-022 establishes the sender of a request; a hand-over from which it establishes none reports refused under IFC-032." DL7: replace "and for which actor it then registers" with "and how whoever hands it over presents that actor's credential".

**Precedent.** kubectl apply -f &lt;directory>: the client reads the directory and sends requests that the API server authenticates like any other; no path registers objects for an identity the server did not authenticate.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; the fix applies IFC-022's existing rule to the hand-over and adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at spec/implementation-defined.yaml:62-63. SUB-010's only words on the actor are in its observable, spec/promises/submissions.yaml:270-272: "the registration is a request like any other, made for an actor that DL7 has the engine name, and is authorized under ACC-033." The rules that establish a sender bind only requests on an IFC-012 server: spec/interface/implementation.yaml:594 "The engine receives a request on a server it presents to a consumer under IFC-012" and :859 likewise; PRV-018 (spec/promises/providers.yaml:549-553) is conditioned on IFC-022. No promise in spec/promises, spec/conventions or spec/interface says how a hand-over's actor is established (grep for 'establish' finds only IFC-022/IFC-032/ACC-037/REC-002 contexts). The first build takes the safe reading: engine/CONFORMANCE.md:25 "The runner supplies directory documents for an established actor". Not recorded: finding 4-03 (docs/proposals/first-build/log-increment-4.md:14) is about relative values, not the actor; no hit for DL7's actor in AUDIT.md, cold-readers.md, IN_FORCE.md, the review list or the ledger.

**Verifier's corrections.** As stated. Unsafe mark holds only in the strict sense: the harm needs an engine to offer a hand-over path outside IFC-012 that someone other than the engine's operating-system user can reach, and to document root as its actor; for a local command run by that user nothing new is reached (stated limit, spec/README.md:424-426). Hence medium, not high. Class could as well be gap (no promise decides how the hand-over's actor is established). The least change must keep SUB-010 a language promise: the sentence sits under "In an engine that keeps the conventions", so the rule belongs in a convention or interface promise that SUB-010 and DL7 cite. Coordinator decides.


## FR-123

**A state directory under any runtime provider but the engine's own runner has no fixed name, privacy or removal**

L11/F3 · gap · unsafe · verifier: confirmed, medium · decides: owner · new

**Promises.** PRV-029, MAN-018, DL16, LIF-035, PND-005

**Where.** spec/names.yaml:38

**Quote.** "The instance's state directory, supplied by the engine's own runner (PRV-029)."

**Situation.** PRV-029 binds the engine's own runner: a directory of the instance's own, private, the same on every run, at a path with the instance ID that is never reused, removed with the instance. Of the rest it says only "Another runtime provider supplies such a directory only where its contract states it does" (spec/promises/providers.yaml:979). MAN-018 still reserves the name in every template. Reading A: "such a directory" has all those properties. Reading B: only that a directory is supplied; the repository's SSH runtime provider takes B: "kept across starts and never removed (PRV-029)" (packages/provider-ssh-runtime/contracts.yaml:43). Steps: an Agent package keeps its login in the state directory (PRV-029 names "credential storage"). The Agent runs under such a provider on a second machine, signs in, is retired and removed. Engine and provider have kept every promise, and the login stays on that machine with no instance and no record. A later Agent run as the same run user reads it. Also, no key says that a Type needs a state directory or that a provider supplies one, though PRV-029 promises that setup "fails with an explicit diagnostic".

**Counterargument.** Ruling 40 leaves what a provider does to its contract, which the user reads when picking it. The limits already say a process of the same operating-system user can read a state directory and that confinement is the runtime provider's.

**Least change.** PRV-029: "A runtime provider that supplies a state directory names it by the same variable and keeps the same properties, its removal with the instance among them; its contract states any it does not keep, and inspection shows which." The comment in names.yaml then reads "supplied by the instance's runtime provider".

**Precedent.** systemd's StateDirectory=, which gives every unit one variable, $STATE_DIRECTORY, and defines removal (systemctl clean); Kubernetes volume reclaim policy.

**Who decides.** owner. It adds an obligation on runtime providers, which is behavior (ruling 40 on provider contracts). The variable's name alone is a small fix the coordinator may make under ruling 71 point 4.

**Verifier.** confirmed, medium. Quote verbatim at spec/names.yaml:38. PRV-029 binds only the engine's own runner: spec/promises/providers.yaml:969 "The engine's own runner starts a process for an instance's run."; :979-981 "Another runtime provider supplies such a directory only where its contract states it does. A Type whose implementation needs one states that requirement; setup under a runner that gives none fails with an explicit diagnostic." The term is defined as the engine runner's: spec/definitions.md:279. LIF-035 states removal with no runner qualifier but "under PRV-029": spec/conventions/lifecycle.yaml:733-734. MAN-018 reserves the variable in every template: spec/promises/manifest.yaml:404-406. Both shipped runtime providers use the variable and never remove the directory: packages/provider-ssh-runtime/contracts.yaml:42-43 "The last points to the per-instance remote state directory, kept across starts and never removed (PRV-029)."; packages/provider-local-process/contracts.yaml:16-18 "reused across starts and never removed". No key exists for a Type to state the need: type-spec.v1 in spec/schemas/contracts.yaml has only family, schema, offers, requires, implementation, controls, signIn, nodeSelector, conditions, records, toolsDirectory; run.v1 has no such input. Not recorded in AUDIT.md known-open, cold-readers.md, IN_FORCE.md:187 (which lists PRV-029 as in spec and leaves only client homes to order 050), the review list, G5.md:162 or rulings 75 and 77.

**Verifier's corrections.** As stated, with the unsafe mark narrowed: that a process of the same operating-system user can read a live state directory is a stated limit (spec/README.md:424-427, written for the engine's machine); what is new is the login left on another machine after its instance is removed, with no record, which the provider's own contract discloses. So medium. Add to the finding that PRV-029's observable names a mechanism the language lacks (a Type stating the need, and the diagnostic), which is a defect on its own and can be fixed apart from the owner's question. Owner decides the obligation on runtime providers.


## FR-124

**The implementation-defined list holds obligations that no promise states**

L11/F4 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** DL17, DL13, DL18, ACC-036, OPS-008, IMP-007, IFC-049

**Where.** spec/implementation-defined.yaml:199

**Quote.** "Every decision that a promise has the engine make reads the state as it stands, as a linearizable read of etcd does. At most one part of the engine acts on any one invocation, and the store of the engine's own state enforces it"

**Situation.** The list is "what this specification deliberately leaves to each engine" (line 1), and the Engine level asks only for "a choice for every item" (spec/README.md:321). DL17 nonetheless states invariants that an engine of several parts must meet: linearizable reads, at most one part acting on an invocation, commits compared against the state read, every run on the old machine ended before one is asked for elsewhere. DL13 states more: the engine's credential is short-lived on an engine of several parts, and each root and key is kept while a run holds it. A search of promises/, conventions/ and interface/ for those sentences finds none; definitions.md:210 and IN_FORCE.md:209 cite DL17 as where the rule lives. They have no ID, level, observable or vector, are not among the 519 promises, and the safety catalogue and G5 count none. A builder cannot tell whether they bind or describe one mechanism. The restatements drift: DL18 says a URL action's wait "ends unknown" (line 250), while IFC-049 has enrollment end succeeded (spec/interface/implementation.yaml:1392). DL13 is one item of 60 lines holding well over a dozen choices, so "a choice for every item" cannot be checked item by item.

**Counterargument.** ACC-036 ("as they stand"), OPS-008 (one invocation for one key) and IMP-007 already imply these outcomes; DL17 says only how an engine of several parts meets them, and everything under spec/ binds.

**Least change.** Write each obligation as a convention promise with an ID, level and observable, such as "Where the engine runs as several parts, at most one part acts on any one invocation"; have the item cite it; leave in each DL item only the choices an engine documents, and split DL13 into them.

**Precedent.** POSIX's conformance document and ISO C Annex J.3: an implementation-defined item names a choice to document, and the requirements stay in the normative clauses.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect of form; moving text into promises adds no behavior. LD5 (docs/AUDIT.md:410) puts shared conventions in promises with vectors.

**Verifier.** confirmed, medium. Quote verbatim at spec/implementation-defined.yaml:199-202. The file calls itself choices: :1-3 "what this specification deliberately leaves to each engine ... A choice must keep every promise." The Engine level asks for promises plus "a conformance document that documents a choice for every item" (spec/README.md:321). The obligations exist nowhere as promises: grep of spec/ for 'short-lived', 'linearizab', 'At most one part', 'compare-and-set', 'on one machine at a time', 'survive the move', 'kept for as long as a run' finds only implementation-defined.yaml:158-159, 200-202, 214, 224, 232 and definitions.md:210, which cites "(DL17)". IN_FORCE.md:209 lists DL17 beside ACC-036 and ACC-037 as where the rule is. Promise count recounted: 519; DL items: 17; docs/implementation/SAFETY_CATALOGUE.md has no DL17 or DL13 row. Source of the content: ruling 43 Q2 a, docs/proposals/ruling-43/README.md:52-55 "meets stated outcomes and chooses its own mechanism". Round 5 added further obligations to DL17 without questioning the form (docs/proposals/reaudit/round-5/DECISIONS.md:113, 130, 131, 137, 141). Not in any known-open list.

**Verifier's corrections.** Class defect and coordinator's fix hold: ruling 43 ruled the outcomes, so moving them into promises adds no behavior. Two corrections. (1) The DL18 and IFC-049 example is not drift: IFC-049 names the exception itself, spec/interface/implementation.yaml:1390-1392 "This is the enrollment exception to DL18's unknown ending." What remains is that DL18 states an outcome ("the invocation ends unknown and no later hook runs", implementation-defined.yaml:250-251) inside a DL item and without the exception. (2) "A builder cannot tell whether they bind" overstates: everything under spec/ binds and DL17 calls them "these outcomes"; the fault is that they have no ID, level, observable or vector, fall outside the wording of the Engine level, and are counted by neither the safety catalogue nor G5.


## FR-125

**Which schema keywords make a reference schema govern a place**

L11/F5 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-077, FR-082.

**Promises.** OBJ-021, OBJ-009, OBJ-010, SEC-001

**Where.** spec/core/references.yaml:1

**Quote.** "A value inside `config` or `arguments` is resolved to an ID only where the schema that governs its place is one of these: directly, as the `items` of a list, or as a branch of `anyOf` or `oneOf`."

**Situation.** OBJ-021 lists the same three positions and adds "Any other value is data" (spec/promises/objects.yaml:369). The schema profile also has `allOf`, `if`/`then`/`else`, `dependentSchemas`, `prefixItems`, `patternProperties` and `additionalProperties`. Take a Type whose configuration is a map of mounts, `additionalProperties: {$ref: core#resource-reference.v1}`, or one that writes `allOf: [{$ref: core#agent-reference.v1}, ...]`. Reading A, the three listed positions only: the value is data, stored as written, never resolved or checked for existence, and an authored `{kind, id}` is accepted. Reading B, every applicator that JSON Schema applies at that place: the value is a reference, stored as `{kind, id}`, and the instance is rejected if it names nothing. The first build takes B: it follows allOf, if/then/else, dependentSchemas, properties, patternProperties, additionalProperties, prefixItems and items (engine/crates/core/src/registration.rs:844). An implementation receives a name under A and an ID under B; under A the value follows the name to a later object, which OBJ-010 says a reference never does.

**Counterargument.** "The schema of that place directly" can be read as any subschema that JSON Schema applies to the place, and round 5 (M26) already settled `$ref` chains by JSON Schema's own meaning.

**Least change.** OBJ-021: "A reference schema governs a place where JSON Schema applies it to that place and the value satisfies it, through `$ref` and any applicator of the profile." Or keep the three positions and reject a Type schema that uses a reference schema anywhere else, as SEC-001 does for `secret-reference.v1`.

**Precedent.** JSON Schema 2020-12 annotation collection: an annotation attaches to an instance location from every subschema that applies there and validates. SEC-001 for the rejecting variant.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; either fix adds no behavior that the text and the first build do not already have.

**Verifier.** confirmed, medium. Quote verbatim at spec/core/references.yaml:1-4. OBJ-021: spec/promises/objects.yaml:370-374 "governs a place where it is the schema of that place directly, where it is the `items` of a list that holds the value, or where it is a branch of `anyOf` or `oneOf` that the value satisfies ... Any other value is data." Same list in spec/definitions.md:118. The profile allows the other applicators: spec/schema-profile.md:69-71. The first build follows all of them, citing the promise: engine/crates/core/src/registration.rs:844 "// SCH-001 / OBJ-021: follow the applicators at this instance location.", then allOf (845-857), if/then/else (858-868), dependentSchemas (870-875), patternProperties (892), additionalProperties (906), prefixItems (924). The repository's second reader of the same sentence differs: tools/check_schemas.py:182-186 takes "the schema of a place" as what `properties`, `additionalProperties` and `$defs` give, plus `items`, and so excludes `patternProperties` and `prefixItems`. Only `$ref` chains were settled before: docs/proposals/reaudit/round-5/DECISIONS.md:83 (M26). Not in any known-open list.

**Verifier's corrections.** Real and new, but split it. (a) For `allOf`, `not`, `if`, `then`, `else`, `dependentSchemas` and `contains` the text decides: SEC-001 calls them "any other applicator" than the three positions (spec/promises/secrets.yaml:47-51, 64-66) and OBJ-021 says "Any other value is data". There the first build departs from the text; that is a first-build finding, not two readings, so the reviewer's `allOf` example is of another class. (b) The true ambiguity is the keywords that give a child place its schema: `properties` must count as "directly" or no configuration could hold a reference, and nothing says whether `additionalProperties`, `patternProperties` and `prefixItems` do; checker and engine already disagree on the last two. Who decides: either fix changes what some Type schemas mean (a value becomes a reference, or a schema is rejected), so it is not a no-behavior fix; the coordinator may make it only as a first-build finding under ruling 46 point 4 or a small finding under ruling 71 point 4, logged for the owner to strike.


## FR-126

**Which rule is the deciding rule when several refuse**

L11/F6 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-004, FR-158, FR-210.

**Promises.** ACC-056, IFC-050, OPS-023, DL9

**Where.** spec/implementation-defined.yaml:75

**Quote.** "The order in which several failed checks that give the same reason are reported."

**Situation.** OPS-023 orders the four reasons; within one reason DL9 leaves the order to the engine. That was harmless while several diagnostics could be listed. ACC-056 now has a refused result carry "one error diagnostic, whose ruleId is the refusing promise" (spec/conventions/access.yaml:915), and IFC-050 makes the ruleId observable. Where two rules refuse, the one ruleId, the `cause` and whether `_meta` carries `restriction` are each engine's pick. Example: an Agent with a bounded right to open asks open.v1 for a Conduit whose consumer is root, to a target outside its bound: ACC-054, or ACC-029 with `restriction`. Example: a call through a Conduit that its target's retirement closed: ACC-001, ACC-011 or CON-008, each a row of ACC-056's table. Two engines return different rules for one request, a vector cannot assert the rule, and an agent caller is told a different remedy.

**Counterargument.** ACC-056 says promises that restate a refusal "reuse the deciding access rule's cause; the ruleId remains the deciding rule", and several rows point back to ACC-001, so a builder can infer most cases.

**Least change.** State the order of the refusing checks and that the first to refuse decides: identity (IFC-032); a Conduit lists the operation (ACC-001); reserved to root (ACC-032, ACC-054, ACC-057, SEC-008); bound (ACC-029, ACC-030, ACC-053, ACC-055); filter (ACC-008). Mark each row of ACC-056's table as deciding or restating, and narrow DL9 to reasons other than refused.

**Precedent.** XACML's first-applicable combining; the Kubernetes authorizer chain and fixed admission order, where the first module to decide names the cause.

**Who decides.** coordinator. Ruling 46 point 4 with ruling 52: an ambiguity; the fix pins an observable and adds no feature.

**Verifier.** confirmed, medium. Quote verbatim at spec/implementation-defined.yaml:75. ACC-056: spec/conventions/access.yaml:915 "Its result carries one error diagnostic, whose ruleId is the refusing promise"; :935-936 "Promises that restate a refusal reuse the deciding access rule's cause; the ruleId remains the deciding rule." No text says which rule decides when two refuse; the table (access.yaml:939-999) gives own causes to rows that restate (ACC-011 at :946, CON-008 at :966) and to rows that point elsewhere (:975, :985, :986). IFC-050 makes the pick observable: spec/interface/implementation.yaml:1348-1349 "Wording is the engine's choice; the ruleId, reason and refusal.v1 fields are observable." OPS-023 (spec/conventions/operations.yaml:493-496) orders reasons only. ACC-054 says only "A delegate is refused whatever its bound" (access.yaml:860), not which rule is reported. The first build's choice is its own order: engine/CONFORMANCE.md (DL9 row) "There is no extra portable ordering rule among diagnostics of the same reason." ACC-056 is not yet built (no ACC-056 in engine/crates or conformance). Not recorded: docs/proposals/spec-batch-1/areas/refusals-and-identity.md:58, 79 speak of "the deciding promise" without an order; no hit in the known-open lists.

**Verifier's corrections.** As stated, with the second example removed: ACC-011 decides it itself, access.yaml:191-192 "A request through it is decided under ACC-001 from the Conduits that are open", so the deciding rule there is ACC-001 and CON-008 restates. The open cases are two deciding rules that both refuse, such as ACC-054 with ACC-029 or ACC-030, or ACC-032 or ACC-057 with ACC-001. Add that DL9 no longer describes the choice at all: it orders several reported checks, and ACC-056 allows one diagnostic. Coordinator decides; XACML first-applicable is the right precedent.


## FR-127

**What a refusal's `cause` holds, and that it may be empty**

L11/F7 · ambiguity · verifier: confirmed, low · decides: coordinator · new

**Promises.** ACC-056, IFC-050

**Where.** spec/core/results.yaml:617

**Quote.** "The two tiers of ACC-056. named echoes only the request, cause states the refusing rule."

**Situation.** refusal.v1 requires `cause` as `type: string` with no minimum length (results.yaml:640), and `operation` likewise. Three readings: the sentence of the rule's row in ACC-056's table, verbatim ("the cause in the refusing rule's row below"); the engine's own words (IFC-050: "Wording is the engine's choice; the ruleId, reason and refusal.v1 fields are observable"); or the rule's ID ("cause states the refusing rule"). Several rows are instructions to a builder and not text for a caller, such as "Use the deciding access rule; this promise gives the form of a reason". One engine returns `cause: "ACC-029"`, another the row, another a paraphrase, and `cause: ""` validates. No vector can compare the field, and refusal.v1 is not yet built.

**Counterargument.** The diagnostic's ruleId and message.text already carry the rule and the explanation, so `cause` is a convenience whose wording IFC-050 leaves to the engine on purpose.

**Least change.** refusal.v1: "`cause` is a non-empty sentence, in the engine's words, that says why this request was refused; ACC-056's table gives what it must convey", with `minLength: 1` on `cause` and `operation`. Or remove `cause` and keep ruleId and message.text.

**Precedent.** RFC 9457 problem details: `type` identifies the problem and `detail` is free text for the occurrence; Kubernetes Status has a fixed `reason` and a free `message`.

**Who decides.** coordinator. Rulings 46 point 4 and 52: an ambiguity in a result's form; no behavior is added.

**Verifier.** confirmed, low. Quote verbatim at spec/core/results.yaml:617-618; `cause: type: string` at :640-641 and `operation` at :627-628, no minLength; both required at :670-674. The two texts that pull apart: spec/conventions/access.yaml:918-919 "and the cause in the refusing rule's row below. It states the specific cause of that rule" against spec/interface/implementation.yaml:1348-1349 "Wording is the engine's choice; the ruleId, reason and refusal.v1 fields are observable." Rows that are instructions and not causes: access.yaml:975, 985, 986. Not in spec/core/README.md:692-712 (what the package does not settle) nor any known-open list; refusal.v1 is unbuilt.

**Verifier's corrections.** Smaller than its title: an empty `cause` is not reachable by an engine that keeps ACC-056, whose `then` requires the row's cause; the missing minLength is a schema nit. The real ambiguity is one question: is the row's sentence the field's fixed text (which "refusal.v1 fields are observable" suggests, and a vector could compare) or only what the engine's own words must convey. The third reading (the rule's ID) is weak, since ruleId already holds it. Coordinator decides; either answer adds no behavior.


## FR-128

**A diagnostic that does not reject has no fixed level or ruleId**

L11/F8 · gap · verifier: confirmed, low · decides: coordinator · new

**Promises.** OBJ-008, NAM-008, CTR-004, TPL-004, IFC-025, OBJ-023

**Where.** spec/core/results.yaml:71

**Quote.** "`ruleId` is the ID of the promise the diagnostic is about; `level` is SARIF's, and a diagnostic that is why a request reported any reason has the level `error`"

**Situation.** Only the error diagnostic is fixed. diagnostic.v1 requires `message` alone (results.yaml:104), and OBJ-008, NAM-008 and CTR-004 say only that a diagnostic "did not prevent" registration or "does not reject". A registration that reports invalid can carry both kinds, such as a schema failure and an unused package. Nothing says the second has the level `warning`, `note` or none, that it may not be `error`, or that it carries a ruleId. An agent reading the result cannot tell which diagnostics it must fix, and a vector for OBJ-008 or NAM-008 has no field to assert.

**Counterargument.** In a result that succeeded every diagnostic is one that does not reject, SARIF reads an absent level as `warning`, and the quoted sentence can be read as "and only such a diagnostic".

**Least change.** diagnostic.v1 and IFC-025: "A diagnostic that does not reject has the level `warning` and names its promise in `ruleId`; only a diagnostic that is why a request reported a reason has the level `error`." Add `ruleId` and `level` to `required`.

**Precedent.** SARIF 2.1.0, where `level` defaults to `warning` and `error` marks a failure; Kubernetes API warnings returned beside a successful response.

**Who decides.** coordinator. Ruling 46 point 4, or ruling 71 point 4 as a small finding; logged for the owner to strike.

**Verifier.** confirmed, low. Quote verbatim at spec/core/results.yaml:71-73; `required: [message]` at :104; `level` enum at :87. Only the error case is fixed: spec/interface/implementation.yaml:705-706 "one whose `level` is `error` and whose `ruleId` is the ID of the promise that decided the reason". The non-rejecting cases state no level: spec/promises/objects.yaml:117-118 "lists the unresolved name as a diagnostic that did not prevent it"; spec/promises/naming.yaml:133; spec/promises/contracts.yaml:117; spec/promises/templates.yaml:110; spec/README.md:329. spec/core/README.md:254-255 names the `diagnostics` output "for the diagnostics that did not prevent the request" with no level. The first build chose `note`: engine/crates/core/src/registration.rs:1414 `if unresolved { "note" } else { "error" }` and engine/crates/core/src/packages.rs:607; it tells rejection by `level == "error"` (registration.rs:1932). No record of the choice in engine/CONFORMANCE.md, the first-build logs or the known-open lists.

**Verifier's corrections.** As stated. One correction to the least change: it proposes `warning`, and the first build emits `note`, so the fix must choose between them knowingly (SARIF reads an absent level as `warning`) and change the build or the text to match. Adding `ruleId` and `level` to `required` also binds diagnostics that an implementation returns under IFC-026 and any SARIF member an engine adds; check that before making them required. Coordinator decides under ruling 71 point 4.


## FR-129

**record.v1 names no member for what the promises say an enrollment or answer record holds**

L11/F9 · gap · verifier: confirmed, low · decides: coordinator · known-new-situation

**Promises.** REC-001, REC-002, ENR-007, PND-008, PND-012

**Where.** spec/core/results.yaml:169

**Quote.** "An engine names the event's `source` and `type`, and may add members."

**Situation.** Known: the record's `source` and `type` are each engine's (spec/core/README.md:705), and PLC-024's cause waits for a portable field (A39; IN_FORCE.md:146). New: ENR-007 has the answer's record hold "enrollment, machine id, evidence shown, the weaker-check flag and whether the fingerprint was compared or the weaker check accepted"; REC-001 has an enrollment record name "its provider, enrollment ID and transition"; REC-002 has a record say that no instance could be established. record.v1 has a member for none of these, and its `answer` is pending-action.v1, which forbids further properties (results.yaml:604, 611), so the flag cannot sit there. Each engine invents names. The record that a machine was admitted on the weaker check, which is what an owner would audit, cannot be found by one tool across engines, and no vector can assert it.

**Counterargument.** The core README says shapes are "settled with the conformance vectors", and enrollment has no vectors and no engine binding yet (staged for 049a and 049b).

**Least change.** Add to record.v1's `data` the members `enrollment`, `provider`, `machine`, `transition`, `weakerCheck`, `fingerprintCompared` and `cause`, each present where it applies, with the enrollment bindings of 049a.

**Precedent.** The Kubernetes audit Event (audit.k8s.io/v1), with fixed fields and stage names; CloudEvents `dataschema` for a typed `data`.

**Who decides.** coordinator. Ruling 46 point 4: naming members that the promises already require adds no behavior.

**Verifier.** confirmed, low. Quote verbatim at spec/core/results.yaml:169-170. What the promises require: spec/promises/enrollment.yaml:174-176 "The answer's record holds enrollment, machine id, evidence shown, the weaker-check flag and whether the fingerprint was compared or the weaker check accepted."; spec/conventions/records.yaml:34-35 "An enrollment record names its provider, enrollment ID and transition"; records.yaml:74-75. record.v1's `data` (results.yaml:185-204) has no member for any of them, and `answer` is pending-action.v1, closed at results.yaml:604 and :611, whose subject cannot be the machine: spec/promises/pending.yaml:342 "A pending machine is not an object and is no subject." Already recorded for the family: spec/core/README.md:706-707 "the `source` and `type` of a record's CloudEvent: an engine names them"; IN_FORCE.md:146 "Spec pending, with the vectors: a portable field for the cause of a machine's loss"; docs/AUDIT.md:226 (A39). The enrollment members are in none of them. The first build's own names exist only in an unmerged order: docs/work-orders/049a1-build-enrollment-core.md:246-255 (a record type `enrollment-change`).

**Verifier's corrections.** Correctly marked known-new-situation. Smaller than it reads: `data` is open (no additionalProperties: false), so an engine can hold everything the promises require; what is missing is portable names, the same kind of gap as A39's. Drop `cause` from the least change, since it is the known A39 item. The fix is best made with 049a's bindings, which already picked names for the first build. Coordinator decides.


## FR-130

**No promise says what the core package's version means once the engine's built-in core differs from the one stored objects hold**

L11/F10 · gap · verifier: confirmed, medium · decides: owner · new

Same fault, or overlapping: FR-073, FR-081.

**Promises.** NAM-009, PKG-005, NAM-020, NAM-021, IFC-033, FLEET-005

**Where.** spec/core/conduit.yaml:12

**Quote.** "A template or an instance that uses the core package holds this address and this version, and no digest."

**Situation.** The engine's program is replaced, over existing state, by one whose core package is 0.2.0. Every stored template and instance holds the core at 0.1.0 (NAM-009, PKG-005). IFC-033 derives a tool "from the Operation contract as the target holds it" and presents none for an operation that "the target's version of the package does not hold". Only one core is built in, and it is never installed or listed (NAM-021). Undecided: whether the engine must keep the earlier core's documents, read stored objects against the new ones, or refuse to start; whether root's `system` Conduit, made from `root-pool` at 0.1.0, gains an operation that the later `root-pool` lists (FLEET-005 covers only a capability's additions); and what `packages` and the `@version` in a tool bundle's keys then show. If the version is meant to carry no meaning, the field is not needed.

**Counterargument.** The specification is a draft whose core is unreleased. Contract names carry `.v1` and may only gain optional things (NAM-024), and an instance keeps what it resolved (INS-013), so most stored objects stay valid.

**Least change.** Either a limit in spec/README.md: "An engine starts only on state made with its own version of the core package; this version defines no upgrade." Or a promise: "A stored object keeps the definitions it resolved; an engine whose core package has a later version reads it as that version held it."

**Precedent.** Kubernetes storage versions and conversion of stored objects; Terraform's state schema version with its upgraders.

**Who decides.** owner. Behavior and scope: what an engine does with state from an earlier core. The owner rules behavior (AGENTS.md, Rules).

**Verifier.** confirmed, medium. Quote verbatim at spec/core/conduit.yaml:12-13; `version: 0.1.0` at :21. Stored objects keep it: spec/promises/naming.yaml:145-146 "For the core package ... it keeps the address and the version alone"; spec/promises/packages.yaml:73-74. The core cannot be installed or replaced: naming.yaml:344-345. Tools come from what the target holds: spec/interface/implementation.yaml:897 and :916-918. FLEET-005 covers only capability additions: spec/promises/fleets.yaml:96-97 "Later capability additions never widen these lists." grep of spec/ for upgrade, migrate or an earlier core finds nothing; nothing in spec/README.md limits (331-617), AUDIT.md known-open, cold-readers.md, IN_FORCE.md or the review list. Added evidence the reviewer did not give: `git log -- spec/core/conduit.yaml` shows the version unchanged since #6 (e271d1c7) while orders 036b, 038a, 037, 040, 038b and 038c changed the exports, so the version already says nothing of the content an engine build holds.

**Verifier's corrections.** As stated. The situation is present, not future: the dogfood engine runs newer builds over kept state (docs/guides/full-review.md:23-24, "still pinned before 070"), and the core's content has changed six times under one version. The first build's store format version (engine/CONFORMANCE.md, DL2 storage row) is an engine's own answer and decides nothing for the language. Owner decides; the limit in spec/README.md is the smaller of the two options and removes no feature.


## FR-131

**Core package descriptions and head comments still state the rules as they stood before Fleets, sign-in and answers**

L11/F11 · contradiction · verifier: weakened, low · decides: coordinator · new

**Promises.** ACC-033, TPL-004, CTR-004, OPS-019

**Where.** spec/core/capabilities.yaml:66

**Quote.** "inspecting what is not a Resource, or is a Resource whose Pool is retired; setting up and retiring what is not a Resource"

**Situation.** The engine builder's reading path sends a reader to core/ at step 6. There: (1) object-management.v1's description says `management` inspects, sets up and retires "what is not a Resource", which includes an Agent, against ACC-033 rows 4A and 6A (the owning Fleet) and against its own second paragraph. (2) capabilities.yaml:31 says "the three capabilities that hold management verbs", where four list `reconcile.v1`. (3) types.yaml:17 says each Type lists under `requires` the operations of the capabilities it offers, while pool.v1 and fleet.v1 leave out `sign-in.v1` and `answer.v1` and management.v1 leaves out `answer.v1`. (4) definitions.md:399 describes the core package without the Fleet capability, `fleet.v1` or the nine templates beyond first start. (5) templates.yaml:1 heads 17 templates as "The templates that first start instantiates". None changes behavior where the promise is read; each misstates it where the document is read first.

**Counterargument.** The core README says the promises are the source of truth and a document that differs is wrong, and TPL-004 makes `requires` irrelevant for an operation the engine implements.

**Least change.** Correct the five passages, and cut from descriptions and head comments what a promise already states, keeping the citation.

**Precedent.** The Kubernetes API reference: one normative description per field, generated into other documents and not restated by hand.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior.

**Verifier.** weakened, low. All five passages checked and true. (1) Quote verbatim at spec/core/capabilities.yaml:66-68, against the same description's second paragraph at :73-74 "live Agents use their Fleet for inspection, setup, retirement and reconciliation" and the head at :34. (2) capabilities.yaml:31-32 "the three capabilities that hold management verbs include `reconcile.v1`"; four list it (:84, :107, :131, :228). (3) spec/core/types.yaml:17 "Each lists under `requires` the operations of the capabilities it offers"; pool.v1 (:54-62) lacks sign-in.v1 and answer.v1 that capabilities.yaml:133-134 list; fleet.v1 (:140-147) lacks those of capabilities.yaml:229-230; management.v1 (:93-105) lacks answer.v1 of capabilities.yaml:90. (4) spec/definitions.md:399 names "the capabilities that `management`, `access` and a Pool offer" and "the templates of the first-start objects and of the three Conduits of `root`", with no Fleet capability, no fleet.v1 and none of the nine later templates. (5) spec/core/templates.yaml:1 heads a file of 17 templates (names at :40-279). None changes behavior: spec/promises/templates.yaml:80-85 gives the engine every operation that an offered capability includes. Not in any known-open list.

**Verifier's corrections.** Class is defect, not contradiction: none of the five is a promise, and spec/core/README.md:4-5 already rules "where a document here and a promise differ, the document is wrong", so no two promises fail to hold together. Severity low. One more instance, inside a promise: TPL-004's observable, spec/promises/templates.yaml:92-93, "The engine implements every operation of `object-management.v1`, `access-control.v1` and `pool-management.v1` with no selection", omits fleet-management.v1. Coordinator decides; cutting restatements to citations is the right fix.


## Further points the verifier noticed

Not verified by a second reader.

- **TPL-004's observable still lists three engine-implemented capabilities and omits fleet-management.v1** (spec/promises/templates.yaml:92). "The engine implements every operation of `object-management.v1`, `access-control.v1` and `pool-management.v1` with no selection". The core's `engine` also implements retire-with-agents.v1, sign-in.v1 and answer.v1 (spec/core/conduit.yaml:182-201), and fleet.v1 offers fleet-management.v1 (spec/core/types.yaml:138-139). Same family as F11 but inside a promise's observable. Defect, not unsafe, low; the `then` (templates.yaml:84-85) already decides by what `engine` lists. Coordinator.
- **The repository's checker and its engine read 'the schema of a place' differently for patternProperties and prefixItems** (tools/check_schemas.py:186). `SECRET_PLACES = ("properties", "additionalProperties", "items", "$defs")`: the checker rejects `secret-reference.v1` under `patternProperties` or `prefixItems` as "any other applicator" (SEC-001, spec/promises/secrets.yaml:50-51), while the engine resolves references under both (engine/crates/core/src/registration.rs:892, 924). SEC-001's own list of rejected keywords (secrets.yaml:64-66) names neither. Both outcomes are safe for a secret (governed or rejected), so not unsafe; it is the evidence that F5's ambiguity has already produced two readings, and whichever sentence fixes OBJ-021 must fix SEC-001 and the checker with it.
- **LIF-035 states removal of the state directory with no runner qualifier** (spec/conventions/lifecycle.yaml:733). "Removing or unregistering an instance removes its state directory and contents under PRV-029." PRV-029 binds only the engine's own runner (spec/promises/providers.yaml:969), and both shipped runtime providers say "never removed (PRV-029)". A reader of LIF-035 alone expects removal under every provider. Part of F3; fix the sentence with it.

## The verifier's notes

Checked at commit 2eb8f258 (git rev-parse in the review tree), read-only; nothing built or run. All eleven quotes are verbatim at the cited lines (several span two or three lines). Counts recounted: 519 promises, 17 implementation-defined items, 17 core templates.

Result: ten confirmed, one weakened (F11, class only), none refuted, none already known. F9 is rightly marked known-new-situation.

What changed against the reviewer's account:
- F1 is the strongest finding. ACC-030's Pool sentence in the observable (access.yaml:534-535) is as loose as its `then`; only the last sentence forbids the widening. definitions.md:186 needs the same correction.
- F2 and F3 keep their unsafe marks only in a narrower sense, so I rated them medium: F2 needs an engine to document an unauthenticated hand-over path of its own; F3's new harm is the login left after removal, which each provider's contract discloses.
- F4's DL18 example is wrong: IFC-049 states the enrollment exception itself.
- F5's `allOf` example is decided by the text (SEC-001 with OBJ-021's "Any other value is data"); there the first build departs from the specification. The live ambiguity is `additionalProperties`, `patternProperties` and `prefixItems`, where the checker and the engine already differ. Its fix changes what some schemas mean, so it is not a no-behavior fix.
- F6's second example is decided by ACC-011.
- F8's proposed level (`warning`) differs from what the first build emits (`note`).
- F10 is present today: the core's version has stayed 0.1.0 through six orders that changed its exports.

Not checked: the conformance vectors' contents beyond a search for a Pool in a bound's `targets` (none found); the reviewer's two unreported smaller points; ruling records other than 43, 59, 61, 62, 75 and 77.
