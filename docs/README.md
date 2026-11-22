# Documentation

Conduit is a configuration language for the relationships between independent agents and the
things they share. The [specification](../spec/README.md) alone binds; this page maps the supporting
documents.

## Reading paths

Read first, in [AGENTS.md's order](../AGENTS.md#read-first):

1. [spec/README.md](../spec/README.md): the layers, contents and form of a promise.
2. [spec/definitions.md](../spec/definitions.md): the terms in force.
3. [IN_FORCE.md](../IN_FORCE.md): every decision in force that the specification does not hold.
4. [docs/AUDIT.md](AUDIT.md): the state of the work, what waits for the owner and the action items.
5. [ROADMAP.md](../ROADMAP.md): the order of the work and experiment register.

The top [README](../README.md) introduces Conduit. The specification's
[reading paths](../spec/README.md#reading-paths) serve document authors, contract and provider
writers, and engine builders. Cold readers start at the [readers' guide](guides/cold-readers.md).

## What each document is

### What binds

[spec/](../spec/README.md) holds the language, shared conventions and implementation interface as
numbered promises, with definitions, schemas, examples and the core package.

### Entry points and tracker

| Location | What to find |
| --- | --- |
| [IN_FORCE.md](../IN_FORCE.md) | Decisions in force outside `spec/`, and accepted language decisions marked *spec pending*. |
| [the ledger](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-log.md) | The chronological ledger: what was ruled, when, the owner's words and each ruling's status. |
| [AUDIT.md](AUDIT.md) | State of the work, open questions, authority, purpose and action items. |
| [ROADMAP.md](../ROADMAP.md) | The work's order, experiment states, scenarios and measures. |
| [AGENTS.md](../AGENTS.md), [CONTRIBUTING.md](../CONTRIBUTING.md) | Contributor rules, promise editing, document upkeep, checks and attribution. |
| [OVERVIEW.md](OVERVIEW.md) | A tour of the language. |
| [guides/](guides/README.md) | Setup and cold-reading instructions. |
| [LANGUAGE_DEFECTS.md](LANGUAGE_DEFECTS.md) | The first defect sweep and what became of its findings. |
| [tools/](../tools/) | Repository checkers; [requirements and coverage](../CONTRIBUTING.md#checks). |

### Records of rulings

The ledger names each ruling and its status; IN_FORCE.md collects what still holds; a
record says why. They are archived as [AGENTS.md](../AGENTS.md#what-holds) says;
[this section at `077fa47e`](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/README.md#records-of-rulings) lists each.

### Process for the builds

| Location | What to find |
| --- | --- |
| [implementation/FIRST_BUILD.md](implementation/FIRST_BUILD.md) | First-build increments, tasks and measurements. |
| [implementation/first-build/](implementation/first-build/log-increment-4.md) | The first build's open log, increment 4's, and the S1 rerun's [measurement plan](implementation/first-build/measurement-plan.md). |
| [implementation/plan-and-apply/](implementation/plan-and-apply/README.md) | The design of `conduit plan` and `conduit apply` and their golden cases. Authors use the [CLI format guide](../engine/CLI.md). |
| [implementation/SAFETY_CATALOGUE.md](implementation/SAFETY_CATALOGUE.md) | The safety promises of `spec/`, and the safety promises each of S1's safety traces tests, for the safety vectors and model checking (A40, G5). |
| [implementation/PROVIDERS.md](implementation/PROVIDERS.md) | The first and second provider of each provider contract, their contract vectors and the engine work they need (A63; work order 120). |
| [engine/](../engine/README.md) | The runnable first build, its [CLI format guide](../engine/CLI.md) and its [conformance choices](../engine/CONFORMANCE.md). |
| [conformance/](../conformance/README.md) | Portable vectors, fixtures, coverage and gaps. |
| [implementation/WORK_ORDERS.md](implementation/WORK_ORDERS.md) | Coordinator and worker roles, dispatch, review and records. |
| [work-orders/](work-orders/README.md) | Open orders and report schemas. |
| [implementation/INDEPENDENT_BUILDS.md](implementation/INDEPENDENT_BUILDS.md) | Independent builds, isolation and readiness gate. Builders use their separate branch. |
| [implementation/ENGINE_BUILD_BRIEF.md](implementation/ENGINE_BUILD_BRIEF.md) | The brief for independent builders. |

### Evidence and history

| Location | What to find |
| --- | --- |
| [experiments/](../experiments/README.md) | Methods, dated results and counted artifacts, registered in the roadmap. |
| [research/](research/README.md) | Sources and comparisons behind the decisions. |
| [archive/](archive/README.md) | Removed-file index and the plan audit's history. |

## Repository boundaries

The [contributor guide](../CONTRIBUTING.md#project-ownership-and-publication) records repository,
package, license and publication rules. The specification alone determines behavior.

## Maintaining the design

Follow the [contributor rules](../CONTRIBUTING.md#keeping-decisions-and-documents-current) for
rulings, extractions, removals and condensation at close. Work state belongs in AUDIT.md and the
roadmap; decision state belongs in IN_FORCE.md and the [decision log](decisions/log.md).
