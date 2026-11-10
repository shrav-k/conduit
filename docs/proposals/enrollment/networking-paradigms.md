Not binding. The [specification](../../../spec/README.md) alone binds.

# Networking paradigms, and whether enrollment (Option C) generalizes to several parties

Research by the coordinator, 2026-11-10, for the owner's question on the
enrollment proposal, `docs/proposals/enrollment/README.md` on branch
`docs/enrollment-proposal`: "Research other networking paradigms to see if this is an approach that
generalizes ... eventually we may allow for multiple machines to communicate without direct
control."

## 1. The question

Today one person owns one engine, every machine sits on one tailnet that person administers, and
the engine is the only decision point. Option C adds a machine that way: the machine provider
opens an enrollment, the person runs one command, the machine joins pending, the root Interface
admits it, and the node is bound to the provider's machine `id`, with labels named by the
enrollment.

Later, machines, engines and Agents may belong to different parties. No one administers all of
them, no one network holds them all, and no one root of trust is accepted by everyone. The
question is which parts of Option C are sound in that world and which only hold because one
person controls everything, so that v0 does not have to be undone.

## 2. Paradigms compared

| Paradigm | Unit of membership | How identity is proven | Who admits | How trust crosses an administrative boundary | Revocation | Through NAT, no inbound | Maturity |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Tailscale tailnet (also Headscale, NetBird) | Device in one tailnet | Device key, registered with the coordination server by login or auth key | The tailnet's admin (device approval) | Does not, inside one tailnet | Remove the device; revoke approval | Yes (relays) | Production |
| Tailscale node sharing | One machine, shared to one user of another tailnet | As above | Both: an admin shares, an admin of the other side accepts a single-use invite | The sharer's policy applies; the shared machine is inbound-only, and tags and groups are stripped | Revoke the share | Yes | Production |
| Tailnet lock | Device key signed by a trusted signing node | Signature from a key held on the owner's own nodes | Owner's signing nodes, not the coordinator | Removes the vendor's coordinator from the root of trust | Disablement secrets; unsigned nodes refused | Yes | Production |
| ZeroTier | Node in a network of a controller | Address derived from the node's public key | The network's controller (a CA) | A node's identity is its own and works in many controllers' networks; membership is per controller | Certificates of membership with a moving time window | Yes | Production |
| Nebula | Host certificate from a CA | Certificate with name, IP and groups, signed by the CA; host cannot alter it | Whoever holds the CA key | Not covered in the page read (trusting several CAs: unverified) | Not read | Yes (lighthouses) | Production |
| SPIFFE / SPIRE federation | Workload in a trust domain | SVID (X.509 or JWT) issued by the domain's server after attestation | Each domain's server, alone | Each pair of domains exchanges trust bundles, first out of band, then from a bundle endpoint (`https_spiffe` or `https_web`); a foreign ID is only validated, never issued | Bundle rotation; short-lived SVIDs | Not its concern | Production (CNCF) |
| Istio multi-mesh, Linkerd multicluster | Workload in a mesh | mTLS from the mesh CA | Mesh operator | Istio: exchange trust bundles, "Istio does not provide any tooling" for it; Linkerd "requires that there is a shared trust anchor" | Certificate rotation | Via gateways | Production |
| Outbound connectors (Cloudflare Tunnel, Boundary multi-hop, Teleport trusted clusters, Tailscale Funnel) | Connector or leaf cluster | Connector credential; Teleport join token | The service's admin; for Teleport, the leaf joins with the root's token | Teleport: the leaf maps the root's roles to its own, "autonomy to decide"; trust is one-way. Funnel authenticates no caller | Delete the tunnel or trust | Yes, by design: the private side dials out | Production |
| Matrix federation | Homeserver named by DNS | Signing keys published at `/_matrix/key/v2/server`; every request signed (`X-Matrix`) | No one: open by default | Web PKI for TLS; keys checked through notaries the receiver picks; room ACLs | Key validity periods; ACLs | Needs a public endpoint | Production |
| ActivityPub, email (DKIM), XMPP s2s | Server named by DNS | HTTP signatures with `keyId` fetched from the actor; DKIM key in DNS (not read); XMPP dialback over DNS, or certificates | No one | No prior relationship; blocklists per server | Blocking | Needs a public endpoint | Production; signatures inconsistent across implementations |
| libp2p, did:key | A key | Peer ID is the hash (or identity multihash) of the public key; self-certifying | No one | Nothing is shared but the key; an introduction gives it | None built in | libp2p: hole punching and relays (not read) | Production (libp2p); W3C Recommendation (DID Core 2022) |
| did:web, Verifiable Credentials | A DID, a credential | DID document at an HTTPS domain; claims signed by an issuer | The issuer of the credential | Verifier chooses which issuers it trusts | Status lists (not read) | n/a | DID Core standard; methods vary |
| Capabilities: macaroons, UCAN, OCapN | A token or reference | Holder presents it; UCAN chains signatures by DIDs; macaroons chain MACs | The resource's owner issues; anyone may attenuate and pass on | Delegation crosses boundaries by passing the token | Hard: UCAN calls revoking downstream "a last resort"; macaroons rely on caveats | n/a | Macaroons in production; UCAN 1.0 spec; OCapN pre-standardization |
| MCP authorization | A client and a token per server | OAuth 2.1 access token whose audience is the server's canonical URI (RFC 8707); client ID metadata documents give a client an HTTPS URL as its ID "when client and server have no prior relationship" | The resource owner, a person, at the authorization server | Web PKI and OAuth; a server "MUST NOT accept or transit any other tokens" | Token expiry; the AS | Server needs a reachable URL | Specification, widely implemented |
| A2A (1.0) | An agent at an HTTPS URL | Agent Card at `/.well-known/agent-card.json`, optionally JWS-signed; declared schemes: API key, HTTP, OAuth 2.0, OIDC, mTLS | Out of band ("Client obtains credentials (out-of-band OAuth 2.0 flow)") | Web PKI; a client "MAY maintain a trusted key store" | Not stated | Server needs a reachable URL | Released 1.0; under the Agentic AI Foundation |
| ANP, NANDA | Agent | ANP: `did:wba`, a DID document over HTTPS; NANDA: an index resolving to signed "AgentFacts" | Not established | Not established | Not established | n/a | Early: ANP proposal stage; NANDA a research index (MIT Media Lab) |
| Zero trust (BeyondCorp, NIST SP 800-207) | None: no privileged network | User and device credentials on each session | The resource's policy | "no implicit trust ... based solely on ... network location" | Per session | Not its concern | Practice and NIST guidance |

Notes:

1. **Overlays federate badly.** Tailscale's only cross-tailnet act is sharing one machine,
   inbound-only, to a person; "A machine cannot be shared with a tag or accessed by tagged
   machines on another tailnet". An engine on a tagged machine could not reach a partner's
   shared machine at all. Headscale serves one tailnet. Overlays are for one owner's machines.
2. **Every cross-owner system keeps issuance local.** SPIFFE domains only validate each other's
   IDs; Teleport's leaf maps the root's roles to its own; Tailscale applies the sharer's policy.
   No system lets one party issue names or rights inside another's domain.
3. **Cross-owner introduction is mutual and person-approved where the parties are known**
   (Tailscale invite and accept, SPIFFE bundle exchange by both operators, Teleport join token
   plus role map), **and open with blocklists where they are not** (Matrix, ActivityPub, email).
4. **Identity that is the key itself is portable** (ZeroTier, libp2p, did:key): the same identity
   joins many networks, and membership is a separate record per network.
5. **The common substrate across owners is HTTPS with Web PKI plus OAuth**: MCP, A2A, Matrix,
   SPIFFE's `https_web`, CIMD and did:web all use a domain name as the anchor. Outbound connectors
   exist precisely to give a private machine such a URL without opening a port.
6. **The agent protocols say nothing about admission.** MCP and A2A specify how a client
   authenticates to a server already chosen; who decides to trust an unknown agent is left out of
   band. ANP and NANDA try to fill that gap and are early.

## 3. The axes that matter for Conduit

1. **Three layers, kept apart.** Reachability (network membership), identity (who sent this),
   and authorization (may it do this). Overlays bundle the first two and sometimes the third
   (tailnet ACLs). Zero trust insists they be separate. Conduit already separates them in v0:
   PRV-018 forbids taking "the address, the machine, the network, the runtime provider or the run"
   as establishing the instance, and IFC-022 establishes it "from what that request presents". So
   tailnet membership already grants nothing in Conduit. This is the property that generalizes
   most.
2. **Who holds the root of trust.** One CA for everyone (Nebula, Linkerd's shared anchor, the
   engine's DL13 root today); one root per party with exchanged bundles (SPIFFE federation, Istio);
   DNS and Web PKI (Matrix, MCP, A2A); or none, the key itself (libp2p, did:key). Across parties
   only the last three work, because no party accepts another's root over its own names.
3. **Who decides a request.** In every cross-owner system the owner of the target decides. This
   is v1's "decision point per machine" one step further: a decision point per party, deciding
   requests into what that party holds.
4. **Network-level or request-level control.** Network-level (tailnet ACLs, sharing) cannot
   express "this instance may call this operation"; request-level can, and survives any network.
   Conduit's gate and per-instance credentials are request-level already.
5. **Admission by one admin, or mutual introduction.** Enrollment is one admin adding a thing it
   will own. Between parties neither owns the other: each admits the other into its own domain,
   on terms it sets.
6. **Identity or capability.** A Conduit is close to a capability (one consumer, one target, the
   operations it lists), but Conduit checks it at a decision point against live state, which gives
   immediate revocation. Bearer capabilities that travel trade that away.

## 4. Does Option C generalize?

**What survives into multi-party, unchanged in substance:**

- **Enrollment as introducing an identity, then admitting it.** A single-use, expiring
  invitation; the newcomer acts on its side; it appears pending; a person admits it after checking
  something shown on both sides. Tailscale's share invites, SPIFFE's bundle exchange, Teleport's
  join token and Salt's fingerprint check have this shape.
- **Admission by a person at the Interface.** The pending action of ruling 50 is the right
  vehicle for "accept a partner" as well as "admit a machine".
- **Attributes issued, never claimed.** Nebula's groups in the certificate, Teleport's roles,
  SPIFFE's selectors from attestation, Tailscale's tags. Across parties the rule becomes: what
  another party says of its own objects is a claim to be mapped by the receiving party, as
  Teleport's `role_map` does, never a label taken as given.
- **Binding to a stable identity, not a name.** Across parties, the stable identity is a key or a
  SPIFFE ID in the other party's domain, never a hostname.
- **Network membership grants nothing** (PRV-018, IFC-022).

**What holds only because one person controls everything:**

- **One machine provider** (PRV-013). Another party's machines are in another party's provider.
- **One network.** There is no tailnet shared with a stranger; sharing is inbound-only, per
  person, and strips tags.
- **One root of trust.** DL13's mechanism issues every credential and names every instance. No
  other party will accept it issuing names in its domain.
- **A node owned by `system`.** A node is where the engine's runtime provider runs instances it
  places. No other party will let our engine place instances on its machine, short of renting
  compute, which is an infrastructure provider, not a peer.

**Which spec notions would change for several parties:**

- **engine:** "in this version it alone decides every request" becomes "decides every request
  into the objects it holds".
- **node:** stays a machine of the engine's own owner. A machine of another party is not a node;
  what runs there is reached, not placed.
- **Interface:** already "where a person, or a system outside, acts on the system". Another
  party's engine, acting into ours, is an Interface. What changes is how IFC-022 establishes it:
  from a credential issued in the other party's trust domain, validated against that domain's
  bundle, not one DL13 issued.
- **`remote`:** already reaches a server at a URL with MCP authorization. Another party's exposed
  Resource or Agent is a Resource of ours whose implementation is `remote` at the other engine's
  URL. The gate rule for a `remote` server ("the server is its own gate") already fits.
- **Conduit:** one consumer, one target, one engine. A cross-party relationship cannot be one
  Conduit, because neither engine may open a Conduit on the other's objects. It is two: our
  Conduit from our instance to the Resource that stands for the partner's target, and the
  partner's Conduit from the Interface that stands for us to their target. Each side closes its
  own.
- **Who may issue a credential naming an instance:** only the engine whose trust domain holds it.
  A foreign credential is validated, never reissued.
- **root Interface:** each party has its own; introduction of a partner is a pending action at
  each.
- **caller:** across parties the caller chain is a claim from the partner; the receiving engine
  records it but authorizes only on the partner's established identity.

So: the shape of Option C generalizes; its scope does not, and should not try to. Enrollment is
how one party grows its own estate. Multi-party is a second, parallel act, introduction, with the
same shape and different objects.

## 5. Refinements to Option C now

1. **State the layers.** Add to Option C that joining the provider's network grants nothing in
   Conduit: admission makes a node; per-instance credentials (IFC-022, DL13) remain the only proof
   of who sends a request. This is already true; saying it keeps a later network swap, or no shared
   network, from touching access.
2. **Keep enrollment provider-neutral.** `enroll.v1`'s output is an action (`url` or `command`)
   and nothing Tailscale-specific; `list-machines.v1`'s `id` is opaque and scoped to the provider.
   Prefer, where the provider has one, an `id` derived from the machine's key, as ZeroTier and
   libp2p do, so it cannot be claimed by a reinstall.
3. **Name the trust root now.** Give the engine's DL13 root a name, a trust domain in SPIFFE form
   chosen at first start and unique, not `conduit.local`, as Istio asks for "a globally unique mesh
   ID". IDs already in SPIFFE form (multi-machine Q1) then never collide once two engines federate.
   Cheap now, a migration later.
4. **Keep labels local.** Labels are this engine's vocabulary. The provider maps them to its own
   means (Tailscale tags), and nothing assumes another party's tags or claims mean anything here.
5. **Make the pending action generic.** Give it a kind, a subject, what the person checks (a code
   or a fingerprint shown on both sides), an expiry and an outcome (admit or refuse). Sign-in and
   enrollment are the first kinds; introducing a partner can be a later one with no new mechanism.
6. **Scope nodes to the owner.** Say in Option C that enrollment adds a machine its person
   controls; anything of another party is reached by `remote` or acts through an Interface.
7. **No capability tokens in v0.** Conduits stay checked at the decision point, credentials
   audience-bound and per instance, revocation by live check. Across parties, OAuth 2.1 tokens
   with an audience (MCP) carry what a Conduit permits; macaroons or UCAN only if offline
   delegation chains are ever needed, given their weak revocation.
8. **Do not build on tailnet sharing for partners.** It is inbound-only, per person, and refuses
   tagged machines, which Option C makes every enrolled machine.

## 6. A sketch of multi-party later

Two owners, A and B, each with an engine, each the decision point for its own objects.

1. **Introduction.** A's root Interface opens an invitation: single-use, expiring, carrying A's
   trust domain name, its bundle endpoint URL and a fingerprint of its root, as kubeadm pins a CA
   hash. B's person takes it into B's root Interface, which shows a pending action to admit A.
   Both now hold each other's bundle and refresh it from the bundle endpoint (SPIFFE federation,
   `https_spiffe` once bootstrapped, or `https_web`). Either may refuse or later drop the other.
2. **Reachability.** Each engine exposes an `https` URL: public, or through an outbound
   connector (Cloudflare Tunnel, Funnel, a Boundary or Teleport-style reverse tunnel) behind a
   provider. No shared network.
3. **Representation.** In B, A is an Interface, say `partner-a`, with Conduits B opens from it,
   under a bound, to the targets B chooses to expose. In A, each exposed target is a Resource whose
   implementation is `remote` at B's engine URL.
4. **A request.** An instance in A calls the Resource; A's engine decides under A's Conduit, then
   calls B's engine with an MCP token whose audience is B's URL, whose subject names the instance
   by its SPIFFE ID in A's domain (JWT-SVID, or RFC 8693 token exchange), with A's engine as the
   OAuth client identified by a client ID metadata document at A's URL. B's engine validates it
   against A's bundle, establishes `partner-a`, and decides under B's Conduit.
5. **Revocation.** B closing its Conduit ends access at once at B. A closing its own ends it at A.
   Dropping the partner removes the bundle.
6. **Discovery of agents.** If agents are to be found by strangers, A2A's Agent Card at a
   well-known URL, signed, is the existing form; MCP stays the wire Conduit uses.

Standards leaned on: SPIFFE federation and SVIDs; OAuth 2.1, RFC 8707, RFC 9728 and CIMD as MCP
uses them; RFC 8693 token exchange (not read in this pass); Web PKI; A2A Agent Cards; outbound
connector tools as providers. Nothing here is Conduit's own except who may open which Conduit, which
is connection and access, as ruling 40 allows.

## 7. Decisions and open questions for the owner

1. **Adopt "network membership grants nothing; identity per instance; authorization per request
   by the target's owner" as a stated principle.** Recommended: yes, in Option C and as a
   sentence in definitions; PRV-018 and IFC-022 already carry it.
2. **Name the engine's trust domain at first start.** Recommended: yes, in SPIFFE form, unique,
   recorded in status; a small change to DL13 and the first-start promises.
3. **Is another party's machine ever a node?** Recommended: no. Nodes are machines the engine's
   owner controls; other parties are reached by `remote` or act through an Interface.
4. **Cross-party relationships: one Conduit or two halves?** Recommended: two halves, each opened
   and closed by its own engine; no engine opens a Conduit on another's objects.
5. **Make ruling 50's pending action generic** (kind, subject, evidence, expiry, outcome).
   Recommended: yes, now, since it is being written anyway.
6. **Capability tokens.** Recommended: not in v0; for v1 cross-party, OAuth 2.1 audience-bound
   tokens as MCP has them; revisit macaroons or UCAN only for offline delegation.
7. **Network between parties.** Recommended: `https` with Web PKI, or an outbound connector
   behind a provider; not tailnet sharing.
8. **The machine `id` in Option C.** Recommended: required, opaque, provider-scoped, key-derived
   where the provider offers it.
9. **A2A, ANP, NANDA.** Recommended: watch; adopt none in v0. A2A's signed Agent Card is the
   likeliest form if v1 needs discovery by strangers. ANP and NANDA are too early.
10. **Open:** whether a partner's caller chain (which of A's Agents asked) should ever be used in
    B's decisions, or only recorded. Recommended for now: recorded only; B authorizes the partner.

## 8. Sources read

- Tailscale node sharing: https://tailscale.com/kb/1084/sharing
- Tailnet lock: https://tailscale.com/kb/1226/tailnet-lock
- Tailscale Funnel: https://tailscale.com/kb/1223/funnel
- Headscale: https://headscale.net/stable/
- ZeroTier protocol: https://docs.zerotier.com/protocol/
- Nebula quick start: https://nebula.defined.net/docs/guides/quick-start/
- SPIFFE federation: https://spiffe.io/docs/latest/architecture/federation/readme/
- Istio deployment models: https://istio.io/latest/docs/ops/deployment/deployment-models/
- Linkerd multicluster: https://linkerd.io/2-edge/features/multicluster/ and
  https://linkerd.io/2-edge/tasks/installing-multicluster/
- Cloudflare Tunnel: https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/
- Boundary multi-hop: https://developer.hashicorp.com/boundary/docs/concepts/connection-workflows/multi-hop
- Matrix server-server API (first part): https://spec.matrix.org/latest/server-server-api/
- ActivityPub HTTP signatures report: https://swicg.github.io/activitypub-http-signature/
- XMPP dialback: https://xmpp.org/extensions/xep-0220.html
- libp2p peer IDs: https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md
- DID Core: https://www.w3.org/TR/did-core/
- UCAN: https://github.com/ucan-wg/spec
- Macaroons (abstract): https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/
- OCapN: https://ocapn.org/
- MCP authorization (draft): https://modelcontextprotocol.io/specification/draft/basic/authorization
  and client registration: .../authorization/client-registration
- A2A specification 1.0: https://a2a-protocol.org/latest/specification/
- BeyondCorp (abstract): https://research.google/pubs/beyondcorp-a-new-approach-to-enterprise-security/
- NIST SP 800-207 (abstract): https://csrc.nist.gov/pubs/sp/800/207/final
- From the repository: the enrollment proposal, rulings 49 and 50, the multi-machine proposal,
  spec/definitions.md, PRV-018 and IFC-022.

**Unverified or from search results only:** Teleport trusted clusters (search snippets of
https://goteleport.com/docs/zero-trust-access/management/trustedclusters/; the page did not load);
ANP and `did:wba` (secondary sources and the `anp` crate, not the normative spec); NANDA (arXiv
2507.14263 abstract via search, trade press); DKIM (RFC 6376 not read); Nebula trusting several
CAs, group-based firewall rules and relays; libp2p NAT traversal and how Noise checks a peer ID;
NetBird and Tailscale details beyond the enrollment proposal's sources; macaroons' third-party
caveats (paper body not read); RFC 8693; Verifiable Credential status lists; SPIRE nested
deployments; Matrix server ACL rules (page truncated).
