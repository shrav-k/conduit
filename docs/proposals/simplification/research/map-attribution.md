Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Where every authored line went in increment 4's two Conduit runs of S1 (coordinator: 332 YAML lines; fresh author: 646 JSON lines plus 130 lines of scripts)

## 0. Method

- **Scope.** I read every file under `docs/proposals/first-build/increment-4/coordinator-run/` and `.../fresh-author-run/`, both logs and both reports. The repo was not modified.
- **Classifier.** Lines were tagged by a script so the totals are exact. It is `<scratch>/attribute.py`. Run it as `python3 -I attribute.py <home> [--dump]`; `--dump` prints every line with its category.
- **Totals match both reports.** Coordinator: 154/73/0/2/62/29/12 = 332. Fresh author: 263/67/0/32/112/44/128 = 646. The coordinator's reported 26 separators and 8 copied digests are also reproduced exactly.
- **Counting rule.** Lines are counted as the plan counts them: non-blank, `---` included, no comments in any file.

### Precedence rule (each line counted once)

1. **Cause categories come first.** They absorb every line of the documents they cover, frames and braces included:
   - (4) Documents for the root Interface: its Pool-management Conduits and its turn Conduits, plus the 2-line manifests written only to submit them.
   - (8) Lines written in T4–T7 only because an existing object cannot be changed. The few lines that state the task's own change stay in their own category ("essential delta" below).
   - (12) **New category.** Failure recovery: the Agents and their Conduits re-declared after T6's turns ended `unknown`.
2. **Every other line is classified by its kind:**
   - (1) Frame: `apiVersion`, `kind`, `metadata`, `name`, `---` and the bare `spec:` wrapper.
   - (2) Manifest lines and digests.
   - (3) Template plumbing: `inputs` / `schema` / `to`, and every `from:` that points at a template.
   - (5) Access facts: `operations`, `consumer`, `target`, and an Agent's `conduits:` / `<name>:`.
   - (6) Resource `type` and arguments (paths).
   - (7) Agent `type` and `config`.
   - (9) JSON lines that hold only `{ } [ ] ,`.
   - (10) Other: `capability:` lines and an empty `config: {}`.
   - (11) **New category.** Pool machinery: the Pool's `type` and `owner:` lines.
3. **Essential delta per task (my judgment):**
   - T4: the 2 lines that move the write operation. For the coordinator these are the moved `file.write.v1` line; for the fresh author, the two changed `from:` lines. Both counted as (5).
   - T5: `version: 0.2.0` plus its digest, as (2).
   - T6: the new root path, as (6).
   - T7: 0 lines. Revoking and restoring are a close and an open; the restore lines exist only because closing removed the Conduit.

## 1. Which files belong to which task (from the logs)

**Coordinator**

- **T1 (154).**
  - Files: `project/conduit.yaml` (11), `project/templates.yaml` (91), `project/instances.yaml` (50) and `project-t1fix/conduit.yaml` (2).
  - `project-t1fix/admin.yaml` (19) holds `pool-admin` and `root-workspace`. After the store was discarded (log 11:10:07) both documents were folded into `project/`, so they are counted once, there.
  - This reconstruction reproduces the report's 154 lines and 14 separators (8 + 6). Counting admin.yaml separately would give 13 separators.
- **T2 (73).** `project-t2/conduit.yaml` (8), `project-t2/history.yaml` (47), `project-t2fix/admin2.yaml` (16) and `project-t2fix/conduit.yaml` (2).
- **T4 (2).** In `project/templates.yaml`, `- example.org/s1/filesystem#file.write.v1` moved from template `read-write` to `read` (now line 29).
  - Applied with `replace`, plus close, `register-replacement` and open for each Agent (log 11:18–11:19).
  - Afterwards the template named `read` grants write.
- **T5 (62).** `project-t5/conduit.yaml` (5) and `project-t5/upgrade.yaml` (57).
- **T6 (29).** `project-t6/conduit.yaml` (5) and `project-t6/replace.yaml` (24).
- **T7 (12).** `project-t7/conduit.yaml` (5) and `project-t7/restore.yaml` (7).
- **Not counted:** `t3-answer.md` (T3 = 0) and `log.txt`.

**Fresh author**

- **T1 (263 lines in 19 files).** `t1/*.json` (17 files, 231 lines) and `t1-root-route/{template,instance}.json` (32).
- **T2 (67 in 5).** `t2/*.json`.
- **T4 (32 in 2).** `t4/{a,b}-directory.json`.
- **T5 (112 in 8).**
  - `t5-template/{conduit,directory-template}.json` (30), used for `conduit replace Resource/directory-template`.
  - `t5/*.json` (82): the new manifest, the `directory-read-v2` and `directory-write-v2` templates, `directory`, and `a-` / `b-directory`.
- **T6 (44 in 3).** `t6/*.json`.
- **T7 (128 in 9).** `t7-recovery/*.json` (8 files, 112 lines) and `t7-restore/a-directory.json` (16).
- **Scripts (adapters, 130 lines), assigned by when the log shows each was created:**
  - T1: `run.py` (22, log line 7), `build_t1.py` (26, line 32), `turn.py` (6, line 57), `stages.py` (23, line 67). Total 77.
  - T6: `checked.py` (16, line 210).
  - T7: `final_check.py` (37, line 262).
- **Not counted:** `t3-answer.md` (58), `t6-recovery-notes.md` (14), `README.txt` and `report.json`.

## 2. Coordinator: tasks × categories (332 lines)

| Category | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All | Share |
|---|---|---|---|---|---|---|---|---|---|
| 1 Frame (incl. `---`, `spec:`) | 54 | 26 | 0 | 0 | 0 | 0 | 0 | 80 | 24.1% |
| 2 Manifest and digests | 11 | 8 | 0 | 0 | 2 | 0 | 0 | 21 | 6.3% |
| 3 Template plumbing | 10 | 7 | 0 | 0 | 0 | 0 | 0 | 17 | 5.1% |
| 4 Root's Pool-management and turn Conduits | 54 | 18 | 0 | 0 | 0 | 0 | 0 | 72 | 21.7% |
| 5 Access facts | 13 | 8 | 0 | 2 | 0 | 0 | 0 | 23 | 6.9% |
| 6 Resource type and arguments | 3 | 3 | 0 | 0 | 0 | 1 | 0 | 7 | 2.1% |
| 7 Agent type and config | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 1.2% |
| 8 Re-declared because objects cannot change | 0 | 0 | 0 | 0 | 60 | 28 | 12 | 100 | 30.1% |
| 9 JSON punctuation | n/a | | | | | | | 0 | |
| 10 Other (`capability:`, `config: {}`) | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 5 | 1.5% |
| 11 Pool `type` and `owner:` | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 0.9% |
| **All** | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 | |

**Category 4, 72 lines.**
- By purpose: 32 are turn Conduits (`claude-turn`, `codex-turn`, `turn-a`, `turn-b`, 8 lines each). 40 are root-to-Pool management (`pool-admin` 12, `root-workspace` 8, t1fix manifest 2, `admin2.yaml` 16, t2fix manifest 2).
- By kind: frame 32, separators 7, `spec` 4, manifest 4, `from` 4, `capability` 4, `operations` 9, consumer/target 8.
- The report's figure of 35 lines is admin.yaml plus admin2.yaml. Mine is 40 because it adds the two 2-line manifests and the extra separator from folding into the project.

**Category 8, 100 lines.**
- By kind: frame 40, separators 7, `spec` 3, manifest 11, digests 2, template inputs 4, `from` 7, `capability` 2, `config: {}` 1, `owner` 2, `operations` 7, consumer/target 10, resource type 1, arguments 3.
- By task:
  - **T5 (60).** A new Resource template `directory-2`; it is `directory` with a new name, since a document cannot replace a template (SUB-007, OBJ-002) and an existing Resource keeps its package version (PKG-005, INS-020). Also a new instance `shared-2` with the same path; Conduit templates `read-2` and `read-write-2`, copies of T1's; and standalone `project-a` / `project-b`, because a declared Conduit cannot be re-pointed (OBJ-011), which gives new tool names.
  - **T6 (28).** The manifest, `shared-new` apart from its root line, and `project-a` / `project-b` submitted again after close and unregister (mistake 3).
  - **T7 (12).** The manifest and `project-a` again, since closing had removed it.
- **Copies.** Of the 103 lines written in T5–T7:
  - 17 are identical to earlier documents: the T6 and T7 manifests, and T7's `project-a`.
  - 44 differ in one line, usually only the new name: `directory-2`, `read-2`, `read-write-2`, and T6's `project-a` / `project-b`.
  - 30 differ in 2 or 3 lines.
  - The rest are the T5 manifest (5) and 7 separators.

## 3. Fresh author: tasks × categories (646 JSON lines)

| Category | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All | Share |
|---|---|---|---|---|---|---|---|---|---|
| 1 Frame (incl. `spec`) | 54 | 17 | 0 | 0 | 0 | 0 | 0 | 71 | 11.0% |
| 2 Manifest and digests | 11 | 8 | 0 | 0 | 2 | 0 | 0 | 21 | 3.3% |
| 3 Template plumbing | 13 | 3 | 0 | 0 | 0 | 0 | 0 | 16 | 2.5% |
| 4 Root's Pool-management and turn Conduits | 90 | 0 | 0 | 0 | 0 | 0 | 0 | 90 | 13.9% |
| 5 Access facts | 19 | 12 | 0 | 2 | 0 | 0 | 0 | 33 | 5.1% |
| 6 Resource type and arguments | 3 | 3 | 0 | 0 | 0 | 1 | 0 | 7 | 1.1% |
| 7 Agent type and config | 8 | 0 | 0 | 0 | 0 | 0 | 0 | 8 | 1.2% |
| 8 Re-declared because objects cannot change | 0 | 0 | 0 | 30 | 110 | 43 | 16 | 199 | 30.8% |
| 9 JSON punctuation-only | 61 | 23 | 0 | 0 | 0 | 0 | 0 | 84 | 13.0% |
| 10 Other (`capability`) | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 0.3% |
| 11 Pool `type` and `owner` | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 3 | 0.5% |
| 12 Recovery: Agents re-declared after T6 | 0 | 0 | 0 | 0 | 0 | 0 | 112 | 112 | 17.3% |
| **All** | 263 | 67 | 0 | 32 | 112 | 44 | 128 | 646 | |
| Adapters (Python, apart) | 77 | 0 | 0 | 0 | 0 | 16 | 37 | 130 | |

**Category 4, 90 lines.** Turn Conduits are 58 (`a-turn-template` 13, `a-turn` 16, `b-turn-template` 13, `b-turn` 16). The root-to-Pool route `pool-admin` and `root-team` is 32. By kind: frame 24, `spec` 3, punctuation 30, `from` 3, `capability` 3, `operations` 9, consumer/target 18.
- T2 needed none. After its T1 mistake, this author put `register.v1` into `pool-admin` from the start (`t1-root-route/template.json:13`).

**Category 8, 199 lines.** By kind: frame 48, `spec` 3, punctuation 69, manifest 7, digest 1, template inputs 7, `from` 7, `capability` 2, `owner` 2, `operations` 7, consumer/target 42, resource type 1, arguments 3.

**Category 12, 112 lines.** By kind: frame 32, punctuation 36, `from` 8, consumer/target 36. All 8 files are byte-identical to files written in T1, T2 or T5.

**Copies in T4–T7.**
- Of 316 lines, 202 (64%) are byte-identical to an earlier file. Examples:
  - `t5-template/directory-template.json` = `t1/directory-template.json`
  - `t5/directory.json` = `t1/directory.json`
  - `t6/{a,b}-directory.json` = `t5/...`
  - all of `t7-recovery/*`
  - `t7-restore/a-directory.json` = `t5/a-directory.json`
- Another 105 differ from an earlier file in a single line:
  - T4's `from:` lines
  - the `-v2` template names
  - T5's `a-` / `b-directory` `from:` lines
  - T6's `root`
- Only the 9-line T5 manifest is new. So 97% of the T4–T7 lines are copies or one-line edits.

**Adapters, 130 lines** (`.py.txt` files):

| Script | Task | Lines | What it does | What it compensates for |
|---|---|---|---|---|
| `build_t1.py` | T1 | 26 | Writes all 17 T1 documents (231 JSON lines). `doc()` (line 6) is a frame factory; loops write a template and an instance per Agent and per turn Conduit (lines 17–21) | Frame (1), template/instance (3), root turn Conduits (4); JSON (9), since PyYAML was missing (log 6) |
| `stages.py` | T1 | 23 | 8 lines generate documents: `doc`, `save`, `route` (a standalone-Conduit factory, lines 12–13), and pins `FS1`/`FS2`. 15 lines wrap open, close, submit, inspect and tools | Re-declaration (8), manifests (2) |
| `run.py` | T1 | 22 | Wraps every `conduit` command, logs it, saves stdout and stderr as evidence, refuses to overwrite | Command ergonomics (4-14); the run's evidence rules |
| `turn.py` | T1 | 6 | Adds a fixed safety suffix and calls `<agent>-turn.turn.v1` | Turn Conduits (4) |
| `checked.py` | T6 | 16 | Retries initialization timeouts; raises on `isError` or a non-`succeeded` outcome even when the exit code is 0 | 4-13, 4-15 |
| `final_check.py` | T7 | 37 | Final check of tool catalogs, package pins, the preserved history and dispatch records | Checks; inspection gaps (4-09) |

By purpose: document generation 34, command wrapping 43, compensating for engine behaviour 16, verification 37.

## 4. Kind-only view (every line by syntax, causes ignored)

| Kind | Coord T1 | T2 | T4 | T5 | T6 | T7 | Coord all | Fresh T1 | T2 | T4 | T5 | T6 | T7 | Fresh all |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| apiVersion/kind/metadata/name | 64 | 28 | 0 | 24 | 12 | 4 | 132 | 72 | 16 | 8 | 24 | 12 | 36 | 168 |
| `---` | 14 | 5 | 0 | 5 | 2 | 0 | 26 | n/a | | | | | | 0 |
| `spec:` | 9 | 3 | 0 | 3 | 0 | 0 | 15 | 9 | 1 | 0 | 3 | 0 | 0 | 13 |
| JSON punctuation | n/a | | | | | | 0 | 91 | 23 | 10 | 40 | 14 | 41 | 219 |
| Manifest (not digest) | 10 | 8 | 0 | 4 | 4 | 4 | 30 | 8 | 6 | 0 | 8 | 0 | 0 | 22 |
| Digests | 3 | 2 | 0 | 1 | 1 | 1 | 8 | 3 | 2 | 0 | 2 | 0 | 0 | 7 |
| Template inputs | 4 | 4 | 0 | 4 | 0 | 0 | 12 | 7 | 0 | 0 | 7 | 0 | 0 | 14 |
| `from:` | 9 | 4 | 0 | 3 | 3 | 1 | 20 | 9 | 3 | 2 | 3 | 3 | 9 | 29 |
| `capability:` | 5 | 2 | 0 | 2 | 0 | 0 | 9 | 5 | 0 | 0 | 2 | 0 | 0 | 7 |
| `config: {}` | 1 | 1 | 0 | 1 | 0 | 0 | 3 | 0 | | | | | | 0 |
| Pool `type` and `owner` | 2 | 1 | 0 | 1 | 1 | 0 | 5 | 2 | 1 | 0 | 1 | 1 | 0 | 5 |
| `operations` | 14 | 6 | 2 | 7 | 0 | 0 | 29 | 16 | 0 | 0 | 7 | 0 | 0 | 23 |
| consumer/target/declared Conduit | 12 | 6 | 0 | 4 | 4 | 2 | 28 | 30 | 12 | 12 | 12 | 12 | 42 | 120 |
| Resource type and arguments | 3 | 3 | 0 | 3 | 2 | 0 | 11 | 3 | 3 | 0 | 3 | 2 | 0 | 11 |
| Agent type and config | 4 | 0 | 0 | 0 | 0 | 0 | 4 | 8 | 0 | 0 | 0 | 0 | 0 | 8 |

- **Coordinator.**
  - The 132 apiVersion/kind/metadata/name lines are exactly 33 documents × 4, and there are 26 separators: 158 lines (47.6%).
  - With `spec:` it is 173 lines (52.1%), which confirms finding 4-11's "about half".
- **Fresh author.**
  - Frame plus `spec` is 181 lines (28.0%); JSON punctuation-only lines are 219 (33.9%); together 400 (61.9%).
  - Without punctuation the fresh run is 427 lines. Without the recovery's non-punctuation lines (76) as well, it is 351, close to the coordinator's 332.
  - So the JSON format and the T6 recovery explain most of the gap between the two runs. The rest is T4 (32 lines against the coordinator's 2) and T5 (two manifests and a resubmitted template).
- **The JSON was emitted by Python.** It came from `json.dumps(indent=2)` (`run.py:13`). The 231 lines of `t1/` came from 26 lines of Python, and the T2–T7 JSON came from `stages.py` helpers called by commands that were not kept.

## 5. Representative snippets by category

**(1) Frame**
- A document whose only content is one link, `coordinator-run/project/instances.yaml:15-20`. Five of its six lines are frame:
  ```
  ---
  apiVersion: github.com/shrav-k/conduit/v0
  kind: Agent
  metadata:
    name: agent-a
  from: claude-reader
  ```
- `coordinator-run/project/templates.yaml:1-6`: the Pool template is 5 frame lines plus `type: github.com/shrav-k/conduit/core#pool.v1`.
- `fresh-author-run/t1/a.json`: 8 lines, of which 4 frame, 3 punctuation and 1 `"from": "a-template"`.

**(2) Manifest and digests**
- `coordinator-run/project/conduit.yaml:1-5`: `apiVersion`, `packages:`, `example.org/s1/filesystem:`, `version: 0.1.0`, `digest: sha256:200de7c1…`.
- `coordinator-run/project-t5/conduit.yaml:4-5`: `version: 0.2.0` and `digest: sha256:cbb831f0…`. These are the only essential lines of T5.
- `fresh-author-run/t1/conduit.json`: 17 lines for three pins, 6 of them punctuation.

**(3) Template plumbing**
- One path threaded through a schema, in `coordinator-run/project/templates.yaml:14-17` and `instances.yaml:11,13-14`:
  ```
  inputs:
    root:
      schema: {type: string, minLength: 1}
      to: [/config/root]
  ```
  and then, in the instance:
  ```
  from: directory
  arguments:
    root: <lab>/shared
  ```
- `fresh-author-run/t1/directory-template.json:9-17`: the same input takes 7 lines plus 4 punctuation. In `t2/history-template.json:9-10` the same author skipped the input and wrote `"config": {"root": "<lab>/history"}` straight into the template. That shows the input exists only because of the template/instance split.
- `coordinator-run/project/templates.yaml:50`, `from: read`: an Agent's declared Conduit pointing at a template used once.

**(4) Root's own Conduits**
- `coordinator-run/project/templates.yaml:81-91` (`pool-admin`), with `instances.yaml:44-50`:
  ```
  consumer: {kind: Interface, name: root}
  target: {kind: Pool, name: workspace}
  ```
- `coordinator-run/project-t2fix/admin2.yaml`: 16 lines written only to add `- github.com/shrav-k/conduit/core#register.v1` (mistake 2).
- `coordinator-run/project/templates.yaml:65-71` and `instances.yaml:27-34`: `claude-turn` and `turn-a`, 15 lines so that the owner can talk to agent A.

**(5) Access facts**
- `coordinator-run/project/templates.yaml:48-51`:
  ```
  conduits:
    project:
      from: read
      target: {kind: Resource, name: shared}
  ```
  with `templates.yaml:26-28`, the `operations` list.
- `coordinator-run/project-t2/history.yaml:38-39`:
  ```
  consumer: {kind: Agent, name: agent-a}
  target: {kind: Resource, name: history}
  ```
- Coordinator T4: the single line `- example.org/s1/filesystem#file.write.v1` moved between templates, which is all of T4.

**(6) Resource type and arguments**
- `coordinator-run/project/templates.yaml:13`: `type: example.org/s1/filesystem#local-directory.v1`.
- `coordinator-run/project/instances.yaml:13-14`: `arguments:` and `root: <lab>/shared`.
- `coordinator-run/project-t6/replace.yaml:8`: `root: <lab>/shared-new`, the only essential line of T6.

**(7) Agent type and config**
- `coordinator-run/project/templates.yaml:46-47`:
  ```
  type: example.org/conduit/agent-claude-code#claude-code.v1
  config: {accountServices: false, peerMessaging: false}
  ```
  Both keys are required and have no default (`packages/agent-claude-code/contracts.yaml:16-19`).
- `fresh-author-run/t1/a-template.json:8-11`: the same, over 4 lines plus braces.

**(8) Re-declared because objects cannot change**
- `coordinator-run/project-t5/upgrade.yaml:1-11`: `directory-2`, identical to `directory` except its name.
- `coordinator-run/project-t7/restore.yaml`: 7 lines identical to T6's `project-a`, needed because closing removed it.
- Fresh: `t7-restore/a-directory.json` and `t6/a-directory.json` are both byte-identical to `t5/a-directory.json`, and `t5-template/directory-template.json` is byte-identical to T1's.

**(9) JSON punctuation**
- `fresh-author-run/t1/directory-template.json:17-21`: `]`, `}`, `}`, `}`, `}`.
- `fresh-author-run/t1/a-template.json:12-14`: `}`, `}`, `}`.
- Each standalone Conduit file (`t1/a-directory.json`) has 5 punctuation lines out of 16.

**(10) Other**
- `coordinator-run/project/templates.yaml:25`: `capability: example.org/s1/filesystem#directory.v1`. Arguably derivable from the target Resource and the qualified operations.
- `coordinator-run/project/templates.yaml:18`: `config: {}`. The fresh author left it out and the engine accepted the document.

**(11) Pool machinery**
- `coordinator-run/project/templates.yaml:6`: `type: github.com/shrav-k/conduit/core#pool.v1`.
- `coordinator-run/project/instances.yaml:12`: `owner: workspace`. OBJ-013 requires it on every Resource instance.
- Everything tied to the Pool: Pool documents, `owner` lines and the root-to-Pool Conduits come to 55 lines for the coordinator and 54 for the fresh author.

**(12) Recovery**
- `fresh-author-run/t7-recovery/*.json`: `a`, `b`, `a-turn`, `b-turn`, `a-directory`, `b-directory`, `a-history`, `b-history`. 112 lines, all byte-identical to earlier files, written after `conduit retire Agent/a` and `Agent/b` (log 222–226).

## 6. Every place the fresh author got stuck (log.txt line, time, category)

| # | Log line, time | Event | Category; finding |
|---|---|---|---|
| 1 | 2, 11:52:36 | MISTAKE: searched ancestor `AGENTS.md` paths outside its directory | 10, environment; no lines |
| 2 | 4, 11:52:52 | MISTAKE: guessed five `.schema.json` filenames | 10, discovery; 4-14 |
| 3 | 5, 11:52:52 | UNCLEAR: package READMEs show the obsolete `conduit register --package` | 10, docs; 4-14 |
| 4 | 6, 11:53:14 | MISTAKE: no PyYAML, so it switched to JSON and Python generators | 9 (219 punctuation lines) and adapters |
| 5 | 35, 11:54:30 | MISTAKE: `submit` failed with "no root management route to owning Pool" | 4; 4-11. Same as the coordinator's mistake 1; cost the 32 lines of `t1-root-route` |
| 6 | 36, 11:54:30 | UNCLEAR: `submit` registered objects and ran some setups before exiting 1, with no rollback; stdout showed only successes | 10, follows from 4; 4-13 |
| 7 | 37, 11:54:30 | UNCLEAR/MISTAKE: `tools --as a` gave "No such file or directory"; its parser failed on empty output | 10; 4-14; adapters |
| 8 | 49, 11:55:01 | UNCLEAR: `--help` is an unknown command; `--as Agent/a` is an unknown client | 10; 4-14 |
| 9 | 50, 11:55:07 | MISTAKE: guessed `team.set-up.v1` instead of `root-team.set-up.v1` (tool names take the Conduit's name, IFC-012); had to resume Agent setup by hand | 4 (tool names follow Conduit names; 4-11 point 5); 4-13 |
| 10 | 54, 11:55:25 | UNCLEAR: no controls for permission, version or root changes; INS-020 keeps instances; chose close-and-recreate and retire-and-recreate for T4–T6 | 8; 4-01. The source of all 199 category-8 lines |
| 11 | 63, 11:56:05 | `tools --as` works only after setup; the error did not say so | 10; 4-14 |
| 12 | 69, 11:57:25 | UNCLEAR: an operation it may not use has no tool, so a denied write can only be shown absent | 10, checks; no lines |
| 13 | 83, 11:58:14 | UNCLEAR: the history is a filesystem convention (append-only, author and order not enforced) | 10, provider semantics; the baseline found the same |
| 14 | 95, 11:59:46 | UNCLEAR: inspection shows `RunStopped` beside a ready state | 10; 4-16 |
| 15 | 96, 11:59:46 | UNCLEAR: native catalogs unverified (Claude) or not reported (Codex) | 10, T3 inspection |
| 16 | 106, 12:00:52 | MISTAKE/UNCLEAR: initialization timeouts while clients overlapped | 10; 4-15; led to `checked.py` |
| 17 | 119, 12:03:59 | UNCLEAR: Codex's native collaboration tools are present despite `multi_agent` being off | 10; 4-06 |
| 18 | 146, 12:08:26 | UNCLEAR: 2.2.0 adds append/copy but package 0.2.0 maps only read/list/write; no way to ask for a version | 10; checking T5 |
| 19 | 190, 12:16:58 | UNCLEAR: opening `b-directory` timed out during initialization, in sequence | 10; 4-15 |
| 20 | 191, 12:16:58 | MISTAKE: the stage script did not check `open_c`'s exit code | Adapters; 4-13 |
| 21 | 199, 12:21:17 | UNCLEAR: B's turn ended `isError` / `unknown` after about 3 minutes, but exited 0 | 12; 4-12, 4-13 |
| 22 | 201, 12:21:39 | UNCLEAR: A's turn also ended `unknown`; conversations poisoned | 12; 4-12 |
| 23 | 202, 12:21:39 | MISTAKE: repeated an unguarded tools-to-JSON pipeline while initialization failed | Adapters; 4-15 |
| 24 | 250, 12:31:43 | UNCLEAR: `reconcile.v1` / OPS-014 offer one `occurred` value; no way to attest part of a turn | 12; 4-12 |
| 25 | 251, 12:31:43 | UNCLEAR: `file.write.v1` creates or replaces; there is no create-if-absent | 10, provider semantics |
| 26 | 255, 12:32:44 | MISTAKE: asked too many operations per turn for the 180-second deadline | 12; 4-12 |

The report's `could_not` entries are both category 12:
- It could not recover the T6 turns' final text and tool catalogs.
- It could not keep the conversations' continuity.

**Which of these cost lines:**
- Row 4 (format): 219 lines.
- Row 5 (root route): 32.
- Row 10 (change by new documents): 199.
- Rows 21–26 (recovery): 112.
- Rows 16, 19, 20 and 23 led to the 16-line `checked.py`.
- The rest cost time. T6 and T7 alone took 24 min 9 s of 45 min 48 s.

**For comparison, the coordinator's three mistakes:**
- T1, 11:04:51: no root setup route. Category 4, plus the store that had to be discarded (4-07).
- T2, 11:13:07: `submit` answered only `refused`; `register.v1` was missing. Category 4; 4-08.
- T6, 11:27:10: `unregister` said invalid because closing had already removed the Conduit. Category 8; 4-10.

## 7. What the attribution implies

These figures overlap, so they are not additive.
- **Frames:** coordinator 173, fresh 181.
- **JSON punctuation:** fresh 219.
- **Re-declaration for change (category 8):** coordinator 100, fresh 199. Plus the fresh author's recovery re-declarations: 112.
- **Root's own Conduits (category 4):** coordinator 72, fresh 90. The fresh author's recovery adds another 32 of turn Conduits.
- **Pool machinery including root-to-Pool Conduits:** coordinator 55, fresh 54.
- **Template inputs and `from:` links:** coordinator 32, fresh 43, before counting the template documents' own frames.
- **Manifests:** coordinator 38 lines (8 digests); fresh 29 (7 digests) plus their braces.

The essential lines (categories 5, 6 and 7, plus the T5 pin) set beside the baseline:

| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
|---|---|---|---|---|---|---|---|---|
| Coordinator | 20 | 11 | 0 | 2 | 2 | 1 | 0 | 36 |
| Fresh author | 30 | 15 | 0 | 2 | 2 | 1 | 0 | 50 |
| Baseline | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |

In today's vocabulary the access facts alone sit at or under the baseline on every task. The excess over the bar is entirely overhead: frames, root's Conduits, change written as new documents, templates, format and recovery.

## Numbers
| Measure | Coordinator | Fresh author |
|---|---|---|
| Lines (T1/T2/T3/T4/T5/T6/T7) | 154/73/0/2/62/29/12 = 332 | 263/67/0/32/112/44/128 = 646 |
| 1 Frame (outside cause categories) | 80 (24.1%) | 71 (11.0%) |
| 2 Manifest and digests | 21 | 21 |
| 3 Template plumbing | 17 | 16 |
| 4 Root's Pool and turn Conduits | 72 (turn 32, Pool 40) | 90 (turn 58, Pool 32) |
| 5 Access facts | 23 | 33 |
| 6 Resource type and arguments | 7 | 7 |
| 7 Agent type and config | 4 | 8 |
| 8 Re-declared, T4–T7 | 100 (30.1%) | 199 (30.8%) |
| 9 JSON punctuation | 0 | 84 (outside cause categories); 219 in total |
| 10 Other | 5 | 2 |
| 11 Pool `type` and `owner` | 3 | 3 |
| 12 Recovery after T6 | 0 | 112 (17.3%) |
| Frame of all kinds (apiVersion/kind/metadata/name + `---` + `spec:`) | 132 + 26 + 15 = 173 (52.1%) | 168 + 0 + 13 = 181 (28.0%) |
| Frame + JSON punctuation | 173 | 400 (61.9%) |
| Essential (5+6+7 + T5 pin) | 36 (T1 20, T2 11, T4 2, T5 2, T6 1, T7 0) | 50 (T1 30, T2 15, T4 2, T5 2, T6 1, T7 0) |
| Baseline (A70) | 64 (T1 33, T2 16, T4 8, T5 3, T6 2, T7 2) | same |
| T4–T7 lines identical to earlier files | 17 of 103 (44 more differ in 1 line) | 202 of 316 (105 more differ in 1 line; 97% copies or 1-line edits) |
| Adapters | 0 | 130 (T1 77, T6 16, T7 37): document generation 34, wrapping 43, engine compensation 16, checks 37 |
| Documents (not manifests) | 33 | 42 (46 files) |
| Stuck points in the fresh log | | 26: 10 MISTAKE + 16 UNCLEAR. Line cost: format 219, root route 32, change 199, recovery 112 |

## Unverified
- **Coordinator T1 reconstruction.** I counted T1's 154 as the final `project/` files (152) plus `project-t1fix/conduit.yaml` (2), with `project-t1fix/admin.yaml`'s two documents counted once inside `project/`. This is inferred: it reproduces the report's 154 lines and 14 separators. The report does not itemize it, and counting admin.yaml separately would give 153 lines and 13 separators.
- **T1 state of the T4 change.** I took T1's `read` / `read-write` from the post-T4 file with the single write line moved back, as the log describes. No pre-T4 copy of the file is kept.
- **Typed versus emitted lines (fresh author).** The JSON was written by Python (`json.dumps(indent=2)`). The 231 T1 lines came from the retained 26-line `build_t1.py`. The T2–T7 documents came from `stages.py` helpers called by commands that were not kept, so how much the author actually typed for T2–T7 is unknown.
- **Script count.** The report says 7 scripts and 130 lines; only 6 `.py.txt` files are kept, and they total 130 lines. The 7th was not found.
- **Category boundaries are my judgment:**
  - Every template `from:` is category 3, except fresh T4's two changed `from:` lines, which are counted as the access change (5).
  - `spec:` is counted as frame. A manifest's `apiVersion` is counted as manifest.
  - `capability:` is counted as other (10), on the view that it is derivable. I did not confirm in the spec that it can always be derived.
  - Each task's essential delta, especially T7 = 0, assumes that closing and opening would not need any lines.
- **Fresh T5 `-v2` templates.** The log does not say why the fresh author needed new `directory-read-v2` / `directory-write-v2` templates in T5. Package pinning under PKG-005/NAM-009 is a guess.
- **History template.** The fresh author wrote the history's root straight into the template's `config`, with no input. The engine accepted it; I did not check it against TPL or INS promises.
- **Not run.** I did not run the engine or re-run either run. Timings and outcomes come from the logs and reports.