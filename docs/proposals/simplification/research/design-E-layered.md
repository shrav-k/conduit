Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Layered authoring surface: one project file compiled to today's core, with plan/apply for change (core unchanged)

Keep the core language and the serving engine exactly as they are, and treat today's documents and requests as an intermediate form. Authors write one project file, keyed by name and shaped like Compose. The CLI compiles it into canonical documents, as kustomize and Helm compile to Kubernetes objects. Change is made by editing the file: `conduit plan`/`conduit apply` diffs the file against the engine's state and records, then sends the close, register, open, set-up, retire and unregister requests itself, as one explicit, confirmed run that reports every step (Terraform's model, with no reconciler).

The design works without any core change because of one choice. Every Agent grant compiles to a standalone Conduit with a stable name, `<key>-<agent>`, which can be closed and registered again under that same name. That keeps tool names and sessions intact through T4–T6. Generated templates are named by a hash of their content, so finding 4-01's shared-template trap cannot occur.

On paper, S1 costs 26 authored lines (29 if command lines that carry values are counted) against the baseline's 64 and the coordinator's 332. That is less on six tasks and more on none. The price is that the core's full cost stays with readers, implementers and anyone working below the CLI. Several guarantees also move from the engine into the CLI's discipline, and tool names keep the agent's name in them.

## Spec changes
IN spec/: nothing. No promise is added, changed or removed, there is no new syntax in the language, no new operation and no new default. The serving engine is unchanged too. Every request the CLI sends is one an author or an Agent can send today through the root Interface's existing Conduits, so preservation test 5 holds.

PROVIDERS: no change is required. One optional change, in line with ruling 48's intent: the two Agent packages could give `accountServices` and `peerMessaging` a schema default of `false`. That removes 2 author lines in T1.

CLI (the author-side client in reference-model/crates/shell, which is author.rs and main.rs today; not the server):
- New authoring file `conduit-project.yaml`, format v0.
- Generated files, never counted as authored: `conduit-project.lock` and `.conduit/applied.json`.
- New commands:
  - `conduit install <archive>...`: computes each digest, sends install.v1, and records address, version, digest and fetch in the lock. Today's command needs `ARCHIVE#SHA256` typed by hand. Installing a newer version of an address that is already locked makes it that address's selected version, as `go get` and `cargo add` do.
  - `conduit render`: prints the canonical documents and manifests, like `docker compose config` and `helm template`.
  - `conduit plan`: pure; reads only.
  - `conduit apply [--yes] [--reopen KEY] [--replace KEY] [--prune]`.
  - `conduit reach [--as AGENT] [--to RESOURCE]`.
- Exit codes and per-step reports for every command, as findings 4-13 and 4-14 ask.
- The existing `submit`, `replace`, `retire`, `unregister`, `call`, `tools`, `inspect` and `records` stay as the core-level escape hatch.

FORMAT v0: what the file holds
- Top-level keys:
  - `conduit: v0`: the format version. It compiles to the `apiVersion` that spec/names.yaml gives.
  - `project: <name>`.
  - `resources: {<name>: {type: <Type address>, config: {...}}}`.
  - `agents: {<name>: {type, config, reach: {<key>: {to: <resource>, ops: [...]}}, turns: root|none}}`.

FORMAT v0: defaults. Each one appears in render, plan and inspection with its rule name (ruling 52).
- D1: a Pool named `<project>` (core#pool.v1) owns every Resource.
- D2: a root Conduit `<project>-admin` to that Pool listing register.v1, inspect.v1, set-up.v1 and retire.v1 (ACC-033 rows 1, 4 and 6). This removes the mistake both authors made first.
- D3: a root Conduit `turn-<agent>` to each Agent. It covers the one capability the Agent's Type offers, with that capability's operations expanded once, at compile time.
- D5: the capability is inferred as the single capability the target's Type offers that holds every listed operation. Otherwise compilation fails and names the candidates.
- D6: an operation may be written by its export name without address or version where exactly one export matches. The compiled document is always fully qualified. Line counts do not depend on D5 or D6; they only make lines narrower.
- D7 (names):
  - A grant becomes the standalone Conduit `<key>-<agent>`.
  - A Resource's engine name is its key, or `<key>-<n>` while a predecessor is still live.
  - Templates are `<project>-<export>-<hash>`, where the hash covers the type or capability, the config and operations, and, for Resources and Agents, the package version.
- D8: each submission's manifest is built from the lock.
- D9: every generated document carries the annotations `conduit-project/source` (file#pointer) and `conduit-project/rule`. OBJ-018 allows annotations and gives them no meaning.

FORMAT v0: how each change is planned. Every step is a request that exists today.
- C1, grant added: register, then open.
- C2, grant removed: close, then unregister the template if nothing uses it.
- C3, a grant's operations or target changed: close, register under the same name, open. This is break-before-make: names are global and a new name would change tool names (IFC-014). Narrowings run before widenings.
- C4, a Resource's config or its Type's package version changed (no control covers it; OBJ-011, PKG-005):
  - register a new instance under a free name and set it up;
  - apply C3 to every grant that targets it;
  - retire the old instance (sending expectedId, LIF-037);
  - unregister the old template (sending its plan-time revision).
- C5, an Agent changed: refused without `--replace` (the plan says the session is lost).
- C6, a Resource removed: refused without `--prune`.
- C7, a name held by an object this project does not manage: refused, as Helm refuses to adopt objects it does not own.
- C8, a grant that was last applied but is now closed by someone else: a conflict. It is never reopened without `--reopen` (G3). Provenance comes from `.conduit/applied.json`, or from records: REC-002 keeps each close's caller and idempotency key, and apply's keys are `apply/<plan-digest>/<step>`. Without the local file, the check falls back to conflict.
- C10, only the pin of a Conduit's capability package changed: nothing is done, and the plan says so (NAM-017, NAM-023).
- Declared Conduits, replace.v1 and register-replacement.v1 are never used. Declared Conduits hold a resolved target (OBJ-009, OBJ-010, POOL-014, LIF-009), so they cannot follow a new Resource in T5 or T6.

FORMAT v0: how apply runs
- E1: steps run in plan order inside one MCP session. Each carries an idempotency key, plus expectedId or the revision read at plan time.
- E2: apply stops at the first step that does not succeed. There is no rollback (LIF-027). It reports succeeded, failed and unknown for every step, with the reconcile command for unknowns. Running apply again resumes, and the idempotency keys keep each effect single.
- E3: on first setup, an Agent's grants are opened before the Agent is set up. The engine allows opening a Conduit whose consumer is merely registered (core lifecycle.rs rejects only a retired consumer), so this ordering imitates TPL-012's guarantee.

FORMAT v0: how errors map back to the author's file
- M1: the format's own errors give file:line:col, the key path and the rule.
- M2: a source map links every generated pointer to the author's pointer. For example, a Resource template's `/spec/config/*` maps to `/resources/<r>/config/*`, and a Conduit template's `/spec/operations/i` to `.../reach/<k>/ops/i`. A derived document maps to the author's line that triggered its rule.
- M3: a pre-flight check runs the pure `conduit_core` resolution on the compiled submission against the store's state. Each SARIF result (SUB-005: document plus JSON Pointer) is printed as three parts: the author's line first, the core diagnostic verbatim with its promise ID second, the generated document third. Nothing is sent if pre-flight fails. Example: `conduit-project.yaml:6:13 resources.shared.config.root: "" rejected by example.org/s1/filesystem#local-directory.v1 (minLength 1) [SUB-005; template s1-local-directory-v1-5a91f3 /spec/config/root]`.
- M4: an execution failure prints the step, the author's line and the engine's result verbatim, including ruling 52's two-tier refusal.
- M5: the annotations let inspection, records and tool names be traced back to the file without this CLI.

LATER, if a second CLI must agree (A34): promote the format to `spec/authoring/` as AUT-* promises, with an "Authoring" conformance claim for a CLI and vectors of (file, state) → plan.

WHAT STAYS: all 455 promises, the strict loader, the core, the 367 conformance cases, ACC-033, the lifecycle, records and the immutability rules. The core's estimated cost is unchanged.

## Rewrite
Outside the measure, as in both runs (harness): `conduit init` and `conduit serve` create and hold the store, and `init` writes `.conduit/config` naming it, so the commands below take no `--store`. Archives sit in `<lab>/packages/`, as in the fresh author's run. Every authored line lives in one file, `<lab>/project/conduit-project.yaml`. Text marked "output" is printed by the CLI, illustrative and not counted. The files are also in <scratch>/layered/ (t1.yaml through t7-restored.yaml).

```yaml T1: conduit-project.yaml (17 lines, whole file)
conduit: v0
project: s1
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    config: {accountServices: false, peerMessaging: false}
    reach:
      project: {to: shared, ops: [file.read, directory.list]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    config: {accountServices: false, peerMessaging: false}
    reach:
      project: {to: shared, ops: [file.read, directory.list, file.write]}
```
```sh T1: commands (2 author requests)
conduit install <lab>/packages/filesystem.tar.gz <lab>/packages/agent-claude-code.tar.gz <lab>/packages/agent-codex.tar.gz
conduit apply
```
`apply` prints the plan and asks Apply? [y/N]; the author answers y.

What T1 compiles to (generated, not counted):
- 19 documents (171 lines with annotations) plus an 11-line manifest. The coordinator wrote 154 lines by hand for T1.
- Documents:
  - Pool template and Pool `s1`;
  - template `s1-pool-management-v1-…` and root Conduit `s1-admin`;
  - Resource template and Resource `shared` (owner s1);
  - two Agent templates and Agents `agent-a`, `agent-b`;
  - two conversation templates and root Conduits `turn-agent-a`, `turn-agent-b`;
  - two directory Conduit templates and Conduits `project-agent-a`, `project-agent-b`.
- 11 requests:
  - 1 register.v1;
  - 3 opens of root's Conduits;
  - set-up of s1, then of shared;
  - 2 opens of the project Conduits;
  - set-up of agent-a and agent-b.
- Agent A's tools: `project-agent-a.file.read.v1` and `project-agent-a.directory.list.v1`. Agent B's: those under `project-agent-b`, plus `project-agent-b.file.write.v1`.

Checks (not counted): `conduit call turn-agent-a.turn.v1 '{"message":"…"}'`, and the same for B.

```yaml T2: lines added (5)
# under resources:, after shared:
  history:
    type: example.org/s1/history#file-history.v1
    config: {root: <lab>/history}
# under agents.agent-a.reach:
      history: {to: history, ops: [file.read, directory.list, file.write]}
# under agents.agent-b.reach:
      history: {to: history, ops: [file.read, directory.list, file.write]}
```
```yaml T2: conduit-project.yaml after T2 (whole file)
conduit: v0
project: s1
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
    config: {accountServices: false, peerMessaging: false}
    reach:
      project: {to: shared, ops: [file.read, directory.list]}
      history: {to: history, ops: [file.read, directory.list, file.write]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    config: {accountServices: false, peerMessaging: false}
    reach:
      project: {to: shared, ops: [file.read, directory.list, file.write]}
      history: {to: history, ops: [file.read, directory.list, file.write]}
```
```sh T2: commands (2)
conduit install <lab>/packages/history.tar.gz
conduit apply
```
What T2 compiles to:
- The history template and Resource `history` (owner s1).
- Conduits `history-agent-a` and `history-agent-b`, both from the existing template that holds read, list and write.
- About 6 requests. Registering into an existing Pool is authorized by `s1-admin`, which already lists register.v1.

```sh T3: no lines; 1 inspection command
conduit reach
```
Output (excerpt):
```text
agent-a: Agent agent-a, claude-code.v1 0.1.1
  project → shared (filesystem 0.1.0, root <lab>/shared):
    project-agent-a.file.read.v1, project-agent-a.directory.list.v1
    [Conduit project-agent-a, open; line 15]
  history → history:
    history-agent-a.{file.read.v1, directory.list.v1, file.write.v1}
  around Conduit: accountServices false, peerMessaging false.
    The Type disables shell, browser, other-Agent tools, hooks and plugins.
    Mediated, not OS isolation (ACC-027).
  driven by: root via turn-agent-a.turn.v1 [rule D3-turn]
agent-b: …
```

```yaml T4: 2 lines changed
# agents.agent-a.reach.project, before → after:
      project: {to: shared, ops: [file.read, directory.list]}
      project: {to: shared, ops: [file.read, directory.list, file.write]}
# agents.agent-b.reach.project, before → after:
      project: {to: shared, ops: [file.read, directory.list, file.write]}
      project: {to: shared, ops: [file.read, directory.list]}
```
```sh T4: command (1)
conduit apply
```
The plan:
- It narrows first: close, register (from the existing read/list template) and open `project-agent-b`. Then it does the same for `project-agent-a` from the read/list/write template.
- No template is created or replaced, and no template name now misstates what it grants.
- Tool names are unchanged; A gains `project-agent-a.file.write.v1` and B loses its write tool.
- Each Agent is without access from its close to its open: two requests within one session. Not measured.
- 8 requests.

```sh T5: no file lines; 2 commands
conduit install <lab>/packages/filesystem-0.2.0.tar.gz
conduit apply
```
Lock after `install` (generated, not counted):
```yaml
example.org/s1/filesystem:
  selected: 0.2.0
  installed:
    0.1.0: {digest: sha256:200de7c1…, fetch: file://<lab>/packages/filesystem.tar.gz}
    0.2.0: {digest: sha256:cbb831f0…, fetch: file://<lab>/packages/filesystem-0.2.0.tar.gz}
```
Plan (output):
```text
-/+ resources.shared (line 4): Resource shared -> shared-2
    why: the Type's package changes version and an instance keeps its versions (PKG-005)
    directory.v1 in 0.2.0 still offers read, list and write; no grant gains an operation
  1  register  template s1-local-directory-v1-c03e7d + Resource shared-2  [management + s1-admin, ACC-033 row 1]
  2  set-up    shared-2
  3-5  close, register, open project-agent-b -> shared-2; tools unchanged
  6-8  the same for project-agent-a
  9  retire    shared (expectedId)
  10 unregister the old template (plan-time revision)
=  agents.*.reach.history: kept (NAM-017)
No Agent replaced; no session lost.
```

```yaml T6: 1 line changed
# resources.shared.config, before → after:
    config: {root: <lab>/shared}
    config: {root: <lab>/shared-new}
```
```sh T6: command (1)
conduit apply
```
The plan:
- Config is frozen (OBJ-011) and no control covers `root`, so the Resource is replaced.
- A new instance is registered under the free name `shared` and set up.
- Both Conduits are re-pointed by close, register and open.
- `shared-2` is retired and the old template unregistered.
- About 12 requests. Tool names are unchanged.

```yaml T7: 1 line, removed then restored (2 edits)
# step 1: delete this line from agents.agent-a.reach:
      project: {to: shared, ops: [file.read, directory.list, file.write]}
# step 3: add the same line back in the same place
      project: {to: shared, ops: [file.read, directory.list, file.write]}
```
```sh T7: commands (2)
conduit apply
conduit apply
```
The first `apply` runs after the line is deleted. Its plan says: "- agents.agent-a.reach.project: close project-agent-a; agent-a loses project-agent-a.*". The second runs after the line is restored. Its plan registers `project-agent-a` from the existing template and opens it. This is an explicit addition, because the last apply did not have the grant (C8).

Check between the two (not counted): `conduit call turn-agent-b.turn.v1 '{"message":"…"}'`.

Equivalent command route: `conduit apply` after `conduit call access.close.v1 '{"conduit":"project-agent-a"}'`. The plan then refuses to reopen it (C8) until the author runs `conduit apply --reopen agents.agent-a.reach.project`. That route is 0 file lines and 2 command lines.

## Counts
Counted by the measurement plan's rules: non-blank lines written or changed, each line once per task, comments excluded, `---` counted. This format has no separators. Files are in <scratch>/layered/ and were counted with grep and diff.

| Task | Lines | Files | Author commands (requests) | Engine requests the CLI sends (estimate) | Baseline | Coordinator | Against the baseline |
|---|---|---|---|---|---|---|---|
| T1 | 17 | 1 | 2 (install with 3 archives, apply) | 3 install + 11 | 33 | 154 | less |
| T2 | 5 | 1 | 2 (install, apply) | 1 + about 6 | 16 | 73 | less |
| T3 | 0 | 0 | 1 view (`conduit reach`) | reads only | 0 (10 items read + 1 command) | 0 (2 commands, 2 READMEs) | same on lines, less inspection effort |
| T4 | 2 | 1 | 1 (apply) | 8 | 8 (1 restart) | 2 (8 requests) | less |
| T5 | 0 | 0 (the lock is rewritten by `install`) | 2 (install, apply) | 1 + about 10 | 3 (+2 recovery steps) | 62 | less |
| T6 | 1 | 1 | 1 (apply) | about 12 | 2 | 29 | less |
| T7 | 1 (2 edits) | 1 | 2 (apply ×2) | about 2 + 2 | 2 (4 edits) | 12 | less |
| All | 26 | 1 distinct | 10 + 1 view | about 59 | 64 | 332 | less on 6, same on 1, much more on none |

Stricter count, adding every command line that carries a value the author chose (the 3 `install` lines): T1 18, T2 6, T5 1, total 29. That is still less than the baseline on the same six tasks.

Variants:

| Variant | Total | Change |
|---|---|---|
| Agent packages default the reach flags to false (provider change, ruling 48's intent) | 24 | T1 −2 |
| Versions written in the file (`packages:` + 3 lines in T1, +1 in T2, the T5 version line) instead of in the lock | 32 | +6 |
| `turns: root` written explicitly for each Agent | +2 | |

Every variant passes.

Other measures:
- Grants made by hand: T1 2, T2 2, T7 1. Root's Conduits, all derived: 3 (`s1-admin`, `turn-agent-a`, `turn-agent-b`).
- Lines naming a provider: 0. Lines naming a contract address: 4 type lines.
- Tool names changed by any task: 0. The coordinator changed them in T2 and T5.
- Time without access: in T4, T5 and T6, each Agent is without access from its close to its open (two requests within one session; not measured). The coordinator's figures were 29–30 s in T4 and about 80 s in T6.

What the compiler emits for T1: 19 documents, 171 lines, plus an 11-line manifest, from 17 authored lines.

Every number above is a paper count. Ruling 45 does not accept paper counts as evidence; a fresh author has to run it on a built CLI.

## Lost
Is it honest?
- On the bar's terms, yes. FIRST_BUILD.md says "Nothing generated by a tool counts as authored", and the ratio of emitted to authored lines is about 10.7:1 in T1.
- But the core's cost does not go away; it moves.
  - Readers and engine builders still face 455 promises, the frames, the template/instance pairs, the 14 rows of ACC-033, and 367 conformance cases.
  - They also get a second layer: a format document (about 30 rules), an estimated 3,000 lines of CLI, and about 50 new vectors. These figures are unverified.
  - In total, the language costs implementers more, not less.
  - The machinery that authors no longer use (declared Conduits, replace.v1, register-replacement, LIF-009) stays in the core and is still implemented and tested. That is evidence the core carries dead weight, which this angle does not remove.

The core still shows through:
- Agents see tool names made from compiled names: `project-agent-a.*`, not `project.*`. Standalone Conduit names are global, and declared Conduits cannot follow a new Resource (OBJ-009/OBJ-010/POOL-014/LIF-009), so the agent's name has to be in the prefix. The names are stable, but they are not the baseline's names.
- Raw inspection, records and engine diagnostics speak in core terms: `shared-2`, `s1-directory-v1-2f6c88`, `s1-admin`.
  - The source map and annotations translate them, but only for tools that read the annotations.
  - Old records name objects that have since been removed. Explaining them needs the local state file or a reading of the records.
- When something fails partway through (partial apply, unknown outcome, the turn deadline of finding 4-12), the author is back in core concepts. There is no rollback.

Guarantees that move from the engine to the CLI's discipline:
- G9 (Conduits open before a run starts). Agent access is all standalone, so the engine no longer enforces this; apply's ordering only imitates it at first setup.
- G3 (no reopening of a revoked grant). It now rests on the CLI's conflict check (local state, or REC-002 caller and idempotency key). Another client can still reopen explicitly.
- No lock across a whole plan (G7). Stale plans are caught step by step, through register name collisions, expectedId and revisions.
- open/close carry no expectedId (LIF-037 does not cover them), so a renamed-and-reused Conduit name is a small race between plan and apply.

Operational costs that remain:
- Every change in T4–T6 still closes and reopens each grant: a short window without access for each Agent, and Agents switched one at a time.
- Each change makes new objects and IDs, which adds churn to records.
- About 59 engine requests across S1, against the coordinator's 35 typed requests.

Less is visible in the author's file:
- Versions and digests live in the lock.
- The defaults are not in the file: the Pool, root's Conduits, the turn Conduits, the inferred capabilities and the short operation names. They appear only in render, plan and inspection. Ruling 52 is met by what the engine and CLI show, not by the file.

Two sources of truth:
- Changes made outside apply are drift. Apply reports drift and never repairs it silently.
- Without a specified format, files are not portable between CLIs (G6 holds only for the compiled output).
- An Agent that operates the system gets the layer only if it runs the CLI.

Rejected inside this angle:
- Declared Conduits: they need Agents recreated, and sessions lost, in T5 and T6.
- Storing apply's state in a template's annotations: OBJ-018 says documents that differ only in annotations "resolve to the same object", so SUB-009 may treat such a replace.v1 as no change.

Open for the owner:
- Round 6 advised against "automatic apply … to make examples look shorter". This apply is explicit, confirmed and reported step by step, but whether it fits that advice is the owner's call.
- So is whether a surface that exists only as tooling meets the language thesis.

## Composable
The author's file never has to change when the core gains targeted operations. Each one only shortens the compiled plan:
- An argument giving a standalone Conduit its local name (one sentence of OBJ-013 and IFC-012) lets the compiler emit `project.*` tool names.
- An atomic swap of a Conduit by consumer and local name removes the T4–T6 windows without access.
- An upgrade or restart in place (A67) keeps the Resource's ID through T5.
- expectedId on open and close closes the race between plan and apply.
- A default Pool, or root's authority over its own Pools, removes the derived `s1-admin` Conduit.

Other designs it combines with:
- Provider-side changes: packages that export Conduit templates or operation sets, and Agent types that default the reach flags to false.
- A normative compact form. This file format can be promoted to `spec/authoring/` with AUT-* promises.
- The views of the prior-art survey (render, plan, reach, why), computed with the same pure core.
- Ruling 50: a plan proposed by an Agent can become a pending action at the root Interface, with the plan as its evidence.
- Ruling 51's peer mode: one plan per engine, one for each half of a relationship.
- Ruling 49: an optional `account:` key per Agent.
- Ruling 52: the two-tier refusals are printed verbatim under the author's line.

It is also a useful probe for a later simplification of the core. The parts the layer never emits (declared Conduits, replace.v1, register-replacement, separate templates) show what the core could drop in v1.

## Verdict counts (minor)
The count holds, with fixes to the text. I diffed the scratch files task by task and got exactly the stated file lines: T1 17, T2 5, T3 0, T4 2, T5 0, T6 1, T7 1 (2 edits), 26 in total. The three `conduit install` commands carry chosen values (archive paths) and are the only such commands, so the plan's stricter count is 29 (T1 18, T2 6, T5 1). Against the baseline (33/16/0/8/3/2/2) that is less on six tasks and the same on T3. The pass also survives every harsher recount I tried. The worst case adds 1 line for each Apply? [y/N] answer, counts T7's remove and re-add as 2 lines, and adds 6 lines for the history convention. It gives less on 4 tasks, the same on 2, and T7 at 4 lines against 2, which is 2 lines more, not "much more". Every task reaches its stated end state on paper, and the person can still request turns through the derived `turn-<agent>` Conduits. Three things are wrong or missing. First, T3's answer elides agent-b, and the only source `conduit reach` could print is the Codex Type's description, which finding 4-06 contradicts. Second, the generated render and the request estimates do not match the design's own text. Third, some person actions and engine reads are not counted. None of these changes the verdict, but the T3 claims and the numbers that describe what the CLI generates need correcting. All of it remains a paper count (ruling 45).

RECOUNT (files in <scratch>/layered/, checked with grep and diff)
- T1: t1.yaml has 17 non-blank lines and no comments. Command with values: `conduit install` with 3 archive paths, 1 line. `conduit apply` carries none. Total 17 (strict 18), baseline 33: less.
- T2: diff t1 to t2 adds 5 lines (3 for the history Resource, 1 reach line per agent). Command with values: 1. Total 5 (strict 6), baseline 16: less.
- T3: 0 lines; `conduit reach` carries no value. Baseline 0: same.
- T4: diff t2 to t4 changes 2 lines (lines 15 and 21). Baseline 8: less.
- T5: t4 and t5 are identical, so 0 file lines. Command with values: 1 (the 0.2.0 archive path). Total 0 (strict 1), baseline 3: less. On the files-only count, the whole upgrade decision is a side effect of `install` changing the lock's "selected" version.
- T6: 1 line changed (line 6). Baseline 2: less.
- T7: 1 line removed then restored, counted once by the baseline's own rule ("counted once in a task even where it was edited more than once", results-2026-11-09.md:122). Baseline 2: less.
- Totals: 26 file lines, plus 3 command lines with values, 29 in all. Lines naming a contract address: 4 (3 in T1, +1 in T2), as stated.
- Same basis as the comparison runs: the coordinator's 332 and the fresh author's 646 count files only, with commands listed apart as requests (coordinator-run-2026-11-10.md, Per task), so 26 against 332 compares like with like.

SENSITIVITY (every case still passes)
- Count each Apply? [y/N] answer as a line, as the baseline counted 9 lines of trust-dialog answers through the files they write. That adds 7 (T1, T2, T4, T5, T6 one each; T7 two). Strict totals become T1 19, T2 7, T4 3, T5 2, T6 2 (equal to the baseline), T7 3 (baseline 2). That is less on 4, the same on 2, and 1 line more on T7. `apply --yes` removes these.
- Count T7's remove and re-add as 2 lines: T7 equals the baseline's 2.
- History posting convention: the baseline wrote 6 lines (CLAUDE.md and AGENTS.md). Here the convention exists only in the history Type's description. The engine derives tool descriptions from the Operation contract (reference-model/crates/core/src/schemas.rs:1112, IFC-033), so no agent is told the convention. If an author must state it, T2 becomes 11 or 12, still under 16. This parity question applies equally to the coordinator's run.
- Worst case, all three together: 19 / 13 / 0 / 3 / 2 / 2 / 4. Less on 4 of 7 (most), and T7 is 2 lines more than the baseline, not much more.

TASK COMPLETENESS
- T1: met. A gets file.read.v1 and directory.list.v1, and B also gets file.write.v1. These are the only three operations the filesystem package maps (conformance/fixtures/s1/packages/filesystem/conduit.yaml). Agent `model` is required but has a schema default of '', and the coordinator also omitted it. Turns come from the derived `turn-agent-a.turn.v1`. Tool naming as local name, dot, export name matches IFC-012 and core access.rs:378.
- T2: met. Both Agents get read, list and write on `history`. The history Type offers filesystem#directory.v1, so D5 and D6 resolve the short names unambiguously. Posting remains an unenforced convention, as in both runs.
- T3: incomplete (main finding below).
- T4: met. Close, register under the same name, then open works: a closed Conduit is removed (LIF-035) and its name can be given again (spec/definitions.md "remove"). The coordinator's log at 11:27:10 confirms the names were free. The template hash avoids 4-01's misleading names.
- T5: met. filesystem 0.2.0 still maps only the same 3 tools, operations are explicit, so no tool is added. The history Resource stays on server 2.1.0 through the history package's own pin of filesystem 0.1.0 (NAM-014), as in the baseline. The `unregister.v1` revision is a required input (core/operations.yaml:350-352), so step 10 is grounded.
- T6: met. One caveat: "registered under the free name `shared`" holds only if the retired `shared` from T5 had its shutdown confirmed (LIF-035, LIF-017). Otherwise rule D7 gives `shared-3`. The plan states it without that condition.
- T7: met. Closing project-agent-a leaves B's Conduits untouched, and the restore is an explicit addition (C1, with C8 not triggered).
- E3 is verified: lifecycle.rs:88-94 refuses an open only when the consumer or target is retired.

MAIN FINDING: T3 IS HAND-WAVED
- The reach output elides agent-b ("agent-b: …").
- The only machine-readable source for reach around the engine is the Type description. For codex.v1 it says "other-Agent ... reach is disabled" (packages/agent-codex/contracts.yaml:5-10).
- Finding 4-06 (log-increment-4.md:17) contradicts this. Codex is still offered collaboration.spawn_agent, send_message, list_agents and three more, settled as a stated route. The fresh author's `list_agents` worked. The Codex README (lines 94-95) still says these are no longer offered.
- The output also omits reach both runs' answers or READMEs record: the shared subscription login and the owner's OS user (both READMEs, lines 38 and 74/84), Codex reading `~/.codex/AGENTS.md` (finding 3-07), and Codex's apply_patch and request for user input.
- So a one-command `conduit reach` would print a false statement for B. A complete answer still needs the READMEs and the findings, as the coordinator needed 2 commands, 2 READMEs and the closeout. The claim of "less inspection effort" is unsupported. Lines are unaffected (0).

INCONSISTENCIES IN WHAT THE CLI GENERATES (not authored, so the line count is unaffected)
- t1-render.yaml has 19 documents and 171 lines, but lines 164-171 are a Pool template `s1-applied` that stores apply's state in annotations. That is the mechanism the design lists under "Rejected inside this angle". Without it: 18 documents, 162 lines, a ratio of about 9.5:1, not 10.7:1. The design's text says "19 documents" but lists 18.
- Engine request estimates do not match their own lists:
  - T1 says 11 and lists 10 (+3 installs).
  - T2 says about 6 and lists 4 (+1).
  - T4 says 8 and lists 6 (3 per Agent).
  - T6 says about 12 and lists 10.
  - T7 says about 2+2 and lists 1+2.
  - The lists sum to about 48, not "about 59".
- Every figure leaves out the reads that plan and apply need: inspect.v1 per object and records, plus M3's pre-flight "against the store's state". Today `conduit inspect` cannot run while the engine serves (4-09), so that read path is unspecified.

UNCOUNTED OR UNLISTED
- 7 y/N confirmations, which are person actions.
- `conduit init`, which in this design also binds the project directory to the store through `.conduit/config`. It is treated as harness, consistently with both runs, but it is closer to author setup than before.
- Digests: `install` computes them from whatever archive it is given. The hand-copied digest, which the measurement plan counts and lists apart, also checked the archive against a published value. That check disappears: allowed by "Nothing generated by a tool counts as authored", but it is a weaker check.
- Time without access in T4 to T6 spans close, then register, then open, which is three requests, not two.

NOT A LINE ISSUE, BUT MATERIAL
- None of `install` writing a lock, `render`, `plan`, `apply` or `reach` exists. Today's usage line is at reference-model/crates/shell/src/main.rs:10, `--store` is required at main.rs:105, and `install` takes ARCHIVE#SHA256.
- The numbers are a paper count, which ruling 45 does not accept as evidence. A fresh author has to run it on a built CLI.

## Verdict safety (major)
Holds with fixes, but not as stated. The engine-level guarantees survive because every step is still an ordinary request that is authorized, recorded and checked by the unchanged core: agent grants keep explicit operation lists, so T5 adds no tool; every permission, derived ones included, is a real Conduit; lifecycle and records are unchanged. Two of the design's central claims do not hold. (1) "Spec changes: nothing." Plan, drift detection, C7/C8 and `conduit reach` (T3) need a list of what exists, and the core does not offer one: inspect.v1 takes one named object. Today's CLI gets around this by reading the store's state.json and the per-instance token files directly (author.rs:18 calls it a "CLI-only supplement"). That bypasses Conduits and records, works only on the engine's machine, and is not available to an Agent operator. (2) The guarantees that move into the CLI's discipline do not hold under several actors, several working copies or peer mode. open.v1 and close.v1 have no expectedId and no revision. Apply state is a local file. Hash-named templates can still be changed with replace.v1. D2 and D3 hard-code root, whose Conduits ACC-024 makes impossible to close. Placement is missing, so C4 can move a directory to another machine. I found 19 counterexamples: 6 major, 6 moderate, 7 minor. None breaks a guarantee in S1 when one operator runs alone on one machine. Several break G3, G10, G16, test 8 or ruling 52 as soon as a second actor, a second working copy, peer mode or a managed node appears. The fixes need at least three small spec changes: a way to list Conduits by consumer and by target, expectedId on open and close, and apply state held by the engine (or a precondition on `from`), plus format-level fixes.

Read only; nothing in the repository was changed. Spec paths are relative to <home> Design files are in <scratch>/layered/.

WHAT SURVIVES (checked against the spec)
- G1 for Agent grants. Every `reach` compiles to an explicit operation list, and the T1 render's Conduit templates list read/list and read/list/write. In the fixtures, filesystem 0.1.0 and 0.2.0 have identical contracts.yaml, and upgrading adds no tool.
- G2. D2 and D3 are real, inspectable Conduits, not exceptions in ACC-033.
- G4 at the engine. Each part is a separate recorded request; record.v1 holds IDs (spec/core/results.yaml:185-188), so reusing a name does not make records ambiguous.
- G11 and the immutability rules are untouched.
- E3 relies on behaviour the spec allows by omission: reference-model/crates/core/src/lifecycle.rs:87-94 rejects an open only when the consumer or target is retired (CON-031). Opening for a merely registered consumer is not specified either way.

COUNTEREXAMPLES, most severe first

1. MAJOR: "no spec change" is false, because the core has no way to list objects.
- inspect.v1 takes one `object` reference (spec/core/operations.yaml:361-428). No core operation lists objects or the Conduits a consumer holds.
- The shell's own inventory says so: reference-model/crates/shell/src/author.rs:18 "CLI-only supplement: inspect.v1 has no defined consumer-Conduit inventory". It reads state.json with store::private_read. server.rs:773-788 reads `client-tokens/<id>` to act `--as` any instance.
- Plan, C7 (unmanaged name), C8 (closed by someone else), drift, M3 pre-flight ("against the store's state") and `reach` all need that inventory.
- Result:
  - (a) The CLI reads the store file. Those reads go through no Conduit and leave no record. They work only on the engine's machine (G16, ruling 51), and an Agent operating through its Conduits cannot do the same (test 5).
  - (b) Or `reach` is built from the file plus inspection of named objects. It then misses every Conduit opened outside the file and understates what an Agent can reach, which is dishonest under ruling 52, G15 and T3.
- Work order 027 (item 4b) already sent finding 4-09 to "the CLI's output alone".
- Fix: a core operation, or an inspect.v1 output, listing the Conduits by consumer and by target, authorized through `management`; plan and `reach` use only that. `reach --as` must never act under the Agent's token, or records attribute the requests to the Agent (test 7).

2. MAJOR: a stale plan can undo another actor's narrowing (G10, G3).
- open.v1 and close.v1 take only `conduit` (spec/core/operations.yaml:709-765). LIF-037's expectedId covers set-up, retire, inspect and unregister only, and REV-012 gives opening and closing no revision.
- Scenario:
  1. The author runs `conduit apply`; it prints a plan with "close/register/open project-agent-b".
  2. Before the author answers y, operator S closes project-agent-b and opens a narrower one under the same name through the escape hatch, for example with a path filter. The format cannot express filters at all.
  3. The author answers y. The close removes S's Conduit, which carries no precondition, and the register and open bring back the wider one.
- The design's "stale plans are caught step by step" does not cover close.
- Related: "Apply reports drift and never repairs it silently". Drift that narrows is still repaired once someone answers y.
- Confused-deputy variant:
  - An actor holds management register.v1 plus a right to close bounded to Pool s1.
  - Between apply's register and open, it closes the newly registered `project-agent-a` and registers its own wider one, with itself as consumer, under that name.
  - Root, whose right to open is unbounded, then opens the actor's Conduit.
- Fix:
  - Extend LIF-037 to open.v1 and close.v1 (spec change); apply sends the ID that register returned and the ID it read at plan time.
  - Re-read state right after confirmation.
  - Treat any narrowing drift as a conflict that needs an explicit flag.

3. MAJOR: G3 at the level of intent rests on a local file, and several operators defeat it (test 8).
- C8 is keyed by the grant's key and by `.conduit/applied.json`.
- The records fallback is not dependable:
  - No core type keeps request inputs (spec/core/types.yaml:46-48). The record of a close.v1 call therefore does not say which Conduit was closed.
  - How records of removed objects are read is implementation-defined (DL8, LIF-019).
- Scenarios:
  - (a) Teammate L pulls a file that contains grant `project` for agent-a. Security operator S had closed that grant with `conduit call access.close.v1`. L's applied.json never held the grant, so the plan shows a plain "+ register, open project-agent-a" and access is restored without any conflict.
  - (b) An author renames the key `project` to `workspace`: C2 drops the old grant and C1 adds the new one, so a revoked grant comes back under a new name.
- The T1 render even contains a Pool template `s1-applied` that keeps "keys to Conduit IDs" in annotations (t1-render.yaml:165-170), a mechanism the design text says it rejected. The 19 documents and 171 lines counted for T1 include it.
- Fix:
  - Keep apply state and revocation tombstones on the engine, with a lock across a whole apply, as Terraform's remote state backends do.
  - Key the conflict by consumer and target, not by grant key.

4. MAJOR: hash-named templates are not content-addressed, which brings back 4-01's coupling.
- (a) Anyone holding management replace.v1 can change a template (SUB-009). An instance document's `from` carries no revision or ID precondition. Every later C3 or C4 instantiates the replaced content (INS-020 protects only instances that already exist).
  - Content addressing makes sharing the norm: `s1-directory-v1-b17d04` serves project-agent-b, history-agent-a and history-agent-b, and project-agent-a after T4.
  - One replace.v1 therefore widens every grant re-registered from it afterwards, and root opens them.
- (b) C10 leaves Conduit package pins out of the hash.
  - After T5 the same name holds filesystem 0.1.0 on the old store and 0.2.0 on a fresh one.
  - `render` (manifest from the lock) misstates what is registered (G6).
- (c) The hash is 6 hex digits, about 24 bits. If the CLI reuses a template by name or hash alone, a collision, accidental or crafted, hands out a different grant.
- (d) Annotations misattribute the grant.
  - After T4, agent-a's read-write grant comes from b17d04, whose `conduit-project/source` is `#/agents/agent-b/reach/project` (t1-render.yaml:147).
  - An auditor asking why A can write is pointed at B's line (ruling 52).
- Fix:
  - Compare the full stored content, including the packages it holds, with the compiled template before every register, and re-check after confirmation.
  - Accept only templates whose IDs apply recorded.
  - Better: a precondition on `from` (expected template ID or revision) in the core, or one template per grant.

5. MAJOR: root is hard-coded as the operator, and ACC-024 freezes the derived Conduits.
- D2 and D3 compile to `consumer: {kind: Interface, name: root}` (t1-render.yaml:35, 102, 120).
- Test 8 says a human's access is an Interface holding grants. In ruling 51's peer mode, the owner's Interface on a peer engine is not that engine's root. D2 and D3 would then grant the peer's root and not the operator, and the operator's own Pool requests are refused.
- ACC-024 and ACC-012: no request ever closes a Conduit whose consumer is root. So C3 can never apply to D2 or D3:
  - `turns: root` -> `none` cannot be applied.
  - D3 is "expanded once at compile time", but in fact it is re-expanded on every compile. If an Agent package adds an operation to conversation.v1, the close is refused mid-plan, after earlier steps have run.
  - D2's list is frozen for good, and it omits reconcile.v1. ACC-033 row 13 needs reconcile.v1 on a Conduit to Pool s1 to reconcile an unknown Resource set-up or retire. That is exactly E2's "reconcile command for unknowns", which would be refused.
- Fix:
  - The consumer is the Interface the CLI authenticates as, shown in the plan.
  - D2 lists reconcile.v1 from the start.
  - Freeze D3's expansion in state.
  - The planner knows ACC-024: it explains that the Agent must be replaced, and never sends the close.

6. MAJOR: placement is missing, so C4 can move a directory to another machine (G16, ruling 4 scope guard).
- The format has no `placement`, and PLC-002 lets the machine provider choose.
- C4 makes a new instance at every upgrade or change of configuration. On a managed engine with several nodes (ruling 51), `shared-2` with `root: <lab>/shared` can land on another node: the same path, a different, empty directory.
- Both Agents are then re-pointed to it under unchanged tool names.
- Fix: the format carries placement. A replacement inherits the predecessor's resolved node unless the author changes it, and the plan shows the node.

7. MODERATE: during re-pointing in T6 the Agents use different directories, and writes can be torn.
- C4 re-points one Agent at a time. In T6, between steps 5 and 8, A writes to the old directory while B uses the new one, and A's writes in that window end up in a retired Resource.
- Retire is sent without waiting for invocations through the closed Conduits to finish. ACC-034 lets them run on, and CON-008 refuses requests from the moment a retirement is dispatched, so writes can be cut off mid-way.
- Retiring the old Resource also closes every Conduit to it (CON-008), including delegations and filtered Conduits made outside the file. The plan cannot list those (see 1), and C4 does not re-point them.
- In T5 two server versions run on the same root at once.
- Ruling 52 point 6 and measurement-plan.md:60-62 require every interval without access to be listed; the design says "not measured".
- Fix:
  - Close every grant to the old target first and wait until no invocation is in flight; then open them all.
  - List the Conduits the retirement will close, and every interval.

8. MODERATE: T6 changes the target silently behind stable tool names (ruling 52).
- A tool's description is the contract's description (IFC-033) and does not name the target. IFC-038 sends only "list changed".
- After T6, `project-agent-a.file.write.v1` writes to `<lab>/shared-new`, and nothing the Agent can see says so.
- Fix: a notice to the consumer naming the new target, or the target in the tool's description (a spec change in IFC-033).

9. MODERATE: D7's naming is not one-to-one.
- OBJ-001 allows hyphens in names, so:
  - grant `project` of agent `agent-a` and grant `project-agent` of agent `a` both give `project-agent-a`;
  - grant key `turn` of agent X gives D3's `turn-X`;
  - agent `admin` with grant key `s1` gives D2's `s1-admin`.
- An authored Resource key `shared-2` collides with the name generated for `shared`'s successor.
- A retired instance whose shutdown is unknown keeps its name (LIF-007), so names drift to `shared-3`.
- Without the local ID map, the diff matches file entries to the wrong objects.
- Fix: an encoding that cannot collide (a separator that keys may not contain); reserved prefixes; successor names from a space authors cannot write; the key-to-ID map as the authority.

10. MODERATE: what is left of the T5 hole.
- "No grant gains an operation" is the wrong check. NAM-024 lets a later release add optional inputs to an operation already granted, which can widen what it does (for example an append mode on file.write.v1).
- IFC-033 then changes the tool's schema under the same name.
- The implementation changes from 2.1.0 to 2.2.0 too.
- Fix: the plan compares each granted operation's contract (inputs, effect) and the implementation's digest, and an explicit flag is needed to accept any addition.

11. MODERATE: the idempotency claim fails across re-runs (G9).
- OPS-009 and OPS-010 match a repeat only on the same key and contents.
- The keys `apply/<plan-digest>/<step>` change as soon as earlier steps have succeeded. A re-run then sends an unknown step again under a new key, which treats an unknown outcome as failed.
- Fix: refuse to plan while any of apply's invocations is listed in `unreconciled`. Derive and keep keys from (object, action, expected ID or revision).

12. MODERATE: an unrelated problem can block a revocation.
- Pre-flight is all or nothing ("nothing is sent if pre-flight fails"). C5 refuses a plan while an Agent package's newer version is selected, which happens on `install` alone.
- An urgent removal of a line in T7 is then blocked by an unrelated error. The escape-hatch close that remains then makes later applies refuse under C8.
- Fix: closes and narrowings are always plannable on their own, and run first.

13. MINOR: actions waiting on a person are not handled (rulings 49 and 50).
- A step can only succeed, fail or end unknown. An Agent's set-up that leaves a sign-in pending reports success while its turns are refused.
- The root session, which is the CLI, gets URL elicitations mid-apply with nothing defined to handle them.

14. MINOR: M3 prints values.
- Its example prints the value the author wrote. A literal mistakenly written at a place marked secret-reference.v1 or isSecret is printed into terminal and CI logs (G5).
- Fix: redact those places.

15. MINOR: the lock can swing versions back and forth.
- Two working copies with different `selected` versions alternately upgrade and downgrade a Resource through C4, and each plan shows it as an ordinary replacement.
- Fix: a downgrade needs a flag; the engine records the digest of the last lock applied.

16. MINOR: D6's scope is undefined.
- It could be installed packages or the inferred capability. After an unrelated install, or when a release adds a `.v2` contract, an unchanged file can stop compiling (safe, but the file's meaning changes).
- Fix: scope D6 to the capability pinned by the Conduit's held template, and keep the resolved names in state.

17. MINOR: the compiled output depends on the engine's state (G6).
- Names (`shared`, `-2`, `-3`) and the pins of reused templates depend on what is in the store. `render` is therefore not a function of (file, lock), and with no AUT-* spec two CLIs may disagree (A34).

18. MINOR: preservation test 5 holds only in letter.
- C7, C8, the ordering of narrowings and pre-flight live only in the CLI, and the CLI needs access to the store's files as root (see 1).
- An Agent operating the system through its Conduits gets none of these protections.

19. MINOR: the format covers only S1's subset.
- It cannot express filters, Conduits to `access` or `management` for Agents, Agent-to-Agent Conduits, Interfaces other than root, placement, or more than one Pool. S2 to S9 would all go through the escape hatch.
- That means drift that never ends, exposed to finding 2's repair on y.

THE SMALLEST SET OF CHANGES THAT WOULD MAKE THE DESIGN HOLD
- In spec/:
  - (i) a way to list Conduits by consumer and by target, authorized through `management`;
  - (ii) expectedId on open.v1 and close.v1 (extending LIF-037);
  - (iii) apply state with tombstones held by the engine, or a precondition on `from` with the template's ID or revision.
- In the format:
  - placement;
  - an operator Interface instead of `root`;
  - D2 with reconcile.v1;
  - D3 frozen;
  - names that cannot collide;
  - closes run first;
  - drained re-pointing;
  - narrowing drift as a conflict;
  - contract diffs at upgrades.

## Verdict cost (minor)
The estimate holds, with fixes. As stated, the design needs no change to spec/, the core, the loader, the conformance runner or any of the 367 vectors. I checked each request it relies on against the promises: ACC-033 rows 1, 2, 4, 6, 7 and 10; OBJ-011 and LIF-035, which free a closed Conduit's name for re-registration; LIF-007 and LIF-017; LIF-037 expectedId on retire and unregister; INS-020, under which a template can be unregistered while it still has instances; CON-031, which refuses only a retired consumer, so E3 is legal; and IFC-012 tool names. Almost all the cost lands in the author-side CLI: about 3,000 to 4,500 lines of Rust source and 2,500 to 3,500 lines of tests. That is large for the shell crate and medium for the repository. Three of its features work today only because the CLI shares a machine with the engine and reads its private store (state.json and the token files under client-tokens/). The spec does not promise them, so the claims of "nothing in spec/" and "preservation test 5 holds" are overstated. The three are: the inventory behind `conduit reach` (T3) and drift detection; M3's pre-flight against the store's state; and M5's annotations shown in inspection. To make the layer honest and portable needs about 2 to 4 promises touched and 0 to 1 added, plus one change to inspect.v1's output schema; that is small. I also found four defects in the format that change no cost and must be fixed before v0 freezes. (1) D2's Conduit omits reconcile.v1. (2) The Conduits whose consumer is root can never be closed (ACC-024), so D2's and D3's output is frozen for good. (3) C8 covers only grants. (4) The name-reuse rule covers only Resources. Migration is fully backward compatible at the document and engine level. Existing objects cannot be adopted into a project file, and the compiler's naming and hashing rules become a new compatibility surface that is not specified. For someone implementing an engine, the spec costs the same to read; the system as a whole costs more.

All paths are under <home> The repository was only read.

## 1. spec/: promises

**A. Needed by the design as written: 0 changed, 0 added, 0 removed.** I checked each mechanism it depends on:
- **C3 and T7 (register again under the same name):** OBJ-011 says "A name becomes free once its object no longer exists". LIF-035 says "A Conduit is so removed when it is closed". close.v1 confirms it: "it is removed when it is closed, however it was closed".
- **T6 (reusing `shared` after T5):** LIF-007 holds the name only until the old instance is removed. LIF-035 removes it once its shutdown is confirmed. If the shutdown is pending or unknown, it needs LIF-017 unregister, or the `<key>-<n>` rule.
- **Unregistering the old template in C4:** INS-020 says "Having instances does not prevent a template from being replaced or unregistered". unregister.v1 requires `revision`, which the plan already sends.
- **expectedId:** LIF-037 covers set-up, retire, inspect and unregister, and not open or close. The design states this.
- **E3 (open a grant before the Agent is set up):** CON-031 refuses only when the consumer or target is retired. lifecycle.rs:88-95 matches.
- **Authority:**
  - T1 registers the Pool and the Resource in one submission, so row 2 applies (management only).
  - T2 registers into an existing Pool, so row 1 applies (management and s1-admin, both listing register.v1).
  - Setting up and retiring a Resource falls under row 4, through s1-admin.
  - Inspecting a Resource falls under row 6, and anything else under row 7.
  - Opening and closing fall under row 10, through root's unbounded access (ACC-022).
- **pool-management.v1:** it lists register, inspect, set-up and retire (core/capabilities.yaml:163-170).
- **Tool names:** IFC-012 joins the local name and the export name, so tools are `project-agent-a.file.read.v1`.
- **Capability check for T2:** the history Type offers `example.org/s1/filesystem#directory.v1`, so the existing read/list/write template applies to `history` (CON-021 passes).

**B. Recommended to make the layer honest and portable (preservation test 5, G16, ruling 52): about 2-4 promises touched, 0-1 added. Small.**
1. **An inventory in inspection.**
   - What is missing: inspect.v1 returns `declared: {type: object}` and says nothing about the Conduits an instance consumes or is the target of. No list operation exists.
   - What the reference does today: author.rs:18 says "CLI-only supplement: inspect.v1 has no defined consumer-Conduit inventory". It builds that list by reading the engine's private state.json (author.rs:7-13; server.rs:773 also reads it, and the token from `client-tokens/`). Work order 027 item 4 said the same: if inspect.v1 has no place for this, do not change spec/; add it to the CLI's output alone.
   - Why it matters: `conduit reach` (T3: "what each agent can reach"), drift detection and C7 then work only on the engine's machine, as root, against an unspecified store format. An Agent, a remote CLI or a second engine (A34) cannot do the same.
   - The fix:
     - LIF-013 and inspect.v1's outputSchema list the Conduits of which the instance is the consumer, and those of which it is the target, each with name, ID, target and state.
     - One sentence in ACC-033 or LIF-013 settles what a caller with only a Pool's authority sees (ruling 52's limit on disclosure).
     - Vectors: 1-3 new ones, plus about 1-5 snapshot updates. Only conventions/records/inspection snapshots `declared`.
2. **Annotations in inspection, for M5.**
   - What the spec says: OBJ-018 lets `metadata.annotations` be written but gives them no meaning, and no promise says inspection returns them.
   - What the reference does: it keeps metadata (registration.rs:541) and leaves it in `declared` (records.rs:261-279), but that is reference behaviour only.
   - The fix: one sentence in OBJ-018 or LIF-013 saying they are kept and returned as given.
3. **Optional: a server-side dry run, for M3.**
   - The current M3 needs a full `conduit_core::State`. The core's `step()` (lib.rs:268) is pure, so the simulation itself is cheap, but the State comes only from the private store. So M3 as written is a reference-model-only feature.
   - A `dryRun` input on register.v1, as in Kubernetes, would cost about 2 new promises (SUB, OPS), touch about 3 (ACC-033, REC-001, OPS-009), add about 4-6 vectors and about 100-150 lines in registration.rs.
   - It cannot pre-check a break-before-make sequence: the new register collides (OBJ-002) until the close has happened.
   - The cheaper honest answer: register.v1 is already all-or-nothing (SUB-003), so map its diagnostics back afterwards. Check client-side what can be checked: IFC-014 collisions and CON-021, from inspected data.

**C. Only if promoted to spec/authoring/ (the design's "LATER", A34):** about 25-30 AUT-* promises, one per rule D1-D9, C1-C10, E1-E3 and M1-M5. Also:
- a project-file schema;
- an "Authoring" conformance claim;
- about 50 (file, lock, state) → plan vectors;
- a plan-vector mode in reference-model/crates/conformance (about 300-600 lines);
- an extension to tools/check_schemas.py;
- probably a note in spec/README's layer table, since this would be a fourth surface.

That is medium-large: comparable to one area of the spec.

## 2. spec/schemas/
None as designed (0 of 8 files). Recommendation B1 changes no schema file, only inspect.v1's outputSchema in spec/core/operations.yaml. Promotion would add one (for example project.yaml).

## 3. spec/core/operations.yaml
- **As designed:** 0 of 23 operations.
- **Recommended:** inspect.v1's outputSchema (B1).
- **Optional:** a `dryRun` input on register.v1 (B3).
- No new operation, and no change to ACC-033 rows.

## 4. conformance/
- 0 of 367 cases invalidated: the core is unchanged.
- B1 updates about 1-5 snapshots and adds 1-3 cases.
- The provider variant touches the live-agents case:
  - conformance/language/registration/live-agents (its fixture.json pins the Agent package digests);
  - conformance/fixtures/agents/*;
  - conformance/fixtures/s1/packages/agents/contracts.yaml, which also makes the two flags required.

## 5. reference-model/

| Crate | Change | Size |
| --- | --- | --- |
| loader (2,879 src) | None. yaml.rs and diagnostic.rs already give Position (line, col) and pointer, which M1 can reuse; at most about 50-100 lines to expose them | Small |
| core (10,465) | None. With B1, about 30-60 lines in records.rs inspect. With B3, about 100-150 in registration.rs | Small |
| conformance (2,741) | None. Plus 300-600 lines only if promoted | — |
| shell (author.rs 284, main.rs 407, today about 6,986 src incl. unit tests) | The new layer: see the breakdown below | Large for this crate |

Breakdown of the new layer. I recommend a separate pure crate (say crates/project): plan is a pure function of (file, lock, observed state) → steps, testable like the core. Pure parts:
- format parse and validate: about 300-400 lines;
- compiler D1-D9, including canonical hashing (reuse the RFC 8785 code of SUB-008) and the D5/D6 inference from package contracts read from the archives (reuse archive_metadata.rs): about 800-1,200;
- planner C1-C10, with narrow-before-widen, conflict and drift: about 700-900;
- source map and three-part diagnostics M1-M4: about 300-400.

Shell glue:
- install and lock: about 200-300 lines;
- state reader (inspect-based, plus applied.json): about 300;
- executor: one reusable MCP session (today server::client opens a session per call, finding 4-15), idempotency keys, revisions, expectedId, per-step reports, resume: about 400-600;
- render, reach, help and exit codes (4-13, 4-14): about 300.

It must also handle URL elicitations from ruling 49 during Agent set-up inside apply's session. Under `--yes` that is a defined failure, not a hang.

Totals: about 3,000-4,500 source lines (the design's 3,000 is the low end), plus about 2,500-3,500 test lines. Those tests are golden (file, state) → plan cases and an S1 T1-T7 run against a serving engine, as tests/changes.rs (851 lines) does today. Roughly 3-5 work orders.

## 6. packages/, tools/, docs/
- **Provider variant (optional, matches ruling 48):** give `accountServices` and `peerMessaging` the default `false` and drop them from `required` in packages/agent-claude-code/contracts.yaml and packages/agent-codex/contracts.yaml. That is compatible under NAM-024 (it accepts more), but it needs:
  - a version bump;
  - repacked dist archives and new .sha256 files;
  - the conformance fixtures above;
  - reference-model/crates/shell/tests/agents.rs;
  - the READMEs.

  Small.
- **tools/:** check_schemas.py and set_names.py are unchanged.
- **Docs:**
  - a format reference of about 30 rules, outside spec/ unless promoted;
  - a FIRST_BUILD note that the project file is what counts as authored;
  - an experiment plan stated in advance (ruling 45) for the fresh author's re-run.

## 7. Size summary

| Area | Size | What changes |
| --- | --- | --- |
| spec/ (as designed) | None | 0 of 455 promises |
| spec/ (recommended B1-B2) | Small | 2-4 touched, 0-1 new |
| spec/ (B3, optional) | Small-medium | about 2 new, 3 touched |
| spec/ (if promoted) | Medium-large | about 25-30 new |
| Schemas | None | 0 |
| Core operations | None | 0, or 1 output schema |
| Conformance | None | 0 invalidated, or about 1-5 |
| Loader | Small or none | — |
| Core | None, or small | — |
| Shell / new crate | Large | — |
| Conformance runner | None, or medium if promoted | — |
| Packages | Small, optional | — |

Every other approach the maps consider means a large change to spec/ (P1/P2: OBJ-002, OBJ-011, SUB-006, SUB-007, LIF-009, INS-001/002, ACC-033 and about 80+ registration vectors). This one is the cheapest by an order of magnitude.

## 8. Defects in the design that should be fixed before format v0 freezes
They change no cost estimate, but each becomes permanent once applied.
1. **D2 omits reconcile.v1.**
   - An unknown set-up or retire of a Resource is reconciled through the Pool that authorized it (ACC-033 row 13), so E2's "reconcile command for unknowns" fails for Resources.
   - root's Conduits are never closed at anyone's request (ACC-024; close.v1: "A Conduit whose consumer is the root Interface is never closed"). So s1-admin cannot be fixed later, only supplemented under a new name.
   - List register, inspect, set-up, retire and reconcile, and consider retire-with-resources.v1 for `--prune` of the whole project.
2. **What D2 and D3 compile for root is frozen for good.** Any later CLI that changes those expansions (more operations, another turn operation, a renamed `turn-<agent>`) cannot close the old Conduit. The format must state that root-consumed derived Conduits are add-only.
3. **C8 covers only grants.**
   - An Agent or Resource that someone else retired would be recreated by the next apply, with fresh grants. That is the pattern G3 forbids (RD2).
   - Generalize C8 to every managed object that was last applied and has since been removed by another caller.
   - D7's `<key>-<n>` rule also needs to cover Agents under `--replace`: LIF-007 holds `agent-a` until the old Agent's shutdown is confirmed.
4. **Reserved names.** The format must reject keys that collide under NAM-027 within one submission (a Resource named `s1`, `s1-admin` or `turn-agent-a`).
5. **What `install` stops checking, and fetch paths.**
   - `install` computes the digest from the local file: trust on first use, as go.sum does. G5 still holds, but the author no longer checks a published digest; say so.
   - The lock's `file://<lab>/…` fetch is a path on the engine's own machine. That works for S1 but not across machines (G16, INS-018), so the format should prefer https or a package-source provider.

## 9. Migration
- **Backward compatible at every layer below the CLI.** All current documents, stores, the 367 vectors and the core-level commands (submit, replace, retire, unregister, call, tools, inspect, records) stay valid. Nothing is a clean break.
- **But no adoption path.** C7 refuses objects the project does not manage, and the generated names (hash-named templates, `<key>-<agent>` Conduits) never match hand-written ones. An existing deployment is therefore rebuilt, or stays managed by hand. Adopting it under its own names would change tool names. The cost is low today, since only S1 stores exist.
- **A new compatibility surface the spec does not cover:**
  - the format version (`conduit: v0`);
  - the hash canonicalization;
  - D7 naming;
  - the annotation keys.

  A CLI release that changes any of them makes every generated template look changed, which would mean mass C4 replacements and closes and reopens. These must be frozen per format version. Hash the canonical document without annotations (OBJ-018 already ignores them), and record the format version in an annotation.
- **Every future core change now costs twice.** Ruling 49's expected-account field, ruling 50 enrollment, and ruling 51 peer mode with its two halves each need a spec change, then a format, compiler, source-map and test change. The CLI depends on the behaviour of about 27 promises: ACC-022, ACC-024, ACC-033, LIF-007, LIF-017, LIF-027, LIF-035, LIF-037, OBJ-002, OBJ-011, OBJ-018, INS-020, INS-025, CON-021, CON-031, NAM-006, NAM-010, NAM-017, NAM-023, NAM-027, PKG-005, PKG-006, IFC-012, IFC-014, REC-002, SUB-003, SUB-005. A change to any of them must be mirrored in the CLI. A rough multiplier of 1.3-1.5x per language feature.

## 10. Reading cost (concepts)
- **Someone implementing an engine:** +0 and -0. Still 455 promises, the frame, template and instance pairs, the 14 rows of ACC-033 and 367 cases. The machinery authors no longer use (declared Conduits, replace.v1, register-replacement.v1, LIF-009) stays to be read, implemented and tested.
- **An author above the CLI, when all goes well:** about 16 concepts drop out of view:
  - the frame (apiVersion, kind, metadata, spec);
  - template, instance and `from`;
  - inputs, `to` and arguments;
  - the manifest and its digest;
  - Pool and `owner`;
  - root's Conduits to the Pool;
  - root's turn Conduits;
  - consumer and target;
  - capability;
  - qualified operation addresses;
  - declared versus standalone Conduits;
  - ACC-033 routing.

  About 8 replace them: project, resource, agent, reach grant, ops, lock, plan and apply.
- **Where the core still shows through:** tool names (`project-agent-a.*`), generated names in records and inspection (`shared-2`, hash-named templates), partial applies, unknowns and the turn deadline of finding 4-12. When those happen, the author needs both sets.
- **A CLI implementer or second CLI (A34):** +about 27 rules and about 10 concepts (project file, lock, applied state, plan, step, conflict, drift, generated-name scheme, source map, rule names). Without promotion, the only definition is the reference code.
- **Net:** the spec costs the same to read; the system costs more. FIRST_BUILD.md's remedy for thesis 1 is "Simplify the documents: fewer concepts, better defaults", and thesis 2 ("the language fully determines what a set of documents means") does not cover the project file unless it is promoted. Whether tooling meets the language thesis stays the owner's call.

## 11. The design's own claims, checked
- **26 lines:** confirmed. The scratch files give 17 lines for t1.yaml and 22 for t2.yaml, a difference of 5. diff of t2 and t4 shows the 2 changed lines.
- **Rendered T1:** it uses only forms the spec allows: instance metadata with annotations (OBJ-013, OBJ-018), one-off templates, root as a Conduit's consumer.
- **No paper count here is evidence under ruling 45.**