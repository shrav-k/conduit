# Consolidation audit

Status: 2026-11-05, for plan-audit action A56. The owner asked for this audit after the language
was found to name things in three unrelated ways (plan-audit ruling 25). It looks for one idea
expressed by several unrelated mechanisms, notations or keys, and for one key or word carrying
several meanings. Findings are recorded here, with the owner's rulings in the [next section](#rulings).

Two cold readers ran the audit: one on the shape of documents, reported below, and one on
concepts and mechanisms, [reported after it](#concepts-and-mechanisms).

## Rulings

Owner rulings of 2026-11-05 (plan-audit rulings 27, 28 and 29). All 26 findings are ruled, and
the specification was rewritten for them once.

| Finding | Ruling |
| --- | --- |
| CC1, grant and open Conduit | Merged. The grant is the access an open Conduit carries; issuing is opening and revoking is closing. |
| CC2, the right to issue | Accepted: it is itself an open Conduit to the `access` Resource, with its bound as a filter, and `root` holds the unlimited one. Every permission is a Conduit. |
| CS1, where things go in a document | Accepted as proposed. |
| CS2, `inputs` | Accepted: `inputs` for what is declared, `arguments` for what is supplied, on instances and steps. |
| CS3, `from` | Accepted: `from` for the template; the graph's selector is `source`. |
| CS5, a reference between entries | Accepted: a distinct structured form. |
| CS6, optionality | Accepted: `optional: true` everywhere; `required` only as the schema keyword. |
| CS7, defaults | Accepted: the input's default first, then the type's schema defaults. |
| CS8, pointers | Accepted: all JSON Pointers, each with a stated root. |
| CS9, selections | Accepted: each is a qualified reference; `driver` and `handler` become `implementation`; applying a requirement has the shape of an instantiation. |
| CS10, CS11, CC8, first-start objects and built-in schemas | Accepted: they come from a built-in core package at a reserved address. |
| CS12, "owner" | Accepted: kept for Pools; the person's special powers are operations only `root` can reach. |
| CS13, CC4, actions and controls | Accepted: every management verb is an operation; a control is an operation that changes its own instance's configuration. |
| CC5, result words | Accepted: one table; "could not verify" is the reason "unavailable"; "unknown" is the one word for "cannot tell". |
| CC7, records and inspection | Accepted: inspecting returns the declared part, the observed status and the records; "record" means a log entry only. |
| CC9, location needs | Accepted: they are requirements on an instance's placement, checked against facts the location reports. |
| CC10, intake verbs and "runtime" | Accepted: register for templates and instances, install for packages; "runtime" defined once. |
| CC11, graph roles | Accepted: a graph always implements one operation; opening a declared Conduit is a step of setup. |
| CC12, where a policy lives | Accepted: applied through an operation on `access` and recorded there. |
| CC13, system objects | Accepted: `access` owns opening and closing; `management` keeps registration; a location's three differences are stated. |
| CS4, CC6, nested instantiation | A Pool's entries are created with the Pool, registered, as declared Conduits are created with their consumer. "Provision" is no longer a verb; it is setting up that Resource. |
| Pool ownership | Considered as a Conduit and not adopted. Ownership is one recorded fact, fixed at creation, that permits nothing. Every power over a Pool or its Resources is an open Conduit, and a right to open stated for a Pool covers exactly the Resources it owns. Azure resource groups and Kubernetes namespaces work this way. As a Conduit it would have needed five exceptions: open at creation, exactly one per target, never closed alone, a consumer that makes no requests, and a scope that must exist before the Conduit meant to create it. |
| CC3, a common lifecycle | One lifecycle for every family, with the states registered, ready and retired, moving only forward. A closed Conduit is retired and never opened again; there is no suspended state, and a temporary limit is a policy. Setup never replaces a retired declared Conduit or entry Resource: a replacement is a new instance with a new ID, made only on an explicit request. Conduits keep the verbs open and close. Running, paused and idle belong to type contracts. |

## Choices made while writing the rulings

These were needed to write the promises and follow from a ruling or an existing practice, but the
owner did not rule on them one by one. Each can be changed by a ruling.

| Choice | Why | Promise |
| --- | --- | --- |
| The core package's address is `conduit.dev/core`. | The ruling said "a reserved address such as" this one. A promise needs the actual value. | NAM-020, FST-005 |
| The core package needs no manifest entry and cannot be replaced. | Go's standard library is used without an entry in `go.mod`. | NAM-020, NAM-021 |
| The reasons given when a request is not dispatched: an unsupported filter and a restriction that cannot be evaluated report unavailable; opening a closed Conduit, setting up a retired instance and unregistering a location that has instances report conflict; setting up or retiring a template reports invalid. | Ruling CC5 requires every such promise to name one of the four reasons. | ACC-013, ACC-014, CON-015, LIF-006, LIF-012, PLC-011 |
| Setting up a Resource is authorized against the Pool that owns it. | Provisioning was the Pool's operation, and it is now setting up that Resource. | ACC-025 |
| A right to open can be passed on only as it is or narrower. | Preservation test five: delegation only narrows. | ACC-030 |
| Closing a Conduit for any cause leaves its consumer ready, with the capability reported unavailable. | The promise used to cover only a target that was retired or lost; a deliberate close has to behave the same way. | CON-011 |
| A Resource's owner never changes. | "Fixed at creation." No transfer operation exists in this version. | POOL-001 |
| Entries may refer to each other in a cycle. | All of a Pool's entry Resources are created together, so the old rule that ordered them, POOL-008, has nothing left to order. | POOL-006 |
| A type's default implementation is a qualified reference resolved through its package's manifest. | The promise still asked for a version inside the selection, which the naming ruling forbids. | PKG-011 |

Still unwritten, and not decided here: who may retire or unregister an object, what unregistering
a ready instance does, what retiring a Pool does to the Resources it owns, and who designates the
default location.

## Why this was found late

The first defect sweep asked readers where two engines could behave differently. It did not ask
where one idea had several notations, so parallel schemes passed through. Several of the
collisions below were also introduced by rulings made on 2026-11-05 itself, when a new shape was
accepted without checking it against the shapes already in use. From now on a proposed shape is
checked against this list before it is offered.

## Shape of documents

### Findings that need a ruling

| ID | Finding | Smallest consolidation proposed |
| --- | --- | --- |
| CS1 | **No rule for where things go in a document.** The five standard arguments are written in three places: name in `metadata`, owner and placement at the top level, consumer and target inside `inputs`. On a nested instantiation they sit directly on the entry or declaration. The object envelope is still unwritten. | One rule. A template is kind, `metadata` and `spec`; everything it declares is under `spec`. An instance is kind, `metadata`, `from`, its standard arguments as top-level keys, and one key holding the values for the template's inputs. A nested instantiation has the same shape without kind and `metadata`. |
| CS2 | **`inputs` has four meanings:** a template's declared inputs, an instance's supplied values, a step's value bindings, and a control's singular `input`, which is a schema. | Two words everywhere, as the vocabulary already says: `inputs` for what is declared, `arguments` for what is supplied. An instance and a step both supply arguments. |
| CS3 | **`from` has two meanings:** an instance's template, and the source selector inside a graph value reference. | Keep `from` for the template. Call the selector `source`, which is the word the protocol text already uses. |
| CS4 | **Nested instantiation has three forms and three timings.** A Pool's entries create nothing until provisioned. A template's declared Conduits are created with their consumer. A composite Conduit template expands into its components. Only the entry form has been written out. | One written shape for all three. State the timing of each as a deliberate difference: a declared Conduit is part of its consumer and exists with it; an entry is an offer and exists once provisioned. |
| CS5 | **A reference from one entry to another collides with an ordinary reference.** It is "the other entry's name", but a bare name in a single-kind field already means a registered object. | A distinct, structured form for a reference to a sibling entry. |
| CS6 | **Optionality has no single marker.** The ruled text is consistent, required unless marked optional, but nothing says how the mark is written, and the old fixtures write `required: true` on steps and Conduits. | One marker, `optional: true`, for inputs, steps, declared Conduits and outputs. `required` remains only as the schema keyword. |
| CS7 | **Three default mechanisms with no stated order:** an input's own default, a schema's default, and controls, which apply none. Promise INS-008 says an unsupplied optional input leaves its destination absent, and SCH-009 then fills the type's default. | State the order: the input's default first, then the type's schema defaults for whatever is still absent. Controls stay as ruled. |
| CS8 | **Three ways to point at a place in an instance:** an input's destination, a value reference's path, and a control's list of what it changes. | All three are JSON Pointers with one stated root. |
| CS9 | **Selecting a contract or an implementation uses a different key and shape each time:** `type` for an object's type and for a filter's requirement, `capability`, `operation`, `driver`, `handler`, and a requirement applied as type and config while a template is applied as `from` and inputs. | Fold into the naming ruling: every such selection is a reference to a package's export, and applying any reusable definition has the same shape, a reference and its arguments. |
| CS10 | **First-start objects have no templates,** although every instance comes from a template. The placement record's example already writes `from: location`. | They are instances of templates exported by a built-in core package. This waits on the naming ruling. |
| CS11 | **Built-in schemas are named in yet another way:** `conduit:reference#/$defs/resource` in one record and `.../$defs/location` in another. | Name them as exports of the built-in core package, under the naming ruling. |
| CS12 | **"Owner" means the Pool that owns a Resource and the person who owns the installation.** | Keep "owner" for the Pool. Describe the person's special powers as the root Interface's. |
| CS13 | **Management verbs are called "actions" in the specification and "operations" in the records:** rename, issue, revoke, declare lost. | Call them operations, since the accepted model is that an agent uses them through the same contract as anything else. |

### Corrected without a ruling

These were errors in promises or examples written on 2026-11-05, where the fix follows directly
from a ruling already made.

| Finding | Correction |
| --- | --- |
| A submission was said to replace a template if it named the template's revision, but a document may not carry a revision. | A submission only creates. Replacing a template is a separate operation that names the template's current revision. Promise SUB-007 reworded. |
| A grant was said to list targets, but a Conduit has one target. | A grant lists the operations it permits on its Conduit's target. Definitions and promise ACC-003 reworded. |
| Promise ACC-010 spoke of a consumer's definition being imported, and PLC-001 of exporting an instance, although a complete instance document no longer exists. | Both reworded. |
| A template was defined as a document that declares inputs, but a template may declare none. | Defined as a document with no `from`. |
| Promise INS-001 rejected an instance document that "declares inputs", which read literally rejects every instance. | It rejects an instance document that carries a `spec`. |
| The instantiation record's template example gave a destination inside a `conduits` block the template did not have. | The example now declares the Conduit. |
| Promise CON-005 duplicated PLC-006. | CON-005 removed. |
| The specification said "revision" for its own version. | It says version; revision is kept for objects. |

### Wording to settle when each area is next edited

- "Target" is used for a Conduit's end, for what a reference resolves to and for what an effect
  changes. Only the first should keep the word.
- "Declaration" is used for any document and for a declared Conduit. A standalone Conduit has no
  declaration for the access promises to read.
- "Create", "instantiate" and "register" are used loosely for making an instance.
- Undefined but used: value reference, status, state, diagnostic, type contract, registry.
- Overlapping promises to merge: INS-016 with SUB-006; PLC-005 with POOL-009; OBJ-002 with
  SUB-004, OBJ-011 and FST-004; INS-005 with SCH-011; INS-002 with OBJ-007.
- The placement, instantiation and naming records still use retired words in their prose. The
  specification itself is clean of them.

### Not inconsistencies

`to` has one meaning. Kind and name, with a bare name where a field fixes the kind, is coherent
apart from CS5. Semantic Versioning for packages beside `v1` for contracts is deliberate.
Required-by-default is uniform in the ruled text.

### The reader's judgement

A few more remain in the text already ruled. Many are latent in shapes not yet converted:
controls, runtimes, outputs, requirement and contract documents, and the manifest each still
carry their own notation. Those must be checked against this list as they are written.

## Concepts and mechanisms

Each finding is classed as a real duplication, or as a principled distinction that only needs
stating.

### Real duplications

| ID | Finding | Smallest consolidation proposed |
| --- | --- | --- |
| CC1 | **A grant and an open Conduit are one thing with two names and two pairs of verbs.** Only an open Conduit carries a grant; opening issues a grant equal to what the Conduit requests; revoking the grant closes the Conduit; a closed Conduit is never reopened; a grant is revoked whole. So issue is open and revoke is close, with the same lifetime and the same content. | Define the grant as the access an open Conduit carries, and issue and revoke as open and close. This makes "the grant is an object" literally true: the object is the Conduit. |
| CC2 | **A right to issue is a third authorization record with no home.** The specification treats it as a separate permission and never says where it is recorded. The accepted prose made it an ordinary grant on the access-management Resource. Authority for an action is checked in five different ways: a grant on the target, a right to issue on the target, a grant on the owning Pool, a grant "that covers reconciling", and "the owner". | A right to issue is an open Conduit to the `access` Resource, with its bound as a filter. One table maps each management operation to the target whose grant covers it. The owner's special operations become grants held by `root`. |
| CC3 | **There is no common lifecycle.** A Conduit is not open, open or closed. Other instances are set up or not. "Running", "stopped", "paused" and "started" appear in promises without definitions. "Unknown" is used as a state. The Agent type's own "open, paused, closed" lets a closed Agent be reopened, the opposite of a closed Conduit. | One lifecycle for all five families: registered, set up, retired, unregistered. Open and close are a Conduit's spelling of set up and retire. "Unknown" qualifies an observation and is not a state. Running, paused and idle move wholly into type contracts. |
| CC4 | **Operation, control and "action" are three names for something invocable.** Only an operation has a contract, yet a control is invoked, has an outcome, an implementation and a revision check. Read literally, the access and outcome conventions cover neither controls nor lifecycle actions. | A control is an operation whose effect is a change to its own instance's configuration. Every management verb is an operation of some capability. The control-specific rules stay as extra rules. |
| CC5 | **Result words overlap and in places contradict.** There are outcomes, four reasons a request is not dispatched, and three access decisions with an undefined "reason class". One rule says an unevaluable restriction makes an operation "unavailable" and the promise for it says "refused"; a reused identity is "refused" in the record and "conflict" in the promise; four promises describe a request that is not dispatched without naming a reason. "Unknown", "uncertain", "unavailable", "missing" and "not run" are scattered. | One table in the definitions. The access decisions map onto the four reasons, with "could not verify" as "unavailable". Every promise about a request that is not dispatched names its reason. "Unknown" is the one word for "cannot tell". |
| CC6 | **Three nested instantiations with three timings.** This is the same finding as CS4. The reader adds that the stated reason for a Pool's lazy entries, that listing an entry must not deploy it, is already met by instantiation having no effect. A composite Conduit that leaves no instance also contradicts "an instance is one realization of a template". | One rule for a declared child. Either both kinds are registered, inert, with their parent, so that provisioning becomes setting up that child, or the difference is stated as deliberate. This needs a ruling. |
| CC7 | **"Record" means a log entry and also an object's registration.** "Inspect" appears in about thirty observables and is undefined, as are "status" and "observation". | Before the conventions on declared against observed state and on records are written, fix one way to read: inspecting returns the declared part, the observed status and the records. Stop using "record" for the registry entry. |
| CC8 | **First-start objects have no templates.** The same finding as CS10. | As CS10. |

### Principled distinctions that need stating

| ID | Finding | What to state |
| --- | --- | --- |
| CC9 | **"Capability" is used twice:** what a target offers a consumer through a Conduit, and what a location offers a type. A location is a Resource, so the second is literally an instance's capability, yet it is matched with no Conduit and no grant, while the shared-host rule was made an ordinary requirement. Location facts are therefore checked by two mechanisms. | Treat a type's location needs as requirements on its placement, with the location reporting facts as it already reports its host. That leaves "capability" one meaning. The reader leans towards calling this a real duplication. |
| CC10 | **Type, template, contract and kind.** The type is the implementer's contract; the template is the author's recipe that narrows it. That is principled. Smaller overlaps: "kind" and "family" are one idea; type defaults overlap template selection; there are three intake verbs, register, install and "accepted"; "runtime" is used and never defined. | State the type and template split once. Settle the intake verbs. Define or remove "runtime". |
| CC11 | **Graphs in three roles** are one mechanism that differs only in what a graph may refer to and in its success rule. Required and optional are defined twice, on steps and on declared Conduits, without saying that opening a Conduit is a step. "Order is derived from references" is stated three times. | A graph always implements exactly one operation. Opening a declared Conduit is a step of setup. State derived ordering once. |
| CC12 | **Requirement, filter, policy and restriction** are one kind of contract distinguished by where it is attached. Where a policy is declared or stored is unstated. | Say so, and say where a policy lives. |
| CC13 | **`management` and `access`** are defensibly two Resources, but the line blurs: Conduits are registry objects while grants sit in `access`. A location is not a Resource like any other: it is its own placement, has its own unregister rule and is depended on without a Conduit. | After CC1, state which of the two owns opening and closing. State the ways a location differs. |

### The reader's judgement

A few more remain, mostly in the four unwritten conventions and in the type contracts, where
lifecycle and observation words will collide again unless CC3, CC5 and CC7 are settled first.
