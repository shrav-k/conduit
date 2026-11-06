# Build brief

Status: draft, 2026-11-05, revised for layer decision LD4. **Not issued.** This is the text a
builder receives as `BUILD.md` on the build base once the
[readiness gate](INDEPENDENT_BUILDS.md#readiness-gate) passes. Of the paths it names, `spec/` and
`tools/` exist; `conformance/` does not exist yet, and the build base is not yet published.
Everything from the next heading down is written to stand alone, because the builder sees
nothing else.

---

## Your task

Build an implementation of the Conduit language, in Rust, from the documents in this checkout
and nothing else.

Conduit is a configuration language. It determines what a set of documents means: which
objects exist, how they relate and which invariants always hold. You are one of two independent
builders. The purpose is to test whether the language is precise enough that two builders who
never talk produce implementations that agree. Your implementation matters, and so does your
record of every place the language left you unsure.

You are not building a full engine. Much of what a running system does is deliberately outside
the language, and you must not invent it.

## What you have

| Path | What it is |
| --- | --- |
| `spec/README.md` | The contents list of the specification. Start here. It lists every normative file. |
| `spec/` | The specification: the language, the shared conventions and the implementation interface, as numbered promises, with the definitions, the schemas of documents and the core package. |
| `tools/` | The two checkers the specification refers to, `check_schemas.py` and `set_names.py`. You may run them. They are not part of the specification. |
| `conformance/README.md` | How conformance vectors are structured and how your implementation must run them. |
| `conformance/` | The public conformance vectors, each tagged with the promises it tests, and the test implementations. |
| `BUILD.md` | This brief. |

The promises are the authority. A vector illustrates a promise. If a vector and a promise
disagree, record it (see below) and follow the promise.

The specification's version is `v0`, as `spec/names.yaml` states. The tag you were given,
`spec-vN`, names one published revision of that version. It is not a version of the
specification.

Some files under `spec/` link to records under `docs/`, which this checkout does not hold. Those
records are the history of a rule. The rule itself is in `spec/`, and you need nothing else.

## What you must not do

- Do not read any other branch, tag or commit of this repository, or its history before the
  commit you were given.
- Do not look for Conduit design documents, discussions or other implementations anywhere,
  including the web. You may read public documentation for Rust, the crates you use, the Model
  Context Protocol, JSON Schema and YAML.
- Do not contact, read or infer anything from the other builder.
- Do not ask the owner what the language means. There is no private clarification. Record the
  question as a defect and continue.
- Do not edit anything under `spec/`, `conformance/` or `tools/`.
- Do not key any behavior on a vector's identifier or contents. Additional vectors you have not
  seen will be run against your implementation.
- Do not add behavior the language does not promise in order to be helpful. Where the language
  says a behavior is implementation-defined, choose one and document it in your runtime profile
  (see Deliverables).

## What to build

Put everything under `impl/` as one Cargo workspace, with a pinned toolchain file.

1. **The language layer.** A library that takes documents and returns either the resolved
   objects they describe or diagnostics. It parses, validates against the schemas, resolves
   references, performs instantiation and checks every static rule in `spec/`. It is a pure
   function: no files written, no network, no clock, no randomness in its result.
2. **A minimal engine for the shared conventions.** The smallest program that honors each shared
   convention in `spec/` for the objects the language layer resolves. It exists to make the
   conventions testable, not to be useful on its own.
3. **The implementation interface.** The engine loads and calls implementations exactly as
   `spec/` defines. Test implementations are supplied under `conformance/`; they must work
   unchanged.
4. **A conformance adapter.** A binary that takes the path of a vector file and writes what your
   implementation produced, in the format `conformance/README.md` defines. This is how your work
   is compared.
5. **Your own tests**, runnable offline with `cargo test`.

Out of scope, and not to be built: anything the language assigns to a type contract, anything
it leaves to the runtime profile beyond the choices your minimal engine needs, user interfaces,
integrations with real model providers or external services, and performance work. If a promise
seems to need one of these, record it as a defect.

## How to work

- Work milestone by milestone in the order `conformance/README.md` gives. Make every vector of a
  milestone pass before starting the next.
- Commit in small steps with messages that say what changed and which promises it implements.
- Cite the promise identifier in a comment wherever code implements a specific promise.
- Record every third-party crate, its exact version and why you chose it in
  `impl/DEPENDENCIES.md`. Tests must not need the network.
- Keep `impl/LOG.md`: one dated line per work session with what you did and how long it took.

## When the language is unclear

This is the most valuable thing you produce. Do not resolve doubt silently.

Record an entry in `impl/DEFECTS.md` whenever any of these happens:

| Kind | Meaning |
| --- | --- |
| U, undetermined | The language covers the situation but does not determine what an observer should see. |
| C, contradiction | Two promises require different outcomes for the same situation. |
| X, unsafe | Following a promise literally breaks another promise. |
| D, dangling | You need a fact the language does not give in order to implement a promise at all. |
| V, vector mismatch | A vector expects something the promises do not require, or forbids something they allow. |

Each entry has: an identifier (`BD-001`, `BD-002`, …); its kind; the promise identifiers involved; the
concrete situation, as specific values and events; the readings you considered; the reading you
chose; and the files where your choice is implemented.

Then continue. Choose the most conservative reading: the one that refuses or reports rather than
acts, and that changes the least state. Mark the code with the entry's identifier, for example
`// DEFECT BD-012`.

Also keep `impl/ASSUMPTIONS.md` for anything you assumed about the environment or about a
standard the language builds on, which is not a defect in the language itself.

## Deliverables

On your branch, under `impl/`:

- The workspace, building and passing its tests offline from a clean checkout.
- The conformance adapter.
- `DEFECTS.md`, `ASSUMPTIONS.md`, `DEPENDENCIES.md` and `LOG.md`.
- `PROFILE.md`: your runtime profile. It states your choice for every item of
  `spec/implementation-defined.yaml`. Where your minimal engine never reaches an item, say so.
- `REPORT.md` at each milestone: which vectors pass, which fail and why, which promises are not
  yet implemented, and how many defects you logged.
- `README.md`: exact commands to build, test, and run the adapter over a vector.

## Done

A milestone is done when every public vector for it passes through your adapter, your own tests
pass offline, and `REPORT.md` and `DEFECTS.md` are current. The build is done when that holds for
every milestone. Report honestly: a failing vector with a logged defect is more useful than a
passing one obtained by guessing.
