# Enrollment: how a machine is added through Conduit

Not binding. The [specification](../../../spec/README.md) alone binds. Research and a proposal by the coordinator,
2026-11-10, for the review that [ruling 50](../ruling-50/README.md) asks for before any promise is
written. [Ruling 51](../ruling-51/README.md) accepted the decisions of the
[networking paradigms](networking-paradigms.md) beside it, and added a managed and a peer mode
for machines the owner controls; [ruling 52](../ruling-52/README.md) accepted decisions 1 to 4,
6 and 7, and [ruling 56](../ruling-56/README.md) settled decision 5, so all seven are decided.

## 1. The question, and what ruling 50 already settled

How does a person add a machine, such as the owner's Windows computer, so that it becomes a node
on which the machine provider can place instances? Ruling 50 settled three things:

1. Enrollment becomes part of the language, as an operation of the machine provider's capability
   that returns an action for a person. Its design waits for this review.
2. In v0 a machine that joins becomes a node only once a person confirms.
3. Every action waiting on a person, ruling 49's sign-ins and enrollment alike, is one mechanism,
   raised at the root Interface, which acts as a control panel.

Ruling 40 bounds the answer: Conduit decides who may join and what a joined machine may be given;
the network, the agent software and the machine's own keys stay with an existing tool behind the
machine provider's contract. Tailscale is the candidate provider.

## 2. What the specification has today, and what is missing

What exists:

- The machine provider reports machines with `list-machines.v1`: `hostname`, `labels` and
  `state` (`reachable`, `unreachable`, `gone`) (PRV-014), and places an instance with `place.v1`
  on a reachable machine whose labels match (PRV-015). At most one exists, owned by `system`
  (PRV-013). Where none exists, the engine's own machine provider reports only the engine's
  machine (PRV-003, PRV-013).
- The engine makes a node for every reported machine that no node represents, at once, with no
  actor asking (PLC-026). A node is a `node.v1` Resource owned by `system` with an empty
  configuration, and its status holds the labels last reported (PLC-008).
- A node represents a hostname, compared in canonical form, "whichever machine provider reports
  it" (PLC-026). A machine reported again after its node was retired gets a new node (PLC-024,
  PLC-026).
- Labels are "reported status and never configuration: no document and no operation sets one"
  (PRV-014, PLC-026).
- Unreachable retires nothing (PLC-023); gone declares the node lost (PLC-024); the root
  Interface can declare a node lost with `declare-lost.v1` (PLC-017).
- Neither the network, the machine nor the runtime provider establishes which instance sent a
  request (PRV-018); the gate and per-instance credentials do (PRV-025, PRV-028; multi-machine
  proposal 1.3, 1.4).

What is missing:

1. **No operation adds a machine.** A machine appears only if the provider's tool already knows
   it, which is the manual setup of [the second-machine guide](../../guides/second-machine.md).
2. **No approval.** PLC-026 makes a node the moment a machine is reported, which contradicts
   ruling 50 point 2.
3. **No action for a person.** The pending sign-in of ruling 49 is the only kind, and it is not
   yet in the specification.
4. **A node is bound to a hostname only.** The hostname is what the machine says of itself. Any
   machine that joins under the hostname of an existing node is that node (PLC-026). Nothing in
   the report identifies a machine more firmly.
5. **No rule says where labels come from.** The provider reports them, but the specification does
   not say whether the joining machine can choose them. A placement selector that keeps sensitive
   instances on a labelled machine is only as strong as that rule.
6. **No removal at the provider.** Declaring a node lost retires it in Conduit, but the machine
   stays in the provider's network and, if reported again, gets a new node (PLC-024).

## 3. How existing systems add a machine

Read on 2026-11-10 from the sources in section 8. "Not read" means the page did not state it.

| System | Join secret | Who verifies whom | Person approval | Labels | Revocation | Re-join |
| --- | --- | --- | --- | --- | --- | --- |
| kubeadm / kubelet | Bootstrap token `[a-z0-9]{6}.[a-z0-9]{16}`, ID public, secret private; optional expiry; authenticates only as `system:bootstrappers` | Node pins the control plane's CA by `--discovery-token-ca-cert-hash` (SHA-256 of the SPKI); the skip flag "weakens the kubeadm security model"; control plane checks the token, then signs a CSR | CSR approved by the controller manager or "an outside process, possibly a person" | NodeRestriction lets the kubelet set `kubernetes.io/hostname`, `os`, `arch` and similar, and forbids `node-restriction.kubernetes.io/` and `node-role.kubernetes.io/`, reserved "to prevent unprivileged nodes from self-declaring cluster roles" | Delete the token; tokencleaner removes expired ones | New CSR with a valid token |
| k3s | Secure token `K10<CA hash>::<credentials>`; bootstrap tokens via `k3s token create`, 24h default TTL | Node checks the CA bundle's hash from the token; the short form "implicitly trusts" the server | Not read | Not read | Token rotation requires restarting agents that joined with it | Not read |
| Docker Swarm | `SWMTKN-1-...`, one per role (worker, manager) | Token holds "the digest of the root CA's certificate and a randomly generated secret": node checks the manager, manager checks the secret | None | Not read | Rotation affects only later joins; CA rotation invalidates old tokens | Not read |
| Nomad 1.11 | Introduction token: a JWT scoped to node name, node pool and TTL | mTLS both ways; the token is a second layer, "It does not replace mTLS" | None; enforcement `warn` or `strict` | Scope fixed by the operator in the token (node pool) | Node identity JWT, 24h, renewed at heartbeats | Not read |
| SPIRE | `join_token` attestor: "one-time-use"; or cloud documents, x509pop, sshpop, TPM | Server attests the node; agent trusts the server's bundle | None | Selectors come from the attestor, not from the node's claims | `agent evict` / `ban` (from the multi-machine research) | Re-attest |
| Teleport | Ephemeral token, 30 min default, 1h recommended; static tokens "strongly discouraged"; delegated methods (IAM, OIDC, TPM); bound keypair | Delegated: a third party vouches for the host; bound keypair: host signs "a unique cryptographic challenge" | None | `roles` fixed in the token and written into the certificate; labels only "suggested" | Locks | Bound keypair recovers within a counted limit; reuse detected by join state |
| Boundary | Controller-led: activation token placed in config; worker-led: worker prints a registration request that a person submits | Not read | Worker-led is a person carrying the request to the controller | Not read | Not read | Each worker its own `auth_storage_path` |
| Fleet | Enroll secret per fleet; one-time secrets optional | Not read | None | Fleet chosen by the secret | Not read | A re-imaged Windows host cannot reuse a one-time secret |
| Salt | None: the minion sends its public key | Person compares fingerprints (`salt-key -f`) | `salt-key -a` accepts a pending key; reject, delete | Not read | Delete the key | A rebuilt minion's new key is denied until the old one is deleted |
| Tailscale | Auth key: one-off or reusable, ephemeral, pre-approved, tagged, 1 to 90 days; OAuth client with `auth_keys` scope (keys must carry tags); OAuth app with `auth_keys:create:once`: "single-use", 1 hour | Client authenticates the coordination server (not read in detail); server checks the key | Device approval: the device "cannot send or receive traffic" until an admin approves; pre-approved keys skip it; API `POST /api/v2/device/{id}/authorized` | Tags: "Only designated tag owners can apply tags"; a tagged key gives the device the key's tags | "Revoking a key does not deauthorize nodes using the key"; approval revoked with `authorized: false` | New device; WSL2 inside Windows is a second node with a clashing name |
| NetBird | Setup key: one-off or reusable with a cap, expiry (7 days pre-filled), ephemeral | Not read | Separate "Approve Peers" feature (not read) | Auto-assigned groups from the key | Revoking the key leaves joined peers connected | Not read |

Patterns that recur:

1. **The secret proves membership, not identity.** It is short-lived, single-use where possible,
   scoped to a role, and once used the machine holds its own key (kubelet CSR, SPIRE SVID,
   Tailscale node key, Teleport keypair).
2. **The joining machine pins the control plane** by a hash carried with the secret (kubeadm,
   k3s, Swarm). Where it is skipped, the docs call it unsafe.
3. **What a machine may be is fixed by the issuer, not claimed by the machine:** token groups,
   Swarm's role token, Nomad's node pool, Teleport's roles, Tailscale's tags, NetBird's groups.
   Kubernetes is explicit: self-reported labels are limited to descriptive ones.
4. **Person approval is a separate step from the secret**, and is either the only control (Salt,
   Boundary worker-led) or a second one (CSR approval, Tailscale device approval). Pre-approval
   is an opt-out.
5. **Revoking a join secret never removes a joined machine.** Removal is its own act on the
   machine's identity.
6. **Re-join after reinstall is a new identity**; systems that bind to a key (Salt, Teleport)
   refuse the old name with a new key until a person clears it.

## 4. Threats specific to enrollment

1. **A stolen join secret.** Whoever holds it joins. Bounded by single use, a short lifetime,
   a scope that grants only what the enrollment names, and, in Conduit's v0, the person's
   confirmation, which a stolen secret cannot pass unless the person confirms the wrong machine.
   The confirmation must therefore show something the person can check on the machine itself.
2. **A machine claiming labels that attract sensitive placements.** If the machine chooses its
   labels, a selector such as `conduit.dev/trusted: "true"` sends secrets' consumers to it. Labels
   other than the descriptive ones must come from the enrollment, through the provider's own
   mechanism (Tailscale tags), never from the machine.
3. **A machine claiming an existing hostname.** Under PLC-026 it becomes the existing node, with
   the instances already placed there. This is the most direct gap in the current text.
4. **A rogue control plane.** The new machine joins a network that is not the user's: fixed by
   the provider's own pinning (a CA hash, or the overlay's coordination server), and stated in its
   contract.
5. **Replay.** A secret used twice, or a confirmation applied to a different machine than the one
   shown. Single use, and a confirmation bound to the provider's machine identity, not to a name.
6. **The shown command leaking.** A join command with a secret in it ends up in shell history,
   the engine's records, a terminal an Agent reads, or the context of a model, including the
   coordinator's own session. By ruling 49's logic a model never sees it: the secret goes only to
   the root Interface, is never recorded, and is never in an Agent's or a provider's output that a
   Conduit carries.

## 5. Options for Conduit

### Option A: confirmation only

The language gains a pending admission: a machine reported that no node represents waits for the
root Interface to admit or refuse it, and PLC-026 makes the node only on admission. Joining stays
entirely the provider's: the person installs Tailscale and signs in by hand. Cost: smallest. It
does not answer "much simpler with Conduit", and labels and hostname reuse stay open.

### Option B: machine-led join, no join secret

The person installs the provider's agent and signs in with the provider's own login (for
Tailscale, `tailscale up` and a browser login, as the person). The machine appears needing
approval; the control panel shows it with a code the machine displays; the person confirms, and
the provider approves the device. Like Salt and Boundary's worker-led mode. No secret passes
through Conduit at all. Cost: low; labels must still come from somewhere, and an interactive login
gives a user-owned device, not a tagged one, so tags would be set after admission by the provider.

### Option C (recommended): an enrollment opened by Conduit, completed by a person

The language gains, on `machine-provider.v1`:

- **`enroll.v1`**: input an enrollment ID, the `labels` the machine is to have beyond the
  descriptive ones, and an expiry. Output: one action for a person, a `url` or a `command`, with
  its expiry and whether it carries a secret. The provider makes whatever secret it needs; the
  engine records the enrollment, never the secret.
- **`list-machines.v1` gains** a provider-stable `id` for each machine (for Tailscale, its device
  ID), the `enrollment` it joined by, and a state `pending` for a machine that has joined and is not
  yet admitted.
- **`admit-machine.v1` and `refuse-machine.v1`**, reserved to the root Interface: admission makes
  the node, bound to the machine's `id` as well as its hostname; refusal asks the provider to
  remove the machine, with a provider operation **`remove-machine.v1`**, which a declaration of a
  node lost may also ask for.
- **The pending action** of ruling 50, kind `enrollment`, shown at the root Interface with the
  action, then, once the machine joins, its hostname, `kubernetes.io/os` and `arch`, its `id`,
  the labels it will have, and a short code the machine shows locally, so the person admits the
  machine in front of them and no other.
- **PLC-026 changes:** a node is made only by admission; a machine reporting an existing node's
  hostname under a different `id` is pending, flagged as a conflict, and never that node.
- **PRV-014 changes:** labels beyond `kubernetes.io/hostname`, `os` and `arch` are those the
  enrollment named, bound by the provider so that the machine cannot change them, as the
  provider's contract states. This is a narrow exception to "no operation sets one": the
  enrollment names them once, before the machine exists; nothing changes them afterwards.

What stays the provider's job: the network, the secret's form, pinning the control plane, the
agent on the machine, approval in its own tool, removal. What the person does: asks the control
panel to add a machine with chosen labels; runs one command, or opens one link, on the new
machine; compares the code; admits.

How it fits the multi-machine proposal: enrollment gives network membership and a machine
identity in the provider's tool. It issues no instance credential: those stay per instance, from
the engine's root of trust, delivered by the runtime provider over a channel that authenticates the
machine (proposal 1.4), and PRV-018 still holds, so the tailnet attributes no request to an
instance. The engine stays the decision point. For v1, admission is the natural place to
bootstrap a per-machine identity that renews workload credentials there (research pattern 2), as
Kubernetes approves a CSR; v0 need not do it, and nothing here rules it out.

Cost: four operations, one output field and a state in the core package; changes to PRV-014,
PLC-026 and PLC-024; one kind of pending action; vectors. Moderate, and most of it is the pending
action ruling 50 already requires.

**How Tailscale would implement it.** The provider holds an OAuth client with the `auth_keys`
scope and the tags it may grant, kept in a secret Resource. `enroll.v1` creates a one-off,
tagged, non-ephemeral, not pre-approved key with a short expiry, with the requested labels mapped
to tags the policy file lets that client own; device approval is on in the tailnet, so the device
joins pending. Its action is the platform's install command with the key. `list-machines.v1`
reads the device list, reports unapproved devices as `pending` and maps tags to labels.
`admit-machine.v1` calls `POST /api/v2/device/{id}/authorized`; `remove-machine.v1` deletes the
device. The OAuth-app flow (`auth_keys:create:once`) is the secret-free variant: the action is a
URL; the person consents in Tailscale, and the key never reaches the engine; but it makes a
user-owned device, and I did not find how tags apply there. On Windows, Tailscale recommends the
Windows host only, not inside WSL2, and Tailscale SSH has no Windows server, so a runtime provider
reaches WSL2 by OpenSSH inside it. Whether WSL2 with `networkingMode=mirrored` uses the host's
tailnet I could not verify.

**How the engine's own machine provider answers.** It serves one machine and enrolls none:
`enroll.v1` reports unavailable with a diagnostic naming the missing provider, and `local` is never
pending, since first start makes it.

### Option D: an enrollment agent of Conduit's own

A Conduit binary on each machine joins the engine directly, kubeadm-style. Rejected: it is a
network and agent of Conduit's own, which ruling 40 leaves to existing tools.

## 6. Decisions for the owner

1. **The shape.** A, B, C or D. Recommended: **C**, with B's secret-free URL used wherever a
   provider offers one.
2. **Bind a node to the provider's machine `id`, not only the hostname.** Recommended: yes; a
   hostname reused under another `id` is pending and flagged, never the old node.
3. **Where labels other than the descriptive ones come from.** Recommended: named by the
   enrollment, bound by the provider, unchangeable by the machine; the machine reports only
   `hostname`, `os` and `arch`, as NodeRestriction allows.
4. **Where the join secret may go.** Recommended: only into the action shown at the root
   Interface; never recorded, never in an Agent's or the coordinator's context; a provider
   that can avoid it (a URL) does.
5. **What the person checks before admitting.** Recommended: a short code shown on the machine and
   in the control panel, plus `id`, hostname, `os`, `arch` and labels.
6. **Does refusing, or declaring a node lost, remove the machine at the provider?** Recommended:
   refusing does; declaring lost asks the person, as a second pending action.
7. **Which `kubernetes.io/os` a Windows computer running WSL2 reports.** Recommended: the
   machine the runtime provider runs on, so `linux` for WSL2, which may mean the WSL2 instance is
   the enrolled machine rather than the Windows host; to be tried in A74.

## 7. What would be tested

- **A74** stays manual and engine-free, as ruling 50 says; it adds one finding for this: whether
  Tailscale on the Windows host with mirrored WSL2 networking makes the WSL2 side reachable, and
  which hostname and OS each reports.
- **S3**, once enrollment exists, adds the Windows machine through it and not by hand, with
  vectors: a one-off key used twice is refused; an expired enrollment is refused; a machine that
  joins with the hostname of an existing node is pending and flagged; a machine cannot gain a
  label the enrollment did not name; a refused machine is removed from the tailnet; a node is made
  only on admission; the engine's own machine provider reports enrollment unavailable; and a scan
  of the records, the Agents' transcripts and the coordinator's session finds no join secret.

## 8. Sources

- https://kubernetes.io/docs/reference/setup-tools/kubeadm/kubeadm-join/
- https://kubernetes.io/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/
- https://kubernetes.io/docs/reference/access-authn-authz/bootstrap-tokens/
- https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/ (NodeRestriction)
- https://docs.k3s.io/cli/token
- https://docs.docker.com/engine/swarm/join-nodes/
- https://docs.docker.com/engine/swarm/how-swarm-mode-works/pki/
- https://developer.hashicorp.com/nomad/docs/deploy/clusters/connect-nodes
- https://github.com/spiffe/spire/blob/main/doc/plugin_server_nodeattestor_jointoken.md
- https://spiffe.io/docs/latest/deploying/configuring/
- https://goteleport.com/docs/reference/deployment/join-methods/
- https://goteleport.com/docs/reference/machine-workload-identity/bound-keypair/concepts/
- https://developer.hashicorp.com/boundary/docs/workers/registration
- https://fleetdm.com/docs/using-fleet/enroll-hosts
- https://docs.saltproject.io/en/latest/ref/cli/salt-key.html
- https://tailscale.com/kb/1085/auth-keys
- https://tailscale.com/kb/1215/oauth-clients
- https://tailscale.com/docs/features/oauth-apps/device-provisioning
- https://tailscale.com/kb/1099/device-approval
- https://tailscale.com/kb/1068/tags
- https://tailscale.com/kb/1193/tailscale-ssh
- https://tailscale.com/kb/1295/install-windows-wsl2
- https://docs.netbird.io/manage/peers/register-machines-using-setup-keys

Not verified: Consul's `auto_config`, Puppet's certificate signing and ZeroTier (pages not read);
Nomad's node identity details (from search results and the multi-machine research); the fields
of Tailscale's device-list API and its delete endpoint (the API reference did not load); whether
the Tailscale client pins its coordination server; how tags apply to OAuth-app devices; Tailscale
on Windows with WSL2 mirrored networking.
