Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Project document as the normative unit: one registered Project per conduit-project.yaml, defined by translation into today's documents, advanced by generations, and changed only through the explicit requests that a pure plan.v1 returns

## name
Project document as the normative unit: one registered Project per conduit-project.yaml, defined by translation into today's documents, advanced by generations, and changed only through the explicit requests that a pure plan.v1 returns

## thesis
THE PROPOSAL
- spec/ gains a second normative surface, the project document `conduit-project.yaml`. It works like the Compose Specification: concise, closed-keyed and normative.
  - It has sections for packages, pools, resources, agents, interfaces, templates and grants.
  - spec/ defines it by numbered translation rules (PRJ-*) into today's canonical documents.
- Those canonical documents stay the language's kernel:
  - they are unchanged and remain the wire and the stored form;
  - they are still the only form for contracts and packages (S7);
  - all of today's roughly 455 promises keep speaking about them alone.

HOW A PROJECT IS REGISTERED AND CHANGED
- The engine registers a project as a whole.
  - `register.v1` takes the document and a `basis` generation, expands the document itself, and registers only the objects the new generation needs. It never rewrites an existing object.
  - It records the generation in a new Project object, which outlives its Pool.
- A later revision is a new generation plus requests:
  - `plan.v1` is a pure core operation. It returns the exact ordered list of today's requests (register, set-up, open, close, retire, unregister), each carrying `expectedId`. It dispatches nothing.
  - The author, the CLI or an Agent sends those requests. Sending them is the approval. Nothing happens otherwise.

WHAT KEEPS IT SAFE
- Objects stay immutable. An edited entry is replaced make-before-break: the successor gets a derived name, and the grants are re-pointed under the same `as`.
- Every grant is a real standalone Conduit that names its capability and lists fully qualified operations (ruling 25, ruling 53 #8).
- Revocation is protected by engine-held tombstones:
  - they are keyed by the relationship (consumer, local name) and by the project key;
  - the engine classes each one when the Conduit is closed: either the project had already dropped or changed that key, or the close went against the project.

HOW THE SKEPTICS' OBJECTIONS ARE ANSWERED
- To C and D, the normative-file designs: there is no revise, no swap, no restart in place and no engine-side apply.
- To B, C and D: the plan that runs is the plan that was computed.
  - `register.v1` carries the basis and the plan's digest.
  - Every later request carries `expectedId`.
  - Nothing reopens, recreates, prunes, re-resolves a name or reverts a control without a named flag.

WHAT IT COSTS AND WHAT IT BUYS
- About 56 promise touches: 31 new and about 25 amended. About 11-14 of them are ruling 53's additions, which both routes need.
- About 5,000-8,500 lines of Rust and about 100-140 new vectors. Expectations change for about 5 existing cases, and 12 snapshot fixtures are regenerated.
- No promise that carries safety is weakened.
- S1 on paper: 28 lines against the baseline's 64, with every connection written (better on 6 tasks, the same on T3). With the optional derived operator grants it is 25.
- Against the layered package it is not better on lines (28 against 27). What it buys:
  - thesis 2 holds for what authors write, including change;
  - a second implementation must agree (A34);
  - Agents can plan through Conduits (preservation test 5);
  - revocation guards are enforced by the engine;
  - the fresh author learns the format from spec/ alone.
- It fails the bar under the strict reading (every command counts as a line, T5 compared with the better baseline), as every design so far does.

## grammar
FILE
- Name: `conduit-project.yaml`, `.yml` or `.json` (SCH-013). One YAML document, in the project's directory, recognised by its name, with no `kind`.
- A directory that holds one has no `conduit.yaml`: the project document takes the manifest's role.
- The only other top-level files allowed beside it are canonical contract documents. A canonical template or instance document beside it is rejected.
- The strict syntax rules apply unchanged: SCH-005, 006, 011, 012, 017, 020 and 021, and SUB-005's UTF-8 rule.
- Documented style: section keys in block style, and each entry and each grant on ONE line as a YAML flow map.

TOP-LEVEL KEYS (closed; any other key is rejected under OBJ-019)
- `conduit`: string, required. The specification version, `github.com/shrav-k/conduit/v0`. Written once; every derived document carries it as `apiVersion`.
- `project`: name, required. OBJ-001 grammar, never containing `--`. The Project's name, unique among Projects.
- `basis`: integer of at least 1, optional. WRITTEN BY THE TOOL after each generation is registered. It is the plan's stale check and has no meaning in the expansion. Absent means a new project.
- `description`: string, optional. Carried to the Project; no meaning.
- `packages`: map, optional. WRITTEN BY `conduit install` (ruling 53 #5). Each key is an address; each value is `{version: exact SemVer, digest: 'sha256:<64 hex>', fetch?: URI reference without user information}`.
  - The schema and rules are manifest.v1's: NAM-006, NAM-007, NAM-008, NAM-020, MAN-005, MAN-006 and PKG-006.
  - The core package is never listed.
- `address`: package address, optional. Required where the directory holds contracts or `implementations` is present (MAN-015, MAN-016). `version` and `exports` are not allowed: a project is not a package.
- `implementations`: map from name to implementation item, optional. The item schema and rules are manifest.v1's unchanged: MAN-009, MAN-010, MAN-017, MAN-018, SEC-021, PKG-019, PKG-020, PKG-030, IFC-043 and IFC-044.
- `templates`: optional. A map from family (`Interface`, `Agent`, `Resource`, `Pool`, `Conduit`) to a map from name to template spec.
  - The spec is exactly templates.yaml's `spec` for that family, so `inputs`, `type`, `config`, `implementation`, `controls`, `conduits`, `entries`, `capability`, `operations` and `requirements` are all available.
  - This is the reuse escape hatch, and every kernel template construct is reachable through it.
- `pools`, `resources`, `agents`, `interfaces`: optional, each a map from key to ENTRY. Default `{}`.
- `grants`: optional. A map from CONSUMER to a map from grant key to GRANT. Default `{}`.

KEYS
- Entry keys, template names and grant keys follow the OBJ-001 grammar and never contain `--`, which is reserved for derived names.
- Entry keys are unique across all four entry sections (NAM-027).
- Template names are unique per family and differ from entry keys.
- Grant keys are unique per consumer. A grant key is the Conduit's local name (`as`), so tool names are `<grant key>.<export>` and fit IFC-014's limit of 63+1+63.

ENTRY: oneOf two closed shapes
- (a) Inline definition: `{type, config?, implementation?, controls?, conduits?, entries?, owner?, placement?, runtime?, description?}`.
  - `type` is required for every family, with no default, Pools included (`github.com/shrav-k/conduit/core#pool.v1`). It is a qualified Type address, or `#name` for the project's own contracts.
  - `config` defaults to `{}`, and Type schema defaults fill it (SCH-009).
  - `conduits` is allowed for Interface, Agent and Resource only; `entries` for Pool only.
  - `inputs` are not allowed: an inline definition is a one-off.
- (b) Instance of a template: `{from, arguments?, owner?, placement?, runtime?, description?}`. `from` names a template in `templates`, or a qualified template exported by a package.
- Standard arguments:
  - `owner` is required for a Resource and allowed nowhere else. Its value is a pool key, or `{kind: Pool, name}` for a Pool outside the document, such as `system`.
  - `placement` (a node name) and `runtime` (a resource key or a reference) are allowed for Agents and Resources only.

GRANT: closed shape
- `{to, capability, operations, requirements?, description?}`.
  - `to`, `capability` and `operations` are required.
  - `capability` is a qualified Capability address; it is never inferred.
  - `operations` is a list of fully qualified operation addresses. It may be `[]`, and there is no wildcard.
  - `requirements` maps a name to `{requirement: qualified address, arguments?: object}`.
- CONSUMER key: an entry key from agents, resources or interfaces, or `Interface/<name>`, `Agent/<name>` or `Resource/<name>` for an existing object outside the document. Such a name may not equal an entry key.
- `to`: an entry key of any section (the section fixes the kind), or `{kind, name}` for an existing object outside the document. That includes `{kind: Resource, name: management}`, `{kind: Resource, name: access}`, outside Pools, Agents and Interfaces.
- A grant to `access` must hold a requirement of `github.com/shrav-k/conduit/core#bound.v1`.

REFERENCES INSIDE THE DOCUMENT
- Where a place's schema is a reference schema (OBJ-021), a reference is `{kind, name}`, or a bare name where the schema fixes the kind.
- A name that is an entry key in that kind's section means the project's current object for that key.
- Any other name means an existing object. It is resolved once to its ID (OBJ-009) and held by the Project.
- Secret references follow SEC-001 and SEC-002 unchanged.

DERIVED NAMES (the engine writes them; the author never does)
- The instance for key K is named `K` when K has never had an object in this project, and `K--<g>` otherwise.
- The inline template for K is `K--t<g>`.
- A project template T is `T`, and a later version of it is `T--t<g>`.
- A grant of consumer C under key L is the Conduit `C--L`, then `C--L--<g>`, with its template `C--L--t<g>`. C is the entry key, or the name part of `Kind/name`.
- g is the generation that makes the object.
- A derived name over 63 characters rejects the document, naming the key.
- A derived name already held by an object this project did not make rejects the generation. Nothing is ever adopted.

OPTIONAL D6, offered for decision 6 and not part of the base
- A top-level key `operator: {kind: Interface, name}`.
- It derives a grant from the operator to each pool key P, with local name P, capability `core#pool-management.v1`, and exactly the operations register, register-replacement, inspect, set-up, retire and reconcile. The list is fixed in spec/ for this `apiVersion`.
- It derives a grant from the operator to each agent key A, with local name A, listing every operation marked `startsModelWork` in the capabilities that A's Type offers at the pinned version.
- An explicit grant under the operator with the same key replaces the derived one.

## meaning
TWO NORMATIVE LEVELS, BOTH IN spec/

1. Expansion (language layer, PRJ-*). A project document at generation g means the canonical documents its expansion gives:
- for each inline entry, a template and an instance (`from`, plus the standard arguments);
- for each `from` entry, an instance;
- for each grant, a Conduit template {capability, operations, requirements} and a standalone Conduit {from, consumer, target, as};
- for the manifest, the projection {address, packages, implementations};
- `apiVersion` is the value of `conduit` on every document.

Every kernel promise applies to these documents as written, and none is restated. IN9 and V2 (rulings 8 and 18) hold: every instance still comes from a template. OBJ-002, OBJ-011, SUB-006, SUB-007, INS-020 and PKG-005 hold: nothing is rewritten.

The expansion has these properties:
- It is total and deterministic.
- It depends only on the text, on the pinned packages (immutable by digest), and, for derived names alone, on the Project's generation history.
- It never depends on the caller, the environment, the directory or the host. INS-018 stands: relative paths are rejected.
- The capability is never inferred, so the expansion never depends on what is stored. This removes C's D3 and the contradiction in its PRJ-011.

As-if rule (Haskell, Rust): an engine may expand any way it likes. The objects, records and diagnostics must equal those that registering the rendered canonical documents would give.

2. Plan (convention layer, PLN-*). What a new generation means for what already exists is the ordered list of requests that plan.v1 returns. That list is a pure function of:
- the document;
- the Project record;
- the objects that exist;
- the tombstones;
- the installed packages;
- what the caller may inspect;
- the flags named in the request.

The routes from one state to another (replacement, re-pointing, cascades on rotation or a provider swap, narrowing first) are therefore language meaning, not CLI behaviour. This answers the scenarios map's question 3: a cascade is part of the language's meaning.

THESIS 2 FOR WHAT AUTHORS WRITE
(1) `expand` vectors are input/output pairs, as in Dhall's A/B tests: a document gives canonical documents, or diagnostics carrying author pointers.
(2) `plan` vectors: (document, state, flags) gives a list of requests. Every skeptic counterexample in B, C, D and E becomes a case.
(3) An equivalence vector: S1-T1's project document registers the same transition as the rendered canonical S1 registered as `documents`. The comparison is modulo the Project object and the engine-owned `project` field. It is modelled on registration-orders (conformance/src/lib.rs:598-631).
(4) The expansion and the plan run in the pure core and emit Decisions that cite PRJ and PLN IDs. Each derived object holds engine-owned provenance {project, id, generation, key, rule} that no client can write (OBJ-004 extended). This answers C's D1, provenance forged by the client.
(5) Diagnostics speak in the author's text: a JSON Pointer into the project document, line and column where text was handed over, the PRJ rule, and the kernel promise that rejected. Values at secret-reference and `isSecret` places are redacted.

WHAT THE AUTHOR'S TEXT SHOWS, AND WHAT IT DOES NOT
- It shows every connection in full: the consumer (by section), the target, the capability, every operation fully qualified, the requirements and the local name.
- It shows every object's family, Type and config, every owner, and the pins (written by the tool).
- It does not show the derived object names, the IDs, the Type schema defaults or the live state. `plan` and `inspect` show those; `render` is plan.v1's `documents`.

RULINGS
- Not amended: 8 (IN9), 18 (V2), 21 (DR5), 25, 27 (NB: one file holds the pins, no lock file), 29 and 41.
- Amended in wording: 35 (the version is known per unit, stated once per project document); 31 and 37 (the project document has its own one-line frame; the canonical frame stays for the kernel).
- To be confirmed by the owner:
  - 22/DD2: registering a generation adds objects and a generation, and rewrites nothing;
  - round 6 D11: plan.v1 is a preview, not an apply of several.

## spec_plan
NEW, LANGUAGE LAYER: spec/promises/projects.yaml (17)
- PRJ-001 Form. The file names; one project document per directory and no manifest beside it; only contract documents beside it; closed top-level keys; strict syntax.
- PRJ-002 Version once. `conduit` is known per document, and every derived document carries it as `apiVersion`; an unknown version is rejected.
- PRJ-003 Manifest role. `packages`, `address` and `implementations` are the project's manifest, and MAN-003 to MAN-018, NAM-006 to NAM-008 and PKG-006 apply. SUB-008's `manifest` member becomes the RFC 8785 form of that projection only, so editing an entry or a grant never changes contract identity. This answers C's SUB-008 hazard.
- PRJ-004 Keys and derived names: grammar, no `--`, uniqueness, the derivation scheme, the 63-character limit and no adoption. Answers E #9 and D #4.
- PRJ-005 Entries. Their expansion into a template and an instance; the keys allowed per family; the owner required for Resources.
- PRJ-006 Project templates. The `templates` section; templates the project holds are changed only by a later generation, and `replace.v1` on one reports conflict. Answers E #4 and D #14.
- PRJ-007 Grants. Their expansion into a Conduit template and a standalone Conduit with `as`; explicit, fully qualified operations; empty allowed; no wildcard.
- PRJ-008 References outside the document. `Kind/name` keys and `{kind, name}` values are resolved once and held per generation, and never resolved again without `rebind`. Answers B #4, C D2 and D #12, with OBJ-010.
- PRJ-009 References to keys. They mean the project's current object for the key, and they make the dependency edges that PLN-002 uses.
- PRJ-010 A grant to `access` needs `bound.v1` (C D7). A grant to `management` gives a diagnostic that does not reject: "cannot be narrowed (ACC-030)".
- PRJ-011 A template exported by a package, used by an entry, may not hold a declared Conduit or an entry whose reference is fixed by name rather than given by an input (D #2).
- PRJ-012 Meaning by expansion. Total and deterministic, with its inputs closed; render; equivalence; the as-if rule.
- PRJ-013 Diagnostics at the author's text, with redaction (amends SUB-005's location).
- PRJ-014 A package holds no project document (the MAN-011 analogue).
- PRJ-015 The Project object.
  - Kind `Project`, a name unique among Projects, and an ID.
  - Generations are append-only. Each holds the document as registered with its RFC 8785 digest, the registrant, the invocation, the time, and a key map giving each key's object and status: current, pending (changed, consent missing) or detached (removed, not retired).
  - It has no lifecycle. It is unregistered only once no object it made still exists.
  - inspect.v1 returns it under ACC-033 row 7. It never lives in a Pool (D #5).
- PRJ-016 `register.v1` with `project`.
  - `basis` must equal the current generation; otherwise conflict, naming every later generation and its registrant (D #6, B #12).
  - The request also takes `plan` (the digest of plan.v1's register part) and the same flags as plan.v1. The engine recomputes the set and reports conflict if it differs, so the plan run is the plan approved (D #7, B #5).
  - It registers exactly that set, all or nothing, rewrites nothing, and sets up and opens nothing.
- PRJ-017 Engine-owned provenance `project` {name, id, generation, key, rule} on every object made. A document or request that carries it is rejected (OBJ-004).

NEW, CONVENTION LAYER: spec/conventions/plans.yaml (12)
- PLN-001 plan.v1.
  - It belongs to object-management.v1. Its inputs are `project`, `reopen`, `recreate`, `retire`, `replace` and `rebind`, each flag a list of key pointers.
  - Its outputs are `generation`, `requests`, `documents`, `tools` (before and after, per consumer), `coverage`, `intervals`, `diagnostics` and `digest`.
  - It is pure and dispatches nothing. It is authorized against `management`.
  - It reads only what the caller may inspect under rows 6 and 7. Every other part gets the first-tier refusal of ruling 52.1 (D #8, B #9).
- PLN-002 How an entry changes.
  - Added: register, then set-up.
  - Changed: make-before-break replacement. An entry is changed when any of these holds:
    - its text changed, `placement` and `runtime` included (D #13);
    - the pin changed of a package that its own text names (D #15, B #8, C's T5 Agent restart);
    - a key it references was replaced;
    - the project template it uses changed;
    - it is named in `rebind` (B #10).
  - Removed: it becomes detached and is reported; it is retired only when `retire` names it.
- PLN-003 How a grant changes.
  - Added: register, then open.
  - Changed (its text, or its target or consumer was replaced): close the predecessor, then open the successor under the same `as`.
  - Removed: close.
  - A change of a pin alone never changes a grant.
  - A grant whose consumer is root, changed or removed, is rejected before any part, citing ACC-024. It is re-pointed only by retiring its target (CON-008). Answers C M2 and B #6.
- PLN-004 Order.
  1. Register the generation.
  2. Every narrowing before any widening (D #10).
  3. A consumer's grants are opened before its setup; a target's setup comes before grants to it are opened (D #21).
  4. A replacement runs: set up the successor; close every grant to the predecessor; wait until no invocation through them is in flight; open the successors; retire the predecessor (E #7).
  5. Retirements that `retire` consented to.
  6. Unregister the templates the project holds that have no instance left.
- PLN-005 Preconditions.
  - Every request names its object by reference (LIF-031) and carries `expectedId`: the ID read when planning, or the ID the register result gives for that key.
  - The register request carries `basis` and `plan`.
- PLN-006 Tombstones in plans.
  - A tombstone of class "as the project" is superseded by the project's own later object for that key.
  - A tombstone of class "against the project" makes the part a conflict unless `reopen` (for Conduits) or `recreate` (for instances) names the key. The plan then puts that tombstone in `supersedes`.
  - Answers C M3, D #1, D #5, B #2 and the critic's gap 2.
- PLN-007 Consent through `replace`. It is needed to replace:
  - an Agent, whose home and sign-in are lost (ruling 49);
  - an instance whose Type requires `retire-hook.v1`;
  - an instance whose version change alters a granted operation's contract, the implementation's tools map, a declared Conduit, or the resolved config by a value not written in the document (D #3, E #10, C M9);
  - a target to which Conduits from outside the project point.
- PLN-008 Controls (the critic's gap 5, B #1).
  - A change confined to pointers that a selected control `changes` is sent as that control, with the revision as read.
  - A value a control set outside the project is reported as drift and never reverted, unless the document changes that value.
- PLN-009 Unknown and in-flight work.
  - A part is a conflict where an invocation of unknown outcome passes through a Conduit the plan would close, or where an earlier plan's request ended unknown.
  - Idempotency keys come from (project, key, action, expectedId). Answers E #11, D #16 and C D4.
- PLN-010 Narrowing is never withheld.
  - A conflict or refused part is reported and skipped; closes and narrowings still go (D #11, E #12).
  - A replacement with any refused part is withheld whole. Example: a successor's root-only get-secret grant planned for a caller that is not root (C M1, SEC-008).
- PLN-011 What a plan reports:
  - tools before and after, per consumer;
  - every Conduit and bound that comes to cover a new Resource (D #4);
  - every Conduit a retirement will close, including those made outside the project;
  - each interval without access;
  - pending sign-ins;
  - the resolved config, with defaulted values marked;
  - contract differences;
  - redaction at secret places.
- PLN-012 Nothing else. Requests take effect only when the caller sends them. A partly sent plan is resumed by planning again, from state. OBS-004 and LIF-008 are unchanged. plan.v1 is not an apply (round 6 D11).

NEW: TOMBSTONES, RULING 53'S ADDITION WRITTEN TO KNOW ABOUT PROJECTS (2)
- LIF-038 What a tombstone holds.
  - Closing a Conduit makes one keyed by (consumer ID, local name), plus (project ID, key) for a project's object.
  - Retiring an instance makes one keyed by (kind, name), plus (project ID, key).
  - Each records the actor, the invocation, the time, the cause, and the class the engine decides at that moment: "as the project" where the current generation no longer maps the key to that object; otherwise "against the project".
  - Tombstones are kept apart from records and survive REC-004.
  - Callers and keys are disclosed only to root and to the closer (the critic's gap 8). Retention is implementation-defined (a DL item).
- LIF-039 Enforcement.
  - `register.v1` and `open.v1` report conflict for an object that a tombstone matches, unless:
    - the request's `supersedes` names that tombstone; or
    - the object is the project's current object for the tombstone's key and the tombstone is "as the project".
  - Superseding is recorded with the request.
  - This binds Agents that bypass the CLI (the critic's gap 2), and replaced consumers through the project key.

AMENDED FOR THE OTHER FOUR ADDITIONS OF RULING 53 (8)
- OBJ-013: `as` becomes a standard argument of a standalone Conduit.
- IFC-012: the local name is `as`, or else the Conduit's own name.
- IFC-014: a successor for the same project key may be registered while its predecessor is not closed; the clash is then checked at open.
- LIF-037: `open.v1` and `close.v1` take `expectedId`.
- LIF-013 and inspect.v1's outputSchema: the inventory of Conduits an object consumes and is the target of, listing only what the caller may inspect and giving no counts; its tombstones; the generations of a Project.
- IFC-033: a tool's description names its target and that target's Type description.
- OBJ-018: `description` gains this meaning on the wire.
- ACC-033: the sentence on what the inventory discloses; plan.v1 and registration with `project` under rows 2 and 7, with row 1 still applying per Resource; inspecting a Project under row 7.

AMENDED FOR THE PROJECT (17)
- OBJ-002 (Project names), OBJ-003 (derived names), OBJ-004 (the `project` field), OBJ-017 and OBJ-024 (the version per project document), OBJ-020 (the kind Project).
- NAM-027.
- MAN-001, MAN-002 and MAN-011 (the directory rule).
- SUB-001, SUB-004, SUB-005, SUB-008, SUB-009 and SUB-010.
- The description of `register.v1`.

SCHEMAS AND THE CORE PACKAGE
- New spec/schemas/projects.yaml, holding project-document.v1, project-entry.v1 and grant.v1. They `$ref` the existing template-spec schemas, so every construct S2 to S10 needs is reached automatically.
- spec/core/operations.yaml:
  - plan.v1 is new;
  - `register.v1` gains `project`, `basis`, `plan`, the flags and `supersedes`;
  - `open.v1` gains `expectedId` and `supersedes`;
  - `close.v1` gains `expectedId`, and its result gains the tombstone;
  - `inspect.v1` gains the inventory, tombstones and the Project.
- capabilities.yaml: object-management.v1 adds plan.v1 (NAM-024 allows the addition).
- templates.yaml: root-management lists plan.v1.
- results.yaml: object.v1 gains `project`; plan-result.v1 and tombstone.v1 are new.
- references.yaml: project-reference.v1 is new.
- The core moves from 0.1.0 to 0.2.0.

TOTALS
- 31 new and about 25 amended: about 56 touches, about 12% of the promises.
- The optional D6 adds PRJ-018 (operator) and PRJ-019 (the derived operator grants, with their fixed lists): 2 promises and 4-6 vectors.

DELETED: NOTHING
- Declared Conduits, `replace.v1` and `register-replacement.v1` stay. S2, S4 and S10 use them, and removing them invalidates 141 of 367 vectors.
- A project's own objects simply never use `replace.v1` or `register-replacement.v1`. Dropping them is left to v1, each by its own ruling.
- The five additions are written once, in this form that knows about projects, as ruling 53 asked.

PHASING (an option)
- A: PRJ-001 to 017, LIF-038 and 039, and the additions: about 44 touches. This makes setup normative, while a non-binding CLI planner uses `register.v1` with `project`.
- B: PLN-001 to 012, which makes change normative.

## s1_rewrite
COUNTING RULES
- Lines marked TOOL are written by `conduit install` (pins) or by `conduit apply` (`basis`). They are not counted.
- These are assumed, as in the synthesis:
  - `conduit init` recorded the store beside the project, so no command takes `--store`;
  - the Agent packages are 0.1.2, with the reach flags defaulting to false (ruling 53 #7);
  - the five additions exist.
- The style is the documented one: one line per entry and per grant.
- The scratch states are under <scratch>/projdoc/ (t1.yaml, t2.yaml, t4.yaml to t7-restored.yaml). They were counted with diff.

=== T1: 18 file lines + 1 command line = 19 ===
Commands:
```
conduit install <lab>/pkgs/filesystem-0.1.0.tgz <lab>/pkgs/agent-claude-code-0.1.2.tgz <lab>/pkgs/agent-codex-0.1.2.tgz   [1 line]
conduit apply                                                                                                         [no value]
```
`conduit-project.yaml` after T1:
```yaml
conduit: github.com/shrav-k/conduit/v0
project: s1
basis: 1                                                                       # TOOL (written by apply)
packages:                                                                      # TOOL
  example.org/s1/filesystem: {version: 0.1.0, digest: 'sha256:200de7c1...'}    # TOOL
  example.org/conduit/agent-claude-code: {version: 0.1.2, digest: 'sha256:...'} # TOOL
  example.org/conduit/agent-codex: {version: 0.1.2, digest: 'sha256:...'}       # TOOL
pools:
  s1: {type: github.com/shrav-k/conduit/core#pool.v1}
resources:
  shared: {owner: s1, type: example.org/s1/filesystem#local-directory.v1, config: {root: <lab>/shared}}
agents:
  agent-a: {type: example.org/conduit/agent-claude-code#claude-code.v1}
  agent-b: {type: example.org/conduit/agent-codex#codex.v1}
grants:
  Interface/root:
    s1: {to: s1, capability: github.com/shrav-k/conduit/core#pool-management.v1, operations: [github.com/shrav-k/conduit/core#register.v1, github.com/shrav-k/conduit/core#register-replacement.v1, github.com/shrav-k/conduit/core#inspect.v1, github.com/shrav-k/conduit/core#set-up.v1, github.com/shrav-k/conduit/core#retire.v1, github.com/shrav-k/conduit/core#reconcile.v1]}
    agent-a: {to: agent-a, capability: example.org/conduit/agent-claude-code#conversation.v1, operations: [example.org/conduit/agent-claude-code#turn.v1]}
    agent-b: {to: agent-b, capability: example.org/conduit/agent-codex#conversation.v1, operations: [example.org/conduit/agent-codex#turn.v1]}
  agent-a:
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
  agent-b:
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
What apply prints (not authored, abridged), as the plan of generation 0 to 1, from Interface root:
- Step 1: `management.register.v1 {project, basis: none, plan: sha256:...}` registers 18 documents. Shown by `render`, for example:
  - Resource template `shared--t1` {type, config} and instance `shared` {from: shared--t1, owner: s1};
  - Conduit template `agent-a--project--t1` {capability, operations} and Conduit `agent-a--project` {consumer: Agent agent-a, target: Resource shared, as: project};
  - `root--s1` (as s1), `root--agent-a`, `root--agent-b`.
- Steps 2-10:
  - `management.set-up.v1` Pool s1;
  - `access.open.v1` root--s1;
  - `s1.set-up.v1` shared;
  - open agent-a--project and agent-b--project;
  - set up agent-a and agent-b;
  - open root--agent-a and root--agent-b.
  - Each carries `expectedId` from step 1.
- Tools after:
  - agent-a: project.file.read.v1, project.directory.list.v1;
  - agent-b: the same, plus project.file.write.v1;
  - root: s1.* (6), agent-a.turn.v1, agent-b.turn.v1.
- Coverage: Resource shared is covered by root--s1, and by no bound.
- The resolved config shows accountServices=false, peerMessaging=false and model='' as Type defaults.

=== T2: 3 file lines + 1 command line = 4 ===
```
conduit install <lab>/pkgs/history-0.1.0.tgz      [1 line; TOOL adds example.org/s1/history: {version: 0.1.0, digest: ...}]
conduit apply                                     [no value]
```
Lines added:
```yaml
  history: {owner: s1, type: example.org/s1/history#file-history.v1, config: {root: <lab>/history}}
    history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
    history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
The first goes under `resources`, the second under `grants.agent-a`, the third under `grants.agent-b`.
- Plan, generation 1 to 2: register history and the 2 grants (row 1: management plus root--s1, both listing register.v1); set-up history through s1; open agent-a--history and agent-b--history.
- With the IFC-033 addition, each history tool's description carries file-history.v1's posting convention.

=== T3: 0 lines ===
```
conduit reach      [no value]
```
- It reads through inspect.v1: the Project's key map and each Agent's inventory.
- Reach around the engine comes from the Types' own statements, which is provider work.

=== T4: 2 file lines ===
The two lines changed:
```yaml
  agent-a:
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
  agent-b:
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
```
```
conduit apply      [no value]
```
Plan, generation 2 to 3, narrowing first:
1. register agent-b--project--3 [read, list] and agent-a--project--3 [read, list, write], both `as project` and not opened (IFC-014 as amended);
2. close agent-b--project (expectedId); tombstone "as the project";
3. open agent-b--project--3;
4. close agent-a--project;
5. open agent-a--project--3;
6-7. unregister agent-a--project--t1 and agent-b--project--t1.
At no moment can both Agents write.

=== T5: 0 file lines + 1 command line = 1 ===
```
conduit install <lab>/pkgs/filesystem-0.2.0.tgz   [1 line; TOOL rewrites the pin to 0.2.0 and its digest]
conduit apply                                     [no value]
```
Plan, generation 3 to 4.
- Resource `shared` changed: its type names example.org/s1/filesystem, whose pin moved from 0.1.0 to 0.2.0.
  - Granted contracts unchanged; tools map unchanged; resolved config unchanged; node inherited. So no consent is needed.
  - It is replaced make-before-break by `shared--4`.
- Unchanged:
  - `history`, which names only example.org/s1/history;
  - its grants, since a pin change alone never changes a grant;
  - both Agents.
- Steps:
  1. register shared--4, agent-a--project--4 and agent-b--project--4;
  2. s1.set-up.v1 shared--4;
  3-4. close both project grants; wait until none is in flight;
  5-6. open both successors;
  7. s1.retire.v1 shared;
  8-10. unregister three templates.
- No Agent gains a tool: 0.2.0 maps neither append_file nor copy_file.

=== T6: 1 file line ===
The line changed:
```yaml
  shared: {owner: s1, type: example.org/s1/filesystem#local-directory.v1, config: {root: <lab>/shared-2}}
```
```
conduit apply      [no value]
```
- It is the same replacement as T5: `shared--5` is made, and both grants are re-pointed under the unchanged `project.*` names.
- The descriptions now name shared--5, so IFC-038 tells each Agent that its list changed.

=== T7: 1 file line (removed, then restored: 2 edits, counted once as the baseline counted its T7) ===
Revoke by deleting this line under `grants.agent-a`:
```yaml
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
```
conduit apply      [no value]
```
- Plan: register generation 6, which holds no new objects; close agent-a--project--5 (expectedId), leaving a tombstone "as the project"; unregister its template.
- Checking B is not counted.

Restore by putting the same line back:
```
conduit apply      [no value]
```
- Plan: register generation 7 with agent-a--project--7 `as project`.
- The tombstone is "as the project" and the Conduit is the project's own object for that key, so LIF-039 lets it through with no flag. Then open it.
- Command route, with the file left alone: `conduit close agent-a/project`, then `conduit apply --reopen agent-a/project`. That is 2 command lines with values, equal to the baseline. The tombstone is then "against the project", so the reopen needs the flag.

The final file has 21 authored lines and is identical to the file after T6.

OPTION D6 (if decision 6 is accepted)
Replace the 4 lines from `Interface/root:` down to its `agent-b:` grant with one line:
```yaml
operator: {kind: Interface, name: root}
```
T1 becomes 15 file lines + 1 command line = 16. No other task changes.

## s1_counts
PRIMARY READING (the measurement plan's own)
Non-blank authored lines, comments and TOOL lines excluded, each line once per task, plus each command line that carries a value the author chose.

| Task | File | Commands with values | Total | Baseline A70 (better) | vs A70 | Layered package | vs layered | Coordinator |
|---|---|---|---|---|---|---|---|---|
| T1 | 18 | 1 | 19 | 33 | less, -14 | 16 | +3 | 154 |
| T2 | 3 | 1 | 4 | 16 | less, -12 | 6 | -2 | 73 |
| T3 | 0 | 0 | 0 | 0 | same | 0 | same | 0 |
| T4 | 2 | 0 | 2 | 8 (about 8) | less, -6 | 2 | same | 2 |
| T5 | 0 | 1 | 1 | 3 (2) | less, -2 (-1) | 1 | same | 62 |
| T6 | 1 | 0 | 1 | 2 | less, -1 | 1 | same | 29 |
| T7 | 1 | 0 | 1 | 2 | less, -1 | 1 | same | 12 |
| All | 25 | 3 | 28 | 64 | less on 6, same on T3, much more on none | 27 | +1 | 332 |

- Under this reading S1 passes the owner's bar on paper.
- With option D6 the row reads (16, 4, 0, 2, 1, 1, 1) = 25, which beats the layered package on T2 and equals it elsewhere.
- T1 is 3 lines above the layered package for one reason: root's three grants are written out (4 lines with their key), not derived. Decision 6 is still open.

SENSITIVITY

| Reading | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All | Bar |
|---|---|---|---|---|---|---|---|---|---|
| Block style for pools, resources and agents (key, owner, type, config on their own lines); grants still one line each | 25 | 7 | 0 | 2 | 1 | 1 | 1 | 37 | pass, less on 6 |
| Block style for grants as well (4 lines each) | 40 | 13 | 0 | 2 | 1 | 1 | 4 | 61 | FAILS: T7 is double the baseline. One-line grants must be the documented form |
| Strict: every command is a line | 20 | 5 | 1 | 3 | 2 | 2 | 3 | 36 | Against A70, less on T1, T2, T4 and T5 (4 of 7). Against the better T5 baseline (2), T5 ties, leaving 3 of 7: FAILS "most", like every design so far |
| Strict, with `install --apply` as one command and T7 by the command route | 19 | 4 | 1 | 3 | 1 | 2 | 2 | 32 | Less on T1, T2, T4 and T5 (4 of 7): passes narrowly |
| JSON, one key per line | about 60 | about 15 | 0 | 2 | 1 | 1 | 1 | about 80 | FAILS on T1. YAML must be the documented form |
| Agent packages left at 0.1.1, with required flags written as `config: {accountServices: false, peerMessaging: false}` on each agent's line | 19 | 4 | 0 | 2 | 1 | 1 | 1 | 28 | no change: the flags take width, not lines |
| Ruling 49's expected account written on each agent's line | 19 | ... | | | | | | 28 | no change |
| `conduit init <store>` counted as a line | 20 | | | | | | | 29 | pass |

OTHER MEASURES (on paper)
- Author commands: 11.
  - T1: install, apply. T2: install, apply. T3: reach. T4: apply. T5: install, apply. T6: apply. T7: apply twice.
  - The coordinator typed 35.
- Engine requests sent by the tool: about 61, every one ordinary and recorded.
  - T1 14, T2 6, T3 3, T4 8, T5 12, T6 11, T7 7.
  - The plan.v1 calls are included.
- Grants made by hand: agents' grants 2 in T1, 2 in T2, 2 edited in T4, 1 in T7. Root's grants are listed apart: 3 in T1.
- Lines naming a contract address: 9 at T1, 12 at the end, against the coordinator's 45.
- Digests typed: 0. Provider-specific lines: 0.
- Duplicated values at the end: about 6, against the baseline's 8. They are the read-write operation list (3 times) and the capability `directory.v1` (4 times).
- Tool names changed by any task: 0.
- Concepts met: about 14 to 15, against about 16 today and the baseline's 20. They are: project document, Pool, Resource, Agent, the root Interface, grant/Conduit, local name, capability, operation, Type, package pin, install, apply/plan, reach, and generation/basis, which the output shows.
- Intervals without access:
  - T4: each Agent between its close and its open, never both writing at once;
  - T5 and T6: both Agents between the close and the open of the successor.
  - At today's 9-17 s per request this may be 10-30 s rather than seconds. The coordinator's T5 had no such interval, because it renamed the tools.
  - T7's interval is intended.
- Longest line: 372 characters, root's Pool grant.
- All of these are paper counts, which ruling 45 does not accept as evidence. The margins on T5 to T7 are 1 line.

## other_scenarios
Every construct of S2 to S10 that the kernel has is reachable. Inline entries carry the full template spec, `templates` carries reusable templates with inputs, and grants carry requirements. What the kernel itself lacks cannot be written in any form. Sketches are abridged where marked: "6 ops" stands for the fully qualified list.

S2: Linear, with its key in 1Password (about 30 lines)
```yaml
pools: {tools: {type: github.com/shrav-k/conduit/core#pool.v1}, vault: {type: github.com/shrav-k/conduit/core#pool.v1}}
resources:
  op-token: {owner: vault, type: example.com/keychain#keychain-secret.v1, config: {service: conduit-op}}
  linear-key: {owner: vault, type: example.com/onepassword#op-secret.v1, config: {locator: 'op://test/linear/key', token: {kind: Resource, name: op-token}}}
  linear: {owner: tools, type: example.com/linear#linear.v1, config: {token: {kind: Resource, name: linear-key}}}
  adapter: {owner: tools, type: example.com/linear#team-adapter.v1, conduits: {upstream: {from: upstream, target: {kind: Resource, name: linear}}}}
templates: {Conduit: {upstream: {capability: example.com/linear#issues.v1, operations: [6 ops]}}}
agents: {reader: {type: ...claude-code.v1}, writer: {type: ...codex.v1}}
grants:
  Interface/root: {tools: {...6 ops}, vault: {...}, reader: {...turn}, writer: {...turn}}
  linear:     {key:   {to: linear-key, capability: github.com/shrav-k/conduit/core#secret-provider.v1, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}}
  linear-key: {token: {to: op-token,   capability: github.com/shrav-k/conduit/core#secret-provider.v1, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}}
  reader: {linear: {to: adapter, capability: example.com/linear#issues.v1, operations: [list, get, search fq], requirements: {team: {requirement: example.com/linear#team.v1, arguments: {team: ENG}}}}}
  writer:
    linear: {to: adapter, capability: example.com/linear#issues.v1, operations: [6 fq]}
    delegate: {to: {kind: Resource, name: access}, capability: github.com/shrav-k/conduit/core#access-control.v1, operations: [github.com/shrav-k/conduit/core#open.v1], requirements: {bound: {requirement: github.com/shrav-k/conduit/core#bound.v1, arguments: {targets: [{kind: Resource, name: adapter}], operations: [example.com/linear#issues.list.v1]}}}}
```
- The plan returns the get-secret opens only for root (SEC-008). For any other caller, every replacement that contains one is withheld (PLN-010).
- E13, rotating the key, is one line: a new locator. The plan then cascades by PLN-002's dependency edges:
  - linear-key is replaced;
  - linear refers to it, so it is replaced;
  - adapter's declared Conduit targets linear, so adapter is replaced;
  - reader's and writer's grants, and writer's bound (whose `targets` names adapter), are re-pointed.
  - Every step and every interval is listed. The cascade is language meaning.
- The expected account (ruling 49) goes on each agent's line once it is a Type config key.
- The adapter's `supports` is provider work (kind B).

S3: two machines (managed mode)
```yaml
resources:
  tailnet: {owner: {kind: Pool, name: system}, type: example.com/tailscale#tailnet.v1, config: {...}}
  m2-docker: {owner: s1, type: example.com/docker#docker-runtime.v1, config: {context: m2}, placement: local}
agents: {agent-b: {type: ...codex.v1, placement: m2, runtime: m2-docker}}
grants:
  Interface/root: {system: {to: {kind: Pool, name: system}, capability: ...pool-management.v1, operations: [register, set-up, retire, inspect, reconcile fq]}}
  agent-b: {project: {to: shared, capability: ..., operations: [...], requirements: {host: {requirement: github.com/shrav-k/conduit/core#shared-host.v1}}}}
```
- `nodeSelector` stays in Types and items (kind B).
- A replacement inherits its predecessor's resolved node (E #6).
- Enrollment and declare-lost are requests, not text. Partitions are behaviour.
- CANNOT EXPRESS: peer mode (ruling 51), meaning an Interface that stands for another engine's actor, or a trust domain. No promise exists yet. Once ruling 51 is written, it would be an `interfaces` entry of that Type.

S4: a partial setup and its retry
- A Pool written inline with `entries` (dependency references `{entry: name}`, `optional`) and placement or runtime given through inputs from the `templates` section.
- Hooks through the Type's `requires` (kind B).
- The operator's Pool grant lists reconcile.v1 explicitly.
- The retry is `conduit apply` on the unchanged document: PLN returns set-up for every instance of the project that is registered and not ready, never on its own.
- Unknown parts block the plan until a reconcile request is sent (PLN-009).
- An entry whose Resource was retired needs `--recreate`, sent as `register-replacement.v1` with `supersedes`.

S5: a control under contention
```yaml
resources:
  docs: {owner: s1, type: example.com/store#store.v1, config: {limit: 40}, controls: {example.com/store#set-read-limit.v1: {inputs: {limit: {schema: {maximum: 50}}}}}}
grants:
  Interface/root: {docs: {to: docs, capability: example.com/store#admin.v1, operations: [example.com/store#set-read-limit.v1]}}
  agent-a: {limit: {to: docs, capability: example.com/store#admin.v1, operations: [example.com/store#set-read-limit.v1]}}
```
- Editing `limit` to 45 is sent as the control with the revision as read (PLN-008). REV-002 settles the race.
- A value agent-a set by the control is drift: it is shown, and never reverted unless the document changes it.

S6: a revocation and a retirement in flight
- Deleting A's grant line closes it with expectedId (ACC-034).
- Removing R's entry leaves R detached until `--retire r`. The plan lists every Conduit the retirement closes, including those outside the project (CON-008, ACC-039). R's retire hook needs `--replace` or `--retire` consent (PLN-007).
- A stale working copy gets a conflict on its basis.
- An Agent that opens or registers a Conduit `as project` for A gets a conflict under LIF-039 unless it names the tombstone.

S7: an outside author packages a server and a plugin
- Contracts, manifest, implementation items, `nodeSelector` and exported templates stay canonical (kind B, unchanged).
- Projects consume them through `from: example.com/pkg#template` entries. PRJ-011 rejects package templates whose references are fixed by name.
- This leaves two normative syntaxes, deliberately: the project document for project authors, the kernel for package and provider writers.

S9: first start, later start, explicit repair
- First-start configuration (trust domain, hostname, runner list, store) stays outside documents.
- Providers are entries with `owner: {kind: Pool, name: system}` and an explicit root grant to `system`.
- A later start changes no document.
- Repair is `apply` on the unchanged document: set-up of instances that are not ready, unknowns listed for reconcile.
- declare-lost is a request. Instances it retires are tombstoned "against the project", so recreating them needs `--recreate` plus a new `placement`.

S10: every provider swapped
- Editing `runtime:` on the entries, or the store Type and locator of the secret entries, makes each changed entry a replacement and cascades through the references.
- Agents need `--replace agent-a,agent-b`, one command line. The plan reports a sign-in pending at root for each (rulings 49 and 50).
- A Git package source is an entry owned by `system`, plus `fetch` in the pins, which the tool writes.
- Docker requires `package` OCI items (provider work, D16).
- The cascade is normative (PLN-002), so thesis 2 covers it.

THE OWNER'S REAL USE: 10 WORKERS (about 85 lines, against about 440-520 today and a floor of about 25)
```yaml
templates:
  Conduit: {tree-rw: {capability: ex.org/git#worktree.v1, operations: [read, list, write fq]}, main-ro: {...}, orders-own: {capability: ex.org/queue#queue.v1, operations: [read, post fq], inputs: {worker: {schema: {type: string}, to: [/requirements/own/arguments/assignee]}}, requirements: {own: {requirement: ex.org/queue#assignee.v1}}}}
  Resource: {worktree: {type: ex.org/git#worktree.v1, inputs: {branch: {schema: {type: string, minLength: 1}, to: [/config/branch]}}, config: {repo: /home/me/conduit}}}
  Agent: {codex-worker: {type: ...codex.v1, inputs: {tree: {schema: {$ref: github.com/shrav-k/conduit/core#resource-reference.v1}, to: [/conduits/tree/target]}, worker: {schema: {type: string}, to: [/conduits/orders/arguments/worker]}}, conduits: {tree: {from: tree-rw}, main: {from: main-ro, target: {kind: Resource, name: main}}, orders: {from: orders-own, target: {kind: Resource, name: orders}}}}, claude-worker: {...}}
resources: {main: {...}, orders: {...}, tree-101: {owner: dev, from: worktree, arguments: {branch: wo/101}}, ...}
agents: {admin: {...}, verifier: {...}, worker-101: {from: codex-worker, arguments: {tree: {kind: Resource, name: tree-101}, worker: worker-101}, placement: wsl}, ...}
grants:
  admin: {main: {...}, orders: {...}, worker-101: {to: worker-101, ...turn}, tree-101: {to: tree-101, ...read, list}, ...}
  verifier: {tree-101: {...read}, ...}
```
- Per worker: 5 lines. Ending an order is `--retire worker-101,tree-101`, one command line.
- CANNOT EXPRESS, because of the kernel and not this form:
  - an Agent and its own Resource in one template;
  - "admin may read every tree" as one line (CON-001 makes N grants);
  - for_each or names derived from the key (IN5): `worker: worker-101` is repeated by hand;
  - a Type as an input (IN2): two worker templates;
  - mixed Agent package versions within one project (NAM-006): a second project is the escape;
  - peer engines.
- An Agent package upgrade at scale is `--replace` for 10 keys, with 10 pending sign-ins.
- The admin creating workers itself needs `management` (plan and register: broad trust under ACC-030) or a broker (D04).

## change_model
HOW A CHANGE IS EXPRESSED
- The author edits the project document. The document always states the whole desired project.
- The direct requests stay available for emergencies: `conduit close <consumer>/<grant>` and `conduit open`, by key, with expectedId. They never edit the document, and the next plan reports their effect as a tombstone conflict, never reverting it.

HOW A CHANGE IS COMPUTED
- `plan.v1` runs in the pure core and is normative (PLN-001 to 012).
- Its inputs: the document (its `basis` is the stale check) and the flags `reopen`, `recreate`, `retire`, `replace` and `rebind`.
- Its output: the ordered requests with their preconditions, the canonical documents, tools before and after for each consumer, coverage, intervals without access, consent needed, and diagnostics.
- It dispatches nothing, which makes it CloudFormation's create-change-set, not an apply.

HOW A CHANGE IS EXECUTED
- The caller sends the returned requests in order. Each is today's request, authorized, recorded and carrying expectedId:
  - `register.v1` with `project`, `basis`, `plan` and the flags;
  - set-up, open, close, retire and unregister.
- `conduit apply` runs plan.v1, prints the plan, sends the requests, reports each step, and exits non-zero unless every one succeeded (4-13). It stops at the first request that fails or ends unknown.
- Resuming means planning again: the plan is computed from state, and conflicts are reported.
- `conduit plan [-o FILE]` is for review only. `conduit apply --plan FILE` sends a saved plan, and any drift since gives a conflict.

APPROVAL (open decision 2)
- Sending the requests is the approval. There is no y/N prompt, so it works for a non-interactive fresh author and for Agents alike (the critic's gap 1).
- Every widening the author wrote is applied as written.
- The dangerous acts each need a flag, whose value counts as a command line:
  - reopening what was closed against the project;
  - recreating what was retired against the project;
  - retiring an entry removed from the document;
  - replacing an Agent, an instance with a retire hook, or anything a version change widens;
  - resolving an outside name again.
- This is consistent with ruling 53 #4: apply runs only on request, never automatically, and never prunes or reopens without a flag.

THE CHANGE CLASSES FOR T4 TO T7
- T4, a grant's text changed: register the successor, close the predecessor, open the successor; narrowing first.
- T5, a pin of the package the entry's type names: make-before-break replacement, then re-point.
- T6, config changed: the same replacement.
- T7, a line removed: close, with a tombstone "as the project". The line restored: register and open, with no flag.

CONCURRENCY AND AGENTS
- Two working copies: the second gets a basis conflict naming who registered what since its basis.
- Agents do the same through Conduits: plan.v1 and register.v1 on `management`, open and close on `access`, set-up and retire per ACC-033 (preservation test 5).
- `management` is still broad trust, and the broker (D04) remains the way to delegate narrowly.

OBJECTIONS TO B, C AND D, ANSWERED ONE BY ONE

Design C
- M1, a revise re-deciding Conduits after its restart: there is no revise or restart. A replacement is a new instance, and every grant to it is a fresh open decided at open. A replacement whose open the caller may not make is withheld whole.
- M2, swap against ACC-024: root's grants are never closed by a plan, and a change to one is rejected up front.
- M3, "closed stays closed" with no stored state: LIF-038 and LIF-039, kept per relationship and per project key, survive purges.
- M4, revocation depending on the revision: a close carries only expectedId and is never withheld.
- M5, a revise as power over other teams' Agents: there is no revise. A replacement needs register plus retire authority, as today.
- M6, PLC-009 and PRV-006: untouched, since a change of placement or runtime means a replacement.
- M7, identity surviving a change of code: every replacement has a new ID, and IFC-032 cuts off the old run.
- M8, grants depending on who runs apply: none are derived in the base, and D6 names its operator in the text.
- M9, default drift: resolved config differences need consent.
- D1, forged provenance: provenance is owned by the engine.
- D2, name reuse: outside names are held as IDs, and resolved again only with `rebind`.
- D3, the inferred capability: the capability is required.
- D4, work in flight: wait for no invocation in flight before retiring; unknown outcomes block the plan.
- D6, ambiguous removal: an entry removed becomes detached, needing `--retire`; a grant removed is closed.
- D7, an unbounded right to open: a grant to `access` needs a bound.
- D8, two working copies: the basis.
- D9, the lock: there is no lock; the tool writes the pins.
- D10, widening first: narrowing first.

Design D
- #1, the guard kept per object: tombstones are per relationship.
- #2, access hidden in package templates: PRJ-011, and the plan lists every Conduit a setup opens.
- #3, widening on upgrade: there are no groups; the operations are explicit; consent covers contract, tools map and defaults.
- #4, the implicit Pool: Pools and owners are explicit, nothing is adopted, and coverage is listed.
- #5, the record dying with the Pool: the Project object outlives it.
- #6, a stale copy reverting a later apply: the basis.
- #7, the approved plan not being the plan run: the plan digest and expectedId.
- #8, the dry run as an inspection oracle: plan.v1 reads only what the caller may inspect.
- #9, swap name reuse: successors take new names.
- #10, widening first: narrowing first.
- #11, revocation held hostage by other parts: closes are never withheld.
- #12, names resolved against engine state: kinds and held IDs.
- #13, placement and runtime edits ignored: any change of text means a replacement.
- #14, local templates with no lifecycle: the project holds its templates.
- #15, transitive pins: only pins of packages the entry's own text names count.
- #16, idempotency across a replacement: keys derived from the key and the expectedId, and unknowns block.
- #17, a target changing behind the same tool names: IFC-033 descriptions.
- #18, decisions in several places: no swap, close then open.
- #19, thin audit: generations are append-only, and flags travel as `supersedes`.
- #21, the order of a replacement: the order rule.

Design B
- #1, changes outside apply reverted: controls are never reverted, and there is no swap.
- #2, a retired object brought back: instance tombstones.
- #3, a race between plan and dispatch: expectedId on every part.
- #4, name reuse: held IDs.
- #5, the plan digest not covering the documents: the document travels in the request, with the plan digest.
- #6: ACC-024 respected.
- #7, an old credential surviving: there is no restart in place.
- #8, the T5 Agent restart: the pin rule.
- #9, disclosure: plan.v1 is scoped to what the caller may inspect.
- #10, template changes crossing projects: `rebind`.
- #11, a re-point without a decision: every open is decided.
- #12, identity by directory: the project name and the basis.
- #13, the lock: no lock.
- #14, derived grants too wide: none in the base; D6 lists fixed operations.
- #16, requirements not classified: any change of a grant's text means close plus open.
- #17, the replacement's name: successor names.

## cost
PROMISES
- About 56 touches (about 12% of 455).
- New: 31.
  - PRJ-001 to 017: 17, the language layer.
  - PLN-001 to 012: 12, the convention layer.
  - LIF-038 and LIF-039: 2, tombstones.
- Amended: about 25.
  - 8 for ruling 53's other four additions: OBJ-013, IFC-012, IFC-014, LIF-037, LIF-013, IFC-033, OBJ-018 and ACC-033.
  - 17 for the project: OBJ-002, 003, 004, 017, 020 and 024; NAM-027; MAN-001, 002 and 011; SUB-001, 004, 005, 008, 009 and 010; the description of `register.v1`.
- None of ACC, PLC, SEC or PRV is weakened. None is deleted.
- The five accepted additions (11-14 touches) are needed by every route. Net over the layered route, about 42-45.
- Comparisons:
  - normative expansion only, with a CLI planner: about 20-30;
  - D: about 50; C: about 80; both with their unsafe parts.
- Option D6 adds 2.
- Spec words grow about 10-12%: 31 new at about 300 words, plus amendments, about 11k words on about 118k.
- Concepts for implementers: about 12 added (project document, Project object, generation and basis, derived names, project key, provenance, plan.v1, change classes, consent flags, tombstone classes, `as`, inventory). None removed.

SCHEMAS AND THE CORE
- spec/schemas/projects.yaml is new, holding three schemas, about 200-350 YAML lines.
- operations.yaml: plan.v1 is new; `register.v1`, `open.v1`, `close.v1` and `inspect.v1` gain optional inputs and outputs.
- capabilities.yaml and templates.yaml: plan.v1 is added to object-management.v1 and to root-management.
- results.yaml and references.yaml: four new schemas.
- The core moves to 0.2.0.
- Spec examples: +15-25. tools/check_schemas.py: +100-250 Python, for the schema and the examples only, not the expansion.

VECTORS
- New: about 100-140.
  - `expand`: 30-40, including derived-name clashes, redaction and pointers.
  - Project registration, basis and the plan digest: 10-15.
  - `plan`: 40-60. One per change class, plus every skeptic counterexample of B, C, D and E and the critic's 18 safety cases.
  - Tombstones: 6-10. Additions: 10-17. Equivalence: 1-2.
- Invalidated: about 5 cases' expectations, because root-management now lists plan.v1 (first-start initial, repeat, lifecycle and protected, and conventions/s1/whole). Also 12 snapshot fixtures regenerated, and 2-6 e05 snapshots for IFC-033's descriptions.
- Every other vector of the 367 survives:
  - canonical documents, manifest.v1 and envelope.v1 are unchanged;
  - every new input is optional;
  - the project document is a separate file name, so loader-schema-attribution's keyword locations do not move.

REFERENCE MODEL, BY CRATE (projections, nothing built)

| Crate | Today | Added | Work |
|---|---|---|---|
| loader | 2,879 | +300-600 | the project-document role by file name; its schema loaded from spec/schemas; a pointer-to-position index the shell uses to put lines on core diagnostics |
| core | 10,465 | +3,500-6,000 (+33-57%) | expand.rs, pure and citing PRJ (1,000-1,600); project.rs: the Project object, generations, the register path with basis and digest, provenance (500-900); plan.rs: classes, order, preconditions, consent, controls, coverage, intervals (1,500-2,500); tombstones in lifecycle and access (200-400); `as`, expectedId, inventory, descriptions (300-600) |
| shell | 6,986 | +700-1,300 | install writing the pins (a YAML editor that keeps comments is a risk, as the critic's gap 13 says); plan, apply, render, reach, close and open by key, pull, the flags, per-step reports. Thinner than the layered CLI, because the planner lives in the core |
| conformance runner | 2,137 | +300-600, and case.schema.json +60-120 | `expand` and `plan` vector kinds; an `equivalent` option on register-project; case directories holding a project document |
| tests | | +2,500-4,000 | |

- In all, about 5,000-8,500 Rust source lines plus tests. That is about 1.3-1.6 times the layered package (CLI 3,500-5,500 plus 300-600 core): the planner is the same work, moved into the core and made normative.
- Migration:
  - documents: fully backward compatible;
  - stores: a clean break, because root's permanent first-start Conduit must list plan.v1, which is acceptable in v0;
  - packages: Agent 0.1.2 is published as a new version, not repacked in place.

WORK ORDERS
About 6 to 8:
1. The additions and tombstones.
2. The Project object and `register.v1` with `project`.
3. The expansion and its vectors.
4. plan.v1.
5. The shell commands.
6. The runner kinds.
7. Providers (Agent defaults and descriptions).
Optional D6 and swap.v1 come later. Phase A (orders 1-3, with a non-binding CLI planner) can come before a rerun, and phase B (plan.v1) after it.

OWNER RULINGS
- Amend: 35's wording; 31 and 37 for the frame.
- Confirm: 22/DD2 and round 6 D11 as read here.
- Decision 9's separate format reference becomes optional, because the format lives in spec/.
- Decision 6 is needed only for D6.

## what_is_lost
1. **Two normative forms.**
   - Implementers must build the expansion and the planner as well as the kernel. About 12 concepts are added and none removed, and the spec grows about 10-12%, against about 3% for the layered route.
   - Package and provider writers (S7, S10) still write canonical documents.
   - Each later change to template, instance or Conduit keys has to decide its project-document surface, and add vectors for it.

2. **No line saved on S1 compared with the layered package.**
   - 28 against 27. T1 is 3 lines worse because root's grants are written out. Only option D6 brings it to 25.
   - The case for this design is normative meaning, change enforced by the engine, and learning the format from spec/. It is not lines.

3. **It fails the bar under the strict reading** (every command a line, T5 against the better baseline), as every design does. The margins on T5 to T7 are 1 line, and nothing has run (ruling 45).

4. **The author's text does not show everything.**
   - It hides derived object names (shared--5, agent-a--project--7), IDs, templates, Type defaults and live state; `plan`, `render` and `inspect` show them.
   - Records gain churn: a new ID and name at every change.

5. **Every edited entry is a replacement.**
   - There is no restart in place (A67 is deferred).
   - A replaced Agent loses its home and sign-in. It needs `--replace` and a fresh sign-in at root (ruling 49), which makes Agent package upgrades at scale costly: 10 workers mean 10 sign-ins.

6. **Windows without access remain** at T4, T5 and T6, for each re-pointed grant, until a swap.v1 exists.
   - T5 gains a window that the coordinator's route, which renamed the tools, did not have.
   - At today's request latency a window may last tens of seconds.

7. **G9 becomes ordering for grants.**
   - Grants are standalone Conduits, so "open before the run" (TPL-012) is a plan ordering rule (PLN-004).
   - An Agent set up outside a plan starts without its grants. It fails closed, and only inspection shows it.

8. **Root's grants can only be added to.**
   - ACC-024 means a root grant written in the document can never be changed or removed.
   - Widening one means adding a second grant key; removing one means retiring its target.

9. **The project document is an input that carries authority.**
   - There is no human gate, because sending is the approval. Whoever can edit the document and run apply can widen access.
   - An Agent with write access to the project's directory could add a grant that rides along with the next routine apply. The plan marks widenings, but does not stop them.

10. **Flags cost lines.** `--reopen`, `--recreate`, `--retire`, `--replace` and `--rebind` are command values the author chooses, counted when used.

11. **One pin per package per project** (NAM-006). Workers on mixed Agent versions need separate projects.

12. **Lines grow wide.**
    - Fully qualified operations (ruling 25) make lines of up to about 370 characters.
    - Block style for grants fails T7, and JSON one key per line fails T1. The format must be documented as YAML with one line per grant.

13. **No repetition construct** (IN5, IN2). N workers cost about 5 lines each, and "admin reads every tree" is N lines.

14. **Not expressible until their promises exist:**
    - peer mode and trust domains (ruling 51);
    - enrollment (decision 5 of ruling 52, open);
    - pending actions (ruling 50);
    - the expected account (ruling 49);
    - the two-tier refusal (52.1);
    - complete filter contracts (D05).

15. **Tombstones and generations grow without bound** in v0. Their retention becomes an implementation-defined (DL) item.

16. **A clean break for existing stores**, because root's permanent management Conduit must list plan.v1. About 5 vector expectations and 12 snapshot fixtures are regenerated.

17. **A Project can be revised by anyone holding `management`**, whose trust cannot be narrowed (ACC-030). A per-project narrowing would need a broker (D04) or a later bound on generations.

## Verdict safety (major)
Not safe as written, but fixable. The shape holds: an unchanged kernel, every grant a real standalone Conduit with fully qualified operations, plan.v1 dispatching nothing, the basis check and digest on register, and expectedId on every request. The new machinery around it reopens what the earlier skeptics found. (1) Tombstone classes depend on the state of the generation, not on who closed, so an emergency revocation made while a generation is stalled or in flight is undone by the next routine apply. (2) Tombstones keyed by (consumer, local name) and (project, key) are bypassed by renaming one key. (3) ACC-024 makes every replacement or removal of an Agent that root has a turn grant to impossible, which includes S1's own Agents, so the S10 and real-use claims fail. (4) PLN-007's consent rule compares four fields, so a package version can silently redirect a Resource: a fixed packageArguments value, or the same item turned into a remote. (5) PRJ-008's held IDs contradict PRJ-012's as-if rule, so thesis 2 fails exactly in the name-reuse case. (6) Anyone holding management.register.v1 can write a generation that the owner's routine pull-and-apply then opens with root's authority. (7) LIF-039 breaks the kernel's own restoration route (register-replacement plus set-up), with an unlisted amendment and more broken vectors than counted. All have spec-level fixes that add no author lines in S1.

SCOPE: I only read the repository and changed nothing. I checked the design's claims against:
- spec/conventions/access.yaml: ACC-005, 006, 012, 022, 024, 029, 030 and 033 (rows 1-14).
- spec/conventions/lifecycle.yaml: LIF-008, 009, 035 and 037.
- spec/conventions/operations.yaml: OPS-008.
- spec/interface/implementation.yaml: IFC-012 and IFC-014.
- spec/promises: objects.yaml (OBJ-003, 004, 010, 018), naming.yaml (NAM-024, 027), manifest.yaml (MAN-010), conduits.yaml (CON-008), secrets.yaml (SEC-008).
- conformance/fixtures/s1/packages/filesystem and filesystem-0.2.0: their contracts.yaml files are identical, and the items differ only in npm version and fileSha256.
- conformance/conventions/s1/whole/case.yaml and conformance/conventions/setup/restore/case.yaml.
- The verdict sections of docs/proposals/simplification/research/design-B, C, D and E, and critic.md.

WHAT HOLDS
- Access still goes only through open Conduits with explicit, fully qualified operations; in the base there are no derived grants, so no new tool is ever granted.
- Every request is an ordinary recorded request. plan.v1 dispatches nothing.
- Engine-side expansion plus the basis and plan digest closes D #6, D #7 and B #5.
- Provenance is engine-owned (C D1). The capability is required (C D3). A grant to `access` needs a bound (C D7).
- There is no revise or restart in place, so C M1, M5, M6 and M7 and B #7 are gone.
- The inventory gives no counts (critic gap 8). Sending is the approval, so it works without a terminal (critic gap 1, at the cost of finding 6).

COUNTEREXAMPLES, most severe first

1. MAJOR (G3, immediate revocation): an emergency close during a stalled or in-flight generation is classed "as the project" and undone with no flag.
- The rule: LIF-038 classes a close "as the project" whenever the current generation no longer maps the key to the closed object. Step 1 of every replacement or edit (PLN-004) registers generation g+1, which maps the key to the successor. From that moment, any close of the predecessor by anyone is classed "as the project".
- Trace with T5:
  1. Step 1 registers generation 4 (shared--4, agent-a--project--4 [read, list, write]).
  2. Step 2, set-up of shared--4, fails (a fetch fails), and apply stops. Generation 4 stays current, and agent-a--project--3 keeps serving.
  3. Days later, root closes agent-a--project--3 during an incident. The tombstone is classed "as the project".
  4. The author re-runs `conduit apply` on the same document. PLN-006 says the tombstone is superseded by the project's own later object, and LIF-039 exempts it, so agent-a--project--4 is opened with write. No conflict, no flag.
- The same holds within the seconds between step 1 and the plan's own close (B #3, E #2).
- Second effect: `conduit close agent-a/project` "by key" resolves through the key map to the current object. That is the registered, not-yet-opened --4. Closing it removes it (LIF-035), while --3 stays open with write. The operator believes the grant is revoked and it is not.
- Fix:
  - Class a close "as the project" only when the close request itself carries the plan's context (project ID, generation, key), which the engine checks against that generation's key map. Every other close is "against the project".
  - Close by key closes every Conduit that is not closed and has that (consumer, local name).
  - LIF-039 exempts only the successor of the plan that sent the close.

2. MAJOR (G3, critic gap 2 not closed): renaming one key bypasses the tombstone.
- Tombstones match only (consumer ID, local name) and (project ID, key).
- Trace: root closes agent-a/project against the project. The next plan reports a conflict. The author, or an Agent with write access to the directory, renames the grant key `project` to `workspace`. The old key is treated as removed (its Conduit is already gone). The new key is "Added: register, then open". No tombstone matches, so write access comes back with no flag.
- The same happens when the consumer's entry key is renamed. This is E #3(b), whose fix ("key the conflict by consumer and target") was not adopted.
- Two related gaps:
  - Anyone allowed to open may name any tombstone in `supersedes`, so the engine adds acknowledgement but no authority. The claim that this "binds Agents that bypass the CLI" is overstated.
  - Retention is implementation-defined (DL), so a conforming engine may keep tombstones for zero seconds.
- Fix:
  - Also key tombstones by relationship: (consumer key or ID, target key or ID, capability). PLN-006 reports a conflict for any new grant on a relationship with a tombstone "against the project", whatever its local name.
  - Owner decision: restrict superseding a tombstone "against the project" to root or the closer.
  - Keep tombstones at least while the Project key exists or until superseded. Only root purges them, and the purge is recorded.

3. MAJOR (scope guard; the design's own S10 and real-use claims; D #11 returns): root's grants make Agents unreplaceable and unremovable.
- T1 writes root turn grants to agent-a and agent-b. PLN-003 counts a grant as changed when its target is replaced, and rejects any changed or removed root grant "before any part".
- So every Agent replacement is impossible: `--replace agent-a,agent-b` in S10, Agent package security upgrades, model or ruling-48 flag changes, and the real-use upgrade of 10 workers. So is removing such an Agent: the entry cannot go while root's grant points at its key, and the grant cannot go.
- Even read leniently, PLN-004's replacement order ("open the successors; retire the predecessor") cannot be met. Root's successor grant `as agent-a` clashes under amended IFC-014 with the predecessor, which cannot be closed (ACC-012, ACC-024), until the target is retired (CON-008).
- S5's root grant to `docs` likewise blocks any replacement of docs.
- "Before any part" also withholds an unrelated revocation made in the same edit, which contradicts PLN-010.
- With D6 these root grants are implicit, so the author cannot see why.
- Fix: a PLN rule for root grants:
  - Re-point: register the successor grant, retire the predecessor target (CON-008 closes root's grant, with a tombstone carrying the plan's context), then open the successor. List the interval.
  - Removal: a root grant line may be removed together with `--retire` of its target.
  - A text edit of a root grant becomes a refused part that is skipped. The plan goes on.

4. MAJOR (T5 hole, no silent widening): PLN-007 checks only the granted contracts, the tools map, declared Conduits and the resolved config.
- NAM-024 constrains contracts. It does not constrain manifest implementation items or templates. The Type local-directory.v1 points at `#filesystem-server` by name.
- So a filesystem 0.2.0 whose `filesystem-server` item makes any of these changes plans exactly like T5, "no consent needed":
  - adds a fixed positional `value: /` (MAN-010 forbids binding fixed values, not stating them);
  - adds runtimeArguments that mount $HOME;
  - becomes a plain `remote` at a third-party https URL with the same tools map.
- shared--4 is then made, and both Agents' reads and writes reach `/` or a remote server.
- The same gap covers an exported template's `implementation` and `controls` maps.
- Fix: require consent whenever the resolved definition or any selected item differs in anything other than the version and digest of the same package identifier, and show the item diff. T5 stays at 1 line, because the fixture differs only in version and fileSha256.

5. MAJOR (thesis 2, B #4): PRJ-008 and PRJ-012 contradict each other on names outside the document.
- PRJ-008 holds an outside name's ID and never resolves it again. PRJ-012's as-if rule requires the objects that registering the rendered documents would give, and those resolve by name (OBJ-009 forbids IDs in documents).
- Trace: a grant to `{kind: Resource, name: datasets}`. The Resource is retired and removed, and another actor registers a new `datasets`. Then agent-a is replaced, or the grant's operations are edited, so the grant is re-registered.
  - Under the as-if reading, the successor points at the squatter's directory.
  - Under PRJ-008 it conflicts.
- Two conforming engines differ, and `render` cannot show what is registered.
- Fix: PRJ-012 binds outside names to held IDs. register.v1 checks that each name still resolves to its held ID, and otherwise reports a conflict naming both and requiring `rebind`. render shows the held IDs. The as-if and equivalence rules are stated modulo this binding.

6. MAJOR (confused deputy, ACC-030, real use): anyone with management can author what the owner's next apply opens.
- Registering a generation needs only `management` (rows 2 and 7, with row 1 only for Resources).
- Trace:
  1. A holder of management.register.v1, with no access rights, registers generation g+1 of the owner's Project. It adds a write grant for its own consumer to the owner's Resource. The grant is registered, which is inert.
  2. The owner's next apply hits a basis conflict, pulls, and applies. Sending is the approval, so root opens the grant.
- Today's kernel has no such path: whoever opens chooses what to open.
- Real use: if conduit-project.yaml lives in the repo that workers' merges reach, a worker's edit rides along with the same apply.
- Fix: per-Project authority to register (a Conduit to the Project that lists register.v1, as a new ACC-033 row). Or: pull and apply require naming, by flag, any widening authored by another registrant. The plan marks who wrote each generation.

7. MAJOR (kernel semantics and cost): LIF-039 blocks LIF-009's explicit restoration.
- Restoration today is register-replacement.v1, then set-up or open (LIF-008, LIF-009, CON-004, ACC-005). It reopens a declared Conduit with the same (consumer, local name), which the tombstone matches.
- Only register.v1 and open.v1 gain `supersedes`. A part of a set-up cannot carry it, and S4's "register-replacement.v1 with supersedes" names an input that does not exist.
- Effect: conformance/conventions/setup/restore/case.yaml breaks, and it is not among the "about 5" cases. So does s1/whole's T7 for this reason, apart from plan.v1. The claim that "every new input is optional" therefore does not mean that vectors survive.
- Instance tombstones keyed by (kind, name) lock a name for everyone after LIF-035 has freed it. That is an amendment of LIF-035 and OBJ-002 that the design does not list.
- Fix:
  - register-replacement.v1 is itself the acknowledgement: it supersedes, on the record, the tombstone of the nested instantiation it replaces. Or it gains `supersedes`, and set-up honours it.
  - Key instance tombstones by ID and project key.
  - List the amended promises and recount the invalidated vectors.

8. MODERATE (S5, B #1): a replacement reverts a narrowing made by a control.
- PLN-008 promises such values are "never reverted". Yet any unrelated change that replaces the instance (another config key, a description, a pin bump) builds the successor from the document.
- Example: root lowers `limit` by control to 10 during an incident. The successor comes up with 40. The plan shows the resolved config but reports no conflict.
- Fix: a replacement whose live values at control pointers differ from the document is a conflict, unless the document states the live value or a flag such as `--take` names it.

9. MODERATE (PLN-010 overstated): revocation through the document is hostage to registering a generation.
- Step 1 is all or nothing. A basis conflict, an unrelated invalid line, PKG-006, or a caller without register.v1 means no close is sent.
- Derived names (`K--<g>`) are predictable, and `--` is not reserved. Anyone with register.v1 can pre-register agent-b--project--3 and block T4's narrowing indefinitely, because g does not advance on a rejection.
- Fix: closes of removed or narrowed grants can be planned and sent without the generation, with tombstones classed "against the project", which is safe. Reserve names containing `--` to the engine (OBJ-001 and OBJ-003; no existing vector uses one).

10. MODERATE (claim "change enforced by the engine"): ordering and drain are advisory.
- PLN-004 is only the content of the plan's output. The engine does not enforce it. A non-CLI client can close A's predecessor and open A's wider successor before B is narrowed.
- "Wait until no invocation is in flight" is not a request and has no observable, so E #7's torn writes remain.
- PLN-009 needs other actors' invocations of unknown outcome, which PLN-001 forbids reading.
- Fix: either the engine refuses to open a widening successor while a narrowing of the same generation is pending, or the claim is downgraded. Make the drain a precondition of retire.v1, or an output of close.v1. Scope PLN-009 to what the caller may inspect.

11. MODERATE (determinism): "text changed" is undefined.
- OBJ-018 says two documents that differ only in description resolve to the same object. Under PLN-002, editing a description, writing `config: {}`, or stating a default explicitly replaces the instance.
  - An Agent then loses its home and needs `--replace`.
  - A Pool's replacement cascades to every Resource it owns and runs into finding 3.
- Fix: compare the resolved expansion, defaults filled, with description and annotations excluded.

12. MODERATE (optional D6): its derived operator grants bring back the T5 hole.
- It lists the startsModelWork operations of the capabilities the Type offers at the pinned version. NAM-024 lets a later release add an operation to a capability, so the operator gains a tool at the next Agent replacement without a line.
- A Type that offers several capabilities gives several Conduits under one local name (CON-001, IFC-014).
- With root as the operator, ACC-024 freezes these grants (finding 3).
- Recommendation: reject D6 as written, or fix every list it derives in spec/.

13. MINOR (disclosure):
- Under ACC-033 row 7, any holder of management.inspect.v1 reads every generation's text: per-instance `arguments`, owner, placement and registrants. Row 6 restricts inspecting those instances to the owning Pool.
- The basis conflict names registrants to any caller of register.v1. Tombstones, meanwhile, hide their closers.
- Fix: redact what the caller could not inspect.

14. MINOR (D #16 only partly answered): an Agent's write that ends unknown after the close is retried through the unchanged tool name. The successor is a new target, so the key scope is new and the write is dispatched again on the same directory. Fix: record the lineage of a re-point, and report conflict for a repeat that names the predecessor until it is reconciled.

15. MINOR: the project is identified only by name plus an integer basis.
- After the Project is unregistered, a squatter's Project of the same name can reach the same basis, and the old Project's tombstones no longer apply.
- Fix: write the project ID beside `basis`.

16. MINOR: derived names can collide.
- A grant key `t1` under an inline consumer `shared` gives a Conduit `shared--t1`, the same name as the Resource template `shared--t1` (NAM-027 rejects two documents of one name in one submission).
- Grants under `Interface/x` and `Agent/x` collide too.
- Both fail closed. Fix: reserve `t<digits>` as grant keys.

17. MINOR (thesis 2): after T5, render shows the unchanged history grants' templates at filesystem 0.2.0, while the live ones hold 0.1.0, so the equivalence does not hold after a pin change.

EFFECT
- None of the fixes adds an S1 line. S1 stays at 28 on paper.
- They add roughly 6-10 promise touches: relationship tombstones, the root re-point rule, held-ID binding, Project register authority, supersedes on restoration, and `--` reservation.
- More existing vectors change than claimed (setup/restore and s1/whole, at least).

## Verdict expressiveness (major)
Not expressive enough as written (severity: major, not fatal). The kernel constructs that S2, S3 (managed mode), S5's narrowing, S7, S8, S9 and S10 need can all be reached through the `templates` section, inline entries and grant `requirements`. But six gaps force the author back to the canonical form or to requests, or make change unsafe. 1. The escape hatch does not work: full-form documents cannot name project objects stably across replacements (critic gap 6 is unfixed). 2. Pool entries cannot be granted to, and cannot be replaced one at a time. 3. Values set by a control are silently reverted when the instance is replaced. 4. The owner's real use cannot keep package versions side by side. 5. Reusing an Agent's connections through a template ties them to the Agent's identity. 6. Every replacement is make-before-break, with no other order. There are also about 7 minor gaps. Every gap can be fixed with an addition; none needs the design reshaped.

I only read the repo and changed nothing. I checked the design's grammar and its PRJ/PLN rules against spec/core/references.yaml, spec/schemas/templates.yaml, spec/promises/{pools,objects,secrets,conduits,naming,instantiation,contracts}.yaml, spec/conventions/{access,lifecycle,revisions}.yaml, spec/interface/implementation.yaml, docs/proposals/ruling-4{9..53}/README.md and docs/proposals/simplification/research/critic.md. Every count is on paper; nothing was run.

WHAT IT EXPRESSES (checked, no fallback needed)
- S2 secrets: `token: {kind: Resource, name: linear-key}` at secret-reference places, and standalone get-secret grants from a Resource. SEC-010 accepts the consumer's own standalone Conduit. SEC-008 makes them root-only, and PLN-010 withholds them for any other caller.
- Rotation cascades by dependency edges.
- Filters with per-grant arguments (S2, S8), bounds on `access`, shared-host (S3).
- Grants to `management`, Agent to Agent turns, Agent to Pool, and Interfaces other than root.
- `placement` and `runtime` per entry (S3 managed mode, S10).
- Providers owned by `{kind: Pool, name: system}` (S9, S10; ruling 50's Tailscale fits PRV-013).
- Hooks through the Type, and `controls` maps with narrowing (S5).
- `from:` package templates (S7). Contracts stay canonical, as the design intends.
- `as` keeps tool names across replacement.
- The expected account (ruling 49) fits in `config` once a Type defines it. Peer mode, enrollment and pending actions are missing from the kernel itself, not from this form.

MAJOR (each needs a fix, or a fallback the design forbids)

M1. The escape hatch fails across replacements, and so do multiple owners and multiple projects. CONFIRMED.
- The design rejects canonical template and instance documents beside the project document. Canonical documents registered anywhere else can name project objects only by derived names: `shared`, then `shared--4`, then `shared--5`. Those come from other projects, from Agents managing through `management` (test 5), from brokers (D04), and from a peer's half of a relationship (51.4).
- OBJ-010 keeps their held IDs. CON-008 closes their Conduits when the predecessor is retired. PLN-007 asks only for `--replace` consent and lists them; the plan cannot re-point them.
- Concrete case: owner B's project grants its Interface `to: {kind: Resource, name: main}` in A's project. A bumps a pin, so `main` becomes `main--5`. B's grant is closed, and B must find and type `main--5`, then `--rebind`. It happens again at every later replacement.
- No reference form names (project, key). The proposed `project-reference.v1` is not defined as one.
- The kernel already has the pattern: POOL-014's `{kind: Pool, name, entry}` resolves to "the Resource of that entry that is not retired".
- Fix: a `{kind, project, key}` reference, resolved once at instantiation (so OBJ-010 holds), with a plan in the referring project that treats the other project's replacement of that key as a dependency change.

M2. Pool entries. CONFIRMED on the text.
- The grant `to` is "an entry key, or `{kind, name}`", and the consumer key is "an entry key, or `Kind/name`". Neither admits `{kind: Pool, name, entry}`, although consumer-reference.v1 and resource-reference.v1 both accept it. So no grant can target or be consumed by an entry's Resource, for example an Agent's grant to the service entry of S4's secret, service and webhook Pool.
- The design says project objects "never use `register-replacement.v1`", yet its S4 sketch sends `--recreate` as register-replacement.v1. The two contradict each other.
- POOL-003 and LIF-009 are the only kernel route to a new Resource for one entry. Without them, editing or adding one entry changes the Pool's text, so the whole Pool is replaced. Through the `owner` edge (PRJ-009), every standalone Resource it owns is replaced too, and every grant to any of them is re-pointed with a window without access.
- Fix: allow pool-entry references in `to` and in consumer keys; add per-entry change classes that use LIF-009 under the entry name; and state that adding an entry is not a Pool replacement.

M3. S5: a replacement reverts what a control set. CONFIRMED.
- REV-007 stores a control's values in the instance's configuration, and PLN-008 promises that such a value is "never reverted unless the document changes that value".
- But any other change to that entry instantiates the successor from the document's `config`, and so silently restores the document's value. Other changes include a pin change of a package its text names, a replaced key it references, a changed template, or an edit to `description`.
- Carrying the live value into the successor would break PRJ-012's as-if rule (render, then register, must give the same objects).
- This is the scenario inventory's open question (whether the file or the live object owns the setting), and it remains unanswered. Fix: make a replacement that would revert a value set by a control need consent, like PLN-007, and report it in the plan.

M4. Real use: package versions side by side. CONFIRMED.
- NAM-006 allows one version per address per manifest, and the project document is the manifest.
- Canonical form keeps versions side by side through separate submissions (NAM-009, PKG-005), and ROADMAP's real use needs this ("versions side by side", rulings 25 and 27).
- Here, workers on 0.1.2 and 0.1.3 exist only through the undeclared state "pending (consent missing)". In that state the document no longer says what runs, and no new worker can be made at the old version. The design's other answer, a second project, is broken by M1.

M5. Real use: reusing an Agent's connections ties them to the Agent's identity. CONFIRMED.
- The design's 10-worker sketch puts `tree`, `main` and `orders` as declared Conduits in `codex-worker`. Any replacement of `main` or `orders` therefore changes that template (PRJ-009), and any replacement of `tree-i` changes `worker-i`. That includes a pin bump of the git package, which the owner rebuilds while developing.
- Each affected Agent is replaced: its home and sign-in are lost (ruling 49), and each needs `--replace`.
- A declared Conduit that was closed comes back only through LIF-009 (LIF-008), which the design bans for project objects. Restoring one therefore means replacing the Agent.
- The route that avoids this is `grants`, but grants have no reuse. The grant shape is closed and has no `from`, so `templates.Conduit` serves declared Conduits only. Every grant restates its capability and fully qualified operations, up to about 370 characters.
- Per worker this is about 9 lines, not the claimed 5, so about 125 lines rather than 85. Fix: allow `from` on a grant, and document that Agents get their connections through grants.

M6. Make-before-break only. PLAUSIBLE: it depends on the worktree Type's hook.
- PLN-004 always sets up the successor before retiring the predecessor, and no flag reverses the order.
- Exclusive things cannot exist twice. `git worktree add` refuses a branch that is already checked out, and the same applies to a port or a lock. So replacing `tree-101` on branch wo/101 fails at the successor's setup.
- The workaround takes two generations: remove the entry and every reference to it, apply with `--retire`, then re-add it. Fix: a consent flag, or a statement in the Type, that selects break-before-make.

MINOR
- m1. Removing a key silently re-targets references to it. Once key K is removed, any remaining `to: K` or `{kind, name: K}` becomes an "outside name". If K's current object was `K--5`, the name `K` may by then belong to a foreign object (LIF-035 frees the names of removed objects). The plan shows this as a widening that is applied as written. Fix: keep every name that was ever a key reserved for that key, and reject such references.
- m2. A grant to `access` must carry a bound, so a co-owner Interface with unbounded `access` cannot be written (test 8), although ACC-030 allows one where the opener's own `access` is unbounded. This is deliberate, but it narrows what the kernel allows.
- m3. S9 repair: `apply` sets up only instances that are not ready. Setting up again an instance that is ready (TPL-012, PRV-007), reconcile and declare-lost are requests outside the plan. The design's direct commands by key cover only close and open.
- m4. Derived names must fit 63 characters at every future generation. `C--L--t<g>` grows with g, so a long consumer key plus a long grant key that is valid today can later become impossible to change.
- m5. Root's grants can only be added (ACC-024). Removing one or editing one rejects the document, so the desired-state file accumulates lines it can never drop.
- m6. "A replacement inherits its predecessor's resolved node" is not in the text. Either the expansion reads stored state, or the author has no way to say "place it again".
- m7. Peer mode (51.4, two halves) is not expressible because the kernel lacks it. Once written, a half will need stable names for the other engine's project objects, which M1 lacks. This is a v1 risk.

ANSWER TO THE ESCAPE-HATCH QUESTION
No. The design removed the framed-document hatch instead of fixing it. Full-form documents still cannot follow a project object across replacements, because derived names change and no reference names (project, key). The `templates` section does reach every template construct of the kernel, but not these instance-level constructs: pool-entry references in grants (M2), a standalone Conduit made `from` a template (M5), names chosen by the author, and `annotations`.

The design's documents are under <scratch>/projdoc/ (t1.yaml to t7-restored.yaml); I did not use them for these checks.

## Verdict cost (minor)
The design's cost claims hold in kind but run 15-35% low. Every base figure it cites is right, but three of its own claims fail on checking. (1) Engine-enforced tombstones, written as its LIF-039, break 3 more existing vectors outright and put 4 at risk; this applies to the layered route too. (2) Its core planner and shell figures are low. (3) Its "phase A" cannot stand without part of the phase-B planner. Corrected, making the project document normative costs about 60-80 promise touches (about 45-60 more than the layered route), 6,000-10,500 lines of Rust (about 1.5-1.8 times the layered route's) and 120-190 new vectors (about 10 times as many), plus a store break that could be avoided. It saves no line on S1 (28 against 27). The corrections do not reverse the comparison. What the normative route buys is thesis 2 for change, agreement between two implementations (A34), and revocation guards and planning the engine enforces. As a cost estimate, the design is acceptable once corrected.

SCOPE: I only read files under <home> and changed nothing there. Helper scripts are in <scratch>/ (p.rb and scan.rb).

WHAT I CONFIRMED (the base figures are right)
- Promises: there are 455 IDs (spec/promises, conventions and interface). Their text is 117,926 words, which is the design's "about 118k". The average promise is about 196 words.
- New IDs: LIF-038 and LIF-039 are free, and every promise the design amends exists.
- Vectors: there are 367 case files. 143 of them depend on declared Conduits, replace.v1 or register-replacement, against the design's 141. That confirms that deleting those constructs is not affordable.
- Crate sizes match exactly: loader 2,879, core 10,465, shell 6,986, conformance runner 2,137.
  - About 2,441 of the shell's lines are tests inside src/.
  - The shell has only four commands today: init, records, inspect and register.
- The registration-orders runner kind is at conformance/src/lib.rs:598-631, as the design says.
- New outputs break no existing vector. The runner compares by projection (lib.rs:299-330): expected keys are a subset, and only arrays must match in length. So new fields in inspect.v1 (the inventory, tombstones, `project`) and new optional inputs pass.
- No vector uses an `as` key or a file named conduit-project.yaml.
- The core version bump to 0.2.0 appears in no vector. The 0.1.0 values in vectors are S1 package pins, and core-pin is a rejection test.
- ACC-024 refuses to close any Conduit whose consumer is root, so "root's grants can only be added to" is correct.
- IFC-014 today counts tools of Conduits "not closed, whether or not they are open yet". The successor-under-the-same-`as` step therefore really does need the amendment.
- ACC-033's rows 2, 6 and 7 are cited correctly.
- The root-management template (spec/core/templates.yaml:111-127) lists its operations explicitly. root's Conduit cannot be closed (ACC-024) and objects are immutable, so an existing store can never gain plan.v1. The store break is real.

COST TABLE (design's claim, then my estimate, then the layered package from synthesis.md:108-130 and critic.md:100-102)

| Item | Design claims | My estimate | Layered package |
|---|---|---|---|
| New promises | 31 | 31 as drafted. At the spec's own size (about 200 words each), the roughly 300-word PLN and PRJ items split into 40-50 | about 2 |
| Amended promises | about 25 | 28-33 | 9-12 |
| Total touches | about 56 (12%) | about 60-80 (13-18%). Net over layered: about 45-60, not 42-45 | 11-14, plus about 3 for the shared tombstone fixes |
| Promise count after | 486 (488 with D6) | 486-505 | about 457 |
| Spec growth | about 11k words (10-12%) | about 12-15k words (10-13%), including definitions.md, README "Conformance" and new implementation-defined items | about 3% |
| Schemas | projects.yaml of 200-350 lines | Plausible: templates.yaml is 340 lines. check_schemas.py (2,588 lines) gaining 100-250 is plausible | small |
| New vectors | 100-140 | 120-190 | 10-17, plus non-binding compiler cases |
| Existing vectors broken | about 5 cases, 12 snapshots, 2-6 e05 | 5 cases, 11 snapshots that may not need regenerating, 1 e05, plus the tombstone breaks below | the tombstone breaks plus 1 e05 |
| Core | +3,500-6,000 | +4,500-7,500 | +300-600 |
| Loader | +300-600 | +250-500 | 0 |
| Shell | +700-1,300 | +1,000-2,000 | CLI 3,500-5,500 (critic: 50-80% of the shell) |
| Runner | +300-600, plus 60-120 schema lines | +300-700 | golden compiler cases only |
| Tests | 2,500-4,000 | same | 2,500-3,500 |
| Rust source in total | 5,000-8,500 (1.3-1.6 times layered) | 6,000-10,500 (about 1.5-1.8 times layered) | about 3,800-6,100 |
| Concepts for implementers | +12, none removed | +13-15 | +5 |
| Work orders | 6-8 | 7-9 | 4-6 |
| Store migration | clean break | clean break, but avoidable | none |

Notes on the table:
- Amendments the design misses:
  - LIF-035: a removed object's name "can be given to another object of its kind". Tombstones contradict this.
  - LIF-009 and LIF-029: whether register-replacement must carry `supersedes`.
  - LIF-017 and LIF-012: unregistering a Project and the templates it holds.
  - An open-time tool-name clash rule, once IFC-014 moves the check from registration to open.
  - CON-012 and the observable of LIF-037, both of which mention name reuse.
- New vectors: the design promises a case for every skeptic counterexample in B, C, D and E plus the critic's 18. That alone is about 85, before one case per change class, PLN-011's eight reports, the disclosure rows and redaction.
- Existing vectors broken by adding plan.v1 to root:
  - 5 cases assert root's operation list: first-start initial, repeat and lifecycle, language/first-start/protected, and conventions/s1/whole.
  - 11 snapshots hold root's list, not 12: 10 initial.json files plus fixtures/root-system-state.json. Existing cases never call plan.v1, so these may not even fail.
  - Only e05-local asserts an exact tool description, not 2-6 e05 cases.
- Core: the design's plan.rs of 1,500-2,500 lines is the same figure critic.md:102 already called low for the CLI planner. The normative plan.v1 adds tools before and after, coverage, intervals, consent found by comparing contracts, tools maps and resolved config, controls, disclosure limits and redaction. That is about 2,500-4,000 lines, the size of registration.rs (2,061) plus lifecycle.rs (1,133).
- Loader: yaml.rs and diagnostic.rs already track spans, JSON pointers and line and column, which is why I put it lower.
- Shell: every command (install's pin editor, plan, apply, render, reach, open and close by key, the flags) is new. The pin editor that keeps comments is still the risk.
- Runner: new kinds follow the 35-line registration-orders pattern.

CORRECTIONS THAT MATTER
1. Tombstones break existing vectors, on both routes.
   - Synthesis counted 0 broken vectors because its tombstones were advisory. The owner chose engine enforcement.
   - Under this design's LIF-039 (keyed by consumer and local name, enforced at register and open, with register-replacement needing `supersedes`), today's close-then-register-replacement route conflicts.
   - Firm breaks: conformance/conventions/changes/t4, setup/restore and setup/restore-current-template. Each closes and then register-replaces a declared Conduit and expects success. s1/whole breaks too but is already in the design's 5.
   - At risk, depending on which check runs first: setup/restore-missing-template and restore-retired-target, changes/retirement (which expects invalid under CON-025), and setup/close (which expects invalid on opening after a close).
   - Cheap fix for the owner to decide: a register-replacement.v1 that names the nested instantiation supersedes its predecessor's tombstone, and the supersession is recorded.
2. A condition on IDs. The core takes object and record IDs in order from each step's evidence list (reference-model/crates/core/src/lib.rs:221-238). Tombstones, Projects and generations must take no ID from that sequence. Otherwise later IDs shift in about 15-25 vectors that close, retire or unregister. Tombstones should be keyed, or given derived IDs.
3. Phase A does not stand alone. PRJ-016 makes the engine recompute "plan.v1's register part" and check its digest. That needs PLN-002 and PLN-003's change classification inside the engine. So phase A is about 47-50 touches and +2,500-4,000 core lines, not about 44.
4. The store break can be avoided. If plan.v1 is authorized like inspect.v1, or is a pure library function, root-management is unchanged. That also saves the 5 case changes. The design does not offer this option.
5. Migrating existing deployments is not costed. Nothing is ever adopted, and derived instance names are the bare keys, which are unique across the whole engine (OBJ-002) and not per project. So an existing canonical deployment cannot move into a Project except by recreating it, which costs Agents their homes and sign-ins (ruling 49).
   - Two projects also cannot share any key of the same kind: Pool s1, agent-a and the grant name root--s1 would clash.
   - This needs a PRJ-004 rule and vectors.
6. Words: about 9.3k new words written at the spec's granularity come to 40-50 promise IDs. The reading cost is the words, about 10-13%, not the ID count.

COMPARISON AND VERDICT
- Normative now costs about 60-80 touches, against:
  - layered now at 11-14, plus about 3 for the shared tombstone fixes;
  - layered now plus promotion later at about 36-44 (synthesis.md:345 puts promotion at 25-30 promises and about 50 plan vectors). Promotion would not include the engine's Project object or the basis and plan-digest checks.
- Code: about 1.5-1.8 times. Vectors: about 10 times as many.
- The normative route breaks existing stores unless correction 4 is taken; the layered route does not.
- Lines on S1: 28 against 27.
- Cost of each later language change (rulings 49-51) on the normative route: about 1-3 more touches and 3-6 more vectors per change for its project-document surface. Additions under template keys follow automatically through `$ref`.
- The design's direction and order of magnitude are right and the comparison stands: the normative route is about 4-5 times the promise work of the layered route, for normativity and engine enforcement rather than lines.