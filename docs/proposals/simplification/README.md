# Simplifying what an author writes: the investigation of 2026-11-10

Not binding. The [specification](../../../spec/README.md) alone binds. A proposal for the owner,
from a multi-agent investigation the owner asked for after both runs of S1's tasks failed the bar
for v0. The working papers are under [research/](research/synthesis.md): six maps of the problem,
five designs each checked by three skeptics, a synthesis and a completeness critic. Every count
here is a paper count: ruling 45 accepts only a run as evidence, so the recommendation ends in a
rerun.

## The problem in numbers

The owner's bar: less of an author's work than the baseline (A70) on most of S1's tasks, and much
more on none. Both runs fail it ([coordinator](../first-build/increment-4/coordinator-run-2026-11-10.md),
[fresh author](../first-build/increment-4/fresh-author-run-2026-11-10.md)).

| | T1 | T2 | T3 | T4 | T5 | T6 | T7 | All |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| The facts S1 needs, one per line ([floor](research/map-floor.md)) | 11 | 5 | 0 | 2 | 1 | 1 | 1 | 21 |
| Baseline (A70) | 33 | 16 | 0 | 8 | 3 | 2 | 2 | 64 |
| Coordinator | 154 | 73 | 0 | 2 | 62 | 29 | 12 | 332 |
| Fresh author (JSON) | 263 | 67 | 0 | 32 | 112 | 44 | 128 | 646, and 130 lines of scripts |

Where the coordinator's 332 lines went ([attribution](research/map-attribution.md)): 100 (30%)
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
[research/](research/synthesis.md).

## The recommendation: layered authoring with small core additions

Built on E, with the safety fixes every skeptic demanded and the best parts of A, C and D
([synthesis](research/synthesis.md)). Every object stays immutable, every grant stays a real
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

The [critic](research/critic.md) recounted the rewrite and found the same numbers, then found gaps
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
