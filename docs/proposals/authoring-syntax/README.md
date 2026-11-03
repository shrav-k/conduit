# Deferred authoring and syntax proposal

Start at [Author with the protocol](../../authoring/README.md) for the accepted semantic path.
Remaining review work is consolidated in [REVIEW](../../REVIEW.md); this packet preserves examples
for later syntax discussion and adds no second active question queue.

Status: **ready for the next syntax/conformance review; examples remain proposals**, 2026-11-02. The
[conceptual closure review](../../archive/reviews/CONCEPTUAL.md) is complete. The published-contract and no-primary-plugin
rulings remain accepted. Templates now use one named dependency map and alias-plus-export references
for setup, runtime and controls, with kind/contract checks and no primary alias. The specific
`spec.plugins` / `{plugin, name}` spellings remain proposals.
Moving these examples does not ratify their names, package layout or scoping rules.

The accepted package entry boundary uses one explicit document declaring exports and dependencies,
with inline or package-local referenced definitions. It may be handwritten or generated; inspection
does not execute plugin code or infer exports from directory layout. Provider-native catalogs remain
dynamic. Concrete filenames, fields and reference notation in this packet remain proposals.
Public export names are unique across contribution kinds within a package, regardless of file layout.
Resolution still checks the expected kind and contract. Internal names, contract identities and
registry object names retain their own rules; exact name grammar remains open.
Packages may explicitly re-export a declared dependency's public contribution under their own export
name without copying or wrapping it. Preserve the original definition, contract, internal scope and
resolved dependency identities; the alias adds no behavior or access. Exact notation remains open.

The first authoring ruling now accepts [package-local scope and explicit dependency bindings](../../protocol/MODEL.md#package-scope-and-public-bindings).
Caller-local aliases cannot override a package's internal references; replaceable dependencies must
be explicitly exposed. Those dependencies may supply default implementation bindings, with compatible
explicit replacements taking precedence and invalid replacements producing errors rather than fallback.
V0 imports require explicit support for the requested contract identity and exact version in addition
to existing compatibility checks; cross-version mappings are deferred. Implementation release versions
remain independent and no backward-compatibility maintenance obligation is added.
Constrained YAML 1.2, equivalent JSON input/interchange and JSON Schema 2020-12 data contracts
are also accepted in the [configuration foundation](../../protocol/CONFIGURATION.md#configuration-language).
The remaining representations and detailed schema profile below are proposals within those boundaries.
The [graph-input boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is also accepted: v0 inputs
use literals and typed references to configuration or earlier outputs, while plugin operations
perform calculations and transformations. Inline binding expressions are deferred. References are
recognized only in designated binding positions; literal payloads and resolved plugin results remain
data rather than being recursively reinterpreted. Explicit object/list assembly from literals and
references is accepted without a plugin merely to assemble the structure. It adds no automatic merging
or inline computations. Exact syntax and literal disambiguation remain open; the examples do not approve them.
Configuration references use values captured when each graph execution begins. Later executions use
then-current settings; capture does not freeze permissions, dependency validity or control revision
checks. Retry/recovery must still revalidate retained results. Exact representation remains open.
Reusable graph-backed operations receive caller configuration through declared typed inputs, without
implicit configuration inheritance. Scoped invocation context and authorized Resource access retain
their own contracts; exact input/context notation remains open.
The [input-validation boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: catch
provable wiring/type errors early and validate actual resolved inputs before each step's dispatch.
V0 requires no proof of complete producer-to-consumer schema inclusion; invalid actual inputs block
the receiving step, with prior effects/evidence preserved. Required schema support remains mandatory;
checking algorithms and diagnostics remain open.
Declared plugin outputs are also validated against their producer's contract before downstream use.
Invalid output is withheld with a contract error, preserving effects/uncertainty rather than claiming
rollback or safe replay. Downstream input checks remain separate, and provider-native schema semantics
are unchanged. This is accepted behavior; exact error/result notation remains open.
After a setup/change graph step fails, independent steps continue when their own prerequisites and
current authority remain satisfied. Unmet prerequisites block affected work, required unfinished
setup remains incomplete, and cancellation/stop rules still apply. No automatic retry or rollback
follows. Detailed scheduling mechanics remain open within this accepted policy.
Explicit graph cancellation stops new ordinary steps and requests supported cancellation of
in-flight children. Track actual outcomes and required cleanup; cancellation does not prove effects
stopped or replace instance lifecycle rules. Exact propagation/transport mechanics remain open.
The accepted [schema-resolution rule](../../protocol/CONFIGURATION.md#schema-resolution) uses
the selected package and supplied/declared schema dependencies, reporting missing definitions
without automatic network fetching during validation. Dependency acquisition remains explicit.
Conduit-authored schemas also [reject unrecognized keywords](../../protocol/CONFIGURATION.md#schema-keyword-acceptance),
with additional custom descriptive annotations in one dedicated map of namespaced entries.
Reuse standard descriptive annotations where applicable. Custom hints do not implement validation,
permissions or runtime behavior; the map keyword, namespace syntax and value schemas remain open.
Provider MCP schemas [retain their declared dialect's semantics](../../protocol/CONFIGURATION.md#provider-mcp-schemas);
allowed descriptive annotations do not alone reject a tool, and unsupported required validation
features are reported without stripping constraints or claiming enforcement. Adapter details remain open.
Formats in Conduit-authored schemas are enforced. V0 selects all 19 JSON Schema 2020-12 standard
formats with offline syntactic checks and conformance tests before support claims. Unsupported
formats are reported; plugin-defined validators are deferred. Supported keywords/reusable schemas
can still describe custom constraints, and operations, Resource contracts and requirement checks
remain extensible.
The accepted [default-conflict rule](../../protocol/CONFIGURATION.md#schema-default-resolution)
requires an explicit valid value or corrected defaults when an omitted field has conflicting
applicable schema defaults. Traversal order cannot select a winner.
The accepted [integer range](../../protocol/CONFIGURATION.md#numeric-interoperability) is
−9,007,199,254,740,991 through 9,007,199,254,740,991 for Conduit-declared configuration, inputs
and outputs. Larger exact values can use an explicitly string-typed contract; implicit conversion
and rounding into range are forbidden. Fractional semantics and core revision/ID encodings remain open.
Resolved package artifacts retain [package identity and a content digest, plus release identity when released](../../protocol/CONFIGURATION.md#implementation-selection),
covering declarations and any bundled code, including template-only packages. Digest
algorithms, bundle boundaries and concrete fields remain open; this does not establish publisher trust.
Managed execution of local packages uses retained immutable snapshots, while direct handler tests
may use editable source. Explicit managed test/start workflows can capture a revision automatically;
the running instance preserves its code/contracts without archiving every edit-mode revision.
Deliberate preservation requires a versioned publication of fixed package contents through a public
or private distribution source. Conduit need not host or publish releases; the mechanism remains open,
and a local-only release facility is not selected. Changed released contents require a new version; draft
version fields or test captures alone are not releases. Ordinary development replaces affected
instances without requiring two active versions or publication per edit. Missing selected code
blocks affected restart/recovery rather than being replaced
silently. Included files/directories are explicitly declared, with directory/pattern rules available;
capture does not default to the
whole working directory. Captured paths and symlink targets must remain within one explicit
source directory; outside targets produce an error rather than implicit inclusion. Exact
inclusion syntax, internal symlink representation and cleanup remain open, and existing instances
do not adopt edited source automatically. Resource paths/runtime access remain separate.

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

Choose API/type/capability version identifiers, the detailed schema profile, supported representation rules,
reference scope and MCP/profile bindings together. Define executable conformance examples from
accepted behavior, including rejected/unsupported and uncertain cases. Do not let fixture contents
automatically become protocol requirements. Update Experiment 001's proposed YAML and Experiment
002's test plan with explicit changes and evidence when that pass happens.

Nothing in this folder is an installable plugin package or an implemented runtime. Existing static
checks prove only the properties they actually tested. This reorganization adds no runtime evidence.
