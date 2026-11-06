# Instructions for agents working in this repository

Conduit is a protocol and configuration language for connecting independent Agents to shared
Resources. This repository holds the language's design. No Conduit engine exists yet.

**If you were asked to build an engine, stop.** Engine builders work only from the `build-base`
branch and must not read this branch. See
[independent builds](docs/implementation/INDEPENDENT_BUILDS.md).

## Read first

1. [docs/AUDIT.md](docs/AUDIT.md): the owner's current direction, the ruling log, priorities,
   action items and the experiment program. It governs where other documents differ.
2. [docs/LANGUAGE_DEFECTS.md](docs/LANGUAGE_DEFECTS.md): known places where the accepted text is
   undetermined, contradictory or unsafe.
3. The [placement](docs/proposals/placement/README.md) and
   [instantiation](docs/proposals/instantiation/README.md) records: accepted rulings not yet
   restated in the protocol topics.

## The goal

The language is the core and the runtime fulfills its promise. Conduit is a configuration language,
not a behavioral specification: the language fully determines what a set of declarations means,
and behavior beyond a few shared conventions belongs to type contracts and a runtime profile. See
the layered claim in [docs/AUDIT.md](docs/AUDIT.md#layered-claim). Work that makes the language more
concise, concrete and provable comes first.

The relationship is the product: an agent decides for itself, so the language declares what it may
reach, under which limits, and who answers for what it did. Check any change against the purpose
and preservation tests in [docs/AUDIT.md](docs/AUDIT.md#purpose).

## What binds and what does not

- The specification is [spec/](spec/README.md): the language and the shared conventions, as
  numbered promises. It is a draft being filled area by area, and it governs what it covers.
- Accepted behavior not yet converted is in `docs/protocol/`, as amended by the records under
  `docs/proposals/` for placement, instantiation, vocabulary and relationships.
- Not binding: `docs/research/`, `docs/archive/`, `docs/proposals/authoring-syntax/`,
  `docs/implementation/ARCHITECTURE.md`, `docs/implementation/ORGANIZATION.md`, and Experiments
  001 to 003. Several predate the current direction and say so in a note under their title.
- Only the five family names are settled: Interface, Agent, Resource, Pool, Conduit. All other
  names are provisional. Do not rename or merge terms; the vocabulary pass comes last.

## Where things go

- An owner ruling: the ruling log in `docs/AUDIT.md`, then the documents it affects.
- A defect in the language: `docs/LANGUAGE_DEFECTS.md`. Fix it in the language, not around it.
- Accepted behavior: a promise in `spec/`, written in the ruled vocabulary.
- Evidence: its own directory under `experiments/`, with dated results. Never present an
  expected outcome as a result.

## Rules

- Any code is Rust, in its own directory under `experiments/` with a `Cargo.lock`, a pinned
  toolchain and ignored build output. The Python checkers in `tools/` are the only exception.
- The owner makes the rulings. When a choice changes behavior, present the options with a
  recommendation and wait; do not record your recommendation as accepted.
- Do not edit `docs/archive/`, the content of `docs/research/`, or the sibling directories
  `../agentwatch` and `../pi-reference`, which are reference clones to consult only.
- Commit or push only when asked, and work on a branch.

## Checks

```sh
python3 tools/check_docs.py
python3 tools/set_names.py --check
git diff --check
```

The names the specification assigns (the core package's address, `apiVersion` and the prefix of
its `_meta` keys) are defined once, in `spec/names.yaml`. Never edit one by hand: run
`python3 tools/set_names.py` with the new values, which rewrites every occurrence together.

Run `cargo test --release` in each Rust experiment directory you change.
