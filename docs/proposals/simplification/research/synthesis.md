Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

## recommendation
PACKAGE: "Layered authoring, small core additions". Authors write one short project file. The CLI compiles it into today's canonical documents and changes what exists with an explicit plan/apply that sends only requests that already exist. Five small core additions give that CLI what it needs to be safe and honest. Every object stays immutable, every grant stays a real Conduit with an explicit list of operations, and nothing runs without a request. The base is design E, the only design whose cost verdict was minor. It takes the safety fixes every skeptic demanded and the best pieces of A, C and D. No ruling among 8, 18, 21, 22, 27, 29, 31, 35 and 37 is amended. The core additions extend ruling 44's D12 list and change IFC-012 and IFC-033, so they need the owner.

Coordinator per-task baseline used below: T1 154, T2 73, T3 0, T4 2, T5 62, T6 29, T7 12. Package result: T1 15, T2 5, T3 0, T4 2, T5 0, T6 1, T7 1 file lines, plus 3 command lines that carry values.

1. Project file and compiler (CLI only; spec unchanged)
- What changes: one file, conduit-project.yaml, with `conduit: v0` stated once and `project: <name>`.
  - `resources:` and `agents:` are keyed maps, each object defined inline. Each grant is one line under its Agent's `reach:` map: `<key>: {to: <resource>, ops: [<export name>...]}`. Optional `capability:`, `requirements:` (filters), `placement:` and `owner:` keys remain available.
  - Framed documents placed beside the file pass through unchanged, so S2-S9 constructs stay writable (scope guard, ruling 4).
  - The compiler emits canonical documents:
    - one fresh one-off template per instance, registered in the same submission;
    - every Agent grant becomes a standalone Conduit carrying `as: <key>`;
    - operations are written out as full addresses;
    - the capability is inferred as the only one the target's Type offers that holds every listed operation, otherwise compilation is refused and the candidates are named.
  - `conduit render` prints the canonical form, as `docker compose config` does.
- Costs it removes: frames, `---`, `spec:`, the template/instance split, input plumbing, `config: {}` and `capability:` lines.
- Lines saved vs the coordinator: T1 70 (frame 54, plumbing 10, other 3, compressing the stated facts 3); T2 41 (26 + 7 + 2 + 6).
- Safety fixes the skeptics demanded:
  - Templates are never shared or reused; this removes E #4 (replace.v1 hijack, 4-01 coupling, annotations pointing at the wrong source).
  - Keys may not contain `--`. Generated names use `--`, live in a reserved space, increase monotonically (shared, shared--2, ...), stay within 63 characters and never collide (E #9).
  - `placement` is in the format, and a replacement inherits its predecessor's resolved node (E #6).
  - A source map gives three-part diagnostics: the author's line, then the core diagnostic verbatim with its promise ID, then the generated document. Values at places marked secret-reference or isSecret are redacted (E #14, ruling 52).
  - Short operation names resolve only within the grant's capability at its pinned version; anything ambiguous is refused (E #16). Versions stay in the names (`file.read.v1`).
  - YAML is the documented form.
- Cost: a pure Rust crate of about 1,100-1,600 source lines plus golden tests. No spec, schema or vector change.

2. CLI-derived defaults (spec unchanged)
- What changes:
  - A default Pool named after the project owns every Resource without `owner:`. An optional `owner:` allows several Pools.
  - An operator Conduit to that Pool lists register, inspect, set-up, retire and reconcile.
  - An operator `turn` Conduit to each Agent, on by default; `turns: none` turns it off.
  - All of these are ordinary documents the CLI writes, each annotated with the rule that made it, and shown in plan, render and inspection.
- Costs it removes: the root Interface's Conduits to its own Pool and its turn Conduits (category 4), the Pool documents and `owner:` lines (category 11), and the first mistake of both authors.
- Lines saved: T1 56, T2 19.
- Safety fixes:
  - The operator is the Interface the CLI authenticates as (root in v0), named in the plan, never hard-coded (E #5, ruling 51).
  - reconcile.v1 is included from the start (E cost defect 1, ACC-033 row 13).
  - Derived Conduits whose consumer is root are add-only. The planner never sends their close (ACC-024) and explains why.
  - A first apply refuses to adopt an existing Pool with the project's name (D #4).
  - For each new Resource, the plan lists every Conduit and bound that comes to cover it (D #4).
  - No exception is added to ACC-033 and no first-start change is made (G2, rulings 29 and 51).
- Cost: part of the compiler.

3. `conduit install` writes the pins (CLI)
- What changes: install sends install.v1 for each archive and writes version and digest into the file's `packages:` section, as `cargo add` and `go get` do. These tool-written lines are not authored.
  - `ARCHIVE#sha256` stays accepted, to assert a published digest.
  - Installing a newer version selects it. A downgrade needs a flag.
- Costs it removes: hand-copied digests and repeated manifests.
- Lines saved: T1 11, T2 8, T5 2 (T5 becomes one command line). It also removes the manifests counted inside category 8.
- Safety:
  - Trust on first use is stated (go.sum-style), and the digest is still checked at each later install (PKG-026).
  - Fetch hints should be https or a package-source provider, not `file://` (E cost defect 5).
  - NAM-006 is unchanged on the wire: the compiled manifest carries version and digest. Keeping the pins in the one project file matches ruling 27's NB (one file of exact pins).
- Cost: about 200-300 lines.

4. Explicit plan/apply (CLI composition of today's operations)
- What changes: `conduit apply` diffs the file against three sources: the IDs this working copy last applied (a tool-written, committable file, conduit-project.applied), the engine's inventory (5c) and its tombstones (5d). It prints the plan, asks y/N, then sends exactly those steps in one MCP session, reports each step, stops at the first part that does not succeed, has no rollback, and can be resumed.
- Classes of change:
  - Grant added: register, then open.
  - Grant removed: close.
  - Grant's operations or target changed: close, register the successor with the same `as`, open. Narrowings run first.
  - A Resource's config, or the version of the package that exports its Type, changed: make-before-break replacement.
    1. Register the successor under a generated name and set it up.
    2. Close every grant to the old Resource and wait until no invocation is in flight.
    3. Register and open the successors under the same `as`.
    4. Retire the old Resource with expectedId and unregister its template.
  - Agent changed: refused without `--replace`; the plan says the session is lost.
  - Object removed from the file: refused without `--prune`.
  - Name held by an object the project does not manage: refused.
  - Anything changed or removed by someone other than this copy's last apply: a conflict that needs `--take` / `--reopen` / `--recreate KEY`.
- It never runs except on an explicit request. There is no reconciler, and drift is shown, never repaired silently.
- Costs it removes:
  - category 8 (change as new documents): T5 60, T6 28, T7 11;
  - T4 goes from 8 requests to 1 command, with template names that stay true;
  - the fresh author's T4 route (32 lines).
- Safety fixes:
  - Every close, open, set-up and retire carries expectedId: the plan-time ID, or the ID register.v1 returned. State is read again after the confirmation. Narrowing drift is a conflict (E #2, A C3/C4).
  - Revocation guard (G3: A C1, B #2, C M3, D #1/#5, E #3): engine-held tombstones keyed by (consumer, local name) for Conduits and (kind, name) for instances, plus the committed applied IDs. A closure or removal made by anything other than this copy's own last apply always needs an explicit flag naming the key. Deleting and re-adding a line, or renaming a key, does not clear it.
  - A stale working copy is a conflict: any live managed object this copy did not create (D #6).
  - Narrowings before widenings, and closes never blocked by unrelated pre-flight failures (D #10/#11, E #12).
  - Re-pointing closes every grant before reopening any. The plan lists every Conduit a retirement will close, including those made outside the file, and refuses without a flag if any would not be re-pointed (E #7).
  - On a version change the plan diffs each granted operation's contract and the implementation's tools map. Any difference needs a flag (E #10). It also prints the resolved config, marking defaulted values (A C2, C M9).
  - Idempotency keys are derived from (object, action, expected ID). Apply refuses to plan while any of its invocations is unknown, and lists in-flight or unknown invocations through grants to a target it will retire (E #11, A C6, D #16).
  - A replacement is triggered only by the package that exports the instance's own Type, so the history Resource is not churned at T5 (D #15 / D cost D2).
  - A set-up that leaves a sign-in pending is reported as pending at the operator; `--yes` never answers an elicitation and never accepts a widening (E #13, rulings 49 and 50).
- Cost: planner and executor about 1,500-2,500 lines plus golden (file, state) -> plan tests.

5. Five small core additions (spec, vectors and core)
- Lines saved: 0. Together they remove the CLI's reliance on reading the engine's private store, which E's skeptics found behind its claim of "no spec change".
- (a) `as`, a standard argument giving a standalone Conduit its local name (OBJ-013, the last sentence of IFC-012, a precedence note in IFC-014).
  - Tool names stay `project.*` and `history.*` through T2 and T4-T6 (fixes 4-11 item 5). Generated object names then never reach Agents.
  - Cost: `local_name` already exists in the core; about 30-60 lines and 3-5 vectors.
- (b) expectedId on open.v1 and close.v1 (the `when` of LIF-037, two input schemas).
  - Closes the confused-deputy swap and the race between plan and apply (A C3, E #2, D #9). About 2-4 vectors.
- (c) An inventory in inspect.v1: the Conduits whose consumer or target is the object, a Pool's entries, each object's registrant, and its annotations returned as given.
  - Promises: LIF-013, one sentence in OBJ-018, inspect.v1's outputSchema, and one disclosure sentence under ACC-033 and ruling 52: list only what the caller may inspect, and count the rest.
  - Gives plan, drift detection and `conduit reach` a route through operations, so preservation test 5, G16 and finding 4-09 are served. Agents can do the same. About 1-3 new vectors and 1-5 snapshot updates.
- (d) Engine-held tombstones (one new promise, e.g. LIF-038).
  - Closing a Conduit, or retiring or unregistering an instance, leaves a mark: kind, name, ID, consumer and local name, caller, invocation, idempotency key, time, and cause (request, or a cascade of invocation X).
  - Marks are kept as engine state apart from records, so REC-004 purges do not remove them, and inspection returns them.
  - register.v1 is unchanged: advisory, enforced by apply. Engine enforcement is offered as an owner option.
  - About 3-5 vectors and about 150-300 core lines. They need a separate state key so that existing expectations, projections over maps, survive; this is to be checked.
- (e) IFC-033: a tool's description adds the target's name and its Type's description.
  - The history convention reaches the Agents without author lines; the baseline spent 6. A re-point in T6 becomes visible.
  - 2-6 snapshot updates (e05-*).
- Total: about 9-12 promises touched and about 2 new (about 3% of 455), 10-17 new vectors, 0 cases invalidated beyond snapshot updates, and about 300-600 core lines.

6. Provider work (counted apart)
- The two Agent Types default accountServices and peerMessaging to false in the Type contract schema, where NAM-024 freezes them, not in template inputs (A C2). This is a new package version; the fixtures are repacked as new versions, not in place. It saves 2 lines in T1.
- The Types' descriptions state reach around the engine correctly:
  - Codex's collaboration tools (4-06), apply_patch and user input;
  - both clients running as the owner's OS user under the subscription login;
  - Codex reading ~/.codex/AGENTS.md (3-07).
  - With these, T3's answer is complete with one command.

7. Engine and CLI defects before the rerun
- Exit codes and per-step reports (4-13), help (4-14), one MCP session per apply and the timeouts (4-15), inspection state (4-16), and the turn deadline with partial attestation (4-12, part of which may need a ruling).
- `conduit init` records the store beside the project file, so no later command needs `--store`.
- Lines saved: 0 for the coordinator. For a fresh author it removes the causes of 130 script lines and 112 recovery lines.
- Cost: medium shell work on findings already logged.

8. Stage 2, optional
- An atomic swap.v1: register the successor first, check IFC-014 at open, close and open at one decision point, with atomicity promised per decision point (A65). It removes the seconds-long windows without access in T4-T6.
- One-step commands (`conduit set`, `grant`, `revoke`, `upgrade`), only if the owner rules that bare commands count as lines.

TOTAL COST
- About 11-14 promises (smaller than A's 20-26, and an order below B's about 60, C's about 80 and D's about 50).
- About 3,500-5,500 CLI source lines and 2,500-3,500 test lines, in a new pure crate plus shell glue.
- About 4-6 work orders.

## projected
Primary reading, the measurement plan's own: non-blank lines in the author's files, plus each command line that carries a value the author chose. Tool-written lines are not counted. The rewrite files are in <scratch>/synth/ (t1.yaml, t2.yaml, t4.yaml to t7-restored.yaml) and were counted by <scratch>/synth/count.py.

| Task | Baseline A70 (better baseline) | Coordinator today | Fresh author today | Package: file lines | Package: command lines with values | Package total | Rewrite used (see rewrite field) | vs A70 |
|---|---|---|---|---|---|---|---|---|
| T1 | 33 | 154 | 263 | 15 | 1 (install, 3 archives) | 16 | One project file; derived Pool, operator, turn and grant Conduits; Type defaults | Better (-17) |
| T2 | 16 | 73 | 67 | 5 | 1 (install history) | 6 | 3 Resource lines + 1 grant line per Agent | Better (-10) |
| T3 | 0 | 0 | 0 | 0 | 0 | 0 | `conduit reach`, 1 command; complete only with the provider statements (item 6) | Same |
| T4 | 8 (better about 8) | 2, 8 requests, about 30 s without access per Agent | 32 | 2 | 0 | 2 | 2 ops lists edited; apply narrows B first; 1 command, 6 engine requests, seconds-long windows, never both writing | Better (-6) |
| T5 | 3 (better 2) | 62 | 112 | 0 | 1 (install 0.2.0) | 1 | Pin rewritten by the tool; apply replaces `shared` make-before-break; contract diff shows no change; no Agent gains append/copy | Better (-2, -1 vs better) |
| T6 | 2 | 29, about 80 s without access | 44 | 1 | 0 | 1 | `root` edited; apply replaces and re-points under the same `project.*` names | Better (-1) |
| T7 | 2 | 12 | 128 | 1 (removed, then restored: 2 edits) | 0 | 1 | Delete A's grant line + apply; check B; restore + apply (no flag: this copy's own apply closed it) | Better (-1); Same if the restore is counted as a second line |
| All | 64 | 332 (35 requests) | 646 + 130 script lines | 24 | 3 | 27 | 12 author commands incl. init; about 51 engine requests sent by the tool; 7 confirmations | Better on 6, same on T3, much more on none |

Where the coordinator's 332 lines go under the package:
- Item 1 (project file): -70 / -41 in T1 / T2.
- Item 2 (derived defaults): -56 / -19.
- Item 3 (pins): -11 / -8 in T1 / T2, and T5's 2 pin lines.
- Item 4 (apply): -60 in T5, -28 in T6, -11 in T7.
- Item 6 (Type defaults): -2 in T1.

Sensitivity, every case counted from the same rewrite:

| Reading | T1 | T2 | T3 | T4 | T5 | T6 | T7 | Total | Bar |
|---|---|---|---|---|---|---|---|---|---|
| No provider defaults (reach flags written per Agent) | 18 | 6 | 0 | 2 | 1 | 1 | 1 | 29 | Pass: better on 6 |
| Block style for every grant (3 lines each) | 20 | 10 | 0 | 2 | 1 | 1 | 3 | 37 | Pass: better on 5; T7 one line more |
| Restore in T7 counted as a second line | 16 | 6 | 0 | 2 | 1 | 1 | 2 | 28 | Pass: better on 5 |
| Strict: every command is a line (apply, reach) | 17 | 7 | 1 | 3 | 2 | 2 | 3 | 35 | Against A70: better on T1, T2, T4, T5 (4 of 7); T3 and T7 one line more. Passes "most" narrowly. Against the better T5 baseline (2), T5 ties: 3 of 7, which FAILS "most" |
| Strict + the stage-2 one-step commands (upgrade, set, revoke/restore) | 17 | 7 | 1 | 2 to 3 | 1 | 1 | 2 | 31 to 32 | Pass: better on 5 |
| JSON, one key per line (fresh author's style) | 40 | 23 | 0 | 4 | 1 | 1 | 7 | 76 | FAILS: more on T1 and T2, much more on T7. YAML must be the documented form |

Other measures (paper):
- Grants by hand: T1 2, T2 2, T7 1. The 3 root Conduits are derived.
- Lines naming a contract address: 4.
- Provider-specific lines: 0.
- Tool names changed by any task: 0.
- Concepts met by the author: about 8 (project, resource, agent, reach grant, ops, install, plan, apply), against about 16 today.
- Duplicated values at the end: about 5 (the ops lists 3 times, `shared` 2, `history` 3), against the baseline's 8.

HONEST VERDICT
- On paper, under the plan's own rule, the package meets the bar on every task.
- It fails the bar under two readings:
  - authoring in JSON with one key per line;
  - counting every command as a line and comparing T5 with the better baseline, unless the stage-2 one-step commands are added.
- The margins on T5, T6 and T7 are 1 to 2 lines.
- Everything here is a paper count, which ruling 45 does not accept as evidence. Only a fresh author's rerun on the built CLI counts.

## rewrite
S1 under the package. The rewrite assumes:
- the store was made once by `conduit init`, which records it beside the project file, so no command needs `--store`;
- the Agent packages default their reach flags to false (item 6);
- the five core additions (item 5) exist.

Lines marked TOOL are written by `conduit install` and are not counted. The files are in <scratch>/synth/.

=== T1: 15 file lines + 1 command line with values = 16 ===
Commands:
```
$ conduit install <lab>/pkgs/filesystem-0.1.0.tgz <lab>/pkgs/agent-claude-code-0.1.2.tgz <lab>/pkgs/agent-codex-0.1.2.tgz   [1 line]
$ conduit apply                                                                                                      [no value]
```
conduit-project.yaml after T1:
```yaml
conduit: v0
project: s1
packages:                                                                  # TOOL
  example.org/s1/filesystem: {version: 0.1.0, digest: sha256:200de7c1...}  # TOOL
  example.org/conduit/agent-claude-code: {version: 0.1.2, digest: ...}     # TOOL
  example.org/conduit/agent-codex: {version: 0.1.2, digest: ...}           # TOOL
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    reach:
      project: {to: shared, ops: [file.read.v1, directory.list.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    reach:
      project: {to: shared, ops: [file.read.v1, directory.list.v1, file.write.v1]}
```
Plan (abridged), operator Interface/root:
```
+ Pool s1                                   rule D1 default Pool
+ Resource shared (owner s1)                resources.shared, line 7
+ Agent agent-a, agent-b                    defaults shown: accountServices=false, peerMessaging=false, model='' (Type contract)
+ Conduit s1--admin  root -> Pool s1        pool-management.v1 [register, inspect, set-up, retire, reconcile]   rule D2
+ Conduit agent-a--turn, agent-b--turn      root -> Agent, conversation.v1 [turn.v1]                            rule D3
+ Conduit agent-a--project  as project      agent-a -> shared  example.org/s1/filesystem#directory.v1 [file.read.v1, directory.list.v1]   line 13
+ Conduit agent-b--project  as project      agent-b -> shared  [file.read.v1, directory.list.v1, file.write.v1]                           line 17
reach after: agent-a project.file.read.v1, project.directory.list.v1; agent-b the same + project.file.write.v1
steps: register (1 submission, 18 documents); open s1--admin; set-up s1; set-up shared; open the two grants; set-up agent-a, agent-b; open the turns
```
- Engine requests: 3 install + 1 register + 5 open + 4 set-up = 13, plus plan reads. Each open and set-up is sent with expectedId set to the ID register returned.
- Checks, not counted: turns through `agent-a--turn.turn.v1`: A reads and is offered no write tool; B writes.

=== T2: 5 file lines + 1 command line = 6 ===
```
$ conduit install <lab>/pkgs/history-0.1.0.tgz      [1 line; TOOL adds example.org/s1/history: {version: 0.1.0, digest: ...}]
```
Added to conduit-project.yaml:
```yaml
  history:                                            # under resources
    type: example.org/s1/history#file-history.v1
    config: {root: <lab>/history}
      history: {to: history, ops: [file.read.v1, directory.list.v1, file.write.v1]}   # under agent-a reach
      history: {to: history, ops: [file.read.v1, directory.list.v1, file.write.v1]}   # under agent-b reach
```
```
$ conduit apply
```
- Plan: + Resource history (registered into the existing Pool s1: ACC-033 row 1, through management and s1--admin, both listing register.v1); + 2 grants `as history`.
- Engine requests: 1 register, 1 set-up, 2 open.
- With 5e, each history tool's description carries file-history.v1's posting convention, so no author line conveys it. The baseline spent 6 lines on it.

=== T3: 0 lines ===
```
$ conduit reach
```
- It prints, per Agent:
  - tools, Conduit name and ID, target, operations and source line, read through inspect.v1's inventory (5c);
  - reach around the engine, from the Type statements (item 6): Claude Code with no shell, built-ins limited as stated, account services and peer messaging off, running as the owner's OS user under the subscription login; Codex with apply_patch, user-input requests, the collaboration tools of 4-06, ~/.codex/AGENTS.md, the same user and login.
- Inspection effort: 1 command. Without item 6 it is the 2 READMEs and the closeout, as today.

=== T4: 2 lines ===
Edits:
```
line 13  project: {to: shared, ops: [file.read.v1, directory.list.v1]}                 -> [..., file.write.v1]
line 17  project: {to: shared, ops: [file.read.v1, directory.list.v1, file.write.v1]}  -> [file.read.v1, directory.list.v1]
```
```
$ conduit apply
```
Plan, narrowing first:
```
~ agent-b project: close agent-b--project (expectedId); register agent-b--project--2 as project [read, list]; open (expectedId)
~ agent-a project: close agent-a--project; register agent-a--project--2 as project [read, list, write]; open
tools: agent-b loses project.file.write.v1; agent-a gains it
```
- Engine requests: 6, from 1 author command (the coordinator needed 8).
- Each Agent is without project access only between its close and its open: seconds, not measured. There is never an interval in which both Agents can write.
- Template names never misdescribe what they grant: each Conduit has its own fresh template.

=== T5: 0 file lines + 1 command line = 1 ===
```
$ conduit install <lab>/pkgs/filesystem-0.2.0.tgz   [1 line; TOOL rewrites the pin to 0.2.0 and its digest]
$ conduit apply
```
Plan:
```
~ Resource shared: example.org/s1/filesystem 0.1.0 -> 0.2.0 (the package that exports its Type) => replace, make-before-break
    implementation: server 2.1.0 -> 2.2.0; tools map unchanged (read_text_file, list_directory, write_file);
    contracts of the granted operations unchanged; placement: inherited node; in-flight or unknown invocations through grants to shared: none
  1 register Resource shared--2 (+ its template); set-up (row 4 via s1--admin)
  2 close agent-b--project--2 and agent-a--project--2 (expectedId); wait until none is in flight
  3 register successors as project -> shared--2; open both (expectedId)
  4 retire shared (expectedId); unregister its template (revision)
= history and its grants: the history package's own pin keeps filesystem 0.1.0 (NAM-014); a grant's capability pin is not compared (NAM-017, NAM-023); no change
no Agent gains a tool: append_file and copy_file are not mapped by 0.2.0
```
- Engine requests: 1 install + 9.
- Agents are not restarted: their one-off templates name only the Agent package, and their access is standalone Conduits.

=== T6: 1 line ===
```
line 9  config: {root: <lab>/shared}  ->  config: {root: <lab>/shared-new}
```
```
$ conduit apply
```
- Plan: the same replacement as T5. shared--2 is replaced by shared--3 with the new root, and the grants are re-pointed under the unchanged `project.*` tool names.
- With 5e, the tool descriptions now name shared--3, so IFC-038 tells each Agent the list changed.
- Engine requests: about 10.

=== T7: 1 line (2 edits) ===
Revoke: delete agent-a's `project:` line.
```
$ conduit apply
```
- Plan: - close agent-a--project--3 (expectedId). The engine leaves a tombstone naming this apply's invocation.
- Check B, not counted: B reads the project directory and posts to history; B's Conduits are untouched.

Restore: put the line back.
```
$ conduit apply
```
- Plan: + grant agent-a project. The plan notes: "last closed by this working copy's apply <id> at <time>; no --reopen needed". It registers agent-a--project--4 `as project` and opens it.
- Engine requests: 1, then 2.
- Alternative operator route: `conduit close agent-a/project`, later `conduit apply --reopen agent-a/project`. That is 2 command lines, the same as the baseline. It leaves a tombstone that any later apply honours, from any working copy.

Final authored file: 20 lines.

## decisions
1. The shape of the simplification.
- Options:
  - (a) a layered authoring surface: a project file compiled by the CLI to today's documents, plus explicit plan/apply and five small core additions;
  - (b) a normative short form and an engine-side apply now (designs C and D);
  - (c) the change model only, with revisioned instances and restart in place (design B);
  - (d) a conservative sectioned file with OBJ-002 equal resubmission (design A).
- Recommendation: (a).
  - Cheapest: about 11-14 promises, against about 50-80 for (b) and (c).
  - It keeps every object immutable and every guarantee in the engine.
  - It is the only route whose cost verdict was minor.
  - (c) still fails T1. (b) and (d) had major safety findings their own text did not close.

2. Does a tooling-level surface count for the bar and for theses 1 and 2?
- Options:
  - (a) yes for v0: the project file is what the author writes, and the canonical documents stay the language that fully determines meaning;
  - (b) no: the short form must be normative (AUT-* promises) before the rerun.
- Recommendation: (a) now, and decide on promotion after the rerun. Promotion is needed only if a second CLI must agree (A34), and costs about 25-30 promises.

3. The five core additions. Each is approved separately.
- (a) `as` local name on standalone Conduits.
  - Recommendation: yes.
  - It reverses only the coordinator's choice in log-followups-ab point 2, whose stated reason (rename) ruling 41 removed.
- (b) expectedId on open.v1 and close.v1.
  - Recommendation: yes.
  - This extends ruling 44's D12 list.
- (c) An inventory in inspect.v1, plus annotations returned.
  - Recommendation: yes.
  - It reverses work order 027's "CLI output only" choice for 4-09.
  - Disclosure: list only what the caller may inspect, and count the rest.
- (d) Engine-held tombstones.
  - Options: (i) advisory, shown by inspection and enforced by apply [recommended]; (ii) also enforced by register.v1 through a `supersedes` input; (iii) none, so G3 rests on local files [not acceptable with several actors].
- (e) IFC-033: the tool description names the target and carries its Type's description.
  - Recommendation: yes.
  - Check that naming the target discloses nothing the consumer may not learn (ruling 52).

4. Explicit plan/apply as a client composition.
- What to confirm: it is consistent with DD2/ruling 22 (register.v1 still only registers) and with round 6's "do not add automatic apply". The plan is D08's "pure preview of the required explicit actions"; apply runs only on a confirmed request, never in the background, and never prunes or reopens without a flag.
- Recommendation: confirm.

5. Where the pins live.
- Options:
  - (a) the project file's `packages:`, written by `conduit install` (version and digest; the cargo add pattern) [recommended: one file, matching ruling 27's NB];
  - (b) a separate tool-written lock (go.sum);
  - (c) the author writes versions and the tool writes digests (+4 lines in T1, +1 in T2; T5 becomes 1 file line + 1 command line).
- Also decide: trust-on-first-use digests by default, with `ARCHIVE#sha256` accepted. Recommendation: yes.

6. The defaults the CLI derives.
- The items: a default Pool per project (with optional `owner:`); an operator Conduit to the Pool (register, inspect, set-up, retire, reconcile); a turn Conduit per Agent, on by default (`turns: none`).
- Rules: each is a real, recorded Conduit, shown with its rule; the operator is the authenticated Interface; Conduits whose consumer is root are add-only.
- Recommendation: accept.
- Rejected alternatives: an ACC-033 exception for root; a default Pool at first start (FST changes).

7. Agent reach flags (ruling 48).
- Options:
  - (a) default false in the Type contract schemas, as a new package version [recommended: frozen by NAM-024; ruling 48 says off by default];
  - (b) keep them required (+2 lines in T1).
- Never as template-input defaults, since a later package version could flip them unseen.

8. Writing operations and capabilities in the file.
- Options:
  - (a) export names within the grant's capability (`file.read.v1`) and an inferred capability; ambiguity is refused, and everything compiles to full addresses [recommended];
  - (b) full addresses and an explicit `capability:`.
- 0 lines either way (only line width). Ruling 27 is unaffected for documents.

9. The route for T5 and T6.
- Options:
  - (a) make-before-break replacement under generated successor names, with grants re-pointed by close, register and open [recommended: no promise changes];
  - (b) retire first and reuse the name: a longer window, and stuck while the old shutdown is pending or unknown (A C5);
  - (c) restart in place (A67): defer. It breaks PLC-009, SEC-014 and others, and needs D09's semantics.

10. Atomic swap.
- Options:
  - (a) defer to stage 2 and measure the windows first [recommended];
  - (b) build swap.v1 now: register the successor first, check IFC-014 at open, close and open at one decision point, atomic per decision point.

11. Counting rules for the rerun, stated before it runs (ruling 45).
- Recommendations:
  - The primary reading is the plan's own: file lines + command lines with chosen values.
  - A line removed and then restored in one task counts once, by the baseline's own rule.
  - Report the strict and the JSON readings beside it.
  - Package Type statements (item 6) and the history convention count as provider work.
- Also decide: whether to add the one-step commands (`conduit set`, `grant`, `revoke`, `upgrade`). Only needed if the strict reading is ruled primary.

12. Engine and CLI defects before the rerun.
- The defects: 4-12 to 4-16, and a default store recorded at `conduit init`.
- Recommendation: fix them all.
- 4-12's partial attestation through reconcile.v1 may need its own ruling.

13. Later, after the rerun (not now): whether to simplify the core along the lines the layer exposes. The candidates:
- inline one-off definitions (amending IN9 and ruling 18's V2);
- the frame stated once per file (rulings 31, 35, 37);
- LIF-009 building from the held definition;
- whether declared Conduits, replace.v1 and register-replacement.v1 earn their place in v1.

## rejected
B (apply in the engine, revisioned instances, update and restart in place): rejected as a whole.
- Cost: large. About 45 promises edited and 13-17 new, the core grows 40-65%, and stores need a clean break.
- It fails the bar on its own: T1 is 60 lines against 33.
- Safety, as the skeptics found:
  - a later apply silently reverts changes made outside apply (swap.v1 or controls);
  - an unrelated apply re-creates retired Agents;
  - the plan digest does not cover the documents (SUB-008 hashes contracts only);
  - APL-006 contradicts ACC-024;
  - a restart in place keeps an old run's credential valid (G12);
  - plan.v1 leaks Resource config past row 6;
  - its T5 contradicts APL-003(c), since the Agents restart;
  - its lock reverses ruling 27's NB.
- Kept: only the observation that LIF-009's "the template as it now stands" protects nothing. It goes to decision 13.

C (Agent-centric project file made normative, with revise.v1 and swap.v1 in the engine): the normative form now, revise.v1 and restart in place are rejected.
- Cost: about 80 promises (18% of the spec), with understated work. It reverses PLC-009, PRV-006 and SEC-014 without listing them.
- Safety, as the skeptics found:
  - M1: Conduits that point at an instance are decided again only after its new code runs, with no rollback; this breaks the secret-store and filter cases;
  - M2: swapping a root Conduit is refused by ACC-024;
  - M3: "closed stays closed" has no stored state;
  - M5: revise gives power over other teams' Agents;
  - M7: identity survives a change of code;
  - M8: the grants made depend on who runs apply;
  - D1: provenance is forged by the client.
- Kept, as CLI-derived and safer: the default Pool, derived Pool and turn Conduits (with the operator made explicit), `as`, provenance as annotations checked against registrant and applied IDs, and the reach view.

D (relationship-first, apply.v1 in the engine, frozen package groups): apply.v1 in the engine and package-exported operation groups are rejected.
- Cost: large, about 50 promises and 4,500-7,500 lines.
- Safety, as the skeptics found:
  - the group freeze is too narrow: requirements, inputs, Resource and Agent templates, and declared or tool-less get-secret Conduits all widen silently;
  - the applied record dies with its Pool;
  - the plan run is not bound to the plan approved;
  - a dry run works as an inspection oracle;
  - one implicit Pool per project widens access through bounds.
- Kept:
  - one line per relationship;
  - `as`;
  - narrowing before widening;
  - the revocation guard keyed per relationship (consumer, local name), not per object;
  - a replacement compared only against the package that exports the instance's own Type (D cost D2).
- Groups are not needed: explicit ops lists cost 0 extra lines in flow style and keep the grant visible in the author's text.

A (conservative): its OBJ-002 equal-document rule, `ownerConduits` on pool.v1, name reuse after retirement and template-input defaults are rejected.
- The equal-document rule turns bulk resubmission into a way to re-create retired objects, and leaks stored fields to callers holding only register.v1 (C1, C8).
- `ownerConduits` on pool.v1 invalidates 124 vectors.
- Reusing a name after retirement cuts access before the name is free (LIF-035, LIF-007) and is stuck on an unknown shutdown (C5).
- Template-input defaults can be flipped by a package (C2).
- Its normative sectioned file is also rejected: it gives the same lines as tooling, at a spec cost.
- Kept: `install --save`, `render`, apply as a client composition with `--reopen` and `--replace`, and the conservative stance on immutability.

E (layered), as written: four parts are rejected and replaced.
- Hash-named templates shared between grants become one fresh template per instance (#4).
- `<key>-<agent>` tool names become `as` (from A, C and D).
- Local-only applied state becomes engine tombstones plus committed applied IDs (#3).
- The claim of "no spec change" is replaced by the five core additions: the CLI otherwise reads the private store (#1). The skeptics' other fixes are adopted (items 1-4).

Also rejected:
- Continuous reconciliation, automatic apply, prune by default, version ranges, aliases (round 6, D17, preservation test 6).
- An ACC-033 exception for root over Pools (G2, ruling 51).
- A default Pool created at first start (FST churn for nothing the CLI cannot do).
- A spec-level lock file (ruling 27, NB).
- JSON-first authoring: the JSON reading fails the bar.
- Upgrade or restart in place for v0 (D09 semantics unwritten, PLC-009, SEC-014, G12).

## risks_and_tests
RISKS
1. The counts are paper counts.
   - Ruling 45 accepts only a run. The margins on T5-T7 are 1 to 2 lines.
   - The fresh author wrote about 1.7 times the coordinator's lines in T1. At 2 times, T1 is about 32 against 33.
2. Authoring format.
   - JSON with one key per line fails: T1 40, T2 23, T7 7.
   - The fresh author chose JSON only because PyYAML was missing for generators. A short hand-written file removes that reason, but the docs and examples must be YAML.
3. Command counting.
   - If every command counts as a line and T5 is compared with the better baseline, the package is better on only 3 of 7 tasks.
   - The remedy is the stage-2 one-step commands, or a ruling, made before the run, that the plan's rule (chosen values only) is primary.
4. Thesis question.
   - The owner may rule that a tooling surface does not satisfy "simplify the documents".
   - Promotion then costs about 25-30 AUT-* promises, a conformance claim for a CLI and about 50 plan vectors.
5. The core's cost moves; it does not disappear.
   - Implementers still read 455 promises.
   - The system grows by a CLI of about 4-5k source lines.
   - Each future core change (rulings 49-51) costs about 1.3-1.5 times as much, because about 27 promises are mirrored in the CLI.
   - The machinery the layer never uses is dead weight until decision 13.
6. More engine requests.
   - About 51 sent by the tool, against 35 typed.
   - Partial applies, unknown outcomes and no rollback mean the author meets core concepts on failure. Per-step reports and resume must be good.
7. G9 is weakened.
   - An Agent's access is now standalone Conduits, opened before its set-up only by the CLI's ordering.
   - An Agent set up outside apply starts without access. That fails closed, but is visible only in inspection.
8. Records gain churn.
   - New IDs and generated names (shared--2, agent-a--project--3) at every change.
   - Readers need annotations, or `conduit reach`, to map them back.
9. Windows without access remain in T4-T6 (seconds) until swap.v1.
   - T5 now has a window where the coordinator's route had none, because the coordinator renamed the tools.
10. Tombstones grow without bound in v0. Retention is a DL item.
11. Provider statements for T3 must be true. The Codex description is false today (4-06).

TESTS
A. The compiled form must be accepted by the unchanged spec.
- Build the compiler first as a Rust spike.
- Render T1-T7 from the files in <scratch>/synth/.
- Load them with today's loader, and run them through the core (the conformance runner on a fresh state).
- Check: rendering, then submitting to a fresh store, gives the same objects up to IDs.

B. Golden (file, applied IDs, inventory, tombstones) -> plan vectors, one per class C1-C8, plus these safety cases:
1. A grant closed by close.v1 outside apply; the same line is edited; apply: conflict, nothing opened.
2. A grant removed and re-added under a renamed key: conflict.
3. Agent-b retired by a responder; an unrelated apply: conflict, nothing re-created.
4. Records purged by root; apply: the tombstone still blocks.
5. A stale working copy re-widens a grant that another copy narrowed: conflict.
6. An actor registers a decoy between apply's register and open: open with expectedId reports conflict.
7. A plan made stale by a narrowing between y/N and dispatch: conflict.
8. An apply killed after its first part in T4: no state in which both Agents write.
9. T5 with a package whose granted operation gains an optional input, or whose tools map changes: refused without a flag.
10. Replacement with a Conduit from outside the file targeting the old Resource: refused without a flag.
11. A write with an unknown outcome through a grant to `shared` before T5: the plan refuses.
12. Placement inherited on replacement on a store with two nodes.
13. A change to a root-consumed derived Conduit: refused before dispatch, with the reason.
14. A key containing `--`, or colliding with a generated name: rejected at compile time.
15. A value at a place marked secret-reference: redacted in every diagnostic.
16. Inventory to a caller with only a Pool's authority: no Agent's Conduits, counts only.
17. A set-up whose sign-in is pending under `--yes`: stops as pending, nothing answered.
18. A downgrade of a pin: needs a flag.

C. Spec vectors for the five core additions: about 10-17 new cases.
- In particular: `as` collisions under IFC-014 and its precedence over a closed declared Conduit; the expectedId mismatch on open and close as conflict after the access decision (LIF-037's order); tombstone kept across purge; inventory disclosure tiers.
- Plus the 21 safety rows and the increment-2 section 3 traces, all passing.
- Fix the manifest's tagging of the 74 cases with no safety_rows key while there.

D. The rerun, under ruling 45 and ruling 52 point 6.
- Write the experiment plan in advance: the counting readings and the bar under each, YAML documented, the same measurement plan, the same tasks, checks as in increment 3's closeout.
- The coordinator does a dry run first. Then a fresh Codex author runs with only spec/, the format reference and what the engine ships.
- Report it beside A70, the coordinator's run and the earlier fresh-author run, with requests, windows without access (from records), concepts met, mistakes and adapters. Adapters must be 0, or the 4-13 to 4-15 fixes did not hold.

E. A cold read of the format reference by a third reader, for G6 ambiguity (the ROADMAP row "Read").

## sequencing
BEFORE THE OWNER DECIDES. Each of these is reversible and changes no behaviour.
1. Fix the shell defects that are pure tooling.
   - Exit codes and per-step reports (4-13), help (4-14), one reusable MCP session and the timeouts (4-15), and inspection state (4-16).
   - `conduit init` records the store, so later commands need no `--store`.
   - These matter for any package. They remove what drove the fresh author's 130 script lines.
   - 4-12 (turn deadline, partial attestation) is prepared as options, not applied.
2. Write the format reference v0 as a proposal under docs/proposals/, not binding. It covers about 30-40 rules:
   - the keys, and the defaults D1-D3 with their rule names;
   - the generated names (reserved `--`), and the change classes C1-C8;
   - the conflict and flag rules, and the source map with redaction.
   - Draft beside it the promise text for the five core additions, with options, as a proposal record. Nothing is applied to spec/.
3. A Rust spike under experiments/ (a pure crate) that compiles and renders T1-T7 from the scratch files. Run test A on it: today's loader and core accept the rendered documents. This turns the T1/T2 render from paper into a check at low cost.
4. Draft the rerun's experiment plan (ruling 45), with the counting readings left as decision 11's options.
5. Correct the Codex Type's statement for 4-06 in the provider proposal (statement accuracy only). Changing the default flags waits for decision 7.

AFTER DECISIONS 1-4 (the shape, the core additions, apply), as work orders.
- WO-1 (spec batch): the five core additions with vectors and the core code. About 9-12 promises touched and 2 new; 10-17 cases; snapshot updates.
  - Run every check in AGENTS.md, and cargo test with clippy.
  - It can run in parallel with WO-2, because the compiler targets today's documents.
- WO-2 (compiler): move the spike into reference-model as a pure crate (project file, compiler, render, source map, golden tests).
- WO-3 (planner and executor): plan, apply, the conflict, tombstone and expectedId logic, drain and re-point, contract diffs, per-step reports, resume, and the safety vectors of test B.
  - It depends on WO-1's inventory, tombstones and expectedId.
- WO-4 (install and reach): install writing the pins (decision 5) and `conduit reach`. Small. It can follow WO-2.
- WO-5 (providers, decision 7): the Agent Type defaults and descriptions, published as new package versions. Not repacked in place, which avoids the cascade through 91 files that pin the filesystem digest.

THEN
1. The coordinator does a dry run of S1 under the package.
2. The fresh Codex author's rerun, under the plan stated in advance (decision 11).
3. Report the results and update the ruling log and to-review-with-the-owner.md.

CONDITIONAL, after the results
- swap.v1 (decision 10), if the measured windows matter.
- The one-step commands, if the strict reading is primary.
- Promotion of the format to spec/authoring/ (decision 2), if A34 needs a second CLI to agree.
- The core simplifications the layer exposes (decision 13), each as its own ruling. These are the v1 candidates.

CRITICAL PATH: WO-1 and WO-3 (planner safety), then the rerun. Everything in the first group (before the owner decides) can start immediately.