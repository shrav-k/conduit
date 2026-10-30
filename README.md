# Conduit

Conduit's working model has five concepts: **Profiles, Sessions, Resources, Pools,
and Conduits**. Their interactions define the system.

- **Profiles** are reusable templates for creating Sessions.
- **Sessions** are durable, independent participants with private context and resolved settings.
- **Resources** are independently defined data or capabilities.
- **Pools** collect references to Resources and supply them to Sessions through Conduits.
- **Conduits** establish authorized resource access and communication between Sessions.

Profiles reference default Conduit declarations. Each Session establishes its own
bindings to Pools and other Sessions. Resources can be reused across Pools; membership,
access and activation have distinct meanings.

Conduit owns its runtime in Rust. Its first frontend is a custom terminal inspired by
Pi. Providers, external agent runtimes, native skills, MCP services, trackers and memory
backends connect through replaceable integrations. Pi supplies design evidence and
potential UI components; Conduit owns execution and shared contracts.

**Status:** documentation and contract design; implementation has not started. The first
experiment targets four concurrent Sessions across two Claude and two Codex/OpenAI
subscription identities, with separate private state and selected shared resources.
Subscription compatibility and the communication contract still need validation/design.

Read in order:

1. [SPEC.md](SPEC.md) — the five-concept model, interactions and intended outcomes.
2. [DESIGN.md](DESIGN.md) — accepted contracts organized by those concepts.
3. [REVIEW.md](REVIEW.md) — open questions, research and experiment planning within the model.

[RESEARCH_PI.md](RESEARCH_PI.md) contains supporting source evidence and proposed
adaptations. Research proposals are not requirements until incorporated into the design.
Update these documents in place; Git history preserves earlier designs. Keep resolved
contracts out of the open-question list and settle schemas before implementation.
