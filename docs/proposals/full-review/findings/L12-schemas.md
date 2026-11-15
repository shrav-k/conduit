# L12-schemas: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Reviewed at commit 2eb8f258 (2026-11-12).

**Agreement.** The schemas and the promises agree closely. I found no finding I would mark unsafe. Every place where a schema is looser than a promise has an engine rule behind it: the resolved form of a reference (OBJ-009), an entry reference outside a Pool (POOL-005), `reservedToRoot` (CTR-009), the core address (NAM-021), and a Fleet template (FLEET-001).

The real disagreements are few:
- **F2:** the form of `owner` differs between two promises, the framed schema, a valid example and the sectioned schema.
- **F3:** where a `default` may stand and be filled is decided by a comment in `schema.v1`.
- **F7:** a handful of rules exist in a schema alone.

The larger faults are structural:
- **F1:** which schema and keyword a rejection reports is decided by a heuristic in code, and the portable vectors pin its result.
- **F4:** a schema rejection's diagnostic is described twice, and the two descriptions differ.

**Are schemas and promises two sources of truth that can drift?** Yes, and there are more than two. A shape is stated in promise prose, in a schema, in the README's tables, in the Python checker's rules and in the Rust loader.

SUB-005 gives the schema's verdict force, and the vectors pin keyword pointers inside the schemas. So the schema is the operative source, while the README says the promise is. Nothing mechanical links the two. The examples (74 schema-rejected, 41 rule-rejected, all consistent when I ran them) catch a wrong verdict only where an example exists.

**Least duplication.** State each shape rule once, in its schema. Keep promises for meaning and for what a schema cannot say. Replace the head comments and the README table with a machine-read field on each promise naming the contracts that encode it, checked against the examples. Inside the schema set, remove the hand copies (F5).

**Is the profile one an ordinary validator implements?** Its core is: `schema.v1` is plain JSON Schema 2020-12 and any validator can apply it. Around that core the profile needs four custom parts, and the repository builds each of them twice, in `tools/check_schemas.py` and in the Rust loader:
- **References.** A bundler is needed for the two non-standard `$ref` forms. A standard validator reads `#name` as an anchor, which the profile excludes.
- **Patterns.** `pattern` is matched as ECMA-262 in Unicode mode behind a token whitelist. The Rust validator could not be re-pointed for `patternProperties`, so the core re-implements it.
- **Formats.** The grammars for `uuid` and `duration` are replaced. The Python checker's `date-time` library rejects the leap second `23:59:60Z`, so SCH-004's cited grammar is not what stock libraries check.
- **Defaults.** Defaults are filled before validation, with SCH-009's conflict check and SCH-019 on top.

Each deviation is recorded as a decision (settlements 1-04, 1-08, 1-13, 1-17 and 2-03), so I report it here and not as a finding. SCH-001's test, "a conforming validator says it is", holds only after those four steps.

**What the reader read.** **Read completely**
- `spec/schemas/README.md` and all nine schema files (52 contracts, recounted).
- `spec/core/references.yaml` and `spec/schema-profile.md`.
- The promise files `objects`, `instantiation`, `templates`, `contracts`, `manifest`, `sections`, `schemas`, `fleets` and `first-start`.

**Read in part**
- The promises the schemas cite in `naming` (NAM-001 to 008, 018 to 022, 027, 028), `conduits` (CON-001, 016 to 020, 027), `pools` (POOL-005 to 014), `packages` (PKG-019, 020), `submissions` (SUB-005, SUB-010), `placement` (PLC-007) and `pending` (PND-013).
- IFC-011, IFC-033, IFC-043, IFC-044 and IFC-049.
- `tools/check_schemas.py` (bundling, pattern, format, routing and tokenizer parts) and the Rust loader's `schemas.rs` and `formats.rs`, as evidence only.
- The known-open lists: `spec/README.md` limits, the `docs/AUDIT.md` list, the cold readers' list, the full-review queue, IN_FORCE topic 5, and the first-build settlements register.

**Examples**
- Listed all of them: 14 valid files in 6 directories, 74 invalid, 41 invalid-by-rule.
- Read 8 invalid, 9 invalid-by-rule and 4 valid files.
- Checked that every invalid-by-rule file names an existing promise and carries an opening statement; two do not carry one.

**Run, writing nothing**
- An in-memory JSON Schema 2020-12 validator (`/usr/bin/python3`, jsonschema 4.20.0, PyYAML 6.0.1), bundling the 77 Schema contracts of `spec/schemas` and `spec/core` as the README describes.
- All 115 invalid and invalid-by-rule examples went through it. Every by-rule document is valid. Every invalid one is rejected, apart from the four that depend on file reading or the pattern dialect (`OBJ-001-name-ending-in-a-line-feed`, `SCH-006`, `SCH-012`, `SCH-021`) and the one that is not UTF-8.
- About 190 probe documents went through it as well, and a mechanical comparison of the duplicated template bodies.

**Not checked**
- The transcription of server.json 2025-12-11 against upstream (no network).
- The Rust validator's format behavior (nothing built or run).
- The schemas of results, records and operations in `spec/core` beyond the reference schemas.
- The translation rules of the short form beyond what `sectioned.v1` states; `sections.yaml` has no vectors, which is already recorded.
- Whether the tool's by-rule checks match each promise in full.


## FR-132

**No promise says which schema and which failing keyword a schema rejection reports when a union fails; the rule lives only in code, and vectors pin its result**

L12/F1 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** SUB-005, OBJ-020, INS-001, SCT-014

**Where.** spec/promises/submissions.yaml:67

**Quote.** "A document, or the manifest, that a Schema contract rejects is rejected under this promise, whichever promise that contract encodes: the diagnostic names the place in the document by a JSON Pointer, in a logical location, and its message names the Schema contract."

**Situation.** SUB-005 goes on to require "the failing keyword's location", in the singular. Most document schemas are unions: document.v1 (spec/schemas/document.yaml:25-30) is allOf of envelope.v1 and an anyOf of template.v1, instance.v1 and contract.v1, each itself an anyOf of 4, 6 and 5 closed branches; manifest.v1, every reference schema, selection.v1, template-reference.v1 and the sectioned inline-or-referenced pairs are unions too. A document that breaks one rule fails every branch, so a JSON Schema 2020-12 validator has many failing keywords, the outermost being the anyOf itself. Builder A follows spec/schemas/README.md:101 ("A framed document, with `kind`" is checked with `document.v1`) and reports the anyOf of document.v1 at the document root. Builder B does what the first build does: it re-validates against the narrower template.v1, instance.v1 or contract.v1 and descends into "the one whose `kind` it has, then one whose type it has, then the one it breaks in the fewest places" (engine/crates/loader/src/schemas.rs:441-443, copied from tools/check_schemas.py:729-733). That heuristic is in no promise. The portable vectors pin its outcome: conformance/language/loader/schema-attribution/case.yaml:53-57 expects contract name.v1, pointer /pattern, root_schema template.v1 for an Interface template named Console. An independent builder working from spec/ alone fails that vector. For unions with no `kind` to choose by (a `type` written `t.v1`, a bare `consumer`), "fewest places" picks one branch's pattern arbitrarily, where the honest location is the anyOf.

**Counterargument.** SCT-015 compares only ruleId and level across engines, and settlement 1-14 of the first build fixed SUB-005 with keywordLocation deliberately. INS-001 and OBJ-020 let a builder infer the routing by `kind` and `from`. The vectors could be read as testing the first build only.

**Least change.** Add to SUB-005's observable: "A framed document is judged against the one closed schema that its `kind` and the presence of `from` select: `contract.v1`, `instance.v1` or `template.v1`, and within it the branch whose `kind` the document has. Where a union has no such branch, the keyword reported is the union's own." Better, rewrite template.v1, instance.v1 and contract.v1 so each branch is chosen by `if: {properties: {kind: {const: ...}}}` and `then`, which the profile permits; an ordinary validator then has one failing branch and no heuristic is needed.

**Precedent.** Kubernetes picks the one schema an object's apiVersion and kind name before validating; OpenAPI 3 calls the same thing a discriminator; JSON Schema's own guidance is if/then on the discriminating property.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (rulings 37 and 46 point 4); it states what the first build and its vectors already do.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/submissions.yaml:67-70; the singular follows at :70-72 ("Its SARIF result carries the failing keyword's location in `properties`"). The union is at spec/schemas/document.yaml:25-30, and spec/schemas/README.md:101 names `document.v1` for a framed document. I ran the vector's document (kind Interface, name `Console`) against the bundled document.v1 with a stock 2020-12 validator: one top-level error, keyword `anyOf` at the document root, keyword path allOf/1/anyOf, three failing branches beneath. The first build instead re-validates against the narrower schema (engine/crates/loader/src/schemas.rs:203-209, narrow_role at :390) and follows the heuristic at :441-443 ("the one whose `kind` it has, then one whose type it has, then the one it breaks in the fewest places"), as tools/check_schemas.py:729-747 and :761-793 do. The vector pins the result: conformance/language/loader/schema-attribution/case.yaml:40 `/metadata/name`, :55-57 contract `name.v1`, pointer `/pattern`, root_schema `template.v1`. Nothing in spec/ decides it: `keywordLocation` occurs only at submissions.yaml:71-72, spec/core/results.yaml:76 and spec/promises/sections.yaml:260. DL9 (spec/implementation-defined.yaml:72-75, "The order in which several failed checks that give the same reason are reported") covers order, not which keyword one diagnostic names; engine/CONFORMANCE.md:28 nonetheless files the first build's choice under DL9. Not recorded: review-list item 5 (docs/proposals/first-build/to-review-with-the-owner.md:18) and settlements 1-01 and 1-14 (docs/proposals/first-build/README.md:57, :70) decide that SUB-005 is cited with a keyword location, not which keyword under a union.

**Verifier's corrections.** Class gap stands; the reviewer's basis line calls it an ambiguity, and either way it is a build finding the coordinator may settle (ruling 46 point 4). Not unsafe: the verdict accept or reject is the same for both builders. Two precisions. (1) The vector is marked `"profile": "first-build"` (case.yaml:10) and conformance/README.md:3 says "Not binding", so an independent builder fails it only because builders are handed the vectors (IN_FORCE.md:283). (2) A third fix exists: state under DL9 that the keyword reported for a failed union is the engine's choice and drop the location facts from the portable expectation. Of the reviewer's two fixes, (b), `if`/`then` on `kind`, is the one that removes the heuristic; it moves keyword pointers that other vectors pin.


## FR-133

**The form of `owner`: two promises say a bare name, the framed schema and a valid example accept `{kind, name}`, and the sectioned schema rejects it**

L12/F2 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** OBJ-013, FLEET-002, OBJ-006, SCT-006, SCT-011

**Where.** spec/promises/objects.yaml:201

**Quote.** "A Resource's owner is a Pool, and an Agent's owner is a Fleet, each written as a bare name because its kind is fixed (OBJ-006)."

**Situation.** FLEET-002 repeats it: "`owner` is a bare name whose kind is fixed by `fleet-reference.v1`" (spec/promises/fleets.yaml:35-36). OBJ-006 says only that the name alone "may be written". Reading A: owner must be a bare name. Then instance.v1 accepts what the promise forbids: it gives `owner` the schemas fleet-reference.v1 and pool-reference.v1 (spec/schemas/instances.yaml:99-100, 118-119), which also take `{kind: Fleet, name: f}`, and the valid example spec/schemas/examples/valid/project/instances.yaml:116 writes `owner: {kind: Pool, name: team}`. Reading B: a bare name is allowed, not required. Then the sectioned schemas reject what the promise allows: sectioned-agent.v1 and sectioned-resource.v1 give `owner` the schema name.v1 alone (spec/schemas/sectioned.yaml:545-546, 593-594, 659-660, 696-697), although SCT-006 says "OBJ-013 governs standard arguments". I confirmed both verdicts with a validator: the framed Agent with `owner: {kind: Fleet, name: f}` is valid, the same owner in a sectioned `agents` member is rejected. A builder who writes the check from OBJ-013 rejects the repository's own valid example.

**Counterargument.** "Written as a bare name because its kind is fixed" can be read as describing the usual way, with OBJ-006 governing. The short form is allowed to be narrower than the canonical form.

**Least change.** In OBJ-013 and FLEET-002 write "each may be written as a bare name, because its kind is fixed (OBJ-006)", and give the sectioned `owner` the schemas `fleet-reference.v1` and `pool-reference.v1` that the framed form uses. SCT-011's "an explicitly written owner equal to X" then needs "in either form".

**Precedent.** OBJ-006's own rule and the core package's reference schemas, which every other kind-fixed reference (`placement`, `runtime`) already uses in both notations.

**Who decides.** coordinator. An ambiguity between a promise and two schemas; the framed form already accepts both forms, so only the short form gains a spelling (rulings 37, 46 point 4; 71 point 4 for the small addition).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/objects.yaml:201-203. spec/promises/fleets.yaml:35-36: "`owner` is a bare name whose kind is fixed by `fleet-reference.v1` (OBJ-006)". OBJ-006 at objects.yaml:87-88 says only "the name alone may be written and means that kind". fleet-reference.v1's own description (spec/core/references.yaml:386-389) reads "The name alone, kind and name, or the resolved kind and ID". Framed schema: spec/schemas/instances.yaml:99-100 and :118-119 give `owner` fleet-reference.v1 and pool-reference.v1. Valid example: spec/schemas/examples/valid/project/instances.yaml:116 `owner: {kind: Pool, name: team}`. Short form: spec/schemas/sectioned.yaml:545-546, 593-594, 659-660, 696-697 give `owner` name.v1 alone, while `placement` and `runtime` beside it take resource-reference.v1 (:547-550). SCT-006 at spec/promises/sections.yaml:110-112: "OBJ-013 governs standard arguments". Validator run on the bundled schemas: a framed Agent with `owner: {kind: Fleet, name: f}` is valid against document.v1; the same owner in a sectioned `agents` or `resources` member is rejected (`oneOf` at /agents/a); the bare name is accepted. No record of it in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md, the first-build logs or the review list; the batch note docs/proposals/spec-batch-1/areas/fleet.md:49 only repeats the promise's wording.

**Verifier's corrections.** Class, unsafe mark (none) and who decides stand. Two additions. (1) The short-form rejection surfaces as a `oneOf` failure at the definition, not at `owner`, so the author is not told which key is wrong (ties to F1 and ruling 52). (2) A fix with no added spelling exists: keep name.v1 in the short form and say in SCT-006 that `owner` there is a bare name; widening the short form is ruling 71 point 4 and must not move S1's counts before the rerun. Either way OBJ-013 and FLEET-002 need "may be written as a bare name", or the valid example must change.


## FR-134

**`schema.v1` accepts a `default` on a property under `contains`, `propertyNames`, `prefixItems`, `additionalProperties` and `patternProperties`; SCH-008 names three routes and SCH-009 does not say which are filled**

L12/F3 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** SCH-008, SCH-009, SCH-010, CTR-008

**Where.** spec/schemas/schema.yaml:23

**Quote.** "Every other applicator, `items`, `prefixItems`, `contains`, `additionalProperties`, `patternProperties`, `propertyNames` and a member of `$defs`, holds a `schema`, whose own `properties` are again places where a `default` is accepted."

**Situation.** That sentence is a comment in a schema file and is the only place the rule is stated. SCH-008's `when` (spec/promises/schemas.yaml:101-104) names three routes to a property that may hold a default: "written directly, reached through `$ref`, or be the schema that `items` gives". I checked with a validator that schema.v1 accepts `{contains: {properties: {a: {default: 1}}}}` and the same under the other four keywords. SCH-009 then speaks only of a property absent from "a map that is present in the value" and of items of a list. Reading A (the first build, engine/crates/core/src/schemas.rs:595-667): fill under `properties`, `patternProperties`, `additionalProperties`, `items` and `prefixItems`; never under `contains` or `propertyNames`, where the default is accepted and silently unused. Reading B: SCH-008's list is complete, so only `properties` and `items` carry defaults and the others are rejected or unfilled. Reading C: fill wherever a schema applies to a present map, `contains` included, which makes every map item match `contains` and breaks `maxContains`. The three readings store different configurations and give different verdicts on `required`: `{}` under an `additionalProperties` schema that requires a defaulted property is valid for A and rejected for B. A filter's argument sent to an implementation differs the same way.

**Counterargument.** SCH-008's observable sentence ("accepted where the schema that states it is written as a member of `properties` and is not inside any of those eight keywords") supports schema.v1, and SCH-009's "a map that is present in the value" supports reading A for maps of maps. Such schemas are rare.

**Least change.** SCH-008: add `contains` and `propertyNames` to the keywords inside which no `default` is accepted (one applies to some items only, the other to names), and route both to `combined` in schema.v1. SCH-009: add "The maps that take defaults are those reached through `properties`, `patternProperties`, `additionalProperties`, `items` and `prefixItems`, and no others."

**Precedent.** Kubernetes structural schemas, which SCH-008 already cites: defaults apply through `properties`, `additionalProperties` and `items`, and nowhere under a conditional or quantifying keyword.

**Who decides.** coordinator. An ambiguity; the fix writes down what the first build does and rejects two placements that can never take effect (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/schemas/schema.yaml:23-25. Admission is decided by a promise, so the comment is not the only source: spec/promises/schemas.yaml:111-112, "A `default` is accepted where the schema that states it is written as a member of `properties` and is not inside any of those eight keywords." schema.v1 does exactly that (validator run: accepts a default on a property under `contains`, `propertyNames`, `additionalProperties`, `patternProperties`, `prefixItems` and `items`; rejects it under `allOf`). Filling is general in SCH-009: schemas.yaml:125-126 "a property that has a `default` in that schema is absent from a map that is present in the value"; :135-136 "This is one rule for every schema that governs a supplied value"; :142-143 "A map that is an item of a list is present when the list holds it: each such item takes the defaults of its own properties". That covers `additionalProperties`, `patternProperties`, `items` and `prefixItems`, so Reading B is not a sound reading. What is left undecided is `contains`: SCH-009's exclusions (schemas.yaml:145-148) name the eight keywords and not `contains`, and the first build does not fill there (engine/crates/core/src/schemas.rs:595-667 fills under `properties`, `patternProperties`, `additionalProperties`, `items` and `prefixItems` only), while the literal text would fill every item map. Not in any known-open list.

**Verifier's corrections.** Narrow the finding to one ambiguity in SCH-009: whether a default on a property under `contains` is filled (first build: no; literal text: every item map). Under `propertyNames` no map is ever present, so a default there is dead text, not an ambiguity. Drop Reading B and the claimed difference in `required` verdicts. Class ambiguity, not unsafe, coordinator. One sentence in SCH-009 settles it. The reviewer's other fix, rejecting both placements in SCH-008 and schema.v1, changes what is accepted, so it is a small change under ruling 71 point 4, not a fix that adds no behavior.


## FR-135

**A schema rejection is owed two different diagnostics: the encoded promise's ("naming the key") and SUB-005's (place, contract, keyword); only library prose bridges them**

L12/F4 · contradiction · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** SUB-005, OBJ-012, OBJ-013, OBJ-019, INS-012, CON-016, MAN-009, IFC-025

**Where.** spec/promises/objects.yaml:187

**Quote.** "A template document with any other top-level key is rejected with a diagnostic naming the key."

**Situation.** About 100 promises are encoded by a schema, and most state their own observable: OBJ-012, OBJ-013, OBJ-019 and MAN-004 a diagnostic "naming the key"; INS-012 one "naming the input and the key its destination reaches"; OBJ-001 one naming "the rule its name breaks". SUB-005 (spec/promises/submissions.yaml:67-72) says such a rejection is made "under this promise, whichever promise that contract encodes" and fixes its content as a JSON Pointer, the contract's name and the keyword location. The two do not coincide. For an unknown key the failing keyword is `additionalProperties` and the pointer the first build reports is the parent map; the key's name appears only in the validator library's sentence, passed through and cut at 200 characters (engine/crates/loader/src/schemas.rs:501-512), and conformance/README.md:90 says "Library prose is not a portable expectation". For INS-012 the caller receives a regular expression. For CON-016's anyOf, MAN-009's oneOf and PKG-020's `not`, the keyword says nothing an author or an agent can act on. One obligation is also split across rule IDs by what the profile can express: a Conduit template with no `operations` and no `inputs` reports SUB-005 (examples/invalid/CON-016-template-without-operations.yaml), one whose inputs fill something else reports CON-016 (examples/invalid-by-rule/CON-016-no-input-fills-operations.yaml). An engine may keep SUB-005 and break OBJ-012's observable, or keep both only by adding a message per rule beside the validator.

**Counterargument.** Settlements 1-01 and 1-14 chose SUB-005 for every schema rejection and rejected per-keyword promise annotations. A pointer plus a keyword location does let a tool find the rule. The examples' file names record which promise each rejection serves.

**Least change.** SUB-005: "The pointer names the member that the failing keyword rejects: for a key that a closed map does not define, that key. The message quotes the `description` of the nearest schema that states one." Then give the subschemas that carry a rule (the destination patterns, CON-016's anyOf, MAN-009's oneOf, PKG-020's if and then, CTR-004's contains) a `description` stating the rule and its promise ID; `description` is already in the profile.

**Precedent.** JSON Schema 2020-12's output format gives each failing unit its own instance location; Kubernetes gives a validation rule a `message` (x-kubernetes-validations) for the same reason; SARIF's `message.text` is where it goes.

**Who decides.** coordinator. A contradiction between two observables, and ruling 52 makes a terse diagnostic a finding; the fix adds message content only (rulings 37, 46 point 4; 71 point 4).

**Verifier.** weakened, medium. Quote verbatim at spec/promises/objects.yaml:187-188. It is not a contradiction: SUB-005 (spec/promises/submissions.yaml:67-72) fixes the rule cited and adds content (a pointer, the contract's name, the keyword location); nothing in it forbids the pointer or the message from naming the key, so one diagnostic can keep both promises. The core of the finding is recorded and owner-reviewed: docs/proposals/first-build/to-review-with-the-owner.md:18, item 5, "A rejection by a schema cites SUB-005, with the failing keyword's location, rather than the promise the schema encodes", answered at :34 "Agree on all"; settlements 1-01 and 1-14 at docs/proposals/first-build/README.md:57 and :70. The split of CON-016 between SUB-005 and CON-016 follows directly from that decision. What survives as a new situation: SUB-005 does not say whether the encoded promise's own observable still binds (OBJ-012 objects.yaml:187; OBJ-019 objects.yaml:334 "Resolving returns a diagnostic naming the key"; INS-012 spec/promises/instantiation.yaml:175; MAN-004 spec/promises/manifest.yaml:81-82). The first build meets it only through validator prose cut at 200 characters (engine/crates/loader/src/schemas.rs:501-504); the vector for an unlisted member pins keyword `additionalProperties` and no location (conformance/language/contracts/sign-in-logins/case.yaml:472-488); conformance/README.md:90 says "Library prose is not a portable expectation". Ruling 52 point 2 (docs/proposals/ruling-52/README.md:23) asks that a diagnostic say what happened and why, in full.

**Verifier's corrections.** Class: ambiguity (does SUB-005 replace or add to the encoded promise's observable), carrying a ruling-52 finding of terse diagnostics; not a contradiction. The reviewer's mark known-new-situation is right; cite review-list item 5 as the known part. Not unsafe. Coordinator. The fix is sound: `description` is in the profile (spec/schema-profile.md:75-76). Its first sentence, making the pointer name the rejected key, moves locations that vectors pin. It overlaps F1 (both are about what a SUB-005 diagnostic holds); settle the two in one change to SUB-005.


## FR-136

**The schema set repeats itself by hand: 19 copies of one name pattern, 13 of the reserved names, each template body written up to three times, and no check that the copies agree**

L12/F5 · needless-complexity · verifier: confirmed, low · decides: coordinator · new

**Promises.** SCT-005, SCT-006, SCT-013, INS-012, TPL-001

**Where.** spec/schemas/sectioned.yaml:27

**Quote.** "pattern: ^[a-z]\([a-z0-9]|-[a-z0-9])*-?$"

**Situation.** sectioned.yaml (961 lines, a quarter of the schema text) writes that pattern 19 times, the list system, local, management, access, root 13 times, and the list t, operator, manages 6 times, all inline. The `spec` of an Interface, Agent or Resource template is written in template-spec.v1, again in three members of sectioned-templates.v1, and again in the inline branch of sectioned-interface.v1, sectioned-agent.v1 and sectioned-resource.v1; the Pool and Conduit bodies likewise. The destination patterns of INS-012 are written 8 times (spec/schemas/templates.yaml:203, 242, 288; spec/schemas/sectioned.yaml:385, 396, 407, 418, 429). SCT-005 promises that a sectioned template's body "is exactly the canonical template `spec`", yet nothing enforces it: tools/check_schemas.py only validates the two sectioned examples, and no generator exists. I compared every copy mechanically at this commit and they agree. The next key added to a template `spec` must be added in up to nine places, as `signIn` and `toolsDirectory` recently were to a Type; a missed copy makes the short form reject what SCT-005 allows, and the only signal would be an author's failed registration.

**Counterargument.** Closed maps cannot be extended in this profile, because it excludes `unevaluatedProperties` and `additionalProperties: false` does not see through `$ref`, so some repetition is forced. The copies do agree today.

**Least change.** Make the two name rules Schema contracts (`sectioned-name.v1`, `local-name.v1`) and refer to them with `propertyNames: {$ref: ...}`, as name.v1 already is. State each template body once as an open contract and close each use with `propertyNames: {enum: [...]}` beside the `$ref`, which the profile allows. Until then, add a step to tools/check_schemas.py that compares each sectioned copy with its canonical schema.

**Precedent.** JSON Schema's own reuse by `$ref`; the `propertyNames` enum idiom for closing an extended map (or 2020-12's `unevaluatedProperties`); Kubernetes generates every copy of an object's schema from one definition.

**Who decides.** coordinator. Needless complexity whose removal changes no verdict; IN_FORCE.md topic 1 says the schema profile sets Conduit apart in nothing and may be simplified freely.

**Verifier.** confirmed, low. Quote verbatim at spec/schemas/sectioned.yaml:27. Recounted in sectioned.yaml: 19 copies of the pattern, 13 lists of the five names, 6 lists of `t`, `operator`, `manages`. Destination patterns at spec/schemas/templates.yaml:203, 242, 288 and sectioned.yaml:385, 396, 407, 418, 429. SCT-005 at spec/promises/sections.yaml:88-89: "Its body is exactly the canonical template `spec` of that family". No code names a sectioned sub-contract: a search of engine/, tools/ and conformance/ for `sectioned-templates`, `sectioned-interface` and `sectioned-agent` returns nothing, so nothing compares the copies. I compared key sets mechanically: each member of sectioned-templates.v1 holds the canonical spec's keys plus `description` and `annotations`; each inline branch holds them less `inputs` (as SCT-006 requires) plus the short-form keys. The copies agree at this commit. IN_FORCE.md:28 lets the schema profile be simplified freely. Not recorded in any known-open list.

**Verifier's corrections.** Class, unsafe mark (none) and who decides stand. Three precisions. (1) 961 of the 2,993 lines of schema YAML is about a third; a quarter only if the README is counted. (2) A key added to an Interface, Agent or Resource `spec` needs 7 places (1 canonical, 3 in the templates section, 3 inline), not nine. (3) Closing each use with a `propertyNames` enum still lists the keys by hand at each use, so it removes the copied value schemas, not every copy; and any move of a keyword changes pointers that vectors pin (F1). Severity low because the copies agree today and the cost falls on maintainers, not on builders who embed the schemas.


## FR-137

**The schemas README has drifted from the files it describes, and two by-rule examples do not say what they show**

L12/F6 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** SCH-009, IFC-033, FLEET-001, FLEET-006, OBJ-020, MAN-010

**Where.** spec/schemas/README.md:252

**Quote.** "38 files, each holding one set of one or more documents and named for the promise the set breaks."

**Situation.** Recounted at this commit: examples/invalid-by-rule holds 41 files, and spec/README.md:242 says 41. All 41 name a promise that exists, and all their documents are valid against the schemas (I ran them). Other stale statements: line 155 says envelope.v1 states "what all ten kinds share" where line 104 and the schema say eleven; line 228 says a Type's `family` is "one of the four families" where type-spec.v1 lists five, Fleet included; lines 226-227 call `plain: false` a choice "of the schemas alone" though MAN-010 states it (spec/promises/manifest.yaml:218). The table "What a schema cannot express" says at lines 166-167 that a by-rule example "exists only for" rules it marks as applied by the tool, but three examples serve rules with no such row: SCH-009's conflicting defaults (two files; SCH-009's only row is marked with a dash) and IFC-033's reserved `$defs` key, both of which the tool applies. The table also has no row for FLEET-001 (template.v1 and type-spec.v1 accept a Fleet template and a Fleet Type from any package) or FLEET-006 (instance.v1 accepts a Pool or Fleet named management, access or system). examples/valid/equal-defaults is in no table. SCH-009-allof-defaults.yaml and SCH-009-conflicting-defaults.yaml open with a blank line and carry no sentence saying what they show, unlike the other 39. An engine builder who takes the table as the list of checks beyond the schemas misses four rules.

**Counterargument.** The README binds nothing that the promises do not, the table does not claim to be complete, and spec/README.md carries the right count. The tool checks that the tables list the schema files.

**Least change.** Correct the four statements; add rows for SCH-009's admission check, IFC-033's reserved key, FLEET-001 and FLEET-006; list equal-defaults; give the two SCH-009 examples the opening comment the others have ("One document, valid against the schemas, that must be rejected: ..."). Have tools/check_schemas.py count the example files against the README and require a row for each promise ID that prefixes a by-rule file.

**Precedent.** The tool's existing check that this page's tables "list what there is", extended to the examples.

**Who decides.** coordinator. A defect in text, with no behavior involved (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/schemas/README.md:252. Recount: 41 files under spec/schemas/examples/invalid-by-rule; spec/README.md:242 says "41 files". README.md:155-156 "all ten kinds" against :104 "eleven kinds" and spec/schemas/envelope.yaml:6. README.md:228 "four families" against spec/schemas/contracts.yaml:146-152, five with Fleet. README.md:226-227 calls `plain: false` a choice of the schemas alone; spec/promises/manifest.yaml:218-219 states it. README.md:166-167 "exists only for those", yet the SCH-009 row at :179 has a dash under Tool and the table has no row for IFC-033, FLEET-001 or FLEET-006 (search of the page). SCH-009-allof-defaults.yaml and SCH-009-conflicting-defaults.yaml open with a blank line and no sentence saying what they show; IFC-033-reserved-root-defs.yaml and the others open with one. examples/valid/equal-defaults is not in the Examples table. Validator run: document.v1 accepts a Fleet template of any name with the core type, a Type of family Fleet, a Fleet instance named `management` and a Pool named `system`, so FLEET-001 and FLEET-006 are engine rules with no row. Not recorded; the brief's "counts go stale" is a general warning.

**Verifier's corrections.** Class defect, not unsafe, coordinator: all stand. The stated harm is smaller: the table does not claim to be complete (README.md:164-165 says only that these promises "are not checked by validating a document against a schema"), and each missing rule is in a promise, so only a builder who reads the README in place of the promises misses them. This is housekeeping; fold it into one README pass.


## FR-138

**Rules that only a schema states, or that a schema states more narrowly than its promise**

L12/F7 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** OBJ-014, OBJ-015, INS-021, CTR-002, CTR-004, CTR-005, CTR-013, MAN-010, SCT-012, SCT-013, INS-012

**Where.** spec/schemas/README.md:226

**Quote.** "These remain choices of the schemas alone: `optional` and `plain` are booleans, so `optional: false` and `plain: false` are accepted; and a Type's `family` is one of the four families that have a type."

**Situation.** The README admits one such rule; I found these, each checked with a validator. (1) `optional: false` is accepted on an input, a declared Conduit, an entry and a Requirement's input (templates.yaml:308, instances.yaml:242 and 260, contracts.yaml:357); OBJ-014, OBJ-015, INS-021 and CTR-005 name only `optional: true`. (2) Every string of `command` must be non-empty (manifest.yaml:169), so an empty argument such as `[prog, --prefix, ""]` is rejected; MAN-010 says "a list of one or more strings". (3) `effect` must be non-empty and `conditions` hold distinct non-empty strings (contracts.yaml:285, 181-186); CTR-002 and CTR-004 are silent. (4) A redirect login's stated ports are 1024 to 65535 and its `authorization` has no query (contracts.yaml:412, 395); CTR-013 says neither, and IFC-049 gives that range only for `any`. (5) `page` may hold user information, which `authorization` and `issuer` may not (contracts.yaml:419 against 395 and 428); no promise says so. (6) The names under `manages` exclude five names (sectioned.yaml:933-939); SCT-012 names three and SCT-013 says only "the same authored-name rule", so a Fleet legally named local cannot be managed in the short form. (7) INS-012 gives one list of destinations; the per-family split, such as no `/config` in a Conduit template, is in the schemas and the README table alone. A builder who writes checks from the promises diverges on each; one who embeds the schemas inherits rules no promise can be cited for.

**Counterargument.** SUB-005 gives the published schemas force, so every engine that embeds them agrees. Each item is small, and item 1 is stated openly. OBJ-022 shows the pattern done well: it says that `labels.v1` gives the form of a label.

**Least change.** For each item either add the sentence to the promise or relax the schema: (1) "`optional: false` means the same as leaving it out", as MAN-010 says of `plain`; (2) drop `minLength` from all but the first string of `command`; (3) and (4) one clause each in CTR-002, CTR-004 and CTR-013; (5) forbid user information in `page`, or say why it is allowed; (6) say in SCT-013 which five names; (7) cite the README's table from INS-012.

**Precedent.** OBJ-022's delegation to `labels.v1`, and MAN-010's sentence on `plain: false`.

**Who decides.** coordinator. Gaps and small ambiguities whose fixes add no behavior beyond what the schemas already enforce (rulings 37 and 46 point 4); item 5, if tightened, is a small change under ruling 71 point 4.

**Verifier.** weakened, low. Quote verbatim at spec/schemas/README.md:226-228. Validator run on the bundled schemas: `optional: false` accepted (spec/schemas/templates.yaml:308); `command: [prog, --prefix, ""]` rejected at /command/2 by `minLength` (spec/schemas/manifest.yaml:169); `effect: ""` rejected (spec/schemas/contracts.yaml:285); `authorization` with a query rejected (:392-395); stated port 80 rejected (:412); `page` with `user:pass@` accepted (:416-419); `manages` naming `local` rejected (spec/schemas/sectioned.yaml:927-939); a Conduit template's input to `/config` rejected (templates.yaml:288). Three items fall. Item 1 and the `conditions` half of item 3 are recorded choices accepted with ruling 37: docs/proposals/core-and-schemas/README.md:53, C8 "`optional` and `plain` are booleans, and `false` means the same as leaving the key out", and :57, C12 "A Type's `conditions` is a list of distinct, non-empty strings", under "Choices that no promise states"; OBJ-015 (spec/promises/objects.yaml:245) gives the meaning: "Without that mark it is required". Item 6 is decided by SCT-013, spec/promises/sections.yaml:227-230: "are none of `system`, `local`, `management`, `access` and `root` ... The names under `manages` follow the same authored-name rule." Item 7 is mostly derivable: CON-016 (spec/promises/conduits.yaml:204-205) closes a Conduit template's keys and INS-011 rejects a destination whose parent is absent; only `/config` in a Conduit template passes INS-011 and INS-012's list as written (INS-024, instantiation.yaml:335, names four families). Items 2, 3 (`effect`), 4 and 5 stand: MAN-010 manifest.yaml:185-186 "a list of one or more strings"; CTR-002 spec/promises/contracts.yaml:42, 48; CTR-013 contracts.yaml:291-292; IFC-049 spec/interface/implementation.yaml:1400-1404 gives 1024 to 65535 only for `any`.

**Verifier's corrections.** Drop item 1, the `conditions` half of item 3 and item 6; narrow item 7 to `/config` in a Conduit template. Item 2 is a defect, not a gap: the schema rejects what MAN-010 allows, and README.md:4-5 says the schema is then wrong. The rest are small gaps where a schema alone states a rule. Not unsafe: SUB-005 binds every engine to the schema's rejection, and `page` is a literal of a public contract. Coordinator. Low: each surviving item is one clause in a promise or one keyword in a schema.


## FR-139

**The shape of a document is stated three times and linked only by comments; the schema's internal layout is already observable, so it is the real source**

L12/F8 · needless-complexity · verifier: weakened, medium · decides: owner · new

**Promises.** SUB-005, OBJ-012, OBJ-013, OBJ-014, OBJ-018, OBJ-019, INS-021, CTR-001, MAN-004, MAN-009

**Where.** spec/schemas/README.md:4

**Quote.** "The promises are the source of truth: where a schema and a promise differ, the schema is wrong."

**Situation.** A document's shape is written as promise prose, as a schema, and again in the README's tables (the page cites 135 promise IDs). About 100 of the 519 promises appear under "Promises encoded" in the schema files' head comments, which no tool reads. Yet SUB-005 makes a schema's verdict binding, and the vectors pin its internals: conformance/language/contracts/sign-in-logins/case.yaml:94-98 expects contract type-spec.v1 and pointer /properties/signIn/required. So moving a keyword inside a schema changes a conforming engine's output, and whether a rule sits in a schema or in the engine decides the rule ID a caller sees: the Pool rule of CTR-004 is in type-spec.v1 and reports SUB-005, while PRV-001, SEC-022 and PKG-020, which the README (lines 199-201) says the profile "could state", are engine rules reporting their own IDs. Nothing mechanical ties prose to schema; the drift I found (F2, F3, F6, F7) was found by reading. Each change to a shape costs an edit to a promise, a schema, up to eight copies (F5), a README row, an example, a Python rule and the Rust loader.

**Counterargument.** Prose promises are what a person reads and what a ruling amends; a schema cannot say why. The examples, 74 schema-rejected and 41 rule-rejected, are a check on drift, and the first build compares its verdicts with the tool's on each.

**Least change.** First, with no change to what binds: add to the form of a promise a field beside `vectors`, `schemas: [&lt;contract>, ...]`, in place of the head comments, and have tools/check_schemas.py require, for each promise that names a schema, an example under examples/invalid that the named contract rejects. Then, for the owner: let a shape live in its schema alone, keeping in promises the meaning and what no schema can say.

**Precedent.** Kubernetes: an object's OpenAPI schema is the single statement of its shape, cross-field rules are attached to it, and the prose conventions state meaning; ruling 40 asks for an existing paradigm outside connection and access.

**Who decides.** owner. The second step changes which text binds a shape and removes promise text, which changes only by a logged ruling (AGENTS.md, "Where things go"); the first step is the coordinator's.

**Verifier.** weakened, medium. Quote verbatim at spec/schemas/README.md:4-5. Counts recounted: 98 distinct promise IDs under the "Promises encoded" heads of the nine schema files, 135 distinct IDs in the README, 519 promises. tools/check_schemas.py never reads those heads (no occurrence of "Promises encoded"; check_promises at :2248 reads only the promise files). The vector lines are as cited: conformance/language/contracts/sign-in-logins/case.yaml:94-97 pins `type-spec.v1` and `/properties/signIn/required`. README.md:199-201 says the profile "could state" PRV-001, SEC-022 and PKG-020. Two claims are overstated. (1) "The schema is the real source": SUB-005 gives force only to a schema's rejection; README.md:217-218 says "A schema therefore answers one question ... It never says a document is accepted", so where a schema is looser the promise governs. (2) "Nothing mechanical ties prose to schema": the 74 files under examples/invalid are named for the promise each breaks, and the tool (README.md:268-270) and the loader's test (docs/proposals/first-build/README.md:57) check each is rejected. Part is known: that the schema's layout is observable, and that the rule cited depends on where a rule sits, both follow from the owner-reviewed decision at docs/proposals/first-build/to-review-with-the-owner.md:18; settlement 1-01 already rejected "Per-keyword promise annotations" as costly. "The promises are the source of truth" was a drafting rule of the batch ruling 37 accepted (docs/proposals/core-and-schemas/README.md:32-33).

**Verifier's corrections.** This is the assessment restated as a finding; its concrete faults are F2, F4, F5 and F7. Present it to the owner as one question (may a shape live in its schema alone), not as a separate fault. Who decides is right: the second step reverses an accepted drafting rule and removes promise text, so it is the owner's. The first step is not clearly a simplification: it adds an eighth field to the form of a promise (the tool enforces exactly seven, tools/check_schemas.py:2249) and a check that needs an invalid example for each encoded promise, though many encoded promises are permissions with no invalid case. Weigh it against ruling 40 before ordering it. Not unsafe.


## Further points the verifier noticed

Not verified by a second reader.

- **A framed Agent with no `owner`, in a file whose sectioned document states `fleet`: SCT-011 supplies the owner, instance.v1 rejects the document** (spec/promises/sections.yaml:185). SCT-011 (sections.yaml:185-186): "supplies X as the owner of every Agent instance defined anywhere in that file, whether sectioned or framed", and :190-191 says Agents in another file "must state `owner` unless their own file supplies one". But instance.v1 requires `owner` of every framed Agent (spec/schemas/instances.yaml:101-105), OBJ-013 says "`owner`, `consumer` and `target` are required for those families" (spec/promises/objects.yaml:200-201), and FLEET-002 rejects "An Agent document with no `owner`" (spec/promises/fleets.yaml:34). No promise says whether a framed document is checked against document.v1 before or after the file's `fleet` supplies its owner. Builder A validates each framed document as authored and rejects under SUB-005, which is what tools/check_schemas.py does (:2405 validates each document as read); builder B supplies the owner first and accepts. Class: contradiction between SCT-011 and the schema with OBJ-013, or an ambiguity of order. Not unsafe. Not in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md, the first-build logs or the review list. Coordinator: one sentence in SCT-011 saying either that the owner is supplied before the schema check, or that a framed Agent always states `owner` and the file's `fleet` only checks it.

## The verifier's notes

Verified at commit 2eb8f258 (2026-11-12), confirmed with `git log -1`; the tree was left clean. Nothing was written, cargo was not run, and no environment was printed.

Result: 4 confirmed (F1, F2, F5, F6), 4 weakened (F3, F4, F7, F8), none refuted, none wholly known. I agree that no finding is unsafe: in every case accept or reject is the same for both builders, or SUB-005 binds all engines to the schema's rejection.

How I checked: every quote was opened at its path:line and is verbatim. Validator claims were rerun in memory with /usr/bin/python3 (PyYAML and jsonschema), bundling the 77 Schema contracts under spec/schemas and spec/core as spec/schemas/README.md:25-27 describes. That run used the stock `pattern` and format checks, not the profile's ECMA-262 matching, which does not affect the cases tested. Counts were recounted: 41 by-rule files, 74 invalid files, 519 promises, 98 IDs in the schema heads, 135 in the README, and 19, 13 and 6 inline copies in sectioned.yaml.

Points for whoever merges these:
- F1 and F4 are two halves of one question, what a SUB-005 diagnostic holds (which keyword and place; what the message says). Settle them in one change to SUB-005. The owner already agreed (review-list item 5, 2026-11-10) that a schema rejection cites SUB-005; neither finding reopens that.
- F8 is the reviewer's assessment restated; its distinct content is one owner question.
- The record docs/proposals/core-and-schemas/README.md, section (b) "Choices that no promise states" (C1 to C16), is not on the brief's list of places to check for known points, but it holds accepted schema-only choices and removed two of F7's items. Other reviewers of schemas should check it.
- engine/CONFORMANCE.md:28 files the first build's choice of reported schema failure under DL9, whose text covers only order; that supports F1 and may itself need correcting when F1 is fixed.

Not verified: the reviewer's assessment claims about formats (the leap second in `date-time`) and about the Rust validator's `patternProperties` handling; they are outside the eight findings.
