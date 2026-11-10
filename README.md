# Conduit

Conduit is a configuration language for the relationships between independent agents and the
things they share. In practice, an engineer uses it to declare what each AI agent may reach
through Conduit (files, folders, services such as GitHub, other agents) and which operations it
may call there. A document holds only a reference to a secret, never its value.

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

As of 2026-11-10, this repository holds the draft language and the first build: a strict loader,
a pure core, the reference model, and a runnable engine shell. The build covers S1, shared
access for two Agents; it is not a complete implementation of the specification.

| Part | State on 2026-11-10 |
| --- | --- |
| [Specification](spec/README.md) | A draft, incomplete and not a release. Every area is written as numbered promises: one rule each, stated so it can be checked. It includes definitions, JSON Schemas, examples and the built-in core package. The first build now tests slices of promises with [conformance vectors](conformance/README.md); its [coverage manifest](conformance/manifest.yaml) distinguishes vectors from missing coverage. The specification declares secrets and work across machines, but the first build has not demonstrated all of them. See the specification's [limits](spec/README.md#limits-of-this-version) and [reading paths](spec/README.md#reading-paths), and the build's [conformance document](reference-model/CONFORMANCE.md) for its implemented scope. |
| How it got here | The owner accepted the specification as it stood at ruling 39, and rulings 40 to 43 had it rewritten around provider contracts, which took graphs, runtimes declared in templates, policies and composite Conduits, among other things, out of this version. Readers who were given only the specification, or the whole repository, read it in six rounds and reported what they found; every finding is resolved and logged ([re-audit](docs/proposals/reaudit/README.md)). Round 5 read the rewrite: six readers reported 119 findings, ten of them unsafe, and every one is fixed or rejected; the 29 findings of the check of its fixes are all resolved, the last, V6, by the coordinator. Round 6 was the owner's cold read by Codex, of the whole repository: six findings, two of them safety-critical, all fixed, and a critique of the design ([record](docs/proposals/reaudit/round-6/README.md)). The owner accepted the decisions on it in ruling 44, which makes the next milestone a working build; the [roadmap](ROADMAP.md#the-order-of-the-work) says what it holds and what comes after it. The tracker ([docs/AUDIT.md](docs/AUDIT.md), called the plan audit) says [where the work stands](docs/AUDIT.md#where-things-stand) and [what waits for the owner](docs/AUDIT.md#waiting-for-the-owner). |
| [Roadmap](ROADMAP.md) | The order of the work, with the state of each step, and every experiment and probe, planned, running or done, with what it feeds. Check it before proposing an experiment. Not binding. |
| [Reference model](reference-model/README.md) | The first build's Rust workspace: the strict loader (increment 1, done 2026-11-09), the pure core on S1 and its vector runner (increment 2, done 2026-11-10), and the shell (increment 3, done 2026-11-10). Each decision of the core cites a promise. [The build logs](docs/proposals/first-build/README.md) record its findings; the [roadmap](ROADMAP.md#the-order-of-the-work) lists increment 4, S1 complete with author measurements, next. |
| Engine | The shell ran real Claude Code and Codex as Agents on one machine through S1's T1 (shared files), T2 (message history) and T7 (close and explicitly restore access). The [dated closeout](docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md) records results and unshown checks. Access is mediated; both Agents run as the engine's operating-system user with shared logins, and Codex still reads its home's instructions. Separate homes and sign-in through the root Interface are ruled in [ruling 49](docs/proposals/ruling-49/README.md), pending implementation. The two independent builds are later work, not this shell. |
| Code and evidence | Five Python tools in `tools/`; the Rust workspace in [reference-model/](reference-model/README.md); standalone [Claude Code](packages/agent-claude-code/README.md) and [Codex](packages/agent-codex/README.md) Agent packages; and Rust evaluations and harnesses under [experiments/](experiments/README.md). The experiments include model checking, the keychain, the S1 baseline, real-client behavior and native launch; Experiment 010 is a desk check of subscription terms. |

## What binds

The specification in [spec/](spec/README.md) alone binds. Nothing else in this repository does.

- Everything under `docs/` and `experiments/` is explanation, record, research or evidence, and
  the [roadmap](ROADMAP.md) is the plan of the work.
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

Follow these links in order, stopping at the path you need:

| # | Read | For |
| --- | --- | --- |
| 1 | [Overview](docs/OVERVIEW.md) | Understand the language through a plain-language tour and one worked example. |
| 2 | [Specification](spec/README.md) | Read the rules that bind, beginning with its definitions and the reading path for your role. |
| 3 | [Run the first build](reference-model/README.md#running-it) | Run the tests and vectors, then follow the shell and live S1 instructions for real Agents. |
| 4 | [Guides](docs/guides/README.md) | Find the owner's setup guides and the collaborators' cold-reading instructions. |

Then, as needed:

| Read | For |
| --- | --- |
| [Overview](docs/OVERVIEW.md) | A plain-language tour of the language with one worked example. Not binding. |
| [Example documents](spec/schemas/examples/) | Valid and invalid documents, checked against the schemas. |
| [Documentation map](docs/README.md) | Every other document, with what it is and whether it is current. |
| [Plan audit](docs/AUDIT.md) and [roadmap](ROADMAP.md) | The owner's rulings, open action items, order of work and experiment register; check the register before proposing an experiment. |
| [AGENTS.md](AGENTS.md) | The rules for agents and contributors, including what not to edit and the checks to run. |
| [Contributing](CONTRIBUTING.md) | Where each kind of work belongs, and the license. |

## License

Original Conduit documentation, contracts and code are licensed under [Apache-2.0](LICENSE).
Copyright 2026 Conduit contributors. Reproduced third-party material retains its own terms and
attribution; see the [license scope](CONTRIBUTING.md#license-and-attribution).
