# Configuration research and proposed object model

2026-10-30. **Proposal for review; no schema or field names are accepted by this document.**
The five concepts and Profile-owned activation are already accepted. This report proposes
how to express them without turning Conduit into another deployment framework.

Read alongside [the Pi re-review](REFINEMENTS.md) and the
[complete illustrative YAML](five-concepts.proposal.yaml). The YAML has synthetic paths,
accounts and endpoints; it is an authoring example, not executable configuration.

## What to borrow

| Source | Useful mechanism | Proposed Conduit adaptation |
| --- | --- | --- |
| [Kubernetes objects](https://kubernetes.io/docs/concepts/overview/working-with-objects/) | Versioned kinds, metadata, desired `spec`, observed `status`. | One envelope across Profile, Session, Resource, Pool and Conduit. YAML authoring and API/CLI inspection represent the same objects. |
| [API conventions](https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md) | Stable UID, opaque concurrency version, desired-state generation and status conditions. | Separate durable identity, configuration changes and observed preparation/execution. Never treat a file path as identity. |
| [Labels/selectors](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/) | Optional metadata and composable set selection; empty-selector behavior is defined by its use. | Pools use refs plus explicit selectors. Conduit filters narrow the resulting candidates; labels do not grant access. |
| [CRD versioning](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definition-versioning/) | API versions can differ from storage versions, with explicit conversion/migration. | Version public schemas separately from storage, plugin releases and provider protocols. Start with one supported authoring version. |
| [CRD validation](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) | Structural schemas and extension subtrees. | Validate core fields strictly; give plugin configuration a named, versioned schema. Prefer a visible typo error to silently pruning unknown configuration. |
| [Gateway API ReferenceGrant](https://gateway-api.sigs.k8s.io/reference/api-types/referencegrant/) | A reference alone does not establish cross-boundary authority; owner approval and runtime reevaluation are separate. | Conduit declarations request relationships; authorized establishment supplies grants. Filters never manufacture permission. No extra top-level Grant concept is necessary to learn this distinction. |
| [Compose include](https://docs.docker.com/reference/compose-file/include/) | Modular files retain a source directory for relative paths. | Explicit configuration roots, source-relative paths and duplicate-identity errors. Avoid discovery determined by the launching terminal's directory. |
| [Compose merge](https://docs.docker.com/reference/compose-file/merge/) | Merge behavior is field-specific, with important sequence exceptions. | Define replacement/addition explicitly; do not inherit a generic YAML deep-merge algorithm for permission lists. |
| [Terraform provider aliases](https://developer.hashicorp.com/terraform/language/block/provider) | Multiple configurations can use the same provider implementation. | Distinct local account/endpoint bindings reuse connector code; avoid implicit provider/account fallback. |
| [Terraform state](https://developer.hashicorp.com/terraform/language/state/purpose) | Configuration addresses and actual object identity need a mapping. | Imported/discovered Resources record native identifiers and management status. Removing a declaration is not permission to delete the external object. |
| [JSON Schema](https://json-schema.org/understanding-json-schema/reference/object) | Required fields, closed objects and extensible structured subtrees. | Publish schemas for editors and validation; cross-object authority/compatibility still requires semantic validation. |

Kubernetes uses controllers to repair desired-state drift, including replacing failed
workloads. Conduit must preserve its explicit-resume rule for stopped executions. Applying
configuration can resolve bindings or show a preparation plan without starting model work.
Likewise, [owner references](https://kubernetes.io/docs/concepts/overview/working-with-objects/owners-dependents/)
have deletion consequences in Kubernetes; they are the wrong representation for Pool membership.

Full server-side field ownership, admission webhooks, Kubernetes namespaces/RBAC, Helm,
Terraform planning engines and a configuration programming language are unnecessary v0
dependencies. Start with explicit objects, validation, resolution and inspection.

## Proposed envelope and versions

```yaml
apiVersion: conduit.example/v1alpha1
kind: Pool
metadata:
  name: frontend-toolkit
  labels:
    domain: engineering
spec:
  members:
    refs:
      - kind: Resource
        name: frontend-skills
```

`conduit.example` is a deliberately reserved illustrative group, not a domain to adopt or
an asserted production identifier. Recommend choosing one actual API group and a Kubernetes-
style `v1alpha1 → v1beta1 → v1` progression when the schema is accepted. Use one core group
for the five kinds initially. Namespaced Resource/Conduit contract identifiers can evolve
independently without inventing a new top-level kind for every integration.

Keep these versions separate:

| Value | Meaning |
| --- | --- |
| `apiVersion` | How to interpret the authored/exported object. |
| Resource/Conduit `contract` version | The payload/operation contract implemented by a plugin or built-in adapter. |
| `metadata.uid` | Registry-assigned durable identity, stable across an explicit rename. |
| `metadata.resourceVersion` | Opaque optimistic-concurrency token for an object update. |
| `metadata.generation` | Which desired configuration the system should evaluate. |
| Recorded source revision/digest | Which skill, document, model catalog or external object was actually used. |
| Provider/MCP version | Wire compatibility negotiated with an external service. |
| Plugin/runtime release | The executable implementation producing behavior. |

The reference distinction between generation, resourceVersion and conditions is useful;
Conduit need not implement Kubernetes' full watch/storage behavior. A stored knowledge
record's immutable content revision is also not automatically the configuration object's
resourceVersion. [API concurrency](https://kubernetes.io/docs/reference/using-api/api-concepts/)

For v0, recommend one local registry, names unique by kind, with duplicate definitions
rejected. Human-authored references use kind/name; resolution records the UID and configuration
generation. A deliberate rename preserves UID through the registry. Deleting/recreating the
same name produces a different UID and must not silently retarget an existing binding.
Cross-registry naming/namespaces can follow when sharing/distribution needs them.

Import/export must distinguish “create a new object from this configuration” from “update
this existing object.” Exporting a Session snapshot must not make applying it restart work,
clone credentials or create a duplicate execution. Session creation is normally a CLI/API
operation; a YAML file per conversation is optional.

## How the five objects connect

### Profile and Session

A Profile describes bootstrap runtime/model/account preferences, default Conduit references,
activation rules and limits. Account references are local credential registrations, not
secrets or an additional top-level `Account` kind. These bootstrap choices do not grant the
agent access to every Resource associated with an account. Resource content still arrives
through Pools and Conduits; reusable instructions/settings bundles do not bypass that route.

The Session resolves one Profile plus explicit overrides and records the snapshot and
origins. Its runtime can construct a preparation plan before inference. Model/account/runtime
changes during work should establish a recorded execution boundary; behavioral Profile edits
do not silently change current Sessions. Live limits remain authoritative.

Proposed Session desired configuration contains `profileRef`, explicit overrides and selected
Conduit declarations. Observed state contains the resolved Profile generation, configuration
digest, runtime/account identity evidence, independent binding IDs, preparation results,
lifecycle, execution state and capability coverage. Avoid independently editable binding
lists on both Session and Conduit objects.

Recommend making the **Session the authority for its binding instances**. A Conduit object
is a reusable declaration; the Session status contains its actual binding references. Other
objects expose reverse indexes/views, not a second competing source of truth. This fits a
Profile instantiated four times without duplicating the declarations.

### Resource

Use a common envelope with an extensible `spec.contract`, `spec.connector` and plugin-specific
`spec.config`. These illustrative names identify a contract and its implementation; they do
not introduce new core concepts. Unknown plugin payloads can be retained for inspection but
must be reported unavailable for activation until their schema/implementation is available.

Keep generic metadata small. Do not hardcode every possible Resource category into Pool,
Profile or Conduit schemas. Compatibility, operations and native schemas can be declared or
discovered through the Resource's contract; labels remain optional organizational metadata.
The trust/source of discovered capability claims needs recording.

A Resource can represent a provider service or collection rather than one YAML object per
ticket, file or tool. For example, an MCP connection Resource exposes native tool identities
and resource URIs beneath it. Filtering and activation can select those sub-capabilities
through that contract. Stable container identity must not hide replacement of a native tool's
schema, source or auth boundary.

Explicit Resources exist independently of Pools. Prefer top-level declarations initially;
defer inline objects unless their identity/update semantics justify the extra syntax. Discovery
may generate registry entries without requiring the user to hand-author them. Imported folders,
repositories and remote objects retain attachment/management status.

### Pool

Proposed membership is the deduplicated union of explicit references and label-selector
matches within the selected registry. Labels belong on Resources because selection examines
the objects being collected; a Pool's own labels describe the Pool. No labels are required
when explicit references suffice. Defer Pool nesting initially.

Recommend standard `matchLabels` plus `matchExpressions` (`In`, `NotIn`, `Exists`,
`DoesNotExist`), with AND inside one selector and OR between selectors. Missing selector
means no selector-derived members. Reject an empty Pool selector rather than accidentally
collecting everything; an intentional all-members mode should be explicit. Missing refs stay
visible as unresolved members. Membership cannot read private payloads merely to evaluate labels.

**Proposed change policy:** membership matches update live; newly admitted Resources become
available, not automatically activated. Removing membership retracts that path on future
managed use, while independent paths still apply. Profile activation results record which
versions were prepared; replacement requires the defined refresh/reconfigure boundary.
Deleting a Pool removes the grouping/path, not the member Resources or underlying data.

### Conduit

A declaration specifies a versioned interaction contract and endpoints, requested operations
and narrowing filters. Session binding supplies authenticated identity and resolved authority.
For a resource Conduit, the target is a Pool. For a communication Conduit, the target can be
a Session or an explicitly resolved set; fan-out/recipient selection still needs a ruling.

In the example, `source: {session: self}` is a template binding placeholder, not an identity
the caller may assert over the wire. Actual sender identity comes from the authenticated
Session execution. `access.requested` expresses what binding asks to use; authoring that YAML
does not grant it. The binding resolver intersects it with established authority, Profile
limits and exclusions. A local user's explicitly authorized setup may establish those grants
without a confirmation for each permitted call.

Recommended access calculation for each binding:

```text
Pool candidates
  ∩ binding's authorized resources and operations
  ∩ Conduit selection/filter
  ∩ live Profile limits
  − explicit exclusions
= usable capabilities for this binding
```

Combine **usable capabilities**, not loose fields from unrelated bindings. If binding A
permits reading `src/**` and binding B permits writing `docs/**`, their union must not grant
writing `src/**`. Keep Resource, operation and constraint together as a capability tuple.
A filter can narrow one path without globally denying another independent authorized path;
use an explicit exclusion/limit for a global denial. This needs concrete tests before schema
approval because naïve merging would change the meaning of the accepted rules.

Resource identity, operation, native sub-capability and supported path constraints belong
in filters. Activation actions, package installers, prompt budgets and load order stay in
the Profile. Transport choice and backend wiring are not filters either.

## Profile activation proposal

The YAML example uses a named list of structured preparation steps with explicit actions,
selectors and required/optional handling. Free-text guidance can coexist, but cannot stand
in for a successful installation, tool registration or access check. Actions are extensible
through compatible adapters rather than an arbitrary embedded workflow language.

Recommended initial behaviors:

| Operation | Proposed behavior |
| --- | --- |
| Advertise skills | Load permitted metadata; full skill text remains on-demand. |
| Register tools | Prepare permitted connection/native schemas; defer model exposure where supported. Check dispatch separately. |
| Load selected instructions | Read permitted Resource revisions into the context budget; record what was loaded. |
| Prepare a workspace | Attach/mount through the compatible execution adapter without implying deletion authority. |
| Unsupported native extension | Report unsupported; never equate compatible model family with compatible runtime hooks. |

All activation matching is evaluated over the **already permitted inventory**. A selector
here chooses preparation targets; it cannot reach another Pool or change the Conduit's
permissions. When multiple steps conflict, report the conflict or use an explicit preference;
do not choose by filesystem scan order. Distinguish downloaded, registered, advertised,
loaded into context and actually invoked in the observed result.

## Resolution, updates and YAML semantics

The proposed v0 loader reads explicit local configuration roots, with optional predictable
directory conventions. Directory placement is organization, not identity or implicit grants.
Multiple files and multi-document YAML are sufficient for modularity; a general include,
template or inheritance language can wait. Relative paths resolve against the declaring file
or the native package's own base, with the resolved base recorded. Moving a config file can
therefore change a relative path and should show in the resolution diff.

Use a documented [YAML 1.2](https://yaml.org/spec/1.2.2/) subset that maps cleanly to JSON:
string keys, unique mapping keys, finite numbers and no executable/custom tags. Reject
duplicate keys, unsupported fields and cyclic/unbounded alias expansion. Do not silently
interpret `yes`/`no` as booleans, evaluate shell strings or expand arbitrary environment
variables. Credentials resolve through explicit references, not string interpolation.
Namespaced plugin config is validated by its declared schema.

Recommended precedence:

1. Resolve one Profile's behavioral defaults.
2. Apply explicit Session behavioral overrides; required limits cannot be weakened.
3. Select Conduit declarations. For review, propose a visible replacement mode or explicit
   add mode across Profile defaults and launch-selected Pool defaults; do not silently union
   the two. The existing rule that explicit choices replace Pool defaults still holds.
4. Resolve membership, authority and filters; then prepare Profile activation.
5. Persist the resolved configuration and its sources; show changes before applying an
   explicit reconfiguration. No mandatory extra approval is implied for authorized changes.

Permission/filter lists should not inherit Compose-style sequence appending. Prefer whole-
list replacement for overrides plus a separate explicit add operation. Named activation steps
can support targeted edits later, once their merge semantics are defined. `null`, omission
and `[]` must have distinct documented meanings wherever they are accepted.

Recommend local atomic validation/commit of an authored configuration batch, with generation-
checked edits and a resolution preview. External preparation is not one distributed transaction:
record each installation/connection outcome and partial availability. An installation already
performed does not become undone merely because a later MCP connection failed.

Status conditions should separately express validity, reference resolution, authorization,
compatibility, preparation and runtime readiness, with the evaluated generation. A single
`ready: true` cannot say whether a model is entitled, a token is fresh, a tool is available or
an OS restriction is enforced. Preserve unknown states. Avoid copying credentials or full
private prompt content into status exports.

## Failure matrix for the example

| Change/failure | Expected result under the proposal |
| --- | --- |
| Same skill Resource appears in two Pools | One identity, both membership paths; compatible identical activation deduplicates. |
| Resource has no labels but is explicitly referenced | Included normally. |
| Profile activation matches a Resource outside its Conduits | No match; no access expansion. |
| New Resource acquires `domain: application` | Live Pool membership can change; authority/filter still checked, no automatic prompt injection. |
| Permission label or requested operation edited by an unauthorized actor | No new authority merely from configuration content. |
| Explicit Session Conduit list is empty | In explicit replacement mode, no default bindings; bootstrap settings alone do not expose Pool resources. |
| Resource-specific read/write grants arrive through different paths | Union of constrained capabilities, not a resource/operation cross-product. |
| Two skill resources expose the same command name | Visible conflict until a preference/alias is selected. |
| Profile instructions change | New Sessions get them; existing Sessions retain resolved behavior until reconfigured. |
| Live Profile limit is narrowed | Future managed calls obey it immediately; isolated non-revocable access stops execution. |
| A required source revision cannot be fetched | Required preparation fails; no silent substitution. Optional preparation can be skipped visibly. |
| Pool deleted | Bindings lose that access path; underlying Resource content remains. |
| Referenced name deleted and recreated | Different UID; existing binding does not silently follow it. |
| Model account refresh fails | Show auth failure; no API-key or alternate-account fallback. |
| Communication peer offline | Contract-specific pending/unavailable state; not an implicit new Session or model turn. |
| Observation stream falls behind | Refresh snapshot with an explicit gap; do not claim every event was consumed. |
| Tool response lost after possible effect | Preserve operation ID and unknown outcome; reconcile before unsafe retry. |
| Plugin schema missing | Retain inspectable declaration and report unsupported capability. |
| YAML duplicate key / unknown limit field | Reject with source location; do not silently choose or discard a value. |

## Scope of validation

The companion YAML passed structural checks using the installed Psych 3.1.0 parser:
15 objects, 20 resolved object references, four distinct Session/account aliases, overlapping
Pool membership and Profile-only activation. Duplicate mapping keys are checked explicitly;
a deliberately duplicated key was rejected. These are example checks, not YAML 1.2 parser
conformance, validation against an implemented Conduit schema or proof of authorization.
Synthetic source paths and accounts are intentionally unresolved at runtime. All fields
remain proposals.

The next discussion can settle this in a few groups: the object/reference envelope;
Profile activation and update rules; default-Conduit composition; and the initial Conduit
interaction families. Rust types, formal schemas and runtime code should follow those rulings.
