# Work across machines: identity, artifacts, enforcement, MCP and failure

> Research; it does not bind. Written on 2026-11-06 because the owner, ruling on questions 1 and
> 8 of [ruling 42](../proposals/ruling-42/README.md), asked for more research on work across
> machines before deciding D5 and D20.

The owner's words were: "I'd like some more research and clarification on how to solve the multi
machine issue and be forward looking even if some functionality doesn't exist in v0". This packet
is that research. It recommends nothing. What it means for Conduit is argued in the
[proposal on work across machines](../proposals/multi-machine/README.md), which waits for the
owner. The two questions are framed in the
[open questions after the rewrite](../proposals/ruling-41/open-decisions.md#for-the-owner): D5
asks who may reach the server of a run on another machine, and whether the engine must prove
itself to it; D20 asks whether a program shipped inside a package may run on another machine.

**Method.** Five researchers each took one topic and read the primary pages of each system on
2026-11-06. The topics are the five sections below: identity and trust between machines; how a
program reaches the machine that runs it; node agents and local enforcement; the Model Context
Protocol (MCP) across machines and its gateways; and what existing systems do when a machine or
the network fails. Three designers drew designs from the five reports, and two reviewers judged
the designs, opening some pages again. The editor opened eleven pages again on 2026-11-06 to
check claims that the proposal leans on. A skeptic then checked the proposal and this packet
against their sources, and the editor opened the pages behind each objection on the same day and
added what they state; each such use is marked "re-opened". Many quotations were
pulled out of a page by a page-reading tool, so a quotation should be checked against its page
before it is put into the text of the specification. Nothing was tested: no cluster, machine,
registry or credential was used.

**How to read it.** A sentence with a citation says what that page states, in the page's own
terms. A sentence marked **Inference** is a researcher's or the editor's own reasoning, and the
[patterns across the systems](#6-patterns-across-the-systems) are inference throughout. Claims
that no page confirmed are listed under [not established](#7-not-established). The version or
date of every page is in the [sources](#8-sources).

Two directions recur, and the packet names them so:

- **Upward:** a workload on a node sends requests to the control plane, as a Pod calls the
  Kubernetes API server.
- **Downward:** the control plane sends requests to a node or to a workload on it, as the API
  server calls a kubelet or a Pod.

## Contents

- [1. Identity and trust across machines](#1-identity-and-trust-across-machines)
- [2. How a program reaches its machine](#2-how-a-program-reaches-its-machine)
- [3. Node agents and local enforcement](#3-node-agents-and-local-enforcement)
- [4. MCP across machines, and its gateways](#4-mcp-across-machines-and-its-gateways)
- [5. Machines lost, partitions and failover](#5-machines-lost-partitions-and-failover)
- [6. Patterns across the systems](#6-patterns-across-the-systems)
- [7. Not established](#7-not-established)
- [8. Sources](#8-sources)

## 1. Identity and trust across machines

### 1.1 Kubernetes

**Upward.**

- "Kubernetes has a "hub-and-spoke" API pattern. All API usage from nodes (or the pods they run)
  terminates at the API server." Nodes "should be provisioned with the public root certificate
  for the cluster such that they can connect securely to the API server along with valid client
  credentials", preferably a client certificate. This path can run over untrusted and public
  networks ([control plane to node][k8s-cpn], re-opened).
- A Pod finds the API server through `KUBERNETES_SERVICE_HOST` and
  `KUBERNETES_SERVICE_PORT_HTTPS`, or the Service name `kubernetes.default.svc`, not a fixed
  host. Its token and `ca.crt` are mounted at `/var/run/secrets/kubernetes.io/serviceaccount/`
  ([API from a Pod][k8s-pod-api]).
- A projected token's audience defaults to the API server, and a recipient not named in the
  audience "should reject the token". `expirationSeconds` defaults to one hour and must be at
  least ten minutes ([projected volumes][k8s-projected]).
- "The kubelet proactively requests rotation for the token if it is older than 80% of its total
  time-to-live (TTL), or if the token is older than 24 hours." "The application is responsible
  for reloading the token when it rotates" ([service account configuration][k8s-sa-config],
  re-opened). Outside parties verify such tokens through OIDC discovery and a JWKS served over
  `https` (same page).
- "When a token is bound to an object, the object's `metadata.name` and `metadata.uid` are stored
  as extra 'private claims' in the issued JWT." Authentication fails 60 seconds or more after the
  object's `deletionTimestamp`, and "If the referenced object no longer exists (or its
  `metadata.uid` does not match), the request will not be authenticated". The check is made at
  the API server: "When a bound token is presented to the kube-apiserver, the service account
  authenticator will extract and verify these claims"
  ([service account administration][k8s-sa-admin], re-opened).
- The `podCertificate` projected volume is stable in v1.37: the kubelet gives a Pod a private key
  and a certificate chain from a named signer, and refreshes them near expiry; the application
  reloads the file ([projected volumes][k8s-projected]). Kubernetes ships no signer for it:
  "built-in signers are deferred". The `noderestriction` plugin admits a PodCertificateRequest only
  from `system:node:<node>` for a Pod scheduled to that node ([KEP-4317][kep-4317]).
- "Kubernetes 1.37 does not support certificate revocation. Any certificate that is issued remains
  valid until it expires" ([authentication][k8s-authn]).

**The kubelet's own identity.**

- The kubelet authenticates first with a bootstrap token, as a member of `system:bootstrappers`,
  and then asks for a client certificate by a CSR with the signer
  `kubernetes.io/kube-apiserver-client-kubelet`, which the controller manager or a person
  approves. It can rotate the certificate as it expires. "All certificates issued have one year
  validity" ([TLS bootstrapping][k8s-tls-bootstrap]).
- Serving certificates need `serverTLSBootstrap`, and the built-in approvers do not approve them
  "for security reasons". A custom approver should check that the requester is
  `system:node:<nodeName>`, that the usages are for serving, and that the IP and DNS names belong
  to the requesting node, with no URI names (same page).
- The Node authorizer lets a kubelet read "secrets, configmaps, persistent volume claims and
  persistent volumes related to pods bound to the kubelet's node". The `NodeRestriction`
  admission plugin limits what it may write. A kubelet may request service account tokens only
  for audiences that Pods on its node reference: "This prevents a compromised node from obtaining
  tokens for arbitrary audiences" ([Node authorization][k8s-node-authz]).

**Downward.**

- The API server calls the kubelet for logs, attach and port forwarding. "By default, the API
  server does not verify the kubelet's serving certificate, which makes the connection subject to
  man-in-the-middle attacks and unsafe to run over untrusted and/or public networks." The flag
  `--kubelet-certificate-authority` makes it verify ([control plane to node][k8s-cpn],
  re-opened).
- On the kubelet's side, `--client-ca-file` enables client-certificate authentication, and the API
  server presents `--kubelet-client-certificate`. With `--authorization-mode=Webhook` "the kubelet
  calls the `SubjectAccessReview` API … to determine whether each request is authorized". By
  default the kubelet accepts anonymous requests, and "The default authorization mode is
  `AlwaysAllow`" ([kubelet authentication][k8s-kubelet-authn]).
- "The connections from the API server to a node, pod, or service default to plain HTTP
  connections and are therefore neither authenticated nor encrypted"
  ([control plane to node][k8s-cpn], re-opened).
- "The Konnectivity agents initiate connections to the Konnectivity server and maintain the
  network connections", and all traffic from the control plane to nodes can go through them
  ([control plane to node][k8s-cpn], re-opened). The documented agent authenticates with a
  projected token whose audience is `system:konnectivity-server`
  ([Konnectivity setup][k8s-konnectivity]).
- "By default, a pod is non-isolated for ingress; all inbound connections are allowed." Network
  policies are carried out by the network plugin ([network policies][k8s-netpol]).
- The Secrets Store CSI driver "uses gRPC to communicate with the provider" over a Unix domain
  socket, and the provider runs as a daemonset on the same hosts as the driver
  ([CSI providers][csi-providers]).

### 1.2 Nomad

- Nomad "optionally uses mutual TLS (mTLS) for all HTTP and RPC communication". Certificates are
  named by role and region, such as `server.us-west.nomad`. `verify_server_hostname` checks the
  role and region, and `verify_https_client` requires API clients to present certificates from
  the same CA. Certificates reload on SIGHUP. The page does not discuss revocation
  ([TLS][nomad-tls]).
- Node identity, from 1.11, "does not replace mTLS"; it adds a layer "to prevent an unauthorized
  client from joining" ([1.11 release notes][nomad-1-11]). An introduction token is a JWT scoped
  to a node name, a node pool and a TTL ([node intro create][nomad-intro]). The node identity is
  a JWT "signed by the leader's keyring", made at registration and at each heartbeat, valid for
  24 hours by default (`node_identity_ttl`), and renewed once it passes 67% of its life
  ([node identity][nomad-node-identity]).
- A workload identity is a JWT signed by the same keyring, with claims `nomad_namespace`,
  `nomad_job_id`, `nomad_allocation_id` and `nomad_task` ([workload identity][nomad-wi]). It
  reaches a task as `NOMAD_TOKEN` or as the file `secrets/nomad_token`. With `ttl` set, "the
  client will renew the identity at roughly half the TTL", and `change_mode` restarts or signals
  the task. "You may not set a TTL on the default identity" ([identity block][nomad-identity]).
  Outside parties verify identities through `/.well-known/jwks.json` and OIDC discovery once
  `oidc_issuer` is set ([keyring][nomad-keyring]).
- The Task API is a Unix socket at `${NOMAD_SECRETS_DIR}/api.sock`. It "always requires
  authentication", and "mTLS is never enabled for the Task API since traffic never leaves the
  node" ([Task API][nomad-task-api]).
- Downward, the server keeps "the multiplexed yamux Session for dialing back" on the connection
  that the client opened, and calls the client over it ([client_rpc.go][nomad-client-rpc]).

### 1.3 Consul

- "The proxy or natively-integrated application enforces intentions on inbound connections or
  requests." L4 intentions use "identities encoded in mTLS certificates". At L4 "modifying an
  intention does not have an effect on existing connections"; at L7 a change to deny blocks new
  requests. Agents and proxies cache intentions and update them by blocking queries
  ([intentions][consul-intention]).
- `leaf_cert_ttl` defaults to 72 hours. It "is also the effective limit on how long a server
  outage can last (with no leader) … before network connections will start being rejected"
  ([Consul CA][consul-ca]).
- Consul uses its built-in CA when no other provider is set, and can move to Vault or another CA
  by a root rotation with cross-signing ([certificates][consul-cert]). Its trust domain is
  `<cluster-id>.consul`, written as a SPIFFE URI ([Consul CA][consul-ca]).

### 1.4 SPIFFE and SPIRE

**The standards.**

- A SPIFFE ID is a URI `spiffe://<trust domain>/<path>`. Path segments may hold only letters,
  digits, dots, dashes and underscores ([SPIFFE ID][spiffe-id]).
- An X509-SVID "MUST contain exactly one URI SAN", and may contain other names, DNS names among
  them. Where a leaf carries extended key usage, both `serverAuth` and `clientAuth` are set, so
  one SVID serves as client or server. The standard defines no revocation
  ([X509-SVID][spiffe-x509]).
- A JWT-SVID must carry `aud` and `exp`. "The `sub` claim MUST be set to the SPIFFE ID of the
  workload to which it is issued." "The `aud` claim MUST be present, containing one or more
  values", "Validators MUST reject tokens without an `aud` claim set", and "It is strongly
  recommended that the number of values be limited to one in normal cases"; values are to be
  "scoped to the service which it is intended to be presented to", and the page requires no URL
  form of them. "Being a bearer token, JWT-SVIDs are susceptible to replay attacks", and
  "Use of an aggressive value for the `exp` claim is recommended" ([JWT-SVID][spiffe-jwt],
  re-opened).
- A trust bundle is a JWK Set with a `spiffe_sequence`, and keys are withdrawn by removing them
  from it ([trust domain and bundle][spiffe-bundle]).
- The Workload API identifies its caller "out-of-band", without direct client authentication, and
  streams updates "like revocations and CA certificate introductions", each message carrying the
  full set ([Workload API][spiffe-wapi]). The Workload Endpoint "SHOULD be exposed through a
  local endpoint", a Unix socket or TCP, and "MUST NOT require any direct authentication of its
  clients" ([Workload Endpoint][spiffe-wep]).
- Federation serves bundles at HTTPS endpoints under the `https_web` or `https_spiffe` profile.
  Clients poll at `spiffe_refresh_hint`, five minutes by default. Trust is one-way, and bundles
  are never merged ([federation][spiffe-fed]).
- The WIT-SVID, still "Incubating", is "bound to a workload's key-pair via the `cnf` claim"
  ([WIT-SVID][spiffe-wit]). The project's roadmap plans an experimental Workload API profile for
  transaction tokens ([roadmap][spiffe-roadmap]).

**SPIRE.**

- A node proves itself by a cloud identity document, a TPM, a Kubernetes token, an existing
  certificate, or a join token; "join tokens expire immediately after use". A workload is
  attested by the kernel: the agent learns the caller's process ID over the Workload API's Unix
  socket and matches selectors against registration entries. The agent fetches the entries for
  its node and caches their SVIDs ([SPIRE concepts][spire-concepts]).
- Default lifetimes are one hour for an X509-SVID, five minutes for a JWT-SVID and 24 hours for
  the CA. A single SVID is not revoked: `localauthority x509 revoke` removes an authority from the
  bundle, and `agent evict` and `agent ban` take a node's attestation away. SPIRE can use Vault,
  AWS Private CA, Google CAS or cert-manager as an upstream authority
  ([SPIRE server][spire-server]). The agent syncs every five seconds and rotates SVIDs at half
  their life ([SPIRE agent][spire-agent]).
- `MintX509SVID` takes a CSR and a TTL, and `MintJWTSVID` an ID, audiences and a TTL; the caller
  "must be local or present an admin X509-SVID" ([svid.proto][spire-mint]). The Delegated
  Identity API lets authorized workloads obtain SVIDs on behalf of other workloads
  ([spire-api crate][spire-deleg]). SPIRE's OIDC discovery provider serves
  `/.well-known/openid-configuration` and `/keys` ([OIDC discovery provider][spire-oidc]).

**Tools that write and use SVIDs.**

- spiffe-helper writes the SVID, its key and the bundle to files and overwrites them on renewal
  ([spiffe-helper][spiffe-helper]). cert-manager's csi-driver-spiffe writes `tls.crt`, `tls.key`
  and `ca.crt` from "the configured cert-manager issuer" ([csi-driver-spiffe][csi-spiffe]).
- The Rust crate spiffe-rustls 0.6.1 builds rustls client and server configurations from a live
  X.509 source, so new handshakes use rotated SVIDs without a restart
  ([spiffe-rustls][spiffe-rustls]).

### 1.5 Istio ambient and Linkerd

- "There is a single instance of the ztunnel proxy on each node." It requests certificates for
  the identities on its node, carries traffic by mutual TLS over HBONE, and applies authorization
  policy at L4; waypoints apply L7 policy. "Ztunnel's own identity is never used for mTLS
  connections between workloads" ([ambient data plane][istio-ambient-dp]). HBONE is HTTP CONNECT
  secured by mutual TLS, on port 15008 ([HBONE][istio-hbone]).
- "The ztunnel uses a distinct credential for every pod which is only issued if the pod is
  currently running on the node", and "does not use cluster-wide or per-node credentials". If a
  ztunnel is compromised, "only credentials for pods currently scheduled on that node could be
  stolen" ([ambient security][istio-ambient-sec]).
- The Istio agent sends istiod a CSR with its credentials, and istiod signs. Under STRICT,
  "Workloads only accept mutual TLS traffic"; under PERMISSIVE they accept both, and a policy
  with no mode is PERMISSIVE. Authorization is enforced "in the server side Envoy proxy", and
  "Istio sends configurations to the targeted endpoints asynchronously"
  ([Istio security][istio-security]). `SECRET_TTL` is 24 hours and `SECRET_GRACE_PERIOD_RATIO`
  0.5 ([pilot-agent][istio-pilot-agent]).
- Linkerd's proxy puts "the actual service account token" in its CSR. Certificates "expire after
  24 hours and are automatically rotated", and the trust anchor lasts 365 days by default
  ([Linkerd mTLS][linkerd-mtls]). The default inbound policy is `all-unauthenticated`, while a
  `Server`'s `accessPolicy` defaults to `deny`. Clients are named by mTLS identity or by network
  range ([Linkerd authorization][linkerd-authz]).

### 1.6 WireGuard and Tailscale

- In WireGuard each peer is a public key, and cryptokey routing makes a packet's source address
  authentic. "All issues of key distribution and pushed configurations are out of scope of
  WireGuard" ([WireGuard][wireguard]).
- Tailscale's coordination server is "a shared drop box for public keys" and holds the policy.
  "Each node is responsible for blocking incoming connections that should not be allowed, at
  decryption time." A node's private key "never, ever leaves its node"
  ([how Tailscale works][ts-how]). Node keys expire after 180 days by default
  ([key expiry][ts-keys]).
- When the coordination server is down, "all your existing connections keep working". "The only
  things that don't work when the bus is down are adding/removing/changing nodes and packet
  filters" ([hypergrowth][ts-hyper]).
- Tailscale Serve adds identity headers such as `Tailscale-User-Login`, strips spoofed copies,
  and advises that the backend listen only on localhost ([Serve][ts-serve]).
- **Inference.** This is identity of a machine or a user, never of one workload among several on
  a machine.

### 1.7 MCP authorization and the standards it uses

- "Authorization is **OPTIONAL**." HTTP transports should follow it; a stdio implementation
  "SHOULD NOT follow this specification, and instead retrieve credentials from the environment".
  A protected server is an OAuth 2.1 resource server. Clients send an RFC 8707 `resource`, and
  servers "MUST validate that access tokens were issued specifically for them as the intended
  audience" and "MUST NOT accept or transit any other tokens". Authorization "MUST be included in
  every HTTP request from client to server" ([MCP authorization][mcp-authz]).
- The `resource` parameter "MUST use the canonical URI of the MCP server", whose valid examples
  are all `https` URLs and whose invalid ones include a URI with a fragment; "MCP servers
  **MUST** implement OAuth 2.0 Protected Resource Metadata (RFC9728)"; and "Invalid or expired
  tokens **MUST** receive a HTTP 401 response" ([MCP authorization][mcp-authz], re-opened).
  RFC 9728 defines the resource identifier as "a URL that uses the `https` scheme and has no
  fragment component" ([RFC 9728][rfc9728]).
- OAuth 2.1, section 1.5: "All the OAuth protocol URLs (URLs exposed by the AS, RS and Client)
  MUST use the `https` scheme except for loopback interface redirect URIs, which MAY use the
  `http` scheme" ([OAuth 2.1][oauth21], re-opened).
- Authorization servers "SHOULD issue short-lived access tokens", and the token used upstream is
  a separate token: "The MCP server **MUST NOT** pass through the token it received from the MCP
  client" ([authorization security considerations][mcp-authz-sec]).
- The client-credentials extension is stable at revision 2026-07-28. It covers
  "machine-to-machine authentication without user interaction" with pre-registered clients, and
  recommends JWT client authentication (RFC 7523) ([client credentials][mcp-cc]). The
  enterprise-managed authorization extension uses the Identity Assertion JWT Authorization Grant
  ([ext-auth][mcp-ext-auth]).
- Three proposals are open: SEP-1933, workload identity federation, which states that workloads
  in Kubernetes or SPIFFE "already receive short-lived, cryptographically verifiable JWTs"
  ([SEP-1933][sep-1933]); SEP-1932, DPoP ([SEP-1932][sep-1932]); and SEP-2752, HTTP message
  signatures, a draft seeking a sponsor ([SEP-2752][sep-2752]).
- RFC 8705 binds an access token to a client certificate, and the resource server "MUST verify
  that the certificate matches" ([RFC 8705][rfc8705]). RFC 7662 introspection returns `active`
  false once a token is revoked, and a resource server that caches the answer opens "a window
  during which a revoked token could be used" ([RFC 7662][rfc7662]).
- Vault's JWT authentication checks signatures through a JWKS or OIDC discovery, and
  `bound_audiences` "is required when an `aud` claim is set" ([Vault JWT][vault-jwt]).
- The IETF WIMSE draft `draft-ietf-wimse-workload-creds-03`, of 30 October 2026, is a working
  group draft, not an RFC. It defines a JWT workload identity token and an X.509 workload
  identity certificate, with three bindings: mutual TLS, a proof token per request, and HTTP
  message signatures ([WIMSE credentials][wimse]).

### 1.8 How long a revocation takes to bite

| System | Credential and default life | Revocation before expiry | When it bites |
| --- | --- | --- | --- |
| Kubernetes kubelet client certificate | X.509, one year ([TLS bootstrapping][k8s-tls-bootstrap]) | None ([authentication][k8s-authn]) | At expiry |
| Kubernetes bound token | JWT, one hour default, ten minutes minimum, refreshed at 80% ([projected volumes][k8s-projected]; [configuration][k8s-sa-config]) | Checked against live objects at the API server ([administration][k8s-sa-admin]) | At the API server, within 60 s of deletion. **Inference:** at an outside verifier, at `exp` |
| Nomad node identity | JWT, 24 hours, renewed at 67% ([node identity][nomad-node-identity]) | Not documented in the pages read | Not documented |
| Consul | Leaf certificate 72 hours ([Consul CA][consul-ca]); intentions pushed | Intention change ([intentions][consul-intention]) | L7: new requests. L4: new connections only. Cut off: up to the leaf TTL |
| SPIRE | X509-SVID one hour, JWT-SVID five minutes ([SPIRE server][spire-server]) | Authority revoked, node evicted or banned | **Inference:** issued SVIDs at expiry |
| Istio, Linkerd | 24-hour certificates ([pilot-agent][istio-pilot-agent]; [Linkerd mTLS][linkerd-mtls]) | Policy push, asynchronous ([Istio security][istio-security]) | No bound stated |
| Tailscale | Node key 180 days ([key expiry][ts-keys]) | Packet filter and node changes ([how Tailscale works][ts-how]) | No bound stated; no change while control is down ([hypergrowth][ts-hyper]) |
| OAuth and MCP | Access token, short-lived recommended ([security considerations][mcp-authz-sec]) | Introspection ([RFC 7662][rfc7662]) | **Inference:** per request if introspected, else at `exp` |

## 2. How a program reaches its machine

### 2.1 CRI and containerd: the runtime pulls by reference or digest

- CRI's `ImageService` belongs to the runtime and has `PullImage`, `ImageStatus`, `ListImages`
  and `RemoveImage`. `PullImage` "pulls an image with authentication config"; its request
  carries `ImageSpec image`, `AuthConfig auth` and `sandbox_config`. `ImageSpec.image` may be an
  "imageID or imageDigest", and `PullImageResponse.image_ref` is "the unique identifier of the
  image on the node". `ContainerConfig` carries `envs` and `mounts` ([api.proto][cri-api]).
- The runtime's `StatusResponse` lists its `runtime_handlers`, each with
  `RuntimeHandlerFeatures`, "a set of features implemented by the runtime handler", such as
  `recursive_read_only_mounts` and `user_namespaces` ([api.proto][cri-api], re-opened). A
  Kubernetes mount whose `recursiveReadOnly` is `Enabled` needs, among other things, a CRI-level
  runtime that supports it, and "It will fail if any of these is not true"
  ([volumes][k8s-volumes]).
- "The kubelet acts as a client" of the container runtime ([CRI][k8s-cri]).
- The kubelet credential provider plugin is "the most robust and versatile way to fetch
  credentials for private registries". With `IfNotPresent` "the image is pulled only if it is not
  already present locally". A digest "pins the code that you run". With
  `KubeletEnsureSecretPulledImages`, beta and on by default since v1.35, Kubernetes validates
  credentials "for every image that requires credentials to be pulled, even if that image is
  already present on the node". Pre-pulled images are "an alternative to authenticating to a
  private registry", but every node needs the same ones, and they do not work reliably where a
  cloud replaces nodes automatically ([images][k8s-images]).
- KEP-4412 lets "a pod … use its own identity to pull images", with the goal to "Avoid needing a
  kubelet/node based identity to pull images". It was alpha in v1.33, beta in v1.34, and is
  planned stable in v1.38 ([KEP-4412][kep-4412]).
- In containerd "Each component is stored in a file whose name is the hash of it"
  ([content flow][ctrd-content]). The writer returns `ErrAlreadyExists` when the blob path exists
  ([store.go][ctrd-store]), and the fetch handler treats that as success without fetching
  ([handlers.go][ctrd-handlers]).
- containerd's hosts configuration: "Resolving (the process of converting a name into a digest)
  must be considered a trusted operation", and "A public mirror should never be trusted to do a
  resolve action". A mirror gets `capabilities = ["pull"]` ([hosts.md][ctrd-hosts]).
- Docker Swarm resolves a tag "to the specific digest the tag points to at the time of service
  creation", so "all service tasks use the same version of the image"
  ([Swarm services][swarm-services]). `--with-registry-auth` sends the client's login token to the
  nodes, which pull for themselves ([service create][docker-service-create]).

### 2.2 OCI artifacts, ORAS, Helm and Flux

- "Content other than OCI container images MAY be packaged using the image manifest."
  `artifactType` names an artifact's type, and `subject` points to another manifest, as
  signatures and SBOMs use ([manifest][oci-manifest]).
- A descriptor's `digest` is "the digest of the targeted content", and "Retrieved content SHOULD
  be verified against this digest when consumed via untrusted sources". Its `urls` "specifies a
  list of URIs from which this object MAY be downloaded". "If the digest can be communicated in a
  secure manner, one can verify content from an insecure source by recalculating the digest
  independently" ([descriptor][oci-descriptor]).
- `application/vnd.oci.image.layer.v1.tar+gzip` is a layer "as a tar archive compressed with
  gzip" ([media types][oci-media-types]). The image layout is "the directory structure for OCI
  content-addressable blobs", and it may travel inside a tar or zip archive
  ([image layout][oci-layout]).
- In the distribution API a manifest reference "MUST be either (a) the digest of the manifest or
  (b) a tag". A blob is fetched by `GET /v2/<name>/blobs/<digest>`, and "Clients SHOULD verify
  that the response body matches the requested digest". The Referrers API is
  `GET /v2/<name>/referrers/<digest>` ([distribution spec][oci-dist]).
- ORAS pushes and pulls arbitrary files to OCI registries ([ORAS][oras]). It sets an artifact
  type and a media type per file, and tars a directory
  ([pushing and pulling][oras-push]). `oras cp` copies through an OCI layout folder, with
  referrers ([oras cp][oras-cp]).
- Helm recommends registries with OCI support for charts. It pushes the chart `.tgz` as
  `application/vnd.cncf.helm.chart.content.v1.tar+gzip` and installs by `oci://…@sha256:…`,
  because "digests are immutable" ([Helm registries][helm-oci]). Flux's `OCIRepository` pins
  `.spec.ref.digest`, verifies with cosign or notation, and selects a layer by media type
  ([Flux OCIRepository][flux-oci]).

### 2.3 Kubernetes image volumes

- Image volumes "mount content from OCI registries inside containers", stable since v1.36; the
  container runtime must support them ([image volumes][k8s-imgvol]).
- The design's use cases include "package and distribute binary artifacts using OCI image and
  distribution specification standards". Mounts are read-only. The kubelet passes
  `PullImageResponse.image_ref` to the mount, through the same `PullImage`
  ([KEP-4639][kep-4639]).

### 2.4 Nomad artifacts

- The `artifact` block "instructs Nomad to fetch and unpack a remote resource, such as a file,
  tarball, or binary", before the task starts, by go-getter. Its `options` hold a checksum, and
  "If the checksum is invalid, an error will be returned". Sources include http, https, git and
  S3 ([artifact][nomad-artifact]).
- The client limits download size and unpacked size and file count, and sandboxes downloads with
  Landlock on Linux ([client configuration][nomad-client]). The exec driver "uses the underlying
  isolation primitives of the operating system" ([exec driver][nomad-exec]).
- go-getter checks `md5`, `sha1`, `sha256` or `sha512`, and "If the destination file exists and
  the checksums match: download will be skipped" ([go-getter][go-getter]).

### 2.5 The MCP registry and MCP bundles

- In server.json, `registryType` says "how to download packages", from an open list such as npm,
  pypi, oci, nuget and mcpb. `identifier` is "either a package name (for registries) or URL (for
  direct downloads)". `version` rejects ranges. `fileSha256` is "Required for MCPB packages and
  optional for other package types" ([server.schema.json][mcp-server-schema]).
- An OCI identifier's tag "can also be specified as a digest". For MCPB, "The MCP Registry does
  not validate this hash; however, MCP clients do validate the hash before installation"
  ([package types][mcp-pkg]). The official registry "does not support private servers" and
  recommends hosting a private registry ([registry][mcp-registry-about]).
- "MCP Bundles (`.mcpb`) are zip archives containing a local MCP server and a `manifest.json`"
  ([MCPB][mcpb-readme]). The manifest's `server` has `entry_point` and `mcp_config`, and
  `${__dirname}` "is replaced with the absolute path to the extension's directory"
  ([manifest][mcpb-manifest]). `mcpb sign` uses PKCS#7 with an X.509 certificate
  ([MCPB CLI][mcpb-cli]).

### 2.6 Running MCP servers on other infrastructure

- stdio carries "newline-delimited messages over the standard streams of a client-launched
  subprocess" ([transports][mcp-transports]).
- ToolHive builds images from `npx://`, `uvx://` and `go://` with `thv build`
  ([build containers][toolhive-build]). For stdio servers it "redirects SSE or Streamable HTTP
  traffic from the client to the container's standard input and output"
  ([run MCP servers][toolhive-run]).

### 2.7 Signing

- Cosign signs a file with `cosign sign-blob` and a bundle ([signing blobs][cosign-blob]), and
  the bundle "supports offline verification" ([verify][cosign-verify]). Signatures can be stored
  as OCI 1.1 referrers ([Red Hat][redhat-referrers]).
- npm provenance is "signed by Sigstore public good servers" and needs a cloud CI runner
  ([npm provenance][npm-provenance]).

### 2.8 Caches by content, and control planes that serve files

- npm's cache is "a cacache-based content-addressable cache", and "All data that passes through
  the cache is fully verified for integrity on both insertion and extraction"
  ([npm cache][npm-cache]).
- In Bazel's remote execution API, content in the CAS "is addressed by the digest of its binary
  data", `FindMissingBlobs` tells "which ones are already present", and execution fails on "a
  missing input" ([remote_execution.proto][reapi]).
- Spegel is "a stateless cluster local OCI registry mirror" that serves "images from the local
  cache first" ([Spegel][spegel]).
- "Puppet Server includes a file server for transferring static file content to agents", and "By
  default, any node with a valid certificate can access the files"
  ([Puppet file serving][puppet-files]). The Salt file server is built into the master and uses
  the same authentication and encryption as the rest of Salt ([Salt file server][salt-files]).
- Kubernetes does not serve files: "The data stored in a ConfigMap cannot exceed 1 MiB"
  ([ConfigMap][k8s-configmap]). No page read showed a Nomad server that hosts artifacts.

## 3. Node agents and local enforcement

### 3.1 Kubernetes: the kubelet and the node lifecycle

- A watch streams changes after a `resourceVersion`. A client whose version is gone gets `410
  Gone` and must clear its cache and list again; etcd keeps changes for five minutes by default
  ([API concepts][k8s-api-concepts]).
- Mounted Secrets are updated in an "eventually-consistent" way, and the delay "can be as long as
  the kubelet sync period + cache propagation delay" ([Secret][k8s-secret]). `--sync-frequency`
  defaults to one minute ([kubelet reference][k8s-kubelet-ref]).
- The kubelet's webhook authorization caches an "authorized" answer for five minutes and an
  "unauthorized" one for 30 seconds ([kubelet reference][k8s-kubelet-ref]). The API server's
  webhook authorizer has the same TTLs and a required `failurePolicy` of `NoOpinion` or `Deny`
  ([authorization][k8s-authz]).
- The CRI "is a plugin interface which enables the kubelet to use a wide variety of container
  runtimes" ([CRI][k8s-cri]). **Inference:** running containers and deciding access are separate
  parts.

### 3.2 Nomad clients

- `token_ttl`, `policy_ttl` and `role_ttl` default to 30 seconds. "If a client cannot reach a
  server, for example because of an outage, the TTL will be ignored and the cached value used"
  ([ACL configuration][nomad-acl]).
- Clients that do not heartbeat in time are `down`; defaults are a grace of ten seconds and a
  failover TTL of five minutes ([server configuration][nomad-server]).

### 3.3 Consul

- `down_policy` is one of "allow", "deny", "extend-cache" or "async-cache", and "extend-cache" is
  the default: cached objects are used whatever their expiry. TTLs default to 30 seconds and
  `default_policy` to `allow` ([ACL configuration][consul-acl]).
- For custom proxies, "changing intentions to deny access does not terminate existing
  connections", and the advice is to "Configure connections to terminate after a maximum
  lifetime" ([custom proxy][consul-custom-proxy]).
- Consul Dataplane "removes the need to run client agents on every node", over "a single gRPC
  connection to the Consul servers" ([dataplane][consul-dataplane]).

### 3.4 Envoy and xDS

- When the management server is unreachable, "The last known configuration received by Envoy
  will persist until the connection is reestablished". An opt-in resource TTL "allows Envoy to
  remove a set of resources after a specified period of time". An ACK "does not mean that the
  configuration has been applied successfully". "Since Envoy's xDS APIs are eventually
  consistent, traffic may drop briefly during updates" ([xDS protocol][envoy-xds]). Envoy "will
  latch on to the previous configuration while actively retrying" ([management server][envoy-mgmt]).
- The `ext_authz` filter calls an outside service to decide each request. With
  `failure_mode_allow` set to `false`, "the filter will `reject` client requests and return
  `Forbidden` if communication with the authorization service has failed, or if the authorization
  service has returned an HTTP 5xx error". It "Defaults to `false`"
  ([ext_authz][envoy-extauthz], re-opened).
- Network RBAC checks on the first byte by default; `CONTINUOUS` applies policies "as data
  arrives" ([RBAC][envoy-rbac]).
- The `jwt_authn` filter's `audiences` is a list of strings: "A JWT containing any of these
  audiences will be accepted"; its examples are not URLs. Keys come from `remote_jwks` or
  `local_jwks` ([JWT authentication][envoy-jwt]).
- HTTP/2 `connection_keepalive` sends PINGs, and "If the remote peer does not respond within the
  configured timeout, the connection will be aborted" ([protocol options][envoy-keepalive]).

### 3.5 Istio ambient: ztunnel

- Inbound traffic is "transparently redirected to the node-local ztunnel", which "will apply
  Authorization Policies and forward the request only if the request passes these checks"
  ([ambient data plane][istio-ambient-dp]). ztunnel takes a reduced set of xDS resources from
  istiod and "obtains mTLS certificates for the Service Accounts of all pods" on its node
  ([ambient control plane][istio-ambient-cp]). Two xDS configurations, workloads and policies,
  need "orders of magnitude less configuration" for meshes of "100,000 pods"
  ([Rust ztunnel][istio-ztunnel-blog]).
- When policies change, ztunnel re-checks open connections and closes those that are no longer
  allowed, logging "connection {} closed because it's no longer allowed after a policy update"
  ([connection_manager.rs][ztunnel-conn]).

### 3.6 OPA

- Bundles are polled every 60 to 120 seconds by default ([configuration][opa-config]). A
  persisted bundle "allows OPA to start with the most recently activated bundle in case OPA cannot
  communicate with the bundle server", and when a signature check fails "OPA continues using its
  existing bundle". No bundle expiry is documented ([bundles][opa-bundles]).
- `/health?bundles` is ready once "all configured bundles have been activated"
  ([REST API][opa-rest]). Decision logs are uploaded in chunks, requeued with backoff on failure
  ([decision logs][opa-logs]).
- "OPA-Envoy plugin extends OPA with a gRPC server that implements the Envoy External
  Authorization API." It is often deployed as a sidecar, and "it can also be used in other
  environments as a standalone process running next to Envoy" ([OPA-Envoy][opa-envoy],
  re-opened).
- "Agentgateway is API-compatible with the Envoy External Authorization gRPC service." The page
  does not say whether it fails open or closed; its console steps choose "Deny" for the failure
  mode ([agentgateway external authorization][agw-extauthz], re-opened).

### 3.7 Tailscale and Cloudflare

- "Central ACL policies are enforced by each Tailscale node's incoming packet filter"
  ([how Tailscale works][ts-how]). No page found says what a policy edit does to open
  connections.
- "Access can immediately revoke a single user session across all applications", and after
  logout "all previously issued tokens will stop being accepted in 20-30 seconds"
  ([session management][cf-session]). The application token is a JWT that "may be used to
  validate requests on your origin", valid 24 hours by default ([authorization cookie][cf-cookie]).
  **Inference:** an origin that checks the token itself is bounded only by its expiry.
- "Cloudflare Tunnel does not use an inbound listener on your origin" ([Tunnel][cf-tunnel]).
- For an MCP server behind its portals, Cloudflare's fix for callers who go around it is: "If you
  want to enforce authentication through Cloudflare Access, configure Access as the server's
  OAuth provider" ([portals][cf-portals], re-opened).

### 3.8 Leases, fencing and call chains

- A lease "gives its holder specified rights over property for a limited period of time"; a
  server that cannot reach a holder "must delay writes … until that lease has expired"; this
  holds only without "clock failure" ([Gray and Cheriton][leases-paper], quoted from
  [a summary][leases-summary] because the paper is a scan).
- Kubernetes uses Leases "to ensure only one instance of a component is running at any given
  time", for "control plane components like `kube-controller-manager` and `kube-scheduler` in HA
  configurations, where only one instance of the component should be actively running", and for
  node heartbeats. Each `kube-apiserver` "uses the Lease API to publish its identity", which lets
  clients "discover how many instances of `kube-apiserver` are operating"
  ([Leases][k8s-leases], re-opened). The API server itself is not elected: "You can run several
  instances of kube-apiserver and balance traffic between those instances"
  ([Kubernetes architecture][k8s-arch]).
- etcd "ensures linearizability for all other operations by default"; serializable reads "may
  access stale data" ([etcd guarantees][etcd]).
- A fencing token is "a number that increases" each time a lock is granted, and the storage
  server rejects a write with an older one. It "requires the storage server to take an active
  role in checking tokens". A client paused past its lease "may go ahead and make some unsafe
  change", and "You simply cannot make any assumptions about timing" ([Kleppmann][kleppmann],
  re-opened).
- Transaction tokens are "short-lived, signed JWTs that assert the identity of a user or a
  workload and an authorization context", carried "throughout the Call Chain within a Trust
  Domain" ([transaction tokens][txn-tokens]). In RFC 8693 the `act` claim says "that delegation
  has occurred", and chains nest one `act` in another ([RFC 8693][rfc8693]).
- A policy enforcement point "Enforces policy decisions in response to a request from a subject
  requesting access to a protected object", and "the access control decisions are made by the
  policy decision point" ([NIST glossary][nist-pep], re-opened). A policy decision point
  "Computes access decisions by evaluating the applicable digital policies and metapolicies"
  ([NIST glossary][nist-pdp]).
- Zanzibar's "authorization decisions respect causal ordering of user actions and thus provide
  external consistency amid changes to access control lists and object contents"
  ([Zanzibar][zanzibar]).

### 3.9 Comparison

| System | How access reaches the machine | Bound while connected | Access when cut off | Open connections after a revocation |
| --- | --- | --- | --- | --- |
| Kubernetes kubelet | Watch from a version, scoped to the node ([API concepts][k8s-api-concepts]; [Node authorization][k8s-node-authz]) | Authorization cache 5 min / 30 s; tokens one hour | Last state; tokens checked live die with the Pod | Not applicable |
| Nomad client | Heartbeat and RPC | ACL cache 30 s ([ACL][nomad-acl]) | Cached ACL used, TTL ignored | Not applicable |
| Consul | Blocking queries; dataplane by xDS | ACL cache 30 s; leaf certificate at most 72 hours | `extend-cache` by default ([ACL][consul-acl]) | Untouched; a maximum lifetime advised ([custom proxy][consul-custom-proxy]) |
| Envoy | xDS with versions and ACK or NACK | Eventual; no stated bound | Last configuration persists; resource TTL opt-in; `ext_authz` fails closed by default ([xDS][envoy-xds]; [ext_authz][envoy-extauthz]) | First byte by default ([RBAC][envoy-rbac]) |
| Istio ambient | xDS: workloads and policies | Eventual; certificates 24 hours | Not stated in the pages read | Closed by ztunnel ([connection_manager.rs][ztunnel-conn]) |
| OPA | Bundle poll | At most the poll interval | Last activated bundle; no expiry ([bundles][opa-bundles]) | Decided per request |
| Tailscale | Network map from the coordination server | Not documented | Existing connections continue ([hypergrowth][ts-hyper]) | Not documented |
| Cloudflare Access | Edge network | 20 to 30 s at the edge ([sessions][cf-session]) | **Inference:** an origin, until token expiry | Not applicable |

## 4. MCP across machines, and its gateways

### 4.1 Revision 2026-07-28

- The release "officially retired the `initialize`/`initialized` exchange along with the
  `Mcp-Session-Id` header"; "any request can land on any instance behind a plain round-robin load
  balancer"; and "Your gateway, rate limiter, or WAF can route and meter on those headers instead
  of parsing JSON bodies" ([release post][mcp-release]).
- "All protocol metadata travels in the message body", in `_meta.io.modelcontextprotocol/*`
  fields ([transports][mcp-transports]).
- Streamable HTTP "mirrors selected JSON-RPC body fields into HTTP headers so that intermediaries
  (load balancers, gateways, observability tooling) can route and inspect requests". Servers
  reject a header that does not match the body. Servers "MUST validate the `Origin` header";
  "When running locally, servers SHOULD bind only to localhost"; and "Servers SHOULD implement
  proper authentication for all connections". "Closing the SSE response stream MUST be treated
  by the server as cancellation of that request" ([Streamable HTTP][mcp-shttp]).
- The server answers each request "with either a single JSON object or a Server-Sent Events
  (SSE) stream scoped to that request", and the client must support both. The cancellation rule
  above speaks of closing the SSE response stream ([Streamable HTTP][mcp-shttp], re-opened).
  **Inference:** a response sent as one JSON object, and a connection cut silently without a
  close, are outside that rule.
- The roadmap says the release "made a remote MCP server a normal HTTP workload". Its third
  priority, agent identity, lists DPoP, workload identity federation, ID-JAG and RFC 8693 token
  exchange, "coordinated with the IETF OAuth and WIMSE working groups" ([roadmap][mcp-roadmap]).
- The Interceptors working group describes "a sprawling landscape of sidecars, proxies, and
  gateways for cross-cutting concerns that are largely non-reusable and non-interoperable"
  ([Interceptors charter][mcp-interceptors]). The Enterprise interest group names "authorization
  handoff across proxies, and policy enforcement at the gateway"
  ([Enterprise IG][mcp-enterprise-ig]). A Gateways interest group met on 2026-09-09
  ([Gateways IG][mcp-gateways-ig]).

### 4.2 Security practice for servers that run locally

All of this is from the MCP security best practices of revision 2026-07-28
([best practices][mcp-bp], re-opened).

- Local servers "may have direct access to the user's system and may be accessible to other
  processes running on the user's machine". Servers meant to run locally should "Use the `stdio`
  transport to limit access to just the MCP client", or over HTTP "Require an authorization
  token" or "Use unix domain sockets or other Interprocess Communication (IPC) mechanisms with
  restricted access".
- For authorization URLs, "The `http://` scheme is acceptable only for loopback addresses (such
  as `localhost`, `127.0.0.1`, or `::1`) during local development; authorization servers in
  production **MUST** use `https://`." Clients should reject `http://` OAuth URLs "except for
  loopback addresses … during development", in line with OAuth 2.1 section 1.5
  ([OAuth 2.1][oauth21]).
- Servers "**MUST NOT** treat possession of a state handle as authentication". Proxies that spawn
  stdio servers are an escalation path, and "MCP proxy services **SHOULD** implement …
  sandboxing or containerization for spawned processes".

### 4.3 Gateways

| Gateway | How clients authenticate | How servers run | How credentials are added | Across hosts |
| --- | --- | --- | --- | --- |
| Docker MCP Gateway, v0.44.3 ([repository][docker-mcp-gw]) | stdio by default; on HTTP transports `MCP_GATEWAY_AUTH_TOKEN` "for localhost authentication to prevent dns rebinding attacks" ([flags][docker-mcp-flags]) | `docker run --rm -i --init --security-opt no-new-privileges`, with `--network none` when the server disables network, over stdio ([clientpool.go][docker-mcp-clientpool]) | Secrets "mounted only into the target container at runtime" ([Docker blog][docker-mcp-blog]) | None documented |
| agentgateway, docs 1.7.x ([overview][agw]) | It "acts as a resource server" ([MCP authentication][agw-mcp-authn]) | Proxies and combines MCP targets | Static keys, signed JWTs, RFC 7523 and ID-JAG; incoming credentials are removed by default ([backend authentication][agw-backend-authn]) | Client certificates for mutual TLS to backends ([backend TLS][agw-backend-tls]) |
| Microsoft MCP Gateway ([README][ms-mcp-gw]) | Entra ID tokens; "Each request carries its protocol metadata and is independently authorized" | StatefulSets with a tool router | Workload identity per tool | One cluster; private networking advised |
| Cloudflare ([authorization][cf-mcp-auth]; [portals][cf-portals]) | Access, OAuth or service tokens | Stateless Workers | Upstream token in `props`, "encrypted and handed to your MCP server with every request" | Private servers through Tunnel ([Tunnel][cf-tunnel]) |
| IBM ContextForge, v1.1.0 ([README][ibm-cf]) | Basic, JWT or SSO | Federates MCP, A2A and REST | Encrypted stored credentials | "Cross-gateway calls forward the user's bearer token" |
| AWS AgentCore Gateway ([inbound][aws-agentcore-in]) | JWT, SigV4, or offloaded, "otherwise any caller can reach your target" | Managed | Its role, the caller's credentials, OAuth, or passthrough, "not the recommended approach for production" | Managed |
| ToolHive ([network isolation][toolhive-netiso]) | Ingress proxy that by default accepts only the container's hostname and loopback; OIDC issuer and audience checks with Cedar rules that deny by default ([auth][toolhive-auth]). The proxy binds `127.0.0.1` by default, and `--host` changes it ([run MCP servers][toolhive-run], re-opened) | In containers; its troubleshooting asks to "Check that Docker, Podman, or Colima is running" ([run MCP servers][toolhive-run]); "The MCP server container connects only to the internal network"; stdio servers "expose no ports" | Not covered | Not covered |

### 4.4 Going around a gateway, as each documents it

- Cloudflare: "Blocked users can still connect to the server (and bypass your Access policies) by
  using its direct URL" ([portals][cf-portals]).
- Microsoft: "Requests without a valid `X-Gateway-Secret` header are rejected with `401
  Unauthorized`" ([README][ms-mcp-gw]).
- AWS: "Front your AgentCore Runtime with an AgentCore Gateway so that the gateway is the only
  entry point … The gateway stamps the source of every request it forwards, and the runtime
  validates that source on the way in" ([source validation][aws-agentcore-src]).
- Consul: a proxy listening on an address other than loopback "may expose it externally" and
  "bypass the service mesh's access enforcement" ([built-in proxy][consul-builtin-proxy]).
- Nomad: "To fully isolate your workloads make sure to bind them only to the `loopback`
  interface" ([Consul service mesh on Nomad][nomad-mesh]).

### 4.5 Tools that keep a server off the network

- "Ghostunnel is a simple TLS proxy with mutual authentication support for securing non-TLS
  backend applications." "Listeners and targets are restricted to localhost and UNIX sockets
  unless explicitly overridden." It can take identities from the SPIFFE Workload API, and flags
  such as `--allow-uri` authorize clients by their certificate ([ghostunnel][ghostunnel],
  re-opened). The flags are `--allow-all`, `--allow-cn`, `--allow-ou`, `--allow-dns`,
  `--allow-uri`, `--allow-policy` for an Open Policy Agent policy, and `--allow-spki-pin`, and
  "Multiple flags are treated as a logical disjunction (OR)" (same page, re-opened).
- OpenSSH's `-L` forwards to "the given host and port, or Unix socket, on the remote side"
  ([ssh][openssh]). "By default, ssh(1) binds local port forwardings to the loopback address"
  (`GatewayPorts`), and a forwarded Unix socket is created with mode 0177, "readable and writable
  only by the owner" (`StreamLocalBindMask`) ([ssh_config][openssh-config], re-opened).
- When no host address is given, "the Docker daemon publishes ports to all host addresses
  (`0.0.0.0` and `[::]`)", and "Publishing container ports is insecure by default". In releases
  older than 28.0.0, hosts within the same L2 segment can reach ports published to localhost
  ([port publishing][docker-ports], re-opened).
- `docker network create --internal` drops all traffic to or from other networks
  ([network create][docker-network-create]). The daemon can be reached over SSH or over TLS with
  `tlsverify` ([protect access][docker-protect]), and serves its API at
  `unix:///var/run/docker.sock` ([dockerd][dockerd]).
- The CSI specification splits a driver into a controller plugin that "MAY be run anywhere" and a
  node plugin that "MUST be run on the Node" where the volume is used ([CSI][csi-spec]).
- The protocol's official Rust SDK, crate `rmcp`, supports stdio and Streamable HTTP
  ([Rust SDK][rmcp], re-opened).

### 4.6 A2A

- "Identity information is handled at the protocol layer, not within A2A semantics." TLS is
  required in production, the server "**MUST** authenticate every incoming request", and the
  schemes include mutual TLS. "Credentials SHOULD be bound to the agent which originated the
  request" ([A2A specification][a2a-spec]; [enterprise ready][a2a-enterprise]).

## 5. Machines lost, partitions and failover

### 5.1 Kubernetes

- The node controller checks nodes every five seconds and, after the monitor grace period, sets
  `Ready` to `Unknown` and taints the node `node.kubernetes.io/unreachable`
  ([nodes][k8s-nodes]). The grace period defaults to 50 seconds
  ([controller manager reference][k8s-kcm-ref]). In a cloud, the controller "asks the cloud
  provider if the VM for that node is still available. If not, the node controller deletes the
  node" ([nodes][k8s-nodes]).
- A toleration "with tolerationSeconds=300" is added automatically, so "Pods remain bound to Nodes
  for 5 minutes". "The decision to delete the pods cannot be communicated to the kubelet until
  communication with the API server is re-established. In the meantime, the pods that are
  scheduled for deletion may continue to run on the partitioned node"
  ([taints and tolerations][k8s-taints]).
- A StatefulSet keeps "at most one Pod with a given identity", and "A Pod is not deleted
  automatically when a node is unreachable". When the node returns, the kubelet "kills the Pod".
  Force deletion asserts "that the Pod in question will never again make contact", and "can lead
  to the duplication of a still-running Pod" ([force delete][k8s-force-delete]).
- "When the kubelet starts, it automatically starts all defined static Pods"
  ([static Pods][k8s-static-pod]).
- A Node's name: "Kubernetes also assumes that a resource with the same name is the same object.
  In the case of a Node, it is implicitly assumed that an instance using the same name will have
  the same state (e.g. network settings, root disk contents) and attributes like node labels."
  "If the Node needs to be replaced or updated significantly, the existing Node object needs to be
  removed from API server first and re-added after the update" ([nodes][k8s-nodes], re-opened).

### 5.2 Nomad

- `lost_after` is how long a client may take to reconnect before its allocations are lost, and
  meanwhile "allocations may continue to run on the disconnected client". `stop_on_client_after`
  is "a duration after which a disconnected Nomad client will stop its allocations", and "The
  Nomad client process must be running for this to occur". `replace` and `reconcile` choose which
  copy survives a return ([disconnect][nomad-disconnect]). The older fields were removed in 1.10
  ([upgrade notes][nomad-upgrade]).

### 5.3 State, leases and fencing

- See [leases, fencing and call chains](#38-leases-fencing-and-call-chains) for Kubernetes
  Leases, etcd's linearizable reads, fencing tokens and the lease paper.

### 5.4 A network cut silently

- Linux's `tcp_keepalive_time` is "How often TCP sends out keepalive messages when keepalive is
  enabled. Default: 2hours." For `tcp_retries2`, "The default value of 15 yields a hypothetical
  timeout of 924.6 seconds and is a lower bound for the effective timeout"
  ([ip-sysctl][kernel-tcp]).
- **Inference.** Where packets are dropped rather than refused, a peer that is not sending learns
  of the cut only from a keepalive of its own, from TCP's or from its protocol's, such as Envoy's
  HTTP/2 PING ([3.4](#34-envoy-and-xds)), or not at all while it waits.

## 6. Patterns across the systems

All of this section is inference from the sections above.

1. **One root of trust, held by the control plane, with a swappable signer.** A CA and a token
   key (Kubernetes), a keyring (Nomad), a built-in CA that can move to Vault (Consul), a server
   with upstream authorities (SPIRE), istiod.
2. **A node identity, bootstrapped once, then node-scoped issuance.** A bootstrap token and CSR,
   an introduction token, or node attestation leads to a node credential, and a node gets
   credentials and data only for what is placed on it: the Node authorizer, ztunnel, the SPIRE
   agent.
3. **Workload credentials as files or a local socket, refreshed in place.** The workload reloads
   them: the kubelet's projected token, Nomad's identity file, spiffe-helper, the Workload API.
4. **Two kinds of verifier.** A control plane that checks live state on each request revokes at
   once (bound tokens at the API server, SubjectAccessReview). A party that checks offline
   against published keys revokes only at expiry or when a key leaves the bundle.
5. **Enforcement on the receiving side, next to the server.** Consul's proxy, ztunnel,
   Tailscale's packet filter, the kubelet's webhook, Envoy in front of an application on loopback.
   The server itself is unmodified and kept off the network.
6. **The downward path is the weak one by default.** Kubernetes does not verify the kubelet's
   serving certificate by default and leaves Pods open; every gateway that forwards authority
   documents a way to stop callers going around it.
7. **Agents dial out.** Konnectivity, Nomad's reverse RPC, cloudflared, the Consul dataplane and
   OPA's polling all start from the node, which also handles NAT.
8. **Content moves by digest from anywhere.** The control plane pins the digest; the node fetches
   from a registry, mirror or layout and verifies; caches skip what they hold.
9. **Only expiry bounds revocation on a cut-off node.** Pushed configuration persists (Envoy, OPA,
   Tailscale), caches are extended (Nomad, Consul), and only credential lifetimes, Envoy's
   opt-in resource TTL and leases bound what a node cut off from its control plane still allows.
   Consul is the one system that states the bound.
10. **What happens to workloads and what happens to access are set separately.** A workload on a
    cut-off node keeps running (Kubernetes, Nomad); its access, and its stop, follow their own
    settings.
11. **Short-lived workload credentials are renewed on the node, by an agent with its own
    identity.** The kubelet requests and refreshes projected tokens, the SPIRE agent rotates
    SVIDs, and the Nomad client renews workload identities; none of them is fed new credentials
    by the control plane without a node identity.
12. **Deciding servers scale out; acting controllers elect a leader.** Several API servers serve
    at once behind a load balancer and read a store with linearizable reads, while the controller
    manager and the scheduler keep one active instance under a Lease. Fencing, where used, is
    checked by the store, not by clocks.
13. **A silent cut is not a close.** Cancellation by closing a stream (MCP) and refusal on a
    failed connection (Envoy) need someone to notice the cut: a keepalive with a timeout, or a
    close by an intermediary.

## 7. Not established

- No page read gave the revocation behaviour of Nomad's workload or node identities.
- No page read said whether a Linkerd or Tailscale policy change affects open connections.
- What Istio's proxies do while istiod is unreachable is stated only in secondary blogs.
- One researcher quoted Istio's data plane page as saying the CA rejects requests for identities
  not running on the node; another found that claim only in secondary sources. It is not used.
- Whether a Cloudflare revocation reaches an origin that checks the application token itself is
  not stated.
- The status of Kubernetes' `--pod-eviction-timeout` was found only in a secondary source.
- No page read showed Nomad sharing downloaded artifacts between allocations.
- The version label of the Consul pages could not be confirmed; they are cited as "latest".
- No charter of the MCP Gateways interest group was found.
- Whether agentgateway fails open or closed when its authorization service fails is not stated;
  only a console step that picks "Deny" was found.
- Whether a Conduit package archive pushed unchanged as one OCI layer keeps the digest that
  Conduit computes for it is an inference from the specifications, untested.
- No page read showed a gate or proxy that checks a lease term or a fencing token: ghostunnel
  authorizes a client by names in its certificate, a pinned key or an OPA policy.
- No page read shows ToolHive's proxy serving clients on another machine; it binds loopback by
  default and takes `--host` to change that.
- When a server learns of a silently cut connection, with and without a keepalive, was not
  measured; section 5.4 gives only the kernel's defaults.

## 8. Sources

Every page was read on 2026-11-06. "Re-opened" marks a page that the editor opened again on that
day. The labels are the ones the text above links.

| Label | URL | Version or date |
| --- | --- | --- |
| k8s-cpn | https://kubernetes.io/docs/concepts/architecture/control-plane-node-communication/ | Kubernetes docs v1.37; re-opened |
| k8s-kubelet-authn | https://kubernetes.io/docs/reference/access-authn-authz/kubelet-authn-authz/ | Last modified 2026-02-19 |
| k8s-node-authz | https://kubernetes.io/docs/reference/access-authn-authz/node/ | Docs v1.37 |
| k8s-tls-bootstrap | https://kubernetes.io/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/ | Docs v1.37 |
| k8s-projected | https://kubernetes.io/docs/concepts/storage/projected-volumes/ | Docs v1.37 |
| k8s-sa-config | https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/ | Last modified 2024-10-31; re-opened |
| k8s-sa-admin | https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/ | Docs v1.37, last modified 2026-08-05; re-opened |
| k8s-arch | https://kubernetes.io/docs/concepts/architecture/ | Docs v1.37, the version menu's first entry; the page states none; re-opened |
| k8s-volumes | https://kubernetes.io/docs/concepts/storage/volumes/ | Docs v1.37, the version menu's first entry; the page states none; re-opened |
| k8s-konnectivity | https://kubernetes.io/docs/tasks/extend-kubernetes/setup-konnectivity/ | Docs v1.37 |
| k8s-authn | https://kubernetes.io/docs/reference/access-authn-authz/authentication/ | Docs v1.37 |
| k8s-pod-api | https://kubernetes.io/docs/tasks/run-application/access-api-from-pod/ | Last modified 2023-02-19 |
| k8s-netpol | https://kubernetes.io/docs/concepts/services-networking/network-policies/ | Docs v1.37 |
| kep-4317 | https://www.kubernetes.dev/resources/keps/4317/ | KEP-4317, stable in v1.37 |
| kep-4412 | https://www.kubernetes.dev/resources/keps/4412/ | KEP-4412, stable milestone v1.38 |
| k8s-images | https://kubernetes.io/docs/concepts/containers/images/ | Docs v1.37 |
| k8s-imgvol | https://kubernetes.io/docs/tasks/configure-pod-container/image-volumes/ | Last modified 2026-02-03 |
| kep-4639 | https://github.com/kubernetes/enhancements/blob/master/keps/sig-node/4639-oci-volume-source/README.md | `master`, history to 2026-01-20 |
| cri-api | https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto | `master`; re-opened |
| k8s-cri | https://kubernetes.io/docs/concepts/architecture/cri/ | Last modified 2026-02-12 |
| k8s-api-concepts | https://kubernetes.io/docs/reference/using-api/api-concepts/ | Last modified 2026-07-03 |
| k8s-secret | https://raw.githubusercontent.com/kubernetes/website/main/content/en/docs/concepts/configuration/secret.md | `main` |
| k8s-kubelet-ref | https://kubernetes.io/docs/reference/command-line-tools-reference/kubelet/ | Docs v1.37, 2026-08-26 |
| k8s-kcm-ref | https://kubernetes.io/docs/reference/command-line-tools-reference/kube-controller-manager/ | Docs v1.37 |
| k8s-authz | https://raw.githubusercontent.com/kubernetes/website/main/content/en/docs/reference/access-authn-authz/authorization.md | `main` |
| k8s-nodes | https://kubernetes.io/docs/concepts/architecture/nodes/ | Last modified 2026-10-31; re-opened |
| k8s-taints | https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/ | Last modified 2026-07-27 |
| k8s-force-delete | https://kubernetes.io/docs/tasks/run-application/force-delete-stateful-set-pod/ | Last modified 2023-02-19 |
| k8s-static-pod | https://kubernetes.io/docs/tasks/configure-pod-container/static-pod/ | Docs v1.37 |
| k8s-leases | https://kubernetes.io/docs/concepts/architecture/leases/ | Last modified 2026-03-31; re-opened |
| k8s-configmap | https://kubernetes.io/docs/concepts/configuration/configmap/ | Docs v1.37 |
| csi-providers | https://secrets-store-csi-driver.sigs.k8s.io/providers | No version shown |
| csi-spec | https://github.com/container-storage-interface/spec/blob/master/spec.md | `master` |
| nomad-tls | https://developer.hashicorp.com/nomad/docs/secure/traffic/tls | Nomad docs v2.1.x |
| nomad-node-identity | https://developer.hashicorp.com/nomad/docs/architecture/cluster/node-identity | v2.1.x |
| nomad-1-11 | https://developer.hashicorp.com/nomad/docs/release-notes/nomad/v1-11-x | 1.11 release notes |
| nomad-intro | https://developer.hashicorp.com/nomad/commands/node/intro/create | v2.1.x |
| nomad-wi | https://developer.hashicorp.com/nomad/docs/concepts/workload-identity | v2.1.x |
| nomad-identity | https://developer.hashicorp.com/nomad/docs/job-specification/identity | v2.1.x |
| nomad-task-api | https://developer.hashicorp.com/nomad/api-docs/task-api | v2.1.x |
| nomad-keyring | https://developer.hashicorp.com/nomad/api-docs/operator/keyring | v2.1.x |
| nomad-client-rpc | https://github.com/hashicorp/nomad/blob/main/nomad/client_rpc.go | `main` |
| nomad-artifact | https://developer.hashicorp.com/nomad/docs/job-specification/artifact | v2.1.x |
| nomad-client | https://developer.hashicorp.com/nomad/docs/configuration/client#artifact-parameters | v2.1.x |
| nomad-exec | https://developer.hashicorp.com/nomad/docs/drivers/exec | v2.1.x |
| nomad-disconnect | https://developer.hashicorp.com/nomad/docs/job-specification/disconnect | v2.1.x |
| nomad-server | https://developer.hashicorp.com/nomad/docs/configuration/server | v2.1.x |
| nomad-acl | https://developer.hashicorp.com/nomad/docs/configuration/acl | v2.1.x |
| nomad-upgrade | https://developer.hashicorp.com/nomad/docs/upgrade/upgrade-specific | v2.1.x |
| nomad-mesh | https://developer.hashicorp.com/nomad/docs/job-declare/consul-service-mesh | v2.1.x |
| go-getter | https://github.com/hashicorp/go-getter/blob/main/README.md | `main` |
| consul-intention | https://developer.hashicorp.com/consul/docs/secure-mesh/intention | "latest" |
| consul-cert | https://developer.hashicorp.com/consul/docs/secure-mesh/certificate | "latest" |
| consul-ca | https://developer.hashicorp.com/consul/docs/connect/ca/consul | "latest" |
| consul-acl | https://developer.hashicorp.com/consul/docs/reference/agent/configuration-file/acl | "latest" |
| consul-custom-proxy | https://developer.hashicorp.com/consul/docs/connect/proxy/custom | "latest" |
| consul-dataplane | https://developer.hashicorp.com/consul/docs/architecture/control-plane/dataplane | "latest" |
| consul-builtin-proxy | https://developer.hashicorp.com/consul/docs/reference/proxy/built-in | "latest" |
| envoy-xds | https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol | Envoy 1.41.0-dev |
| envoy-mgmt | https://www.envoyproxy.io/docs/envoy/latest/configuration/overview/mgmt_server | Envoy 1.41.0-dev |
| envoy-extauthz | https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/ext_authz/v3/ext_authz.proto | Envoy 1.41.0-dev-1830cb; re-opened |
| envoy-rbac | https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/network/rbac/v3/rbac.proto | Envoy 1.41.0-dev |
| envoy-jwt | https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/jwt_authn/v3/config.proto | Envoy 1.41.0-dev-1830cb; re-opened |
| envoy-keepalive | https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/core/v3/protocol.proto | Envoy 1.41.0-dev-1830cb; re-opened |
| istio-ambient-dp | https://istio.io/latest/docs/ambient/architecture/data-plane/ | Istio 1.31.4 |
| istio-ambient-cp | https://istio.io/latest/docs/ambient/architecture/control-plane/ | Istio 1.31.4 |
| istio-hbone | https://istio.io/latest/docs/ambient/architecture/hbone/ | Istio 1.31.4 |
| istio-security | https://istio.io/latest/docs/concepts/security/ | Istio 1.31 |
| istio-pilot-agent | https://istio.io/latest/docs/reference/commands/pilot-agent/ | Istio 1.31 |
| istio-ztunnel-blog | https://istio.io/latest/blog/2023/rust-based-ztunnel/ | 2023-02-28 |
| istio-ambient-sec | https://istio.io/latest/blog/2022/ambient-security/ | 2022-09-07 |
| ztunnel-conn | https://github.com/istio/ztunnel/blob/master/src/proxy/connection_manager.rs | `master` |
| linkerd-mtls | https://linkerd.io/2/features/automatic-mtls/ | Site of 2026, 2.20 banner |
| linkerd-authz | https://linkerd.io/2/reference/authorization-policy/ | Site of 2026 |
| spiffe-id | https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE-ID.md | `main`, Stable |
| spiffe-x509 | https://github.com/spiffe/spiffe/blob/main/standards/X509-SVID.md | `main`, Stable |
| spiffe-jwt | https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md | `main`, Stable; re-opened |
| spiffe-bundle | https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Trust_Domain_and_Bundle.md | `main`, Stable |
| spiffe-wapi | https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_API.md | `main`, Stable |
| spiffe-wep | https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_Endpoint.md | `main`, Stable |
| spiffe-fed | https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Federation.md | `main`, Stable |
| spiffe-wit | https://spiffe.io/docs/latest/spiffe-specs/wit-svid/ | Incubating |
| spiffe-roadmap | https://spiffe.io/blog/2026-08-18-spiffe-standard-roadmap/ | 2026-08-18 |
| spire-concepts | https://spiffe.io/docs/latest/spire-about/spire-concepts/ | "latest" |
| spire-server | https://spiffe.io/docs/latest/deploying/spire_server/ | "latest" |
| spire-agent | https://spiffe.io/docs/latest/deploying/spire_agent/ | "latest" |
| spire-mint | https://raw.githubusercontent.com/spiffe/spire-api-sdk/main/proto/spire/api/server/svid/v1/svid.proto | `main` |
| spire-deleg | https://docs.rs/crate/spire-api/latest | spire-api 0.11.0, 2026-10-28 |
| spire-oidc | https://github.com/spiffe/spire/blob/main/support/oidc-discovery-provider/README.md | `main` |
| spiffe-helper | https://github.com/spiffe/spiffe-helper/blob/main/README.md | `main`, no version shown |
| csi-spiffe | https://cert-manager.io/docs/usage/csi-driver-spiffe/ | "latest" |
| spiffe-rustls | https://docs.rs/spiffe-rustls/0.6.1/spiffe_rustls/ | 0.6.1 |
| wireguard | https://www.wireguard.com/ | Site copyright 2015 to 2022 |
| ts-how | https://tailscale.com/blog/how-tailscale-works | 2020-03-20 |
| ts-keys | https://tailscale.com/kb/1028/key-expiry | Validated 2026-01-05 |
| ts-hyper | https://tailscale.com/blog/hypergrowth-isnt-always-easy | 2026-01-21 |
| ts-serve | https://tailscale.com/kb/1312/serve | Validated 2026-01-20 |
| cf-session | https://developers.cloudflare.com/cloudflare-one/identity/users/session-management/ | 2026-11-02 |
| cf-cookie | https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/ | 2026-11-02 |
| cf-tunnel | https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/ | 2026-08-04 |
| cf-mcp-auth | https://developers.cloudflare.com/agents/model-context-protocol/authorization/ | 2026-10-27 |
| cf-portals | https://developers.cloudflare.com/cloudflare-one/access-controls/ai-controls/mcp-portals/ | 2026-11-05; re-opened |
| mcp-authz | https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization | Revision 2026-07-28; re-opened |
| mcp-authz-sec | https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations | Revision 2026-07-28 |
| mcp-shttp | https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http | Revision 2026-07-28; re-opened |
| mcp-transports | https://modelcontextprotocol.io/specification/2026-07-28/basic/transports | Revision 2026-07-28 |
| mcp-bp | https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices | Revision 2026-07-28; re-opened |
| mcp-cc | https://github.com/modelcontextprotocol/ext-auth/blob/main/specification/stable/oauth-client-credentials/2026-07-28/oauth-client-credentials.mdx | Stable 2026-07-28 |
| mcp-ext-auth | https://github.com/modelcontextprotocol/ext-auth | `main` |
| mcp-release | https://blog.modelcontextprotocol.io/posts/2026-07-28/ | 2026-07-28 |
| mcp-roadmap | https://modelcontextprotocol.io/development/roadmap | Updated 2026-08-22 |
| mcp-interceptors | https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/community/working-groups/interceptors.mdx | `main` |
| mcp-enterprise-ig | https://modelcontextprotocol.io/community/interest-groups/enterprise | No version shown |
| mcp-gateways-ig | https://meet.modelcontextprotocol.io/2026/09/gateways-interest-group-meeting-BosfPpSu5SpU | Meeting of 2026-09-09 |
| sep-1933 | https://github.com/modelcontextprotocol/modelcontextprotocol/pull/1933 | Open, last updated 2026-11-02 |
| sep-1932 | https://github.com/modelcontextprotocol/modelcontextprotocol/pull/1932 | Open |
| sep-2752 | https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2752 | Draft |
| mcp-pkg | https://modelcontextprotocol.io/registry/package-types | Registry in preview |
| mcp-registry-about | https://modelcontextprotocol.io/registry/about | Registry in preview |
| mcp-server-schema | https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json | Schema 2025-12-11 |
| mcpb-readme | https://github.com/modelcontextprotocol/mcpb/blob/main/README.md | `main` |
| mcpb-manifest | https://github.com/modelcontextprotocol/mcpb/blob/main/MANIFEST.md | v0.3, updated 2025-12-02 |
| mcpb-cli | https://github.com/modelcontextprotocol/mcpb/blob/main/CLI.md | `main` |
| rmcp | https://github.com/modelcontextprotocol/rust-sdk | `main`, no release shown; re-opened |
| rfc8705 | https://www.rfc-editor.org/rfc/rfc8705.html | February 2020 |
| rfc7662 | https://www.rfc-editor.org/rfc/rfc7662.html | October 2015 |
| rfc8693 | https://www.rfc-editor.org/rfc/rfc8693 | January 2020 |
| rfc9728 | https://www.rfc-editor.org/rfc/rfc9728.html | April 2025; re-opened |
| oauth21 | https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1-13#section-1.5 | draft-ietf-oauth-v2-1-13, 2025-05-28; re-opened |
| wimse | https://datatracker.ietf.org/doc/draft-ietf-wimse-workload-creds/ | draft-03, 2026-10-30 |
| txn-tokens | https://datatracker.ietf.org/doc/draft-ietf-oauth-transaction-tokens/ | draft-11, 2026-07-30 |
| vault-jwt | https://developer.hashicorp.com/vault/docs/auth/jwt | "latest" |
| oci-manifest | https://github.com/opencontainers/image-spec/blob/v1.1.1/manifest.md | image-spec v1.1.1 |
| oci-descriptor | https://github.com/opencontainers/image-spec/blob/v1.1.1/descriptor.md | image-spec v1.1.1 |
| oci-media-types | https://github.com/opencontainers/image-spec/blob/v1.1.1/media-types.md | image-spec v1.1.1 |
| oci-layout | https://github.com/opencontainers/image-spec/blob/v1.1.1/image-layout.md | image-spec v1.1.1 |
| oci-dist | https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md | distribution-spec v1.1.1 |
| oras | https://oras.land/docs/ | ORAS docs 1.3 |
| oras-push | https://oras.land/docs/how_to_guides/pushing_and_pulling | ORAS docs 1.3 |
| oras-cp | https://oras.land/docs/commands/oras_cp | ORAS docs 1.3 |
| helm-oci | https://helm.sh/docs/topics/registries/ | Helm docs 4.3.1 |
| flux-oci | https://fluxcd.io/flux/components/source/ocirepositories/ | Flux v2.10 |
| ctrd-content | https://github.com/containerd/containerd/blob/main/docs/content-flow.md | `main` |
| ctrd-store | https://github.com/containerd/containerd/blob/main/plugins/content/local/store.go | `main` |
| ctrd-handlers | https://github.com/containerd/containerd/blob/main/core/remotes/handlers.go | `main` |
| ctrd-hosts | https://github.com/containerd/containerd/blob/main/docs/hosts.md | `main` |
| swarm-services | https://docs.docker.com/engine/swarm/services/ | No version shown |
| docker-service-create | https://docs.docker.com/reference/cli/docker/service/create/ | No version shown |
| docker-protect | https://docs.docker.com/engine/security/protect-access/ | No version shown |
| docker-ports | https://docs.docker.com/engine/network/port-publishing/ | No version shown; re-opened |
| docker-network-create | https://docs.docker.com/reference/cli/docker/network/create/ | No version shown |
| dockerd | https://docs.docker.com/reference/cli/dockerd/ | No version shown |
| docker-mcp-gw | https://github.com/docker/mcp-gateway | v0.44.3, 2026-10-26 |
| docker-mcp-clientpool | https://github.com/docker/mcp-gateway/blob/7e0f34277c91855ad861eeff536b4fdc77420593/pkg/gateway/clientpool.go | Commit 7e0f342 |
| docker-mcp-flags | https://github.com/docker/mcp-gateway/blob/7e0f34277c91855ad861eeff536b4fdc77420593/docs/generator/reference/docker_mcp_gateway_run.yaml | Commit 7e0f342 |
| docker-mcp-blog | https://www.docker.com/blog/mcp-toolkit-gateway-explained/ | 2025-10-24 |
| toolhive-build | https://docs.stacklok.com/toolhive/guides-cli/build-containers | Updated 2026-11-02 |
| toolhive-run | https://docs.stacklok.com/toolhive/guides-cli/run-mcp-servers | Updated 2026-11-02; re-opened |
| toolhive-auth | https://docs.stacklok.com/toolhive/guides-cli/auth | Updated 2026-11-02 |
| toolhive-netiso | https://docs.stacklok.com/toolhive/guides-cli/network-isolation | Updated 2026-11-02 |
| agw | https://agentgateway.dev/llms.txt | Docs 1.7.x |
| agw-mcp-authn | https://agentgateway.dev/docs/standalone/latest/documentation/mcp/mcp-authn.md | Docs 1.7.x |
| agw-backend-authn | https://agentgateway.dev/docs/standalone/latest/documentation/configuration/security/backend-authn/key.md | Docs 1.7.x |
| agw-backend-tls | https://agentgateway.dev/docs/standalone/latest/documentation/configuration/security/backend-tls.md | Docs 1.7.x |
| agw-extauthz | https://agentgateway.dev/docs/standalone/latest/configuration/security/external-authz/ | "latest"; re-opened |
| ms-mcp-gw | https://github.com/microsoft/mcp-gateway/blob/cf9f5e9eff89276bc3fc42f0ae4d6109d70e5509/README.md | README of 2026-11-04 |
| ibm-cf | https://github.com/IBM/mcp-context-forge/blob/7d34038ca1f2057bf62c3cb27eb90818296b4947/README.md | v1.1.0, 2026-10-20 |
| aws-agentcore-in | https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-inbound-auth.html | No date shown |
| aws-agentcore-src | https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-target-http-runtime.html#gateway-target-http-runtime-source-validation | No date shown |
| a2a-spec | https://github.com/a2aproject/A2A/blob/v1.0.1/docs/specification.md | v1.0.1, 2026-05-28 |
| a2a-enterprise | https://github.com/a2aproject/A2A/blob/v1.0.1/docs/topics/enterprise-ready.md | v1.0.1 |
| ghostunnel | https://github.com/ghostunnel/ghostunnel/blob/master/README.md | `master`; re-opened |
| openssh | https://man.openbsd.org/ssh | OpenBSD-current, 2026-11-02 |
| openssh-config | https://man.openbsd.org/ssh_config | OpenBSD-current, 2026-10-30; re-opened |
| opa-config | https://www.openpolicyagent.org/docs/configuration | OPA docs v1.x |
| opa-bundles | https://www.openpolicyagent.org/docs/management-bundles | OPA docs v1.x |
| opa-rest | https://www.openpolicyagent.org/docs/rest-api | OPA docs v1.x |
| opa-logs | https://www.openpolicyagent.org/docs/management-decision-logs | OPA docs v1.x |
| opa-envoy | https://www.openpolicyagent.org/docs/envoy | No version shown; re-opened |
| puppet-files | https://help.puppet.com/core/current/Content/PuppetCore/file_serving.htm | Puppet Core 9.1.3 |
| salt-files | https://docs.saltproject.io/en/latest/ref/file_server/index.html | "latest" |
| npm-cache | https://docs.npmjs.com/cli/v11/commands/npm-cache | npm CLI 11.21.2 |
| npm-provenance | https://docs.npmjs.com/generating-provenance-statements | Edited 2026-05-04 |
| cosign-blob | https://docs.sigstore.dev/cosign/signing/signing_with_blobs/ | No date shown |
| cosign-verify | https://docs.sigstore.dev/cosign/verifying/verify/ | No date shown |
| redhat-referrers | https://www.redhat.com/en/blog/announcing-open-container-initiativereferrers-api-quayio-step-towards-enhanced-security-and-compliance | 2024-12-02 |
| reapi | https://github.com/bazelbuild/remote-apis/blob/main/build/bazel/remote/execution/v2/remote_execution.proto | `main` |
| spegel | https://github.com/spegel-org/spegel | No version shown |
| etcd | https://etcd.io/docs/v3.5/learning/api_guarantees/ | etcd v3.5 |
| kleppmann | https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html | 2016-02-08; re-opened |
| kernel-tcp | https://docs.kernel.org/networking/ip-sysctl.html | Linux kernel 7.4.0-rc1; re-opened |
| leases-paper | https://doi.org/10.1145/74850.74870 | SOSP 1989 |
| leases-summary | https://blog.acolyer.org/2014/10/31/leases-an-efficient-fault-tolerant-mechanism-for-distributed-file-cache-consistency/ | 2014-10-31 |
| nist-pep | https://csrc.nist.gov/glossary/term/policy_enforcement_point | NIST glossary, citing SP 800-162 and SP 1800-15C; re-opened |
| nist-pdp | https://csrc.nist.gov/glossary/term/policy_decision_point | NIST glossary, citing SP 800-162; re-opened |
| zanzibar | https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/ | USENIX ATC 2019 |

[k8s-cpn]: https://kubernetes.io/docs/concepts/architecture/control-plane-node-communication/
[k8s-kubelet-authn]: https://kubernetes.io/docs/reference/access-authn-authz/kubelet-authn-authz/
[k8s-node-authz]: https://kubernetes.io/docs/reference/access-authn-authz/node/
[k8s-tls-bootstrap]:
  https://kubernetes.io/docs/reference/access-authn-authz/kubelet-tls-bootstrapping/
[k8s-projected]: https://kubernetes.io/docs/concepts/storage/projected-volumes/
[k8s-sa-config]:
  https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
[k8s-sa-admin]: https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/
[k8s-arch]: https://kubernetes.io/docs/concepts/architecture/
[k8s-volumes]: https://kubernetes.io/docs/concepts/storage/volumes/
[k8s-konnectivity]: https://kubernetes.io/docs/tasks/extend-kubernetes/setup-konnectivity/
[k8s-authn]: https://kubernetes.io/docs/reference/access-authn-authz/authentication/
[k8s-pod-api]: https://kubernetes.io/docs/tasks/run-application/access-api-from-pod/
[k8s-netpol]: https://kubernetes.io/docs/concepts/services-networking/network-policies/
[kep-4317]: https://www.kubernetes.dev/resources/keps/4317/
[kep-4412]: https://www.kubernetes.dev/resources/keps/4412/
[k8s-images]: https://kubernetes.io/docs/concepts/containers/images/
[k8s-imgvol]: https://kubernetes.io/docs/tasks/configure-pod-container/image-volumes/
[kep-4639]:
  https://github.com/kubernetes/enhancements/blob/master/keps/sig-node/4639-oci-volume-source/README.md
[cri-api]: https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto
[k8s-cri]: https://kubernetes.io/docs/concepts/architecture/cri/
[k8s-api-concepts]: https://kubernetes.io/docs/reference/using-api/api-concepts/
[k8s-secret]:
  https://raw.githubusercontent.com/kubernetes/website/main/content/en/docs/concepts/configuration/secret.md
[k8s-kubelet-ref]: https://kubernetes.io/docs/reference/command-line-tools-reference/kubelet/
[k8s-kcm-ref]:
  https://kubernetes.io/docs/reference/command-line-tools-reference/kube-controller-manager/
[k8s-authz]:
  https://raw.githubusercontent.com/kubernetes/website/main/content/en/docs/reference/access-authn-authz/authorization.md
[k8s-nodes]: https://kubernetes.io/docs/concepts/architecture/nodes/
[k8s-taints]: https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/
[k8s-force-delete]:
  https://kubernetes.io/docs/tasks/run-application/force-delete-stateful-set-pod/
[k8s-static-pod]: https://kubernetes.io/docs/tasks/configure-pod-container/static-pod/
[k8s-leases]: https://kubernetes.io/docs/concepts/architecture/leases/
[k8s-configmap]: https://kubernetes.io/docs/concepts/configuration/configmap/
[csi-providers]: https://secrets-store-csi-driver.sigs.k8s.io/providers
[csi-spec]: https://github.com/container-storage-interface/spec/blob/master/spec.md
[nomad-tls]: https://developer.hashicorp.com/nomad/docs/secure/traffic/tls
[nomad-node-identity]:
  https://developer.hashicorp.com/nomad/docs/architecture/cluster/node-identity
[nomad-1-11]: https://developer.hashicorp.com/nomad/docs/release-notes/nomad/v1-11-x
[nomad-intro]: https://developer.hashicorp.com/nomad/commands/node/intro/create
[nomad-wi]: https://developer.hashicorp.com/nomad/docs/concepts/workload-identity
[nomad-identity]: https://developer.hashicorp.com/nomad/docs/job-specification/identity
[nomad-task-api]: https://developer.hashicorp.com/nomad/api-docs/task-api
[nomad-keyring]: https://developer.hashicorp.com/nomad/api-docs/operator/keyring
[nomad-client-rpc]: https://github.com/hashicorp/nomad/blob/main/nomad/client_rpc.go
[nomad-artifact]: https://developer.hashicorp.com/nomad/docs/job-specification/artifact
[nomad-client]:
  https://developer.hashicorp.com/nomad/docs/configuration/client#artifact-parameters
[nomad-exec]: https://developer.hashicorp.com/nomad/docs/drivers/exec
[nomad-disconnect]: https://developer.hashicorp.com/nomad/docs/job-specification/disconnect
[nomad-server]: https://developer.hashicorp.com/nomad/docs/configuration/server
[nomad-acl]: https://developer.hashicorp.com/nomad/docs/configuration/acl
[nomad-upgrade]: https://developer.hashicorp.com/nomad/docs/upgrade/upgrade-specific
[nomad-mesh]: https://developer.hashicorp.com/nomad/docs/job-declare/consul-service-mesh
[go-getter]: https://github.com/hashicorp/go-getter/blob/main/README.md
[consul-intention]: https://developer.hashicorp.com/consul/docs/secure-mesh/intention
[consul-cert]: https://developer.hashicorp.com/consul/docs/secure-mesh/certificate
[consul-ca]: https://developer.hashicorp.com/consul/docs/connect/ca/consul
[consul-acl]:
  https://developer.hashicorp.com/consul/docs/reference/agent/configuration-file/acl
[consul-custom-proxy]: https://developer.hashicorp.com/consul/docs/connect/proxy/custom
[consul-dataplane]:
  https://developer.hashicorp.com/consul/docs/architecture/control-plane/dataplane
[consul-builtin-proxy]: https://developer.hashicorp.com/consul/docs/reference/proxy/built-in
[envoy-xds]: https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol
[envoy-mgmt]:
  https://www.envoyproxy.io/docs/envoy/latest/configuration/overview/mgmt_server
[envoy-extauthz]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/ext_authz/v3/ext_authz.proto
[envoy-rbac]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/network/rbac/v3/rbac.proto
[envoy-jwt]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/extensions/filters/http/jwt_authn/v3/config.proto
[envoy-keepalive]:
  https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/core/v3/protocol.proto
[istio-ambient-dp]: https://istio.io/latest/docs/ambient/architecture/data-plane/
[istio-ambient-cp]: https://istio.io/latest/docs/ambient/architecture/control-plane/
[istio-hbone]: https://istio.io/latest/docs/ambient/architecture/hbone/
[istio-security]: https://istio.io/latest/docs/concepts/security/
[istio-pilot-agent]: https://istio.io/latest/docs/reference/commands/pilot-agent/
[istio-ztunnel-blog]: https://istio.io/latest/blog/2023/rust-based-ztunnel/
[istio-ambient-sec]: https://istio.io/latest/blog/2022/ambient-security/
[ztunnel-conn]: https://github.com/istio/ztunnel/blob/master/src/proxy/connection_manager.rs
[linkerd-mtls]: https://linkerd.io/2/features/automatic-mtls/
[linkerd-authz]: https://linkerd.io/2/reference/authorization-policy/
[spiffe-id]: https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE-ID.md
[spiffe-x509]: https://github.com/spiffe/spiffe/blob/main/standards/X509-SVID.md
[spiffe-jwt]: https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md
[spiffe-bundle]:
  https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Trust_Domain_and_Bundle.md
[spiffe-wapi]: https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_API.md
[spiffe-wep]:
  https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_Endpoint.md
[spiffe-fed]: https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Federation.md
[spiffe-wit]: https://spiffe.io/docs/latest/spiffe-specs/wit-svid/
[spiffe-roadmap]: https://spiffe.io/blog/2026-08-18-spiffe-standard-roadmap/
[spire-concepts]: https://spiffe.io/docs/latest/spire-about/spire-concepts/
[spire-server]: https://spiffe.io/docs/latest/deploying/spire_server/
[spire-agent]: https://spiffe.io/docs/latest/deploying/spire_agent/
[spire-mint]:
  https://raw.githubusercontent.com/spiffe/spire-api-sdk/main/proto/spire/api/server/svid/v1/svid.proto
[spire-deleg]: https://docs.rs/crate/spire-api/latest
[spire-oidc]:
  https://github.com/spiffe/spire/blob/main/support/oidc-discovery-provider/README.md
[spiffe-helper]: https://github.com/spiffe/spiffe-helper/blob/main/README.md
[csi-spiffe]: https://cert-manager.io/docs/usage/csi-driver-spiffe/
[spiffe-rustls]: https://docs.rs/spiffe-rustls/0.6.1/spiffe_rustls/
[wireguard]: https://www.wireguard.com/
[ts-how]: https://tailscale.com/blog/how-tailscale-works
[ts-keys]: https://tailscale.com/kb/1028/key-expiry
[ts-hyper]: https://tailscale.com/blog/hypergrowth-isnt-always-easy
[ts-serve]: https://tailscale.com/kb/1312/serve
[cf-session]:
  https://developers.cloudflare.com/cloudflare-one/identity/users/session-management/
[cf-cookie]: https://developers.cloudflare.com/cloudflare-one/identity/authorization-cookie/
[cf-tunnel]:
  https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/
[cf-mcp-auth]: https://developers.cloudflare.com/agents/model-context-protocol/authorization/
[cf-portals]:
  https://developers.cloudflare.com/cloudflare-one/access-controls/ai-controls/mcp-portals/
[mcp-authz]: https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization
[mcp-authz-sec]:
  https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations
[mcp-shttp]:
  https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http
[mcp-transports]: https://modelcontextprotocol.io/specification/2026-07-28/basic/transports
[mcp-bp]:
  https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices
[mcp-cc]:
  https://github.com/modelcontextprotocol/ext-auth/blob/main/specification/stable/oauth-client-credentials/2026-07-28/oauth-client-credentials.mdx
[mcp-ext-auth]: https://github.com/modelcontextprotocol/ext-auth
[mcp-release]: https://blog.modelcontextprotocol.io/posts/2026-07-28/
[mcp-roadmap]: https://modelcontextprotocol.io/development/roadmap
[mcp-interceptors]:
  https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/community/working-groups/interceptors.mdx
[mcp-enterprise-ig]: https://modelcontextprotocol.io/community/interest-groups/enterprise
[mcp-gateways-ig]:
  https://meet.modelcontextprotocol.io/2026/09/gateways-interest-group-meeting-BosfPpSu5SpU
[sep-1933]: https://github.com/modelcontextprotocol/modelcontextprotocol/pull/1933
[sep-1932]: https://github.com/modelcontextprotocol/modelcontextprotocol/pull/1932
[sep-2752]: https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2752
[mcp-pkg]: https://modelcontextprotocol.io/registry/package-types
[mcp-registry-about]: https://modelcontextprotocol.io/registry/about
[mcp-server-schema]: https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json
[mcpb-readme]: https://github.com/modelcontextprotocol/mcpb/blob/main/README.md
[mcpb-manifest]: https://github.com/modelcontextprotocol/mcpb/blob/main/MANIFEST.md
[mcpb-cli]: https://github.com/modelcontextprotocol/mcpb/blob/main/CLI.md
[rmcp]: https://github.com/modelcontextprotocol/rust-sdk
[rfc8705]: https://www.rfc-editor.org/rfc/rfc8705.html
[rfc7662]: https://www.rfc-editor.org/rfc/rfc7662.html
[rfc8693]: https://www.rfc-editor.org/rfc/rfc8693
[rfc9728]: https://www.rfc-editor.org/rfc/rfc9728.html
[oauth21]: https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1-13#section-1.5
[wimse]: https://datatracker.ietf.org/doc/draft-ietf-wimse-workload-creds/
[txn-tokens]: https://datatracker.ietf.org/doc/draft-ietf-oauth-transaction-tokens/
[vault-jwt]: https://developer.hashicorp.com/vault/docs/auth/jwt
[oci-manifest]: https://github.com/opencontainers/image-spec/blob/v1.1.1/manifest.md
[oci-descriptor]: https://github.com/opencontainers/image-spec/blob/v1.1.1/descriptor.md
[oci-media-types]: https://github.com/opencontainers/image-spec/blob/v1.1.1/media-types.md
[oci-layout]: https://github.com/opencontainers/image-spec/blob/v1.1.1/image-layout.md
[oci-dist]: https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md
[oras]: https://oras.land/docs/
[oras-push]: https://oras.land/docs/how_to_guides/pushing_and_pulling
[oras-cp]: https://oras.land/docs/commands/oras_cp
[helm-oci]: https://helm.sh/docs/topics/registries/
[flux-oci]: https://fluxcd.io/flux/components/source/ocirepositories/
[ctrd-content]: https://github.com/containerd/containerd/blob/main/docs/content-flow.md
[ctrd-store]:
  https://github.com/containerd/containerd/blob/main/plugins/content/local/store.go
[ctrd-handlers]: https://github.com/containerd/containerd/blob/main/core/remotes/handlers.go
[ctrd-hosts]: https://github.com/containerd/containerd/blob/main/docs/hosts.md
[swarm-services]: https://docs.docker.com/engine/swarm/services/
[docker-service-create]: https://docs.docker.com/reference/cli/docker/service/create/
[docker-protect]: https://docs.docker.com/engine/security/protect-access/
[docker-ports]: https://docs.docker.com/engine/network/port-publishing/
[docker-network-create]: https://docs.docker.com/reference/cli/docker/network/create/
[dockerd]: https://docs.docker.com/reference/cli/dockerd/
[docker-mcp-gw]: https://github.com/docker/mcp-gateway
[docker-mcp-clientpool]:
  https://github.com/docker/mcp-gateway/blob/7e0f34277c91855ad861eeff536b4fdc77420593/pkg/gateway/clientpool.go
[docker-mcp-flags]:
  https://github.com/docker/mcp-gateway/blob/7e0f34277c91855ad861eeff536b4fdc77420593/docs/generator/reference/docker_mcp_gateway_run.yaml
[docker-mcp-blog]: https://www.docker.com/blog/mcp-toolkit-gateway-explained/
[toolhive-build]: https://docs.stacklok.com/toolhive/guides-cli/build-containers
[toolhive-run]: https://docs.stacklok.com/toolhive/guides-cli/run-mcp-servers
[toolhive-auth]: https://docs.stacklok.com/toolhive/guides-cli/auth
[toolhive-netiso]: https://docs.stacklok.com/toolhive/guides-cli/network-isolation
[agw]: https://agentgateway.dev/llms.txt
[agw-mcp-authn]:
  https://agentgateway.dev/docs/standalone/latest/documentation/mcp/mcp-authn.md
[agw-backend-authn]:
  https://agentgateway.dev/docs/standalone/latest/documentation/configuration/security/backend-authn/key.md
[agw-backend-tls]:
  https://agentgateway.dev/docs/standalone/latest/documentation/configuration/security/backend-tls.md
[agw-extauthz]:
  https://agentgateway.dev/docs/standalone/latest/configuration/security/external-authz/
[ms-mcp-gw]:
  https://github.com/microsoft/mcp-gateway/blob/cf9f5e9eff89276bc3fc42f0ae4d6109d70e5509/README.md
[ibm-cf]:
  https://github.com/IBM/mcp-context-forge/blob/7d34038ca1f2057bf62c3cb27eb90818296b4947/README.md
[aws-agentcore-in]:
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-inbound-auth.html
[aws-agentcore-src]:
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-target-http-runtime.html#gateway-target-http-runtime-source-validation
[a2a-spec]: https://github.com/a2aproject/A2A/blob/v1.0.1/docs/specification.md
[a2a-enterprise]: https://github.com/a2aproject/A2A/blob/v1.0.1/docs/topics/enterprise-ready.md
[ghostunnel]: https://github.com/ghostunnel/ghostunnel/blob/master/README.md
[openssh]: https://man.openbsd.org/ssh
[openssh-config]: https://man.openbsd.org/ssh_config
[opa-config]: https://www.openpolicyagent.org/docs/configuration
[opa-bundles]: https://www.openpolicyagent.org/docs/management-bundles
[opa-rest]: https://www.openpolicyagent.org/docs/rest-api
[opa-logs]: https://www.openpolicyagent.org/docs/management-decision-logs
[opa-envoy]: https://www.openpolicyagent.org/docs/envoy
[puppet-files]: https://help.puppet.com/core/current/Content/PuppetCore/file_serving.htm
[salt-files]: https://docs.saltproject.io/en/latest/ref/file_server/index.html
[npm-cache]: https://docs.npmjs.com/cli/v11/commands/npm-cache
[npm-provenance]: https://docs.npmjs.com/generating-provenance-statements
[cosign-blob]: https://docs.sigstore.dev/cosign/signing/signing_with_blobs/
[cosign-verify]: https://docs.sigstore.dev/cosign/verifying/verify/
[redhat-referrers]:
  https://www.redhat.com/en/blog/announcing-open-container-initiativereferrers-api-quayio-step-towards-enhanced-security-and-compliance
[reapi]:
  https://github.com/bazelbuild/remote-apis/blob/main/build/bazel/remote/execution/v2/remote_execution.proto
[spegel]: https://github.com/spegel-org/spegel
[etcd]: https://etcd.io/docs/v3.5/learning/api_guarantees/
[kleppmann]: https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html
[kernel-tcp]: https://docs.kernel.org/networking/ip-sysctl.html
[leases-paper]: https://doi.org/10.1145/74850.74870
[leases-summary]:
  https://blog.acolyer.org/2014/10/31/leases-an-efficient-fault-tolerant-mechanism-for-distributed-file-cache-consistency/
[nist-pep]: https://csrc.nist.gov/glossary/term/policy_enforcement_point
[nist-pdp]: https://csrc.nist.gov/glossary/term/policy_decision_point
[zanzibar]: https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/
