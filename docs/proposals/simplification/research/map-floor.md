Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# The information floor of S1: what an author must state, and what the baseline and both Conduit runs spend beyond it

SCOPE AND METHOD

Repository: <home> I only read it and changed nothing. All paths below are relative to that root.

What I read:
- The baseline: experiments/007-s1-baseline/setup/*, results-2026-11-09.md and its log.
- The coordinator's run: every file under docs/proposals/first-build/increment-4/coordinator-run/, its log, t3-answer.md and the run report.
- The fresh author's run: all 46 JSON files, log.txt, t6-recovery-notes.md and the report.
- The shipped package contracts: packages/agent-*/contracts.yaml and conformance/fixtures/s1/packages/*.
- log-increment-4.md, rulings 49 to 52, and the purpose and preservation tests in docs/AUDIT.md.

How facts and lines are counted:
- A **fact** is information that any system must get from the author to do the task, and that it cannot derive from another fact or from a sensible default.
- Some things S1 needs to be true can come from a default:
  - the owner may run turns of the agents it declares;
  - an agent reaches nothing it was not given;
  - a grant needs no approval on each call;
  - one machine, one owner, one Pool;
  - each agent gets a home of its own.
  These are listed separately. They cost 0 lines in the "strict" floor and about 1 line each in the "explicit" floor.
- Lines are counted as the measures count them: non-blank lines written or changed, each line once per task, `---` counted, comments excluded.
- I wrote the floor forms out and counted them with grep. They are in <scratch>/floor/ (t1-strict.yaml, t2-strict-add.yaml, t2-noprovider-add.yaml, t1-explicit.yaml, t1-k8s-manifest.yaml, t1-k8s-docs.yaml, t2-k8s-add.yaml).

1. THE IRREDUCIBLE FACTS, TASK BY TASK

**T1 (9 facts)**
1. A shared thing named `project` exists. The name is needed so that grants can refer to it.
2. It is a directory served by the filesystem server at version 2.1.0 (which provider, and which version, on one line).
3. Its location is <lab>/shared.
4. Agent `a` exists.
5. a is Claude Code.
6. a may read `project` (read and list).
7. Agent `b` exists.
8. b is Codex.
9. b may read and write `project` (read, list and write).

The baseline gets facts 4, 5, 7 and 8 for nothing: which file a line sits in says which agent and which client it is for.

**T2 (5 facts)**
- History `history` exists.
- It is served by a history provider, or by filesystem 2.1.0.
- It lives at <lab>/history.
- a may post and read.
- b may post and read.

There is one conditional fact. If the backing tool is a bare filesystem server, the posting convention must be stated once, about 3 lines: one new file per post named <UTC>-<agent>.md, never change an existing post, read in name order. In Conduit this is provider work: the history package's type description carries it (conformance/fixtures/s1/packages/history/contracts.yaml). The baseline wrote it twice: CLAUDE.md.txt:1-3 and AGENTS.md.txt:1-3.

**T3 (0 facts)**
Nothing is written. The ideal inspection is one command that reports, for each agent, its grants and its reach around them.

**T4 (2 facts)**
- a: read becomes read-write.
- b: read-write becomes read.

"Live" is a property of the system, not information the author gives.

**T5 (1 fact)**
- The directory's server goes from 2.1.0 to 2.2.0.

Knowing that 2.2.0 adds append_file and copy_file is needed only when access is written as a list of tool names. The baseline needed it (concepts C20 and C21). Conduit's package 0.2.0 maps only three operations (conformance/fixtures/s1/packages/filesystem-0.2.0/conduit.yaml), so a list of allowed operations is safe across the upgrade.

**T6 (1 fact)**
- The location changes from <lab>/shared to <lab>/shared-2.

"Keeping access" adds nothing if grants name the directory rather than its path.

**T7 (2 facts on 1 line)**
- a's access to `project` is closed, then opened again.

Checking that b still works is a check, not a fact. The floor is either one line edited twice (1 line by the counting rule) or two requests and no lines.

**A possible future fact, absent from these runs:** the account each Agent is expected to sign in as (ruling 49, point 3). That is up to 1 line per Agent unless it gets a default.

2. THE FLOOR FORM

Strict floor for T1, 11 lines (2 section keys plus 9 facts):

```yaml
resources:
  project:
    use: filesystem@2.1.0
    root: <lab>/shared
agents:
  a:
    use: claude-code
    project: read
  b:
    use: codex
    project: read-write
```

The other tasks in the strict floor:
- **T2 adds 5 lines:** `history:`, `use: history@0.1.0` and `root: <lab>/history`, plus `history: post-read` under each agent. It is 8 lines if there is no history provider and the convention goes once into a `notes:` key.
- **T4:** 2 lines changed.
- **T5:** 1 line, `use: filesystem@2.2.0`.
- **T6:** 1 line, `root:`.
- **T7:** 1 line edited twice.

The explicit floor:
- T1 is 16 lines: it adds `conduit: v0`, plus `turns: owner` and `reach: none` for each agent.
- T5 is 2 lines if the author writes the digest rather than a tool generating it into a lock file.

3. THE PER-TASK TABLE (lines)

| Task | Facts | Strict floor | Explicit floor | Baseline A70 | Coordinator | Fresh author |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | 9 | 11 | 16 | 33 | 154 | 263 in 19 files |
| T2 | 5 (+ convention) | 5 | 5 to 8 | 16 | 73 | 67 |
| T3 | 0 | 0 | 0 | 0 | 0 | 0 |
| T4 | 2 | 2 | 2 | 8 | 2, plus 8 requests | 32 |
| T5 | 1 | 1 | 2 | 3 | 62 | 112 |
| T6 | 1 | 1 | 1 | 2 | 29 | 44 |
| T7 | 2 (1 line) | 1 | 1 | 2 | 12 | 128 (16 restore + 112 recreating both Agents) |
| All | 20 | 21 | 27 to 30 | 64 | 332 | 646, plus 130 lines of scripts |
| Times the strict floor | | 1.0 | 1.3 to 1.4 | 3.0 | 15.8 | 30.8 |

Lines beyond the strict floor:

| | T1 | T2 | T4 | T5 | T6 | T7 | All |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Baseline | +22 | +11 | +6 | +2 | +1 | +1 | +43 |
| Coordinator | +143 | +68 | 0 | +61 | +28 | +11 | +311 |
| Fresh author | +252 | +62 | +30 | +111 | +43 | +127 | +625 |

Author requests that are not checks:
- **Floor:** about 8, one apply per task and two for T7.
- **Coordinator:** 35.
- **Fresh author:** at least 41, counting the log's `COMMAND tN: conduit …` lines.
- **Baseline:** no commands, but two trust dialogs, one restart in T4 and one more restart expected in T5.

4. BASELINE AGAINST THE FLOOR (64 against 21)

About 11 of the baseline's 64 lines carry facts. That is:
- A's server entry and its allow line in T1 and T2;
- the 2 facts of T4, which sit inside tool-name edits;
- A's version line in T5, path line in T6 and deny line in T7.

The other 53 lines:
- **Trust entries, 9:** home-additions/claude.json:1-7 and codex-config.toml:1-2. All of them fall in T1.
- **JSON and TOML structure, 16:** T1 11, T2 4, T7 1 (a comma).
- **Tool-name lists, 13** (beyond the 2 facts they carry):
  - T1: A's read-only access written as `deny` plus 4 tool names, 6 lines for one fact (settings.json at T1, results:153).
  - T4: removing them (7), plus B's `disabled_tools` (1).
  - T5: removing `disabled_tools` (1).
- **Server and path restated for B, 6:** config.toml:4-5 and 9-10 restate .mcp.json, then once more each in T5 and T6.
- **The convention written per agent, 6** (3 more than stating it once).
- **Approval and sandbox settings, 3:** config.toml:1, 6 and 11.

Per task:
- **T1:** +22, mostly trust, structure and the deny list.
- **T2:** +11, mostly the convention twice, B restating the server and path, and structure.
- **T4:** +6. Two facts written as tool-name list edits, plus 1 restart.
- **T5:** +2. The version restated for B, and B's restriction rewritten as `--read-only` because a list of tool names was not safe across the upgrade. On top of that: 2 recovery steps, 1 expected, and one write B should not have been able to make.
- **T6:** +1, the path restated.
- **T7:** +1, a JSON comma.

If the trust entries are read as two dialog clicks, as results:112-115 allows, the baseline's T1 is 24 and its total is 55.

37 of those 53 lines have no counterpart in Conduit's files: trust, tool-name lists, restated definitions, the per-agent convention, approval and sandbox. This is real credit to Conduit. Conduit's T3 (2 commands) and its T5 safety (no Agent gained a tool) are also near the ideal.

5. COORDINATOR AGAINST THE FLOOR (332 against 21)

21 lines carry facts: T1 10, T2 6, T4 2, T5 1, T6 1, T7 1. I attributed the other 311 by document and by type of line; the sums match the report exactly.

**(a) Change by new documents (finding 4-01): 100 lines**
- **T5, 61 beyond the version:**
  - 4 manifest lines, 1 of them a digest;
  - a new Resource template, 12;
  - a new Resource instance, 9;
  - two new access templates, 21;
  - two new standalone grants, 15 (project-t5/upgrade.yaml:1-57).
- **T6, 28:**
  - the manifest restated, 5;
  - a new Resource instance, 8 beyond its root line;
  - two re-pointed grants, 15 (project-t6/replace.yaml).
- **T7, 11:** the revocation was one request with no lines. The restore restated the manifest (5) and the grant (7), because closing a Conduit had removed it (project-t7/restore.yaml; log 11:34:16; finding 4-10).

**(b) Whole documents S1 needs only as defaults: 83 lines**
- **Root to Agent turn Conduits, 32:** templates.yaml:65-79 and instances.yaml:28-42.
- **Root's Conduits to its own Pool for set-up and register, 38:** templates.yaml:81-91, instances.yaml:44-50, project-t2fix/admin2.yaml, and their 2-line manifests. They were the coordinator's mistakes 1 and 2, and a discarded store (findings 4-07 and 4-08).
- **The Pool template and instance, 13:** templates.yaml:1-6 and instances.yaml:1-5.

**(c) Frame and structure around the facts in T1 and T2: 128 lines**
- **T1, 79 lines:**
  - document frame of the 8 documents S1 needs, 42 (apiVersion, kind, metadata, name, spec, `---`);
  - the manifest beyond the version, 10: 3 digests, 4 lines pinning the Agent packages, 3 frame and address;
  - capability and operation-address lines, 9;
  - Agent structure, 8: 2 reach flags and 6 `conduits`, `project` and `target` keys;
  - Resource plumbing, 7: `inputs`, `schema`, `to`, `config: {}`, `owner`, `arguments`;
  - `from:` lines, 3.
- **T2, 49 lines:**
  - the Resource template, 11;
  - the Resource instance, 7;
  - the access template `post-read`, 11;
  - two standalone grants, 13;
  - the manifest, 7 (the filesystem pin restated, plus a digest).

**T4** matches the floor in lines (2). But it took 8 requests, left each agent about 30 seconds without access, and left the templates named `read` and `read-write` granting the opposite of their names: in the final templates.yaml:20-39, `read` lists file.write.

**Frame overall:**
- 128 lines of apiVersion, kind, metadata, spec and packages, plus 26 separators: 154 lines, 46%.
- Adding the 29 names of objects that S1 does not need named: 183 lines, 55%.
- There are also 45 lines of contract addresses.

6. FRESH AUTHOR AGAINST THE FLOOR (646 against 21)

The causes are the same, made larger by JSON:
- 219 of the 646 lines (34%) hold only braces or brackets.
- 183 lines hold apiVersion, kind, metadata or spec.

Per task:
- **T1, 263 lines:**
  - 108 in documents that defaults should supply: turns 58, the root route 32, the Pool 18;
  - the manifest, 17, with 3 digests;
  - 138 in the 10 documents that carry the 9 facts.
- **T2, 67 lines:** the manifest 13 (filesystem restated), the history template 13, the instance 9, and two grants of 16 lines each.
- **T4, 32 lines:** the author used standalone grants, so changing access meant close, redeclare two 16-line grants, submit, open. The coordinator's route through templates cost 2 lines. The same change costs 2 or 32 lines depending on the route.
- **T5, 112 lines:**
  - the template resubmitted with its manifest, 30;
  - a new Resource with its manifest, 21;
  - two new access templates, 29;
  - two new grants, 32.
- **T6, 44 lines:** a new Resource (12) and two re-pointed grants (32).
- **T7, 128 lines:** 112 recreate both Agents and every grant after T6's turns passed the 180-second deadline and ended unknown (finding 4-12; t6-recovery-notes.md). 16 restore A.

The author also wrote 130 lines of Python to drive the commands. It used JSON because PyYAML was missing (log.txt line 6). Having no edit-and-apply workflow pushed it into scripting, and scripting pushed it into the format that costs the most lines.

7. FACTS CONDUIT STATES THAT THE BASELINE NEVER STATES, AND WHETHER THEY ARE WORTH THEIR LINES

| Fact | Baseline | Coordinator lines | Worth it? | Floor lines |
| --- | --- | --- | --- | --- |
| Who may run each agent's turns (root to Agent Conduits) | Implicit: whoever runs the CLI | 32 | Worth existing as an object (preservation tests 5 and 8). Not worth writing by hand in S1: the engine can make it for the Interface that registers the Agent, and it stays inspectable and closable | 0 (2 explicit) |
| Root's authority to set up and register in its own Pool | n/a | 38 | No. The root already holds this authority, the only actor with access at first start. It was the first mistake of both authors | 0 |
| Ownership by a Pool | n/a | 13, plus `owner:` on each Resource | Not in S1, which has one owner. Needed from S2 ("establish once, share"). Use a default Pool | 0 |
| Package pins with digests | Version only (`npx -y …@2.1.0`) | 38 manifest lines in 7 files, 8 digests | The version, yes, one line. Integrity, yes, but generated into a lock file (the npm and Cargo way, ruling 40) and not restated with every submission | 1 per version |
| What each agent reaches around Conduit (accountServices, peerMessaging) | Never stated. The baseline's T3 answer missed Codex's account-level servers (mistake 7) | 2 | Yes as information (ruling 52 and the T3 rule), but they should default to off. Both Agent types say these are "off by default" (packages/agent-*/contracts.yaml:7) yet require both with no default (:19) | 0 (2 explicit) |
| Consumer and target named on every grant | Implicit, from which file | 3 lines per declared grant, 7 to 8 per standalone grant | The pair is the grant: 1 line. Kinds and a separate document per grant are not worth it, and separate grants also change tool names (finding 4-11) | 1 per grant |
| Operations listed by contract address | Tool names in deny or disabled lists | 45 address lines | Allowing a list of operations is worth it: it is why T5 gave no Agent a new tool. Full addresses on every grant are not: a preset defined by the provider (`read`, `read-write`, `post-read`) is the same allow list in one word | 0 extra |
| Records of every request | None | 0 | Yes, and free: 93 access decisions recorded at no cost to the author | 0 |
| Revocability as an object | One edit in one agent's syntax | Revoke: 1 request, no lines. Restore: 12 lines and 2 requests | Yes. The cost of restoring is incidental: closing removes the Conduit | 0 to 1 |
| Template and instance for each thing | n/a | Every object written twice | No, not in S1: each template is used once, except `post-read` | 0 |
| Resource input plumbing (inputs, schema, to) | n/a | 5 per Resource template | No. The Type already states its config's schema | 0 |

8. WHAT THE BAR NEEDS

**a) Changes have only 1 or 2 lines of headroom.**
- The baseline's T5, T6 and T7 are 3, 2 and 2 lines; the floor's are 1, 1 and 1.
- Any change that writes a new document, restates a manifest or makes the author write a digest ties or loses.
- So "less on most tasks" needs two things:
  - change as an in-place edit that is applied again (4-01's "resubmitting a changed document updates the object");
  - closing that keeps the Conduit so it can be opened again.

**b) One Kubernetes-style document per object loses T1 even with every other fix.**
I counted it in t1-k8s-*.yaml. The fixes kept were:
- no templates;
- no Pool;
- no root Conduits;
- default turns;
- presets;
- digests in a lock file.
Only apiVersion, kind, metadata, name and spec per object remained.

The result:
- T1 is 31 lines, or 29 if the Agent packages need no pin. That is against the baseline's 33, or its 24 if the trust entries count as clicks.
- T2 is 12 against 16.
- The frame alone, about 6 lines per object across 4 objects, comes to 24 lines, three quarters of the baseline's T1.
- A single file in sections reaches 11. That is the Compose shape: each thing declared once at the top level, and one line per consumer for its access mode, like a Compose volume's `:ro`.

**c) Both 4-01 and 4-11 are needed.**
- Fixing only how change works leaves T1 at 154 and T2 at 73: much more than the baseline.
- Fixing only the form makes T5 and T6 each a new Resource plus re-pointed grants, about 5 lines against 3 and 2.

**d) The format is part of the measure.** JSON costs about 1.5 times the lines of YAML.

**e) Ruling 52 does not conflict with a short form.** Explicitness belongs in results, refusals, inspection and records. Those can expand `project: read` into its operations, its Conduit and its provider. Preservation test 2 still holds if every one-line grant becomes a Conduit object.

**f) One lever needs no ruling, and it is small.** A package may hold templates (spec/definitions.md:342). The S1 packages could therefore export the directory, `read`, `read-write`, `post-read` and Agent templates. That removes about 40 lines across T1 and T2, and leaves T1 at about 75 or more.

## Numbers
| Measure | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Facts | 9 | 5 (+ convention) | 0 | 2 | 1 | 1 | 2 (1 line) | 20 |
| Strict floor (lines) | 11 | 5 | 0 | 2 | 1 | 1 | 1 | 21 |
| Explicit floor | 16 | 5 to 8 | 0 | 2 | 2 | 1 | 1 | 27 to 30 |
| Baseline A70 | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 (3.0 times the floor) |
| Coordinator | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 (15.8 times) |
| Fresh author | 263 | 67 | 0 | 32 | 112 | 44 | 128 | 646 (30.8 times) |
| Kubernetes frame with every other fix | 31 (29) | 12 | 0 | 2 | 1 | 1 | 1 | about 48 |
| Requests: floor / coordinator / fresh | 1 / 4 / 11 | 1 / 5 / 4 | 1 / 2 / – | 1 / 8 / 5 | 1 / 5 / 6 | 1 / 8 / 5 | 2 / 3 / 10 | about 8 / 35 / at least 41 |

| Where the lines beyond the floor go | Lines |
| --- | --- |
| Baseline overhead, total | 53 |
| …trust entries | 9 |
| …JSON and TOML structure | 16 |
| …tool-name lists | 13 |
| …server and path restated for B | 6 |
| …convention written per agent | 6 |
| …approval and sandbox | 3 |
| Baseline overhead with no counterpart in Conduit's files | 37 of 53 |
| Coordinator overhead, total | 311 |
| …change by new documents (T5 to T7) | 100 |
| …documents S1 needs only as defaults | 83 (turns 32, root Pool-admin 38, Pool 13) |
| …frame and structure in T1 and T2 | 128 |
| Coordinator frame | 154 (46%), or 183 (55%) with names S1 does not need |
| Coordinator contract-address lines | 45 |
| Coordinator digests | 8 |
| Fresh author: brace-only lines | 219 (34%) |
| Fresh author: frame-key lines | 183 |
| Fresh author: T7 recreating Agents after deadline timeouts | 112 |
| Fresh author: scripts | 130 lines in 7 files |
| Headroom over the baseline on T5 / T6 / T7 | 2 / 1 / 1 lines |

## Unverified
- **The floor forms are hypothetical.** No loader accepts them. I counted them with grep in the scratchpad, and did not check them against spec/schemas. The Kubernetes-frame variant writes `config` straight on an object with no template, which the current schemas do not allow.
- **The attribution of the coordinator's and fresh author's lines to causes is my own.** The totals match the reports exactly. Lines that serve two purposes (for example, names that are both frame and identity) were each put in one bucket.
- **The baseline's "about 11 fact-carrying lines" and its overhead by mechanism are approximate,** because some structure lines also carry names.
- **The coordinator's T1 count of 154** I inferred as 11 + 91 + 50 + 2 (with t1fix/conduit.yaml, and without t1fix/admin.yaml, whose documents were merged into the project files). Its report does not spell this out.
- **The fresh author's request counts come from matching `COMMAND tN: conduit …` lines in its log.** Its helper scripts may have issued requests that are logged differently, so the counts are lower bounds. I did not count its T3 requests.
- **I did not check that the floor's presets (`read`, `read-write`, `post-read`) can be expressed in today's spec.** The nearest construct is a Conduit template exported by a package, which spec/definitions.md:342 allows. I did not try it with the loader.
- **Whether ruling 49's expected account becomes a required line per Agent is undecided.** It may raise the floor by up to 2 lines.
- **Whether the baseline's T4 restart is needed in a trusted Claude Code workspace is open** in the baseline's own results. That bears on T4's comparison but not on lines.
- **I ran nothing,** no engine, client or checker. Every number is read or counted from the files.