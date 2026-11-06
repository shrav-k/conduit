# Conduit

Conduit is a configuration language for the relationships between independent agents and the
things they share.

Languages such as Kubernetes, Docker Compose and Terraform declare workloads, which do what
their configuration says. An agent decides for itself what to do. So Conduit declares what
surrounds the agent: what it may reach, under which limits, and who answers for what it did.

An author writes documents. They describe agents, the shared things those agents use, such as a
project directory or a service login, and the relationships between them. Each relationship is a
Conduit: it joins one consumer to one target and lists the operations it permits. A request is
permitted only when an open Conduit from its consumer to its target lists the operation. An
engine is a running program that implements the language. The work behind an operation is done
by an implementation, and the shared wire between an engine and an implementation is the Model
Context Protocol (MCP).

## What exists

This repository holds the design of the language. It holds no engine.

| Part | State on 2026-11-05 |
| --- | --- |
| [Specification](spec/README.md) | A draft. Every area of the language is written as numbered promises. A promise is one rule, stated so that it can be checked. With them are the definitions of the terms, JSON Schemas of the documents, example documents and the core package, which is the package built into every engine. Three readers who were given only the specification have audited it once; the [plan audit](docs/AUDIT.md#where-things-stand) says what followed. No promise is yet tested by a conformance vector, which is a test case that any engine can be run against. The contents table of [spec/README.md](spec/README.md#contents) gives the files and the counts. |
| Reference model | An executable model of the language is planned. It does not exist. |
| Engine | Does not exist. |
| Code | Four checkers in `tools/`, a model-checking evaluation in Rust ([Experiment 004](experiments/004-stateright-evaluation/README.md)) and a library probe ([Experiment 003](experiments/003-language-probes/README.md)). None of it runs Conduit. |

## What binds

The specification in [spec/](spec/README.md) alone binds. Nothing else in this repository does.

- Everything under `docs/` and `experiments/` is explanation, record, research or evidence.
- [docs/protocol/](docs/protocol/README.md) is the design as it stood before the specification.
  It is superseded and is kept as source material for the type contracts still to be written.
- A behavior that is missing from `spec/` is a gap in `spec/`. It is not supplied by an older
  document.
- Accepted behavior changes only by a ruling of the project's owner, logged in the
  [plan audit](docs/AUDIT.md#ruling-log).

## Start here

Read these in order.

| # | Read | For |
| --- | --- | --- |
| 1 | [Specification](spec/README.md) | What the specification is, its three layers, its contents and the form of a promise. |
| 2 | [Definitions](spec/definitions.md) | The terms in force, each with one meaning. |
| 3 | [Plan audit](docs/AUDIT.md) | The state of the work, the owner's rulings, the open action items and the experiment program. |

Then, as needed:

| Read | For |
| --- | --- |
| [Overview](docs/OVERVIEW.md) | A plain-language tour of the language with one worked example. Not binding. |
| [Example documents](spec/schemas/examples/) | Valid and invalid documents, checked against the schemas. |
| [Documentation map](docs/README.md) | Every other document, with what it is and whether it is current. |
| [AGENTS.md](AGENTS.md) | The rules for agents and contributors, including what not to edit and the checks to run. |
| [Contributing](CONTRIBUTING.md) | Where each kind of work belongs, and the license. |

## License

Original Conduit documentation, contracts and code are licensed under [Apache-2.0](LICENSE).
Copyright 2026 Conduit contributors. Reproduced third-party material retains its own terms and
attribution; see the [license scope](CONTRIBUTING.md#license-and-attribution).
