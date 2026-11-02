# Deferred authoring and syntax proposal

Start at [Author with the protocol](../../authoring/README.md) for the accepted semantic path.
Remaining review work is consolidated in [REVIEW](../../REVIEW.md); this packet preserves examples
for later syntax discussion and adds no second active question queue.

Status: **ready for the next syntax/conformance review; examples remain proposals**, 2026-11-02. The
[conceptual closure review](../../archive/reviews/CONCEPTUAL.md) is complete. The published-contract and no-primary-plugin
rulings remain accepted; the most recent `spec.plugins` / `{plugin, name}` proposal was not accepted.
Moving these examples does not ratify their names, package layout or scoping rules.

The first authoring ruling now accepts [package-local scope and explicit dependency bindings](../../protocol/MODEL.md#package-scope-and-public-bindings).
Caller-local aliases cannot override a package's internal references; replaceable dependencies must
be explicitly exposed. Those dependencies may supply default implementation bindings, with compatible
explicit replacements taking precedence and invalid replacements producing errors rather than fallback.
The representations below remain proposals within that accepted boundary.

Before resuming syntax rulings, read the [language/protocol research](../../research/language-protocol-foundations/README.md)
and [worked comparisons](../../research/language-protocol-foundations/WALKTHROUGHS.md).
WIT/Smithy import boundaries, CWL composition and YANG controls provide concrete alternatives;
their recommendations do not silently amend this packet or the accepted protocol.

| Document | Preserve it for |
| --- | --- |
| [AUTHORIZATION](AUTHORIZATION.md) | Accepted requirements/grants illustrated with unapproved representation and enforcement details, moved here from the runtime packet. |
| [AUTHORING](AUTHORING.md) | Small-handler, managed-capability, code-optional MCP and composite-operation author journeys; illustrative APIs. |
| [CONTRACTS](CONTRACTS.md) | Proposed schemas, graph/output bindings, operation context and managed MCP mappings. |
| [PACKAGE_WALKTHROUGH](PACKAGE_WALKTHROUGH.md) | One history package from publication through setup/runtime/controls, applying accepted package scope while leaving binding/default syntax open. |

These documents include useful conceptual examples, but the [design](../../protocol/README.md) is the source
of accepted semantics. Earlier question sequences remain in Git rather than another active queue.
The [implementation proposal](../../implementation/README.md) preserves runtime options separately.
The [post-migration review](../../REVIEW.md) is complete: lightweight delivery, no backward-compatibility
promise, contexts as ordinary Resources, Apache-2.0, deferred public naming and owner-led authority
are recorded. These decisions do not approve the package/export/schema spellings in this packet.

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
