Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# How other configuration and authorization languages keep authors' work low and handle change: 12 patterns for simplifying Conduit

## 0. Bottom line

Every system surveyed separates three things that Conduit currently merges into one: (a) a short source that the author writes, (b) a stable identity under which a later edit counts as a change rather than a new object, and (c) generated explicit views (canonical form, plan, effective-permission query, explanation) that carry the audit burden. Conduit puts all three into the source. Each fact is a framed document. Templates and instances are both written out. Any change is a new object, because OBJ-002 says "A submission only adds objects and never changes one" (spec/promises/objects.yaml:69) and OBJ-011 says "no document rewrites the definition of an instance" (spec/promises/objects.yaml:197). Ruling 52's explicitness can be kept, and the author's lines cut, by moving the explicitness into the generated views (plan, render, reach, why). Terraform, Compose, kubectl, IAM, Cedar, SpiceDB and Claude Code all work this way.

On paper, combining the patterns below takes S1 from the coordinator's 332 lines to about 24 (keyed single file) or about 55-59 (current framed documents kept). The baseline is 64. These are paper estimates and need a run (section 5).

## 1. What the runs say the cost is made of, and the constraints any pattern must keep

The cost drivers, from the coordinator's report (docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md:77-96) and findings 4-01 and 4-11 (docs/proposals/first-build/log-increment-4.md:12, :22):

- **Frames.** About 130 frame lines plus 26 `---` separators, roughly half of the 332.
- **Template and instance for everything.** Every object is written twice, though each template is used once.
- **Change is new documents.** T5 took 62 lines and T6 took 29, against the baseline's 3 and 2. T4 took 2 lines but 8 requests, and each Agent was without access for about 30 s. The T4 route was to edit the templates `read` and `read-write`. The template named `read` now grants write (coordinator-run/project/templates.yaml), which is exactly 4-01's complaint that the name "describ[es] what it no longer grants".
- **Root's own Conduits to its Pool.** 35 lines. Both authors made the same first mistake here, and the coordinator's store was wedged.
- **Tool names follow standalone Conduit names.** `history-a.*`, then `project-a.*` after T5.
- **Copied digests.** 8 lines.
- **Required booleans with no safe default.** `accountServices` and `peerMessaging` are `required` with no default in packages/agent-claude-code/contracts.yaml:16-19.

Constraints the patterns must respect:

- **Preservation tests** (docs/AUDIT.md:473ff):
  - 2: every access is an inspectable, revocable relationship.
  - 5: every management action is available to an agent under a grant.
  - 6: nothing implicit; "no background reconciler or automatic wake" (docs/AUDIT.md:463).
  - 7: attributable.
  - 8: no rule assumes exactly one owner.
- **Ruling 40:** borrow existing paradigms.
- **Ruling 52:** be maximally explicit.
- **Round 6's table:** "Automatic apply, rollback, key inference, general policies/graphs: Do not add to make examples look shorter. Tools may prepare explicit actions; effects and uncertainty remain visible" (docs/proposals/reaudit/round-6/design-critique.md:655).
- **D08** (same file :282-303) allows "a pure preview of the required explicit actions" and asks for a complete trace before a targeted language change. Increment 4 now supplies that trace.

The consequence is that the change model to borrow is Terraform's and Compose's: an explicit one-shot apply, with no controller loop. Kubernetes controllers continuously reconcile, and that violates test 6.

## 2. Survey by system

Minimal example: "two principals, one shared thing, A read-only, B read-write". Lines are counted by the measurement plan's rule (non-blank, comments excluded, `---` counted). The examples are in <scratch>/examples/ and counted with count.sh there.

| System | Minimal example | Lines | Identity / name | Update in place vs replace | Defaults and shorthands | Explicitness for audit |
|---|---|---|---|---|---|---|
| Docker Compose | 2 services, bind mount `./shared:/shared:ro` vs rw | 9. Validated locally with `docker compose config`, which rendered a 24-line canonical form | Key in the `services` map. Project name defaults to the directory | `up` recreates only containers whose config or image changed, "preserving mounted volumes". The service DNS name survives a recreate. `--remove-orphans` is opt-in | Default network `<project>_default`. `version` obsolete. Short and long syntax. `x-` fragments plus anchors. Access mode defaults to `rw` (permissive) | `docker compose config` renders "the actual data model to be applied", and `--hash` gives a per-service config hash. No plan before recreate |
| Kubernetes RBAC | 2 Roles + 2 RoleBindings | 43 in 4 documents. 16 frame lines + 3 separators = 44%, the same share as Conduit's. Imperatively, 4 `kubectl create` commands | apiVersion/kind/namespace/name. Name immutable | `kubectl apply` is a three-way merge of file, live object and last-applied. It clears fields removed from the file and keeps fields set by others (`replicas` set by `kubectl scale`). Server-side apply adds field managers, conflicts and `--force-conflicts`. Some fields are immutable: RoleBinding `roleRef`, Deployment `.spec.selector`, most Pod fields. `kubectl auth reconcile` deletes and recreates bindings for the author | Defaults filled by the API server. Built-in roles `view`/`edit`/`admin`. Aggregated ClusterRoles. Bootstrap bindings are auto-reconciled at each start | `kubectl diff` (server dry-run), `kubectl auth can-i … --as=…` and `--list`, `managedFields` |
| kustomize / Helm | Overlay patch for T4 | 9 lines for the T4 overlay, validated with `kubectl kustomize`, worse than editing 2 lines | Same as Kubernetes | Helm 3 uses "a three-way strategic merge patch… old manifest, its live state, and the new manifest" | values.yaml defaults; text templates | Rendered output |
| Terraform / OpenTofu | GitHub repo + collaborators with `for_each = { agent-a = "pull", agent-b = "push" }` | 14 (5 of them the provider pin) | Resource address `type.name[key]`. A rename is destroy+create unless there is a `moved` block. `import` and `removed` blocks exist | The provider schema decides per attribute. In integrations/github every configurable field of `github_repository_collaborator` is `ForceNew`, so a permission change is a replacement (`-/+`). Lifecycle options: `create_before_destroy`, `prevent_destroy`, `ignore_changes`, `replace_triggered_by`; also `-replace` | `optional(type, default)`; modules | `plan` symbols `+ ~ - -/+`. Saved plan via `-out`. `.terraform.lock.hcl` with hashes is created by `terraform init`. Changes happen only on an explicit apply |
| Pulumi | Same, in TypeScript | 5 | URN (stack, project, type, parent, name). A rename is delete+create unless there are `aliases` | Provider decides, plus `replaceOnChanges`, `deleteBeforeReplace` and `protect` | Language defaults | `preview` |
| CUE | No domain of its own; a front end to a schema | 3-6 data lines given a schema | Path in the value tree | Not applicable (a config language, not a deployer) | Defaults are `*`. Closed definitions reject unknown fields ("field not allowed"). "An instance may never violate the constraints of its parent". Constraints with defaults "validate and reduce boilerplate" | `cue export` shows the resolved value |
| Pkl | Same | Similar | Property path | Not applicable | `amends`: `parrot = (pigeon) { name = "Parrot" }` keeps the other fields. Module amends. Late binding | `pkl eval` |
| Nix modules | Same | Similar | Attribute path | `nixos-rebuild switch` restarts only units that changed. Per-service `restartIfChanged` and `reloadTriggers` | Option `default`. Type-directed merge (lists concatenate). Priorities: `mkDefault` 1000, plain 100, `mkForce` 50. Two definitions of a unique option are an error ("defined multiple times") | `nixos-option` shows the merged value |
| Dhall | Same | Similar | Not applicable | Not applicable | `Schema::{ name = "Jane" }` completion from `{Type, default}` | Normal form |
| AWS IAM | Bucket policy with 2 statements | 17 (checked as JSON with jq only) | ARN | Inline policies are 1:1. Managed policies are shared: a change applies to every attached principal, IAM keeps up to 5 versions, and rollback is by setting the default version. AWS widens managed policies itself (ReadOnlyAccess gains new services) | Implicit deny. Explicit deny overrides. Identity and resource policies form a union; boundaries and SCPs intersect | The policy simulator names the matching statement. Access Analyzer reports unused access and generates policies from CloudTrail |
| Cedar | 2 policies + 5-line schema with an action group | 7 | Entity UIDs `Agent::"a"` | Templates with `?principal`/`?resource`. Linked policies take a template change "automatically and immediately" | Default deny. `forbid` overrides. Action groups (`action read, list in [readOnly]`) | The response's diagnostics give the determining policies and errors. Schema validation. AWS AgentCore Policy applies Cedar to an agent gateway's tools and, by default, rejects policies that are overly permissive, overly restrictive or ineffective |
| SpiceDB (Zanzibar) | 7-line schema + `directory:shared#reader@agent:a`, `…#writer@agent:b` | 9 | Object type:id. A relationship is one tuple | Access changes are tuple writes: CREATE (errors if present), TOUCH (idempotent upsert), DELETE (no-op if absent), combinable atomically | Computed permissions: `permission read = reader + writer` | `zed permission check --explain` trace; lookup of resources and subjects |
| OpenFGA | 7-line model + 2 tuples | 9 | type:id | Tuple writes and deletes | `define reader: [agent] or writer` | Check, Read, Expand, ListObjects ("objects… a specific user has a specific relationship with"), ListUsers. Assertions guard against regressions when tuples or the model change |
| Claude Code | `.mcp.json` + `permissions.allow`/`deny` | The baseline's T1 was 33 lines across both clients, including trust | Server key, so tool names are `mcp__<server>__<tool>` | Scopes local > project > user > plugin > connectors. "The entire server entry from that source is used; fields are not merged". `list_changed` refreshes tools live | Rules evaluated deny, then ask, then allow; first match wins. A deny at any level wins. Managed settings are highest | `/permissions` "lists all permission rules and the settings.json file each rule comes from". `claude mcp add` writes the config |
| Codex | `[mcp_servers.<name>]` | Included in the baseline | Table key | The app and IDE ask for a restart after saving | `enabled`. `enabled_tools` is an allow list; `disabled_tools` is a deny list "applied after enabled_tools". `default_tools_approval_mode` and per-tool `approval_mode` | `codex mcp add` |

Cross-cutting observations:

1. **No surveyed system makes the author write a separate template for a thing used once.** Kubernetes embeds the pod template in the Deployment: "PodTemplates… are included in workload resources such as Deployments". IAM offers inline policies. Cedar has static policies beside its templates. Shared, named definitions are opt-in, and every system that has them states what happens to their holders when they change. IAM: "applied to all principal entities that the policy is attached to", with 5 versions kept. Cedar: linked policies update "automatically and immediately".
2. **The systems with low author effort key objects by map key inside one file and state the frame once:** Compose, Terraform, Nix, `.mcp.json`, Codex. Compose went further and made `version` obsolete. Kubernetes has a per-object frame, costs the same ~44% as Conduit, and its users lean on generators, kustomize and Helm.
3. **Every deployer distinguishes update-in-place from replace per field and says which in the preview.** Kubernetes makes some fields immutable on purpose. Its RBAC docs give the reason for `roleRef`: "A binding to a different role is a fundamentally different binding." It still ships `kubectl auth reconcile` so the author does not do the delete and recreate by hand.
4. **Authorization systems change access by changing an assignment (a tuple, a binding's subjects, a linked policy), not by editing the level's definition.** The natural Kubernetes T4 is moving subjects between two bindings: subjects are mutable, so it is 2 lines in place. The coordinator's T4 instead edited the level, the template `read`, which is the anti-pattern.
5. **Deny lists fail open on upgrade.** In the baseline, A's read-only access was a deny list of write tools. At T5, 2.2.0's `append_file` and `copy_file` were not on it, and A wrote with them (coordinator-run-2026-11-10.md, T5 row). Allow lists pinned to a version (Conduit's current behaviour, Codex `enabled_tools`) do not.
6. **Effective-permission and explanation queries are standard and cheap:** `kubectl auth can-i --list --as`, ListObjects/ListUsers, the IAM simulator's matched statement, Cedar's determining policies, Claude's `/permissions` with provenance. They are how these systems stay explicit without making authors write more.

## 3. Twelve patterns for Conduit, each with its trade-off

### P1. Resubmitting a changed document updates the object, through a pure plan of explicit operations, applied once

- **Precedent.** Terraform plan/apply with `-out`; `kubectl apply`/`kubectl diff`; Compose `up`.
- **For Conduit.** Same kind and name means the same object. A changed document yields a plan whose every step is an existing operation:
  - a control for a setting a control covers;
  - close/open or an in-place operations change for access;
  - set-up/retire for a replacement.
- **How apply runs.** `apply` performs exactly the plan as the caller. Each step is authorized under ACC-033 and recorded. Revisions are checked, so a stale plan reports conflict. Drift is shown and is never corrected in the background.
- **Promises touched.** OBJ-002, OBJ-011 (its second clause), SUB-007, INS-020, LIF-009, PKG-005.
- **Effect on S1.** T4 goes from 8 requests to 1. T5 goes from 62 lines to about 1, T6 from 29 to about 1, T7 from 12 to about 2.
- **Ruling 50.** A plan submitted by an Agent can become the "pending action" of ruling 50 at the root Interface, with the plan as its evidence.
- **Trade-off.** The engine must diff and classify every change, and "same object" now depends on stored state. The plan can go stale. One more concept (plan) is added, but it is borrowed. Test 6 holds only if apply is never automatic.

### P2. Each field has a change class, declared by the contract and shown in the plan

- **Precedent.** Terraform ForceNew and "forces replacement"; Kubernetes immutable fields; Pulumi provider-decided replacement plus `replaceOnChanges`; NixOS `restartIfChanged`/`reloadTriggers`.
- **For Conduit.**
  - Reuse controls. A setting that some control's `changes` covers is in-place. Any other config setting, and the package version, forces replacement of the instance.
  - A Conduit's consumer, target and capability are its identity, so changing one makes a new Conduit, as with `roleRef`. Its operations within the capability change in place as one atomic swap, which removes the 30 s gap in T4.
  - The plan states each consequence: tools gained or lost per Agent, sessions lost, Conduits moved, any interval without access.
- **Lifecycle flags.**
  - `keep` (like `prevent_destroy` or `protect`) refuses plans that would retire an Agent with a live session.
  - The order of each pair is stated: narrowing before widening by default, or make-before-break for continuity (`create_before_destroy`).
- **Trade-off.**
  - Contracts must state the classes, and a wrong class surprises the author.
  - Ordering rules are new vocabulary.
  - Kubernetes shows that immutable fields without a tool that does the replacement (as with Deployment selectors) push work back onto the author.

### P3. Templates are inline by default; named shared templates are opt-in

- **Precedent.** The Deployment's embedded pod template; IAM inline vs managed policies; Cedar static vs template-linked policies.
- **For Conduit.** An instance document may carry its own `spec`, so no `from` is needed. A named template is written only when reused. A change to a shared template appears in the plan for every holder (Cedar and IAM managed semantics), versioned so it can be rolled back (IAM keeps 5 versions). It never moves holders silently.
- **Promises touched.** The template/instance definitions, TPL-001, the INS area.
- **Effect on S1.** Removes the second copy of 9 objects in T1 and more in T2, roughly 45-60 of the coordinator's lines.
- **Trade-off.** INS-020's rule that an instance "keeps the definition… it resolved" must be restated for linked templates. Two ways to write an instance. Some reuse naming is lost.

### P4. Named operation sets come from the package; change access by changing which set an Agent gets

- **Precedent.** Kubernetes `view`/`edit`/`admin`; AWS managed ReadOnlyAccess; Cedar action groups; SpiceDB `permission view = reader + writer`; OpenFGA `reader: [user] or writer`.
- **For Conduit.** The filesystem package exports `read` (file.read, directory.list) and `read-write` (adds file.write) as operation sets of `directory.v1`, and a Conduit's `operations` may name a set. The capability stays one per Conduit, which round 6 says to retain. T4 becomes swapping two words, and the set's name stays true.
- **Expansion on upgrade.** Kubernetes aggregates roles and AWS widens managed policies on its own. In Conduit a set is resolved against the pinned package version, and the plan must print any widening, for example "agent-b gains file.append via read-write". This keeps T5's safety.
- **Trade-off.** Package authors must define and version the sets. Sets differ between packages. Grants can become coarser than needed.

### P5. One file, keyed maps, frame once; the framed documents remain the canonical long form

- **Precedent.** Compose `services:`, Terraform `resource "type" "name"`, Nix attribute paths, `mcpServers`/`[mcp_servers.x]`. Compose dropped `version` and defaults `name`. Its short and long syntaxes are both accepted, and `docker compose config` renders the long form.
- **For Conduit.** `resources:`, `agents:` (and `pools:` and `interfaces:` when needed) carry the kind; the key carries the name; `apiVersion` appears once. The loader expands the file into today's documents, so the schemas, the strict loader and the vectors keep working on the canonical form. `conduit render` prints that form.
- **Effect on S1.** T1 goes from 34 lines (framed, P1-P4 and P6-P8 applied) to 14 (section 5).
- **Trade-off.** Two syntaxes to specify and test. Diagnostics must point to the line the author wrote. Prefer flow maps over colon-strings; Compose's short syntax is known for parsing ambiguity.
- **Safe defaults.** Fields with a safe value should get it as a default, so they can be omitted, for example `accountServices: false` and `peerMessaging: false`. Defaults for access must be the narrow choice. Compose's `rw` default is the counter-example.

### P6. Names the consumer sees are stable and separate from object names

- **Precedent.** A Compose service keeps its DNS name across recreate ("joins the network under a different IP address but the same name"). Claude Code tool names come from the config key. Kubernetes Services stay put while Pods change.
- **For Conduit.** Every Conduit whose consumer is an Agent has a key (`as:`, defaulting to today's names) that sets its tool prefix. A replacement keeps the key. Access added later goes under the Agent's reach map, or as a standalone Conduit with `as: history`.
- **Effect on S1.** Removes the `history-a.*` and `project-a.*` renames (finding 4-11, item 5). Agents keep their learned tool names across T5 and T6.
- **Trade-off.** Two Conduits with the same key to one consumer must be rejected. The link from a tool to its object becomes indirect, and inspection must show it.

### P7. Owner authority is made by the engine at registration as visible, closable Conduits, with a default Pool per project

- **Precedent.** Kubernetes' bootstrap `cluster-admin`/`system:masters` bindings, labeled and auto-reconciled at each start, with an `autoupdate=false` opt-out. Compose's and Terraform's invoking user. Compose's default network.
- **For Conduit.** The Interface that registers a project gets Conduits to each Pool the project makes, and `turn` Conduits to each Agent. All are ordinary objects, listed by inspection, and can be closed. They are made only at registration, never re-granted on restart, so there is no reconciler (test 6). Phrased for "the registering Interface", so no single owner is assumed (test 8).
- **Effect on S1.** Removes 35 lines and 3 more root Conduits in T1, the first mistake both authors made, and the coordinator's wedged store (4-07).
- **Trade-off.** It is an implicit grant, explicit only after the fact, so `render` and `plan` must print it. Delegation-only-narrows must still hold.

### P8. The tool writes the lock; the author writes constraints

- **Precedent.** Terraform's `.terraform.lock.hcl`, which `terraform init` creates and `init -upgrade` updates; Cargo.lock; Helm's Chart.lock.
- **For Conduit.** The author writes `example.org/s1/filesystem@0.1`. `conduit install` or `apply` records the version and digest (the MAN-010 pin) in the manifest's lock section or a lock file.
- **Effect on S1.** Removes 8 copied lines and a class of copy mistakes. T5 becomes a 1-line change of the constraint plus the plan.
- **Trade-off.** One more file. Trust-on-first-use of digests (Terraform records several hashes). Ruling 40 is unaffected, because fetching stays a provider's job.

### P9. Explicitness lives in four generated views: render, plan, reach, why

- **Precedent.**
  - render: `docker compose config`, `nixos-option`.
  - plan: Terraform plan, `kubectl diff`.
  - reach: `kubectl auth can-i --list --as=`, OpenFGA ListObjects/ListUsers, SpiceDB lookups.
  - why: Cedar's determining policies, the IAM simulator's matched statement, `zed --explain`, Claude `/permissions` with the source file of each rule.
- **For Conduit.**
  - `conduit reach --as agent-a` answers forward; `--to shared` answers reverse.
  - Reach includes the Agent type's declared means around the engine, so T3 needs no README reading.
  - `conduit why agent-a shared file.write` names the Conduit that allows the call, or gives ruling 52's two-tier refusal.
  - Plan-time findings in the manner of AgentCore Policy: an ineffective Conduit (an operation the target lacks), an over-broad set, a grant no consumer uses.
  - Every view is computed by the same decision code the engine uses. The IAM simulator warns that its results "can differ from your live AWS environment"; Conduit's must not.
- **Trade-off.** More commands to specify and test. The views must be exact, or they become a source of false assurance.

### P10. Field ownership between the file and live operations

- **Precedent.** Kubernetes server-side apply field managers and conflicts. `kubectl apply` keeps fields that are not in the file (`replicas` set by `kubectl scale`). Terraform `ignore_changes`. Helm 3's three-way merge.
- **For Conduit.** Agents and Interfaces change live state through controls and open/close (test 5). A re-apply must not silently undo those changes. Settings the file names belong to the file. A setting changed by a control since the last apply, and also changed in the file, is a conflict in the plan and needs `--force`. A declared Conduit that an actor closed is shown as "would re-open (closed by X at T)", not reopened quietly.
- **Trade-off.** It needs last-applied state per object and is the subtlest part. Kubernetes took several releases to make server-side apply GA. It could be deferred if v0 accepts "file wins, conflicts shown".

### P11. Grants as one-line assignments, with imperative helpers that edit the file

- **Precedent.**
  - SpiceDB and OpenFGA tuples, with CREATE/TOUCH/DELETE.
  - Kubernetes binding subjects, which are mutable.
  - `claude mcp add` and `codex mcp add`; `kubectl create rolebinding --dry-run=client -o yaml` (generated locally).
- **For Conduit.** In the reach map, one grant is one line, e.g. `project: {to: shared, operations: read}`. `conduit grant` and `conduit revoke` edit the file and apply it, idempotently like TOUCH and DELETE, which the idempotency keys already support. T7 becomes 2 lines or 2 commands.
- **Trade-off.** There are two places to express access (the reach map and standalone Conduits), so pick one canonical place. The YAML writer must preserve the author's formatting. Options such as requirements and filters need the long form.

### P12. Renames and removals are explicit and safe

- **Precedent.** Terraform `moved`; `removed {lifecycle {destroy = false}}`; `prevent_destroy`. Pulumi `aliases` and `protect`. Compose `--remove-orphans` is opt-in. kubectl `--prune` is still alpha, and the docs call explicit `kubectl delete -f` "the recommended approach".
- **For Conduit.** OBJ-011's fixed names can stay. A `moved: {from, to}` entry keeps the object's ID and records the new name. An object missing from the file is left alone unless `--prune`. A removal is a plan line ("retire agent-a: session lost") that needs confirmation.
- **Effect on S1.** Prevents the fresh author's T7, where both Agents were recreated and lost their sessions.
- **Trade-off.** More vocabulary. Pruning is where Kubernetes users get hurt, so keep it opt-in.

## 4. What not to adopt, and why

- **Kubernetes controllers and continuous reconciliation.** They break test 6.
- **Helm's text templating over YAML.** It is a second language. Round 6 retains "no expression language".
- **CUE, Nix, Dhall or Pkl as the authoring language.** Unification, laziness and priorities add concepts. Borrow their ideas instead: schema defaults (already in INS-019 and SCH), closed schemas (Conduit's strict loader), and "constraints never overridden", which matches "delegation only narrows".
- **kustomize overlays for S1.** The T4 overlay is 9 lines against 2 for an edit, and in Kubernetes it would fail anyway, because `roleRef` is immutable.
- **Deny lists as the access form.** They fail open on upgrade (the baseline's T5).
- **Kubernetes-style per-object frames as the authored form.** They cost about 44% in Kubernetes too.

## 5. Paper sketch: what the patterns could buy on S1 (hypothetical, not a proposal, not run)

- **Sketch A** keeps today's framed documents and applies P1-P4 and P6-P8: one Resource and two Agents, each with a nested Conduit naming the target and an operation set, plus a 5-line manifest. That is 34 lines for T1 (<scratch>/examples/sketch-a/).
- **Sketch B** adds P5, a keyed single file of 14 lines (sketch-b/conduit.yaml):

```yaml
conduit: v0
packages:
  fs: example.org/s1/filesystem@0.1
  claude: example.org/conduit/agent-claude-code@0.1
  codex: example.org/conduit/agent-codex@0.1
resources:
  shared: {type: fs#local-directory, root: <lab>/shared}
agents:
  agent-a:
    type: claude#claude-code
    reach: {project: shared fs#read}
  agent-b:
    type: codex#codex
    reach: {project: shared fs#read-write}
```

| Task | Baseline | Coordinator | Sketch A (framed), est. | Sketch B (keyed), est. | How |
|---|---|---|---|---|---|
| T1 | 33 | 154 | 34 | 14 | P3, P4, P5, P7, P8 |
| T2 | 16 | 73 | about 15 | 4 (1 package, 1 resource, 2 changed reach lines) | P6 keeps `history.*` names |
| T3 | 0 | 0 | 0 | 0 | P9, 1 command |
| T4 | 8 | 2 (8 requests) | 2 | 2 | P1, P2, P4: in-place operation swap, no gap |
| T5 | 3 | 62 | 1 | 1 | P1, P2, P8: plan shows the replacement and that no Agent gains append/copy |
| T6 | 2 | 29 | 1 | 1 | P1, P2, P6: target replaced, tool names kept |
| T7 | 2 | 12 | 6 (or 2 commands) | 2 | P11 |
| Total | 64 | 332 | about 55-59 | about 24 | |

Against the bar, sketch A reaches about parity: T1 equal, T7 more unless commands are used. Sketch B passes on paper: less on 5 tasks, equal on T3 and T7, more on none.

The sketch's short string `shared fs#read` should become a flow map, per P5's ambiguity warning, which adds no lines. These figures are paper counts. Ruling 45 dropped paper counts as evidence (ROADMAP.md:83), so each needs a run by a fresh author.

## 6. A possible order of evaluation (options for the owner; the owner rules)

1. P1+P2 with P9's plan. This is 4-01's "kubectl apply" option and has the largest effect.
2. P7 and P8, small and independent.
3. P3 and P4, which together remove the template/instance doubling and make T4 two words.
4. P6.
5. P5 last, as a front end over the canonical form, because it touches the loader and schemas.
6. P10-P12 can follow in v0's simplest forms: file wins with conflicts shown; prune opt-in.

## Sources (primary unless noted)

**Compose:**
- docs.docker.com/reference/cli/docker/compose/up/
- docs.docker.com/compose/how-tos/networking/
- docs.docker.com/reference/compose-file/extension/
- docs.docker.com/reference/cli/docker/compose/config/
- docs.docker.com/reference/compose-file/version-and-name/
- docs.docker.com/reference/compose-file/services/

**Kubernetes:**
- kubernetes.io/docs/reference/using-api/server-side-apply/
- kubernetes.io/docs/tasks/manage-kubernetes-objects/declarative-config/
- kubernetes.io/docs/reference/access-authn-authz/rbac/
- kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/kubectl_auth_can-i/
- kubernetes.io/docs/concepts/workloads/pods/
- kubernetes.io/docs/concepts/workloads/controllers/deployment/
- kubectl.docs.kubernetes.io/guides/introduction/kustomize/
- helm.sh/docs/faq/changes_since_helm2/

**Terraform:**
- developer.hashicorp.com/terraform/language/meta-arguments/lifecycle
- …/language/moved
- …/language/modules/develop/refactoring
- …/cli/commands/plan
- …/language/meta-arguments/for_each
- …/language/expressions/type-constraints
- …/language/import
- …/language/files/dependency-lock
- raw.githubusercontent.com/integrations/terraform-provider-github/main/github/resource_github_repository_collaborator.go

**CUE, Pkl, Nix, Dhall:**
- cuelang.org/docs/tour/types/defaults/, /concept/the-logic-of-cue/, /tour/types/closed/
- pkl-lang.org/main/current/language-reference/
- nixos.org/manual/nixos/stable/
- Nix priorities from discourse.nixos.org and manual mirrors (secondary)
- restartIfChanged from mynixos.com (secondary)
- Dhall FAQ wiki and the dhall-kubernetes PR (secondary)

**Pulumi:**
- pulumi.com/docs/iac/concepts/options/aliases/, /replaceonchanges/, /protect/

**AWS IAM:**
- docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic.html
- …/access_policies_testing-policies.html
- …/access_policies_managed-vs-inline.html
- …/access_policies_managed-versioning.html
- …/what-is-access-analyzer.html

**Cedar:**
- docs.cedarpolicy.com/policies/syntax-policy.html, /policies/templates.html, /auth/authorization.html, /schema/human-readable-schema.html
- docs.aws.amazon.com/bedrock-agentcore/latest/devguide/policy-validation-overview.html

**SpiceDB:**
- authzed.com/docs/spicedb/concepts/schema, /concepts/relationships, /ops/data/writing-relationships
- zed README v1.14.0 on pkg.go.dev, for `--explain`

**OpenFGA:**
- openfga.dev/docs/modeling/getting-started
- openfga.dev/docs/interacting/relationship-queries

**Claude Code:**
- code.claude.com/docs/en/permissions
- code.claude.com/docs/en/mcp

**Codex:**
- learn.chatgpt.com/docs/extend/mcp?surface=cli (developers.openai.com/codex/mcp redirects there)

**Repo** (read only; nothing in it was modified):
- docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md
- docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md
- docs/proposals/first-build/increment-4/coordinator-run/
- docs/proposals/first-build/increment-4/measurement-plan.md
- docs/proposals/first-build/log-increment-4.md
- experiments/007-s1-baseline/setup/
- experiments/007-s1-baseline/results-2026-11-09.md
- spec/promises/objects.yaml
- spec/promises/instantiation.yaml
- spec/promises/submissions.yaml
- spec/definitions.md
- docs/AUDIT.md
- docs/proposals/reaudit/round-6/design-critique.md
- docs/proposals/ruling-49 to ruling-52/README.md
- packages/agent-claude-code/contracts.yaml

## Numbers
| Item | Number |
|---|---|
| Baseline A70 lines (T1..T7) | 33 / 16 / 0 / 8 / 3 / 2 / 2 = 64 |
| Coordinator lines (T1..T7) | 154 / 73 / 0 / 2 (8 requests) / 62 / 29 / 12 = 332 |
| Coordinator frame lines + separators (its report) | about 130 + 26, about half |
| Root's Conduits to its own Pool (coordinator) | 35 lines |
| Copied digests (coordinator) | 8 lines |
| Fresh author | 646 JSON lines, 46 files, plus 130 script lines |
| Minimal "A read, B read-write" example: Docker Compose | 9 lines (validated; renders to 24 canonical lines) |
| Minimal example: Kubernetes RBAC | 43 lines in 4 documents; 16 frame + 3 separators = 44% |
| Minimal example: kustomize T4 overlay | 9 lines (validated with kubectl kustomize) |
| Minimal example: Terraform (GitHub) | 14 lines (9 without the provider pin) |
| Minimal example: Pulumi TypeScript | 5 lines |
| Minimal example: AWS IAM bucket policy | 17 lines |
| Minimal example: Cedar | 2 policies + 5-line schema = 7 |
| Minimal example: SpiceDB | 7 + 2 = 9 |
| Minimal example: OpenFGA | 7 + 2 = 9 |
| IAM managed policy versions kept | 5 |
| Nix priorities | mkForce 50, plain 100, mkDefault 1000, option default 1500 (secondary sources) |
| Paper sketch A (framed, P1-P4 and P6-P8) | T1 34; S1 total about 55-59 |
| Paper sketch B (keyed file, all patterns) | T1 14; T2 4; T3 0; T4 2; T5 1; T6 1; T7 2; total about 24 |
| Patterns proposed | 12 (P1-P12) |

## Unverified
**Not run through their own tools** (none installed locally): the minimal examples for Terraform, Pulumi, Cedar (schema syntax `action read, list in [readOnly] appliesTo {...}` and bare `action` in scope), SpiceDB, OpenFGA, CUE, Pkl, Nix and Dhall. The ones that did run:
- Compose, through `docker compose config` and `--hash`.
- The Kubernetes kustomize base and T4 overlay, through `kubectl kustomize` (kubectl 1.34.1).
- The RoleBinding, generated with `kubectl create --dry-run=client`. The Role generator needed a cluster, so the Role was written by hand.
- The IAM policy, checked as JSON with jq only.

**The S1 sketch figures are paper estimates.** They are not runs, and ruling 45 dropped paper counts as evidence. A fresh author's run is needed.

**Not confirmed from a fetched primary page:**
- The exact Terraform plan annotation "# forces replacement". The plan page confirms the `-/+` symbol; the annotation text is from my own knowledge.
- Nix priority numbers. Secondary sources conflict on the priority of plain definitions (100 in recent manual text, 1000 in an older book).
- NixOS `restartIfChanged`/`reloadTriggers` semantics (MyNixOS and commit messages).
- Dhall `::` semantics (FAQ wiki and a PR, not the language standard).
- `zed --explain` (zed v1.14.0 README).
- OpenTofu, assumed identical to Terraform for these features. Its docs were not checked.

**Truncated fetches:**
- The Kubernetes server-side apply page: the merge-strategy and ownership-transfer sections were not read.
- The Pulumi preview diff symbols were not fetched.

**Codex and Claude Code points not stated in their docs:**
- That `codex mcp add` writes config.toml. The page implies it but does not say it.
- Whether a running Codex CLI session picks up config.toml edits.
- Whether a running Claude Code session picks up `.mcp.json` edits without a reconnect.

**AgentCore Policy** is described from its validation page only.

**Conduit promises read in full:** OBJ-002, OBJ-011, INS-020 and SUB-007. TPL-001 and PKG-005 were read in part. Several promises are named only as likely to be affected and were not read in full: LIF-009, CTR-006, MAN-010, ACC-033, CON-008, IFC-012, IFC-033 and REV-*. A designer must check each before relying on the mapping.

**Not checked:**
- Whether Conduit's filesystem type offers a control for `root`. This decides whether T6 is in-place or a replacement under P2.
- Whether Agent packages could default `accountServices` and `peerMessaging` safely without a ruling.
- Whether moving explicitness into generated views satisfies ruling 52 and preservation test 2 as the owner reads them. That is the owner's ruling.