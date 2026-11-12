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
| [the ledger](proposals/ruling-log.md) | The chronological ledger: what was ruled, when, the owner's words and each ruling's status. |
| [AUDIT.md](AUDIT.md) | State of the work, open questions, authority, purpose and action items. |
| [ROADMAP.md](../ROADMAP.md) | The work's order, experiment states, scenarios and measures. |
| [AGENTS.md](../AGENTS.md), [CONTRIBUTING.md](../CONTRIBUTING.md) | Contributor rules, promise editing, document upkeep, checks and attribution. |
| [OVERVIEW.md](OVERVIEW.md) | A tour of the language. |
| [guides/](guides/README.md) | Setup and cold-reading instructions. |
| [LANGUAGE_DEFECTS.md](LANGUAGE_DEFECTS.md) | The first defect sweep and what became of its findings. |
| [tools/](../tools/) | Repository checkers; [requirements and coverage](../CONTRIBUTING.md#checks). |

### Records of rulings

The ledger names each ruling and its status; IN_FORCE.md collects what still holds. Records say
what was put to the owner, what was decided, by whom and why. The specification governs wherever
a record differs. Ruling 58, point 1, accepts logged choices made before ruling 57 except review
items 15 to 19 and increment 4's open decisions. The index below lists the surviving records;
removed documents are in the [archive index](archive/README.md).

| Record | Subject |
| --- | --- |
| [the ledger](proposals/ruling-log.md) | Chronological ruling ledger and verbatim history. |
| [proposals/reaudit/](proposals/reaudit/README.md) | Re-audit decisions and index of rounds. |
| [proposals/placement/](proposals/placement/README.md) | Placement of instances. |
| [proposals/instantiation/](proposals/instantiation/README.md) | What a template lets its instantiator supply. |
| [proposals/normative-surface/](proposals/normative-surface/README.md) | Specification structure and promise form. |
| [proposals/vocabulary/](proposals/vocabulary/README.md) | Vocabulary principles and decisions. |
| [proposals/relationships/](proposals/relationships/README.md) | Conduits, access and grant lifetime. |
| [proposals/operations/](proposals/operations/README.md) | Request identity, outcomes and revisions. |
| [proposals/declarations/](proposals/declarations/README.md) | References, submissions and Pool entries. |
| [proposals/graphs-and-schemas/](proposals/graphs-and-schemas/README.md) | Graph decisions and the schema profile. |
| [proposals/packages-and-start/](proposals/packages-and-start/README.md) | Packages, first start and conformance. |
| [proposals/naming/](proposals/naming/README.md) | Package addresses, exports and references. |
| [proposals/consolidation/](proposals/consolidation/README.md) | CS/CC audit of overlapping notations and meanings. |
| [proposals/conventions/](proposals/conventions/README.md) | Shared conventions. |
| [proposals/schemas-and-interface/](proposals/schemas-and-interface/README.md) | Documents, contracts and the implementation interface. |
| [proposals/core-and-schemas/](proposals/core-and-schemas/README.md) | Core contracts, schemas and application choices. |
| [proposals/docs-audit/](proposals/docs-audit/README.md) | Supporting-document audit and gap register. |
| [proposals/ruling-38/](proposals/ruling-38/README.md) | Ruling 38: the open points and the coordinator's least certain decisions |
| [proposals/ruling-39/](proposals/ruling-39/README.md) | Ruling 39: the open points before the cold read by Codex |
| [proposals/ruling-40/](proposals/ruling-40/README.md) | Ruling 40: Conduit innovates on connection and access; everything else is swappable |
| [proposals/principles-review/](proposals/principles-review/README.md) | Provider boundary and the reduced secret contract. |
| [proposals/secrets/](proposals/secrets/README.md) | Secret-type options and reasoning. |
| [proposals/ruling-41/](proposals/ruling-41/README.md) | Ruling 41: the review of the whole design, and the secret type |
| [proposals/vocabulary-audit/](proposals/vocabulary-audit/README.md) | Twenty names and their precedents. |
| [proposals/ruling-42/](proposals/ruling-42/README.md) | Ruling 42: the questions after the rewrite, and the names |
| [proposals/multi-machine/](proposals/multi-machine/README.md) | Machine identity, reachability and fault cases. |
| [proposals/ruling-43/](proposals/ruling-43/README.md) | Ruling 43: work across machines |
| [proposals/reaudit/round-5/](proposals/reaudit/round-5/README.md) | Re-audit, round 5 |
| [proposals/reaudit/round-6/](proposals/reaudit/round-6/README.md) | Round 6 of the re-audit |
| [proposals/ruling-44/](proposals/ruling-44/README.md) | Ruling 44: the Codex read, and building next |
| [proposals/ruling-45/](proposals/ruling-45/README.md) | Ruling 45: a collaborator's two proposals, and the roadmap |
| [proposals/ruling-46/](proposals/ruling-46/README.md) | Ruling 46: the first build |
| [proposals/ruling-47/](proposals/ruling-47/README.md) | Ruling 47 — Organization and publication after v0 |
| [proposals/ruling-48/](proposals/ruling-48/README.md) | Ruling 48: what the first build's review put to the owner, and who does the work |
| [proposals/ruling-49/](proposals/ruling-49/README.md) | Ruling 49: signing in through the engine |
| [proposals/ruling-50/](proposals/ruling-50/README.md) | Ruling 50: adding machines, and the Interface as a control panel |
| [proposals/ruling-51/](proposals/ruling-51/README.md) | Ruling 51: managed machines and peer engines |
| [proposals/ruling-52/](proposals/ruling-52/README.md) | Ruling 52: explicit refusals, submissions, enrollment's decisions and the measurement plan |
| [proposals/ruling-53/](proposals/ruling-53/README.md) | Ruling 53: the simplification's first decisions |
| [proposals/ruling-54/](proposals/ruling-54/README.md) | Ruling 54: agents stay in their project, and revocation is individual |
| [proposals/ruling-55/](proposals/ruling-55/README.md) | Ruling 55: the short form, and the old inconsistencies |
| [proposals/ruling-56/](proposals/ruling-56/README.md) | Ruling 56: how a person checks a joining machine |
| [proposals/ruling-57/](proposals/ruling-57/README.md) | Ruling 57: historical documents may be edited and condensed |
| [proposals/ruling-58/](proposals/ruling-58/README.md) | Ruling 58: how the repository is condensed, and the form of ruling records |
| [proposals/ruling-59/](proposals/ruling-59/README.md) | Ruling 59: the structure of the agent-owning family |
| [proposals/sign-in/](proposals/sign-in/README.md) | Root sign-in and account checks. |
| [proposals/enrollment/](proposals/enrollment/README.md) | Machine admission, networking and verification research. |
| [proposals/simplification/](proposals/simplification/README.md) | Authoring costs and alternatives. |
| [proposals/language-shape/](proposals/language-shape/README.md) | Short form and Agent ownership investigations. |
| [proposals/first-build/](proposals/first-build/README.md) | Build index, findings, settlements and evidence. |
| [proposals/condensation/](proposals/condensation/README.md) | Condensation plan and critic. |
| [proposals/metrics/](proposals/metrics/README.md) | Proposed accounting for Fleets, Pools, Agents and Resources. |
| [proposals/ruling-60/](proposals/ruling-60/README.md) | Ruling 60: the agent-owning family is a Fleet |
| [proposals/ruling-61/](proposals/ruling-61/README.md) | Ruling 61: the authority of a Fleet |
| [proposals/ruling-62/](proposals/ruling-62/README.md) | Ruling 62: five smaller points of the Fleet brief |
| [proposals/ruling-63/](proposals/ruling-63/README.md) | Ruling 63: how the S1 rerun is counted, the documented style, and package templates |
| [proposals/ruling-64/](proposals/ruling-64/README.md) | Ruling 64: organization and repository |
| [proposals/ruling-65/](proposals/ruling-65/README.md) | Ruling 65: the public name is ConduitLang |
| [proposals/ruling-66/](proposals/ruling-66/README.md) | Ruling 66: scoped Interfaces |
| [proposals/ruling-67/](proposals/ruling-67/README.md) | Ruling 67: decisions in force, a ledger, and records that say why |
| [proposals/owner-direction/](proposals/owner-direction/2026-11-10-projects.md) | The owner's direction for Agent ownership. |
| [proposals/projects/](proposals/projects/brief.md) | Fleet questions, constraints and ruling sources. |

### Process for the builds

| Location | What to find |
| --- | --- |
| [implementation/FIRST_BUILD.md](implementation/FIRST_BUILD.md) | First-build increments, tasks and measurements. |
| [implementation/plan-and-apply/](implementation/plan-and-apply/README.md) | The CLI reference for `conduit plan` and `conduit apply`, with golden cases that do not bind (spec batch 1, K3). Being written by work order 069. |
| [implementation/SAFETY_CATALOGUE.md](implementation/SAFETY_CATALOGUE.md) | The safety promises of `spec/`, and the safety promises each of S1's safety traces tests, for the safety vectors and model checking (A40, G5). Being written by work order 100. |
| [implementation/PROVIDERS.md](implementation/PROVIDERS.md) | The first and second provider of each provider contract, their contract vectors and the engine work they need (A63; work order 120). |
| [engine/](../engine/README.md) | The first build, runnable, and its [conformance choices](../engine/CONFORMANCE.md). |
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
| [archive/](archive/README.md) | Removed-file index, attribution, dissent and the plan audit's history. |

## Repository boundaries

The [contributor guide](../CONTRIBUTING.md#project-ownership-and-publication) records repository,
package, license and publication rules. The specification alone determines behavior.

## Maintaining the design

Follow the [contributor rules](../CONTRIBUTING.md#keeping-decisions-and-documents-current) for
rulings, extractions, removals and condensation at close. Work state belongs in AUDIT.md and the
roadmap; decision state belongs in IN_FORCE.md and the ledger.
