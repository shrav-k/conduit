# Consolidation pass 1 — log of group C

Written 2026-11-05 by the editor of group C, under the rules of
[CONSOLIDATION.md](CONSOLIDATION.md). Group C owns `promises/objects.yaml`,
`promises/instantiation.yaml`, `promises/submissions.yaml`, `promises/templates.yaml`,
`promises/graphs.yaml`, `promises/contracts.yaml`, `promises/schemas.yaml`,
`promises/naming.yaml`, `promises/packages.yaml` and `schema-profile.md`. No other file was
edited.

Finding IDs: `R1-n.m` is reader 1's section and item, `R2-Tn`, `R2-Sn` and `R2-Nn` are reader
2's term, shape and standards findings, `R3-n` is reader 3's finding. Decision IDs are those of
the [re-audit record](README.md). "Brief" means the coordinator's instructions to group C.

## Counts

| File | Before | Changed | Removed | Added | After |
| --- | --- | --- | --- | --- | --- |
| `promises/objects.yaml` | 19 | 15 | 0 | 3 | 22 |
| `promises/instantiation.yaml` | 19 | 15 | 1 | 3 | 21 |
| `promises/submissions.yaml` | 7 | 6 | 0 | 1 | 8 |
| `promises/templates.yaml` | 12 | 9 | 0 | 0 | 12 |
| `promises/graphs.yaml` | 22 | 10 | 2 | 2 | 22 |
| `promises/contracts.yaml` | 6 | 5 | 0 | 1 | 7 |
| `promises/schemas.yaml` | 14 | 7 | 0 | 3 | 17 |
| `promises/naming.yaml` | 21 | 15 | 0 | 5 | 26 |
| `promises/packages.yaml` | 20 | 8 | 6 | 1 | 15 |
| Total | 140 | 90 | 9 | 19 | 150 |

The validation command of CONSOLIDATION.md passes with 339 promises across `spec/`, and
`python3 tools/check_docs.py` reports no error. Every line of the nine YAML files is under 97
characters. Every promise ID cited in group C's files exists, and no promise in any file of
`spec/` cites a removed ID of group C outside a header comment. OBJ-015 was only re-wrapped to
fit the line length and is not counted.

## objects.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| OBJ-001 | Scoped to templates and instances of the five families; a contract's name follows NAM-004. | R1-2.13, R2-S5 |
| OBJ-002 | Registration reports conflict; "a submission only adds objects and never changes one"; the first-start names are taken in the same way (FST-004 folded in). | W4, R1-1.10, R2-T26 |
| OBJ-003 | Covers nested instances; dots in the template's name become hyphens; shortened to fit 63 characters; cites POOL-014 for reaching an entry's Resource from outside. | G3, R3-15, R3-41, R2-S10 |
| OBJ-004 | Names the fields: `id`, `revision`, `status` and the kept package list; "the system" became "the engine". | R2-S11, R2-T20 |
| OBJ-005 | "A registered object" became "An object". | W1 |
| OBJ-006 | A reference is `{kind: <Kind>, name: <name>}`, or the bare name where the field's schema fixes the kind. | G2, R2-S14 |
| OBJ-009 | "Created" became "instantiated"; the instance document is rejected. | W3, W4 |
| OBJ-010 | "Target" and "removed" replaced; "reports it as missing" left to LIF-019. | R2-T11, R1-1.24 |
| OBJ-011 | Reduced to the rename operation. The rejection is OBJ-002. | R1-5.1, R1-1.10 |
| OBJ-012 | Adds `apiVersion` and names `kind`. | R1-2.12 |
| OBJ-013 | Adds `apiVersion`; names the standard-argument keys `owner`, `placement`, `consumer`, `target`; takes in INS-015. | R1-2.12, R2-S11, R1-1.20 |
| OBJ-014 | The three nested instantiations stated separately, with what each may not carry and how an entry gets its placement. | G3, R2-S3, R2-T31, R1-3.3 |
| OBJ-016 | "Path" became `pointer`; string form of JSON Pointer only; `-` not accepted in a destination. | R2-N8, R2-T28 |
| OBJ-018 | `name` required in a template and a contract, optional in an instance. | R2-S4 |
| OBJ-019 | Exempts names that key a map of named members, members of an `object` binding, and the qualified-reference keys of `implementation`, `controls` and `tools`; exempts `arguments` only on an instance, a nested instantiation and an applied requirement. | R2-S6, R2-S9 |
| OBJ-020 | Added. Every template, instance and contract carries `kind`, one of nine. | R2-T12, R2-S11 |
| OBJ-021 | Added. A value in `config` or `arguments` is a reference only where its schema is one of the core package's reference schemas; anything else is data and is never rewritten. | G2, R3-16 |
| OBJ-022 | Added. The maps of named members, and the rule their names follow (OBJ-001's). | R2-S9, R2-S17 |

## instantiation.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| INS-001 | The test applies only to the five family kinds. | R2-T12 |
| INS-002 | "Not created" became "rejected"; the stale "no instance exists without a template" reworded. | G7, W3 |
| INS-003 | No longer treats standard arguments as written under `arguments`. | R1-3.3 |
| INS-004 | Names `default` and `optional: true`; "rejected". | R2-S12, W3 |
| INS-005 | Drops "no value is converted" (SCH-011 has it). | R1-1.20 |
| INS-006 | Drops "no part of a string is substituted" (GRA-019 has it). | R1-1.20 |
| INS-007 | Names the key `default`. | R2-S12 |
| INS-008 | Trimmed to what is unique; the order is INS-019's. | R1-1.20 |
| INS-012 | Names the keys a destination may not reach and the places it may. | R1-2.23, R2-S18 |
| INS-013 | The instance holds its nested instantiations; a replacement is made from them (LIF-009); "record" removed. | G7, W2 |
| INS-014 | The type's `schema` is the schema of the configuration. | R2-T32 |
| INS-015 | Removed, folded into OBJ-013. | R1-1.20 |
| INS-016 | Drops the state clause (LIF-002 has it); states the composite case by citing CON-003. | R1-1.2, R1-2.16 |
| INS-017 | "Registered objects" became "objects that exist". | W1 |
| INS-018 | "Submitter" removed. | R2-T7 |
| INS-019 | Cites SCH-009 for the last step. | R1-1.20 |
| INS-020 | Added. Instances keep what they resolved when their template is replaced or unregistered, and do not prevent either. | G7, R3-26 |
| INS-021 | Added. A template input: `inputs.<name>` with `schema`, `to`, and optional `default` and `optional: true`. | R2-S12 |
| INS-022 | Added. Instantiation makes every check of registration again. | R3-28 |

## submissions.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| SUB-002 | "Creates" and "registered" reworded. No statement of authority was present, so decision A1 changes nothing here. | W1, W3 |
| SUB-003 | "Registered objects" became "objects that exist". | W1 |
| SUB-004 | "Or would change the same object" deleted. | R1-3.2 |
| SUB-005 | A submission may contain contract documents. | G10, R1-2.13, R2-T35 |
| SUB-006 | Second sentence and state clause removed, citing INS-016; mentions SUB-008. | R1-1.2 |
| SUB-007 | Reduced to the replace operation; cites INS-020 for instances made before a replace. The rejection is OBJ-002. | R1-5.1, R1-1.10, G7 |
| SUB-008 | Added. Contracts in a submission are made available to that project's documents and are not registered. | G10 |

## templates.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| TPL-001 | States which keys are maps keyed by name. | R2-S17 |
| TPL-003 | A value is a string (a reference) or a map (a graph in place); one selection for an operation (PKG-010 folded in). | R2-S8, R2-S15, R1-1.11 |
| TPL-004 | "Entry" became "key"; a default resolves through its own package's manifest (PKG-009 and PKG-011 folded in). | R1-2.7, R1-1.11, R2-T15 |
| TPL-006 | A runtime is a member of a `runtimes` map with `start`, `stop` and optional `optional: true`. | R2-S17 |
| TPL-007 | A runtime the instance already holds is not started again; the step is left as succeeded. | U5, R3-4 |
| TPL-008 | "Has no outputs" left to GRA-021; cites GRA-010 and GRA-020. | R1-1.15 |
| TPL-009 | An instance never set up does not run its `retire` graph; "nothing else" now excepts closing its Conduits under CON-008; the stops are part of the retire operation; when shutdown is confirmed is left to the lifecycle convention. | G12, R1-2.9, R1-1.23 |
| TPL-010 | Keys are qualified references to the type's controls; each value may hold `inputs`, a map of input name to a narrowing `schema`; "a template cannot define a control" dropped. | G11, R2-S2, R1-1.22 |
| TPL-011 | Follows TPL-010; "invoked" became "requested". | G11, R2-T28 |

## graphs.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| GRA-001 | "Author" removed. | R2-T7 |
| GRA-002 | Layer became `convention`. | R1-4.2 |
| GRA-003 | Layer became `convention`; cites OPS-022 for the outcome; states once that a required declared Conduit or entry that is not opened or set up stops setup succeeding, and an optional one is reported. | Q3, R1-4.2, R1-1.8 |
| GRA-010 | `config` is accepted only in a setup or retire graph; the "may refer only" sentence deleted. | R1-2.8, R2-S7, R3-35 |
| GRA-011 | Kept only for a pointer into a handle. | R1-1.15 |
| GRA-013 | Removed, folded into GRA-019. | R1-1.15 |
| GRA-015 | `arguments` is a map keyed by input name. | R2-S6 |
| GRA-016 | Removed, folded into GRA-020. | R1-1.15 |
| GRA-017 | A step may also carry `conduit`. | G4 |
| GRA-019 | Takes GRA-013's rejection examples. | R1-1.15 |
| GRA-020 | Takes "`from` in a value reference is rejected"; `step`, `runtime` and `instance` are accepted in every graph. Keys unchanged. | R1-1.15, R3-35 |
| GRA-022 | "Written step" became "a step written under `steps`". | R2-T8 |
| GRA-023 | Added. A graph is `steps`, a map of step names to steps, and `outputs` where GRA-021 requires them; `setup` and `retire` each hold `steps`. | R2-S15 |
| GRA-024 | Added. Without `conduit` a step invokes the implementation selected for its own instance; with it the step is a request through that declared Conduit, which must list the operation. | G4, R3-17, R2-S16 |

## contracts.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| CTR-001 | Names `kind`; the name follows NAM-004; a contract may also arrive in a submitted project. | G10, R1-2.13 |
| CTR-002 | Shape only: member keys of `inputs` and `outputs`, `implementation` is one qualified reference, `changes` for a control; the meaning of the marks is left to IMP-003 and OPS-012. | R1-1.16, R2-S8, G11 |
| CTR-004 | `implementation` in TPL-003's map form; `controls` a list of qualified references; `placement` a map of applied requirements; a default the package's manifest cannot resolve is rejected (PKG-011 and PLC-014 folded in). | G11, G5, R1-2.15, R2-S1, R2-S8, R1-1.11 |
| CTR-005 | A requirement that applies to placement states `schema` and declares no `inputs`. | G5, R3-18 |
| CTR-006 | A control is an Operation contract with `changes`, listed by a Type under `controls`. | G11, R3-24, R2-S21 |
| CTR-007 | Added. Two operations in one capability may not share an export name. | R3-40 |

## schemas.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| SCH-001 | For a listed format, SCH-004's grammar is the one checked. | R2-N5 |
| SCH-002 | "Accepted into the registry" and "author" replaced: the document is rejected when its package is installed or when it is submitted. | R1-3.5, R2-T4, R2-T7 |
| SCH-003 | The same. | R1-3.5 |
| SCH-008 | The same. | R1-3.5 |
| SCH-009 | "Created" became "instantiated". | W3 |
| SCH-010 | "Author" removed. | R2-T7 |
| SCH-014 | "External MCP service" became "existing Model Context Protocol server". | R2-T28, vocabulary table |
| SCH-015 | Added. `$ref` is a same-document JSON Pointer or a qualified reference to a schema a package exports. | R2-N5 |
| SCH-016 | Added. `pattern` is an ECMA-262 regular expression. | R2-N5 |
| SCH-017 | Added. YAML numbers with no JSON form are rejected. | R2-N6 |

## naming.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| NAM-002 | Scoped to a package the document does not belong to, since NAM-022 gives a form for one's own. | G10 |
| NAM-003 | "Registered objects" became "the objects that exist". The rule is unchanged. | W1, G10 |
| NAM-004 | The example `directory.establish` dropped. | R1-3.12 |
| NAM-006 | Takes "a package is used at exactly one version" from PKG-002; "entry" became "item". | R1-1.12, R2-T15 |
| NAM-007 | Last sentence dropped (NAM-020 has it). | R1-1.24 |
| NAM-009 | "Records" became "keeps". | W2, R2-T10 |
| NAM-010 | "Recorded" and "creation" reworded. | W2, W3 |
| NAM-011 | Takes PKG-008: installing or removing a package changes nothing a template or instance holds; "pinned" and "unavailable" removed. | R1-1.12, R2-T8, R2-T21 |
| NAM-012 | Cites PKG-021 for the digest. | U11 |
| NAM-013 | "Fully" dropped; "registered objects" reworded. | R2-T13, W1 |
| NAM-016 | Layer became `convention`. | R1-4.5 |
| NAM-017 | Layer became `convention`; "refused" became "kept from opening"; the observable is "package versions are not compared" alone. | R1-4.5, R1-2.22, R2-T21 |
| NAM-018 | Adds the graph case and the `#name` form; `driver` and `handler` dropped. | R1-2.22, R1-3.12, G10 |
| NAM-019 | Names the key `requirement`. | R2-S14 |
| NAM-020 | "Entry" became "item"; "built-in schemas" became the schemas the core package exports, its reference schemas among them. | R2-T15, G2 |
| NAM-022 | Added. `#name` refers to a document in the same project or package. | G10, R3-46 |
| NAM-023 | Added, `convention`. The contract as the target holds it governs a request. | G8, R3-27 |
| NAM-024 | Added. Within one contract version a later release may add optional things and may not remove or change any. | G8 |
| NAM-025 | Added. A breaking release takes a new address. | R3-45 |
| NAM-026 | Added, `convention`. A request for an operation the target's version lacks reports unavailable. | G8, R3-27 |

## packages.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| PKG-002 | Removed, folded into NAM-006. | R1-1.12 |
| PKG-003 | "Dependency" became "manifest item"; versions compared by Semantic Versioning precedence, build metadata set aside; the digest decides identity. | R1-3.6, R2-N3 |
| PKG-004 | The digest is written `sha256:` and lowercase hexadecimal. | R2-N4 |
| PKG-005 | The instance holds the versions and digests its template holds. | R1-1.12, R1-3.6 |
| PKG-006 | Cites INS-022 for the checks made later. | R3-28 |
| PKG-007 | "Depends on" became "uses"; "rejected". | R1-3.6, R2-T16 |
| PKG-008 | Removed, folded into NAM-011. | R1-1.12 |
| PKG-009 | Removed, folded into TPL-004. | R1-2.7 |
| PKG-010 | Removed, folded into TPL-003. | R1-1.11 |
| PKG-011 | Removed, folded into TPL-004 and CTR-004. | R1-1.11 |
| PKG-015 | Removed, folded into PLC-007. | R1-1.13 |
| PKG-016 | Scoped to an implementation not marked `plain`; layer became `interface`. | R3-49, R1-4.9 |
| PKG-018 | Cites NAM-022 for a reference from inside the package. | G10, R3-46 |
| PKG-020 | A plain implementation states no `supports` and never counts as deduplicating; "knows nothing of Conduit" and "entry" reworded. | U1, U4, R2-T20, R2-T15 |
| PKG-021 | Added. A package whose archive digest differs from the manifest's is unresolved, as under PKG-003. | U11, R3-8 |

## schema-profile.md

Rows added for the Model Context Protocol revision 2026-07-28, Semantic Versioning 2.0.0,
SHA-256 (FIPS 180-4) and ECMA-262 (R2-N1, R2-N3, R2-N4, R2-N5). The JSON Pointer row states the
string form and that `-` is not accepted in a destination (R2-N8). The YAML section lists the
numbers with no JSON form (R2-N6). A paragraph states the two forms of `$ref` (R2-N5). The
formats section says the cited grammar is the one checked. "Owner" as a person and "MCP" were
removed from the prose (R1-3.9, vocabulary table).

## Choices the coordinator should check

1. **OBJ-011 and SUB-007 kept their IDs as the rename and replace promises.** R1-1.10 says to
   reduce each to its operation, and the rejection each also stated is OBJ-002. No new ID was
   needed for split 5.1.
2. **Layers changed in place under rule 3 beyond the two the brief named:** NAM-016 and NAM-017
   to `convention` (R1-4.5), and PKG-016 to `interface` (R1-4.9). Group B's log reads R1-4.9
   as a move and left it; if that reading is preferred, set PKG-016 back to `convention`.
3. **Trims not on the brief's list but in reader 1 section 1:** R1-1.2 on INS-016 and SUB-006,
   and R1-2.16 on INS-016 (settled by CON-003).
4. **A template's `controls` map is keyed by qualified reference** (decision G11), so it is not
   among the maps of author-chosen names in OBJ-022, though the brief's list had it there.
5. **The U5 rule is inside TPL-007**, not a new promise, because the decision table and
   OPS-021 both point at TPL-007.
6. **OBJ-014 holds all three shapes in one promise.** It overlaps POOL-012 and CON-020, which
   group A owns.

## Left undone, and why

1. **The grammar of an operation's input and output names.** The brief asked that every
   author-chosen map key follow OBJ-001. OBJ-022 applies that to a template's `inputs`,
   `conduits`, `entries`, `runtimes`, `requirements` and `components`, a Type's `runtimes` and
   `placement`, a Requirement's `inputs` and `constituents`, and a graph's `steps`. It leaves
   out the `inputs` and `outputs` of an Operation contract, and so the keys of a step's
   `arguments` and of a graph's `outputs`. A plain server's tool takes the operation's inputs
   as its arguments under IFC-001, and existing servers name arguments in camelCase or with
   underscores, which OBJ-001 would reject. Needs a decision.
2. **Whether a template that a package exports may have dots in its name.** OBJ-001 covers
   every template of the five families, and NAM-004 allows dots in an export's name. OBJ-003
   replaces dots as the brief asked; under OBJ-001 alone none can occur.
3. **Destinations neither forbidden nor listed in INS-012:** a place inside a component's
   `arguments` in a composite Conduit template (without it such a template's inputs can reach
   nothing), a Conduit template's `capability` and `operations`, and the `from` and `optional`
   of a nested instantiation. R2-S18 asked for a ruling; the record lists only what INS-012
   now states.
4. **`conduit` on a step that has `runtime`** is neither accepted nor rejected by GRA-024.
   Whether a step with `conduit` depends on that declared Conduit's step without saying so is
   also not stated. Decision G4 does not cover either.
5. **Which kinds `consumer` and `target` accept** (R2-S14). Not in the record.
6. **"Resolving returns" against "Registration returns"** (R2-T3). No decision; left as it was.
7. **"Source" with three meanings** (R2-T14, NAM-012). A rename is not in the vocabulary table.
8. **Whether a schema is an export, and of what kind** (R2-T17). SCH-015 and NAM-020 speak of
   a schema that a package exports, as the brief worded it. The export names are decision G1.
9. **`default` on a property of an object inside `items`** (R2-N5, SCH-008). The text can be
   read either way; needs a decision.
10. **The JSON form of a file that holds several documents** (R2-N6), **the grammar of the host
    part of an address** (R2-N7) and **the archive format** (R2-N4). Not in the record.
11. **NAM-025 is not checked by anything** (R3-45). PKG-001 accepts any valid version at an
    address, and "breaks" is defined only through NAM-024.
12. **W4 as a promise.** Only OBJ-002 states a reason, conflict. That a rejected document
    otherwise reports invalid is left to `definitions.md`, as group B also left it.
13. **Key names left to decision G1:** the kept package list of OBJ-004, the manifest's file
    name and item keys, how a re-export is written, the values of `command`, `url`, `graph`,
    `supports` and an implementation's `placement`, the values of `family`, `offers`,
    `requires`, `effect` and `conditions`, the pattern of a version segment, and how an
    operation is marked as reserved to the root Interface.
14. **What the `metadata.name` of an instance document names when its template is a composite
    Conduit** (R2-S23). Not in the record.
15. **How long a project's contracts stay available, and whether a template keeps them as it
    keeps package versions** (SUB-008). Decision G10 does not say.
16. **Other tool-name collisions of R3-40:** components of a declared composite Conduit, and a
    rename onto a declared Conduit's name. Only the capability case was instructed (CTR-007).
17. **Not moved**, under rule 3: TPL-009 (R1-4.10), PKG-016 (R1-4.9), IMP-003 to IMP-005
    (R1-4.8, group B's files). **Splits not applied**, as instructed: R1-5.5 (TPL-010), 5.6
    (CTR-002, CTR-003, CTR-005) and the parts of 5.10 that concern OBJ-019 and GRA-019.
18. **"Owner rulings" in the header comments of the YAML files** (R2-T22) was left; the fix is
    either there or in the definition of owner, which the coordinator edits.

## For `definitions.md` and `README.md`

Rows that need to follow this pass: family and kind (nine kinds); document, project and
submission (contracts, decision G10); template and instance (`apiVersion`); nested instantiation
(three shapes, generated names); standard argument (the four keys); input and destination
(`schema`, `to`, `default`); optional (entry and runtime); reference (decision G2) and the
`#name` form; type (its `schema` is of `config`); implementation ("or a runtime"); control
(decision G11); definition (nested instantiations are part of it); digest (how it is written);
contract (NAM-024); core package (reference schemas). Terms the new text uses without a row:
hold, nested instance, applied requirement, item, reference schema.

README promise counts: objects 22, instantiation 21, submissions 8, templates 12, graphs 22,
contracts 7, schemas 17, naming 26, packages 15.
