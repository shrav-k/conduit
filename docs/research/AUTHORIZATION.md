# Authorization: policies, grants and Conduit enforcement

Research date: 2026-11-01. The user directed Conduit to formalize Policy and Grant alongside the
other concepts beneath its five object families, covering Agents as well as Interfaces. Independent
policy, broad default owner administration, current access checks and no offline grants are already
accepted. The user subsequently selected recorded grants on ordinary managed participation,
including root management; policy governs issuance/current use without a grantless alternative.
The user also selected issuer-independent grant lifetime: check authority when administering a
grant; do not preserve live parent-grant/provenance dependencies. Authorized withdrawal and the
recipient's related participation/Conduit lifetime remain effective alongside current restrictions.
The detailed structures in the [authorization proposal](../proposals/implementation-v0/AUTHORIZATION.md)
remain for joint review. This report supplies evidence, not additional rulings.

Method: read primary specifications, project documentation and research papers. No authorization
engine was installed, no cloud account was accessed and no implementation/conformance tests were
run. Provider/project facts below are sourced; Conduit implications are our analysis. MCP's latest
authorization URL resolved to the 2026-07-28 specification during this research.

## 1. The design question

The previous discussion focused on the Interface as administrator. The same authorization model
must also cover an Agent using a Resource, an Agent controlling another Agent, Resource-to-Resource
access and management operations. An Interface is not the sole place where permission matters.

Keep four distinctions:

- A policy states authoritative access/delegation rules.
- A grant records particular authority issued to a subject, under applicable rules.
- A requirement defines a guarantee that a relationship/use must satisfy, including restrictions.
- A Resource can expose management/evaluation capabilities; plugins implement those capabilities.

This is a protocol distinction, not a demand for a service/process per record or a sixth object
family. The existing [cloud comparison](CLOUD_AUTHORITY.md#9-interface-administration-and-independent-policy)
already supports policy independent of administrative-client lifecycle.

## 2. Primary-source comparison

### Cedar: typed authorization and explicit evaluation behavior

Cedar asks whether a principal may perform an action on a resource in a context. Its authorization
semantics include default denial, satisfied forbids overriding permits, and skipping policies that
evaluate with errors while returning diagnostics. A schema describes entity/action/context types
and supports policy validation. These are separate concerns: well-typed policy is not proof that
the supplied context is authentic or current.
[Authorization semantics](https://docs.cedarpolicy.com/auth/authorization.html),
[schema](https://docs.cedarpolicy.com/schema/schema.html)

**Conduit implication:** define a common typed decision input and preserve diagnostics. Do not
adopt an evaluator's defaults silently: a skipped required restriction cannot be treated as
satisfied under Conduit's existing rules. A Cedar adapter would need to handle relevant errors
and prove compatibility with our path/requirement semantics before its decision could admit work.

Cedar explicitly treats permission to change permissions as part of the authorization model. It
also documents alternative ways to represent an Agent acting for a user, depending on whose rights
the application evaluates. [Administrative permissions](https://docs.cedarpolicy.com/bestpractices/bp-meta-permissions.html),
[Agent/user identity](https://docs.cedarpolicy.com/bestpractices/bp-using-the-context.html)

**Conduit implication:** keep the managed actor, acting human/delegator where applicable and
downstream provider identity distinguishable. An Agent's access must not become the full rights
of the user who launched it. Policy administration, delegation and target use need distinct actions.

### Kubernetes RBAC: rules, bindings and delegation authority

Kubernetes separates permission rules in Roles/ClusterRoles from bindings to users, groups or
service accounts. Its RBAC rules are additive. Role/binding write permissions alone are insufficient
to confer arbitrary authority: additional rules require the caller to hold the permissions being
assigned or possess the appropriate explicit escalation/binding authority.
[RBAC objects and privilege-escalation checks](https://kubernetes.io/docs/reference/access-authn-authz/rbac/)

**Conduit implication:** distinguish reusable declarations, authorized installation and issued
bindings/grants. This supports a root or authorized Agent delegating to another Agent through a
normal managed operation. Copy the separation, not Kubernetes' object families, namespace system,
additive-only evaluation or administrator exceptions. Conduit's restrictions and whole-path
composition remain in force.

### OPA: separate decisions from application enforcement

OPA decouples policy decisions from service code and evaluates policy against structured input/data.
It can be deployed with a service as a library, sidecar or daemon. Its decision results need not be
just booleans; the application integration determines how to use them.
[OPA philosophy and integration model](https://www.openpolicyagent.org/docs/philosophy)

**Conduit implication:** a policy capability can fit the Resource/plugin model without owning the
protected Resource's lifecycle. The endpoint still has to enforce the decision and any required
obligations. Conduit needs a common result contract even if different engines implement evaluation;
arbitrary plugin output or an unverified claim of permission cannot substitute for it.

### Zanzibar: relationship state and permission freshness

Zanzibar separates authorization relationships from application data and evaluates access across
those relationships. Its consistency design addresses ordering between permission changes and
content changes, rather than treating all cached authorization as equivalent to current access.
[Paper](https://www.usenix.org/system/files/atc19-pang.pdf),
[authors' presentation and abstract](https://www.usenix.org/conference/atc19/presentation/pang)

**Conduit implication:** preserve stable identities, explicit relationship dependencies and the
authorization state used for a decision. Revocation correctness needs an admission/consistency
contract, not merely a stored boolean. The v0 local proof need not adopt Zanzibar's global storage,
relationship language or consistency-token protocol. Exact ordering across independent services
must be specified and tested before claiming a stronger guarantee than our managed-request rules.

### Macaroons and Biscuit: delegation that narrows authority

Macaroons introduce bearer credentials whose caveats restrict delegated authority by context.
The paper demonstrates decentralized delegation and attenuation as a way to share access with
additional constraints. [Macaroons, NDSS 2014](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/)

Biscuit likewise supports attenuation and public-key verification. Its specification distinguishes
signature verification from successful authorization; authorization evaluates checks and
authorizer policies. All applicable checks must succeed. Rust tooling is available.
[Biscuit specification](https://doc.biscuitsec.org/reference/specifications),
[Rust support](https://doc.biscuitsec.org/usage/rust)

**Conduit implication after the lifetime ruling:** check permitted grant extent when administering
access and retain its own restrictions. Conduit does not adopt live grant ancestry or cryptographic
attenuation chains. A grant's identity or a cryptographically valid
token is not enough to establish current permission. Offline attenuation/verification does not
provide knowledge of every subsequent revocation. V0 retains online/current verification, not
offline bearer grants. A future credential representation could use these ideas without changing
the semantic grant model; no token format or crypto implementation is selected now.

### OAuth and MCP: identity transport does not replace domain authorization

OAuth token exchange distinguishes delegation from impersonation and provides ways to represent
the actor separately from the subject. This is useful when a service acts for a user without
collapsing their identities. [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693.html)

MCP 2026-07-28 defines HTTP transport authorization; its stdio guidance differs. Protected servers
must validate their token audience. A server calling an upstream API uses separately issued
upstream authorization, rather than forwarding the token received from its client.
[MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization),
[audiences and upstream access](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations)

**Conduit implication:** preserve native MCP operations, authenticate each actual boundary and
bind the request to the actor/path Conduit knows. The shared Linear identity belongs to the
provider-facing Resource; it does not become every Agent's Conduit identity. MCP transports our
inspection, grant-management and invocation contracts but does not by itself specify Pool ownership,
Conduit participation or grant revocation semantics. A Conduit Grant is a managed authorization
record, not an OAuth grant type, access token, MCP session ID or reusable decision receipt.

## 3. What to formalize now

The protocol work should specify one model for Agents, Interfaces and other managed callers:

1. Typed Policy and Grant contracts, with stable identity/revision and distinct authorization roles.
2. A normalized request identifying actor, target, operation, selected path and relevant context.
3. A decision/evidence contract distinguishing verified permission, refusal and inability to verify.
4. Administrative operations for inspecting/changing policy and issuing/withdrawing grants.
5. Current administrative authority/bounds, independent issued grants and authorized revocation.
6. Enforcement responsibilities on endpoints, retaining the requirement contracts already accepted.

These are semantic responsibilities. The companion proposal supplies a concrete starting structure;
the user has not yet selected all fields, combining algorithms, representation or engine. Policies
and grants belong in the protocol's working vocabulary now, rather than being optional helpers.

No implication follows that a small plugin author must implement an IAM server. Authors should
declare capability operations and supported requirement contracts, receive trusted invocation
context through the runtime, and enforce the duties they explicitly support. Policy evaluation
and grant management can be shared implementations under the same published contracts.

## 4. Implementation options and limits

| Option | Why investigate it | What it does not settle |
| --- | --- | --- |
| Small built-in evaluator for initial typed grants/restrictions | Directly exercises the intended contract with a small fixture. | Designing all semantics remains our responsibility; it is not evidence of general policy-language correctness. |
| Cedar behind an evaluation boundary | Existing Rust implementation and typed policy validation. | Required-error behavior, current-state acquisition, delegation lifecycle, Resource hosting and path composition need explicit adaptation. |
| OPA integration | Established policy/data separation with flexible decision outputs. | A generic decision still needs Conduit's typed meaning, admission rules and trustworthy inputs. |
| Biscuit or another capability credential | Potential future portable, attenuated credentials. | Current revocation, no-offline-grants compliance, service enrollment and audience binding remain necessary. |

The [Cedar implementation](https://github.com/cedar-policy/cedar) is Rust, making it a relevant
candidate for a later proof. This research does not select it, install it or claim performance.
Engine, storage and transport are independent choices. Publishing a permission evaluator as an MCP
capability does not make every such service an authority: the runtime must bind the designated
trusted authority and validate responses under the selected contract.

## 5. Questions to carry into review

Resolve these within I4 before the final I7 syntax pass:

- Settled after this comparison: ordinary managed use requires a recorded grant on the selected
  participation, including root management. Policy governs issuance/current use; one grant may
  cover many requests. Establishing the first root authority still needs the startup contract.
- Settled after the dependency examples: check current authority on grant administration; issued
  grants do not depend on issuer history/permissions. Another authorized administrator may revoke
  them; recipient participation/Conduit end also ends access. No ancestry structure is required.
- Should the public policy contract be portable typed Conduit data with implementation plugins,
  or primarily engine-native policy documents? This boundary is distinct from backend selection.
- How are policy sets applied and conflicts/errors handled without dropping required restrictions
  or mixing grants across paths? Which restrictions can be proven to narrow custom conditions?
- Which management Resource capability exposes these records, and how is its first trusted
  authority established and repaired? No wrapper resolves its own bootstrap dependency.

Detailed schemas, package exports, provider-operation matching and implementation bindings remain
for I7 after these conceptual choices. A proposed extension to Experiment 002 should exercise
the same action through Interface and Agent paths, delegated access, invalid broadening, revocation,
unavailable authority and requirement failures. It is not permission to implement or enlarge the
accepted experiment automatically.
