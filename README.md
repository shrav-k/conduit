# Conduit

Conduit is a configuration language for the access around AI agents. Engineers use it to declare
exactly what each agent can reach (files, folders, services like GitHub, other agents) and which
operations it may perform. Credentials stay in stores like 1Password or the Keychain, so agents
use services without ever seeing the secrets. The [components](docs/COMPONENTS.md) page names
every part in a line each.

Languages such as Kubernetes, Docker Compose and Terraform declare workloads, which do what
their configuration says. An agent decides for itself what to do. So Conduit declares what
surrounds the agent: what it may reach, under which limits, and who answers for what it did.

An author writes documents. They describe objects of five families. An Agent is a participant
whose model decides its own actions. An Interface is where a person, or a system outside, acts
on the system. A Resource is a shared thing that agents use, such as a project directory or a
service login. A Pool is what owns and groups Resources. A Conduit is a relationship: it joins
one consumer to one target and lists the operations it permits. A request is permitted only
when an open Conduit from its consumer to its target lists the operation. Conduit invents only
the connection and the access. Everything underneath is existing infrastructure the user
chooses, reached through a contract: a secret store such as the macOS keychain or 1Password, the
runtime that runs things, the machines they run on, the source of packages. An engine is a
running program that implements the language. The work behind an operation is done by an
implementation, and the shared wire between an engine and an implementation is the Model
Context Protocol (MCP).

## What exists

This repository holds the design of the language. It holds no engine.

| Part | State on 2026-11-07 |
| --- | --- |
| [Specification](spec/README.md) | A draft. Every area of the language is written as numbered promises, counted by file and by layer in its [contents table](spec/README.md#contents). A promise is one rule, stated so that it can be checked. With them are the definitions of the terms, JSON Schemas of the documents, example documents and the core package, which is the package built into every engine. "Draft" means that the specification is not complete and is not a release: no promise is yet tested by a conformance vector, which is a test case that any engine can be run against. Secrets are in it ([secrets.yaml](spec/promises/secrets.yaml)): a document refers to a secret and never holds it, and each kind of store is reached through a contract that a package implements. It works across machines: the server of a run that a runtime provider other than the engine's own runner makes, and a server reached by `remote` that takes Conduit's keys, stand behind a gate that admits only a request that proves the engine sent it, in one of two standard forms, mutual TLS or a signed token, and the engine's address always accepts the token form, which is the Model Context Protocol's own authorization. What this version leaves out or does not narrow is in its [limits](spec/README.md#limits-of-this-version). [Its reading paths](spec/README.md#reading-paths) say what to read for writing documents, for writing contracts and providers, and for building an engine. |
| How it got here | The owner accepted the specification as it stood at ruling 39, and rulings 40 to 43 had it rewritten around provider contracts, which took graphs, runtimes declared in templates, policies and composite Conduits, among other things, out of this version. Readers who were given only the specification, or the whole repository, read it in six rounds and reported what they found; every finding is resolved and logged ([re-audit](docs/proposals/reaudit/README.md)). Round 5 read the rewrite: six readers reported 119 findings, ten of them unsafe, and every one is fixed or rejected; the 29 findings of the check of its fixes are all resolved, the last, V6, by the coordinator. Round 6 was the owner's cold read by Codex, of the whole repository: six findings, two of them safety-critical, all fixed, and a critique of the design ([record](docs/proposals/reaudit/round-6/README.md)). The owner accepted the decisions on it in ruling 44, which makes the next milestone a working build: the first scenario through a reference model in Rust, the vectors for the safety cases first and the first providers. The tracker ([docs/AUDIT.md](docs/AUDIT.md#where-things-stand), called the plan audit) says where the work stands and what waits for the owner. |
| Reference model | An executable model of the language, in Rust, is the next milestone (ruling 44). It does not exist yet. |
| Engine | Does not exist. |
| Code | Four Python tools in `tools/`: three checks that are run before each commit, and the record of one past migration. A model-checking evaluation in Rust ([Experiment 004](experiments/004-stateright-evaluation/README.md)) and a probe of the macOS keychain, for the first secret store ([Experiment 005](experiments/005-macos-keychain/README.md)). None of it runs Conduit. |

## What binds

The specification in [spec/](spec/README.md) alone binds. Nothing else in this repository does.

- Everything under `docs/` and `experiments/` is explanation, record, research or evidence.
- The design as it stood before the specification, and the other documents that the
  specification superseded, were removed from the tree. The
  [archive's index](docs/archive/README.md#removed-from-the-tree) lists them, with a link to
  each last version, as source material for the type contracts still to be written.
- A behavior that is missing from `spec/` is a gap in `spec/`. It is not supplied by an older
  document.
- Accepted behavior changes by a ruling of the project's owner, logged in the
  [plan audit](docs/AUDIT.md#ruling-log). A finding of the re-audit may also be settled by the
  session that coordinates the work, without a ruling on it, and each such decision is logged;
  ruling 39 accepted every one made before it. [Who decides what](docs/AUDIT.md#who-decides-what)
  says which is which.

## Start here

Read these in order. It is the order for work on the specification. A reader who wants the idea
before the detail can read the [overview](docs/OVERVIEW.md) first: it is a plain-language tour
with one worked example, and it does not bind.

| # | Read | For |
| --- | --- | --- |
| 1 | [Specification](spec/README.md) | What the specification is, its three layers, its contents and the form of a promise. |
| 2 | [Definitions](spec/definitions.md) | The terms in force, each with one meaning. |
| 3 | [Plan audit](docs/AUDIT.md) | The tracker: the state of the work, the owner's rulings, the open action items and the experiment program. |

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
