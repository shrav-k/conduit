# Simplifying what an author writes: the investigation of 2026-11-10

Not binding. The [specification](../../../spec/README.md) alone binds. A proposal for the owner,
from a multi-agent investigation the owner asked for after both runs of S1's tasks failed the bar
for v0. The working papers are under [research/](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md): six maps of the problem,
five designs each checked by three skeptics, a synthesis and a completeness critic. Every count
here is a paper count: ruling 45 accepts only a run as evidence, so the recommendation ends in a
rerun.

## The problem in numbers

The owner's bar: less of an author's work than the baseline (A70) on most of S1's tasks, and much
more on none. Both runs fail it ([coordinator](../first-build/increment-4/coordinator-run-2026-11-10.md),
[fresh author](../first-build/increment-4/fresh-author-run-2026-11-10.md)).

| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| The facts S1 needs, one per line ([floor](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-floor.md)) | 11 | 5 | 0 | 2 | 1 | 1 | 1 | 21 |
| Baseline (A70) | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| Coordinator | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 |
| Fresh author (JSON) | 263 | 67 | 0 | 32 | 112 | 44 | 128 | 646, and 130 lines of scripts |

Where the coordinator's 332 lines went ([attribution](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-attribution.md)): 100 (30%)
re-declared things because nothing that exists can change (T5 to T7); 80 (24%) document frames;
72 (22%) the root Interface's own Conduits, to its Pool and for turns; 21 manifests and digests;
17 template plumbing. **The access facts themselves were 23 lines.** The fresh author's run adds
219 brace-only JSON lines and 112 lines recreating Agents after turns ran past their deadline.

## What was tried

Five designers each took one angle and rewrote all of T1 to T7 in it; three skeptics checked each
for its counts, its safety and meaning, and its cost to the specification and the build.

| Design | Idea | Lines | Counts | Safety | Cost |
| --- | --- | --- | --- | --- | --- |
| A, conservative | Defaults and shorthands in the language; equal resubmission allowed | 41 | major | major | minor |
| B, apply in the engine | Resubmitting a changed document updates the object, revisions and restarts in place | 81 | major | major | major (about 60 promises) |
| C, agent-centric file | One Compose-like file, normative; revise and swap in the engine | 33 | major | major | major (about 80 promises) |
| D, relationship-first | One line per relationship; apply in the engine; operation groups | 26 | minor | major | major (about 50 promises) |
| E, layered authoring | A short project file compiled by the CLI into today's documents; plan and apply compose today's requests | 26 | minor | major | minor |

Every design beat the baseline on paper; every one had a major safety objection; only A and E were
cheap. The skeptics' objections are the most useful output: a later apply silently reverting a
revocation, derived objects whose access an operator cannot see, a package version flipping a
default unseen, name reuse cutting access early, a plan approved but not the plan run. Each is in
[research/](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md).

## The recommendation: layered authoring with small core additions

Built on E, with the safety fixes every skeptic demanded and the best parts of A, C and D
([synthesis](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md)). Every object stays immutable, every grant stays a real
Conduit listing its operations, and nothing runs without a request.

1. **One short project file**, compiled by the CLI into today's canonical documents. Resources and
   Agents are keyed maps; each grant is one line under its Agent:
   `project: {to: shared, ops: [file.read.v1, directory.list.v1]}`. `conduit render` shows the
   canonical documents. A diagnostic points at the author's line and quotes the engine's
   diagnostic and promise ID. Spec unchanged.
2. **Defaults the CLI derives**, each a real, recorded Conduit shown with the rule that made it: a
   Pool per project, the operator's Conduit to it, and a turn Conduit per Agent. This removes the
   first mistake both authors made. Spec unchanged.
3. **`conduit install` writes the pins** into the file, as `cargo add` does. No hand-copied digest.
4. **An explicit plan, then apply**: the CLI diffs the file against what exists and sends today's
   requests: open and close for grants, and for an upgrade or a new directory a replacement made
   before the old is retired, re-pointing the grants. It narrows before it widens, never runs in the
   background, and never prunes or reopens something closed outside it without a flag.
5. **Five small core additions** (about 11 to 14 promises, about 3% of the specification): `as`,
   a local name for a standalone Conduit, so tool names never change (finding 4-11);
   `expectedId` on open and close, so a plan cannot act on a swapped object; an inventory in
   `inspect.v1` of the Conduits an object consumes and is the target of (4-09); tombstones the
   engine keeps for what was closed or retired, so a revocation is not undone by a later apply;
   and tool descriptions that name their target and its Type's description.
6. **Provider work**: the Agent Types default their reach flags to false in their contracts, and
   state their reach around the engine truly (4-06, 3-07), so T3 is one command.
7. **The command defects**, which order 028 is fixing, and the turn deadline (4-12).

Projected, by the measurement plan's own rule:

| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Baseline | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| Package | 16 | 6 | 0 | 2 | 1 | 1 | 1 | 27 |

Better on six tasks, the same on T3. The full rewrite is in the synthesis.

## What the critic found, which must be settled first

The [critic](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/critic.md) recounted the rewrite and found the same numbers, then found gaps
that can turn the pass into a fail:

1. **Confirming a plan without a terminal.** A fresh Codex author cannot answer `y/N`. Approving a
   saved plan by its ID is a copied value, which counts as a line. Counted so, the package is
   better on only three or four tasks: the margins on T5 to T7 are one or two lines.
2. **The revocation guard can be bypassed** when an Agent is replaced, and Agents, which act through
   requests and not the CLI, never see it. It needs keying by the grant, and enforcing in the
   engine, not only in the CLI.
3. **Where a fresh author learns the format.** The project file is not in `spec/`; ruling 48 gives
   the fresh author only the specification and the tasks. The honest concept count is 15 to 20,
   not 8.
4. **Short operation names contradict ruling 25** (fully qualified, no aliases) and save no line.
5. Medium: controls, the language's existing in-place change, are ignored; the file cannot yet
   express what S2 to S9 need; T5 gains a window without access; counting hidden Conduits
   discloses that they exist; the compiler's meaning has no vectors, so thesis 2 is not tested for
   what authors write.

## The question underneath

The cheap route keeps the language as it is and puts a short authoring layer in front of it. That
meets the bar on paper, but the language an implementer reads stays as large, and what the author
writes is defined by a CLI, not by the specification. The alternative, making the short form the
language, costs about 50 to 80 promises and a rebuild of much of increments 1 to 3. The synthesis
recommends the layer now and folding its lessons into the core later (frames once per file, inline
one-off definitions, whether `replace.v1` and declared Conduits earn their place), each by its own
ruling, after a rerun shows which savings are real.

## Decisions for the owner

1. **Does an authoring layer count for the bar and for thesis 1?** (a) Yes for v0: the project file
   is what an author writes, and the canonical documents remain the language that fully decides
   meaning; promote it into `spec/` later if a second implementation must agree. (b) No: the short
   form must be the language before the rerun. **Recommended: (a)**, with the critic's point 9
   answered by golden compiler cases under `conformance/`, marked not binding.
2. **How a plan is approved without a terminal**, and how that counts: (a) a saved plan applied by
   its ID, counted as a line; (b) one-step commands (`conduit grant`, `revoke`, `upgrade`) beside
   the file; (c) apply without confirmation for the fresh author's run, with the plan printed and
   recorded. **Recommended: (b)** for changes, keeping the file for setup: it removes the copied
   value and matches how the baseline was counted. To be settled before the rerun (ruling 45).
3. **The five core additions**, each approved apart: `as` (**yes**), `expectedId` on open and close
   (**yes**), the inventory (**yes**, listing only what the caller may inspect, without counts of
   the rest), tombstones (**yes, enforced by the engine** at open and register, per the critic),
   and descriptive tool descriptions (**yes**, ruling 52).
4. **Explicit plan and apply as a client of today's operations**, never automatic, never pruning
   without a flag. **Recommended: confirm.**
5. **Where pins live**: in the project file, written by `install` (**recommended**), or in a lock
   file the tool writes.
6. **The derived defaults** (a Pool per project, the operator's Conduit, a turn Conduit per Agent),
   each recorded and shown. **Recommended: accept.**
7. **Agent reach flags default to false in the Type contracts** (ruling 48's "off by default").
   **Recommended: yes**, as new package versions.
8. **Operation names in the file stay fully qualified** (ruling 25). **Recommended: yes**; only line
   width is at stake.
9. **What the fresh author receives in the rerun**: the specification, the tasks, and a format
   reference for the project file shipped with the CLI, as "what the engine ships".
   **Recommended: yes**, stated before the run.

## What would happen next

Before the owner decides, reversibly: order 028's command fixes (running); a format reference
drafted as a proposal; a Rust spike under `experiments/` that compiles T1 to T7 and checks that
today's loader and core accept the result. After decisions 1 to 4: a specification batch for the
core additions, the compiler, the planner with the skeptics' safety cases, and the provider
versions, as work orders. Then a dry run by the coordinator and a fresh author's rerun, under a
plan written in advance.

## Carried from the papers

Historical options and costs, not new decisions. Later rulings govern their status.

### Rejected, and why

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md#rejected). Copied verbatim.

B (apply in the engine, revisioned instances, update and restart in place): rejected as a whole.
- Cost: large. About 45 promises edited and 13-17 new, the core grows 40-65%, and stores need a clean break.
- It fails the bar on its own: T1 is 60 lines against 33.
- Safety, as the skeptics found:
  - a later apply silently reverts changes made outside apply (swap.v1 or controls);
  - an unrelated apply re-creates retired Agents;
  - the plan digest does not cover the documents (SUB-008 hashes contracts only);
  - [APL-006](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/design-B-apply.md#L61) contradicts ACC-024;
  - a restart in place keeps an old run's credential valid (G12);
  - plan.v1 leaks Resource config past row 6;
  - its T5 contradicts [APL-003](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/design-B-apply.md#L40)(c), since the Agents restart;
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

### The load-bearing guarantees and incidental costs

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-mechanisms.md#1-short-answer). Copied verbatim.

- **Where the lines go.** About half of the coordinator's document lines are the per-document frame: `apiVersion`, `kind`, `metadata`, `name` and `---` make up 164 of the 328 lines in its final files. A quarter is plumbing for the root Interface that has no counterpart in the baseline and protects nothing for root (81 lines). That plumbing is the root Interface's Conduits to its own Pool (38 lines), its Conduits to each Agent for turns (30 lines; finding 4-11 does not list these) and a Pool template plus its instance (13 lines). 27% is change done by writing new documents (T5–T7: 88 lines plus 15 lines of manifests). 10% is manifests (34 lines), of which 8 are copied digests and 10–14 are manifests the spec does not require.
- **What is load-bearing.** The safety guarantees behind drivers (b) and (d) are real, but narrower than the rules that carry them. Eight must survive any simplification:
  1. Stored references hold IDs, never names.
  2. An open Conduit's content never changes. Narrowing is close + open, and every open passes `access` with its bound check.
  3. Nothing implicit re-issues a revoked grant.
  4. The code and configuration running under an already-opened grant never change unless that grant is decided again.
  5. What runs is the installed digest.
  6. Operation lists are explicit and fixed.
  7. Revision preconditions protect against concurrent actors.
  8. Every request is authorized by a named open Conduit and recorded.
- **What is incidental.** These cost lines and protect nothing:
  - the per-document frame;
  - the mandatory template behind every instance (IN9);
  - template input declarations for one-off values;
  - a standalone Conduit's tool prefix being its global object name (the recorded reason was rename, which ruling 41 removed);
  - a closed Conduit being removed, so restoring it needs a document;
  - LIF-009 rebuilding a replacement from "the template as it now stands";
  - root needing explicit Conduits to its own Pools and Agents. Root already holds that authority through ACC-022 plus `root-management`, and ACC-024 makes those Conduits unclosable, so they add no revocability;
  - hand-copied digests and unneeded manifests.
- **Paper estimates against the bar** (§5):
  - Tooling and packages only, no language change: about 136 lines against 64. Fails.
  - Targeted change operations on today's document form: about 61–70. Borderline; still more on T2 and T6.
  - Those operations plus a compact authoring form and an explicit, plan-based apply: about 25–34. Passes on paper.
  - To meet the bar, both the frame and in-place change must move.

### What the v1-candidate reductions lose

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N2-new-core.md#whatislost). Copied verbatim.

1. The long form.
   - No one-object-per-document syntax remains for authors, for diffs in review or for Kubernetes-style tooling.
   - `render` prints the explicit file instead, with every default written out and derived objects marked.
   - Every existing document, fixture archive, digest and store breaks. This is a clean break, acceptable only because v0 has no users.
2. Declared Conduits.
   - A template, a package-exported one included, can no longer bundle relationships. Reuse at scale writes a grant line per instance; the real use costs about 9 lines per worker in block-map style (about 6 flow), not 1.
   - TPL-012's guarantee that Conduits are open before the run becomes "the run fails closed without them" (SEC-010).
   - Setup opens nothing. An Agent set up outside the planner starts with no tools until its grants are opened (G9 weakened, as in the layered route).
   - Optional Conduit parts of a setup are gone (S4 keeps optional entries).
   - The gain is safety: skeptic D's HIGH #2, package templates whose Conduits opened under root's unbounded access, disappears.
3. replace.v1.
   - Templates are immutable. A changed reusable template means unregister and register again, or a new name.
   - The fresh-context replacement of D11 goes away, with its complexity.
4. register-replacement.v1 for Conduits.
   - Restoring a grant is register plus open under the same handle.
   - register-replacement.v1 remains only for Pool entries.
5. Conduit names.
   - A Conduit is known only by (consumer, local name) and its ID.
   - Conduit references, records display and the open, close and inspect inputs change shape.
   - Successor objects carry tool-chosen names (`shared--2`), which no promise determines.
6. Meaning versus change. Thesis 2 covers what the file means, not how a change is carried out: order, successor names and plan choices belong to the tool, as in the layered route.
7. The derivation is now language.
   - The project Pool and its operator grant are fixed by [FIL-007](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N2-new-core.md#L272), so decision 6 must be settled before the spec batch.
   - Explicit turn grants cost 3 lines in T1, plus the 1-line `grants:` header, so T1 is 20 against the layered package's 16.
8. Duplication. Because definitions are inline, two Agents with the same grant write its operation list twice unless the author makes a template.
9. Rulings to amend: 8 IN9, 18 V2, 29 (declared Conduits and setup), 31, 35 and 37 (the frame), 41 (CON-004's content), and 53 decision 6. Ruling 53 #1 itself, the short form as the language.
10. Evidence and time.
    - Increments 1 to 3 must be re-evidenced, and about 70-80% of the vector inputs converted.
    - The rerun waits for stage 1 at least.
11. Still out of reach in any form until promises exist: peer mode and trust domains, enrollment, pending actions, the expected account, the restart verb, and a composite template that makes an Agent with its own Resources.
12. Also not solved. These need decisions 2 and 11 and a stage-2 swap.v1:
    - the strict command-counting reading, under which S1 is better on only 3 of 7 tasks against the better T5 baseline;
    - authoring in JSON, which fails the bar;
    - the windows without access in T4 to T6.
13. Cut but optional: removing Pool entries too would delete about 17 more promises. It would lose S4's composite setup and entry-stable replacement, so I do not include it without an owner ruling under ruling 4's guard.

### Measured costs of core reductions

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/inventory-build.md#key-points). Copied verbatim.

9. Removing declared Conduits saves little code: they share the nested machinery with Pool entries, and the Conduit-specific setup branch is about 15 lines (lifecycle.rs:537-549). It still invalidates 126 of 367 vectors with regenerated expectations, because all 118 S1 cases shift IDs. It touches about 30-40 promises and 6 shell test files. It also moves TPL-012's guarantee (Conduits opened before the run starts) to clients.

10. register-replacement.v1 also serves Pool entries and D09's rotation by explicit replacement (S2, S10), so removing it fails the scope guard. replace.v1 is about 120 lines; removing it touches 15 vectors.

11. Removing declared Conduits, replace.v1 and register-replacement.v1 together invalidates 141 of 367 vectors (38%). Making the short form the only accepted form touches about 252 of 367 (69%), plus all 114 spec examples and check_schemas.py.

### Later core candidates

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md#decisions). Copied verbatim.

13. Later, after the rerun (not now): whether to simplify the core along the lines the layer exposes. The candidates:
- inline one-off definitions (amending IN9 and ruling 18's V2);
- the frame stated once per file (rulings 31, 35, 37);
- LIF-009 building from the held definition;
- whether declared Conduits, replace.v1 and register-replacement.v1 earn their place in v1.
