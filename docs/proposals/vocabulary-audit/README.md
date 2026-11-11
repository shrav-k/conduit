# Vocabulary audit before the Codex read

Status: ruling 42: in force; amended by 43 (D17); done (D5 and D20: ruling 43) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 3. Vocabulary and the project's names](../../../IN_FORCE.md#3-vocabulary-and-the-projects-names), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [PLC-026](../../../spec/promises/placement.yaml), [SEC-023](../../../spec/promises/secrets.yaml). This record says why.

The text below is the audit as it was put to the owner.

Was: proposed 2026-11-06, for the owner. [Ruling 41](../ruling-41/README.md) asked for it:
"I don't necessarily like the name secret reveal for the contract, we should do another vocab
audit as well". This is a proposal. Nothing in it is ruled.

## Scope and method

The audit read the whole of [`spec/`](../../../spec/README.md) as rulings 40 and 41 left it, in
the terms of [spec/definitions.md](../../../spec/definitions.md). It covers the names that the
review and the work order coined ([P1](../ruling-41/PLAN.md#planners-choices) and the
[terms](../ruling-41/PLAN.md#1-terms) of the work order), the terms of the new promise files
[providers.yaml](../../../spec/promises/providers.yaml) and
[secrets.yaml](../../../spec/promises/secrets.yaml), the exports of the
[core package](../../../spec/core/README.md), and the terms that the integration left
[for the next editors](../ruling-41/log-integration.md#for-the-next-editors).

Five readers each read the specification through a different lens. Their 47 proposals were
merged by term. Two checks then challenged each one: one on precedent, one on clarity and
collisions. Every standard term cited below was confirmed in its primary source, which is
linked where it is cited. A rename comes to the owner when both checks kept it, or when one
kept it on a verified precedent. A text that strays from a term in force is an editorial fix.
The rest is listed under [Considered and kept](#considered-and-kept).

The owner's principles decide the choices: existing paradigms first, nothing overcomplicated,
and Conduit innovates only on connection and access. A term of JSON Schema, YAML or the Model
Context Protocol keeps its own meaning. A term that the vocabulary rulings (17, 18, 27 to 33,
in the [vocabulary record](../vocabulary/README.md)) settled is reopened only where rulings 40
and 41 changed what it names, or where it now collides with a standard that the rewrite
adopted. Each such case says which.

At a glance:

| # | Now | Proposed | Occurrences under `spec/` |
| --- | --- | --- | --- |
| N1 | `secret-reveal.v1`, `reveal-secret.v1` | `secret-provider.v1`, `get-secret.v1` | 91 in 18 files |
| N2 | machines provider; the engine's own machine | machine provider; the engine's own machine provider | 70 and 12 |
| N3 | `run.v1`'s input `location` | `machine` | about 12 |
| N4 | `inspect-run.v1`'s state `stopped` | keep (`exited` was proposed) | 4 |
| N5 | `run.v1`'s output `endpoint` | `url` | 9 |
| N6 | `arguments` of an implementation item and of `run.v1` | `packageArguments` | about 38 |
| N7 | `digest` of a `package` item | the Package's own `fileSha256`, required | about 18 |
| N8 | `label-selector.v1` | `labels.v1` | 15 |
| N9 | `placement` in a Type or an item; `place.v1`'s input `selector` | `nodeSelector` | about 28 |
| N10 | location, `location.v1`; stand for | node, `node.v1`; represent | 318 and 30 |
| N11 | counts as declared lost | the engine declares the location lost | about 5 |
| N12 | `record.v1`'s `data.instance` | `data.consumer` | about 6 |
| N13 | `observedGeneration` | remove | 3 |
| N14 | origin, and the condition key `origin` | source | 20 |
| N15 | operation identity, `operationIdentity` | idempotency key, `idempotencyKey` | 52 |
| N16 | `_meta` key `io.github.shrav-k.conduit/restrictions` | `io.github.shrav-k.conduit/filters` | 12, and prose |
| N17 | extension `io.github.shrav-k.conduit/interface` | `io.github.shrav-k.conduit/implementation-interface` | 4 |
| N18 | runtime profile | conformance document | 7 |
| N19 | trust domain (of a secret) | domain of trust | 4 |
| N20 | its own setting for restarts | restart policy | 5 |

## Names for the owner

### N1. The secret contract: `secret-reveal.v1` and `reveal-secret.v1`

**What and where.** `secret-reveal.v1` is the core capability that the type of every secret
Resource offers ([capabilities.yaml](../../../spec/core/capabilities.yaml)).
`reveal-secret.v1` is its one operation, which takes no input and returns `value`
([operations.yaml](../../../spec/core/operations.yaml)). The word "reveal" occurs 91 times in
18 files under `spec/`, all of them in these two names. `secret-reveal.v1` occurs 31 times in
13 files, 7 of them in PRV-001, PRV-004, SEC-002, SEC-005 and SEC-020. `reveal-secret.v1`
occurs 60 times in 14 files, 22 of them in 14 promises: ACC-001, ACC-003, ACC-005, ACC-006,
IFC-012, MAN-012, SEC-005 to SEC-010, SEC-015 and SEC-020. `docs/OVERVIEW.md` has neither.

**Problem.** The owner objected to the word. No secrets system names this contract or its
operation for revealing: in 1Password, `--reveal` is a flag of `op item get` that shows
concealed fields, and Kubernetes uses "reveal" only in prose. The capability is also the one
provider capability named for an act, and the one written noun then verb. The review coined
both names and ruling 41 left them open, so no earlier ruling settled them.

**Options.**

| Name | For | Precedent | Source |
| --- | --- | --- | --- |
| `secret-provider.v1` | capability | External Secrets Operator: "Provider is a common interface for interacting with secret backends", and a SecretStore names one ("exactly one provider must be configured"). The Secrets Store CSI driver "communicates with the provider using gRPC to fetch the mount contents from external Secrets Store". Conduit's `runtime-provider.v1` and `machine-provider.v1` | https://github.com/external-secrets/external-secrets/blob/main/apis/externalsecrets/v1/provider.go, https://external-secrets.io/latest/api/secretstore/, https://secrets-store-csi-driver.sigs.k8s.io/concepts.html |
| `secret-store.v1` | capability | Dapr: "SecretStore is the interface for a component that handles secrets management". The ESO kind `SecretStore` "maps to exactly one instance of an external API" | https://github.com/dapr/components-contrib/blob/main/secretstores/secret_store.go, https://external-secrets.io/latest/api/secretstore/ |
| `secret.v1` | capability | The Kubernetes kind `Secret`, which holds the value. Decision 19 of the review dropped this name for a core Type | https://kubernetes.io/docs/concepts/configuration/secret/ |
| `get-secret.v1` | operation | ESO: "GetSecret returns a single secret from the provider". Dapr `GetSecret`. Azure Key Vault "Get Secret", whose secret is in `value`. Bitwarden `bws secret get`. The Kubernetes request verb `get` | https://github.com/external-secrets/external-secrets/blob/main/apis/externalsecrets/v1/provider.go, https://docs.dapr.io/reference/api/secrets_api/, https://learn.microsoft.com/en-us/rest/api/keyvault/secrets/get-secret/get-secret, https://bitwarden.com/help/secrets-manager-cli/, https://kubernetes.io/docs/reference/access-authn-authz/authorization/ |
| `read-secret.v1` | operation | `vault read`. 1Password `op read`, which reads "the value of the field in 1Password specified by a secret reference". The Kubernetes API reference lists the GET of one Secret as "`get` Read" | https://developer.hashicorp.com/vault/docs/commands/read, https://www.1password.dev/cli/reference/commands/read/, https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/ |
| `get-secret-value.v1` | operation | AWS Secrets Manager `GetSecretValue` | https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html |
| `access-secret.v1` | operation | Google Secret Manager `versions.access`, "This call returns the secret data" | https://docs.cloud.google.com/secret-manager/docs/reference/rest/v1/projects.secrets.versions/access |

**Recommendation.** `secret-provider.v1` and `get-secret.v1`. Both checks chose this pair;
three readers of five chose the capability and four the verb.

- ESO pairs `Provider` with `GetSecret`, and secrets.yaml already says the contract follows
  ESO and Dapr.
- `secret-store.v1` would give "store" a second meaning. In Conduit a store is the system
  outside the engine that a locator names (SEC-006). In ESO and Dapr a store serves many
  secrets, each named in the request, while a Conduit secret Resource locates one secret and
  its operation takes no input.
- `get` is the verb that the core README's Names rule gives, as `kubectl get secret` does.
  The noun "the engine's read of a secret" stays, as Kubernetes calls the GET of a Secret
  "`get` Read". The runner-up is `read-secret.v1`, which would sit beside MCP's
  `resources/read`.
- `access-secret.v1` collides with Conduit's `access`.

**What changes with it.** All 91 occurrences, among them PRV-001's list, the definitions rows
provider, provider capability and secret Resource, capabilities.yaml, operations.yaml,
references.yaml, conduit.yaml, the core README, the schemas and the example project. The row
secret Resource adds that each secret Resource locates exactly one secret, so its operation
takes no key. The row provider call and PRV-005 keep saying that the engine's read of a secret
is a request and not a provider call. The descriptions that say "secret store" follow
(editorial fix E17). These stay: secret Resource, store, locator, `secret-reference.v1` and
the output `value`.

### N2. The machines provider, and the engine's own machine

**What and where.** "machines provider" occurs 70 times in 16 files under `spec/`, 33 of them
in 23 promises, among them PLC-002, PLC-007, PLC-023, PLC-024, PLC-026, POOL-016, PRV-001 to
PRV-003, PRV-013 to PRV-015, PRV-018 and PRV-019. `docs/OVERVIEW.md` has 4. The capability is
`machine-provider.v1` (23 times in 9 files). "machine provider" occurs nowhere. "own machine"
occurs 12 times in 6 files. "The engine's own machine" names the built-in implementation of the
capability in PLC-002, PLC-026, PRV-003, PRV-014 and PRV-015, two definitions rows,
capabilities.yaml and the core README.

**Problem.** The prose is plural and the export singular. Every other provider's prose name is
its capability's name: runtime provider for `runtime-provider.v1`. A search for one spelling
misses the other. And "machine" is defined as a computer, so PRV-014's "The engine's own
machine reports the machine the engine runs on" reads as a computer reporting itself. The
authors already read the phrase as the computer: `docs/OVERVIEW.md` and D15 of the
[coverage check](../ruling-41/log-integration.md#coverage-check) do. The work order coined
both forms, and neither is a standard term.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| machine provider (prose), keeping `machine-provider.v1` | A noun used as a modifier is singular, as in Kubernetes' "cloud provider". Conduit's pair runtime provider and `runtime-provider.v1` | https://kubernetes.io/docs/concepts/architecture/cloud-controller/ |
| machines provider, with the export renamed `machines-provider.v1` | The work order's term | [PLAN.md](../ruling-41/PLAN.md#1-terms) |
| the engine's own machine provider | A built-in keeps its role's name. Nomad: "The Nomad binary contains several bundled task drivers". Kubernetes: "If no runtimeClassName is specified, the default RuntimeHandler will be used" | https://developer.hashicorp.com/nomad/docs/job-declare/task-driver, https://kubernetes.io/docs/concepts/containers/runtime-class/ |

**Recommendation.** Write "machine provider" and keep `machine-provider.v1`. Write "the
engine's own machine provider" for the built-in. Both checks and all five readers agree.

**What changes with it.** The row title becomes "the engine's own runner, machine provider and
fetching". "The machine the engine runs on" stays for the computer, and "own runner" and
"fetching" stay. The row provider adds that the engine's own runner, machine provider and
fetching are part of the engine and are no Resource (PRV-003).

### N3. The input `location` of `run.v1`

**What and where.** `run.v1` takes `location: {hostname, labels}`
([operations.yaml](../../../spec/core/operations.yaml): its head comment, description, input and
effect). Also PRV-007 (3 times), PRV-019, PLC-027, the core README (2) and the definitions rows
run and runtime provider: about 12 in all.

**Problem.** Everywhere else a location is a Resource of `location.v1` (PLC-008), and
`declare-lost.v1`'s input `location` is a reference to one. This input holds the description of
a machine instead: the shape of an item of `list-machines.v1`'s output `machines`. So PRV-019
has to say "the machine that `location` names", which reads as the instance's location. The
rewrite coined the input.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `machine` | Conduit's term in force, machine, and the output `machines` of `list-machines.v1`. Kubernetes names the node itself in `nodeName`, and "A node may be a virtual or physical machine" | https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/, https://kubernetes.io/docs/concepts/architecture/nodes/ |
| `location` (keep) | None | |

**Recommendation.** `machine`, holding `hostname` and `labels`. Both checks agree. It does not
depend on N10.

**What changes with it.** PRV-007, PRV-019, PLC-027, `run.v1`, the core README and the two rows.
"The machine that `location` names" becomes "that machine".

### N4. The state `stopped` of `inspect-run.v1`

**What and where.** `inspect-run.v1` reports `running`, `stopped` or `unknown`
([operations.yaml](../../../spec/core/operations.yaml), 2 times). Also PRV-009 and LIF-011, which
map the state to the condition `Running`: 4 in 3 files.

**Problem.** The runtime provider follows CRI and Nomad's task drivers. Both, and Docker, say
"exited" for a container or task that ran and has ended. One check kept the rename on that
precedent. The other found that "exited" is narrower than Conduit's state. PRV-007 asks
`inspect-run.v1` before the first run, so a provider reports `stopped` for a run that was never
asked for, and for one that `stop.v1` ended and the provider removed. In CRI and Docker a
container that never started is `created`, and one that is not known is an error, not a state.
`exited` would say that the run ran.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `exited` | CRI `CONTAINER_EXITED`; Nomad `TaskStateExited = "exited"`; Docker status `exited` | https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto, https://github.com/hashicorp/nomad/blob/main/plugins/drivers/driver.go, https://docs.docker.com/reference/cli/docker/container/ls/ |
| `stopped` (keep) | Pairs with Conduit's own `stop.v1`, and covers a run that is not running for any reason | [operations.yaml](../../../spec/core/operations.yaml) |

**Recommendation.** Keep `stopped`. If the owner prefers `exited`, PRV-009 and LIF-011 say
"whose run has exited", and the condition `Running` maps as before.

### N5. The output `endpoint` of `run.v1`

**What and where.** `run.v1` returns `endpoint`, a Streamable HTTP URL (P18). It occurs 9 times
in 5 files: operations.yaml, PRV-003, PRV-007, PRV-018, IFC-017 (2), the row run and
spec/README.md.

**Problem.** IFC-017 reaches a remote item "at the URL of its Remote", and a run "at the
`endpoint` that its run returns": two words for one kind of value. server.json, the form an
implementation item takes, names this value `url`, in a Remote and in a Package's
`streamable-http` transport alike. The checks split. One kept `url` and added that "endpoint"
is the word the vocabulary record dropped for a Conduit's end. The other would keep
`endpoint`, which is MCP's word in its prose.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `url` | server.json `StreamableHttpTransport.url`: "URL template for the streamable-http transport" | https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json |
| `endpoint` (keep) | MCP: "a single HTTP endpoint path (hereafter referred to as the MCP endpoint)" | https://modelcontextprotocol.io/specification/2025-11-25/basic/transports |

**Recommendation.** `url`, at low priority. The engine then reaches every Streamable HTTP server
at a `url`, and the prose may still say "the MCP endpoint", as MCP's own prose does. MAN-009's
rejection of a top-level `url` in an item is a rule about item keys and does not conflict.

### N6. The `arguments` of an implementation item and of `run.v1`

**What and where.** P1 coined the item key. It maps the names that a server.json Package
declares to JSON Pointers into the configuration (implementation-item.v1 in
[manifest.yaml](../../../spec/schemas/manifest.yaml)), and `run.v1` takes the values in its input
`arguments`. About 38 occurrences in 12 files: MAN-009, MAN-010, MAN-017, OBJ-016, OBJ-019,
PKG-019, PKG-020, PRV-007, SEC-006, SEC-010, SEC-020 and SEC-021, and the definitions rows
argument, implementation item, plain, run and the engine's read of a secret.

**Problem.** The key collides three times.

- An argument is a ruled term (rulings 17 and 18): the value supplied for an input, written
  under `arguments` in an instance (OBJ-013). The definitions row has to carve the item key out.
- MCP's `tools/call` holds an operation's inputs in `arguments` (IFC-001), so a provider call to
  `run.v1` carries `params.arguments.arguments`.
- A filter item has `arguments` too (IFC-018).

It is also unclear: a Package declares two lists, and MAN-010's "an argument that the Package
declares" does not say which.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `packageArguments` | server.json Package `packageArguments`: "A list of arguments to be passed to the package's binary." | https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json |
| `runtimeArguments`, beside it | The same schema: "A list of arguments to be passed to the package's runtime command (such as docker or npx)." | https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json |
| `args` | Kubernetes and CRI `args`: a final list of arguments, not a map | https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto |

**Recommendation.** `packageArguments`, for the item key and for `run.v1`'s input. Both checks
and four readers agree. The item already does the same with `headers`, beside the Remote's
`headers`.

**What changes with it.** State in MAN-010 that a key is a named argument's `name`, with its
dashes, or a positional argument's `valueHint`, as server.json resolves its own URL variables.
That answers the positional-key part of R27. Leave `runtimeArguments` unbound: they belong to
the runtime command, which is the runtime provider's (PRV-012). Keep `headers` and
`environment` (see [Considered and kept](#considered-and-kept)). The definitions row argument
loses its carve-out.

### N7. The `digest` of a `package` item

**What and where.** P15 put `digest`, "the SHA-256 of the package's artifact" in PKG-004's
`sha256:<hex>` form, beside the server.json Package of a `package` item. About 18 occurrences in
8 files: implementation-item.v1, MAN-009, MAN-010, PKG-019, the definitions rows implementation
item and digest, and the core README.

**Problem.** The Package already carries this hash as `fileSha256`, which server-package.v1
admits. So one item can state one hash twice, in two notations, and no promise says that they
agree or which governs. In one manifest, `digest` also names a Conduit package's archive hash
(PKG-004).

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `fileSha256`, required for every `package` item | server.json Package `fileSha256`: "SHA-256 hash of the package file for integrity verification." Required for MCPB, optional otherwise | https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json, https://modelcontextprotocol.io/registry/package-types |
| `digest`, rejecting `fileSha256` | The OCI descriptor `digest`, `algorithm ":" encoded` | https://github.com/opencontainers/image-spec/blob/main/descriptor.md |

**Recommendation.** Remove the item's `digest` and require the Package's own `fileSha256`. Both
checks agree. Decision 25a asked for a required digest, not for a key, so it holds. The second
option would reject every MCPB Package copied from a registry, which must state `fileSha256`.

**For the owner as well.** An `oci` image has no single file to hash. server.json pins an image
by writing a digest as the tag of its `identifier`. The owner decides whether an `oci` item is
pinned that way instead. Check the registry types of PRV-012 before applying.

### N8. The schema `label-selector.v1`

**What and where.** 15 occurrences in 9 files. It is defined in
[values.yaml](../../../spec/schemas/values.yaml). It types `placement` in type-spec.v1 and in
implementation-item.v1, and in operations.yaml it types `run.v1`'s `location.labels`,
`list-machines.v1`'s `machines[].labels` and `place.v1`'s `selector`. Also the core README, the
schemas README, OBJ-022 and the definitions rows label and label selector.

**Problem.** One schema named for a selector types two things: a selector, and the labels a
machine has. Its description, "An empty map selects every location", is wrong for the second.
Kubernetes keeps labels, "key/value pairs that are attached to objects", apart from a label
selector, with which "the client/user can identify a set of objects". The CRI does call a plain
map a `label_selector`, so the name suits the selector role. The defect is that it also types
labels.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `labels.v1` | Kubernetes labels; CRI `map<string, string> labels`. The same map shape serves as `nodeSelector` and CRI `label_selector` | https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/, https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto |
| `node-selector.v1` | Kubernetes `nodeSelector` | https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/ |
| `label-selector.v1` (keep) | CRI `label_selector`; Kubernetes `LabelSelector` has `matchLabels` and `matchExpressions` instead | https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/label-selector/ |

**Recommendation.** `labels.v1`: "a map from a label to its value, the shape of Kubernetes
labels and `nodeSelector`". Both checks agree. Keep "label selector" in the prose for the role
under `placement` (or `nodeSelector`, N9) and in `place.v1`. The empty-map rule stays in CTR-004
and in the row label selector.

### N9. The key `placement` of a Type or an item, and `place.v1`'s `selector`

**What and where.** About 28 occurrences in 15 files: type-spec.v1
([contracts.yaml](../../../spec/schemas/contracts.yaml)), implementation-item.v1 and the core form
of manifest.v1 ([manifest.yaml](../../../spec/schemas/manifest.yaml)), `place.v1`'s input
`selector`, CTR-004, MAN-009, MAN-010, OBJ-019, OBJ-022, PKG-019, PLC-007, PLC-025 and the
definitions rows type, implementation item, label selector and placement.

**Problem.** In an instance, `placement` is the standard argument, a reference to a location,
which rulings 17 and 18 settled and which is not in question. Ruling 41 (decision 8a) gave the
same key a second shape in a Type or an item: a map of labels that the location's labels must
hold. And `place.v1` calls that same value `selector`. So one key has two shapes and one value
has two names. Kubernetes keeps exactly these two apart, as `nodeName` and `nodeSelector`.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `nodeSelector` | PodSpec `nodeSelector`: "Kubernetes only schedules the Pod onto nodes that have each of the labels you specify". RuntimeClass `scheduling.nodeSelector` is merged with the Pod's, and a conflict is rejected, which is PLC-025's rule | https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/, https://kubernetes.io/docs/concepts/containers/runtime-class/ |
| `selector` | Conduit's own `place.v1` input. In Kubernetes, a Service's `selector` picks the Pods it routes to, another meaning | https://github.com/kubernetes/api/blob/master/core/v1/types.go |
| `placement`, renaming `place.v1`'s input to match | Compose `deploy.placement`, which holds constraints and preferences, not a map of labels | https://docs.docker.com/reference/compose-file/deploy/ |

**Recommendation.** `nodeSelector`, in a Type, an implementation item, the core form of the
manifest and `place.v1`'s input. `placement` then means only the standard argument. Both checks
agree. "Node" in the key is Kubernetes' name for what is selected, whether or not N10 adopts the
word. If the owner refuses "node", keep `placement` and rename `place.v1`'s input to
`placement`: that removes the second name but keeps the two shapes.

**What changes with it.** The rows say that the selector is matched against the labels reported
for the machine.

### N10. Location and `location.v1`, and "stand for"

**What and where.** "location" and "locations" occur 318 times in 36 files under `spec/`, in 51
promises: all of placement.yaml, and FST-001, FST-005, FST-006, LIF-007, LIF-017, LIF-022 to
LIF-026, PRV-003, PRV-007, PRV-015, PRV-019, PRV-020 and REC-005, among others. `location.v1`
occurs 21 times in 10 files, and the core package exports the template `location`. Outside
`spec/`: `tools/check_schemas.py` (16) and two example file names. "stand for" and "stands for"
occur 30 times in 7 files, in 14 promises when "for which `local` stands" is counted.

**Problem.** Vocabulary decision V4 settled "location" when a location could be any place an
actor registered with facts. Ruling 41 changed what it names. The engine now registers one
location for each machine that the machine provider reports, names it from the hostname, holds
its `kubernetes.io/hostname`, `os` and `arch` labels, makes its instances unavailable while the
machine is unreachable and retires it when the machine is gone (PLC-023, PLC-024, PLC-026).
That is the Kubernetes Node, which spec/README.md and providers.yaml cite. The word also
collides with standards the rewrite adopted:

- SARIF's `location`, "A location within a programming artifact", which `diagnostic.v1` holds
  as `locations[].logicalLocations` (P16);
- JSON Schema's "instance location", a JSON Pointer;
- a cloud location, which is a region: Azure's `location`, and Google Secret Manager's
  `locations/*` in the very locators a secret Resource reads. PRV-014 lets a provider report
  `topology.kubernetes.io/region` as a label of a location.

"Stand for" is defined for a location and its machine, but PRV-003 also uses it for the
built-ins ("no Resource stands for them") and SCH-015 for a `$ref`.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| node: `node.v1`, the template `node` | Kubernetes: "A node may be a virtual or physical machine"; "The kubelet on a node self-registers to the control plane". Nomad's node is a machine running a client | https://kubernetes.io/docs/concepts/architecture/nodes/, https://developer.hashicorp.com/nomad/docs/glossary |
| location (keep, with a note) | Settled by the vocabulary record. Azure's `location` is where a resource is deployed | [vocabulary record](../vocabulary/README.md), https://learn.microsoft.com/en-us/azure/azure-resource-manager/templates/resource-location |
| represent, for "stand for" | Kubernetes: "Kubernetes creates a Node object internally (the representation)" | https://kubernetes.io/docs/concepts/architecture/nodes/ |
| the colliding senses | SARIF `location`; JSON Schema "instance location" | https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/schemas/sarif-schema-2.1.0.json, https://json-schema.org/draft/2020-12/json-schema-core |

**Recommendation.** Rename location to node, and write "a node represents the machine it was
registered for". Both checks agree. This is the largest change proposed, but it is mechanical,
and pull request 7 is unmerged and no engine exists, so it will never be cheaper.

- Keep machine for the computer, `placement` for the standard argument, `local` as a name, and
  SARIF's `locations` as they are.
- The row says "as Kubernetes represents a machine by a Node", not that the policies are the
  same: Kubernetes evicts Pods from an unreachable node and lets users label nodes, and Conduit
  does neither.
- The row adds that a node is a Resource of the type `node.v1`, and is not a YAML node.

If the owner keeps location, its row adds that a location is never a region or zone and is not
SARIF's `locations`, "stand for" stays, and only editorial fixes E12 and E13 apply.

**What changes with it.** "Value tied to a location" becomes "value tied to a node", "declare a
location lost" becomes "declare a node lost", and `declare-lost.v1`'s input `location` becomes
`node`. N3 and N9 read naturally with it. `tools/check_schemas.py` and the example file names
follow. Three citations of NAM-020 for the template `location` are stale whichever way the owner
rules (E23).

### N11. "Counts as declared lost"

**What and where.** PLC-024 says a location whose machine is reported gone "counts from then on
as declared lost"; PLC-027 says "was declared lost ... or counts as declared lost"; LIF-017
needs "in this sense". Also the definitions rows declare lost and unreachable, gone. "Declared
lost" occurs 32 times in 18 promises.

**Problem.** "Declare lost" names the root Interface's operation `declare-lost.v1`. PLC-024 makes
a report count as a declaration, so "was declared lost" elsewhere silently includes a case in
which nobody declared anything. Ruling 41 changed what the term covers (decision 8e), so it may
be reopened. Both checks kept the defect and differed on the fix.

**Options.**

| Fix | Precedent | Source |
| --- | --- | --- |
| The engine declares the location lost when its machine is reported gone, with PLC-017's effect | None needed: no new word | [placement.yaml](../../../spec/promises/placement.yaml) |
| Define "lost": a location declared lost or whose machine is reported gone | Nomad: "the client node will be marked as "down" and the allocation will be marked as "lost"" | https://developer.hashicorp.com/nomad/docs/job-specification/disconnect |
| Keep as it is | | |

**Recommendation.** The first. "Declared lost" is then true for both causes, and "counts as" and
"in this sense" go: about 5 edits. "Lost" as a word of its own would read as a fourth lifecycle
state beside registered, ready and retired, while a lost location is retired. "Lost" already
means lost results and records (IFC-016, OPS-018, REC-006). And Kubernetes' pod reason
`NodeLost` means unreachable
(https://github.com/kubernetes/kubernetes/blob/master/pkg/util/node/node.go), which in Conduit
retires nothing (PLC-023).

### N12. The member `data.instance` of `record.v1`

**What and where.** `record.v1` in [results.yaml](../../../spec/core/results.yaml) names "the
instance that acted for the caller" in `data.instance`. Also REC-002 and SEC-015. "Acted for the
caller" occurs 3 times in 3 files and "reading instance" 5 times in 4.

**Problem.** `instance` names a different role in each place. The `_meta` key `instance` names
the instance whose implementation receives a request (IFC-006). A provider call's `instance` is
the provider's own (PRV-005). The provider operations' input `instance` is the instance whose
run is meant. The record's `data.instance` is the sender. In the record of a provider call a
reader cannot tell which. CloudEvents has no attribute for the sender, so the name is Conduit's
to choose.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `data.consumer` | Conduit's ruled term consumer, "The instance that makes requests through a Conduit", beside `data.conduit` and `data.target` | [definitions.md](../../../spec/definitions.md), [vocabulary record](../vocabulary/README.md) |
| `data.instance` (keep), widening the row "the instance that sent a request" | Conduit's own row | [definitions.md](../../../spec/definitions.md) |

**Recommendation.** `data.consumer`: the consumer of the Conduit used, present where
`data.conduit` is. Both checks agree. It covers the engine's read of a secret, which goes
through the instance's own Conduit (SEC-010), without stretching another row. REC-002 and
SEC-015 say "the consumer of the Conduit used", and "acted for the caller" and "reading
instance" go.

### N13. `observedGeneration` in `condition.v1`

**What and where.** 3 occurrences in 2 files: `condition.v1` in
[results.yaml](../../../spec/core/results.yaml) and the list of fields in LIF-011. No row.

**Problem.** Decision 24b adopted Kubernetes' condition as it is, and so took in this field. It
"represents the .metadata.generation that the condition was set based upon", and generation is
"A sequence number representing a specific generation of the desired state". No Conduit object
has a generation. Its revision is an opaque string compared only for equality (REV-001), which
cannot fill an integer. No promise says what an engine writes in the field.

**Options.**

| Fix | Precedent | Source |
| --- | --- | --- |
| Remove the field | It is `+optional` in Kubernetes, so a condition without it keeps Kubernetes' shape. NodeCondition has none | https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go |
| Keep it, with a note that Conduit never sets it | The same source | https://github.com/kubernetes/apimachinery/blob/master/pkg/apis/meta/v1/types.go |
| `observedRevision` | None: it would be invented | |

**Recommendation.** Remove it from `condition.v1` and from LIF-011. Both checks agree.
`condition.v1` is open, so a condition copied from Kubernetes that carries the field still
validates. Decision 24b named the field, so this is the owner's.

### N14. Origin

**What and where.** "origin" and "origins" occur 20 times in 7 files: REC-005 (7), OBS-002 (3),
LIF-011, PRV-009, the `condition.v1` key `origin` (2), definitions.md (5, in the rows origin,
record, observation and condition) and [schema-profile.md](../../../spec/schema-profile.md).

**Problem.** Ruling 33 chose "origin" because `source` was then the key of a value reference.
Ruling 41 removed value references and that key, and made every record a CloudEvent. A
CloudEvent's required `source` "Identifies the context in which an event happened", and the
sequence extension orders events within one `source`: what REC-005 says of records at one
origin. So a record's origin is now written in `source` under a second word, and no field of a
record is said to hold the origin by which REC-005 orders records. The term is reopened for
both reasons: its reason is gone, and it collides with a standard the rewrite adopted.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| source: the term and the condition key | CloudEvents: "The "source" is the context in which the occurrence happened". Kubernetes Event `source`, "The component reporting this event" | https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md, https://github.com/cloudevents/spec/blob/main/cloudevents/extensions/sequence.md, https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/event-v1/ |
| origin (keep), stating that a record's origin is its CloudEvent's `source` | Ruling 33 | [vocabulary record](../vocabulary/README.md) |

**Recommendation.** source, for the term and for the condition key. Both checks agree. The row
says the term is the CloudEvents attribute. The cost is a near clash with `package-source.v1`,
which is kept (see [Considered and kept](#considered-and-kept)), and with a few plain uses of
"source" in the READMEs. Keeping "origin" also leaves a clash with the HTTP `Origin` header,
which MCP's Streamable HTTP has servers validate.

**For the owner as well.** Whether a condition's `source` takes the record's form, a
URI-reference that the engine names; today the condition's `origin` is any string. And REC-005
then ties an engine's ordering promise to how it chooses `source`: an engine that uses one
`source` for every record owes one total order.

### N15. Operation identity

**What and where.** "operation identity" occurs 44 times in 9 files and `operationIdentity` 8
times in 6, in 21 promises: CTR-002, IFC-006, IFC-016, IFC-020, IFC-023, OPS-008 to OPS-012,
OPS-018, OPS-019, OPS-021, OPS-024, OPS-025, OPS-028, PRV-005, REC-002, REC-004, TPL-017 and
TPL-018. Also `record.v1`'s `data.operationIdentity`, the definitions row and the name of the
first shared convention in spec/README.md, "operation identity and outcomes".

**Problem.** The key sits beside the `_meta` key and record field `operation`, which hold the
operation's qualified reference, so it reads as "the identity of the operation". It is the
caller's value by which repeats are recognized: the definitions row says so almost word for word
as the IETF draft defines an idempotency key. Decision 6 of the review adopted that draft's
behaviour and Google AIP-155's, and the review calls it "the operation identity as an
idempotency key". The vocabulary rulings did not settle the term.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| idempotency key, `idempotencyKey` | IETF draft-ietf-httpapi-idempotency-key-header-08: "An idempotency key is a unique value generated by the client which the resource uses to recognize subsequent retries of the same request". MCP SEP-3182 proposed `idempotencyKey` on `tools/call`; it was closed unmerged | https://datatracker.ietf.org/doc/draft-ietf-httpapi-idempotency-key-header/, https://github.com/modelcontextprotocol/modelcontextprotocol/pull/3182 |
| request ID, `requestId` | Google AIP-155 `request_id`. MCP's `RequestId` is the JSON-RPC id, which a repeat does not keep | https://google.aip.dev/155, https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2025-11-25/schema.ts |
| operation identity (keep) | None | |

**Recommendation.** idempotency key and `idempotencyKey`, with the convention named
"idempotency keys and outcomes". Both checks agree. It touches 52 occurrences in 21
promises, all mechanically. `deduplicates` stays, and editorial fix E3 keeps it apart from
MCP's `idempotentHint`. If MCP later adopts a key of its own, follow it.

### N16. The `_meta` key `io.github.shrav-k.conduit/restrictions`

**What and where.** 8 occurrences in 2 files
([implementation.yaml](../../../spec/interface/implementation.yaml) 6,
[access.yaml](../../../spec/conventions/access.yaml) 2), in ACC-017, ACC-018, IFC-006, IFC-018,
IFC-019, IFC-020 and IFC-026, and 4 more written short as `restrictions` (the row Conduit's
keys, the core README, PRV-005 and a head comment). About 20 of the 62 prose uses of
"restriction" mean what the key carries.

**Problem.** A restriction is a filter or a bound. Under C20 a bound is applied by the engine and
never sent, and ruling 41 removed policies. So the key holds filters only: IFC-018 says the list
"holds every filter that applies". By definition it can never hold half of what its name
covers. Ruling 41 changed what it carries, so its name may be reopened. No outside standard
applies.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `io.github.shrav-k.conduit/filters` | Conduit's ruled term filter, which the row filter already says is sent with each request | [definitions.md](../../../spec/definitions.md), [vocabulary record](../vocabulary/README.md) |
| `io.github.shrav-k.conduit/restrictions` (keep) | None | |

**Recommendation.** `io.github.shrav-k.conduit/filters`. Both checks agree. Write "filters"
where only filters are carried or sent: ACC-009's clause on what is carried, ACC-017, ACC-018,
IFC-006, IFC-018 to IFC-020, IFC-026's `when`, IFC-040, PRV-005, SEC-011, TPL-017 and the rows
onward request and hook. Keep "restriction" in ACC-008, ACC-014, ACC-027 and the general case,
and keep the result key `io.github.shrav-k.conduit/restriction`, which can name a bound
(IFC-025). A later version that sends policies has its own prefix (IFC-029) and may widen the
key then.

### N17. The extension `io.github.shrav-k.conduit/interface`

**What and where.** 4 occurrences in 3 files: IFC-037 and the head comment of
[implementation.yaml](../../../spec/interface/implementation.yaml), the row Conduit's extension
and the core README. IFC-029 derives it: "that prefix followed by `interface`".

**Problem.** Conduit's `_meta` keys share the prefix, and the part after the slash names what the
key holds: `/instance`, `/caller`, `/conduit`. So `/interface` reads as a key that holds an
Interface, the family of `interface.v1`. MCP names an extension for the feature it adds. The
feature here is the implementation interface, a ruled term. P1 coined the identifier.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| `io.github.shrav-k.conduit/implementation-interface` | MCP: the format is `{vendor-prefix}/{extension-name}`, as in `io.modelcontextprotocol/oauth-client-credentials`; Conduit's ruled term implementation interface | https://modelcontextprotocol.io/extensions/overview |
| `io.github.shrav-k/conduit` | The MCP Registry's server names, `io.github.username/server`. It would read as a server's name and leave the prefix of spec/names.yaml | https://modelcontextprotocol.io/registry/about |
| `io.github.shrav-k.conduit/interface` (keep) | None | |

**Recommendation.** `io.github.shrav-k.conduit/implementation-interface`, at low priority. Both
checks agree. It keeps the prefix that [names.yaml](../../../spec/names.yaml) assigns, so
IFC-029's derivation changes only its last word.

### N18. Runtime profile

**What and where.** 7 occurrences in 4 files under `spec/`: the definitions row, spec/README.md
(3, the Engine conformance level among them),
[implementation-defined.yaml](../../../spec/implementation-defined.yaml) and the head comment of
first-start.yaml. No promise uses it. AGENTS.md has 1, docs/AUDIT.md 14 and docs/OVERVIEW.md 2.

**Problem.** Rulings 40 and 41 changed what both words name. `runtime` is now the standard
argument that names a runtime provider, so the term reads as a runtime provider's profile, and
the row has to deny it. Ruling 40 moved process layout and placement out of the profile into
the providers. And "the profile" in the SCH and CTR promises is the schema profile.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| conformance document | POSIX.1-2024, 2.1.2: "The conformance document shall describe the behavior of the implementation for all implementation-defined features" | https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap02.html |
| runtime profile (keep) | Settled by the vocabulary record, under Records and layers | [vocabulary record](../vocabulary/README.md) |

**Recommendation.** conformance document. Both checks agree. It sits beside the claim of
conformance and the conformance vectors of spec/README.md. The row adds that it is not a
document in the sense of definitions.md. docs/AUDIT.md's layered claim is kept in the words of
rulings 13 and 14, so add the term to its reading list rather than rewrite it.

### N19. Trust domain (of a secret)

**What and where.** 4 occurrences in 4 files: the definitions row, spec/README.md, PRV-011's
observable and the head comment of secrets.yaml. The term is new with ruling 41.

**Problem.** It means what can read a secret once the engine has read it. SPIFFE's trust domain
"corresponds to the trust root of a system", an identity namespace, and spec/README.md, IFC-022
and DL13 name SPIFFE as a mechanism for proving which instance sent a request. Both senses can
meet in one deployment. The checks split. One kept "trust boundary". The other found that a
trust boundary is a line where trust changes, not a set of places, and that the design's own
source names this set exactly.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| domain of trust | AWS Workload Credentials Provider, formerly the Secrets Manager Agent, which the secrets proposal cites: "the domain of trust is where the provider endpoint and SSRF token are accessible, which is usually the entire host" | https://docs.aws.amazon.com/secretsmanager/latest/userguide/workload-credentials-provider.html |
| trust boundary | OWASP: "Boundaries show any location where the level of trust changes". MCP's security page uses the phrase | https://owasp.org/www-community/Threat_Modeling_Process, https://modelcontextprotocol.io/specification/2026-07-28/basic/security_best_practices |
| trust domain (keep), with "not SPIFFE's trust domain" in the row | SPIFFE's sense is the one to set apart | https://spiffe.io/docs/latest/spiffe-about/spiffe-concepts/ |

**Recommendation.** "domain of trust", AWS's phrase for exactly this set, which removes the
SPIFFE reading at the cost of 4 edits. Keeping "trust domain" with a note is the close second.

### N20. "Its own setting for restarts"

**What and where.** PRV-010 (`then` and observable), OBS-004's observable, and the definitions
rows run and runtime (among the words not used): "own setting" occurs 5 times in 3 files.
Also docs/OVERVIEW.md.

**Problem.** The integration replaced "restart policy" because "policy" went under item 3a. But
3a removed Conduit's access policies, not the runtime's own term, and the review heads item 2b
"Restart policy". The new wording also uses setting, Conduit's word for one place in an
instance's configuration, which a provider's restart behaviour need not be.

**Options.**

| Name | Precedent | Source |
| --- | --- | --- |
| restart policy | Docker: "Docker provides restart policies to control whether your containers start automatically when they exit". Compose `restart`, "the policy that the platform applies on container termination". Kubernetes `restartPolicy` | https://docs.docker.com/engine/containers/start-containers-automatically/, https://docs.docker.com/reference/compose-file/services/, https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/ |
| its own setting (keep) | systemd calls `Restart=` a "restart setting" | https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html |

**Recommendation.** "under its own restart policy". Both checks agree. The row policy adds that
the word survives only in "restart policy", a runtime provider's own term, as the row facts
does for "host".

## Editorial fixes

These conform text to terms in force, or to a term that both checks kept. They need no ruling
and are for the coordinator to apply. E17 waits for N1, and E10 and E13 change with N10.

| # | Where | Now | Becomes | Why |
| --- | --- | --- | --- | --- |
| E1 | [spec/core/README.md](../../../spec/core/README.md), the Names table | No row for hooks, so `setup-hook.v1` and `retire-hook.v1` break the Operation row (a verb in the imperative) | A row: "Hook \| Named for the invocation it belongs to, then `-hook` \| `setup-hook.v1`, `retire-hook.v1`" | Both checks keep the names (see [Considered and kept](#considered-and-kept)). Cite no Kubernetes precedent: Kubernetes names hooks pre- or post- and the event |
| E2 | definitions.md, row reconcile | No note | Add: "It is not a Kubernetes controller's reconcile: nothing is driven toward what was declared (OBS-004)." | Kubernetes conditions now sit in the status, and controller-runtime's Reconcile moves current state toward desired state |
| E3 | definitions.md, row `deduplicates`; CTR-002's observable | Nothing relates it to `idempotentHint`, and the valid example declares both | Add: "It is not the Model Context Protocol's `idempotentHint`, which hints that calling the tool again with the same arguments has no further effect: the engine passes that hint into the tool's `annotations` (IFC-033) and never decides a new attempt by it (OPS-012)." | MCP: the hint is keyed on arguments and untrusted; `deduplicates` is keyed on an operation identity and acted on |
| E4 | IFC-004, IFC-037; definitions.md rows Conduit's extension and revision | "the extension's `revision` setting", "its setting `revision`" | "`revision` in the settings object of Conduit's extension", adding in the row that MCP prescribes the name | Setting is Conduit's word for a place in a configuration. MCP says "settings object" and names the member `revision` |
| E5 | IFC-008's observable | "the revision that the request declared" | "the revision of Conduit's extension that the request declared" | Every request carries two revisions |
| E6 | definitions.md: a new row call; rows request and provider call | "call" is used about 90 times in 58 promises with no row | Row "call: One `tools/call` request of the Model Context Protocol: what a consumer sends the engine, and what the engine sends an implementation for a request, a hook or a provider call." The rows request and provider call add that MCP calls any JSON-RPC request a request | MCP: "To invoke a tool, clients send a `tools/call` request"; MCP speaks of tool calls |
| E7 | IFC-006 (`when`, `then`), IFC-008 (`when`), IFC-019's observable | A hook called "the request" or "any request" | "call" where a hook is meant | The row request: a hook and a provider call are not requests |
| E8 | TPL-017's observable | "Its request carries the restrictions" | "It carries the restrictions" (the filters, after N16) | The same |
| E9 | PRV-011 (`when`) | "a request for `run.v1`" | "a provider call to `run.v1`" | PRV-005: a provider call is not a request |
| E10 | definitions.md, row location type; spec/core/types.yaml, head comment and the description of `location.v1` | "location type"; "The location type." | Remove the row and write "the type `location.v1`" (`node.v1` after N10) | No promise uses it: PLC-008 and PLC-028 say "the type `location.v1`" |
| E11 | definitions.md, row setup operation, retire operation, and rows set up and retire; TPL-008's observable | "An invocation of `set-up.v1` ..."; "Carry out the setup operation"; "by its retire operation"; "The setup operation and the retire operation of an instance mean these two invocations" | Remove the row and TPL-008's sentence. Row set up: "Carry out the setup of an instance, which is one invocation of `set-up.v1`"; row retire likewise for retirement | An operation is the action of an Operation contract, so `set-up.v1` itself is the setup operation. The nouns setup and retirement already name the invocation |
| E12 | SCH-015 (`then`) | "Each of the last two stands for the schema that is that contract's `spec`" | "Each of the last two references the schema that is that contract's `spec`" | JSON Schema: `$ref` is used "to reference a statically identified schema" |
| E13 | spec/core/README.md, The operations | "One contract stands for one verb" | "One contract for one verb", as operations.yaml says | "Stand for" is defined for a location and its machine |
| E14 | definitions.md, row ready | No note | Add: "A lifecycle state, a phase in Kubernetes' terms, and neither the Kubernetes condition `Ready` nor CRI's `RuntimeReady`. It does not change when the run stops or the machine becomes unreachable, and "not ready" means registered or retired. Conditions report health." | Ruling 29 settled ready. Conditions now sit beside it in the status, and CRI's `RuntimeReady` means healthy, where PRV-007 says "the runtime provider is not ready" |
| E15 | definitions.md, row Resource | No note | Add: "Not a resource of the Model Context Protocol, which a result may carry as a resource link or an embedded resource; a Resource template is not an MCP resource template." | MCP has resources and "resource templates", and a Conduit result is an MCP tool result |
| E16 | definitions.md, rows connection and item that is run | Connection: only "the engine's connection to an implementation item reached by `remote`". Item: "An item reached by `remote` is not run: the engine connects to it." | Connection: MCP's connection of a client to a server; the connection to a `remote` item is the part of a setup. Item: "is not run: the engine reaches it at its Remote's URL (IFC-041)" | MCP: "Each MCP client maintains a dedicated connection with its corresponding MCP server". IFC-017 connects to every item |
| E17 | spec/core (README, capabilities.yaml, operations.yaml, types.yaml), spec/schemas/README.md, the example project's conduit.yaml and contracts.yaml; definitions.md row secret store | "secret store", 13 times in 8 files | "secret Resource", "the type of a secret Resource", or "an implementation of `get-secret.v1`"; the row becomes "Not used." with the three words | After N1. A store is the system outside the engine (SEC-006); in ESO and Dapr a store serves many secrets |
| E18 | SEC-010, SEC-017 and SEC-020, observables | "store" for a secret Resource, about 11 times: "the store's own item", "the store's run", "a store's implementation", "a file store" | "secret Resource", "its implementation"; SEC-020: "each is reached by a package's implementation of the same contract, one that reads a file among them" | The row store; the CSI driver keeps the "external Secrets Store" apart from its providers |
| E19 | definitions.md, row record; REC-001 and CON-007, observables | "a request, a provider call or an event"; "Each such event"; "after the event" | "a request, a provider call or another occurrence"; "Each such occurrence"; "after it" | CloudEvents: "An "event" is a data record expressing an occurrence", so a record is the event |
| E20 | definitions.md, rows Agent and model work; docs/OVERVIEW.md, the families table | "Its model being run is model work"; "An Agent's model being run: a turn started or continued" | "A turn of its model is model work"; "A turn of an Agent's model, started or continued" | Run now names the running of an item, which starts no model work (IMP-002) |
| E21 | SEC-001's observable | "This rule is stricter than OBJ-021" | "This rule is narrower than OBJ-021" | OBJ-021 and the row reference schema say narrower, and the row narrow is in force: C20 removed only the key `narrows` |
| E22 | definitions.md, the history paragraph of its head | About 50 lines on rows that no longer exist | One sentence with a link; the paragraph moves to the [docs log](../ruling-41/log-docs.md) of ruling 41 | A cold reader meets removed features before the first term. This reverses the docs editor's logged choice, so the coordinator strikes it or applies it |
| E23 | spec/core/README.md, the Names table and the export table; spec/core/templates.yaml, head comment | NAM-020 cited for the template `location` | The Names table: "except `location`, which FST-005 names"; drop NAM-020 from the other two | NAM-020 resolves the core package and names no template |

## Considered and kept

Proposals that no check kept, or whose checks found the current name sound. By ruling on this
proposal, the owner also confirms these names.

- `setup-hook.v1`, `retire-hook.v1`: Kubernetes' PostStart runs concurrently with the
  entrypoint and PreStop fails on a stopped container, while Conduit's hooks are parts of a
  setup or retirement and also serve Pools and `remote` items; Helm's pre- and post- form would
  be a coinage. E1 adds the Names-table row.
- `package-source.v1` (proposed: `package-downloader.v1`): Helm 4 renamed its downloader plugins
  to getter plugins (https://helm.sh/docs/plugins/migrate), and Cargo's `Source` trait fits as
  well as Helm's word did.
- `reconcile.v1` (proposed: `settle.v1`): "settle" already has its plain sense in PRV-008 and
  PRV-009, and a JavaScript Promise is no system Conduit adopts; E2 adds one sentence instead.
- `environment` (proposed: `environmentVariables`): Compose's word, chosen under decision 25d,
  and it also serves `command` items, which have no server.json list.
- `idempotentHint`, `destructiveHint`, `openWorldHint` at the root of `spec` (proposed: under
  `annotations`): a `spec.annotations` would sit beside `metadata.annotations`, which has no
  meaning (OBJ-018).
- provider call: follows MCP's "tool call"; E6 relates it to the protocol's request.
- management verb: Kubernetes' API request verb
  (https://kubernetes.io/docs/reference/access-authn-authz/authorization/) in the same
  authorization role; the backticked words of the Secret API reference are HTTP methods.
- item that is run, connect: no standard word covers `command` and `package`, and MCP's "local"
  collides with `local`; E16 widens the row connection.
- claim, of a URI scheme: RFC 8252 says that apps "claim" URIs
  (https://www.rfc-editor.org/rfc/rfc8252.html), and the proposed "serves" is a Conduit term.
- narrower rule (proposed: stricter rule): narrow is in force; E21 makes SEC-001 agree.
- obligation, in IFC-026: spec/README.md's own word for what a promise's `then` states.
- removed, of a package: DL7 and NAM-011 define it.
- part, of a setup or retirement: the standard word "step" went with graphs under ruling 41.

## How to apply

- `tools/set_names.py` does not cover these names. It changes only the three names in
  [names.yaml](../../../spec/names.yaml): the core package's address, `apiVersion` and the
  prefix of the `_meta` keys. Its check reads the `_meta` keys from the interface file, so it
  can report a key left behind after N16 or N17. Every other name is found by search.
- Each rename the owner accepts goes through in one change: the promises under `spec/`; the core
  package (spec/core/ and its README, with the export table); the schemas under spec/schemas/
  and their examples, the example project among them; the rules of `tools/check_schemas.py`
  that name the term; [definitions.md](../../../spec/definitions.md), with the old word added to
  its list of words the promises do not use; spec/README.md; and docs/OVERVIEW.md. Then run the
  checks of [AGENTS.md](../../../AGENTS.md#checks).
- The sentences that call the coined names not final (spec/README.md, definitions.md, the core
  README, and the head comments of secrets.yaml and providers.yaml) then cite the owner's
  ruling.
- Older records keep their words: the records under docs/proposals/, among them ruling 41's work
  order and logs and the principles review, and everything under docs/research/ and
  docs/archive/.
- Order. N1 first. N2 before N10. N3, N9, N10 and N11 touch the same sentences, so apply them
  together.
