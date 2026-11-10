Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Conservative: same objects, written once, changed by re-registering under the same name

This design keeps the object model as it is: the five families, templates and instances, Conduits as immutable objects, fixed names, resolved IDs, close-then-open, explicit operation lists and digest pins. It removes only cost that protects nothing. The changes:
- The frame is stated once. A file can list objects in sections such as `resources:` and `agents:`, and `apiVersion` comes from the manifest.
- Packages export the templates authors keep rewriting.
- The manifest is written by `conduit install --save`.
- The engine gives root a visible Conduit to each Pool and Agent it registers.
- Resubmitting an unchanged document is accepted and does nothing.

Change still means making new objects. A short-lived name is freed when its object is closed or retired, so a new object can be registered under the same name. Tool names therefore stay the same. A client-side `conduit apply` previews and then runs the explicit close, retire, register, set-up and open requests, and never reopens a grant someone else closed. On paper S1 costs 34 lines in files (41 counting the commands that carry an author's value) against the baseline's 64. That is less than the baseline on 5 tasks (6 counting file lines only), the same on the others, and much more on none. Nothing here has been run.

## Spec changes
CHANGED PROMISES (wording drafts)
- OBJ-017. A document carries `apiVersion`, or takes the one from its project's manifest (for a package, its `conduit.yaml`) when it gives none. A document with neither is rejected with a diagnostic naming it.
  - The manifest promise that requires the manifest's own `apiVersion` (spec/promises/manifest.yaml, near line 139) is reworded to match.
  - OBJ-024's observable: the version is known per file or per manifest.
- OBJ-002 (and OBJ-025, SUB-007 observables). Today a document whose kind and name match an existing object is always rejected. New rule: where the document, resolved now, equals that object as stored, it is accepted, registers nothing and is reported `unchanged`.
  - Equality is SUB-009's test: SCH-007 equality, the same template reference, the same resolved references (IDs) and the same package address, version and digest.
  - Any other match is still rejected with conflict. The diagnostic names each field that differs (ruling 52).
  - "A submission only adds objects and never changes one" stands unchanged.
- SUB-006. "Its templates and instances are registered, and the owner Conduits of ACC-044 instantiated. Nothing else happens."
- register.v1's output (spec/core/operations.yaml) lists, for each document, `registered` or `unchanged`. For each derived Conduit it lists the Conduit and the rule that made it.
- Observables extended so that they also cover an entry of a sectioned file:
  - OBJ-012, OBJ-013, OBJ-020;
  - SUB-005 (the diagnostic gives the file and a JSON Pointer into the file as written);
  - MAN-011 (a documents file may be sectioned);
  - NAM-027 (names stay unique across all sections and documents).
- FST-005 lists the two new core templates.

NEW PROMISES
- OBJ-029, sectioned file (language, must).
  - Form: a file of documents may instead hold one YAML or JSON mapping. Its keys are among `pools`, `resources`, `agents`, `conduits`, `interfaces` and optionally `apiVersion`. Each section maps names to entries.
  - Reading: an entry `N: B` under section S is read as the document `{apiVersion (file or manifest), kind: <family of S>, metadata: {name: N, + B.metadata}, ...B}`.
  - An entry may not hold `apiVersion`, `kind` or `metadata.name`.
  - Contracts are not written in sections.
  - INS-001's marker is unchanged: an entry with `from` is an instance, one with `spec` a template.
  - Every other promise applies to the document as read. The framed document stays the canonical form that `register.v1` carries and inspection shows.
  - Precedent: Compose `services:` and Terraform blocks.
- ACC-044, owner Conduits (convention, must).
  - When a registration requested by the root Interface makes an instance whose Type lists `ownerConduits`, the registration also instantiates one Conduit from each listed template. Its consumer is `root`, its target the instance, and its state registered.
  - Each is named after the instance, or `<instance>-<template>` when the Type lists several. A name collision rejects the registration under OBJ-002.
  - Each is listed in the result with the rule.
  - Each is opened only by an explicit `open.v1`, which ACC-022 permits.
  - Each is unclosable under ACC-024, as every root Conduit already is.
  - None is ever made again for an existing instance: no reconciler, test 6.
  - Every power is still a Conduit, so ACC-033 is unchanged: setting up `shared` is authorized by root's Conduit to `workspace`, exactly as row 4 says.
  - Precedent: the Kubernetes bootstrap bindings; the creator of an Azure resource group holds authority over it.
- PKG-030, exported Conduit templates keep their grant (language, must). Installing a package version is rejected when it exports a Conduit template under a name that another installed version of the same address exports with a different capability or operations, compared by export name. This keeps G6 when access levels come from packages: an upgrade cannot widen `read-write`.

CORE PACKAGE AND SCHEMAS
- Core gains a Pool template `pool` (`type: pool.v1`, `config: {}`).
- Core gains a Conduit template `pool-owner` (`pool-management.v1`, all 7 operations).
- `pool.v1` gains `ownerConduits: ['#pool-owner']`.
- The Type contract schema (spec/schemas/contracts.yaml, CTR) gains an optional `ownerConduits`. It lists Conduit templates of the same package whose capability the Type offers, checked at install.
- spec/schemas/document.yaml gains the sectioned-file schema.
- tools/check_schemas.py and the loader (authored.rs, project.rs) read it.
- New vectors for OBJ-029, ACC-044, PKG-030 and the equal-document case of OBJ-002. The `existing-name` and `fixed-kind-references` cases are revisited.

PROVIDER WORK (not spec; counted apart)
- filesystem 0.1.1 and 0.2.1 export `local-directory` (input `root`), `read` (read and list) and `read-write` (read, list and write).
- history 0.1.1 exports `history` and `post-read`.
- agent-claude-code and agent-codex 0.1.2 export the Agent templates `claude-code` and `codex`. Their inputs `accountServices` and `peerMessaging` default to false, as ruling 48 has it.
- Each Agent package also exports a Conduit template `turn`, and its Type lists `ownerConduits: ['#turn']`.

TOOLING (reference shell only; no spec)
- `conduit install --save ARCHIVE#SHA256`: installs, then writes or updates the pin in `conduit.yaml`, as `cargo add` does. The author still copies the published digest into the command.
- `conduit apply [--plan] [--reopen KIND/NAME] [--replace Agent/NAME]`. It does nothing a person or an Agent could not do with ordinary operations (test 5). It holds no state, prunes nothing and rolls nothing back. Steps:
  1. It classifies each entry against the stored state, using the same pure core: unchanged, new or differs. It prints the plan: each request, which tools each Agent gains or loses, sessions kept or lost, and operation lists before and after. It then asks y/N.
  2. For a changed Conduit it closes it. For a changed Resource or Pool it retires it, with LIF-037's `expectedId`, which catches a stale plan.
  3. It registers the whole project in one `register.v1`. Unchanged entries are no-ops under OBJ-002 as changed, so a concurrent change is rejected all or nothing.
  4. It opens root's new Conduits and sets up new instances.
  5. It opens a newly registered grant only in two cases: no Conduit of that name and consumer ever existed (from the records), or this run closed it.
  - A grant closed outside this run is registered but never opened unless `--reopen` names it. That is G3; the report gives who closed it and when (CON-008 record).
  - A change that would retire an Agent needs `--replace`.
  - It stops at the first part that does not succeed and reports every part (ruling 52, 4-13).
  - It follows Terraform's plan and apply, run only on an explicit request.
- `conduit close KIND/NAME` and `conduit render` (prints the canonical framed documents, as `docker compose config` does).

UNCHANGED
- OBJ-009, OBJ-010, OBJ-011 (no document rewrites an instance; names fixed).
- INS-001, INS-002, INS-013, INS-020, INS-025, IN9 (no inline templates).
- LIF-008, LIF-009, LIF-035 (a closed Conduit is removed; restoring it makes a new one).
- ACC-011 (narrowing is close plus open), ACC-022, ACC-024, ACC-033.
- IFC-012 and IFC-014 (tool name = Conduit name).
- FST-001 to FST-003; NAM-006, NAM-009, NAM-010; PKG-004, PKG-005; CON-004, CON-012; REV-*.
- Records, secrets and outcomes.

CONSIDERED, NOT ADOPTED (S1 does not need them): inline templates (INS-025 would get a fifth form), an `as:` local-name argument, capability inference, a default Pool, an atomic swap, upgrade in place.

RULINGS THE OWNER WOULD AMEND: ruling 37 (the frame becomes optional; the canonical form is kept), ruling 35's wording (`apiVersion` per document), and ruling 22/DD2 in letter only (an equal document registers nothing).

## Rewrite
Conventions:
- Commands run in `<lab>/project` while `conduit serve` holds the store. `conduit init` and `conduit serve` are harness steps and are not counted.
- `apply` prints its plan and asks y/N. The answer is one keystroke and is not counted as a line.
- Every checking command is a check (`conduit tools --as`, `conduit call <agent>.turn.v1`) and is not counted.
- The packages are rebuilt with the templates added, so their versions are bumped and their digests are placeholders.

T1. File `project.yaml`, authored, 19 lines:
```yaml
pools:
  workspace: {from: github.com/shrav-k/conduit/core#pool}
resources:
  shared:
    from: example.org/s1/filesystem#local-directory
    owner: workspace
    arguments: {root: <lab>/shared}
agents:
  agent-a: {from: example.org/conduit/agent-claude-code#claude-code}
  agent-b: {from: example.org/conduit/agent-codex#codex}
conduits:
  a-project:
    from: example.org/s1/filesystem#read
    consumer: {kind: Agent, name: agent-a}
    target: {kind: Resource, name: shared}
  b-project:
    from: example.org/s1/filesystem#read-write
    consumer: {kind: Agent, name: agent-b}
    target: {kind: Resource, name: shared}
```
T1. Commands: 3 carry a value (each has a copied digest), plus `apply`:
```sh
conduit install --save <lab>/packages/filesystem-0.1.1.tar.gz#<sha256 of filesystem-0.1.1.tar.gz>
conduit install --save <lab>/packages/agent-claude-code-0.1.2.tar.gz#<sha256 of agent-claude-code-0.1.2.tar.gz>
conduit install --save <lab>/packages/agent-codex-0.1.2.tar.gz#<sha256 of agent-codex-0.1.2.tar.gz>
conduit apply
```
T1. `conduit.yaml`, written by `install --save`, not authored:
```yaml
apiVersion: github.com/shrav-k/conduit/v0
packages:
  example.org/s1/filesystem: {version: 0.1.1, digest: 'sha256:<…>'}
  example.org/conduit/agent-claude-code: {version: 0.1.2, digest: 'sha256:<…>'}
  example.org/conduit/agent-codex: {version: 0.1.2, digest: 'sha256:<…>'}
```
T1. What `apply` shows and does (engine output, not authored):
```text
new  Pool/workspace, Resource/shared, Agent/agent-a, Agent/agent-b, Conduit/a-project, Conduit/b-project
derived (ACC-044, consumer root): Conduit/workspace (pool-management.v1: 7 operations), Conduit/agent-a (conversation.v1: turn.v1), Conduit/agent-b (turn.v1)
1 register.v1 6 documents  2-4 open.v1 workspace, agent-a, agent-b  5-8 set-up.v1 Pool/workspace, Resource/shared (through workspace), Agent/agent-a, Agent/agent-b
9 open.v1 a-project -> agent-a gains a-project.file.read.v1, a-project.directory.list.v1
10 open.v1 b-project -> agent-b gains a-project-like tools b-project.file.read.v1, b-project.directory.list.v1, b-project.file.write.v1
```
T1. Checks: `conduit tools --as agent-a`, `conduit call agent-a.turn.v1 '{"message":"…"}'`, `conduit call agent-b.turn.v1 '{"message":"…"}'`.

T2. File `project.yaml` as a whole after T2. 12 of its lines are new: the `history` block (4 lines), `a-history` (4) and `b-history` (4).
```yaml
pools:
  workspace: {from: github.com/shrav-k/conduit/core#pool}
resources:
  shared:
    from: example.org/s1/filesystem#local-directory
    owner: workspace
    arguments: {root: <lab>/shared}
  history:
    from: example.org/s1/history#history
    owner: workspace
    arguments: {root: <lab>/history}
agents:
  agent-a: {from: example.org/conduit/agent-claude-code#claude-code}
  agent-b: {from: example.org/conduit/agent-codex#codex}
conduits:
  a-project:
    from: example.org/s1/filesystem#read
    consumer: {kind: Agent, name: agent-a}
    target: {kind: Resource, name: shared}
  b-project:
    from: example.org/s1/filesystem#read-write
    consumer: {kind: Agent, name: agent-b}
    target: {kind: Resource, name: shared}
  a-history:
    from: example.org/s1/history#post-read
    consumer: {kind: Agent, name: agent-a}
    target: {kind: Resource, name: history}
  b-history:
    from: example.org/s1/history#post-read
    consumer: {kind: Agent, name: agent-b}
    target: {kind: Resource, name: history}
```
T2. Commands:
```sh
conduit install --save <lab>/packages/history-0.1.1.tar.gz#<sha256 of history-0.1.1.tar.gz>
conduit apply
```
`apply` reports:
- `unchanged`: 6 objects.
- `new`: Resource/history, Conduit/a-history, Conduit/b-history.
- Root's derived `workspace` Conduit lists `register.v1` and `set-up.v1`, so ACC-033 rows 1 and 4 hold. Neither of the coordinator's mistakes 1 and 2 can occur.

T3. No file and no line. The answer comes from:
```sh
conduit inspect
conduit tools --as agent-a
conduit tools --as agent-b
```
- `inspect` shows, per Agent, the Conduits it consumes with their operations (order 027), and the resolved config `accountServices: false, peerMessaging: false` from the template defaults.
- `tools --as` gives the presented tools.

T4. File `project.yaml`, 2 lines changed (lines 17 and 21):
```diff
   a-project:
-    from: example.org/s1/filesystem#read
+    from: example.org/s1/filesystem#read-write
   b-project:
-    from: example.org/s1/filesystem#read-write
+    from: example.org/s1/filesystem#read
```
T4. Command:
```sh
conduit apply
```
The plan shows:
- `differs` for Conduit/a-project (operations +file.write.v1) and Conduit/b-project (-file.write.v1).
- Steps: close b-project, close a-project; register.v1 (2 new, 10 unchanged); open b-project, open a-project.
- agent-a gains a-project.file.write.v1 and agent-b loses b-project.file.write.v1. Tool names are kept and no session is lost.

T5. No file line written by the author; the tool rewrites the pin. Commands:
```sh
conduit install --save <lab>/packages/filesystem-0.2.1.tar.gz#<sha256 of filesystem-0.2.1.tar.gz>
conduit apply
```
The plan shows (engine output, not authored):
```text
differs  Resource/shared: example.org/s1/filesystem 0.1.1 -> 0.2.1 (server-filesystem 2.1.0 -> 2.2.0); operations offered unchanged
differs  Conduit/a-project, Conduit/b-project: package 0.1.1 -> 0.2.1; operations unchanged (PKG-030)
unchanged Pool/workspace, Resource/history, Agent/agent-a, Agent/agent-b, Conduit/a-history, Conduit/b-history
1 retire.v1 Resource/shared expectedId <id> through workspace (CON-008 closes a-project, b-project)
2 register.v1 (3 new, 6 unchanged)  3 set-up.v1 Resource/shared  4 open.v1 a-project  5 open.v1 b-project
No Agent gains or loses a tool (append_file and copy_file are mapped by no operation). Sessions kept.
```

T6. File `project.yaml`, 1 line changed (line 7):
```diff
-    arguments: {root: <lab>/shared}
+    arguments: {root: <lab>/shared-2}
```
T6. Command:
```sh
conduit apply
```
The plan shows:
- `differs`: Resource/shared, config root.
- Steps: retire shared (closes a-project and b-project), register (3 new), set up shared, open a-project, open b-project.
- Tool names are unchanged.

T7. No file line. Commands:
```sh
conduit close Conduit/a-project
conduit apply --reopen Conduit/a-project
```
Checks between the two commands: `conduit tools --as agent-a` (no a-project tools), `conduit call agent-b.turn.v1 '{"message":"…"}'`.

Without `--reopen`, `apply` reports:
```text
Conduit/a-project: in project.yaml, closed by root at <time> (close.v1, record <id>); not restored. Pass --reopen Conduit/a-project to register and open a new one.
```
With it, `apply` registers a new a-project, with a new ID and the same name and tools, and opens it.

FINAL `project.yaml` after T6 and T7, 31 lines:
```yaml
pools:
  workspace: {from: github.com/shrav-k/conduit/core#pool}
resources:
  shared:
    from: example.org/s1/filesystem#local-directory
    owner: workspace
    arguments: {root: <lab>/shared-2}
  history:
    from: example.org/s1/history#history
    owner: workspace
    arguments: {root: <lab>/history}
agents:
  agent-a: {from: example.org/conduit/agent-claude-code#claude-code}
  agent-b: {from: example.org/conduit/agent-codex#codex}
conduits:
  a-project:
    from: example.org/s1/filesystem#read-write
    consumer: {kind: Agent, name: agent-a}
    target: {kind: Resource, name: shared}
  b-project:
    from: example.org/s1/filesystem#read
    consumer: {kind: Agent, name: agent-b}
    target: {kind: Resource, name: shared}
  a-history:
    from: example.org/s1/history#post-read
    consumer: {kind: Agent, name: agent-a}
    target: {kind: Resource, name: history}
  b-history:
    from: example.org/s1/history#post-read
    consumer: {kind: Agent, name: agent-b}
    target: {kind: Resource, name: history}
```

PROVIDER WORK, counted apart, not the author's. Package files in sectioned form.

filesystem 0.1.1 and 0.2.1, `templates.yaml` (15 lines each), plus 3 `exports` lines each:
```yaml
resources:
  local-directory:
    spec:
      type: '#local-directory.v1'
      inputs:
        root: {schema: {type: string, minLength: 1}, to: [/config/root]}
conduits:
  read:
    spec:
      capability: '#directory.v1'
      operations: ['#file.read.v1', '#directory.list.v1']
  read-write:
    spec:
      capability: '#directory.v1'
      operations: ['#file.read.v1', '#directory.list.v1', '#file.write.v1']
```
history 0.1.1, `templates.yaml` (11 lines), plus 2 `exports` lines:
```yaml
resources:
  history:
    spec:
      type: '#file-history.v1'
      inputs:
        root: {schema: {type: string, minLength: 1}, to: [/config/root]}
conduits:
  post-read:
    spec:
      capability: example.org/s1/filesystem#directory.v1
      operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
```
agent-claude-code 0.1.2, `templates.yaml` (11 lines), plus 2 `exports` lines and `ownerConduits: ['#turn']` on the Type claude-code.v1. agent-codex 0.1.2 is the same with `codex`:
```yaml
agents:
  claude-code:
    spec:
      type: '#claude-code.v1'
      config: {model: ''}
      inputs:
        accountServices: {schema: {type: boolean}, default: false, to: [/config/accountServices]}
        peerMessaging: {schema: {type: boolean}, default: false, to: [/config/peerMessaging]}
conduits:
  turn:
    spec: {capability: '#conversation.v1', operations: ['#turn.v1']}
```
Core additions, which are spec rather than provider work: the template `pool` (`type: pool.v1`, `config: {}`), the template `pool-owner` (pool-management.v1, all 7 operations), and `ownerConduits: ['#pool-owner']` on `pool.v1`.

## Counts
Counting rule:
- Non-blank lines, `---` included, comments excluded.
- The headline column (B) is file lines plus every command that carries a value the author chose: an `install` with its archive and copied digest, or `close` and `--reopen` with a name. That is the measurement plan's literal rule.
- Column A counts file lines only, as both reports did.
- Column C also counts every bare `conduit apply`.

| Task | A: file lines | B: headline | C: with every command | Files edited by the author | Author requests | Baseline | Coordinator | vs baseline (B) | vs coordinator (B) |
|---|---|---|---|---|---|---|---|---|---|
| T1 | 19 | 22 (3 installs, 3 copied digests) | 23 | 1 (project.yaml; tool writes conduit.yaml) | 4 (3 install + apply) | 33 | 154 | less (-11) | -132 |
| T2 | 12 | 13 (1 install) | 14 | 1 | 2 (install + apply) | 16 | 73 | less (-3) | -60 |
| T3 | 0 | 0 | 0 | 0 | 2 checks (tools --as x2) + inspect | 0 | 0 | same | same |
| T4 | 2 | 2 | 3 | 1 | 1 (apply; the engine runs 2 close, 1 register, 2 open) | 8 (+1 restart) | 2 (8 requests) | less (-6) | same lines, 7 fewer requests |
| T5 | 0 | 1 (install with its digest) | 2 | 0 (tool rewrites the pin) | 2 (install + apply) | 3 (+2 recovery steps) | 62 | less (-2) | -61 |
| T6 | 1 | 1 | 2 | 1 | 1 | 2 | 29 | less (-1) | -28 |
| T7 | 0 | 2 (close name, reopen name) | 2 | 0 | 2 (close + apply --reopen) | 2 | 12 | same | -10 |
| All | 34 | 41 | 46 | 1 distinct authored file | 14 (12 excluding T3 checks) vs coordinator 35 | 64 | 332 | less on 5, same on 2, more on 0 | -291 |

Under A: less on 6 tasks (all but T3, which ties). Under C: less on 4 (T1, T2, T4, T5) and the same on T3, T6 and T7. The bar ("less on most, much more on none") holds under all three counts.

Copied digests: 5 (one per install), against the coordinator's 8.

Grants written by hand: 4 (T1 2, T2 2), against the coordinator's 9. Root Conduits written by hand: 0 (derived) against 4.

Sensitivity:
- All-block YAML (no one-line flow entries): T1 23 and T2 13 in files; B total 46. Still passes.
- Pools-only owner Conduits, with root's turn Conduits written by hand from the packages' `turn` template: T1 +8, giving 27 file lines and 30 under B. It still passes, but T1's margin narrows to 3.
- If the build pre-installs the Agent packages: T1 B = 20.
- JSON with one key per line would add about 14 brace-only lines to T1 (about 33 under A), which ties the baseline.

Provider work, counted apart, not the author's: filesystem 2 x (15 + 3 exports) = 36; history 13; Agent packages 2 x (11 + 2 exports + 1 Type line) = 28; 77 in total. Core additions (spec): about 9 lines.

Time without access (not measured; expected): T4, within one `apply`, between `close` and `open`. T5 and T6, while the new server sets up. Each window should be shorter than the coordinator's human-paced 30 s and 80 s, but none is zero.

## Lost
- **Brief windows without access.** Nothing changes in place, so every change of access, version or path is a close or retire followed by new objects. T4 has a window between close and open, even inside one `apply`. T5 and T6 have one while the new server starts. The owner's earlier reports measured 30 s and 80 s by hand; here the windows are shorter but not zero. Removing them needs an atomic swap or an in-place restart, which are deferred.
- **One name, several objects over time.** `a-project` and `shared` are reused after removal, so a name denotes different objects, each with its own ID. Records, inspection and `render` must show the ID history. A reader of records who goes by name can conflate them. OBJ-010 still holds: references resolve to IDs.
- **Declared Conduits are no longer the natural form.**
  - S1's access is all standalone grants, so an Agent's definition no longer says what it needs.
  - G9 (required Conduits open before the run starts) is not exercised.
  - Changing a declared Conduit still means retiring the Agent and losing its session. `apply` refuses without `--replace` and says so.
- **Access levels move into packages.** The author's file shows `#read` or `#read-write`, not the operations.
  - Explicitness now lives in what the engine shows (`render`, `apply`'s plan, inspection) and in PKG-030's rule that an exported Conduit template never changes its grant across installed versions.
  - That rule binds package authors: a wider level needs a new template name.
  - It only checks against versions installed side by side. A version installed after the earlier one was removed is caught only by the plan's printed operation diff.
- **Derived owner Conduits.** Root receives Conduits it did not write. They are grants explicit only after the fact, in the registration's result and in inspection. They are root-only and unclosable, as all root Conduits are. A Type gains a key (`ownerConduits`) that the language must check.
- **A second file form.** Sectioned files and framed documents must both be specified, schema-checked, loaded and covered by vectors, and diagnostics must map back to the file as written. Two ways to write the same thing.
- **Weaker error on resubmission.** An equal resubmission is no longer an error. An accidental duplicate submission is silently a no-op, and is reported only as `unchanged`.
- **`apply` is tooling, not language.** Independent builds and other clients do not get it from `spec/`. An Agent with the same Conduits can do the same steps (test 5), but must compute the diff itself. OBJ-002's equal-document rule makes resubmitting the whole project safe.
- **Inherited `apiVersion`.** A document can no longer be read alone, because it takes its version from the manifest. Copying one file into another project can change its meaning across specification versions.
- **The manifest is tool-written.** The pins are edited by `install --save`, so the author no longer reads them in a file they wrote. The digest is still copied once per install.
- **Not addressed:**
  - JSON authoring costs (braces);
  - the `owner:` and Pool lines (a default Pool would remove 3 lines);
  - the long contract addresses (width only);
  - the engine and command defects of increment 4 (4-12 to 4-15), which the fresh author's 130 script lines compensated for.
- **Unverified.**
  - Every count is a paper count. Ruling 45 dropped paper counts as evidence, so a fresh-author run is needed.
  - The packages with templates are not built, so their digests are placeholders.
  - `install --save`, `apply`, `close` and `render` do not exist.
  - The equality test for OBJ-002 must be specified exactly, or `apply` will churn.
  - Whether a Resource template needs `config: {}` was not checked.
  - The windows without access were not measured.

## Composable
- **Compact and keyed-file designs.** The sectioned file is a step toward Compose's shape. Nesting grants under agents, a default Pool (no `pools:` and `owner:` lines, about 3 lines less) and capability inference all stack on it.
- **Explicit plan and apply designs** (the prior-art map's P1, P2 and P9). This `apply` is their conservative subset: a client composition with no in-place mutation. A spec-level apply could replace it later.
- **Targeted operations:**
  - an atomic swap, `open.v1` with `replacing:`, to remove T4's window;
  - an in-place upgrade or restart (A67), to remove the windows in T5 and T6;
  - an `as:` local-name argument (IFC-012), to decouple tool prefixes from object names.
- **Inline templates** (INS-025's fifth form), for one-off objects that no package ships.
- **Tooling that makes no spec change:** a lock file written by a tool, Terraform-style.
- **Ruling 52's two-tier refusals and the fixes for 4-13 and 4-14**, independent of this design.
- **Rulings 49 to 51.** Owner Conduits give the root Interface a recorded channel to every Agent, for sign-in prompts and pending actions. Conduits stay immutable values swapped as a set, which suits v1's leased local enforcement (A65) and ruling 51's two-half relationships.

## Verdict counts (major)
Holds with fixes, on paper only. I recounted every block mechanically and the design's own numbers are right: 19 file lines in T1, 12 in T2, 2 in T4, 0 in T5, 1 in T6, 0 in T7, 34 in all, and a 31-line final file. Counting the commands that carry a chosen value gives 41; counting every command gives 46. Under the headline count (column B) that is less than the baseline on 5 tasks, the same on 2 and more on none. Each rewrite does reach the stated access on paper, and the person can request turns through the derived root Conduits `agent-a` and `agent-b`. The pass is narrower than the design claims, though, and one of its robustness claims is miscalculated. Written as JSON with one key per line, which is how the fresh author actually wrote, T1 is 48 file lines, not "about 33". That is much more than the baseline on T1 and T2, so the bar fails. Fully block YAML also fails. Under the design's own column C (every command counted), it fails if either of two plausible attributions is applied: the access templates `read`, `read-write` and `post-read` counted as the author's (FIRST_BUILD says "S1's own project, its templates ... is the author's"), or the baseline's 6 lines of history-convention instructions given an equivalent. T3's answer about reach around the engine is incomplete as written. T5 adds a window without access where the coordinator's T5 had none. Reusing a name after a retire depends on the old object being removed, which the design glosses over. And `apply` has no route for a changed template, so T5 works only because the templates moved into packages.

RECOUNT (scratch files in <scratch>/audit/)

Per task, measurement plan rules (non-blank lines, comments excluded):

| Task | File lines | Commands with a value | B | Bare apply | C | Baseline | Result under B |
|---|---|---|---|---|---|---|---|
| T1 | 19 | 3 installs | 22 | 1 | 23 | 33 | less |
| T2 | 12 | 1 install | 13 | 1 | 14 | 16 | less |
| T3 | 0 | 0 (inspect has no value; tools --as is a check) | 0 | 0 | 0 | 0 | same |
| T4 | 2 (lines 17 and 21 of the T2 file, confirmed) | 0 | 2 | 1 | 3 | 8 | less |
| T5 | 0 | 1 install | 1 | 1 | 2 | 3 | less |
| T6 | 1 (line 7, confirmed) | 0 | 1 | 1 | 2 | 2 | less |
| T7 | 0 | close plus --reopen | 2 | 0 | 2 | 2 | same |
| All | 34 | 7 | 41 | 5 | 46 | 64 | less on 5, same on 2, more on 0 |

Column C is less on 4 tasks. The final file is 31 lines. Copied digests: 5. On the literal rule: the plan's table row ("each command the author types that is not a check is listed with its lines") can be read to support column C. Column B is the narrower reading.

SENSITIVITY (my counts; the design's estimates corrected)

- **JSON, one key per line (the fresh author's style).** T1 = 48 file lines (51 with commands) against 33. T2 adds 29 lines (30 with its install) against 16. That is much more on T1 and T2, so the bar fails. The design's "about 14 brace lines, about 33, ties" is wrong by about 15 lines. It assumed flow-style objects, which is not one key per line.
- **Fully block YAML** (consumer and target written in block). T1 = 31 (34 under B), which is more than 33. T2 adds 21 (22 under B, +38%). Less on only T4, T5 and T6, so not "most": fails. The design's "all-block 23/13" keeps consumer and target in flow.
- **Access templates counted as the author's** (12 lines: `read` 4, `read-write` 4, `post-read` 4):
  - Under B: T1 = 30 and T2 = 17 (one more than 16). Still passes, narrowly.
  - Under C: less on only T1, T4 and T5. Fails.
  - It also breaks T5. Templates registered by the author hold filesystem 0.1.1 (NAM-009). Resolved again after `install --save` pins 0.2.1, they differ, so OBJ-002 rejects them as a conflict. `apply`'s steps cover changed Conduits (close), Resources and Pools (retire) and Agents (--replace). They do not cover templates (`replace.v1` under SUB-009).
- **History convention.** The baseline's T2 count of 16 includes 6 lines of CLAUDE.md and AGENTS.md telling each agent how to post (experiments/007-s1-baseline/setup/*). The baseline prompts said "as your instructions say".
  - In the design there is no authored place for this convention. IFC-033 takes each tool's description from the operation contract (filesystem's "read_text_file" and so on), not from the description of the history Type `file-history.v1`.
  - So the convention has to travel in each turn's prompt, which is a check and goes uncounted. The fresh author's prompts spelled out file names.
  - Counting an equivalent makes T2 the same or more. Under C the bar then fails (less on 3).
- The design's own pool-only sensitivity (+8 lines, T1 = 27/30) checks out.

TASK COMPLETENESS

- **T1:** done. A gets read and list; B gets read, list and write. Order is fine: ACC-033 row 2 covers a Pool and its Resource registered in one submission, and ACC-022 lets root open to a target that is not ready.
- **T2:** done for access: both agents can post (write) and read. The convention caveat above applies.
- **T3:** incomplete.
  - `inspect` and `tools --as` give the engine's reach plus `accountServices` and `peerMessaging` false.
  - Reach around the engine also needs facts that inspection does not show: no shell or built-in tools; Codex's `apply_patch`, user-input tool and code-mode host; Codex reading `~/.codex/AGENTS.md` (3-07); Codex's collaboration tools (4-06); the same OS user; the shared subscription login.
  - Those come from the two Agent packages' READMEs and the closeout, as in the coordinator's t3-answer.md. Inspection effort is therefore understated. The 0 lines stand.
- **T4:** done. Both Agents lose project access between the closes and the opens.
- **T5:** done, and no new tool is granted. filesystem 0.2.x maps only 3 tools, and PKG-030 guards `read-write`. NAM-014 keeps history on filesystem 0.1.1, so "history and its two Conduits unchanged" is right.
- **T6:** done. Same tool names.
- **T7:** done: revoke, B unaffected, restore. The same author-written count as the baseline.
- **Turns:** possible through the derived ACC-044 Conduits, which `apply` opens.

HAND-WAVED OR MISSING

1. **Name reuse after a retire.** A name is freed when its object is removed (OBJ-011 observable; LIF-035), and a retired Resource is removed only once its shutdown is confirmed. "Freed when closed or retired" is wrong for Resources.
   - `apply` must wait for removal before registering the new `shared`.
   - If the shutdown is pending or unknown, `apply` stops with both Agents' project Conduits already closed (CON-008 closes them when the retire is dispatched). A person then has to recover by unregistering. Neither the plan nor the counts mention this.
2. **The T5 window regresses.** The coordinator's T5 had no time without access: it made the new Resource first and retired the old one afterwards. The design must retire first so the name can be reused, which opens a window that lasts through shutdown, setup and open. The claim that the window is "shorter than the coordinator's 30 s and 80 s" is false for T5.
3. **Step 5's "this run closed it".** In T5 and T6 the Conduits are closed by CON-008 as a consequence of this run's retire, not by a close this run issued. The rule must say that counts, or a-project and b-project are registered and never opened, and the task fails. The records do name the retire invocation (CON-008 observable), so this is fixable.
4. **T7 text contradicts itself.** "A grant closed outside this run is registered but never opened" conflicts with "Pass --reopen to register and open a new one". What a plain `apply` run during the revocation leaves behind, and what --reopen then does, is unspecified.
5. **Request counts.**
   - `apply` composes many requests. At the operation level: T1 13 (3 install, 1 register, 5 open, 4 set-up), T2 5, T4 5, T5 6, T6 5, T7 3. That is 37 in all, against the coordinator's 35 author commands.
   - "7 fewer requests" at T4 compares author commands (1 against 8). At the operation level it is 5 against 8.
   - The design's own rows sum to 15, not 14.
   - "Grants by hand" should be 5, not 4, if `--reopen` counts as a grant, as the coordinator counted T7's 1.
6. **Arithmetic slips.** T4's plan says "2 new, 10 unchanged"; the project has 9 documents, so it should be 7 unchanged. T1's plan line 10 has the typo "a-project-like tools".
7. **Not counted, consistent with the earlier runs:**
   - `--store DIR` on every command (main.rs requires it);
   - 6 y/N keystrokes;
   - `conduit init`.
8. **Provider work left off the list:**
   - version-bump lines in the 5 package manifests;
   - history's dependency pin re-digested for filesystem 0.1.1.
   - Adding `ownerConduits` to the existing contracts `claude-code.v1`, `codex.v1` and `pool.v1` under the same names: NAM-024 allows only optional additions, and this one changes what registering does. It may need new version segments.
9. **Derived Conduits share their instance's name** (Conduit/workspace with Pool/workspace; Conduit/agent-a with Agent/agent-a). NAM-027 binds only documents, so this is legal. But rendered or exported documents would collide under NAM-027, and an author's Conduit named after an Agent would collide under OBJ-002.
10. **Engine defects not addressed** (4-12 turn deadline, 4-13 exit codes, 4-14 help, 4-15 timeouts). These drove the fresh author's 130 lines of scripts and the T7 recreation (128 lines). Nothing here rules out a repeat, and ruling 45 makes a fresh-author re-run the only evidence that counts.
11. **Agents cannot discover the shared root through the engine** (no `list_allowed_directories` is mapped), so every prompt carries an absolute path. This is inherited from both Conduit runs and uncounted.

SPEC FACTS CHECKED

- ACC-033 rows 1, 2, 4, 5 and 10.
- pool-management.v1 has 7 operations.
- Core already has a Pool template `system`, which FST-006 forbids instantiating, so a new `pool` template is needed.
- IFC-012 names tools `<local name>.<operation>`.
- ACC-024: root Conduits cannot be closed.
- CON-008: a Conduit is closed at once when its target's retirement is dispatched.
- NAM-014: a package resolves through its own manifest.
- Template input `default` is allowed (INS-021).
- The coordinator's files total 349 lines, and its T5 time without access was "none".

## Verdict safety (major)
The design fails as written, and every failure has a fix. The engine's core holds: Conduits stay immutable, change is close then open, references resolve to IDs, operation lists stay explicit, ACC-033 routing is untouched, and T5 adds no tool to any Agent. The trouble is that the design moves several guarantees to places that cannot carry them.
(1) The guard against reopening revoked grants lives only in a client. It reads records, which can be purged (REC-004), and records of removed objects are read as DL8 leaves to each engine. It covers Conduits only. And OBJ-002's equal-document rule turns any bulk resubmission, plain `conduit submit` included, into a way to recreate objects someone deliberately retired. That breaks G3.
(2) Defaults that decide access now come from packages. PKG-030 freezes only Conduit templates, so a package upgrade can switch on an Agent's `accountServices`, which today's Type contract makes the author state, or widen a Resource's config. The plan shows neither. This is the T5 hole again.
(3) Names are reused, but `open.v1` and `close.v1` take no `expectedId`. That allows a confused-deputy swap between register and open, and lets a stale plan silently undo a newer change (G10).
(4) LIF-035 frees a retired Resource's or Agent's name only once its shutdown is confirmed. A failed or unknown retirement therefore cuts access at step 2 and blocks step 3, with no rollback. The design's wording also contradicts LIF-035, which it lists as unchanged.
(5) Tool names stay stable while the target ID changes. OPS-009 expects a retry under the same tool name to be a repeat, but here it is a new invocation, so a write whose outcome was unknown is applied again (G9).
(6) The equality test does not look at state, may need a placement call, which breaks the pure core, and is incomplete next to SUB-009's. It also leaks stored definitions to callers who may not inspect them (ruling 52).
No single item is fatal, because each has a small fix at the spec level. Taken together they are major.

All paths are under <home> Nothing was modified.

C1. MAJOR: revocation is reversed by objects other than Conduits, by bulk resubmission, and by apply's rule 5 (G3, preservation test 2, RD2/DO2)
- a) Rule 5 covers grants only. Apply classifies every absent object as `new`.
  - A responder retires agent-b, which was compromised. Its shutdown is confirmed, so LIF-035 removes it.
  - Later the owner runs `conduit apply` for an unrelated T6 edit. The plan prints `new Agent/agent-b` and does not say who retired it or when. Apply registers it and sets it up, and ACC-044 opens root's `turn` Conduit to it.
  - The same happens to a Resource that someone retired on purpose.
- b) "No Conduit of that name and consumer ever existed (from the records)" is undecidable or unsafe:
  - REC-004 lets root purge records. After a purge, a revoked a-project looks as though it never existed, so apply opens it.
  - Records hold IDs (REC-002). After `--replace Agent/agent-a` the consumer has a new ID. If rule 5 matches by ID, a-history, revoked earlier, is reopened for the new agent-a.
  - Matching by name needs a name history of removed objects, which DL8 leaves to each engine (LIF-019). G3's behaviour would then differ from engine to engine.
- c) The OBJ-002 equal-document rule changes what a plain resubmission does:
  - Today `conduit submit project/` after T7 fails on the first existing name.
  - Under the new rule the same command succeeds. Existing objects report `unchanged`; a-project, which was revoked, is registered again, and every removed Pool, Resource or Agent is registered again.
  - The shell's submit (reference-model/crates/shell/src/author.rs:233-281) then sets up every created Pool, Resource and Agent. Under CON-004 that setup opens their declared Conduits.
  - Re-registering a revoked grant, even left unopened, lowers what it takes to reverse the revocation from register.v1 plus open to open alone (ACC-030: a bound lets its holder open, for anyone, a Conduit that already exists).
- d) Every CLI user is root. "Closed by root at T" therefore cannot tell a deliberate revocation from apply's own crashed run, and operators will learn to pass `--reopen` by habit.
- FIX:
  - The engine holds a tombstone as its own state, which no purge touches, as OPS-008's state is not. It is written when an actor closes a Conduit, or retires or unregisters an object, at its own request (not through CON-008's cascade, and not where the request says it is part of a replacement).
  - `register.v1` rejects a new object under (kind, name) while a tombstone stands, unless the request carries `supersedes: <tombstoned ID>`. The result names who made the tombstone and when.
  - `close.v1`, `retire.v1` and `unregister.v1` gain an optional intent, `revoke` or `replace`, recorded under REC-002.
  - Apply passes `supersedes` only for objects it removed itself in the same run, and needs `--reopen` or `--recreate` for anything else.
  - A gap in the records (REC-006) never counts as "never existed".

C2. MAJOR: package defaults widen access silently; the T5 hole moves to Agents and Resources (ruling 48, G1, G6 of the second map)
- packages/agent-claude-code/contracts.yaml:16-19 deliberately requires `accountServices` and `peerMessaging` with no default, so the author must state them.
- The design moves `default: false` into the package's template input. NAM-024 freezes contracts and PKG-030 freezes Conduit templates, but nothing freezes template inputs, defaults or fixed `config`.
  - Agent example: agent-claude-code 0.1.3 makes the default `true`. A fresh T1, or an `apply --replace` run for any other reason, gives the Agent its login's account connectors. The project file never mentions them.
  - Resource example: filesystem 0.2.2 keeps `read` and `read-write` and the Type unchanged, but its `local-directory` template adds a second allowed directory or maps `root` to a wider config place.
- The T5 plan prints only "package 0.1.1 -> 0.2.1; operations unchanged" and "No Agent gains or loses a tool", which is true and misleading: what `file.read.v1` reaches has widened.
- FIX: either of the following, and in both cases the plan prints the full resolved-definition diff for every `differs` object (config, placement, runtime, implementation item, server version), not just operations.
  - Keep reach defaults in the Type contract's schema, where NAM-024 freezes them, and leave them out of template inputs.
  - Or extend the new PKG-030 to every exported template's fixed config, input destinations and defaults.

C3. MAJOR: confused-deputy name swap between register and open (G2, ACC-030)
- `open.v1` and `close.v1` take only a `conduit` reference (spec/core/operations.yaml:709-765), and LIF-037's `expectedId` does not apply to them. Apply opens grants by name, as today's submit does (author.rs:263, `{"conduit": o["name"]}`).
- Actor X holds `management` `register.v1` (ACC-030 already trusts such a holder to register anything) and a narrow `access` bound covering `shared` and read.
- During T4, after apply's step 3 registers the new b-project (`read`), X closes it. ACC-012 permits this because X's bound covers the target and its operations, and the close removes it and frees the name.
- X then registers its own `Conduit/b-project`, with any consumer, any target and any operations, for example its own Agent to `history` with write.
- Step 4 (setups, which can wait minutes on a ruling 49 sign-in) gives X the time. At step 5, root's unbounded `access` opens X's Conduit. X has gained root's power to open.
- FIX:
  - Extend LIF-037 to `open.v1` and `close.v1`.
  - Apply opens only by the IDs that `register.v1` returned, and sets up with `expectedId`.
  - The derived owner Conduits are opened by ID taken from the result, not by name.

C4. MAJOR: a stale plan becomes last-writer-wins (G10/REV, preservation tests 5 and 8)
- Operator A applies T4. Operator B, holding an older project.yaml, then runs apply and sees "a-project read-write -> read".
- B answers y. Apply closes a-project, re-registers it read-only and reopens it ("this run closed it"). A's newer change is reverted.
- Nothing is stale-checked: `close.v1` has no `expectedId` or revision. "A concurrent change is rejected all or nothing" holds only for `unchanged` entries, never for the entries apply closes. The y/N pause widens the window.
- FIX:
  - Every close, retire and set-up step carries the plan-time `expectedId`.
  - The plan shows who registered or opened each object it will change, and when.
  - An entry whose stored object is newer than the run's last read is a conflict unless `--force` is given.

C5. MAJOR for availability, and it contradicts "unchanged LIF-035": a reused name is not free after retirement
- The thesis says "a short-lived name is freed when its object is closed or retired". That holds for Conduits only.
- LIF-035 and LIF-017: a retired Resource or Agent keeps its name until its shutdown is confirmed, which needs `retire.v1` to succeed (LIF-024). CON-008 keeps a retired consumer's Conduits open, under ACC-039, until then.
- T5 and T6: step 2 retires `shared`, and CON-008 closes a-project and b-project at once. If retire.v1 fails or ends unknown, step 3 hits an OBJ-002 conflict because the stored definition differs. Apply stops with both grants gone and no rollback (LIF-027).
- `--replace Agent` is stuck whenever a turn outlasts the retirement (4-12: 180 s turns).
- Across machines, a silent cut (multi-machine README, about line 340) leaves the shutdown unknown for a long time. The only way to get the name back is `unregister.v1`, which removes an instance whose run may still be live (LIF-017).
- FIX:
  - Apply proceeds past a retire only once removal is confirmed, and otherwise stops before cutting access. Retire and register are reordered so the old object goes only once its successor can take the name, which the plan states.
  - Or register the successor with `supersedes`, with the engine reserving the name for it.
  - Correct the thesis sentence to "removed", not "retired".

C6. MAJOR: a stable tool name over a new target breaks at-most-once (G9; safety rows S1-unknown-write, S1-key-scope)
- OPS-008 scopes an idempotency key to caller, sending instance and target. OPS-009 (operations.yaml around 579) explicitly expects a repeat under the same tool name, through a replacement declared Conduit with the same target, to be a repeat.
- In T5, a write by agent-b through `b-project.file.write.v1` ends unknown, because retirement stopped the server. agent-b retries with the same key and the same tool name.
- The target is now the new `shared`, with a new ID, so the retry is a new invocation and writes to the same directory a second time.
- Reconciling it (OPS-014) needs `management`, because the old instance no longer exists (ACC-033 row 12), and Agents do not have it.
- Today the tool names changed and the retry was refused. The design removes that signal. The same happens in T6, where the retry goes to `shared-2`.
- FIX:
  - The plan lists invocations still in flight or unknown on every object it will retire, and refuses without `--force`. Retirement drains in-flight work first.
  - Or OPS-008 records a predecessor link, so a key repeated under the same local name to the successor reports conflict naming the old invocation instead of being dispatched.

C7. MODERATE: the equality test ignores state, may call a provider, and is incomplete (G6, G7, ruling 46, README Conformance line 323)
- a) It ignores state. A document equal to a retired object whose shutdown is pending reports `unchanged`. Apply sets nothing up, and the plan says all is well while agent-b or `shared` is dead (ruling 52).
- b) It may call a provider. OBJ-013 lets placement be left out. PLC-002 then asks the machine provider (PRV-015), and register.v1 can report unavailable for it. "Resolved now" therefore needs a provider call, so the core is no longer pure. On a provider that answers differently, equality churns. A `differs` Resource is retired and replaced, possibly on another node, where the absolute path `<lab>/shared` names a different machine's disk behind the same tool name (G16).
- c) It is incomplete. The design's list omits SUB-009's project-contracts digest (SUB-008) and the selected implementation items. A changed local contract or program with unchanged text reports `unchanged` and is silently ignored.
- d) OBJ-018 says description and annotations do not change the object, so an edited description reports `unchanged` and is silently dropped. README line 323 says an engine "never accepts a document and ignores part of it unsaid".
- e) SCH-007 compares lists in order, so reordering `operations` closes and reopens the grant, a window without access, for a cosmetic edit.
- FIX:
  - Reuse SUB-009's test verbatim.
  - An object that is retired, or not removed, never compares equal.
  - Compare only what the document states, with stored placement and runtime kept when the document leaves them out, and no provider call.
  - Report ignored metadata in a diagnostic that does not reject.

C8. MODERATE: equality leaks what the caller may not inspect (ruling 52's exception, OPS-023)
- `register.v1` is authorized against `management` (row 2), while inspecting a Resource needs its Pool (row 6) and inspecting an Agent needs `management` `inspect.v1` (row 7).
- An actor with `register.v1` but no `inspect.v1` can submit guesses. `unchanged` versus conflict, plus a diagnostic naming each field that differs, tells it, for example, agent-a's `accountServices`, its model, or a path.
- FIX: accept as `unchanged`, and give the field-level diagnostic, only where the actor may also inspect the object. Otherwise answer conflict exactly as today.

C9. MODERATE: ACC-044 is root-only and squats the Conduit namespace (ruling 51, test 8, G2)
- In peer mode the owner's Interface on a peer engine is not that engine's `root`, so nothing is derived there and the line savings vanish. Deriving for "the registering actor" instead must not let apply open those Conduits without an ACC-030 bound check.
- Derived names collide with root's declared local names `management` and `access` (FST-002):
  - A root-registered Pool named `management` fails IFC-014 on `management.register.v1`. The same Pool registered by an Agent succeeds.
  - An Agent named `management` gives root both `management.turn.v1` and `management.register.v1`, which reads like one target.
- Cross-project or pre-registered Conduit names block root's registrations.
- `<instance>-<template>` can exceed 63 characters. Moving from one owner Conduit to two renames root's tools.
- The Azure precedent is wrong: creating a resource group creates no role assignment. GCP's project creator receiving roles/owner is the real precedent.
- FIX: prefix derived names (`owner-<name>`); define the length rule; derive for whichever actor registers, opened only within its bound; restrict `ownerConduits` to Pool and Agent Types.

C10. MODERATE: audit across reused names
- No promise is drafted for the "ID history" the design says it needs.
- `inspect Conduit/a-project` finds only the current object's records (LIF-013). Earlier holders of the name are reachable only through DL8.
- Apply's 5 to 10 requests carry no shared correlation, unlike a setup's parts (REC-002 `serving`).
- An object an attacker pre-registered with equal content is opened by root and reported `unchanged`, while its registration record names the attacker.
- FIX:
  - A promise that inspection lists every earlier ID that held a name.
  - An optional change ID recorded with each request.
  - Report `unchanged (registered by X at T)`.

C11. MINOR TO MODERATE: defaults that are ambiguous
- The reading rule `{..., metadata: {name: N, + B.metadata}, ...B}` lets `...B` overwrite the composed metadata. It should read "B without metadata".
- No precedence is stated when a file's `apiVersion` differs from the manifest's.
- `install --save` writes `apiVersion` when it creates conduit.yaml, so a tool's version decides the spec version of the author's documents, against ruling 35. It should never write or change `apiVersion`.
- No rule tells a sectioned mapping from a framed document inside a `---` stream.
- Apply's behaviour for a changed template or Interface is undefined: OBJ-002 gives conflict, and `replace.v1` brings back 4-01 through LIF-009.
- A derived root Conduit left registered by a failed run is never opened by a later one, since apply opens only "new" ones.

C12. MINOR: PKG-030 itself
- The ID is already taken: spec/promises/packages.yaml:459 is PKG-030, on `remote` provider items.
- "Compared by export name" must say that versioned selections are compared by address and name. Otherwise every upgrade differs and T5 is rejected.
- There is no `uninstall.v1`, so the check depends on what one store has installed. A fresh engine, a peer, or a discarded store accepts a widened `read`. Whether a pinned project installs then differs by engine (G6), and only the plan protects it.

C13. MINOR, for v1 (A65)
- Within a lease bound, a machine can hold both a-project, ID X, closed but still leased, and a-project, ID Y.
- The tool name then maps to two Conduits for one consumer. Decision points must key on Conduit ID, and must not present Y until X's lease is revoked.

C14. MINOR: ruling 49
- The expected account per Agent home or Resource is absent from the rewrite of S1.
- Either lines are missing from the count, or the sign-in check has nothing to check against.

Holds, checked: ACC-024 already makes every root Conduit unclosable. CON-008 closes a derived Conduit when its target is retired. Close-then-open ordering in T4 never gives two Agents write at once. ACC-034 lets in-flight work finish. Secrets stay references (G5). No new infrastructure is invented (ruling 40).

## Verdict cost (minor)
The design holds with fixes. Building it is medium-sized work: about 20 to 26 promises edited and 3 to 5 added, out of 455. The change is additive at the level of documents: the framed form stays the canonical one, and the existing archives, digests and manifests survive. The core and the loader need moderate additions, and the shell needs the most new code. One choice in the design multiplies the cost and is avoidable: putting `ownerConduits` on the core Type `pool.v1`. Every Pool in the conformance fixtures is of `pool.v1`. So 124 of the 367 cases (88 of them tagged with safety rows) would get an extra derived Conduit, an extra record and extra state entries. Those cases compare `state.objects` and `records` as arrays of exact length, and no tool regenerates them. If the core `pool` template selects a new Type instead, almost none of today's vectors are invalidated. The design's list of spec changes is also incomplete or wrong in places: - PKG-030 is already taken. - FST-005 is the wrong promise to change. - It misses INS-016, OBJ-011, CTR-004, SCH-018 and NAM-024. - It has no expectedId on `open.v1` or `close.v1`, so a stale plan is caught for retirement only. - It leaves open how `apply` classifies the project while `conduit serve` holds the store. All of these can be fixed, and none touches a safety guarantee. For an implementer the specification gets more complex: 5 concepts are added and none is removed.

Scope: I read the specification, the reference model and the conformance cases. Nothing was changed. The counts below come from scripts run over conformance/ (YAML parsed with /usr/bin/python3 and PyYAML 6.0.1).

## 1. Size per area

| Area | Size | Main cost |
|---|---|---|
| spec/ promises | Medium | About 20-26 promises touched, 3-5 of them new |
| spec/schemas | Small to medium | Schema for the sectioned file; `type-spec.v1` |
| spec/core | Small | Two templates, `ownerConduits`, `register.v1` output |
| conformance/ | Small if `pool.v1` is left alone, large (124 cases) if not | Plus 25-30 new cases |
| Loader crate | Medium | Reading a sectioned file and mapping its pointers, about 300-500 lines with tests |
| Core crate | Medium | Equality, derived Conduits, install checks, a pure classify step, about 600-1,000 lines with tests |
| Shell crate | Medium to large | `install --save`, `apply`, `close`, `render`, about 800-1,500 lines with tests |
| Conformance crate | Small | — |
| tools/check_schemas.py | Small to medium | The expansion written a second time in Python, the `ownerConduits` check and the export list of the core README, about 150-300 lines |
| Packages | Small to medium | Provider work, counted apart |

**Overall: medium.**

## 2. Promises

### 2a. Changes the design lists that I checked and that hold
- **OBJ-017.** Changes as drafted.
- **MAN-003.** This is "the manifest promise near line 139". It says "It carries `apiVersion`, as OBJ-017 requires of a document."
- **OBJ-024.** Its observable changes.
- **OBJ-002.** Changes as drafted.
- **OBJ-025.** Its observable changes.
- **SUB-007.** Its observable says an existing template's name is rejected under OBJ-002.
- **SUB-006.** Changes as drafted.
- **OBJ-012, OBJ-013 and OBJ-020.** Their observables change.
- **SUB-005.** The pointer must point into the file as written.
- **MAN-011 and NAM-027.** Change as drafted.

### 2b. Wrong or misnumbered
- **"PKG-030" is taken.** It already exists in spec/promises/packages.yaml line 459, the rule on a `remote` item that implements a provider operation. The new promise should be PKG-031.
- **FST-005 should not change.** It lists the templates of the first-start objects (`system`, `node`, `management`, `access`, `root`, `root-management` and `root-access`). `pool` and `pool-owner` are not first-start templates. The change belongs in two other places:
  - the header comment of spec/core/templates.yaml;
  - the export list in spec/core/README.md, which tools/check_schemas.py's `check_readmes` checks.
- **OBJ-029 and ACC-044 are free IDs.** OBJ-026 to 028 and ACC-040 to 043 were removed.

### 2c. Changes the design missed
1. **INS-016** (language). "The only objects made are the instance and its nested instances." ACC-044 contradicts it.
   - There is also a layering problem. spec/README.md says "which objects they make" is decided by the language layer. ACC-044 is a convention, and it makes the set of objects depend on the actor (root or not).
   - The owner should choose between two options:
     - the derived Conduit's consumer is always root, whoever registers, and the rule is a language rule;
     - or the layer table is amended.
2. **OBJ-011.** Its observable says "A document submitted under the kind and name of an existing instance is rejected under OBJ-002". The design lists OBJ-011 as unchanged, but the equal-document case contradicts this sentence.
3. **CTR-004.** It closes the list of keys of a Type, and `type-spec.v1` has `additionalProperties: false`. The check at install needs a home, either a new CTR-013 or a clause of CTR-004. That check is:
   - the listed templates are in the same package;
   - the Type offers their capability;
   - and, also missing from the design, no listed template includes `get-secret.v1` (SEC-008, SEC-009) or a provider operation (PRV-004).
4. **SCH-018.** It says a JSON file whose value is not an array "holds one document, that value". A sectioned JSON map is not a document. SUB-005 cites SCH-018.
5. **NAM-024 and NAM-025.** A later release may only "add optional things" to a contract of the same name. Adding `ownerConduits` to `pool.v1`, `claude-code.v1` or `codex.v1` changes what registering an instance of them makes. Either NAM-024 is amended, or the change uses new Type names.
6. **LIF-037, plus the inputs of `open.v1` and `close.v1`.** Today `expectedId` exists only on `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1`. `close.v1` takes only `conduit`. So `apply`'s claim that it "catches a stale plan" covers retirement only. Closing a Conduit in T4 is unguarded against a same-name replacement (G10). Either extend LIF-037 or narrow the claim.
7. **OBJ-019.** The section maps must be added to its list of maps keyed by name. A one-sentence change.
8. **FST-006.** Its observable says first-start documents are rejected "because their names are taken under OBJ-002". Only the wording changes, since FST-006 rejects them anyway.
9. **ACC-044 also needs four clauses of its own:**
   - It excludes first start explicitly. `system` is a `pool.v1` Pool, so otherwise FST-002 and FST-003 break.
   - It handles name collisions under IFC-014. A Pool named `management` or `access` is legal today. It would derive a Conduit whose tools collide with root's declared local names, so that Pool could no longer be registered.
   - It caps the length of `<instance>-<template>` names (OBJ-001, 63 characters).
   - It handles a collision under OBJ-002 with a Conduit of another project.
10. **OBJ-002 needs three more clauses:**
    - A retired object that is not yet removed never counts as equal. LIF-007 keeps the name until removal; otherwise `apply` would report `unchanged` for a retired `shared`.
    - Generated names and IDs of nested objects are set aside, as INS-017 does.
    - A two-tier rule from ruling 52. A conflict diagnostic that "names each field that differs" tells a caller who holds `register.v1` but not `inspect.v1` about the stored definition. Repeated attempts would reveal its values.
11. **The design's phrase "a name is freed when its object is closed or retired" is inaccurate.**
    - LIF-035 and LIF-007 free a name only when the object is removed (shutdown confirmed) or unregistered (LIF-017).
    - If `retire.v1` ends pending or unknown, `apply`'s register step fails under OBJ-002. `apply` must then wait, or unregister.
    - No promise has to change, but the steps `apply` takes do.
12. **Editorial edits.**
    - In spec/definitions.md: the rows "document" (twice) and "template", and new terms (sectioned file, owner Conduit, unchanged).
    - The counts and history in spec/README.md, and the ruling log in docs/AUDIT.md.

### 2d. New promises
- OBJ-029.
- ACC-044.
- PKG-031 (the design's "PKG-030").
- Possibly CTR-013 for the check at install.
- Possibly LIF-038 for `expectedId` on `open.v1` and `close.v1`, if LIF-037 is not extended.

**Total: about 20-26 promises touched, about 5% of 455.** The wording ripples:
- OBJ-002 is cited by 11 promises: DL15, DL17, FST-006, LIF-035, LIF-037, OBJ-003, OBJ-011, OBJ-023, OBJ-025, PLC-026 and SUB-007. Four of them (OBJ-011, SUB-007, FST-006 and OBJ-025) need edits. The rest speak only of a taken name, which stays true.
- CTR-004 is cited by 5 and NAM-024 by 4. These need checking, not necessarily editing.

## 3. Schemas
- **document.yaml.** Gains a sectioned-file Schema, which is a new core export. It also needs entry schemas per section, or one schema using `patternProperties`, built from template.v1 and instance.v1 without the frame.
- **contracts.yaml.** `type-spec.v1` gains `ownerConduits`.
- **envelope.yaml stays unchanged only if inheriting `apiVersion` is limited to sectioned files.** The design's OBJ-017 wording also lets a framed document omit it. That would remove `apiVersion` from `required` in `envelope.v1`, and so from the `documents` input of `register.v1`. It would also flip the loader's recorded verdict on spec/schemas/examples/invalid/OBJ-017-no-api-version.yaml. Recommendation: limit inheritance to sectioned files.
- **The result object schema.** `object.v1` (or `register.v1`'s output) gains `registered` or `unchanged`, and the rule that made each derived Conduit.
- **Examples.** About 8 new example files, and new lines in reference-model/crates/loader/tests/expected/*.txt.

## 4. Core package and operations
- **templates.yaml.** Gains `pool` and `pool-owner`. `pool-owner` lists all 7 operations of `pool-management.v1`; I checked that the capability has exactly 7.
- **types.yaml.** Either `pool.v1` gains `ownerConduits` (as designed), or a new Pool Type does (recommended).
- **register.v1 in operations.yaml.** Its output schema, its `effect` ("Adds the templates and instances...") and its description change.
- **open.v1 and close.v1.** Gain `expectedId` if item 6 of 2c is adopted.
- **spec/core/README.md.** Its export list changes.
- **Possibly a dry-run input on `register.v1`** (see 6c).
- **Names.** No change to names.yaml, and no run of set_names.py.

## 5. Conformance

### 5a. Invalidated
- **If `pool.v1` gains `ownerConduits`: 124 cases.** I found 141 cases with Pool instances. 124 of them have a registration that root sends (sender ...0005) and that is accepted, followed by expectations of exact length:
  - by expectation: records and state 81; plus object_count 38; plus reply.objects 5;
  - by area: requests 59, setup 38, registration 16, access 7, changes 3, s1-whole 1;
  - 88 of the 141 carry safety rows, including S1-key-scope, S1-plain-ends, S1-reconcile and S1-unknown-write.
  - There is no tool to regenerate them, so the edits are by hand. The re-measurement under ruling 52 point 6 needs these rows passing, so the edits are on the critical path.
- **If a new Type is used instead: about 0 cases.**
- **The equality rule of OBJ-002 invalidates none.**
  - `registration-existing-name` submits a Pool template named `system` against the Pool instance `system`, so it stays a conflict.
  - `access-fixed-kind-references` is not affected.
  - No case resubmits an equal document.
- **Archives and digests survive.** The new package versions 0.1.1 and 0.2.1 are added beside the existing fixtures, so none of the 268 files that pin a digest changes.

### 5b. New cases: about 25-30
- OBJ-029: 8-10 (YAML and JSON forms, keys an entry may not hold, contracts in a section, collision across sections, the pointer into the file, `apiVersion` from the manifest, neither present, a package in sectioned form).
- OBJ-002 equal documents: 5-6.
- ACC-044: 6-8.
- PKG-031: 2.
- The check at install: 3.
- `expectedId`: 2.
- conformance/manifest.yaml needs the matching entries.

## 6. Reference model

### 6a. Loader
- read.rs and project.rs: detect a sectioned file (one map with no `kind`), expand it into framed `Document`s, and take `apiVersion` from the file or the manifest.
- diagnostic.rs: map pointers back, for example `/from` to `/conduits/a-project/from`, and `/metadata/name` to the entry's key.
- Archives reuse `read_submitted_files`, so packages get the same reader for free.
- Core diagnostics reach the shell without file positions, so the shell must also map them back to the file.

### 6b. Core
- **registration.rs.**
  - The OBJ-002 check (around lines 1478-1510) runs before resolution today and must move after it, with an equality that excludes engine-owned fields, generated names and retired objects.
  - Derive the owner Conduits through `check_conduit` (around lines 1706-1717), including IFC-014.
  - Report each document's status in the reply (around 1720-1745).
  - Extra IDs and records come from the evidence the caller supplies.
- **packages.rs.** The `ownerConduits` check at install, and PKG-031.
- **lib.rs.** A new pure input that classifies a project (unchanged, new or differs) without committing anything.
- **access.rs and lifecycle.rs.** `expectedId` on `open.v1` and `close.v1`, if adopted.
- **Untouched:** retirement.rs, requests.rs, archives.rs and schemas.rs.

### 6c. Shell
- **New code:** author.rs and main.rs gain `install --save` (rewriting conduit.yaml), `apply` (plan, ordering, `--reopen`, `--replace`, one report line per part), `close` and `render`.
- **The existing change routes stay valid,** so tests/changes.rs survives and new tests are added.
- **Build risk 1: `apply` cannot read the store.** Finding 4-09 says the store cannot be read while `conduit serve` holds it. So the plan must come from the engine, by one of two routes:
  - a dry-run input on `register.v1` (a spec change);
  - or diagnostics from a whole-project register attempt, structured enough to list each differing document and field. These cannot preview new objects: a project with no conflicts registers at once.
- **Build risk 2: "from the records" may not be reachable through ordinary operations.**
  - The rule "no Conduit of that name and consumer ever existed" needs history.
  - A removed Conduit cannot be inspected (OPS-029).
  - The record of a close names `access` as its target, so finding it means scanning the records of `inspect access`.
  - Whether an Agent can do the same through ordinary operations (test 5) is unverified.

### 6d. Other
- **Conformance crate.** The runner survives; at most a small addition for a new kind of step.
- **tools/check_schemas.py** (2,588 lines). Must implement the same expansion so the examples' verdicts stay equal between Python and Rust.
- **Provider packages** (counted apart): packages/agent-claude-code and packages/agent-codex at 0.1.2 gain:
  - a templates file and its `exports`;
  - `ownerConduits` on their Types;
  - defaults for the two inputs;
  - repacked archives with new digests, and their READMEs.

  The fixture packages filesystem 0.1.1 and 0.2.1 and history 0.1.1 are new directories and archives. The Rust code of the Agent packages does not change.

## 7. Migration
- **Additive at the level of documents.**
  - Every valid framed document, manifest, archive and digest stays valid, and the framed form stays canonical on the wire.
  - No stored state needs migrating: derived Conduits are never made for existing instances.
- **Two changes in behaviour:**
  1. An equal resubmission now succeeds as `unchanged` instead of reporting conflict. No vector depends on the conflict.
  2. With `pool.v1` changed, every registration by root of a Pool of that type makes one more Conduit:
     - Projects that wrote root's Conduits to their Pool by hand still register, but root then holds duplicate grants. Examples are the S1 fixture's `root-workspace` and the coordinator's `pool-admin` and `root-workspace-register`.
     - A Pool named `management` or `access` becomes impossible to register.
     - A Pool or Agent named like an existing Conduit is rejected under OBJ-002.
- **With a new Pool Type, the only change in behaviour is the equal resubmission.** The apiVersion `github.com/shrav-k/conduit/v0` is unchanged.

## 8. Reading cost for an implementer

**Concepts added: 5.**
1. A second file form: sections, the reading rule, the keys an entry may not hold, and pointers mapped back to the file.
2. `apiVersion` inherited from the manifest.
3. `unchanged` as an outcome of registration, with a precisely defined equality of documents.
4. Owner Conduits: a Type key, a trigger for root only, a naming rule, and their place among the layers.
5. A Conduit template exported under one name keeps its grant across installed versions.

There are also 2 new core templates and possibly 1 new Type. These are not new concepts.

**Concepts removed: 0.** The frame, templates and instances, immutable Conduits, close-then-open and the manifest all stay.

The tooling concepts (plan and apply, `--reopen`, `--replace`, `install --save`, `render`) live in the shell, not in spec/. Independent builds do not get them.

So the specification gets more complex for an implementer (+5 concepts, two parse paths) and simpler for an author.

## 9. Ways to cut the cost
1. Put `ownerConduits` on a new core Pool Type that the `pool` template selects, and leave `pool.v1` alone. This keeps 124 cases unchanged, leaves first start untouched and avoids the NAM-024 problem for core.
2. Limit inheritance of `apiVersion` to sectioned files.
3. Renumber the new package promise to PKG-031. Drop the FST-005 change and edit the core README and the header of templates.yaml instead.
4. Add INS-016, OBJ-011, CTR-004 (or CTR-013), SCH-018, OBJ-019 and NAM-024 to the list of changes. Decide the layer of ACC-044.
5. Decide how `apply` gets its plan from the engine (a dry-run, or structured conflict diagnostics) before any shell work starts.

Key files:
- spec/promises/objects.yaml
- spec/promises/instantiation.yaml (INS-016)
- spec/promises/packages.yaml (PKG-030 already taken)
- spec/promises/contracts.yaml (CTR-004)
- spec/promises/schemas.yaml (SCH-018)
- spec/promises/naming.yaml (NAM-024)
- spec/conventions/lifecycle.yaml (LIF-007, LIF-035, LIF-037)
- spec/core/types.yaml (`pool.v1`)
- spec/core/operations.yaml (`register.v1`, `close.v1`)
- reference-model/crates/core/src/registration.rs
- conformance/manifest.yaml

The scan scripts are <scratch>/scan.py, <scratch>/scan2.py and <scratch>/scan3.py. They write their results to <scratch>/scan.json.