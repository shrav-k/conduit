Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Normative sugar: the project form in conduit.yaml, defined in spec/ by numbered translation rules (AUT) into today's canonical documents

## name
Normative sugar: the project form in conduit.yaml, defined in spec/ by numbered translation rules (AUT) into today's canonical documents

## thesis
Make the short form part of the language the way the Haskell report makes do-notation part of Haskell. spec/ gains an authoring chapter, spec/promises/authoring.yaml, whose numbered promises AUT-001 to AUT-018 translate a "project form" into today's canonical documents and manifest. The project form is written as new sections of the existing manifest, conduit.yaml.

What stays and what changes:
- The kernel stays. Today's object model and its 455 promises are unchanged, apart from the five additions ruling 53 accepted and about ten small amendments.
- Both forms are accepted. The long form stays the kernel and the wire for replace.v1 and inspect.v1. register.v1 also accepts the form inside its `manifest` value, so an Agent can manage with the same language (preservation test 5).

Four design choices carry the safety argument:
1. **Object names are a pure function of the author's keys.** For example, `shared`, `agent-a--project`, `shared--def`.
   - So the link between a line and a stored object is defined by the spec, not by a tool's state file.
   - This is what lets three safeguards survive replacement of either end: tombstones (keyed by stable names), expectedId, and long-form documents placed beside the form.
2. **The translation depends only on three things:** the text, its apiVersion, and the contracts its pins name.
   - It never depends on the caller, on stored state, or on the environment. The skeptics' "meaning depends on who runs apply" cannot arise.
3. **The derived defaults are translation rules, switched on by one written line, `operator: root`.**
   - What it derives: root's Conduit to each Pool, and a turn Conduit to each Agent.
   - These grant root nothing it lacks: root's right to open is already unbounded (ACC-022).
   - Any other operator writes its grants out explicitly.
4. **Change stays explicit requests.** The accepted plan/apply is tooling over a normative definition of when stored state matches the file (AUT-018).
   - Grants are changed by close, register and open with expectedId.
   - A Resource is replaced break-before-make: drain, retire, then register again under the same name.
   - There is no revise, no swap and no restart in place.

On paper, S1 costs 28 lines per task (17, 6, 0, 2, 1, 1, 1). The baseline is 64 and the layered package 27.

What normativity buys:
- thesis 2 holds for what authors write;
- the fresh author can learn the format from spec/ alone;
- a second implementation must agree with the first (A34).

What it costs, beyond the five accepted additions every route needs:
- about 28 promise touches;
- about 45-60 new vectors, and 0 changed vectors (expected; not yet run);
- about 600-1,200 more lines of code than the layered route.

It leaves the bar's margins exactly where the layered route leaves them.

## grammar
WHERE IT IS WRITTEN
- The project form lives in a project's manifest file: conduit.yaml, .yml or .json (MAN-001). It is also accepted as the `manifest` value of register.v1.
- A manifest that holds any of the six new sections is a project form.
- Every other .yaml/.yml/.json file in the directory is a long-form document, exactly as today (MAN-011, SUB-010).
- YAML is the documented form. JSON is allowed by SCH-013 but is never used in examples.

TOP LEVEL (closed keys; checked by the Schema contract `core#project.v1` before translation)
- Existing manifest keys, unchanged:
  - apiVersion: string, required (OBJ-017 value);
  - packages: {<address>: {version, digest, fetch?}}, written by `conduit install` (ruling 53 #5);
  - address?, version?, exports?, implementations?
- operator?: "root". Absent means nothing is derived. Any other value is rejected.
- pools?:      {<Key>: Entry<Pool>}
- resources?:  {<Key>: Entry<Resource>}
- agents?:     {<Key>: Entry<Agent>}
- interfaces?: {<Key>: Entry<Interface>}
- templates?:  {<Name>: Template}
- A package's manifest may hold `templates` and nothing else of these.

KEYS
- An entry <Key> follows OBJ-001: `[a-z](?:[a-z0-9-])*`, at most 63 characters. In addition:
  - it contains no "--" (reserved for translated names);
  - it is not a first-start name (system, local, management, access, root);
  - it is unique across all sections and against every name of the project's other documents (NAM-027).
- A grant key <GKey> follows OBJ-022, contains no "--", and is not "def".
- A key becomes the object's name, so the derived names must fit 63 characters (AUT-016).

ENTRY<F>: exactly one of three forms
- (a) Inline. These keys may appear:
  - type: Type address. Required for Agent and Resource. A Pool defaults to github.com/shrav-k/conduit/core#pool.v1 and an Interface to core#interface.v1.
  - config?: object, default {}. Copied as written; never rewritten.
  - implementation?: map (TPL-003).
  - controls?: map (TPL-010).
  - conduits?: {<GKey>: Declared}. Not on a Pool.
  - entries?: {<EKey>: PoolEntry}. Pool only.
  - description?: string.
  - the standard arguments, below.
  - reach?: Reach. Not on a Pool.
- (b) From a template. These keys may appear:
  - from: template reference, in any of the four INS-025 forms;
  - arguments?: object, default {};
  - description?;
  - the standard arguments;
  - reach?.
- (c) External: {external: true, reach?: Reach}, and no other key. It names an existing object of that family and that name, and makes no document. Allowed in agents, resources and interfaces.

Standard arguments:
- owner?: Resource only. The default is the form's only Pool entry; with none or several, it is required.
- placement?: Agent and Resource.
- runtime?: Agent and Resource.
- Values are written as the kernel writes them (OBJ-006). A bare name is the object of that name, and since names are keys, a key reaches its entry.

GRANTS
- Reach: {<GKey>: Grant}. A grant under entry e has e as its consumer, and translates to a standalone Conduit.
- Grant is one of two forms:
  - {target: Target, operations: [<address#export>...], capability?: <address#export>, requirements?: {<RName>: {requirement: <address#export>, arguments?: object}}}
  - {from: <Conduit template ref>, target: Target, arguments?: object}
- Rules on `operations`: fully qualified (rulings 25 and 53 #8); copied in order; never grouped; may be empty; required.
- `capability` is optional: it is inferred under AUT-013, and fails closed.
- Declared: Grant plus optional?: boolean. It sits in `conduits`, is a declared Conduit (opened at setup, TPL-012), and has the new instance as its consumer.
- PoolEntry: an inline definition (type, config, implementation, controls, description) or {from, arguments?}, plus optional?: boolean.
- Target is one of four:
  - a Key of the form, whose kind is supplied from its section;
  - management or access (Resource), root (Interface), or system (Pool);
  - {kind, name};
  - {kind: Pool, name, entry} (POOL-014).

TEMPLATES
- Template: {kind: Interface|Agent|Resource|Pool|Conduit, description?, ...the template `spec` keys of that kind (TPL-001, CON-016), with `inputs` exactly as today}.
- Its `conduits` and `entries` items also accept the Grant and PoolEntry forms above.
- It is the frame removed and nothing else. Explicit inputs (ruling 8, IN4) stay.

DEFAULTS, all stated in AUT promises
- config {}; arguments {}; requirements none; optional false.
- A Pool's or Interface's type is the core basic Type.
- A Resource's owner is the only Pool.
- capability: AUT-013.
- Root's Conduits: only with operator: root (AUT-014).
- Nothing else is defaulted. A Type's schema defaults fill config in the kernel under SCH-009, as today.

## meaning
NORMATIVE DEFINITION
- A project form means its translation: the canonical documents and the canonical manifest that AUT-006 to AUT-016 produce from it (AUT-004).
- Registering a project whose manifest is a form registers that translation together with the project's other documents, as one submission:
  - the set rules of SUB-001 to SUB-004 apply;
  - all or nothing (SUB-003);
  - it only registers (SUB-006).

AS-IF RULE (AUT-004), after Haskell's "it is not expected that it will use them directly"
- An engine may translate any way it likes. The objects, records and diagnostics must equal those of the translated documents.
- Equivalence property: submitting the rendered translation as long-form documents to a fresh engine gives the same objects (kinds, names, declared definitions, held pins; IDs apart) as submitting the form.
- A vector kind asserts this property.

PURITY (AUT-005)
- The translation is a function of three inputs: the form's text, its apiVersion (which fixes the core version), and the contracts of the packages that its `packages` pins at their digests.
- It depends on nothing else: not the caller, not the objects that exist, not the directory, the environment, the host or the time.
- config and arguments values are copied byte-for-byte as data, never interpolated or rewritten (IN5, OBJ-021, INS-018). Names are joined from keys by fixed rules; no author writes an expression.

TWO STAGES, both normative
- **Stage 1 is syntax.** It covers the frame, names, sections to documents, reach to Conduits, operator Pool Conduits, owner and target kinds. The loader can run it over its position-carrying Node trees.
- **Stage 2 needs the pinned contracts.** It covers two rules:
  - capability inference (AUT-013);
  - root's turn Conduits (AUT-014b).

  It runs where packages are resolved (the core).
- Each stage's output is fully explicit canonical text, so the stored object never depends on how the engine split the work.
- With stage 2 switched off, the translation is purely syntactic. The cost is that `capability` and root's turn grants are written out; S1 then costs 34 lines.

HOW THESIS 2 HOLDS FOR WHAT AUTHORS WRITE
1. Every construct an author writes has its meaning given by one numbered AUT promise plus the kernel promises its output meets. No CLI behaviour is part of the meaning.
2. Every derived object comes from a numbered rule and is written out in full in the stored canonical object: the Pool owner, the inferred capability, root's Pool and turn Conduits with every operation listed. Inspection therefore shows exactly what the language made. The author's form never stores less than the long form would.
3. `expand` vectors pin each rule as an input/expected pair (form in; canonical documents and the rule ID per document out, or diagnostics with authored locations out), as Dhall's A/B tests and SAM's translator pairs do. Equivalence vectors tie the two forms together. A second implementation (A34) must pass both.
4. Diagnostics speak in the author's text (AUT-017). Each one names:
   - the file, a JSON Pointer into the form, and line:col;
   - the AUT rule;
   - the kernel promise that rejected the translated document.

   Values at secret-reference or isSecret places are never echoed.

WHAT THE FORM IS NOT
- **It is not a second semantics.** Every kernel rule still speaks only of canonical documents, as Scheme's formal semantics covers only primitives.
- **It is not a state machine.** It defines what a file means when registered, plus a normative correspondence between objects and keys (AUT-018):
  - object X corresponds to entry K when X's kind and name are those that K's translation gives;
  - the state conforms to the form when four conditions hold:
    1. every translated instance exists and is not retired;
    2. each one's declared definition equals what its translated document would resolve to now;
    3. two exceptions to condition 2: values at pointers that a control of the object changes (S5), and the package pins that a Conduit holds for its capability (NAM-017);
    4. every translated Conduit is open.
- Placement that the form leaves out is not compared.
- Long-form documents of the project translate to themselves, so they are covered too.
- Plans and inspection tools state drift in these terms. Nothing acts on drift unasked (OBS-004, LIF-008).

THE WIRE
- register.v1's existing `manifest` input accepts a form, translated by the same rules.
- replace.v1 still takes a canonical template, contracts and manifest; a form's sections are rejected there.
- inspect.v1 still returns `declared` in canonical form, plus ruling 53's inventory.
- A store holds only canonical objects. No provenance is stored, which avoids INS-013's conflict and the forged-provenance objection (C D1). The link to the form is the name.

## spec_plan
NEW FILE spec/promises/authoring.yaml (layer: language), 18 promises
- AUT-001 **Form and place.** These cover:
  - the six sections, in the manifest or in register.v1's `manifest` value;
  - a package's manifest holds `templates` alone, and any other section is rejected at install (as MAN-011 rejects instance documents).
- AUT-002 **Schema.** These cover:
  - the form is validated against core#project.v1 before translation;
  - keys are closed (OBJ-019);
  - a rejection names a JSON Pointer into the form, plus line:col when read from a file, with keywordLocation (SUB-005).
- AUT-003 **Keys.** These cover:
  - the name grammar;
  - no "--";
  - no first-start name;
  - grant key not "def";
  - uniqueness across sections and with the project's documents.
- AUT-004 **Meaning is translation.** These cover:
  - registration of form plus documents is one submission;
  - the as-if rule;
  - the equivalence property.
- AUT-005 **Purity.** These cover:
  - inputs are text, apiVersion and pinned contracts only;
  - values are copied as data.
- AUT-006 **Frame.** These cover:
  - apiVersion on every translated document;
  - kind from the section;
  - name from the key;
  - `description` becomes metadata.description.
- AUT-007 **Manifest projection.** The canonical manifest is the form without its six sections. SUB-008's digest and register.v1's manifest rules apply to it, so editing a grant never changes a project's contract digest.
- AUT-008 **Entries.** These cover:
  - an inline entry becomes template <key>--def plus instance <key> from it, with the standard arguments;
  - a `from` entry becomes instance <key>;
  - Pool and Interface Types default to the core basic Type.
- AUT-009 **Owner.** A Resource without `owner` gets the only Pool entry. With none or several, it is rejected naming the entry.
- AUT-010 **Targets.** A bare key gets {kind: <section's family>, name}. A first-start name gets its kind. Anything else is written as OBJ-006/OBJ-013 require.
- AUT-011 **Reach.** These cover:
  - grant k of entry e becomes Conduit template e--k--def plus Conduit e--k, with consumer e, the target and `as: k`;
  - the `from` variant.
- AUT-012 **Grant forms in `conduits` and `entries`.** These cover:
  - templates e--k--def and p--entry--def;
  - nested instantiations with from, target, arguments and optional.
- AUT-013 **Operations and capability.** These cover:
  - operations are copied in order, never grouped, and may be empty;
  - an omitted capability is the one capability that the target entry's Type offers, at its pinned version, whose operations include every listed one;
  - none or several is rejected, naming the candidates;
  - a target outside the form requires `capability`.
- AUT-014 **operator: root.** It derives two kinds of Conduit:
  - (a) for each Pool entry, Conduit root--<pool>, as <pool>, on core#pool-management.v1, listing register.v1, register-replacement.v1, inspect.v1, set-up.v1, retire.v1, retire-with-resources.v1 and reconcile.v1 (the list is printed in the promise, per core version);
  - (b) for each Agent entry, Conduit root--<agent>, as <agent>, on the single capability of the Agent's Type that has operations marked startsModelWork, listing exactly those operations (stage 2). Zero or several such capabilities is rejected.
- AUT-015 **`templates` and `external`.** These cover:
  - a `templates` entry becomes a canonical template named by its key, with the frame removed only;
  - an external entry makes no document, its key reaches the existing object, and it is the consumer of its grants.
- AUT-016 **Names.** These cover:
  - the shapes <key>, <key>--def, <e>--<k>, <e>--<k>--def, <p>--<entry>--def, root--<x> and root--<x>--def;
  - a name over 63 characters is rejected at its key;
  - a clash under SUB-004, NAM-027 or OBJ-002 is rejected naming the authored key and the other document.
- AUT-017 **Diagnostics.** These cover:
  - the authored place, the AUT rule and the kernel promise;
  - SEC-003 redaction at secret places.
- AUT-018 **Correspondence and conformance.** As defined under meaning. Used by inspection tools and plans; nothing acts on it unasked.

AMENDED, about 10
- SUB-005: diagnostics locate into the form.
- SUB-008: hash the manifest projection, not "the manifest as handed over".
- SUB-010: a directory whose manifest is a form.
- MAN-003: a manifest may hold the six sections, and still no kind, metadata or spec.
- MAN-011: a package manifest holds no instance sections.
- OBJ-017 and OBJ-024: translated documents carry the form's apiVersion.
- OBJ-018: `description` given by the form.
- NAM-027: translated names count.
- register.v1's description and inputSchema in spec/core/operations.yaml.
- No other kernel promise changes. INS-001, INS-002, INS-013, CON-016 and OBJ-003 hold of the translated documents as written.

SCHEMAS AND CORE
- spec/schemas/project.yaml: `project.v1` and its parts, about 250-350 lines.
  - Inline definitions $ref the existing template-spec schemas per kind (minus `inputs`), so S2-S10 keys (requirements, controls, entries, placement, runtime, implementation) come for free.
- manifest.v1: the six keys are added as optional properties of $defs/manifest, which $ref project.v1. additionalProperties keeps its keyword location.
- Core operations: no new operation. register.v1 accepts a form in `manifest`.
- No new core template or Type.
- spec/definitions.md gains 7 terms: project form, entry, translation, translated name, operator (of a form), external entry, conformance. The form uses `reach`, never "grant", which definitions.md keeps out of access.
- Examples: +15-25.

THE FIVE ACCEPTED ADDITIONS (ruling 53 #3), each with its home
1. **`as`.** A standard argument of a standalone Conduit: OBJ-013, the last sentence of IFC-012, a precedence note in IFC-014. AUT-011 and AUT-014 always write it, so tool names (project.*, history.*, agent-a.turn.v1, s1.*) never change.
2. **expectedId on open.v1 and close.v1.** LIF-037 `when`, plus two input schemas. Every open and close a plan sends carries it.
3. **Inventory in inspect.v1.** LIF-013, the output schema, and one ACC-033 disclosure sentence: only what the caller may inspect, with no counts. AUT-018 conformance and T3 read it.
4. **Tombstones.** A new LIF-038, with cross-notes in ACC-006, OBJ-002 and LIF-009.
   - Closing a Conduit leaves a mark keyed by (consumer kind+name, local name).
   - Retiring an instance leaves a mark keyed by (kind, name).
   - Each mark records the ID, caller, invocation, idempotency key, time and cause.
   - Marks are kept apart from records, so they survive REC-004 purges.
   - open.v1, register.v1 and register-replacement.v1 are refused with conflict where a mark matches, unless the request names the marked ID in a new input `supersedes` (a precondition, like expectedId).
   - Because translated names are stable, the key survives replacement of either end. This closes the critic's gap 2 structurally.
5. **Descriptions.** IFC-033 names the target and gives its Type's description and the target's metadata.description, which the form's `description:` sets. OBJ-018's "no meaning" sentence is amended.

These are about 11-14 touches, needed by every route. They are counted apart.

DELETED
- Nothing.
  - Declared Conduits stay: they are the form's `conduits`.
  - register-replacement.v1 stays: it serves entries and declared Conduits.
  - replace.v1 stays: plans use it on <key>--def templates.

RULINGS THE OWNER MUST CONFIRM
- 8 ("one shape everywhere", IN4) and 18 (V2): the kernel keeps one shape and a template for every instance. The form is a second surface defined by translation.
- 35: apiVersion is read per translated document.
- 31 and 37: the frame as drafted.
- Names joined from keys: confirm this is not IN5 interpolation.
- 53 #6: answered by AUT-009 and AUT-014.
- 47: reserved, unread.

## s1_rewrite
HOW THIS IS COUNTED
Counted per the measurement plan:
- Non-blank lines added, changed or removed in the author's files.
- `---` and separators count. Comments are excluded.
- Each command line that carries a value the author chose counts, apart.
- Lines marked TOOL are written by `conduit install` and are not counted.

Assumptions:
- Harness, not counted, as in the synthesis: `conduit init --store <lab>/store` once, which records the store beside the project; `conduit serve`; every check (turns, `tools --as`).
- The Agent packages are 0.1.2, with reach flags false by default in their Type contracts (ruling 53 #7).
- The five accepted additions exist.
- `conduit apply` takes no value. How it is approved without a terminal is decision 2, which is still open; see the counts.

=== T1: 16 file lines + 1 command line = 17 ===

Commands:
```
$ conduit install <lab>/packages/filesystem-0.1.0.tgz <lab>/packages/agent-claude-code-0.1.2.tgz <lab>/packages/agent-codex-0.1.2.tgz    [1]
$ conduit apply                                                                                                                         [0]
```

conduit.yaml:
```yaml
apiVersion: github.com/shrav-k/conduit/v0
packages:                                                                          # TOOL
  example.org/s1/filesystem: {version: 0.1.0, digest: sha256:200de7c103aa5602...}  # TOOL
  example.org/conduit/agent-claude-code: {version: 0.1.2, digest: sha256:...}      # TOOL
  example.org/conduit/agent-codex: {version: 0.1.2, digest: sha256:...}            # TOOL
operator: root
pools: {s1: {}}
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      project: {target: shared, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    reach:
      project: {target: shared, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
```
The apiVersion line is counted, although install could write it.

**Translation, not authored.** `conduit render` shows 18 documents and the manifest, each tagged with its rule:
- AUT-008: Pool template s1--def {type: github.com/shrav-k/conduit/core#pool.v1} and Pool s1.
- AUT-008 and AUT-009: Resource template shared--def {type, config: {root: <lab>/shared}} and Resource shared {from: shared--def, owner: s1}.
- AUT-008: agent-a--def {type: ...#claude-code.v1}, agent-a, agent-b--def and agent-b. Their config is filled by the Type's defaults (SCH-009): accountServices false, peerMessaging false, model ''.
- AUT-011 and AUT-013: agent-a--project--def {capability: example.org/s1/filesystem#directory.v1, which was inferred; operations [file.read.v1, directory.list.v1]} and agent-a--project {consumer: {kind: Agent, name: agent-a}, target: {kind: Resource, name: shared}, as: project}. The same for agent-b, with file.write.v1 added.
- AUT-014a: root--s1--def {capability: core#pool-management.v1, operations: register.v1, register-replacement.v1, inspect.v1, set-up.v1, retire.v1, retire-with-resources.v1, reconcile.v1} and root--s1 {consumer: {kind: Interface, name: root}, target: {kind: Pool, name: s1}, as: s1}.
- AUT-014b: root--agent-a--def {capability: example.org/conduit/agent-claude-code#conversation.v1, operations: [example.org/conduit/agent-claude-code#turn.v1]} and root--agent-a {as: agent-a}. The same for agent-b with the codex package.
- AUT-007: the manifest is apiVersion plus packages.

**Plan, not authored.** As root:
1. register: 1 submission of 18 documents;
2. set-up s1 (through management);
3. open root--s1 (expectedId);
4. set-up shared (through root--s1, ACC-033 row 4);
5. open agent-a--project and agent-b--project (expectedId);
6. set-up agent-a and agent-b;
7. open root--agent-a and root--agent-b.

That is 13 engine requests, counting the 3 installs.

Tools:
- root: s1.*, agent-a.turn.v1, agent-b.turn.v1;
- agent-a: project.file.read.v1, project.directory.list.v1;
- agent-b: the same, plus project.file.write.v1.

Mistakes 1 and 2 of both earlier authors cannot occur: root--s1 lists set-up and register.

=== T2: 5 file lines + 1 command line = 6 ===

```
$ conduit install <lab>/packages/history-0.1.0.tgz    [1]
```
TOOL adds `  example.org/s1/history: {version: 0.1.0, digest: sha256:e946746509490be4...}`.

Added to conduit.yaml:
```yaml
  history:                                                   # under resources:
    type: example.org/s1/history#file-history.v1
    config: {root: <lab>/history}
      history: {target: history, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # under agent-a reach:
      history: {target: history, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # under agent-b reach:
```
```
$ conduit apply    [0]
```

**Plan:**
1. register history--def and history (owner s1 by AUT-009; ACC-033 row 1 through management and root--s1, both listing register.v1);
2. register agent-a--history(--def) and agent-b--history(--def). The capability is inferred from file-history.v1's offers as example.org/s1/filesystem#directory.v1;
3. set-up history;
4. open the two Conduits.

That is 5 engine requests. The history tools' descriptions carry file-history.v1's posting convention (addition 5).

=== T3: 0 ===

```
$ conduit reach    [0; a view, no value]
```

The answer has three parts:
- **What each Agent reaches through the engine** is the form's own `reach` lines:
  - agent-a: project read and list; history read, list and write;
  - agent-b: project read, list and write; history the same.
- **Nothing else.** inspect.v1's inventory confirms that no other Conduit has either Agent as consumer. Root's derived turn Conduits are listed apart.
- **What each Agent reaches around the engine** comes from the Agent Types' descriptions (provider work, stated accurately for 4-06 and 3-07).

Inspection effort: one file and 1 command.

=== T4: 2 ===

Edited lines:
```yaml
      project: {target: shared, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # agent-a
      project: {target: shared, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}                                          # agent-b
```
```
$ conduit apply    [0]
```

**Plan, narrowing first.** For agent-b:
1. close agent-b--project (expectedId), which leaves tombstone (agent-b, project);
2. replace.v1 agent-b--project--def (revision);
3. register agent-b--project (as project);
4. open it (expectedId = the new ID, supersedes = the closed ID).

Then the same for agent-a. That is 8 requests. Tool names are unchanged, and at no moment can both Agents write.

=== T5: 0 file lines + 1 command line = 1 ===

```
$ conduit install <lab>/packages/filesystem-0.2.0.tgz    [1]
```
TOOL rewrites the filesystem line to `{version: 0.2.0, digest: sha256:cbb831f07d2b251c...}`.
```
$ conduit apply    [0]
```

**Plan.** `shared` does not conform: shared--def holds filesystem 0.1.0 (AUT-018). The tool first checks that the granted operations' contracts and the implementation's tools map are equal between 0.1.0 and 0.2.0, so append_file and copy_file reach no Agent. Then it replaces break-before-make:
1. close agent-a--project and agent-b--project (expectedId);
2. wait until no invocation through them is in flight;
3. retire shared (expectedId, through root--s1). Once its shutdown is confirmed it is removed and its name freed (LIF-035);
4. replace.v1 shared--def with the 0.2.0 pins;
5. register shared (supersedes its tombstone), together with agent-a--project and agent-b--project, whose templates conform because a Conduit's capability pin is not compared (AUT-018);
6. set-up shared;
7. open both Conduits (expectedId, supersedes).

That is 1 install and about 9 requests. history, its Conduits and both Agents conform and are not touched. Both Agents are without project access from step 1 to step 7.

=== T6: 1 ===

Edited:
```yaml
    config: {root: <lab>/shared-new}
```
```
$ conduit apply    [0]
```
Plan: the same replacement as T5, with the new root. Tool names are unchanged. With addition 5 the descriptions change and IFC-038 tells each Agent its list changed.

=== T7: 1 (one line removed, then restored; counted once, as the baseline counted its T7) ===

1. Remove agent-a's `project:` line, then run `conduit apply`:
   - close agent-a--project (expectedId), which leaves the engine's tombstone (Agent agent-a, project);
   - unregister agent-a--project--def.
2. Check, not counted: B reads the project and posts to history.
3. Put the same line back, then run `conduit apply`:
   - the tombstone was made by this working copy's own apply, as recorded in .conduit/applied (TOOL);
   - so the tool registers agent-a--project--def and agent-a--project and opens it with supersedes.

A tombstone that anyone else left (a `conduit close`, an Agent, a responder) is a conflict instead. It needs `--reopen agent-a/project`, which is one more command line.

=== FINAL AUTHORED FILE ===
21 lines, plus 5 TOOL lines.

## s1_counts
PRIMARY READING (the measurement plan's): file lines plus command lines with chosen values

| Task | This design (file + command) | Baseline A70 | Layered package | Coordinator | Fresh author | vs A70 | vs layered |
|---|---|---|---|---|---|---|---|
| T1 | 16 + 1 = 17 | 33 | 16 | 154 | 263 | less | +1 (the `operator: root` line) |
| T2 | 5 + 1 = 6 | 16 | 6 | 73 | 67 | less | same |
| T3 | 0 | 0 | 0 | 0 | 0 | same | same |
| T4 | 2 | 8 | 2 | 2 | 32 | less | same |
| T5 | 0 + 1 = 1 | 3 (better baseline: 2) | 1 | 62 | 112 | less | same |
| T6 | 1 | 2 | 1 | 29 | 44 | less | same |
| T7 | 1 | 2 | 1 | 12 | 128 | less | same |
| All | 28 | 64 | 27 | 332 | 646 | less on 6, same on 1, much more on none | +1 |

SENSITIVITY (same rewrite)

| Reading | T1 | T2 | T3 | T4 | T5 | T6 | T7 | Total | Bar |
|---|---|---|---|---|---|---|---|---|---|
| Root derivation on by default, with no `operator` line | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 | pass, better on 6 |
| Stage 2 off: `capability` written (width only); root's turns written in an `interfaces: {root: {external: true, reach: ...}}` block of 6 lines | 23 | 6 | 0 | 2 | 1 | 1 | 1 | 34 | pass, better on 6 |
| No derived defaults: no `operator`; root's 3 Conduits written in that block | 23 | 6 | 0 | 2 | 1 | 1 | 1 | 34 | pass, better on 6 (one 7-operation line is about 400 characters wide) |
| Block-style grants, 3 lines each | 21 | 10 | 0 | 2 | 1 | 1 | 3 | 38 | pass on 5; T7 one more |
| T7 restore counted as a second line | 17 | 6 | 0 | 2 | 1 | 1 | 2 | 29 | pass on 5 |
| Strict: every command a line (install, apply, reach) | 18 | 7 | 1 | 3 | 2 | 2 | 3 | 36 | against A70, better on T1, T2, T4 and T5 (4 of 7), a narrow pass; against the better T5 baseline (2), T5 ties: 3 of 7, FAIL |
| Approval by a copied plan ID, +1 per apply (decision 2 option a) | 18 | 7 | 0 | 3 | 2 | 2 | 3 | 35 | FAILS against the better T5 baseline, as the critic found for the layered package |
| JSON with one key per line | ~41 | ~23 | 0 | ~4 | 1 | 1 | ~7 | ~77 | FAIL: more on T1, T2 and T7. YAML must be the documented form |

OTHER MEASURES (paper)
- Grants made by hand: T1 2, T2 2, T7 1. Root's 3 derived Conduits (root--s1, root--agent-a, root--agent-b) are listed apart and shown by render and inspect.
- Lines naming a contract address: 8 in the final file (4 type lines and 4 grant lines).
- Provider-specific lines: 0.
- Tool names changed by any task: 0.
- Engine requests sent by the tool: about 50, against the coordinator's 35.
- Windows without access:
  - T4: per Agent, 4 requests;
  - T5 and T6: both Agents at once, from the closes to the opens, about 8 requests including one server stop and one start. That is longer than the layered package's make-before-break T5, and the coordinator's T5 had none. Seconds only once 4-15 (one MCP session, timeouts) is fixed; to be measured in the rerun.
- Concepts:
  - from the form: sections, entry/key, type, config, reach, target, operations, operator;
  - from the spec: Pool, Conduit, root Interface, capability, package and pin, tombstone/expectedId as plans show them;
  - commands: install, apply, reach.

  About 15-18 in all, now learned from spec/ alone (ruling 48), which answers critic gap 3.
- Duplicated values: the operation lists (as in the baseline) and `shared`/`history` as targets.
- Margins on T5-T7 are 1 line each. Normativity changes no line count: lines are a property of the surface, not of where it is defined.
- All counts are on paper. Ruling 45 accepts only a rerun.

## other_scenarios
GENERAL ESCAPE RULE
Long-form documents stay legal in the same project. The form may refer to them by reference forms, and they may refer to the form's objects by translated names, which never change. The plan covers them under AUT-018. This is Compose's mixed short and long syntax and SAM's plain resources, without ALFA's round-trip requirement.

S2: one Linear login shared by two Agents with different restrictions
```yaml
operator: root
pools: {tools: {}, vault: {}}
resources:
  op-token: {owner: vault, type: example.org/keychain#keychain-item.v1, config: {service: conduit-op, account: test-vault}}
  linear-key:
    owner: vault
    type: example.org/onepassword#op-item.v1
    config: {ref: op://test-vault/linear/api-key, token: {kind: Resource, name: op-token}}
    reach:
      store: {target: op-token, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}
  linear:
    owner: tools
    type: example.org/linear#linear.v1
    config: {token: {kind: Resource, name: linear-key}}
    reach:
      key: {target: linear-key, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}
  scoped:
    owner: tools
    type: example.org/linear-adapter#scoped-linear.v1
    conduits:
      upstream: {target: linear, operations: [example.org/linear#issue.list.v1, example.org/linear#issue.get.v1, example.org/linear#issue.create.v1, example.org/linear#issue.update.v1, example.org/linear#comment.create.v1]}
agents:
  reader:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      linear: {target: linear, operations: [example.org/linear#issue.list.v1, example.org/linear#issue.get.v1, example.org/linear#issue.search.v1]}
  writer:
    type: example.org/conduit/agent-codex#codex.v1
    reach:
      linear: {target: scoped, operations: [...six operations...], requirements: {team: {requirement: example.org/linear-adapter#team.v1, arguments: {team: ENG}}}}
      share: {target: access, operations: [github.com/shrav-k/conduit/core#open.v1], requirements: {limit: {requirement: github.com/shrav-k/conduit/core#bound.v1, arguments: {targets: [{kind: Resource, name: linear}], operations: [example.org/linear#issue.list.v1]}}}}
```
- Secret references stay in their canonical form inside config. Data is never rewritten, and SEC-001 forbids bare names there.
- The get-secret.v1 Conduits are opened by the plan as root (SEC-008); a non-root apply is refused, which fails closed.
- E13 rotation is not an edit. It is `conduit apply --replace linear`, an explicit identity change, as with Terraform's statements. The plan lists the cascade:
  - linear's Conduits are re-pointed under the same names;
  - scoped's declared `upstream` holds linear's old ID (OBJ-010), so scoped is replaced too and the writer's Conduit is re-pointed.
- Not expressible: the expected account (ruling 49 is not yet in the kernel; once it is a config key or a standard argument, the form carries it by the same copy rule). Filter contracts (D05) are long-form Requirement contracts in the adapter's package.

S3: two machines
```yaml
resources:
  tailnet: {owner: system, type: example.org/tailscale#tailnet.v1, config: {tailnet: example.ts.net, key: {kind: Resource, name: ts-key}}}
  m2-docker: {type: example.org/docker#docker-context.v1, config: {host: ssh://m2.example.ts.net}}
  shared: {type: example.org/s1/filesystem#local-directory.v1, config: {root: /home/me/shared}, placement: m2, runtime: m2-docker}
agents:
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    placement: m2
    runtime: m2-docker
    reach:
      project: {target: shared, operations: [...], requirements: {host: {requirement: github.com/shrav-k/conduit/core#shared-host.v1}}}
```
- `placement: m2` names an engine-made node by its bare name, as the kernel allows (the field fixes the kind).
- nodeSelector stays on Types and items, in packages.
- Partition and loss are behaviour, not documents.
- A replacement keeps the predecessor's node when the form names none: the plan sends it, and AUT-018 does not compare placement that the form leaves out.
- Not expressible: peer mode (an Interface for another engine's actor) and trust domains (ruling 51); the kernel has neither. Enrollment is a request (ruling 50), not a document.

S5: a control under contention
```yaml
interfaces:
  ops:
    reach:
      tune: {target: catalog, operations: [example.org/search#set-read-limit.v1]}
resources:
  catalog:
    type: example.org/search#index.v1
    config: {limit: 20}
    controls: {example.org/search#set-read-limit.v1: <narrowing to maximum 50, in TPL-010's shape>}
agents:
  agent-a: {type: ..., reach: {tune: {target: catalog, operations: [example.org/search#set-read-limit.v1, example.org/search#query.v1]}}}
```
- AUT-018 leaves values at control pointers out of conformance. A live control is not drift, which answers critic gap 5.
- Editing `limit` in the form makes the plan send the control with the revision as its precondition, not a replacement.

S6: revocation and retirement in flight
- Deleting a `reach` line becomes close.v1 with expectedId. The engine keeps a tombstone keyed (consumer, local name) and the call in flight runs to its end (ACC-034).
- Deleting an entry is refused without `--prune KEY`. With it, the instance is retired with expectedId; CON-008 closes every Conduit to it, including those outside the form, which the plan lists first.
- Tombstones stop any later apply from any working copy, and any Agent's open, unless `supersedes` is sent: `--reopen` or `--recreate`.

S7: packaging an existing MCP server and a code plugin
- The package's conduit.yaml keeps address, version, exports and implementations (plain `package` item, isSecret bindings, supports) unchanged, and may add:
```yaml
templates:
  linear-service: {kind: Resource, type: '#linear.v1', inputs: {token: {schema: {$ref: github.com/shrav-k/conduit/core#secret-reference.v1}, to: [/config/token]}}}
```
- Contracts (Type, Capability, Operation, Requirement) stay long form in contracts.yaml.
- Honest: a package writer meets two syntaxes, with one meaning.

S9: first start and repair
- First start (FST) and engine configuration (trust domain, hostname, runner list, store) are outside documents.
- Providers are entries with `owner: system`.
- Repair is explicit requests:
  - `conduit apply --replace KEY`;
  - set-up again;
  - `conduit reconcile`;
  - declare-lost;
  - unregister.
- Three states are kept apart: the form (its translation), what is registered (inspect and inventory), and what is observed (status). AUT-018 compares only the first two.
- A retired or lost instance is reported. Registering it again needs `--recreate`, because register-time tombstones enforce it. Nothing is ever recreated unasked.

S10: every provider swapped
- Changing `runtime:` on entries, a store entry's type or config, or adding a git source entry (`owner: system`, with `fetch` in the pins) makes each affected entry a replacement, because runtime, locator and placement are fixed (PRV-006, SEC-005, PLC-009).
- The cascade is the plan's route, not part of the form's meaning. The form means only the target state.
  - Example: a new store gives a new linear-key, so linear is replaced because its resolved reference is fixed, and its Conduits are re-pointed.
- Replacing an Agent shows the pending sign-ins (rulings 49 and 50).
- No project-wide `runtime` default exists. It would be one more AUT rule; until then `runtime` is written per entry.

OWNER'S REAL USE: 10 workers
- Shared part, about 35 lines:
  - operator, Pool, main and orders Resources;
  - an admin Agent with reach to main and orders;
  - a verifier;
  - two worker templates in `templates:` (one each for Codex and Claude Code, since a Type is never an input under IN2), each with declared Conduits to main and orders and an input filling the `tree` Conduit's target.
- Per worker i, about 5 lines:
```yaml
  tree-3: {type: example.org/dev/git#worktree.v1, config: {repo: /lab/conduit, branch: wo/103}}       # resources
  worker-3: {from: codex-worker, arguments: {tree: {kind: Resource, name: tree-3}}}                  # agents
      worker-3: {target: worker-3, operations: [example.org/conduit/agent-codex#turn.v1]}             # admin reach
      tree-3: {target: tree-3, operations: [example.org/dev/git#file.read.v1, example.org/dev/git#directory.list.v1]}   # admin reach
      tree-3: {target: tree-3, operations: [example.org/dev/git#file.read.v1]}                         # verifier reach
```
- Total: about 85 lines, against about 440-520 today and a floor of about 25.
- What it still cannot do:
  - no template makes an Agent and its own tree together, because the kernel lacks composites;
  - no for_each or replicas (IN5);
  - the admin's grants cannot sit in the worker template (CON-002).
- An Agent-package upgrade replaces all 10 workers (INS-020) with 10 sign-ins, which the plan lists.
- Agent-driven worker churn needs `management` trust or a broker (D04). An admin Agent can submit a form through register.v1, and the engine enforces tombstones and expectedId for it.

## change_model
WHAT IS NORMATIVE
- the translation;
- the correspondence by name and the conformance definition (AUT-018);
- every request and its promises;
- the five accepted additions: `as`, expectedId on open and close, the inventory, tombstones enforced at open and register with `supersedes`, and descriptions.

WHAT IS TOOLING, for now (ruling 53 #4: "explicit plan and apply, composed of today's requests, never automatic, never pruning or reopening without a flag")
- **`conduit plan` is pure.** It takes:
  - the form's translation, with the project's long-form documents;
  - the inventory and inspect.v1 of each translated name;
  - the tombstones;
  - the committed, tool-written .conduit/applied record of this working copy's own closes.

  It prints the requests that would make the state conform.
- **`conduit apply` runs them only on an explicit request.** There is no reconciler and nothing in the background. It runs in one MCP session and stops at the first part that does not succeed. There is no rollback (LIF-027). It can resume, with deterministic idempotency keys derived from (object, action, expected ID).
- **An Agent can do the same** from the same normative inputs (preservation test 5). The engine, not the tool, enforces expectedId and tombstones.

CHANGE CLASSES, each made of today's requests
- **Grant added:** register (template and Conduit), then open.
- **Grant removed:** close (expectedId). The tombstone is kept, and the --def template is unregistered.
- **Grant's operations, target or requirements changed:** close, replace.v1 of e--k--def, register, then open with supersedes. Narrowings run before widenings, and closes are never blocked by unrelated failures.
- **Entry added:** register, set-up, then its grants.
- **Entry's definition, or the pin of the package exporting its Type, changed:** replacement, break-before-make under the same name:
  1. close every Conduit to and from it, including long-form ones and ones outside the form, all listed;
  2. drain: wait until nothing is in flight;
  3. retire (expectedId) and wait for the shutdown to be confirmed (LIF-035 frees the name);
  4. replace.v1 the --def template;
  5. register (supersedes);
  6. set-up;
  7. register and open the Conduits again.

  Further rules for replacement:
  - The cascade through resolved references is listed in the plan: declared Conduits and config references that hold the old ID force their holders to be replaced too.
  - At a version change the plan diffs the granted operations' contracts and the implementation's tools map, and any widening needs a flag.
  - It refuses while an invocation through those Conduits is unknown.
  - Replacing an Agent warns that its home and login are lost (ruling 49).
- **A value at a control's pointer changed:** a control request with the revision as its precondition.
- **Explicit identity change with unchanged text:** `--replace KEY`, for a secret rotation or a new run.
- **Entry removed:** refused without `--prune KEY`.
- **Tombstone not made by this working copy:** a conflict that needs `--reopen` or `--recreate KEY`, which sends `supersedes`.
- **Root's derived Conduits** are add-only (ACC-024). Removing `operator` reports them as kept until their targets retire.
- **Drift** (anything that does not conform, and Conduits not from the form) is shown and never repaired unasked.

WHY BREAK-BEFORE-MAKE
- Names stay pure functions of keys, so the translation, tombstones and long-form references stay true.
- In T6 the Agents never see two directories (E skeptic #7).
- In T5 two server versions never run on one root.
- The price: a longer window without access at T5 and T6, and a name held while a shutdown is unknown, which fails closed and is resolved by reconcile.
- Option for the owner, not recommended: a tool-written `generation:` field per entry gives make-before-break, at the cost of names like shared--2 and a tool-owned value inside the author's entry. About 2 more promises.

APPROVAL WITHOUT A TERMINAL
- Decision 2 stays open. This design is neutral among its options:
  - (a) a saved plan applied by its ID, counted as a line;
  - (b) one-step commands;
  - (c) apply with a printed and recorded plan.
- The counts give each reading.

NOT INCLUDED
- revise.v1, swap.v1, restart in place, an engine-side apply, and equal resubmission (A's rule).
- Optional later: make the plan normative as (form, inventory, tombstones) → requests, if a second tool must agree. About 25-30 more promises and about 50 plan vectors.

## cost
SPEC PROMISES
- For the form: 18 new (AUT-001 to AUT-018) and about 10 amended (SUB-005, SUB-008, SUB-010, MAN-003, MAN-011, OBJ-017, OBJ-024, OBJ-018, NAM-027, register.v1). That is about 28 touches, roughly 6% of 455.
- The five accepted additions: 11-14 touches, needed by every route.
- Total: about 39-42.
- Comparison:
  - layered: 11-14;
  - this design: about 28 more than layered;
  - C and D: 50-80, which bundled engine apply and revise.
- Removed: none.
- Also:
  - definitions.md: +7 terms;
  - schemas/project.yaml: about 250-350 lines;
  - manifest.v1: six optional properties;
  - examples: +15-25.

VECTORS
- **Existing: 0 of 367 changed, expected.**
  - No case or example uses the new manifest keys (grep checked).
  - The keys are added to $defs/manifest's properties, so additionalProperties keeps its keyword location.
  - The canonical form, document.v1 and envelope.v1 are untouched.
  - To be confirmed by running cargo test. About 15 manifest-rejection and schema-attribution cases are the ones to watch.
- **New for the form: about 45-60.**

  | Group | Cases |
  |---|---|
  | Stage-1 rules, one valid and one invalid each | 30-40 |
  | Stage-2 rules, with package fixtures | 6-8 |
  | Equivalence: S1 T1 and T2 forms register the same transition as their rendering, after the registration-orders comparison | 2-3 |
  | Diagnostics mapped to the authored line, with secrets redacted | 4-6 |
  | Conformance: control pointers, capability pins, placement left out, long-form documents | 4-6 |
  | Mixing and package `templates` | 3-4 |

- **The additions: 10-17**, in both routes.

REFERENCE MODEL, BY CRATE (projections; nothing built)

| Crate | Today | Added | Growth |
|---|---|---|---|
| loader | 2,879 | stage-1 expander over Node with positions 700-1,100; manifest role, project.v1 and rules 80-150; provenance map for diagnostics 150-250 | +930-1,500, +32-52% |
| core | 10,465 | stage 2 in registration.rs 150-250 (reusing offered() at line 1241 and capability_includes); register.v1 calls the loader's pure expander 30-60; SUB-008 projection 10-20; diagnostics mapped back 50-100 | +240-430, +2-4%. The additions add 300-600 in both routes |
| shell | 6,986 | render 50-100; install editing the `packages` block of conduit.yaml line by line, with no mature Rust YAML editor 200-400; plan, apply and reach 1,500-2,500 | +1,750-3,000 |
| conformance runner | 2,137 | `expand` step 100-200; `equivalent` option 50-100; case.schema.json 40-80 | |
| tools/check_schemas.py | 2,588 | stage-1 expansion so example verdicts match the loader 300-500 (only 50-100 for schema checks alone) | |

- The shell's plan is about the same as layered, somewhat simpler: correspondence by name, with no successor names and no annotation source maps.

ROUGH SIZE AND SCHEDULE
- Compared with the layered route, the compiler code is the same, but moved from the shell into the loader and core.
- Added: about 600-1,200 more source lines (the Python and the runner), about 28 promises and about 50 vectors.
- No core rebuild: increments 1-3 stand.
- About 6 work orders:
  1. spec batch for AUT;
  2. spec batch for the additions;
  3. loader and core expander;
  4. runner and vectors;
  5. shell install, render, plan and apply;
  6. provider versions.
- Prerequisites:
  - decision 6 must be settled first, since the defaults become language;
  - rulings 8, 18 and 35 must be confirmed as binding the kernel.

## what_is_lost
1. **The language does not get smaller for implementers. It gets about 6% bigger.**
   - All 455 kernel promises stay, and so does the dead weight the layer never uses for S1.
   - A second grammar, its expander and its Python twin must be kept in step with the kernel.
   - Every later kernel key needs a decision on its form surface and 1-3 vectors.
   - Two normative syntaxes exist, with one meaning. Package writers meet both, because contracts stay long form.
2. **No line is saved against the layered route.** It is 28 against 27, or the same with root derivation on by default.
   - The bar's margins on T5-T7 stay at 1 line.
   - It still fails under the strict reading against the better T5 baseline, under approval by a copied plan ID, and in JSON.
   - Normativity buys meaning, learnability and agreement, not fewer lines.
3. **Names are fixed functions of keys.**
   - "--" is reserved.
   - Keys can never be renamed in place: a rename is a remove plus an add, needing flags.
   - Generated names (shared--def, agent-a--project, root--s1) appear in records and inspection.
   - Each one-off instance carries a --def template in the store.
   - Joining keys into names may need the owner's reading of IN5.
4. **Break-before-make replacement** gives T5 and T6 a window without access for both Agents at once.
   - It is longer than the layered package's make-before-break T5. The coordinator's T5 had none.
   - A retirement whose shutdown is unknown holds the name until it is reconciled, which fails closed.
   - Declared Conduits and config references cannot follow a replaced target, so cascades replace their holders too.
5. **G9 (Conduits open before the run) for `reach` grants rests on the plan's order, not on setup.** Declared `conduits` keep it.
6. **Stage 2 makes the translation depend on pinned contracts.**
   - The loader alone cannot translate.
   - Diagnostics from stage 2 need positions passed into the core.
   - Turning stage 2 off costs 6 lines in T1.
7. **The defaults are narrow by design.**
   - Only root is ever a derived operator.
   - Root's derived Conduits can never be closed by a request (ACC-024).
   - A non-root operator, peer mode or a delegate writes every grant out.
   - There is no project-wide runtime or placement default.
8. **Rulings need the owner's confirmation:**
   - 8 ("one shape everywhere", explicit inputs) and 18 (V2), as binding the kernel, not the authored surface;
   - 35, read as apiVersion per translated document;
   - 31 and 37 (frame);
   - 53 #6, which this design answers with AUT-009 and AUT-014.
9. **Not covered, and escaping to long form or to future kernel work:**
   - contracts;
   - inputs on one-off entries (use `templates`);
   - peer mode, trust domains, enrollment, pending actions and the expected account (rulings 49-51, unwritten in the kernel);
   - composites of an Agent and its own Resource;
   - replicas or for_each;
   - a normative plan.
10. **The state of the evidence.**
    - Every count and cost is a projection on paper. Nothing was built or run, and ruling 45 accepts only the rerun.
    - I did not read ruling 47.
    - I did not check that adding the keys to manifest.v1 leaves every loader diagnostic unchanged.

## Verdict safety (major)
The kernel's guarantees hold, but the design as written does not keep them. I found 6 major, 9 moderate and 8 minor counterexamples. Each has a small fix, mostly in spec text, and none is fatal. Three of the design's claims fail as stated:
- "closes critic gap 2 structurally": `supersedes` is an unauthorized precondition.
- "the link between a line and a stored object is defined by the spec": correspondence by bare name adopts other parties' objects.
- "no silent widening": package templates, defaults that packages add, and live controls each reopen the T5 hole.

I only read the repository; nothing was changed. Sources:
- spec/: ACC-005, 012, 022, 024, 029, 030, 033; CON-004, 008; LIF-009, 035, 037; OBJ-001, 002, 003, 010, 011, 018; IFC-012, 014; SEC-008, 009; NAM-017, 024; REC-004; TPL-012; core pool-management.v1.
- The design verdicts in docs/proposals/simplification/research/design-{A,B,C,D,E}-*.md, critic.md and synthesis.md.
- docs/proposals/ruling-53/README.md.

WHAT SURVIVES (checked)
- Access only through open Conduits that list operations. The translation emits only canonical Conduits with explicit, fully qualified lists (ACC-003 untouched). Every change is one of today's requests, so it is recorded. close.v1 still takes effect at once, and ACC-034 is unchanged.
- T5 cannot grant a new tool. Operations are never grouped, so append_file and copy_file are never listed.
- Purity defeats design C's M8 ("meaning depends on who runs apply"). AUT-014 is root-only and switched on by a written line, and no provenance is stored, so design C's D1 (forged provenance) cannot arise.
- The `--` naming is injective for the shapes AUT-016 lists:
  - keys exclude `--`, first-start names, and `def` as a grant key;
  - a trailing hyphen ("a-" + "--" + "b") still parses one way;
  - pools never have `reach`.
  - Gap: EKey (Pool entry keys) is not restricted from `--` or `def`. Restrict it the same way.
- AUT-014a lists all 7 operations of pool-management.v1, so design E's missing reconcile.v1 is fixed.

MAJOR

M1. `supersedes` is a precondition, not an authorization, so engine tombstones do not stop the actors critic gap 2 named (preservation test 5).
- Trace:
  1. A root responder closes agent-b--project, leaving mark (Agent agent-b, project).
  2. Agent X holds an `access` bound covering `shared`, which ACC-030 allows to open for any consumer.
  3. X's open is refused with conflict. Like LIF-037, the conflict names the ID, and IDs can also be read from records.
  4. X retries with `supersedes: <ID>` and the grant is open again.
- The same one-retry bypass lets a squatter, during T4's close-to-register window, register a Conduit with consumer agent-b, `as: project`, and its own target. agent-b's trusted tool names `project.*` then point at the squatter's directory.
- Fix: superseding needs authority. Either only root or the caller that made the mark may supersede, or it takes a distinct operation of `access` (for example reopen.v1) that a bound can withhold. Record each supersede with the mark: who, when, and which invocation. No core type keeps request inputs today.

M2. Correspondence by bare (kind, name) (AUT-018) adopts objects the project never registered. The replacement class then destroys them without a flag (ruling 53 #4, test 8).
- Trace:
  1. Project B on the same engine also has key `shared`.
  2. B's first `conduit apply` finds Resource `shared`, owned by project A. It "corresponds", and its definition differs.
  3. The plan replaces it break-before-make. It closes every Conduit to it, including A's Agents' grants ("including ones outside the form"), retires A's Resource, and registers B's.
  4. A's next apply sees `shared` nonconforming and replaces it back. The two projects ping-pong, each run cutting access.
- `external` entries are never compared at all. A retired external `reviewer`, re-registered by someone else (using M1's bypass), silently receives the form's grants on the next registration (design D #12).
- Fix:
  - Correspondence also requires a project identity that the engine stores at registration. Alternatively, translated names carry a project prefix.
  - An existing object this project did not register is "foreign": refused without `--adopt KEY`.
  - Closing or retiring anything whose consumer is outside the form needs a flag.
  - The plan flags any change of ID behind an external name.

M3. AUT-018's "what its translated document would resolve to now" plus register.v1 with no precondition on `from` makes root a confused deputy (design A C3, design E #4).
- T4, T5 and T7 register Conduits from stored `<e>--<k>--def` templates; T5 says the templates "conform".
- An actor with management replace.v1, which cannot open anything, replaces agent-b--project--def to add file.write.v1 or to drop a filter. It can do this before the plan reads state, so resolution "now" conforms, or between the plan and the register.
- Root's apply then registers from the changed template and opens the result with expectedId equal to the ID that register just returned. expectedId guards the instance's ID, not its content.
- Fix:
  - AUT-018 compares against the translated templates, never the stored ones, and covers templates explicitly.
  - Before any open, the plan compares the definition that register returned with the translation.
  - Or register.v1 gains an expected revision for `from`.

M4. Package templates carry access the author never sees, and a package version can add it.
- An entry `from: pkg#tpl` (or a package `templates:` item, now normative with Grant forms) translates to one instance document. Its declared Conduits come from the package template at instantiation.
- Neither render nor plan shows them before approval. Root's setup then opens them under unbounded `access` (ACC-005). A declared get-secret.v1 Conduit shows no tool (IFC-012), but the engine reads the secret into the run (SEC-010).
- Templates are frozen by nothing: NAM-024 covers contracts only, and PKG-030 is about remote provider items. So 0.1.3 can add a declared Conduit to `management`, and the next replacement opens it.
- Fix:
  - Render and plan expand and list every declared Conduit and entry of every template used, with target and operations; any added one is a widening that needs a flag.
  - An AUT rule: in a package template, the target of a declared Conduit comes from an input, never from a fixed name (design D #2).

M5. "Defaults a package version could flip" is still open.
- NAM-024 forbids a `default` only on an input added to a Requirement contract. A Type may add an optional schema property with `default: true`, for example a new reach flag on claude-code.v1.
- SCH-009 fills it in on the next replacement, while the authored file is unchanged (only the TOOL pin line moved).
- The design's T5 check compares only operation contracts and the tools map. It misses filled config, implementation bindings (packageArguments or environment adding a second allowed root, design D 3b), server version and nodeSelector. It is also tooling only, so an Agent manager (test 5) has no check at all.
- Fix: amend NAM-024 so that a property added to a Type's schema states no default. Make the full diff of the resolved definition and the implementation item part of what the plan must show, and treat any widening as needing a flag.

M6. Values set by a live control are silently reverted.
- AUT-018 leaves control pointers out of conformance.
- An operator narrows an allowlist or limit by control. A later pin bump or edit forces a break-before-make replacement, which instantiates the form's wider value.
- The plan shows no difference, because conformance ignores those pointers.
- Fix: a replacement carries the live control values forward, or shows them and needs a flag. Conformance reports such values as "set by control", visible rather than ignored.

MODERATE

m1. A revocation through the file is hostage to the whole translation.
- Translation is all or nothing.
- AUT-013 and AUT-014b become ambiguous when a package adds an offered capability or a capability operation, as NAM-024 allows.
- So an unrelated `conduit install` makes the form untranslatable and blocks removing a `reach` line.
- An expectedId conflict on a close also halts the apply.
- Fix: removals are planned from the translation last applied and run first. A conflict on a close re-reads state and closes whatever holds (consumer, local name).

m2. A grant to `access` with no bound is unlimited (ACC-029), in one terse line, and root's apply opens it. A grant to `management` is broad trust.
- Fix: AUT requires an explicit `bound.v1`, or `unbounded: true`, on a grant to `access`, and an explicit acknowledgment on a grant to `management`. This costs 0 lines in S1.

m3. AUT-009's default owner puts each new Resource line in the only Pool. Every existing bound that names that Pool then silently covers it (ACC-029; design D #4).
- Fix: the plan lists, for each new Resource, every bound that comes to cover it.

m4. Tombstones have no defined read path or disclosure rule.
- The plan needs the marks, but no operation lists them.
- A register conflict on a removed name is a new oracle that the object once existed (against ACC-033's last paragraph).
- Marks keep the caller, invocation and idempotency key beyond REC-004 purges.
- Retention is unspecified, and churning workers accumulate marks.
- Fix: define the read path and its authority. Disclose the caller and idempotency key only to root or the closer. Define retention.

m5. Tombstone keys miss several paths:
- A renamed grant key (project to workspace) brings a revoked grant back.
- Pool entries and declared Conduits get OBJ-003 generated names, so "names are a pure function of keys" is false for them. A retire mark keyed by (kind, name) does not match register-replacement.v1 of an entry.
- Whether cascade closes (CON-008) leave marks is undefined.
- What `--recreate` supersedes is undefined.
- Fix: key the mark of a nested instance by (holder, nested key). Also key Conduit marks by (consumer, target name). State that `--recreate` supersedes only the instance's mark.

m6. `.conduit/applied` is an authority input to a tool that acts as root.
- T7 auto-supersedes a mark that a previous run made, by matching an invocation ID in a committed file.
- Anyone who can write the repository can add a responder's close invocation, which they learn from the conflict, and reopen without `--reopen`.
- This departs from design A's fix: supersede only what this apply removed in the same run.
- Fixing it as design A did costs T7 one more line (the `--reopen`), which removes the 1-line margin.

m7. Explanation under ruling 52.
- No provenance is stored. Once the file is edited, inspect.v1 cannot say which rule made root--agent-a or shared--def.
- Fix: the translation writes a fixed description or annotation naming the AUT rule and the authored pointer, which the engine can check by translating again.

m8. The translated names are reserved only inside the form.
- The kernel allows `--` in any name, so other actors can squat names such as root--s1 or agent-b--project. That blocks T1, or a re-registration inside T4's window.
- Fix: reserve `--` names for translated documents, at the kernel level.

m9. The safety of transitions is not normative.
- Narrowing before widening, closes never blocked, the drain, and keeping the plan's hands off root's Conduits are all CLI behaviour.
- The replacement class says "close every Conduit to and from it". For an Agent that includes root--agent-x, which ACC-024 refuses, so the apply stops after access is already cut.
- Fix: about 5 normative invariants for transitions, not the 25-30 promises of a full normative plan.

MINOR
- n1. AUT-003 forbids the key `root`, yet the "stage 2 off" and "no derived defaults" readings write `interfaces: {root: {external: true, ...}}`. Neither reading, nor root's turn grants when derivation is off, is expressible.
- n2. AUT-013 does not say at which version the capability is read: the form's pin (filesystem 0.2.0) or the target Type's transitive pin (history holds 0.1.0). The capability's operation list can differ between them, so two engines can disagree (A34).
- n3. The as-if rule requires equal records and diagnostics, which contradicts AUT-017's located diagnostics and register.v1's differing inputs.
- n4. Description edits are either ignored, which leaves stale tool descriptions in front of Agents, or force a replacement. AUT-018 and the OBJ-018 amendment must say which.
- n5. Root's derived Conduits stay nonconforming forever after a change to the core or to an Agent's capability (ACC-024).
- n6. register.v1 accepts a form only for a first registration (OBJ-002 and SUB-007 conflicts), so an Agent manager makes every later change in long form.
- n7. AUT-017's redaction covers diagnostics only. render, plan and `.conduit/applied` should be redacted too.
- n8. Derived root Conduits put s1.retire-with-resources.v1 and similar tools in any MCP client that acts as root.

Compared with the layered route: M1, M4 and M5 affect both routes. M2, M3, M6 and m9 come from this design's AUT-018 and its break-before-make plan, and normativity makes them binding spec text, so they must be fixed before AUT is written.

Key files:
- spec/promises/naming.yaml (NAM-024)
- spec/conventions/access.yaml (ACC-005, 024, 029, 030, 033)
- spec/promises/objects.yaml (OBJ-003)
- spec/conventions/lifecycle.yaml (LIF-009, 037)
- docs/proposals/simplification/research/design-A-conservative.md (C1 fix: supersede only what this apply removed in the same run)
- docs/proposals/simplification/research/design-D-relationship-first.md (#2, #4, #12)
- docs/proposals/simplification/research/critic.md (gaps 2, 5, 8)

## Verdict expressiveness (major)
S2 to S10 can be expressed only with long-form fallbacks in four places. There are also three defects that matter for safety. Nothing is fatal, and each gap closes with a small AUT or addition change. The escape hatch half works. Names stay stable for top-level entries, `--def` templates and reach Conduits. They do not stay stable for Pool-entry Resources or declared Conduits, whose names the engine generates. A reference resolves to an ID when it is registered, so after a replacement the plan has to register the referring document again. It does that only for documents inside the project.

I read the repository at <home> and changed nothing. I checked these promises in spec/: ACC-022, ACC-024 and ACC-033 (spec/conventions/access.yaml); FST-001, FST-002, FST-005 and FST-006 (spec/promises/first-start.yaml); OBJ-002, OBJ-003, OBJ-013 and OBJ-014 (objects.yaml); CON-002, CON-004 and CON-008 (conduits.yaml); LIF-008, LIF-009, LIF-029 and LIF-035 (lifecycle.yaml); POOL-002 to POOL-015 (pools.yaml); SEC-008 and SEC-009 (secrets.yaml); TPL-010; NAM-027; ACC-029 and bound.v1 (spec/core/requirements.yaml); and packages/agent-claude-code/contracts.yaml. Everything below is from reading only. Nothing was built or run.

MAJOR

M1. Grants whose consumer is root cannot be written, apart from AUT-014's two derivations.
- AUT-003 forbids `root` as a key, and that includes external entries. Yet the "stage 2 off" and "no derived defaults" readings use `interfaces: {root: {external: true, reach: ...}}`. As drafted, the 34-line sensitivity rows are invalid.
- Root holds only `management` and `access` (FST-002). Three consequences follow:
  - (a) S3, S9 and S10 providers owned by `system` (the machine provider, the package source) need a root→system Conduit listing register.v1 and set-up.v1 (ACC-033 rows 1 and 4). AUT-014a derives Conduits only for Pool entries, so the plan cannot register or set those providers up.
  - (b) S5: a control is authorized by a Conduit to the instance itself (ACC-033 row 11). The plan runs as root and holds none, so "edit limit, and the plan sends the control" cannot happen.
  - (c) Under purity (AUT-005), root's turn grants cannot be derived for external Agents (for example, workers an Agent made).
- Without a fix, each of these needs a long-form document beside the form.
- Fix: allow first-start names as external keys so they can be consumers. Optionally derive root--system when any entry has `owner: system`.

M2. Matching stored objects to entries by name alone is engine-wide and unscoped.
- OBJ-002 makes names unique per kind across the whole engine. No provenance is stored.
- So a second project on the same engine (test 8, multiple owners, or the lab beside conduit-dev) treats any object of the same kind and name as its own under AUT-018. Its plan would propose replacing that object break-before-make: closing every Conduit to and from it, then retiring it.
- Retirement tombstones keyed by (kind, name) also block another project's reuse of the name. Derived names such as root--<x> and <x>--def collide across projects.
- Fix: project-scoped names (a prefix), or a stored project mark. Short of that, refuse to touch objects not made from this project's `--def` templates.

M3. Tombstones block declared Conduits after their consumer is replaced.
- Break-before-make closes "every Conduit to and from" the instance. That leaves marks keyed (consumer kind+name, local name) for its declared Conduits.
- The successor gets the same name. Its setup opens each declared Conduit as open.v1 (CON-004), which carries no `supersedes`, so the open is refused and the setup fails.
- Where it bites:
  - S2: E13 replaces `scoped` (its `upstream` Conduit);
  - S8: `search` and its Conduit to docs;
  - S4: running set-up again after a replacement;
  - the real use: workers renewed under the same names.
- The plan could open those Conduits first with `supersedes`, but this is not specified. A plain set-up, or one an Agent drives, fails.
- Fix: `supersedes` on set-up.v1, or exempt nested instantiations whose consumer ID is new.

M4. Declared Conduits cannot follow a replaced target.
- LIF-009 makes the replacement from the holder's resolved target, which is the old ID.
- The real-use layout (about 85 lines) puts main, orders and tree in the worker templates' declared `conduits`. Any replacement of main or orders (package upgrade, path change) therefore replaces all 10 worker Agents, losing 10 homes and logins (experiment 010).
- It can be avoided inside the form by writing standalone `reach` per worker: about +2 lines per worker, about 105 lines in all, and Conduits are then no longer opened as part of setup (G9).
- The form makes `conduits` and `reach` look the same. The plan must warn, or the docs must steer authors to `reach`.

MODERATE

M5. Pool entries are only partly expressible.
- An inline PoolEntry cannot express:
  - `{entry: x}` dependencies: POOL-005 rejects them outside a Pool template's entries, and inline config lands in a separate template p--x--def;
  - entry placement or runtime, which come only from Pool inputs (POOL-009), and an inline Pool has no inputs;
  - declared Conduits on an entry;
  - grants whose consumer is an entry's Resource, because Pool entries have no `reach`.
- So S4 (a chain of secret, then service, then webhook) and S3 Pools placed on m2 need `templates:`, which is the canonical spec minus the frame. The design's claim that "placement, runtime come for free" is wrong for entries.
- The change model has no class for entries:
  - generated names (OBJ-003) defeat matching by name and the (kind, name) tombstones;
  - adding or removing an entry, or changing its arguments, changes the Pool's resolved definition. That forces the Pool to be replaced, and with it every Resource it owns (the owner is fixed, POOL-001): a project-wide cascade.

M6. Controls (S5) are inconsistent.
- AUT-018 leaves values at control pointers out of the comparison, so an edit to `limit` in the file is invisible to it. Yet the design says the plan sends a control.
- The plan also lacks the Conduit to send one (M1b).
- Any replacement for another reason registers the instance again from the file's value. That silently undoes a live control.

M7. Limits of the escape hatch.
- (a) A long-form instance without metadata.name gets a generated name (OBJ-003), so AUT-018 cannot match it. A rule requiring names on long-form instances in a form project is needed.
- (b) Conduits outside the project (made by Agents, opened under a bound, or from other projects) are closed by break-before-make and never restored. The plan only lists them.
- (c) bound.v1 `targets` are references that resolve to IDs (ACC-029). The `share` bound in S2 goes stale on E13, and the listed cascade omits it. The comparison would catch it.

MINOR
- AUT-013 requires `capability` for targets outside the form (`access`, `management`). The S2 example's `share` grant omits it.
- AUT-014b rejects the whole form if any Agent Type has zero or several capabilities that start model work. There is no opt-out per entry.
- When the file names no placement, the plan sends the predecessor's node. The registered document then differs from the translation (the as-if rule, AUT-004), so that field's meaning depends on stored state.
- An inline Grant has no inputs. So `operations`, or a requirement's arguments, filled from an input (INS-012, INS-023) need a Conduit template in `templates:`. The real use's queue filter for each worker needs this.
- The AUT-018 comparison ignores whether an object is set up, so a failed setup (S4) counts as matching the file.
- The form's standard arguments are a fixed list. When rulings 49 and 51 add fields, the form needs AUT amendments; long form does not.
- After an additive package upgrade, capability inference can become ambiguous, which rejects the form. This fails closed.

BLOCKED ON THE KERNEL, the same for every route
- Peer mode and trust domains (51).
- Enrollment, which is a request (50).
- The expected account and pending sign-ins (49).
- An Agent made together with its own tree.
- for_each.
- A bound cannot cover future Agents, because Agents have no owner Pool. So an admin cannot be delegated turn grants to workers created later.

EXPRESSIBLE AS CLAIMED
- S2: secrets, requirements and filters, a bound on `access`, get-secret grants opened by root.
- S3: `placement` and `runtime` on top-level entries, shared-host.
- S6: close with expectedId, tombstones, `--prune`.
- S7: package `templates`, with contracts left in long form (two syntaxes).
- S8: filters with arguments for each grant.
- S9: repair as explicit requests.
- S10: a runtime swap means replacement for each entry. Swaps that involve providers owned by `system` (a package source, a machine provider) need the root→system Conduit of M1.

Overall severity: major. The form is viable only if M1 to M4 are fixed before it becomes normative.

## Verdict cost (minor)
The cost claims for normative sugar hold up in size, but are 15-30% low in five places. The form is affordable and bounded: no core rebuild, no store or document migration, and 0 existing vectors changed by the form itself. Its price over the layered package is about 31-35 more promise touches (claimed 28), about 50-65 new vectors, and about 700-1,500 more source lines (claimed 600-1,200). It saves no lines (S1 28 against 27). It buys meaning, learnability and agreement between implementations. In return every second engine must implement the translation, and every later kernel key pays a form tax.

Repository: <home> I only read it and changed nothing. Every figure below is a paper estimate; nothing was built or run.

## 1. Claims that check out
- **Promise count.** Exactly 455 promises across spec/promises, spec/conventions and spec/interface.
- **Crate sizes.** loader 2,879, core 10,465, shell 6,986, conformance 2,137, tools/check_schemas.py 2,588, all as stated. There are 367 cases.
- **The loader can run stage 1.** It already has `Node {value, position}` (loader/src/value.rs:31), and core already depends on the loader (core/Cargo.toml), so register.v1 can call a loader-side expander without a new crate edge.
- **Stage 2 has code to reuse.** `offered()` is at core/src/registration.rs:1241 and `capability_includes` at core/src/schemas.rs:919.
- **AUT-014a's list is right.** pool-management.v1 lists exactly the seven operations named (spec/core/capabilities.yaml:163-170).
- **Root's derived Conduits.** ACC-024 makes Conduits consumed by root impossible to close by request, and ACC-022 gives root unbounded open, as claimed.
- **Inference is unambiguous on S1.** local-directory.v1 and file-history.v1 each offer exactly one capability (directory.v1); each Agent Type offers only conversation.v1, whose turn.v1 is marked startsModelWork.
- **No existing vector changes because of the form.**
  - None of the 204 conduit.yaml/.json files under conformance/, spec/schemas/examples and packages/ has a top-level pools, resources, agents, interfaces, templates or operator key.
  - The only inline matches are bound arguments (`pools`) and fixture file maps.
  - Only 2 vectors pin a keyword pointer (`/pattern` and `/$defs/packages/type`), and adding optional properties to `$defs/manifest` moves neither.
  - MAN-001 to MAN-005 have no portable vectors. Their workspace examples (MAN-003 with `kind`, MAN-004 version without address) are untouched.
- **Increments 1-3 stand.** Every core change is additive.

## 2. Corrections: costs the design leaves out
1. **Promise touches for the form: 31-35, not about 28.**
   - **MAN-004 is missing.** MAN-004, not MAN-003, is the promise that closes the manifest's keys ("There are no other keys", spec/promises/manifest.yaml). It must be amended; MAN-003 may need no change.
   - **SUB-009 and replace.v1 are missing.** replace.v1's `manifest` input refers to `core#manifest.v1` (spec/core/operations.yaml), so once that schema gains the six sections replace.v1 accepts them. The design's rule that "a form's sections are rejected there" needs its own text.
   - **MAN-007 and PKG-018** (exports name the package's own documents) need a sentence once a package's manifest `templates` section produces exportable templates.
   - **SUB-006** needs a cross-note.
   - **OBJ-018 is counted twice**, once for the form and once in addition 5.
   - **project.v1 is a new core Schema export.** It enters spec/core/conduit.yaml's exports and the tables in spec/core/README.md and spec/schemas/README.md, which check_schemas.py step 5 requires to match exactly.
2. **"project.v1 `$ref`s the template-spec schemas (minus inputs), so S2-S10 keys come free" cannot be built within the profile.**
   - Every template-spec schema is closed with `additionalProperties: false` (spec/schemas/templates.yaml), and `unevaluatedProperties` is outside the profile (spec/schema-profile.md:128).
   - So an entry that adds reach, owner, placement, runtime or description cannot simply reference them. The options are:
     - per-key `$ref`s (implementation-map.v1, template-controls.v1, selection.v1) inside project.v1's own closed entry schemas;
     - refactoring the template-spec schemas into open bodies, which still needs a mirrored key list to close them;
     - a profile amendment (SCH-001/SCH-002 and schema.v1).
   - Either way every future template key costs one more line in project.v1 per kind. That is a standing mirroring tax. Size: +50-150 schema lines, or +1-2 promises for the profile route.
3. **Stage 1 does not produce "fully explicit canonical text".**
   - conduit-template-spec.v1 requires `capability` (templates.yaml:260). With inference or AUT-014b in play, stage 1's output is an intermediate.
   - So translated Conduit templates are schema-checked in core after stage 2, with positions passed into core.
   - **AUT-018 must also live in a library crate.** The conformance runner depends only on core and loader, and the design proposes 4-6 conformance vectors.
   - Core: +450-900 lines, not +240-430.
4. **The as-if rule (AUT-004) contradicts AUT-017.**
   - AUT-004 says diagnostics equal those of the translated documents, but AUT-017 places them in the form.
   - register.v1's records hold the form text, so records differ too. The equivalence property must exclude both.
   - **Purity has a gap at external entries.** An external target's Type is knowable only from stored state. AUT-013 must treat external entries as outside the form, and AUT-014b must skip external Agents.
   - These are wording fixes with no count change.
5. **The shell estimate (+1,750-3,000) inherits the planning-code estimate the critic flagged as low** (research/critic.md:102). This design's plan adds:
   - break-before-make with drain waits;
   - cascades through resolved references;
   - contract and tools-map diffs;
   - supersedes and the .conduit/applied record;
   - control-pointer changes sent as controls;
   - four flags and resume.

   A realistic figure is +2,250-4,000.
6. **The five additions do change existing vectors, on both routes.**
   - The synthesis's "0 invalidated" assumed advisory tombstones with register.v1 unchanged (synthesis.md:100-108).
   - Under ruling 53's engine-enforced marks with `supersedes` at register-replacement and open, conventions/s1/whole changes: it closes a Conduit (about line 4542) and then restores it with register-replacement.v1 (about line 5194).
   - The setup-restore family (4 cases) and probably changes/t4 also close and then re-register. Estimate 4-7 changed vectors, plus 3-11 snapshot updates for descriptions and the inventory.
   - LIF-035 ("its name can be given to another object") and REC-004 also need touches. The additions therefore come to about 13-17 promise touches plus 5 operation contracts (open, close, inspect, register, register-replacement), not 11-14.

## 3. Cost table

| Item | Layered package | Normative sugar (claimed) | Normative sugar (my estimate) |
|---|---|---|---|
| Five accepted additions, both routes | 11-14 touches (synthesis) | 11-14 | 13-17 touches + 5 operation contracts |
| Form promises: new + amended | 0 | 18 + about 10 = 28 | 18 + 13-17 = 31-35, plus register.v1 and replace.v1 text |
| Promise count after | 456 | 474 | 474 (455 + 18 AUT + LIF-038), about 45-52 touched |
| New spec text | Format reference in docs/ | authoring.yaml, project.yaml (250-350), +7 terms, +15-25 examples | Same, plus about 500-800 lines of AUT YAML, README reading path and contents, core export lists |
| Existing vectors changed | 4-7 changed + 3-11 snapshots (additions) | 0 (form) | 0 (form, verified by grep) + the same 4-7 + 3-11 from the additions |
| New vectors | 10-17 | 10-17 + 45-60 | 10-17 + 50-65 (adds AUT-018 and replace-rejects-form cases); about 5-10k lines of case data, much of it generated (S1 equivalence cases are about 1,500 lines each, like registration/s1) |
| loader | 0 | +930-1,500 | +930-1,500 (agree) |
| core, beyond the additions' +300-600 | 0 | +240-430 | +450-900 |
| shell / CLI | 3,500-5,500, likely the upper half per the critic | +1,750-3,000 | +2,250-4,000 |
| Runner + case.schema.json | Unit tests 2,500-3,500 | +190-380 | +240-480 (expand, equivalent, conforms steps) |
| tools/check_schemas.py | 0 | +300-500 | +300-500 (step 5/6 reference and rule checks on examples need a Python expander; 50-100 only for schema validation alone) |
| Total new source lines | about 3.5-5.5k | about 3.4-5.8k | about 4.2-7.4k |
| Delta over layered | — | +600-1,200 | +700-1,500 |
| Store migration | Tombstones only | None | None for the form. Tombstones add one State key: old stores load (serde default under deny_unknown_fields), but closes made before the upgrade are unmarked unless backfilled from unpurged records |
| Document migration | None | None | None. Every canonical document stays valid and the form is opt-in |
| Adopting an existing deployment | Not addressed | Not addressed | Only by re-creation or `external`. Correspondence is by translated name only, and the coordinator's objects (project-a, turn-a, workspace-pool, root-workspace) match none of the translated names. No import/moved mechanism; a key rename is remove plus add. Cheap for v0, since the rerun starts from an empty store |
| Obligation on a second engine | None | Expander + stage 2 + AUT-018 | About 1.0-1.5k lines to claim the Language level |
| Work orders | 4-6 | 6 | 6-7 |
| S1 lines | 27 | 28 (27 with root derivation on by default) | Same. Normativity changes no line count |

## 4. Reading cost
- **Implementer.** 455 → 474 promises (+4%), with none removed. They gain a second grammar (project.v1) and 18 AUT rules, +7 definition terms, and the additions' concepts (as, supersedes, tombstone, inventory). The dead weight S1 never uses stays.
- **Author.** The reading path becomes definitions.md + authoring.yaml + the kernel promises that diagnostics cite. Inspection and records still show canonical objects (including `--def` templates and `e--k` names), so authors still meet the canonical shapes. The claim of 15-18 concepts is plausible for S1 only.
- **Package writers.** They meet two syntaxes, since contracts stay long form.

## 5. Verdict
- The design's cost model is sound in shape, and its central safety-cost claims hold: no rebuild, no store migration, 0 form-induced vector changes, and code that mostly moves from the CLI into the loader and core.
- It understates by about 15-30%: missing amendments (MAN-004, SUB-009/replace.v1, MAN-007/PKG-018, the core export lists), the profile's closed-schema limit, AUT-018 and Conduit-template validation landing in core, and the planning-code estimate the critic flagged.
- Normative sugar is in effect "promote the translation now, without a normative plan". It costs about 1.4-1.6 times the layered package's spec-side touches overall (about 44-52 against 13-17), and about 15-30% more code.
- It saves zero lines, so the bar's margins and its failure readings are unchanged.
- The decision should rest on whether thesis 2 for authored text and agreement between implementations (A34) are worth a permanent second grammar.
- If the owner accepts it, fix before the spec batch:
  - amend MAN-004 instead of MAN-003;
  - add a rule that replace.v1 rejects form sections;
  - pick a project.v1 composition strategy that fits the profile;
  - reconcile the as-if rule with AUT-017;
  - exclude external entries from AUT-013 inference and AUT-014b;
  - budget the 4-7 vectors that enforced tombstones invalidate on either route.