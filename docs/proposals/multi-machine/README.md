# Work across machines: D5, D20 and the road to local enforcement

Status: ruling 43: in force; amended by 58 (M13, M47, M66: later logged choices) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [IFC-041, IFC-042](../../../spec/interface/implementation.yaml), [PRV-022](../../../spec/promises/providers.yaml). This record says why.

The text below is the
proposal as it was put to the owner.

Was: proposed 2026-11-06, for the owner; not ruled. It answers the owner's ruling on
questions 1 and 8 of [ruling 42](../ruling-42/README.md): "I'd like some more research and
clarification on how to solve the multi machine issue and be forward looking even if some
functionality doesn't exist in v0". Nothing here is accepted, and nothing here is part of the
specification. Each decision below is a recommendation that waits for the owner. Until the owner
rules, the text keeps what the coordinator settled: for D5 the gap stated in
[spec/README.md](../../../spec/README.md#what-the-guarantee-for-secrets-covers), and for D20 the
guard under which no runtime provider other than the engine's own runner is asked to run a
program shipped inside a package.

Written on 2026-11-06 against the specification as it stood that day. Promise IDs are those of
[spec/](../../../spec/README.md). Another job was applying ruling 42 to `spec/` as this was
written, so this proposal cites promises by ID, which do not change, and uses the names of
ruling 42: node (was location), machine provider (was machines provider), idempotency key (was
operation identity), `secret-provider.v1` (was `secret-reveal.v1`), `get-secret.v1` (was
`reveal-secret.v1`) and source (was origin).

Facts about other systems are stated, with the URL and the version or date of each page, in the
[research packet on work across machines](../../research/MULTI_MACHINE.md). This proposal links
the packet's sections, and links directly the pages that its main arguments rest on, listed
under [sources](#sources). A sentence about another system is what its page states; the
designs built on them are this proposal's own, and a sentence marked **inference** is reasoning.

**How it was made.** Five researchers covered identity, the shipping of programs, local
enforcement, MCP gateways, and failures across machines. Three designers drew a minimal design,
a design with a node agent on every machine, and a design built on network identity. Two
reviewers judged them for robustness, simplicity, existing paradigms, looking ahead and
swappability. Both chose the minimal design, and named the faults it had and the ideas to take
from the other two. A skeptic then checked the result against the specification and its
sources. This version fixes what the skeptic found: the gate now covers `remote` servers, says
plainly who on the engine's own machine can get past it, and does not rely on renewing
credentials in v0; a cut network is no longer taken for a cancellation; the failover proposal
states outcomes only; and the terms, the bearer profile and the D20 recommendation now match
their sources.

## In brief, for the owner

### The problem

Conduit's engine sits between agents and the things they use. For every request it decides
whether a Conduit allows it, then passes the request to the server that does the work, and tells
that server who is asking and under which limits. When that server runs on another machine, or
is reached at a URL, nothing today stops anything else on the network from calling it directly,
around the engine. The server believes what each request says about who is asking and under
which limits, so a direct call can claim anything. For a secret store's server that means reading
any secret the store can reach. For an Agent it means starting work that no Conduit allows. That
is D5. Kubernetes has the same weak spot by default, and the MCP gateways that pass authority on
to servers each document how they stop callers from going around them.

D20 is smaller. A program shipped inside a Conduit package sits on the engine's machine, so no
other machine can run it today. Behind both questions is the one you asked: how Conduit works
robustly across machines now, with one engine deciding every request as ruled, while leaving
room for v1, where each machine enforces access itself, keeps working when cut off, and a
revocation takes no longer than a stated bound to arrive.

### The recommended architecture

- One engine decides every request in v0, as ruled. Every server that the engine does not run
  itself sits behind a **gate**: the server of a run on any runtime provider other than the
  engine's own runner, and a server reached by `remote` that takes Conduit's keys. The gate lets
  a request through only if it carries cryptographic proof that the engine sent it, and on the
  server's machine nothing else can reach the server. Before it sends anything, the engine checks
  that the server answering is the one it asked for.
- One thing the gate cannot stop, said plainly: programs that run as the engine's own
  operating-system user can read the engine's keys, so they can get through every gate. That
  includes Agents that the engine's own runner, which a user gets by naming no runtime, runs
  under that user. The specification already puts that user inside the trust domain of every
  secret. An Agent that should not be trusted that far runs under a runtime provider that
  isolates it.
- Conduit writes this as a rule, not as machinery. The runtime provider that the user picks
  supplies the gate from existing tools: a TLS proxy such as ghostunnel, a service mesh, a proxy
  that checks tokens, MCP's own authorization for a `remote` server, or `docker run -i` over SSH
  with the check at the provider's end.
- One set of credentials, in standard forms, proves who is who in both directions: mutual TLS
  certificates, or signed tokens in SPIFFE's JWT-SVID form, each naming one instance by its ID.
  In v0 a credential lasts as long as its instance, and the engine checks on every request that
  the instance still exists, so revocation is immediate without renewal.
- The rules are written for roles, as NIST splits them: a **decision point** decides a request
  under the Conduits, and an **enforcement point** lets it through only once one has. In v0 the
  engine is both. In v1 an existing proxy on each machine enforces, and asks Conduit's decision
  point on that machine, with nothing changed in what users write.
- Programs inside packages still run only on the engine's machine in v0, as the coordinator
  recommended. In v1 a provider that declares it can fetch a package by digest and run it
  anywhere, as Nomad and Kubernetes ship files to their machines.

### What v0 promises, and what v1 adds

| | v0 | v1 adds |
| --- | --- | --- |
| Who decides a request | The engine, every time, from the Conduits as they stand | Each machine's decision point decides requests whose target is on that machine, from a copy of the Conduits held under a lease; an existing proxy there enforces |
| Who may reach a server | Only the engine, with proof; nothing else on the server's machine. Programs of the engine's own operating-system user can too, since they can read its keys | Also the enforcement point of the server's own machine |
| Proof of who is who | One credential per instance, naming its ID, lasting as long as the instance, checked against live state at the engine on every request | Short lifetimes, renewed on each machine under that machine's own identity, checked there without asking the engine; the issuer swappable by the user |
| A Conduit closed | Its access ends at once | At once at the engine, and within a stated bound everywhere |
| A machine cut off from the engine | Nothing new starts through Conduit on it, and nothing is retired. Work already sent there may run on until its gate notices the cut | Instances on it go on working with each other until the bound runs out, then stop |
| Programs inside packages | The engine's machine only | A provider that declares it fetches them by digest and runs them anywhere |
| What a user writes | Placement, runtime, Conduits | The same documents, unchanged |

### What you are asked to rule

| # | Question | Options | Recommended |
| --- | --- | --- | --- |
| [D5](#3-d5-who-may-reach-a-server-on-another-machine) | Who may reach the server of a run, or a `remote` server that takes Conduit's keys | **A** every such server, as an outcome; **B** secret stores only; **C** requests to the engine only, as now; **D** no run off the engine's machine | A |
| [D5, second part](#d5-second-part-a-header-that-carries-a-secret) | Where a header that carries a secret may go | **a** `https` only; **b** `https`, or loopback on an engine that never fails over; **c** anywhere, as now | a |
| [D20](#4-d20-packaged-programs-elsewhere) | Programs inside packages, on other machines | **A** the engine's runner only, guarded by the engine, as now, with fetching added in v1 as a feature a provider declares; **B** as A, and every provider also refuses such a program unless it says it fetches packages; **C** a provider fetches packages in v0; **D** a package source on each machine | A |
| [Q1](#q1-forms-of-proof-and-the-names-in-identity) | Forms of proof, and the names in `identity` | **a** two standard profiles, X509-SVID and JWT-SVID in form, with MCP's authorization toward a `remote`, and fixed names; IDs in SPIFFE form recommended; **b** each engine's own, as now; **c** SPIFFE required | a |
| [Q2](#q2-the-engine-across-a-failover) | The engine across a failover | **a** outcomes only, the mechanism the engine's: linearizable decisions, at most one part of the engine acting on an invocation as the state store checks, an address and keys that survive, and the machines that may run the engine kept alike under its hostname; **b** the engine does not move to another machine in v0 | a |
| [Q3](#q3-provider-servers-after-the-engine-restarts) | Provider servers after the engine restarts | **a** an actor sets each up again, as now; **b** the engine runs each again when it first needs it, reading its secrets for the caller of its last setup, a second exception to SEC-011; **c** the runner keeps their values and restarts them | b |
| [Q4](#q4-the-shape-of-v1-recorded-in-a65) | The shape of v1, recorded in A65 | **a** an existing proxy enforces, Conduit's decision point decides from a copy under a lease and refuses once the lease lapses; **b** leave A65 open | a |
| [Q5](#q5-who-on-the-engines-machine-can-get-past-a-gate) | Who on the engine's machine can get past a gate | **a** programs of the engine's own operating-system user, as the trust domain now has it, said plainly; **b** the engine's own runner runs every process under a user that cannot read the engine's keys | a |

Two of these differ from the coordinator's earlier recommendations in the
[open questions after the rewrite](../ruling-41/open-decisions.md#for-the-owner). For D5 the
coordinator recommended B and left A to v1. The research shows that A costs about what B costs,
since it is the same mechanism on more runs, and that a store reached by `remote` has the same
hole as a store's run. For D5's second part the coordinator recommended `https` or loopback. The
sources give loopback only to redirect URIs and to local development, and loopback breaks once
the engine moves to another machine, so this recommends `https` only. For D20 this agrees with
the coordinator.

## Contents

- [1. The architecture](#1-the-architecture)
- [2. Scenarios](#2-scenarios)
- [3. D5: who may reach a server on another machine](#3-d5-who-may-reach-a-server-on-another-machine)
- [4. D20: packaged programs elsewhere](#4-d20-packaged-programs-elsewhere)
- [5. The smaller decisions](#5-the-smaller-decisions)
- [6. How it fits the rulings and the action items](#6-how-it-fits-the-rulings-and-the-action-items)
- [7. What a user writes](#7-what-a-user-writes)
- [8. Provider contracts](#8-provider-contracts)
- [9. The promises that change](#9-the-promises-that-change)
- [10. Left to v1, and what v0 must not preclude](#10-left-to-v1-and-what-v0-must-not-preclude)
- [11. An experiment across two machines](#11-an-experiment-across-two-machines)
- [12. Costs, and what this gives up](#12-costs-and-what-this-gives-up)
- [Sources](#sources)

## 1. The architecture

### 1.1 The path across machines today

1. The servers of the runtime, machine and package-source providers run on the engine's node,
   `local`, on the engine's own runner (PRV-002). Each reaches other machines by its own means.
2. The machine provider places an instance on a machine, and the engine registers one node for
   each machine it reports (PRV-015, PLC-026). A placement is fixed for life (PLC-009).
3. A setup opens the instance's Conduits, reads its secrets, then asks its runtime provider for
   the run (TPL-012, SEC-010, PRV-007). `run.v1` carries `identity`: the engine's address and a
   credential file, filled by the mechanism of DL13 (PRV-022, D17).
4. The runtime provider runs the item on the named machine and returns an `endpoint` (PRV-019,
   IFC-017). The engine sends requests there over Streamable HTTP, with Conduit's keys in `_meta`:
   `caller`, `restrictions`, `config` and `revision` (IFC-006, IFC-018). The server takes them as
   given. An item reached by `remote` that is not `plain` receives the same keys (IFC-037,
   IFC-041).
5. A run sends its own requests to the engine, which establishes the instance by cryptographic
   proof wherever the request crosses machines (IFC-022, DL13), and decides each request anew
   (ACC-036).
6. The gap: nothing limits who reaches an `endpoint`, and the engine proves nothing to it. To a
   `remote` item it proves itself only by the header values that the instance's configuration
   gives (IFC-041), which are the same for every request. `spec/README.md` records the gap, and
   ruling 42 left it open as D5.

### 1.2 Decision point and enforcement point: roles, not machines

Two new terms, for [spec/definitions.md](../../../spec/definitions.md), split as NIST splits
them:

- A **decision point** decides a request under the Conduits. NIST's policy decision point
  "Computes access decisions by evaluating the applicable digital policies and metapolicies"
  ([NIST glossary, PDP][nist-pdp]).
- An **enforcement point** is a component through which a request passes on its way to a server,
  and that passes it on only once a decision point has permitted it. NIST's policy enforcement
  point "Enforces policy decisions in response to a request from a subject requesting access to a
  protected object", while "the access control decisions are made by the policy decision point"
  ([NIST glossary, PEP][nist-pep]).

In v0 the engine is the only decision point and the only enforcement point. In v1 an existing
proxy in front of the servers on each machine is an enforcement point, and asks Conduit's
decision point on that machine ([Q4](#q4-the-shape-of-v1-recorded-in-a65)).

Every new rule below names a role, never "the engine". "Mediated" in ACC-027 becomes "passes
through an enforcement point". The definition of the engine also changes: it may be made of
several parts on several machines, and a promise that says what the engine keeps, records or
does binds the whole, whichever part does it. That is how v0 leaves B of ruling 40's point 3f
open: [section 6](#ruling-40-point-3f-and-a65) shows that only the promises on how and when a
decision is made then assume one place of decision.

### 1.3 The gate in front of every server the engine does not run itself

The server of every run that a runtime provider other than the engine's own runner makes sits
behind a gate, and so does every server reached by `remote` that takes Conduit's keys. The gate
admits a request only with proof that an enforcement point sent it, and keeps everything else on
the server's machine away from the server. For a `remote` server, the server is its own gate.
The engine, for its part, checks who answers before it sends. The provider picks the means.
[Section 3](#3-d5-who-may-reach-a-server-on-another-machine) gives the rule and the means.

Kubernetes closes the same gap for its own node agent with two flags: the kubelet checks the API
server's client certificate (`--client-ca-file`), and the API server checks the kubelet's serving
certificate (`--kubelet-certificate-authority`)
([kubelet authentication][k8s-kubelet-authn]; [control plane to node][k8s-cpn]). The gate is
that pattern, made the default and moved in front of every server. Kubernetes itself leaves the
API server's own traffic to Pods unauthenticated ([control plane to node][k8s-cpn]), so for
workloads it is a precedent for the weak default, not for the gate.

### 1.4 One way to prove who is who

One root of trust, held by DL13's mechanism, issues every credential, in standard forms. The
proposal is in [Q1](#q1-forms-of-proof-and-the-names-in-identity). In short:

- **Two profiles.** Mutual TLS with X.509 certificates in the form of an X509-SVID
  ([X509-SVID][spiffe-x509]); or a signed JWT in the form of a JWT-SVID, over TLS, whose `aud`
  names the receiver ([JWT-SVID][spiffe-jwt]). Toward a run, the audience is the instance's ID,
  since its endpoint's URL may change. Toward a receiver at a stable `https` URL, the engine's own
  address or a `remote` server, the token is MCP's authorization proper: its audience is the
  receiver's canonical URI, which MCP requires ([MCP authorization][mcp-authz]) and which RFC
  9728 makes "a URL that uses the `https` scheme and has no fragment component"
  ([RFC 9728][rfc9728]).
- **Named by ID.** A run's credential names its instance by ID, never by name, and names none
  once the instance is removed, even when a new instance takes the name. Kubernetes binds a token
  to an object's `uid` for the same reason ([service account administration][k8s-sa-admin]).
- **As long as the instance, in v0.** A run's credential, and the certificate its gate serves,
  last as long as the instance. Revocation is the engine's check against live state on every
  request (IFC-022, IFC-032), as the Kubernetes API server checks a bound token "When a bound
  token is presented to the kube-apiserver" ([service account administration][k8s-sa-admin]).
  Short lifetimes would buy nothing in v0 for requests to the engine, and nothing could renew
  them: PRV-022 lets a provider replace a file in place, but no provider operation gives it new
  content. Renewal comes in v1, on each machine under the machine's own identity, as the kubelet,
  the SPIRE agent and the Nomad client renew workload credentials under theirs
  ([research, 6](../../research/MULTI_MACHINE.md#6-patterns-across-the-systems)). The engine's
  own credential, which it mints for itself, may be short-lived at no cost.
- **A trust bundle beside the credential**, so that a run can check who answers at the engine's
  address, as Kubernetes nodes get the cluster's root certificate ([control plane to
  node][k8s-cpn]). The engine keeps each root for as long as a run that holds it lasts.
- **Delivered in confidence.** A runtime provider delivers `secrets` and `identity` to a machine
  only over a channel that authenticates that machine and keeps them confidential, and its
  contract names the channel.
- **Signed, so it can be checked without the engine.** v1's decision points check credentials
  against published keys, as outside parties check Kubernetes and Nomad tokens through a JWKS
  ([research, 1.1 and 1.2](../../research/MULTI_MACHINE.md#11-kubernetes)).

### 1.5 The engine across a failover

Ruling 40 point 3f A says the engine survives the loss of a machine by keeping its state in a
store that does, and point 2 leaves the engine's own state to each engine "for now". So
[Q2](#q2-the-engine-across-a-failover) proposes outcomes only, and leaves the mechanism to the
engine: decisions read the state linearizably; at most one part of the engine acts on any one
invocation, as the state store checks, since a fencing check has to be made by the resource
being protected ([Kleppmann][kleppmann]); runs reach an engine at an address that survives the
move; the signing keys survive it; and the machines that may run the engine are kept alike under
one hostname, so that `local` still represents one hostname. Several engines may then serve at
once, as several instances of the Kubernetes API server do behind a load balancer
([Kubernetes architecture][k8s-arch]), or one may lead under a lease, as the controller manager
does ([Leases][k8s-leases]). Which is the engine's choice.

### 1.6 What Conduit writes, and what existing tools bring

| Conduit writes (connection and access) | Existing tools bring |
| --- | --- |
| Who may reach a server: only an enforcement point, with proof | Mutual TLS proxies (ghostunnel), meshes (Istio, Linkerd), proxies that check JWTs (Envoy), MCP authorization, stdio bridges, `docker run -i` over SSH, SSH and outbound tunnels |
| Which instance a credential names, and for how long | X.509 and JWT, SPIFFE IDs, issuers such as an engine CA, SPIRE, cert-manager or Vault |
| What a request reports when proof fails or a machine is cut off | TLS handshakes, HTTP 401, keepalives |
| What a program is, by digest, and who may run it | OCI registries and descriptors, mirrors, MCPB, npm and PyPI |
| In v1: what a decision point decides, and the bound on a revocation | Proxies that speak Envoy's external authorization API (Envoy, agentgateway) |

Conduit ships no proxy, tunnel, workload runner or certificate format of its own. In v1 it does
run code on each machine: its decision point, because deciding access is Conduit's own job
(ruling 40, point 1). That decision point sits behind an existing proxy, and the format of the
copy it decides from is published, so that another implementation can serve it.

## 2. Scenarios

M1 is the engine's machine. Each scenario says what breaks today, what the recommendation does
in v0, and what v1 adds.

### S1. An Agent on M2 uses a tool server on M3 through the engine on M1

- **Today.** The tool server's endpoint admits anything that reaches it. A direct call can send
  any `caller`, `restrictions: []` and any `config` (IFC-006, IFC-018), so every Conduit to that
  server, and every restriction on it, becomes advice. The Agent's own endpoint is open the same
  way, so anything can start model work on it outside any Conduit, against IMP-001. Nothing
  promises that the path from M1 to M3 is confidential.
- **v0.** Both endpoints sit behind gates. The engine proves itself to each and checks that each
  serves the instance it names. Requests and results cross confidentially. The Agent calls the
  engine with its own credential (IFC-022). Every request goes M2 to M1 to M3 and back, so if M1
  is down nothing works: that is option A of ruling 40, point 3f, as ruled.
- **v1.** The same documents. The proxy on M3 enforces, and M3's decision point decides requests
  to the tool server from its copy of the Conduits, so the decision is made on the receiving
  side, as Consul, ztunnel and Tailscale do
  ([research, 3](../../research/MULTI_MACHINE.md#3-node-agents-and-local-enforcement)).

### S2. A secret store's server on M2, read by the engine on M1 for an instance on M3

- **Today.** Anyone who reaches the store's endpoint can send a locator of their choosing and
  read any secret the store's own credential reaches (D5). The same holds for a store registered
  as `remote: {url: …}` that takes Conduit's keys: it learns its locator from `config` (SEC-006),
  and the engine proves itself to it only by a header value from configuration, which anyone who
  holds it can send. The value crosses M2 to M1 and M1 to M3 with no promise that it stays
  confidential.
- **v0.** The store's run is gated like any other, so only the engine can ask it, and the store's
  package needs no new code. A store reached by `remote` checks the engine's proof itself, under
  MCP's authorization or mutual TLS, so it must be told the engine's issuer; a store that cannot
  be told it is used as `plain`, and receives no locator from Conduit. The value crosses only
  between the gate and the engine, in confidence, and reaches M3 over the runtime provider's
  authenticated, confidential channel. While M2 is unreachable every read through that store
  reports unavailable (PLC-023, SEC-012); the guidance is to place a store where the engine
  reaches it reliably.
- **v1.** Reads stay with the engine, in setups, as now (SEC-010, SEC-011, SEC-023). Reading
  secrets on each machine, as Kubernetes' Secrets Store CSI driver does, is not adopted.

### S3. A Docker-run Agent on M2

- **Today.** It works by D17, but its endpoint is open, its credential is not tied to its ID, and
  it names an engine address that may not survive a failover.
- **v0.** The Docker provider reaches M2's daemon over SSH or TLS, both of which Docker supports
  ([research, 4.5](../../research/MULTI_MACHINE.md#45-tools-that-keep-a-server-off-the-network)).
  It runs the Agent's container with a gate, in one of two forms. Either ghostunnel runs in the
  container's own network namespace, with the server on loopback inside it. Or the provider runs
  the server with `docker run -i` over the daemon's SSH channel, as Docker MCP Gateway runs its
  servers over stdio ([clientpool.go][docker-mcp-clientpool]), and the provider's own end on M1
  checks the engine's proof; then the server has no listener on M2 at all. `identity` carries the
  trust bundle, and the credential names the Agent's ID and lasts as long as the Agent.
- **v1.** The address in `identity` names M2's enforcement point, and the credential is renewed
  on M2 under M2's own identity. A provider that checks at its own end on M1 adds a gate on M2 to
  take part in local enforcement.

### S4. A server shipped inside a package, to run on M2

- **Today and v0.** It runs only on the engine's runner, on M1. A server meant for M2 is
  published as a pinned `package` item: an OCI image pinned by digest, an npm or PyPI package
  with its digest, or an MCP bundle with its `fileSha256`.
- **v1.** A provider that declares the feature is given the package as an OCI descriptor,
  fetches it from any listed place, checks the digest, and runs it behind its gate. The engine
  sends such a run to no provider that does not declare it.

### S5. A partition between M1 and M2, with requests in flight

- **v0.** A silent cut, where packets are dropped, closes nothing that either side sees at once.
  The engine ends a request it stops waiting for with its outcome unknown (DL3, DL10, OPS-007).
  The server on M2 may not learn of the cut for a long time: Linux sends its first keepalive
  probe after two hours by default, and its default retransmissions give "a hypothetical timeout
  of 924.6 seconds", which "is a lower bound" ([ip-sysctl][kernel-tcp]). MCP has a server treat
  the close of an SSE response stream as a cancellation ([Streamable HTTP][mcp-shttp]); a silent
  cut is not a close, and a response sent as one JSON object has no stream to close. So the work
  on M2 may run to its end, side effects included, while the engine records unknown
  (**inference**). The guidance follows: instances off the engine's machine send idempotency
  keys, long work uses tasks (IFC-016), and an unknown outcome is reconciled (OPS-014). To bound
  how long work goes on unseen, a gate may close the server's stream once the engine's side goes
  silent, as Envoy aborts a connection whose peer does not answer its HTTP/2 PING in time
  ([Envoy protocol options][envoy-keepalive]); the provider's contract states its gate's bound,
  and where the server answered with SSE, MCP then has it treat the close as a cancellation. Once
  M2 is reported unreachable or left out of a report, new requests to it report unavailable and
  nothing is retired (PLC-023). With the gate, nothing on M2's network can reach a target there
  around the engine.
- **v1.** M2's decision point goes on deciding requests between instances on M2 while its lease
  holds, then refuses new ones. Envoy's `ext_authz` refuses by default when the decision service
  cannot be reached ([ext_authz][envoy-extauthz]). Records are kept on M2 and sent later.

### S6. M2 unreachable, then back

- **v0.** Requests report unavailable meanwhile, and are decided as any other on return.
  Credentials last as long as their instances, so none has lapsed in the meantime. Stops the
  provider owes are carried out (PRV-021).
- **v1.** The decision point resumes its copy from its last version, or fetches it whole when
  that version is gone, as a Kubernetes client does after `410 Gone`. The proxy closes open
  streams that the new state no longer allows, as ztunnel does
  ([research, 3.1 and 3.5](../../research/MULTI_MACHINE.md#35-istio-ambient-ztunnel)).
  Credentials that lapsed while M2 was cut off are renewed there under M2's own identity.

### S7. M2 reported gone, then back

- **v0.** Its node and instances are retired and their Conduits closed (PLC-024, CON-008). The
  stop is asked for, and once more when each instance is unregistered (LIF-036, D11). When M2
  comes back, the old runs are inert through Conduit: their credentials name removed IDs, so the
  engine refuses them (IFC-032), and their gates admit only an enforcement point, which sends
  nothing to a removed instance. What they can do outside Conduit with the secrets they hold
  remains a stated limit. One fix is needed: "gone only where the provider knows the machine no
  longer exists" moves from PRV-014's observable into its rule, because Kubernetes' force
  deletion shows that "gone" on silence "can lead to the duplication of a still-running Pod"
  ([force delete][k8s-force-delete]).
- **v1.** M2's decision point stopped allowing new requests at most one bound after the cut. On
  return it learns that its node is retired, and M2 must register as a new node, as SPIRE's
  `agent evict` takes a node's attestation away
  ([research, 1.4](../../research/MULTI_MACHINE.md#14-spiffe-and-spire)).

### S8. The engine restarts, or fails over to M4

- **v0, restart.** Invocations in progress end unknown (IMP-007). Connections to `remote` items
  read their secrets again (SEC-023, D12). Runs on other machines keep running, and their
  credentials still verify, because the keys are not lost. Provider servers, which are child
  processes of the engine's runner, end with it:
  [Q3](#q3-provider-servers-after-the-engine-restarts) proposes that the engine runs each again
  when it first needs it.
- **v0, failover.** Under [Q2](#q2-the-engine-across-a-failover): decisions read the state store
  linearizably; at most one part of the engine acts on any one invocation, as the store checks;
  the keys survive the move, and the runtime profile says where they are held; runs reach an
  engine at an address that survives the move; stops a provider owes survive it; and M4 is one of
  the machines kept alike under the engine's hostname, so `local` still represents one hostname.
- **v1.** Decision points keep deciding under their leases, so a failover shorter than the bound
  does not interrupt traffic within a machine. Copies carry the store's revision, and a decision
  point ignores a copy older than the one it holds, as a Kubernetes watch resumes from a
  `resourceVersion`.

### S9. A Conduit closed while M2 is cut off

- **v0.** The close takes effect at the engine at once (ACC-011, CON-008). Nothing new starts
  through Conduit on M2 while it is cut off, and the gate keeps anything on M2 from reaching a
  target directly. Values already delivered stay with their runs (SEC-014), a stated limit.
- **v1.** M2's decision point allows requests through the closed Conduit until it hears of the
  close or its lease lapses, so within the bound. Keeping the last configuration when cut off, as
  Envoy, OPA and Tailscale do, and extending caches, as Nomad and Consul do, leave a revocation
  with no bound, so they are not adopted
  ([research, 3.9](../../research/MULTI_MACHINE.md#39-comparison)). Consul is the precedent for
  stating the bound: its leaf certificate lifetime "is also the effective limit on how long a
  server outage can last" ([Consul CA][consul-ca]).

## 3. D5: who may reach a server on another machine

### The question

Does ruling 40 point 3e, proof of the sender on every request that crosses machines, also bind
what the engine sends to a server on another machine, so that the server accepts only the
engine? The full framing is in the [open questions][od-d5]. The framing speaks of a run's
endpoint; a server reached by `remote` that takes Conduit's keys has the same hole, so this
section covers both.

### The options

**A. Every such server, as an outcome (recommended).** Every endpoint that a runtime provider
returns, and every server reached by `remote` that takes Conduit's keys, admits only a request
with proof that an enforcement point sent it, and nothing else on the server's machine reaches
the server. The engine checks who answers before it sends. A provider's contract names its means;
the operator of a `remote` server configures it to trust the engine's issuer.

- Precedent: Kubernetes' kubelet with `--client-ca-file` and Webhook authorization, with the API
  server's `--kubelet-certificate-authority` ([kubelet authentication][k8s-kubelet-authn];
  [control plane to node][k8s-cpn]); Istio's STRICT mode, where "Workloads only accept mutual TLS
  traffic" ([Istio security][istio-security]); AWS AgentCore, where the gateway is "the only
  entry point" and the runtime "validates that source" ([AgentCore][aws-agentcore-src]);
  Cloudflare, which warns that users can bypass its policies by a server's direct URL and
  advises "configure Access as the server's OAuth provider" ([MCP portals][cf-portals]).
- Cost: every runtime provider other than the engine's own runner must gate. Publishing a
  container port with `docker run -p` alone no longer conforms, nor does plain HTTP on a LAN. A
  server reached by `remote` that takes Conduit's keys must be told the engine's issuer; one that
  cannot be told it is used as `plain`, and receives none of Conduit's keys.

**B. Secret stores only.** The same rule, for the runs of secret Resources and the `remote`
servers they select alone. Every other provider's contract states who else can reach its
endpoints.

- Precedent: Kubernetes protects the path of a Secret's value, over TLS and a node-local socket,
  while it leaves Pods open ([CSI providers][csi-providers]; [control plane to node][k8s-cpn]).
- Cost: it needs the same mechanism, so it saves little. It leaves every other server open to a
  forged `caller`, `restrictions` and `config`, and every Agent open to work outside a Conduit.
  Its loopback form, a store reached on the engine machine's loopback, breaks once the engine
  moves to another machine.

**C. Requests to the engine only, as now.** Point 3e covers requests to the engine, and the gap
stays stated in `spec/README.md`.

- Precedent: Kubernetes leaves who reaches a Pod to network policy or a mesh; "By default, a pod
  is non-isolated for ingress" ([network policies][k8s-netpol]).
- Cost: a store's server stays open to anything that reaches it.

**D. No run off the engine's machine in v0.** This contradicts ruling 40 point 3b, revised.

### Recommendation: A

1. **The hole is every server, not only stores.** A server takes Conduit's keys as the request
   gives them (IFC-006), so a direct call voids every Conduit and restriction to that server, and
   can start model work on an Agent outside any Conduit (IMP-001).
2. **The cost is close to B's.** It is the same mechanism on more runs. The runtime provider
   already delivers `identity` to each run (D17), and the engine needs one TLS client.
3. **Every gateway that passes authority on guards against callers going around it.**
   Cloudflare warns that blocked users "can still connect to the server (and bypass your Access
   policies) by using its direct URL"; AWS makes its gateway "the only entry point"; Consul and
   Nomad bind the application to loopback behind a proxy
   ([research, 4.4][r-44]).
4. **v1 builds on it.** The outcome that only an enforcement point reaches a server is what a
   machine's own enforcement point takes over in v1. A gate on the server's machine is where it
   attaches; a provider that checks at its own end on the engine's machine adds one then.

### The gate, in substance

A new promise in [providers.yaml](../../../spec/promises/providers.yaml), numbered when written:

- **When** a runtime provider other than the engine's own runner returns an `endpoint` for a
  run; or the engine connects to an item reached by `remote` that is not `plain`, to which it
  sends Conduit's keys (IFC-037, IFC-041).
- **Then:**
  1. A request reaches the server only with proof, by a mechanism of DL13, that an enforcement
     point sent it, in a form that no server that receives it can replay to another. In v0 the
     engine is the only enforcement point.
  2. The proof is checked by the server itself, by a gate between the network and the server,
     or, for a run, by the provider's own end of a channel that it holds to the run's machine,
     where that channel authenticates the machine and keeps what crosses it confidential and
     unaltered.
  3. Nothing else on the server's machine reaches the server: it listens on stdio, on a Unix
     socket that only the gate's operating-system user can open, or in a network namespace that
     no other run shares; or, for a server that is its own gate, only behind its own check. Root
     and the operating-system user or group that the provider uses on that machine, which its
     contract names, are excepted.
  4. Before the engine sends, the server's side proves who it is: for a run, that it serves that
     instance, by a certificate that names the instance's ID, or by being the provider's channel
     to the process it started for that instance; for a `remote` server, by TLS for the host of
     its URL. Between the engine and the side that checks the proof, the request and its result
     stay confidential and unaltered.
  5. The provider's contract states its means, and how long its gate takes to close a server's
     stream once the engine's side of the connection has gone silent.
- **Observable.** The engine's own runner meets it by stdio. A provider that runs `package`
  items of the `stdio` transport bridges them behind its gate, or refuses them. A server takes
  `caller`, `restrictions` and `config` as the request gives them, so this promise is what makes
  a Conduit binding on a server on another machine, a secret store's among them. Programs of the
  engine's own operating-system user can make the engine's proof
  ([Q5](#q5-who-on-the-engines-machine-can-get-past-a-gate)), so the gate keeps out everything
  else, not them.

And the engine's side, in IFC-017 and IFC-041. The engine sends a request to a run's endpoint
only once that endpoint has proved it serves the instance, and to a `remote` server that takes
Conduit's keys only over `https`. It presents proof as an enforcement point. Under the JWT
profile, the token's audience is the instance's ID for a run, never the endpoint's URL, which may
change; for a `remote` server it is the server's canonical URI, as MCP's authorization has it
([MCP authorization][mcp-authz]). The server's protected resource metadata names, as its
authorization server, the engine's issuer, or one that the user runs and that issues tokens to
the engine as a pre-registered client under the client-credentials extension
([client credentials][mcp-cc]). Where the engine cannot verify who answers, nothing is
dispatched, the request or the part of the setup reports unavailable, and its record says why.
The engine reaches the latest endpoint its provider reported, through a new optional `endpoint`
output of `inspect-run.v1`. Under the JWT profile the engine's token takes the `Authorization`
header, so a `remote` item whose `headers` also names `Authorization` collides with it; whether
such an item must then use mutual TLS, or may not name that header, is a detail for whoever
writes the promise.

### Means a provider may use

| Means | Where the proof is checked | How nothing else reaches the server | Precedent |
| --- | --- | --- | --- |
| A mutual TLS proxy beside the server | The proxy admits the engine's certificate, as ghostunnel's `--allow-uri` does, and serves the instance's certificate | The server listens on a Unix socket or on loopback inside a network namespace that only the proxy and the server share | [ghostunnel][ghostunnel]; [kubelet authentication][k8s-kubelet-authn] |
| A service mesh | The mesh's proxy admits only the engine's identity | One network namespace per workload, strict mutual TLS | [Istio ambient][istio-ambient-dp]; [Istio security][istio-security] |
| A proxy or server that checks JWTs | It checks a token in JWT-SVID form whose audience is the instance's ID; Envoy's `jwt_authn` checks a configured list of audience strings | As the first row, or the server is its own gate | [Envoy JWT authentication][envoy-jwt]; [JWT-SVID][spiffe-jwt] |
| MCP's authorization, for a `remote` server | The server validates a token whose audience is its canonical URI, and answers 401 otherwise | The server is its own gate | [MCP authorization][mcp-authz]; [RFC 9728][rfc9728]; [MCP portals][cf-portals] |
| A stdio bridge on the run's machine | The bridge checks the proof, then writes to the server's standard input | stdio, with no network for the server | ToolHive's proxy carries HTTP to a container's standard streams; it works with a container runtime such as Docker or Podman and binds 127.0.0.1 unless `--host` says otherwise, so on the run's machine it would be told to listen further ([ToolHive][toolhive-run]) |
| stdio over the runtime's own channel | The provider's own end, on the engine's machine, checks the proof; SSH or verified TLS to the daemon authenticates the run's machine | `docker run -i` gives the server no listener on the run's machine | [Docker MCP Gateway][docker-mcp-clientpool]; [Docker daemon access][docker-protect] |
| A tunnel to the run's machine, including one the run's machine dials out | Reachability only: the proof travels inside, and one of the rows above checks it | As that row | [Konnectivity][k8s-cpn]; [Cloudflare Tunnel][cf-tunnel]; [OpenSSH][openssh] |

What does not count:

- **Network position alone**, such as an address, a VPN or a network policy. It names a machine,
  never a run, and PRV-018 already says a machine's address attributes nothing.
- **A shared static secret**, such as the `X-Gateway-Secret` of Microsoft's gateway
  ([README][ms-mcp-gw]), or a header value from configuration that is the same for every request
  (IFC-041 today). It is not bound to one server, so a server that receives it can replay it to
  another, and anyone who holds it can send any `config`.
- **A near end that admits without the proof.** OpenSSH binds a forwarded port to loopback
  ([ssh_config][openssh-config]), where every process on the engine's machine can reach it,
  whatever its user. The provider's end of a channel counts only where it checks the proof.
- **Loopback on a machine where other runs share it.** "Publishing container ports is insecure
  by default", and in Docker releases older than 28.0.0 other hosts on the same L2 segment can
  reach ports published to localhost ([port publishing][docker-ports]). MCP warns that local
  servers "may be accessible to other processes running on the user's machine"
  ([best practices][mcp-bp]).

### D5, second part: a header that carries a secret

May the engine send a header that carries a secret to an `http` URL (IFC-041, SEC-013)?

- **a. `https` only (recommended).** The simplest rule, and the one the sources support. OAuth
  2.1 says "All the OAuth protocol URLs (URLs exposed by the AS, RS and Client) MUST use the
  `https` scheme except for loopback interface redirect URIs" ([OAuth 2.1][oauth21]); a server
  that is sent a bearer secret is in the position of a resource server (RS), which gets no
  loopback exception (**inference**, by analogy). MCP allows `http://` on loopback only for
  authorization URLs "during local development" ([best practices][mcp-bp]). Cost: a local server
  that needs a secret header cannot be reached at `http://localhost`; during development it runs
  as a `command` or `package` item on the engine's own runner, over stdio, which needs no header
  (**inference**).
- **b. `https`, or a loopback host on an engine that never fails over.** This was the earlier
  recommendation here, and it misread its sources, as above. It also makes whether a document
  works depend on a flag of the runtime profile: turning failover on breaks every `http`
  loopback `remote` that carries a secret header. The coordinator's rule, `https` or loopback,
  breaks on a failover in the same way: after the engine moves, a `remote` item at
  `http://127.0.0.1` sends the secret to whatever listens on the new machine, and SEC-023 makes
  exactly that read after a failover.
- **c. Anywhere, as now.** The secret may cross the network in clear.

### What v1 adds to D5

- On each machine, an existing proxy in front of the servers becomes that machine's enforcement
  point. It admits a request from an instance there, or from another machine's enforcement
  point, once its own machine's decision point permits it. The credential names the role and the
  machine, as a kubelet may read only what concerns Pods "bound to the kubelet's node"
  ([Node authorization][k8s-node-authz]).
- A provider may then bind endpoints to the machine alone, since the machine's enforcement point
  is local. Its outbound connection to the engine can carry what the engine sends, as
  Konnectivity agents do ([control plane to node][k8s-cpn]).
- Proof bound to the sender's key may be required: RFC 8705, DPoP or WIMSE
  ([research, 1.7](../../research/MULTI_MACHINE.md#17-mcp-authorization-and-the-standards-it-uses)).

## 4. D20: packaged programs elsewhere

### The question

A `command` item may run a program shipped inside a package, such as `bin/files-server`, which
sits on the engine's machine. Should such an item run only on the engine's own runner, or may
another runtime provider fetch the package and run it on any machine? The framing is in the
[open questions][od-d20].

### The options

**A. The engine's own runner only, guarded by the engine, as now (recommended).** The engine
never asks another provider for such a run (PRV-007). A server meant for another machine is
published as a `package` item. In v1, fetching comes as a feature that a provider declares, by a
new version of the operation or a feature it reports, and the engine keeps its guard for every
provider that declares nothing, so no v0 provider ever receives such a run.

- Precedent: a container runtime runs only what it pulls by reference or digest
  ([CRI][cri-api]); MCP clients check an MCP bundle's `fileSha256` before installing it
  ([package types][mcp-pkg]). For the v1 addition: a CRI runtime reports in its status which
  features each of its handlers supports, as `RuntimeHandlerFeatures` ([CRI][cri-api]), and a
  Kubernetes mount that needs one the runtime lacks "will fail" ([volumes][k8s-volumes]).
- Cost: none in v0.

**B. As A in v0, and every provider refuses such a program unless it says otherwise.** The guard
is also written in PRV-012's form, as a duty of every runtime provider: one whose contract does
not say it runs packaged programs ends such a run failed and runs nothing. This was the earlier
recommendation here.

- Precedent: PRV-012 has a provider refuse a registry type its contract does not list.
- Cost: a duty that no conforming v0 engine exercises, since the engine's guard means no provider
  receives such a run, so no vector can test it through an engine. The hazard it guards against
  arises only if v1 adds an optional input to `run.v1` that a v0 provider would ignore, and v1
  need not add it that way.

**C. A provider fetches the package in v0.** `run.v1` gains the package's download places and
digest now, and a provider that opts in runs it.

- Precedent: Nomad's `artifact` block ([artifact][nomad-artifact]); Kubernetes image volumes,
  stable since v1.36 ([image volumes][k8s-imgvol]).
- Cost: fetching before any machine identity can limit which machine fetches what, and a package
  installed from a `file:` URL still cannot be fetched elsewhere. No v0 provider needs it.

**D. A package source on each machine.** A per-machine fetcher of Conduit's own.

- Cost: it duplicates the fetcher every runtime platform already has (CRI's image service,
  Nomad's client), and conflicts with PRV-002, under which package sources run on `local`.

### Recommendation: A

It keeps the norm of the protocol's ecosystem in v0, publishing pinned packages, and needs no new
text. To run a server elsewhere in v0, its author publishes it as a `package` item: an OCI image
pinned by a digest in its identifier (N7 of ruling 42), an npm, PyPI, NuGet or cargo package with
its digest (MAN-010), or an MCP bundle with its `fileSha256`. The valid example's Pool template
`workspace` changes under any answer, as the record already notes.

### A gap D20 did not raise: stdio across machines

MAN-010 allows a `package` whose transport is `stdio`, and IFC-017 has the engine reach every run
of another provider over Streamable HTTP. So a remote provider that accepts a stdio package must
bridge it, as ToolHive "redirects SSE or Streamable HTTP traffic from the client to the
container's standard input and output" ([ToolHive][toolhive-run]), or refuse it. Its contract
states which, and PRV-012 covers the refusal. The bridge is also a natural place for the gate,
since the server then never listens on a network.

### What v0 must not rule out

1. The SHA-256 of the archive exactly as distributed (PKG-004) stays the only identity of its
   bytes. Nothing hashes an unpacked tree.
2. `fetch` is not the only record of where an archive can be got.
3. Credentials to fetch stay apart from `identity` (PRV-022).
4. DL7 stays open until v1 decides whether the engine serves archives. If it does, the engine
   keeps each archive exactly as distributed, since packing it again would change its digest.

### What v1 adds

- The package as an OCI descriptor, given only to a provider that declares the feature:
  `{mediaType: application/vnd.oci.image.layer.v1.tar+gzip, digest, size, urls}`. A descriptor's
  `urls` "specifies a list of URIs from which this object MAY be downloaded", and "one can verify
  content from an insecure source by recalculating the digest independently"
  ([descriptor][oci-descriptor]). The media type is the standard gzip-compressed tar, which is the
  form of a Conduit archive (PKG-022).
- Such a provider fetches from any listed place, checks the digest before unpacking, keeps a
  cache by digest, and runs the program from the archive's root behind its gate. The places may
  be a registry blob, a mirror, an OCI layout folder, or the engine, which turns DL7 into a
  promise.
- Who fetches: the run's own identity, as KEP-4412 lets "a pod … use its own identity to pull
  images" ([KEP-4412][kep-4412]), or the machine's, limited to packages of instances placed
  there.
- Signatures stay a choice made at install, on the engine. The digest carries that choice to
  every machine, as containerd trusts only the step that turns a name into a digest
  ([research, 2.1][r-21]).
- Untested: a Conduit archive pushed unchanged as one OCI layer should keep its PKG-004 digest,
  since a registry addresses a blob by the SHA-256 of its bytes. The
  [experiment](#11-an-experiment-across-two-machines) tests it.

## 5. The smaller decisions

### Q1. Forms of proof, and the names in `identity`

DL13 leaves the whole mechanism to each engine. But under D5 a gate that a third party writes
must check the engine's proof, so a provider written for one engine might not work with another.

- **a. Two standard profiles and fixed names (recommended).**
  - DL13 offers one or both of two profiles. *Mutual TLS*: X.509 certificates that name their
    holder in exactly one URI, with both client and server use, as an X509-SVID does
    ([X509-SVID][spiffe-x509]). *JWT*: a signed JWT over TLS in the form of a JWT-SVID, whose
    `sub` names the sender and whose `aud` names the receiver ([JWT-SVID][spiffe-jwt]); the
    server's TLS certificate comes from the same root and names the instance. Toward a run the
    audience is the instance's ID. Toward the engine's address, or a `remote` server, the
    audience is the receiver's canonical `https` URI, and the receiver publishes RFC 9728
    metadata, so that the token is MCP's authorization proper ([MCP authorization][mcp-authz];
    [RFC 9728][rfc9728]). MCP's authorization cannot name a run by its ID, since its audience
    must be an `https` URL, which is why the token toward a run follows JWT-SVID instead.
  - IDs are recommended, not required, in SPIFFE form: `spiffe://<trust domain>/conduit/engine`
    for the engine's role, `spiffe://<trust domain>/conduit/instance/<id>` for an instance, and
    in v1 `spiffe://<trust domain>/conduit/node/<id>` for a machine. DL13 states how an ID fits
    the characters that a SPIFFE path allows ([research, 1.4][r-14]). A move to SPIRE,
    cert-manager or Vault then changes nothing.
  - The names of the three things `identity` carries are fixed once, with the other names that
    the specification assigns: the variable that holds the engine's address, the credential
    files, and the trust bundle file. A harness or a gate written once then works under any
    engine. Kubernetes fixes the paths of its token and `ca.crt` the same way
    ([API from a Pod][k8s-pod-api]).
  - Both profiles count as the cryptographic proof of point 3e, which names mutual TLS, SPIFFE
    and MCP's authorization: the JWT profile is SPIFFE's JWT-SVID toward a run, and MCP's
    authorization toward a receiver at an `https` URL. Proof bound to the sender's key is a v1
    tightening.
  - IFC-022's "not from the connection it arrived on" becomes "not from the address, machine,
    network, runtime provider or run it came from", so that mutual TLS, which proves the sender
    once per connection, still counts.
- **b. Each engine's own, as now.** Cost: gates and harnesses may not carry across engines.
- **c. SPIFFE required.** It narrows point 3e, which names mutual TLS, SPIFFE and MCP's
  authorization as alternatives, and makes a non-SPIFFE issuer need an adapter.

### Q2. The engine across a failover

Ruling 40 point 2 leaves the engine's own state to each engine "for now", while asking to go as
far as possible toward leaving architecture to the user as long as it meets requirements. So
this states requirements and leaves the mechanism alone.

- **a. Outcomes only (recommended).** A new item of the implementation-defined list states how
  an engine whose runtime profile says it can fail over, or run as several parts, meets these:
  - Every decision reads the state as it stands: reads are linearizable, as etcd gives by
    default ([etcd][etcd]).
  - At most one part of the engine acts on any one invocation, and the state store enforces it,
    for example by a compare-and-set on the invocation's state. The store is then the resource
    that checks, which is what fencing requires: it "requires the storage server to take an
    active role in checking tokens", and a design that relies on timing does not hold, since
    "You simply cannot make any assumptions about timing" ([Kleppmann][kleppmann]). Several
    engines may then serve at once, as "You can run several instances of kube-apiserver and
    balance traffic between those instances" ([Kubernetes architecture][k8s-arch]); or one may
    lead under a lease, as the controller manager and the scheduler do, "where only one instance
    of the component should be actively running" ([Leases][k8s-leases]). Which is the engine's
    choice; no gate is asked to check a term.
  - The engine's address in `identity` keeps reaching an engine after a move, as Pods reach the
    API server at `kubernetes.default.svc` ([API from a Pod][k8s-pod-api]).
  - DL13's signing keys survive the move. The runtime profile says where they are held, and so
    who is inside the trust domain of every secret: with an outside issuer or a key management
    service, or in the state store, whose backups and operators are then inside it.
  - `local` keeps representing one hostname, as PLC-026 says. The engine moves only to a machine
    that takes its configured hostname (DL5) and holds what the instances on `local` need of
    their machine: the providers' own configuration, such as a Docker context, and any file an
    instance on `local` names (PLC-020). Kubernetes makes the same assumption of a Node name,
    that "an instance using the same name will have the same state (e.g. network settings, root
    disk contents)" ([nodes][k8s-nodes]); here it is the operator's duty, stated in the runtime
    profile, which also names those machines and places nothing else of Conduit's on them, so
    that no other node represents them. No document changes when failover is turned on.
- **b. The engine does not move to another machine in v0.** The runtime profile says the engine
  runs on the machine `local` represents, and restarts there with its state from the store. Cost:
  3f A's "the engine survives the loss of a machine" then holds for the store's machines only,
  and losing the engine's own machine stops Conduit until an operator restores it.

### Q3. Provider servers after the engine restarts

The servers of the runtime, machine and package-source providers run as children of the
engine's own runner (PRV-002), reached over stdio, so they end when the engine does. Read
literally, after a restart no provider runs until an actor sets each up again. Once the engine
has waited as long as DL5 states for a report from the machine provider, every machine but
`local`'s counts as left out (PLC-023), so every instance elsewhere is unavailable.

- **a. As now.** An actor sets each provider up again.
- **b. Run again when first needed (recommended).** When the engine must make a provider call to
  a provider whose run ended with the engine, it first asks its own runner for that run again,
  reading the provider's secrets for the caller of the setup that last ran it, as the Conduits
  stand then. This is the run's counterpart of SEC-023, the owner's answer to D12. It is also a
  second exception to SEC-011, under which the engine asks for a run only within an invocation;
  the owner ruled on SEC-023's exception and has not ruled on this one, so choosing b rules it.
  The kubelet "automatically starts all defined static Pods" when it starts
  ([static Pods][k8s-static-pod]), which is eager; b waits for a call instead, so that the engine
  acts only for a call it must make, its regular report from the machine provider among them
  (DL5), and OBS-004's rule against acting on a difference alone is kept. An engine restart starts
  no model work (IMP-002).
- **c. The runner keeps their values and restarts them**, as systemd restarts a unit with its
  credentials. Cost: secret values in persistent state, which the owner's answer to D12 avoided.

### Q4. The shape of v1, recorded in A65

- **a. Record it now (recommended).**
  - On each machine an existing proxy is the enforcement point: one that speaks Envoy's external
    authorization API, such as Envoy, whose `ext_authz` refuses by default when the decision
    service fails ([ext_authz][envoy-extauthz]), or agentgateway, which "is API-compatible with
    the Envoy External Authorization gRPC service" ([agentgateway][agw-extauthz]). OPA already
    runs as such a decision service beside Envoy ([OPA-Envoy][opa-envoy]).
  - Conduit's decision point decides, from a versioned copy of the Conduits that have an end on
    that machine. A placement is fixed for life (PLC-009), so that slice is well defined. The
    copy is held under a lease, measured on the machine's monotonic clock from receipt, and once
    the lease lapses the decision point refuses new requests. The bound Δ is the lease term plus
    the allowed clock error, since Gray and Cheriton's leases hold only for clocks that do not
    fail ([leases][leases-paper]).
  - The decision point is Conduit's own code on each machine, since deciding access is
    Conduit's own; the copy's format is published, so a decision point other than the engine's
    can serve it. Conduit ships no proxy, tunnel or workload runner of its own.
  - Credentials live no longer than Δ, renewed on each machine under its own identity. The keys
    that v1's decision points accept are new at the upgrade, so a credential issued in v0 under
    an instance-long life is accepted only by the engine, which checks live state; SPIRE
    withdraws an authority from a bundle in the same way ([research, 1.4][r-14]). The engine
    keeps presenting credentials under the v0 root to the gates of runs made in v0, until they
    end. Open streams are closed when a change no longer allows them. Records are kept on the
    machine before dispatch and sent later. A machine that comes back after being reported gone
    registers as a new node.
  - Δ is a value of the runtime profile, never of a user's document.
- **b. Leave A65 open until v1.**

### Q5. Who on the engine's machine can get past a gate

A process that the engine's own runner starts runs under the operating-system user that DL16
leaves to the engine, which is usually the engine's own. Such a process can read the engine's
keys and the providers' credentials, so it can make the engine's proof, or, through a provider's
SSH key, reach a run's machine directly (**inference**). A user who names no runtime gets the
engine's own runner (PRV-003).

- **a. Say it plainly (recommended).** Programs of the engine's own operating-system user are
  inside the engine's trust domain, as `spec/README.md` already has it for every secret, and can
  get past every gate. `spec/README.md` says so. An Agent that should not be trusted that far
  runs under a runtime provider that isolates it, such as a container provider on `local`. MCP's
  best practices ask proxies that spawn stdio servers to "implement … sandboxing or
  containerization for spawned processes" ([best practices][mcp-bp]). With this, a check at the
  provider's own end on the engine's machine is as strong as one on the run's side, which is why
  the gate allows it. Cost: the default setup gives every local Agent the engine's reach.
- **b. Run every process under a user that cannot read the engine's keys**, which DL16 already
  lets an engine choose. Cost (**inference**): on Linux and macOS a process starts another under
  a different user only with privilege, so the engine would run as root or start processes
  through a container runtime, and local development gets heavier.

## 6. How it fits the rulings and the action items

### D17: how a run elsewhere reaches the engine

Ruling 42 accepted a channel in `run.v1` for variables and files that DL13's mechanism fills.
This proposal adds to it and changes none of it:

- `identity` also carries the trust bundle, and, for a run that returns an endpoint, the
  certificate the run's side presents as that endpoint's. The gate may use what `identity` holds
  for its means, and for nothing else.
- The credential names the instance by ID and, in v0, lasts as long as the instance. PRV-022
  already lets a provider replace the file in place, and v1 uses that.
- The provider delivers `identity` and `secrets` only over an authenticated, confidential
  channel, which its contract names.
- The names of what `identity` carries are fixed once, if the owner picks Q1 a.
- In v1 the address names the machine's enforcement point "as the run sees it", which D17's
  wording already allows, and the credential is renewed on the machine under the machine's own
  identity, as the kubelet renews a projected token and the application reloads it
  ([service account configuration][k8s-sa-config]).

### D11: a run left on a machine that comes back

Ruling 42 accepted one more stop at unregistration, and the provider's contract for a stop it
cannot carry out now (PRV-021). Two additions:

- The stops a provider owes survive a failover of the engine and the provider's own retirement
  (PRV-020). Its contract states where it keeps them. A provider whose part on the machine stops
  the run itself, as a disconnected Nomad client stops its allocations after
  `stop_on_client_after` ([disconnect][nomad-disconnect]), needs to keep nothing on the engine's
  side.
- With the gate and credentials named by ID, a run left behind is inert through Conduit, as S7
  shows. What it does outside Conduit with secrets it holds stays a stated limit.

### D12: a remote server's secret headers after a restart

Ruling 42 accepted reading the secret again for the caller of the original setup (SEC-023). This
proposal keeps it, and:

- applies D5's second part to those reads: a secret header goes only to `https`, so a failover
  never sends one to whatever listens on the new machine's loopback;
- proposes its counterpart for provider servers in Q3, which is a second exception to SEC-011.

### Ruling 40, point 3b: placement by the machine provider

Where each instance runs stays Conduit's promise (PRV-019), and the gate gives it weight: only
the engine reaches what runs on a machine. The machine provider still places instances, and only
one rule changes: a machine is `gone` only where the provider knows it no longer exists (PRV-014,
moved into its rule).

### Ruling 40, point 3e, and A64

Point 3e now binds both directions: requests to the engine (IFC-022) and requests from an
enforcement point to a server, a run's or a `remote` one (the gate, IFC-017, IFC-041). The
mechanism stays swappable among mutual TLS, SPIFFE and MCP's authorization, in the two profiles
of Q1.

A64's "done when" grows by two vectors: a request to a gated server without an enforcement
point's proof is refused, with a failed TLS handshake under mutual TLS or HTTP 401 under the JWT
profile, and the server receives nothing; and the engine sends nothing to an endpoint that does
not prove it serves the instance.

### Ruling 40, point 3f, and A65

- **v0 is option A.** The engine decides every request, from the Conduits as they stand
  (ACC-036, ACC-037).
- **v0 leaves B open, once two definitions change.** Ruling 40 asks that nothing outside the
  promises on how a decision is made assume that decisions are made in one place. Today the
  definition of the engine, "A running program", reads as one place, and so does every promise
  that says what "the engine keeps" or records, OPS-008 among them. With the engine defined as
  possibly several parts on several machines, and "mediated" defined by the enforcement point
  ([section 1.2](#12-decision-point-and-enforcement-point-roles-not-machines)), the only promises
  that assume one place of decision are those on how and when a decision is made, listed in
  [section 9](#in-v1-for-a65). Without the two definitions, the ruling's condition is not met.
- **How that was checked.** A search of `spec/` for "at once", "as they stand" and "as it
  stands" finds, beyond the decision promises, only promises on the engine's own state:
  LIF-016, LIF-024, LIF-025, PLC-017, PLC-024, REV-012, REV-014 and TPL-009, and SEC-011 and
  SEC-023, whose reads stay with the engine. That state still changes at once, in the store; only
  its effect on a decision on another machine waits for Δ. No wider survey of `spec/` was made.
- **A robust B** is Q4: an existing proxy enforces, Conduit's decision point decides from a copy
  under a lease and refuses once the lease lapses, and a bound Δ is stated in the runtime
  profile. Nothing that keeps the last configuration without a bound is adopted.
- **A65's list.** It names ACC-011, ACC-034, ACC-037 and CON-008. Add ACC-036, ACC-039, ACC-019,
  IFC-028, PLC-023, IFC-012, IFC-013 and IFC-038, the rest of the promises on how and when a
  decision is made, as section 9 lists them.

## 7. What a user writes

Nothing in a user's document changes between v0 and v1, and nothing names a gate, a credential,
an enforcement point, a decision point or Δ.

```yaml
# A runtime provider: its server runs on the engine's own runner, on `local` (PRV-002).
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: m2-docker
from: docker-context
owner: team
placement: local
arguments:
  context: m2        # a Docker context whose host is ssh://ops@m2.example
---
# An Agent on the node that the engine registered for m2.example (PLC-026). Leaving out
# `placement` lets the machine provider place it under the labels of its type (PRV-015).
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: reviewer-2
from: reviewer
placement: m2
runtime: m2-docker
arguments:
  checkout: {kind: Pool, name: team, entry: checkout}
---
# The Conduit reads as it does for an Agent on the engine's machine.
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: reviewer-2-scratch
from: directory-read
consumer: {kind: Agent, name: reviewer-2}
target: {kind: Resource, name: scratch}
arguments:
  limit: 10
```

- `placement` names a node or is left out. Label selectors stay on types and items (PLC-007),
  never on an instance.
- How the provider gates its runs is the provider type's own configuration, such as its Docker
  context, never a key of Conduit's language.
- In v1 the same documents get local enforcement once the engine and the provider packages are
  upgraded. Δ is a value of the runtime profile. A provider type adds only optional settings, with
  defaults, so a v0 provider Resource stays valid. A Δ per Conduit is not recommended.

## 8. Provider contracts

| Provider | What it must implement in v0 | What v1 adds, as an option its contract states |
| --- | --- | --- |
| Runtime provider | As now: `run.v1`, `stop.v1` and `inspect-run.v1`; running only on the named machine (PRV-019); delivering `secrets` and `identity` (PRV-011, PRV-022). New: the gate, with its means and its bound on a silent engine in the contract; delivery of `secrets` and `identity` only over an authenticated, confidential channel, named in the contract; bridging `stdio` packages behind the gate, or refusing them (PRV-012); stops it owes kept where they survive a failover and its own retirement. Optional: reporting the current `endpoint` in `inspect-run.v1` | Fetching packages by descriptor, as a feature it declares; a gate on the run's machine that its machine's enforcement point uses; giving runs that point's address; renewing credentials on the machine; confining a run so that it reaches targets only through an enforcement point, which makes a restriction enforced (ACC-027) |
| Machine provider | As now, with `gone` only where it knows the machine no longer exists (PRV-014) | It may take heartbeats from decision points, as Kubernetes nodes renew a Lease ([Leases][k8s-leases]) |
| Package-source provider | Unchanged; it serves installs, on the engine's side (PRV-002, PRV-017) | It may supply download places for the descriptor |
| Secret provider (`secret-provider.v1`) | Unchanged in its package. Its run is gated like any run; a store reached by `remote` that takes Conduit's keys checks the engine's proof itself, or is used as `plain` | Nothing |
| The engine's own runner | stdio meets the gate. It delivers `identity` as any provider does (PRV-003). Its processes run as DL16 says, usually as the engine's own user, and are then inside the engine's trust domain (Q5): an Agent that should not be trusted that far runs under a provider that isolates it | Nothing new |
| Identity provider (`identity-provider.v1`, v1 only) | Not in v0: DL13's mechanism serves | Mints X.509 and JWT credentials for IDs the engine names, and serves the trust bundle, shaped on SPIRE's `MintX509SVID`, `MintJWTSVID` and bundle ([research, 1.4](../../research/MULTI_MACHINE.md#14-spiffe-and-spire)). Owned by `system`, at most one; the engine's own serves when none is registered, as under PRV-003. The name may collide with the user's sign-in provider, a point for the vocabulary |
| Decision point on each machine (v1 only) | Not in v0 | Conduit's own: decides under Conduit's decision promises from the published copy, for a proxy that speaks Envoy's external authorization API |

## 9. The promises that change

### In v0

| ID | Change in substance |
| --- | --- |
| New, in `providers.yaml` | The gate, as in [section 3](#the-gate-in-substance), for runs and for `remote` items that take Conduit's keys |
| IFC-017 | The engine sends only once the endpoint has proved it serves the instance, presents proof as an enforcement point, with a JWT's audience the instance's ID, reports unavailable where it cannot verify, and reaches the latest endpoint reported |
| IFC-041, SEC-013 | To a `remote` item that is not `plain`, the engine presents an enforcement point's proof, by mutual TLS or by MCP's authorization with the server's canonical URI as audience, only over `https`. A header that carries a secret goes only to `https` |
| `inspect-run.v1`, PRV-009 | An optional output, `endpoint` |
| PRV-011, PRV-022 | Delivery of `secrets` and `identity` only over an authenticated, confidential channel named in the contract. `identity` also carries the trust bundle and, for a run that returns an endpoint, its serving certificate; the gate may use them and nothing else may |
| IFC-022, PRV-018 | "Not from the connection" becomes "not from the address, machine, network, runtime provider or run it came from". The credential names the instance by ID, and none once it is removed. A JWT-SVID over TLS with a bound audience counts |
| DL13 | Both directions, under one root of trust; the profiles of Q1; IDs; credentials that last as long as their instance; signed for checking without the engine; where its keys are held; each root kept while a run that holds it lasts |
| PRV-012, MAN-010 | PRV-012 also covers a `package` whose transport the provider neither serves nor bridges |
| PRV-014 | "Gone only where the provider knows the machine no longer exists" moves from the observable into the rule |
| PRV-021, PRV-020 | Stops a provider owes survive the engine's failover and the provider's retirement, kept where its contract states |
| OPS-027 | The observable's "A connection over Streamable HTTP that drops while the request is in progress closes its stream, and so cancels the invocation" becomes: a connection that the engine sees drop ends the invocation as cancelled, with its outcome unknown, for the engine; whether and when the implementation learns of it is its transport's and its gate's, and a silent cut may reach it late or never |
| ACC-027, definitions | "Decision point" and "enforcement point" defined, as NIST splits them; "mediated" means passing through an enforcement point, which in v0 is the engine; the engine may be several parts on several machines, and a promise on what it keeps, records or does binds the whole |
| New item of the implementation-defined list | Q2: the outcomes an engine that can fail over meets, the mechanism its own |
| New, in `providers.yaml`; SEC-011 | Q3 b: a provider server whose run ended with the engine is run again when first needed, its secrets read for the caller of its last setup; SEC-011 gains that second exception |
| `spec/README.md` | The D5 gap goes. The trust domain gains the run's machine and its gate for what that run serves, and states that programs of the engine's own operating-system user, the runs on its own runner among them, can get past every gate (Q5) |
| A64, A65 in the audit | A64 covers both directions, with the two vectors of section 6. A65 records Q4 and the list below |

### In v1, for A65

The promises on how and when a decision is made, which ruling 40 lets v1 revisit:

| ID | Change in substance |
| --- | --- |
| ACC-036 | Decided from a versioned copy no older than Δ; the record names the version |
| ACC-037, PLC-023 | A decision point decides while its lease holds and reports unavailable after; "even for a request between two instances on one node" goes, and a request within a machine that is cut off may proceed under the lease |
| ACC-011, ACC-019, ACC-034, ACC-039, IFC-028, CON-008 | "At once" becomes at once at the engine and within Δ at every decision point |
| IFC-012, IFC-013, IFC-038 | The tool list comes from the decision point's copy, and may lag by up to Δ |

Additions in v1 that change no meaning of v0:

| ID | Addition |
| --- | --- |
| ACC-027 | A restriction can be enforced where the runtime provider confines the run; "in this version" goes |
| IFC-022, IFC-032, DL13 | Credentials checked on every machine without the engine, living no longer than Δ |
| PRV-022 | The address of the machine's enforcement point; renewal on the machine |
| PRV-007, `run.v1`, DL7 | The package descriptor, given only to a provider that declares the feature; DL7 becomes a promise only if the engine serves archives |
| PLC-024 | A machine reported gone loses its machine identity |
| PRV-001, PRV-002, PRV-013 | `identity-provider.v1`, owned by `system`, at most one |

Unchanged in v1, once the engine may be several parts: OPS-008 and the other promises on
idempotency keys, whose keys are scoped to the sending instance and so to one machine; REC-001
and SEC-016, which a record kept on the machine before dispatch meets; ACC-017 and IFC-020,
whose chain state the engine's parts share, in the store or as a signed context in the shape of
a transaction token ([transaction tokens][txn-tokens]); SEC-010, SEC-011 and SEC-023; the
promises on revisions; the promises on placement, PLC-009 above all; and the provider calls.

## 10. Left to v1, and what v0 must not preclude

**Left to v1:** the enforcement point and the decision point on each machine, and Δ; a machine's
own identity, with a bootstrap by one-time token and credentials issued only for instances placed
there, as the kubelet bootstraps and the Node authorizer limits it
([research, 1.1](../../research/MULTI_MACHINE.md#11-kubernetes)); renewal of credentials on each
machine; `identity-provider.v1`; short-lived credentials checked without the engine; proof bound
to the sender's key; fetching packages; the engine serving archives; chain contexts between
machines; tables of idempotency keys and records per machine; enforced restrictions; federation
between two engines' trust domains, which SPIFFE's one-way federation allows
([research, 1.4](../../research/MULTI_MACHINE.md#14-spiffe-and-spire)).

**What v0 must not preclude, and what keeps each open:**

1. No promise assumes that only the engine can check a credential: DL13's credentials are
   signed and their keys published.
2. Credentials are per instance, by ID, in files that PRV-022 lets a provider replace in place.
3. The gate admits a role, the enforcement point, which v1 scopes to a machine.
4. "Mediated" is defined by that role, the engine may be several parts, and only the promises on
   how and when a decision is made assume one place of decision.
5. `invocation` and `serving` stay opaque `id.v1` values, so a signed chain context fits in v1.
6. Idempotency keys stay scoped to the sending instance (OPS-008), so each machine can own the
   keys of its instances.
7. No promise says a record is visible at the moment of a decision.
8. The digest of PKG-004 stays the only identity of an archive's bytes, and `fetch` is not the
   only place an archive can come from.
9. Placement stays fixed for life (PLC-009), which makes each machine's slice of the Conduits
   well defined.
10. Reads of secrets stay with the engine, in setups (SEC-010, SEC-011).
11. No document names an enforcement point, a decision point, a gate, a credential or Δ.

## 11. An experiment across two machines

A probe of the mechanisms, not an engine: `experiments/006-across-machines/`, in Rust, with a
`Cargo.lock`, a pinned toolchain and ignored build output. It prepares scenario S3 of the
[experiment program](../../AUDIT.md#experiment-program), which needs an engine.

**Set-up.** Two real machines on one network, M1 and M2, and for H10 a third machine, M3,
behind NAT. On M1, a Rust stand-in plays the engine's two network paths: it issues credentials
from its own root, sends MCP calls to endpoints, and receives calls from runs. On M2, Docker,
ghostunnel, OpenSSH, a local OCI registry and ORAS, and an MCP server written with the
protocol's Rust SDK, `rmcp` ([Rust SDK][rmcp]), run once over stdio behind a bridge and once over
Streamable HTTP on a Unix socket behind ghostunnel. Crates such as tokio, hyper, rustls and rcgen
are chosen when the code is written.

**Hypotheses.** Each is an expected outcome, not a result, and the record states what was
measured.

| # | Hypothesis | How it is measured |
| --- | --- | --- |
| H1 | A call to M2's endpoint without the engine's proof is refused, by a failed TLS handshake under mutual TLS or HTTP 401 under the JWT profile, and the server receives nothing; with the proof it succeeds | Calls with and without a client certificate, and with a token for another instance; the server's log |
| H2 | Another run on M2 cannot reach the server: not a second container, not a process of another provider, not another user | Attempts from each against the Unix socket and against loopback in each namespace |
| H3 | The stand-in refuses to send when the endpoint presents a certificate for another instance's ID | A gate given the wrong certificate; the stand-in's outcome |
| H4 | After an instance is removed, its run's credential is refused, even when a new instance of the same name exists | Remove and re-create; replay the old credential |
| H5 | A credential file replaced in place is picked up by a run that reads it again, with no failed call; v0 does not rely on it, v1's renewal does | Renewal every minute under steady load |
| H6 | After a silent cut between M1 and M2 in mid-call, the server on M2 goes on with the call until its gate's keepalive closes the stream, and for much longer without one; with an SSE response the server then sees a cancellation, with a JSON response it does not | A packet filter drop on M1; the time until the server's log shows the close, with the gate's HTTP/2 keepalive on and off, for a JSON and an SSE response |
| H7 | A port that `ssh -L` forwards on M1 is reachable by a process of another user on M1; a provider end that checks the engine's proof refuses that process; and `docker -H ssh://… run -i` behind that end leaves the server no listener on M2 | A second user on M1; listening sockets on M2 |
| H8 | A Conduit archive pushed unchanged as one OCI layer keeps its PKG-004 digest, and the registry returns identical bytes | `oras push`, then `GET /v2/<name>/blobs/<digest>`; compare SHA-256 |
| H9 | Two stand-ins over one store dispatch each invocation at most once between them, whatever their timing | Invocations claimed by compare-and-set in etcd; one stand-in paused in mid-claim |
| H10 | A gate on M3, behind NAT, is reachable through a tunnel that M3 dials out, with the proof still checked on M3 | An outbound SSH or TLS tunnel from M3 |

A result that contradicts a promise is raised with a counterexample, and fixed in the
specification by an owner's ruling.

## 12. Costs, and what this gives up

- **Every runtime provider must gate.** A simple "run it on that box" provider must add a proxy,
  a bridge, a mesh, or `docker run -i` with a check at its own end. This is the main cost, and
  the reason B of D5 exists.
- **Programs of the engine's own user get past every gate**, the Agents on its own runner among
  them, since they can read its keys and the providers' credentials. The default setup, which
  names no runtime, gives every local Agent that reach; an Agent not trusted that far needs a
  provider that isolates it (Q5).
- **The run's machine is in the trust domain** of what the run serves, as its provider's place
  already is (PRV-011, PRV-022).
- **A `remote` server that takes Conduit's keys must trust the engine's issuer**, or be used as
  `plain`.
- **In v0 the engine is on the path of every request**, and a machine cut off from it can do
  nothing new through Conduit, even between two instances on it, until v1.
- **A silent cut leaves work running unseen** on the far side, until the gate's keepalive closes
  its stream, with its outcome unknown at the engine.
- **v0 credentials last as long as their instances, and may be bearer tokens.** Revocation
  relies on the engine's live check. A model that can run commands can read its own credential,
  as D17 already states, and nothing binds a token to its sender's key until v1.
- **Failover is the user's operations work**: a store with linearizable reads and
  compare-and-set, keys that survive, and the machines that may run the engine kept alike under
  its hostname. v0 requires none of it of an engine that does not fail over.
- **A secret header cannot go to `http://localhost`**; local development uses stdio instead.
- **Programs inside packages stay on the engine's machine in v0**, so a Pool spread over several
  machines cannot use them; authors publish pinned packages instead.
- **v1 trades "at once" for "within Δ"** on a machine that is cut off, and splits tables of
  idempotency keys and records across machines.
- **Not adopted:** network position as proof; shared static secrets; a near end that admits
  without the proof; fencing by clock; SPIFFE as a requirement; a duty on providers that no v0
  engine exercises; a node agent built by Conduit beyond its decision point; package sources on
  each machine; reading secrets on each machine; keeping the last configuration without a bound.

## Sources

Every page was read on 2026-11-06. The [research packet](../../research/MULTI_MACHINE.md#8-sources)
lists every page it uses; these are the ones this proposal links directly.

| Label | URL | Version or date |
| --- | --- | --- |
| k8s-cpn | https://kubernetes.io/docs/concepts/architecture/control-plane-node-communication/ | Kubernetes docs v1.37 |
| k8s-kubelet-authn | https://kubernetes.io/docs/reference/access-authn-authz/kubelet-authn-authz/ | Last modified 2026-02-19 |
| k8s-node-authz | https://kubernetes.io/docs/reference/access-authn-authz/node/ | Docs v1.37 |
| k8s-sa-config | https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/ | Last modified 2024-10-31 |
| k8s-sa-admin | https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/ | Docs v1.37 |
| k8s-pod-api | https://kubernetes.io/docs/tasks/run-application/access-api-from-pod/ | Last modified 2023-02-19 |
| k8s-netpol | https://kubernetes.io/docs/concepts/services-networking/network-policies/ | Docs v1.37 |
| k8s-imgvol | https://kubernetes.io/docs/tasks/configure-pod-container/image-volumes/ | Last modified 2026-02-03 |
| k8s-volumes | https://kubernetes.io/docs/concepts/storage/volumes/ | Docs v1.37 (the version menu's first entry; the page states none) |
| k8s-force-delete | https://kubernetes.io/docs/tasks/run-application/force-delete-stateful-set-pod/ | Last modified 2023-02-19 |
| k8s-static-pod | https://kubernetes.io/docs/tasks/configure-pod-container/static-pod/ | Docs v1.37 |
| k8s-leases | https://kubernetes.io/docs/concepts/architecture/leases/ | Last modified 2026-03-31 |
| k8s-arch | https://kubernetes.io/docs/concepts/architecture/ | Docs v1.37 (the version menu's first entry; the page states none) |
| k8s-nodes | https://kubernetes.io/docs/concepts/architecture/nodes/ | Last modified 2026-10-31 |
| kep-4412 | https://www.kubernetes.dev/resources/keps/4412/ | KEP-4412, stable milestone v1.38 |
| cri-api | https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto | `master` |
| csi-providers | https://secrets-store-csi-driver.sigs.k8s.io/providers | No version shown |
| nomad-artifact | https://developer.hashicorp.com/nomad/docs/job-specification/artifact | Nomad docs v2.1.x |
| nomad-disconnect | https://developer.hashicorp.com/nomad/docs/job-specification/disconnect | Nomad docs v2.1.x |
| consul-ca | https://developer.hashicorp.com/consul/docs/connect/ca/consul | "latest" |
| envoy-extauthz | https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/ext_authz/v3/ext_authz.proto | Envoy 1.41.0-dev-1830cb |
| envoy-jwt | https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/jwt_authn/v3/config.proto | Envoy 1.41.0-dev-1830cb |
| envoy-keepalive | https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/core/v3/protocol.proto | Envoy 1.41.0-dev-1830cb |
| agw-extauthz | https://agentgateway.dev/docs/standalone/latest/configuration/security/external-authz/ | "latest" |
| opa-envoy | https://www.openpolicyagent.org/docs/envoy | No version shown |
| istio-ambient-dp | https://istio.io/latest/docs/ambient/architecture/data-plane/ | Istio 1.31.4 |
| istio-security | https://istio.io/latest/docs/concepts/security/ | Istio 1.31 |
| spiffe-x509 | https://github.com/spiffe/spiffe/blob/main/standards/X509-SVID.md | `main`, Stable |
| spiffe-jwt | https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md | `main`, Stable |
| mcp-authz | https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization | Revision 2026-07-28 |
| mcp-shttp | https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http | Revision 2026-07-28 |
| mcp-bp | https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices | Revision 2026-07-28 |
| mcp-cc | https://github.com/modelcontextprotocol/ext-auth/blob/main/specification/stable/oauth-client-credentials/2026-07-28/oauth-client-credentials.mdx | Stable 2026-07-28 |
| mcp-pkg | https://modelcontextprotocol.io/registry/package-types | Registry in preview |
| rmcp | https://github.com/modelcontextprotocol/rust-sdk | `main`, no release shown |
| rfc9728 | https://www.rfc-editor.org/rfc/rfc9728.html | RFC 9728, April 2025 |
| oauth21 | https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1-13#section-1.5 | draft-ietf-oauth-v2-1-13, 2025-05-28 |
| txn-tokens | https://datatracker.ietf.org/doc/draft-ietf-oauth-transaction-tokens/ | draft-11, 2026-07-30 |
| oci-descriptor | https://github.com/opencontainers/image-spec/blob/v1.1.1/descriptor.md | image-spec v1.1.1 |
| toolhive-run | https://docs.stacklok.com/toolhive/guides-cli/run-mcp-servers | Updated 2026-11-02 |
| docker-mcp-clientpool | https://github.com/docker/mcp-gateway/blob/7e0f34277c91855ad861eeff536b4fdc77420593/pkg/gateway/clientpool.go | Commit 7e0f342, v0.44.3 |
| docker-protect | https://docs.docker.com/engine/security/protect-access/ | No version shown |
| docker-ports | https://docs.docker.com/engine/network/port-publishing/ | No version shown |
| ms-mcp-gw | https://github.com/microsoft/mcp-gateway/blob/cf9f5e9eff89276bc3fc42f0ae4d6109d70e5509/README.md | README of 2026-11-04 |
| aws-agentcore-src | https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-target-http-runtime.html#gateway-target-http-runtime-source-validation | No date shown |
| cf-portals | https://developers.cloudflare.com/cloudflare-one/access-controls/ai-controls/mcp-portals/ | Updated 2026-11-05 |
| cf-tunnel | https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/ | 2026-08-04 |
| ghostunnel | https://github.com/ghostunnel/ghostunnel/blob/master/README.md | `master` |
| openssh | https://man.openbsd.org/ssh | OpenBSD-current, 2026-11-02 |
| openssh-config | https://man.openbsd.org/ssh_config | OpenBSD-current, 2026-10-30 |
| kernel-tcp | https://docs.kernel.org/networking/ip-sysctl.html | Linux kernel 7.4.0-rc1 |
| etcd | https://etcd.io/docs/v3.5/learning/api_guarantees/ | etcd v3.5 |
| kleppmann | https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html | 2016-02-08 |
| leases-paper | https://doi.org/10.1145/74850.74870 | SOSP 1989 |
| nist-pep | https://csrc.nist.gov/glossary/term/policy_enforcement_point | NIST glossary, citing SP 800-162 and SP 1800-15C |
| nist-pdp | https://csrc.nist.gov/glossary/term/policy_decision_point | NIST glossary, citing SP 800-162 |

[k8s-cpn]: https://kubernetes.io/docs/concepts/architecture/control-plane-node-communication/
[k8s-kubelet-authn]: https://kubernetes.io/docs/reference/access-authn-authz/kubelet-authn-authz/
[k8s-node-authz]: https://kubernetes.io/docs/reference/access-authn-authz/node/
[k8s-sa-config]:
  https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
[k8s-sa-admin]: https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/
[k8s-pod-api]: https://kubernetes.io/docs/tasks/run-application/access-api-from-pod/
[k8s-netpol]: https://kubernetes.io/docs/concepts/services-networking/network-policies/
[k8s-imgvol]: https://kubernetes.io/docs/tasks/configure-pod-container/image-volumes/
[k8s-volumes]: https://kubernetes.io/docs/concepts/storage/volumes/
[k8s-force-delete]:
  https://kubernetes.io/docs/tasks/run-application/force-delete-stateful-set-pod/
[k8s-static-pod]: https://kubernetes.io/docs/tasks/configure-pod-container/static-pod/
[k8s-leases]: https://kubernetes.io/docs/concepts/architecture/leases/
[k8s-arch]: https://kubernetes.io/docs/concepts/architecture/
[k8s-nodes]: https://kubernetes.io/docs/concepts/architecture/nodes/
[kep-4412]: https://www.kubernetes.dev/resources/keps/4412/
[cri-api]: https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto
[csi-providers]: https://secrets-store-csi-driver.sigs.k8s.io/providers
[nomad-artifact]: https://developer.hashicorp.com/nomad/docs/job-specification/artifact
[nomad-disconnect]: https://developer.hashicorp.com/nomad/docs/job-specification/disconnect
[consul-ca]: https://developer.hashicorp.com/consul/docs/connect/ca/consul
[envoy-extauthz]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/ext_authz/v3/ext_authz.proto
[envoy-jwt]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/jwt_authn/v3/config.proto
[envoy-keepalive]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/core/v3/protocol.proto
[agw-extauthz]:
  https://agentgateway.dev/docs/standalone/latest/configuration/security/external-authz/
[opa-envoy]: https://www.openpolicyagent.org/docs/envoy
[istio-ambient-dp]: https://istio.io/latest/docs/ambient/architecture/data-plane/
[istio-security]: https://istio.io/latest/docs/concepts/security/
[spiffe-x509]: https://github.com/spiffe/spiffe/blob/main/standards/X509-SVID.md
[spiffe-jwt]: https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md
[mcp-authz]: https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization
[mcp-shttp]:
  https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http
[mcp-bp]:
  https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices
[mcp-cc]:
  https://github.com/modelcontextprotocol/ext-auth/blob/main/specification/stable/oauth-client-credentials/2026-07-28/oauth-client-credentials.mdx
[mcp-pkg]: https://modelcontextprotocol.io/registry/package-types
[rmcp]: https://github.com/modelcontextprotocol/rust-sdk
[rfc9728]: https://www.rfc-editor.org/rfc/rfc9728.html
[oauth21]: https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1-13#section-1.5
[txn-tokens]: https://datatracker.ietf.org/doc/draft-ietf-oauth-transaction-tokens/
[oci-descriptor]: https://github.com/opencontainers/image-spec/blob/v1.1.1/descriptor.md
[toolhive-run]: https://docs.stacklok.com/toolhive/guides-cli/run-mcp-servers
[docker-mcp-clientpool]:
  https://github.com/docker/mcp-gateway/blob/7e0f34277c91855ad861eeff536b4fdc77420593/pkg/gateway/clientpool.go
[docker-protect]: https://docs.docker.com/engine/security/protect-access/
[docker-ports]: https://docs.docker.com/engine/network/port-publishing/
[ms-mcp-gw]:
  https://github.com/microsoft/mcp-gateway/blob/cf9f5e9eff89276bc3fc42f0ae4d6109d70e5509/README.md
[aws-agentcore-src]:
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-target-http-runtime.html#gateway-target-http-runtime-source-validation
[cf-portals]:
  https://developers.cloudflare.com/cloudflare-one/access-controls/ai-controls/mcp-portals/
[cf-tunnel]:
  https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/
[ghostunnel]: https://github.com/ghostunnel/ghostunnel/blob/master/README.md
[openssh]: https://man.openbsd.org/ssh
[openssh-config]: https://man.openbsd.org/ssh_config
[kernel-tcp]: https://docs.kernel.org/networking/ip-sysctl.html
[etcd]: https://etcd.io/docs/v3.5/learning/api_guarantees/
[kleppmann]: https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html
[leases-paper]: https://doi.org/10.1145/74850.74870
[nist-pep]: https://csrc.nist.gov/glossary/term/policy_enforcement_point
[nist-pdp]: https://csrc.nist.gov/glossary/term/policy_decision_point
[od-d5]:
  ../ruling-41/open-decisions.md#d5-requests-from-the-engine-to-a-server-on-another-machine
[od-d20]:
  ../ruling-41/open-decisions.md#d20-programs-inside-a-package-under-another-runtime-provider
[r-44]:
  ../../research/MULTI_MACHINE.md#44-going-around-a-gateway-as-each-documents-it
[r-21]:
  ../../research/MULTI_MACHINE.md#21-cri-and-containerd-the-runtime-pulls-by-reference-or-digest
[r-14]: ../../research/MULTI_MACHINE.md#14-spiffe-and-spire
