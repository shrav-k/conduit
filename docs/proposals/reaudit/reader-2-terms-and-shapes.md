# Re-audit, reader 2: terms and document shapes

Cold reader's report of 2026-11-05, kept verbatim apart from absolute paths, which are
shortened to paths within the repository. The reader saw only `spec/`. Findings are
not rulings; what was done about each is in the [re-audit record](README.md).

```text
Scope read: every file under spec/ in full (25 files, 4,444 lines, 312 promises, 12 implementation-defined entries). Nothing outside spec/ was read and nothing was edited. The README's per-file promise counts match the files. I did not check the Model Context Protocol text itself, so the MCP findings are limited to what spec/ says.

TERMS

(a) Used but not defined (README rule 4: "Every term is defined in definitions.md")

T1. Interface, Agent, Resource, Pool have no definition. `family` only lists them; only Conduit has a row. Fix: one row each.

T2. "diagnostic" is used in well over 100 observables and never defined. It has two classes: one that rejects, and "a diagnostic that did not prevent it" (OBJ-008, NAM-008, PKG-006). Fix: define it and name the two classes.

T3. "resolve / resolving / resolved" is undefined and carries four meanings:
- the act on documents: "Resolving returns a diagnostic ... and produces no objects" (OBJ-001)
- a reference reaching an object: "resolved to an ID" (OBJ-009)
- a package dependency: "Each dependency is resolved to an installed package" (PKG-005)
- an adjective: "resolved instance", "resolved template" (TPL-004), "resolved graph" (GRA-001)
It is also unclear how "Resolving returns" differs from "Registration returns" (used for templates in GRA, INS-010 to 012, CON-020, but OBJ-001, OBJ-019 and SCH-012 say "Resolving" for any document) and from "Installation returns". Needs a ruling: either resolving is the pure function of the Language level ("declarations in, resolved objects or diagnostics out", README) and registration is the operation that runs it, or they are synonyms.

T4. "request", "dispatch", "registered objects" and "condition" are used inside other definitions but have no row. SCH-002, SCH-003 and SCH-008 say "accepted into the registry", a word used nowhere else. Fix: add rows; replace "registry".

T5. "declaration" is undefined with two meanings: any submitted document (README "a declaration it cannot honor"; CON-006 "The declaration is rejected") and a nested instantiation (LIF-008 "a declaration inside another object, such as a declared Conduit or a Pool's entry"; CON-004, CON-010, LIF-009). Fix: use "document" for the first and "nested instantiation" for the second.

T6. "hold / held" is undefined with four senses:
- an instance holds a handle (TPL-007 "lists the runtime as held")
- an actor "holds a right to open" (ACC-005) or "whatever Conduits it holds" (OPS-015), meaning it is the consumer of an open Conduit
- an object "holds an ID" (OBJ-009)
- a request "is held" (DL4)
Fix: define the first; write the second as "is the consumer of".

T7. Role words not defined: "requester" (POOL-007), "holder" (ACC-029), "submitter" (INS-018), "observer" (OBS-006), "author" (SCH-002), "reader" (REC-005). Fix: use actor or caller, or define them.

T8. Other undefined terms a promise relies on:
- "shared-host requirement" (CON-006, PLC-013, CON-018)
- "location type" (PLC-008)
- "built-in schemas" (NAM-020)
- "re-export" (PKG-013, PKG-014)
- "pinned" (NAM-011), "publish / published" (PKG-004, PKG-017)
- "access decision" (ACC-002, ACC-020, REC-001)
- "attempt" (OPS-012), "in progress" (OPS-009), "withheld" (OPS-017)
- "default location" (PLC-003, PLC-016)
- "written step" (GRA-022)
- "isolation framework" (definition of enforced)
- "detached" (REC-003), "service login" (FST-003), "message or a notice" (IMP-002)
- the step results "not run" (GRA-002), "not opened" (ACC-005), "not set up" (POOL-013), "skips" (LIF-008)

(b) One term, two meanings

T9. "registered" is both a lifecycle state and "present among the registered objects". LIF-007 uses both in one sentence: "It remains registered, in the state retired". LIF-020 "a Resource that is still registered" and PLC-011 "The location and its instances remain registered" cannot be read without guessing. Fix: keep "registered" for the state and say "exists" or "not unregistered" for the other, or rename the state.

T10. "record" is defined as an append-only log entry ("The word is not used for an object's registration"; REC-003 says it must not be "removed or altered by anything other than a purge"). Other uses:
- one mutable record per operation identity: OPS-008 "keeps one record for that identity", OPS-009 "the state of the existing record: in progress, or its outcome", OPS-014 "The outcome becomes succeeded". REC-001 meanwhile yields at least two records per invocation (dispatched, ended).
- stored facts: NAM-009 "It records the address, version and digest", NAM-010, NAM-011, ACC-031 "the policy is recorded there", LIF-010 "as last recorded".
Needs a ruling: either OPS-008 to OPS-014 describe a derived view over log entries, or there is a second noun for the per-identity invocation state.

T11. "target" is defined as "used only for this end of a Conduit". Violations: OBJ-010 "the reference reports its target as missing"; OPS-002, OPS-003 and OPS-018 "the target of the effect". Fix: "the object it referred to"; "what the effect changes".

T12. "kind" is defined as the same idea as family (five values), but CTR-001 adds four more: `Type`, `Capability`, `Operation`, `Requirement`. INS-001 ("a template if it has no `from` ... Both have the kind of their family; there is no other marker") therefore classifies every contract as a template. Fix: define kind as nine values, and restrict INS-001 to the five family kinds.

T13. "reference" is defined as "a declared pointer to a registered object: a kind and a name". But `from` may be a qualified reference to a package template (FST-005; NAM-002 example), and installing "registers nothing" (NAM-013). NAM-013 also says "fully qualified reference". Fix: define local reference and qualified reference as the two forms; drop "fully".

T14. "source" has three meanings: where a record or observation was made (definition, REC-005); the `source` key of a value reference (GRA-016); where a package is fetched from (NAM-012 "a source that differs from the address"). Fix: rename one, for example "fetch location" in NAM-012.

T15. "entry" is defined only as a Pool's entry but is also used for a requirements-map item (CON-018 "Each entry has a name"), a manifest entry (NAM-006, PKG-020), an `implementation`-map entry (TPL-004), a `tools` entry (PKG-020), a log entry (LIF-013) and an implementation-defined entry (README). Fix: use "item" or "line" elsewhere.

T16. "depends on" is defined as "the relation that orders steps". It is also used for packages (PKG-003 "A dependency names a version", PKG-005), placement (PLC-015) and entries (POOL-006). Fix: widen the definition or use "uses" for packages.

T17. "export" is defined as "a contract, a template or an implementation". PKG-018 says `exports` "are the names of the documents in the package", and implementations are not documents. NAM-020 and the core-package row add "built-in schemas" as exports, a fourth thing. REC-003 uses "an export of it has failed" for records. Needs a ruling on whether a schema is an exportable thing and of what kind.

T18. "implementation" is defined as what does the work behind an operation, but the README uses it for an engine: "An implementation claims conformance", "Neither binds other implementations". Fix: say "engine" in the README.

T19. "host" is the machine a location reports, and also a conformance level in the README ("Host | The language level, plus..."). Fix: rename the level.

T20. "Conduit" is both the family and the product or system: "handed to Conduit" (submission), "pass through Conduit" (mediated), "knows nothing of Conduit" (PKG-020), "through Conduit" (IFC-015). "the system" (OBJ-004, FST-001) and "engine" compete for the same thing. Fix: say "the engine" for the running thing.

T21. "refused" is defined as "No open Conduit lists the operation, or a restriction forbids it". It is used for other causes: reserved to root (ACC-032, REC-004), first-start objects (LIF-022, ACC-024), not the original caller "whatever Conduits it holds" (OPS-015), a bound exceeded (ACC-030). NAM-017 says "The Conduit is refused". "unavailable" is likewise used as a status value, not a request ending (CON-011, PKG-008). Fix: widen the meaning of refused to "not permitted"; in NAM-017 say "not opened".

T22. "owner" — the definition says "The word is not used for a person", but README line 3 ("as the owner rules") and every file header ("Owner rulings") use it for a person. Fix: change the headers or drop the sentence.

T23. "definition" has three senses: the fixed part of an instance; "a published definition" (contract row); "a reusable definition" (template row). The first lists "its type, graph, runtimes and controls", omitting declared Conduits, entries and the `implementation` map, though INS-012 treats "selections" as structural.

T24. "covers" is used for a bound (ACC-005, ACC-006, ACC-029), but CON-013 says "its next request through it is covered", meaning permitted. Fix: "is permitted".

(c) Two terms for one thing

T25. Making an instance: "created" (about 40 uses, for example OBJ-009), "instantiated" (POOL-002, CON-002), "registered" (SUB-006 "Its templates and instances are registered"), "made" (POOL-003, CON-020). The entry definition even says "created, registered". OPS-019 lists "register, instantiate" as separate verbs, while the `management` definition lists only "register, replace, rename and unregister". Needs a ruling: is instantiation a distinct operation, or is it registering an instance document? The operation asked for in POOL-003, POOL-010, LIF-009 and CON-010 ("a new instance for a declaration") has no name.

T26. Negative results: "document/template/submission/package/manifest/declaration is rejected", "instance is not created", "not dispatched", "refused", "unresolved". Nothing maps a language-level rejection to a request ending, although registration is an operation (OPS-019) that "ends in one of the ways the definitions list". Needs a ruling: rejected means invalid (or conflict for OBJ-002, OBJ-011, SUB-007). SCH-002 reads "rejected when it is accepted".

T27. retire/close and set up/open. The definitions make them synonyms for a Conduit, but ACC-033 authorizes "inspect, set up, retire or unregister ... a Conduit" against `management` and, in the next sentence, "Opening and closing a Conduit" against `access`. LIF-022 and LIF-023 say "retire" of a Conduit. Fix: in ACC-033 list only inspect and unregister for a Conduit.

T28. Smaller pairs:
- "existing MCP service" (implementation row, SCH-014) vs "existing server" (plain row, PKG-020, IFC-010)
- "composite Conduit" (OBJ-014) vs "composition" (CON-003, CON-019; CTR-005 uses it for requirements)
- "pointer" (GRA-020 key) vs "path" (OBJ-016 "a value reference's path")
- "allowed" (ACC-020) vs "permitted" vs "authorized" (ACC-025, ACC-033, CON-008)
- "location fact" (definition) vs "facts" (PLC-007, PLC-008)
- "missing" (OBJ-010, LIF-019) vs "unresolved" (NAM-003) vs "unavailable" (PKG-008)
- "invoked" for a control that "is not dispatched" (TPL-011), although an invocation is by definition dispatched
- "configuration", "settings" and `config` are never stated to be the same thing
- "Result" is the column head of the request-ending table, the implementation's returned result, and "the result of registration"

(d) Definitions unused or contradicted

T29. Unused by any promise: "grant" (appears only in the access.yaml header comment); the terms "location fact" and "entry reference" (the form `{entry: name}` is used in POOL-005, the term is not).

T30. "optional" says "Inputs, steps, declared Conduits and outputs". OBJ-015 adds "a Pool's entry, a runtime". Fix: align the definition.

T31. "standard argument" and "nested instantiation" ("the shape of an instance document without the kind and `metadata`") contradict POOL-012 (an entry is "`from`, `arguments`, and optionally `optional: true`", no owner), POOL-009 (no location) and CON-020 (a declared Conduit has no consumer; a component has neither end). OBJ-014's "The three are written alike" is false as written. See S3.

T32. "type ... its schema says what a valid instance holds" and INS-014 ("definition and configuration ... do not satisfy the schema of the instance's type") contradict CTR-004: "`schema`, the schema of an instance's `config`".

T33. "implementation: what does the work behind an operation or a runtime", PKG-009 and PKG-010 ("operation or runtime") contradict TPL-003: each key "is the qualified reference of an operation", and a runtime carrying `implementation` is rejected. PKG-009 also omits the third default of TPL-004 ("or else the default the operation's own contract states"). PKG-010 cannot occur in a map once SCH-012 forbids duplicate keys.

T34. "`management` ... register, replace, rename and unregister objects" omits what ACC-033 routes to it (inspect, set up, retire) and overstates unregister, which for a Resource goes to its Pool.

T35. "document: One object written out in YAML" excludes contracts (CTR-001 "It is a document") and the manifest. "project: The directory of documents submitted together" conflicts with "submission: one file, several files or a directory". SUB-005 rejects a submission containing anything that "is not a template, an instance or the project's one manifest", so a project that holds contracts (that is, any package before publication) cannot be submitted. Needs a ruling: contracts are allowed in a project and installed rather than registered, or project and submission are different things.

DOCUMENT SHAPE

(a) Stated two ways

S1. Type `placement`: CTR-004 "writes an empty map"; PLC-014 "an empty list of requirements on placement". Pick one; CON-018 uses a map for the same thing on a Conduit.

S2. Control input: CTR-004 "`controls`, each with `inputs` and `changes`" (a map of named inputs). TPL-010 "a `schema` that further narrows that control's input", TPL-011 "the schema the type gives the control", REV-006 "its input omits a field" (one object). Needs a ruling: `inputs` map or one `schema`.

S3. Nested instantiation: OBJ-014 "`from`, standard arguments and `arguments`"; POOL-012 "`from`, `arguments`, and optionally `optional: true`"; POOL-009 "An entry's placement is the destination of one of the Pool template's inputs", so a `placement` member exists but may not be written. `optional` is missing from OBJ-014. Fix: state each of the three shapes separately.

S4. `metadata.name`: OBJ-018 "It holds `name`" (required) against OBJ-003 "An instance is created without a name" and INS-015. Fix: required for templates and contracts, optional for instances.

S5. Name grammar: OBJ-001 rejects any document name "not made only of lowercase letters, digits and hyphens". CTR-001 says a contract's "name is its export name", with dots (`local-directory.v1`, NAM-004). Fix: scope OBJ-001 to the five families.

S6. `arguments`: plain values in an instance (OBJ-013, INS-006), value bindings in a step (GRA-015, "A bare value ... rejected" GRA-013). OBJ-019 exempts everything "inside ... `arguments`" from key checking, which would let a malformed binding through. Fix: OBJ-019 exempts instance, nested and requirement `arguments` only.

S7. Sources allowed in an operation's graph: GRA-010 "may refer only to the operation's declared inputs, to its own steps' outputs and to handles it was given". GRA-020 forbids only `config` there, leaving `runtime` and `instance`. Needs a ruling.

S8. `implementation` has three shapes under one key: template, a map of operation to reference or graph (TPL-003); Type, "its default implementations" (CTR-004, shape unstated); Operation, one reference (CTR-002). State each.

S9. OBJ-019 rejects every key "this specification does not define" outside `config`, `arguments`, `annotations`, a literal and a schema, and requires camelCase. Author-chosen map keys are neither: input, step, entry, declared-Conduit, runtime, output, requirement and component names; `object` binding members; and the qualified-reference keys of `implementation`, `tools` and `controls`. Fix: exempt named maps. No promise gives the grammar of those names.

S10. Declared Conduit name: IFC-012 "the name is the one its consumer's template gives it" and IFC-014 (a clash is per consumer). OBJ-002 and the name definition make a Conduit's name unique in the family, so two instances of one template would collide; OBJ-003 would generate one. Needs a ruling: instance name against per-consumer local name. IFC-014 is a document-validity rule in the `interface` layer.

(b) Used but never defined as a key

S11. The keys `kind`, `consumer`, `target` and the instance key `placement` never appear in code format in any promise. `owner` appears once (POOL-012). The system-owned fields of OBJ-004 ("an ID, a revision or status") have no key names, so a schema cannot reject them. The parts of an inspection result (LIF-013) have no key names.

S12. Template input: no promise names the keys for its schema, default or destinations. INS-004 says "declares without a default"; SCH-008 forbids a `default` at the top of a schema, so the default cannot live in the input's schema. Only `optional` is named.

S13. Manifest: the "digest" and "source" of a package item (NAM-006, NAM-012) have no key names. `conduit.dev/core#location` (NAM-020) is the only core export ever named. The default-location setting and its control (PLC-003, PLC-016) are unnamed.

(c) Could not be determined

S14. How a reference with a kind is written (OBJ-006 "It states both the kind and the name"), and which kinds `consumer` and `target` accept. Also how a reference inside `config` or `arguments` is recognized as one (OBJ-009 resolves "every reference it holds"; POOL-005 `{entry: name}`), since the schema profile has no keyword or format for it.

S15. Where a graph's steps are written and whether they are a map by name or a list. GRA-021 gives a graph an `outputs` map; no key for steps exists. It follows that the `implementation` map cannot tell a graph "written in place" from anything else except by not being a string.

S16. How a step addresses another instance. A step has only `operation` or `runtime` (GRA-017), yet ACC-018 treats it as an onward request and GRA-022 lets it depend on a declared Conduit. Options: a key naming the Conduit, or steps only ever call the instance's own implementations.

S17. How a declared Conduit names its target (CON-020 forbids only `consumer`): a literal reference, an input destination `/conduits/<name>/target`, or `{entry: name}`. Whether `conduits`, `entries` and template `inputs` are maps by name (only `components` and `requirements` are said to be).

S18. Which places an input's destination may reach. INS-012 forbids "its setup graph, its runtimes, its controls or its selections", but declared Conduits and entries "are steps of" the setup graph (GRA-022) and `capability` and `operations` are selections by definition. Undetermined: `/entries/<n>/placement` (which POOL-009 requires), `/entries/<n>/arguments/<x>`, `/conduits/<n>/target`, a Conduit template's `/operations` and `/requirements/<n>/arguments/<x>`. Needs a ruling.

S19. How a policy is written when applied, and how it relates to a capability (ACC-015 "the capability the policy governs"). How a bound is written: "Its filters are a bound" names no requirement contract, and ACC-022 says "has no bound".

S20. Values of `supports` and the implementation's `placement` (PKG-019); of `command`, `url` and `graph`; of `family`; of `effect` ("a statement"); of `conditions`; of `offers` and `requires`. How a Type's `changes` pointers relate to a control's input fields.

S21. How a Conduit lists a control. OPS-020 says "A control needs an open Conduit that lists it"; CON-017 says `operations` are qualified references to operations "that capability includes"; controls are bare names in a Type. Likewise how an Operation is marked "reserved to the root Interface" (CTR-003, ACC-032). Things promises defer to a type contract have no key in CTR-004: what shutdown releases (LIF-016), what record content is kept (REC-002), whether a runtime may be restored (OBS-005).

S22. Manifest: file name; whether it carries `apiVersion` and a kind; whether `packages` is a list or a map by address; digest encoding; how a re-export is written (PKG-013); whether implementations appear in `exports` as well as `implementations`.

S23. Composed Conduit: what the instance document's `metadata.name` names when "no instance represents the composition itself" (CON-003; INS-016 says the result is "the instance and its nested instantiations"). How component Conduits are named. Whether a component may be `optional`. What a declared Conduit `from` a composition yields, given CON-002 "One Conduit ... for each declared Conduit".

S24. The value of `conduit.dev/interfaceVersion` (IFC-006) and its relation to `apiVersion: conduit.dev/v0` and to the README's "named version of this specification".

S25. The core package's exports: the template, Type, Capability, Operation and Requirement names for first start, the management verbs, shared-host, and the built-in schemas. FST-005 and OPS-019 require them to exist as qualified references.

(d) Spelling, and two keys for one purpose

S26. No defined key breaks camelCase. Duplications:
- value reference uses `name` for an input or runtime but `step` for a step, and `name: name` for the instance source (GRA-020)
- `components` (Conduit) vs `constituents` (Requirement) for the parts of a composition
- `placement` is a location reference on an instance and a set of requirements in a Type and in an implementation item
- `requires` (operations, CTR-004) sits beside `requirements` and the schema keyword `required`
- `appliesTo` values are lowercase (`conduit`) while kinds are capitalized

STANDARDS

N1. schema-profile.md is "what is relied on" (README), but its table omits three standards that promises bind to: the Model Context Protocol revision 2026-07-28 (IFC-002), Semantic Versioning 2.0.0 (PKG-001) and SHA-256 (PKG-004). Fix: add the rows.

N2. MCP: no promise cites a part by name. Relied on only descriptively:
- "the flag that marks a result as an error", "structured content" (IFC-004)
- "the protocol's own negotiation" for "each request" (IFC-003), which IFC-004 does not list among what a non-baseline revision must provide
- "a protocol error that says the tool is unknown or the request malformed" (IFC-010), with no error codes
- "the protocol's own facility for tasks" (IFC-016)
- tool-name limits for names of the form Conduit name, dot, export name (IFC-012)
Also `command` and `url` (PKG-019) imply a transport that IFC-017 leaves to the engine. Fix: name the protocol's fields and sections.

N3. Semantic Versioning: PKG-003 "a different version" does not say whether build metadata counts (1.4.0+a against 1.4.0). Needs a ruling: exact string, or SemVer precedence.

N4. Digest: algorithm standard not cited; text encoding and any prefix unstated; archive format unstated ("archive file exactly as distributed").

N5. JSON Schema 2020-12:
- `$ref` is permitted but `$id`, `$anchor` and `$schema` are not. Whether `$ref` may be anything other than a same-document pointer, in particular a qualified reference to a "built-in schema" (NAM-020), is not stated.
- The regular-expression dialect of `pattern` is not stated.
- SCH-001 defers to "a conforming JSON Schema 2020-12 validator, with format checking on", while the profile cites RFC 9562 for `uuid`, which 2020-12 does not. Say that SCH-004 governs.
- SCH-008 forbids `default` "inside ... `items`" while the profile says it follows Kubernetes. Whether a property of an object inside `items` may have a default is unclear.

N6. YAML 1.2.2 core schema: non-finite and non-decimal numbers (`.inf`, `.nan`, `0x1F`, `0o17`) are valid there and have no JSON or I-JSON equivalent (SCH-013). The JSON form of a file holding several documents is not stated.

N7. Package address: "a host name that contains a dot" (NAM-001) cites no grammar for the host part (case, port, internationalized names).

N8. JSON Pointer: the string form is implied by `/config/model`. Whether the fragment form and the `-` token are accepted in a destination is not stated.

RECONSTRUCTED KEYS ("?" = not stated in any promise, inferred)

| Document | Keys | Source |
| --- | --- | --- |
| Envelope, every document | `apiVersion` (`conduit.dev/v0`), `kind`?, `metadata`, then `spec` or `from` | OBJ-017, OBJ-012, INS-001 |
| `metadata` | `name` (required for templates and contracts, optional for instances?), `description` string, `annotations` map of strings | OBJ-018, OBJ-003 |
| Template `spec`: Interface, Agent, Resource, Pool | `type` (qualified reference, required), `inputs`, `config`, `conduits`, `implementation`, `runtimes`, `setup`, `retire`, `controls`; Pool only: `entries` | TPL-001, TPL-002 |
| Template input (`inputs.<name>`) | schema key? (`schema` by analogy), default key?, destinations key? (one or more JSON Pointers rooted at `spec`), `optional` | definitions, INS-004 to 012, OBJ-015 |
| `implementation` (template) | map: operation qualified reference to implementation qualified reference, or a graph in place | TPL-003 |
| Runtime (`runtimes.<name>`) | `start`, `stop` (qualified references), `optional` | TPL-006 |
| Control in a template (`controls.<name>`) | `schema` (optional) | TPL-010 |
| Graph (`setup`, `retire`, in-place implementation) | steps under an unnamed key?; `outputs` (not in setup or retire) | GRA-021 |
| Step | name (map key?); exactly one of `operation`, `runtime`; `arguments` (input to value binding); `dependsOn` (list of step names); `optional` | GRA-015, 017, 018 |
| Graph output (`outputs.<name>`) | `value` (value binding), `optional` | GRA-021 |
| Value binding | exactly one of `{literal: v}`, a value reference, `{object: {k: binding}}`, `{list: [binding]}` | GRA-019 |
| Value reference | `source`: `input` + `name`; `config`; `step` + `step` + `output`; `runtime` + `name`; `instance` + `name` (`id` or `name`). `pointer` allowed on input, config, step | GRA-020 |
| Entry reference | `{entry: name}` | POOL-005 |
| Conduit template `spec` | `inputs`, and either `capability` (one qualified reference), `operations` (list, may be empty), `requirements`; or `components` | CON-016, CON-017 |
| Applied requirement (`requirements.<name>`, `constituents.<name>`?, Type `placement`?) | `requirement` (qualified reference), `arguments` | CON-018, NAM-019 |
| Instance document | envelope, `from`, `arguments`; Resource: `owner`, `placement`?; Agent: `placement`?; Conduit: `consumer`?, `target`? | OBJ-013, INS-015, POOL-001, PLC-001, PLC-006 |
| Declared Conduit (`conduits.<name>`?) | `from`, `target`?, `arguments`, `optional`; never `consumer` | OBJ-014, CON-020, OBJ-015 |
| Pool entry (`entries.<name>`?) | `from`, `arguments`, `optional`; never `owner`; `placement` only as an input destination | POOL-012, POOL-009 |
| Conduit component (`components.<name>`) | `from`, `arguments`; never `consumer` or `target`; `optional`? | CON-019, CON-020 |
| Operation `spec` | `inputs`, `outputs` (name to `schema` or `handle: true`, plus `optional`), `effect`, `startsModelWork`, `deduplicates`, `implementation` (one qualified reference) | CTR-002 |
| Capability `spec` | `operations` (list of qualified references) | CTR-003 |
| Type `spec` | `family` (required), `schema` (of `config`), `offers`, `requires`, `runtimes`, `implementation`, `controls` (name to `inputs`, `changes`), `placement` (required; map or list?), `conditions` | CTR-004, CTR-006, PLC-014 |
| Requirement `spec` | `inputs`, `appliesTo` (from `conduit`, `policy`, `placement`), `narrows`, `constituents` | CTR-005 |
| Manifest, project | `packages`: per package an address, one exact version, a digest, optional source (key names and list-or-map?) | NAM-006, NAM-012, PKG-002 |
| Manifest, package | adds `address`, `version`, `exports` (document names), `implementations` | PKG-017 to 019 |
| Implementation item | name (an export), `implements`, one of `command`, `url`, `graph`, `supports`, `placement`, `plain: true`, `tools` (operation to tool name) | PKG-019, PKG-020 |
| Wire `_meta` | request: `conduit.dev/interfaceVersion`, `conduit.dev/instance`, `conduit.dev/operation`, `conduit.dev/operationIdentity`, `conduit.dev/caller`; result: `conduit.dev/outcome` | IFC-006, IFC-009 |
| System-owned, rejected on submission | ID, revision, status (no key names) | OBJ-004 |

JUDGEMENT

The behavioral promises are far more settled than the document shapes they rest on. An implementer writing the JSON Schemas would stop first at three gaps. First, references: no promise says how a kind-and-name reference is written, which kinds `consumer` and `target` accept, or how a reference inside `config` or `arguments` is told from data, and the keys `kind`, `consumer` and `target` are never named at all (S11, S14). Second, the two most-used inner shapes are missing their keys: a template input's schema, default and destinations (S12), and the place where a graph's steps are written and how a step reaches another instance (S15, S16). Third, contracts and the manifest do not fit the document rules written before them: OBJ-001's name grammar rejects every contract name, INS-001 classifies a contract as a template, SUB-005 rejects a project that contains one, and the manifest has no file name, envelope or item keys (T12, T35, S5, S22). After those, the two flat contradictions (S1 map against list, S2 `inputs` against `schema`) and the stale "operation or runtime" wording (T33) are quick fixes, while T9, T10, T25 and T26 need one ruling each before the vocabulary can be called consistent.
```
