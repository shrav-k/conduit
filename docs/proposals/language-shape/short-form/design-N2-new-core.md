Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Smaller core: one sectioned file over a pruned object model, with every grant a relationship named by its consumer

## name
Smaller core: one sectioned file over a pruned object model, with every grant a relationship named by its consumer

## thesis
The concise form can be the only normative form if the object model shrinks to what authors actually state. Five changes do that:
- Each object is defined where it is used: an inline definition is an anonymous template, and templates are kept only for reuse.
- Every Conduit is a standalone relationship, identified by its consumer and its local name (the grant's key). That subsumes ruling 53's `as`.
- A project's Pool is derived from one `project:` line, together with a real, recorded operator grant.
- The frame is stated once per file. The file is also the wire value of register.v1.
- Declared Conduits, replace.v1 and the Conduit half of register-replacement.v1 are deleted. Pool entries stay (S4, S10 and D09 need them), and register-replacement.v1 is cut down to entries.

Change stays outside the language. It is ruling 53's explicit plan/apply, built from core requests and guarded by engine-held tombstones, expectedId and an inventory.

On paper, S1 costs 31 authored lines: T1 20, T2 6, T3 0, T4 2, T5 1, T6 1, T7 1.
- Against the baseline (64): better on six tasks, the same on T3.
- Against the layered package (27): 4 lines worse, all in T1. Those 4 lines are the explicit turn grants and their `grants:` header.
- The fresh author learns the format from spec/ alone, and what authors write has a normative meaning tested by vectors.

The price is a clean break:
- about 100 of 455 promises touched (6 removed, about 84 amended, about 11 new);
- about 70-80% of the 367 vector inputs converted mechanically, and about 140-200 vectors with regenerated expectations;
- all 114 examples and 84 fixture archives rewritten;
- increments 1-3 re-evidenced.

The spec gets smaller in concepts (about 7 removed, 5 added) but not in words (net about ±1%).

## grammar
ONE FORM: the Conduit file. One YAML (or JSON, SCH-013) map per file.
- The strict syntax rules are unchanged: SCH-012, SCH-017, SCH-005/006/020/021, UTF-8 with one optional BOM, and DL10 limits.
- SCH-018 is reworded: a JSON file holds one map, not an array.
- Keys are closed (OBJ-019) and camelCase.

TOP-LEVEL KEYS

`apiVersion`
- A string, required in every file. Today's value is `github.com/shrav-k/conduit/v0` (names.yaml).
- Every entry in the file is written for that version (OBJ-017 per file, ruling 35's purpose). All files of one project state the same value.

Project-level keys. They are allowed only in the main file, conduit.yaml, conduit.yml or conduit.json (MAN-001), which is the manifest.
- `project`: a name (OBJ-001), optional. It declares the project Pool (FIL-007) and makes it the default owner.
- `operator`: the name of an Interface, default `root`. It is the consumer of each derived operator grant.
- `address`: a string (NAM-001). Required where the project has a contract section or lists implementations (MAN-015), and in a package.
- `version`: exact SemVer, only beside `address` (MAN-004). Required in a package (PKG-017).
- `packages`: a map from address to `{version: SemVer, digest: 'sha256:<hex>', fetch?: URI-reference}`.
  - Written by `conduit install` (ruling 53 #5).
  - Governed by NAM-006/007/008, MAN-005/006 and PKG-006.
  - Default `{}`.
- `exports`: a list of names. Packages only (MAN-007): contracts, templates and implementations.
- `implementations`: a map from name to item. The items keep today's keys and rules: MAN-009/010, PKG-019/020/030, MAN-017/018, SEC-021, IFC-043/044.
- `external`: a map from name to Interface | Agent | Resource | Pool, default `{}`.
  - It lists the objects outside the project that the file names.
  - `system`, `local`, `management`, `access` and `root` are external implicitly.

Object sections. They may appear in any file of the project; a package may hold only `templates`.
- `interfaces`, `agents`, `resources`, `pools`: each a map from name to Instance.
- `templates`: a map from name to Template.
- `grants`: a map from consumer to a map from local name to Grant.

Contract sections. They may appear in any file, of a project or a package.
- `types`, `capabilities`, `operations`, `requirements`, `schemas`: each a map from export name (NAM-004, ending in `.vN`) to the contract body.
- The body is exactly what today's `spec` holds (CTR-002 to CTR-008, CTR-012, unchanged), plus an optional `description`.

INSTANCE. An entry of interfaces, agents, resources or pools takes exactly one of two shapes.

(a) An inline definition:
- `type`: a selection of a Type of the section's family (TPL-002). Required.
- `config`: a map, default `{}` (INS-024). It is filled by the Type schema's defaults (SCH-009/010) and checked against that schema (INS-014).
- `implementation`: a map from operation selection to implementation selection, default `{}`, so the Type's defaults apply (TPL-003/004/005/019). Hooks are TPL-017/018.
- `controls`: a map from control selection to `{inputs?: {name: {schema}}}`. Default: every control the Type lists (TPL-010/011/013).
- `entries`: pools only. A map from name to Entry, default `{}`.

(b) From a template:
- `from`: name | `{kind, name}` | qualified | `#name` (INS-025).
- `arguments`: a map, default `{}` (INS-003 to INS-009).

Keys allowed in either shape:
- `owner`: resources only. A Pool name. Default: the project Pool. Required when the project has no `project` key (POOL-001/004).
- `placement`: agents and resources. A node name (external, or `local`). Optional; PLC-002 decides when it is left out.
- `runtime`: agents and resources. A runtime-provider Resource. Optional; with none, the engine's own runner runs the instance (PRV-006).
- `operator`: pools only. An Interface name or `none`. Default: the file's `operator`.
- `succeeds`: agents and resources. The name of an instance in the same section. Optional, and written by plan tools (LIF-039).
- `description`: a string, optional. It is shown in the description of every tool whose Conduit targets the object (IFC-033 under ruling 53).
- `annotations`: a map of strings, with no meaning.

Rejected in an instance: `id`, `revision`, `status`, `packages` (OBJ-004), and `kind`, `metadata`, `spec`, `apiVersion`.

ENTRY. A Pool's entry is inline (`type`, `config`, `implementation`, `controls`) or `from` with `arguments`.
- `optional`: a boolean, default false.
- No `owner`, `placement` or `runtime` (POOL-009/012). Placement and runtime come only from the inputs of a Pool template.
- One entry refers to another as `{entry: name}` (POOL-005/006/011).

TEMPLATE. A template exists only for reuse.
- `kind`: Interface | Agent | Resource | Pool | Conduit. Required.
- `inputs`: a map from name to `{schema, to: [JSON Pointer, ...] (non-empty), default?, optional?: bool}` (INS-021, OBJ-015/016).
  - The destinations are a closed list (INS-012 without the declared-Conduit places):
    - `config`, or a place inside it;
    - an entry's `placement`, `runtime` or `arguments`, or a place inside them;
    - a Conduit template's `operations`;
    - a requirement's `arguments`, or a place inside them.
- The definition keys of that kind:
  - for Interface, Agent, Resource and Pool, those of an inline Instance;
  - for Conduit, `capability`, `operations` and `requirements` (CON-016/017/018).
- `description` and `annotations`.
- Once registered, a template is immutable. There is no replace.v1.

GRANT. An entry of `grants[<consumer>]`.
- The consumer is the map key above the grant. It is one of:
  - an Interface, Agent or Resource of the project;
  - a name listed in `external`;
  - `root`;
  - `<pool>.<entry>`, meaning the Resource of that entry that is not retired (POOL-014). A dot never occurs in a name (OBJ-001), so this form cannot be ambiguous.
- The local name is the grant's own key (OBJ-022 grammar).
  - The consumer and the local name identify the Conduit (FIL-006).
  - Its tools are named `<local>.<export>` (IFC-012).
- `to`: required. A name (an entry of the project, a name in `external`, or a first-start name), `{kind, name}`, or `{kind: Pool, name, entry}`.
- Inline form:
  - `capability`: required. It is never inferred (CON-016).
  - `operations`: a list of fully qualified selections, required, possibly empty (CON-017, ACC-003, ruling 25, ruling 53 #8).
  - `requirements`: a map from name to `{requirement, arguments?}`, default `{}`. This covers filters, `bound.v1` and `shared-host.v1` (CON-018/006/022/028, NAM-019).
- Or the template form: `from` (a Conduit template) and `arguments` (INS-023).
- `annotations`.
- No `name`, `consumer`, `kind`, `id` or `optional` key.

DIRECTORY
- A project is its main file plus every other top-level `.yaml`, `.yml` or `.json` file whose name does not begin with a dot (MAN-011, SUB-010). Links are followed, and an entry that cannot be read is rejected.
- The sections of all the files are merged.
- A key that appears twice in one section is rejected (SUB-004).
- Names are unique across every section except `grants`, whose keys are scoped by consumer (NAM-027).

WIRE
- register.v1 takes `{project: <one merged file value>}`. A partial file is still a file.
- inspect.v1 returns an object in the same entry shape, resolved:
  - references as `{kind, id}`;
  - defaults filled;
  - the packages the object holds;
  - `source`, `derivedBy` and the lineage;
  - the inventory (ruling 53).

SUMMARY OF DEFAULTS

| Key | Default |
| --- | --- |
| `config` | `{}`, then the Type's defaults |
| `arguments` | `{}` |
| `implementation` | the Type's defaults |
| `controls` | every control the Type lists |
| `owner` | the project Pool |
| `operator` | `root` |
| `placement` | PLC-002 |
| `runtime` | the engine's own runner |
| `requirements` | `{}` |
| `packages` | `{}` |
| `external` | `{}` |
| `optional` | false |
| capability | never defaulted |
| operations | never defaulted |

## meaning
The spec would define the language directly, over the file and the stored objects. No second, one-document-per-object syntax remains, and no kernel document form sits underneath. Meaning comes from nine new promises (FIL-001 to FIL-009) and from today's promises, which keep applying through one device: the anonymous template.

1. Anonymous template (FIL-004)
- An Instance or Entry that has no `from` is made from the anonymous template its definition keys form. That template has no inputs, is not registered, has no name, and is instantiated exactly once.
- A Grant without `from` is made from the anonymous Conduit template of its capability, operations and requirements.
- So every promise about templates and instantiation applies to what authors write, word for word, with no second wording: TPL-*, INS-003 to INS-024, CON-016 to CON-022, PLC-005/025, MAN-017/018, SEC-021, CTR checks and INS-013's full resolved definition.
- The anonymous template keeps the submission's pins (NAM-009), and the instance holds them (PKG-005). NAM-010 resolves through them.
- An instance with `from` resolves exactly as today.

2. Sections and keys (FIL-001, FIL-002)
- The section is the family, so the kind is fixed by the section. The key is the name.
- `apiVersion` is stated once per file, so the version is known per unit (ruling 35's purpose).
- `project`, `packages` and the other project-level keys live only in the main file, which is the manifest.

3. Grants (FIL-005, FIL-006)
- A grant makes exactly one Conduit. Its consumer is the section key, its local name is the grant key, its target is `to`, and it has one capability and its listed operations (CON-001).
- A Conduit has no name of its own. Among the Conduits that are not removed, its identity is (consumer, local name), plus its ID.
  - IFC-014 already makes that pair unique.
  - LIF-035 removes a Conduit when it is closed, so the pair is always free or unambiguous.
- References to a Conduit, in open.v1, close.v1, inspect.v1 and on display in records, take the form `{consumer: <ref>, name: <local>}`.
- Tool names stay `<local>.<export>` whatever happens to the target. This is ruling 53's `as`, by construction.

4. Names (FIL-003)
- A bare name resolves among three sets: the project's entries, the names in `external` (whose kinds are given), and the five first-start names.
- Any other bare name is rejected, with a diagnostic that names it and suggests `external`.
- Each name resolves once, to an ID, at registration (OBJ-009), and is never resolved again (OBJ-010).
- Which object an external name reaches is the only state the meaning depends on. It is shown by ID in the result and in the plan.

5. Derivation (FIL-007): the only derived defaults
- `project: P` makes a Pool P of `core#pool.v1` with `config: {}`. Every Resource that names no owner is owned by P.
- Each Pool of the file whose `operator` is not `none` also gets a Conduit:
  - its consumer is the `operator` Interface named in the text, never whoever submits;
  - its local name is the Pool's name, and its target is the Pool;
  - its capability is `core#pool-management.v1`;
  - it lists every operation that capability has in the core version the file's apiVersion names, written out in full in the stored Conduit: register, register-replacement, inspect, set-up, retire, retire-with-resources and reconcile;
  - it is registered and not opened (SUB-006), and it is stored with `derivedBy: FIL-007`.
- Nothing else is derived: no turn grant, no capability, no operation list, no bound.
- If an object named P already exists, the submission is rejected under OBJ-002. An existing Pool is never adopted (skeptic D#4).

6. Provenance and diagnostics (FIL-008)
- The engine records `source` (the project and a pointer into the file) from the text it expanded itself, so no client asserts it (C's D1), and `derivedBy` on derived objects.
- INS-013's observable is amended to allow `derivedBy` on objects. Values are still never marked.
- Every diagnostic names the file, a JSON Pointer into the author's text, the line and column where the text was read from a file, and the promise ID (SUB-005, ruling 52).
- A diagnostic about a derived object points at the `project:` key or the Pool's key.
- Values at places marked secret are redacted (SEC-003).

7. Registration (SUB-001 to SUB-006, OBJ-002, OBJ-011)
- A file only adds objects. Registering the same file twice is rejected.
- No object, template or instance, has its definition rewritten by any document or operation. Only a control changes configuration (REV-*).
- Submissions are all or nothing, so a template and the instances made from it in one submission succeed or fail together.

8. Contract identity (SUB-008)
- The digest is RFC 8785 over the contract sections, ordered by kind and then name, plus the package-level keys (`address`, `version`, `packages`, `implementations`, `exports`).
- It never covers the object sections, so editing a grant never changes contract identity.

THESIS 2 FOR WHAT AUTHORS WRITE
- What a set of files means is the set of stored objects that FIL-001 to FIL-009 and the unchanged promises make. It is fixed by:
  - the text;
  - its apiVersion;
  - the pins it holds;
  - the IDs its external names resolve to.
- No default depends on the caller, the engine's history, the environment, the working directory or a tool.
- The core cites a promise for every object it makes from a file: FIL-004 or FIL-005 for each entry, FIL-007 for each derived object.
- Conformance cases are directories in this form. Today's `resolve-project` and `register-project` steps assert the resolved objects, so the meaning of the author's own text is tested directly, not through a compiler mirror that carries no binding vectors.

WHAT THESIS 2 DOES NOT COVER
- How a change is carried out:
  - the order of requests;
  - the names of successors (`shared--2`);
  - which requests the plan picks.
- That belongs to the plan tool, and every request it sends has normative meaning.

## spec_plan
The counts are mine, from regex scans of spec/ and from reading the key promises. The 455 promises would become about 460: 6 removed, about 11 new. About 101 promises are touched in all, about 22%.

REMOVED (6; 1,313 words, measured)
- CON-002: declared Conduits are made with the instance.
- CON-004: setup opens declared Conduits.
- ACC-005: authorization of that setup part.
- LIF-029: replacing a declared Conduit that is not retired.
- SUB-007: folded into OBJ-002 and OBJ-011.
- SUB-009: replace.v1.
- Optional further cut, put to the owner apart: remove Pool entries as well. That removes about 17 more promises (POOL-002/003/005/006/007/009/011/012/013/014/015/016/017, OBJ-014, LIF-008/009/033) and the rest of register-replacement.v1, but it fails ruling 4's guard for S4's composite setup unless the owner rules.

AMENDED (about 84; 77 found by scanning for the removed constructs, plus about 7 more)
- A. File form in place of the document frame (20): OBJ-012, -013, -017, -018, -019, -020, -022, -024, INS-001, CTR-001, MAN-001, -003, -004, -011, SUB-004, -005, -010, SCH-012, -013, -018.
- B. Inline definitions (11): INS-002, INS-013, INS-019, INS-025, NAM-009, NAM-010, PKG-005, PKG-007, OBJ-003, TPL-001, TPL-002.
  - OBJ-003: generated names remain only for entries' Resources.
  - TPL-001: no `conduits` key.
  - INS-002: every instance has a template, possibly an anonymous one.
- C. Grants identified by (consumer, local name) (11): OBJ-002, OBJ-006, OBJ-011, CON-012, CON-016, CON-020, CON-027, IFC-012, IFC-014, NAM-003, NAM-027.
- D. Declared-Conduit clauses deleted (about 26): CON-031, FST-002, INS-006, INS-011, INS-012, INS-020, NAM-018, NAM-022, NAM-028, OBJ-010, OBJ-014, OBJ-015, POOL-012, POOL-015, PRV-007, SEC-008, SEC-010, TPL-012, TPL-017, ACC-033, LIF-015, LIF-027, OPS-022, OPS-024, OPS-029.
  - TPL-012's setup becomes: entries (Pools only), then the run and remote connections, then the hook.
  - Where TPL-012 said "a run starts only after its declared Conduits are open", it now says the run's reads go only through open Conduits and otherwise fail (SEC-010).
- E. Clauses about replace.v1 deleted (5): NAM-021, PKG-006, ACC-030, REV-011, IFC-033.
- F. register-replacement.v1 for entries only, built from the held entry definition and not from "the template as it now stands" (6): LIF-008, LIF-009, LIF-033, POOL-003, POOL-017, PRV-015.
- G. Project Pool (2): POOL-001 (the default owner) and POOL-004.
- H. Ruling 53 additions (3, needed by either route): LIF-037 (expectedId on open.v1 and close.v1), LIF-013 (inventory), IFC-033 plus the `description` key (descriptive tools).
- I. SUB-008 (digest over the contract sections and package keys).
- J. Contract wording "the `spec` of" becomes "the body of" (8): CTR-002 to CTR-006, CTR-008, CTR-012, SCH-015.
- Most of D, E and J delete or reword text without changing meaning. About 26 amendments change meaning.

NEW (about 11)
- FIL-001: the file, apiVersion once, closed sections.
- FIL-002: directory, merge, one version per project.
- FIL-003: name resolution through `external` and the first-start names.
- FIL-004: anonymous template.
- FIL-005: a grant makes a Conduit.
- FIL-006: Conduit identity and the reference form.
- FIL-007: project Pool and operator grants.
- FIL-008: source, derivedBy and diagnostics in the author's text.
- FIL-009: register.v1 takes a file value, and a partial file is a file.
- LIF-038: tombstones (accepted).
- LIF-039: lineage through `succeeds`, which carries tombstones to a successor and maps file keys to live objects. This one is new beyond ruling 53 and needs the owner.

SCHEMAS
- envelope.yaml, document.yaml, instances.yaml, templates.yaml and manifest.yaml (about 1,500 lines) are replaced by one file.yaml of about 800-1,100 lines.
  - `conduit-file.v1` with `$defs`: instance, entry, template, grant, implementation-item.
  - The template and grant bodies `$ref` today's template-spec, conduit-template-spec, template-input and implementation-map definitions, which are kept.
- contracts.yaml keeps the contract bodies and drops the frame.
- conduit-reference.v1 becomes `{consumer, name}`.
- values.yaml and schema.yaml are unchanged.

CORE PACKAGE (version 0.2.0, since a removed operation is not an addition under NAM-024)
- All spec/core files are rewritten mechanically into sections.
- register.v1's inputSchema becomes `{project: conduit-file.v1}`.
- replace.v1 is deleted. object-management.v1 and root-management no longer list it.
- register-replacement.v1: `nested` becomes `entry`.
- open.v1 and close.v1: Conduit handle plus `expectedId`, and `acknowledge` for tombstones.
- inspect.v1:
  - takes the handle for Conduits;
  - its output gives `declared` in entry shape, with `source`, `derivedBy` and lineage;
  - its output gives the inventory of the Conduits the object consumes and is the target of, listing only what the caller may inspect, with no counts;
  - its output gives the tombstones; who closed or retired an object is shown only to root and to that closer.
- The root template no longer declares Conduits. First start makes root/management and root/access from root-management and root-access (FST-002). Tool names are unchanged.

HOW THE FIVE ACCEPTED ADDITIONS FIT
1. `as`: native. The local name is the grant key, so no separate argument is needed.
2. expectedId on open and close: LIF-037.
3. Inventory: LIF-013 and inspect.v1.
4. Tombstones: LIF-038.
   - Closing a Conduit leaves a tombstone keyed by (consumer lineage, local name).
   - Retiring an instance leaves one keyed by (kind, name, lineage).
   - register.v1 and open.v1 report conflict for a matching Conduit or instance unless the request lists the tombstone in `acknowledge`.
   - Tombstones are kept apart from records, so REC-004 does not purge them. Only root purges them.
5. Descriptive tools: IFC-033 names the target, its `description` and its Type's description.

RULINGS TO AMEND (the owner)
- 8 IN9 and 18 V2: every instance comes from a template, now possibly an anonymous one.
- 29: declared Conduits and their replacement are gone, and setup opens nothing.
- 31, 35 and 37: the frame.
- 41: CON-004's carried content.
- 53, decision 6: only the project Pool and its operator grant are derived; turns are explicit.
- Rulings 21 and 22 are kept, and immutability is stronger, since templates are immutable too.

DELETED OUTRIGHT
- The document frame (`apiVersion` per document, `kind`, `metadata`, `spec`, `---`).
- Declared Conduits and the `conduits` map.
- Template replacement and its special project context (D11).
- Generated Conduit names.
- The standalone/declared distinction.

## s1_rewrite
The counting rule is the measurement plan's: non-blank authored lines, a separator counts, comments do not, and each command line that carries a value the author chose counts separately. Lines marked TOOL are written by `conduit install` and are not counted. The store was made by `conduit init`, which records it beside the file and is not counted, as in the synthesis. The Agent packages are 0.1.2, with the reach flags defaulting to false (ruling 53 #7). The files and the counter are in <scratch>/smallcore/ (t1.yaml, t2.yaml, t4.yaml, t5.yaml, t6.yaml, t7-revoked.yaml, t7-restored.yaml, count.py).

=== T1: 19 file lines + 1 command line = 20 ===
$ conduit install <lab>/pkgs/filesystem-0.1.0.tgz <lab>/pkgs/agent-claude-code-0.1.2.tgz <lab>/pkgs/agent-codex-0.1.2.tgz   [1]
conduit.yaml:
```yaml
apiVersion: github.com/shrav-k/conduit/v0
project: s1
packages:                                                                                  # TOOL
  example.org/s1/filesystem: {version: 0.1.0, digest: 'sha256:200de7c1...'}                 # TOOL
  example.org/conduit/agent-claude-code: {version: 0.1.2, digest: 'sha256:...'}             # TOOL
  example.org/conduit/agent-codex: {version: 0.1.2, digest: 'sha256:...'}                   # TOOL
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
grants:
  root:
    agent-a: {to: agent-a, capability: example.org/conduit/agent-claude-code#conversation.v1, operations: [example.org/conduit/agent-claude-code#turn.v1]}
    agent-b: {to: agent-b, capability: example.org/conduit/agent-codex#conversation.v1, operations: [example.org/conduit/agent-codex#turn.v1]}
  agent-a:
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
  agent-b:
    project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
$ conduit apply   [no value]

Plan, abridged (operator root):
- `+ Pool s1`. Rule FIL-007, from line 2.
- `+ Conduit root/s1 -> Pool s1`, `core#pool-management.v1` with all seven operations. Derived by FIL-007; registered and not open.
- `+ Resource shared`, owner s1 by default.
- `+ Agent agent-a, agent-b`. The filled config is shown: accountServices false, peerMessaging false, model ''.
- `+ Conduit root/agent-a, root/agent-b`, [turn.v1].
- `+ Conduit agent-a/project`, [read, list].
- `+ Conduit agent-b/project`, [read, list, write].

Requests:
1. register (one submission);
2. open root/s1;
3. set-up shared (ACC-033 row 4 through root/s1);
4. open agent-a/project and agent-b/project, with expectedId;
5. set-up agent-a and agent-b;
6. open root/agent-a and root/agent-b.

That is 12 requests with the 3 installs.

Tools:
- root: `s1.*`, `agent-a.turn.v1`, `agent-b.turn.v1`.
- A: `project.file.read.v1`, `project.directory.list.v1`.
- B: the same, plus `project.file.write.v1`.

=== T2: 5 file lines + 1 command line = 6 ===
$ conduit install <lab>/pkgs/history-0.1.0.tgz   [1; TOOL adds `example.org/s1/history: {version: 0.1.0, digest: 'sha256:e9467465...'}`]
Added under `resources:`:
```yaml
  history:
    type: example.org/s1/history#file-history.v1
    config: {root: <lab>/history}
```
Added under `grants: agent-a:` and under `grants: agent-b:`, one line each:
```yaml
    history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
$ conduit apply

The plan sends one partial file:
- `external: {s1: Pool, agent-a: Agent, agent-b: Agent}`;
- `resources: {history: {..., owner: s1}}`;
- the two grants.

Requests: register (ACC-033 row 1, through management and root/s1), set-up history, and open both grants. The posting convention reaches both Agents through the tool descriptions (ruling 53 #5); no author line is spent on it.

=== T3: 0 lines ===
$ conduit reach   [no value]

- The answer for the engine is the `grants:` section of one file, confirmed against the live inventory of inspect.v1: each handle, ID, target, operations and source line.
- Reach around the engine comes from the Agent Types' statements (provider work).
- Inspection effort: 1 command.

=== T4: 2 lines ===
Edit two lines:
- `grants.agent-a.project`: operations become [read, list, write], adding `example.org/s1/filesystem#file.write.v1`.
- `grants.agent-b.project`: operations become [read, list].

$ conduit apply

The plan narrows first:
1. Close agent-b/project (expectedId). The engine leaves tombstone t1.
2. Register agent-b/project with [read, list], acknowledging t1. Open it (expectedId).
3. Then the same for agent-a/project with [read, list, write].

That is 6 requests. Tool names are unchanged. Each Agent is without access only between its own close and open, and the two Agents never write at the same time.

=== T5: 0 file lines + 1 command line = 1 ===
$ conduit install <lab>/pkgs/filesystem-0.2.0.tgz   [1; TOOL rewrites the pin to `{version: 0.2.0, digest: 'sha256:cbb831f0...'}`]
$ conduit apply

Plan: shared's Type comes from filesystem, whose pin changed, so shared is replaced make-before-break.
1. Register `resources: {shared--2: {type, config: {root: <lab>/shared}, owner: s1, placement: <shared's node>, succeeds: shared}}`, and set it up.
2. Close agent-a/project and agent-b/project (expectedId), then wait until nothing is in flight.
3. Register both grants with `to: shared--2`, acknowledging the plan's own tombstones, and open them (expectedId).
4. Retire shared (expectedId).

What the plan checks and shows:
- The contracts of the granted operations and the tools map are unchanged.
- No Agent gains append_file or copy_file (G1).
- history and its grants are unchanged: only the package of an instance's own Type triggers a replacement, and a grant's capability pin is not compared.

About 9 requests.

=== T6: 1 line ===
Edit `config: {root: <lab>/shared}` to `config: {root: <lab>/shared-new}`.

$ conduit apply

- The same replacement as T5: shared--3, with `succeeds: shared--2`. The grants are re-pointed under the unchanged names `project.*`, and shared--2 is retired.
- The tool descriptions now name shared--3, and IFC-038 tells each Agent that its list changed.

=== T7: 1 line (removed, then restored; it counts once by the baseline's rule) ===
Revoke: delete the line `project: {...}` under `grants: agent-a:`.

$ conduit apply

- The plan closes agent-a/project (expectedId).
- Tombstone t3 names this apply's invocation.
- Check B (not counted): B still reads, writes and posts.

Restore: put the same line back.

$ conduit apply

- The plan reads: "+ agent-a/project; t3 was closed by this working copy's apply <id>, so it is acknowledged without a flag".
- It sends register (acknowledge t3) and open (expectedId).

An alternative route is `conduit close agent-a/project` followed later by `conduit apply --reopen agent-a/project`: 2 command lines. A tombstone left by any other actor always needs `--reopen` with its ID.

Final authored file: 24 lines.

## s1_counts
Primary reading (the measurement plan's rule): file lines, plus command lines that carry values. Resources and agents are written in block style, grants in flow style, as in the synthesis.

| Task | File lines | Command lines with values | Total | Baseline A70 | vs A70 | Layered package | vs layered |
|---|---|---|---|---|---|---|---|
| T1 | 19 | 1 | 20 | 33 | better (-13) | 16 | +4 |
| T2 | 5 | 1 | 6 | 16 | better (-10) | 6 | same |
| T3 | 0 | 0 | 0 | 0 | same | 0 | same |
| T4 | 2 | 0 | 2 | 8 | better (-6) | 2 | same |
| T5 | 0 | 1 | 1 | 3 (better baseline 2) | better (-2 / -1) | 1 | same |
| T6 | 1 | 0 | 1 | 2 | better (-1) | 1 | same |
| T7 | 1 | 0 | 1 | 2 | better (-1) | 1 | same |
| All | 28 | 3 | 31 | 64 | better on 6, same on T3, much more on none | 27 | +4, all in T1 |

The +4 in T1 against the layered package is 3 lines of explicit turn grants from root plus the `grants:` header. This design derives no turn grants, under ruling 52 and decision 6.

Sensitivity:

| Reading | T1 | T2 | T3 | T4 | T5 | T6 | T7 | Total | Bar |
|---|---|---|---|---|---|---|---|---|---|
| Flow style for instances (`shared: {type, config}`) | 16 | 4 | 0 | 2 | 1 | 1 | 1 | 25 | Pass, better on 6 |
| Turns derived as in the layered package (needs decision 6 extended) | 17 | 6 | 0 | 2 | 1 | 1 | 1 | 28 | Pass, better on 6 (flow style: 13 in T1, 22 in all) |
| Restore in T7 counted as a second line | 20 | 6 | 0 | 2 | 1 | 1 | 2 | 32 | Pass, better on 5 |
| Block style for every grant (3 lines each) | 28 | 10 | 0 | 2 | 1 | 1 | 3 | 45 | Pass, better on 5; T7 1 more |
| Strict: every command is a line (install, apply, reach) | 21 | 7 | 1 | 3 | 2 | 2 | 3 | 39 | Against A70: better on T1, T2, T4, T5 (4 of 7). Against the better T5 baseline: 3 of 7, which FAILS "most". The layered package fails this reading the same way; it is decided by decision 2 and decision 11, not by the form |
| JSON, one key per line | about 45 | about 15 | 0 | 2 | 1 | 1 | about 3 | about 67 | FAILS. The spec's examples must be YAML |

Other measures (on paper):
- Grants made by hand: T1 2 Agent grants, plus 2 root turn grants listed apart; T2 2; T4 2 edits; T7 1. Derived: 1, root/s1.
- Lines naming a contract address: 9 (3 Types, 6 grant lines).
- Provider-specific lines: 0.
- Tool names changed by any task: 0.
- Engine requests sent by the tool: T1 12, T2 5, T4 6, T5 about 9, T6 about 8, T7 1 + 2. About 43 in all, against the coordinator's 35 typed.
- Concepts the author meets: about 13-15. They are apiVersion, project (Pool and operator), resources, agents, type, config, grants (consumer, local name, `to`, capability, operations), root, packages and install, plan and apply, and tool names. Today's count is 16 and the baseline's 20.
- Files edited: 1.
- Duplicated values at the end: the capability ×4, the read/list operations ×4, `shared` ×2, `history` ×3, each Agent's key ×3.
- Windows without access: T4 one Agent at a time; T5 and T6 both Agents between close and open, as in the layered package (critic gap 7). That waits for swap.v1 in stage 2.

All of these are paper counts, which ruling 45 does not accept as evidence. Only a rerun counts.

## other_scenarios
Every construct of today's canonical documents has a counterpart in a section, so no escape hatch into a second syntax is needed. What cannot be expressed is either what has no promise yet, or what this design removes. Both are listed at the end. Addresses are abbreviated as `core#`, `lin#` and so on.

S2: Linear behind 1Password
```yaml
project: s2
pools:
  creds: {type: core#pool.v1, operator: root}        # separate authority over secrets; derived root/creds
resources:
  op-token: {type: keychain#keychain-secret.v1, owner: creds, config: {service: conduit, account: op-sa}}
  linear-key: {type: op#op-secret.v1, owner: creds, config: {locator: 'op://test/linear/key', token: op-token}}  # secret-reference place (SEC-001)
  linear: {type: lin#linear.v1, config: {token: linear-key}}
  linear-adapter: {type: lin#adapter.v1}             # Conduit-aware, supports lin#team.v1
agents:
  reader: {type: cc#claude-code.v1}
  writer: {type: cx#codex.v1}
grants:
  linear-key: {token: {to: op-token, capability: core#secret-provider.v1, operations: [core#get-secret.v1]}}
  linear: {token: {to: linear-key, capability: core#secret-provider.v1, operations: [core#get-secret.v1]}}
  linear-adapter: {upstream: {to: linear, capability: lin#issues.v1, operations: [lin#list.v1, lin#get.v1, lin#search.v1, lin#create.v1, lin#update.v1, lin#comment.v1]}}
  reader: {linear: {to: linear-adapter, capability: lin#issues.v1, operations: [lin#list.v1, lin#get.v1, lin#search.v1], requirements: {team: {requirement: lin#team.v1, arguments: {team: ENG}}}}}
  writer:
    linear: {to: linear-adapter, capability: lin#issues.v1, operations: [list, get, search, create, update, comment, all qualified]}
    delegate: {to: access, capability: core#access-control.v1, operations: [core#open.v1], requirements: {bound: {requirement: core#bound.v1, arguments: {targets: [linear-adapter], operations: [lin#list.v1]}}}}
  root: {reader: {...turn}, writer: {...turn}}
```
- Only root opens the get-secret grants (SEC-008); the planner opens them before set-up.
- If linear is set up before its token grant is open, the run's read is refused (SEC-010) and the setup fails closed, with a reason naming the grant. TPL-012's ordering is no longer the guard.
- E13 rotation: `conduit apply --replace linear` is 1 command line. The plan registers linear--2 (succeeds linear), re-points `upstream` and the token grant, then retires linear. The cascade is a plan, not language meaning.
- The expected account (ruling 49) cannot be expressed until its field is written. It would be a config key of the Agent Type, or a standard key if the owner wants it in the language.

S3: two machines
```yaml
external: {m2: Resource}                             # a node, made by enrollment (a request, not a document)
resources:
  tailscale: {type: ts#machine-provider.v1, owner: system}   # PRV-013; needs a root grant to Pool system, written explicitly
  m2-docker: {type: dk#docker-context.v1, config: {host: 'ssh://m2'}}
  shared: {type: fs#local-directory.v1, config: {root: /work}, placement: m2, runtime: m2-docker}
agents:
  agent-b: {type: cx#codex.v1, placement: m2, runtime: m2-docker}
grants:
  agent-b: {shell: {to: workbench, capability: ..., operations: [...], requirements: {same: {requirement: core#shared-host.v1}}}}
  root: {system: {to: system, capability: core#pool-management.v1, operations: [core#register.v1, ...]}}
```
- nodeSelector stays on Types and items.
- A partition or a lost machine needs no authoring.
- Peer mode cannot be expressed: there is no Interface that stands for another engine's actor and no trust domain. That needs ruling 51's promises first. The likely home is `interfaces:` plus a key in `external`.

S4: setup that fails partway
- Pool entries are kept:
```yaml
pools:
  hooks:
    type: core#pool.v1
    entries:
      secret: {type: kc#keychain-secret.v1, config: {...}}
      service: {type: lin#linear.v1, config: {token: {entry: secret}}}
      webhook: {type: lin#webhook.v1, config: {service: {entry: service}}, optional: true}
grants:
  hooks.webhook: {svc: {to: {kind: Pool, name: hooks, entry: service}, capability: ..., operations: [...]}}
```
- The webhook Type's `requires` selects setup-hook.v1 and retire-hook.v1.
- The derived root/hooks lists reconcile.v1.
- Retry is set-up again. Nothing rolls back.
- Lost: optional Conduit parts of a setup (no declared Conduits remain). Optional entries remain.

S5: a control under contention
```yaml
resources:
  store:
    type: kv#store.v1
    config: {limit: 20}
    controls: {kv#set-read-limit.v1: {inputs: {limit: {schema: {type: integer, maximum: 50}}}}}
grants:
  agent-a: {store: {to: store, capability: kv#kv.v1, operations: [kv#get.v1, kv#set-read-limit.v1]}}
  root: {store: {...same...}}
```
- Contention uses revisions (REV-002).
- The planner turns a file edit at a control pointer into a control request, with the revision as its precondition. A live value changed by another actor's control is shown as drift and never reverted without a flag.

S6: revocation and retirement in flight
- Removing a grant line is an explicit close (expectedId) that leaves an engine tombstone.
- Removing a Resource is refused without `--prune`. With it, the plan retires the Resource, and CON-008 closes the Conduits to it.
- The adapter's onward grant is an ordinary grant line under the Resource.
- In-flight behaviour is the engine's (ACC-034, ACC-039, IFC-013, IFC-038).

S7: an outside author packages a server and a plugin, in the same form
```yaml
apiVersion: github.com/shrav-k/conduit/v0
address: example.org/acme/search
version: 0.1.0
exports: [search.v1, query.v1, find.v1, visible-prefix.v1, server, plugin, searcher]
types:
  search.v1:
    family: Resource
    description: Full-text search over a directory.
    schema: {type: object, properties: {token: {$ref: core#secret-reference.v1}}, required: [token]}
    offers: ['#query.v1']
    nodeSelector: {kubernetes.io/os: linux}
    records: {inputs: true, outputs: false}
capabilities: {query.v1: {operations: ['#find.v1']}}
operations: {find.v1: {inputSchema: {...}, outputSchema: {...}}}
requirements: {visible-prefix.v1: {inputs: {prefix: {schema: {type: string}}}}}
implementations:
  server: {implements: ['#find.v1'], package: {...oci with digest...}, plain: true, tools: {'#find.v1': search}, environment: {TOKEN: /token}}
  plugin: {implements: ['#find.v1'], command: [...], supports: ['#visible-prefix.v1']}
templates:
  searcher: {kind: Resource, type: '#search.v1', inputs: {token: {schema: {...}, to: [/config/token]}}}
```
- One syntax for projects and packages.
- SUB-008's digest covers these sections and the package keys.

S8: filtered search
- `search` holds a grant line `docs` to the storage Resource.
- Agents hold grant lines to search, each with its own requirement arguments: `visible-prefix` of `/public` for one, of `/` for another, and a deadline filter for a third.
- Both implementations list the filter in `supports` (CON-022).
- The search Type sets `records.outputs: false`.

S9: first start and repair
- The trust domain, hostname, runner list and store are engine configuration, outside the file.
- Root's Conduits are first-start objects: root/management and root/access.
- Providers owned by `system` need an explicit grant from root to `system`.
- Explicit repair is a request in each case: set-up again, reconcile, declare-lost, unregister, or `conduit apply --replace X`.
- The file, the inventory and the observed status are three different states, and nothing is ever repaired without a request (OBS-004, LIF-008).

S10: every provider swapped
- A new store Type: a new secret Resource, because a locator is fixed (SEC-005).
- A Docker runtime: `runtime: docker` on each instance, which means a replacement of each (PRV-006). Items must be OCI with digests.
- A Git package source, owned by system, claiming a scheme.
- Each swap is a cascade the planner lists. Replaced Agents must sign in again (ruling 49). The cascade is tooling, not meaning.

The owner's real use (10 workers)
```yaml
project: conduit-dev
templates:
  codex-worker: {kind: Agent, type: cx#codex.v1}
  worktree: {kind: Resource, type: git#worktree.v1, config: {repository: <repo>}, inputs: {branch: {schema: {type: string}, to: [/config/branch]}}}
  rw: {kind: Conduit, capability: git#tree.v1, operations: [git#read.v1, git#list.v1, git#write.v1]}
  ro: {kind: Conduit, capability: git#tree.v1, operations: [git#read.v1, git#list.v1]}
  queue: {kind: Conduit, capability: q#queue.v1, operations: [q#read.v1, q#post.v1]}
  turn: {kind: Conduit, capability: cx#conversation.v1, operations: [cx#turn.v1]}
resources:
  main: {type: git#checkout.v1, config: {path: <repo>}}
  orders: {type: q#queue.v1}
  tree-1: {from: worktree, arguments: {branch: wo/101}}    # one line per worker
agents:
  admin: {type: cc#claude-code.v1}
  worker-1: {from: codex-worker}                           # one line per worker
grants:
  admin: {main: {to: main, from: rw}, orders: {to: orders, from: queue}, worker-1: {to: worker-1, from: turn}, tree-1: {to: tree-1, from: ro}}
  worker-1: {tree: {to: tree-1, from: rw}, main: {to: main, from: ro}, orders: {to: orders, from: queue}}
  verifier: {tree-1: {to: tree-1, from: ro}}
```
- Cost: about 9 lines per worker in block-map style (about 6 if each consumer's grants are written as one flow map), plus about 25 shared lines. That is about 85-115 lines for 10 workers, against about 440-520 today. The floor is about 1 line per worker plus 15.
- What cannot be expressed:
  - one line per worker, which needs a composite template that makes an Agent, its tree and their grants together, plus derived names, which run into IN5;
  - a template that bundles relationships, now removed.
- Each goes to the owner; there is no fallback syntax.
- The admin creating workers needs broad trust (`management` plus `access` bounded to conduit-dev) or a broker (D04).
- Upgrading the Agent package replaces every worker: 10 sign-ins.

CHECKLIST (the inventory's 15 items)

| # | Item | Status |
|---|---|---|
| 1 | Kinds and names | ✓ section and key |
| 2 | Closed keys and diagnostics | ✓ |
| 3 | Standard arguments | ✓ |
| 4 | Every grant | ✓, except `optional` on Conduits (removed) |
| 5 | Types and implementations | ✓ |
| 6 | Pools | ✓, entries kept |
| 7 | Secrets | ✓ unchanged |
| 8 | Packages | ✓, digest redefined |
| 9 | Contracts | ✓, all five kinds as sections |
| 10 | Reuse | LIF-009 kept for entries only, built from the held definition; INS-020 kept; exported templates kept; Conduit ordering moves to the planner |
| 11 | Change | ✓, stronger immutability |
| 12 | Determinism | ✓ |
| 13 | The wire | ✓ register.v1 and inspect.v1 in file form; replace.v1 removed |
| 14 | Files | ✓ |
| 15 | The 36 promises outside S1 | each kept, since requirements, controls, remote, placement, entries, providers and secrets are unchanged or reworded |

## change_model
Change is not part of the language. It is ruling 53 #4's explicit plan/apply, composed only of core requests and never automatic. The plan is reference tooling for v0, with golden (file, state) -> plan cases marked not binding. It could be promoted later, at about 25-30 promises. The plan calls the core's own pure expansion, so unlike the layered route no compiler mirrors the core.

THE CORE REQUESTS IT USES (this design leaves no others)
- register.v1, with a partial file;
- open.v1 and close.v1, with the handle, expectedId and `acknowledge`;
- set-up.v1;
- retire.v1, with expectedId;
- unregister.v1, for templates and retired instances;
- register-replacement.v1, for entries only;
- controls, with a revision;
- install.v1.

There is no replace.v1, no revise, no swap and no restart in place.

WHAT THE PLAN READS
1. The file.
2. The inventory (ruling 53), filtered by what the caller may inspect.
3. The engine's tombstones and the lineage given by `succeeds`. Because these are held by the engine, losing the local applied-ID file, or a Pool, does not erase them (fixes D#5).
4. The tool-written applied-ID file.

CLASSES OF CHANGE

| Class | Change in the file | What the plan sends |
|---|---|---|
| C1 | Grant added | register, then open (expectedId = the ID register returned) |
| C2 | Grant removed | close (expectedId). The engine leaves a tombstone keyed by (consumer lineage, local name) |
| C3 | Grant's `to`, capability, operations or requirements changed | close, then register under the same handle acknowledging the plan's own tombstone, then open. Narrowings always run before widenings (fixes D#10) |
| C4 | A Resource's type, config, placement or runtime changed, or the pin of the package that exports its own Type (D cost D2) | Make before break: (1) register `<key>--N` with `succeeds`, inheriting the predecessor's resolved node unless the file changes it (E#6), and set it up; (2) close every grant to the old Resource and wait until nothing is in flight; (3) register and open the successors under the same handles; (4) retire the old Resource (expectedId). The plan lists every Conduit the retirement will close, including those made outside the file, and refuses without a flag if any would not be re-pointed (E#7) |
| C5 | An Agent changed | Refused without `--replace`. The plan says that the session and the login are lost (ruling 49) |
| C6 | An object removed from the file | Refused without `--prune` |
| C7 | A config value at a control's pointer changed | A control request with the revision as its precondition. A revision change made by a control is known, not drift (critic gap 5) |
| C8 | A reusable template changed | Unregister the template (expectedId), then register it again together with the new instances in one all-or-nothing submission. Existing instances keep their definitions (INS-020). A name taken in between makes the submission fail closed under OBJ-002 |
| C9 | A pin bumped by install | C4 or C5, but only for instances whose own Type's package changed. The plan diffs each granted operation's contract and the implementation's tools map; any difference needs a flag (E#10). A downgrade needs a flag |

SAFETY RULES CARRIED OVER FROM THE SKEPTICS

Held by the engine, so Agents acting through register.v1 and open.v1 meet them too (preservation test 5):
- expectedId on every request that names an object, so a plan cannot act on a swapped object.
- Tombstones enforced at register and open:
  - a grant closed or an instance retired by any other actor, or by a stale working copy, can only be reopened with `--reopen <tombstone ID>`;
  - renaming a key does not clear a tombstone, because the handle's consumer lineage persists;
  - replacing an Agent carries its tombstones to the successor through `succeeds`, which fixes critic gap 2.

Plan rules, which bind the tool only:
- State is read again after approval, and any drift between plan and dispatch is a conflict (D#7). Approving a saved plan binds the plan's digest.
- A close or a narrowing is never blocked by an unrelated failure (D#11).
- Idempotency keys are derived from (object, action, expected ID), and the tool refuses to plan while an invocation of its own is unknown (E#11).
- The tool stops at the first part that does not succeed. There is no rollback, and a stopped apply can be resumed.
- The tool never runs in the background, never prunes or reopens without a flag, and never adopts an existing Pool of the project's name: the first submission fails under OBJ-002.
- Values at secret places are redacted in every output.
- Disclosure: no counts of hidden objects. Who closed something is shown only to root and to the closer.

WHAT STAYS OPEN, AS IN THE LAYERED ROUTE
- Approving a plan without a terminal (decision 2). The options are a saved plan applied by its digest, counted as a line, or one-step commands: `conduit grant`, `revoke`, `upgrade`, `--replace`.
- Windows without access in T4 to T6, until an atomic swap.v1 in stage 2.
- An apply lock against two concurrent applies.

## cost
These are estimates. Nothing was built or run.

SPEC

Promises:
- Removed: 6 (1,313 words, measured).
- Amended: about 84.
  - About 28 are only rewording of the frame or of contract wording.
  - About 30 only delete a clause about declared Conduits or replace.v1.
  - About 26 change meaning.
- New: about 11 (FIL-001 to FIL-009, LIF-038 tombstones, LIF-039 lineage).
- Touched in all: about 101 of 455 (22%). The total goes from 455 to about 460.
- For comparison, design C touched about 80, of which about 34 bore on safety through revise and swap. This design changes none of PLC-009, SEC-014, ACC-024 or the PRV promises.

Words:
- About 1,300 removed with the removed promises, and about 1,500-2,500 in deleted clauses.
- About 2,500-3,500 added.
- Net about ±1% of 83,875.

Concepts for implementers:
- Removed, about 7:
  - the document frame;
  - the manifest as a separate kind of file;
  - declared Conduits;
  - the declared/standalone distinction;
  - generated Conduit names;
  - template replacement and its project context;
  - replacement of declared Conduits.
- Added, about 5: sections and the merge rule, the anonymous template, the Conduit handle, the project Pool with its operator grant, and lineage.
- Both routes add tombstones, the inventory and expectedId on open and close.

Schemas:
- Five of the eight files are replaced by one file.yaml: about 1,500 lines become 800-1,100.
- contracts.yaml loses the frame.
- conduit-reference.v1 becomes a handle.

Core package:
- Rewritten mechanically into the new form, as core 0.2.0.
- replace.v1 is deleted.
- register.v1, open.v1, close.v1, inspect.v1 and register-replacement.v1 change shape.

Examples: all 114 rewritten by a converter, about 10 that rely on the frame replaced, and 20-30 new.

tools/check_schemas.py (2,588 lines): the steps tied to document shapes are reworked, about ±400-900 Python lines.

CONFORMANCE (counts from the build paper, partly re-checked here)
- Inputs converted mechanically: about 280-316 of 367 cases, all those that carry documents. 482 files hold a frame and 161 hold Conduit documents.
- About 140-200 cases get regenerated expectations:
  - the 118 S1 cases, whose IDs shift;
  - the 126 cases with declared Conduits;
  - the first-start operation arrays, since root-management loses replace.v1;
  - the root-system-state.json snapshot.
- About 20-30 cases are deleted or rewritten as refusals: the setup opening of CON-004 and ACC-005, LIF-029, the replace.v1 and E11 cases.
- New cases: about 40-70. They cover:
  - the file form and the merge;
  - external names;
  - inline definitions;
  - handles and IFC-014 by handle;
  - project Pool derivation and the refusal to adopt;
  - diagnostics in the author's text;
  - SUB-008 unaffected by object sections;
  - tombstones with acknowledge, lineage and purge;
  - expectedId on open and close;
  - the inventory's disclosure tiers;
  - entries-only replacement from the held definition;
  - removed operations reporting invalid.
- Fixture archives: all 84 repacked in the new form, which gives new digests (PKG-004), so the pins in about 268 files are rewritten by the converter.
- The Agent packages are rewritten as 0.1.2, which ruling 53 #7 requires anyway.

REFERENCE MODEL
- Loader (2,879 lines): about 800-1,400 lines changed, in project.rs (merge and positions), schemas.rs (the file role) and rules.rs (the frame rules).
- Core (10,465 lines): about 1,700-3,000 lines touched, net change about -200 to +600.
  - registration.rs: prepare from sections, anonymous templates, grants into handles, FIL-007, the declared slot removed and the entries slot kept, replacement for entries only.
  - packages.rs: the digest and the manifest keys.
  - access.rs: references by handle.
  - lifecycle.rs: the declared-Conduit part removed, about 15 lines, plus the ACC-005 path.
  - records.rs: replace.v1 removed, about 120 lines; inspect's inventory and tombstones.
  - lib.rs: root's first-start Conduits.
  - Tombstones themselves: 150-300 lines, needed by either route.
- Shell: the planner, executor, install, render and reach are the same as the layered route's, about 3,000-4,500 source lines and 2,500-3,500 test lines. This route saves about 800-1,200 lines of compiler that would mirror the core, because the core's expansion is called directly. Six shell test files that rely on S1's declared Conduits are rewritten.
- Runner: small.
- A converter for vectors and examples: a one-off of about 400-800 lines, in Rust under experiments/ or as a checker under tools/.

RE-EVIDENCE
- s1-whole and increment 3's run with the real clients (Claude Code and Codex through S1) must be redone on the new form.
- The rerun for increment 4 is needed in any case.

ABOUT 10-14 WORK ORDERS
1. Spec batch: FIL promises, file.yaml, core 0.2.0, the five additions.
2. The converter, and regeneration of vectors and examples.
3. The loader.
4. The core.
5. Packages repacked.
6. check_schemas.py.
7. Planner and install.
8. render and reach.
9. Re-evidence increments 2 and 3.
10. Dry run and fresh rerun.

COMPARED

| Route | Promises | Vectors invalidated | Code |
|---|---|---|---|
| Layered | 11-14 | 0, beyond snapshot updates | A CLI of 3.5-5.5k lines, mirroring about 27 promises, so each later core change costs about 1.3-1.5 times as much. Thesis 2 untested for what authors write |
| Normative expansion into today's documents | 20-30 | 0; 25-45 new | Loader +1.1-2k lines. Two normative forms |
| This design | about 101 touched | about 70-80% of inputs converted, 140-200 regenerated | About 2-3k core lines touched, the same planner. One form, a smaller object model, thesis 2 tested |

CUTTING THE RISK (a staging I would recommend to the owner)
- Stage 1, before the rerun:
  - write FIL-001 to FIL-009 and the grammar in spec/ as the authoring form, implemented in the loader by expansion into today's model, with the five additions;
  - keep today's documents as a deprecated second form, as vectors only;
  - this costs about 30-40 promises and breaks 0 vectors.
- Stage 2, after the rerun shows the savings: delete the frame, declared Conduits and replace.v1, and convert.
- The rerun measures the final grammar either way. Two forms exist only between the stages.

## what_is_lost
1. The long form.
   - No one-object-per-document syntax remains for authors, for diffs in review or for Kubernetes-style tooling.
   - `render` prints the explicit file instead, with every default written out and derived objects marked.
   - Every existing document, fixture archive, digest and store breaks. This is a clean break, acceptable only because v0 has no users.
2. Declared Conduits.
   - A template, a package-exported one included, can no longer bundle relationships. Reuse at scale writes a grant line per instance; the real use costs about 9 lines per worker in block-map style (about 6 flow), not 1.
   - TPL-012's guarantee that Conduits are open before the run becomes "the run fails closed without them" (SEC-010).
   - Setup opens nothing. An Agent set up outside the planner starts with no tools until its grants are opened (G9 weakened, as in the layered route).
   - Optional Conduit parts of a setup are gone (S4 keeps optional entries).
   - The gain is safety: skeptic D's HIGH #2, package templates whose Conduits opened under root's unbounded access, disappears.
3. replace.v1.
   - Templates are immutable. A changed reusable template means unregister and register again, or a new name.
   - The fresh-context replacement of D11 goes away, with its complexity.
4. register-replacement.v1 for Conduits.
   - Restoring a grant is register plus open under the same handle.
   - register-replacement.v1 remains only for Pool entries.
5. Conduit names.
   - A Conduit is known only by (consumer, local name) and its ID.
   - Conduit references, records display and the open, close and inspect inputs change shape.
   - Successor objects carry tool-chosen names (`shared--2`), which no promise determines.
6. Meaning versus change. Thesis 2 covers what the file means, not how a change is carried out: order, successor names and plan choices belong to the tool, as in the layered route.
7. The derivation is now language.
   - The project Pool and its operator grant are fixed by FIL-007, so decision 6 must be settled before the spec batch.
   - Explicit turn grants cost 3 lines in T1, plus the 1-line `grants:` header, so T1 is 20 against the layered package's 16.
8. Duplication. Because definitions are inline, two Agents with the same grant write its operation list twice unless the author makes a template.
9. Rulings to amend: 8 IN9, 18 V2, 29 (declared Conduits and setup), 31, 35 and 37 (the frame), 41 (CON-004's content), and 53 decision 6. Ruling 53 #1 itself, the short form as the language.
10. Evidence and time.
    - Increments 1 to 3 must be re-evidenced, and about 70-80% of the vector inputs converted.
    - The rerun waits for stage 1 at least.
11. Still out of reach in any form until promises exist: peer mode and trust domains, enrollment, pending actions, the expected account, the restart verb, and a composite template that makes an Agent with its own Resources.
12. Also not solved. These need decisions 2 and 11 and a stage-2 swap.v1:
    - the strict command-counting reading, under which S1 is better on only 3 of 7 tasks against the better T5 baseline;
    - authoring in JSON, which fails the bar;
    - the windows without access in T4 to T6.
13. Cut but optional: removing Pool entries too would delete about 17 more promises. It would lose S4's composite setup and entry-stable replacement, so I do not include it without an owner ruling under ruling 4's guard.

## Verdict safety (major)
Holds with fixes, not as written. The engine's access model survives unchanged. Access still goes only through open Conduits with explicit, fully qualified operation lists. Every request is still recorded. close.v1 and CON-008 still revoke at once. Deleting declared Conduits removes skeptic D's HIGH #2, a real safety gain. But seven claims fail against spec/ or the earlier skeptics:
(1) FIL-006 says IFC-014 makes (consumer, local name) unique. It does not: IFC-014 checks tool names only, so two Conduits can share a handle.
(2) Bare names in grant keys and `to` drop OBJ-013's kinds. FIL-003 neither reserves the first-start names nor says which set wins, so an Agent named `root` can capture `grants: root:`.
(3) A narrowing made outside the file, or by another working copy, is undone by the next routine apply. B#1 and D#6 are still open.
(4) Tombstones keyed by (consumer lineage, local name) are bypassed by renaming the grant key, which contradicts the design's own claim. Its own tombstones are also acknowledged without a flag across applies.
(5) `succeeds`/LIF-039 maps file keys to live objects but nothing authorizes or limits it, so another actor can hijack a key.
(6) Silent widening remains:
- `requirements: {}` is the default, so a grant to `access` with no requirements is unbounded.
- C9 compares only operation contracts and the tools map, while Type defaults are frozen only by NAM-024, which nothing checks at install.
- A narrowed template does not reach the grants made from it.
(7) Approval without a terminal is still open, and the line counts assume no approval at all.
None of these is fatal, and most fixes cost 0 lines in S1.

SCOPE. I only read the repository and changed nothing. Promise texts I checked: ACC-006/012/022/024/029/030/033, IFC-012/014, LIF-006/017/035/037, CON-008/017, OBJ-002/006/010/013/017/021, NAM-020/021/024/027, FST-001, SEC-001/008/010, MAN-010/017, REC-002/004, INS-006. I also checked spec/core/capabilities.yaml (pool-management.v1 has the 7 operations the design lists), spec/core/types.yaml ("No core type states records"), spec/core/conduit.yaml (core version 0.1.0, which apiVersion does not name), packages/agent-claude-code/contracts.yaml, and the five skeptic verdicts plus critic.md. Design files: <scratch>/smallcore/.

WHAT SURVIVES (checked)
- Access only through open Conduits listing operations. ACC-001/003/006/012 and IFC-012 are unchanged; capability is never inferred and operations are never defaulted. T5 gives no Agent append_file or copy_file. Ruling 25 is kept.
- Every request recorded. The plan uses only core requests (REC-001/002 unchanged).
- Immediate revocation. close.v1, CON-008 and ACC-034/039 are unchanged, and narrowing runs first.
- Fixes from earlier skeptics that hold:
  - D HIGH #2 (package templates' Conduits opened under root at setup) is gone, because CON-004 and ACC-005 go.
  - The held definition is used for LIF-009 (D#14).
  - Templates are immutable; replace.v1 is gone (E#4).
  - An existing Pool is never adopted (D#4, part).
  - The operator is named in the text, never taken from the caller (C M8).
  - The derived grant includes reconcile.v1 (E#5).
  - expectedId on open and close (E#2, D#9).
  - Engine-held tombstones survive purge and Pool loss (D#5).
  - The inventory gives no counts (critic #8).
  - Placement is inherited by a successor (E#6).

COUNTEREXAMPLES, most severe first

1. MAJOR: the Conduit handle is not unique (FIL-006 is false as written).
- IFC-014 rejects a Conduit only when one of its TOOL NAMES equals another tool of the same consumer, whether or not that Conduit is open yet. Its observable even contemplates two Conduits sharing a local name with no tool in common.
- So these can coexist under the same (agent-a, project):
  - a Conduit listing `operations: []` (CON-017 allows an empty list);
  - one listing only get-secret.v1, which presents no tool (IFC-012);
  - one listing only file.write.v1, beside agent-a/project [read, list].
- ACC-033 row 2 lets any holder of management register.v1 register a Conduit for any consumer.
- Effects:
  - `close {consumer: agent-a, name: project}` is either ambiguous (invalid, so the revocation fails) or closes one of them while the other stays open.
  - Tombstones keyed by the handle are written while a Conduit under the same handle is still live.
  - Tool splicing: an actor with a bounded access Conduit opens agent-a/project [file.write.v1] to its own directory. The Agent then sees `project.file.read.v1` going to shared and `project.file.write.v1` going elsewhere, under one prefix.
- Fix: a new rule in FIL-006 that (consumer, local name) is unique among Conduits not removed, checked at registration whatever the tools are. Every handle reference resolves to exactly one Conduit or reports invalid. 0 lines.

2. MAJOR: bare names drop OBJ-013's kinds, so first-start names can be captured.
- OBJ-013 requires a kind on consumer and target precisely because each accepts several kinds. OBJ-002 is per kind, and NAM-027 only covers names within one project. So a project may define `agents: {root: ...}` or `pools: {access: ...}` / `{management: ...}`.
- FIL-003 resolves a bare name among "the project's entries, external, and the five first-start names", with no precedence and no reservation.
- Trace: an Agent with write access to the project directory adds zz.yaml containing `agents: {root: {type: <its Type>}}`. FIL-002 merges it. If project entries win, every `grants: root:` line now makes that Agent the consumer, including turn grants on agent-a and agent-b. The plan, run as root, opens them. The plan line "+ Conduit root/agent-a" looks the same as before.
- `project: management` shadows `to: management` in the same way.
- Fix:
  - FIL-003 reserves system, local, management, access and root in every section and in `external`, and any shadowing is rejected.
  - Names are unique across the sections and `external` together.
  - The plan shows the kind and ID of every consumer and target.
  - 0 lines.

3. MAJOR (G3, B#1 and D#6 not fixed): a narrowing made outside this apply is silently reversed.
- Tombstones only guard a handle that was closed and left closed.
- Trace A (stale working copy): WC1 runs T4. It closes agent-b/project (tombstone t1), registers [read, list] acknowledging t1, and opens it. Teammate WC2, or CI building an older commit, still has [read, list, write]. Its plan classifies the line as C3: close the live Conduit (expectedId = the live ID, so no conflict), register, open.
  - If acknowledged tombstones are used up, nothing blocks this, and agent-b gets write back.
  - If they persist, every later touch of the handle by anyone needs `--reopen`, and the flag becomes routine.
  - The design says neither.
- Trace B: during an incident, security operator S narrows agent-b with close and register. The author's next unrelated apply restores the file's wider list.
- "Drift between plan and dispatch is a conflict" does not cover drift between the last apply and this plan.
- Fix: register under a handle carries a precondition, `expectedPrevious: <the Conduit ID or tombstone ID this copy last applied>`. The engine reports conflict when the handle changed after that, by any invocation not named. A plan treats any live object whose ID differs from its applied record as foreign and needs a flag naming who changed it and when. 0 lines.

4. MAJOR (critic gap 2, only partly fixed): tombstones can be bypassed.
- (a) Renaming the grant key. The tombstone key is (consumer lineage, local name), so the design's claim that "renaming a key does not clear a tombstone" is false for the grant key. Trace: S closes agent-b/project. A merged refactor renames `project:` to `files:`. The plan treats removing a closed grant as already done, then C1 `+ agent-b/files -> shared [read, list, write]`. No tombstone matches, and the revoked access returns without a flag (E#3b).
- (b) Its own tombstones across applies. At T7 a tombstone is "acknowledged without a flag" because this working copy closed it. Trace: agent-a is revoked because it was compromised. Later `git pull` merges a branch that still has the line, and an unrelated apply reopens agent-a/project silently.
- (c) A replacement written by hand, or by an Agent through register.v1, without `succeeds` carries no tombstones.
- Fix:
  - Also match tombstones on (consumer lineage, target lineage) under any local name.
  - Acknowledge without a flag only a tombstone made earlier in the SAME apply (C3/C4 pairs). Reopening anything closed in an earlier apply always needs explicit approval.
  - Cost: T7's restore becomes 2 lines (the tombstone ID), tying the baseline. That is better on 5 of 7, which still passes.

5. MAJOR: `succeeds`/LIF-039 can be used to hijack a file key.
- LIF-039 "maps file keys to live objects". `succeeds` is an authored key, and nothing says who may write it or how many successors an object may have.
- Trace (row 2: an Agent needs only `management`; the real use's admin Agent has broad trust):
  1. Actor X registers `agents: {x1: {type: <X's>, succeeds: agent-a}}`.
  2. The plan follows the lineage, so the file key agent-a now means x1.
  3. x1 holds no `project` grant, so the plan sends C1 `register x1/project -> shared, open`. X's Agent gets agent-a's access.
- The same works for a Resource: an object with the file's config but on another node or runtime becomes the object behind the key `shared`.
- Fix:
  - `succeeds` is authorized as retiring the predecessor (ACC-033 rows 4/5).
  - It must name the same kind and the same owner Pool.
  - It is refused if the predecessor already has a successor.
  - The plan follows only lineage links made by invocations in its own applied record, and reports any other link as a conflict.
  - The owner must rule on LIF-039 before stage 1.

6. MAJOR (no silent widening): three defaults or omissions widen access.
- (a) `requirements` defaults to `{}`. On a grant to `access`, no bound means unlimited (ACC-029). So `delegate: {to: access, capability: core#access-control.v1, operations: [core#open.v1]}` gives the Agent root's right to open, minus SEC-008 and ACC-032, once root's plan opens it (ACC-030). This is C's D7.
  - Fix: FIL-005 requires a `bound` on every grant to `access` (an explicit unbounded marker if the owner wants one), and the plan flags grants to `access` and `management` as high-trust. 0 lines in S1.
- (b) Defaults and implementation items that a package version can change. Reach flags rest on Type defaults that the author never writes (claude-code.v1 lists them in `required`; 0.1.2 adds defaults). NAM-024 forbids changing a default, but no promise checks it at install.
  - Trace: `conduit install agent-claude-code-0.1.3.tgz` with `accountServices: default true`, then `apply --replace agent-a`. C9 compares only the granted operations' contracts and the tools map, so agent-a--2 gets reach outside the engine (C M9, A#2).
  - The same holds for item changes: a new server or fixed packageArguments, which MAN-010 lets a Package state.
  - Package-exported Conduit templates are not contracts, so NAM-024 does not cover them. A grant `from: pkg#ro` re-resolves under the new pin at the next C3/C4 re-point (for example T5 step 3) and can list more operations.
  - Fix: C4, C5 and C9 diff the whole resolved definition of each successor and each re-registered Conduit against the live one: filled config, operations, requirements, the item with its digest and bindings, nodeSelector and records. Any widening needs approval. Install checks NAM-024 against the release the file previously pinned. Or reach flags must be written (0 lines in flow style).
- (c) Narrowing a template is not applied. C8 says existing instances keep their definitions (INS-020).
  - Trace: real use. `rw` drops write to revoke it from 10 workers. C8 re-registers `rw`, but every grant made from it keeps write. A fresh engine would read the file as read-only; the live engine still allows write.
  - Fix: C8 also plans C3 for every grant or instance whose `from` names the template, narrowing first.

7. MAJOR (critic gap 1, still open): approval without a terminal.
- The counts (T4 2 lines, T7 1 line) assume apply widens without any approval.
- Engine-level guarantees cannot depend on approval anyway, since Agents call register.v1 and open.v1 directly.
- Approving by a saved plan's digest adds 1 value-bearing line per apply and fails "most" under the strict reading, the same as the layered route.
- Decide before the rerun.

8. MEDIUM: ACC-024 makes every `grants: root:` line add-only, and the planner does not account for it.
- ACC-024 refuses any close of a Conduit whose consumer is root, "whoever makes it". CON-008 closes one only when its target retires.
- Effects:
  - C2 or C3 on a root turn grant can never apply, so the file says one thing and the engine another.
  - The S5 example writes `root: {store: ...}`. A later C4 of `store` reaches step 2 ("close every grant to the old Resource") and is refused. By then the Agents' grants are already closed, so the apply stops with the Agents locked out, and resuming hits the same refusal.
  - C5 cannot re-point root/agent-a while the old one stands, under the handle uniqueness of item 1.
- Fix: the planner re-points root's grants only by retiring the old target first (CON-008, break before make), and rejects edits to root lines up front with a diagnostic. Or amend ACC-024 so that root may close root's non-first-start Conduits at its own request. The design's list of promises it leaves unchanged includes ACC-024, so this has to be stated.

9. MEDIUM (thesis 2 is overclaimed).
- Normative vectors fix the file's meaning only at first registration on an empty engine.
- After T1, every S1 task is the plan's translation of (file, state) into partial files. Those partial files carry `external`, an explicit `owner: s1` (`project:` cannot be resent: FIL-007 would derive again and OBJ-002 would reject), `to: shared--2`, an inherited `placement`, and `succeeds`. None of that translation is normative or tested.
- So "the plan calls the core's expansion, unlike a compiler mirror" is only partly true. The live meaning of an edit is the plan's.
- FIL-007 also derives "every operation the capability has in the core version the file's apiVersion names". apiVersion names the spec version, not a core version: core is 0.1.0 today, built into the engine and never pinned (NAM-020). A later core release that adds an operation to pool-management.v1 changes the derived grant, and the text does not.
- Fix:
  - Write the 7 operations into FIL-007 itself.
  - State thesis 2 as "meaning on an empty engine".
  - Either make the plan classes S1 uses normative (the design's own estimate is about 25-30 promises) or add golden (file, state) -> plan cases, as the layered route would.
  - Update the COMPARED table to match.

10. MEDIUM: provenance and project identity are undefined after T1, so C's D1 is not fixed.
- FIL-008's `source` (project, pointer) is "from the text the engine expanded". For a partial file that text is the planner's, and "the project" has no carrier without `project:`.
- So either the client asserts it, and any registrant can stamp `source: s1#/grants/agent-a/project`, or it is empty, and the plan cannot find T2+ objects without the local applied-ID file.
- Fix: a submission names the project as `{pool: s1, expectedId}`, authorized through the operator grant. `source` is recorded as (Pool ID, request ID, pointer into that request), and is shown as such, never as "the author's line".

11. MEDIUM: overriding a revocation leaves no trace.
- No core type states `records` (spec/core/types.yaml), and REC-002 keeps inputs only under it.
- So the record of register.v1 or open.v1 does not show which tombstone `acknowledge` named.
- Fix: LIF-038 marks the tombstone "acknowledged by invocation X, caller Y" and keeps it, or REC-002 records `acknowledge`.

12. MEDIUM: drift can block a revocation.
- C2 sends close with expectedId, so if another actor replaced the Conduit under the handle, the close reports conflict, the apply stops, and the access stays open.
- Closing is always a narrowing.
- Fix: on conflict in C2, read the handle again and close whatever is there, reporting it. Keep expectedId strict for C3 and C4.

13. MEDIUM: file keys and engine names drift apart, and names can be reused.
- After C4/C5 the key `agent-a` is `agent-a--2` in the engine. An emergency `close agent-a/project` sent by key then targets nothing, or, after removal plus an acknowledged reuse, someone else's Agent.
- Authors may write `--` (OBJ-001), so an authored `shared--2` collides with a generated successor name (E#9).
- `external` carries kinds and no IDs, so a name reused between the plan's re-read and register resolves to the new object.
- Fix:
  - Reserve `--` in authored names.
  - The CLI resolves keys through verified lineage to IDs and always sends expectedId.
  - Allow `external: {x: {kind, expectedId}}` on the wire.
  - Before open, check the resolved consumer and target IDs of the newly registered Conduit.

LOW
- 14. Disclosure through tombstones.
  - Tombstones listed by inspect must follow the inventory's tier: Conduits are inspected through `management`, row 7. Otherwise a caller with only a Pool's authority, inspecting a Resource, learns the local names of Conduits it could never inspect.
  - Instance tombstones turn register's conflict into an oracle for the names of retired objects, not only live ones.
  - The C4 promise to "list every Conduit the retirement will close" means nothing for an operator with only Pool authority, who sees none of them.
- 15. Free text in tool descriptions. The new `description` comes from whoever registered the target and goes into every consumer's tool descriptions, which allows prompt injection across projects. Show it in the plan, quote it as data, flag foreign targets, and limit its length.
- 16. Secret places.
  - The S2 sketch writes bare names at secret places (`token: linear-key`), which unchanged SEC-001 rejects.
  - FIL-003's diagnostic that "names" an unknown bare name would echo a literal secret written at such a place (SEC-003).
  - Keep SEC-001 and redact that diagnostic.
- 17. Squatting root's local names.
  - Root's local names are one namespace for the whole engine, and FIL-007 and turn grants depend on them.
  - Any holder of management register.v1 can register a root Conduit to `management` under a name such as `s1`. ACC-024 makes it impossible to close, and `management` is never retired, so `project: s1` is blocked for good.
  - Allow closing a root Conduit that was never opened.
- 18. Repeated writes after a re-point (A#5, D#16, still open). With tool names stable across a re-point, a write whose outcome was unknown on shared--2, retried with the same idempotency key, is a new request on shared--3 over the same directory, so it can apply twice. Record the lineage in the key's scope.
- 19. The directory as an authority input. FIL-002 merges every top-level yaml/json and follows links, so the directory becomes an authority input (D#22). The plan should list the files that contributed.

NET
- Normative-specific hazards: items 1, 2, 5, 6a, 8, 10, 16 and 17.
- Hazards shared with the layered route: 3, 4, 6b, 7, 12, 13, 18 and 19.
- All fixes are spec text or plan rules. S1 costs +1 line (T7 restore with a tombstone ID) and +1 per apply if approval is by digest.
- Severity: major. Not fatal, because no fix needs a second syntax or a weaker access model.

## Verdict expressiveness (major)
MAJOR, not fatal. The small-core file can state almost every construct S2-S10 need, one for one: secrets, placement and runtime, controls, filters, bounds, shared-host, providers, contracts and entries. It has no escape hatch, so whatever it drops is simply gone. Three of the things it drops matter to scenarios outside S1: (1) a template, including one a package exports, can no longer carry the grants its instances need; (2) S4 loses setup ordering between Pool entries that depend on each other only through a Conduit, and an entry's replacement no longer brings its grants back; (3) no file can refer to an object by a name that survives a replacement. So the answer to the escape-hatch question is no. Peer mode, enrollment and the expected account cannot be expressed either, but that is equally true today. The design's real-use claim that the admin works with "access bounded to conduit-dev" is wrong under ACC-029. One fix covers gaps 1 and 2: keep a grants map inside templates, registered with each instance and never opened by setup.

I only read the repository at <home> I changed nothing. Each claim below is checked against the spec text cited.

MAJOR GAPS (caused by this design, owner must rule)

M1. No reusable template can carry relationships, and nothing in the language states the local names an implementation depends on.
- The design deletes declared Conduits and the input destination "a declared Conduit's target" (INS-012).
- An implementation makes onward requests by tool name `<local>.<export>` (IFC-012, IFC-015).
- Today the only place a package can state "an instance of this needs a Conduit named `docs` of capability X, with its target as an input" is an exported template's `conduits` map. A Type cannot state it (CTR-004: `requires` lists operations only).
- Under the design, the S2 adapter (`upstream`), the S8 search Resource (`docs`) and the S7 plugin must each have their consumer guess the right grant key from documentation. A wrong key is not caught when the file is registered; it shows up as refused onward calls at run time, which goes against ruling 52.
- The owner's real use also loses its worker template that bundles tree/main/orders: about 6-9 lines per worker, against about 4 for a short form that kept declared Conduits.
- Example: spec/schemas/examples/valid/project/templates.yaml, the `reviewer` template, whose input fills `/conduits/checkout/target`.

M2. S4: Pool entries lose ordering and repair through Conduits.
- POOL-013 orders entry setups "from the references among the entries alone". POOL-005 allows `{entry: x}` only inside entries.
- Today an entry whose template's declared Conduit targets `{entry: service}` depends on `service` (POOL-006, INS-012). Under the design the grant `hooks.webhook/svc` sits outside the Pool and creates no dependency. A webhook entry whose setup hook calls `service` gets ordered correctly only if its Type happens to have a config place for a reference. The design's own S4 sketch quietly relies on such a place: `config: {service: {entry: service}}`.
- register-replacement.v1, kept for entries only, makes a Resource without its onward grants. Today LIF-009 re-makes the declared Conduits too. An Agent repairing through the core request (test 5, S9) leaves the entry unable to reach its secret or service until someone registers and opens new grants.
- A Pool template, including a package-exported one, cannot carry its entries' grants, so each instantiation means hand-written `<pool>.<entry>` grants.

M3. No reference survives a replacement, so the escape-hatch question is answered no.
- Every replacement gets a new name chosen by the tool (`shared--2`). OBJ-011 forbids renames, and OBJ-002 makes names unique across the engine.
- FIL-003 resolves a bare or `external` name literally and once. Nothing resolves through `succeeds` lineage.
- So any other writer cannot follow the live object after an S2 E13 rotation, an S10 provider swap or a real-use package upgrade. "Other writers" means another project's file, the admin Agent's register.v1 partial files, or a stage-1 canonical document. Their reference hits the retired object (rejected by CON-025), then nothing once it is unregistered. Worse, once the name is freed, a later registration can take it and the reference silently binds to an unrelated object.
- Conduit handles stay stable only while the consumer is not replaced.
- In the author's own file, after the first replacement the bare names no longer denote what the engine holds. The mapping lives only in LIF-039, which is new and unaccepted, and in the plan tool, which is not binding. So thesis 2 holds for the first registration only.
- Example: after T5 the history grants still hold the 0.1.0 pins while the file says 0.2.0, and the design chooses not to compare them.

GAPS THAT EXIST TODAY TOO (not caused by this design; flagged because the design states some incorrectly)

P1. Real use: an admin bounded to `conduit-dev` cannot open turn grants to workers it creates.
- ACC-029: naming a Pool covers that Pool and its Resources. Agents have no owner Pool, so a new `worker-i` is never covered.
- Registering an Agent goes only through `management`, which cannot be narrowed (ACC-033 row 2).
- So agent-driven worker churn needs unlimited access or a broker (D04). The design's "management plus access bounded to conduit-dev" does not cover it. Major for the owner's real use.

P2. Peer mode cannot be expressed (ruling 51.2/51.4): there is no way to bind an Interface to a peer's trust domain, and no two-halves relationship. A peer's admitted Interface can be written syntactically (`interfaces:` plus `external`), with no identity binding. Major for S3 peer mode, the same as today.

P3. Enrollment (rulings 50, 52.5) has no promises yet. Nodes are Resources (core/templates.yaml, the `node` template), so `external: {m2: Resource}` works once a node exists.

P4. The expected account (ruling 49 #3) is a config key of the Type and is not yet written. Pending actions need no authoring.

MINOR (fix in the spec batch)

m1. The S2 sketch breaks SEC-001.
- `token: linear-key` writes a bare name at a secret-reference place, and SEC-001 rejects bare names there. The `targets: [linear-adapter]` in the same sketch also fails target-reference.v1, which requires `{kind, name}`.
- FIL-003's general bare-name rule needs either the SEC-001 exception kept or the reference schemas amended. Neither is in the amendment list.

m2. FIL-006 says IFC-014 "already makes that pair unique". It does not: IFC-014 forbids equal tool names, not equal local names. Grants with `operations: []` (allowed by CON-017), or with disjoint operations, could share a handle. A new rule that local names are unique per consumer is needed.

m3. reference.v1 still has a `{kind: Conduit, name}` branch, and it is used by inspect, set-up, retire and unregister (spec/core/operations.yaml:349, 387, 457, 493, 574). The spec plan changes only conduit-reference.v1.

m4. The S7 sketch lists implementations in `exports`, which MAN-007 forbids. MAN-007 is not in the amendment list.

m5. A Grant has no `description` key. Today a Conduit document has `metadata.description`, so this is a step back under ruling 52 and ruling 53's descriptive tools.

m6. S5: when a C4 replacement follows a control change, the plan does not say whether the successor gets the file's config value or the live one. Taking the file's value silently reverts another actor's control.

m7. S6: with `--prune`, the plan must retire the Resource before closing its own consumer grants. Otherwise its retire hook's onward request is refused (ACC-039 only lets it through while the Conduit stays open). The design does not state this order.

m8. Inline Pools cannot place or run entries in a chosen place: the anonymous template has no inputs, and POOL-009 allows placement and runtime only through inputs. S3 combined with S4 therefore needs a named Pool template.

m9. Real-use churn: reusing a name (`worker-1`, or `admin/worker-1`) needs `acknowledge` on every cycle, and only root purges tombstones. The admin Agent's tombstones pile up.

m10. `optional` on relationship parts of a setup (S4) is gone. Optional entries remain.

m11. Stage 1 (expansion into today's documents) needs `as` plus a derivation of Conduit names that cannot collide and fits 63 characters: `a-b` + `c` and `a` + `b-c` must not both become `a-b-c`.

PER SCENARIO
- S2: OK once m1 is fixed. The adapter's `upstream` name falls under M1.
- S3 managed mode: OK. Peer mode: P2.
- S4: partly lost (M2).
- S5: OK, with m6.
- S6: OK, with m7.
- S7: partly lost. A package cannot ship an adapter or composite ready to instantiate (M1).
- S8: OK apart from M1's local-name contract.
- S9: OK, except entry repair by an Agent (M2).
- S10: the cascades can be expressed, but other files' references break (M3).
- Rulings 49, 50 and 51: P2 to P4.

SUGGESTED FIX for M1 and M2, at modest cost
- Keep a `grants:` map inside templates, whose consumer is the new instance.
- These grants are registered with the instance, re-made by register-replacement for entries, and may target `{entry: x}`, which also restores the ordering dependency.
- Setup never opens them (ACC-005 and CON-004 stay deleted), so skeptic D's HIGH #2 stays solved.
- In return this keeps CON-002 and the INS-012 target destination.

For M3 the owner needs to choose between two options:
- a reference form that resolves through lineage, such as `{lineage: shared}`;
- normative successor naming, with replacement visible to other files.

## Verdict cost (major)
The design's counts of vectors, code, archives and examples check out against the repository. Its spec cost does not: about 120-140 promises are touched (26-31%), not about 101 (22%). The design also leaves out four costs: binding prose outside the promises, the loader and conformance-crate tests, a NAM-025 conflict that needs an owner ruling, and re-authoring (not just regenerating) about 104 S1 cases. A clean break costs about ten times the layered route in promises, about 150 vectors and a store break, for the same shell, and it barely shrinks what an implementer reads. The design's own staging is the efficient route: stage 1, the file form made normative by expansion into today's model (about 35-45 promises, 0 vectors broken), before the rerun; the deletions in stage 2 only after the rerun and an owner ruling.

I only read; nothing in the repo was changed. Scans were run from scratch scripts in <scratch>/est/ (promises.py, cases.py, p.json, design.txt).

== CONFIRMED EXACTLY ==
- Promises: 455; promise words: 83,875. Layers: 203 language, 208 convention, 44 interface.
- The six removed promises (CON-002, CON-004, ACC-005, LIF-029, SUB-007, SUB-009) total exactly 1,313 words.
- Conformance:
  - 367 cases;
  - 481 files carry a frame;
  - 161 files hold Conduit documents;
  - 84 archives, 81 of which contain framed documents;
  - 268 files carry sha256 pins.
- Examples: 114 (11 valid, 62 invalid, 41 invalid-by-rule).
- Crate sizes: loader 2,879 source lines, core 10,465, check_schemas.py 2,588.
- Case counts I measured:
  - 317 cases have framed inputs or a changed behaviour; the design says 280-316.
  - 152 cases change expectations or steps (the union below); the design says 140-200.
  - 123 cases use fixtures/s1, whose Agents each declare `project` and `history` Conduits; the design says 118.
- Archives are regenerated by reference-model/crates/conformance/examples/make_archives.rs, so repacking them is mechanical.

== UNDERSTATED OR MISSING ==

1. Promise count.
- The design's own A-J list holds 91 unique IDs, not "about 84" (IFC-033 appears twice).
- The scan finds 14 more that need real edits:
  - pointer or wording rooted at `spec`: INS-021, OBJ-016 (fixes where a destination is rooted), OBJ-021, TPL-003;
  - declared Conduits: SEC-009 ("a Conduit that an Interface template declares") and INS-016 (nested instances);
  - Conduit names: LIF-035 ("its name can be given to another object"), OBJ-001, and OBJ-025 (whether a clash reports conflict or invalid);
  - cross-references to removed promises: ACC-006, OPS-021, OPS-027, REC-002 (all cite CON-004) and PKG-029 (cites SUB-009).
- 16 more say "instance document": FST-006, INS-003, INS-004, INS-014, INS-023, MAN-007, MAN-015, OBJ-009, PKG-028, PLC-002, PLC-006, PLC-008, PLC-028, POOL-009, PRV-002, PRV-006. They need no edit only if definitions.md redefines the term.
- "IFC-014 already makes the pair unique" is false. IFC-014 rejects a Conduit only when a tool name would collide, so two Conduits of one consumer may share a local name if their operations differ or are empty. FIL-006 therefore needs a new uniqueness rule, and that changes meaning.
- "None of the PRV promises" is changed: false by the design's own list, which amends PRV-007 and PRV-015.
- Corrected total: 6 removed, about 105-123 amended, 11-13 new. That is about 120-140 touched (26-31%), leaving about 460-463 promises.

2. Core contract compatibility (unbudgeted; needs a ruling).
- The core is 0.1.0 (spec/core/conduit.yaml). NAM-024 allows a later release only to add, and NAM-025 says a release that breaks takes a new address. The spec has no 0.x exception.
- These changes are not additions:
  - removing replace.v1 from object-management.v1;
  - register.v1's input becoming conduit-file.v1;
  - inspect.v1's `declared`;
  - register-replacement.v1's `nested` becoming `entry`;
  - conduit-reference.v1 and reference.v1, which name Conduits by `{kind, name}`;
  - object.v1, which requires `name` for every kind and whose `nested` describes declared Conduits.
- So "core 0.2.0" is not allowed as the spec is written. The options are:
  - (a) a ruling that redefines the core in place at v0;
  - (b) a new core address, which tools/set_names.py rewrites mechanically (69 spec files, 155 conformance files);
  - (c) `.v2` contracts, which change tool names; `management.replace.v1` alone appears 10 times in the vectors.
- The layered route's additions are optional inputs and outputs, so it stays within NAM-024.

3. Binding prose is not costed.
- spec/definitions.md is 26,940 words. About 20-25 rows of its "Objects and documents" and "Resolving and registration" tables assume the frame or declared Conduits: kind, document, project, submission, name, template, instance, nested instantiation, nested instance, input, argument, standard argument, destination, optional, register, replace, reference.
- spec/README.md, spec/schemas/README.md and spec/core/README.md (about 25k words) mention documents about 140 times.
- Estimate: 3,000-6,000 words rewritten. The design's "±1%" covers promise text only.

4. Schemas.
- The five files to be replaced are 1,109 lines, not about 1,500 (envelope 111, document 38, instances 190, templates 340, manifest 430). A file.yaml of 800-1,100 lines is therefore roughly flat.
- core references.yaml and results.yaml also change.

5. Vectors.
- 104 of the 123 S1 cases go through setup. Under CON-004, setup opens their declared Conduits. Those cases need open steps inserted and their evidence ID allocations re-sequenced: their steps are re-authored, not just their expectations regenerated.
- Expectations regenerated by running the new model are not independent evidence, so each difference needs review against the promises.
- Other affected cases:
  - 85 cases name Conduits by `{kind, name}` in steps or expectations;
  - 26 assert root's Conduits;
  - 15 list `#replace.v1`;
  - 10 cite removed promises;
  - 197 load fixtures/root-system-state.json, which must be regenerated.
- About 15-20 examples (not about 10) are tied to rules about the frame or declared Conduits: OBJ-012/-013/-014/-015/-017/-018/-019/-020, INS-001, INS-012 (four examples), MAN-003, TPL-001 and CON-020.

6. Code not budgeted.
- Loader tests: 2,241 lines; project.rs, formats.rs, pattern_rewrites.rs, examples.rs, rules.rs and s1_drafts.rs depend on documents. About +400-900 lines.
- Conformance crate: 2,137 + 604 lines; its register-project and resolve-project readers must change. About 200-500 lines.
- Core: the design's 1.7-3k is plausible, and could be 1-2k with an anonymous-template shim, because the declared-Conduit slot shares its code with entries (registration.rs expand, lifecycle.rs slots). The real cost is Conduit identity (reference.v1, object.v1) across access.rs, requests.rs, lifecycle.rs and records.rs, plus lib.rs's first start, which reads root's `spec.conduits`.

7. Store migration.
- shell/src/store.rs: `VERSION = 1` with deny_unknown_fields over core State.
- The design implies bumping the version and refusing old stores. There is no migration: the owner's increment-4 lab stores are discarded.
- In the layered route, tombstones go under a separate state key, which is additive.

8. Re-evidence.
- Increment 1's done-when is defined over spec/schemas/examples.
- Increment 2's done-when names "the opening of declared Conduits (CON-004)".
- Increment 3's runs used real clients.
- All three must be redone, and the rerun waits at least for stage 1.

9. ACC-024 is ignored by the change model. Any close of a Conduit whose consumer is root is refused, so:
- root's turn grants and the operator grants FIL-007 derives can only be added, never edited or removed by the plan;
- re-pointing a grant root holds (S5's `root: {store}`) must wait for the old target's retirement under CON-008.

The layered route shares this constraint, but this design's C2 and C3 classes claim close-then-register for every grant.

== COST TABLE ==

| Measure | Layered (synthesis + critic) | Stage 1 (normative file by expansion) | This design, as claimed | This design, as checked |
|---|---|---|---|---|
| Promises touched | 11-14 | about 35-45 | about 101 (22%) | about 120-140 (26-31%) |
| Promises after | about 457 | about 485-495 (two forms) | about 460 | about 460-463 |
| Prose outside promises | about 0 | +1.5-3k words | not costed | 3-6k words rewritten |
| Core contract rules | additive (0.1.x) | additive | "0.2.0" | breaks NAM-025: ruling, new address or .v2 |
| Cases with inputs converted | 0 | 0 | 280-316 | 317 |
| Cases with expectations or steps changed | 0-8 (tombstones on re-register) + snapshot | 0 | 140-200 | 152, about 104 with re-authored steps |
| Cases deleted or recast | 0 | 0 | 20-30 | 20-30 |
| New cases | 10-17 + about 50 plan cases, not binding | 40-70 | 40-70 | 40-70 + handle uniqueness |
| Examples | 0 | +10-20 | 114 converted | 114; 15-20 recast |
| Archives / files with pins | Agent packages only | same | 84 / 268 | 84 (81 framed) / 268, by generator |
| Loader | 0 | +1.1-2k | 0.8-1.4k | 0.8-1.4k + 0.4-0.9k tests |
| Core | 0.3-0.6k | 0.3-0.6k | 1.7-3k | 1-3k, mostly Conduit identity |
| Conformance crate | small | small-medium | not costed | 0.2-0.5k |
| Shell / CLI | 3.5-5.5k + 2.5-3.5k tests | same less a 0.8-1.2k compiler | same less compiler | same less compiler |
| check_schemas.py | 0 | +0.2-0.4k | ±0.4-0.9k | ±0.4-0.9k (167 lines tied to document shapes) |
| Stores | additive | additive | break | break, no migration |
| Re-evidence | none beyond the increment-4 rerun | none | increments 1-3 | increments 1-3 |
| Work orders | 5 + dry run + rerun | about 7-8 | 10-14 | about 12-16 |
| Thesis 2 for what authors write | untested | tested | tested | tested |
| Cost multiplier on each later core change | 1.3-1.5x (mirror) | about 1.0-1.1x | 1.0x | 1.0x |

== READING COST FOR AN IMPLEMENTER ==
- About 7 concepts go: the frame, the separate manifest, declared Conduits, the declared/standalone split, generated Conduit names, template replacement, and replacing declared Conduits.
- About 5 come in: sections and the merge, the anonymous template, the Conduit handle, the project Pool with its operator grant, and lineage.
- Promise count and words are unchanged within ±1%. The implementer still reads about 460 promises and about 110k binding words. The gain is one form and no compiler, not a smaller spec.

== VERDICT ==
Severity is major because the totals are understated by more than 20% and one ruling (NAM-025) is unbudgeted. The design's own conclusion, to stage it, survives and is strengthened:
- Stage 1, before the rerun: write the FIL promises and the grammar as the normative authoring form, expanding into today's model, with the five ruling-53 additions. It costs about 35-45 promises and breaks 0 vectors. It answers the critic's gap 3 (where a fresh author learns the format) and gap 9 (thesis 2 untested), and it removes the compiler mirror.
- Stage 2, after the rerun, as its own costed ruling: delete the frame, declared Conduits and replace.v1, at about 120-140 promises, about 150 re-expected cases, a store break and re-evidence of increments 1-3, with NAM-025 settled first.