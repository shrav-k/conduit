Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Sectioned documents: the minimal normative delta (the frame once, by translation into today's documents)

## name
Sectioned documents: the minimal normative delta (the frame once, by translation into today's documents)

## thesis
The cheapest way to make the short form the language is to add a notation, not a new object model. One new kind of document becomes normative: a map with no `kind`. Its top-level keys are the five families, and each entry is keyed by its name. Its meaning is defined by translation into today's canonical documents. Beside that come four syntactic rules: inline one-off definitions (IN9's "a template with no inputs, instantiated once", written in one place), grants written under their consumer, a grant from the operator written on its target, and bare names that resolve within the project.

Everything else stays as it is: the objects, the wire (`register.v1`, `replace.v1`, `inspect.v1`), the core, immutability (OBJ-002, SUB-007), declared Conduits, `replace.v1` and `register-replacement.v1`. Change (T4 to T7) stays with the plan/apply CLI that ruling 53 accepted. The translation reads no package, no stored state, no caller and no environment, so defaults depend only on the text and its `apiVersion`.

I measured each step of the ladder on T1 with one counting convention:
- frame-once alone: 65 lines, fails;
- plus inline definitions: 42, fails;
- plus grants under the consumer: 34, fails;
- plus the operator grant: 21, passes with no derived default at all, so decision 6 is not needed;
- plus an owner default and an optional core-only Pool grant: 19.

With these, S1 totals 30 lines (baseline 64, layered package 27). It is better than the baseline on 6 tasks and the same on T3.

Cost:
- About 19 to 21 promise touches beyond the five additions already accepted.
- About 30 to 40 new vectors, with 0 of the 367 existing ones invalidated.
- 0 lines in the core.
- The layered route's compiler moves into the loader at about the same size.

What it buys: thesis 2 holds for what authors write, `spec/` itself becomes the format reference, and a second implementation must agree. Its honest limit is that the normative meaning covers registration only. Changes are still plans chosen by a tool.

## grammar
SECTIONED DOCUMENT (new; spec/schemas/sectioned.yaml, `sectioned.v1`, closed keys)

What it is: a YAML document, a JSON value or an item of a JSON array that is a map with no key `kind` and at least one key. It may stand in any file of documents except the manifest file (`conduit.yaml`, `.yml` or `.json`).

Top-level keys (all optional; at least one section required):
- `apiVersion`: string (`api-version.v1`).
  - Default: the `apiVersion` of the project's manifest, or of the package's manifest for a document inside a package.
  - Required when there is no manifest.
- `operator`: a reference to an Interface: a bare name (the kind is fixed to Interface), `{kind: Interface, name}`, or `null`.
  - Default: `{kind: Interface, name: root}`.
  - With `null`, no operator grant is made, and an entry's `operator` member is rejected.
- `templates`: a map whose keys are among `interfaces`, `agents`, `resources`, `pools` and `conduits`. Each is a map NAME → TEMPLATE-BODY.
- `interfaces`, `agents`, `resources`, `pools`, `conduits`: each a map NAME → ENTRY of that family.

NAME: the name grammar of OBJ-001 (`[a-z](?:[a-z0-9-])*`, at most 63 characters), with no `--` anywhere.

TEMPLATE-BODY: exactly today's template `spec` for that family (`template-spec.v1`, `pool-template-spec.v1`, `conduit-template-spec.v1`, `inputs` allowed), plus optional `metadata: {description, annotations}`. It translates to the framed template NAME. This form exists for reuse and for templates exported by packages.

ENTRY of family F: either REFERENCED or INLINE, each with optional `metadata: {description?, annotations?}` and the short members below.
- REFERENCED: `from` (`template-reference.v1`, all four forms of INS-025), `arguments?` (map), and F's standard arguments. It translates to the instance document NAME exactly as written, with the frame added.
- INLINE: no `from`. `arguments` and `inputs` are rejected. It holds F's definition keys and F's standard arguments.
  - Definition keys:
    - Interface, Agent, Resource: `type` (required), `config?`, `implementation?`, `controls?`, `conduits?` (a map LOCAL → `{from, target?, arguments?, optional?}`, as today's declared Conduit).
    - Pool: `type` (required), `config?`, `implementation?`, `controls?`, `entries?` (`pool-entry.v1`).
    - Conduit: `capability` (required, fully qualified), `operations` (required, a list of fully qualified selections, which may be empty), `requirements?` (a map NAME → `{requirement, arguments?}`).
  - Standard arguments:
    - Agent: `placement?`, `runtime?`.
    - Resource: `owner?`, `placement?`, `runtime?`. With `owner` left out, the owner is the one Pool that the project's documents make; with no Pool or more than one, `owner` is required.
    - Conduit: `consumer` (required), `target` (required), `as?`.
    - Interface and Pool: none.

SHORT MEMBERS
- `grants` (on Interface, Agent and Resource entries): a map LOCAL → GRANT, where GRANT = `{to: REF, capability: selection, operations: [selection, ...], requirements?: map, metadata?}`. Each grant makes a standalone Conduit NAME--LOCAL, made from template NAME--LOCAL--t. Its consumer is this entry, its target is `to`, and its `as` is LOCAL.
- `operator` (on Interface, Agent, Resource and Pool entries): `{capability, operations, requirements?}` or `null`. It makes Conduit NAME--operator, from template NAME--operator--t. Its consumer is the document's `operator`, its target is this entry, and its `as` is NAME.
  - Option for decision 6 (SCT-010): a Pool entry with no `operator` member gets `{capability: <core>#pool-management.v1, operations: [<core>#register.v1, <core>#register-replacement.v1, <core>#inspect.v1, <core>#set-up.v1, <core>#retire.v1, <core>#retire-with-resources.v1, <core>#reconcile.v1]}`, all written out in full. `operator: null` gives none.

REF (in `owner`, `placement`, `runtime`, `consumer`, `target`, a grant's `to`, and `conduits.*.target`):
- A NAME that an instance document of the same project makes (an entry of any sectioned document, or a framed instance) becomes `{kind: <its family>, name}`. NAM-027 makes this unambiguous.
- Any other value is kept as written, so OBJ-006 still applies: a bare name only where the field fixes the kind, otherwise `{kind, name}`.
- Values inside `config` and `arguments` are never rewritten (OBJ-021 and SEC-001 unchanged).

GENERATED NAMES: NAME--t, NAME--LOCAL, NAME--LOCAL--t, NAME--operator and NAME--operator--t. Each must be at most 63 characters and unique in the project (NAM-027); otherwise the submission is rejected, naming the entries that made the name.

SELECTIONS: fully qualified `address#export`, or `#name` for the project's own contracts. No short names and no aliases (rulings 25 and 53 #8).

NOT IN THE FORM:
- contracts (always framed);
- the manifest and its pins (`conduit.yaml`, written by `conduit install`);
- fields the engine owns (`id`, `revision`, `status`, `packages`);
- the resolved form `{kind, id}`;
- any expression or interpolation;
- capability inference;
- derived turn Conduits.

## meaning
Meaning is defined normatively by translation. The canonical documents remain the kernel: every existing promise keeps speaking only of framed documents and the objects they make. A new file, spec/promises/sections.yaml (SCT-*), states the translation as the meaning of a sectioned document (SCT-003): "a sectioned document means exactly the framed documents it translates to, and nothing else". Those documents are registered with the project's other documents as one submission, so SUB-001 to SUB-007, OBJ-002, NAM-027, INS-*, CON-* and IFC-014 apply to them unchanged.

Rules that keep thesis 2 for what authors write:
1. **Purity.** The translation depends only on:
   - the project's documents (to resolve bare names and the one-Pool owner);
   - the stated `apiVersion`, or the manifest's.

   It reads no package or contract and no stored object, caller, file name, directory or environment. So it is pure syntax, and the same file means the same objects on every engine and to every caller. This rules out C's M8 (meaning depending on who applies) and A's C2 (package defaults flipping). The only derived values are:
   - the kind of a name;
   - the one-Pool owner;
   - the root Interface as the default operator;
   - optionally, the seven core operations of the Pool grant. These are written in the promise text itself and bound to the specification version by OBJ-024, never to a package version.
2. **Totality and an as-if rule.** Every document that `sectioned.v1` accepts translates. An engine may implement the translation however it likes, provided the objects, records and diagnostics are those of the translated documents (Haskell's "it is not expected that it will use them directly"). There is one authority, the translation, and no second description.
3. **Everything derived is written out.** Kinds, owners, `as`, full operation lists and the operator's reference all appear in the stored canonical objects. No provenance is stored, so INS-013 and OBJ-018 are untouched and no forged provenance is possible (C's D1). The generated names (`agent-a--project`, `s1--operator`, `shared--t`) are themselves the record of which entry and rule made each object, under a separator that authors may not write.
4. **Diagnostics at the author's text (SCT-009).** Every diagnostic about a translated document, at whatever later check, names:
   - the file;
   - a JSON Pointer into the sectioned document (for example `/agents/agent-a/grants/project/operations/1`), with line and column where the reader has them;
   - the SCT rule that made the document;
   - the canonical promise that rejected it.

   Values at secret-reference places are not echoed (SEC-003). The reference model already keeps positions in the loader (`value.rs`), and the shell maps the core's `/objects/<name>` back through the translation map.
5. **Tests.** Each rule is pinned by A/B `translate` vectors: authored files in, then either the exact canonical documents and rule IDs out, or diagnostics with authored pointers. One equivalence property holds: registering the rendered translation gives exactly the transition that registering the sectioned document gives (modeled on the registration-orders comparison, conformance/src/lib.rs:598-631). The loader emits no core Decision, so the translation map's rule IDs are what the vectors assert.

The wire and storage stay canonical:
- `register.v1` takes `document.v1` values as today.
- A directory registration (SUB-010) hands it the translation of what it reads.
- `replace.v1` and `inspect.v1` (`declared`) are unchanged.

So Agents managing through Conduits (preservation test 5) send canonical documents. The translation is public and deterministic, so they can produce them, and nothing an Agent can do is lost. Contracts and the manifest are unchanged, so SUB-008's digest is unchanged. Long-form documents stay legal beside sectioned ones in the same project (Compose's both-syntaxes rule), and no round trip is required.

Evidence, not paper: I hand-translated the T2 state of the S1 file under these rules into 24 framed documents (12 templates, 12 instances: the `--t` templates, the root→Pool grant listing 7 operations, root's turn grants, and four grants). I ran them through the existing conformance runner and today's loader and core. The registration was accepted with no diagnostics, and all 24 objects were made. Two departures:
- `as` was left out, because the core does not have it yet.
- The reach flags were written in `config`, because the fixture Agent packages are 0.1.1, which have no defaults.

Scratch case: <scratch>/repo/conformance/language/registration/sectioned-s1/ (templates.yaml, instances.yaml, case.yaml). Its only failure is my placeholder `records: []` expectation.

## spec_plan
NEW: spec/promises/sections.yaml, prefix SCT (free; layer language, level must)
- SCT-001, the form:
  - A map with no `kind` is a sectioned document. Its closed keys and sections are as stated, and keys are names without `--`.
  - It is validated against `sectioned.v1` under SUB-005, with pointers into it.
  - When it holds `metadata` or `spec`, the diagnostic says that a framed document needs `kind`.
  - Contracts are never written in it.
- SCT-002, the version: the stated `apiVersion`, or else the manifest's. With neither, it is rejected. Every translated document carries the version, and OBJ-024's mixing rule applies.
- SCT-003, the meaning: translation, purity, totality, the as-if rule, and one submission with the project's other documents.
- SCT-004, entries:
  - Templates under `templates` become framed templates.
  - A referenced entry becomes its instance document.
  - An inline entry becomes template NAME--t plus instance NAME. Inline entries have no inputs and no arguments.
- SCT-005, grants: GRANT becomes template NAME--LOCAL--t and standalone Conduit NAME--LOCAL, with consumer the entry, target `to` and `as` LOCAL.
- SCT-006, the operator: the document's `operator` (root by default) and an entry's `operator` member, which make Conduit NAME--operator with `as` NAME.
- SCT-007, references: bare names that the project's instance documents make, and the one-Pool owner default.
- SCT-008, generated names: the reserved `--`, the 63-character limit, and uniqueness under NAM-027, with diagnostics that name the entries.
- SCT-009, diagnostics: at the authored pointer and line, naming the rule and the rejecting promise, with secret values redacted.
- SCT-010 (option for decision 6): the core-only default Pool grant, with its seven operations written out and an opt-out of `null`.

AMENDED (wording or observables)
- OBJ-017: a framed document carries `apiVersion`; a sectioned document follows SCT-002. The OBJ-017 example with `kind` and no `apiVersion` stays rejected.
- OBJ-020: `kind` applies to framed documents; a map without `kind` falls under SCT-001.
- INS-001: the `from` marker applies to framed documents. In a sectioned document, membership of a section decides.
- OBJ-019: the exempt list adds the section maps, `templates.*` and `grants`, all keyed by name.
- OBJ-022: named members add sections and grants.
- SUB-005: a submission may hold sectioned documents, and pointers point into them.
- SUB-010: the values handed to `register.v1` are the translation.
- SCH-018: a JSON value or array item may be a sectioned document.
- MAN-011: files of documents may hold sectioned documents. A package may hold only `templates` sections; an entry section is rejected at install, as an instance document is.
- NAM-027 (observable): it counts translated documents.
- Light observable edits to OBJ-012 and OBJ-013, to say "framed". OBJ-013 is touched anyway by `as`.

Total: 9 new (+1 optional) and about 10 amended, so about 19 to 21 touches, about 4 to 5% of the 455 promises.

DEFINITIONS: spec/definitions.md gains sectioned document, section, entry (inline or referenced), grant (in a sectioned document), operator (of a document) and translation. spec/README.md gets its contents list updated.

SCHEMAS AND CORE
- spec/schemas/sectioned.yaml, about 200 to 300 lines: `sectioned.v1`, one entry schema per family, `grant.v1`. It reuses `selection.v1`, `template-reference.v1`, `implementation-map.v1`, `template-controls.v1`, `pool-entry.v1`, `applied-requirement.v1`, the reference schemas and `instance-metadata.v1` without `name`.
- spec/core/README.md lists the new Schema exports, which `Specification::from_documents` picks up.
- 10 to 15 new examples.
- `envelope.v1`, `document.v1`, `template.v1`, `instance.v1` (apart from `as`) and `manifest.v1` are unchanged.
- No core operation changes for the delta.

THE FIVE CORE ADDITIONS RULING 53 ACCEPTED (11 to 14 touches, needed by any route, written once in the canonical layer). Sectioned variants are not needed.
1. `as`: a standard argument of a Conduit instance (OBJ-013), the last sentence of IFC-012, a note in IFC-014, and `instance.v1`'s conduit. Grant keys and operator members set it.
2. `expectedId` on `open.v1` and `close.v1`: the `when` of LIF-037 and two input schemas. The plan sends it on every open and close.
3. The inventory in `inspect.v1`: LIF-013, the output schema, and a disclosure sentence that gives no counts. Plan and `reach` use it.
4. Tombstones (a new LIF-038), enforced at open and register:
   - I propose keying them by (kind, name).
   - The translation's names make (Conduit, NAME--LOCAL) a stable identity for "this grant of this entry", across replacement of the consumer. That closes critic gap 2 in the engine.
   - An override must name the tombstoned ID. The plan supplies it only for closes made by this working copy's own applies.
   - Re-registering the whole file after a revocation is refused for that grant.
5. Descriptive tool descriptions (IFC-033): unaffected; they surface the history Type's posting convention.

OTHER ACCEPTED POINTS
- #5: `install` writes the pins into `conduit.yaml`, which is tool-written.
- #7: reach flags default false, as new Agent package versions 0.1.2.
- #8: fully qualified names, as the grammar requires.
- #9: the format reference is now `spec/` itself.

REMOVED: nothing. Kept on purpose: framed documents, templates and inputs, declared Conduits (TPL-012 for S4 and S9), `replace.v1` (the plan uses it for T4), `register-replacement.v1` (entries; D09 for S2 and S10), every promise of ACC, PLC, SEC and LIF.

RULINGS
- Ruling 35's wording ("a document's apiVersion") becomes "known per document; a sectioned document takes the manifest's".
- Ruling 37's frame becomes optional for templates and instances; the framed form stays canonical.
- The owner should confirm that ruling 8's IN9 and ruling 18's V2 are kept to the letter: every instance comes from a registered template, and an inline entry is notation for IN9's one-off.
- Decision 6 is reduced to the single optional SCT-010.
- Rulings 21, 22, 25, 27, 29 and 52 stand.

NOT TAKEN, AND WHY (the cheapest path)
- Derived turn Conduits: they need the Agent Type's contract, so the translation stops being syntactic. They cost about 100 to 200 core lines and make a default depend on a package version (ruling 53's concern). They save 2 lines in S1.
- Capability inference: package-dependent, and saves 0 lines.
- Inline definitions in the core with no template object: 250 to 500 core lines and 12 to 18 promises, for 0 author lines.
- Sections inside `conduit.yaml`: SUB-008's digest would churn on every grant edit.
- `register.v1` accepting sectioned values: 1 to 2 promises and about 30 lines; possible later.
- A normative plan: 25 to 30 promises and about 50 vectors. Ruling 53 #4 keeps plan and apply in the CLI for now.

## s1_rewrite
Conventions:
- As in the coordinator's files: block style for sections, entries and maps keyed by name; each member's value in flow on one line. Each grant is one member of `grants`.
- Counted: non-blank lines, comments excluded, plus each command line that carries a value the author chose.
- Not counted: `conduit init`, `serve`, checks (`tools --as`, `call ... .turn.v1`) and tool-written lines (marked TOOL).
- Assumed: Agent packages 0.1.2 with reach flags false by default (ruling 53 #7), and the five accepted additions.
- Primary configuration: owner default plus the SCT-010 Pool grant. The variant without SCT-010 is in s1_counts.
- Files: <scratch>/min/t1.yaml and t2.yaml.

=== T1: 18 file lines + 1 command line = 19 ===
```
$ conduit install <lab>/pkgs/filesystem-0.1.0.tar.gz <lab>/pkgs/agent-claude-code-0.1.2.tar.gz <lab>/pkgs/agent-codex-0.1.2.tar.gz   [1]
$ conduit apply                                                                                                                      [no value]
```
conduit.yaml (TOOL, written by install):
```yaml
apiVersion: github.com/shrav-k/conduit/v0
packages:
  example.org/s1/filesystem: {version: 0.1.0, digest: 'sha256:200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160'}
  example.org/conduit/agent-claude-code: {version: 0.1.2, digest: 'sha256:<computed by install>'}
  example.org/conduit/agent-codex: {version: 0.1.2, digest: 'sha256:<computed by install>'}
```
s1.yaml (authored, 18 lines):
```yaml
pools:
  s1:
    type: github.com/shrav-k/conduit/core#pool.v1
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    operator: {capability: example.org/conduit/agent-claude-code#conversation.v1, operations: [example.org/conduit/agent-claude-code#turn.v1]}
    grants:
      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    operator: {capability: example.org/conduit/agent-codex#conversation.v1, operations: [example.org/conduit/agent-codex#turn.v1]}
    grants:
      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
Its meaning: the normative translation, 18 framed documents, which `conduit render` prints. None is authored.
- Pool template `s1--t` {type core pool.v1} and Pool `s1` [SCT-004].
- Conduit template `s1--operator--t` {core pool-management.v1: register, register-replacement, inspect, set-up, retire, retire-with-resources, reconcile}. Conduit `s1--operator`: consumer {Interface root}, target {Pool s1}, as `s1` [SCT-010].
- Resource template `shared--t` {type local-directory.v1, config {root}}. Resource `shared`, owner {Pool s1} [SCT-004, SCT-007].
- Agent templates `agent-a--t` and `agent-b--t` {type}. Agents `agent-a` and `agent-b`.
- `agent-a--operator--t` {conversation.v1 [turn.v1]}. `agent-a--operator`: root → {Agent agent-a}, as `agent-a`. The same for agent-b [SCT-006].
- `agent-a--project--t` {directory.v1 [file.read.v1, directory.list.v1]}. `agent-a--project`: {Agent agent-a} → {Resource shared}, as `project`.
- `agent-b--project--t` {[read, list, write]}. `agent-b--project`, as `project` [SCT-005].

For example, the grant translates to:
```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata: {name: agent-a--project--t}
spec: {capability: 'example.org/s1/filesystem#directory.v1', operations: ['example.org/s1/filesystem#file.read.v1', 'example.org/s1/filesystem#directory.list.v1']}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata: {name: agent-a--project}
from: agent-a--project--t
consumer: {kind: Agent, name: agent-a}
target: {kind: Resource, name: shared}
as: project
```
The plan apply sends, 13 requests:
1. 3 `install.v1`.
2. `register.v1` (18 documents).
3. Open `s1--operator`.
4. Set up `s1`, through management.
5. Set up `shared`, through `s1--operator` (ACC-033 row 4, so mistake 1 cannot occur).
6. Open `agent-a--project` and `agent-b--project`.
7. Set up `agent-a` and `agent-b`.
8. Open `agent-a--operator` and `agent-b--operator`.

Each open and set-up carries `expectedId` from the register reply.

Tools after T1:
- root: `s1.*`, `agent-a.turn.v1`, `agent-b.turn.v1`.
- agent-a: `project.file.read.v1`, `project.directory.list.v1`.
- agent-b: the same plus `project.file.write.v1`.

Checks (not counted): `conduit tools --as agent-a`; `conduit call agent-a.turn.v1 '{"message":"..."}'`; `conduit call agent-b.turn.v1 '{"message":"..."}'`.

=== T2: 5 file lines + 1 command line = 6 ===
```
$ conduit install <lab>/pkgs/history-0.1.0.tar.gz      [1; TOOL adds example.org/s1/history: {version: 0.1.0, digest: 'sha256:e946746509490be49cbcc07d41ff5fed24a6e6c8b377bf423c77f79c02502f69'}]
$ conduit apply
```
Lines added to s1.yaml (lines 8 to 10, 17 and 23 of the final file):
```yaml
  history:                                              # under resources
    type: example.org/s1/history#file-history.v1
    config: {root: <lab>/history}
      history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # under agent-a grants
      history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # under agent-b grants
```
The plan registers 6 documents: `history--t`, `history`, `agent-a--history--t`, `agent-a--history`, `agent-b--history--t` and `agent-b--history`. Registration is authorized through management and `s1--operator`, both of which list `register.v1` (ACC-033 row 1, so mistake 2 cannot occur). It then sets up `history` and opens both grants: 4 requests. The tools are `history.*`. Their descriptions carry file-history.v1's posting convention (accepted addition 5).

=== T3: 0 lines ===
```
$ conduit reach        (or the checks conduit tools --as agent-a / --as agent-b)
```
The inventory in `inspect.v1` lists each Agent's Conduits, the standalone grants among them, which closes finding 4-09. Their names show the entry (`agent-a--project` as `project`, from agents.agent-a.grants.project). Reach around the engine comes from the Types' descriptions (provider work).

=== T4: 2 lines changed ===
```diff
16-      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
16+      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
22-      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
22+      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
```
```
$ conduit apply
~ agents.agent-b.grants.project (line 22), narrowing first:
    close agent-b--project (expectedId)
    replace.v1 agent-b--project--t (revision): -file.write.v1
    register agent-b--project as project, naming the tombstone of that close
    open (expectedId)
~ agents.agent-a.grants.project (line 16): the same four steps, +file.write.v1
tools: agent-b loses project.file.write.v1; agent-a gains it; names unchanged; no session lost
```
8 requests. Windows without access last from each close to its open (seconds, not measured). There is never a moment when both Agents can write. After T4 the store equals the file's translation.

=== T5: 0 file lines + 1 command line = 1 ===
```
$ conduit install <lab>/pkgs/filesystem-0.2.0.tar.gz    [1; TOOL rewrites the pin to 0.2.0 and digest sha256:cbb831f07d2b251c84fcd768a03353203a8a1b2215e862881fef47f866d84436]
$ conduit apply
```
The plan:
```
~ resources.shared: example.org/s1/filesystem 0.1.0 -> 0.2.0 (the package of its own Type) => make-before-break replacement, successor named shared--2 (a CLI convention)
  contracts of the granted operations unchanged; the tools map unchanged (read_text_file, list_directory, write_file); append_file and copy_file are mapped by no operation
  1 register shared--2--t and shared--2 (owner s1); set up shared--2 through s1--operator
  2 close agent-b--project and agent-a--project (expectedId); wait until none is in flight
  3 register agent-a--project and agent-b--project as project -> shared--2, naming the tombstones just made; open both (expectedId)
  4 retire shared (expectedId); unregister shared--t
= history and its grants unchanged: the history package's own pin keeps filesystem 0.1.0 (NAM-014); a grant's capability pin is not compared (NAM-023)
```
About 11 requests. No Agent gains a tool. The translation still says `shared`, while the store holds `shared--2`; `conduit render --bound` shows the binding.

=== T6: 1 line changed ===
```diff
7-    config: {root: <lab>/shared}
7+    config: {root: <lab>/shared-new}
```
`$ conduit apply` makes the same make-before-break replacement as T5, to `shared--3`, with the grants re-pointed under the unchanged `project.*` names. The tool descriptions name the new target (IFC-038 list change).

=== T7: 1 line, removed then restored (counted once, by the baseline's rule) ===
Revoke: delete line 16, then:
```
$ conduit apply
- close agent-a--project (expectedId); the engine records a tombstone for Conduit agent-a--project
```
Check B (not counted): it reads and writes the project; B's Conduits are untouched.

Restore: put line 16 back, then:
```
$ conduit apply
+ register agent-a--project as project, naming the tombstone this working copy's own apply made (from its applied file); open
```
A close made by anyone else would need `--reopen agents.agent-a.grants.project`. Before registering, the plan checks that `agent-a--project--t` still equals the translation (its revision and definition); otherwise it reports a conflict.

Final s1.yaml after T7: 23 lines, the 18 of T1 plus the 5 of T2, with T4's and T6's edits.

## s1_counts
Primary reading: the measurement plan's own rule (authored file lines plus command lines that carry values; tool-written lines not counted). Configuration: grammar plus the owner default plus the SCT-010 Pool grant.

| Task | File lines | Command lines with values | Total | Baseline A70 | Layered package | vs baseline | vs layered |
|---|---|---|---|---|---|---|---|
| T1 | 18 | 1 | 19 | 33 | 16 | better (-14) | +3 |
| T2 | 5 | 1 | 6 | 16 | 6 | better (-10) | same |
| T3 | 0 | 0 | 0 | 0 | 0 | same | same |
| T4 | 2 | 0 | 2 | 8 | 2 | better (-6) | same |
| T5 | 0 | 1 | 1 | 3 (better baseline 2) | 1 | better (-2; -1 vs the better baseline) | same |
| T6 | 1 | 0 | 1 | 2 | 1 | better (-1) | same |
| T7 | 1 | 0 | 1 | 2 | 1 | better (-1) | same |
| All | 27 | 3 | 30 | 64 | 27 | better on 6, same on 1, much more on none: PASSES | +3, all in T1 |

T1 is 3 lines above the layered package for two reasons:
- the `pools` section takes 3 lines, against the package's 2 header lines;
- the two explicit `operator` turn lines, which my design does not derive (derived turns are package-dependent).

THE LADDER: which deltas are load-bearing. T1 totals under the same convention; files r1.yaml to r4.yaml and t1.yaml in the scratchpad's min/ directory:
- R1: frame once (sections and keys), `from` instances, a templates section and a `conduits` section with `as`. 64 + 1 = 65. FAILS.
- R2: R1 plus inline one-off entries. 41 + 1 = 42. FAILS.
- R3: R2 plus `grants` under the consumer and bare names within the project. 33 + 1 = 34. FAILS (more than 33).
- R4: R3 plus the `operator` member, written explicitly, with no default of any kind. 20 + 1 = 21. Passes.
- R4 plus the owner default: 20. Passes.
- R5: R4 plus the owner default plus the SCT-010 Pool grant. 19. Passes; this is the primary configuration.

So inline entries, grants under the consumer and the operator member are each necessary. The derived Pool grant is not: decision 6 can be answered "no derived grants" at +1 line in T1, giving 20, 6, 0, 2, 1, 1, 1 = 31, which still passes.

OTHER READINGS
- Strict, where every command is a line (3 separate installs, `apply` and `reach` counted): T1 22, T2 7, T3 1, T4 3, T5 2, T6 2, T7 3.
  - Against A70: better on T1, T2, T4 and T5, same on T6, one line worse on T3 and T7. That is 4 of 7, a narrow pass of "most".
  - Against the better T5 baseline (2), T5 ties, giving 3 of 7: FAILS.
  - This is identical to the layered package's strict reading, because the change model is the same CLI. It depends on decision 2 (one-step commands, or a counting ruling made before the rerun).
- Block style for each grant (4 lines) and each operator member (3 lines): T1 29, T2 12, T4 2, T5 1, T6 1, T7 4 (a whole grant block). Better on 5, same on T3, more on T7 (4 against 2).
- JSON with one key per line: fails T1 and T2, as for the layered package. YAML must be the documented form of `spec/`'s examples.
- Fresh author: at the 1.7 times factor the earlier fresh author showed in T1, 19 becomes about 32 against 33. That is no margin. The same risk hits the layered package (16 becomes about 27).

OTHER MEASURES (paper)
- Files the author edits: 1 (s1.yaml); `conduit.yaml` is tool-written.
- Grants to Agents made by hand: T1 2, T2 2, T4 2 edits, T7 1.
- Root's grants: 2 written (operator turn members) and 1 derived (`s1--operator`), listed apart.
- Lines naming a contract address: T1 8 and T2 3, against 45 for the coordinator.
- Provider-specific lines: 0.
- Tool names changed by any task: 0.
- Engine requests sent by the tool: about 13 + 5 + 0 + 8 + 12 + 11 + 3, about 52, against 35 typed by the coordinator and about 51 for the layered package.
- Mistakes 1 and 2 cannot occur under R5. Under R4, a missing Pool operator line is diagnosed: a warning that does not reject, and the two-tier refusal at set-up.
- Concepts met: about 15 to 17 (sectioned document and sections, entry, inline type and config, grant (to, capability, operations), operator, Pool, Conduit, capability, operation, root Interface, Type, install, apply, plan, render or reach). This is about the same as today's 16 and the critic's honest 15 to 20 for the layered package.
- Longest line: 221 characters.

Everything above is a paper count, except that today's loader and core accept the translated T2 documents (a run). Ruling 45 still requires a fresh author's rerun.

## other_scenarios
All sketches below use the same grammar. Pools are written as `pools: {x: {type: <core>#pool.v1}}`, with entries in block style. FQ means fully qualified.

S2: one Linear login shared under different limits; the key in 1Password; a filter, a bound and dependent calls. About 45 to 50 lines.
```yaml
pools:
  creds: {type: github.com/shrav-k/conduit/core#pool.v1}
  tools: {type: github.com/shrav-k/conduit/core#pool.v1}
templates:
  conduits:
    linear-all: {capability: example.org/linear#issues.v1, operations: [<six FQ issue operations>]}
resources:
  op-token:
    type: example.org/keychain#keychain-secret.v1
    owner: creds
    config: {service: conduit, account: op-service-account}
  linear-key:
    type: example.org/onepassword#op-secret.v1
    owner: creds
    config: {locator: 'op://conduit-test/linear/credential', token: {kind: Resource, name: op-token}}
    grants:
      token: {to: op-token, capability: github.com/shrav-k/conduit/core#secret-provider.v1, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}
  linear:
    type: example.org/linear#linear.v1
    owner: tools
    config: {token: {kind: Resource, name: linear-key}}
    grants:
      key: {to: linear-key, capability: github.com/shrav-k/conduit/core#secret-provider.v1, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}
  linear-scoped:
    type: example.org/linear#scoped-adapter.v1
    owner: tools
    conduits:
      upstream: {from: linear-all, target: linear}
agents:
  reader:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    operator: {capability: <...#conversation.v1>, operations: [<...#turn.v1>]}
    grants:
      linear:
        to: linear-scoped
        capability: example.org/linear#issues.v1
        operations: [example.org/linear#issue.list.v1, example.org/linear#issue.get.v1, example.org/linear#issue.search.v1]
        requirements: {team: {requirement: example.org/linear#team.v1, arguments: {team: ENG}}}
  writer:
    type: example.org/conduit/agent-codex#codex.v1
    operator: {...}
    grants:
      linear: {to: linear, capability: example.org/linear#issues.v1, operations: [<six FQ>]}
      delegate:
        to: {kind: Resource, name: access}
        capability: github.com/shrav-k/conduit/core#access-control.v1
        operations: [github.com/shrav-k/conduit/core#open.v1]
        requirements: {bound: {requirement: github.com/shrav-k/conduit/core#bound.v1, arguments: {targets: [{kind: Resource, name: linear}], operations: [example.org/linear#issue.list.v1]}}}
```
- Secret references stay explicit `{kind, name}` (SEC-001; the bare-name rule never touches `config`).
- The get-secret grants have Resource consumers (SEC-009) and are opened by root, since apply runs as root (SEC-008).
- The SCT-010 default gives root a grant to both Pools.
- Not expressible yet: the expected account (ruling 49). It becomes a `config` key once the Types have it, with no grammar change.
- E13 (key rotation) is a plan cascade, not language: new `linear-key--2`, replace `linear`, re-point both grants under tombstone-safe supersession.

S3: two machines, placement, runtime and shared-host.
```yaml
resources:
  tailnet: {type: example.org/tailscale#machine-provider.v1, owner: system, config: {tailnet: example.ts.net}}
  m2-docker: {type: example.org/docker#runtime.v1, owner: s1, config: {host: 'ssh://m2'}}
  shared: {type: <...local-directory.v1>, placement: m2, runtime: m2-docker, config: {root: /home/me/shared}}
agents:
  agent-b:
    type: <codex.v1>
    placement: m2
    runtime: m2-docker
    grants:
      edit:
        to: shared
        capability: <...>
        operations: [<...>]
        requirements: {host: {requirement: github.com/shrav-k/conduit/core#shared-host.v1}}
conduits:
  root-system: {consumer: {kind: Interface, name: root}, target: {kind: Pool, name: system}, capability: github.com/shrav-k/conduit/core#pool-management.v1, operations: [<FQ register.v1, set-up.v1, ...>]}
```
- Entries are shown in flow here for brevity.
- `system` is not an entry, so root's grant to it is written in `conduits:`.
- `nodeSelector` stays in Types and items, which are framed contracts. Enrollment is a request.
- NOT EXPRESSIBLE, here or in the canonical form: peer mode (an Interface standing for another engine's actor, a trust domain). This waits for ruling 51's promises. The sectioned form gains such a construct for free once the canonical form has it, because entries mirror canonical keys.

S5: a control under contention.
```yaml
resources:
  catalog:
    type: example.org/catalog#catalog.v1
    config: {limit: 20}
    controls:
      example.org/catalog#set-read-limit.v1: {inputs: {limit: {schema: {maximum: 50}}}}
agents:
  agent-a:
    grants:
      catalog: {to: catalog, capability: example.org/catalog#catalog.v1, operations: [example.org/catalog#read.v1, example.org/catalog#set-read-limit.v1]}
interfaces:
  ops:
    type: github.com/shrav-k/conduit/core#interface.v1
    grants:
      catalog: {to: catalog, capability: example.org/catalog#catalog.v1, operations: [example.org/catalog#set-read-limit.v1]}
```
- The language says `config` is the value at registration. After a control, `inspect` shows the live value with its revision.
- The plan treats a value that a control changed as known, and turns an edit at a control's pointer into a control request with a revision (critic gap 5). That is CLI work.

S6: revocation and retirement in flight.
- R carries `conduits: {upstream: {from: <template>, target: backing}}`. A retire hook and task-capable operations live in the Types.
- Removing A's grant line becomes an explicit `close.v1` with `expectedId`, plus an engine tombstone keyed by `a--<grant>`. No later apply, register or re-registration of the file reopens it without naming the tombstone.
- CON-008, ACC-034 and ACC-039 are unchanged.

S7: an outside author packages a server.
- Contracts and manifest items stay framed, unchanged.
- The package may ship a templates-only sectioned document:
```yaml
templates:
  resources:
    linear:
      type: '#linear.v1'
      inputs:
        token: {schema: {$ref: github.com/shrav-k/conduit/core#secret-reference.v1}, to: [/config/token]}
  conduits:
    read: {capability: '#issues.v1', operations: ['#issue.list.v1', '#issue.get.v1']}
```
- `exports` lists `linear` and `read`. A consumer writes `linear: {from: example.org/linear#linear, owner: tools, arguments: {token: {kind: Resource, name: linear-key}}}`.
- Templates have two notations; contracts have one.

S9: first start and repair.
- Engine configuration (trust domain, hostname, the list set on the runner, the store) stays outside documents.
- Operator Interfaces are `interfaces:` entries, with the document's `operator: ops` making them the target of every operator member and Pool default.
- Providers owned by `system` need a written root→system grant.
- Repair is requests (set-up again, reconcile, register-replacement, declare-lost, unregister). The plan shows the file, what is registered and what is observed apart, and never repairs on its own (OBS-004).

S10: providers swapped.
- New entries: a Docker runtime Resource; `runtime:` on instances; a Git package-source Resource owned by `system`, claiming its scheme; new store Types. `fetch` lines in `conduit.yaml` are tool-written.
- Each swap makes new objects (PRV-006, SEC-005 and PLC-009 hold for life). The cascade (replace every instance, re-point grants, new sign-ins for replaced Agents) is a plan. The language assigns it no meaning beyond immutability: the honest answer to the scenarios checklist's question 3.

S4 and S8 (for completeness).
- S4: an inline Pool with `entries` from `templates.resources`, `{entry: x}` references in entry arguments, `optional: true`, hooks through the Types' `requires`, and the default Pool grant including `reconcile.v1`.
- S8: per-Agent filter grants, a declared upstream that carries filters onward, `supports` in items, and `records` in the Type.

THE OWNER'S REAL USE: 10 workers. About 40 shared lines plus about 9 per worker, about 130 lines, against about 340 to 420 today; the floor is about 1 per worker.
```yaml
pools:
  dev: {type: github.com/shrav-k/conduit/core#pool.v1}
templates:
  resources:
    tree:
      type: example.org/git#worktree.v1
      inputs:
        branch: {schema: {type: string, minLength: 1}, to: [/config/branch]}
  conduits:
    tree-rw: {capability: example.org/git#tree.v1, operations: [<FQ read, list, write>]}
    tree-ro: {capability: example.org/git#tree.v1, operations: [<FQ read, list>]}
    queue: {capability: example.org/queue#queue.v1, operations: [<FQ read, post>]}
  agents:
    codex-worker:
      type: example.org/conduit/agent-codex#codex.v1
      inputs:
        tree: {schema: {$ref: github.com/shrav-k/conduit/core#resource-reference.v1}, to: [/conduits/tree/target]}
      conduits:
        tree: {from: tree-rw}
        main: {from: tree-ro, target: main}
        orders: {from: queue, target: orders}
    claude-worker: <the same with claude-code.v1>
resources:
  main: {type: example.org/git#checkout.v1, config: {path: /src/conduit}}
  orders: {type: example.org/queue#queue.v1, config: {...}}
  tree-1:
    from: tree
    arguments: {branch: wo/101}
agents:
  admin:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    operator: {<turn>}
    grants:
      main: {...}
      orders: {...}
      worker-1: {to: worker-1, capability: example.org/conduit/agent-codex#conversation.v1, operations: [example.org/conduit/agent-codex#turn.v1]}
      tree-1: {to: tree-1, capability: example.org/git#tree.v1, operations: [<FQ read, list>]}
  worker-1:
    from: codex-worker
    arguments: {tree: tree-1}
```
- The worker's own grants are declared Conduits, so TPL-012 opens them before its run.
- What remains:
  - no replicas or for_each (IN5);
  - two templates for the two clients (IN2);
  - the admin's grants cannot sit in the worker template (CON-002);
  - a Pool cannot make Agents;
  - package upgrades replace every worker, bringing ten sign-ins (INS-020, experiment 010);
  - the expected account waits for ruling 49;
  - the per-worker view of the queue needs a filter or a queue per worker.
- "Grant admin read on every tree" sugar is not in the minimal delta. If it is ever added, it must expand to N explicit Conduits.

WHAT ESCAPES TO THE FULL FORM
- Contracts and the manifest only. These are not an escape: they are the one notation for those kinds.
- Every template and instance construct is reachable inline or through `templates` with `from`. Framed documents remain legal beside sectioned ones, referring to entries by their stable names.
- After a CLI replacement renames an object (`shared--2`), the plan refuses to retire anything that a Conduit outside the file still targets, unless it re-points that Conduit or is given a flag (the layered package's fix for critic gap 6).

## change_model
Ruling 53 #4 holds: change is explicit plan and apply, composed of today's requests, never automatic, and never pruning or reopening without a flag. None of it becomes language in the minimal delta.

HOW A CHANGE IS EXPRESSED. The author edits the sectioned file, which is normative for what it means when registered, or installs a package (the tool rewrites the pin). The author then runs `conduit plan` and `conduit apply`. How a plan is approved without a terminal, and how that counts, is still open (decision 2). The options are the same as the layered package's: a saved plan applied by its ID, one-step commands (`grant`, `revoke`, `upgrade`), or a counting ruling stated before the rerun.

HOW IT IS EXECUTED. The plan works as follows:
1. It reads the inventory, the tombstones and its committed file of applied IDs.
2. It diffs them against the file's translation.
3. It prints each request with:
   - the IDs it expects;
   - the tools each Agent gains or loses;
   - the windows without access;
   - the authored line behind each request.
4. It sends exactly those requests: `register.v1`, `replace.v1` (for a generated template whose content changed), `open.v1` and `close.v1` with `expectedId`, `set-up.v1`, `retire.v1`, `unregister.v1`.

Classes of change:
- Grant added: register, then open.
- Grant removed: close, which leaves a tombstone, then unregister the grant's template.
- A grant's operations, target or requirements changed: close, `replace.v1` its `--t` template, register under the same name and `as` (naming this apply's own tombstone), open. Narrowings run first.
- A Resource's config, or the version of the package exporting its Type, changed: a make-before-break replacement under a successor name `<key>--<n>`, then grants closed and re-registered toward the successor, then the old Resource retired with `expectedId`.
- A config value at a control's pointer changed: a control request carrying the revision (critic gap 5).
- An Agent changed: refused without `--replace`; the plan states that its session and login are lost (ruling 49).
- An entry removed: refused without `--prune`.
- Anything closed or retired by anyone other than this working copy's applies: refused without `--reopen KEY`.
- Root-consumed operator grants are add-only (ACC-024): the plan refuses to narrow or remove them.

Before instantiating from a generated template, the plan checks that its definition equals the translation. Otherwise it reports a conflict, which closes the `replace.v1` hijack the skeptics found in design E.

WHAT IS NORMATIVE AND WHAT IS NOT
- Normative:
  - the file's meaning when registered (SCT-*);
  - every request and its preconditions (`expectedId`, revisions);
  - engine tombstones (LIF-038), keyed by the translation's stable names, so a revocation survives replacement of the consumer, another working copy, an Agent operator, and re-registration of the file;
  - SUB-007 and OBJ-002 (no implicit change).
- Not normative:
  - the diff;
  - the choice and order of requests;
  - make-before-break against break-before-make;
  - successor names;
  - the applied-ID file;
  - flags.

  A second CLI may plan differently, but can only send normative requests. Thesis 2 therefore covers what the file means and what each request does, not which plan was chosen. A normative plan function would be a later, separate ruling (about 25 to 30 promises and 50 plan vectors), as the prior-art map recommends: CloudFormation's change set, not an apply inside the engine.

T4 TO T7 UNDER THIS MODEL (detail in s1_rewrite)
- T4: 2 edits and 1 apply, giving 8 requests, narrowing first. The store then equals the translation.
- T5: install, then apply, giving about 11 requests, make-before-break to `shared--2`. No tool gained; the history grants are untouched.
- T6: 1 edit and 1 apply, the same replacement, to `shared--3`.
- T7: delete a line and apply, which closes and leaves a tombstone. Restore the line and apply, which registers and opens, naming the tombstone this copy made. A close by a responder would need `--reopen`.
- Windows without access last seconds in T4 to T6, and T5 gains a window that the coordinator's renaming route did not have. This is the same as the layered package; an atomic `swap.v1` stays a later option.

## cost
SPECIFICATION
- New: SCT-001 to SCT-009, plus SCT-010 if decision 6 takes the core-only Pool grant.
- Amended: OBJ-017, OBJ-019, OBJ-020, OBJ-022, INS-001, SUB-005, SUB-010, SCH-018 and MAN-011; observable edits to NAM-027, OBJ-012 and OBJ-013.
- Total: about 19 to 21 promise touches, about 4 to 5% of 455.
- spec/definitions.md gains 6 terms; spec/README.md's contents change; spec/core/README.md's export list gains the new Schema contracts.
- spec/schemas/sectioned.yaml: about 200 to 300 lines. 10 to 15 new examples.
- No core operation changes for the delta.
- Ruling wording: confirm or amend rulings 35 and 37, and confirm the reading of ruling 8's IN9 and ruling 18's V2.
- Beside these, and needed by any route: the five accepted additions, about 11 to 14 touches, about 300 to 600 core lines and 10 to 17 vectors. All in, about 30 to 35 promise touches, against 11 to 14 for the layered route and 50 to 80 for designs C and D.

VECTORS: about 30 to 40 new; 0 of 367 invalidated; 0 snapshots regenerated
- I searched spec/schemas/examples, the loader tests and conformance/ for a document that is a map with no `kind` and found none, so the route change in SCT-001 alters no recorded verdict. This needs confirming by running the suite.
- About 8 loader `read`/`load` cases: recognition, closed keys, a hint about `kind`, `apiVersion` from the manifest or missing, JSON array items.
- About 18 to 24 `translate` cases, a new runner input kind: A/B pairs per SCT rule, positive and negative (inline entries per family, `inputs`/`arguments` rejected inline, grants with targets inside and outside the project, operator explicit, null and default, owner default with one Pool or ambiguous, bare names, `--` in keys, names over 63 characters, clashes with framed documents, diagnostics at authored pointers with secrets redacted).
- About 4 to 8 registration cases:
  - the S1 equivalence: the sectioned file against its rendered translation, giving exactly the same transition, prototyped by my scratch case;
  - order independence;
  - a mixed sectioned and framed project;
  - a package with a templates-only sectioned document, accepted and rejected at install;
  - the derived Pool grant visible and not opened by registration;
  - a tombstone blocking re-registration of a translated grant name (with LIF-038).

REFERENCE MODEL REBUILD BY CRATE
- loader:
  - translate.rs, about 500 to 800 lines: pure syntax over Node, keeping positions; no package reads; no capability inference.
  - Routing a map with no `kind` to `Role::Sectioned`, plus the schema pickup through `Specification::from_documents`: about 60 to 120.
  - The translation map (derived document, pointer, rule): about 100 to 150.
  - Tests: about 400 to 700.
- core: 0 lines for the delta. Every caller already receives canonical JSON (core/src/packages.rs:16), and archive admission reuses the loader, so packages get templates-only sectioned documents free.
- shell:
  - mapping core diagnostics (`/objects/<name>`) back to authored pointers and lines: about 100 to 200;
  - `render`: about 50.
  - CLI work shared with the layered route and not part of the delta: install writing pins, about 200 to 400; plan, apply and executor with the safety cases, about 1,500 to 2,500 plus golden plan tests; `reach`, about 100 to 200.
- conformance runner: the `translate` input kind and an `equivalent` option on register-project, about 150 to 300; case.schema.json, about 40 to 80.
- tools/check_schemas.py: about 60 to 120 lines to validate `sectioned.v1` and its examples by schema. Verdicts that depend on the translation are left to the Rust vectors, so the translation is not written a second time in Python.

ROUGH SIZE OF THE NORMATIVE DELTA
- About 1,400 to 2,500 lines including tests, 0 of them in the core.
- 2 work orders: (1) the spec batch, schema, examples and Python checker; (2) the loader translator, diagnostic mapping, runner kind and vectors.
- These sit beside the 3 orders the layered route already needs: the five additions, the planner and apply, and install, reach and the provider versions.

AGAINST THE LAYERED ROUTE
- The layered route's CLI compiler (1,100 to 1,600 lines plus golden tests, which are not binding) is replaced by a loader translator of similar or smaller size, because it has no short names, no capability inference and no monotonic naming.
- What normativity adds: about 20 promise touches, the schema, 30 to 40 vectors, and the runner and Python work. It also needs decision 6, but only for SCT-010, which is optional.
- What it saves:
  - the separate format reference (decision 9 becomes `spec/`);
  - the about 27 promises the CLI would mirror, since each future canonical key appears in the sectioned form through a schema `$ref`, not compiler code;
  - the Kubernetes trajectory of generators that are used widely but specified nowhere.

## what_is_lost
1. **Self-description.** A sectioned document takes its version from the manifest and its kinds from its sections, so it cannot be read alone. Copied into another project, it can change meaning across specification versions, as a Cargo edition does.
2. **One notation per kind.** Templates and instances get two normative notations, framed and sectioned; contracts keep one. Implementers learn about 6 more concepts, and the specification grows about 4 to 5%. None of the existing 455 promises or their machinery is simplified: declared Conduits, `replace.v1`, `register-replacement.v1`, inputs and the frame all stay.
3. **Store clutter.** Every inline entry and every grant makes a generated `NAME--t` template, so in S1, 9 of the 18 objects are templates. They show in inspection and records. A third party holding `replace.v1` can change them; the plan detects this and reports a conflict rather than instantiating from them.
4. **TPL-012 for grants.** Grants are standalone Conduits, so the guarantee that Conduits open before the run is not exercised for them. Apply opens them first, and the failure is closed but visible only in inspection. Declared Conduits remain for S4, S9 and the workers in the real use.
5. **Change is not language.** The normative meaning covers registration only. After a CLI replacement, stored names (`shared--2`) differ from the translation's (`shared`). How T4 to T7 are carried out is a tool's plan; a second CLI may choose differently, though it can only send normative requests. So thesis 2 holds for what the file and each request mean, not for the transition chosen.
6. **Root's operator grants are add-only (ACC-024).** Removing or narrowing an Agent's `operator` member, or a Pool's derived grant, cannot be applied.
7. **Long lines.** Capability written per grant and every name fully qualified give lines up to about 220 characters. Authors who write in block style lose T7's margin (4 lines against 2) and most of T1's.
8. **T1 is 3 lines above the layered package** (19 against 16): the price of having no package-dependent defaults (no derived turns, no capability inference). All counts are on paper apart from the scratch registration of the translated documents. Against the strict reading and the better T5 baseline, the result rests on decision 2.
9. **The optional Pool default is an implicit grant.** If decision 6 takes SCT-010, root's Pool grant is written by no one: shown in `render`, plan and inspection, fixed by the text and the specification version, but not visible in the author's text.
10. **`register.v1` stays canonical.** Agents that manage through Conduits cannot send sectioned documents; they produce the public, deterministic translation or write framed documents.
11. **No reuse at scale.** About 9 lines per worker against a floor of about 1: no replicas or for_each (IN5), two templates per client Type (IN2), no Agent made together with its own Resource, and admin grants written per worker.
12. **Windows without access** remain for seconds in T4 to T6, and T5 gains one that the coordinator's renaming route did not have. The same holds for the layered package, until an atomic `swap.v1`.

## Verdict safety (major)
Holds with fixes, but not as claimed. The engine-level guarantees survive because the core still only ever sees canonical documents. Every grant is a real Conduit with an explicit, fully qualified operation list; registering never opens anything; and immutability, records and ACC-024 are untouched. Two of the design's safety claims are false as written, though. (1) "Thesis 2 holds for what authors write": after the first replacement made by the command-line tool (T5 or T6), the file's normative translation names objects that the live system no longer uses. It re-resolves those names on every re-registration. Any second conforming client, or any Agent that follows "the public, deterministic translation", can be silently re-pointed at a reused name. (2) "A revocation survives another working copy and an Agent operator": the tombstone override is supplied from a committed, shared file of applied IDs, and it can be bypassed by renaming a grant key. Four medium holes: `as` lets two Conduits of one consumer share a local name; the owner default puts new Resources inside existing Pool bounds without a word in the file; the Pool default grant fans out to a non-root operator; and Type schema defaults still come from packages. Each has a cheap fix. With the fixes the file costs about 33 lines, against the baseline's 64, and still passes on the primary reading.

Everything below is from reading the files. I changed nothing in the repo. Paths are under <home>

WHAT HOLDS (checked)
- **Access only through open Conduits with explicit lists.** Every grant, operator member and Pool default translates to a standalone Conduit with a fully qualified `operations` list (ACC-003). Registering opens nothing (SUB-006, INS-016). The wire stays `document.v1`, so the core's checks all apply. The scratch run of the hand-translated T2 documents is consistent with this.
- **SCT-010's seven operations match `pool-management.v1` exactly** (spec/core/capabilities.yaml:163-170), `reconcile.v1` included. That fixes design E's defect (1).
- **Generated names cannot collide with engine-generated names.** OBJ-003 names an unnamed instance with its template's name, one hyphen and a suffix (spec/promises/objects.yaml OBJ-003), never `--`.
- **Critic gap 2 is closed for consumer replacement.** Tombstones are keyed by the grant's name (`agent-a--project`), which stays the same when the Agent is replaced.
- **No forged provenance field.** Nothing beyond the canonical object is stored, so the client cannot plant one (C's D1).

COUNTEREXAMPLES, MOST SEVERE FIRST

1. MAJOR (thesis 2, OBJ-010 spirit, G1): after a replacement, the normative meaning and the live system diverge, and the normative meaning redirects by name.
   - Setup: T5 moves the grants to `shared--2`, then retires and unregisters `shared` and `shared--t`. LIF-035 frees the name `shared`.
   - The file still says `to: shared`. SCT-003 says the file means exactly its translation, `target: {kind: Resource, name: shared}`. The binding `shared` to `shared--2` lives only in the tool's applied file and `render --bound`, which is not normative.
   - Now another project, or a delegate holding `register.v1`, registers a Resource named `shared`. Three conforming clients then misbehave:
     - (a) An Agent managing through Conduits (preservation test 5), told by the design to "produce the public, deterministic translation", re-registers `agent-a--project`. It resolves to the impostor, with no error.
     - (b) A second CLI, or the same CLI on a second machine (S3) without the applied file, diffs the live grant (`shared--2`) against the translation. It plans "~ target shared--2 -> shared": a plausible-looking redirect.
     - (c) The design's own claim that "a second implementation must agree" fails: two conforming planners choose different targets for the same authored line.
   - Fix (pick one):
     - Replacement keeps the entry's own name: break-before-make under `shared`, so the store always equals the translation, with tombstone overrides and windows. `as` already keeps tool names stable.
     - Or add a precondition to `register.v1` mapping each written reference to the ID it must resolve to, extending addition 2 (`expectedId`). Every client then fails closed on a reused name.
   - In either case, state in SCT-003 that the meaning covers registration on an engine where the names are unbound or bound to this file's own objects.

2. MAJOR (G3, immediate and durable revocation, tests 5 and 8): the tombstone override is available to anyone who holds the shared file or the diagnostic.
   - (a) The plan supplies the override "only for closes made by this working copy's own applies", but reads them from its *committed* file of applied IDs.
     - Copy A deletes line 16 and applies, and the close is recorded. Copy B's s1.yaml still has the line, for example on a branch, or after a merge that kept B's file but took A's applied file.
     - B's next apply for an unrelated edit sees the grant missing and the tombstone ID "own", and reopens it without a flag. This is design E's verdict #3 again; the design claims to survive "another working copy".
   - (b) Renaming a grant key (`project` to `workspace`) makes `agent-a--workspace`, which no tombstone covers, so a revoked grant comes back under a new name. Design E's verdict #3(b) is unaddressed.
   - (c) If the refusal names the tombstoned ID, which ruling 52 and LIF-037's style suggest, an LLM Agent that retries with the value from the error message defeats the guard in one step.
   - (d) Predictable names let anyone holding register and open authority pre-tombstone `agent-a--project` and block the author's grant. The owner's real use reuses names on every order, so overrides become routine and the guard empty.
   - Fix:
     - Clearing a tombstone is a separate operation (for example on `access`) that can be withheld from delegates.
     - The plan supplies an override only within the one apply that made the close (T4, T5).
     - T7's restore requires `--reopen KEY`, which costs +1 line in T7: 2 against the baseline's 2.
     - The plan also flags any new grant whose consumer and target entries match a tombstone as a reopen.
     - State who may read a tombstone (critic gap 8 remains).

3. MEDIUM-MAJOR (ruling 52, test 5): `as` lets local names be shared or squatted.
   - IFC-014 (spec/interface/implementation.yaml) rejects only an *equal tool name*. Two Conduits of one consumer may share a local name when their operations differ.
   - Example: agent-a has `project` to `shared` [read, list]. A delegate with an `access` bound over its own Pool (ACC-030: "for any consumer") opens Conduit `x`, `as: project`, to its Resource, listing [file.write.v1]. agent-a now sees `project.file.read.v1`, `project.directory.list.v1` and `project.file.write.v1`, and writes "project" files into the delegate's Resource.
   - The same thing happens by accident when a package template's declared Conduit `tree` meets a grant `tree` that lists different operations.
   - During T4's or T7's close-to-open window, a squatter's `as: project` makes the author's re-registration fail under IFC-014 and leaves the impostor in place.
   - Fix: when IFC-014 is amended for `as`, make local names unique per consumer among Conduits that are not closed, and reserve the local name of a tombstoned grant for that consumer.

4. MEDIUM (ruling 52, ACC-029, design D's HIGH #4): the owner default widens delegates invisibly.
   - ACC-029: a bound naming a Pool covers "every Resource that Pool owns, including those instantiated after".
   - In the real-use sketch, admin holds `access` bounded by Pool `dev`. Adding `prod-db: {type: ...}` with no owner puts it in `dev`, so admin may open Conduits to it for any consumer. The authored line says nothing about this.
   - Fix: drop the owner default and write `owner` on every Resource entry. Cost: +1 line in T1 and +1 in T2.

5. MEDIUM (G2, S2, test 8): SCT-010 and the document's `operator` fan out.
   - For root, SCT-010 adds no authority, since root is unbounded.
   - With `operator: ops`, one line grants `ops` the seven pool-management operations, `retire-with-resources` and `register` among them, on every Pool. That includes S2's credentials Pool, which exists to keep authority over secrets apart from authority over tools.
   - The document's `operator` is a bare name kept as written when the project does not make it. If the file defining `ops` is missing, or moved into a subdirectory that SUB-010 does not read, the default resolves to another owner's `ops` Interface on the engine, and every operator member and Pool default goes to that owner.
   - Moving an entry into another document silently changes its operator.
   - Fix:
     - Apply SCT-010 only when the operator is root.
     - Require the document's `operator` to be root or an Interface that this project makes.

6. MEDIUM (the T5 hole and A's C2): "defaults depend only on the text" holds only for the translation, not for the objects.
   - The objects still take package defaults: Type schema defaults (SCH-009), default implementations (TPL-004) and input defaults of package templates for referenced entries. t1.yaml relies on them by omitting Agent `config`.
   - NAM-024 (spec/promises/naming.yaml:467) forbids a `default` only on added *Requirement* inputs. A later release of `claude-code.v1` may add an optional property with `default: true`, for example a reach flag. An upgrade with `--replace`, and 0 file lines, picks it up.
   - The premise of ruling 53 #7 conflicts with NAM-024. packages/agent-claude-code/contracts.yaml:16-19 has the flags `required` with no default. Relaxing that in 0.1.2 under the same name "changes something the earlier one held", so 0.1.2 must export `claude-code.v2`, and t1.yaml's `claude-code.v1` with omitted config would be rejected.
   - Fix:
     - Amend NAM-024 so that a property added to a Type's schema states no `default`.
     - Have the plan diff the default-filled config across package versions and treat any change as a widening.
     - Use `.v2` in the examples.

7. MEDIUM (G1, design E's verdict #4): a time-of-check-to-time-of-use race on `replace.v1`.
   - T4 sends `replace.v1` for the `--t` template, then `register` for the grant, as two requests. The plan checks the template before it instantiates.
   - A holder of `replace.v1` can swap the template in between. INS-020 protects only instances that already exist, so the new grant instantiates the swapped content and the plan then opens it.
   - Fix: inspect the registered Conduit's resolved definition (INS-013) and compare it with the translation *before* `open`, which carries `expectedId`. Put a new template and its instance in one submission.

8. LOW-MEDIUM (ruling 52 explanations): names are not provenance.
   - `--` is reserved only in sectioned keys. OBJ-001 allows it in framed documents and on the wire, so anyone may register `agent-a--project` or `shared--2`.
   - `reach` and inventory explanations that infer "from agents.agent-a.grants.project" out of a name can therefore be forged.
   - Fix: show the registering caller and request from the records beside the inferred entry.

9. LOW (thesis 2 for authored text): `apiVersion` defaults to the manifest's, which tools write.
   - If `install` or `init` ever bumps it, every sectioned document changes meaning (SCT-010, default rules) with no edit by the author.
   - Fix: tools never change the manifest's `apiVersion`, or sectioned documents state it themselves (+1 line per file).

10. LOW (cost claim): "a map with no kind: found none; 0 vectors invalidated" is false.
    - Five sets under spec/schemas/examples/invalid-by-rule/ hold manifests with no `kind` in files that are not manifests: MAN-018-item-binds-a-variable-of-identity, SCH-009-allof-defaults, SCH-009-conflicting-defaults, SEC-024-secret-header-to-http and TPL-003-template-selects-an-operation-of-the-engine.
    - Both tools/check_schemas.py:53-56 and reference-model/crates/loader/tests/examples.rs:159-166 (`by_rule_role`) treat "no `kind`" as "manifest". That is exactly the trigger SCT-001 claims.
    - Both harnesses and these files must change, and the routing rule conflicts with an existing convention.

11. LOW: root's namespace.
    - root's declared Conduits have the local names `management` and `access` (spec/core/templates.yaml:103-109).
    - An Agent entry named `access` with an operator member gives root `access.turn.v1` next to `access.open.v1`; IFC-014 permits it. A placement written as `m2` becomes `{kind: Resource, name: m2}` if the project has a Resource `m2`.
    - Fix: reserve the first-start names as entry names, and rewrite a bare name only where the field's kind is not fixed.

12. INHERITED, NOT RESOLVED:
    - Approval without a terminal (decision 2). Under the critic's one-apply-per-change reading, this design fails exactly as the layered one does.
    - ACC-024 makes every `operator` member under root, and the Pool default, impossible to edit or remove. The file then diverges from the store for good.
    - TPL-012 is never exercised for grants, and T5 adds a window without access.

COST OF THE FIXES
- Lines: +1 in T1 and +1 in T2 (owner written), +1 in T7 (`--reopen`).
- New S1 total: 20, 7, 0, 2, 1, 1, 2 = 33, against the baseline's 64. That is better on 5 tasks and the same on 2, so it still passes on the primary reading.
- Specification: about 3 to 5 more promise touches. These are a local-name rule in IFC-014, a tombstone-clearing operation, the NAM-024 amendment for Type schemas, and optionally a reference precondition on `register.v1`.

## Verdict expressiveness (major)
The sectioned form can express the registration-time configuration of S2-S10, the owner's 10-worker use and rulings 49-51 (as far as they are defined) without framed templates or instances. Contracts and manifests stay framed, as designed. It does not stay expressive across change, though. Once the CLI does its first make-before-break replacement (T5/T6, E13, S10 swaps, worker upgrades), entry names stop naming the live objects. From then on the escape hatch fails, new lines that name a replaced entry cannot be registered, and the plan's own rule that "a generated template must equal the translation" conflicts with the re-pointing it has to do. One major gap, five medium and several minor. Each has a stated fix, and none needs a fallback to the canonical form.

Repo read-only; checked against spec/ and the reference model. Paths are under <home> unless absolute.

MAJOR
M1. Names do not follow CLI replacements, so the escape hatch and SCT-003's "means exactly its translation" break after the first make-before-break.
- What happens: T5/T6 replace `shared` with `shared--2`/`shared--3` and unregister `shared--t`. The translation still says `{kind: Resource, name: shared}`. Under OBJ-010 stored references are never redirected, and under LIF-035 the old name is freed once the instance is removed.
- Failure 1: after T6, add `agent-c` with `grants: {project: {to: shared, ...}}`. The normative translation names a retired object (CON-025) or a missing one (OBJ-009), so it is rejected. It registers only because the CLI rewrites it through its private binding, so what is registered is no longer the translation.
- Failure 2 (escape hatch): a framed document written after the replacement can refer to `shared` and `shared--t` by name, but they resolve to retired or missing objects. Only grant names (NAME--LOCAL) and `templates:` names stay stable. The critic's gap 6 (docs/proposals/simplification/research/critic.md:57-60) is not closed. The design's guard ("refuse to retire what an outside Conduit targets unless re-pointed") covers only references already stored, and re-pointing a framed Conduit makes the store differ from that document too.
- Failure 3 (cascades): the same staleness hits references the translation never rewrites:
  - secret references in `config` (S2 `linear.config.token`; S10 store swap);
  - `bound.v1` `targets` in requirement arguments (S2 E07; the admin's bound in the real use);
  - `runtime` (S10);
  - declared-Conduit targets (S2 `linear-scoped.upstream`; S6 R to its backing Resource; S8 search to docs; workers to `main` and `orders`).
- Declared Conduits are worse. LIF-009 makes the replacement from the stored resolved target, so `register-replacement.v1` to a replaced target reports invalid (CON-025). The consumer must then be replaced, which loses the login for Agents (ruling 49), and its generated template must be rewritten with `replace.v1` to `linear--2`. That trips the design's own conflict rule ("plan checks a generated template equals the translation").
- Failure 4: a second working copy without the applied-ID file reads `to: shared` as a new `shared` and duplicates it, even though tombstones are said to survive "another working copy".
- Fix (pick one, normatively): (a) replacement that keeps names (break-before-make, naming the copy's own tombstone; longer windows); or (b) an engine-level "entry slot" for top-level entries that resolves a name to its current non-retired holder, as POOL-014 already does for Pool entries; or (c) declare thesis 2 to hold only until the first replacement.

MEDIUM
D1. A declared Conduit cannot be revoked per instance in the file (S6, real use).
- Workers' grants to `main`, `orders` and their tree live in the shared `codex-worker` template, so there is no line to delete for worker-3 alone.
- Editing a template's `conduits` is a template change, so the consumer is replaced (Agents are refused without `--replace`, and the login is lost). Revocation is therefore only a `close.v1` request outside the file.
- Tombstones keyed by (kind, name) do not protect declared Conduits: their names are generated (OBJ-003) and LIF-009 issues new ones.
- Fix: say that declared-Conduit revocation is request-only, or key tombstones by (consumer entry, local name).

D2. Tombstones keyed by name collide with the real use's name reuse.
- Ruling 53 #3 tombstones both closes and retirements (docs/proposals/ruling-53/README.md).
- Every order cycle reuses `worker-1`, `tree-1`, `admin--worker-1` and `worker-1--operator`. Closes by CON-008 (a retired target) and closes or retirements made by the admin Agent (test 5) are not "this copy's own", so each cycle needs `--reopen` per name. The admin Agent also needs a way to name tombstones in `register.v1`.
- Fix: say which closes leave a tombstone (CON-008 or not), and add a tombstone override to the wire.

D3. A non-root `operator` cannot be applied (S9, test 8).
- The CLI sends every request as root (reference-model/crates/shell/src/main.rs:157, 229-345). `--as` exists only for `tools` (:47).
- With `operator: ops`, the SCT-010 and operator Conduits go to `ops`, so root has no Pool Conduit and set-up is refused (ACC-033 row 4): mistake 1 again.
- SEC-008 still requires root to open get-secret grants, so one apply needs two identities.
- Also, S9's sketch says "target"; it should say "consumer".

D4. Controls against a desired-state file (S5).
- If a control set `limit` to 30 while the file says 20, any edit to another config key leads to a make-before-break whose new config is the translation's 20, silently undoing the control. Carrying 30 forward makes the store differ from the translation.
- The file has no way to mark a value as initial only. The design handles only edits at the control's own pointer (critic gap 5).

D5. The project's own contracts and implementations need an authored manifest (S7, S8, real use).
- MAN-015 requires `address` where a project holds a contract or lists an implementation. S8's filter Requirement, local hook plugins and the owner's in-repo package development (SUB-009, archives from `file:` URLs) all do.
- The design treats `conduit.yaml` as tool-written, which ruling 53 #5 covered only for pins. The author still writes framed contracts and edits the manifest by hand (by design, but the "1 file edited" claim fails).

MINOR
m1. The design's own real-use sketch does not validate.
- `templates.agents.codex-worker.conduits.main: {from: tree-ro, target: main}` writes a bare name. `declared-conduit.v1` takes `target-reference.v1`, which requires `{kind, name}` (spec/schemas/instances.yaml:160; spec/core/references.yaml target-reference.v1), and TEMPLATE-BODY is "exactly today's" spec.
- The REF rule lists `conduits.*.target` without saying whether it applies inside templates. If it does, a project and a package read the same text differently, against INS-025's symmetry.

m2. An inline Pool cannot give its entries placement or runtime. POOL-009 allows that only through Pool inputs, and inline entries reject `inputs`. S3 and S4 need `templates.pools` plus a referenced entry. Expressible, not inline.

m3. Generated names narrow the name space.
- NAME--operator--t limits entry names to 51 characters, and NAME--LOCAL--t limits NAME+LOCAL to 58. The canonical form allows 63 for each.
- Entry names cannot contain `--`, which framed names may.
- Framed authors and Agents can squat `x--y` names; the plan must compare definitions before treating an object as applied.

m4. The REF rewrite applies even in fields that fix the kind. A project object named `m2` turns `placement: m2` (a node) into `{kind: Resource, name: m2}`, which is rejected. Skip the rewrite where the field's schema fixes the kind.

m5. A grant has no `from`, so it cannot use a package-exported Conduit template (S7's `read`) or one whose operations or filter arguments are inputs. It needs the `conduits:` section with explicit consumer, target and `as`, and loses the short path.

m6. E13 (rotation) and S9 repair have no file expression. Under SEC-005 the locator is unchanged, so no `linear-key--2` is needed, contrary to the sketch. The real need is a new run of `linear`, and with no restart verb that means replacement (TPL-012's set-up again leaves a running run as it is). A CLI verb is needed; none of the change classes covers it.

m7. ACC-024: Conduits consumed by root can never be closed. Removing or narrowing an `operator` member, or an SCT-010 Pool grant, describes a state the engine cannot reach (acknowledged as "Lost 6").

m8. S10 has no project-wide default for runtime or provider, so a swap edits every entry and every Agent needs `--replace` and a new sign-in. `fetch` for a Git package source lives in the tool-written manifest, with no authoring path except install arguments.

m9. Peer mode (ruling 51) is not expressible in either form. "Free once canonical has it" holds only if it arrives as Type `config` or a `remote` implementation. A new family, a new standard argument or a reference to another engine's object needs `sectioned.v1` and SCT changes.

NO GAP FOUND
- Secrets (S2): SEC-001 references, SEC-009 Resource consumers, and SEC-008 opens by root as the CLI runs. SEC-010 accepts a standalone get-secret Conduit if it is open before set-up.
- Placement and shared-host (S3); controls narrowing (S5, TPL-010); in-flight semantics (S6, CON-008, ACC-034 and ACC-039 unchanged).
- Packages exporting templates-only documents (S7); filter grants with per-grant arguments (S8).
- Grants to `access` with a bound, to `management`, to Agents and to Pools, and Pool-entry targets (POOL-014 forms are kept as written).
- Enrollment and sign-in (rulings 50 and 49) are requests; the expected account becomes a `config` key.
- The SCT-010 operations match pool-management.v1 exactly (spec/core/capabilities.yaml:153-170).

## Verdict cost (minor)
The cost claims mostly hold. This is the cheapest normative short form costed so far, at about half of designs C and D. Corrected, the delta is about 22-26 promise touches (the design says 19-21) and 40-55 new vectors (it says 30-40). Of the 367 existing vectors, 0 are invalidated, which I confirmed. No store needs migrating. Core lines are 0 for the translation, and 0-350 depending on how strict SCT-009 is. Code is about equal to the layered route. The real price over the layered route is about 22-26 more promise touches, 30-40 more vectors and 7-8 more concepts for an implementer, with nothing removed. Four small fixes are needed before a spec batch: two vocabulary conflicts, the examples harness, the reserved generated names, and SCT-009's scope.

I only read the repository and changed nothing in it. Scripts and outputs are in <scratch>/est/.

WHAT I RAN
- I reran the designer's scratch case with the existing release runner: reference-model/target/release/conduit-conformance against scratchpad/repo, case `registration-sectioned-s1`.
  - Result: accepted, no diagnostics, 24 objects made under the generated names (`s1--t`, `s1--operator`, `agent-a--project--t`, and so on).
  - Its only failure is the placeholder `records: []`; 28 records were made.
  - So today's loader and core accept the translated T2 documents, as the design says.
- I scanned every document file the runner hands to the loader:
  - 831 files and 3,464 documents, through load, submit-files, resolve-project, register-project and registration-orders fixtures, including snapshot recipes;
  - 340 documents inside the .tar.gz archives.
  - None is a map without `kind`, and none is an empty map. The rule that sends such a map to SCT-001 changes no recorded verdict among the 367 cases.

COST TABLE (design's claim, then my estimate)

| Item | Design claims | My estimate |
|---|---|---|
| New promises | SCT-001 to SCT-009, plus optional SCT-010 | Same: 9-10 |
| Amended promises | OBJ-017, OBJ-019, OBJ-020, OBJ-022, INS-001, SUB-005, SUB-010, SCH-018, MAN-011; observable edits to NAM-027, OBJ-012, OBJ-013 | Add OBJ-006 (a bare name in `consumer`/`target`/`to` is rejected today; SCT-007 accepts it), OBJ-024 ("a document's apiVersion says which version's names apply"), MAN-003 (it cites OBJ-017's wording), and probably SUB-004 (its diagnostic would name entries) |
| Delta total | 19-21 touches | About 22-26 touches (4.8-5.7% of 455; I confirmed the 455) |
| Five accepted additions | 11-14 touches | 13-18 touches. The synthesis costed enforcing tombstones at register as optional; ruling 53 accepted it. Keying tombstones by (kind, name) also contradicts LIF-035 ("its name can be given to another object") and the name rule of OBJ-002. This applies to both routes. |
| All in | About 30-35 touches | About 35-42, against about 13-18 for the layered route |
| Definitions | 6 new terms | 6 new terms, plus two collisions with existing rows. `entry` ("In the promises the word is used only for this", meaning a Pool's entry) and `grant` ("Not used for access", from round 5's vocabulary decisions) both clash. Either rename the key and the term, or the owner rules on vocabulary. |
| Schema | 200-300 lines | 250-400 lines. `unevaluatedProperties` is outside the profile (spec/schema-profile.md), and `template-spec.v1`, `pool-template-spec.v1`, `conduit-template-spec.v1` and `instance.v1` are closed with `additionalProperties: false`. So the entry and template-body schemas must restate each family's keys. A future template key is mirrored in two schemas, not reached through one `$ref`. |
| Exports | Not counted | About 8-10 more in spec/core/conduit.yaml and spec/core/README.md |
| Examples | 10-15 new | 10-15 new, plus 3 existing ones restructured (see finding 2) |
| Vectors invalidated | 0 of 367 | 0 of 367, confirmed |
| New vectors | 30-40 | 40-55. SCT-009's mapping alone needs cases for about 8 rejecting checks (SUB-005, OBJ-002, NAM-027, INS-002, CON-021, IFC-014, PKG-006, NAM-028). |
| Loader | About 660-1,070 lines, plus 400-700 of tests | 700-1,200, plus 500-800 of tests |
| Core | 0 lines | 0 for the translation. 0-350 for SCT-009 (finding 3), plus about 10-30 in archives.rs to reject entry sections in a package against the authored text. |
| Shell (diagnostic mapping and `render`) | 150-250 | 150-300 |
| Runner (`translate` kind, `equivalent` option) and case.schema.json | 190-380 | 200-400 |
| tools/check_schemas.py | 60-120 | 80-200. It must only validate sectioned examples by schema, and keep them in their own directory, or its cross-document rules (`kinds_in`, `contents`, NAM-027) misjudge them. |
| CLI work shared with the layered route (install, plan/apply, reach) | 1,800-3,100 | Same. The critic thinks the planner estimate is low. |
| Source code, all in | Not totalled | About 3,200-6,200 lines, against 3,500-5,500 for the layered route plus 300-600 core lines for the additions. About equal. |
| Store migration | None | None for the delta, since the wire and the store stay canonical. The additions' state is additive: `State` has `deny_unknown_fields`, but new optional fields with serde defaults keep old stores readable and existing state projections unchanged, with no store version bump. |
| Document migration | Framed documents stay legal | Confirmed. Converting an existing framed project to sectioned form renames its templates (`x--t`), so a plan sees them all as new. There is no adoption path; conversion is optional. |
| Reading cost | About 6 concepts, 4-5% | 455 promises become about 466-469. About 7-8 concepts added (sectioned document and sections, inline or referenced entry, `grants` beside declared `conduits` on the same entry, the document's operator, translation with generated names and the reserved `--`, bare names within the project, the one-Pool owner default, authored-pointer diagnostics). 0 removed. Templates and instances get two normative notations. |

CLAIMS THAT CHECK OUT
- Every caller of the core already hands it canonical JSON (reference-model/crates/core/src/packages.rs:16, `Project{documents: Vec<Value>}`).
- There is a single place to insert the translation, `load()` in reference-model/crates/loader/src/project.rs. All three paths reach it:
  - the shell, through `boundary::project` and `read_project`;
  - the runner, through `packages::load` and `read_submitted_files`;
  - archive admission, through core/src/archives.rs, which calls `read_submitted_files`.
  So packages get templates-only sectioned documents for free.
- `Specification::from_documents` bundles any new Schema contract, but needs one more validator field.
- The built-in project the core receives already includes every spec/schemas/*.yaml (conformance/src/packages.rs:147-175).
- Positions are kept in `value.rs` (`Node.position`).
- The registration-orders comparison is real (conformance/src/lib.rs:597-631).
- `pool-management.v1` has exactly the 7 operations SCT-010 writes out.

FINDINGS, MOST IMPORTANT FIRST
1. Vocabulary conflicts. spec/definitions.md reserves `entry` for a Pool's entry and says `grant` is not used for access. The design makes both normative terms, and makes `grants` a key. This needs a renaming or a vocabulary ruling before the spec batch, so it is a ruling-level cost.
2. The examples harness treats a map without `kind` as a manifest. That convention contradicts SCT-001:
   - reference-model/crates/loader/tests/examples.rs:159-165, `by_rule_role`;
   - tools/check_schemas.py:2519-2525.
   Three examples rely on it: spec/schemas/examples/invalid-by-rule/MAN-018-item-binds-a-variable-of-identity.yaml, SEC-024-secret-header-to-http.yaml and TPL-003-template-selects-an-operation-of-the-engine.yaml. Another 21 such maps in the examples are routed by their `-manifest-` file names and are unaffected. The design's statement that it found no kindless map under spec/schemas/examples is wrong. The fix is small: 3 files and about 20 lines across the two harnesses.
3. SCT-009 is stronger than "0 core lines" allows. It promises a JSON Pointer into the authored text at any later check, for example `/agents/agent-a/grants/project/operations/1`. The core cannot supply that today:
   - core/src/registration.rs:51 locates diagnostics only as `/objects/<name>`, which is enough to find the entry;
   - core/src/packages.rs:92 `diag()` gives no location at all, at 23 call sites (NAM-027, NAM-028, TPL-002, PKG-006, NAM-007, MAN-*);
   - core/src/schemas.rs:752 gives a pointer without naming the document.
   There are two options:
   - weaken SCT-009 to "the entry, and the field where the rejecting check gives one", which keeps 0 core lines;
   - or add 150-350 core lines that carry document pointers. This invalidates no vector, because conformance/README.md says maps are compared as projections that tolerate extra members.
4. Generated names can collide. A grant keyed `t` makes the Conduit `NAME--t`, which is also the name of the entry's inline template; NAM-027 rejects names shared across kinds. A grant keyed `operator` collides with the operator member under SUB-004. SCT-005 or SCT-008 should reserve these keys. That is one sentence, not a new promise.
5. The design says each future canonical key will reach the sectioned form through a `$ref`. Because the profile has no `unevaluatedProperties`, keys must be restated (see the schema row), so a small schema-level copy of each new key remains. It is still far smaller than the layered CLI's estimated 27 mirrored promises.
6. Tombstones keyed by (kind, name) block a name that LIF-035 frees, for every caller and every project, because names are global per kind. That needs LIF-035 and OBJ-002 amended. It also adds friction for the owner's real use, where workers' names are reused per order. This belongs to the five additions, not to the sectioned delta, but the keying choice is this design's.

COMPARISON WITH THE LAYERED ROUTE
- The layered route: about 11-14 promise touches (13-18 with tombstones enforced at register), a CLI of about 3,500-5,500 lines, and golden compiler cases that do not bind.
- This route: about 35-42 promise touches all in, about the same code, 40-55 binding vectors, and spec/ becomes the format reference.
- So the extra cost of making the short form normative is about 22-26 promise touches, 40-55 vectors, schema and examples, and two vocabulary decisions. The code is about the same.
- It is cheaper than the 25-30 promises the synthesis estimated for promoting the layer later, because the plan stays outside the spec.

I did not recount S1's line totals; that is outside an estimator's task. Every figure above is on paper except the scratch registration and the scans.