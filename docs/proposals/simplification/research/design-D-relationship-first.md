Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Relationship-first: one line per relationship, an explicit apply that writes the new objects, and operation groups frozen per name

The author's unit is the relationship. Each is one line, `consumer: {local-name: operation-group}`, where the group is a Conduit template the package exports. Each agent and resource is also one line, and the whole project is one keyed file with a single frame, so there are no hand-written templates, manifests, digests or kinds. Every object stays exactly as immutable as it is today: no OBJ-002/OBJ-011/INS-020/PKG-005 relaxation. A changed line still becomes a new Conduit or a replaced Resource, as Zanzibar turns a change into delete plus create, but the engine writes those objects, not the author. It does so through one explicit, plan-first `apply`: a recorded composition of existing operations plus one new atomic `swap` of a Conduit under the same local name. Groups are frozen per name across releases and expanded into the explicit operation list each Conduit stores, so an upgrade can never widen a grant (T5). Explicitness moves into what the engine shows (plan, render, reach, two-tier refusals). On paper, S1 takes 23 lines in the file plus 3 command lines, 26 in all, against the baseline's 64. That is less on six tasks, equal on T3 and much more on none.

## Spec changes
A. NEW LANGUAGE PROMISES: THE PROJECT FORM (PRJ-001 to PRJ-008). These are a normative short form that expands into today's instance documents. Canonical documents stay the form for packages, register.v1, vectors and render.
- PRJ-001. A project directory may hold one project file, `conduit.yaml`, which is a map.
  - Required keys: `apiVersion` (ruling 35, once per file) and `name`.
  - Optional keys: `resources`, `agents`, `interfaces` and `conduits`. No other key is allowed.
  - Canonical template documents may sit beside it and be named by `from`, which is how templates are reused at scale. Instance documents may not sit beside it, so there is one source of instances.
- PRJ-002. `name` names the project and its Pool, an instance of the new core template `pool` (type core#pool.v1). The Pool owns every Resource the file declares, which supplies `owner` (POOL-001 unchanged).
- PRJ-003. Each entry under `resources`, `agents` and `interfaces` is an instance named by its key.
  - A scalar value is its `from`.
  - A map value holds `from` and, optionally, `arguments`, `placement` and `runtime`.
  - The entry expands to `{apiVersion, kind, metadata.name: <key>, from, arguments, owner: <name>}`, with `owner` for Resources only.
  - Every instance still comes from a template, normally one that a package exports (INS-001, INS-002, INS-025 and IN9 unchanged).
- PRJ-004. `conduits` maps a consumer to a map from local name to a relationship.
  - The consumer key names an instance the file declares, or else exactly one existing Interface or Agent, such as `root`.
  - A scalar value is the relationship's `from`, a Conduit template: a package's group, or a local template.
  - A map value is `{from, target?, arguments?}`.
  - The target is `target` if given, else the object of that name that the file declares (the Pool `name` included; names are unique across kinds, NAM-027), else the line is rejected.
  - It expands to a standalone Conduit instance named `<consumer>-<local>`, with `consumer`, `target` and `as: <local>`. A name collision is rejected and the diagnostic names both lines.
- PRJ-005. The expansion is total and deterministic, and `render` returns it. Every diagnostic on an expanded document names the file, line and key path the author wrote (amends SUB-005; ruling 52).
- PRJ-006. A project in this form has no hand-written manifest. Its manifest is `conduit.lock` (apiVersion + `packages`, exactly as MAN-005 and NAM-006). The reference CLI writes it with `conduit add`, which runs install.v1 and records each version and digest. NAM-007 is checked against the lock. Amends MAN-001.
- PRJ-007. Full qualified references only. No aliases, so ruling 27 and D17 are kept.
- PRJ-008. YAML or the equivalent JSON (SCH-013).

B. CONDUIT LOCAL NAMES
- OBJ-013 gains the standard argument `as` for a Conduit instance: its local name, at most 63 characters, optional.
- IFC-012's last sentence becomes: "...and that of a Conduit instantiated on its own is its `as`, or its own name where it gives none."
- Tool names now follow the relationship's key (`project.file.read.v1`), not the Conduit's object name. This removes 4-11 item 5.

C. OPERATION GROUPS = CONDUIT TEMPLATES EXPORTED BY PACKAGES (no new kind)
- New NAM-0xx (language, must). A Conduit template that a package exports lists the same `capability` and `operations` in every release at that address under that name. Installing a release whose template differs from that of an installed release is rejected, and the diagnostic names the template and the operations. Widening means a new group name.
- NAM-024 still lets a capability gain operations. A group never does.
- At open, the group expands into the explicit list the Conduit holds (CON-017 and G6 unchanged).

D. NEW CORE OPERATIONS
- `access#swap.v1` (in access-control.v1).
  - It closes one Conduit and registers and opens another with the same consumer and the same `as`, as one change: either both take effect or neither, and no access decision sees neither or both.
  - It is authorized as the close (ACC-012) plus the register (ACC-033 row 2) plus the open (ACC-006).
  - IFC-014 excludes the Conduit being closed.
  - It is recorded as a close and an open, each serving the swap.
  - This keeps ruling 41 item 3a (narrowing = close + open) with no window without access.
- `management#apply.v1` (in object-management.v1), a new convention area APL-001 to APL-010:
  - APL-001: inputs are the expanded project, its lock, the project's applied revision (none for a first apply) and an optional `dryRun`. A dry run returns the plan and dispatches nothing.
  - APL-002: the plan is the ordered list of parts derived from three things: the new project, the project's last applied form, and the objects as they stand. Each part is an existing operation (register, open, close, swap, set-up, retire, unregister) with its target and the Conduit that authorizes it, plus the tools each Agent gains and loses.
  - APL-003, change classes:
    - relationship added: register + open;
    - relationship removed: close;
    - relationship's group, target or arguments changed: swap;
    - a Resource, Agent or Interface whose `from`, arguments or held package versions (by the lock) differ: replace. Retire; once removed (LIF-035), register a new instance under the same name; set up; reopen its relationships under the same local names;
    - an instance removed from the file: retired only if the request names it in `retire`, otherwise a conflict part.
  - APL-004 (must not), keeping G3:
    - A Conduit that the last apply opened, that is now closed, and that no apply part closed, is never reopened unless the request names it in `reopen`.
    - Likewise, a retired instance is never made again unless the request names it in `recreate`.
  - APL-005: every part is authorized as the same request of the caller, so apply adds no authority (test 5, G2). Authority is checked for all parts before any is dispatched. If any part would be refused, none is dispatched, and the two-tier refusal names each missing Conduit (ruling 52).
  - APL-006: parts run in plan order. A part that fails or ends unknown stops the apply. The outcome follows OPS-022 and there is no rollback (LIF-027). Each part is reported with its outcome and the IDs made and removed.
  - APL-007: a stale revision gives conflict (REV-002). A missing revision on a non-first apply gives invalid (REV-011). The CLI supplies the revision read at plan time, so a plan that has gone stale is rejected.
  - APL-008: the applied form, with resolved IDs, and its revision are kept with the project's Pool, and inspecting the Pool returns them.
  - APL-009 (must not): nothing but an apply request applies a project. There is no reconciler and no apply on a file change, serve or restart (G8, DO2).
  - APL-010: an instance's relationships are opened before its setup within the apply (G9 as ordering).

E. AMENDED PROMISES
- ACC-033 gains rows for apply.v1 (`management`, with each part under its own row) and swap.v1 (`access`). The single-target rule is unchanged.
- REC-001 gains the apply and swap events. Under REC-002 a part's `serving` is the apply invocation.
- IFC-033 (recommended): a tool's description includes the target type's contract description, so the history's posting convention reaches the Agent with no author line.
- ACC-024 (optional, owner's choice): narrowed to root's two first-start Conduits, so that removing a root relationship line can take effect.

F. CORE AND PROVIDER WORK (counted apart, about 149 canonical lines)
- Core exports the Pool template `pool` and the Conduit template `pool-admin` (pool-management.v1: register, register-replacement, inspect, set-up, retire, reconcile).
- Filesystem 0.1.0 and 0.2.0 each export the Resource template `directory` (input `root`) and the groups `read` and `read-write`, identical in both releases.
- History 0.1.0 exports `history` and `post-read`.
- Each Agent package exports an Agent template (inputs `accountServices` and `peerMessaging` defaulting to false, per ruling 48; `model` '') and a group `converse` (turn.v1).
- Files: <scratch>/relfirst/provider/

G. INSPECTION
- inspect.v1 of an actor lists the Conduits it consumes (order 027's fix of 4-09), each with its group expanded, its tool names, and the type's stated reach around the engine. The CLI shows this as `conduit reach`, the T3 answer.

H. REFERENCE CLI (not promises)
- `conduit add ARCHIVE...`: install.v1 each archive, then write the lock.
- `conduit plan`: apply with dryRun.
- `conduit apply [--reopen C] [--retire X] [--recreate X] [--yes]`.
- `conduit render`.
- `conduit reach [--as X]`.
- Every command exits non-zero unless every part succeeded (addresses 4-13).

WHAT STAYS UNCHANGED
- OBJ-002, OBJ-011, SUB-006, SUB-007, INS-001, INS-002, INS-013, INS-020, PKG-005, NAM-009, NAM-010, LIF-006, LIF-008, LIF-009, LIF-035, ACC-011, ACC-022, CON-001, CON-017, FST-001 to FST-006, ACC-033's single-target rule, the canonical envelope for packages and the wire, declared Conduits and templates for reuse, and one capability per Conduit.

RULINGS TOUCHED
- Reversed: round 6 D11's "not an apply of several".
- Kept but qualified: ruling 22/DD2. "Submitting only registers" still holds for register.v1; apply is a separate explicit verb.
- Carried out: D17's tooling half (the generated lock).
- Moved to canonical documents only: rulings 31 and 37's per-document envelope. The authored file no longer carries it.

## Rewrite
Paths are under `<lab>/`. Harness steps are not counted, as in the measurement plan: `conduit init`, `conduit serve`, and checks (`conduit call agent-a.turn.v1 '{"message": ...}'`, `conduit tools --as`). The project directory starts empty. The scratchpad copies of each state are in <scratch>/relfirst/states/

```sh
# T1: commands (cwd <lab>/project)
conduit add <lab>/packages/filesystem.tar.gz <lab>/packages/agent-claude-code.tar.gz <lab>/packages/agent-codex.tar.gz
```

```yaml
# T1: <lab>/project/conduit.yaml (16 lines)
apiVersion: github.com/shrav-k/conduit/v0
name: workspace
resources:
  project: {from: example.org/s1/filesystem#directory, arguments: {root: <lab>/shared}}
agents:
  agent-a: example.org/conduit/agent-claude-code#claude-code
  agent-b: example.org/conduit/agent-codex#codex
conduits:
  root:
    workspace: github.com/shrav-k/conduit/core#pool-admin
    agent-a: example.org/conduit/agent-claude-code#converse
    agent-b: example.org/conduit/agent-codex#converse
  agent-a:
    project: example.org/s1/filesystem#read
  agent-b:
    project: example.org/s1/filesystem#read-write
```

```sh
# T1
conduit apply
```

```yaml
# T1: <lab>/project/conduit.lock, GENERATED by conduit add, not authored
apiVersion: github.com/shrav-k/conduit/v0
packages:
  example.org/s1/filesystem: {version: 0.1.0, digest: sha256:200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160}
  example.org/conduit/agent-claude-code: {version: 0.1.1, digest: sha256:166ce271e4c5e83ff13c228fe56075f1fa7f98557b633dbe60f4d2fe1ee60e9e}
  example.org/conduit/agent-codex: {version: 0.1.1, digest: sha256:05b47a8571743789ae5b9fc060ce1cb43777b92c61206a617cf49334aee626d6}
```

```text
# T1: what apply SHOWS before it asks y/N (engine output, abbreviated)
project workspace: revision none -> 1, caller Interface root; every part's authority checked first: all permitted
+ Pool workspace        from core#pool                          register.v1 via root/management (ACC-033 row 2)
+ Resource project      from filesystem#directory 0.1.0 sha256:200de7c1..., root <lab>/shared, owner workspace (row 2: owner registered in this apply)
+ Agent agent-a         from agent-claude-code#claude-code 0.1.1; accountServices false, peerMessaging false (input defaults)
+ Agent agent-b         from agent-codex#codex 0.1.1; accountServices false, peerMessaging false
+ Conduit root-workspace   root -> Pool workspace, pool-admin: register, register-replacement, inspect, set-up, retire, reconcile; tools workspace.*
+ Conduit root-agent-a     root -> agent-a, converse: turn.v1; tool agent-a.turn.v1   (likewise root-agent-b)
+ Conduit agent-a-project  agent-a -> Resource project, read: file.read.v1, directory.list.v1; tools project.file.read.v1, project.directory.list.v1
+ Conduit agent-b-project  agent-b -> Resource project, read-write: + file.write.v1; tools project.file.read.v1, project.directory.list.v1, project.file.write.v1
order: register (1 request) -> open root's 3 via root/access -> set up workspace (management) -> set up project (Pool workspace via root-workspace, row 4) -> open agent-a-project, agent-b-project -> set up agent-a, agent-b
```

```sh
# T2: commands
conduit add <lab>/packages/history.tar.gz
```

```diff
# T2: conduit.yaml, 3 lines added
 resources:
   project: {from: example.org/s1/filesystem#directory, arguments: {root: <lab>/shared}}
+  history: {from: example.org/s1/history#history, arguments: {root: <lab>/history}}
 ...
   agent-a:
     project: example.org/s1/filesystem#read
+    history: example.org/s1/history#post-read
   agent-b:
     project: example.org/s1/filesystem#read-write
+    history: example.org/s1/history#post-read
```

```sh
# T2
conduit apply
# Plan: + Resource history (register via management + Pool workspace, row 1, through root-workspace's register.v1; set up via the Pool)
#       + agent-a-history, agent-b-history: post-read (file.read.v1, directory.list.v1, file.write.v1); tools history.*
#       (the tool descriptions carry the type's posting convention, per the recommended IFC-033 change)
```

```sh
# T3: no lines written
conduit reach
```

```text
# T3: engine output (abbreviated)
agent-a  Agent, from agent-claude-code#claude-code 0.1.1, ready
  through the engine:
    project -> Resource project (filesystem#local-directory.v1, root <lab>/shared): read = file.read.v1, directory.list.v1; tools project.file.read.v1, project.directory.list.v1 [Conduit agent-a-project, open]
    history -> Resource history (history#file-history.v1, root <lab>/history): post-read = file.read.v1, directory.list.v1, file.write.v1; tools history.* [agent-a-history, open]
  around the engine (type claude-code.v1 as configured): accountServices false, peerMessaging false; shell, code execution, browser, other-Agent, hook and plugin reach off; access is mediated, not OS isolation
agent-b  ... project: read-write (+ file.write.v1); history: post-read; type codex.v1 as configured ...
root     management, access (first start); workspace: pool-admin; agent-a, agent-b: converse
```

```diff
# T4: conduit.yaml, 2 lines changed
   agent-a:
-    project: example.org/s1/filesystem#read
+    project: example.org/s1/filesystem#read-write
     history: example.org/s1/history#post-read
   agent-b:
-    project: example.org/s1/filesystem#read-write
+    project: example.org/s1/filesystem#read
     history: example.org/s1/history#post-read
```

```sh
# T4
conduit apply
# Plan: ~ agent-a project: read -> read-write   swap.v1 (close agent-a-project, open new under `as: project`); widens: + project.file.write.v1
#       ~ agent-b project: read-write -> read   swap.v1; narrows: - project.file.write.v1
#       no interval without access; tool names unchanged; lists change via listChanged; no session lost; template names unchanged and true
```

```sh
# T5: no lines in the file
conduit add <lab>/packages/filesystem-0.2.0.tar.gz
conduit apply
# Plan: lock example.org/s1/filesystem 0.1.0 -> 0.2.0 (sha256:cbb831f0...)
#       -/+ Resource project replace: filesystem#directory 0.1.0 -> 0.2.0; filesystem-server @modelcontextprotocol/server-filesystem 2.1.0 -> 2.2.0; root unchanged
#           retire project (id X) via Pool workspace -> register project (new id) via management + Pool -> set up -> reopen agent-a-project, agent-b-project (same local name)
#           groups read, read-write identical in 0.2.0 (frozen-group rule); package 0.2.0 maps 3 operations; no Agent gains a tool (append_file and copy_file are never presented)
#           agent-a, agent-b: no access to project from the retirement to the reopening (seconds); history, the Agents and root's Conduits unchanged
```

```diff
# T6: conduit.yaml, 1 line changed (<lab>/shared-new exists, as in the baseline's lab)
-  project: {from: example.org/s1/filesystem#directory, arguments: {root: <lab>/shared}}
+  project: {from: example.org/s1/filesystem#directory, arguments: {root: <lab>/shared-new}}
```

```sh
# T6
conduit apply
# Plan: -/+ Resource project replace: arguments.root <lab>/shared -> <lab>/shared-new; relationships reopened under the same local names; tools unchanged; window as in T5
```

```diff
# T7a: revoke. Remove 1 line (agent-a now holds read-write after T4)
   agent-a:
-    project: example.org/s1/filesystem#read-write
     history: example.org/s1/history#post-read
```

```sh
# T7a
conduit apply
# Plan: - agent-a project (read-write): close.v1 agent-a-project; agent-a loses project.*; agent-b unchanged
# (check: agent-b still reads and writes project; agent-a's turn is refused with the two-tier reason)
```

```diff
# T7b: restore the same line (counted once in the task)
   agent-a:
+    project: example.org/s1/filesystem#read-write
     history: example.org/s1/history#post-read
```

```sh
# T7b
conduit apply
# Plan: + agent-a project: read-write; register and open a new Conduit (new id), tools project.* as before
# Had agent-a-project been closed OUTSIDE the file (e.g. conduit call access.close.v1), apply would instead report:
#   ! agent-a project closed by root at <time> (record <id>), not by an apply: not reopened; use --reopen agent-a-project or remove the line
```

```yaml
# Final conduit.yaml after T7 (19 lines)
apiVersion: github.com/shrav-k/conduit/v0
name: workspace
resources:
  project: {from: example.org/s1/filesystem#directory, arguments: {root: <lab>/shared-new}}
  history: {from: example.org/s1/history#history, arguments: {root: <lab>/history}}
agents:
  agent-a: example.org/conduit/agent-claude-code#claude-code
  agent-b: example.org/conduit/agent-codex#codex
conduits:
  root:
    workspace: github.com/shrav-k/conduit/core#pool-admin
    agent-a: example.org/conduit/agent-claude-code#converse
    agent-b: example.org/conduit/agent-codex#converse
  agent-a:
    project: example.org/s1/filesystem#read-write
    history: example.org/s1/history#post-read
  agent-b:
    project: example.org/s1/filesystem#read
    history: example.org/s1/history#post-read
```

## Counts
Counted with diff over the scratchpad states (<scratch>/relfirst/). Rule: non-blank lines, comments excluded, no `---` (one file, no separators), each line counted once per task. "Cmd lines" are commands that carry a value the author chose (archive paths), which the measurement plan counts as authored. `apply` and `reach`, run in the project directory, carry none.

| Task | File lines | Files edited | Author commands (requests) | Cmd lines | Total lines | Baseline | vs baseline | Coordinator | Engine operations inside |
|---|---|---|---|---|---|---|---|---|---|
| T1 | 16 | 1 (+ lock, generated) | 2 (add, apply) | 1 | 17 | 33 | less (-16) | 154 | 3 install, 1 register, 5 open, 4 set-up |
| T2 | 3 | 1 | 2 (add, apply) | 1 | 4 | 16 | less (-12) | 73 | 1 install, 1 register, 2 open, 1 set-up |
| T3 | 0 | 0 | 1 (reach) | 0 | 0 | 0 | same lines; 1 command against the baseline's 10 items read + 1 command | 0 | inspect |
| T4 | 2 | 1 | 1 (apply) | 0 | 2 | 8 (+1 restart) | less (-6); 0 s without access; no session lost | 2 (8 requests, ~30 s gap each) | 2 swap |
| T5 | 0 | 0 (lock regenerated) | 2 (add, apply) | 1 | 1 | 3 (+2 recovery steps) | less (-2); no tool gained | 62 | 1 install, retire, register, set-up, 2 open |
| T6 | 1 | 1 | 1 (apply) | 0 | 1 | 2 | less (-1) | 29 (~80 s gap) | retire, register, set-up, 2 open |
| T7 | 1 (2 edits) | 1 | 2 (apply, apply) | 0 | 1 | 2 | less (-1) | 12 | close; register + open |
| All | 23 | 1 distinct authored file | 11 (coordinator 35) | 3 | 26 | 64 | less on 6, same on 1 (T3), more on 0 | 332 | |

Sensitivities (honest):
- Strictest reading, every command a line (each `apply` too): T1 18, T2 5, T4 3, T5 2, T6 2, T7 3; 33 in all. The comparison then becomes less on 4 tasks (T1, T2, T4, T5), the same on T3 and T6, and +1 on T7. That still meets the bar (most less, none much more). T7 by commands (`conduit revoke`/`grant`, which edit the file and apply) would be 2 = baseline.
- Versions written in the file instead of the lock only (a `packages:` block of `address: version`): T1 +4, T2 +1, T5 becomes a 1-line edit plus `add`. Total about 31; still passes.
- Root's three relationship lines derived by the engine instead of written: T1 13 (by the strict reading); total 23 or 30.
- T6 by re-pointing to a second Resource instead of editing `root`: about 4 lines against 2. That would be more, so the edit route is the one to use.
- Time without access: T4 none (atomic swap). T5 and T6 have a window of seconds from the retirement to the reopening. This is worse than the coordinator's T5, which had none but renamed the tools; better than its T6 (~80 s). It is removable only by an in-place update (A67), which relaxes OBJ-011 and PKG-005.
- Provider and core work, counted apart: about 149 canonical lines. Filesystem 0.1.0 and 0.2.0 add 29 each, history 23, each Agent package 24, core 20.
- These are paper counts: nothing was run (ruling 45). A fresh author's run is needed.

## Lost
1. **The file no longer shows what it grants.** `read` means what the package's frozen group lists. Operation lists, kinds, owners, consumer and target kinds, versions and digests now live in packages, the lock and generated views (render, plan, reach). Ruling 52's explicitness is met only by what the engine shows, never by the text alone. A reader without the tool, or without the package, cannot tell what a line grants.

2. **Two forms of the language.** The project form and the canonical documents must both be specified and kept in step. That means new vectors, `check_schemas.py` and loader work, and diagnostics that map back to the author's lines. Inference rules replace explicit text:
   - the target kind is inferred from a unique name;
   - a consumer key that the file does not declare (`root`) is resolved among existing actors;
   - the local name defaults to the key;
   - the owner defaults to the project's Pool;
   - Conduit names are generated as `<consumer>-<local>`.

3. **A new compound verb with state and new failure modes.** apply.v1 brings:
   - a per-project applied record and its revision;
   - a three-way diff, with conflict parts for Conduits closed outside the file;
   - partial applies that stop and are not rolled back;
   - stale plans, and concurrent applies that end in conflict.

   It reverses round 6 D11 ("not an apply of several") and qualifies DD2. "Submitting only registers" survives only for register.v1. Round 6 warned against compound verbs that hide effects; this depends on the plan being shown and on every part being reported.

4. **Break-before-make replacement.** T5 and T6 retire and re-register `project` under the same name. That gives a window of seconds with no access, and the records show two IDs under one name. Agents mid-turn see refusals during the window. An instance whose shutdown stays unknown keeps its name and stops the apply.

5. **G9 weakened for relationships.** An Agent's access is a set of standalone Conduits, not declared ones. Opening them before the Agent's setup becomes an apply ordering promise (APL-010), not a part of the instance's own setup. An Agent set up outside apply may run without its access (fail closed). Its definition no longer documents its access, so inspection of consumed Conduits (order 027) becomes essential.

6. **Groups are coarse and frozen.**
   - Packages must export and version groups. Widening one means a new group name.
   - A grant of exactly one operation needs a local Conduit template or a group with an `operations` input.
   - There are no usersets or groups of consumers, so N Agents with the same access cost N lines unless an Agent template with declared Conduits is used.

7. **Trust on first use for digests.** `conduit add` records the archive's digest, and the author no longer copies or compares one. The pin is still checked at every later install (PKG-026).

8. **Root's lines cannot be removed.** Under ACC-024, removing a root relationship line has no effect unless ACC-024 is narrowed. Root's three lines are written by hand (3 lines of T1), which is explicit but is authority the owner already holds.

9. **Ruled shape set aside for authored files.** The per-document Kubernetes envelope (rulings 31 and 37) no longer applies to authored files, and `apiVersion` appears once per file (ruling 35's wording). Owner-amendable shape, not safety.

10. **Kept.** OBJ-002, OBJ-011, INS-020, PKG-005, LIF-008, LIF-009, LIF-035, and with them G1, G2, G4 and G5 in full; G3 through APL-004; G6, strengthened by frozen groups; G7 and G8. Nothing is changed in place.

## Composable
It composes with the following:

- **Compact or keyed project-form designs.** This is one, organized by relationship. The `conduits:` map can sit under any frame proposal.
- **Explicit plan/apply (4-01's kubectl/Terraform option).** apply.v1 here is that, restricted to existing operations plus swap.v1.
- **Tool-written lock (P8 / D17's tooling half).**
- **Package-exported templates (P3 via packages).** Packages carry Resource, Agent and Conduit templates.
- **Stable local names (P6).** The `as` argument is independent and useful alone.
- **Root authority by default (P7 / default-Pool / ACC-033-for-root designs).** Any of them would remove the 3 root lines. Under this design the Pool is the project, as Compose's project, Helm's release or Azure's resource group is.
- **In-place update / restart verb (A67).** It would turn T5 and T6's replacement into a restart that keeps the IDs and removes the window. That relaxes OBJ-011's second clause and PKG-005, so it is optional.
- **Reach/why views (P9) and ruling 52's two-tier refusals.** The plan pre-check uses the same decision code.
- **Field ownership / three-way conflicts (P10).** This is APL-004.
- **Rulings 50 and 51, pending actions.** A dry-run plan submitted by an Agent can be the evidence of a pending action at the root Interface.
- **Peer mode (ruling 51).** A relationship line whose target is on a peer engine expands into this engine's half, and the peer's own project file holds the other half.
- **Ruling 49.** An expected-account argument is one more argument per Agent line, no extra line.

It does not depend on JSON or YAML choice (SCH-013). Its line count needs both the short form and apply. Either alone fails T1/T2 or T5/T6, as the floor analysis found.

## Verdict counts (minor)
The headline count holds with fixes. I diffed the scratchpad states myself and get the claimed numbers: 23 file lines (T1 16, T2 3, T3 0, T4 2, T5 0, T6 1, T7 1 in 2 edits) plus 3 command lines carrying chosen values (one `conduit add` each in T1, T2 and T5), 26 in all against the baseline's 64. Under the measurement plan's own rule (a command counts only if it carries a value the author chose), that is less work on 6 tasks, the same on T3, and much more on none. On paper, then, it meets the owner's bar. Every task's end state is reached, and the person can request turns through root's `converse` lines. Several parts are wrong or hand-waved, though: 1) The design's own count.py does not give its table; it prints T4=5. 2) The "strict reading still passes" sensitivity is misstated. It leaves out T3's `reach` command, and against the better baseline for T5 (2 lines) it leaves only 3 of 7 tasks with less work, so it fails "most". 3) The shipped Agent templates break INS-004, so T1's bare Agent lines would be rejected until the provider files are fixed. 4) The T3 answer to reach around the engine is cut short for B and is less complete than the coordinator's. 5) T2's posting convention depends on an IFC-033 change made only "recommended". 6) APL-003's "held package versions (by the lock)" can be read so that T5 replaces the history too. 7) The T7 check text expects B to write, but B has been read-only since T4. 8) The request count (11) leaves out installs, dry-run plans and confirmations. None of these makes the primary count worse than the baseline on any task. The margins on T5, T6 and T7 are 1 line each, and nothing has been run.

RECOUNT
(I diffed the files in <scratch>/relfirst/states/ with `diff` and applied the baseline's rule from experiments/007-s1-baseline/results-2026-11-09.md:122-124: non-blank lines added, changed or removed, each counted once per task.)

| Task | File lines | Command lines (chosen values) | Total | A70 | Better baseline (T4/T5) | Result |
|---|---|---|---|---|---|---|
| T1 | 16 (t1.yaml: 16 non-blank, no separators) | 1 (`conduit add` with 3 archives) | 17 | 33 | - | less |
| T2 | 3 (3 added) | 1 | 4 | 16 | - | less |
| T3 | 0 | 0 | 0 | 0 | - | same |
| T4 | 2 (lines 15c15 and 18c18) | 0 | 2 | 8 (+1 restart) | ~8 | less |
| T5 | 0 (lock is generated; FIRST_BUILD.md: "Nothing generated by a tool counts as authored") | 1 | 1 | 3 | 2 | less (by 1) |
| T6 | 1 (4c4) | 0 | 1 | 2 | - | less (by 1) |
| T7 | 1 (the same line removed, then re-added; 2 edits, as the baseline counted its own T7) | 0 | 1 | 2 | - | less (by 1) |
| All | 23 | 3 | 26 | 64 | | less on 6, same on 1, much more on 0 |

The arithmetic in the design's table checks out (17+4+0+2+1+1+1 = 26). The three supporting sums also check: provider lines 29+29+23+24+24+20 = 149; commands 2+2+1+1+2+1+2 = 11; the final file has 19 lines.

DISCREPANCIES AND ERRORS IN THE COUNT
1. The evidence script does not reproduce the table. `python3 -I count.py states` prints T1 16, T2 3, T3 0, T4 5, T5 0, T6 1, T7 1, total 26. difflib pairs the swapped T4 lines badly, so it reports 5 instead of 2. Its "26" is therefore 26 file lines, not the design's 23 file + 3 command. The two totals match only by coincidence. The correct count, by `diff`, is T4 = 2 and 23 file lines.
2. The strict-reading sensitivity is misstated.
   - The design gives 33. It leaves out T3's `conduit reach`; with it the total is 34. T3 is then 1 against 0, or the same if the baseline's "at least 1 command more" is counted too.
   - Against A70 the strict reading gives less on T1, T2, T4 and T5 (4 of 7), the same on T6 (2 = 2), and +1 on T7 (and T3).
   - FIRST_BUILD.md's counting rules also require comparing T5 with the better baseline (allow lists plus `--read-only`, 2 lines, no recovery). Strict T5 is 2, which equals it, so only 3 of 7 tasks have less work and the bar's "most" fails.
   - So the pass rests on `apply` and `reach` not counting as lines. That is the measurement plan's rule ("carries a value the author chose"), so the primary reading stands, but the margin is thin.
3. The sensitivity for deriving root's lines removes 4 lines, not 3 (the `root:` key goes too). That gives T1 = 13 on the primary reading and totals of 22 or 29/30, not "13 (by the strict reading) ... 23 or 30". The "versions in the file" sensitivity comes to about 32, not 31. Both are minor.
4. Flow-style YAML: block style would add about 3 lines to T1 and 3 to T2 (about 32 in all). The verdict survives, and T4 to T7 are unchanged.

TASK COMPLETENESS
- T1: A has the `read` group (file.read.v1, directory.list.v1). B has `read-write` (adds file.write.v1). Root has `converse` to both Agents, so turns can be requested. One provider defect: provider/agent-*-templates.yaml put `default: false` inside the input's `schema`. INS-004 rejects omitting an input that declares no `default` at input level and is not marked `optional` (INS-021 places `default` beside `schema`). The type schemas require accountServices and peerMessaging with no default. As shipped, the bare lines `agent-a: ...#claude-code` and `agent-b: ...#codex` would be rejected. The fix is provider-side, with 0 author lines, but the design's provider files are wrong.
- T2: The structure is fine: history Resource, `post-read` for both Agents, row-1 registration through `root-workspace`. The posting convention is hand-waved. Today IFC-033 sets a tool's description from the Operation contract, so the file-history.v1 type description reaches no Agent. The design relies on a change it makes only "recommended". If an engine does not adopt it, the convention (6 lines in the baseline) has no place in the project form. The coordinator's run had the same gap, so parity holds, but the claim is not secured.
- T3: The answer is 0 lines and 1 command, but its coverage is overstated.
  - Agent-b's reach around the engine is cut short ("type codex.v1 as configured ...").
  - `conduit reach` can only repeat the type descriptions. Those descriptions do mention apply_patch and user-input requests for B. They omit what the coordinator's answer (coordinator-run/t3-answer.md) found from 2 READMEs and the closeout: both Agents run as the owner's OS user, both use the shared subscription login, B reads ~/.codex/AGENTS.md (3-07), and Codex lists collaboration tools despite multi_agent being off (4-06).
  - A complete answer still needs those reads, or provider work on the descriptions that is not in the 149 lines.
  - It also needs the new inspection of consumed Conduits (order 027), which does not exist yet.
- T4: A `read` -> `read-write` and B `read-write` -> `read` by atomic swap. This is correct. Minor point: the plan widens A before narrowing B, so for a moment both Agents can write. APL-002 does not say narrowing goes first.
- T5: No Agent gains a tool. The 0.2.0 package maps only 3 operations, and frozen groups guard later releases. One hand-wave: APL-003 replaces any instance whose "held package versions (by the lock) differ". The history package pins filesystem 0.1.0 (conformance/fixtures/s1/packages/history/conduit.yaml), and its type's implementation is filesystem#filesystem-server, so the history instance holds filesystem 0.1.0 through NAM-014. Read literally against a lock that now says 0.2.0, history and its 2 Conduits would be replaced as well. That contradicts the plan's "history ... unchanged" and widens the gap without access. The rule needs to say it compares only the packages the project names directly. A window of seconds without access is conceded, which is worse than the coordinator's T5. Also, the T5 operations column omits the 2 new Conduit registrations: retiring the Resource closes and removes them (CON-008, LIF-035).
- T6: One line changes `root`, and the replacement keeps access under the same local names. The same seconds-long window applies.
- T7: The line is removed (close.v1), then the same line is added back (register and open a new Conduit). Counted once, this is consistent with the baseline's rule. Error: the T7a check says "agent-b still reads and writes project", but B is read-only since T4 (states/t7-revoked.yaml: agent-b project#read). As written, the check would expect a write that must fail. Also, A's turn is not refused: A's project tools are withdrawn, and a stale call through them would be refused.

LINES OR REQUESTS FORGOTTEN, AND THEIR EFFECT
- Manifest and pins: the lock is generated by `add`, so 0 lines is legitimate. The digests shown are stale, though. Adding templates and exports to the packages repacks them, so every digest changes, history's filesystem pin changes, and 0.1.0 cannot be re-released with different bytes (PKG-026), so the versions move. This affects provider work, not author lines.
- Package installation: counted (3 command lines). Today's `install` takes ARCHIVE#SHA256, which the fresh author typed. `add` drops the digest (trust on first use), as the design admits.
- `--store DIR` is required by every command of today's CLI (reference-model/crates/shell/src/main.rs: "--store DIR is required"). If it is kept, `apply` and `reach` carry a chosen value and the strict reading applies. It is arguably a harness value.
- Requests are undercounted.
  - `add` with 3 archives makes 3 install.v1 requests.
  - Each `apply` is a dry-run plan plus the apply, and probably a read of the revision (APL-001, APL-007).
  - `reach` makes at least 1 inspect request.
  - That is about 20 to 22 engine requests, not 11, plus 7 y/N confirmations by a person. Still well under the coordinator's 35.
- Not reported: concepts met, grants made by hand, duplicated configuration (about 5 package addresses repeat in the final file), time and mistakes. The bar is about the author's work, not lines alone.
- Provider shift: about 149 canonical lines (including 20 core) move into packages. FIRST_BUILD.md says "S1's own project, its templates, instances and Conduits, is the author's". The count is valid only if package-exported groups are accepted as provider work, which is defensible as the analogue of the server's `--read-only`. That must be stated next to the 26.
- Ruling 49 (each Agent's expected account) will add an argument per Agent, and a sign-in step per Agent under ruling 50. In flow style this adds no line, but it is not in the rewrite.
- Paper only: nothing was run (ruling 45). The fresh author's cost, which was 646 + 130 lines today, is not modelled.

Files: <scratch>/relfirst/count.py, <scratch>/relfirst/provider/agent-claude-code-templates.yaml (INS-004 defect, same in agent-codex-templates.yaml), spec/promises/instantiation.yaml (INS-004, INS-021), spec/interface/implementation.yaml (IFC-033), conformance/fixtures/s1/packages/history/conduit.yaml (filesystem 0.1.0 pin), docs/implementation/FIRST_BUILD.md (measures, better-baseline rule), experiments/007-s1-baseline/results-2026-11-09.md (counting rule, T3/T5/T7).

## Verdict safety (major)
The design fails as written, though every problem found can be fixed. Its core holds. No object changes in place. Every access is still an open Conduit decided at one target. A swap is recorded as a close plus an open. Apply adds no authority beyond the caller's and runs no reconciler. But two of its headline safety claims do not hold. First, "an upgrade can never widen a grant (T5)" is false: the freeze covers only a group's capability and operations, and leaves out its filters, the package's Resource, Agent and implementation definitions, the declared Conduits of package templates (including tool-less get-secret ones) and local templates. Second, revocation (G3) can be undone by several paths the text permits: APL-004 is keyed to the Conduit object rather than to the relationship, the applied record lives with a Pool that can be removed, a stale copy of the file can be applied over a later revocation, and the plan that runs is not bound to the plan that was approved. There are also an implicit widening through the one defaulted Pool per project, a disclosure path through dry-run plans, and a name-reuse hole once a swap renames a Conduit. None of this is fatal: every fix is spec text or engine output, and almost none adds author lines (S1 stays at about 26). Severity: major.

SCOPE. I read spec/ (ACC-001..039, ACC-005, ACC-029 plus bound.v1 in spec/core/requirements.yaml, OBJ-002/006/009/010/011/013, IFC-012/013/014/033/038, LIF-009/035/037, CON-004/008, TPL-012, SEC-008, OPS-009/032, REC-002, POOL-001, NAM-006/022/024, PKG-029, INS-025). I also read docs/proposals/multi-machine/README.md, rulings 49 to 52, round-6 D11 and D12, the S1 package fixtures (conformance/fixtures/s1/packages/*), and the design's scratchpad files (<scratch>/relfirst/). Nothing was changed.

WHAT SURVIVES (checked; the gaps below are listed separately)
- No object changes in place. OBJ-002, OBJ-011, INS-020 and PKG-005 hold.
- Each change of a relationship is a close plus an open, so ACC-011 and ruling 41 item 3a are kept.
- ACC-033's single-target rule is kept, and apply is authorized part by part with no added authority. Setup parts are a precedent for parts that are the caller's own requests recorded as serving a compound invocation (ACC-005, CON-004, REC-002).
- Nothing runs without a request (APL-009).
- Every change still lands as recorded register, open, close and retire events.

COUNTEREXAMPLES, most severe first

1. [HIGH, G3] APL-004 protects Conduit objects, not relationships.
- Problem: APL-004 says a Conduit closed outside apply "is never reopened". But swap.v1 "closes one Conduit and registers and opens another", and a replace "reopen[s] its relationships under the same local names". Both make NEW Conduits, so neither is a "reopen" of the closed one.
- Trace: the operator runs `access.close.v1 agent-b-project` (an emergency revocation). Later the author makes T4's edit (agent-b read-write -> read) and runs `conduit apply`. APL-003 classifies the line as "group changed: swap". The old Conduit no longer exists (LIF-035). Either the swap reports invalid (OPS-029) and the apply stops partway, or it opens the new read Conduit, and agent-b regains access the operator revoked.
- Same path for T5 and T6: replacing `project` reopens every relationship to it, including one closed outside the file.
- Fix: write APL-004 per relationship (consumer, local name). If its last Conduit was closed by anything other than an apply part, no part of any class (add, swap, replace-reopen) may give it a new Conduit without `--reopen`. A swap whose old Conduit is not open reports conflict.

2. [HIGH, G1/G2/G5] Package templates carry access that neither the file nor the plan shows.
- Problem: the one-line Agent `agent-a: example.org/conduit/agent-claude-code#claude-code` instantiates whatever the package's template declares. Package templates may declare Conduits whose targets are resolved by name at instantiation; the core package does exactly this, in spec/core/templates.yaml lines 103-109 (`target: {kind: Resource, name: management}`).
- Effect: apply's set-up part is root's own request (ACC-005), so each declared Conduit opens under root's unbounded `access` (ACC-022). A declared Conduit listing get-secret.v1 even passes SEC-008, because root sent the setup. It presents no tool (IFC-012), so APL-002's "tools each Agent gains" never shows it, yet the engine reads the secret into the Agent's run (SEC-010).
- Upgrades: a later release of the Agent package can add such a Conduit. The apply after `conduit add` replaces agent-a, and the new Agent gets it. NAM-0xx freezes only Conduit templates.
- Fix:
  - (a) A template used through the project form may declare no Conduits and no nested instantiations, so all access is relationship lines. Or, at least, the plan, render and reach list every Conduit that opens, declared or tool-less, with its operations.
  - (b) A package template's references must come from inputs, never from names resolved among existing objects.

3. [HIGH, G1/G6] "An upgrade can never widen a grant" is false. NAM-0xx compares only `capability` and `operations`. Each of these widens with every op list identical:
- (a) A group's `requirements` (filters) or input defaults change. Example: 0.1.0's `read-docs` has a path filter that 0.2.0 drops. The T5 replace reopens the relationships with the 0.2.0 template and nothing flags it.
- (b) A Resource template's config, or the implementation's `packageArguments` or `tools` map, change. @modelcontextprotocol/server-filesystem accepts several allowed directories, so a 0.2.0 manifest that adds a second positional root (say $HOME) lets agent-a's `read` read $HOME. The plan says "root unchanged; no Agent gains a tool".
- (c) An Agent template flips `accountServices` or `peerMessaging` to default true (ruling 48). The file never names them. The lock bump replaces the Agent with reach around the engine turned on.
- (d) The freeze is checked only "against an installed release". There is no uninstall, but a fresh engine, a peer engine or a rebuilt store that never held 0.1.0 accepts a 0.2.0 whose `read` lists file.write.v1.
- Fix: freeze the whole canonical template of every export the project form uses (groups, Resource and Agent templates: requirements, inputs, defaults, config, declared Conduits). Record in the lock a digest of each export used, plus the expanded operation list, and compare against the lock, not only against installed releases. Every replace caused by a lock change shows a full definition and implementation diff, and any widening needs a flag.

4. [HIGH, G2 / test 8 / ruling 4] One implicit Pool per project widens access silently.
- Problem: PRJ-002 and PRJ-003 make every Resource the file declares owned by the one Pool `name`; there is no `owner` key.
- Pool bounds: under ACC-029 a bound naming that Pool covers "every Resource it owns, including those instantiated after". So adding a Resource line, say `vault`, silently puts it under every existing delegate whose `access` bound or Pool Conduit names `workspace`. The file shows no owner, and the plan shows "owner workspace" but not who gains authority.
- Lost expressiveness: putting Resources in different Pools for different teams can no longer be written.
- First apply when a Pool of that name already exists: OBJ-002's name is free once the old Pool is removed. If the apply adopts the existing Pool instead of failing, the other party's Pool Conduits and bounds cover the owner's Resources.
- Fix:
  - an optional `owner:` per Resource, with several Pools allowed;
  - first apply fails, and never adopts, when the Pool name exists;
  - the plan lists, for each new Resource, every Conduit and bound that comes to cover it.

5. [HIGH, G3] The applied record lives with the Pool, so losing the Pool erases the protections.
- APL-004's "never reopened" and "never made again" both depend on that record (APL-008). If an operator retires the whole Pool (retire-with-resources.v1) and it is removed, the next routine `conduit apply` is a "first apply". The plan is "+ everything", exactly like T1, and with `--yes`, from CI or by an Agent holding apply.v1, it recreates the project. That includes Agents and relationships the operator revoked; the Agents are not even owned by the Pool.
- Two directories, or two authors, with the same `name` share one record. Relationship removal needs no flag (APL-003), so each one's apply closes the other's Conduits.
- Fix:
  - keep applied records apart from the Pool, in the records or as a permanent object (LIF-019 style);
  - a first apply under a name that has history requires `--recreate`;
  - the project's identity is an ID that is checked, not the Pool's name.

6. [HIGH, G3/G10] A stale copy of the file reverts later applies.
- Problem: APL-007's revision proves only that the plan is fresh, not that the file is current.
- Trace: author 1 removes agent-b's write line and applies (revision 1 -> 2). Author 2 (or an Agent working from an older checkout) plans at revision 2, so the plan is not stale. Its three-way diff against author 1's applied form shows "+ agent-b project: read-write", and APL-004 does not apply, because an apply part did the close. One "y" re-grants the revoked write. Terraform has the same hole; Conduit's preservation tests 5 and 8 make several actors the norm.
- Fix: the lock (or the file) records the applied revision it was last synced from. An apply whose base is older than the current revision reports conflict and names every part that would reverse a later apply (who, when, record ID), unless the reversals are confirmed with a flag.

7. [MAJOR, ruling 52 / G10] The approved plan is not bound to the executed plan.
- Problem: apply.v1's inputs are the project, the lock and the revision (APL-001). The server derives the parts from "the objects as they stand". Closing a Conduit, retiring an instance or registering a new external object do not bump the project's revision.
- Effect: between plan and "y", what runs can differ from what the person saw. For example, a newly registered Agent `reviewer` (a name the file uses as an external consumer) gets a Conduit nobody was shown.
- Fix: apply carries a digest of the approved plan (parts with resolved IDs). The server recomputes and reports conflict on any difference, as Terraform does for a saved plan.

8. [MAJOR, ruling 52's exception / ACC-033's last paragraph / OPS-023] A dry-run plan works as an inspection oracle.
- Problem: a caller holding management {apply.v1} but no Pool authority (row 6) can still dry-run a file that names arbitrary objects.
- What it learns: whether they exist, their `from`, their argument values ("root <lab>/shared -> ..."), and, per the T7b example output, who closed a Conduit, when, and the record ID. ACC-033 promises such a caller "learns nothing of whether the object existed".
- Fix: build the plan only from what the caller may inspect (inspect.v1 under each object's own row). Objects outside that set appear only as "refused", in the first tier.

9. [MAJOR, G3 / OBJ-002 / LIF-037] The swap cannot keep the generated name, which opens a name-reuse gap in revocation.
- Problem: a swap with no window must register the new Conduit while the old one still holds `<consumer>-<local>`, and OBJ-002 is unchanged. So the new Conduit gets another name, and render's `agent-a-project` no longer names what is live.
- Trace: after T4, X (holding management register.v1 plus an `access` bound over X's own Pool) registers a decoy Conduit named `agent-a-project`. The operator runs `close.v1 agent-a-project`: it succeeds on the decoy, while the real read-write Conduit stays open. close.v1 cannot carry `expectedId` (LIF-037 covers set-up, retire, inspect and unregister only).
- Fix: either swap atomically transfers the name (an explicit, narrow exception to OBJ-002), or revocation is addressed by (consumer, local name), with LIF-037 extended to close.v1 and open.v1. Render and reach show the live names and IDs.

10. [MAJOR, G3 intent] Plan order can widen before it narrows.
- Problem: APL-006 runs parts in plan order and stops at the first failure, with no rollback. T4's printed plan swaps agent-a (read -> read-write) before agent-b (read-write -> read).
- Effect: if the second swap fails or ends unknown, both Agents keep write. Even when both succeed, there is an interval in which both write; "no decision sees both" holds per Conduit, not across the plan.
- Fix: run every close and narrowing part before any open, widening or replace, or swap the set atomically. Report the interval.

11. [MAJOR, G3] Revoking through the file is hostage to the whole apply.
- APL-005 is all-or-nothing, so one revocation line is blocked by any unrelated refused part. Examples:
  - a removed root line, whose close ACC-024 refuses, so the apply is refused (the design's "has no effect" contradicts APL-005);
  - a pending lock bump whose replace the caller cannot authorize;
  - "conflict parts", whose effect on the rest of the apply is never defined.
- Removing a line whose Conduit is already closed is a close of a removed object, which reports invalid (OPS-029). So the clean-up apply after an emergency `close.v1` fails.
- Fix: closes and narrowings are never blocked by other parts. Conflict parts are reported and skipped. Removing a relationship that is already closed counts as succeeded. A root line can neither be removed nor have its group changed, and the file is rejected up front, with a diagnostic, rather than at apply.

12. [MAJOR, G6 / OBJ-006 / OBJ-010] Meaning depends on engine state, and existing names can be reused.
- Problem: PRJ-004 drops the kind that OBJ-006 requires for consumer and target. It resolves a consumer the file does not declare to "exactly one existing Interface or Agent". The same file therefore means different actors on different engines; a peer engine's `root` (ruling 51) is a different Interface.
- Reuse trace: external consumer `reviewer` is retired and removed, and another actor registers an Agent named `reviewer`. The operator answers the conflict with `--reopen` and grants the impostor.
- The same happens when a consumer is replaced: other projects' `--reopen` grants a new definition they never vetted.
- Fix: a name the file does not declare carries its kind (for example `Interface/root:`, at no extra line). The plan flags any change of ID, kind or definition behind a name since the last apply. `--reopen` takes an expected ID.

13. [MAJOR, conformance: "never silently ignores part of a document"] Edits to `placement` or `runtime` are ignored.
- Problem: PRJ-003 accepts `placement` and `runtime`, but APL-003's replace class compares only `from`, arguments and held versions.
- Effect: switching agent-a to a confining runtime provider, the only route to an "enforced" restriction (ACC-027), or moving it to another node, plans as "no changes". The file then says one thing while another runs.
- Fix: both are replace triggers, shown in the plan.

14. [MAJOR, 4-01 returns] Local templates have no lifecycle under apply.
- Problem: PRJ-001 allows local Conduit templates beside the file, but APL-003 has no class for adding or changing a template. The freeze covers only package exports.
- Effect: a replace re-expands each relationship's `from` "as it now stands", as LIF-009 does. If `my-read` was edited (or replaced by another actor's replace.v1), an unrelated T5 or T6 replace widens agent-a, and the plan's "groups identical" line is wrong.
- Fix: a replace's reopen copies the closed Conduit's resolved operations and requirements unless the line changed. Local templates are immutable under apply (a change takes a new name), or covered by the freeze.

15. [MEDIUM, determinism / availability] Transitive pins make "held package versions (by the lock) differ" ambiguous.
- Facts: history 0.1.0 pins filesystem 0.1.0 (conformance/fixtures/s1/packages/history/conduit.yaml), and its type is served by filesystem#filesystem-server.
- After T5 the lock says 0.2.0, so one of two things happens:
  - read literally, history is replaced on every apply, because a new history still holds 0.1.0. The plan never converges, and each apply cuts both Agents off from history;
  - or history stays on server 2.1.0, while the plan reports "history unchanged" and the operator believes the server was upgraded.
- Fix: compare only the version of the package whose template the instance is made from. The plan lists the packages that remain pinned transitively.

16. [MEDIUM, G9] Idempotency is lost across a replace.
- Background: S1-key-scope (increment-2 plan.md:576) makes a request to a different target a new request.
- Problem: T5 and T6 keep tool names while the target ID changes. A write that ended unknown on the retired instance, retried with the same key through `project.file.write.v1`, is dispatched again on the new instance over the same directory, so the effect can happen twice.
- Fix: a replace records the lineage, and a keyed repeat that names a predecessor reports conflict until the old invocation is reconciled. At minimum, the apply result names the in-flight and unknown invocations of what it retired.

17. [MEDIUM, ruling 52 / G13] Stable local names hide a change of target.
- Problem: IFC-033's definitions do not name the target. A swap of `target` (or the T6 replace) keeps identical names and definitions, so IFC-038 sends no listChanged. An Agent in the middle of a task reads from X and writes to Y.
- Squatting: during T7's revocation window, any actor with an `access` bound over its own Resource can open a Conduit for agent-a with `as: project`. It points at that actor's directory under the old tool names (marginally easier than today's freed-name route).
- Fix: each tool's description names the target and its ID. Any change of the Conduit behind a name sends listChanged. The local names of a project's relationships are reserved to the project's apply.

18. [MEDIUM, A65 / multi-machine] "No access decision sees neither or both" assumes one place of decision.
- The multi-machine table (docs/proposals/multi-machine/README.md:96-100) has v1 decision points deciding from leased copies, with a close taking effect "within a stated bound everywhere". A narrowing swap would leave the old, wider Conduit in force on some machines for that bound.
- Fix: promise atomicity per decision point, with the swap's close under the same revocation bound as a close.

19. [MEDIUM, G4] What a revision records is too thin for audit.
- APL-008 keeps one current applied form. Whether apply.v1's inputs are recorded depends on its contract's `records` (REC-002), and the design does not say.
- Lost: the history of declared intent, the digest of the approved plan, and the flags (`--reopen`, `--recreate`, `--retire`), which are the only trace that a re-grant was deliberate. The records also show two IDs under one name after every replace.
- Fix: the apply record holds the RFC 8785 digest of the expanded project, the plan digest and the flags. Applied forms are kept append-only, per revision.

20. [MEDIUM, ruling 49] The expected account is missing.
- The provider Agent templates have inputs only for `accountServices` and `peerMessaging`, so there is no expected account and the check of ruling 49 point 3 cannot run.
- Every Agent replace, including any lock bump of an Agent package, destroys the Agent's home and login, and the plan never says so.
- Fix: a required `account` input (agents stay one line as maps). Replace parts state "home and sign-in lost; a sign-in will be pending at root".

21. [LOW] The order of a replace is contradictory.
- APL-003's replace runs set up and then reopen, while APL-010 opens relationships before setup. A replaced Agent is set up before its own Conduits are open.
- Fix: open Conduits whose consumer is the instance before its setup, and Conduits whose target is the instance after it.

22. [LOW] The project directory becomes an authority input.
- conduit.yaml and conduit.lock are re-sent on every apply, so a line added by an Agent with write access to that directory rides along with the next routine apply, and with `--yes` nobody sees it.
- Fix: the plan highlights every widening, `--yes` refuses widening parts, and the CLI warns when a declared Resource's root contains the project directory.

23. [LOW] IFC-033 type descriptions are provider text in the Agent's context.
- The recommended change puts each type's contract description into tool descriptions. That text can change between package versions while the plan reports "no tool gained".
- Fix: show description changes as plan parts.

24. [LOW, ruling 4/51] Cross-party work cannot be written.
- A cross-party relationship (two halves on two engines) has no form in the project file. This narrows scope but is not unsafe.

EFFECT OF THE FIXES ON THE LINE COUNT
- Most fixes are spec text and engine output: the plan, render and reach, record contents, and ordering.
- In the file:
  - kinds on names the file does not declare change keys only (0 lines);
  - `owner` is optional (0 lines in S1);
  - an Agent's `account` fits into its one-line map (0 lines).
- S1 stays at about 26 lines against the baseline's 64, so the bar is unaffected.
- The fixes that cost the design something are:
  - the whole-template freeze, or a ban on declared Conduits in project-form templates (items 2 and 3);
  - per-relationship APL-004 and an applied record that survives the Pool (items 1 and 5);
  - plan-digest binding and base-revision checks (items 6 and 7).

## Verdict cost (major)
The design holds, with fixes. It can be built as an additive superset of today's language: canonical documents, packages, register.v1 and the vectors keep their meaning. But the work is large, and the design understates it. About 50 of the 455 promises are touched: roughly 25 new and 25 amended. Of the amended ones, about 12 are not named by the design (SUB-005, SUB-010, MAN-002, MAN-011, OPS-022, REC-002, REV-001/REV-003, LIF-013, ACC-036, OBJ-002 if a swap reuses the name, the templates of root's first-start Conduits, and the IFC-014 precedence rule). The work also needs 2 new core operations, 2 new core templates, 4 to 7 new Schema contracts and 55 to 80 new vectors. Expect about 4,500 to 7,500 lines of Rust and Python, with apply.v1 in the pure core as the largest piece. About 20 to 45 existing vectors would have to be regenerated mechanically, plus 91 more files if the S1 fixture packages are repacked in place. Migration is mostly backward compatible, with three forced breaks: `conduit.yaml` would mean two things, existing stores cannot add apply or swap to root's permanent first-start Conduits, and repacked package archives get new digests. For an implementer the spec gets more complex: about 12 concepts are added and none removed, and two forms must be kept in step. Only the author's side gets simpler. Before any promise is written, four defects in the proposed text must be fixed. As worded, APL-004 lets a later apply silently reopen a revoked grant (a break of G3). APL-003 makes the history Resource be replaced again on every apply after T5. Removing one of root's lines blocks every later apply. A swap that reuses a Conduit's name collides with OBJ-002.

Scope: I only read the repository <home> Promise texts were extracted from spec/promises, spec/conventions and spec/interface (455 promises). Vector counts come from conformance/manifest.yaml (367 cases).

== 1. DEFECTS IN THE DESIGN'S NEW TEXT (fix before writing promises) ==

D1. APL-004 as worded leaves a G3 hole, so a revoked grant can be reopened. The text covers only "A Conduit that the last apply opened". Counterexample:
- Apply 1 (T1) opens agent-a-project.
- Apply 2 (T2) opens only the history Conduits.
- Root then runs access.close.v1 on agent-a-project.
- At apply 3 (say T4, which edits agent-b only), the last apply did not open agent-a-project, so APL-004 does not apply.
- APL-003 has no class for "line unchanged, object missing". It is either undefined, so independent builds disagree, or read as "relationship added": register plus open, which silently reopens the grant.

The fix: "a relationship in the applied form whose Conduit is now closed, and which no apply part closed (counting closures that CON-008 makes as a result of an apply part's retirement), is a conflict part unless named in `reopen`". Also add "line unchanged, object missing" explicitly to APL-003 as a conflict class.

D2. APL-003's replace trigger ("held package versions (by the lock) differ") never converges.
- The history Resource holds filesystem 0.1.0 through its type (conformance/conventions/s1/whole/case.yaml:1006-1012: message-history holds both example.org/s1/filesystem 0.1.0 and example.org/s1/history 0.1.0).
- The pin comes from history's own manifest (conformance/fixtures/s1/packages/history/conduit.yaml).
- After T5 the lock lists filesystem 0.2.0. So history differs, and apply replaces it, against the design's claim that history is unchanged.
- The new instance again holds filesystem 0.1.0 (NAM-009 and PKG-005 through history's template), so every later apply retires and re-registers history, each time with a window without access.

The fix: compare only the version of the package that exports the instance's `from` template with the lock's entry for that address.

D3. Removing one of root's relationship lines blocks every later apply.
- The removal becomes a close part.
- ACC-012 and ACC-024 refuse to close any Conduit whose consumer is root.
- APL-005 checks all parts before dispatching any, so the whole apply is refused, not "has no effect" as the design says. A swap of a root line (a group change) fails the same way.

The fix: an APL rule that a removed or changed line whose consumer is root is a conflict part, reported and not dispatched. Or narrow ACC-024, the design's optional item.

D4. A swap that reuses the name collides with OBJ-002.
- PRJ-004 names both the old and the new Conduit `<consumer>-<local>`, e.g. agent-a-project.
- OBJ-002 rejects a document whose kind and name an existing object has, and the old Conduit exists until the swap closes it.

There are two ways out:
- (a) swap.v1 takes the new document, resolved "as if the closed Conduit no longer exists". This needs an amendment to OBJ-002 and IFC-014. It also needs swap's input to carry a document and a manifest (NAM-007), so `access` would resolve documents and need `management` authority.
- (b) Register the new Conduit first under a generated name (OBJ-003), then swap.v1 {close A, open B}, which is pure `access` and authorized under ACC-012 plus ACC-006. This needs IFC-014 to allow a registered, not-yet-open Conduit to share its tool names with an open Conduit of the same `as`, and the check moves to open time.

I recommend (b): it is smaller and leaves OBJ-002 and the single-target rows alone. The cost is that render's Conduit names become generated, not `<consumer>-<local>`.

Lesser fixes:
- `<consumer>-<local>` can exceed OBJ-001's 63 characters, so it needs a rejection or shortening rule.
- An object of the same name, made outside the file, has an ID that differs from the applied form. Apply must report a conflict, not adopt it silently (OBJ-010's spirit). This needs a new APL rule.
- swap's records must include the registration as well as the close and the open (REC-001 lists "an object is registered").
- APL-005's authority pre-check is a prediction. Each part must still be decided at its own dispatch (ACC-036), and apply stops if a part is refused then.
- `as` needs a precedence rule against a declared local name (the round 4 edge: a standalone Conduit can block register-replacement.v1).
- Root's first-start templates must list the new operations (see section 4).

== 2. PROMISES THAT MUST CHANGE (read and confirmed) ==

Amended, named by the design: OBJ-013 (standard argument `as`), IFC-012 (last sentence), ACC-033 (rows for apply.v1 and swap.v1), REC-001, MAN-001, NAM-007 (checked against the lock), IFC-033 (recommended), and ACC-024 (optional).

Amended, missed by the design:
- SUB-005: a submission may hold only templates, instances, contracts and one manifest, and the project file is none of these. It also needs file, line and key-path attribution (PRJ-005).
- SUB-010: "the manifest is the file named conduit.yaml".
- MAN-002: must also reject conduit.lock beside a canonical manifest.
- MAN-004, or a new rule: the lock holds only apiVersion and packages, so a project in this form can hold no contract or implementation (MAN-015).
- MAN-011: reading a directory.
- OBJ-024: expanded documents take the file's apiVersion.
- OBJ-019: `as` becomes a defined key.
- OBJ-002: only under option (a) of D4.
- IFC-014: the swap exclusion, `as` precedence, and the open-time check under option (b).
- IFC-038 and ACC-036: no list served and no decision made that sees neither or both Conduits of a swap.
- OPS-022: its [when] covers only set-up.v1 and retire.v1, and APL-006 relies on it.
- REC-002: `serving` for the parts of an apply and a swap.
- REV-001 and REV-003: REV-001 says "exactly one revision ... no second revision", so the applied revision must be the Pool's, and REV-003 must say that a change to the applied form advances it.
- LIF-013 and inspect.v1: the applied form (a fourth output, not part of `status`, since OBS-001 and the meaning of status rule that out) and the reach view.
- NAM-024 (observable): a group never gains operations.
- NAM-027 (light): generated names count in cross-kind uniqueness.

New promises:
- PRJ-001 to PRJ-008: 8.
- APL-001 to APL-010 as corrected, plus about 2 more (root lines as conflicts; no adoption of same-name objects): about 12.
- The frozen-group rule, NAM, with its install rejection under PKG-023: 1 or 2.
- swap.v1's authorization, atomicity and reconciliation: 2 or 3.

That makes about 24 to 26 new.

Total touched: about 48 to 55 promises, 10 to 12% of 455.

Also changed:
- definitions.md: about 10 rows. The rows project, manifest, register, replace ("not an apply of several") and tool/local name change. The rows project file, lock, group, apply, plan, swap and applied form are new.
- README.md: the Contents table, the reading paths, and the layer table, if APL becomes an eighth convention. The README states "seven shared conventions" (LD3). To keep seven, file APL under conventions/observation.yaml as the explicit request that OBS-004 anticipates.

== 3. SCHEMAS (spec/schemas) ==

- New spec/schemas/project.yaml with project.v1 and its entry and relationship schemas: 2 to 4 contracts.
- The lock: manifest.v1 plus a rule, or a new lock.v1: 0 or 1.
- instances.yaml instance.v1: `as` for a Conduit.
- core results.yaml: plan-part.v1 (operation, target, authorizing Conduit, outcome, IDs made and removed, tools gained and lost): 1.
- inspect.v1's outputSchema: applied form and reach.
- document.yaml and envelope.yaml: unchanged.
- examples/: about 15 to 25 new valid, invalid and invalid-by-rule files.
- Core exports go from 102 to about 108-110.
- tools/check_schemas.py (2,588 lines): is_manifest_file and role_of must tell the project file from a manifest, and the new examples need validating. Size: small-medium, about 200-400 lines.

== 4. CORE PACKAGE (spec/core) ==

- operations.yaml: add apply.v1 and swap.v1, each with inputSchema, outputSchema, effect and deduplicates; change inspect.v1.
- capabilities.yaml: object-management.v1 gains apply.v1, and access-control.v1 gains swap.v1.
- templates.yaml: add `pool` and `pool-admin`. root-management and root-access list "every operation" of their capability (templates.yaml:112-145), so they must add apply.v1 and swap.v1, or root cannot use the new verbs. The design does not mention this.
- conduit.yaml: exports, and the engine implementation's list.
- core/README: the export table ("7 templates" becomes 9, 23 Operation contracts become 25).
- FST text is unchanged, but its fixtures change (section 5).
- Size: medium.

== 5. CONFORMANCE ==

Invalidated, all regenerated mechanically:
- conformance/fixtures/root-system-state.json (referenced by 198 files) and about 20 case directories that embed root's operation lists: first-start/{initial, repeat, lifecycle}, language/first-start/protected, records/{purge, purge-nonroot, purge-succeeded, input-reference}, access/{capability-skew, collision-missing-contract, entry-authority, fixed-kind-references}, s1/whole, registration/{entry-active-replacement, entry-retired-only, owner-retired, placement-retired, runtime-retired}, and others.
- access-bounds, if ACC-024 is narrowed.
- 2 to 6 snapshots of tool definitions under IFC-033 (e05-local and e05-target-version carry descriptions).
- In all, about 22 to 45 cases.
- If the S1 fixture packages are repacked with groups in place: the filesystem 0.1.0 digest 200de7c1... appears in 91 conformance files, and history's manifest pins it, so that is a further cascade. Avoid it by shipping the groups as new fixture packages.

Not invalidated:
- The frozen-group rule breaks no case: no package in the vectors exports a Conduit template.
- `as` defaults to today's local name.

New cases, about 55 to 80 (+15 to 22% on 367):
- Project form and expansion, with attribution and errors: 15 to 25.
- Lock: about 5.
- `as`: 3 to 5.
- Frozen groups: 3 or 4.
- swap (atomicity, authority, IFC-014, root refusal, records, reconciliation): 6 to 8.
- apply (each change class, conflicts, reopen/retire/recreate, stale and missing revision, stop on failure or unknown, refused pre-check, no reconciler): 15 to 25.
- Reach view: 2 or 3.
- S1 end to end in the project form: 1 or 2.

Also: profiles/first-build.yaml, and the manifest's safety_rows for the new G3 traces.

== 6. REFERENCE MODEL (reference-model/crates) ==

| Area | Work | Size |
| --- | --- | --- |
| loader (2,879 src + 2,144 tests) | project.rs (directory role, conduit.lock), new project-form module: expansion with a source map (positions already exist in diagnostic.rs), lock reader, schemas.rs | medium, about 800-1,300 lines with tests |
| core (10,465) | apply module: 3-way diff, change classes, authority pre-check reusing access.rs routing, parts as invocations with `serving`, stop rule, applied form and revision on the Pool, conflict detection from records (about 1,500-2,500). swap in access.rs, registration.rs and records.rs, requests.rs for outcome and reconciliation (300-500). `as` (local_name already exists: registration.rs:1903, access.rs:103, lifecycle.rs:347, schemas.rs:1107; 30-60). Frozen groups at install, packages.rs and archives.rs (100-150). Inspect applied form and reach (150-300). lib.rs Input and first start (small). IFC-033 description (about 30) | large, about 2,300-3,700 lines (+22-35%) |
| conformance (2,741) | new case kinds: expansion/render, plan or dry run, apply steps | small-medium, 200-400 |
| shell (6,986 + 3,297 tests) | author.rs and main.rs: add, plan, apply (flags, y/N, exit codes for 4-13), render, reach. store.rs for persistence. server.rs for one listChanged per swap. tests/changes.rs (851) rewritten for apply routes | medium, 1,200-2,000 |
| packages/agent-* and S1 fixtures | Agent template plus `converse` (about 24 lines each), repacked dist, READMEs. Fixtures: groups in new packages or versions (about 76 lines) | small |
| Untouched | runner, identity and gates, archives, strict parsing, idempotency, export of records | none |

Total: about 4,500 to 7,500 lines, in roughly 5 or 6 work orders: the spec batch, loader plus check_schemas, core swap/as/groups, core apply, the shell, and vectors with fixtures. **Overall: LARGE.**

== 7. MIGRATION ==

The language is an additive superset, so canonical documents, packages and register.v1 keep their meaning. Three breaks are forced:

1. `conduit.yaml` would mean two things. Today it is the manifest (MAN-001, SUB-010, and every package archive). In the project form it is the project file, and the manifest becomes conduit.lock. Telling the two apart by content (`name` versus `packages`) or by whether conduit.lock is present keeps old directories valid, but one filename then has two meanings, and the `.lock` ending needs a syntax rule under SUB-005 and SCH-012. A distinct name for the project file avoids this.
2. Root's first-start Conduits are permanent and cannot be closed (FST, ACC-024; LIF-009 serves only retired declared Conduits). An existing store can never add apply.v1 or swap.v1 to them. It needs a fresh first start, or root must open extra Conduits for itself. The cost is small, since the first build's stores are throwaway, but the spec should state it, because every future management verb hits the same problem.
3. Repacking S1 or Agent packages with groups changes their digests (PKG-004), so the coordinator's and fresh author's manifests and 91 vector files stop matching unless new versions are published.

The design's own lock shows today's digest 200de7c1... for a filesystem 0.1.0 that would now carry groups. That cosmetic mismatch follows from break 3.

== 8. READING COST FOR AN IMPLEMENTER ==

Concepts added (about 12):
1. The project form and its expansion.
2. The lock.
3. A persistent project, meaning its Pool, its applied form and its applied revision. This changes "project" from what is handed over in one registration.
4. The relationship line, and `as`.
5. The group and the frozen rule.
6. swap.
7. apply: its plan, parts and change classes.
8. Conflict parts and the reopen, retire and recreate flags.
9. Replacement by retiring first and then re-registering under the same name.
10. Inference rules: target by a unique name, consumer among existing actors, owner by default, generated names.
11. The reach view.
12. The core templates pool and pool-admin.

Concepts removed: 0. Promises go from 455 to about 480 (+5 to 6%), with about 25 more rewritten, and two forms must be specified and kept in step. For the author the concepts met drop sharply: no template/instance split, frame, reference kinds, owner, digests, or choice between declared and standalone Conduits. But the spec as read gets more complex, not simpler.

== 9. CHEAPER STAGING (for the owner) ==

Stage A, spec-light:
- Normative: `as`, swap.v1 (option b), frozen groups, core pool and pool-admin, packages shipping groups. About 8 to 10 promises touched.
- Tooling only: the project form, and a CLI apply that composes existing operations. This is allowed by DD5, and the measurement counts the authored file whichever tool reads it.

Stage B: make PRJ and APL normative once a fresh author's run confirms the line count.

Caveat: without the engine-side applied form, APL-004's G3 protection rests on the tool reading records, and with several actors or machines (ruling 4, G16) a local state file is not robust. Engine-side apply is the right final shape, and it is the bulk of the cost.

== 10. RULINGS (owner decisions this cost depends on) ==

- DD2 (ruling 22): qualified.
- Round 6's "Automatic apply ... key inference ... Do not add to make examples look shorter" (design-critique.md:655): apply is explicit, but PRJ-004 is key inference.
- D11 concerns replace.v1 only (DECISIONS.md:115). apply.v1 sidesteps it rather than reversing it.
- The envelope of rulings 31 and 37 applies only to canonical documents.
- Ruling 35: apiVersion once per file.
- ACC-024, if narrowed.
- An eighth convention, if APL is not filed under observation.