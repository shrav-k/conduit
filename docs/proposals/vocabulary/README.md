# Final vocabulary — accepted

Status: ruling 17: in force; amended by 27 (grants); amended by 28 (grants); amended by 29 (grants); amended by 42 (names); ruling 18: in force; amended by 42 (names); amended by 55 (A6) (ruled by the owner on 2026-11-05). Holds now: [IN_FORCE.md: 1. Purpose, scope and principles](../../../IN_FORCE.md#1-purpose-scope-and-principles), [IN_FORCE.md: 2. The specification: form, layers and conformance](../../../IN_FORCE.md#2-the-specification-form-layers-and-conformance), [IN_FORCE.md: 3. Vocabulary and the project's names](../../../IN_FORCE.md#3-vocabulary-and-the-projects-names), [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission); [IMP-001 to IMP-003, IMP-007, IMP-008](../../../spec/conventions/implicit.yaml), [OBJ-012 to OBJ-014](../../../spec/promises/objects.yaml). This record says why.

The owner
ruled that the vocabulary is decided now and applied as each area is reworked, so that names and
behavior are fixed together and the text is rewritten once. The names are applied in the
specification. The older documents keep the earlier words. Where this record says "this
proposal", it means this record as it was put to the owner.

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ: [`spec/definitions.md`](../../../spec/definitions.md) is the current
vocabulary, and the tables below are the record. Amended by:

- ruling 27: a selection is a qualified reference, with no alias; the manifest is a project's
  one file, which lists the packages it uses;
- rulings 28 and 29, the [consolidation rulings](../consolidation/README.md#rulings): a grant is
  the access an open Conduit carries, so "issue" and "revoke" are open and close; "right to
  issue" is the right to open; "provision" is setting up an entry's Resource; "location
  capability" is a requirement on placement; the lifecycle states are registered, ready and
  retired; "owner" is used only for a Pool; a control is an operation; and a record is a log
  entry only;
- ruling 31: an implementation is what does the work behind an operation, since a runtime is a
  pair of operations; a contract is a document with a kind of its own;
- ruling 32: "registered" names a lifecycle state, and an object that has been registered
  "exists"; "create" is not used for an operation; a location's facts are stored, not reported;
  the pass that applied this ruling also retired the word "declaration", so what is submitted is
  a document;
- ruling 33: where a record or an observation was made is its origin, and `source` is the key
  of a value reference only; there is a fifth kind of contract, `Schema`.

The notes above the tables below name the rows these rulings replaced. They are not a complete
comparison: read `spec/definitions.md` for any word.

It combines three inputs:

- the [candidate research](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/CANDIDATES.md) for topics V01 to V37;
- the reviewer's [simplification pass](../../research/vocabulary/SIMPLIFICATION.md) and review of
  pull request 5, which proposes dropping about 20 terms;
- the owner's rulings of 2026-11-05 on the [layers](../../AUDIT.md#layered-claim),
  [instantiation](../instantiation/README.md), [placement](../placement/README.md) and the
  [purpose](../../AUDIT.md#purpose).

## Principles

1. One word for one idea, and one idea for one word.
2. Plain technical words for authoring and operation; flow imagery only around Conduits. Both the
   research and the reviewer reached this.
3. The specification's words are decided here. Words that belong to one type, or to the runtime
   profile, are settled with that type or profile and are listed at the end.

## Vocabulary

Source column: **R** research recommendation, **S** simplification pass, **A** follows from a
2026-11-05 ruling or from this audit.

### Objects and documents

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| family | One of the five: Interface, Agent, Resource, Pool, Conduit. Settled. | — | — |
| template | A reusable definition from which instances are made. | — | R |
| instance | One identified realization of a template. It can be stopped and still exist. | — | R |
| type | What specializes a family, such as a Resource type or an Agent type. Its defining document is a type contract. | "type contract" as the everyday noun | R |
| definition | The fixed part of an instance: its type, graph, runtimes and controls. | — | R |
| configuration | The stored settings of an instance that controls can change. | — | R |
| status | What the system reports about an instance. | — | R |
| input | A value a template declares that its instantiator supplies. Also a value an operation takes. | "port" | A, S |
| argument | The value supplied for an input. Name, owner and placement are standard arguments. | — | A |
| instantiate | Create an instance from a template and its arguments. | "draft editing" | A |
| reference | A declared pointer: an object reference, which is a kind and a name, or a value reference inside a graph. | "form" inside references | S, A |
| handle | A live value passed between operations, such as a running service or a protected login. | "managed reference", "runtime target", "descriptor" | S |
| ID | The stable identity a reference resolves to. | "resolved object binding" | S |
| name | The readable, changeable lookup for an object. | — | R |

### Packages and implementations

> Later rulings replaced three rows below. selection: a qualified reference, the package's
> address, `#` and the export's name, with no alias (ruling 27; NAM-002, NAM-018). manifest: the
> one file of a project, which lists each package the project uses; a package's manifest also
> states its exports (ruling 27; NAM-006). implementation: what does the work behind an
> operation only (ruling 31; TPL-003).

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| package | What an author ships. | "bundle" | S |
| manifest | The one document in a package that declares what it exports and depends on. | "entry document" | R |
| export | Something a package makes public: a contract, a template or an implementation. | "contribution" | S |
| implementation | What does the work behind an operation, a runtime or a control: code, an existing MCP service or a graph. | "plugin" as a primitive, "implementation binding", "driver", "handler" | S |
| selection | A template's choice of an implementation, by package alias and export name. | "implementation binding" | S |
| contract | A published definition with meaning: of an operation, a requirement, a capability or a type. | — | R, S |
| schema | The shape of valid data. A contract can contain schemas. | — | R, S |
| capability | What an instance offers to others. Offering is not permission. | — | R, S |
| implementation interface | The shared wire between an engine and an implementation backed by code or an MCP service. | "Conduit profile", "plugin interface" | A |

"Plugin" remains a casual word for a package with implementations.

### Operations and graphs

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| operation | A callable action with typed inputs and outputs. | — | R |
| invocation | One request of an operation. | "execution" of an operation | S |
| attempt | One try at an invocation. | — | R, S |
| outcome | How an invocation ended, from a closed set that includes unknown. | "result" in that sense | A |
| output | A value an operation or graph produces. | "port", "public result" | S |
| graph | A finite set of steps and their dependencies. | — | R |
| step | One operation call in a graph. | "node" | R |
| depends on | The relation that orders steps. | "prerequisite" | A |
| value binding | How a step's input gets its value: literal, reference, object or list. | "binding" in every other sense | R, S |
| control | A named, typed change to an instance's configuration. | — | R |
| revision | The counter a change is checked against. | — | R |

### Lifecycle verbs

> Later rulings replaced three rows below. provision: no longer a verb; a Pool's entries are
> instantiated with the Pool, and what was provisioning is setting up the entry's Resource
> (ruling 29; POOL-002, POOL-013). register: takes instance documents as well as templates, and
> registering an instance document instantiates its template (ruling 32). declare lost: an
> operation reserved to the root Interface, not "the owner" (ruling 28; PLC-017).

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| register | Record a template. Has no other effect. | — | R |
| set up | Run an instance's setup graph. The noun is setup. | "establish", "prepare" | S |
| provision | A Pool instantiating and setting up one of its entries. | — | R, S |
| use existing | Adopt something that already exists, in place of creating it. | "attach" for Resources | S |
| start, stop | Begin and halt an instance's ongoing activity. A stopped instance still exists. | — | R, S |
| retire | End an instance for good, with its shutdown duties. | — | R |
| unregister | Remove the record, once retirement is confirmed. | "delete" unqualified | R |
| purge | Remove retained data. | — | R, S |
| cancel | Ask an invocation in progress to stop. | — | R |
| connect | A frontend joining an Interface. | "attach" for frontends | S |
| declare lost | The owner recording that a location can never confirm shutdown. | — | A |

### Relationships and authority

> Later rulings replaced four rows below. grant: not used; the access an open Conduit carries
> is the operations it lists, and it is opened and closed, not issued and revoked (rulings 27
> and 28; ACC-003, ACC-011). open, close: to open a Conduit is to set it up, and to close it is
> to retire it (ruling 29). owner: only the Pool that owns a Resource; no promise uses the word
> for a person (ruling 28; POOL-001). entry: a named nested instantiation of a Resource
> template, whose Resource is instantiated with the Pool (ruling 29; POOL-002, POOL-012).

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| consumer, target | The two ends of a Conduit. | "endpoint", "provider", "peer" | A |
| open, close | Create and end a Conduit. | "attach", "join", "leave", "participation" | A |
| grant | Recorded permission on a Conduit. It is issued and revoked. | "delegation" and "issuance" as nouns | S |
| requirement | Something that must hold for a relationship or its use. Its parts are requirements too. | "constituent", "obligation" | S |
| filter | A requirement that only narrows what may pass. | — | R |
| policy | A requirement applied to chosen actors or targets. A limit on an Agent is a policy on that Agent. | "requirement binding", "governing binding", "Agent limit" | S, A |
| route | The Conduit, or chain of Conduits, one request travels through. | "access path" | S |
| owner | The Pool that owns a Resource. Also the person who owns the installation. | — | R |
| entry | A named template a Pool can provision. | "catalog" | R |

A Conduit is pairwise (decision V3): one consumer, one target.

### Placement

> Later ruling replaced one row below. host: one of a location's stored facts; the location
> does not report it (ruling 32; PLC-008).

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| location | Where an Agent or Resource runs: a Resource of the location type in the system Pool. | "execution location", "executor" in the language | A |
| placement | An instance's reference to its location. | — | A |
| host | The machine a location reports it is on. | — | A |

### Records and layers

> Later rulings replaced three rows below. record: a log entry about a request or an event, and
> nothing else (ruling 28; REC-002). observation: reported with the time it was made and its
> origin, not its source (ruling 33; OBS-002). specification: it has a third layer, the
> implementation interface, and binds every engine (ruling 31).

| Term | Meaning | Replaces or drops | Source |
| --- | --- | --- | --- |
| record | What is kept about an invocation: who, what, the outcome. | "evidence" | A |
| observation | A report of state, with its source and freshness. | — | R |
| specification | The language and the shared conventions: the documents that bind every implementation. | "normative surface" | A |
| shared convention | One of the few behaviors every part must agree on. | — | A |
| runtime profile | The recorded behavior of one engine beyond the specification. | — | A |
| promise | One numbered, testable statement in the specification. | — | A |
| vector | One data file that tests promises. | "fixture" in that sense | A |

## Kept distinct

The reviewer argues these look repetitive but behave differently when something fails. This
proposal agrees with every one:

- provision, start, stop, unregister, purge
- contract version, configuration revision, package release
- contract and schema
- capability and grant
- snapshot, feed, cursor
- invocation context (trusted facts) and invocation metadata (declared values)

## Where this differs from the simplification pass

| Point | Simplification pass | This proposal | Why |
| --- | --- | --- | --- |
| Conduit verbs | join and leave | open and close | Follows from the pairwise Conduit (V3). If Conduits stay multi-party, join and leave stand. |
| "reference" | For graphs only | For any declared pointer, qualified as object reference or value reference | Registry lookups by kind and name are also called references and need a word. |
| "execution" | Dropped for invocation and attempt | Dropped for operations; kept inside the Agent type for an Agent's running process | An Agent's process is not an invocation. |
| "form" | Drop it | Drop it; `from` marks an instance (V2) | A document must still say whether it is a template. |
| Retire into stop | Undecided | Keep both | A stopped instance can start again; a retired one cannot. |

## Decisions

| ID | Decision | Outcome |
| --- | --- | --- |
| V1 | Adopt the simplification pass as the base, with the differences above. | Accepted. |
| V2 | How a document says it is a template once "form" is dropped. | Accepted: one kind per family. A document that declares inputs and has no `from` is a template; a document with `from` is an instance of it. Two kinds per family was rejected as cumbersome, since the template flows into the instance. See the [instantiation record](../instantiation/README.md). |
| V3 | The words for a Conduit's ends and verbs. | Accepted: a Conduit is pairwise, with consumer, target, open and close. |
| V4 | The word for where an instance runs. | Accepted: "location". |
| V5 | The word that orders steps. | Accepted: "depends on", as in Docker Compose and Terraform. |
| V6 | Whether retire folds into stop. | Accepted: keep both. |
| V7 | The formal name of the wire to code and MCP services, given that "plugin" becomes casual. | Accepted: "implementation interface". |
| V8 | Five small merges: evidence into record; Agent limit into policy; "Conduit profile" dropped; entry document into manifest; normative surface into specification. | Accepted. |

## Left to their layer

These are not specification words and are settled with the type or profile that owns them:

- **Agent type:** execution, load and expose (replacing "activation"), checkpoint, resume, fresh,
  fork, account, model.
- **Resource types:** memory, history, record collections, worktree, share.
- **Runtime profile:** management service, execution service, helper, snapshot, feed, cursor.
