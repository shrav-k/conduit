# Deferred authoring and syntax proposal

Status: **parked for a future PR**, 2026-11-02. The
[conceptual closure review](../../archive/reviews/CONCEPTUAL.md) is complete. The published-contract and no-primary-plugin
rulings remain accepted; the most recent `spec.plugins` / `{plugin, name}` proposal was not accepted.
Moving these examples does not ratify their names, package layout or scoping rules.

| Document | Preserve it for |
| --- | --- |
| [AUTHORING](AUTHORING.md) | Small-handler, managed-capability, code-optional MCP and composite-operation author journeys; illustrative APIs. |
| [CONTRACTS](CONTRACTS.md) | Proposed schemas, graph/output bindings, operation context and managed MCP mappings. |
| [PACKAGE_WALKTHROUGH](PACKAGE_WALKTHROUGH.md) | One history package from publication through setup/runtime/controls, including the unresolved package-local versus caller-alias scope. |

These documents include useful conceptual examples, but the [design](../../protocol/README.md) is the source
of accepted semantics. Earlier question sequences remain in Git rather than another active queue.
The [implementation proposal](../../implementation/README.md) preserves runtime options separately.

## Later PR scope

Review one connected contract/package/template/instance journey, including defaults, fixed
implementation identities, graph composition, controls and failures. Select syntax only after the
semantics are clear. Keep package identity, template-local aliases, published exports and provider
MCP method names distinguishable. No mandatory primary implementation is reintroduced.

Choose API/type/capability version identifiers, schema dialect, supported representation rules,
reference scope and MCP/profile bindings together. Define executable conformance examples from
accepted behavior, including rejected/unsupported and uncertain cases. Do not let fixture contents
automatically become protocol requirements. Update Experiment 001's proposed YAML and Experiment
002's test plan with explicit changes and evidence when that pass happens.

Nothing in this folder is an installable plugin package or an implemented runtime. Existing static
checks prove only the properties they actually tested. This reorganization adds no runtime evidence.
