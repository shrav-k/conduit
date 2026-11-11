Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Projects as a scope for objects and authority: how ten systems do it, and 12 lessons for Conduit

## Key points
1. In every system studied, an object has exactly one home scope, fixed or tightly controlled. Kubernetes: one namespace, immutable. GCP: "exactly one parent". Azure: one resource group. HCP: exactly one project. Sharing to several scopes is always a separate, explicit edge, never multi-homing. So "pools accessible from multiple projects" should be one home plus share edges.
2. The owner of the shared thing creates the share; the consumer's scope then grants within it. Examples: Gateway API ReferenceGrant in the target namespace; RAM's managed permission as "the maximum permissions"; Shared VPC host; GitHub org secret `selected` repos; producer-side remote state sharing. Shares are non-transitive: RAM says "can't share a resource that's shared with you".
3. A cross-scope request needs both sides to allow it: AWS performs "two evaluations", and ruling 51 has two halves. Outside a trust boundary the consumer accepts an invitation that expires (RAM: 12 hours or 7 days).
4. Membership grants nothing. Examples: ACC-028; Backstage owner "not to be used … to assign authorization"; Azure Service Groups "no RBAC inheritance"; OpenFGA parents inherit only where the model declares it. Authority over a project must be an explicit binding.
5. "Interfaces over one or many projects" is the RoleBinding, IAM-binding and Lighthouse pattern. One reusable operation set is bound per project, and each binding is created and revocable by that project's authority, not held as a list on the Interface.
6. This requires project-bounded `management`, the largest core change. Today CON-028 and ACC-030 forbid narrowing `management` (brokers only). Create rights per scope are what Kubernetes namespaces exist for.
7. Separate caps from grants. SCPs and permissions boundaries grant nothing, they limit. A project should carry a cap (its own Pools plus shared-in Pools with operation ceilings), and every bound used inside it stays within the cap.
8. A grant on a container covers future members. Examples: GCP networkUser covers "subnets … added … in the future"; Kubernetes wildcards and aggregated roles; HCP global sets; ACC-029 already does this. "All projects" and "whole project" must be explicit and shown in inspection and in the plan. The simplification's skeptic already found that an implicit Pool per project widens access through bounds.
9. A scope is only as strong as its weakest create right. In Kubernetes, the right to create pods gives every Secret and ServiceAccount in the namespace. AWS: delegating user creation "effectively trusts" the delegate. Whoever registers Agents in a project holds the combined reach of every Pool shared into it, so an auth Pool needs its own project or named consumers.
10. Confused deputy (CVE-2021-25740, AWS SourceArn/ExternalId): a broker serving several projects must carry and check the requesting project (`serving`, caller chain). An object naming another project's target needs that project's consent. Existence across projects must not be disclosed without a grant (Gateway "MUST NOT"; ruling 52).
11. Record membership on the member and compute the project's inventory; do not keep a mutable member list on the project. Kubernetes ApplySet uses a part-of label, with each object in at most one set, after allowlist prune deleted things unexpectedly. A concise project file can still list everything for the author.
12. Names: global names (today's OBJ-002) need never-reuse, as GCP project IDs are never reused, which matches ruling 53's tombstones. Scoped names need qualified references and reserved prefixes (`kube-`, the TLD collision). Never resolve collisions by precedence: GitHub's "lowest level takes precedence" is a silent shadow.
13. Revocation is immediate and respects order: ReferenceGrant deletion forces recalculation, RAM removal is automatic, and Zanzibar names the new enemy problem. Narrowing never waits; unsharing applies at decision time and leaves tombstones.
14. Approvals as pending actions: Kubernetes CSR needs an `approve` verb scoped to the signer; GitHub reviewers are declared and can be barred from self-review; GCP PAM declares approvers and expires grants. Add to ruling 51's pending action: a declared approver set, no approval beyond the approver's own authority (Kubernetes bind/escalate), no self-approval, an expiry, and a tie to the exact change (`expectedId`/revision). Avoid GitHub's default admin bypass.
15. Vocabulary and families: "project" is today's submission unit. Compose, Pulumi and Terraform separate the source from the deployed scope; Terraform CLI workspaces are "not a suitable isolation mechanism". Conduit must decide per family what is engine-scoped and what is project-scoped (Kubernetes: never both), whether projects nest (recommend flat in v0), whether membership is immutable (recommend yes), and the scope's name. The name needs a ruling.

## Findings
# 0. Where Conduit stands today (repo, read-only)

- "project" is only a term for what is handed over together in one registration (spec/definitions.md:16, :65). Nothing registered records it.
- Names are unique per kind across the whole engine, templates and instances counted together (OBJ-002, spec/promises/objects.yaml:69). There is no namespace.
- Every Resource has exactly one owner, a Pool, fixed when it is instantiated (POOL-001, spec/promises/pools.yaml:45). Ownership permits nothing (ACC-028, spec/conventions/access.yaml:535; spec/definitions.md:122).
- A bound on a Conduit to `access` names Pools, targets and operations. Naming a Pool also covers every Resource instantiated in it later (ACC-029, access.yaml:547).
- A right to open is passed on only as it is or narrower (ACC-030, access.yaml:583). A bound does not limit filters.
- `management` takes no bound, so it cannot be narrowed to one template or one scope (ACC-030; CON-028, spec/promises/conduits.yaml:430; ACC-033, access.yaml:652). The root Interface's Conduit to `access` is unbounded (ACC-022, access.yaml:473).
- The simplification work derived three CLI defaults: a Pool per project, an operator Conduit to it, and a turn Conduit per Agent (docs/proposals/simplification/research/synthesis.md:29-44). Its skeptics found that "one implicit Pool per project widens access through bounds" (synthesis.md:457). Ruling 53 left decision 6 open (docs/proposals/ruling-53/README.md:20).
- Ruling 51 settled two points (docs/proposals/ruling-51/README.md:22-27):
  - network membership grants nothing, and the target's owner authorizes each request;
  - a relationship across parties is two halves, and a pending action is generic (kind, subject, evidence, expiry, outcome).

# 1. Kubernetes: namespaces, RBAC, ReferenceGrant

**Unit of scope:** the namespace. "Names of resources need to be unique within a namespace, but not across namespaces." "Namespaces cannot be nested inside one another and each Kubernetes resource can only be in one namespace." (https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/)

**Global vs scoped:** every kind is one or the other: "a Kubernetes object always has to be either namespaced or not namespaced; it can't be both" (https://kubernetes.io/docs/reference/access-authn-authz/rbac/).
- Cluster-scoped kinds include Nodes, PersistentVolumes, StorageClass and Namespaces themselves.
- The system's own objects live in `kube-system`, and the `kube-` prefix is reserved.

**Authority per scope:**
- A Role is namespaced and a ClusterRole is not. A RoleBinding grants within one namespace, and a ClusterRoleBinding grants cluster-wide.
- A RoleBinding may reference a ClusterRole "and bind that ClusterRole to the namespace of the RoleBinding". So one definition is reused in each scope.
- "Permissions are purely additive (there are no "deny" rules)."
- `roleRef` is immutable. Two reasons: someone may edit a binding's subjects without being able to change its role, and "A binding to a different role is a fundamentally different binding."
- Aggregated ClusterRoles fill their rules from label-selected ClusterRoles. The user-facing `admin`, `edit` and `view` roles grow this way when an admin adds `aggregate-to-*` labels.

**No escalation:** "You can only create/update a role if … You already have all the permissions contained in the role, at the same scope", or you hold `escalate`. You can create a role binding only "if you already have all the permissions contained in the referenced role (at the same scope …) or if you have been authorized to perform the bind verb". Bootstrapping goes through `system:masters`, and RBAC can never revoke that group (https://kubernetes.io/docs/concepts/security/rbac-good-practices/).

**Pitfalls (rbac-good-practices):**
- Escalation inside a scope: "Permission to create workloads … in a namespace implicitly grants access to many other resources in that namespace, such as Secrets … since Pods can run as any ServiceAccount … also implicitly grants the API access levels of any service account in that namespace." Also: "namespaces should be used to separate resources requiring different levels of trust or tenancy … boundaries within a namespace should be considered weak."
- Future coverage: "wildcard access gives rights not just to all object types that currently exist … but also to all object types which are created in the future."
- `list` and `watch` on Secrets reveal their contents.

**Name collision:** a namespace named like a public TLD makes short DNS names "overlap with public DNS records". The advice is to restrict who may create namespaces (namespaces page).

**Cross-namespace ownership:** "Cross-namespace owner references are disallowed by design … A namespaced owner must exist in the same namespace as the dependent." An invalid owner reference is treated as absent (https://kubernetes.io/docs/concepts/overview/working-with-objects/owners-dependents/).

**Sharing across namespaces (Gateway API ReferenceGrant):**
- "we need to enforce a handshake mechanism that requires resources in both namespaces to agree to this reference" (GEP-709, https://gateway-api.sigs.k8s.io/geps/gep-709/).
- "the object's owner must create a ReferenceGrant … Without a ReferenceGrant, a cross namespace reference is invalid." The grant lives in the target's namespace, and `to` has no namespace field.
- Implementations "MUST NOT expose information about the existence of a resource in another namespace" without a grant.
- They "MUST watch for changes … and recalculate the validity … after each change or deletion" (https://gateway-api.sigs.k8s.io/reference/api-types/referencegrant/).
- Namespace selectors were rejected: they "may encourage unnecessarily insecure configuration".
- Core Kubernetes reuses ReferenceGrant for cross-namespace PVC data sources (https://kubernetes.io/docs/concepts/storage/volume-populators-and-data-sources/).

**Confused deputy (CVE-2021-25740, https://github.com/kubernetes/kubernetes/issues/103675):**
- A user who could write Endpoints in their own namespace could make a LoadBalancer or Ingress (the trusted deputy) forward to backends in other namespaces.
- No patch exists. The mitigation removed Endpoints write from the aggregated `edit` and `admin` roles.
- Cilium CVE-2023-34242: a missing check on which namespace a ReferenceGrant was created in leaked Secrets.

**Approval as an object:** a CertificateSigningRequest is a pending object.
- Approving it needs `update` on `certificatesigningrequests/approval` and `approve` on `signers` with `resourceName: <signerName>` or `<domain>/*`. So who may approve is itself RBAC, scoped by signer.
- Pending CSRs are deleted after 24 hours (https://kubernetes.io/docs/reference/access-authn-authz/certificate-signing-requests/).

**Set membership for prune (ApplySet):**
- "a server-side object … that kubectl can use to accurately and efficiently track set membership across apply operations." It was introduced as a replacement for allowlist pruning, which "can lead to objects being unexpectedly deleted or retained".
- Members carry `applyset.kubernetes.io/part-of`. "Each object may be a member of at most one set." Sets spanning namespaces need a cluster-scoped parent (https://kubernetes.io/docs/tasks/manage-kubernetes-objects/declarative-config/).

# 2. Google Cloud: organization, folders, projects; IAM; Shared VPC

**Unit of scope:** the project, under optional folders, under the organization. "All resources except for the highest resource in a hierarchy have exactly one parent" (https://docs.cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy).
- The project ID is "globally unique", "permanent", and cannot be reused, even after the project is deleted. The display name is free text and editable (https://docs.cloud.google.com/resource-manager/docs/creating-managing-projects).

**Inheritance:**
- An allow policy on a container "also applies to all resources in that container"; "The union of all of these policies is called the effective allow policy" (https://docs.cloud.google.com/iam/docs/overview).
- Because allow is a union, a child cannot take back a parent's grant. The explicit wording ("child policies cannot restrict") is only in a third-party note.
- Deny policies inherit downward, and "IAM always checks relevant deny policies before checking relevant allow policies" (https://docs.cloud.google.com/iam/docs/deny-overview).
- Google advises granting at the smallest scope, and at a folder when a role must cover several projects (https://docs.cloud.google.com/iam/docs/resource-hierarchy-access-control).

**Moving a project:**
- It needs `resourcemanager.projects.update` on the project and `resourcemanager.projects.move` on both the source and the destination parent.
- Directly attached policies move with the project; inherited ones do not, so its access changes (https://docs.cloud.google.com/resource-manager/docs/moving-projects-folders).

**Sharing (Shared VPC, https://docs.cloud.google.com/vpc/docs/shared-vpc):**
- A host project shares its networks with attached service projects.
  - "A project cannot be both a host project and a service project."
  - "each service project can only be attached to a single host project."
  - Both must be in the same organization.
- A Shared VPC Admin, holding the role at the organization or at the folders involved, does the attaching. So authority over both sides is required.
- `compute.networkUser` is granted on the whole host project or on chosen subnets. A project-level grant covers "subnets and VPC networks added to the host project in the future".
- The host keeps network control, and billing goes to the service project.

**Approvals:** Privileged Access Manager entitlements declare eligible requesters, roles, a maximum duration and optional approvers. Grants are revoked at the end of their duration (https://docs.cloud.google.com/iam/docs/pam-overview).

# 3. AWS: accounts, Organizations, RAM, resource policies

**Unit of scope:** the account, used as a resource container, an isolation boundary and the unit of billing and quotas. AWS uses separate accounts to "create distinct data perimeters" and to "Limit scope of impact" (https://docs.aws.amazon.com/whitepapers/latest/organizing-your-aws-environment/benefits-of-using-multiple-aws-accounts.html).

**Caps from above (SCPs):**
- "SCPs do not grant permissions … An SCP defines a permission guardrail." "Any account has only those permissions permitted by every parent above it."
- SCPs do not affect the management account, service-linked roles, or resource-based policies granting to outsiders (https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scps.html).

**Cross-account access needs both sides:** the resource policy in the trusting account and the identity policy in the trusted account. "AWS performs two evaluations … The request is allowed only if both evaluations return a decision of Allow" (https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic-cross-account.html).

**RAM, sharing by the owner (https://docs.aws.amazon.com/ram/latest/userguide/what-is.html):**
- The owning account creates a resource share listing resources, principals (accounts, OUs, the organization, roles, users) and a managed permission per resource type.
- For other accounts, that permission is "the maximum permissions available"; their administrators "must then grant individual roles and users access … can't exceed" it.
- "The resource owning account retains full ownership." Owners "can see which principals have access."

**RAM rules (https://docs.aws.amazon.com/ram/latest/userguide/working-with-sharing-create.html):**
- "You can share a resource only if you own it. You can't share a resource that's shared with you." Sharing is non-transitive.
- Inside the organization, no invitation is needed. Outside it, the consumer must accept an invitation, which expires after 12 hours, or 7 days for some resource types.
- Membership changes "dynamically affect access": an account added to a shared OU gets access, and one removed loses it.
- Pitfall: sharing with an org or OU that includes the owning account grants all of the owner's principals access, because RAM attaches `"Principal": "*"`.
- Remove a resource from shares before deleting it.

**Confused deputy (https://docs.aws.amazon.com/IAM/latest/UserGuide/confused-deputy.html):** "an entity that doesn't have permission to perform an action can coerce a more-privileged entity to perform the action."
- Fixes: an ExternalId chosen by the deputy for each customer, and `aws:SourceArn`, `aws:SourceAccount` and `aws:SourceOrgID` conditions. The resource checks on whose behalf a service principal acts.
- A resource policy that trusts the service principal with no conditions authorizes "the AWS service principal, and not the actor who configured the calling service".

**Delegated admin (permissions boundaries, https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html):**
- A delegate may create users only with a fixed boundary attached, and may not edit or remove it.
- AWS adds: "By delegating these administrative actions, Maria effectively trusts Zhang with access to Amazon S3." The delegate can create principals that reach what the delegate itself cannot.

# 4. Azure: management groups, subscriptions, resource groups, Service Groups, Lighthouse

**Scopes:** management group, subscription, resource group, resource. "Lower levels inherit role permissions from higher levels" (https://learn.microsoft.com/en-us/azure/role-based-access-control/scope-overview). A role assignment is an extension resource of its scope.

**Resource groups (https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/overview):**
- "Each resource can exist in only one resource group."
- Resources in a group "should share the same lifecycle."
- "A resource can connect to resources in other resource groups."
- "When you delete a resource group, all resources in the resource group are also deleted."

**Management groups (https://learn.microsoft.com/en-us/azure/governance/management-groups/overview):**
- "Each management group and subscription can support only one parent." Depth is limited to six levels. A root exists per directory, and policies set there "can't" be bypassed.
- A move needs write on the child and on both the old and the new parent. A move that breaks a custom role's assignable-scope path is refused.

**Service Groups (public preview, https://learn.microsoft.com/en-us/azure/governance/service-groups/overview):** a separate kind of grouping.
- "Resources can belong to multiple service groups simultaneously, and access on service groups doesn't grant access to the underlying resources." There is "no RBAC inheritance through the memberships."
- Adding a member needs Contributor on the group and `Microsoft.Relationship/write` on the resource, so both sides consent.
- Azure's own comparison table marks the resource group, subscription and management group "Not Supported" for "Grouping of resources that are shared across scope boundaries". Only Service Groups support it.

**Lighthouse (https://learn.microsoft.com/en-us/azure/lighthouse/overview):** customers delegate subscriptions or resource groups to users and roles in a managing tenant. They have "precise control over the scopes they delegate and the permissions that are allowed", can audit, and can "remove access completely at any time". One principal holds authority over many scopes, and each scope's owner grants and revokes its own part.

# 5. GitHub: organizations, repositories, teams, organization secrets

**Units of scope:** organization, then repository, then environment.
- "Organization owners have admin access to every repository owned by the organization."
- The repository roles are Read, Triage, Write, Maintain and Admin. Only Admin manages team and collaborator access (https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization).

**Org secrets shared to repositories, the closest analog to "skills pool for all projects, auth pool for one":**
- "Organization owners can create secrets or variables at the organization level."
- Visibility is `all`, `private` or `selected`, with `selected_repository_ids`. "You can only provide a list of repository ids when the visibility is set to selected" (https://docs.github.com/en/rest/actions/secrets).
- Endpoints list, set, add and remove selected repositories per secret. The owner of the shared thing does all of it; the repository does not request.

**Pitfalls (https://docs.github.com/en/actions/reference/security/secrets):**
- Precedence: "If a secret with the same name exists at multiple levels, the secret at the lowest level takes precedence." A narrower scope silently shadows a wider one.
- A repository can use only "the first 100 organization secrets (sorted alphabetically)".
- "secrets are not passed to the runner when a workflow is triggered from a forked repository."

**Approvals (https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments):**
- Environments list "up to six users or teams as reviewers", and "Only one … needs to approve".
- "Prevent self-review" is optional.
- "a job cannot access environment secrets until one of the required reviewers approves it."
- "By default, administrators can bypass the protection rules."

# 6. Terraform and Pulumi

**Terraform CLI workspaces** are "separate instances of state data inside the same Terraform working directory". They are "not a suitable isolation mechanism" for deployments with different credentials and access controls, because they share one backend (https://developer.hashicorp.com/terraform/cli/workspaces). Separate state is not separate access.

**HCP Terraform projects:**
- "Every workspace and Stack must belong to exactly one project"; there is a default project.
- Team access is granted per project and is "more granular than organization-level" access.
- Workspaces can move between projects (https://developer.hashicorp.com/terraform/cloud-docs/projects).

**Variable sets:**
- Global ("all existing and future workspaces"), for selected projects ("all current and future workspaces"), or for selected workspaces.
- Organization-owned sets and project-owned sets need different authority.
- Workspace variables override sets. "Priority" sets override more specific scopes (https://developer.hashicorp.com/terraform/cloud-docs/variables/managing-variables).

**Remote state sharing** is configured on the producer workspace: off by default, or shared with all, with the project, or with specific workspaces. But "Remote state access controls do not apply when using the tfe_outputs data source" (https://developer.hashicorp.com/terraform/cloud-docs/workspaces/state). That is two paths to the same data under different controls.

**Pulumi** separates the program (the project) from its instances (stacks), named `org/project/stack`. A StackReference reads another stack's outputs (https://www.pulumi.com/docs/iac/concepts/stacks/).
- ESC environments are shared configuration and secrets that stacks import.
- Their permissions separate `read` from `open`: "The open permission is what allows a user to decrypt secrets" (https://www.pulumi.com/docs/esc/environments/).

# 7. Backstage and Docker Compose

**Backstage (https://backstage.io/docs/features/software-catalog/descriptor-format):**
- `metadata.namespace` "has no special semantics apart from bounding the name uniqueness constraint"; the default is `default`. References are `[kind:]\[namespace/]name`, with missing parts filled from context.
- `spec.owner` is "the singular entity (commonly a team) that bears ultimate responsibility" and is "not to be used by automated processes to for example assign authorization in runtime systems."
- System and Domain group components and APIs.

**Docker Compose:**
- "Compose uses a project name to isolate environments." The name is resolved in this order: the `-p` flag, then `COMPOSE_PROJECT_NAME`, then the top-level `name:`, then the directory (https://docs.docker.com/compose/how-tos/project-name/).
- "A project is an individual deployment of an application specification", and the same file can be deployed twice under two names. Resources carry `com.docker.compose.project` (https://docs.docker.com/compose/intro/compose-application-model/).
- `external` resources have a lifecycle "maintained outside of that of the application" (https://docs.docker.com/reference/compose-file/networks/).
- `down` removes only the project's own resources: "Networks and volumes defined as external are never removed." Containers not in the file are orphans, removed only with `--remove-orphans` (https://docs.docker.com/reference/cli/docker/compose/down/).

# 8. Zanzibar, SpiceDB, OpenFGA

**Model:**
- Access control lists are relation tuples `object#relation@user`, where the user may be a userset `object#relation`.
- Rewrites per relation: `_this`, `computed_userset` and `tuple_to_userset`. The last follows a `parent` tupleset "to inherit [the folder's] viewers" (Zanzibar, annotated at https://zanzibar.tech/).
- SpiceDB writes this as `permission read = reader + parent_folder->read`, and subject relations such as `group:finance#member` (https://authzed.com/docs/spicedb/concepts/schema).

**Inheritance must be declared:**
- In OpenFGA a parent tuple "does not inherit the editor relation from parent to document" unless the model says so (https://openfga.dev/docs/modeling/building-blocks/object-to-object-relationships).
- The tupleset after `from` must be directly assigned (https://openfga.dev/docs/modeling/parent-child).
- Several parent tuples express membership in several groups (a DAG). That is how "shared to several scopes" is modeled.

**New enemy problem:** it "can arise when we fail to respect the ordering between ACL updates or when we apply old ACLs to new content". If Bob is removed and then content is added, Bob must not see it.
- Zookies are opaque so that clients cannot pick arbitrary timestamps, and they guarantee the check is "at least as fresh as" the content.

**Pitfalls:** wildcards (`user:*`): "Be very careful … Only grant it to read permissions." For listing only what a caller may see, SpiceDB offers LookupResources and bulk checks (https://authzed.com/docs/spicedb/modeling/protecting-a-list-endpoint).

# 9. Cross-system pattern table

| System | Unit | Homes per object | Sharing edge | Who grants | Ceiling | Consumer acceptance | Inheritance | Can it be moved? |
|---|---|---|---|---|---|---|---|---|
| Kubernetes | namespace | exactly one | ReferenceGrant, RoleBinding | target namespace's owner | role or reference | no | none (flat) | no (immutable) |
| Google Cloud | project | exactly one parent | Shared VPC; IAM on the host project | org/folder admin and host | networkUser on the project or a subnet | no | allow union; deny downward | yes, with rights on both parents |
| AWS | account | exactly one | RAM share; resource policy | the owning account | managed permission | yes, outside the org | SCP caps | n/a |
| Azure | resource group / subscription | exactly one | Service Group membership (view only); Lighthouse | member and group; the customer | delegated role | customer onboards | down the hierarchy, not through Service Groups | yes, with rights on both parents |
| GitHub | repository | exactly one | org secret visibility | org owner | the secret | no | org, then repo, then environment; the lowest shadows | n/a |
| HCP Terraform | project / workspace | exactly one project | variable sets; remote state sharing | org or project admin; the producer | | no | sets apply to future workspaces | yes |
| Compose | project name | one label | `external` | outside Compose | | n/a | none | n/a |
| Zanzibar | any object | many parents allowed | tuples | writer of the tuple | relation | n/a | only where declared | n/a |

# 10. The owner's direction, tested against this prior art

**(a) "Project should reference every object under it."**
- Every system above records membership on the member, fixed at creation, and computes the container's inventory. Examples: Kubernetes `metadata.namespace`; GCP's single parent; Azure resource IDs; the HCP workspace's project; Compose labels; the ApplySet part-of label.
- None keeps a member list on the container object. A list there would be a second source of truth that drifts and races.
- Recommendation: each registered object records its project, immutable as POOL-001's owner is. The project's inspection lists its members (the inventory of ruling 53, limited to what the caller may inspect). In the concise form, one file per project lists everything, as Compose's `name:` and file do. That satisfies "references every object" for the author without a mutable list in the engine.

**(b) "Pools accessible from multiple projects."**
- No system multi-homes a shared thing. It has one home (an owning account, host project, org-level secret or producer workspace) and explicit share edges that its home's authority creates.
- Each edge names its consumers and carries a ceiling of operations, as RAM's managed permission does. The consumer side grants within that ceiling. Edges are non-transitive (RAM) and visible to the owner.
- Multi-membership without authority exists only as views (Azure Service Groups, tags).
- Recommendation: a Pool's home is either the engine (as a GitHub org secret or an org-owned variable set) or exactly one project (as a repository secret, a project-owned variable set, or a Shared VPC host project). The home grants shares to `selected` or `all` projects. "All" must say whether it includes future projects and the home itself (RAM's `Principal: *` pitfall).

**(c) "Projects determine an Interface's permissions; Interfaces over one or many projects."**
- Universal pattern: a principal is global, a reusable set of operations is defined once (ClusterRole, Azure role definition, RAM permission), and each scope binds it separately (RoleBinding, IAM binding at a project, HCP team access per project, Lighthouse delegation per subscription).
- The grant should live with, and be revocable by, each project's authority, not as one list on the Interface.
- In Conduit this means a bound that can name a Project, and a `management` that can be narrowed to a project. Today CON-028 and ACC-030 forbid narrowing `management`, and ACC-030 points to brokers instead. This is the largest core change, and the one namespaces exist for: create rights per scope.
- A project-level cap, like an SCP (it grants nothing), would limit every bound used inside the project: its own Pools, plus shared-in Pools with their ceilings.

**(d) Specialized Pools.** GitHub and HCP show the shape: org-level and all-scope items for common things, scoped items for sensitive ones.
- Their pitfalls must be avoided:
  - precedence shadowing ("lowest level takes precedence");
  - priority overrides;
  - "all" silently covering future scopes;
  - the 100-secret truncation.
- For agents there is a further reason to make a share accepted by the consumer project, as a RAM invitation is: content from a shared Pool reaches its Agents. This is my inference, not something the sources say.

**(e) Approval of widening as a pending action; narrowing never waits; approvers declared.**
- Prior art: a Kubernetes CSR is approved with an `approve` verb scoped to a signer name. GitHub reviewers are declared, can be barred from self-review, and admins bypass by default. GCP PAM entitlements declare approvers and expire. RAM invitations expire after 12 hours or 7 days.
- Revocations take effect at once: a ReferenceGrant deletion forces recalculation, and RAM removal is automatic.
- Ruling 51's generic pending action (kind, subject, evidence, expiry, outcome) fits these. The prior art adds four things:
  - the approver set is declared per project and is itself a bounded right;
  - an approver cannot approve beyond their own authority (Kubernetes bind/escalate, ACC-030);
  - no self-approval, unless explicitly allowed;
  - approval is tied to the exact change (`expectedId` and revision), so the plan approved is the plan run.

**(f) Global vs scoped in Conduit, a sketch to decide family by family as Kubernetes does.**
- Engine scope: the engine and trust domain, nodes, providers, installed packages, `access`, `management`, the system Pool, the root Interface, and Projects themselves. Interfaces are probably here too, as principals with per-project grants, matching the owner's "authority over one or multiple projects".
- Project scope: Agents, project-homed Pools and their Resources, and Conduits.
- Cross-project Conduits are open: the Kubernetes RoleBinding sits on the target side, AWS evaluates both sides, and ruling 51 has two halves.
- Templates are open: they could be engine-wide like a ClusterRole, or per project.

**(g) Further challenges.**
- Vocabulary: "project" today is the source unit (the submission and its `address`). Compose, Pulumi and Terraform separate the source from the deployed scope; the same Compose file can be deployed under two project names. Conduit must decide whether the registered scope is named in the request (like `-p` or `-n`) or in the documents, whether one submission may span projects (an ApplySet needs a cluster-scoped parent to do so), and what the scope is called. The vocabulary is ruled, so naming needs a ruling.
- Names: today they are global (OBJ-002). Kubernetes and Backstage scope names per namespace and then need qualified cross-scope references and reserved prefixes. GCP keeps project IDs global and never reuses them, which matches ruling 53's tombstones.
- Nesting: Kubernetes namespaces are flat. Cloud hierarchies nest, with union inheritance (a child cannot restrict) and moves that need rights on both parents. Flat projects and immutable membership are the conservative v0.
- Retiring a project: retire its own members and never touch shared-in Pools (Compose `external` is never removed). Remove its outgoing shares first (RAM's advice), or refuse the retirement.
- Ruling 40: project scoping is access, so it is Conduit's lane. Lifecycle grouping and prune should reuse existing paradigms (ApplySet, Compose labels, plan/apply), not invent new ones.

# 11. Twelve lessons for Conduit

1. **One home, explicit shares, flat and immutable.** Every object belongs to exactly one scope, fixed at creation. Sharing is a separate edge, never a second home. Multi-membership without authority is only a view. Keep projects flat in v0; moving means replacement.
2. **The owner of the shared thing grants, and the consumer's scope grants within that.** Examples: ReferenceGrant in the target namespace; RAM's managed permission as the maximum; Shared VPC host; org secret visibility; producer-side remote state sharing. A cross-scope request needs both sides (AWS's two evaluations, ruling 51's two halves). Across a trust boundary the consumer accepts, with an expiry.
3. **Membership grants nothing; authority is an explicit binding per scope.** Examples: ACC-028; Backstage's "not to be used … to assign authorization"; OpenFGA's undeclared parent; Service Groups' "no RBAC inheritance". A reusable set of operations is bound per project. An Interface over N projects holds N bindings, each made and revocable by that project's authority.
4. **A grant on a container covers its future members, so make that explicit and visible.** Examples: GCP networkUser covers future subnets; Kubernetes wildcards and aggregated roles; HCP sets apply to future workspaces; RAM OU membership; ACC-029. "All projects" and "whole project" must be an explicit choice, shown in inspection and in the plan.
5. **A scope is only as strong as its weakest create right.** In Kubernetes, the right to create pods gives every Secret and every ServiceAccount in the namespace. AWS says delegating user creation "effectively trusts" the delegate with what new users can reach. In Conduit, whoever may register Agents or open Conduits in a project holds the combined reach of every Pool shared into it. Put sensitive Pools (auth) in their own project, or share them only to named consumers.
6. **Guard against confused deputies across scopes.** Examples: CVE-2021-25740; AWS SourceArn and ExternalId; the Gateway handshake. A Resource or broker that serves several projects must act with, and check, the requesting project, using Conduit's `serving` key and caller chain. Any object that names a target in another project needs that project's consent. Never disclose that something exists across projects without a grant (Gateway's MUST NOT; ruling 52's tiers; inventory without counts).
7. **No widening through delegation, and no re-sharing.** Examples: Kubernetes escalate/bind; ACC-030; RAM's "can't share a resource that's shared with you"; permissions boundaries. Shares are non-transitive. Only the home's authority shares a Pool on. Approvers cannot approve past their own authority.
8. **Caps differ from grants.** SCPs and permissions boundaries grant nothing; IAM policies and bindings grant. A project should carry a cap (what may be reached from inside it) with Conduits as grants within it. That is the mechanism by which "projects determine an Interface's permissions".
9. **Names: scoped names need qualified references and reserved prefixes; global names need never-reuse; never shadow.** Examples: the Kubernetes TLD collision and the `kube-` prefix; GCP IDs never reused; GitHub's "lowest level takes precedence" shadowing; Backstage's defaults filled from context. Conduit should reject collisions explicitly (ruling 52), not resolve them by precedence.
10. **Revocation is immediate and respects order.** A ReferenceGrant deletion forces recalculation; RAM removal is automatic; Zanzibar names the new enemy problem. Unsharing or narrowing applies at decision time, before new Resources land, and leaves tombstones (ruling 53). Narrowing never waits.
11. **Approvals are pending objects with declared, scoped approvers.** Kubernetes CSR approval is scoped by signer; GitHub reviewers can be barred from self-review, and admin bypass should be avoided; GCP PAM declares approvers and expires grants; RAM invitations expire. Each approval is tied to the exact change (`expectedId`/revision) and expires. This is ruling 51's pending action plus an approver set.
12. **The engine must record set membership, and prune only what it owns.** ApplySet replaced allowlist prune, which deleted unexpectedly; each object is in at most one set. Compose removes only its own resources and never external ones. Terraform workspaces show that separate state is not separate access. The project scope must be remembered by the engine, both for per-project drift, prune and tombstones and for access; a label alone is not enough.

## Unverified
- Zanzibar quotes come from the annotated copy at zanzibar.tech (AuthZed). The USENIX PDF was downloaded, but no PDF text tool was available to read it.
- GCP "a child cannot restrict a parent's allow grant": inferred from the documented union semantics. The explicit wording appeared only in a third-party note found by search.
- GitHub:
  - Whether "all repositories" visibility covers repositories created later is not confirmed by the docs fetched.
  - "Write access to a repository gives a path to its secrets": GitHub's docs confirm only that any write-access user can reach GITHUB_TOKEN by editing a workflow. The broader claim is from secondary sources (DEV Community, Classmethod).
- Kubernetes:
  - That deleting a namespace cascades to everything in it is common knowledge and was not fetched.
  - The status of the Hierarchical Namespace Controller was not checked.
  - CVE-2021-25740's mechanics come from the WebFetch summary of kubernetes/kubernetes#103675, not a verbatim advisory.
  - ApplySet is alpha.
- OpenFGA: exact DSL syntax (`define viewer: [user] or viewer from parent`) was not shown on the fetched pages; only the prose was.
- Pulumi: the permission needed to read another stack through a StackReference was not found.
- AWS RAM: whether consumers can modify or delete shared resources was not confirmed.
- Azure Service Groups are in public preview; their behavior may change.
- My own inferences, not the sources' claims:
  - Sharing a Pool into a project exposes that project's Agents to the Pool's content (a prompt-injection reason for consumer acceptance).
  - The global-vs-scoped split of Conduit's families in section 10(f).
  - Cross-project Conduits as a target-side share plus a consumer-side Conduit within a project cap.
- Which Conduit promises a Project scope would touch was not estimated. It likely includes OBJ-002, POOL-001, ACC-029, ACC-030, ACC-033, CON-028, SUB-005, SUB-008, NAM-022, NAM-027 and the inventory in LIF-013.
- No experiment was run. Nothing here is a measured result.