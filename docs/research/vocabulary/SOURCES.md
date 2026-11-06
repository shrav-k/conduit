# Sources and evidence boundaries

> Not binding. This research predates the specification in [`spec/`](../../../spec/README.md); where it says accepted, current, SPEC.md or DESIGN.md, it means the documents as they stood before 2026-11-05.

Research date: **2026-11-04**. This packet examines vocabulary and the behavior a name suggests,
not implementation fitness, security certification or popularity. The [precedents](PRECEDENTS.md)
link each technical comparison to its primary source. The [candidate table](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/CANDIDATES.md) and
[walkthroughs](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/WALKTHROUGHS.md) are our analysis against Conduit's accepted model.

## What was inspected

- 54 primary documentation/specification/source URLs were retrieved for this pass. Relevant sections
  were inspected for terminology; this was not a complete audit of every specification.
- Ten files were inspected in clean, pinned Pi and Xerj checkouts. Their commit identities and hashes
  are separate from the web retrievals. No latest-release claim is made from these pins.
- Earlier Conduit research supplied the comparison questions and historical context. Its experiments
  were not rerun, and its conclusions were not silently converted into new naming decisions.

[retrievals.json](retrievals.json) records requested/final URLs, retrieval date, HTTP status, content
type, response byte count, SHA-256 of the retrieved response bytes, and extracted-text byte count.
All 54 retrievals returned HTTP 200. A response hash identifies what was retrieved; it does not prove
publisher authenticity, semantic stability or conformance. Extracted temporary copies were used for
reading; source-page bodies are not vendored in this packet. Hashes are not a promise that mutable
web pages can be recovered byte-for-byte later.

[source-pins.json](source-pins.json) records repository, exact commit, file path, content hash and
source URL for the local inspections. Both checkouts had no reported changes. The selected Pi pin
matches the original Pi research baseline; later reports inspected other pins, which retain their
own identities. Neither pin is relabeled as the newest upstream version.

## Version and redirect notes

- RFC 7950, 8342, 9293 and 9000, JSON Schema 2020-12, CWL v1.2, Matrix v1.16,
  OpenTofu v1.11 and Kafka 4.1 are the identified document baselines, not claims of latest versions.
- TOSCA is **2.0 CS01**. Committee-specification status is kept in its name rather than implied to
  be a later ratified release.
- The MCP page is the design's **2026-07-28** baseline. Pi's client is a separately inspected
  implementation; terminology resemblance does not establish version compatibility.
- The containerd and CEL URLs are commit-pinned. OCI `main` and documentation using `latest`,
  `stable` or unversioned routes are moving sources inspected on the date above.
- The older NATS concept URLs redirected to the current stream and pull-consumer tutorials.
  Comparisons use what those returned pages actually describe; they are not presented as unchanged
  copies of the old conceptual reference. Prefect and Google Cloud also redirected to canonical
  hosts/paths. Requested and resolved URLs remain in the manifest.

## Web source inventory

The key matches `retrievals.json`; these are citation keys for this report, not protocol identifiers.

| Key | Primary source | Naming area |
| --- | --- | --- |
| `kubernetes-objects` | [Kubernetes](https://kubernetes.io/docs/concepts/overview/working-with-objects/) | Objects, manifests, spec/status |
| `kubernetes-controllers` | [Kubernetes](https://kubernetes.io/docs/concepts/architecture/controller/) | Controllers and reconciliation |
| `kubernetes-rbac` | [Kubernetes RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) | Roles, bindings and authority |
| `docker-images` | [Docker](https://docs.docker.com/get-started/docker-concepts/the-basics/what-is-an-image/) | Images, layers, reusable artifacts |
| `compose` | [Docker Compose](https://docs.docker.com/compose/intro/compose-application-model/) | Services, networks, volumes, projects |
| `nomad` | [Nomad](https://developer.hashicorp.com/nomad/docs/glossary) | Jobs, allocations, placement |
| `tilt` | [Tilt](https://docs.tilt.dev/tiltfile_concepts.html) | Development resources and reload |
| `helm` | [Helm](https://helm.sh/docs/topics/charts/) | Charts and packaging |
| `containerd` | [containerd](https://raw.githubusercontent.com/containerd/containerd/8a9c72699b5a29daa94cbf1bfe270f1bd1b52d12/SCOPE.md) | Primitives and implementation boundaries |
| `oci` | [OCI](https://raw.githubusercontent.com/opencontainers/runtime-spec/main/spec.md) | Runtime, bundle and lifecycle |
| `opentofu` | [OpenTofu](https://opentofu.org/docs/v1.11/language/providers/requirements/) | Dependencies, local names, source identities |
| `pulumi` | [Pulumi](https://www.pulumi.com/docs/iac/concepts/inputs-outputs/) | Inputs, outputs and dependency flow |
| `wit` | [WIT](https://component-model.bytecodealliance.org/design/wit.html) | Contracts, worlds, interfaces, handles |
| `smithy` | [Smithy](https://smithy.io/2.0/spec/service-types.html) | Services, operations and shapes |
| `yang` | [YANG](https://www.rfc-editor.org/rfc/rfc7950.html) | Modules, groupings, operations, state |
| `nmda` | [NMDA](https://www.rfc-editor.org/rfc/rfc8342.html) | Configured, intended and operational state |
| `tosca` | [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/cs01/TOSCA-v2.0-cs01.md) | Topology, relationship, requirement, workflow |
| `cwl` | [CWL](https://www.commonwl.org/v1.2/Workflow.html) | Processes, steps and data links |
| `tekton` | [Tekton](https://tekton.dev/docs/pipelines/pipelines/) | Pipelines and tasks |
| `tekton-runs` | [Tekton](https://tekton.dev/docs/pipelines/taskruns/) | Definition versus execution |
| `argo` | [Argo Workflows](https://argo-workflows.readthedocs.io/en/latest/workflow-concepts/) | Live workflows and templates |
| `temporal-workflow` | [Temporal](https://docs.temporal.io/workflow-execution) | Definitions and durable executions |
| `temporal-activity` | [Temporal](https://docs.temporal.io/activity-execution) | Activities, workers and retries |
| `prefect` | [Prefect](https://www.prefect.io/docs/v3/concepts/flows) | Flows and flow runs |
| `otel-components` | [OpenTelemetry](https://opentelemetry.io/docs/collector/components/) | Receiver/processor/exporter roles |
| `otel-signals` | [OpenTelemetry](https://opentelemetry.io/docs/concepts/signals/) | Traces, metrics and logs |
| `nats-streams` | [NATS JetStream](https://docs.nats.io/learn/jetstream/your-first-stream) | Subjects and retained streams |
| `nats-consumers` | [NATS JetStream](https://docs.nats.io/learn/jetstream/pull-consumers) | Consumer and retrieval lifetime |
| `kafka` | [Kafka](https://kafka.apache.org/41/getting-started/introduction/) | Topics, partitions and events |
| `amqp` | [RabbitMQ / AMQP](https://www.rabbitmq.com/tutorials/amqp-concepts) | Exchanges, queues and bindings |
| `tcp` | [TCP](https://www.rfc-editor.org/rfc/rfc9293.html) | Connection and byte-stream semantics |
| `quic` | [QUIC](https://www.rfc-editor.org/rfc/rfc9000.html) | Connections and multiplexed streams |
| `signal` | [Signal Sesame](https://signal.org/docs/specifications/sesame/) | Encryption sessions, devices, mailboxes |
| `matrix` | [Matrix](https://spec.matrix.org/v1.16/client-server-api/) | Rooms, timelines, state and ephemeral events |
| `envoy` | [Envoy](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/intro/terminology) | Relative endpoint/network roles |
| `cilium` | [Cilium / Hubble](https://docs.cilium.io/en/stable/overview/intro/) | Network and security observability |
| `aws-iam` | [AWS IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html) | Principals, policies, boundaries |
| `gcp-iam` | [Google Cloud IAM](https://docs.cloud.google.com/iam/docs/allow-policies) | Allow policies and role bindings |
| `azure-rbac` | [Azure RBAC](https://learn.microsoft.com/en-us/azure/role-based-access-control/overview) | Principals, roles and scopes |
| `cedar` | [Cedar](https://docs.cedarpolicy.com/policies/syntax-policy.html) | Applicability, effects and conditions |
| `opa` | [Open Policy Agent](https://www.openpolicyagent.org/docs/policy-language) | Evaluation and policy decisions |
| `mcp` | [MCP](https://modelcontextprotocol.io/specification/2026-07-28/architecture) | Hosts, clients and servers |
| `json-schema` | [JSON Schema](https://json-schema.org/draft/2020-12/json-schema-core) | Schema, instance, vocabulary and dialect |
| `cue` | [CUE](https://cuelang.org/docs/reference/spec/) | Constraints and unification |
| `cel` | [CEL](https://raw.githubusercontent.com/cel-expr/cel-spec/21ec6f4e52e8d694687ff9ed7408d3eccbb955d7/doc/langdef.md) | Expressions and evaluation |
| `dhall` | [Dhall](https://docs.dhall-lang.org/discussions/Safety-guarantees.html) | Configuration language and imports |
| `node-red` | [Node-RED](https://nodered.org/docs/user-guide/concepts) | Nodes, wires and reusable subflows |
| `nifi` | [Apache NiFi](https://nifi.apache.org/docs/nifi-docs/html/overview.html) | Processors, connections and data flow |
| `gstreamer` | [GStreamer](https://gstreamer.freedesktop.org/documentation/application-development/basics/elements.html) | Elements, pads and direction |
| `rust` | [Rust](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) | Package, crate and module |
| `tokio` | [Tokio](https://docs.rs/tokio/latest/tokio/runtime/index.html) | Runtime, task and driver |
| `dapr` | [Dapr](https://docs.dapr.io/developing-applications/building-blocks/actors/actors-overview/) | Actor identity and activation |
| `orleans` | [Orleans](https://learn.microsoft.com/en-us/dotnet/orleans/overview) | Grains, silos and virtual lifetime |
| `extism` | [Extism](https://extism.org/docs/concepts/plug-in-system/) | Plugins and extension points |

## Pinned source inventory

| Repository and revision | File | Inspected terminology |
| --- | --- | --- |
| `earendil-works/pi` at `0441483926f2` | [packages/coding-agent/docs/packages.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/coding-agent/docs/packages.md) | Distribution of several contribution kinds |
| `earendil-works/pi` at `0441483926f2` | [packages/coding-agent/docs/extensions.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/coding-agent/docs/extensions.md) | Executable extensions, events, tools and commands |
| `earendil-works/pi` at `0441483926f2` | [packages/coding-agent/docs/skills.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/coding-agent/docs/skills.md) | Instructions and activation |
| `earendil-works/pi` at `0441483926f2` | [packages/ai/README.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/ai/README.md) | Models, providers and context |
| `earendil-works/pi` at `0441483926f2` | [packages/mcp/README.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/mcp/README.md) | Client/transport/tool distinctions |
| `earendil-works/pi` at `0441483926f2` | [packages/telemetry/README.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/telemetry/README.md) | Telemetry contracts and contexts |
| `earendil-works/pi` at `0441483926f2` | [packages/durable/README.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/durable/README.md) | Harness, task, transcript, run and submission |
| `earendil-works/pi` at `0441483926f2` | [packages/chord/README.md](https://github.com/earendil-works/pi/blob/0441483926f283e42ca44eb7b4a46552e9469d6f/packages/chord/README.md) | Object, reference, principal, host and link |
| `xerj-org/xerj` at `63b6aef5e66e` | [engine/crates/xerj-wasm/src/lib.rs](https://github.com/xerj-org/xerj/blob/63b6aef5e66e14df12a18929caf704e550e2c4f5/engine/crates/xerj-wasm/src/lib.rs) | Native transform contract despite crate name |
| `xerj-org/xerj` at `63b6aef5e66e` | [engine/crates/xerj-wasm/src/pipeline.rs](https://github.com/xerj-org/xerj/blob/63b6aef5e66e14df12a18929caf704e550e2c4f5/engine/crates/xerj-wasm/src/pipeline.rs) | Ordered stages and error policies |

## Relationship to the earlier research

The corpus is broad, but “all frameworks” is not a claim that every citation in every historical
report was freshly audited. The following remain useful reference collections. Their dates/pins and
probe results remain historical unless explicitly refreshed above.

| Earlier report | How it informs this inquiry |
| --- | --- |
| [Language/protocol foundations](../language-protocol-foundations/SOURCES.md) | Broader IDL, schema and workflow comparisons, including Open Workflow/Serverless Workflow and Amazon States Language. This pass focuses on vocabulary that affects existing V topics. |
| [Graph authoring review](../language-protocol-foundations/GRAPH_AUTHORING_REVIEW.md) | Accepted graph choices and their costs; naming is not permission to add expressions, loops or implicit merging. |
| [Configuration](../CONFIGURATION.md) | Docker/Kubernetes and language/paper findings behind the accepted YAML/data model. |
| [Ecosystem organization](../ECOSYSTEM_ORGANIZATION.md) | Wasmtime, uv, OCI and repository/publication boundaries; no new distribution policy follows from naming a package. |
| [Authorization](../AUTHORIZATION.md) | Zanzibar, Macaroons/Biscuit and cloud-policy evidence; our accepted grant model is not renamed into their semantics. |
| [Distributed runtime](../DISTRIBUTED_RUNTIME.md) | Runtime/ownership/outage research and systems papers; no process layout or remote-support claim is added here. |
| [Observability](../OBSERVABILITY.md) | Cloud and OpenTelemetry comparison; the Interface's observation obligations remain authoritative. |
| [Pi connectors](../PI_CONNECTORS.md), [Pi runtime](../PI_RUNTIME.md), [Rust runtime](../RUST_RUNTIME.md) | Detailed historical implementation evidence, beyond the naming-focused source excerpts inspected here. |

## Limits and follow-up evidence

This is qualitative research. No runtime experiments, compatibility probes, performance tests or
user-comprehension study were run for the naming packet. No project was adopted as an implementation
dependency. The repository's local link checks cannot prove external documentation will remain live.

The next useful evidence is a walkthrough with a new author: can they identify the package, exported
contract, implementation selection, setup invocation, live instance, access grant and observed outcome
without relying on unstated context? Test several candidates against the same examples before selecting
field spellings. Do not infer a winner merely from how many projects use a word.
