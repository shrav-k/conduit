# Re-audit round 2, reader 2: terms and document shapes

I read every file under `spec/` in full (README, definitions, schema profile, names, the implementation-defined list, all 395 promises in 22 files, the 8 files of `spec/core/`, the 9 schema files, their README, and all 73 example files) and the "Known before the round" list of this directory's README; I saw nothing else.

Tools, as run from the repository root. Both exit 0. Last lines of each:

```text
$ python3 tools/check_schemas.py
4. 55 invalid documents in 55 files were each rejected by a schema, or refused when read.
5. Core package: 98 exports, 0 documents deliberately private and 1 implementation. Checked 396 references for the kind they name; 1 name packages that are not in this repository and were not checked. Checked 3 controls, the 5 operations that take no `revision`, the tables of the two READMEs, and 689 citations of promises in 93 files. The example packages keep 0 documents private.
6. Applied 10 rules of this tool, which come from 9 promises, to the core package and 3 example directories. 11 sets of documents in 11 files under examples/invalid-by-rule/ were each rejected by the rule they are named for; each of their 16 documents is valid against the schemas.
   SCH-019: validated 8 defaults of the core package and the valid examples against their schemas; 0 were not validated, because a `$ref` in the schema leads outside this repository.
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x401, apiVersion x208, metaPrefix x63. Errors: 0
```

The counts in `spec/README.md` are right: 395 promises, with each file's count as stated; 14 implementation-defined items; 98 exports (7 templates, 6 types, 5 capabilities, 20 operations, 2 requirements, 58 Schema contracts) and 1 implementation; 55 and 11 invalid examples.

Findings are numbered within each heading, most severe first. Points on the "Known before the round" list are not repeated.

## Terms

**T1. "Setup operation" and "retire operation" name two different things, and neither has a row.**
GRA-014: "A setup graph implements its instance's setup operation." TPL-008: "The graph implements the retire operation of its instances." IMP-005: "The setup operation the graph implements is never marked as starting model work", which implies a contract that could carry the mark. Definitions, `set up`: "Invoke an instance's setup operation". But the definitions also say an operation is "defined by an Operation contract", and the only such contracts are `set-up.v1` and `retire.v1` in `spec/core/operations.yaml`, each with `implementation: github.com/shrav-k/conduit/core#engine`. So either the instance's setup operation is `set-up.v1`, and then a template's graph "implements" an operation whose implementation is the engine, outside the `implementation` map of TPL-003 and TPL-004; or it is a second, unnamed operation with no contract. TPL-009 adds that "The stops are part of the retire operation", so the `retire` graph does not implement the whole of it either. LIF-018 reconciles "the retire operation" with `reconcile.v1`, which names an invocation, so there the term means an invocation of `retire.v1`.
Smallest fix: add rows for "setup operation" and "retire operation" that say they are `set-up.v1` and `retire.v1` (for a Conduit `open.v1` and `close.v1`) applied to the instance, and that the `setup` and `retire` graphs are what the engine runs as part of them; reword GRA-014, TPL-008, IMP-005 and IMP-009 so the graph is not said to "implement" a contract whose implementation is `engine`. If a separate operation is meant, it needs a contract: needs a decision.

**T2. "Project" is defined as a directory, but the one operation that takes documents takes no directory.**
Definitions: "project | One directory with at most one manifest." `register.v1` takes `documents` (a list of `document.v1`) and an optional `manifest`. MAN-001 already allows "several files are submitted without a directory". NAM-027 ("Two documents of one project"), SUB-008 ("made available to the documents of that project") and NAM-022 ("the same project") all depend on what a project is for a `register.v1` request. Read literally, each request is its own project, so a later request cannot reach an earlier request's contracts as `#name`.
Smallest fix: define a project by its content (the documents and the one manifest handed over together, of which a directory is one written form), or state that the `documents` and `manifest` of one `register.v1` request are one project. Needs a decision between "one request is one project" and "a project persists across requests".

**T3. "Resolve" has a third, run-time meaning that contradicts the definition.**
Definitions: resolving "runs nothing and stores nothing. What it yields is called resolved"; the README says the language layer "is answered by resolving". GRA-025 uses the word at run time: "the arguments that its value bindings resolve to", "The check is made on the resolved values"; GRA-026: "A step's arguments are resolved"; definitions, `step`: "A step whose resolved arguments". Other uses also fall outside the two rows: "A default resolves through the manifest" (TPL-004), "a relative one is resolved against the place of the manifest" (MAN-006), "never opens, resolves or checks it" (PLC-020).
Smallest fix: in GRA-025, GRA-026 and the `step` row say "the values its value bindings yield" (GRA-020 already uses "yields"), and keep "resolve" for documents and references.

**T4. A step that "is not a request" still "reports invalid" and is "dispatched".**
Definitions: "Every request ends in exactly one of these seven ways"; `dispatch` is "The engine's passing of a request to the implementation". ACC-018: "A step with no `conduit` invokes the implementation selected for its own instance, and is not a request through any Conduit ... makes no request to another instance." Yet GRA-025 and GRA-026: "The step is not dispatched, and reports invalid", and OPS-026: "before the request or the step is dispatched". LIF-008 has a step "report conflict". So the reasons and "dispatch" are applied to something the definitions exclude.
Smallest fix: widen the rows `request` and `dispatch`, or the "How a request ends" paragraph, to say that a step of a graph ends in the same seven ways and is dispatched in the same sense, whether or not it is a request through a Conduit.

**T5. "Manifest item" means two things.**
PKG-003: "A manifest item names a version of a package" (an item of `packages`); PKG-020: "Its manifest item is marked `plain: true`" (an item of `implementations`); definitions, `requirement on placement`: "an implementation states its own in its manifest item"; definitions, `resolve`: "a manifest item, that reaches nothing is unresolved"; `fetch` row: "The key of a manifest item". MAN-009 and MAN-010 already say "implementation item".
Smallest fix: use "item of `packages`" and "implementation item" throughout, and say which the `resolve` row means.

**T6. "Local name" is used for a local reference.**
Definitions: "local name | The name by which a consumer's tools name one of its Conduits" (IFC-012). NAM-003 observable: "A local name resolves only among the objects that exist." NAM-003 also says "a reference to a local object", and "local object" has no row.
Smallest fix: in NAM-003 write "A local reference resolves only among ..." and "a reference to an object that exists".

**T7. "Entry" is used outside the one meaning the definitions allow, and the implementation-defined list is called both entries and items.**
Definitions, `entry`: "In the promises the word is used only for this." LIF-013: "The records are the log entries that name the instance." Definitions, `record`: "A log entry"; `purge`: "as entry DL2 of the implementation-defined list", while `cancel` says "item DL14" and the `item` row says "an item of ... the implementation-defined list". `implementation-defined.yaml` keys each item `entry:` and says "its choice for every entry"; `spec/README.md` says "every item".
Smallest fix: "log lines" or just "records" in LIF-013 and the `record` row; "item DL2" in the `purge` row; rename the key `entry` (for example `subject`) and say "every item" in `implementation-defined.yaml`.

**T8. "Plain" has three meanings, all three used in `definitions.md` itself.**
Row `plain`: "Said of an implementation that is an existing server". Row `argument`: "write plain values there"; row `applied requirement`: "are plain values" (also `applied-requirement.v1`). Row `core package`: "the plain Interface and Pool types" (also `spec/README.md`, `core/README.md`, `core/types.yaml`).
Smallest fix: "literal values" or "values as written" for arguments (the row `argument` then contrasts them with value bindings), and "basic" or "untyped-configuration" for `interface.v1` and `pool.v1`.

**T9. Verbs and nouns the promises rely on that have no row.**
- "replace" and "replacement": `replace.v1` replaces a template (SUB-007); LIF-009 calls a new instance for a nested instantiation "a replacement" and names `register-replacement.v1`. Two meanings and no row. "rename" has no row either, although every other management verb has one.
- "an operation ... that manages an object" (CON-024: "for an operation other than one that manages an object"). It is not said whether this equals "management verb" (which is defined inside the `operation` row), nor whether `inspect.v1` or a control counts. CON-024's outcome depends on it.
- "work" and "arranged" (ACC-019: "Work that a caller arranged through a Conduit runs at a later time"; IMP-006: "Work is to start at a later time"). Only IMP-006 ties it to model work.
- "combiner" (SCH-009: "a combiner gives no default"; `schema-profile.md`): means `allOf`, `anyOf`, `oneOf`.
- "supports" (ACC-015: "whose implementation does not support it"; CON-022): explained only inside PKG-019.
- "a reference" in IFC-016: "The call returns a reference among its outputs". It is none of the forms of reference the definitions list; see G11.
- "editing mode" (DL7: "and any editing mode"): nothing else in `spec/` says what it is.
Smallest fix: one row each, or reword the promise in defined terms.

**T10. Three words for what comes back from a request.**
"result" (92 lines of the promises and definitions use it), "answer" (31) and "response" (14) are used for the same thing: OBJ-023 "The response carries the diagnostics"; IFC-025 "The engine answers a consumer's request ... the result's `_meta`"; OPS-009 "the response gives the invocation's state ... The answer to a repeat"; OPS-005 "The response to the caller". Only "retained result" has a row.
Smallest fix: one row, "result: what a request returns, a tool result of the protocol", and use it; keep "answer" as a verb only.

**T11. "Publish" is used for exporting or holding a contract.**
Definitions, `published`: "Made available as an archive under an address." TPL-004: "the package that publishes the contract stating it"; NAM-023: "the package that publishes the operation's contract". For a contract of an unpublished package the phrase contradicts the definition. Also "A schema published by an existing Model Context Protocol server" (SCH-014) and "five kinds of published definition" (header of `promises/contracts.yaml`).
Smallest fix: "the package that holds the contract" in TPL-004 and NAM-023; "a schema of an existing server" in SCH-014.

**T12. Statements in the definitions about how a word is used, that the text contradicts.**
- `optional` row: "The word `required` appears only as the schema keyword." The same row says "is required unless it is marked", and promises use the word freely: OBJ-015 "Without that mark it is required", OBJ-018 "`name` is required in a template", TPL-006 "exactly one required input", GRA-021 "a required output", NAM-019 "a missing required one", CON-016 "requires `capability`". Fix: "The key `required` appears only as the schema keyword."
- `record` row: "The word is used for nothing else". It is also a verb (OPS-014 "An actor records, with what it checked"), and `core/README.md`, `schemas/README.md` and the file headers use "the record" for the design record under `docs/proposals/`. Fix: say "As a noun in the promises".
- `source` row: "The word has no other use". `spec/README.md`, `core/README.md` and `schemas/README.md` say "source material" and "source of truth". Fix: limit the statement to the promises.
- `hold` row defines "Keep as part of what is stored for a template or an instance", but the promises use it far more often for "a map has this key": OBJ-014 "A declared Conduit holds `from`", CTR-002 "Its `spec` may hold", IFC-006 "The request's `_meta` holds", REC-002 "It holds the caller". Fix: add that second sense to the row, or use "has" for keys.

**T13. Reserved words reused in an ordinary sense.** Each is small; together they undo "each with one meaning".
- "bound": the noun is a limit on a Conduit to `access`. ACC-019 and IMP-006: work "is bound to the Conduit through which it was arranged"; and the defined term "value bound to a location".
- "refused": a reason for a request. `spec/README.md` and `schemas/README.md`: documents "that are refused when the file is read"; `schema-profile.md`: "refusing a `default` placed inside a combiner". Documents are "rejected".
- "enforced" and "permitted": ACC-027's value for a restriction, and "permitted" for a request; `schema-profile.md` has "Enforced formats" and "Permitted schema keywords".
- "object": "A template or an instance of one of the five families", and also a JSON object ("`settings` is an object keyed by ...", "one object that holds `documents`", the `object` value binding).
- "document": "A manifest is not a document in this sense", but MAN-001: "it holds exactly one document ... A manifest that holds two documents".
- "state": the key `state` holds a lifecycle state in the `status` of `inspect.v1` and an invocation's state in `retrieve-result.v1` and in `io.github.shrav-k.conduit/state`.
- "implementation": "What does the work behind an operation", but `isolation plugin` is defined as "An implementation that confines a consumer", which is behind no operation.
- "tool": "An implementation has one for each operation it implements (IFC-001)". IFC-001 covers only implementations reached by `command` or `url`; a `graph` implementation and `engine` have none.
Smallest fix: reword each quoted sentence; none needs a ruling.

**T14. `capabilities.yaml` widens "right to open".**
Definitions: a right to open is what an actor has "when it is the consumer of an open Conduit to `access` that lists the operation that opens Conduits". `core/capabilities.yaml`, `access-control.v1`: "An open Conduit to `access` is a right to open." A Conduit to `access` that lists only `apply-policy.v1` is not one under the definition.
Smallest fix: "An open Conduit to `access` that lists `open.v1` is a right to open."

**T15. NAM-019 says a document applies a policy; no document can.**
NAM-019: "A document applies a requirement, as a filter, as a policy or as a requirement on placement." Definitions, `policy`: "It is never written in a template", and it "is applied by `apply-policy.v1`"; OBJ-019 rejects "a policy written in a template"; `applied-requirement.v1` describes itself as "on a Conduit or on placement". The row `argument` and OBJ-019 speak of "the `arguments` of ... an applied requirement" as a document's keys only.
Smallest fix: in NAM-019 write "A document applies a requirement as a filter or as a requirement on placement, and a request applies one as a policy; each is written as `requirement` and `arguments`".

**T16. Small disagreements between a row and its promise.**
- `manifest` row: "`exports` is a list of names". MAN-007: "each item is either the name of a document of the package ... or a qualified reference".
- `core package` row lists what it exports and omits the two result schemas `object.v1` and `diagnostic.v1`.
- `input` row gives the shape of a template's input and of an Operation's input, and says a Requirement contract declares inputs, but gives no shape for those (see S2).
- `nested instance` row: "A component Conduit is made in the same way from a component", which leaves open whether a component Conduit is a nested instance; LIF-008 and LIF-009 depend on it (see C2).

## Shapes

**S1. No promise states what the value of `from` may be, and the definitions and the schema disagree about `#name`.**
OBJ-013 and OBJ-014 say only that a document "has `from`"; INS-002 says it must "resolve to a template of the same kind"; FST-005 says first-start objects report "a `from` that is a qualified reference". The forms are given only by `template-reference.v1`: a bare name, `{kind, name}`, or `selection.v1` (a qualified reference or `#name`). Three points follow.
- The schema reuses `selection.v1`, and PKG-013 treats `from` as a selection ("A selection resolves to something a package re-exports"; MAN-008's example re-exports the template `shared-reviewer`). The `selection` row and NAM-018 limit a selection to "a type, a capability, an operation, a requirement or an implementation", and NAM-018's list of places does not include `from`.
- `template-reference.v1` says "`#` and a name for a template of the same package". The `#name` row and NAM-022 say "the same project or package". NAM-022's observable speaks only of contracts for a project. So whether `from: '#reviewer'` is valid in a project that is not published cannot be determined. If it is, a template of a project can be named two ways (`reviewer` under NAM-003 and SUB-002, `#reviewer` under NAM-022) with different lifetimes.
- Inside a package a bare name is "never looked up in a package" (NAM-003), so a package's composite or Pool template must write `from: '#name'`. No promise or example says so; both valid examples that nest instantiations are in the unpublished project and use bare names.
Smallest fix: one promise that lists the forms of `from` and what each resolves against; add "a template" to the `selection` row and `from` to NAM-018, or stop calling it a selection in PKG-013. Needs a decision on `#name` for a template of an unpublished project.

**S2. CTR-004 and CTR-005 say the `spec` "holds" keys that the schemas make optional, and say nothing of what leaving one out means.**
CTR-002 says "may hold". CTR-004: "Its `spec` holds `family`; `schema` ...; `offers` ...; `requires` ...; `runtimes`; `implementation` ...; `controls` ...; `placement` ...; `conditions`". `type-spec.v1` requires only `family` and `placement`; the core types omit `runtimes`, `implementation`, `controls` and `conditions`, and the Type in each of two examples (`invalid/CTR-004-records-as-a-list.yaml`, which is named as invalid for its `records` alone, and `invalid-by-rule/NAM-027-two-documents-one-name.yaml`, which the tool reports as valid against the schemas) omits `schema`. CTR-005: "Its `spec` holds `appliesTo` ...; `inputs`; `narrows`"; `requirement-spec.v1` requires only `appliesTo`, `shared-host.v1` has no `inputs`, and `local-paths.v1` has neither `inputs` nor `narrows`.
Not stated anywhere: what configuration a Type with no `schema` accepts (anything, or only `{}`); that an absent `narrows` is false (CTR-002 says this for its three booleans, CTR-005 does not); what an empty `appliesTo` means.
Also, the shape of a Requirement's input is in no promise. `schemas/README.md` lists it as a choice "of the schemas alone" (`schema`, and optionally `default` and `optional`), yet OBJ-019, SCH-019 and ACC-029 ("each a list that is empty where it is left out", which is `default: []` in `bound.v1`) already rely on a Requirement input's `default`.
Smallest fix: in CTR-004 and CTR-005 say "may hold" and name the required keys; add one sentence each for an absent `schema`, `offers`, `requires` and `narrows`; add the shape of a Requirement's input to CTR-005 and to the `input` row.

**S3. A Type contract may declare `runtimes`, but only a template's runtimes can be used.**
TPL-006: "A template or a Type contract declares a runtime." CTR-004 lists `runtimes`. GRA-017: "`runtime`, the name of a runtime the template declares ... a runtime name the template does not declare, is rejected" (same wording in the `step` row). Nothing says an instance has its type's runtimes, whether a template's step may name one, or what happens when both declare one name. The valid example declares `watcher` twice with the same operations, in `local-directory.v1` and in the template `project-directory`, which is what an author is driven to.
Smallest fix: needs a decision. Either a template's instances have the runtimes of its type as well as its own (then GRA-017 says "the template or its type declares", and a name declared by both is rejected or must be equal), or a Type's `runtimes` is removed.

**S4. INS-011 does not say whether a left-out `arguments` map is "present".**
INS-011 rejects "a destination whose parent is not present in the template", and INS-024 had to be added to make an unwritten `config` "present and empty". For `arguments` the nearest text is OBJ-014 ("exactly as one that writes an empty map") and NAM-019 ("then means no arguments"), neither of which speaks of presence. So `to: [/requirements/rate/arguments/limit]` beside `rate: {requirement: ...}` with no `arguments`, or `/conduits/checkout/arguments/limit` beside a declared Conduit with no `arguments`, is undetermined. The third document of `examples/invalid-by-rule/SCH-019-input-default-fails-its-schema.yaml` is exactly this case; its header says it is rejected for SCH-019, and it may also break INS-011.
Smallest fix: needs a decision. Either extend INS-024's rule to a left-out `arguments` of a nested instantiation and of an applied requirement, or say in INS-011 that such a destination is rejected. Then give that example document an `arguments` map so that it breaks one rule only.

**S5. A composite Conduit template may take an input whose destination is `/operations`.**
`conduit-template-spec.v1` uses one `reaches` pattern for all three branches, so the composite branch accepts `to: [/operations]`. INS-012 allows "in a Conduit template, `operations` as a whole" without excluding a composite. INS-011 does not reject it, because the parent of `/operations` is the `spec` itself. Instantiating would put `operations` beside `components`, which CON-016 forbids ("either `capability`, `operations` and `requirements`, or `components`").
Smallest fix: in INS-012 say "in a Conduit template that is not composed, `operations` as a whole; in a composite, the `arguments` of a component", and split the pattern in the schema to match.

**S6. `inspect.v1` returns `packages` as an object; OBJ-004 calls it a list; an unpublished package has no key.**
OBJ-004: "the list of packages the engine keeps for a template or an instance". `inspect.v1`: `packages: {schema: {type: object}}`. If the object is keyed by address as a manifest's `packages` is, the unpublished package of NAM-009, which "has no address", has no key, and the core package has no digest. The key name `packages` that OBJ-004 rejects in a submitted document is also not named in OBJ-004 (`id`, `revision` and `status` are).
Smallest fix: name the key in OBJ-004, and state the shape once (a promise, or the `inspect.v1` schema): for example a list of `{address, version, digest}` with `address` and `version` absent for the unpublished package and `digest` absent for the core package. Needs a decision on the shape.

**S7. The keys of `_meta`: the names agree everywhere; two gaps remain.**
Every file agrees on these 15 keys under `io.github.shrav-k.conduit/`, on where each appears and on what it holds:

| Key | Appears on | Holds | Stated in |
| --- | --- | --- | --- |
| `interfaceVersion` | engine to implementation, request | `v0` | IFC-006, IFC-008, definitions |
| `instance` | same | the instance's ID | IFC-006 |
| `operation` | same | qualified reference, or `#name` with `package` | IFC-006 |
| `invocation` | same | the engine's ID for the invocation | IFC-006, IFC-020 |
| `caller` | same | the caller's ID | IFC-006, IFC-020, IFC-030 |
| `restrictions` | same | list of `{requirement, arguments}` | IFC-006, IFC-018, ACC-017 |
| `operationIdentity` | consumer to engine, and engine to implementation, request | the identity | IFC-006, IFC-023 |
| `conduit` | engine to implementation, request | the Conduit's ID | IFC-006, ACC-016, IFC-030 |
| `package` | same | digest of an unpublished package | IFC-006, core README |
| `serving` | instance to engine, onward request | the served invocation's ID | IFC-020, IFC-021, IFC-028, ACC-016, ACC-017, ACC-034, IMP-008 |
| `outcome` | result, both directions | `succeeded`, `failed`, `unknown` | IFC-009, IFC-025 |
| `reason` | result | `refused` or `unavailable` from an implementation; all four to a consumer | IFC-026, IFC-025 |
| `restriction` | result, beside a reason | the item `{requirement, arguments}` | IFC-026, IFC-025, ACC-008, ACC-014, ACC-015 |
| `state` | engine to consumer, result | `inProgress` | IFC-025, OPS-009 |
| `withheld` | engine to consumer, result | `true` | IFC-025, ACC-034, definitions |

Gaps:
- IFC-029: "A request or a result carries the keys of one version only, which its interface version names." No result carries `interfaceVersion` (IFC-009, IFC-025), and no promise says that a consumer's request to the engine carries it (IFC-020 and IFC-023 name only `serving` and `operationIdentity`). The definitions row lists `interfaceVersion` and `serving` together "On a request" without saying that the first is sent by the engine and the second by an instance. Smallest fix: say in IFC-029 how the version of a result and of a consumer's request is known (by the prefix alone, or by requiring the key), and split the row by direction.
- The type of `operationIdentity` in `_meta` is stated nowhere; the only hint is the input `operationIdentity: {type: string, minLength: 1}` of `retrieve-result.v1`. Smallest fix: say in IFC-023 that it is a non-empty string.

**S8. What `schemas/README.md` says of the split between schema and engine: omissions and two rules the schemas enforce beyond the promises.**
- MAN-006: "`fetch` ... is a URI reference". `manifest.v1` and `install.v1` accept any non-empty string, and the table "What a schema cannot express" has no row for it (the profile has no `uri-reference` format). Add a row.
- Not in the table although only the engine can check them: CTR-003 (a capability outside the core package that lists a reserved operation; it is in the header of `contracts.yaml` only), PKG-017 (the stated address differs from the one listed or fetched under), OBJ-024 (one version's `apiVersion` with another version's core address). Add them.
- The row "Other uniqueness across documents ... PKG-012" is marked as not enforced, but `manifest.v1` gives `exports` `uniqueItems: true`, which does reject a name written twice. Say so.
- `implementation-item.v1` rejects `tools` on an item that is not `plain: true`. PKG-020 states only the converse (a plain item gives `tools`), and MAN-009 says "where they are given, `plain`, `tools`, `supports` and `placement`". Add to PKG-020: "An item that is not plain states no `tools`."
- OBJ-022 also names the members of a Type's `placement` and of a Requirement's `inputs` and `constituents`; the first row of the table lists only "an input, a step, a declared Conduit, an entry, a runtime, a requirement or a component".

**S9. NAM-001 accepts `.` and `..` as path segments of an address.**
NAM-001 allows a path segment made of "lowercase letters, digits, `-`, `_` and `.`", and `address.v1` accepts `example.org/..` and `example.org/.` (checked against the pattern). MAN-014 rejects a `..` segment in an archive member, so the omission here looks unintended; an engine that stores packages by address (DL7) is exposed to it.
Smallest fix: needs a decision. Either NAM-001 excludes segments that are `.` or `..` (or requires a segment to start with a letter or digit), or says they are ordinary names.

**S10. NAM-005 has nothing to detect, and its example is rejected by NAM-001's character rule.**
NAM-005: "A reference carries a version of the package. The document is rejected." No syntax for carrying a version exists. `NAM-005-reference-with-a-version.yaml` writes `example.org/files@0.3.0#local-directory.v1`, which the schema rejects only because `@` is not a path character; `example.org/files-0.3.0#x` and `example.org/files/v2#x` are valid addresses and pass.
Smallest fix: restate NAM-005 as what it is ("a qualified reference has no part that selects a release; the address and the manifest decide"), or name the forms it rejects (`@`), and rename the example for NAM-001 if the first.

**S11. Smaller points.**
- OBJ-004 and the `status` row say a submitted document may not carry `status`; inside `config` the key is the author's (OBJ-019), so `config: {status: ...}` is accepted. This is consistent, and worth one sentence in OBJ-004.
- `schemas/README.md`: "Every object these schemas describe is closed ... `envelope.v1` is the one object schema that is not closed." The helper `$defs/reaches` in the three `spec` schemas is also open by design. Harmless.

## Core package

**C1. "`pool-management.v1`, which every Pool offers" is not made true by any rule.**
OPS-019 (the quoted words), the `operation` row ("a capability that `management`, `access` or a Pool offers") and `capabilities.yaml` ("What every Pool offers") assume it. A Pool offers what its type's `offers` lists (CTR-004, CON-021), and neither CTR-004 nor `type-spec.v1` makes a Type of family `Pool` list `pool-management.v1`. A Pool type of another package that omits it yields Pools to which no Conduit with that capability can be instantiated (CON-021), so under ACC-033 its Resources can never be registered into it, set up, retired or inspected.
Needs a decision: (a) every Pool offers `pool-management.v1` whatever its type lists, stated in CTR-004 or CON-021; (b) a Type of family `Pool` that does not list it is rejected when installed; or (c) reword OPS-019 to "which the core Pool types offer" and accept unusable Pools.

**C2. `register-replacement.v1` covers "a component of a composite Conduit", which LIF-009's text cannot.**
The contract's description: "a declared Conduit, the Resource of a Pool's entry or a component of a composite Conduit". LIF-009: "A new instance is instantiated from the nested instantiation as the instance it belongs to holds it in its resolved definition." A component Conduit made by an instance document from a composite template belongs to no instance: CON-003 says "no instance represents the composition itself". LIF-008's examples are only "a declared Conduit or the Resource of a Pool's entry". The one alternative, registering the composite again, clashes under OBJ-002 with the names of the retired component Conduits until they are unregistered.
Needs a decision: either the engine keeps the composition's nested instantiations for a component Conduit (say where), or a component of a composite instantiated on its own cannot be replaced and the description drops it.

**C3. `core/README.md` and `core/types.yaml` cite a definition that does not exist.**
README: "The two facts the definitions name are `host` and `localPaths`". `types.yaml`: "`host` and `localPaths` are the two facts the definitions name". `definitions.md` names only `host` (rows `host` and `location fact`); `localPaths` occurs nowhere in it or in any promise. It is fixed only by the schemas of `location.v1`, the template `location` and `set-facts.v1`.
Smallest fix: add `localPaths` to the `location fact` row with its meaning, or reword both sentences to "the two facts the core package's schemas name".

**C4. A bound cannot name an operation of an unpublished package, and a policy cannot be a requirement of one.**
`bound.v1`: `operations` has `items: {$ref: ...#qualified-reference.v1}`. `apply-policy.v1`: `requirement` has the schema `qualified-reference.v1`. A contract of an unpublished package "has no address and so no qualified reference" (IFC-006). ACC-006 requires that the bound "names every operation the new Conduit lists", so only an actor whose Conduit to `access` has no bound can open a Conduit that lists an operation of a project's own contract; IFC-018 nevertheless provides the `#name` form for "a requirement of an unpublished package" among the restrictions, policies included. This is a different consequence from the known point about the single `package` key.
Needs a decision: accept the limit and state it (in ACC-029 and ACC-031), or give both inputs a form that can name such a contract (which needs the package's digest beside the name).

**C5. `register-replacement.v1` declares no `diagnostics`.**
LIF-009: "The request is a registration". OBJ-023: a rejected registration carries its diagnostics "in the `diagnostics` of its `structuredContent`". INS-022 makes every check again when the replacement is instantiated, so it can be rejected. `core/README.md` lists only `register.v1`, `replace.v1` and `install.v1` as declaring the output.
Smallest fix: add the output `diagnostics` to `register-replacement.v1` and to that list.

**C6. The naming table of `core/README.md` does not fit two templates.**
"Template | The name of the first-start object made from it, except `location`". `root-management` and `root-access` are not the names of first-start objects: the two Conduits are nested instances and are named "with its template's name, followed by a hyphen and a generated suffix" (OBJ-003), so they are `root-management-<suffix>` and `root-access-<suffix>`.
Smallest fix: add "and the two Conduit templates, named for their consumer and target".

What I checked and found right is under Clean.

## Guesses

What I could not determine from `spec/`. Each is something a first implementation must decide.

**G1. The stored and wire form of a resolved reference.** OBJ-009: "The resolved instance holds an ID for each reference"; OBJ-021: a value governed by a reference schema "is resolved to an ID"; POOL-006: "The Resource holds the ID of that Resource where the reference was". Not stated: whether the ID replaces the written value inside `config` and `arguments` (an `id.v1` string where the schema says `name.v1` or `{kind, name}`, so the stored configuration no longer satisfies the type's `schema` that INS-014 checked); what `{source: config, pointer: /cache}` yields; what an implementation receives in `arguments` for an input governed by a reference schema; what `settings: {/defaultLocation: ...}` returns; what `declared` shows; and what the `arguments` of a `bound.v1` item hold in `io.github.shrav-k.conduit/restrictions`. Options: keep the written reference and hold the ID beside it; replace it with the ID; replace it with `{kind, id}`.

**G2. Which input of a control governs which setting.** REV-006: "The setting that input governs is left as stored." REV-009: "the settings the control addressed are reported as unknown". A control's contract has `inputs` and `changes` (a list of pointers) and nothing that relates them; in the three controls that exist the names differ (`location` and `/defaultLocation`, `limit` and `/readLimit`, `facts` and `/facts`). Without the relation an engine cannot tell which settings a control with some inputs left out "addressed". Options: a key on the input that names its pointer; `changes` as a map from input name to pointer; or say that "addressed" means every pointer of `changes`.

**G3. A composed requirement on the wire and in `supports`.** CTR-010 says how a constituent's arguments are bound. Not stated: whether CON-022 needs the target's implementation to list the composition, its constituents or both in `supports` (the example `files-server` lists `#rate-limit.v1` and not `#review-limit.v1`); whether an item of `restrictions` names the composition or one item per constituent; which of them `io.github.shrav-k.conduit/restriction` names; and whether `narrows` and `appliesTo` of a composition must agree with its constituents'.

**G4. Replacing a template that uses the project's own contracts.** `replace.v1` takes `template`, `revision` and `manifest`, and no contract documents. A replacing template that writes `#model-name.v1` has nothing to resolve it against unless the engine reuses the unpublished package the old template holds (NAM-009), which no promise says. The same question arises for an instance document, registered later, of a template that held an unpublished package. Tied to T2.

**G5. The inspection result, beyond what `core/README.md` already lists as unsettled.** That page admits the shapes of `declared`, of a diagnostic and of where ACC-027 and ACC-038 report. Also unstated: the keys of a record (REC-002 lists nine things it holds and names no key); the shape of an observation with its time and origin (OBS-002, OBS-003); where conditions appear and in what form (LIF-011); where "the word unknown against each setting" goes (REV-009); how the status "names the setup and its outcome" (LIF-004), "what retired it" (LIF-005), which event closed a Conduit (CON-008) and a closed Conduit on its consumer (CON-011); the members that list "the actors or targets each applies to" beside a policy's `id` (ACC-031); where an instance's `from` is reported (FST-005); how a gap in records is shown (REC-006).

**G6. What `optional: true` means on a runtime and on a graph's output.** OBJ-015 and TPL-006 allow it on a runtime and no promise gives it an effect (GRA-003 speaks only of steps). GRA-021 lets a graph "mark an output the contract makes optional as `optional: true`" while `graph.v1` still requires `value`; what the mark changes is not said.

**G7. Tool names on the implementation's side.** IFC-001: "The tool's `name` is the operation's export name". Two operations of different packages may share an export name (CTR-007 says so). One implementation that lists both in `implements` would have two tools of one name; CTR-007 and IFC-014 cover capabilities and consumers only. Also unstated: the `inputSchema` the engine presents for a tool to a consumer, given that an operation's input schemas may hold a `$ref` that is a qualified reference or `#name`, which a Model Context Protocol client cannot follow.

**G8. What `actors` and `targets` of `apply-policy.v1` each do.** The `policy` row: it "limits every request of which one of them is the consumer or the target". Not stated: whether an instance listed under `actors` is limited only as a consumer and one under `targets` only as a target; how a Resource that makes onward requests is named as a consumer (it is not an actor, and `actor-reference.v1` rejects it); and whether a policy whose requirement `obliges` only one end changes this.

**G9. The owner of a location's document.** PLC-008: a location is "owned by the system Pool". `instance.v1` requires every Resource document to write `owner`. Not stated: whether a document from the template `location` must write `owner: system`, and what one that names another Pool reports.

**G10. Generated names.** OBJ-003: "a hyphen and a generated suffix. The template's part is shortened where the whole name would otherwise be longer". The suffix's length and alphabet and the amount of shortening are left to the engine, but `implementation-defined.yaml` has no item for it, and the `implementation-defined` row of the definitions says that list holds "every such point".

**G11. IFC-016's "reference".** "The call returns a reference among its outputs, and the result is retrieved later". Which output, in what form, and how it relates to the operation identity and to `retrieve-result.v1` (which takes `operationIdentity` and `target`, not a reference) is not stated.

## Standards

**St1. The Model Context Protocol, revision 2026-07-28, is relied on for specific features with no section cited.** IFC-003: the revision is named "for each request in `io.modelcontextprotocol/protocolVersion` in the request's `_meta`", and a mismatch is "the protocol's JSON-RPC error for an unsupported version" (no code, while IFC-010 gives -32602 for the other error). IFC-010: `resultType` with the values `complete` and `input_required`. IFC-012: tool names "use only ... letters, digits, `_`, `-` and `.`" and "The protocol limits a tool name to 128 characters". IFC-016: the extension `io.modelcontextprotocol/tasks`. `schema-profile.md` lists only "Model Context Protocol, revision 2026-07-28". I could not check these from `spec/`. Smallest fix: cite the section of that revision for each, and give the error code in IFC-003.

**St2. "tar, as POSIX (IEEE Std 1003.1) defines it".** No edition and no format: the ustar interchange format and the pax format differ (long names, extended headers), and common tools write GNU extensions. Needs a decision: name the edition and say which of ustar and pax an engine must read and whether extended headers are accepted.

**St3. ECMA-262.** SCH-016 and the profile name no edition and do not say whether a `pattern` is matched in Unicode mode. JSON Schema counts `maxLength` in characters, and without Unicode mode `.` and a negated class match UTF-16 code units, so `pointer.v1` and `destination.v1` can differ between engines on characters outside the Basic Multilingual Plane. Smallest fix: name the edition and say "with the `u` flag".

**St4. "JSON Schema 2020-12, core and validation".** In those two documents `format` is an annotation unless the format-assertion vocabulary is used. SCH-001 requires "format checking on", and MAN-010 depends on it (`url: {type: string, format: uri}` is the only thing that rejects `MAN-010-manifest-url-not-a-uri.yaml`). Smallest fix: add "with the format-assertion vocabulary" to the row. Also `uuid`: the profile cites RFC 9562 where JSON Schema 2020-12 cites RFC 4122; SCH-004 makes the profile's citation govern, which is enough, and worth one clause.

**St5. Smaller.** RFC 1123 is cited for label syntax; the limit on the whole host name is not taken over, and `address.v1` has no `maxLength`. RFC 5890: an `xn--` label is accepted by shape and not checked to be a valid A-label. I-JSON is cited for two of its rules; RFC 8785, used by SUB-008, also needs strings without unpaired surrogates, which no promise requires of a document. None changes a result in the examples.

**St6. No schema under `spec/schemas/` or `spec/core/` breaks `schema-profile.md`.** I checked the keywords, the three forms of `$ref`, the formats and the placement of `default` by reading, and the tool reports the same.

## Clean

Checked and found consistent across promises, schemas, examples and the core package.

- The envelope and `metadata` (OBJ-012, OBJ-017, OBJ-018, OBJ-020, CTR-001): keys, which are required, and the name patterns `name.v1`, `export-name.v1`, `contract-name.v1`.
- Template `spec` for Interface, Agent and Resource, for Pool, and for Conduit single and composite (TPL-001, TPL-002, CON-016 to CON-020), apart from S5; inputs (INS-021) and the closed list of destinations (INS-012) against the three `reaches` patterns; the `implementation` map (TPL-003); a runtime's keys (TPL-006); a template's `controls` (TPL-010).
- Graphs, steps, value bindings, the five sources and which keys each takes, `outputs` (GRA-015, GRA-017 to GRA-023) against `graph.v1`, `lifecycle-graph.v1`, `step.v1`, `value-binding.v1`, `value-reference.v1`; the constituent's binding (CTR-010) against `requirement-spec.v1`.
- Instance documents of the five families and the three nested instantiations (OBJ-013, OBJ-014, POOL-001, POOL-012, PLC-006, CON-027), apart from S1.
- Operation contracts (CTR-002, CTR-006, CTR-009): inputs and outputs, `default` only beside `schema` and never on an output or a handle, the three marks, `implementation` as one selection, `changes`. Type's `restoresRuntimes` and `records`, Requirement's `obliges`, `schema` and `constituents`: each is in its promise, its schema and one valid example.
- The manifest and an implementation item (MAN-003 to MAN-012, PKG-017 to PKG-020), including the core package's own form, apart from S8.
- References: the eleven reference schemas match OBJ-006, OBJ-021, POOL-005, POOL-014 and the `reference schema` row, and a bare name is accepted exactly where one kind is.
- The valid examples: I walked each document of the project, the package and first start against the promises it touches (defaults of implementations under TPL-004, CON-017, CON-021, CON-022, CON-028, PLC-007, INS-010 to INS-014, NAM-027, IFC-014) and found none that breaks one.
- The 55 invalid examples: each is rejected for the reason its file name gives, except S10. The 11 sets under `invalid-by-rule`: each is rejected by its named rule, with the remark in S4.
- The core package: every export of the manifest is a document and every document is exported; none is private. Each operation a promise names exists with the inputs and outputs the promise implies (`purge.v1` `before`; `reconcile.v1` `revision` and `settings`; `retrieve-result.v1` `operationIdentity`, `target`, `state`, `withheld`, `outputs`; `set-default-location.v1` `location`; `set-facts.v1` `facts`; `inspect.v1` `id`, `revision`, `packages`, `declared`, `status`, `records`; `retire-with-resources.v1` `remaining`; `apply-policy.v1` returns an ID and `remove-policy.v1` takes it). The capabilities hold every operation ACC-033, ACC-006, ACC-012, ACC-031, LIF-009, LIF-021, PLC-008 and PLC-016 route to them. Each control is in its type's `controls` and in a capability the type offers, and its `changes` is a place the type's schema describes. The first-start templates produce exactly the five objects of FST-001 and the two Conduits of FST-002, and `root-access` applies no bound. Export names satisfy NAM-004, CTR-001 and NAM-027, and template names hold no dot (OBJ-003).
- Terms from the list that I found used in one sense, as defined: install, exists (for objects), attempt, invocation, the invocation's state, repeat, origin, item (apart from T7), restore, target, permitted (with "authorized against", which the row covers), restriction, filter, requirement, applied requirement (apart from T15), obliges (known point only), mediated, isolation plugin (apart from T13), facts (apart from C3), default location, unpublished package, submission, contract, export, re-export, handle, runtime, control, settings, condition, status, shutdown, model work (apart from T9).
- The three words the definitions say are not used: `grant` appears only in a comment that says there is none, `declaration` nowhere, and `create` only in an example's `effect` text.

## Judgement

An implementer would first hit S1, G1 and T1. The very first instance document needs the forms of `from`, which only a schema gives and which the definitions contradict on `#name`. The first stored instance needs the form of a resolved reference, on which the type check, graphs, controls and the wire all depend. The first `setup` graph needs to know what operation it implements when `set-up.v1` already has the engine as its implementation. After those come the control's input-to-setting relation (G2) and what a project is for a `register.v1` request (T2). This round is not yet close to clean on vocabulary, though the remaining drift is mostly wording that needs no ruling. On shapes it is close: the schemas, the examples and the core package agree with the promises nearly everywhere, and what remains is about ten places where a promise is silent and a schema or a description decided for it, six of which need an owner's decision (S1, S3, S4, S6, C1, C2, with S9 and C4 as smaller ones).
