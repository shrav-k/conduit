# Independent runtimes, management and network Conduits

Research date: 2026-11-01. **Subsequent I1 ruling:** separate management and execution services
and shared policy with instance-owned state are accepted. V0 uses no offline grants. The first
proof uses management and two independent executor processes on one machine, with controlled
connection failures; two-machine validation precedes remote-support claims. The user wants independently
operating objects, eventual placement across systems, Resources from one Pool on different systems
and network Conduits. The earlier combined-host design is a comparison baseline. This report informs the
[implementation review](../proposals/implementation-v0/REVIEW.md#i1--independent-execution-and-management).
It does not select a distributed database, transport, scheduler or new object family.

The investigation combines primary documentation, selected source inspection and papers. No
runtime was built, service started, credentials connected or crash behavior experimentally
verified. Recommendations below are our inferences from that evidence, not capabilities already
implemented by Conduit. Earlier language research remains in [CONFIGURATION](CONFIGURATION.md);
Pi/Xerj and plugin mechanisms remain in [RUST_RUNTIME](RUST_RUNTIME.md).

The follow-up [CLOUD_AUTHORITY](CLOUD_AUTHORITY.md) compares AWS, Google Cloud and Azure through
configuration-change and recovery examples. It separates configuration ownership from policy
ownership and compares a shared-policy/instance-state alternative that the user subsequently
accepted. The user also selected no offline grants in v0 and the one-machine, separate-process
first proof. These rulings do not constitute successful runtime validation.

**Current design boundary:** code-optional MCP bindings and independently accessible component
capabilities are now accepted. Execution services now own Conduit-managed activity separately from
management; existing MCP services retain their external supervision. Shared registration/policy
and instance-owned definitions, confirmed configuration and results are accepted. V0 retains
supervision/dispatched outcomes while blocking new managed requests without verifiable permission.
Offline grants are deferred. The
[design-fit audit](../REVIEW.md#design-fit-audit-code-optional-plugins-and-independent-mcp-access)
records these consequences; other topology candidates remain research comparisons.

## 1. The boundary to establish

The earlier recommendation combined too many choices: a shared registry, a single process owning
all execution, central dispatch, one evidence store and shutdown on host disconnect. None follows
automatically from the five-family model. Separate these responsibilities before choosing binaries.

| Concern | Meaning for Conduit | Independence it should preserve |
| --- | --- | --- |
| Logical ownership | Exactly one Pool owns a Resource instance. | Ownership does not require the Pool and Resource to run on the same machine. |
| Definition authority | A service accepts checked definitions, bindings and supported changes. | Its process need not parent every workload or forward every capability call. |
| Execution placement | A runtime supervises an instance's actual activity at a particular location. | Several execution locations can serve one Pool; one location can serve several Pools. |
| Capability traffic | A selected Conduit carries authorized interactions between endpoints. | Local and network paths can preserve the same capability semantics without pretending their failure modes are identical. |
| Evidence | A component records effects it can actually confirm. | Interface aggregation need not be the only place from which evidence can be recovered. |
| Presentation | Interface attachments inspect and control permitted objects. | A unified view does not require one execution process or a globally current snapshot. |

An independently placed instance does not necessarily need its own daemon, database or network
listener. Conversely, an external database can continue running even when its Conduit driver is
unavailable. Logical identity, execution, backing service and route availability must remain
distinguishable. A Pool is not a Kubernetes Pod, Docker container or machine boundary.

## 2. A concrete distributed example

Consider one Euphony Pool and two Agents. This is a design example, not approved placement YAML:

| Object | Possible placement | What the Pool/Interface retains |
| --- | --- | --- |
| Pool `euphony` | Management record; composition work runs where scheduled | Ownership and named provisioning entries, not a required enclosing process. |
| Git-worktree Resource | Developer laptop | Resource ID, owner and the executor-local workspace association. |
| Authentication + Linear Resources | A separately supervised service machine | Two Resource identities and their restricted connection; service credentials stay under authentication custody. |
| Storage Resource | Driver near a managed database; backing database elsewhere | Configured backing association and operation contract. |
| Search Resource | Compute machine | Search capability and a Conduit to the storage Resource. |
| Agent A / Agent B | Laptop / compute machine | Independent account/private state and executions. |
| Interface | A frontend attached from either machine | Correlated, freshness-labelled views and supported control paths. |

Agent A can reach Linear through a network Conduit without moving the Linear credentials to the
laptop. Both Agents can use the same history Resource without sharing private context. The search
Resource accesses storage through its own Conduit. Losing search must not imply that the Pool,
Linear, history and both Agents have ended.

A local-directory path belongs to its execution environment. A symlink cannot expose another
machine's path by itself: remote file access, a mount or an explicit copy needs a suitable
capability and implementation. Likewise, an OS keychain binding cannot be treated as a portable
secret merely because a Resource reference is portable. Placement compatibility includes available
code, filesystem semantics, credential custody and actual network reachability.

Network Conduits could be implemented by endpoint adapters, a proxy, a relay, or a combination.
That is distinct from the logical Conduit identity. Direct provider-native tools can remain visible
to the Agent while a service-side adapter enforces its selected route and records attribution.
Bypassing a central management process must not mean bypassing Conduit mediation.

## 3. What the reference systems establish

### Tilt: a useful Interface reference, not a complete lifetime model

Tilt exposes inspectable API objects such as commands, image builds, deployments and UI resources;
the same object API supports automation and diagnosis. Its dependency declarations express startup
readiness. Manual trigger mode and `auto_init=False` can separate configuration from execution.
These are useful precedents for visible setup progress, explicit controls and one inspection
surface. [API reference][tilt-api] · [dependencies][tilt-deps] · [manual control][tilt-manual]

At inspected Tilt commit `f3acf27d8459924249776d72a82e1d5d99139c33`, the local command controller's
`TearDown` stops its commands; the process executor handles cancellation by shutting down the
process and escalating where necessary. This is evidence of coupling in that local-command path,
not proof of arbitrary daemon-crash behavior or Kubernetes workload termination.
[Controller source][tilt-controller] · [process executor][tilt-execer]

**Conduit implication:** borrow the relationship between objects, controls, readiness and UI.
Do not adopt Tiltfile re-evaluation/automatic rebuild as Conduit's configuration semantics, or infer
that all independently running Conduit objects should die with an orchestration command.

### Docker and containerd: separate coordination from execution custody

Docker documents live restore as an explicit mode with limits: daemon options and upgrade
compatibility matter, and undrained logging buffers can eventually block a container. Therefore
“the process survives” is weaker than “the workload keeps working normally.”
[Docker live restore][docker-live]

Containerd Runtime v2 separates the manager, a shim endpoint and the execution engine. Its shim
can manage one or multiple containers; the manager does not require one process per logical
container. Startup returns a reconnectable address, and lifecycle operations/events have defined
relationships. These are useful boundaries for a Conduit execution service that owns native
processes independently of a management client's connection. [Runtime v2 design][containerd-v2]

**Conduit implication:** explore a small durable execution supervisor with explicit identity,
reattachment and bounded output handling. Do not copy containerd's cleanup-on-unreachable behavior
or assume its container guarantees automatically apply to macOS native agents, remote MCP servers
or Conduit's deliberate cleanup rules.

### Kubernetes: distributed execution does not require distributed ownership everywhere

Kubernetes separates API/storage/scheduling/controllers from node-local kubelets and container
runtimes. That is a concrete example of coordinating workloads across machines without making
the UI their process parent. It also distinguishes workload networking from the control API.
[Components][k8s-components]

**Conduit implication:** logical authority, execution custody and capability traffic can have
different locations. We can test that separation without installing Kubernetes or reproducing its
scheduler. Its controller model is not permission to recreate a deleted Conduit Resource, replay
an uncertain tool call, restart stopped Agent work or alter fixed instance definitions. Those
remain explicit Conduit contracts. Eventual remote deployment also does not require an immediate
multi-writer registry or one consensus group per object.

### Rust systems worth studying selectively

| Project | Verified pattern | Use for Conduit; boundary to preserve |
| --- | --- | --- |
| containerd Rust extensions | Runtime-v2 shim and task-service interfaces; the async shim exposes startup, wait, cleanup and service creation. | A concrete Rust supervision/reattachment reference. The inspected crate does not establish support for Conduit's native-agent environments. [Repository][rust-shim] · [pinned async interface][rust-shim-source] |
| Oxide Omicron | Nexus coordinates changes through a shared database; sled agents manage resources beside the hardware. Local ledgers support startup without every central service already available. | Study explicit authority versus local execution, bootstrapping and evidence. CockroachDB, rack management and automatic infrastructure reconciliation are not recommended dependencies. [Pinned architecture][omicron] |
| Steno | Action registry, dependency DAG, execution coordinator, saga log and pluggable storage; actions can have compensations. | Useful graph/recovery vocabulary and failure-test inspiration. Automatic unwinding differs from Conduit's accepted retain-partial-work default. Do not substitute one policy for the other. [Pinned interface][steno] · [project status][steno-status] |
| kube-rs | Rust controllers watch objects and requeue reconciliation; authors must implement appropriate idempotent behavior. | Useful watch/reconnect/work-queue patterns, and a future Kubernetes integration. It is not a generic guarantee that replaying Conduit operations is safe. [Controller guide][kube] |
| Restate | Rust server journals durable execution; service code can be deployed separately. Durable-step documentation exposes retry policies and recorded results. | Study invocation identity, waits and recovery. Its service protocol and automatic retry model would be a substantial runtime commitment, not a small plugin substitution. External effects still require a valid recovery/idempotency contract. [Architecture][restate] · [steps][restate-steps] |
| wasmCloud | V1 used a distributed lattice; the v2 migration guide explicitly separates control traffic from local capability calls and makes distributed communication deliberate. | A valuable counterexample to forcing every operation over a broker. Learn capability binding and transport separation without inheriting a Wasm/Kubernetes platform. [V2 migration][wasmcloud] |

The wasmCloud version distinction matters: its older lattice documentation should not be used as
a description of the current v2 architecture. Its published throughput comparisons were not
reproduced here and are not performance estimates for Conduit.

Pi still contributes durable write-before-publication, account/context and observation patterns.
Xerj still contributes a possible independently deployed search/storage implementation. Neither
establishes where Conduit's registry, supervisors or authorization decisions must run. Their
already-pinned investigations remain relevant; no new Pi/Xerj runtime validation was performed.

## 4. Papers and the obligations they reveal

These are deliberately narrow readings relevant to I1, not a new distributed-systems curriculum.

| Primary source | Result or argument used | Consequence for this proposal |
| --- | --- | --- |
| Saltzer, Reed, Clark, *End-to-End Arguments in System Design* (1984) | Some correctness properties need application endpoint knowledge; lower layers alone cannot establish them. | A delivered RPC or healthy TCP connection does not prove a configuration change, durable message or external deployment completed. Preserve target-generated evidence. [Paper][end-to-end] |
| Lee et al., *Implementing Linearizability at Large Scale and Low Latency* (SOSP 2015), RIFL | Duplicate suppression relies on durable completion records, atomicity with mutations and finding those records on retry. | Operation IDs alone are insufficient. The target must recover its result; arbitrary provider calls cannot share a transaction with a Conduit journal. Lost replies can remain uncertain. [Paper, §§3–4][rifl] |
| Garcia-Molina, Salem, *Sagas* (1987) | Long transactions can be decomposed into transactions with compensating actions. | Compensation is a separately defined effect, not universal rollback. A partly established Pool can remain useful; retiring or undoing it must follow the accepted contracts. The original report abstract was accessible; its PDF fetch was blocked. [Authors' report record][sagas] |
| Chandra, Toueg, *Unreliable Failure Detectors for Reliable Distributed Systems* (1996) | Failure information is characterized by accuracy and completeness in an asynchronous crash model. | A timeout is not proof that a remote execution ended. An unreachable executor cannot justify launching a duplicate Agent. This practical inference is narrower than a claim that we implement their consensus model. [Authors' bibliography and abstract][failure-detectors] |
| Ongaro, Ousterhout, *In Search of an Understandable Consensus Algorithm* (2014) | Replicated-log consensus separates leadership, replication and safety, with majority-based agreement. | Consider consensus if the same authority needs replicated availability. Running independent Resources on multiple machines alone does not require Raft; nor would a replicated registry make external effects exactly once. [Paper][raft] |

This reinforces existing no-blind-retry rules. The new implication is where those rules must be
implemented: an execution endpoint needs sufficient retained identity and outcome evidence to
participate in recovery, rather than depending on a transient worker pipe to tell the whole story.

## 5. Candidate arrangements

All options retain the same five families. Process/service names below are implementation roles.
Federation between independently administered registries is a separate decision from distributing
execution under one registry.

| Arrangement | Advantages | Costs and limitations |
| --- | --- | --- |
| A. One management/execution host with child workers | Simple first deployment and local transactions. | Common failure/lifetime boundary; a network proxy later does not automatically remove that coupling. Keep as a comparison baseline, not the accepted direction. |
| B. Shared management/discovery with independent execution services | Instances can run on different machines; Pool ownership stays stable; local and remote calls share contracts; management can reconnect. | Needs authenticated execution identity, local recovery evidence, clear authority ownership and explicit partition behavior. Central management may still be unavailable for new control changes. |
| C. Independently authoritative object services, with Interface aggregation | Maximum decentralization of lifecycle and availability. | Discovery, names, trust, revision authority, cross-object operations and repair become distributed from the outset. Per-object deployment costs may be disproportionate for inert Resources. |

**Subsequent ruling:** the management/execution separation illustrated by B is accepted, with
shared policy and instance-owned state as defined in DESIGN. The [cloud comparison](CLOUD_AUTHORITY.md)
records the authority comparison within separate services; its A–C labels concern authority,
not the deployment arrangements in this table. Do not make a shared registry imply a shared process parent or a
mandatory central data hop. Several execution services can initially run on one laptop. That is a
testable step toward distributed placement, not a promise of federation, transparent migration,
automatic failover or production remote support.

This separation changes the previous combined-host proposal. Merely adding Rust traits inside a single
process would not test the independent lifetimes the user is asking for. Conversely, requiring a
daemon per Pool would couple logical organization to deployment unnecessarily.

### Responsibilities in arrangement B

- A management service maintains the registry and accepts checked definition/management changes.
  One authority per registry can remain the first hypothesis; replication/federation are unchosen.
- An execution service supervises its assigned instance executions, plugin drivers and local
  process handles. It retains enough execution evidence to reconnect after management failure.
  It can host many logical objects without merging their authority, cancellation or private state.
- A Conduit endpoint admits an invocation under the chosen complete path, target and service
  identity. Where supported it can communicate with a remote endpoint without routing payloads
  through the Interface or global registry service. A proxy remains valid when a capability needs it.
- Interface observation assembles registry information and execution reports. It shows each
  report's source, generation, freshness and gaps; it does not claim a simultaneous global snapshot.

Placement can be explicit before any scheduler exists. The protocol needs an execution-location
identity separate from a mutable address, but that does not require a sixth authored family.
Whether location is deployment configuration, an environment capability or another existing
Resource type remains open; this report does not invent YAML fields for it.

### Authority is more important than process count

Each mutable fact needs one authoritative owner. A runner's process observation and the last
confirmed configuration are different facts, not competing writable copies of one spec.
If management owns configuration, a target's durable application receipt and the registry's eventual
commit cannot be assumed atomic. Reserve/reconcile the operation rather than allow another writer
to treat the old configuration as definitely current. The cloud follow-up motivated the now-accepted arrangement keeping
confirmed configuration and operation records together at an instance authority, with management
aggregating observations. Store placement and cross-authority coordination remain implementation work.

A coordinator generation helps reject stale requests only if the execution endpoint checks it.
It does not fence a side effect already admitted by a third-party service. Do not assign the same
exclusive execution to another service while its old ownership is uncertain. Initial fixed
placement plus explicit recovery is much smaller than automatic rescheduling.

## 6. Disconnection and autonomy need explicit semantics

Three promises cannot be conflated: keeping a process alive, continuing existing work, and admitting
new effects. A model stream already in flight may continue while the next privileged tool call
waits. A database can remain running while a configuration mutation is unavailable. Provider or
executor loss affects its dependents, not every object in its Pool.

The user selected no offline grants in v0. The following comparison preserves the rationale
for requiring verifiable current permission for new managed requests:

| Policy candidate | What it permits | What it cannot promise |
| --- | --- | --- |
| Require reachable current authority for affected operations | Keeps strict current-access checking; unrelated execution can continue. | New protected operations may wait during an authority outage. |
| Explicit bounded offline delegation | An endpoint can admit specified effects while its grant remains valid under an agreed lease/epoch contract. | Immediate withdrawal across a disconnected network. Deferred beyond v0; introducing it would require a future ruling. |
| Independent local authority for an object | That authority can govern its own operations while another registry is unavailable. | A single global order or instant cross-authority revocation without additional coordination. |

The accepted v0 baseline retains supervision and outcomes of already-dispatched work while
blocking new managed requests whose current permission cannot be established. Existing exclusive
ownership, stop/revocation and delivery checks remain applicable. Do not silently introduce cached
grants to claim availability. Concrete crash survival still requires experimental evidence.

Other required distinctions:

- Frontend exit is not executor exit. Management restart is not stop-all. Executor stop, whole
  deployment stop and object retirement need distinct explicit actions.
- Network failure does not remove logical participation. Do not end a Conduit merely because its
  last *reachable* participant is temporarily unreachable. Lease-based abandonment would need
  separate agreed semantics; the last *detached* participant still ends it.
- Reconnect to verified execution and reconcile retained results. Do not launch a new Agent turn,
  replay an uncertain write, change account or select an alternate route as a recovery shortcut.
- Local evidence needs bounds and backpressure if aggregation is down. Preserve essential outcomes;
  report gaps in optional telemetry. A full logging pipe can otherwise undermine claimed autonomy.
- Creation on another machine is not migration of live handles. Exact code, backing identity and
  private state must be compatible before supported replacement/reuse is attempted.

## 7. Accepted scope for experiment 002

Keep its existing synthetic graphs, controls, permissions and exact-worker tests. Add **two
independently supervised execution services on one machine**, one management service and an
Interface client. One fixture Pool owns Resources on both execution services. Use a genuine
serialized transport boundary or a faithful fault-injected transport; no shared mutable Rust
references across that boundary. Production remote authentication remains a later gate.

Compare management loss, one executor's loss, worker loss, capability-route interruption and
observation-export failure independently. Test a cross-executor Resource-to-Resource Conduit as
well as Agent access. Verify that management outage does not itself kill surviving processes;
report which new operations are blocked for lack of authority. Reconnect without new instance
identities or duplicate work, and reject delayed requests from an obsolete coordinator generation.

This layout is now accepted for the first proof; its behavior remains untested. Two-machine
validation follows before claiming remote support. The previous combined-host layout can still
be measured as a baseline if useful. Test records should state the deployed
arrangement and actual guarantees, rather than summarizing every failure as “host restart.”

## 8. What to review next

I1 is settled for the first implementation; preserve the distinction among its accepted choices:

1. **Execution custody — accepted:** separate execution services supervise Conduit-managed Agents
   and Resources; management coordinates configuration/control and the Interface presents them.
   Each executor may host many instances; existing external MCP services retain their supervision.
2. **Authority — accepted:** shared registration/template and access-policy authority; one
   designated service owns each instance's definition, confirmed configuration/revision and results.
   A unified Interface aggregates the sources without becoming another writable authority.
3. **Outage admission — accepted:** no offline grants; retain supervision and dispatched outcomes,
   while new managed requests require verifiable current permission.
4. **First proof — accepted:** management and two independent executors as separate processes
   on one machine, with controlled connection failures. Validate across two machines before
   claiming remote support.

I3's worker packaging and I4's persistence choices must preserve these accepted boundaries. Tokio, exact artifacts,
finite graphs, MCP-first capability access and Interface observability remain useful under all
three arrangements. Accepted outage policy still needs implementation evidence and does not
choose storage technology.

## Source scope and reproducibility

Primary web pages were consulted on the research date; their current documentation can change.
Tilt was inspected at the commit below without running it. Selected raw source files from the
other repositories were fetched at exact commits; entire projects were not built or audited.
Paper citations use original publication years, not search-engine crawl/publication estimates.

| Source snapshot | Files inspected |
| --- | --- |
| Tilt `f3acf27d8459924249776d72a82e1d5d99139c33` | Local command controller/executor; API and dependency/trigger docs. |
| containerd Rust extensions `96f196dd5aad5e4789b74dba1d313c805fb742e5` | README, shim `lib.rs`, async shim module, especially startup/service/wait/shutdown. |
| Omicron `d47c6f61f20aaedec65b829c60c2025bc70dc245` | `docs/control-plane-architecture.adoc`, particularly component ownership, data flow, ledgers and cold start. |
| Steno `9c9f511d7dbabce23c0a7de52e3f1b4c2309e2ac` | `src/lib.rs` exported DAG/action/store/coordinator surface; public project status separately. |

[tilt-api]: https://api.tilt.dev/
[tilt-deps]: https://docs.tilt.dev/resource_dependencies.html
[tilt-manual]: https://docs.tilt.dev/manual_update_control.html
[tilt-controller]: https://github.com/tilt-dev/tilt/blob/f3acf27d8459924249776d72a82e1d5d99139c33/internal/controllers/core/cmd/controller.go#L89-L107
[tilt-execer]: https://github.com/tilt-dev/tilt/blob/f3acf27d8459924249776d72a82e1d5d99139c33/internal/controllers/core/cmd/execer.go#L195-L278
[docker-live]: https://docs.docker.com/engine/daemon/live-restore/
[containerd-v2]: https://github.com/containerd/containerd/blob/main/docs/runtime-v2.md
[k8s-components]: https://kubernetes.io/docs/concepts/overview/components/
[rust-shim]: https://github.com/containerd/rust-extensions/tree/96f196dd5aad5e4789b74dba1d313c805fb742e5
[rust-shim-source]: https://github.com/containerd/rust-extensions/blob/96f196dd5aad5e4789b74dba1d313c805fb742e5/crates/shim/src/asynchronous/mod.rs
[omicron]: https://github.com/oxidecomputer/omicron/blob/d47c6f61f20aaedec65b829c60c2025bc70dc245/docs/control-plane-architecture.adoc
[steno]: https://github.com/oxidecomputer/steno/blob/9c9f511d7dbabce23c0a7de52e3f1b4c2309e2ac/src/lib.rs
[steno-status]: https://github.com/oxidecomputer/steno
[kube]: https://kube.rs/controllers/intro/
[restate]: https://docs.restate.dev/foundations/key-concepts
[restate-steps]: https://docs.restate.dev/develop/ts/durable-steps
[wasmcloud]: https://wasmcloud.com/docs/migration/
[end-to-end]: https://web.mit.edu/saltzer/www/publications/endtoend/endtoend.pdf
[rifl]: https://web.stanford.edu/~ouster/cgi-bin/papers/rifl.pdf
[sagas]: https://www.cs.princeton.edu/research/techreps/598
[failure-detectors]: https://www.cs.cornell.edu/info/people/sam/FDpapers.html
[raft]: https://raft.github.io/raft.pdf
