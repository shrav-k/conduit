# Conduit

Conduit connects independent agents to shared capabilities, knowledge and one another.
Pools own Resources and shared service authentication; Conduits expose permitted access;
Interfaces provide unified control and observability.

The project is in protocol design. There is no runnable Conduit implementation yet.
The implementation direction is Rust, with replaceable frontends and provider integrations.

- [Documentation map](docs/README.md)
- [Specification](docs/SPEC.md) — the five-family model and core behavior
- [Design contracts](docs/DESIGN.md) — detailed accepted rules
- [Review](docs/REVIEW.md) — settled boundaries and remaining schema work
- [Concrete YAML walkthrough](experiments/001-shared-access/README.md)
- [Research](docs/research/README.md) — evidence, alternatives and source limitations

Work continues on `research/conduits-as-graphs`. The earlier design baseline remains on
`main` at `c013f0a`; this branch has not been merged.
