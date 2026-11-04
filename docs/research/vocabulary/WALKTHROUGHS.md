# Vocabulary walkthroughs

These examples test names against accepted behavior. They are neither executable tutorials nor
proposed YAML schemas. Names below the five families remain provisional. Read the
[candidate tradeoffs](CANDIDATES.md) alongside each example; accepted definitions are linked through
the [vocabulary map](../../VOCABULARY.md).

## 1. An author publishes directory functionality

An author wants to support attaching an existing folder, creating a new folder, and checking its
layout. A package publishes the relevant contracts and implementation bindings. Another package can
provide the layout check. A Resource template selects those dependencies and composes the required
operations. There is no mandatory primary implementation.

| Thing in the example | Working name | What it is not |
| --- | --- | --- |
| Installable collection with an entry document and public exports | Package | A running Resource or automatically granted access. |
| Meaning and valid configuration of the directory specialization | Resource type contract | The selected executable or a Git repository. |
| Behavior an endpoint offers to authorized callers | Directory capability contract | Authority for every Agent to use the directory. |
| Typed callable behavior for creating a missing directory | Operation contract | One already-running invocation. |
| Selected code/MCP/graph binding satisfying behavior | Implementation / plugin binding | Necessarily a process or the whole package. |
| Reusable construction and runtime/control definition | Resource template | The directory on disk. |
| Pool-owned, independently identified realization | Resource instance | A per-Agent symlink or the backing files themselves. |

A setup graph for **creation** might create the directory and then check its layout. An **attachment**
variant instead verifies that the selected existing directory can be used; it must not silently create
a missing path. This distinction belongs in the selected setup contract, not in a misleading universal
“deploy” command. Registration itself starts neither operation.

Consider the names in a graph without choosing field syntax:

| Step | Input value comes from | Published result used next |
| --- | --- | --- |
| `create-directory` | The execution's captured configuration selects an absolute location. | A directory descriptor, named `directory` by that operation's declared output contract. |
| `check-layout` | An explicit reference selects `create-directory`'s `directory` output. | The check's declared outcome. |

Here **directory** is the author's contract-local output name. It does not come from a universal
variable, a same-named Resource or automatic name matching. The reference selects the whole output
when its nested pointer is omitted; a pointer selects a child only when needed. The runtime validates
the output and the receiving input under their contracts. A typed managed reference and the graph
binding that selects it remain different things.

Later, two Agents can obtain permitted access to that one Resource. Preparing a symlink for each
Agent does not create two extra directory Resources. Retiring the Resource does not silently delete
the original folder or the Agents.

**Naming test:** “The package exports the implementation; the template selects it; setup invokes it;
the Resource owns the resulting managed behavior.” If “plugin” makes any two of these sound identical,
the definition needs tightening. Calling all of them “components” would hide rather than solve that
problem. Topics: V01–V12, V24–V25, V28, V34.

## 2. Shared Linear access from an Euphony Pool

The Pool offers named Resource-template entries for authentication and Linear access. Those entries
are provisioning options, not deployments. Explicit setup provisions the chosen Resources or uses
selected existing instances. The Linear Resource's declared dependency is bound to the chosen
authentication Resource through a Conduit.

An Agent then participates in a Conduit to the Linear capability. Its grant might allow reading
Euphony issues. Applicable requirements narrow the workspace, supported operations and returned
content. Endpoint implementations fulfill their declared duties: for example, presenting permitted
tools on the Agent side and enforcing supported restrictions on the Resource side. Shared service
authentication does not hand the Agent unrestricted credentials.

| Question | Answer in this example | Naming consequence |
| --- | --- | --- |
| What offers the integration? | The Pool-owned Linear Resource. | Resource is not synonymous with MCP tool. |
| What holds reusable authentication? | The Pool-owned authentication Resource, under its custody contract. | Credential, account and grant are distinct. |
| What establishes the logical relationship? | A Conduit instance and the Agent's participation. | Participation is not a socket. |
| What authorizes managed use? | The recorded grant on the selected participation, checked against current governing rules. | A filter or discovered tool does not create authority. |
| What narrows the request/result? | Applicable typed requirements along the selected path. | Filter is a narrowing role, not every possible requirement. |
| What executes an integration behavior? | Selected endpoint implementations; physical helpers may be hosted by their responsible services. | Conduit does not imply its own worker process. |
| What does the Agent see? | Permitted native provider tools as supported by the adapter/catalog contracts. | The author need not invent a generic wrapper tool for every integration. |

If two authorized routes remain, such as Euphony access and a different account's access, the caller
must deliberately select one. It cannot combine the first route's workspace permission with the
second route's update permission. A failed transport can reconnect on the selected path where
supported; it does not silently switch accounts.

If Agent A detaches, Agent B's participation and the shared Resources remain. When the last logical
consumer/peer leaves, that Conduit flow ends. Later participation opens a new Conduit instance;
reusing the Pool Resources is a separate explicit choice.

**Naming test:** compare these sentences without changing the facts:

- “The Agent receives a **grant**, joins a **Conduit**, and invokes the Resource through a selected
  **route** subject to **requirements**.” This keeps authority and connection duties visible.
- “A **valve** opens a **channel** from a **source**.” This is shorter, but leaves unclear who is
  authorized, whether traffic is bidirectional, who enforces result restrictions and what ends.

The second sentence could be an illustration caption. It is not an adequate protocol vocabulary.
Topics: V04–V05, V17–V23, V34–V36.

## 3. Database setup, failure and a later control

A Resource template declares a managed database runtime and a finite setup graph. Its setup obtains
prepared storage, requests an instance-managed start, waits for the necessary usable result, and
runs a migration. The runtime declaration is not itself a live database connection.

| Stage | Accepted behavior | Terminology that needs to survive it |
| --- | --- | --- |
| Before setup | The instance has its resolved definition and initial stored settings. | Definition, configuration, runtime declaration; not “already running.” |
| Startup invocation | A typed operation receives an explicit declared runtime target and prepared inputs. | Operation versus invocation; target versus live output. |
| Database available | The instance manages the service even while setup continues. | Instance runtime versus temporary setup step. |
| Migration fails | Setup is incomplete; the already-started runtime stays managed for inspection or explicit action. | Failed setup is not universal shutdown or rollback. |
| Migration outcome unknown | Evidence retains uncertainty; a retry must use the operation's recovery contract. | Recovery versus blindly repeating an effect. |
| Explicit retry | Still-valid recorded progress can be reused; validity/authority are checked. | Same logical work, retained results and attempts need distinct names. |
| Later supported setting change | A named control validates inputs/revision and applies through its handler. | Control versus arbitrary definition editing. |
| Confirmation | The stored configuration changes only as application is confirmed under the contract. | Confirmed settings versus pending/uncertain request or observed status. |

An unrelated setup branch may still finish if its prerequisites hold. Cancellation stops admission of
ordinary new steps and requests supported cancellation of active work; neither the word “cancelled”
nor the caller losing a connection proves a remote effect was undone. Required cleanup and retained
outcomes still matter.

This example is why “flow” is risky as the sole name for both setup and a Conduit. The database setup
can complete while the service and access relationships keep running. It is also why “controller” is
not interchangeable with control: no universal loop is continually forcing the latest requested
configuration into effect.

**Naming test:** “A setup **procedure**, represented as a dependency **graph**, invokes a startup
**operation** for an instance **runtime**.” This is explanatory prose, not a proposal to require
both Procedure and Graph as separate authored entities. We can choose one primary authoring noun
after comparing its usefulness. Topics: V09–V15, V24–V29.

## 4. Shared Agent history and independent readers

A Pool owns a Resource implementing the first retained-message capability. Two Agents explicitly
participate through a Conduit and read a shared append-only history with independent read positions.
A third participant could join if authorized; a direct conversation and a group discussion need not
be different core families.

| Event | What can be said accurately |
| --- | --- |
| A post is retained | The history operation confirmed the append. This does not say another Agent read it or acted. |
| An Agent has unread entries | Its next normal turn can receive the supported unread notice. This does not start a new model turn automatically. |
| A reader catches up | It retrieves a bounded permitted portion and manages its own read position under the capability. |
| A transport drops | The logical participation may remain; the history stays with its Resource. |
| All participants leave | The live Conduit ends; retained history follows the Resource's separate lifecycle. |
| Another discussion starts | A new Conduit can explicitly reuse permitted history Resources. It does not revive an ended flow's identity. |

“Conversation” is a useful domain term here, but not a synonym for Agent instance. “History” describes
retained data, not model context. “Room” could be a richer Resource capability later; it is not
mandatory. A delivery acknowledgment, a read position and a model response are three different facts.

**Naming test:** if a phrase says “delete the connection,” can a reader tell whether it means detach
a participant, end the Conduit, unregister an object or purge history? If not, choose a precise verb.
Topics: V21, V24–V26, V29, V33.

## 5. The Interface explains a partial failure

A user opens a terminal attached to the reusable root Interface. The Interface observes a Pool with
a ready checkout and a Linear Resource waiting for authentication. Ready Resources can remain usable
through authorized connections; the Pool's incomplete setup is visible.

The Interface combines views from the services responsible for their own records. It can present a
source snapshot and continue from that source's cursor, reporting gaps/staleness under the observation
contract. It must not invent one globally ordered history of everything. An exported telemetry trace
is useful evidence but does not replace the responsible service's confirmed operation outcome.

The user follows the selected adapter's authentication interaction. The Interface can request supported
changes, inspect outcomes and perform authorized recovery. It does not become the credential store,
the sole policy owner or the process hosting every instance. An authorized Agent may use the same
capabilities through its own Conduits.

**Naming test:** “The **frontend** presents an **Interface** that inspects an **execution service's
operation outcome** and an **observation feed**.” These names identify separate responsibilities.
Calling all of them the “host” or “console” would make outage and authority explanations harder.
Topics: V19–V20, V27, V29–V31, V35.

## How to use the examples in review

Start with the authoring example, compare candidate words, then run those words through shared access
and failure. Ask whether the vocabulary still distinguishes a definition from a realization, a
permission from a restriction, and a report from an authoritative fact. Record adopted names in the
[existing vocabulary map](../../VOCABULARY.md), preserving topic anchors.

Only after a cluster is clear should we revise YAML fields, manifests, error messages and API names.
This keeps the naming conversation concrete without using an illustrative spelling as an accidental
protocol decision.
