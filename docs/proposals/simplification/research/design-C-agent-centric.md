Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Agent-centric project file: one Compose-style conduit.yaml with derived Conduits, changed by plan and apply

The author writes one file, `conduit.yaml`, which is today's manifest grown into a Compose-style project file. It has the version once, a project name, packages by exact version, resources keyed by name, and agents keyed by name, each with its Conduits inline at one line per grant (`project: {target: shared, operations: [file.read.v1, directory.list.v1]}`). The engine expands that file by numbered, deterministic rules in spec/ into the same stored objects as today: a Pool, Resources, Agents and Conduits, each Conduit with a consumer, a target, a capability and an explicit list of operations, all decided per request and recorded. The root Interface's Conduits to its Pool and to each Agent are derived rather than written, and the digests go into a lock file that the tool writes. Change is edit-and-apply: a plan computed by the pure core lists every object, grant and tool before and after, each with the file line and the rule that made it. Apply then carries out the plan as single-object requests: `register.v1` for new entries; a new `revise.v1`, which gives an existing instance a new revision and keeps its ID, name and tool names; and a new `swap.v1`, which replaces an open Conduit with a new one in one decision, so Conduits stay immutable values and every grant is decided again when what it reaches changes. Explicitness under ruling 52 moves from the author's text into what the engine shows: the plan, a report for each step, `render` (the full long form, which can itself be submitted), `reach`, provenance in inspection, and two-tier refusals that name the derived Conduit. On paper S1 takes 30 file lines, or 33 counting command values, against the baseline's 64 and the coordinator's 332. That is less on six tasks, equal on T3, and much more on none.

## Spec changes
## A. New: the project file, a language-layer form beside the long form

Proposed file: spec/promises/project-file.yaml, PRJ-001 to PRJ-012, plus a schema `project-file.v1`.

- **PRJ-001 (form).** A submitted project's manifest `conduit.yaml` may also hold:
  - `project` (a name);
  - `pools`, `resources`, `agents` and `interfaces`: maps keyed by instance name;
  - `conduits`: standalone Conduits;
  - `templates`: named long-form templates, for reuse.

  Other rules of the form:
  - An item of `packages` may be a bare exact version (`example.org/s1/filesystem: 0.1.0`). Its digest then comes from `conduit.lock` (PRJ-008).
  - `apiVersion` is stated once, and every object expanded from the file carries it (OBJ-017, OBJ-024).
  - Unknown keys are rejected (strict loader).
  - A package's manifest may not hold the instance sections (MAN-011 kept).
  - Amends MAN-003, MAN-004 and MAN-005. MAN-001 and MAN-002 are unchanged: it is still the one `conduit.yaml`.

- **PRJ-002 (entries).** Each entry is one instance of the family its section names, named by its key.
  - NAM-027 applies across the whole file, derived names included. A clash is rejected, naming both lines or the rule.
  - Long-form documents beside the manifest stay valid.

- **PRJ-003 (inline definitions).** An entry's `type`, `config`, `conduits`, `owner`, `placement` and `runtime` are an inline definition: IN9's one-off, "a template with no inputs, instantiated once", but unnamed. `from: <template>` is still allowed in place of an inline definition, for reuse.
  - Amends INS-001: a template has `spec`; an instance has `from` or `definition`.
  - Amends INS-002 (made from its own anonymous template).
  - Amends OBJ-003: a nested instance with no template name is named `<consumer>-<local>-<suffix>`.
  - Rulings 8 (IN9) and 18 (V2) need the owner's amendment.

- **PRJ-004 (Conduit entries).** The form is `<local>: {target: <name>, operations: [...], requirements: {...}}`.
  - **Target.** `target` names an entry of the file, or `management` or `access`. `{kind, name}` reaches an object outside the file.
  - **Capability.** It is inferred as the one capability the target's Type offers, at the version the target holds, whose operations include every listed one.
    - With none or several, the file is rejected, with a diagnostic listing each candidate and its operations.
    - `capability:` may be written to choose.
    - The inferred capability is stored, and inferred again only when the entry changes.
  - **Operations.** They are written by export name and qualified by that capability's package.
    - The list is always explicit: there is no "all" (CON-017, G6).
    - The owner rules whether relative names are acceptable under ruling 27's "fully qualified". The fallback is full addresses: same line count, wider lines.

- **PRJ-005 (default Pool).** With no `pools`, the project has one Pool named by `project`, of type `core#pool.v1`. It owns every Resource that names no `owner`. POOL-001 still holds: exactly one owner, never changed.

- **PRJ-006 (Pool administration).** For each Pool registered from the file, the expansion adds a standalone Conduit `root-to-<pool>`.
  - It runs from the registering Interface to the Pool, with `as: <pool>`.
  - It lists every operation of `pool-management.v1` at the core version in force, written out in the stored Conduit. The list is expanded once and never again.
  - This is a real Conduit, opened through the registering Interface's own Conduit to `access` (ACC-022 for root), so G2 holds and ACC-033 gains no exception.

- **PRJ-007 (operators).** Each Agent gets one standalone Conduit `<iface>-to-<agent>` from each Interface in `operators`, per capability its Type offers.
  - `operators` defaults to the registering Interface; `operators: []` makes none.
  - Each Conduit has `as: <agent>` and lists that capability's operations, expanded once.
  - Root's are unclosable under ACC-024, as today.

- **PRJ-008 (pins and lock).** `packages` gives one exact version per address (NAM-006).
  - The digest and `fetch` of each come from `conduit.lock`, which installation writes and which is part of the manifest.
  - A listed package that is missing from the lock, or not installed at that digest, is rejected, naming the package (PKG-006).
  - No ranges, no implicit installation, no aliases (D17).
  - Amends NAM-006 and MAN-005 as to where the digest is written. It is still required and still checked (G5).

- **PRJ-009 (provenance).** Every object expanded from a file holds `source`:
  - the file's digest, and the line and key path of its entry;
  - the rule that made it, for a derived object;
  - the Interface that registered or last revised it, and its revision.

  `inspect.v1` returns `source` to callers that may inspect the object. Diagnostics point at the file's lines (SUB-005, ruling 52).

- **PRJ-010 (visible defaults).** Values filled from a Type's schema defaults (INS-019) are marked as defaults in inspection and in `render`.

- **PRJ-011 (canonical form).** The expansion depends only on the file, the lock and the installed contracts: not on the working directory, the environment or the host (INS-017, INS-018, G6). Rendered as long-form documents, it is a valid submission that resolves to the same objects.

- **PRJ-012 (nothing implicit).** Expanding registers nothing. A file takes effect only through `register.v1` and `revise.v1` requests (SUB-006, G8).

## B. New: changing what exists (conventions and core)

**`revise.v1`**, in `object-management.v1` and in `pool-management.v1`.

Inputs:
- `instance`: a reference (LIF-031);
- `revision`: the current one, so REV-002 and REV-011 apply;
- `definition`: the whole new held definition;
- `contracts` and `manifest`, as `replace.v1` takes them under SUB-009;
- optional `reopen`: a list of local names.

Effects, as parts in this order (OPS-022):
1. For each nested Conduit that the new definition drops, `close`.
2. For each nested Conduit whose consumer-side content changes (target, capability, operations or requirements), a swap: a new Conduit under the same local name.
3. Where the instance's own type, config, packages, placement or runtime change:
   - every open Conduit the instance consumes is re-decided by a swap for an identical one;
   - then its run is stopped and a new run is started at the new revision (PRV-007, one run per instance, break before make), and its setup hook runs (TPL-012 order);
   - then every open Conduit that targets it is re-decided by a swap.
   - So no grant outlives a change of the code or configuration it reaches or runs under (G4).
4. For each nested Conduit that the new definition adds: instantiate it, then open it through the requester's own Conduit to `access` (as ACC-005 does).

Rules that bind every revision:
- A nested Conduit that a request closed, whose declaration the new definition keeps unchanged, stays closed. That part reports conflict, unless `reopen` names it (G3).
- The ID and name are kept and the revision advances.
- Agents' sessions continue; tool changes reach them through IFC-038.

**`swap.v1`**, in `access-control.v1`. It closes an open Conduit and, in its place, opens a new Conduit with the same consumer and local name, in one change of the engine's decision state.
- No request is decided under neither Conduit, or under both.
- It is authorized as a close (ACC-012) plus an open with its bound (ACC-006, ACC-030), and only root opens one that lists `get-secret.v1` (SEC-008).
- In-flight work follows ACC-034.
- The new Conduit has a new ID. Both events are recorded with one swap ID.
- ACC-011's "close and open a narrower one" is carried out by this.
- Precedent: SpiceDB and OpenFGA atomic delete-plus-create writes; `kubectl auth reconcile`.

**ACC-033 gains row 15.** `revise.v1`: `management`, and for a Resource also the Pool that owns it, each listing `revise.v1`. Its parts are each authorized as their own operation.

**Core templates:**
- `root-management` adds `revise.v1`;
- `pool-management.v1` adds `revise.v1`;
- `root-access` adds `swap.v1`.

**Amended promises:**
- OBJ-011, second clause: "no document rewrites an instance's definition; only `revise.v1` gives it a new revision, keeping its ID and name".
- PKG-005, INS-020 and NAM-011: an instance keeps its definition and packages "until a `revise.v1` names it".
- LIF-009: for an inline nested instantiation, the replacement is built from the held definition, not from "the template as it stands". This removes 4-01's shared-template coupling.
- REC-001 adds revise and swap events. REC-002 adds the revisions of the consumer and the target at dispatch, so that behaviour is attributed to code.

**Unchanged:** OBJ-002, SUB-006 and SUB-007 (submitting still only registers, so ruling 22/DD2 stands), LIF-035, LIF-008, ACC-011, ACC-024, CON-008 and no reconciler (OBS-004).

## C. Interface

- OBJ-013 gives standalone Conduits a standard argument `as`, the local name, defaulting to the Conduit's own name. IFC-012 is amended to match.
- IFC-014 is unchanged and still rejects clashes for each consumer.
- Root's tools become `s1.set-up.v1` and `agent-a.turn.v1`, and Agents' tools keep `project.*` and `history.*` through T4 to T7.

## D. Packages (provider work, not counted)

The Claude Code and Codex Agent Types give `default: false` to `accountServices` and `peerMessaging`: ruling 48's "off by default", which today's schemas require with no default.

## E. Reference command line (tooling; no promise, as PKG-006 allows one install command)

- **`conduit install --from DIR`.** Installs (`install.v1`) every package `conduit.yaml` lists that is not installed at its version, from the archives in DIR. It prints each digest and writes `conduit.lock`. `path#sha256:...` is still accepted to assert a digest.
- **`conduit plan [-o FILE]`.** The pure core expands the file, inspects the project's objects (by `source`), and lists:
  - each step: register, set-up, open, revise with its parts, or swap;
  - every object and grant, with its line and rule;
  - each Agent's tools before and after, and any restart;
  - objects not in the file, left alone unless `--prune`;
  - conflicts: an object revised by another actor since the last apply, or a Conduit closed by a request.
- **`conduit apply [FILE|PLAN]`.**
  - It shows the plan and asks for confirmation (`--yes` skips the prompt).
  - It sends each step with the revision read at plan time, so a stale plan reports conflict.
  - It reports each step's outcome, the IDs made and removed, and tool-list changes, and exits non-zero unless every step succeeded (4-13, ruling 52).
  - It never retires without `--prune`, never reopens without `--reopen`, and never overwrites another actor's revision without `--force`.
  - It is a client composition of operations an Agent could also be given (preservation test 5), like `submit` today.
- **Views.**
  - `conduit render`: the canonical documents.
  - `conduit reach [AGENT]`: tools, targets and provenance for each Agent, plus reach around the engine as its Type states it.
  - `conduit close|open AGENT/LOCAL`: `open` composes `register-replacement.v1` with `open.v1`.

## F. How explicitness (ruling 52) and inspection survive derivation

1. **Derived objects are real objects.** Every one is a stored canonical object (a Conduit with consumer, target, capability and its operations written out), with an ID, records and inspection. Nothing is virtual or computed at decision time.
2. **Derivation is visible and repeatable.** It happens only at an explicit, confirmed plan or apply, by numbered rules, deterministically. The plan shows its full result before anything happens.
3. **Each view says why.** Every view (plan, step report, `inspect.v1`, `render`, `reach`) names the line and rule behind each object. Defaults are marked as defaults. Ambiguity is refused, never guessed. An "every operation" expansion is stored once, and an upgrade never re-expands it: the plan states "tools unchanged".
4. **Refusals follow ruling 52's two tiers.**
   - An Agent hears the operation, the target and that no open Conduit lists it.
   - A caller that may inspect also sees each Conduit from it to the target, open or closed, with its operations and its `source`.
   - File paths and lines are shown only at that tier, so they disclose nothing to Agents.
5. **Reviews see the expanded form.** `render` is a full, valid long form, so a reviewer can diff exactly what the engine will hold, and `plan` shows that diff and not the diff of the short file.

## G. What stays

- the five families;
- the Conduit as the only permission, pairwise, one capability, with explicit operations;
- decision at the target per request;
- the ACC-033 table, with one row added;
- revocation that is immediate, and closed Conduits that are removed;
- no reconciler and no rollback, with unknown as an outcome;
- revisions;
- explicit install and digest pins;
- all secrets rules;
- the long form for packages and reuse;
- one run per instance.

## H. Rebuild cost

- **Loader:** a project-file reader and schema.
- **Core:**
  - the expansion, which is pure and cites PRJ-*;
  - inline definitions in `registration.rs`;
  - revise and swap in `lifecycle.rs`, `access.rs`, `records.rs` and `requests.rs`;
  - first start: `root-management` and `root-access` gain operations.
- **Shell:** install `--from`, plan, apply, render, reach.
- **Tooling:** `check_schemas.py`.
- **Vectors:** new vectors for PRJ, revise and swap. Existing long-form vectors survive, except those that assert PKG-005, OBJ-011 and LIF-009 immutability.

## Rewrite
Paths are absolute values the author chooses (ruling 52, point 3). `conduit serve` and checks (turns, `tools --as`) are harness and are not shown. Engine output is shown only where marked "not authored". The authored files, one per task state, are at <scratch>/agent-centric/ (t1.yaml, t2.yaml, t4.yaml, t5.yaml, t6.yaml, t7-revoked.yaml, t7-restored.yaml).

### T1: two agents share a project directory, A reads, B reads and writes

File `conduit.yaml` (new, 19 lines):
```yaml
apiVersion: github.com/shrav-k/conduit/v0
project: s1
packages:
  example.org/s1/filesystem: 0.1.0
  example.org/conduit/agent-claude-code: 0.1.1
  example.org/conduit/agent-codex: 0.1.1
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    conduits:
      project: {target: shared, operations: [file.read.v1, directory.list.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    conduits:
      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```
Commands (2 requests; the first carries 1 value the author chose):
```sh
conduit install --from <lab>/packages
conduit apply
```
Not authored: `install` prints three `install.v1` results with their digests (sha256:200de7c1..., 166ce271..., 05b47a85...) and writes `conduit.lock`. `apply` prints, before asking:
```text
Plan: project s1 (conduit.yaml sha256:..., conduit.lock sha256:...), as Interface root
 + Pool s1                                  conduit.yaml:2   PRJ-005 default Pool, core#pool.v1
 + Conduit root-to-s1  root -> Pool s1  as s1   conduit.yaml:2   PRJ-006  pool-management.v1: register.v1 register-replacement.v1 revise.v1 inspect.v1 set-up.v1 retire.v1 retire-with-resources.v1
 + Resource shared  owner s1 (PRJ-005)      conduit.yaml:8   example.org/s1/filesystem#local-directory.v1 @0.1.0 sha256:200de7c1...
     config {root: <lab>/shared}; runs npm @modelcontextprotocol/server-filesystem 2.1.0 (package item filesystem-server)
 + Agent agent-a                            conduit.yaml:12  example.org/conduit/agent-claude-code#claude-code.v1 @0.1.1
     config {accountServices: false (default), peerMessaging: false (default), model: '' (default)}
   + Conduit agent-a/project -> Resource shared   conduit.yaml:15  capability example.org/s1/filesystem#directory.v1 (PRJ-004)
     operations file.read.v1 directory.list.v1   (not file.write.v1)
 + Conduit root-to-agent-a  root -> Agent agent-a  as agent-a   conduit.yaml:12  PRJ-007 operators [root] by default; conversation.v1: turn.v1
 + Agent agent-b ... + Conduit agent-b/project: file.read.v1 directory.list.v1 file.write.v1 ... + Conduit root-to-agent-b
 Tools after:  agent-a project.file.read.v1 project.directory.list.v1
               agent-b project.file.read.v1 project.directory.list.v1 project.file.write.v1
               root    + s1.* (7), agent-a.turn.v1, agent-b.turn.v1
 Requests: management.register.v1 (7 documents, 9 objects); management.set-up.v1 Pool s1; access.open.v1 root-to-s1;
   s1.set-up.v1 Resource shared; management.set-up.v1 Agent agent-a (opens agent-a/project);
   management.set-up.v1 Agent agent-b (opens agent-b/project); access.open.v1 root-to-agent-a; access.open.v1 root-to-agent-b
Apply 8 requests? [y/N]
```

### T2: a shared message history, both post and read

Edits to `conduit.yaml` (6 lines added; nothing else changes):
```diff
 packages:
   example.org/s1/filesystem: 0.1.0
+  example.org/s1/history: 0.1.0
   example.org/conduit/agent-claude-code: 0.1.1
   example.org/conduit/agent-codex: 0.1.1
 resources:
   shared:
     type: example.org/s1/filesystem#local-directory.v1
     config: {root: <lab>/shared}
+  history:
+    type: example.org/s1/history#file-history.v1
+    config: {root: <lab>/history}
 agents:
   agent-a:
     type: example.org/conduit/agent-claude-code#claude-code.v1
     conduits:
       project: {target: shared, operations: [file.read.v1, directory.list.v1]}
+      history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
   agent-b:
     type: example.org/conduit/agent-codex#codex.v1
     conduits:
       project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
+      history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```
Commands (2 requests; 1 author value):
```sh
conduit install --from <lab>/packages
conduit apply
```
Not authored. The plan shows:
- + Resource history (owner s1), by `s1.register.v1` and `management.register.v1`, then `s1.set-up.v1`.
- ~ Agent agent-a, revision 1 -> 2: + Conduit agent-a/history, open, tools gained `history.file.read.v1`, `history.directory.list.v1`, `history.file.write.v1`. The same for agent-b.
- No restart and no session lost. The history's posting convention is shown from the type `file-history.v1`'s description, which is provider work.
- 5 operation requests in all.

The whole file after T2 (25 lines):
```yaml
apiVersion: github.com/shrav-k/conduit/v0
project: s1
packages:
  example.org/s1/filesystem: 0.1.0
  example.org/s1/history: 0.1.0
  example.org/conduit/agent-claude-code: 0.1.1
  example.org/conduit/agent-codex: 0.1.1
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
  history:
    type: example.org/s1/history#file-history.v1
    config: {root: <lab>/history}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    conduits:
      project: {target: shared, operations: [file.read.v1, directory.list.v1]}
      history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    conduits:
      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
      history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```

### T3: what each agent can reach

Nothing is written. One command:
```sh
conduit reach
```
Not authored. Excerpt of the output:
```text
agent-a  Agent, claude-code.v1 @0.1.1, ready, revision 2, operated by root (agent-a.turn.v1)
  through Conduit:
    project.file.read.v1, project.directory.list.v1 -> Resource shared (<lab>/shared)   agent-a/project, open, conduit.yaml:19
    history.file.read.v1, history.directory.list.v1, history.file.write.v1 -> Resource history (<lab>/history)   agent-a/history, open, conduit.yaml:20
  around Conduit, as claude-code.v1 states it: shell, code execution, browser, other-Agent, hook and plugin reach disabled;
    accountServices false (default); peerMessaging false (default); mediated, not OS isolation
agent-b  ... plus Codex's collaboration.* tools (finding 4-06, stated by the package)
```

### T4: A may also write, B may only read

Edits to `conduit.yaml` (2 lines changed):
```diff
   agent-a:
     type: example.org/conduit/agent-claude-code#claude-code.v1
     conduits:
-      project: {target: shared, operations: [file.read.v1, directory.list.v1]}
+      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
       history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
   agent-b:
     type: example.org/conduit/agent-codex#codex.v1
     conduits:
-      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
+      project: {target: shared, operations: [file.read.v1, directory.list.v1]}
       history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```
Command (1 request):
```sh
conduit apply
```
Not authored:
```text
 ~ Agent agent-a  revision 2 -> 3   conduit.yaml:19
   ~ Conduit agent-a/project: swap (swap.v1), operations + file.write.v1; tools gained project.file.write.v1
 ~ Agent agent-b  revision 2 -> 3   conduit.yaml:24
   ~ Conduit agent-b/project: swap, operations - file.write.v1; tools lost project.file.write.v1
 No interval without access. Requests: management.revise.v1 agent-a (rev 2); management.revise.v1 agent-b (rev 2)
```

### T5: upgrade the server behind the directory, 2.1.0 to 2.2.0

Edit to `conduit.yaml` (1 line changed):
```diff
 packages:
-  example.org/s1/filesystem: 0.1.0
+  example.org/s1/filesystem: 0.2.0
```
Commands (2 requests; 1 author value):
```sh
conduit install --from <lab>/packages
conduit apply
```
Not authored:
```text
 install.v1 example.org/s1/filesystem 0.2.0 sha256:cbb831f0... installed beside 0.1.0 (NAM-011); conduit.lock updated
 ~ Resource shared  revision 1 -> 2   conduit.yaml:4
     package example.org/s1/filesystem 0.1.0 -> 0.2.0; implementation npm @modelcontextprotocol/server-filesystem 2.1.0 -> 2.2.0
     run restarts (requests to shared meanwhile report unavailable); re-decided by swap, same operations: agent-a/project, agent-b/project
     tools: unchanged for every Agent (0.2.0 maps file.read.v1, directory.list.v1, file.write.v1; append_file and copy_file are not presented)
 = Resource history: unchanged (resolves filesystem through example.org/s1/history 0.1.0, NAM-014); = Conduits */history: unchanged (NAM-024)
 Requests: s1.revise.v1 + management.revise.v1 Resource shared (rev 1)
```

### T6: replace the directory with another, keeping access

Edit to `conduit.yaml` (1 line changed):
```diff
   shared:
     type: example.org/s1/filesystem#local-directory.v1
-    config: {root: <lab>/shared}
+    config: {root: <lab>/shared-2}
```
Command (1 request):
```sh
conduit apply
```
Not authored:
```text
 ~ Resource shared  revision 2 -> 3   conduit.yaml:11  config.root <lab>/shared -> <lab>/shared-2
     run restarts; re-decided by swap: agent-a/project, agent-b/project now reach <lab>/shared-2; tool names unchanged
```

### T7: revoke A's access, check B, restore A

Revoke: delete 1 line from `conduit.yaml`.
```diff
   agent-a:
     type: example.org/conduit/agent-claude-code#claude-code.v1
     conduits:
-      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
       history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```
```sh
conduit apply
```
Not authored: ~ Agent agent-a, revision 3 -> 4: - Conduit agent-a/project, close; tools lost `project.*` (3). The check of B is a turn through `agent-b.turn.v1`, not counted. A call A forces receives: "refused: no open Conduit from agent-a to Resource shared lists file.read.v1".

Restore: put the same line back. Under the counting rule it is the same line, counted once per task.
```diff
     conduits:
+      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
       history: {target: history, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```
```sh
conduit apply
```
Not authored: ~ Agent agent-a, revision 4 -> 5: + Conduit agent-a/project, register and open. It is a new Conduit, because the closed one was removed (LIF-035), but it has the same local name and tools.

Alternative route for T7, with the file untouched: 0 file lines and 2 command lines with author values. `apply` would not reopen the Conduit later: the plan would show it as closed by request, a conflict.
```sh
conduit close agent-a/project
conduit open agent-a/project
```
After T7 the file is identical to the file after T6 (25 lines).

## Counts
How the counts were made:
- **Lines** follow the measurement plan: non-blank, comments excluded, `---` counted (none occur), a line changed twice in one task counted once, removed lines counted as changed.
- **Two counts are given.** "File lines" is the count both Conduit runs reported. "Strict" also counts each command that carries a value the author chose, as the plan literally says; neither run counted those.
- **Requests** are the author's commands that are not checks. The operation requests underneath them are in brackets.

| Task | File lines | Command lines with author values | Strict | Files edited | Requests (operation requests) | Baseline A70 | vs baseline | Coordinator | vs coordinator |
|---|---|---|---|---|---|---|---|---|---|
| T1 | 19 | 1 (`install --from <lab>/packages`) | 20 | 1 (lock written by tool) | 2 (11: 3 install, 1 register, 4 set-up, 3 open) | 33 in 5 files | Less (-13) | 154, 4 requests | -134 |
| T2 | 6 | 1 | 7 | 1 | 2 (5: install, register, set-up, 2 revise) | 16 in 5 files | Less (-9) | 73, 5 requests | -66 |
| T3 | 0 | 0 | 0 | 0 | 1 (`reach`) | 0 (10 items read + 1 command) | Equal on lines, less effort | 0, 2 commands | Equal |
| T4 | 2 | 0 | 2 | 1 | 1 (2 revise, each a swap) | 8 in 2 files, 1 restart | Less (-6) | 2, 8 requests, ~30 s without access each | Equal lines; 7 fewer requests; no gap; names stay true |
| T5 | 1 | 1 | 2 | 1 | 2 (install, revise) | 3 in 2 files, 2 recoveries + 1 expected | Less (-1) | 62, 5 requests | -60 |
| T6 | 1 | 0 | 1 | 1 | 1 (1 revise) | 2 in 2 files | Less (-1) | 29, 8 requests, ~80 s without access | -28 |
| T7 | 1 (removed then restored) | 0 | 1 | 1 | 2 (2 revise) | 2 in 1 file | Less (-1) | 12, 3 requests | -11 |
| All | 30 | 3 | 33 | 1 distinct | 11 (23 + T3's inspections) | 64 | Less on 6, equal on T3, much more on none | 332, 35 requests | -299 |

**Other measures, from paper:**
- Duplicated configuration: 0 values (baseline 8).
- Provider-specific lines: 0.
- Contract-address lines: 8 (4 package pins + 4 `type:`), against the coordinator's 45.
- Digests typed: 0 (coordinator 8).
- Root's Conduits written: 0 (coordinator 4 documents, 70+ lines).
- Grants made by hand: T1 2, T2 2, T7 1.
- Time without access: T4 none (one swap per Agent). T5 and T6 have no interval without a grant, but an interval of unavailability while `shared`'s stdio server restarts; it is expected to be seconds and is not measured. T7: A's closure was intended.

**Sensitivity, where each line comes from, honestly:**

| Variant | T1 | T2 | T7 | Strict total |
|---|---|---|---|---|
| Agent Types keep `accountServices`/`peerMessaging` required with no default | +2 | | | +2 |
| Ruling 49's expected account becomes a required line per Agent | +2 | | | +2 |
| `operators` written instead of derived | +2 | | | +2 |
| Block style per Conduit (3 lines each, not one line per grant) | +4 | +4 | 3 by the file route, so T7 uses the command route (strict 2, equal to the baseline) | +9 |
| Fully qualified operation names | 0 (wider lines only) | | | 0 |

- **Worst combination:** T1 30 against 33, T2 11 against 16, T3 0, T4 2, T5 2, T6 1, T7 2; strict 48. That is still less on 5, equal on T3 and T7, and much more on none.
- **The floor investigation's limits:** the strict floor is 21 and the explicit floor 27 to 30; this design's 30 file lines sit at the explicit floor.
- **These are paper counts.** No loader accepts the project file, nothing ran, and ruling 45 does not accept paper counts as evidence. A fresh-author run on an implementation is required.

## Lost
1. **The author's text no longer describes itself.** `conduit.yaml` omits:
   - the capability;
   - fully qualified operations;
   - the owning Pool;
   - root's grants to the Pool and to each Agent;
   - Type defaults;
   - digests.

   A reviewer reading only the file, or a git diff of it, does not see those. They appear only in `plan`, `render`, `reach` and inspection. Ruling 52 is about what the engine says, so this design meets it, but the owner must accept that explicitness moves from the authored text into generated views. Those views must be computed by the engine's own decision code, or they become false assurance.

2. **Grants are made by rule, not by the author's hand.** PRJ-006 and PRJ-007 make root's Conduits to its Pool (every pool-management operation) and to each Agent (every operation of its offered capabilities) without the author writing them.
   - They are real, recorded and inspectable, and shown before apply.
   - Under ACC-024, root's cannot be closed, so `operators: []` set later cannot remove them; only retiring the target does.
   - The derived Pool Conduit is broad, though root already holds that authority through ACC-022.

3. **Instances are no longer immutable.** OBJ-011's second clause, PKG-005, INS-020, NAM-011 and DR5 (ruling 21) are reversed in part. An ID no longer always means one definition.
   - What keeps this safe: `revise.v1` needs the current revision, records every part, and re-decides every Conduit to and from the instance by swap. Secret-reading Conduits are re-opened only by root (SEC-008).
   - What it costs: records must now carry the revision of each end to attribute behaviour to code. In-flight invocations and hooks at a restart (round 6's D09) are not fully worked through.

4. **Restarts are visible.** Under one run per instance, T5 and T6 are break-before-make: a short interval in which `shared` reports unavailable. The coordinator's routes ran a new Resource beside the old one in T5 and had no such interval. Make-before-break would need two runs per instance, which round 6 says to keep at one.

5. **A named template is no longer the norm.** IN9 and V2 (rulings 8 and 18) are amended, and inline anonymous definitions become the default. A shape reused across many Agents must be repeated, or written as a named long-form template under `templates:`.

6. **Two normative forms.** The project file and the long form must both be specified, schema-checked (`check_schemas.py`), loaded and covered by vectors. Independent builds must implement the expansion identically, which makes the spec bigger.

7. **The capability is inferred.** Inference is exact or refused, but the author no longer states it. A later package that offers a second capability holding the same operations makes that entry ambiguous at its next change (refused, never guessed).

8. **Identity is by name within the project.** Renaming an entry is a retirement and a new object, which loses an Agent's session, until a `moved` construct exists.
   - File-against-live conflicts are detected for a whole object, coarser than Kubernetes' field managers: a control or an Agent's revise of any part of an object makes the next apply conflict on that object until `--force`.
   - Objects missing from the file are left alone unless `--prune`, so the file does not by itself describe everything that exists.

9. **No rollback (G9).** An apply can stop partway and leave a mixed state. It is reported step by step, and the author plans again.

10. **Restoring is still a new Conduit.** Under LIF-035 a restore in T7 is a new Conduit with a new ID. Only the local name and the tools are kept.

11. **`management` still cannot be narrowed to one instance.** It cannot be narrowed to one instance for `revise.v1` any more than for `register.v1` or `replace.v1` today (ACC-030). Narrow delegation still needs a broker (D04).

12. **Owner rulings that need amending:**
    - 8 (IN9) and 18 (V2): inline instances;
    - 21 (DR5, in part): revise;
    - 27: operation names written relative to the target's capability, or keep full addresses;
    - 31, 35 and 37: the frame stays for the long form, and a second normative form is added.

    Ruling 22 (DD2, "submitting only registers") and round 6's D11 stand: apply is a client composition of single-object requests.

13. **What this design does not fix.** The commands' failures behind the fresh author's 130 script lines and 112 recovery lines (4-12 to 4-15) are engine and command-line fixes; this design does not solve them. Its explicit step reports only address 4-13 in part.

## Composable
- **Targeted-operation designs.** `revise.v1`, `swap.v1` and the standalone `as` local name are the same primitives that an "operations on today's documents" design needs (4-01 option b), so they can be adopted first, with the project file layered on later.
- **Designs that keep the framed long form.** PRJ-011 makes the project file expand into, and render as, that form, so both can coexist in one project.
- **Package-exported operation sets.** P4: `operations: read` in place of a list. This is orthogonal and saves width, not lines. Any set must be expanded once and stored, and the plan must print any widening.
- **A default Pool and derived owner authority** (P7). PRJ-005, PRJ-006 and PRJ-007 are one way to do them. Do not also give root an ACC-033 exception over Pools: pick one.
- **Lock files written by `install`** (P8).
- **The views: render, plan, reach, why** (P9). A `why` command fits beside `reach`.
- **Conflict handling with field ownership** (P10). The whole-object check here can be refined to field managers later.
- **Explicit `moved` and opt-in prune** (P12).
- **Ruling 50's pending actions.** A plan submitted by an Agent can become a pending action at the root Interface, with the plan as its evidence.
- **Ruling 51's peer mode.** The file applies to one engine; a relationship across parties stays two halves, each declared in its own engine's file.
- **Not composable with:**
  - designs that change an open Conduit in place: that conflicts with swap and with G2;
  - designs that drop explicit operation lists;
  - a continuously reconciling controller.

## Verdict counts (major)
Holds with fixes. I recounted the authored files in the scratchpad by diffing each task state against the one before. The file-line counts are exactly as claimed: T1 19, T2 6, T3 0, T4 2, T5 1, T6 1, T7 1 (one line removed and then restored, counted once as the baseline counted its T7), 30 in all, against the baseline's 64. Each task does what it states on paper, and the person can request turns through the derived root-to-agent Conduits. The headline still rests on three things the design does not specify. (1) The "strict" count of 33 assumes that `apply` and `reach` take no value the author chose and that no `conduit init` is needed. Today's shell requires `--store DIR` on every command and `init` before anything else, and section E specifies no default store. Left as it is, the strict count is 42, less than the baseline only on T1, T2 and T4 (3 of 7). That fails "most". (2) After the T5 pin change, a deterministic expansion gives each Agent a different held filesystem pin (NAM-009, PKG-005), so its own revise step 3 restarts both Agents. The plan output does not show this, and PRJ-004's "stored, re-inferred only when the entry changes" contradicts PRJ-011. (3) The T2 posting convention and T3's Codex reach depend on provider statements that are either never shown to Agents or contradicted by finding 4-06. T5, T6 and T7 each beat the baseline by exactly 1 line, so any line the design forgot in one of them turns "less" into "equal". These are still paper counts, which ruling 45 does not accept as evidence.

RECOUNT (I diffed the files t1, t2, t4, t5, t6, t7-revoked and t7-restored in <scratch>/agent-centric/)
- T1: 19 non-blank lines (confirmed).
- T2: +6: the history pin, `history:`, its `type`, its `config`, and two Conduit lines (confirmed).
- T4: 2 changed (lines 19 and 24).
- T5: 1 (line 4).
- T6: 1 (line 11).
- T7: 1 line removed, then the identical line restored. That is 1 line in 2 edits, consistent with the baseline's "2 (4 edits)" rule. t7-restored.yaml is byte-identical to t6.yaml.
- File total: 30. With the proposal's own command count (one `install --from <lab>/packages` in each of T1, T2 and T5) it is 33. Both are confirmed as arithmetic.

Strict count if the CLI keeps today's contract
- reference-model/crates/shell/src/main.rs:106 says "--store DIR is required" for every command, and main.rs:107-108 requires `conduit init`.
- A store path is a value the author chooses. It adds a line only to commands that otherwise carry no value: `apply` and `reach`. `init` also becomes an author request.

| Task | Strict as proposed | Strict with --store and init | Baseline | Result with --store |
|---|---|---|---|---|
| T1 | 20 | 22 (19 + install + apply + init) | 33 | less |
| T2 | 7 | 8 | 16 | less |
| T3 | 0 | 1 (`reach`) | 0 | more by 1 |
| T4 | 2 | 3 | 8 | less |
| T5 | 2 | 3 | 3 | equal |
| T6 | 1 | 2 | 2 | equal |
| T7, file route | 1 | 3 | 2 | more by 1 |
| T7, command route | 2 | 2 | 2 | equal |
| Total | 33 | 42 | 64 | |

So with `--store` the result is less on 3 of 7 tasks, which fails "most". Section E must specify a store the commands find without a value (for example, recorded beside conduit.yaml at init). Otherwise the strict claim is unsupported. If an environment variable is set once instead, T1 gains 1 line and the rest holds.

Neither earlier run counted command values in its lines. The fresh author's run.py passes `--store` on every call, which was counted as adapter lines. So on the comparable "file lines" metric (30 against 64) the claim holds.

Requests not counted
- `conduit init` (the coordinator's log line 3, "T1 init and register run").
- T2 lists "s1.register.v1 and management.register.v1", and T5 "s1.revise.v1 + management.revise.v1". Each is counted as 1 operation request. That is valid only if ACC-033's row 15, like row 1, is one request authorized by two Conduits. The design should say so.
- Possibly 2 Agent revisions in T5; see below.

TASK COMPLETENESS
- T1: done. A has read and list; B has read, list and write. Turns go through the derived root-to-agent Conduits (`conversation.v1`, `turn.v1`).
- T2: done functionally; both post and read.
  - The claim that "the posting convention is shown from the type file-history.v1's description" is false today. IFC-033 takes a tool's description from the Operation contract. The history package exports only a Type, and server.rs sends no `instructions`.
  - The Agent Types' schemas (`additionalProperties: false`, with `accountServices`, `peerMessaging` and `model` only) leave the author nowhere to write the convention.
  - The baseline's 16 lines include 6 for the convention. The Conduit route conveys it only through uncounted turn prompts. Both Conduit runs share this gap.
- T3: covered by `reach`, but B's reach around the engine is taken from the Codex Type's description. That description says "other-Agent ... reach is disabled", which finding 4-06 contradicts (the `collaboration.*` tools). The package change that would state this is not in section D, so the reach output shown is not true of the shipped package.
- T4: done. Swap under the same local names, no interval without access.
- T5: done for `shared` (0.2.0 maps only the 3 operations, so `append_file` and `copy_file` are not presented), but with a hidden effect:
  - Each Agent's inline definition refers to `example.org/s1/filesystem` through its nested Conduits' capability.
  - Under NAM-009 and PKG-005 it holds that package "as its project's manifest gave them". After the pin changes to 0.2.0, the deterministic expansion (PRJ-011) differs for agent-a and agent-b.
  - Revise step 3 ("type, config, packages ... change → run stopped and restarted") then restarts both Agents, with 2 more requests and sessions at risk. This contradicts the claims of no restart and no session lost.
  - If instead pins stay as stored (PRJ-004's "stored, inferred again only when the entry changes"), the expansion depends on stored state, which contradicts PRJ-011. `render` would then not resubmit to the same objects.
  - The same surfaces at T6 or T7: the T7 revise of agent-a would restart it, where the baseline reached the running session.
  - Mixed versions follow too: `history` Conduits at filesystem 0.1.0 (the target holds it through NAM-014) beside `project` Conduits at 0.2.0, against NAM-006's "used at exactly one version".
  - The design needs an explicit rule, for example that a package referred to only through a nested Conduit's capability is not part of the Agent's held packages for restart.
- T6: done (config change, restart, swaps; the restart interval is acknowledged).
- T7: done. Revoke closes at once, B is unaffected, restore makes a new Conduit with the same local name and tools. Hidden restart risk as for T5.

MISSING OR INCONSISTENT DETAILS (no change to file lines)
- Section D changes both Agent packages (defaults for `accountServices` and `peerMessaging`). That means new archives and digests, so new versions, yet the file pins 0.1.1 and the plan prints the old digests 166ce271 and 05b47a85. If the packages stay as they are, it is +2 lines in T1 (already in the sensitivity table).
- The plan's root-to-s1 list leaves out `reconcile.v1`. `pool-management.v1` has 7 operations today including `reconcile`; with `revise.v1` it would have 8, not "s1.* (7)".
- "Duplicated configuration: 0" does not survive the baseline's rule, which counted repeated names such as `project` and `history`. The final file repeats `shared` (3 places), `history` (5 places) and `project` (2), and the list [read, list, write] 3 times: at least 3 duplicated values.
- "Coordinator root Conduits 70+ lines": the analysis this rests on gives 38 + 30 = 68.
- Concepts met (a plan measure) are not reported. The design adds the project file, the lock, plan and apply, revise, swap, derived Conduits, the default Pool, `operators`, `as` and provenance.
- How `install --from DIR` finds archives by address and version, and `reach` working while the engine serves (finding 4-09), are only described as tooling.

SENSITIVITY: the worst-case table's arithmetic checks out (T1 30, T2 11, total 48). Neither it nor the base case includes the `--store` and `init` issue or the T5 Agent restarts.

## Verdict safety (major)
As written, the design fails. Most of what breaks can be fixed. It keeps the stored shape: real Conduits, explicit operation lists, decisions at the target, digest pins. But revise.v1 and swap.v1, together with the derived Conduits, break G3, G4, G6, G12 and part of G2, and they quietly reverse three promises the design does not list: PLC-009, PRV-006 and the ACC-024 path. The worst cases are these. (1) The order of revise re-decides the Conduits that point at an instance only after its new code is already running, and there is no rollback. So a refused re-decision leaves the new code serving under the old grants. One concrete case: a non-root delegate replaces a secret store's code, and the Conduits that only root may open, those that read secrets, stay open to it. Another: a filter is silently dropped. (2) Every Pool and every Agent gets a root Conduit, and ACC-024 forbids closing a root Conduit, so swapping one is refused. Either every revise of an Agent's configuration fails after its restart, or an exception lets anyone swap away root's grants. (3) "Closed by request stays closed" has no stored state. Editing that line, or a purge of the records, quietly reopens a revoked grant. (4) Revise keeps an instance's identity while its code changes. A leaked credential, or an old run that never stopped, stays valid, and a peer engine's half of a relationship is never asked again. (5) The derived defaults depend on who runs apply and on history, so the same file can mean different things. The fixes are almost all on the engine and spec side. With them, S1 lands near the design's own worst case (strict 48), so the bar is probably still met on paper. Several fixes need owner rulings.

Repo read-only; promise text checked in spec. "Today" means the current spec.

MAJOR

M1. Revise re-decides the Conduits that point at an instance only after restarting it, with no rollback (G4, G5, G15, SEC-008, CON-022).
- Order in section B, step 3: (a) swap the Conduits the instance consumes; (b) stop the run and start the new revision; (c) swap the Conduits that target the instance. Parts begin only after earlier ones succeed (TPL-012). LIF-027 and G9 allow no rollback.
- Case A, secret store. Pool `vault` owns Resource `secrets`, a keychain-style store that consumes nothing. Resources `db` and `api` hold Conduits to it that list get-secret.v1, opened by root. Dana holds management revise.v1 and Pool revise.v1. She revises `secrets` to a package version whose implementation she chose (ACC-030 trusts her with that choice). Steps (a) and (b) succeed and her code runs. Step (c) is refused, because SEC-008 lets only root open Conduits listing get-secret.v1. The root-decided Conduits stay open to her code, and the engine's later secret reads for db and api (SEC-010, SEC-023) go to it. Today OBJ-011 and PKG-005 make this impossible.
- Case B, dropped filter. A Conduit with a filter points at a Resource. A revise moves it to 0.2.0, whose implementation is `plain: true`, as the S1 filesystem 0.2.0 package is. The new swap fails CON-022, which is checked only at instantiation, and ACC-015 does not re-check a Conduit's own filters on each request. So the old Conduit keeps dispatching to an implementation that is sent no filter.
- Case C, commit point. When the new definition is stored is unspecified. Suppose step (a) is refused (db's Conduit that reads secrets) but the definition was already stored. Then the next setup.v1 by root runs Dana's code under root's secret Conduit.
- Case D, race. A Conduit opened to the instance between the enumeration and step (c) is never re-decided.
- Fix: decide every part before any effect: each close and open, SEC-008, CON-022 against the new implementation, IFC-014 and IFC-033. Store the new definition only after that succeeds. Treat every Conduit that targets the instance as closed from the stop until its swap. Extend REV-012 so that no Conduit to or from the instance is opened or closed while the revise runs.

M2. Swap contradicts ACC-024.
- PRJ-006 and PRJ-007 give root a Conduit to every Pool and every Agent. Swap is authorized "as a close (ACC-012)", and ACC-012 refuses closing a root Conduit "whoever makes it".
- So step 3 cannot succeed for any Agent revise that changes type, config, packages, placement or runtime: a package bump from 0.1.1 to 0.1.2, or a ruling-48 toggle. It fails after the restart.
- If an exemption is added instead, any holder of access with close and open covering agent-a can swap root-to-agent-a for a Conduit that lists nothing, which bypasses ACC-024.
- Fix: state that root's Conduits are never re-decided (under ACC-022 the decision is empty anyway), and that only root may swap a root Conduit, and only for an identical one.

M3. "Closed by a request stays closed" has no stored state (G3, RD2, ruling 29).
- LIF-035 removes a Conduit when it closes, and CON-004 only sees that none exists.
- Case A: at 10:00 a responder runs close.v1 on agent-b/project. At 10:30 the author applies an edit to that same line (T4). The rule covers only "declaration unchanged", so step 2 or step 4 opens a new Conduit and no conflict is shown.
- Case B: root purges the records (REC-004). The only evidence of the close is gone, so the next apply reopens it.
- Case C: deleting the line and adding it back clears the mark without --reopen.
- Fix: keep a `closedBy: <request>` mark on the nested instantiation. It survives purges and edits. Any open under that local name needs --reopen and appears as a plan conflict.

M4. Revocation by the file depends on the revision (G3: revocation independent of both ends).
- Deleting a line becomes revise.v1, which is checked under REV-002. If a control (REV-003) or another actor changed the Agent between plan and apply, the result is conflict and the access stays open.
- Fix: apply first sends a plain close.v1 (ACC-012, no revision check), or a standalone narrowing swap, for every grant it removes or narrows. Only then does it send revise.

M5. Revise becomes a power over other teams' Agents (preservation test 8, ruling 51).
- Row 15 authorizes revise of an Agent by `management` alone, and `management` cannot be narrowed (ACC-030).
- A team delegate can then change another team's existing Agent: accountServices or peerMessaging true, `runtime` with no confinement, or `type`. Today OBJ-011 forbids this.
- An Agent with no Conduits, or only root's (re-decided by nothing), has nothing at all to re-decide.
- Fix: also require authority over the instance itself, or that of its registrant. Require root, or a new instance, for type, runtime and the fields that change reach outside Conduit.

M6. Promises reversed without being listed.
- Revise changes placement and runtime, but PLC-009 and PRV-006 say "no operation changes" them. A65's leased copies rely on PLC-009 ("a placement is fixed for life, so that slice is well defined").
- PRV-020 would let the old runtime provider be retired while the old run's stop is still unknown.
- Nothing excludes first-start objects (FST-006) or nodes (PLC-008) from revise.
- Fix: limit revise to config and to the package version within one address. A change of type, runtime or placement makes a new instance. Exclude objects the engine makes.

M7. An instance keeps its identity while its code changes (G12; DL13, IFC-032).
- Credentials name `instance/<id>`, and IFC-032 cuts off a stale run only by removing the instance.
- After a T5-style security upgrade, a credential stolen from the vulnerable 2.1.0 run, or a run whose stop is unknown (PRV-007 warns that two runs can present as one), still authenticates.
- A peer engine's half (ruling 51, decision 4) keeps admitting agent-a after its type or config changed.
- Fix: bind credentials to the revision, rotate them at restart, and refuse those of earlier revisions. Make the revision visible to peers so their half is decided again, or refuse revise for instances that have halves across parties.

M8. Grants made for whoever runs apply, and meaning that depends on who that is (G2, G6, G8; PRJ-011 is false).
- `operators` defaults to "the registering Interface". So a CI or delegate Interface gets turn.v1, which starts model work, on every Agent, and every pool-management operation.
- The result depends on who runs apply and on "the core version in force".
- An applier with a bounded access Conduit cannot cover a new Pool or Agent: pool and target references resolve to IDs (core references.yaml). So its apply fails partway and leaves objects registered with no administration (preservation test 5).
- An Agent package that adds a capability yields a new Conduit listing every operation of it for non-root operators. That is the T5 hole again.
- Fix: derive these Conduits for root only. Make non-root operators explicit, with their operations. Record the expanded lists in the lock. Refuse, before any step, an apply that cannot open the derived Conduits.

M9. Default drift is the T5 hole moved to reach outside Conduit; ruling 49 is bypassed.
- Section D gives ruling 48's fields defaults. NAM-024 lets a later release add optional Type properties that have defaults. On a pin bump, re-expansion fills them in for every Agent of that type, while the plan says "tools unchanged".
- The `account` that ruling 49 requires is missing from the file.
- Revise keeps the Agent's home, which is "made and removed with" the Agent. A changed expected account therefore keeps the old login, and the check that runs "after a sign-in" never runs.
- Fix: keep the fields that change reach required, or pin resolved defaults per instance in the lock and show any change as a widening. Make `account` required. When it changes, clear the login and check the account at every run start.

MEDIUM

D1. Forged provenance. `source` (file digest, line, rule) comes from the client in register and revise, and the engine cannot check it. The plan finds the project's objects "by source". An actor with management register.v1 can plant a Conduit claiming `agents.agent-a.conduits.project`; the plan shows "=" and `reach` shows the claim. Fix: the engine writes only what it can check (registrant, request, revision). The plan matches objects by IDs kept in the lock and flags any other registrant.

D2. A swap rebuilds the target from the file text, so any edit of the line resolves the name again (OBJ-010, G1). An external `{kind, name}` whose name was reused moves the grant silently, and the plan shows the same name. Fix: keep the held resolved ID unless `target` itself changed; print IDs and flag when a name resolves to another object.

D3. The inferred capability depends on history. It is "stored, inferred again only when the entry changes", yet it depends on "the version the target holds". So an engine with history and a fresh engine disagree, the claimed round trip through `render` fails, and reformatting a line can re-infer the capability or make the entry ambiguous. Fix: record the inferred capability and the operation addresses in the lock, infer again on every plan, and show the difference.

D4. Work in flight.
- ACC-034 applies to every swap, identical or wider: onward requests are refused and tasks/get results are not delivered.
- A restart kills invocations that ACC-034 says run to their end.
- OPS-008 scopes keys by caller, target and sender, not revision. So a write with an unknown outcome made before T6 is reconciled against shared-2, the wrong place (G9).
- Fix: carry in-flight invocations to the new Conduit, as OPS-024 does for a latest attempt. Drain the run, or report each invocation unknown with its revision. Put the target's revision into the key scope and into reconciliation.

D5. Audit. No promise keeps earlier definitions, and revisions are opaque (REV-001). If a restart ends unknown, REC-002 would credit the new revision while the old run is still serving. Fix: the revise record holds the full definition before and after, and REC-002 records the revision of the run that served.

D6. Removal is ambiguous.
- Deleting a nested line revokes, but deleting an Agent block leaves it fully granted unless --prune is given.
- Adding `pools:` drops the default Pool from the expansion, and --prune would then retire Resources that are still in the file.
- Fix: a removed entry is a plan conflict that needs --prune or a restore. Record the default Pool in the lock.

D7. `{target: access, operations: [open.v1]}` with no requirements is unbounded (ACC-029): the short form defaults to every target. Fix: require an explicit `bound` for any Conduit to `access`, and have the plan flag rights to open and grants on `management`.

D8. Two checkouts applying as root: one silently reverts the other's change, because the plan reads the revision at plan time. Fix: keep the last applied revision per object in the lock, and report conflict unless --force.

D9. The lock trusts on first use, and its fetch hints are local paths.
- Whether `install --from` checks existing lock entries is unspecified; local `file:` fetch hints break on other machines (G16).
- The lock lacks transitive pins: history 0.1.0 needs filesystem 0.1.0.
- Fix: lock entries are authoritative and change only by explicit request; record transitive pins.

D10. T4 runs per object, widening before narrowing, so both Agents can write for a moment, and a partial apply leaves it so. Fix: narrow first, or let a swap act on a set of Conduits (A65).

MINOR
- The T1 plan's root-to-s1 leaves out reconcile.v1, which pool-management.v1 lists; "(7)" should read 8. ACC-033 row 13 then has no route to reconcile a setup.
- IFC-014 counts the old, not-yet-closed Conduit, so the new Conduit of a swap clashes with it. ACC-033 row 10 needs to name swap.
- During T7's window, the local name is free for a squatter's Conduit with `as: project`, which then blocks the restore. Fix: reserve dropped local names.
- On existing engines root can never gain swap.v1 (ACC-030, FST).
- "Tools unchanged" compares names only. It should compare definitions (IFC-033, NAM-024 optional inputs).
- The T7 refusal names Resource `shared` after the declaration was removed. The second tier's "closed" Conduits are gone (LIF-035).

## Verdict cost (major)
The design can be built, but it holds only with fixes, and its own cost section (H) understates the work about threefold. The cost is large overall. About 25 promises are new (12 PRJ, the rest implied but not written) and about 55 existing promises change, against the 17 the design names. That is roughly 80 of 455 promises, or 18%. The core grows by 45 to 65%, and the shell gains about 1.5 to 2.5k lines. Migration is mostly additive. Every current long-form document, manifest and vector stays valid except about 5 first-start cases, and existing stores need a clean break. Six promises the design calls unchanged, or never mentions, must change, and some of these carry safety:
1. **IFC-014.** It rejects the successor Conduit a swap must register under the same local name.
2. **ACC-024.** It refuses the swap of root's own Conduits to an Agent or Pool whose code changes. Combined with "parts authorized each as their own operation" and no rollback, this is a G4 hole unless revise pre-authorizes every part.
3. **PLC-009 and PLC-013.** Placement never changes, yet revise lists placement among what it can change.
4. **SEC-014.** "No operation ends the run of a ready instance."
5. **SUB-008.** The contracts digest is taken over the manifest as handed over, so every edit to a grant line would change a project's contract identity.
6. **LIF-031 with OBJ-010.** Revise's `definition` cannot carry resolved IDs, so unchanged references would be resolved by name again.

The design is also missing a list operation for `plan`, which preservation test 5 needs. It also needs a rule for version drift: as written, T7's revise of agent-a (after T5 pinned filesystem 0.2.0) would see a package change, restart the Agent and swap root's turn Conduit, which ACC-024 refuses. Reading cost goes up for implementers: about 16 concepts are added, none are removed, two normative forms exist, and the spec grows by about 9% in words. It goes down for authors on S1's path. Each defect has a contained fix, so the severity is major, not fatal.

Everything below was read in <home> nothing there was modified. Promise texts were read with a lookup over spec/**/*.yaml (470 IDs; 455 if the DL items are excluded). Case-to-promise counts come from conformance/manifest.yaml (367 cases).

## 1. Size by area

| Area | Size | Work |
|---|---|---|
| spec promises, conventions, interface | **L** | ~25 new; ~55 amended (17 the design names, ~38 it does not) |
| spec/definitions.md (about 261 terms) | M | ~15 terms added, ~10 amended (instance "Every instance comes from a template", template, nested instance, name "fixed", replace/replacement, register, project, submission, document, ID); none removed |
| spec/schemas | M | ~5 of 8 files, +250 to 400 lines |
| spec/core | M | 2 or 3 new operations; 3 capabilities, 2 templates, inspect.v1 and record.v1 changed; core version 0.1.0 -> 0.2.0 |
| spec/schemas/examples (114) | S-M | +10 to 20 examples; none invalidated (checked: NAM-006 digest-less object item, INS-001, POOL-001 stay invalid) |
| tools/check_schemas.py (2,588) | M | +300 to 500 Python lines (project-file schema, PRJ rule checks, example verdicts); the loader tests compare against its verdicts, so both move together |
| loader (2,879 + 2,144 test lines) | M | +600 to 1,000 lines. project.rs (lock file, project-file manifest role), schemas.rs, rules.rs. Line and column positions already exist in diagnostic.rs, so PRJ-009 and SUB-005 line attribution is feasible |
| core (10,465) | **L** | +4,500 to 7,000 lines (+45 to 65%); breakdown in section 6 |
| shell (6,986 + 3,297 test lines) | **L** relative to author.rs (284) | +1,500 to 2,500 lines |
| conformance runner (2,137 + 604) | S-M | New input kinds (expand, plan, revise, swap). Records and objects are compared by required-field projection, so new record fields do not break existing cases |
| conformance vectors | **L** for new cases, **S** for invalidated ones | New: ~60 to 90 cases. Invalidated: ~5 case.yaml files plus 12 snapshots to regenerate |
| packages/agent-* | S | `default: false` on accountServices and peerMessaging; repack; new digests (provider work) |
| owner rulings | — | Amend rulings 8 (IN9), 18 (V2), 21 (DR5) and 27, and the frame rulings 31, 35 and 37, as the design says. Also the owners of PLC-009, SEC-014 and ACC-024. Round 6's D09 and A67 apply: the restart verb must come to the owner "with exact identity, in-flight invocation, hook, secret-read and stop semantics". Two-tier refusals (OPS-023) are ruled (ruling 52) but not yet written into spec, and the design depends on them |

## 2. Promises

### 2a. New (~25)
- **PRJ-001 to PRJ-012:** 12.
- **Implied by the design but not written (~9 to 11):**
  - R1: revise's parts and their order.
  - R2: revise keeps the ID and name, advances the revision, and REV-002 and REV-011 apply.
  - R3: a nested Conduit closed by request reports conflict; `reopen`.
  - R4: restart semantics. Stop, then run at the new revision, then the setup hook. It must also say whether the retire hook runs, how secrets are read again, what an unknown stop does, and what happens to in-flight work (D09).
  - R5: every part of a revise is authorized before any part is dispatched. Otherwise a refused swap after a restart leaves an old grant over new code, which breaks G4.
  - R6: revise outcomes and no rollback, or fold this into OPS-022 and LIF-027.
  - R7: how references in `definition` are treated. Unchanged parts keep their resolved IDs; see LIF-031 and OBJ-010.
  - R8: version drift (section 4, point 3).
  - R9: first-start objects cannot be revised (LIF-034).
  - S1: the swap is atomic.
  - S2: swap authorization, including root-consumed Conduits (ACC-024), SEC-008 and ACC-030.
  - S3: a swap ID in records.
- **Missing from the design:**
  - L1: an operation that lists the project's objects (by `source`). `plan`'s "objects not in the file", `--prune` and conflict detection need it. The core has none: the operations are register, replace, unregister, inspect, set-up, retire, reconcile, register-replacement, install, declare-lost, purge, retire-with-resources, open, close and the provider operations. Today author.rs reads state.json directly, which only works because the shell holds the store. Preservation test 5, and peer mode under ruling 51, need it as an operation, with an ACC-033 row.
  - Unavailability during a restart. CON-024 gives unavailable only for a target that is not ready. During a revise restart the target stays ready, so requests would be dispatched to a stopped run. The design claims "unavailable", which no promise gives.

### 2b. Amended, as the design names them (17)
MAN-003, MAN-004, MAN-005, INS-001, INS-002, OBJ-003, OBJ-011, OBJ-013 (`as`), NAM-006, NAM-011, PKG-005, INS-020, LIF-009, REC-001, REC-002, ACC-033 (row 15), IFC-012. ACC-011 also needs a wording edit.

### 2c. Amended, missed by the design (~38; a narrower revise could avoid some)

**Project-file side (15):**
- MAN-001: the lock is "part of the manifest" but is a second file; MAN-001 allows exactly one.
- MAN-011: the lock file; package manifests may not hold instance sections, which needs text.
- SUB-005: diagnostics locate by JSON Pointer in a logical location; file lines must be added.
- **SUB-008:** the contracts digest is RFC 8785 over "the project's manifest as it was handed over". With instances in the manifest, every grant edit changes the contract identity of any project that has contracts. The instance sections must be excluded from the hash.
- SUB-010: a directory read must include the lock.
- PKG-006: the digest comes from the lock.
- NAM-007: relative operation names are qualified through the capability's package. For history that is example.org/s1/filesystem, reached through history's manifest, so a project that does not list filesystem would fail NAM-007.
- NAM-009 and NAM-010: packages held by anonymous templates and by a revise. NAM-010's observable ("two instances ... at different times hold the same versions") no longer holds for inline definitions.
- NAM-027: written about documents; derived names need adding.
- OBJ-017 and OBJ-018: `apiVersion` once per file; the name comes from the key.
- INS-013: its observable says "nothing that says which values came from arguments", which conflicts with PRJ-010's marked defaults.
- INS-025: `definition` as the alternative to `from`.
- CON-016: `capability` is required, but here it is inferred.

**Revise and swap side (~23):**
- **IFC-014.** The design calls it unchanged, but it rejects instantiating a Conduit whose tool names equal those of the consumer's other Conduits "that are not closed, whether or not they are open yet". A swap successor under the same local name is exactly that.
- CON-004 and LIF-008: they assume at most one live instance per nested instantiation, while a swap briefly makes two.
- **ACC-024 and ACC-012.** A revise that changes an Agent's or Pool's type, config, packages or runtime must re-decide every Conduit that targets it, including root-to-agent-a and root-to-s1. Their consumer is root, so closing them "reports refused, whoever makes it". Either exempt a swap to an identical successor, or exempt root-consumed Conduits from re-decision. The second is sound, since ACC-022 means root's re-decision always passes.
- ACC-005, ACC-006, ACC-030 (swap of Conduits to `access`) and ACC-034 (onward requests of in-flight work are refused after a swap even when the operations are unchanged).
- **PLC-009** ("No operation changes the placement of an existing instance") and **PLC-013** ("decided once ... never checked again"). The alternative is to drop placement from revise.
- **SEC-014** ("No operation ends the run of an instance that is ready") and SEC-013 (connection values are held "until a later setup's connection").
- TPL-009 and TPL-018: the stop path outside retirement, and whether the retire hook runs.
- TPL-012, PRV-007 and PRV-008: run and stop outside setup and retirement.
- OPS-022: outcome rules for revise's parts. OPS-023: tier 2 now includes `source`.
- CON-024: as above.
- REV-012 and REV-014: a revise changes other objects' Conduits, so concurrency must hold across objects.
- REV-009 and REV-010: a control of unknown outcome against a revise, since REV-010 applies values to "the current configuration".
- **LIF-031 with OBJ-010.** An actor's request may not carry resolved IDs. So `definition` names references, and a naive revise resolves unchanged references by name again. If the name was reused after LIF-035 removed its object, the target is silently redirected: exactly what OBJ-010 forbids.
- LIF-034 and LIF-027.

**Claims of "unchanged" that are wrong:** IFC-014, LIF-008, ACC-024 (implicitly), and "Existing long-form vectors survive, except those that assert PKG-005, OBJ-011 and LIF-009" (section 5 shows those survive and others break).

## 3. Schemas and core operations

**Schemas:**
- manifest.yaml: either `manifest.v1` gains `project`, the instance sections and bare-version items, or a new `project-file.v1` is added.
- instances.yaml:
  - `instance.v1`: `definition` as the alternative to `from`, a `source` field and the `as` argument;
  - `declared-conduit.v1` and `pool-entry.v1`: inline forms;
  - the short Conduit entry `{target, operations, requirements, capability?}`.
- templates.yaml: shared definitions for inline and anonymous templates.
- envelope.yaml: metadata rules for derived names and for `source`.
- values.yaml: lock-file schema, if specified.
- core/results.yaml: `record.v1` gains a swap ID and the consumer's and target's revisions.

**Core package, spec/core:**
- operations.yaml: add `revise.v1` and `swap.v1`, plus `list.v1` or an inspect selector (L1). `inspect.v1`'s outputSchema gains `source` and default markers; `register.v1`'s description changes.
- capabilities.yaml:
  - `object-management.v1` gains revise.v1;
  - `pool-management.v1` gains revise.v1, which makes 8 operations;
  - `access-control.v1` gains swap.v1.
  - These additions are allowed by NAM-024.
- templates.yaml: `root-management` lists 12 operations and `root-access` lists 4.
- conduit.yaml: core version 0.2.0.

**Minor error in the design's illustrative plan:** PRJ-006's Conduit to the Pool must list every operation of pool-management.v1. The plan shows 7 operations ("s1.* (7)") and omits `reconcile.v1`; with revise.v1 there are 8.

## 4. Conformance

**Invalidated (expectations become false):** the four case.yaml files that assert root's Conduit operation arrays exactly (arrays are compared exactly; maps by projection):
- conventions/first-start/initial
- conventions/first-start/repeat
- conventions/first-start/lifecycle
- language/first-start/protected

Plus conventions/s1/whole, through FST-002.

**Regenerate for consistency (12 snapshot files):**
- conformance/fixtures/root-system-state.json, used by 197 cases;
- 11 initial.json snapshots (registration/placement-retired, runtime-retired, entry-active-replacement, owner-retired, entry-retired-only; records/initial; access/fixed-kind-references, entry-authority, collision-missing-contract, capability-skew; records/purge-nonroot/requests.json).

**Survive (checked):**
- registration-existing-name (OBJ-002 stays);
- registration-frozen-template, setup-restore-current-template, setup-review-replacement-reference, records-replace and change-t4: LIF-009 changes only for inline definitions, and revise is never called;
- change-t5 and archives-registered-project-holder (PKG-005);
- every IFC-012 tool-name case, since `as` defaults to the Conduit's own name;
- every record assertion, since new fields are projection-safe.

**New (~60 to 90 cases):**
- PRJ (~25 to 35):
  - capability inference: unique, ambiguous and none;
  - default Pool;
  - the derived Conduits of PRJ-006 and PRJ-007, and `operators: []`;
  - lock missing or mismatched;
  - a clash with a derived name;
  - `source`;
  - golden round-trip vectors for PRJ-011: (file, lock, installed) gives the expected long form. These are the cheapest way to keep independent builds (A34) identical.
- Revise (~20 to 30): each part; stale revision; a closed nested Conduit gives conflict, and `reopen`; restart order; unknown stop; a non-root revise of a secret-reading Resource is refused; pre-authorization; root-consumed swap; version drift; first-start objects refused.
- Swap (~10 to 15): atomicity (no request decided under neither or both); auth; ACC-024; SEC-008; ACC-030; ACC-034 in-flight; IFC-014 successor; records.
- L1 (~3 to 5).

**Version drift (R8): a concrete failure in the design's own S1 run.**
- The history Type offers `example.org/s1/filesystem#directory.v1`, through history's own pin of filesystem 0.1.0.
- After T5 the file pins filesystem 0.2.0, while agent-a still holds 0.1.0 for its inline history and project Conduit definitions.
- At T7, revise.v1 for agent-a is resolved against the current manifest, as replace.v1 is under SUB-009, so agent-a's held packages change.
- Revise step 3 then restarts agent-a, losing its session, and swaps root-to-agent-a, which ACC-024 refuses.
- The design's T7 ("tools lost project.* (3)", no restart) and its T5 ("= Conduits */history: unchanged") need a rule that the packages reached only through nested Conduits are held per Conduit, and that an unchanged nested Conduit keeps its held version.
- PRJ-011 must then say that `render` reproduces the held state, not the file's fresh expansion.

## 5. Reference model rebuild

- **Core:**
  - new expansion.rs, pure, citing PRJ-*: inference, defaults, derivation, naming, `source`, canonical render. About 1,500 to 2,500 lines.
  - new plan/diff module: about 800 to 1,500.
  - registration.rs: `definition` instances, `as`, `source`, OBJ-003 naming, and the IFC-014 exception around lines 1,906 to 1,935. Medium.
  - packages.rs: bare versions plus the lock, and per-Conduit held packages. Small to medium.
  - lifecycle.rs and retirement.rs: a stop/run restart path outside retirement (the stop logic is today only in retirement.rs around line 270). Medium.
  - access.rs: ACC-033 row 15, the atomic swap decision, the ACC-024 exception, `as` in tool naming. Medium.
  - records.rs: revise and swap events, swap ID, revisions. Small.
  - requests.rs: compound outcome, pre-authorization. Medium.
  - lib.rs: new Input variants and first start with the extra operations. Small.
  - Stored objects already carry `local_name`, so `as` is cheap.
- **Shell:** `install --from` writing the lock; plan, apply, render, reach, close and open in author.rs and main.rs. The run restart for one instance ID in runner.rs (1,514 lines) is a medium change. store.rs: small. tests/changes.rs (9 tests, 851 lines) survives and gets new tests.
- **Loader:** see section 1.
- **Conformance runner:** see section 1.
- **Unaffected:** archives, identity and gates, strict parsing, idempotency.

## 6. Migration

- **Documents: backward compatible.** Everything stays valid: the long form, the object-form `packages` items with a digest, standalone Conduits, `replace.v1` and `register-replacement.v1`. The project file is additive.
- **Guarantees: not compatible.** revise.v1 applies to long-form instances too, so the OBJ-011, INS-020 and PKG-005 immutability (DR5) is gone for every instance, not only for project-file ones.
- **Stores: clean break.** First-start objects are permanent (ruling 30), and root's two Conduits are unclosable (ACC-024). An existing store therefore cannot gain revise.v1 or swap.v1 for root without a new migration promise, or a swap exception that applies to root itself.
  - The core package moves to 0.2.0; instances holding core 0.1.0 under NAM-009 have no defined meaning.
  - Discarding stores is cheap in v0: no users, persistence is listed as free to simplify, and the coordinator already discarded stores.
- **Vectors:** about 99% survive.

## 7. Reading cost

**For implementers, more complex.** Concepts added (~16):
- the project file and its entries;
- inline and anonymous definitions;
- capability inference;
- relative operation names;
- the default Pool;
- derived Conduits and `operators`;
- the lock;
- `source` provenance;
- visible defaults;
- canonical render and round-trip;
- the `as` local name;
- revise, with parts, reopen and conflict;
- swap and its successor;
- restart at a revision;
- the version-drift rule;
- the list operation.

Concepts removed: 0. The long form, templates, standalone Conduits, `register-replacement.v1` and `replace.v1` all stay, and there are now two normative forms. Spec words grow about 9% (promise text is about 118k words; ~25 new promises at ~300 words plus ~55 amendments at ~80 words, about 11k words).

**For authors on S1's path, much simpler.** About 8 to 10 concepts are no longer needed: frame, template/instance split, inputs, root's Pool and turn Conduits, owner, digests, standalone-Conduit tool names. Refusals and `render` still expose the long form, though.

## 8. Ways to cut cost (for the owner, not decided)

1. **Phase it:**
   - A: project file, lock, derived root Conduits, `as`, plan/apply for registration only. Gives T1 to T3.
   - B: swap.v1, plus revise limited to nested-Conduit changes with no restart. Gives T4 and T7, and avoids PLC, SEC, TPL and PRV changes.
   - C: restart, for T5 and T6. This is D09 territory and the riskiest part.
2. **For T5 and T6, replace instead of restart.** Register a new Resource for the entry under a generated name, swap the consumers' Conduits, then retire the old one. This is make-before-break with no interval of unavailability, and it avoids amending PLC-009, PLC-013, SEC-013, SEC-014, TPL-009, TPL-018, PRV-007, PRV-008 and CON-024, and R4. The costs: the entry key no longer equals the object name (identity carried in `source`), and IDs change at T5 and T6. File lines are unchanged (1 per task).
3. **Expand on the client.** Specify the expansion as a pure language-layer function, and have register.v1 and revise.v1 take only the long form plus `source`, marked as asserted by the requester. If register.v1 instead accepts a project-file manifest, it must handle entries that already exist (OBJ-002), a much larger change.

## Key files
- spec/interface/implementation.yaml (IFC-012, IFC-014)
- spec/conventions/access.yaml (ACC-024, ACC-033)
- spec/promises/placement.yaml (PLC-009)
- spec/promises/secrets.yaml (SEC-014)
- spec/promises/submissions.yaml (SUB-008)
- spec/conventions/lifecycle.yaml (LIF-031)
- spec/core/operations.yaml, capabilities.yaml, templates.yaml
- conformance/fixtures/root-system-state.json
- conformance/fixtures/s1/packages/history/contracts.yaml (offers filesystem#directory.v1)
- reference-model/crates/shell/src/author.rs (reads state.json directly; no list operation)
- docs/proposals/reaudit/round-6/design-critique.md (D09)