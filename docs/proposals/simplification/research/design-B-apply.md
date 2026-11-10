Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Apply: revisioned objects, swapped Conduits (change model first)

Today, changing something in Conduit means creating something new. This design makes change its own act, explicit and recorded. The author edits the project's files and runs `conduit apply`. The engine first computes a pure plan: a diff of the documents against the objects this project holds, ordered, with every consequence shown, much as `terraform plan` and `kubectl diff` do. It then carries out exactly that plan on an explicit request that names the plan's digest.

Every part of a plan is one of five acts:
- an atomic swap of a Conduit under the same local name: never an edit, so close followed by open, decided at one point;
- an in-place update of an instance's definition: a new revision, with its run restarted if needed;
- a control;
- a replacement, only for identity fields;
- a new registration.

Each part is authorized and recorded exactly as the explicit route it stands for would be. Immutability moves from objects to their revisions, and stays with Conduits:
- An open Conduit is still never edited.
- Earlier definitions are kept, and every record names the revision that served it.
- Nothing closed is reopened unless a request names it.
- An in-place change is permitted only where register, close and reopen would be.

On S1, T4 to T7 drop from the coordinator's 105 lines and 24 requests to 4 lines and 6 commands. That is below the baseline on all four, with no Conduit window and no session lost. But the change model cannot fix T1 (60 lines against 33): its excess is the document frame, root's turn Conduits and the Pool, and none of those are change costs.

## Spec changes
NEW: spec/conventions/apply.yaml (layer: convention), operations in spec/core/operations.yaml.

- APL-001 `plan.v1`, a new operation of `object-management.v1`, authorized against `management` as `inspect.v1` is.
  - Inputs: project name, documents, contracts, manifest (digests taken from the lock).
  - Pure: it changes nothing and dispatches nothing. It sits in the pure core (ruling 46).
  - Returns the ordered parts. Each part gives:
    - the kind of act;
    - the object and the revision it read;
    - the field diff, as pointer, old value and new value;
    - the promise that classified it;
    - the Conduit that would authorize it, or ruling 52's two-tier refusal.
  - Consequences shown for each part: tools gained and lost per consumer; tool names kept or changed; Conduit IDs closed and opened; runs restarted; conversations ended; any interval in which calls report unavailable.
  - Notices: drift (APL-007), orphans (APL-008), and instances of other projects made from a changed template (left unchanged, INS-020).
  - The plan's digest covers the project digest (SUB-008), every revision read and every Conduit ID read.
- APL-002 Identity.
  - Within a project, a document and an object of the same kind and name that the project holds (APL-012) are the same object.
  - A document that names an object the project does not hold is rejected, as OBJ-002 states, naming who holds it.
- APL-003 Classification of each change. Nothing is done where the documents and the object are equal. Otherwise:
  - (a) A Conduit whose consumer, target, capability, operation export names or filters differ is swapped (APL-004). A difference only in the package versions it resolves through changes nothing (NAM-017, NAM-023).
  - (b) A setting that a control of the type covers is changed by that control.
  - (c) Any other change to an instance's configuration, package pins or implementation selection is an update with a restart (APL-005).
  - (d) A change only to an instance's declared Conduits is an update with no restart; each changed Conduit is a swap, an open or a close.
  - (e) A change of `owner`, `type`, `placement` or `runtime` is a replacement. The new instance is made first, every Conduit to and from the old one is swapped to it, then the old one is retired.
  - (f) A changed template is replaced as `replace.v1` replaces one, and every instance of this project made from it is classified again.
- APL-004 Swap.
  - It closes a Conduit and opens its successor at one decision point (ACC-036). No request is decided between the two.
  - Requests dispatched before the swap finish under ACC-034.
  - The successor keeps the local name, or the object name for a standalone Conduit, whose predecessor is removed in the same step under LIF-035. Tool names under IFC-012 are therefore kept, and IFC-014 does not count the predecessor.
  - It is decided as a close (ACC-012) and as an open (ACC-006, ACC-030 bound, SEC-008). The consumer is notified (IFC-038). Both IDs are recorded and linked.
  - It is also offered as `swap.v1` of `access-control.v1`.
- APL-005 Update in place.
  - The stored definition is replaced and the revision advances (REV-003). The earlier definition is kept as a revision until a purge (REC-004), and inspection lists the revisions with the invocation that made each.
  - With a restart:
    - Every check that opening each Conduit to and from the instance makes (CON-022, IFC-014, IFC-033) is made against the new definition first. A failure stops the part before anything changes.
    - Each Conduit the instance consumes is decided again as an open, under ACC-006 and SEC-008, and the decision is recorded.
    - In-flight invocations end first. New requests report `unavailable`, and can be retried, until the new run is ready.
    - The run is stopped, then started, keeping one run per instance.
    - Conduits keep their IDs, and no model work is started.
- APL-006 Authority equivalence. A part is permitted exactly where the explicit route it stands for would be:
  - update or restart: register (ACC-033 rows 1–2), set-up and retire (rows 4–5), and closing and opening every Conduit to and from the instance (row 10);
  - swap: close and open;
  - replacement: all of those, for both instances;
  - derived Conduit: open.
  - Where a Conduit lists `get-secret.v1`, only the root Interface, acting on its own behalf, is permitted (SEC-008).
- APL-007 No reopening.
  - A Conduit of the project that a request outside this project's applies closed is reported with who closed it, when, and in which invocation. It is left closed.
  - It is reopened only where the request names it in `reopen`, or by `register-replacement.v1` and `open.v1`.
- APL-008 Nothing is retired by being left out of the documents, unless `prune` names it.
- APL-009 Order and failure.
  - Order: pure narrowings and closes first; then updates, restarts and replacements; then widenings and new opens.
  - A part that fails, is refused or ends unknown stops every later part that depends on it, and every widening.
  - There is no rollback (LIF-027). The outcome is given per part and overall, as OPS-022 gives it.
  - Every Conduit is, at every moment, in its old form or its new one.
- APL-010 `apply.v1` (`object-management.v1`).
  - It names the plan's digest. The engine computes the plan again; where the digest differs, it dispatches nothing and reports conflict (REV-002).
  - Otherwise it dispatches exactly the parts, as the caller. Each part is an invocation that serves the apply (REC-002 `serving`).
- APL-011 Derived routes.
  - Where a part needs the caller to hold a Conduit to a Pool, and the caller has none but its Conduit to `access` would permit opening one, the plan adds that Conduit.
  - It lists the `pool-management.v1` operations that the caller's bound names. It belongs to the project, and the plan shows it as derived, with this rule.
- APL-012 Project holdings. Each object an apply registers holds, as status the system owns, the project's name and the invocation that last changed it. An object belongs to one project.
- APL-013 Explicitness (ruling 52). The plan and the result carry everything APL-001 lists, as it happened.

CHANGED:
- OBJ-002: `register.v1` still only adds. A document of an applied project that names an object the project holds is a change under APL-002.
- OBJ-011: names stay fixed. Its second clause becomes: no document rewrites an instance's definition except an apply part, which keeps the earlier definition as a revision.
- SUB-007: a template is replaced by `replace.v1` or by an apply part.
- INS-001, INS-002, OBJ-012, OBJ-013, OBJ-014: inline definitions.
  - A document is a template if and only if it has `spec`.
  - An instance has either `from`, or its definition at the top level: `type`, `config` and `conduits`; for a Conduit, `capability`, `operations` and `requirements`. It is made from an anonymous template it holds, so "every instance has a template" (ruling 8 IN9, ruling 18) still holds in the model, though the author no longer writes it.
  - A declared Conduit may hold `capability` and `operations` in place of `from`.
- INS-020: instances of the project whose template that project's apply changes are changed by that apply. All others keep their definitions.
- PKG-005: an instance holds the pins of its current revision.
- LIF-009: a replacement is made from the nested instantiation as the instance holds it at its current revision, no longer "from the template as it now stands". This removes 4-01's coupling between holders of a shared template; no record gives a reason for the old clause.
- REC-001: new events for apply, swap and restart.
- REC-002: adds the target's revision at dispatch.
- ACC-033: rows for `plan.v1` (`management`, as inspect) and `apply.v1` (`management`, with each part under its own row, per APL-006).
- Core: `root-management` and `object-management.v1` gain `plan.v1` and `apply.v1`; `root-access` and `access-control.v1` gain `swap.v1`.
- IFC-014: the predecessor of a swap is not counted.
- Manifest: a package entry may give the version alone. A lock that a tool writes (`conduit.lock`, as `terraform init` writes `.terraform.lock.hcl`) gives the digest, and the request still carries both, so NAM-006 and PKG-004/006/026 are unchanged on the wire. Installation stays explicit (D17).

UNCHANGED:
- `register.v1` only registers (DD2 holds for it).
- `replace.v1` and `register-replacement.v1`, for actors that do not use apply.
- The Kubernetes frame (OBJ-012, `apiVersion` on every document).
- Explicit operation lists (G6) and one capability per Conduit.
- LIF-035: a closed Conduit is removed. Ruling 29's lifecycle, with no new state.
- ACC-022, ACC-024, FST-*, IFC-012 naming, the Pool and `owner`, IN4 for reusable templates.
- No reconciler and no automatic apply: drift is shown, never repaired.

RULINGS TO AMEND (the owner's):
- 21 (DR5: no rewrite through the registry).
- 8 IN9 and 18 V2: one-off instances get inline definitions.
- Round 6 D11 ("not an apply of several").
- Round 6's row "automatic apply ... do not add". This design is not automatic: it is D08's "pure preview of the required explicit actions", made an operation so that Agents can use it (preservation test 5).
- It takes A67's restart verb, but only as a part of apply, and its "address owned by the project that first submitted it", as APL-012.

WHAT IMMUTABILITY PROTECTED, AND HOW APPLY KEEPS IT:
- G1, no silent redirect (DD1, OBJ-009/010). IDs are still held and never re-resolved between applies. Names resolve to the project's own holdings, and a re-point is a plan part shown before it runs.
- G2, an open Conduit never changes (RR14, ACC-011). Every Conduit change is a swap: close and open, with the open decided with its bound.
- G3, nothing reissues a revoked grant (RD2, ruling 29, LIF-008). APL-007: drift is reported, never reopened.
- G4, code under an opened grant changes only when that grant is decided again (DR5/L109, ACC-030 "afterwards", SEC-008). APL-005 decides the consumed Conduits again; APL-006 makes the equivalence; get-secret.v1 stays root-only.
- G5, pins (ruling 27). The lock holds digests; an upgrade is an explicit pin change shown in the plan.
- G6, no widening by upgrade. Operations are listed by export name; the T5 plan states that no Conduit gains one.
- G7, stale knowledge never overwrites (REV-*). The plan's digest covers every revision, and a stale plan is a conflict.
- G8, records (REC-*). One record per part, linked to the apply, with the target's revision.
- G9, Conduits open before a run (TPL-012). The restart re-decides them first.
- Revocation is still immediate (OL1). Test 6 holds: there is no background apply, and a restart starts no turn.

## Rewrite
Design (variant B): apply, inline definitions and derived Pool routes. The envelope, the Pool and root's turn Conduits are kept. Lines are counted as the measurement plan counts them.

```harness (not counted; the same as in both runs)
conduit init
conduit serve
```

## T1: 60 lines in 2 files, 4 commands

```yaml T1 <lab>/project/conduit.yaml (5 lines)
apiVersion: github.com/shrav-k/conduit/v0
packages:
  example.org/s1/filesystem: 0.1.0
  example.org/conduit/agent-claude-code: 0.1.1
  example.org/conduit/agent-codex: 0.1.1
```

```yaml T1 <lab>/project/project.yaml (55 lines)
apiVersion: github.com/shrav-k/conduit/v0
kind: Pool
metadata:
  name: workspace
type: github.com/shrav-k/conduit/core#pool.v1
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: shared
owner: workspace
type: example.org/s1/filesystem#local-directory.v1
config: {root: <lab>/shared}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: agent-a
type: example.org/conduit/agent-claude-code#claude-code.v1
config: {accountServices: false, peerMessaging: false}
conduits:
  project:
    target: {kind: Resource, name: shared}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: agent-b
type: example.org/conduit/agent-codex#codex.v1
config: {accountServices: false, peerMessaging: false}
conduits:
  project:
    target: {kind: Resource, name: shared}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-a
consumer: {kind: Interface, name: root}
target: {kind: Agent, name: agent-a}
capability: example.org/conduit/agent-claude-code#conversation.v1
operations: [example.org/conduit/agent-claude-code#turn.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-b
consumer: {kind: Interface, name: root}
target: {kind: Agent, name: agent-b}
capability: example.org/conduit/agent-codex#conversation.v1
operations: [example.org/conduit/agent-codex#turn.v1]
```

```sh T1 commands (4 requests)
conduit install <lab>/packages/filesystem.tar.gz#200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160
conduit install <lab>/packages/agent-claude-code.tar.gz#166ce271e4c5e83ff13c228fe56075f1fa7f98557b633dbe60f4d2fe1ee60e9e
conduit install <lab>/packages/agent-codex.tar.gz#05b47a8571743789ae5b9fc060ce1cb43777b92c61206a617cf49334aee626d6
conduit apply <lab>/project
```

```text T1 generated by the tool, not authored: <lab>/project/conduit.lock, and the plan shown before the confirmation
lock     example.org/s1/filesystem 0.1.0 sha256:200de7c1...; agent-claude-code 0.1.1 sha256:166ce271...; agent-codex 0.1.1 sha256:05b47a85... (each installed)
register Pool/workspace, Resource/shared (owner workspace), Agent/agent-a, Agent/agent-b, Conduit/turn-a, Conduit/turn-b   [management.register.v1]
derive   Conduit/root-workspace: Interface root -> Pool workspace, every operation of pool-management.v1   [APL-011: setting up Resource shared needs a Conduit to its owning Pool (ACC-033 row 4); root's Conduit to access permits opening it; its consumer is root, so it cannot be closed (ACC-024)]
open     turn-a, turn-b, root-workspace
set up   workspace; shared through root-workspace.set-up.v1; agent-a and agent-b, each opening its declared Conduit project
tools    agent-a: project.file.read.v1, project.directory.list.v1 | agent-b: the same plus project.file.write.v1
Apply this plan? [y/N]
```

## T2: 17 lines in 2 files, 2 commands

In `conduit.yaml`, 1 line is added under `packages:`:

```yaml T2 conduit.yaml, line added
  example.org/s1/history: 0.1.0
```

In `project.yaml`, 16 lines are added:
- 4 under agent-a's `conduits:`;
- 4 under agent-b's `conduits:`;
- 8 for the new Resource document, its `---` included.

The whole file after T2:

```yaml T2 <lab>/project/project.yaml after T2
apiVersion: github.com/shrav-k/conduit/v0
kind: Pool
metadata:
  name: workspace
type: github.com/shrav-k/conduit/core#pool.v1
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: shared
owner: workspace
type: example.org/s1/filesystem#local-directory.v1
config: {root: <lab>/shared}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: history
owner: workspace
type: example.org/s1/history#file-history.v1
config: {root: <lab>/history}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: agent-a
type: example.org/conduit/agent-claude-code#claude-code.v1
config: {accountServices: false, peerMessaging: false}
conduits:
  project:
    target: {kind: Resource, name: shared}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]
  history:
    target: {kind: Resource, name: history}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: agent-b
type: example.org/conduit/agent-codex#codex.v1
config: {accountServices: false, peerMessaging: false}
conduits:
  project:
    target: {kind: Resource, name: shared}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
  history:
    target: {kind: Resource, name: history}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-a
consumer: {kind: Interface, name: root}
target: {kind: Agent, name: agent-a}
capability: example.org/conduit/agent-claude-code#conversation.v1
operations: [example.org/conduit/agent-claude-code#turn.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-b
consumer: {kind: Interface, name: root}
target: {kind: Agent, name: agent-b}
capability: example.org/conduit/agent-codex#conversation.v1
operations: [example.org/conduit/agent-codex#turn.v1]
```

```sh T2 commands
conduit install <lab>/packages/history.tar.gz#e946746509490be49cbcc07d41ff5fed24a6e6c8b377bf423c77f79c02502f69
conduit apply <lab>/project
```

```text T2 plan (tool output)
register Resource/history (owner workspace) through root-workspace.register.v1 (the derived route already lists it: no mistake 2)
update   agent-a rev 1->2, agent-b rev 1->2: declared Conduit history added and opened; no run restarted; conversations kept
tools    each Agent gains history.file.read.v1, history.directory.list.v1 and history.file.write.v1. The names are stable: no history-a.* or history-b.*
```

## T3: 0 lines

```sh T3 inspection, as today
conduit tools --as agent-a
conduit tools --as agent-b
```

## T4: 2 lines in 1 file, 1 command

In project.yaml, the `operations` line of agent-a's `project` changes from:

```yaml
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]
```

to:

```yaml
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
```

The `operations` line of agent-b's `project` changes from:

```yaml
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
```

to:

```yaml
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]
```

```sh T4
conduit apply <lab>/project
```

```text T4 plan (tool output)
1 swap   agent-b/project  -file.write.v1 (narrowing, first)  agent-b loses project.file.write.v1
2 swap   agent-a/project  +file.write.v1 (widening)          agent-a gains project.file.write.v1
- update agent-a rev 2->3, agent-b rev 2->3 (declared Conduits only; no run restarted)
Tool names are kept. No interval without access: each swap is one decision (APL-004). Both sessions are kept, and each sees the new list through listChanged.
```

## T5: 1 line in 1 file, 2 commands

In conduit.yaml, `  example.org/s1/filesystem: 0.1.0` becomes:

```yaml T5
  example.org/s1/filesystem: 0.2.0
```

```sh T5
conduit install <lab>/packages/filesystem-0.2.0.tar.gz#cbb831f07d2b251c84fcd768a03353203a8a1b2215e862881fef47f866d84436
conduit apply <lab>/project
```

```text T5 plan (tool output)
lock      example.org/s1/filesystem 0.2.0 sha256:cbb831f0... (installed)
1 restart Resource/shared rev 1->2: example.org/s1/filesystem 0.1.0 -> 0.2.0; filesystem-server npm @modelcontextprotocol/server-filesystem 2.1.0 -> 2.2.0
          authorized as register, retire and reopen would be: management, root-workspace, root's access (APL-006)
          Conduits to it are kept with unchanged IDs: agent-a/project (read, list, write), agent-b/project (read, list); tool definitions unchanged (IFC-033)
          no Conduit gains an operation; 2.2.0's append_file and copy_file are mapped by no operation of package 0.2.0 and reach no one
          in-flight calls finish first; new calls report unavailable until the new run is ready
unchanged Resource/history (filesystem 0.1.0 through example.org/s1/history 0.1.0); agent-a and agent-b (their Conduits permit the same operations, NAM-023)
```

## T6: 1 line in 1 file, 1 command

In project.yaml, Resource shared's `config: {root: <lab>/shared}` becomes:

```yaml T6
config: {root: <lab>/shared-new}
```

```sh T6
conduit apply <lab>/project
```

```text T6 plan (tool output)
1 restart Resource/shared rev 2->3: /config/root <lab>/shared -> <lab>/shared-new (local-directory.v1 has no control for /root)
          Conduits to it are kept: agent-a/project and agent-b/project, which now reach <lab>/shared-new; tool names and sessions kept
```

## T7: 0 lines, 2 commands

```sh T7
conduit close agent-a/project
conduit open agent-a/project
```

The check of B runs between the two commands; checks are not counted.

The CLI resolves `agent-a/project` to the Conduit made for that declared Conduit:
- `close` is `access.close.v1`.
- `open` is `management.register-replacement.v1` (`instance: Agent/agent-a`, `nested: project`), built from the definition as agent-a holds it at rev 3 (read, list, write), followed by `access.open.v1`.

An optional `conduit plan <lab>/project` run between them is a check. It shows:

```text
drift  agent-a/project: closed by Interface root (access.close.v1, invocation recorded); the project declares it; not reopened (APL-007)
no changes
```

## VARIANT B+: B plus two borrowed fixes that are not about change

The two fixes:
1. The plan derives a turn Conduit from the registering Interface to each Agent it registers.
2. A default Pool is made at first start, with root's Conduit to it, and `owner` defaults to it.

T1 is 35 lines. conduit.yaml is as in B (5 lines). project.yaml (30 lines):

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: shared
type: example.org/s1/filesystem#local-directory.v1
config: {root: <lab>/shared}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: agent-a
type: example.org/conduit/agent-claude-code#claude-code.v1
config: {accountServices: false, peerMessaging: false}
conduits:
  project:
    target: {kind: Resource, name: shared}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: agent-b
type: example.org/conduit/agent-codex#codex.v1
config: {accountServices: false, peerMessaging: false}
conduits:
  project:
    target: {kind: Resource, name: shared}
    capability: example.org/s1/filesystem#directory.v1
    operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]
```

T2 is 16 lines: B's T2 without the `owner: workspace` line. T3 to T7 are identical to B, with the same commands.

## VARIANT A: apply only, today's document form, for comparison

T1 is 153 lines. The files are in `<scratch>/apply-design/A/t1/`. They are the coordinator's T1 files written right the first time:
- conduit.yaml: 11 lines, unchanged, with digests.
- templates.yaml: 92 lines. Differences from the coordinator's:
  - `read` and `read-write` in their T1 form;
  - the Agent templates renamed `claude-agent` and `codex-agent`;
  - `pool-admin` lists register.v1, register-replacement.v1, inspect.v1, set-up.v1 and retire.v1.
- instances.yaml: 50 lines, with `from:` renamed.
- There is no t1fix manifest.

T2 is 40 lines:
- Added to conduit.yaml (3 lines):
  ```yaml
    example.org/s1/history:
      version: 0.1.0
      digest: sha256:e946746509490be49cbcc07d41ff5fed24a6e6c8b377bf423c77f79c02502f69
  ```
- history.yaml (31 lines): the coordinator's `message-history` template, `history` instance and `post-read` template, without history-a and history-b.
- Added under each Agent template's `conduits:` (3 lines each):
  ```yaml
      history:
        from: post-read
        target: {kind: Resource, name: history}
  ```

T3 to T7:
- T4: `from: read` becomes `from: read-write` in claude-agent, and `from: read-write` becomes `from: read` in codex-agent (2 lines). Then `conduit apply`.
- T5: conduit.yaml `version: 0.2.0` and `digest: sha256:cbb831f07d2b251c84fcd768a03353203a8a1b2215e862881fef47f866d84436` (2 lines). Then install and apply.
- T6: instances.yaml `  root: <lab>/shared-new` (1 line). Then apply.
- T7: the same commands as B.

## Counts
Lines are counted as the measurement plan counts them: non-blank lines written or changed, `---` included, comments excluded. The files were counted by script. They are in <scratch>/apply-design/{A,B,Bplus}/.

| Task | Baseline A70 | Coordinator | Fresh author | A: apply only, today's form | B: this design | B, block-style lists | B+: B + 2 borrowed fixes | B files | B commands (non-check) | Coordinator requests |
|---|---|---|---|---|---|---|---|---|---|---|
| T1 | 33 | 154 | 263 | 153 | 60 | 65 | 35 | 2 | 4 (3 install, 1 apply) | 4 |
| T2 | 16 | 73 | 67 | 40 | 17 | 23 | 16 | 2 | 2 (install, apply) | 5 |
| T3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 inspection | 2 |
| T4 | 8 | 2 | 32 | 2 | 2 | 2 | 2 | 1 | 1 (apply) | 8 |
| T5 | 3 | 62 | 112 | 2 | 1 | 1 | 1 | 1 | 2 (install, apply) | 5 |
| T6 | 2 | 29 | 44 | 1 | 1 | 1 | 1 | 1 | 1 (apply) | 8 |
| T7 | 2 | 12 | 128 | 0 | 0 | 0 | 0 | 0 | 2 (close, open) | 3 |
| All | 64 | 332 | 646 + 130 script | 198 | 81 | 92 | 55 | | 12 + 2 inspection | 35 |

**Against the bar** ("less on most of S1's tasks, much more on none"):
- **A** is less on T4 to T7 (4 of 7), but much more on T1 (4.6x) and T2 (2.5x). It fails.
- **B** is less on T4, T5, T6 and T7, and equal on T3. It is barely more on T2 (17 against 16; 23 with block lists) and much more on T1 (60 against 33, 1.8x). It fails only the "much more on none" clause, and only on T1.
- **Where B's 27 extra T1 lines go:**
  - the per-document frame: 6 documents x (apiVersion, kind, metadata, name), plus 5 separators = 29 lines;
  - root's turn Conduits: 18 lines;
  - the Pool and `owner`: 7 lines.
  None of these is a change cost.
- **B+** adds two small fixes from other angles: derived turn Conduits, and a default Pool. It gives T1 35 against 33 (+6%), T2 16 against 16, and T4 to T7 less. It passes on paper with the Kubernetes envelope kept.
- **With a keyed project file** (the frame angle), T1 comes to about 14 to 16 and the whole of S1 to about 21 to 25. That is from the floor and prior-art sketches, not counted here.

**T4 to T7 alone:** B takes 4 lines and 6 commands, against the coordinator's 105 lines and 24 requests, and the baseline's 15 lines.

**Sensitivity: commands counted as lines.** Both runs and ruling 52 point 6 count a command as a request, not a line. If every command that carries a value the author chose counts as a line:
- B is 93: T1 64, T2 19, T4 3, T5 3, T6 2, T7 2.
- B+ is 67.
- T5 to T7 then tie with the baseline (3/3, 2/2, 2/2), so B and B+ are less only on T4. Even counting the baseline's restarts and recovery steps symmetrically (T4 9, T5 5 to 6), they are less only on T4 and T5.
- Under that reading, the bar needs commands that edit the file and apply in one step (`conduit set`, `kubectl set` / `cargo add` style). These are not part of this design.

**Other measures for B:**
- Time without access: none in T4. In T5 and T6, from the stop until the new run of `shared` is ready, calls report `unavailable` (retryable; one run per instance; not measured). The coordinator's T4 left each Agent about 30 s without access, and T6 about 80 s.
- Sessions lost: none in S1.
- Tool names: stable throughout (`project.*`, `history.*`).
- Grants made by hand: 2 in T1 and 2 in T2. Root's 2 turn Conduits are listed apart, and 1 root Conduit is derived, not written.
- Adapters: 0. Provider-specific lines: 0.
- Contract-address lines (listed apart): T1 12, T2 5, T4 2.
- Digests: 0 in files. They appear only in the 5 install commands, and the lock is tool-written. Typed into files instead, they add 3 in T1, 1 in T2 and 1 in T5.
- Requests per task: T1 4, T2 2, T3 2 (inspection), T4 1, T5 2, T6 1, T7 2 = 14.

## Lost
1. **Instance definitions are no longer immutable.**
   - Which definition an object has depends on its revision. Records need the target's revision, which this design adds, and old revisions must be kept for audits.
   - INS-020's blanket rule ("instances keep what they resolved") shrinks to instances outside the project being applied.
   - DR5 (ruling 21) and IN9 (ruling 8, as restated by ruling 18) are amended.
2. **Conduit IDs change on every swap.**
   - Only the local name, and so the tool names, is stable.
   - An outside reference to an old ID finds a removed Conduit; the record of the swap links the old and new IDs.
3. **A shared template inside a project becomes a fan-out change.**
   - Editing it moves every holder at the next apply.
   - The plan lists each holder, but the "frozen until replaced" reading is gone for instances in the project.
4. **A short window during a restart, and D09's hard cases.**
   - With one run per instance, the stop comes first. Calls report `unavailable` while a Resource restarts.
   - Draining in-flight invocations, hooks, and secret reads at the new run must be specified and tested.
   - A restart whose stop or start ends unknown leaves the instance at a new revision with an unknown run.
5. **Partial applies exist, and there is no rollback.**
   - The ordering means every Conduit is, at every moment, in its old or its new form, and no widening precedes a narrowing.
   - The system can still stop between the old and the new state.
6. **A new kind of state: which project holds which object.** It is what Terraform state, kubectl's last-applied and ApplySet hold.
   - An object belongs to one project.
   - Adopting an existing object is refused.
   - The project's name, defaulting to the directory as in Compose, becomes part of what identifies an object for apply.
7. **Authority gets more complex.**
   - `apply.v1` on `management` lets its holder change existing instances, which today's `replace.v1` cannot.
   - APL-006 keeps this equal to the explicit route of register, set-up, retire, close and open. But it is a multi-row authorization rule that must be specified exactly, and a mistake in it would be an escalation.
8. **Explicitness moves from what the author writes to what the engine shows.**
   - Inline instances hide the template.
   - A derived root-to-Pool Conduit is an object nobody wrote.
   - Ruling 52 is met only if the plan and the result are complete. An author who confirms without reading the plan loses that explicitness.
9. **"Submitting only registers" (DD2) holds only for `register.v1`.** The author's usual path becomes apply. `replace.v1` and `register-replacement.v1` stay, but authors rarely need them.
10. **It does not fix T1 or the frame.**
    - The frame, root's turn Conduits, the Pool, contract addresses and `capability` lines all remain.
    - Alone, the design fails the bar on T1.
    - Under a strict count that turns commands into lines, T5 to T7 only tie with the baseline.
11. **A large rebuild.**
    - Reference model: re-resolution in registration.rs, restart in lifecycle.rs, swap in access.rs, revisions and new events in records.rs, the compound outcome in requests.rs, and new shell commands.
    - Inline-instance schemas: templates.yaml and instances.yaml, check_schemas.py, and about 84 registration vectors.
    - Rewritten vectors: S1-duplicate, setup-close, setup-restore, records-replace and conventions/changes.
    - A new family of vectors for apply.
12. **Agent restarts still lose conversations.** The Claude Code package runs with `--no-session-persistence`, so an Agent configuration change (not in S1) ends its conversation unless the package changes. The plan says so, but sessions are kept only where no Agent run restarts.

## Composable
- **Compact or keyed project-file designs (the frame angle).**
  - `plan.v1` and `apply.v1` take any front end that expands into documents.
  - Diagnostics and plan lines point at what the author wrote.
  - Together they give about 14 to 16 lines for T1 and about 21 to 25 for S1.
- **Root and first-start designs.**
  - A default Pool, or root's authority over the Pools it makes, replaces APL-011's derived routes. It also removes the Pool document and the `owner` lines (B+).
- **Derived turn Conduits for the Interface that registers an Agent.**
  - This is the control panel of ruling 50, and part of B+.
- **Package-exported Conduit templates or operation sets.**
  - Examples: `example.org/s1/filesystem#read` and `#read-write`.
  - T4 stays a 2-word change, and the names stay true.
  - APL-003 must then flag any widening that a package upgrade causes as a plan part needing confirmation.
- **Ruling 50's pending actions.**
  - An Agent's plan can be raised at the root Interface for approval, with the plan as its evidence. Agents can run `plan.v1` and `apply.v1` under their own grants (test 5).
- **Ruling 52's two-tier refusals.**
  - These are used verbatim in the plan, for parts that would be refused.
- **Imperative helpers that edit the file and then apply.**
  - Examples: `conduit set`, `conduit grant` (P11).
  - These are needed only if commands are counted as lines.
- **A65's v1 leases.**
  - Conduits stay immutable values: a swap is a new value in the leased set.
  - Instance revisions only add the target's revision to records.
- **Ruling 51's peer engines.**
  - Each engine applies its own half of a relationship across parties.
- **A67.**
  - This design takes its restart verb as an apply part, with exact semantics.
  - APL-012's project holdings are a form of A67's "address owned by the project that first submitted it".

## Verdict counts (major)
Holds with fixes. A script recount of the design's files matches every per-task number. B is 81 lines: T1 60 (5+55), T2 17 (1+16), T3 0, T4 2, T5 1, T6 1, T7 0. B+ is 55 (T1 35, T2 16). A is 198. Counting each command that carries a chosen value as a line gives B 93 and B+ 67, as the design says. On paper every task's rewrite reaches the task as stated, and the person can still request turns, but six problems weaken the claims. (1) The T5 plan says agent-a and agent-b are unchanged, but the design's own rules restart them. With inline definitions each Agent's anonymous template names example.org/s1/filesystem, so under NAM-009 and PKG-005 the Agent holds pins for that package. APL-003(c) makes a pin change an update with a restart, and APL-003(a)'s "changes nothing" covers only Conduits. If (c) applies, A's conversation ends, because the Claude Code package runs with --no-session-persistence, so "no session lost in S1" does not follow from the rules. (2) Moving the copied digests from the manifest into the install commands drops them from the count, but ruling 52 point 6 counts copied digests as lines. That adds 5 lines: B becomes 86 and B+ 60 (T1 38, T2 17, T5 2). (3) Contract-address lines are undercounted against the coordinator's method: 15/6/2/1, not 12/5/2/-. (4) The 6 commands for T4 to T7 are 10 engine requests plus 3 confirmations; that is still far fewer than the coordinator's 24. (5) The message-history posting convention has no authored line, while the baseline spent 6 of its 16 T2 lines on it. (6) B+ depends on two spec changes the design does not write. B's own verdict, fail on T1, is correct. B+ passes only under the most favourable reading: it fails if commands count as lines, or if digests count and T5 is compared with the stronger baseline. One correction to the files: no line was forgotten. The manifest, pins and installs are all present.

RECOUNT (non-blank lines, comments excluded, `---` counted; counted by script on <scratch>/apply-design/)

| Task | Baseline A70 | B as claimed | B recount | separators | B+ recount | A recount |
|---|---|---|---|---|---|---|
| T1 | 33 | 60 | 60 (conduit.yaml 5 + project.yaml 55) | 5 | 35 (5 + 30) | 153 (11+92+50) |
| T2 | 16 | 17 | 17 (1 manifest + 4 + 4 + 8 incl. 1 sep) | 1 | 16 (1 + 15) | 40 (3+31+6) |
| T3 | 0 | 0 | 0 | 0 | 0 | 0 |
| T4 | 8 | 2 | 2 changed | 0 | 2 | 2 |
| T5 | 3 | 1 | 1 changed | 0 | 1 | 2 |
| T6 | 2 | 1 | 1 changed | 0 | 1 | 1 |
| T7 | 2 | 0 | 0 | 0 | 0 | 0 |
| Total | 64 | 81 | 81 | | 55 | 198 |
The final B file after T2 has 71 lines, which is 55 + 16, as it should be. The block-list variant gives 65/23 if single-item lists stay inline, which is the coordinator's style; if those also become blocks, T1 is 62.

COMMANDS THAT CARRY CHOSEN VALUES, LISTED APART (T3's 2 `tools --as` are checks)
T1: 4 (3 `install ARCHIVE#DIGEST` and 1 `apply`). T2: 2. T4: 1. T5: 2. T6: 1. T7: 2 (`close` and `open`). That is 12.
Files + commands: B 93 (64/19/0/3/3/2/2); B+ 67 (39/18/0/3/3/2/2). This matches the design's sensitivity section.

COPIED DIGESTS (ruling 52 point 6: "copied digests counted as lines and listed apart")
The 5 digests move from the manifest into the install commands (T1 3, T2 1, T5 1). The design lists them but does not count them. Counted, B is 86 (63/18/0/2/2/1/0) and B+ is 60 (38/17/0/2/2/1/0).

B+ AGAINST THE BAR UNDER EACH READING
- Files only: less on T4 to T7 (4 of 7); T1 35 vs 33 (+6%); T2 equal. Passes on paper.
- Files + digests, against A70: less on T4 to T7 (T5 is 2 vs 3); T1 38 vs 33 (+15%); T2 17 vs 16. Passes narrowly.
- Files + digests, with T5 compared to the stronger baseline the counting rules name (`--read-only` already set, 2 version lines, no recovery): T5 ties, so it is less on only 3 of 7 and fails "most".
- Files + commands: less only on T4 (T5 to T7 tie at 3/3, 2/2, 2/2). Fails.

CONTRACT-ADDRESS LINES (listed apart)
The coordinator's 23 for T1 includes its 3 manifest package-address lines (3 + 20). By that method B has T1 15 (12 + 3), T2 6 (5 + 1), T4 2 and T5 1, not 12/5/2/-.

REQUESTS
Ruling 52 point 6 counts each author command as one request, so B has 12 commands outside T3. Under the design's own operations, though, each `apply` is plan.v1 + apply.v1 with a y/N confirmation between them, and `open` is register-replacement.v1 + open.v1. Engine requests are therefore:
- T1 5, T2 3, T4 2, T5 3, T6 2, T7 3: 18, plus 5 confirmations.
- T4 to T7: 10 requests plus 3 confirmations, not "6". That is still far below the coordinator's 24, but "6 commands vs 24 requests" compares different units.

TASK BY TASK
- T1: reached on paper. A has read and list, B has read, list and write; turn-a and turn-b let the person request turns. APL-011 derives the root→workspace Conduit that ACC-033 row 4 needs for set-up; this was mistake 1 in both runs.
- T2: both post and read; both Agents get read, list and write on history, with stable `history.*` tool names.
  - Uncounted asymmetry: 6 of the baseline's 16 lines (CLAUDE.md 3, AGENTS.md 3) tell the agents the posting convention (one new file per message, UTC-time name, never replace a post). B, like both real runs, writes none.
  - The engine takes tool descriptions from the Operation's metadata.description (reference-model/crates/core/src/schemas.rs:1112). file.write.v1 says "creating or replacing the file", not file-history.v1's convention.
  - The Agent types have no input for instructions; their config is accountServices, peerMessaging and model.
  - In the fresh author's log (lines 88 and 91), the uncounted check prompts dictated the post filenames.
  - So either the convention is not conveyed, or B owes up to about 6 lines somewhere the language has no place for.
- T3: 0 lines, 2 commands. Reach around the engine (shell, built-in tools, account services, messaging) still needs the 2 Agent package READMEs the coordinator read. The design leaves these out of its inspection effort.
- T4: reached. 2 changed lines; the narrowing swap comes first, there is no window, and local names are kept.
- T5: reached for `shared`. Package 0.2.0 maps only 3 tools, so append_file and copy_file reach no one. history stays on server 2.1.0 through history 0.1.0's own pin (NAM-014), as in the baseline.
  - Contradiction: under inline definitions, agent-a's and agent-b's anonymous templates name example.org/s1/filesystem (capability and operations of `project` and `history`). NAM-009 and PKG-005 therefore give each Agent filesystem 0.1.0 pins.
  - After the manifest edit their documents resolve to 0.2.0. APL-003(c): "any other change to an instance's ... package pins ... is an update with a restart". APL-003(a)'s "changes nothing" applies only to Conduits.
  - The plan's "unchanged agent-a and agent-b (NAM-023)" does not follow from the rules. If (c) applies, both Agent runs restart, and Claude Code's conversation ends (packages/agent-claude-code/src/lib.rs:164, --no-session-persistence). "Sessions lost: none in S1" then fails, and so does "no time without access" for the Agents.
  - If instead the Agents keep their 0.1.0 pins, the design must say so, and say how plan avoids reporting that difference on every later run.
  - Fix: a precedence rule, either pins reached only through nested Conduits' contract names advance the revision without a restart, or they are not compared. Variant A's T5 has the same problem.
- T6: reached by re-pointing config.root in place.
  - Hand-wave: APL-005 decides again only the Conduits the instance consumes. agent-a/project and agent-b/project now reach a different directory with no access decision on them, only the CON-022/IFC-014/IFC-033 checks and APL-006's equivalence of authority.
  - The new directory is data, not counted, as in the baseline.
- T7: reached. close, then register-replacement + open from rev 3 (read, list, write, which is A's state after T4); B is unaffected.
  - Gap: APL-012 gives a project holding only to objects an apply registers. The replacement Conduit made by `conduit open` outside an apply has no stated holding, so what the next apply does with it is unspecified.

ELIDED OR UNSTATED
- The commands are not today's CLI. Every command needs `--store DIR` (shell main.rs: "--store DIR is required"), and apply, plan, close, open and name resolution of `agent-a/project` are new commands. No lines change, but concepts met (plan, apply, revision, lock, swap, drift, derived Conduit, project holding) is not reported. Neither is duplicated configuration: by the end of T2 the capability line is written 4 times, each target twice, and the operation lists 3 times.
- OBJ-003 names a nested declared Conduit after its template. Inline declared Conduits have an anonymous template, yet OBJ-003 is not in the CHANGED list.
- B+'s two fixes are not written as promises: a default Pool (changes FST-001 to FST-006 and OL5, and adds an `owner` default under OBJ-013/POOL-001) and derived turn Conduits (a new rule, which operations they list, and their ACC-033 route). Its 35 lines rest on them.
- "Where B's 27 extra T1 lines go" sums to 54 (29 + 18 + 7): the frame lines of the Pool and turn documents are counted twice. This is cosmetic.
- This is a paper count by the designer, with no mistakes, recovery steps or time measured. In the real runs the fresh author wrote about twice what the coordinator wrote, and both hit the root→Pool mistake first. Ruling 45/52 calls for a re-run against the same measurement plan.
- Forgotten lines: none found. The manifest (5 lines, then +1 and 1 changed), the lock (tool-written, not counted), the 5 installs and all pins are present. The core package needs no manifest entry (NAM-007/NAM-020). The Agent config's required `model` has a default.

## Verdict safety (major)
The design fails as written and could hold with fixes. Its core is sound: a Conduit change is a close and an open made at one decision point, an open Conduit is never edited, operation lists stay explicit (so T5 gives no Agent append_file or copy_file), and there is no background apply. But the text breaks several guarantees, and I have a concrete counterexample for each. Revocation (G3): a narrowing or control made outside apply is quietly undone by the next apply, a retired Agent is brought back by an unrelated apply, and an operator's close made during an apply can be reopened. A reused name can re-point a grant (G1, OBJ-010). The plan digest does not cover the instance or template documents, because SUB-008's digest covers only contract documents and the manifest. So apply can carry out a plan nobody saw (G7). APL-006 requires closing every Conduit to the instance, which ACC-024 forbids for root's turn Conduits. Under that rule B's own T2 and T4 updates are refused, or else ACC-024 is bypassed. A restart that keeps the instance ID leaves an unstopped old run's per-instance credential valid (G12; the multi-machine S7 argument). plan.v1, authorized by `management` alone, discloses Resource configuration that only the owning Pool (ACC-033 row 6) may show. The T5 rewrite contradicts APL-003(c), so whether the Agents restart is undefined. The project name defaults to the directory. The lock file reverses ruling 27's NB without listing it. Each has a known fix: last-applied state with field-manager conflicts, tombstones, preconditions on each part, a digest over the whole plan, per-run credentials, and authorization of each plan row against its own target. So the severity is major, not fatal. B's S1 numbers for T2, T4 and T5 hold only once points 6 and 8 below are settled.

Repository read only, nothing changed. Promise texts were checked in spec/ (conventions/access.yaml, lifecycle.yaml, revisions.yaml; promises/objects.yaml, naming.yaml, packages.yaml, submissions.yaml, conduits.yaml, secrets.yaml; interface/implementation.yaml; implementation-defined.yaml DL13), plus conformance/fixtures/s1/packages/*, docs/proposals/ruling-49 to ruling-52, docs/proposals/naming/README.md (NB), docs/proposals/multi-machine/README.md and docs/proposals/reaudit/round-6/design-critique.md (D08, D09).

COUNTEREXAMPLES, most severe first. CONFIRMED means it follows from the design's own text plus the spec. PLAUSIBLE means it depends on how an underspecified point is read.

1. MAJOR, CONFIRMED (G3; the "no reconciler" claim). A change made outside apply is reverted by the next apply. APL-007 protects only Conduits that were closed. Nothing protects changes made by `swap.v1` (which the design offers on access-control.v1) or by a control.
   - Scenario: during an incident the owner runs `swap.v1` to narrow agent-b/project to read-only. A teammate later runs `conduit apply` for an unrelated edit. The documents still list file.write.v1, so APL-003(a) classifies this as a widening swap, and agent-b gets write back.
   - The same happens to a control that tightened a setting: APL-003(b) sets it back to the documented value. This contradicts the design's own UNCHANGED line "drift is shown, never repaired", and OBS-004.
   - swap.v1 on a declared Conduit faces a dilemma either way:
     - If it rewrites the holder's stored definition, an actor with only `access` changes an Agent's definition, which today needs `management` (escalation; OBJ-011/DR5).
     - If it does not, the live Conduit and the held nested instantiation diverge. The redefined LIF-009 then rebuilds the wider operations at the next register-replacement (for example T7's `conduit open`).
   - The same divergence appears if a swap succeeds and the holder's "update rev" part then fails, since T4's plan orders the update after the swaps.
   - Fix: keep a last-applied definition for each object and field, as Kubernetes server-side apply's managed fields do. Any field whose current value differs from last-applied because of a request outside this project's applies is a conflict that names who changed it, and is changed only where the request names it (`--take`/`reopen`). Make a swap and its holder's revision advance one part. Make swap.v1 either management-authorized and revision-checked, or forbid it on declared Conduits.

2. MAJOR, CONFIRMED (G3: RD2, LIF-008 in spirit, DO2). An unrelated apply brings back an object someone retired.
   - Holdings are "status the system owns" on the object (APL-012). LIF-035 removes the object, and its holding with it, once its shutdown is confirmed. APL-007 covers Conduits, not instances.
   - Scenario: the owner retires the misbehaving agent-b. It is removed, and the name is free under OBJ-002. The next routine `conduit apply`, for say a T6 edit, finds Agent/agent-b in the documents and no held object of that name. It plans a new registration. Setup opens agent-b's declared Conduits (CON-004), and turn-b is re-registered and opened.
   - The new Conduits are new objects, so APL-007 never sees them. Revoked access is reissued.
   - The same holds for a Resource retired to stop a compromised server.
   - Fix: keep holdings as their own per-project state (as Terraform state does), with tombstones that record who retired or removed what and when. Any part that would register an object, or open a Conduit, for a tombstoned name is a conflict until `recreate`/`reopen` names it.

3. MAJOR, PLAUSIBLE (G3, race; OL1). Revocation between the recomputed plan and the dispatch of a part.
   - APL-010 checks the digest once, then dispatches the parts in sequence. A restart part can take tens of seconds (T6 in the coordinator's run took about 80 s). Swaps carry no precondition: LIF-037's `expectedId` exists only for set-up, retire, inspect and unregister, and REV-012 says open and close name no revision.
   - Scenario: a plan restarts `shared`, then widens agent-a/project. During the restart the owner closes agent-a/project (LIF-035 removes it). The widening swap's close half reports invalid (OPS-029), and the text does not say the open half is then withheld. An implementation that ensures the successor is open will reissue the revoked grant with write.
   - Related: when the declaration of a Conduit that was closed outside apply is edited, APL-003(a) (swap or open) and APL-007 (left closed) collide, and no precedence is given. The same happens when the declaration is removed in one apply and re-added in the next.
   - Fix:
     - Every part carries the IDs and revisions it read.
     - A swap is dispatched only if its predecessor is open with the ID it read; otherwise it reports conflict and opens nothing.
     - A closure is sticky per local name until `reopen` names it, whatever later edits the declaration gets.

4. MAJOR, PLAUSIBLE (G1, OBJ-010). A reused name re-points a grant.
   - Today a held reference is never resolved again (OBJ-010), and LIF-009 reports invalid for a removed target. Apply instead compares the documents' names with held IDs, so "target differs" is decided by resolving names again.
   - Scenario: project P's agent has a declared Conduit to an outside Resource `datasets` in team T's Pool. T retires it and it is removed. Another actor registers a new Resource named `datasets` pointing at its own directory. P's next apply sees that the target's ID differs and swaps or opens agent-a's Conduit to the new object. The agent now writes into the squatter's directory. The plan shows only "target datasets".
   - The design's line "names resolve to the project's own holdings" either forbids outside targets, which breaks multi-owner use (test 8), or leaves this hole.
   - Fix: when a name now resolves to a different ID than the one held, report it as a conflict naming both IDs. Re-point only on an explicit request that names the new ID.

5. MAJOR, CONFIRMED (G7, G10). The plan digest does not cover what the author confirmed.
   - APL-001's digest covers the project digest of SUB-008, the revisions read and the Conduit IDs read. SUB-008 hashes only the project's contract documents and the manifest as written, before relative programs are resolved.
   - Scenario: an Agent (test 5) or a co-author calls plan.v1 and the plan is reviewed. Before apply.v1 is called with that digest, project.yaml is edited, for example to give agent-b file.write.v1 back. The same objects are touched, so the same revisions and IDs are read, and the digest is unchanged. Apply dispatches the new parts.
   - The same project digest is also produced from a different checkout directory whose relative programs differ (SUB-010).
   - Fix: the digest is the RFC 8785 form of the full plan (every part, field diff, consequence and resolved implementation item). Better still, apply takes the plan itself, as Terraform applies a saved plan file.

6. MAJOR, CONFIRMED (G2, ACC-024; B's own S1). APL-006 contradicts ACC-024.
   - APL-006 permits an "update or restart" only where its caller could close and open every Conduit to and from the instance (row 10). ACC-012 and ACC-024 refuse the close of any Conduit whose consumer is root, "whoever makes it". turn-a and turn-b target agent-a and agent-b.
   - Strict reading: B's T2 and T4 updates of the Agents are refused, as is any change to turn-a, turn-b or the derived root-workspace, and the Agents' replacements (APL-003(e)).
   - Lenient reading: the swaps inside a replacement do close root's Conduits, which bypasses ACC-024. A delegate whose bound covers the Agent could drop root's turn Conduit by changing the Agent's `runtime`.
   - Either way, the replacement of an Agent can stop part-way with no rollback: a new Agent exists, some Conduits have been swapped, the old Agent is not retired, and two runs are live.
   - Fix: state that root-consumed Conduits are excluded from the equivalence and are re-pointed by opening the successor and leaving the predecessor to CON-008 when its target retires. Or refuse such parts before anything is done, with the reason. Never close them.

7. MAJOR, PLAUSIBLE (G12; multi-machine gate model; D09's "exact identity ... stop semantics"). A restart in place keeps old runs authenticated.
   - Under DL13, credentials name the instance by its ID and, in v0, last as long as the instance. The S7 argument that left-over runs are inert, and IFC-032, rely on the ID being removed.
   - Scenario: agent-a runs on a managed node (ruling 51), for example WSL. Apply changes agent-a's config, perhaps narrowing its reach. The stop ends unknown because of a partition.
   - If the start proceeds, there are two runs. If it does not, the old run is still not stopped: when the partition heals, it presents a valid token for the same ID. It acts through agent-a's Conduits with the old configuration, and records attribute it to the new revision.
   - Fix: give each run its own credential generation, so an older run's credential is refused after a restart. Start only after the stop is confirmed. Otherwise treat a restart across machines as a replacement. Write the hook, secret-read and drain semantics D09 asks for before claiming G9 and G12.

8. MAJOR, CONFIRMED (G6, determinism). The T5 plan contradicts APL-003.
   - agent-a and agent-b's anonymous templates refer to example.org/s1/filesystem through their declared Conduits' capability and operations. So they hold its pins (NAM-009, PKG-005).
   - After T5's manifest change, the definition resolved again differs in its pins, and APL-003(c) says "package pins ... update with a restart". That restart loses the Claude Code session, which runs with `--no-session-persistence`. The plan instead says the Agents are "unchanged", using (a)'s rule for Conduits.
   - Two conforming builds can therefore differ. If (a) wins, the engine silently ignores part of the documents, contrary to the conformance rule in spec/README.md. The next unrelated edit to agent-a then picks up the pin change and restarts it unexpectedly. Uninstalling 0.1.0 later (NAM-011) makes T7's register-replacement fail under PKG-007.
   - Fix: state a precedence. For example: pins reached only through Conduit references never classify the holder as (c), and are advanced without restart and shown in the plan.

9. MAJOR, CONFIRMED (ruling 52 point 1; ACC-033 rows 6 and 7). plan.v1 discloses what inspect would not.
   - The amended ACC-033 authorizes plan.v1 against `management` alone. Its field diff carries old values, for example `/config/root` of a Resource. Inspecting a Resource needs a Conduit to its owning Pool.
   - Project names are chosen by the caller, so a holder of `management.plan.v1` with no Pool Conduit can name the owner's project and read every held Resource's configuration.
   - It also learns who closed which Conduit, when, and in which invocation (APL-007), and which project holds a name (APL-002).
   - Fix: authorize each part's read against its own row (6 or 7). Answer parts the caller may not inspect with the first-tier refusal only.

10. MAJOR, PLAUSIBLE (G4 confused deputy; ACC-030 "afterwards"; INS-020 change). Template changes cross project lines.
   - An instance whose `from` names a template outside the project is resolved again at every apply.
   - Scenario: a holder of `management.replace.v1` changes that template's implementation items, its programs or `remote` URLs. The next unrelated apply of every other project restarts its instances with the new code, under their existing grants, and the applier, often root, authorizes it.
   - ACC-030 limits the trust given to replace.v1 to instances made afterwards. This erases that limit.
   - Fix: instances hold the template revision they were made from. Moving to a newer revision of a template outside the project needs the document to name it, or an explicit flag shown in the plan.

11. MAJOR, CONFIRMED (G4 records; ruling 52). A restart or re-point keeps Conduits to the instance without a decision.
   - APL-005 decides again only the Conduits the instance consumes.
   - In T6, agent-a's grant, opened by root for /lab/shared, now reaches /lab/shared-new. No access decision or record attributes this to the actor who applied the change. The tool list is unchanged, so no listChanged is sent and the Agent is not told.
   - REC-002 gains the target's revision but not the consumer's, so after an Agent's in-place update the records cannot tell which configuration made a request.
   - Fix: record a decision for each Conduit to the instance, attributed to the actor who applied the change. Notify consumers when a target's revision changes. Add the consumer's and caller's revisions to REC-002. Or treat changes to settings that select the data as replacements.

12. MAJOR, CONFIRMED (G6, G16; INS-018/DD5). Project identity defaults to the directory.
   - B's own path, `<lab>/project`, collides with any other project directory named `project` on the same engine. The second one's apply plans in-place updates of the first one's agent-a.
   - A checkout in a directory with another name has every name rejected under APL-002.
   - Meaning then depends on the working directory.
   - Fix: require the project name in conduit.yaml, and bind it to a project ID made at the first apply and held in the lock or state. Refuse an apply whose ID does not match.

13. MAJOR, CONFIRMED (process; G5, G6). The lock file reverses ruling 27's NB.
   - NB (docs/proposals/naming/README.md:89) chose option (1), one manifest with exact versions and digests. It rejected option (2), a separate generated lock, because "a second file would only repeat them".
   - NAM-006 is a language promise that a manifest which omits a digest is rejected, so it does change; it is not "unchanged on the wire".
   - A lock that is missing is filled from whatever was installed on that engine, so meaning varies by machine.
   - "Nothing generated by a tool counts as authored" already makes digests that `install` writes into conduit.yaml cost 0 lines. The lock buys nothing.
   - Fix: have `conduit install` write version and digest into the manifest, or add NB to the rulings to amend and make the lock mandatory and part of the digest.

14. MINOR to MAJOR, CONFIRMED (least privilege; G1's "no whole-capability grant"; G8). Derived Conduits are too wide.
   - APL-011 derives "every operation of pool-management.v1" for root, or every operation the caller's bound names for a delegate. That includes retire-with-resources.v1, when a part needs only register.v1 and set-up.v1.
   - The result is a standing grant nobody wrote. Root's copy is unclosable and, per point 6, can never be changed.
   - If it is derived again after a later core release adds an operation (NAM-024), it is either a T5-style widening for delegates or a swap that always fails for root.
   - Fix: derive exactly the operations the parts need, expanded once. Record the rule on the Conduit's status, not only in the plan.

15. MINOR, CONFIRMED (G9; the "no interval" claim). A swap cuts in-flight work.
   - ACC-034 refuses every onward request serving an invocation dispatched through the closed predecessor, even when the successor is wider.
   - Swapping a turn Conduit, or an Agent-to-Agent or Resource Conduit whose target makes onward requests, kills the in-flight turn's onward requests. That leaves unknown or failed outcomes.
   - Fix: where the successor permits the operation, with filters equal or stricter, an invocation dispatched through the predecessor counts as dispatched through the successor.

16. MINOR, CONFIRMED (G2 ambiguity). APL-003(a) lists filters but not other requirements.
   - A change to bound.v1 on a Conduit to `access`, or to the shared-host requirement, is not classified.
   - If (c) were read as an in-place update, a bound would be widened without the ACC-030 check.
   - Fix: any change to a Conduit's definition, every `requirements` entry included, is a swap.

17. MINOR to MAJOR, CONFIRMED (OBJ-002, OBJ-011). A replacement under APL-003(e) makes the new instance first.
   - While the old instance exists, the new one cannot take its name, and names never change.
   - Swapping other projects' Conduits leaves it unclear which project holds the successors.
   - While its swaps run, consumers are split between two live backends.
   - Fix: give the replacement a generated name and keep the project's name only in holdings, or retire the old instance first with a stated window.

18. MINOR, PLAUSIBLE (ruling 49). Accounts are missing from the identity fields.
   - Ruling 49 has the configuration name the account each Agent home or Resource is for, and the identity fields do not include it.
   - An in-place account change keeps the home and its old login. The ruling's check runs "after a sign-in", so the new run can act as the old account.
   - A restart that waits on a pending sign-in (ruling 50) holds REV-014, so retiring that instance reports conflict until the sign-in completes.
   - Fix: make the account an identity field, or check the account at every start. End the restart part when the run is asked for, and let a retire preempt it.

19. MINOR, PLAUSIBLE (SEC-001, SEC-021). The plan's diff could print a literal secret.
   - A literal typed by mistake at a place marked secret-reference would appear as a "new value" in the plan result unless the plan rejects it before diffing.
   - Fix: run SEC-001 and SEC-021 before the diff, and never print values at such places.

20. MINOR. Other underspecified points:
   - (d) adds a non-optional declared Conduit to a running Agent with no restart. If the open fails, the Agent is ready with a required Conduit closed, and there is no rollback (CON-004, TPL-012).
   - Which project holds T7's register-replacement Conduit is unstated (APL-012 covers only what an apply registers).
   - Root's apply cannot be stopped mid-way, because root's management Conduit cannot be closed.
   - In peer mode (ruling 51) a relationship is two halves, and under A65 decisions are made from leased copies. So "every Conduit old or new" and "narrowings first" hold only per engine. The plan should say so.
   - B+'s defaulted `owner` means the default Pool of whichever engine loads the documents (G6).

WHAT HOLDS:
- A swap as close and open at one decision point, with the open checked against the bound, keeps G2 for operations and targets.
- Explicit operation lists keep G6/T5: the fixture's filesystem 0.2.0 maps only three tools, and its contracts are unchanged.
- No background apply keeps test 6, and ACC-036 decides each part afresh, so closing the caller's own `management` Conduit stops later parts (ACC-034).
- Narrowings ordered first fail closed.
- Changing LIF-009 to use the held definition is sound.
- Keeping get-secret.v1 root-only matches SEC-008.

EFFECT ON B'S S1 NUMBERS:
- Under a strict reading of point 6, T2 and T4 are refused.
- Under a strict reading of point 8, T5 restarts both Agents and loses their sessions.
- T4 to T7's claims of "no session lost" and "no window" hold only after points 6, 8 and 15 are settled.
- T6's one line is a re-point with no decision attributed to its author (point 11).

## Verdict cost (major)
The design can be built, but only after fixes, and the build is large. Measured against the repository, it changes about 45 of the 455 promises: the 15 the design names and about 30 it does not. It also needs 13 to 17 new promises, edits to 7 of the 9 files in spec/core, and 3 of the schema files. In the reference model, the core grows by about 4,000 to 7,000 lines of Rust, or 40 to 65% of its present 10,465. Conformance needs 45 to 70 new vectors, most of them long lifecycle cases. Existing documents stay valid. I checked every family document under spec/ and conformance/: none lacks both `spec` and `from`, and no declared Conduit or Pool entry lacks `from`. So the inline form only adds, and the design's claim that about 84 registration vectors must be rewritten does not hold. Three changes still break behaviour. `register-replacement.v1` changes meaning, because LIF-009 would build from the held definition instead of the current template; this flips 3 vectors and 1 shell test. The root Conduits' operation lists change, which alters the expected results of 2 to 3 first-start and S1 cases. And existing engine stores need a fresh start. Four defects need fixing before any text is written, and three need owner rulings the design does not list:
- **Replacement cannot keep the name.** APL-003(e) makes the new instance before retiring the old one. OBJ-002 forbids two objects with the same kind and name, and the old one is retired, not removed, so the name is still taken.
- **Root's Conduits cannot be closed.** ACC-024 (decision RD5) refuses any close of a Conduit whose consumer is root. So turn-a, turn-b and the derived root-workspace can never be swapped. Read literally, APL-006 also makes any Agent that has a root turn Conduit impossible to restart.
- **The lock file reverses a decision.** `conduit.lock` is option (2) of decision NB, which ruling 27 rejected. NAM-006 also rejects a package entry that gives no digest today, contrary to the design's claim that it is unchanged.
- **References are resolved again.** An update in place re-resolves references, which contradicts DD1 under ruling 21 and OBJ-009.

For an implementer, the spec gets more complex: about 13 concepts are added and none removed.

SCOPE AND METHOD. I read spec/ only for what binds. Every promise named below was read with a small script in <scratch>/est/. Vector coverage comes from conformance/manifest.yaml: 367 cases. Code sizes come from wc. Nothing in the repository was changed. The spec has 455 promises: ACC 29, CON 22, CTR 10, FST 5, IFC 44, IMP 5, INS 23, LIF 36, MAN 17, NAM 28, OBJ 24, OBS 5, OPS 31, PKG 22, PLC 21, POOL 15, PRV 28, REC 6, REV 14, SCH 21, SEC 24, SUB 11, TPL 14.

1. EXISTING PROMISES THAT MUST CHANGE (about 45, about 10% of the spec)

A. Named by the design (15): OBJ-002, OBJ-011, OBJ-012, OBJ-013, OBJ-014, INS-001, INS-002, INS-020, SUB-007, PKG-005, LIF-009, REC-001, REC-002, ACC-033, IFC-014.

B. Missed by the design, and contradicted unless changed (about 22):
- OBJ-003: the generated name is "its template's name" plus a suffix. An inline declared Conduit has no template name.
- OBJ-009: references resolve only "when a template is instantiated". An update re-resolves them. This is DD1(a) of ruling 21: "resolves once ... and never again".
- OBJ-018: `metadata.name` is optional on an instance. APL-002's identity by kind and name needs it, or a rule rejecting unnamed documents in an applied project.
- INS-013: its observable restates the old source in LIF-009, "the template ... as that template then stands".
- SUB-009: its observable and replace.v1's description say instances made earlier keep what they resolved. That is no longer true inside the applied project.
- NAM-006: "A manifest that ... omits a version or a digest is rejected." The design's version-only entry violates this. The invalid example spec/schemas/examples/invalid/NAM-006-manifest-item-without-digest.yaml then flips.
- MAN-001 and MAN-005: one manifest file, and every item states version and digest. A lock file changes both.
- SUB-008 and SUB-010: which manifest bytes are hashed, and what a directory handed over contains, once a lock exists.
- NAM-009: pins are kept "when a template is registered". The anonymous template inside an inline instance needs the same.
- PLC-028: it rejects templates of type node.v1. An inline instance of type node.v1 would bypass it.
- CON-004: "Setup instantiates no Conduit". APL-003(d), as in T2, instantiates and opens a declared Conduit on a ready instance outside setup.
- LIF-008: only "an explicit request under LIF-009" makes another instance for a nested instantiation. A swap is a second route.
- LIF-029: a request for a new declared Conduit while the old one is not retired reports conflict. A swap makes its successor while the predecessor is still open.
- ACC-024 (decision RD5): closing a Conduit whose consumer is root is always refused. A swap, and APL-006's restart authority, need an exception. This needs the owner.
- REV-011: every change names a revision. apply.v1 names a plan digest instead, so an equivalence must be stated.
- REV-014: a restart is a change, and it must hold back setups, retirements and changes, and be held back by them.
- REC-004: "A purge removes records and nothing else." APL-005 keeps earlier revisions until a purge.
- OPS-022: a compound outcome is defined only for set-up.v1 and retire.v1. apply.v1 needs it.
- TPL-012, PRV-007 and PRV-008: setup parts, the run part and the stop are defined only for setup and retirement. A restart of a ready instance is new.
- IMP-002: "an instance's run is restarted by an apply" must be added to the events that start no model work.
- IFC-038: listChanged fires only when the tool list changes. A restart under new pins can change tool definitions (IFC-033).

C. Wording or observables to align (about 8 to 10):
- REV-001 (kept history; the plan's "rev 1->2" against an opaque revision with no order).
- LIF-013 and the inspect.v1 outputs (revisions and project holding).
- NAM-010.
- SUB-003 (an apply is not all-or-nothing; say so).
- ACC-030 and spec/README.md's section on the stated limits (lines 337-375): a holder of `management` can now change instances in place.
- IFC-012 (the local name comes from "its consumer's definition", not its template).
- ACC-011.
- OPS-012 and OPS-028 (repeats of apply.v1).
- SEC-010 and SEC-014 (secret reads at a restart started by the engine).

NEW PROMISES: 13 to 17.
- APL-001 to APL-013 in spec/conventions/apply.yaml. At the spec's density, about 500 to 900 lines, between revisions.yaml (322) and lifecycle.yaml (877).
- If the lock stays: 1 to 2 new MAN promises.
- 1 to 2 for the restart (drain, the window in which calls report unavailable, the parts, hooks), unless all of it goes into APL-005.
- 1 to define swap.v1 on access-control.v1 alone. `access` cannot change the definition a consumer holds, so either restrict it to standalone Conduits or make it "close plus open a registered successor".

definitions.md: about 8 rows change (project, template and instance, nested instantiation, definition, revision, register, replace, management verb) and about 10 are new (plan, apply, part, swap, update in place, restart, drift, project holding, derived Conduit, lock).

implementation-defined.yaml: 2 to 3 new DLs (form of the plan digest, how long revisions are kept, how long a drain may take). The first-build profile's DL8 event list grows.

2. SCHEMAS (spec/schemas): 3 to 4 files, medium.
- instances.yaml: instance.v1 gains an inline alternative for each of the 5 families. declared-conduit.v1 gains `capability`/`operations`/`requirements` in place of `from`. pool-entry.v1 probably follows.
- The design lists only type, config and conduits for the inline form. It must also carry `implementation` (APL-003(c) relies on it), `controls`, `entries` for a Pool, and placement and runtime.
- document.yaml: classify by `spec`, no longer by `from`.
- manifest.yaml: a version-only item, plus a schema for the lock (only if the lock stays).
- Examples: about 10 to 15 new, 1 flipped. spec/schemas/README.md is updated.

3. CORE PACKAGE (spec/core): 7 of 9 files, small to medium.
- operations.yaml: new plan.v1, apply.v1 and swap.v1. Edit register-replacement.v1 (LIF-009), replace.v1 ("instances made before keep...") and inspect.v1 (outputs: revisions, project).
- capabilities.yaml: object-management.v1 adds plan and apply; access-control.v1 adds swap.
- types.yaml: the `requires` lists of management.v1 and access.v1.
- templates.yaml: the operation lists of root-management and root-access.
- results.yaml: record.v1 gains the target's revision; 2 to 4 new Schema contracts (plan, part, field diff, consequence).
- conduit.yaml: the exports, and possibly the core version (0.1.0).
- README.md.

4. CONFORMANCE
- Expectations that flip:
  - change-t4: a replacement from the current template gives write access under the old LIF-009, not the new.
  - setup-restore-current-template.
  - setup-restore-missing-template: invalid today, it succeeds once the template is no longer needed.
- Exact arrays to regenerate: first-start, and very likely s1-whole and language/first-start/protected, because conventions/first-start/initial/case.yaml asserts the operation arrays of root's Conduits.
- Optional, for consistency: fixtures/root-system-state.json and about 8 initial.json snapshots that pin root-management's operations. The 197 cases that load the snapshot still pass.
- Survive:
  - the 84 registration, 61 packages, 33 interface/schemas and 40 archive cases, since no package is repacked and no digest moves;
  - records-replace, registration-frozen-template and registration-existing-name, since register.v1 and replace.v1 are unchanged;
  - setup-restore-retired-target.
- The runner matches maps as projections, so the field REC-002 adds breaks nothing.
- New apply family, about 45 to 70 cases:
  - plan purity and digest, 3 to 5;
  - identity and holdings, 4 to 6;
  - classification (a) to (f), 8 to 12;
  - swap, 6 to 8 (IFC-014 predecessor, standalone name reuse, ACC-034 in flight, refusal for a root consumer);
  - restart, 8 to 12 (drain, unavailable, stop or start ending unknown, a consumed Conduit refused on re-decision, secrets);
  - authority-equivalence negatives, 6 to 10 (where an escalation would show);
  - drift and no reopen, 3; prune, 2; order and failure, 4 to 6; stale plan, 2; derived route, 3;
  - inline forms, 8 to 12; lock, 3 to 5.
- Size: the median case is 164 lines and the 90th percentile 1,536. These cases are lifecycle-heavy, so roughly 20,000 to 50,000 lines of JSON.
- New safety_rows are also needed: no widening before a narrowing, no reopening, authority equivalence, stale plan.

5. REFERENCE MODEL (reference-model/crates)

| Crate | Size | Work |
| --- | --- | --- |
| loader | small, 150 to 300 lines | Classification sites, and project.rs reading the lock |
| core, apply.rs | large, about 2,300 to 4,000 lines | Plan: a trial resolution that reuses registration::prepare as records.rs::replacement does, a structural diff, the APL-003 classifier (needs each control's `changes` pointers), consequences through access::tool_list_changes on a simulated state, ordering, APL-011 derivation by simulating routing(), and the digest; about 1,500 to 2,500. Execution: a compound invocation built on lifecycle's invoke_part and settle, stopping on a failed dependency; about 800 to 1,500 |
| core, registration.rs (2,061 now) | 500 to 900 changed | Anonymous inline templates, OBJ-003 naming, a frozen copy of each nested definition so LIF-009 has a source, re-resolution on update, excluding the predecessor under IFC-014 |
| core, lifecycle.rs and retirement.rs | 400 to 700 | Restart: drain, stop, decide the consumed Conduits again, run, connect again; the REV-014 hold |
| core, access.rs | 200 to 400 | swap.v1 as close and open in one transition, the routing rows, the ACC-024 exception |
| core, records.rs | 200 to 400 | Revision history, new events, the revision in REC-002, inspect outputs |
| core, requests.rs | 300 to 500 | Compound outcome, idempotency of apply.v1, unavailable during a drain |
| core, all | 4,000 to 7,000 including unit tests | 40 to 65% of the present core |
| shell | medium, 800 to 1,500 plus tests | `plan` and `apply` (render, confirm, send the digest); `close` and `open <agent>/<local>` with open = register-replacement.v1 + open.v1; `install` writing the pins; a default project name; restart in runner.rs. tests/changes.rs t4 (851 lines in the file) partly rewritten |
| conformance crate | small, 100 to 300 | |
| tools/check_schemas.py | medium, 200 to 500 lines of Python | About 12 sites classify on `"from" in doc`, for example role_of at line 888 and line 754; the core crates have 59 sites on from or spec |
| packages/ | none for B | |

6. MIGRATION
- Documents and manifests: backward compatible. The inline form and the change in marker are additive for everything in the repository, as verified above.
- Behaviour: breaking. register-replacement.v1 now builds from the definition the instance holds; the coordinator's T4 route (replace the template, then register a replacement) no longer changes access. INS-020's guarantee narrows to other projects. Under CTR-001's versioning this would strictly be register-replacement.v2. In a draft, editing in place is the norm, but it should be logged.
- Engine stores: a clean break.
  - Objects made by register.v1 have no project holding, and APL-002 refuses to adopt them.
  - Stored instances hold no copy of their nested templates, so the new LIF-009 has no source for them.
  - Root's first-start Conduits are permanent and cannot be closed (ACC-024), and they list the old operations. Root must open a second Conduit to `management`, which ACC-022 allows, or the store is re-initialised. The cleanest path is `conduit init` on a fresh store.

7. READING COST FOR AN IMPLEMENTER
- Concepts added, about 13:
  - plan and plan digest, a stale plan as conflict;
  - apply as a compound invocation with ordering by dependency and partial outcomes;
  - the five acts, with swap and update-or-restart as new mechanisms;
  - revision history;
  - project name and holdings: a project becomes a persistent identity, where today it exists only inside one request;
  - drift and orphans, and prune;
  - derived routes;
  - authority equivalence (APL-006), a meta-rule over the rows of ACC-033;
  - the restart window and the drain;
  - inline definitions with anonymous templates;
  - the lock file;
  - swap.v1.
- Concepts removed: 0. The frame, the split between template and instance, replace.v1 and register-replacement.v1, Pools and root's Conduits all stay.
- One coupling goes: LIF-009's "the template as it now stands".
- The spec has two change paths that must agree exactly, two forms of instance and two pin files. It gets simpler for an author and more complex for an implementer.

8. DESIGN DEFECTS TO FIX FIRST
- (1) APL-003(e) and OBJ-002/OBJ-011. A replacement made before the old instance is retired cannot take its name: a retired instance is removed only once its shutdown is confirmed (LIF-035). Either the replacement takes a generated name and the project's holding records it, or the old instance is retired and removed first, which leaves a window.
- (2) ACC-024, under RD5, blocks a swap of turn-a, turn-b and the derived root-workspace. Read literally, APL-006 also makes any Agent with a root turn Conduit impossible to restart. S1 does not reach this, because T4 to T7 restart only `shared`.
- (3) Ruling 27, decision NB, chose a single manifest over "a separate generated lock file". Cheaper and needing no ruling: `conduit install` writes the version and digest into conduit.yaml, and the tool-written lines are not counted. That keeps NAM-006, MAN-001, MAN-005, SUB-008 and SUB-010 unchanged.
- (4) DD1(a) under ruling 21 must be added to the rulings to amend. So must RD5 and ruling 27 NB.
- (5) The inline schema must cover implementation, controls, entries, placement and runtime.
- (6) Control parts (APL-003(b)) are authorized by a Conduit to the instance itself (ACC-033 row 11). S1's root holds none, and APL-011 derives only routes to Pools. A fallback is needed, such as deriving that route or using a restart.
- (7) APL-005 decides again only the Conduits the instance consumes. Conduits to it are authorized but not recorded as decided again. Say so explicitly for G4.

SIZES: spec large, about 45 promises edited and 13 to 17 new; core package medium; schemas medium; loader small; core large; shell medium; check_schemas medium; vectors large.

B+ DELTA: about 8 to 10 more promises, small to medium. FST-001, FST-002, FST-003 and FST-005; OL5 of ruling 30; POOL-001 and OBJ-013 (a default owner); APL-011 extended to turn Conduits. templates.yaml gains a default Pool. first-start, first-start-repeat, s1-whole and root-system-state.json are regenerated; POOL-001-resource-without-owner.yaml flips, and probably registration-standard-argument-owner too.