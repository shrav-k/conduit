# Spec batch 1, area: short-form

Not binding. An area planner's draft for [the batch 1 plan](../README.md); where the plan differs, the plan governs.

## Scope

The short form and its keys, for spec batch 1. This covers ruling 55's sectioned documents: the SCT promises, the translation table, the schema `sectioned.v1`, examples, the error mapping, the as-if and equivalence rules, and stable names. It also covers ruling 63's documented flow style and the standard Conduit templates that packages ship, NAM-024 with the Agent Types `claude-code.v2` and `codex.v2`, and the amended wording of rulings 8, 18, 31, 35 and 37 (ruling 19 is kept). Last, it covers the `fleet:` key (ruling 59 A6 and ruling 60) and the `manages:` key (ruling 66). Repository root for every path below: /Users/shravankumaran/Projects/Conduit/conduit. Nothing was modified.

## Promise and core changes

A. NEW FILE spec/promises/sections.yaml, prefix SCT (free). All promises are layer `language`, level `must`. Its head comment says three things: canonical (framed) documents stay the kernel, the wire and the stored form; the SCT rules define the short form by translation, as the Haskell report defines its sugar; and change (plan and apply) stays tooling under ruling 53 #4. SCT-010 of the synthesis, the derived Pool operator, is not written: ruling 59 A3 makes root's standing Conduit to every Pool and Fleet the engine's.

- SCT-001, the form. WHEN a file of documents of a project or package holds a map with no key `kind`, as a YAML document, a JSON value or an item of a JSON array. THEN it is a sectioned document. Its keys are closed: `apiVersion`, `fleet`, `templates`, `interfaces`, `agents`, `resources`, `pools` and `conduits`. It holds at least one key other than `apiVersion`, and it is valid against `sectioned.v1`. OBSERVABLE:
  - Another key, or `apiVersion` alone, is rejected under SUB-005 with a pointer into the authored text.
  - A map holding `metadata` or `spec` without `kind` is rejected with a message that a framed document needs `kind`.
  - The manifest is never read as one (MAN-001). Contracts and the manifest are always framed.
  - The wire stays canonical (ruling 55 A1): a sectioned value in `register.v1`'s `documents` is rejected under SUB-005.
- SCT-002, the version. THEN a sectioned document's version is the `apiVersion` it states, or else its manifest's; with neither, it is rejected. Every translated document carries that version. OBSERVABLE: OBJ-017's unknown-version rejection and OBJ-024's mixing rule apply to this version.
- SCT-003, the meaning. WHEN a project or package whose files hold sectioned documents is registered (SUB-010, submitted files) or installed. THEN it means exactly the project in which each sectioned document is replaced by the framed documents of its translation (SCT-005 to SCT-012), handed over as one submission with the project's framed documents and manifest, and nothing else. OBSERVABLE:
  - SUB-001 to SUB-007, OBJ-002, NAM-027, the INS rules, CON-021, IFC-014 and the rest apply to the translated documents unchanged.
  - No provenance of the translation is stored. The generated names are its record.
  - Inspection, records and the wire show framed objects only.
- SCT-004, purity and determinism. THEN the translation depends only on the text of the sectioned documents of its project or package and on the SCT-002 version. It reads no package, contract, stored object, caller, file name, directory or environment, and the same text gives the same documents. OBSERVABLE:
  - Two translations are equal under SCH-007. Order of keys and files means nothing (SUB-001).
  - A definition whose text is unchanged translates to unchanged documents, so its objects keep their names across a replacement (ruling 55 A3).
  - Names joined from keys by fixed rules are not ruling 8's IN5 interpolation.
- SCT-005, the `templates` section. THEN each `templates.<interfaces|agents|resources|pools|conduits>.N` translates to the framed template N of that family. Its `spec` is the member without `description` and `annotations`, which go to its `metadata`. OBSERVABLE: the body is exactly that family's template `spec`, with `inputs` allowed. There is no `templates.fleets` in this version.
- SCT-006, definitions. WHEN a member N of `interfaces`, `agents`, `resources` or `pools` is read. THEN:
  - Inline (it states `type`): it translates to template `N--t` (with `type`, `config`, `implementation`, `controls`, and either `conduits` or, for a Pool, `entries`) plus instance N {from: N--t, standard arguments}. This amends rulings 8 and 18: a template and its instance are made together.
  - Referenced (it states `from`, in any of INS-025's four forms, and optionally `arguments`): it translates to instance N.
  - `description` and `annotations` go to the instance's `metadata` only.
  OBSERVABLE:
  - Both or neither of `type` and `from` is rejected. `inputs` or `arguments` on an inline definition is rejected.
  - Standard arguments follow OBJ-013: `owner` is required on a Resource and on an Agent unless its document states `fleet`; `placement` and `runtime` are optional.
  - Nothing inside `config`, `arguments`, `implementation`, `controls`, `conduits` or `entries` is rewritten (OBJ-021, SEC-001).
- SCT-007, `consumes`. WHEN a definition N of family F (Interface, Agent or Resource) holds `consumes: {L: item}`. THEN each item translates to Conduit `N--L` {consumer {kind: F, name: N}, target: `to`, as: L}. Form (a) {capability, operations, requirements?} makes it from a generated template `N--L--t`. Form (b) {from, arguments?} makes it from the named template, which may be a package's qualified template or a project's own. OBSERVABLE:
  - `to` names its kind, `{kind, name}` or a reference to a Pool's entry. A bare name is rejected (OBJ-006; ruling 19 kept).
  - Operations are fully qualified and never grouped (rulings 25 and 53 #8).
  - Tools are `L.<export>` (IFC-012 with `as`). Mixing the two forms is rejected. A Pool has no `consumes`.
- SCT-008, Conduits to `management` and `access` (synthesis F5). WHEN a `consumes` item's `to` is `management`, or is `access` with no `bound.v1` applied. THEN the document is rejected. OBSERVABLE: the diagnostic says such a Conduit is written in the `conduits` section with consumer and target written out. `access` with a bound is accepted.
- SCT-009, `operator`. WHEN a definition N of an Interface, Agent or Resource holds `operator`. THEN it translates to Conduit `N--operator` {consumer {kind: Interface, name: root}, target {kind: F, name: N}, as: N}. It is made from `N--operator--t` (when it states capability, operations and requirements) or from its `from` template. OBSERVABLE:
  - Root's tools are `N.<export>`.
  - A Pool takes no `operator`: root's standing Conduit is the engine's.
  - An `as` equal to another of root's Conduits' `as` is rejected under the per-consumer `as` uniqueness rule (an amendment to IFC-014 that the `as` area writes), located at `/…/operator`.
  - Removing the line is reachable now that ACC-024 lets root close its own non-default Conduits (ruling 59 A3).
- SCT-010, the `conduits` section. THEN each member N translates to a standalone Conduit N, made from `N--t` when written inline ({capability, operations, requirements?}) or from its `from` with `arguments?`. Each states `consumer` and `target`, both with kinds, and optionally `as`, `description` and `annotations`. OBSERVABLE: without `as`, the local name is N (IFC-012). A Conduit of any kind the kernel accepts may be written here, including ones to `management` or to `access` without a bound.
- SCT-011, `fleet:`. WHEN a sectioned document states `fleet: X`. THEN its translation holds the Fleet X {from: github.com/shrav-k/conduit/core#fleet}, and every Agent that document defines has owner X. The Fleet is one document for the whole project however many of its files state `fleet: X`. OBSERVABLE:
  - An Agent of that file that writes `owner` is rejected, naming both.
  - X is a bare name, since the key fixes the kind (ruling 19).
  - Agents in other files, or under a Fleet another submission made, state `owner:` (ruling 59 A6).
  - `fleet` in a package is rejected, as an instance document is (MAN-011).
  - It writes nothing for root: root's standing Conduit to X is the engine's.
- SCT-012, `manages:`. WHEN a definition N of an Interface, or of an Agent, holds `manages: {fleets?: {T: role}, pools?: {T: role}}` with role `manage` or `view`. THEN each T translates to Conduit `N--manages--T` {consumer N, target {kind: Fleet|Pool, name: T}, as: T}, made from the core's standard template `fleet-manage`, `fleet-view`, `pool-manage` or `pool-view`. OBSERVABLE:
  - Nothing else is made or stored: a scope is its Conduits (ruling 66 points 1 and 6; rulings 27 to 29).
  - `system`, `management` and `access` are never members, and no member is a wildcard.
  - An Agent's `manages` holds only `fleets`, whose one member is its own Fleet (its file's `fleet` or its `owner`). Fleets as holders come in batch 2.
  - The templates list no register, retire, cascade or `access` operation (ruling 66 point 2). Root opens them at apply.
- SCT-013, names. THEN the names a sectioned document gives (definitions, templates, `conduits` members, and the value of `fleet`) follow OBJ-001, contain no `--`, and are none of `system`, `local`, `management`, `access` and `root`. `consumes` keys follow OBJ-022, contain no `--`, and are none of `t`, `operator` and `manages`. Every generated name (`N--t`, `N--L`, `N--L--t`, `N--operator`, `N--operator--t`, `N--manages--T`) is at most 63 characters. OBSERVABLE:
  - A violation is rejected at the authored key.
  - These names are fixed by the specification, not by DL15.
  - A generated name equal to another document's name, framed documents included, is rejected under NAM-027, naming both authored places.
  - The pattern without `--` is `^[a-z](?:[a-z0-9]|-[a-z0-9])*-?$`, which needs no lookahead (SCH-016).
- SCT-014, the error mapping. THEN every diagnostic about a sectioned document or a translated document is located in the authored text. Its pointer leads to the definition, item or member that made the document, and to the field within it where the rejecting check names one. `ruleId` is the promise that rejected it. `properties.translation` holds {rule: the SCT ID that made the document, document: the generated name}. OBSERVABLE:
  - Mapping: `spec.X` of N--t → /F/N/X; instance N's standard arguments and `arguments` → the same keys; N--L--t → /F/N/consumes/L/…; N--L `target` → …/to; operator and manages analogously; the Fleet and its owners → /fleet.
  - Schema rejections cite SUB-005 with keywordLocation.
  - The reason reported stays OBJ-023 (invalid) or OBJ-025 (conflict).
  - A value at a place governed by a secret reference is never echoed (SEC-003).
- SCT-015, as-if. THEN whatever its method of translating, an engine's objects, records and reason are those of registering the translated documents. Its diagnostics have the same `ruleId` and `level`, located under SCT-014. OBSERVABLE: registering the rendered translation in one state, and the sectioned project in an equal state, gives equal states (IDs apart) and equal records.

B. AMENDED PROMISES
- OBJ-012: a template document is framed; a sectioned document's `templates` section is the other notation (SCT-005).
- OBJ-013: the word "framed". Write this once together with the `as` area (Conduit `as`) and the Fleet area (Agent `owner`).
- OBJ-017: a framed document carries `apiVersion`; a sectioned one follows SCT-002 (ruling 31 and 35 wording).
- OBJ-019: exempts keys that the author names: section maps, `templates.*`, `consumes`, and the maps under `manages`.
- OBJ-020: `kind` applies to framed documents; a map without `kind` falls under SCT-001.
- OBJ-022: named members also include section members, `consumes` keys and `manages` members.
- OBJ-024: "a document's `apiVersion`" becomes "the version of each document (OBJ-017, SCT-002)".
- INS-001: the `from` marker applies to framed documents; in a sectioned document the section decides (rulings 8 and 18 amended).
- MAN-003: observable cites SCT-002.
- MAN-007 and PKG-018: observable adds that a package's export may be a template that a sectioned document of it makes.
- MAN-011: a file of documents may hold sectioned documents. A package's sectioned document holds only `templates`, and any other section, or `fleet`, is rejected at install.
- SUB-004: the diagnostic names the authored places.
- SUB-005: a submission's files may hold sectioned documents, and schema pointers point into them.
- SUB-010: registers "the framed documents read and the translation of the sectioned documents read".
- SCH-018: an array item or JSON value may be a sectioned document.
- NAM-027: observable says it counts translated documents, generated names included.
- NAM-024, new THEN: "The later contract may add optional things, such as an optional input of an operation, a further operation of a capability or an optional property of a Type's `schema`. A property that it adds to a Type's `schema`, at any depth, and an input that it adds to a Requirement contract, state no `default`. It does not remove or change anything the earlier one held." Observable adds two things:
  - Why: under SCH-009 a default would flip what an unchanged document means at the next registration after a pin moves. A default comes only with a new version segment, as `claude-code.v2` and `codex.v2`.
  - Proposed coordinator decision D7: installing a release while another release at the same address is installed, where a same-named contract breaks this rule, is rejected (PKG-023 invalid, NAM-024 diagnostic naming the contract and the pointer).

C. NEW PKG-031 (layer language, level should; ruling 63 #3). A package that exports a capability, or a Type offering one, should also export Conduit templates for that capability's common uses. Each is named for its use (`read`, `read-write`, `turn`) and lists fully qualified operations, so authors write `from: <address>#read` and no operation list. A later release at the same address keeps each such template's operations and adds a new template for a wider use. Observable: INS-013 resolves the template at the pinned version (NAM-010); the plan's widening check covers what this should-rule cannot enforce.

D. CORE PACKAGE AND SCHEMAS
- spec/core/templates.yaml gains four Conduit templates; core/conduit.yaml exports them; core/README.md maps each to SCT-012 and PKG-031:
  - `pool-manage`: core#pool-management.v1 with [list.v1, inspect.v1, set-up.v1, reconcile.v1];
  - `pool-view`: [list.v1, inspect.v1];
  - `fleet-manage`: core#fleet-management.v1 with [list.v1, inspect.v1, set-up.v1, reconcile.v1], with no act.v1 until batch 2;
  - `fleet-view`: [list.v1, inspect.v1].
  All are written fully qualified.
- New spec/schemas/sectioned.yaml, about 350 to 450 lines. Every key is restated, because the profile has no unevaluatedProperties. It holds:
  - `sectioned.v1`, `sectioned-templates.v1`, and one schema each for `sectioned-interface.v1`, `sectioned-agent.v1`, `sectioned-resource.v1` and `sectioned-pool.v1`, each a oneOf of inline and referenced definitions plus the common keys;
  - `sectioned-conduit.v1`;
  - `consumes-item.v1`, with two forms and `to` as target-reference.v1;
  - `sectioned-operator.v1`, with two forms;
  - `manages.v1`, maps of name to an enum [manage, view] with propertyNames excluding the reserved names.
  It reuses selection.v1, template-reference.v1, implementation-map.v1, template-controls.v1, declared-conduit.v1, pool-entry.v1, applied-requirement.v1 and the reference schemas. schemas/README.md gets a row under "which schema" (`kind` present: document.v1; absent, outside the manifest: sectioned.v1). document.v1, instance.v1, template.v1 and manifest.v1 are unchanged by this area.
- core/results.yaml: `diagnostic.v1`'s description names `properties.translation`.
- Examples, written in flow style (ruling 63 #2):
  - examples/valid/sectioned-project/: conduit.yaml plus s1.yaml, S1 at T2 with `fleet: s1`, package templates and `.v2`, and one Interface with `manages`;
  - examples/valid/sectioned-package/: a templates-only document exporting `read` and `read-write`;
  - about 12 cases under examples/invalid/: unknown top key, `metadata` without `kind`, `type` with `from`, mixed `consumes` forms, a bare `to`, `operator` on a Pool, a `manages` role outside the enum, `manages` naming `system`, a name with `--`, a reserved local name, `templates.fleets`, `inputs` inline.

E. spec/definitions.md
- New rows:
  - sectioned document;
  - frame and framed document (`apiVersion`/`kind`/`metadata`/`spec`);
  - section;
  - sectioned definition (inline or referenced), avoiding "entry", which stays the Pool's;
  - translation;
  - generated name;
  - the keys `consumes`, `operator` (root's Conduit to the object; the prose word "operator" keeps its existing sense), `fleet` and `manages`;
  - scope (an Interface's open Conduits to Pools and Fleets; nothing stored besides them);
  - standard Conduit template.
- Updated rows: document, template, instance, name, reference (ruling 19 kept), project, contract (NAM-024).
- The "words not used" row for `grant` notes that the short form says `consumes`.

F. Other documents
- spec/README.md:
  - Contents rows and counts (455 plus 15 SCT plus PKG-031);
  - the author's reading path, which starts with the short form in flow style; spec/ is the format reference (ruling 53 #9 read with A1);
  - a new limit "What the short form's meaning covers": registration only; the plan is tooling; the store equals the translation after an apply except IDs, held pins and values set by controls.
- docs/AUDIT.md "Later amendments" gets rows:
  - 8: one shape; an inline definition makes N--t and N together;
  - 18: in a sectioned document the section, not `from`, decides; every instance still comes from a registered template;
  - 31: the frame is optional;
  - 35: the version is known per document, stated or the manifest's;
  - 37: `sectioned.v1`; contracts and the manifest are always framed.
  Ruling 19 is noted as kept.

G. COORDINATOR DECISIONS TAKEN IN DRAFTING. To be logged on to-review-with-the-owner.md; none changes behavior beyond the rulings.
- D1: `manages` is a map of name to role, with no default role (ruling 52's explicitness), in the brief's `{name: {...}}` style. Alternatives: lists with a required `role`, or lists plus a sibling `views:` key.
- D2: Agent `manages` (own Fleet only) is in batch 1; a Fleet as holder waits for batch 2.
- D3: the `fleets:` section and `templates.fleets` are deferred to batch 2 (with `holds`).
- D4: `operator` is rejected on Pools and takes `{from}`.
- D5: an inline definition's description goes to the instance only.
- D6: SCT-008, from F5, is adopted as part of the accepted grammar.
- D7: the NAM-024 check at install.
- D8: new fixture versions rather than re-pinned ones.
- D9: scope is defined by open Conduits, not by which template made them.
- D10: `properties.translation`.
- D11: `fleet` takes a bare string only, deduplicated across files.
- D12: reserved local names and the `N--manages--T` scheme.
- D13: the names of the core templates.
- D14: PKG-031 as a should-rule, with stable templates across releases.

Size: 15 new and about 18 amended promises in this area, 1 should-promise, 4 core templates, about 10 schemas, and about 14 examples.

## Vectors

New cases go under conformance/language/sectioned/<case> with id `sectioned-<case>`. They need two runner features: a new step kind `translate` (expected framed documents plus each document's SCT rule, or diagnostics with authored pointers and properties.translation.rule), and an `equivalent: true` option on `register-project`, modelled on the registration-orders comparison. Each case is recorded in its promise's `vectors` field and in conformance/manifest.yaml, and the SCT IDs are added to s1-promises.yaml. About 65 to 75 cases.

FORM, SCT-001 (plus SUB-005 and SCH-018):
- form-minimal: one Pool gives N--t and N.
- form-no-section: `apiVersion` alone is rejected.
- form-unknown-key: `grants:` is rejected at /grants.
- form-frame-without-kind: rejected; the message names `kind`.
- form-json-array: framed and sectioned items in one array.
- form-contract-key: rejected.
- form-wire-canonical: register.v1 with a sectioned value is invalid under SUB-005.

VERSION, SCT-002:
- version-from-manifest
- version-stated-no-manifest
- version-none-rejected
- version-unknown-rejected

MEANING, PURITY AND STYLE, SCT-003 and SCT-004:
- mixed-framed-sectioned: a framed template is used by a sectioned `from`, and a framed Conduit's consumer is a sectioned Agent.
- atomic-rejection (SUB-003).
- order-independence: files and keys reordered give equal output.
- flow-block-identical: S1 at T2 in flow and in block style gives byte-equal translations (ruling 63 #2: block stays legal).

TEMPLATES, SCT-005:
- templates-each-family: inputs kept, description goes to metadata.
- templates-fleets-rejected

DEFINITIONS, SCT-006:
- inline-each-family
- referenced-four-from-forms
- type-and-from-rejected
- neither-rejected
- inline-arguments-rejected
- inline-inputs-rejected
- resource-without-owner: POOL-001, located at /resources/x.
- config-not-rewritten: a `{kind, name}` map under a data schema and a canonical secret reference are both kept.
- description-to-instance-only

CONSUMES, SCT-007:
- consumes-inline: N--L--t and N--L with as L; list-tools shows L.*.
- consumes-package-template: from a qualified `#read`, with no --t; operations equal the template's.
- consumes-local-template
- consumes-bare-to-rejected (ruling 19 kept)
- consumes-mixed-forms-rejected
- consumes-on-pool-rejected
- consumes-pool-entry-target

CONDUITS TO `management` AND `access`, SCT-008:
- consumes-to-management-rejected
- consumes-to-access-unbounded-rejected
- consumes-to-access-bounded-accepted

OPERATOR, SCT-009:
- operator-inline: consumer root, as N; root's tool is N.turn.v1.
- operator-from-package-turn
- operator-on-pool-rejected
- operator-as-clash: an Agent whose name equals that of a Pool registered earlier, so root's `as` clashes; mapped to /agents/x/operator.

CONDUITS SECTION, SCT-010:
- conduits-inline-and-from: `as` passes through; without `as`, the own name.
- conduits-to-management-accepted

FLEET, SCT-011:
- fleet-key: a Fleet from core#fleet, with owner set on each Agent.
- fleet-two-files-one-fleet
- fleet-two-fleets
- fleet-agent-owner-conflict
- agent-without-owner-rejected
- fleet-pool-name-clash (NAM-027)
- fleet-reserved-name
- fleet-in-package-rejected (install-archive)

MANAGES, SCT-012:
- manages-interface: manage on a Fleet and view on a Pool give two Conduits from the core templates, with as equal to the target name.
- manages-reserved-rejected
- manages-bad-role-rejected
- manages-agent-own-fleet
- manages-agent-other-fleet-rejected
- manages-agent-pools-rejected
- manages-as-clash
- manages-template-operations: inspection of the registered Conduits shows exactly the printed operation lists.

NAMES, SCT-013:
- names-double-hyphen
- names-reserved
- names-local-reserved (t, operator, manages)
- names-generated-too-long: located at the authored key.
- names-clash-with-framed: NAM-027, naming both places.

ERROR MAPPING, SCT-014:
- diag-target-capability (CON-021 at /…/consumes/L)
- diag-from-missing (INS-002 at …/from)
- diag-selection-missing (NAM-028 at …/type)
- diag-package-not-installed (PKG-006)
- diag-existing-name (OBJ-002, reported as conflict under OBJ-025)
- diag-arguments-schema (INS-005 at …/arguments/x)
- diag-secret-redacted (SEC-003)

AS-IF AND EQUIVALENCE, SCT-015:
- s1-equivalent: the exact S1 at T2 in flow style, with `fleet: s1`, `.v2`, the package `read` and `read-write` templates and the operators, registered both ways.
- s2-equivalent: a secret Resource consuming get-secret, and a bounded `access` item in `conduits`.

PACKAGES (MAN-011, PKG-031):
- package-templates-only: installs, and exports name the translated template.
- package-instance-section-rejected

NAM-024 (language/packages):
- release-adds-default-rejected: 0.1.0 installed, then 0.2.0 adds `{default: false}` to a same-named Type.
- release-adds-optional-accepted
- release-new-version-segment-with-default-accepted

AGENT TYPES:
- registration/agents-v2-defaults: claude-code.v2 and codex.v2 with empty config resolve to false, false and ''.

EXISTING VECTORS WHOSE OUTCOME CHANGES: none expected from this area. What I read and checked:
- Every kindless map under conformance/ and spec/schemas/examples is either a manifest or a `read`-step loader input: conformance/language/loader/bom/case.yaml and strings/case.yaml have only `"kind": "read"` steps, which parse and never register. Registration routing therefore changes no verdict.
- NAM-024's install check: conventions/changes/t5 installs filesystem 0.2.0 beside 0.1.0, whose contracts equal 0.1.0's (conformance/fixtures/s1/README.md). It is expected unchanged; the order must confirm by running it.
- The new fixture versions (filesystem 0.1.1 and 0.2.1, agent contract fixtures 0.2.0) are added beside the old ones, so no existing pin changes.

To fix in the harnesses, not in verdicts:
- tools/check_schemas.py and reference-model/crates/loader/tests/examples.rs (`by_rule_role`) route every kindless map to `manifest.v1`.
- Three invalid-by-rule sets hold a manifest in a file not named `-manifest-`: TPL-003-template-selects-an-operation-of-the-engine.yaml, SEC-024-secret-header-to-http.yaml and MAN-018-item-binds-a-variable-of-identity.yaml. These must keep routing as manifests, either by a key-set rule (no section key) or by splitting the files.
- The 17 files under examples/invalid named `*-manifest-*` route by name.

Not a vector, but it will flip: experiments/011-short-form-spike/tests/spike.rs asserts the 14 rejections, and it reads the live spec/ and reference-model crates, so it fails once `as` lands. Rerun the spike after batch 1 and record dated follow-up results showing the translation accepted.

## Build

All work is in Rust except tools/check_schemas.py. It splits into about four work orders after the spec order.

1. LOADER, reference-model/crates/loader:
   - new src/sectioned.rs, the translator, about 0.9 to 1.3k lines. It works over the positioned Value. It covers routing, SCT-002 versions, inline and referenced definitions, `consumes`, `operator`, the `conduits` section, the `fleet` key (deduplicated per project), `manages` expanded to qualified core templates, the name rules, and a translation map from each generated document to its SCT rule, file and authored pointer;
   - project.rs: kindless maps outside the manifest are routed to sectioned (about 50 to 100 lines);
   - diagnostic.rs: maps kernel diagnostics through the translation map, sets properties.translation and redacts secret places (about 150 to 250 lines);
   - lib.rs: exports;
   - tests: about 0.6 to 0.9k lines, including the examples.rs routing fix.
   `sectioned.v1` is read from spec/ like the other core schemas, so no hand-coded schema is needed.
2. CORE, reference-model/crates/core:
   - 0 lines for the form, since the core registers translated documents (as-if);
   - the NAM-024 comparison of releases at install (packages.rs and archives.rs), about 150 to 250 lines plus tests;
   - the four standard templates are spec documents, read as the core package is.
3. CONFORMANCE, reference-model/crates/conformance and conformance/case.schema.json: the `translate` step kind and the `equivalent` option on register-project, about 300 to 450 lines, plus the 65 to 75 cases and fixtures.
4. PACKAGES:
   - packages/agent-claude-code and packages/agent-codex move to 0.2.0. Each keeps `.v1` unchanged, as NAM-024 and NAM-025 require, and adds `claude-code.v2` / `codex.v2`: `accountServices` and `peerMessaging` {type: boolean, default: false}, `model` with default '', all three still required (MAN-017), with true descriptions. The Codex description states the collaboration route of finding 4-06 and the $CODEX_HOME/AGENTS.md leak of finding 3-07; how descriptions are surfaced is the descriptions area's.
   - Each package adds an exported Conduit template `turn` (its conversation.v1 capability with [#turn.v1]) and updates its exports, README and tests. The dist archives are rebuilt.
   - conformance/fixtures/agents/*-0.2.0 copies are added beside the 0.1.1 ones.
5. FIXTURES: conformance/fixtures/s1 gains filesystem 0.1.1 and 0.2.1 with the `read` and `read-write` templates (ruling 63 #3), built by the s1-archive builder, with digests in its README. The S1 rerun's T5 installs 0.2.1.
6. SHELL, reference-model/crates/shell, about 250 to 450 lines:
   - `conduit render`: framed YAML with each document's SCT rule;
   - `conduit init` writes a flow-style skeleton (ruling 63 #2) and never changes `apiVersion` afterwards;
   - CLI diagnostics printed as file:line:col with the rule and its SCT origin.
   Plan and apply, the named flags, break-before-make under stable names and `install` writing pins belong to the plan/apply order, which depends on this one.
7. TOOLS, tools/check_schemas.py (about 120 to 220 lines): validate sectioned documents against `sectioned.v1` (schema only; the SCT rules are tested by vectors), route by file role and key set, check the new example directories and the core exports.
8. DOCUMENTS, after the translator: docs/guides authoring examples in flow-style short form, and the rerun's S1 project (reference-model/examples/s1) rewritten in the short form. Both belong to the rerun-preparation order.

Work orders:
- one spec order, Codex, part of batch 1;
- WO-T, the loader translator, conformance features and vectors;
- WO-P, the packages, fixtures, NAM-024 check and Agent vectors;
- the shell items folded into the plan/apply order.
WO-T and WO-P run in parallel once the kernel orders for `as` and the Fleet kind land. Gate: every AGENTS.md check, `cargo test --release` and `clippy` in reference-model/ and both packages, and the experiment 011 rerun.

## Ruling points

WHERE EACH RULED POINT IS IMPLEMENTED
- Ruling 55 A1, the short form as language, with canonical documents as kernel, wire and stored form and legal beside it: SCT-001 to SCT-015 in spec/promises/sections.yaml, `sectioned.v1`, and the SUB-010, MAN-011, SUB-005 and SCH-018 amendments. register.v1, replace.v1 and inspect.v1 stay canonical, and SCT-001 rejects a sectioned value on the wire. The spike that A1's "what follows" required was run as experiment 011; its finding that `as` is needed first is the first dependency below.
- Ruling 55 A2, reuse through Conduit templates with names always fully qualified: SCT-007 form (b) (`from`) and SCT-009 form (b), the `templates.conduits` section (SCT-005), and NAM-018 unchanged. No grouping or aliasing anywhere.
- Ruling 55 A3, stable names:
  - SCT-004 makes translation deterministic and SCT-013 makes generated names pure functions of the authored keys, so a replacement under the same key keeps the name.
  - Break-before-make (retire, removal under LIF-035 or unregister, then register under the freed name, with tombstones lifted by root under A7) is the plan's job under ruling 53 #4. It goes in the CLI reference with golden cases that do not bind, and spec/README gains a limit stating that the meaning covers registration.
  - The windows without access are measured in the rerun (measurement plan).
- Ruling 55 A5, no defaults added to a published Type: the NAM-024 amendment plus an install-time check (D7), and the Agent packages at 0.2.0 shipping `claude-code.v2` and `codex.v2` beside unchanged `.v1`.
- Ruling 55 A6, amend rulings 8, 18, 31, 35 and 37 and keep 19:
  - the AUDIT "Later amendments" rows;
  - the wording of OBJ-012, OBJ-017, OBJ-020, OBJ-024, INS-001 and MAN-003;
  - SCT-006 (an inline definition makes N--t and N);
  - definitions.md rows.
  Keeping ruling 19 strikes the synthesis's bare-name rewriting: `to`, `consumer` and `target` name their kind. Fields with one fixed kind (`owner`, `placement`, `runtime`, the `fleet` value, `manages` keys) take bare names as OBJ-006 already allows. Experiment 011 already wrote kinds; in the documented flow style this costs width, not lines.
- Ruling 55 A4 and A7 are not this area's. The interplay: the SCT translation is what the plan compares against, and T7's `--reopen` relies on A7's lift.
- Ruling 63 #2, flow style: every new spec example is in flow style; the spec/README author path names flow as the documented style; `conduit init` writes a flow skeleton; block style stays legal (flow-block-identical vector); the guides are converted after the translator lands.
- Ruling 63 #3, packages ship standard Conduit templates: PKG-031 (should), the definition "standard Conduit template", the four core standard templates, the filesystem fixture's `read` and `read-write`, and the Agent packages' `turn`. A side effect: package templates are never registered (NAM-013), so two projects that both use `fs#read` cannot clash, as two local `templates.conduits.read` would under OBJ-002. Counting them as provider work is the measurement plan's.
- Ruling 63 #1, the counting rules, belongs to the measurement plan and is not specified here.
- Ruling 59 A6 and ruling 60, the `fleet:` key: SCT-011 (makes the Fleet from core#fleet, owns the file's Agents, other Agents state `owner:`).
- Ruling 66 point 1, `manages:`, and point 6, no Scope object: SCT-012, the four core templates, and the definition of "scope". Point 2, what a scope covers, is encoded by the templates' operation lists, and SCT-012 excludes `system`, `management` and `access`. "Shown back as a scope" relies on the inventory area showing each Conduit, after which the CLI renders. Points 3 and 4 belong to the pending-actions and access areas.
- Ruling 53 #9, the fresh author's format reference, is read as spec/ itself plus the CLI's command page (the spec/README author path).

FINDINGS FOR THE OWNER: nothing is unimplementable as ruled. Four points are staged or adjusted, and none needs a new ruling before batch 1:
1. Ruling 66 names "a Fleet, for Pools" as a holder, and its manage role covers "acting for its Fleets". Neither can exist in batch 1: a Fleet is not a consumer, and act.v1 does not exist before ruling 61 (batch 2). Batch 1 implements Interfaces, and Agents for their own Fleet. Batch 2 must decide how act.v1 joins the manage role without silently widening what `fleet-manage` means. The recommendation is a separate act template, or a bumped core version that the plan reports as a widening needing a flag. Already-open Conduits keep their old lists (INS-020).
2. The synthesis's SCT-010 (root's Conduit to each Pool, derived) is superseded by ruling 59 A3. Experiment 011's translator still emits it, so the loader's translator must not; `operator` on a Pool is rejected.
3. As written, NAM-024 binds package authors and has no engine observable (the manifest records it as uncovered, "Required before T5"). Making ruling 55 A5 testable takes the install-time check of D7, a rejection that only enforces what the ruling forbids. It is logged as a coordinator decision the owner can strike.
4. The ruling's sketch `manages: {fleets: [...], pools: [...]}` leaves how the role is spelled open. D1 chooses a map of name to role, with no default role. The alternatives are listed under changes, section G.

## Dependencies

MUST LAND BEFORE THIS AREA, in the same batch, ordered before this area's spec text and before WO-T:
1. Ruling 53's `as` on standalone Conduits: the OBJ-013 standard argument, IFC-012's local name, `instance.v1`'s Conduit branch, and IFC-014's uniqueness per consumer (synthesis F3). Experiment 011 found 0 of 14 states registering without it, and SCT-007, SCT-009, SCT-012 and SCT-014 all depend on it.
2. The Fleet family (rulings 59 and 60 group A):
   - `kind: Fleet`, the core template `core#fleet`, the Type `core#fleet.v1`, the capability `core#fleet-management.v1` (without act);
   - the Agent's required, fixed `owner`; Fleet in target-reference.v1 and in instance.v1's enums;
   - Pool and Fleet may not share a name, and `management`, `access` and `system` are reserved;
   - root's engine-made standing Conduits to every Pool and Fleet (which replace SCT-010), with `as` equal to the target's name;
   - the ACC-024 amendment;
   - `list.v1` on object-management, pool-management and fleet-management (listed by the standard templates).
3. The vocabulary rows (definitions.md), written first in the spec order, so that no SCT text uses "entry" or "grant" (synthesis risk 12).

NEEDED BY THE PLAN BUT NOT BY THE TRANSLATION, from other batch-1 areas:
- tombstones and A7's lift, and `expectedId` on open and close (stable-name replacement and `--reopen`);
- the inventory in inspect.v1, showing each Conduit (and its template), so that a scope can be shown back;
- descriptive tool descriptions (IFC-033), for the `.v2` descriptions.
Ruling 62 C1 (only root opens a Conduit whose consumer is root) is consistent with operator Conduits being opened by root at apply.

CROSS-AREA DECISIONS TO ALIGN:
- D9, the definition of "scope", is used by ruling 66 point 3's pending actions. Recommended: the Interface's open Conduits to Pools and Fleets, whichever template made them.
- OBJ-013 should be written once, by the `as`, Fleet and short-form areas together.

WHAT THIS AREA BLOCKS:
- WO-T and WO-P;
- the shell's plan, apply, render, init and install pins;
- the rewrite of the rerun's S1 project and the fresh author's pack (spec/ as the format reference);
- the measurement plan's flow-style count;
- the docs/guides conversion;
- A78's orders 1 to 3 written in the short form;
- the experiment 011 rerun.

DOES NOT FORECLOSE BATCH 2 (ruling 61):
- the `fleets:` section and `templates.fleets` are kept free for `holds:` and a Fleet's `manages: {pools}`;
- fleet-management.v1 can gain act.v1, which NAM-024 allows;
- the standard templates are named by role, not by their operations;
- the consumer kind Fleet is absent and the schema stays closed until then;
- the generated-name scheme leaves room for `F--holds--K`.
