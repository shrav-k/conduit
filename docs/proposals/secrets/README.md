# Secrets: a secret type in the core package

Status: **proposed, not ruled.** It follows [ruling 38](../ruling-38/README.md), point 1, which
set the direction and asked for the secret type to be explored. Nothing here is accepted, and
nothing here is part of the specification. Each decision below is a recommendation that waits
for the owner. Until the owner rules, the specification says nothing of secrets, and row GP3 of
the [conversion list](../docs-audit/GAPS.md) stays unconverted.

Written on 2026-11-06 against the specification as it then stood. Promise IDs are those of
[spec/](../../../spec/README.md) on that day. Terms are those of
[spec/definitions.md](../../../spec/definitions.md); a new term this proposal needs is marked
as proposed where it first appears.

## Contents

- [The problem](#the-problem)
- [What the owner has already ruled](#what-the-owner-has-already-ruled)
- [What existing systems do](#what-existing-systems-do)
- [The proposal in one page](#the-proposal-in-one-page)
- [Decisions](#decisions): [S1](#s1-what-a-secret-is), [S2](#s2-how-a-document-marks-a-secret),
  [S3](#s3-how-the-value-reaches-the-implementation-that-needs-it),
  [S4](#s4-who-may-read-a-secret), [S5](#s5-where-the-value-is-stored),
  [S6](#s6-rotation-and-revocation), [S7](#s7-authentication),
  [S8](#s8-existing-servers-that-read-credentials-from-the-environment),
  [S9](#s9-what-a-value-is), [S10](#s10-what-the-guarantee-covers)
- [Promises that would follow](#promises-that-would-follow)
- [Existing promises and documents that would
  change](#existing-promises-and-documents-that-would-change)
- [Left out of the first version](#left-out-of-the-first-version)
- [For the owner to weigh](#for-the-owner-to-weigh)

## The problem

Conduit's purpose names the case directly. "A login, checkout or memory is owned by a Pool and
used by agents from any vendor, each under its own limits, with the credential never handed
over" ([purpose](../../AUDIT.md#purpose)). The older design stated the rule that has to become a
promise: "Secrets never appear in configuration, records or events, and a credential is never
handed to a caller" (row GP3 of the [conversion list](../docs-audit/GAPS.md)).

The specification cannot state that rule today, for three reasons.

1. Nothing marks a value as secret. A value inside `config` or `arguments` is either a
   reference, where a reference schema governs its place, or data (OBJ-021). Data is stored as
   written, shown by inspection (LIF-013), sent to implementations, read by graphs (GRA-020),
   and kept in records where the type's `records` says so (REC-002).
2. Nothing says how a credential reaches the implementation that needs it, such as the
   implementation of a Resource that calls an outside service with a token.
3. Nothing stops an operation from returning a credential to the actor that asked for it.

## What the owner has already ruled

Ruling 38, point 1, put two options:

- **A.** The schema keyword `writeOnly: true`, with the engine keeping a value so marked out of
  inspection results, records, diagnostics and events.
- **B.** A marked property may hold only a reference to a secret Resource, and a literal is
  rejected. This needs a secret type in the core package.

The coordinator recommended A. The owner ruled B: "B I think we need to explore a secret type as
this will be extremely important for many areas including access and authentication."

So three things are settled and are not reopened here:

- A secret is a thing of its own, referred to and never written as a literal.
- A literal where a secret is expected is rejected.
- There is a secret type, and it is expected to serve access and authentication.

Everything else is open, and is set out below. The owner's standing direction applies to each
decision: adopt an existing paradigm for anything that is not what makes Conduit different, and
invent nothing where a standard exists
([existing standards first](../../../spec/README.md#existing-standards-first)). The first
user is a single developer on one machine; teams come later, and no rule may assume one owner.
Enforcement beyond what passes through the engine is a future, optional part.

## What existing systems do

Each row was read from the system's own documentation on 2026-11-06. The column "Lesson for
Conduit" is this proposal's reading, not the source's.

| System | How a secret is referred to | How the value reaches the user of it | What is kept out, and the known gaps | Rotation and revocation | Lesson for Conduit | Source |
| --- | --- | --- | --- | --- | --- | --- |
| Kubernetes Secret | A Secret object, named in a Pod by `env[].valueFrom.secretKeyRef` or as a volume | An environment variable, or a file in a mounted volume | Values are base64, which "does NOT provide any useful level of confidentiality", and are stored unencrypted in etcd by default. "Anyone who is authorized to create a Pod in a namespace can use that access to read any Secret in that namespace." | A mounted volume is updated "using an eventually-consistent approach"; a `subPath` mount is not. An immutable Secret can only be deleted and recreated. | One kind of object for every secret, and reference by name. Whoever can make a workload that reads a secret can read it. | [Secrets](https://kubernetes.io/docs/concepts/configuration/secret/), [projected volumes](https://kubernetes.io/docs/concepts/storage/projected-volumes/) |
| Secrets Store CSI Driver | A `SecretProviderClass` naming an outside store (Vault, AWS, Azure, GCP and others) | Files in a CSI volume, read from the store when the Pod starts; copying into a Kubernetes Secret is optional | The value need not be stored in the cluster at all | Rotation is alpha and polls, by default every 2 minutes. The application "will need to watch for the file change"; "The pod needs to be restarted to get the latest secret as environment variable." | One reference form, many stores behind it as providers. | [Driver](https://secrets-store-csi-driver.sigs.k8s.io/), [rotation](https://secrets-store-csi-driver.sigs.k8s.io/topics/secret-auto-rotation), [sync](https://secrets-store-csi-driver.sigs.k8s.io/topics/sync-as-kubernetes-secret) |
| External Secrets Operator | An `ExternalSecret` names a `SecretStore` and a `remoteRef` (key, version, property) | Copies the value into an ordinary Kubernetes Secret | Every Kubernetes caveat applies to the copy. The operator can "read and write to all secrets across all namespaces". | Read again every `refreshInterval`; zero means fetch once | A store is configured once and many secrets name it. Copying into the cluster brings the cluster's gaps with it. | [Overview](https://external-secrets.io/latest/), [ExternalSecret](https://external-secrets.io/latest/api/externalsecret/), [security](https://external-secrets.io/latest/guides/security-best-practices/) |
| Docker Compose | A top-level `secrets:` element, with a `file` source or an `environment` source on the host | A file at `/run/secrets/<secret_name>` | "Services can only access secrets when explicitly granted by a `secrets` attribute." Passing secrets as environment variables risks "unintentional information exposure". | Not covered | Two simple sources, a file and an environment variable, and an explicit grant per service. | [Secrets element](https://docs.docker.com/reference/compose-file/secrets/), [how-to](https://docs.docker.com/compose/how-tos/use-secrets/) |
| Terraform | `sensitive = true` on a variable or output; ephemeral resources and values; write-only arguments ending `_wo` | Through the provider | `sensitive` only redacts the CLI's log: values are still stored "in both state and plan files". Ephemeral values are omitted "from state and plan files entirely", and "Terraform restricts where you can reference ephemeral values". | A write-only argument is changed by incrementing its `_wo_version`; Terraform cannot see the value change | Marking for display is not enough; what matters is that the value is never stored. Where a secret value may flow must be restricted. | [Outputs](https://developer.hashicorp.com/terraform/language/values/outputs), [sensitive data](https://developer.hashicorp.com/terraform/language/manage-sensitive-data), [ephemeral](https://developer.hashicorp.com/terraform/language/resources/ephemeral), [write-only](https://developer.hashicorp.com/terraform/language/resources/ephemeral/write-only) |
| HashiCorp Vault | A path, read by a client that presents its own token | An API call at the moment of use | Response wrapping puts a response behind a single-use token, so "Information stored in logs or captured along the way do not directly see the sensitive information." | Dynamic secrets have leases; "When a lease is revoked, it invalidates that secret immediately." | The workload fetches the value itself, under its own identity and policy, when it needs it. Real revocation needs the issuer's cooperation. | [Leases](https://developer.hashicorp.com/vault/docs/concepts/lease), [response wrapping](https://developer.hashicorp.com/vault/docs/concepts/response-wrapping), [database engine](https://developer.hashicorp.com/vault/docs/secrets/databases) |
| AWS Secrets Manager | ARN or name; versions by staging label `AWSCURRENT`, `AWSPREVIOUS`, `AWSPENDING` | API call; "Secrets Manager returns that version [`AWSCURRENT`] by default" | Access by IAM policy; values encrypted with a KMS key; rotation logged to CloudTrail | Rotation makes a new version and moves the label | Refer to the current version by default; the store keeps versions. | [What's in a secret](https://docs.aws.amazon.com/secretsmanager/latest/userguide/whats-in-a-secret.html), [rotation](https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets.html) |
| Google Secret Manager | `projects/…/secrets/SECRET_ID/versions/VERSION_ID`, or `latest` | API call | Reading needs the role `secretmanager.secretAccessor` | Versions can be disabled or destroyed | The same: a name, and a version or "latest". | [Overview](https://docs.cloud.google.com/secret-manager/docs/overview), [access a version](https://docs.cloud.google.com/secret-manager/docs/access-secret-version) |
| Azure Key Vault | `https://<vault>.vault.azure.net/secrets/<name>/<version>`; without the version, the latest | API call | Identifiers must not carry sensitive information | New versions; the base identifier reads the latest | The same. | [Keys, secrets and certificates](https://learn.microsoft.com/en-us/azure/key-vault/general/about-keys-secrets-certificates) |
| SPIFFE and SPIRE | No shared secret: a workload is identified by a SPIFFE ID and proves it with an SVID (X.509 or JWT) | The Workload API, which needs no "authentication token when calling the API"; the endpoint identifies the caller out of band | Keys are "short lived, rotated frequently and automatically" | Updates are streamed to the workload | Identity of a workload is best not a stored secret at all. | [Overview](https://spiffe.io/docs/latest/spiffe-about/overview/), [concepts](https://spiffe.io/docs/latest/spiffe-about/spiffe-concepts/), [Workload API](https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_API.md) |
| OAuth 2.0 Token Exchange, RFC 8693 | A `subject_token` for whom the request is made, an `actor_token` for who acts | A new token from a token service, for an `audience` | Delegation ("A is an agent for B") is recorded in the `act` claim | Tokens are short-lived and issued per use | The standard way to act for someone at an outside service without holding their credential. Later, not v0. | [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) |
| Model Context Protocol, revision 2026-07-28 | Authorization is optional and defined for HTTP: an OAuth 2.1 bearer token in the `Authorization` header of every request. Over stdio, implementations "SHOULD NOT follow this specification, and instead retrieve credentials from the environment." | The client presents the token; the server validates it was issued for itself | "MCP servers MUST NOT accept or transit any other tokens." "The MCP server MUST NOT pass through the token it received from the MCP client." Token passthrough is "explicitly forbidden". | Authorization servers "SHOULD issue short-lived access tokens" | The engine is an MCP server to its consumers and must not pass their credentials on, which IFC-030 already states. An existing stdio server takes its credential from its environment. | [Authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization), [security considerations](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations), [best practices](https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices) |
| JSON Schema 2020-12, `writeOnly` | An annotation: "the value is never present when the instance is retrieved from the owning authority" | — | An annotation only; it is in section 9, the annotation keywords, and asserts nothing | — | It says a value is not read back. It cannot reject a literal, which is what ruling 38 asks. | [Validation, section 9.4](https://json-schema.org/draft/2020-12/json-schema-validation#section-9.4) |
| OpenAPI 3.1, `format: password` | `string` with `format: password`: "A hint to UIs to obscure input." | — | A hint only | — | The same: a display hint, not a rule. | [Data types](https://spec.openapis.org/oas/v3.1.0#data-types) |
| GitHub Actions secrets | `${{ secrets.NAME }}` | Into the job's environment | Values are "redacted from logs", but with no promise for transformed values; "secrets are not passed to the runner when a workflow is triggered from a forked repository" | — | Masking printed values is a useful extra and never a guarantee. | [Using secrets](https://docs.github.com/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions) |

What these systems agree on, read as one:

- **A secret is referred to by name, and the store keeps the value.** Every system above writes
  a reference in configuration, never the value, except Terraform's `sensitive`, whose own
  documentation says the value still reaches state.
- **One reference form, many stores.** Kubernetes has one Secret kind with the CSI driver and
  the External Secrets Operator in front of outside stores. Compose has one `secrets:` element
  with a `file` source and an `environment` source.
- **Two ways to deliver.** Either the platform puts the value into the workload's environment
  or a file (Kubernetes, Compose, CSI), or the workload asks the store for it under its own
  identity when it needs it (Vault and the cloud managers). Only the second sees a new value
  without a restart.
- **Who may read is an explicit grant.** Compose grants per service; Vault and the cloud
  managers by policy on the reader's identity. Kubernetes shows the gap: whoever can create a
  workload can read every secret it may mount.
- **Revoking access stops later reads and cannot recall a copy.** Only an issuer that can
  invalidate the credential itself, as Vault does with dynamic secrets, revokes the copy too.
- **Workload identity is a different problem.** SPIFFE removes the shared secret from how a
  workload proves who it is; the Model Context Protocol keeps a client's token away from the
  services behind the server.

## The proposal in one page

This is what the recommendations below add up to, if all of them were accepted.

- The core package exports a Type contract, proposed name `secret.v1`, of the family
  Resource. A secret is a Resource of that type, owned by a Pool like any Resource. Its
  configuration says where the value is kept, never the value.
- The core package exports a reference schema, proposed name `secret-reference.v1`. A place
  that it governs holds a reference to a Resource of the type `secret.v1`, and nothing else: a
  literal, and even a bare name, is rejected. This is the mark that ruling 38 asked for.
- The value is read by one operation, proposed name `reveal-secret.v1`, in one capability,
  proposed name `secret-reveal.v1`, which `secret.v1` offers. Its one output is marked
  `secret: true`, a new mark of an Operation contract's output, beside `handle: true`.
- The right to read a secret is an open Conduit to it that lists `reveal-secret.v1`, as every
  permission is (ACC-001, ACC-003). Such a Conduit is never opened for an actor, the root
  Interface included. Only a Resource reads a secret, and only its implementation sees the
  value, by an onward request made while it serves an invocation.
- A secret output never appears in a record, an inspection result, an observation, a
  diagnostic, a graph, or a retained result, and is delivered only to the instance that sent
  the request. An implementation that receives one never passes it on to an actor or into the
  engine again.
- The value stays in an existing store. The engine's own implementation of `reveal-secret.v1`
  reads a file or an environment variable, as Docker Compose does. The operating system's
  keychain and the secret managers are ordinary packages. No operation writes a secret.
- The store is read at each request, so a value changed in the store is seen at the next read,
  with nothing set up again and no revision advanced. Closing the Conduit stops later reads.
- How an instance proves who it is to the engine stays outside the secret type (DL13).

A short illustration, not checked by any tool. A secret in a Pool's entry, a Resource that
reads it, and an instance that names it:

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: github-token
spec:
  type: github.com/shrav-k/conduit/core#secret.v1
  config:
    environment: GITHUB_TOKEN       # where the value is kept, read by the engine's implementation
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: reveal
spec:
  capability: github.com/shrav-k/conduit/core#secret-reveal.v1
  operations: [github.com/shrav-k/conduit/core#reveal-secret.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: issue-tracker
spec:
  type: "#issues.v1"
  inputs:
    token:
      schema: {$ref: github.com/shrav-k/conduit/core#secret-reference.v1}
      to: [/conduits/token/target]
  conduits:
    token:
      from: reveal                  # opened at setup; its consumer is the Resource
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
from: issue-tracker
owner: tools
arguments:
  token: {kind: Pool, name: credentials, entry: github}
  # token: ghp_0123...   rejected: a literal where a secret is marked
  # token: hunter        rejected: a bare name is not accepted in a secret's place
```

An Agent that is the consumer of a Conduit to `issue-tracker` calls its operations. The
Resource's implementation reveals the token through its own Conduit while it serves the Agent's
invocation, calls the outside service, and returns the issues. The Agent never holds the token.

## Decisions

Each decision gives the options, what each costs, and a recommendation with its reason. "Fits"
says how the recommendation sits with the promises as they stand.

### S1. What a secret is

**The question.** What kind of thing a secret is in Conduit, and where its definition lives.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | A Resource of one secret type, owned by a Pool. Access to it is a Conduit like any other. | A new Type contract, capability and operation in the core package. |
| B | A sixth family, `Secret`. | A new family touches the five-family rule (OBJ-020), every reference schema, inspection, lifecycle and access. Nothing that a secret needs is missing from a Resource. |
| C | A value that `access` or the engine stores outside any object, as `access` stores policies (ACC-031). | No ownership, no placement, no lifecycle, no Conduit: access to it would need a second mechanism, against "every permission is an open Conduit". |
| D | Any Type contract in any package that carries a new core mark, such as `secret: true` on the type. | Many secret types. The engine must trust every package's claim, and the rules of S2 to S4 must be stated against a mark rather than one type. |

**Sub-question: where the type lives.**

| Option | What it is | What it costs |
| --- | --- | --- |
| **A1** | The type, its capability and its operation in the core package; the stores behind it are implementations, in the core package (S5) or in ordinary packages. | Three exports added to the core package. |
| A2 | Only the reference schema and the output mark in the core; the type in an ordinary package. | The reference schema must check a Resource's type, and the engine can only check a type it knows. A schema in the core that names a type outside it would make the core depend on a package. |

**Recommendation: A with A1.** A secret is a Resource of the core type `secret.v1`, owned by a
Pool. One type with many stores behind it is the paradigm of Kubernetes, whose one Secret kind
fronts outside stores through the CSI driver and the External Secrets Operator, and of Compose,
whose one `secrets:` element has two sources. A Resource already has everything a secret needs:
an owner, so a Pool groups a developer's credentials and a bound that names the Pool covers
them (ACC-029); a placement, which matters because where the value is kept is tied to a location
(PLC-020); a lifecycle; inspection; and Conduits for access, which is what the owner asked the
type to serve. The type must be in the core package because the language checks it: OBJ-021
makes a value a reference only where one of the core package's reference schemas governs it,
and S2 needs that schema to name a type the engine knows.

**Fits.** POOL-001 (one owner, a Pool), ACC-028 (ownership permits nothing, so the Pool that
owns a secret cannot read it), CON-021 (a Conduit's capability must be offered by its target's
type), TPL-003 (a template selects the implementation of each operation). No family is added.

### S2. How a document marks a secret

**The question.** How a schema says "this place holds a secret", what the language then
rejects, and how that fits the [schema profile](../../../spec/schema-profile.md) and the
reference schemas that exist.

| Option | What it is | What it costs |
| --- | --- | --- |
| A | `writeOnly: true` | Ruled out by ruling 38. It is an annotation and rejects nothing. |
| **B** | A new reference schema of the core package, `secret-reference.v1`, used by `$ref` like the others. | One more Schema contract; one rule that no schema can express, checked by the engine. |
| C | A new format, such as `format: secret-reference` | A format in the profile is a grammar that is asserted (SCH-004). A reference is a map, not a string, and has no grammar. This would invent a format. |
| D | A new keyword, such as `secret: true`, inside a schema | A keyword outside JSON Schema 2020-12. SCH-002 rejects any keyword outside the profile, and the profile invents none. |

**What `secret-reference.v1` would accept.** The forms of a reference to a Resource that
`resource-reference.v1` accepts, less one:

- `{kind: Resource, name: <name>}`;
- the reference to a Pool's entry, `{kind: Pool, name: <pool>, entry: <entry>}` (POOL-014);
- the entry reference `{entry: <name>}`, inside a Pool's template (POOL-005);
- the resolved form `{kind: Resource, id: <ID>}`, which only the engine writes (OBJ-009).

It would **not** accept a bare name, which `resource-reference.v1` accepts. A bare name is a
string. A secret that happens to be written in lowercase letters, digits and hyphens, such as a
weak password, would pass the schema as a name; resolving would then find no object of that
name and reject the document with a diagnostic that names the reference (OBJ-009), which would
print the secret. Without the bare name, every string in that place fails the schema, and a
literal is rejected by the schema itself.

**The rule no schema can express.** The Resource that the reference resolves to has the type
`secret.v1`. A reference to any other Resource makes the document rejected, as OBJ-007 rejects
a reference to a template where an instance is expected.

**A second rule, on diagnostics.** A diagnostic about a value in a place that
`secret-reference.v1` governs names the place and the rule it breaks, and never quotes the value.
A JSON Schema validator commonly prints the failing instance; this rule forbids that here. The
result of `register.v1` is returned to its caller and its content is retained for retrieval
(OPS-016), so a quoted literal would be kept.

**Where the mark may be used.** Wherever a schema is written: a type's `schema` for `config`, a
template's input, an Operation contract's input, and a Requirement contract's input. In each
case it holds a reference, so the existing rules on references apply unchanged: it is resolved
when the instance is made (OBJ-009), never redirected (OBJ-010), stored and shown in the
resolved form (OBJ-021), and a request that an actor makes on its own behalf names it by
reference, never by ID (LIF-031).

**Recommendation: B, without the bare name, with the rule on diagnostics.** It uses only what
exists: `$ref` to a Schema contract of the core package, the mechanism by which every other
reference is marked. The profile does not change. It is the only option that rejects a literal,
which the ruling requires. Because a marked place only ever holds a reference, configuration,
records, inspection, observations and graphs hold the reference and never the value, with no
new rule for any of them.

**Fits.** OBJ-021 gains one schema in its list. SCH-001 and SCH-002 are unchanged. INS-006 is
unchanged; its rule that a bare name is accepted where the schema fixes the kind does not apply,
because this schema accepts no bare name. The definition of reference schema gains one name.

**What it cannot do.** It marks only what an author marks. A token written into a place that a
plain `{type: string}` governs is data, and no rule can tell it from any other string. See
[S10](#s10-what-the-guarantee-covers).

### S3. How the value reaches the implementation that needs it

**The question.** How the value gets to the implementation that uses it, without appearing in
configuration, records, events, inspection results, diagnostics or operation results; and what
the engine itself may see.

How an implementation learns anything today: it receives the `arguments` of each request and
Conduit's keys in `_meta` (IFC-006). It is not sent the instance's configuration; a graph reads
configuration with a value reference (GRA-020) and passes it as a step's arguments. An instance
that is not an actor makes a request only to serve an invocation (IFC-021), and an onward
request is permitted only under the instance's own open Conduit (ACC-016).

| Option | What it is | What it costs |
| --- | --- | --- |
| A | **Substitution at dispatch.** Where a request's arguments hold the resolved form of a secret reference, the engine puts the value in its place before it sends the request. As Kubernetes's `secretKeyRef` and Compose do for a container. | The value is in the arguments of ordinary requests, so every rule on arguments needs an exception: records of inputs (REC-002), the digest of contents that a purge keeps (REC-004) and that a repeat is compared by (OPS-009), the arguments of restrictions (IFC-018). It breaks OBJ-021, under which the implementation receives the resolved form. Who may read is decided by who may name the reference, not by a Conduit. |
| B | **A key in `_meta`.** Every request to an instance carries the values of the secrets its configuration names, in a new key. | The same exposure as A on every request, whether or not the operation needs the value. Who may read is again not a Conduit. |
| **C** | **The implementation asks.** The instance has a declared Conduit to the secret Resource that lists `reveal-secret.v1`. Its implementation, while serving an invocation, makes an onward request for it, and receives the value as the operation's output. As Vault and the cloud secret managers do: the workload reads the store under its own identity, when it needs it. | Each read is a request, with an access decision and a record. The implementation must be written to ask. An existing server that is not written for this specification cannot ask (see S8). |
| D | **A file or an environment variable at process start.** As Compose's `/run/secrets` and Kubernetes's volumes and environment. | Process start is the engine's own choice today (DL12), several instances may share a process, and a new value needs a new process. Only useful where the implementation cannot ask; taken up in S8. |
| E | **A handle.** `reveal-secret.v1` returns an opaque handle, and the implementation exchanges it at the store, as Vault's response wrapping does. | The implementation must then reach the store itself, with a credential of its own, which only moves the problem. |

**What the engine may see.** Under C the value passes through the engine once per read: the
engine is the server the implementation calls (IFC-012) and the client of the store's
implementation. The engine already sees every argument and result, and the first user runs the
engine on their own machine. The recommendation is that the engine may hold the value in memory
to deliver it, and keeps nothing of it: no record, no retained content, no diagnostic, no
observation and no inspection result holds it (S4). Delivery that the engine cannot read, by
wrapping the value for the reader alone, is left out of the first version.

**Recommendation: C.** The value reaches only the implementation that asks for it, at the
moment it asks, through a Conduit that can be inspected, narrowed by a filter or a policy, and
closed. That makes the right to read a secret "an open Conduit like any other access", which
is what the owner asked the type to serve. Every read is recorded with its caller and its
sending instance (REC-001, REC-002), so the record shows which Resource read which secret on
whose behalf, as CloudTrail and Vault's audit do, without the value. Reading the store at each
request gives rotation for free (S6). The existing machinery does the rest: the declared
Conduit is opened at setup (ACC-005, CON-004), it is closed when its consumer's shutdown is
confirmed (CON-008), and the restrictions of the served invocation apply to the read (ACC-017),
so a policy can stop a Resource from reading a secret on behalf of a particular Agent.

**Fits.** IFC-021, ACC-016, ACC-017 and IFC-020 unchanged. OBJ-021 unchanged: the
implementation receives the resolved form, and the value only as the output of the read.

### S4. Who may read a secret

**The question.** Who may read; what the operation is; and what "a credential is never handed
to a caller" means for an operation's outputs.

**The right.** The consumer of an open Conduit to the secret Resource that lists
`reveal-secret.v1`, under every restriction that applies to the request (ACC-001, ACC-008). The
capability `secret-reveal.v1` lists that one operation. The operation takes no input beyond
what a request carries, because the Conduit names the target, and has one output, `value`.

**The mark.** An output of an Operation contract may hold `secret: true` (proposed). It is a
mark of the same kind as `handle: true` (CTR-002): it says what the output is, and the engine
treats it accordingly. A secret output is never recorded, whatever the type's `records` says;
its content is never retained for retrieval; no value reference in a graph may read it; it
appears in no inspection result, observation or diagnostic; and it is delivered only to the
instance that sent the request. No input carries the mark in the first version, because no
operation of the first version takes a secret value as an input (S5).

**Who may be the consumer.**

| Option | Who may read | What it costs |
| --- | --- | --- |
| A | Any consumer whose Conduit lists the operation, actors included. As Kubernetes, where anyone with `get` on a Secret reads it. | "A credential is never handed to a caller" becomes untrue as soon as an actor is given such a Conduit. An Agent's model would hold the value in its context. |
| **B** | Only a consumer that is not an actor: a Resource. A Conduit whose consumer is an Interface or an Agent and that lists an operation with a secret output is never opened, the root Interface's included, and a template that declares one is rejected. | The developer cannot read a secret back through Conduit; they read it in the store, where they put it. An Agent that needs a credential for itself has to get it outside Conduit (S7). |
| C | As B, except that the root Interface may read. | One exception to a rule that is otherwise plain. The root Interface is a person or a program acting for them, and its results are as easily logged as any other. |
| D | As B, and also: only the root Interface may open a Conduit that lists such an operation. | Strong and simple for one developer. A team could never delegate the right, which the purpose says must stay possible. |

**What "never handed to a caller" means.** The caller of a read is the actor whose invocation
the Resource is serving (IFC-020). Under B the value is delivered to the sending instance, the
Resource, and never to the caller. What the Resource's implementation does with it then is its
own act, which the engine cannot see: the proposal states that as an obligation of the
implementation, at the Implementation level of conformance, in the way that the Model Context
Protocol forbids a server to pass a client's token through and IFC-031 forbids an implementation
to forward Conduit's keys. The implementation uses the value toward the outside service it is
for, and never puts it in a result, a diagnostic, the arguments or `_meta` of a request to the
engine, or anything it sends an actor.

The comparison with a handle is close. A handle is an opaque string that the engine passes back
unchanged, and "holding one permits nothing" (IFC-011). A reference to a secret is the same: an
actor may hold the resolved form `{kind: Resource, id}` of a secret, see it in inspection, and
pass it as an argument, and that permits nothing. Only a Conduit permits a read.

**Recommendation: B, with the generic output mark.** It is the one option under which the
sentence of GP3 is a promise that can be checked: no actor is ever the consumer of a Conduit
that reads a secret. It keeps delegation possible: a Conduit to `access` whose bound names
`reveal-secret.v1` and a Pool of secrets lets its consumer open reading Conduits for Resources,
and one whose bound does not name the operation does not (ACC-029, ACC-030, which already
require a bound to name every operation of a Conduit it opens). The mark is generic, rather than
fixed to `reveal-secret.v1`, so that a package can later define an operation that issues a
short-lived credential to a Resource, as Vault's dynamic secrets do, and have the same rules
apply.

**What this cannot stop.** An actor that may both register a Resource with an implementation
of its own choosing and open a reading Conduit for it can have that implementation hand the
value back. That needs `register.v1` through `management` and a bound that names
`reveal-secret.v1`. The proposal states it as the limit of the guarantee (S10) and does not
reserve the right to the root Interface (option D); the owner may prefer D for the first
version.

**Fits.** ACC-032 is the model for the refusal: an operation that some consumers may not have
listed, refused at the opening of the Conduit. CTR-002 gains the mark. REC-002 gains an
exception. OPS-016 and IFC-024 gain the case of a result that has a secret output. GRA-020 gains
a rejection.

### S5. Where the value is stored

**The question.** What keeps the value, and what is left to each engine.

| Option | What keeps the value | What it costs |
| --- | --- | --- |
| A | **The engine**, in a store of its own, with an operation to write a value. | The engine becomes a secret store, with encryption at rest, backup and key management to specify or leave to each engine. The value enters through a request's arguments, so records, digests and repeats need exceptions (S3, option A). It invents what existing stores already do. |
| **B** | **An existing store**, read by an implementation of `reveal-secret.v1`. The secret Resource's configuration says where the value is (a path, a variable's name, a keychain item, a secret manager's name and version), never the value. The person puts the value in the store with the store's own tools. | Nothing in Conduit writes a secret. Each store needs an implementation. |
| C | Both. | The costs of A, for a gain the first user does not need. |

**Which stores the engine reads itself.** The core package's one implementation is the engine
(MAN-012), so whatever the core package implements, every engine implements.

| Option | What `engine` reads | What it costs |
| --- | --- | --- |
| B0 | Nothing. Every store is an ordinary package. | A developer cannot use a secret without first installing a package. |
| **B1** | A file, named by an absolute path, and an environment variable of the engine's own process, named by its name. As Compose's two sources. | Two small sources. A file is protected only by the file system's permissions, and an environment variable by the engine's process. |
| B2 | B1 and the operating system's keychain. | Platform-specific code in every engine. Whether a keychain can be read unattended, from a daemon or a rebuilt binary, is still open (experiment E10 of the [tracker](../../AUDIT.md)). |

**Recommendation: B, with B1.** Keep the value in an existing store and read it there. This is
what the CSI driver, the External Secrets Operator, Vault and the cloud managers do, and what
the older design wanted ("custody through ... implementation bundle", the OS credential store by
default; [older text](../../protocol/POOLS.md), not binding). The engine reads a file or an
environment variable, as Compose does, so a single developer can start with no package. The
keychain and the secret managers are ordinary packages that implement `reveal-secret.v1`; the
template of a secret selects one under `implementation` (TPL-003). The path and the variable's
name are values tied to a location (PLC-020): they are read only at the secret Resource's
placement.

Three further rules follow, each from the older design or the sources:

- An implementation of `reveal-secret.v1` takes the value from the store its configuration
  locates, never from the configuration itself. Otherwise a store that "reads the value from
  `config`" would bring back the literal that ruling 38 rejects.
- Where the store cannot be read (the file is missing, the keychain is locked, the manager
  refuses), the read has the outcome failed and no value, with no fallback to another source
  ("without silent plaintext fallback", [older text](../../protocol/POOLS.md)).
- Nothing in the first version writes a secret's value. The developer writes it to the file,
  the shell profile or the keychain. A Resource that writes values (a password manager) can come
  later in a package, with the mark on inputs that it would need.

**What is implementation-defined.** A new item of the
[implementation-defined list](../../../spec/implementation-defined.yaml), DL16 in the drafts
below: how `engine` reads a file and an environment variable (the encoding, a trailing newline,
the largest value), and whether it reads them with its own process's rights or another's.

**Fits.** MAN-012 (the engine implements the core package), TPL-003, PLC-020, CTR-004 (the type
may state no `schema`, and each store's template narrows `config` to its own locator). The
configuration of `secret.v1` is configuration like any other: it is inspected, recorded and
revisioned, and holds nothing secret.

### S6. Rotation and revocation

**The question.** What changing a secret does to the instances that use it, and what closing
the Conduit does.

**Rotation.**

| Option | What a change of value does | What it costs |
| --- | --- | --- |
| **A** | Nothing in Conduit. The value is changed in the store, and the next read returns the new value. No revision advances and nothing is set up again. As the cloud managers' "latest" and `AWSCURRENT`, Vault, and the CSI driver's rotation. | An implementation that kept the old value keeps using it until it reads again. The engine cannot see the change. |
| B | A control on the secret with a revision, such as `rotate.v1`, as Terraform's `_wo_version`. | Only meaningful where the engine stores the value (S5, option A). |
| C | The instances that use the secret are set up again. | An instance is set up once; setting it up again means retiring it and making another (LIF-006). It would make rotation an outage. |

**Recommendation: A.** Rotation happens in the store, where it already happens. The secret's
locator should name the current version (a file, a variable, a keychain item, `latest`,
`AWSCURRENT`), so that rotation needs nothing in Conduit. A locator that pins a version is
changed only by making a new secret Resource; the first version gives `secret.v1` no control to
change its locator, and a consumer keeps the resolved form it was made with (OBJ-010). The
proposal recommends, as guidance to authors of implementations and not as a promise, reading at
each use or again after the outside service refuses a credential, rather than once at setup.

**Revocation.**

- **Closing the Conduit** from a Resource to a secret ends its right to read: every later read
  reports refused (ACC-011, ACC-036). It does not recall a value already read. No system above
  recalls a copy, except an issuer that can invalidate the credential, as Vault does by
  revoking a lease. The honest statement is: closing the Conduit stops later reads; to revoke
  the credential, rotate it in the store.
- **Retiring the secret** closes every Conduit to it at once (CON-008), so no later read is
  permitted. Its records stay (REC-003).
- **Retiring the reader** leaves its Conduit to the secret open until its shutdown is confirmed,
  for the requests of its own retirement only (ACC-039), then closes it (CON-008).
- **A read in progress when the Conduit is closed** runs to its end, and its content is
  withheld (ACC-034, OPS-017): the value is not delivered.

**Fits.** No promise changes. REV-003 is unchanged: a value in a store is not configuration.
OBS-001 is unchanged.

### S7. Authentication

**The question.** The owner named authentication. There are three credentials in play, and they
should not be confused.

1. **The credential an instance presents to the engine**, to show which instance it is
   (IFC-022, DL13). The engine never passes it on (IFC-030), and a request from which no
   instance can be established is refused (IFC-032), or answered with HTTP 401 where the
   protocol's own authorization is used.
2. **The credentials an implementation uses toward outside services**: a token for a code
   host, a key for a model provider, a password for a database.
3. **An Agent's own subscription or login**, with which its model runs.

| Option | What the secret type covers | What it costs |
| --- | --- | --- |
| A | All three. The engine issues each instance a credential and keeps it as a secret Resource. | Circular: to read a secret, an instance must first be established by the credential that the secret would hold. The engine would store secrets (S5, option A). |
| **B** | The second only. The first stays the engine's own business, left to each engine by DL13. The third stays the Agent's own. | The engine's own identification of consumers is not a Conduit. An Agent's provider login is outside Conduit. |
| C | The second and the third. | An Agent's implementation would read its own credential; but the Agent is an actor, and its model could print what its process holds. |

**Recommendation: B.**

- The identity of an instance is not a stored secret but something the engine establishes for
  each request. That is SPIFFE's lesson: a workload proves who it is through an endpoint that
  knows it, with no secret to bootstrap. Over HTTP the protocol's own bearer tokens apply; over
  stdio the engine knows the process it started. DL13 stays as it is. A later version could
  adopt workload identity, with the engine issuing short-lived credentials per instance.
- Outside credentials are exactly what the secret type is for, read as S3 describes.
- A shared login, the purpose's first example, is a Resource whose implementation reads the
  credential and calls the outside service on the caller's behalf, exposing operations and never
  the credential. Its type belongs in an ordinary package, built on `secret.v1`. Refreshing a
  login, OAuth flows and token exchange under RFC 8693 are left out of the first version; RFC
  8693's delegation, with an `act` claim naming the agent, is the standard to adopt when they
  come in, and Conduit already records the caller and the Conduit of every request for it.
- An Agent's own subscription stays its own, as the purpose says: "An agent's subscription and
  context are its own." How a native Agent is launched with its own login is experiment E1 of
  the tracker, not this proposal.

**Fits.** IFC-022, IFC-030, IFC-032 and DL13 unchanged. The new obligation on an implementation
(S4) is the counterpart of the protocol's rule against token passthrough: the engine does not
pass a consumer's credential on, and an implementation does not pass a revealed secret on.

### S8. Existing servers that read credentials from the environment

**The question.** A plain implementation is an existing Model Context Protocol server that was
not written for this specification (PKG-020). It cannot make an onward request, because it does
not know Conduit's keys (IFC-007). Many such servers run over stdio and take their credential
from an environment variable, as the protocol tells them to: "retrieve credentials from the
environment". For the first user these are most of the servers there are.

| Option | What it is | What it costs |
| --- | --- | --- |
| A | Not in the first version. The developer gives such a server its credential outside Conduit, in the environment the engine was started with. | The credential is then outside every Conduit: it is not inspected, narrowed, recorded or closed. Every plain server sees the engine's whole environment. |
| **B** | An implementation item that states `command` may state `environment` (proposed): a map from a variable's name to a JSON Pointer into the configuration of the instance it serves, at a place that `secret-reference.v1` governs. Before it starts the process for an instance, the engine reads each secret through that instance's own open Conduit to it, as an onward request that serves the invocation it is about to dispatch, and sets the variable. A process started this way serves that one instance. | A new key on an implementation item. Process start, which DL12 leaves to each engine, gains a rule. A new value needs a new process, as Kubernetes and the CSI driver say of environment variables. The value is in the process's environment, which Compose warns may be exposed. |
| C | A wrapper: an implementation that carries Conduit's keys, reads the secret, and starts the plain server itself. | Every plain server needs a wrapper written for it. |

**Recommendation: B, weakly.** It is Compose's `environment` and Kubernetes's `secretKeyRef`,
fitted to how the protocol tells stdio servers to take credentials, and the read is still a
request through a Conduit of the instance, decided, recorded and closable like any other. The
recommendation is weak because it is the one place where the proposal adds a mechanism rather
than a rule, and because a process's environment is a weaker place to keep a secret than a
value read at each use. Remote plain servers, reached by `url`, would need the engine to present
an `Authorization` header for them; that is left out (see below).

**Fits.** MAN-009 and MAN-010 gain the key. PKG-019 gains what it means. DL12 gains that an item
with `environment` starts one process per instance. IFC-007 is unchanged: the server receives
none of Conduit's keys, only its environment.

### S9. What a value is

**The question.** What the output `value` of `reveal-secret.v1` holds.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | A string, as the Model Context Protocol carries it in `structuredContent`. | A binary key must be stored in a text form, such as base64 or PEM, by the person who stores it. |
| B | A string or bytes, with a mark for the encoding. | A second shape for one case the first user rarely has. |

**Recommendation: A.** A token, a password and a PEM key are all text. The output's schema is
`{type: string}`. What the engine does with a file that is not valid UTF-8 is part of DL16.

### S10. What the guarantee covers

**The question.** What the promises would claim, so that no reader takes them for more.

**Recommendation.** State the guarantee as exactly this, in the specification's README or in
the definition of a secret, and nothing broader:

- A place that `secret-reference.v1` governs never holds a value, only a reference, so the value
  is never in configuration, in a record, in an inspection result, in an observation, in a graph
  or in a diagnostic.
- A secret output is never recorded, retained, shown or delivered to any instance other than the
  one that sent the request, and never to an actor.
- An implementation that receives one does not pass it on (an obligation at the Implementation
  level, which the engine cannot check).

And state what it does not cover:

- A secret written where no secret is marked. It is data.
- An actor that can both choose a Resource's implementation and open a reading Conduit for it.
  The bound decides who can do both.
- What an implementation does with a value outside the engine. Restrictions in this version are
  mediated, not enforced (ACC-027); an isolation plugin is a future, optional part.
- A copy already read, after the Conduit is closed.

**Fits.** This is the same honesty as ACC-027's "mediated" and the purpose's "Enforcement: a
future, optional part of the language".

## Promises that would follow

These are drafts, written in the specification's promise form so that the owner can judge the
shape. IDs are placeholders, `SEC-1` onward; on acceptance each would go to the file its topic
belongs to and take an ID there. The layer and level are proposed. Every draft assumes the
recommendation of each decision. Where a draft depends on S8, it says so. Proposed names
(`secret.v1`, `secret-reveal.v1`, `reveal-secret.v1`, `secret-reference.v1`) follow NAM-004 and
the core package's naming, and may change.

```yaml
- id: SEC-1
  layer: language
  level: must
  when: >-
    A value is written in a place that the core package's reference schema
    `secret-reference.v1` governs, directly, as the `items` of a list, or as a branch of `anyOf`
    or `oneOf` that the value satisfies.
  then: >-
    The value is a reference to a Resource, written `{kind: Resource, name: <name>}`, as a
    reference to a Pool's entry, or, inside a Pool's template, as an entry reference. A bare name
    is not accepted there, and any other value makes the document that holds it rejected.
  observable: >-
    A string such as `ghp_0123` or `hunter` in that place fails the schema, and resolving
    returns a diagnostic naming the place, and produces no objects. `{kind: Resource, name:
    github-token}` is accepted. The resolved form is stored in its place under OBJ-009, and
    every reference schema, this one included, accepts it.

- id: SEC-2
  layer: language
  level: must
  when: >-
    A reference in a place that `secret-reference.v1` governs resolves to a Resource whose type
    is not the core package's `secret.v1`.
  then: >-
    The document that holds the reference is rejected.
  observable: >-
    Resolving returns a diagnostic naming the place, the Resource found and the type that was
    expected, as OBJ-007 names what was expected. A template whose reference names no object
    that exists is registered with a diagnostic under OBJ-008, and the check is made when it is
    instantiated.

- id: SEC-3
  layer: language
  level: must not
  when: >-
    A diagnostic is returned about a value in a place that `secret-reference.v1` governs.
  then: >-
    The diagnostic holds the value, or any part of it.
  observable: >-
    The diagnostic names the document, the place and the rule broken. Registering a document
    that writes a known string in that place returns a result in which that string appears
    nowhere, and the retained result of the registration holds it nowhere either.

- id: SEC-4
  layer: language
  level: must
  when: >-
    An Operation contract's output holds `secret: true`.
  then: >-
    The output is a secret output. It holds a `schema`, and does not hold `handle: true`. No
    input carries the mark.
  observable: >-
    An output with `secret: true` and `handle: true`, or with no `schema`, and an input with
    `secret: true`, are each rejected when the contract's package is installed or its project
    is submitted. `secret` is a boolean, and one that is absent is false.

- id: SEC-5
  layer: language
  level: must
  when: >-
    A value reference in a graph has the source `step`, and the step it names invokes an
    operation one of whose outputs is a secret output.
  then: >-
    The document that holds the graph is rejected.
  observable: >-
    Resolving returns a diagnostic naming the value reference and the output. No step, no
    graph output and no constituent can be given a secret value through a graph, whatever
    `pointer` the reference states.

- id: SEC-6
  layer: language
  level: must
  when: >-
    A template of an Interface or an Agent declares a Conduit whose `operations` list an
    operation that has a secret output.
  then: >-
    The template is rejected.
  observable: >-
    Resolving returns a diagnostic naming the declared Conduit and the operation. A Resource
    template may declare such a Conduit.

- id: SEC-7
  layer: convention
  level: must
  when: >-
    An actor asks to open a Conduit whose consumer is an Interface or an Agent, the root
    Interface included, and that lists an operation that has a secret output.
  then: >-
    The request is not dispatched, and reports refused.
  observable: >-
    It reports refused whoever asks, the root Interface included, and the Conduit's state is
    still registered. No Conduit whose consumer is an actor ever lists such an operation, so no
    tool for one is presented to an actor under IFC-012. ACC-032 refuses the opening of a
    Conduit that lists an operation reserved to the root Interface in the same way.

- id: SEC-8
  layer: convention
  level: must not
  when: >-
    A record is kept of an invocation of an operation that has a secret output.
  then: >-
    The record holds the value of that output, whatever the `records` of the target's type
    states.
  observable: >-
    The record holds what REC-002 lists: the caller, the instance that acted for it, the
    Conduit, the target, the operation, the engine's ID for the invocation, the time and how it
    ended. A purge keeps what REC-004 lists, none of which is the output.

- id: SEC-9
  layer: convention
  level: must
  when: >-
    An instance asks, with `retrieve-result.v1`, for the result of an invocation of an
    operation that has a secret output.
  then: >-
    The result gives the invocation's state and the engine's ID for it, and no outputs. The
    engine keeps no content of such a result for retrieval.
  observable: >-
    The retrieval says what OPS-016 says of the state and the ID, and its `outputs` hold no
    secret output. The value is delivered once, in the result of the request itself, or not at
    all.

- id: SEC-10
  layer: convention
  level: must not
  when: >-
    An invocation of an operation that has a secret output ends.
  then: >-
    The value of that output appears anywhere other than in the result that the engine returns
    to the instance that sent the request: in an inspection result, an observation, a
    diagnostic, a record, the result given to any other instance, or the `_meta` of any
    request.
  observable: >-
    With a known value in the store, the value is found in the one result returned to the
    sending instance and in nothing that inspecting, retrieving or reading records returns.

- id: SEC-11
  layer: interface
  level: must not
  when: >-
    An implementation that carries Conduit's keys has received the value of a secret output.
  then: >-
    It puts the value, or any part of it, into a result, a diagnostic, the `content` of a
    result, or the `arguments` or `_meta` of any request it sends to the engine.
  observable: >-
    With a known value in the store, no result that the implementation returns, and no request
    it sends to the engine, holds the value. It may use the value toward the outside service it
    is for. This binds the implementation at the Implementation level, as IFC-031 does; the
    engine cannot check what an implementation does outside it.

- id: SEC-12
  layer: interface
  level: must
  when: >-
    An implementation of `reveal-secret.v1` serves a request.
  then: >-
    It takes the value from the store that the secret Resource's configuration locates, and
    never from that configuration. Where the store cannot be read, it answers with the outcome
    failed and no value.
  observable: >-
    A secret whose configuration holds a string in place of a locator yields no value. A
    missing file, a locked keychain or a refusal by a secret manager gives failed, with a
    message that says the store could not be read and holds no value, and no other source is
    tried.

- id: SEC-13
  layer: convention
  level: must
  when: >-
    `reveal-secret.v1` is requested of a secret Resource.
  then: >-
    The store is read for that request. The value the result holds is the one the store holds
    then.
  observable: >-
    Changing the value in the store between two reads gives the new value at the second, with
    no revision advanced, no control requested and nothing set up again. No earlier value is
    kept by the engine to stand in for a read.

- id: SEC-14
  layer: convention
  level: must
  when: >-
    The implementation `engine` of the core package serves `reveal-secret.v1` for a secret
    Resource.
  then: >-
    It reads the store that the configuration names: under `file`, the file at that absolute
    path; under `environment`, the variable of that name in the engine's own environment. The
    configuration names exactly one of the two.
  observable: >-
    Both are values tied to a location under PLC-020, and are read at the secret Resource's
    placement. A configuration that names both, or neither, gives the outcome failed under
    SEC-12. How a file is read, its encoding and its largest size, are DL16.

# The two below follow S8 and fall with it.

- id: SEC-15
  layer: language
  level: must
  when: >-
    An implementation item that states `command` states `environment`.
  then: >-
    `environment` is a map from the name of an environment variable to a JSON Pointer into the
    configuration of the instance the implementation serves, and the place it points at is
    one that `secret-reference.v1` governs in the instance's type. An item that states `url` or
    `graph` states no `environment`.
  observable: >-
    A template that selects the implementation, and whose type's schema does not put
    `secret-reference.v1` at a pointer that `environment` states, is rejected with a diagnostic
    naming the item and the pointer.

- id: SEC-16
  layer: convention
  level: must
  when: >-
    The engine starts the process of an implementation whose item states `environment`.
  then: >-
    The process serves one instance. Before starting it, the engine reads each secret that
    `environment` points at through that instance's own open Conduit to it, as an onward
    request that serves the invocation the engine is about to dispatch, and sets the variable
    to the value. Where any read does not succeed, the process is not started and the
    invocation has the outcome failed.
  observable: >-
    The record of each read names the instance as the sending instance and the caller of the
    invocation as the caller. A process started this way is not given the variables of another
    instance. A value changed in the store is seen by a process started after the change.
```

Exports the core package would gain, as documents: the Type contract `secret.v1` (family
Resource, no `schema`, `offers: [secret-reveal.v1]`, no controls, `records` with `inputs` and
`outputs` false, and `engine` as the default implementation of `reveal-secret.v1`); the
Capability contract `secret-reveal.v1`, listing `reveal-secret.v1`; the Operation contract
`reveal-secret.v1`, with no inputs, the output `value` with `schema: {type: string}` and
`secret: true`, and no `effect`; and the Schema contract `secret-reference.v1`. The package
would export 102 documents.

New item of the implementation-defined list:

```yaml
- id: DL16
  subject: Secrets that the engine reads
  document: >-
    How the engine's own implementation of `reveal-secret.v1` reads a file and an environment
    variable under SEC-14: the encoding it accepts, whether a final newline is kept, the
    largest value it reads, and with whose rights it reads them.
```

## Existing promises and documents that would change

If every recommendation were accepted. None of these is changed by this proposal.

| Where | What would change |
| --- | --- |
| [OBJ-021](../../../spec/promises/objects.yaml) | The list of reference schemas gains `secret-reference.v1`. |
| [CTR-002](../../../spec/promises/contracts.yaml) | An output may hold `secret: true` (SEC-4). The sentence on what an output holds, "either a `schema` or, for a handle, `handle: true`", gains the mark. |
| [GRA-020](../../../spec/promises/graphs.yaml) | Its observable points to SEC-5: a value reference may not read a secret output. |
| [ACC-032](../../../spec/conventions/access.yaml) and ACC-006 | ACC-006's condition for opening points to SEC-7 beside ACC-032. |
| [REC-002](../../../spec/conventions/records.yaml) | The sentence on what content is kept gains the exception of SEC-8. |
| [OPS-016](../../../spec/conventions/operations.yaml) and [IFC-024](../../../spec/interface/implementation.yaml) | A retained result of an operation that has a secret output keeps no content (SEC-9). |
| [IFC-025](../../../spec/interface/implementation.yaml) | Its observable points to SEC-10: the result that holds a secret output goes to the sending instance only. |
| IFC-031 | Unchanged; SEC-11 stands beside it. |
| [MAN-009, MAN-010](../../../spec/promises/manifest.yaml) and [PKG-019](../../../spec/promises/packages.yaml) | The key `environment` on an implementation item (S8 only). |
| [DL12](../../../spec/implementation-defined.yaml) | An item with `environment` starts one process per instance (S8 only). |
| The implementation-defined list | DL16 is added. |
| [spec/core/](../../../spec/core/README.md) | `types.yaml`, `capabilities.yaml`, `operations.yaml` and `references.yaml` gain one document each; `conduit.yaml` lists them; the README's counts change to 102 exports, 21 Operation contracts, 6 Capability contracts, 7 Type contracts and 12 reference schemas. |
| [spec/schemas/](../../../spec/schemas/README.md) | The schema of an Operation contract accepts `secret` on an output; the schema of an implementation item accepts `environment` (S8 only). New invalid examples for SEC-1, SEC-4 and SEC-15, and invalid-by-rule examples for SEC-2, SEC-5 and SEC-6. |
| [spec/definitions.md](../../../spec/definitions.md) | New rows for secret, secret output and `secret-reference.v1`; the row for reference schema gains the name; the row for handle may say that a secret's reference, like a handle, permits nothing. |
| [spec/schema-profile.md](../../../spec/schema-profile.md) | Unchanged. No keyword or format is added. |
| [GAPS.md](../docs-audit/GAPS.md), row GP3 | Converted, with the promises that state it. |
| [docs/AUDIT.md](../../AUDIT.md) | The ruling log, the list for the owner and the note for the next reader. |

Promises read and found unchanged by the recommendations: OBJ-007 to OBJ-010, POOL-001,
POOL-014, ACC-001, ACC-003, ACC-005, ACC-008, ACC-011, ACC-016, ACC-017, ACC-028 to ACC-030,
ACC-034, ACC-036, ACC-039, CON-008, CON-021, REC-001, REC-003, REC-004, OBS-001, OBS-006,
IFC-006, IFC-007, IFC-011, IFC-020 to IFC-022, IFC-030, IFC-032, LIF-031, OPS-009, PLC-020 and
DL13.

## Left out of the first version

Each of these was considered and is proposed for later, not rejected.

- **Writing a secret through Conduit.** No operation sets a value. The person writes it with
  the store's own tools. A password manager Resource that takes values would need the mark on
  inputs, and exceptions for the digest of contents (REC-004, OPS-009).
- **An engine-owned encrypted store.** The engine reads existing stores (S5).
- **The keychain in the core package.** An ordinary package, after experiment E10.
- **Dynamic secrets and leases**, as in Vault: an operation that issues a short-lived
  credential to a Resource and can revoke it at the issuer. The generic output mark leaves room.
- **Delivery the engine cannot read**, by wrapping the value for its reader, as Vault's response
  wrapping does.
- **Logins, refresh and OAuth flows**, and token exchange under RFC 8693. A login Resource type
  in a package, later.
- **A credential for a remote plain server**, which would need the engine to present an
  `Authorization` header as an OAuth client.
- **Workload identity**, with the engine issuing short-lived credentials to each instance as
  SPIFFE does. DL13 stays as it is.
- **Masking**: the engine replacing a known secret wherever it appears in a result, as GitHub
  Actions masks logs. Useful, never a guarantee, and an engine may still do it.
- **Secret values in graphs** (SEC-5 rejects them), and secret inputs of operations.
- **Changing a secret's locator** by a control, and pinned versions that rotate.
- **Binary values** (S9).
- **Detecting a secret written where none is marked**, such as by its entropy.
- **Isolation**: keeping an implementation from sending a value elsewhere. Enforcement is a
  future, optional part.

## For the owner to weigh

The points where the recommendation is least certain, or where the owner's view of the product
decides:

1. **No actor reads a secret, not even the root Interface (S4, option B).** The developer can
   never see a value through Conduit; they see it in the store. If the owner wants the root
   Interface to read, option C is one exception.
2. **The limit of the guarantee (S4, S10).** An actor that may both register a Resource with
   an implementation it chose and open a reading Conduit for it can recover the value. Option D
   of S4 reserves opening reading Conduits to the root Interface, which is simple for one
   developer and closes delegation for teams.
3. **Plain servers (S8).** The one new mechanism in the proposal, recommended weakly. Without it
   the commonest servers keep taking credentials outside Conduit.
4. **The engine sees each value in transit (S3).** Acceptable for one developer on one machine;
   a team may want delivery the engine cannot read.
5. **An Agent's own login stays outside (S7).** If the owner reads "authentication" as covering
   an Agent's own provider credential, that needs a different answer, and S4 option B forbids
   delivering a secret to an actor.
6. **The engine reads a file and an environment variable (S5, B1).** If the owner prefers the
   older design's default, the keychain, option B2 puts it in every engine.
