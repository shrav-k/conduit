# Conduit

Conduit is an extensible protocol and configuration language for defining capabilities,
establishing instances and connecting independent agents to shared resources and one another.
Pools own Resources and shared service authentication; Conduits expose permitted access;
Interfaces provide unified control and observability.
Conduit templates compose reusable typed requirements, with a small base vocabulary and author
extensions; endpoint-selected plugins provide compatible behavior and manage their local bindings.

The language and public contracts are the primary deliverable. A Rust reference implementation,
replaceable frontends and a thin set of plugins/templates will demonstrate them. Richer integrations
can be maintained as supported ancillary packages. The project is in protocol design;
there is no runnable Conduit implementation yet.
Plugins publish contracts backed by authored code, existing MCP services or both. Components
can expose supported MCP capabilities independently, with an Interface providing the overall view.

- [Documentation map](docs/README.md)
- [Specification](docs/SPEC.md) — the five-family model and core behavior
- [Design contracts](docs/DESIGN.md) — detailed accepted rules
- [Review](docs/REVIEW.md) — completed conceptual review and deferred authoring work
- [Repository review and transition plan](docs/proposals/REPOSITORY.md) — preserve the design while preparing protocol documentation, authoring and implementation
- [Implementation proposal](docs/proposals/implementation-v0/README.md) — architecture, contracts, organization and decisions for review
- [Collaborator proposals](docs/proposals/COLLABORATOR_REVIEW.md) — comparison with the current design
- [Deferred authoring examples](docs/proposals/authoring-syntax/README.md) — future PR input
- [Concrete YAML walkthrough](experiments/001-shared-access/README.md) — static, proposed syntax
- [Research](docs/research/README.md) — evidence, alternatives and source limitations

The conceptual review is complete. Repository, publication and documentation boundaries are now under
review; concrete syntax and conformance fixtures return in a later PR.
The earlier design baseline remains available in Git history at `c013f0a`.
