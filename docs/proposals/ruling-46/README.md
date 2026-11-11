# Ruling 46: the first build

Status: in force; amended by 58 (point 4: increment logs become settlements register); amended by 68 (point 4: point 2's fallback void for v0) (ruled by the owner on 2026-11-09). Holds now: [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

Ruling 44 made the first build (A68) the next milestone, and asked to discuss its nature, and
how it tests the theses and refines the language, before it starts. Ruling 45 left one question
in the order of the work for that discussion: whether S2 to S9 run on the reference model (A38)
or on the reference engine (A50). The owner asked: "It seems like it's finally time to try and
build the runtime and run experiments to try and refine our design. What should be the first
step and is there anything I should rule on?"

The coordinator proposed a first step and put five points to the owner, each with a
recommendation. The first step it proposed is the strict loader in Rust (A43), with the first
build's pre-registration written beside it and the baseline for S1 (A70) run alongside. The
owner's answer: "I accept all your recommendations, let's proceed".

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | What the first build is | **a**: one codebase in Rust in two parts. A pure, deterministic core, the reference model (A38), in which every decision cites a promise, driven by the conformance vectors. A thin shell that makes it runnable: it runs servers on the engine's own runner, serves consumers at the engine's address with a token, and keeps records. The shell is the first slice of the reference engine (A50), and S2 to S9 run on the same codebase as it grows. This is the build that AGENTS.md sets apart from its stop rule. |
| 2 | The real Agents of S1 | **a**: Claude Code and Codex, so that two different clients, unchanged, are tested. Two instances of Claude Code if one of them cannot reach an MCP server over HTTP with a token. |
| 3 | Success criteria, stated before the build runs | Accepted: round 6's suggested criteria for the first author trial, with the comparison against the baseline for S1 (A70). Observed time and mistakes are recorded before any numeric target is set. |
| 4 | How the build refines the language | **a**: each increment logs what it finds: a gap, an ambiguity, a contradiction or needless complexity, each against the IDs of the promises. The coordinator fixes it in the same pull request, with a vector, and logs the fix as its own, as it settles a cold reader's finding under rulings 37 and 38; the owner can strike any of them. A finding whose fix adds or removes a feature, or changes what Conduit is for, comes to the owner. |
| 5 | The providers of S1 | **a**: the engine's built-in providers count as S1's first providers: its own runner, the local machine and fetching from a local directory. S1 has no secret, so the first packaged provider is a secret store, a file store and the macOS keychain, which begins with S2. |

On point 3, the criteria of round 6's
[critique](../reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage), in its
words:

- a competent reader can complete the checked S1 path without reading ruling history or
  editing a provider;
- the S2 restrictions differ as intended without copying the shared service credential to
  agents;
- closing one agent's access leaves the other working;
- the user can identify and resolve uncertainty without guessing that an effect failed.

The second applies when S2 runs.

The coordinator's own choices, which the owner did not object to:

- The code goes in `reference-model/` and the vectors in `conformance/`, the directories the
  repository already plans.
- It uses existing crates, such as the official Rust SDK of the Model Context Protocol and an
  existing JSON Schema validator, rather than writing its own.
- Each increment is a branch and a pull request, merged by squashing once its checks pass.

## What this settles

- **The nature of the first build (A68)**, which ruling 44 left for this discussion.
- **The open point in the order of the work** that ruling 45 left: S2 to S9 run on the same
  codebase as S1, the core and the shell, as the shell grows into the reference engine (A50).
- **Who settles what the build finds.** Point 4 extends the delegation of rulings 37 and 38 to
  findings of the build.

## What follows

1. The pre-registration of the first build: the theses, the increments with what each must
   show, the success criteria and the measures shared with the baseline for S1. It is in
   [the first build](../../implementation/FIRST_BUILD.md).
2. Increment 1, the strict loader in Rust (A43), on a branch of its own.
3. The baseline for S1 (A70), in `experiments/007-s1-baseline/`, which states before it runs
   what it measures.
