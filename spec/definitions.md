# Definitions

Draft. These are the terms the promises rely on, each with one meaning. The promises govern:
where a row names a promise, that promise states the rule and the row only says what the word
means. The history of the vocabulary is in the
[vocabulary record](../docs/proposals/vocabulary/README.md); where the
[consolidation rulings](../docs/proposals/consolidation/README.md#rulings) or the
[re-audit](../docs/proposals/reaudit/README.md) changed a word, this file has the current one.
The names of the core package's exports, and the definitions of Interface, Agent, Resource,
Pool, model work, mediated, enforced, project and published, are those of the
[ruling on the core package and the schemas](../docs/proposals/core-and-schemas/APPLY.md).

## Objects and documents

| Term | Meaning |
| --- | --- |
| family | One of the five kinds of object: Interface, Agent, Resource, Pool and Conduit. The first four have rows here, and Conduit under [Relationships](#relationships). |
| Interface | One of the five families: where a person, or a system outside, acts on the system. An Interface is an actor: it makes requests on its own behalf (IFC-021). It has no placement (PLC-006), and can be the consumer or the target of a Conduit (OBJ-013). |
| Agent | One of the five families: a participant whose model decides its own actions. An Agent is an actor, as an Interface is, and has exactly one placement (PLC-001). Its model being run is model work. |
| Resource | One of the five families: a shared thing that instances use. A Pool owns it: a Resource has exactly one owner, which is a Pool (POOL-001), and exactly one placement (PLC-001). It is not an actor: it makes a request only to serve an invocation it received (IFC-021). |
| Pool | One of the five families: what owns and groups Resources. Its template may declare entries (POOL-002). It has no placement, can be the target of a Conduit, and is never a consumer or a caller (OBJ-013, ACC-028). |
| kind | The value of a document's `kind`. There are ten: the five families, and the five kinds of contract `Type`, `Capability`, `Operation`, `Requirement` and `Schema` (OBJ-020). A manifest has no `kind` (MAN-003). |
| object | A template or an instance of one of the five families. A contract is not an object (SUB-008). |
| document | One template, instance or contract written out, in YAML or in the equivalent JSON (SCH-013). It carries `apiVersion`, `kind` and `metadata`; only an instance document may leave `metadata` out (OBJ-013). A YAML file can hold several documents, and a JSON file holds one document or one array of documents (SCH-018). Within one project or package no two documents have the same name (NAM-027). The JSON Schema of each kind of document is a Schema contract under [schemas/](schemas/README.md). A manifest is not a document in this sense. |
| project | One directory with at most one manifest. Its documents are the files in the directory itself whose names end in `.yaml` or `.json`, apart from the manifest, and nothing below it is read (MAN-001, MAN-011). Its contracts are available to its own documents (SUB-008). A package is a project that has been published. |
| published | Made available as an archive under an address. A package is a project that has been published: its manifest states its own `address` and `version` (PKG-017, MAN-004), and its archive is the one PKG-022 and MAN-013 describe. A project that is not published states neither. |
| submission | The documents handed to the engine together to be registered: templates, instances, contracts and the project's one manifest (SUB-005). Where a directory is submitted, they are the files MAN-011 names. They are a set, so their order has no meaning (SUB-001), and if any of them is rejected nothing in the submission is registered (SUB-003). |
| name | The readable lookup for an object, written in `metadata.name`: lowercase letters, digits and hyphens, starting with a letter, at most 63 characters (OBJ-001). No two objects of one kind that exist have the same name, templates and instances counted together (OBJ-002), and no two documents of one project or package have the same name, whatever their kinds (NAM-027). An instance that is given no name gets a generated one (OBJ-003). Other names have their own rules: an export's name (NAM-004), a contract's name (CTR-001), the name of a member of a map (OBJ-022), and the names of an operation's inputs and outputs (CTR-002). |
| type | What specializes a family other than Conduit. It is a Type contract, which a template selects under `type` (TPL-002) and which states, among other things, the schema of an instance's `config`, the capabilities its instances offer and their controls (CTR-004). A Conduit has no type. |
| schema | The shape of valid data, written in JSON Schema 2020-12 as [schema-profile.md](schema-profile.md) narrows it (SCH-001, SCH-002). |
| template | A document of one of the five families that has no `from`: `apiVersion`, `kind`, `metadata` and a `spec` that holds everything it declares (INS-001, OBJ-012). Instances are made from it. It can narrow what its type allows and cannot widen it (INS-014), and it has no lifecycle state (LIF-012). |
| instance | One identified object made from a template. Its document has `from` and the standard arguments of its family, may have `metadata` and `arguments`, and has no `spec` (INS-001, OBJ-013). Every instance comes from a template and has exactly one lifecycle state. |
| instantiate | Make an instance from a template and its arguments. It is what registering an instance document does. The instance's nested instances are made with it, and nothing is set up or opened (INS-016). |
| nested instantiation | An instantiation written inside a template, with no `kind`, `metadata` or `spec` (OBJ-014). It has three shapes. A declared Conduit holds `from`, and may hold `arguments`, `target` and `optional: true`. A Pool's entry holds `from`, and may hold `arguments` and `optional: true`. A component of a composite Conduit holds `from`, and may hold `arguments`. |
| nested instance | An instance made by a nested instantiation when the template that holds it is instantiated: the Conduit of a declared Conduit, or the Resource of a Pool's entry (INS-016). A component Conduit is made in the same way from a component (CON-003). When a nested instance is retired, another is made in its place only by an explicit request (LIF-008, LIF-009). |
| input | A named value that a template, an Operation contract or a Requirement contract declares, and that whoever uses it supplies. A template's input is a member of the `inputs` map of its `spec` and holds `schema`, `to`, which lists one or more destinations, and optionally `default` and `optional: true` (INS-021). |
| argument | The value supplied for an input, written under `arguments`. An instance document, a nested instantiation and an applied requirement write plain values there (OBJ-013, NAM-019). A step writes value bindings (GRA-015). An `arguments` map that is left out supplies no arguments (OBJ-013, OBJ-014, NAM-019). |
| standard argument | An argument that belongs to a family and not to a template. It is written as a top-level key of an instance document and is accepted whatever inputs the template declares: `owner` for a Resource, `placement` for an Agent and a Resource, and `consumer` and `target` for a Conduit (OBJ-013). |
| destination | A place in a template's `spec` where an input's value is assigned. It is a JSON Pointer in its string form whose root is the `spec`, and it does not use the `-` token (OBJ-016). INS-012 gives the closed list of the places it may reach. |
| optional | An input, a step, a declared Conduit, a Pool's entry, a runtime or an output is required unless it is marked `optional: true` (OBJ-015). The word `required` appears only as the schema keyword. |
| item | A member of a map or a list that is not a Pool's entry, such as an item of a manifest or of the implementation-defined list. |

## Resolving and registration

| Term | Meaning |
| --- | --- |
| engine | A running program that implements this specification. It is the one word for the running thing; "implementation" is never used for it. |
| resolving | The function from a set of documents, read against what already exists, to either the objects those documents make or diagnostics. It runs nothing and stores nothing. What it yields is called resolved: a resolved template, a resolved instance, a resolved graph. |
| resolve (a reference) | A reference resolves to the one thing it names. In an instance every reference is resolved to an ID when the instance is made (OBJ-009). A reference, or a manifest item, that reaches nothing is unresolved. |
| diagnostic | What resolving, or the installation of a package, returns about something that is wrong: it names the document or the package and the rule it breaks. There are two classes. One rejects. The other is reported and does not prevent registration (OBJ-008, NAM-008, PKG-006). |
| rejected | Said of a document for which resolving returns a diagnostic of the first class. Its registration is not dispatched and reports invalid (OBJ-023), or conflict where the document clashes with the name of an object that exists (OBJ-002). A package is rejected when it is installed, and is then not available; the request to install it reports invalid (PKG-023). |
| register, registration | The one operation that takes documents, `register.v1` of the core package. It runs resolving on a submission and, if no document is rejected, stores the templates and instances the documents make, which then exist (SUB-006). Registering an instance document instantiates its template. It has no other effect. An object that has been registered and not unregistered is said to exist; "registered" as a description of an instance names a lifecycle state. |
| install | Make a package available, so that what it exports can be reached by qualified references. It registers nothing (NAM-013). It is the operation `install.v1` of `management` (ACC-033). PKG-023 to PKG-025 say what a request that installs nothing reports. |
| exists | An object exists from the moment it is made, by a registration or at first start, until it is unregistered, whatever its lifecycle state (LIF-007, LIF-012). |
| ID | The stable identity of an object, which the engine gives it. A reference in an instance is resolved to an ID, and an object's ID does not change when the object is renamed (OBJ-005, OBJ-009). A request names an object by a reference and never by its ID (LIF-007). The engine also gives an ID to every invocation and to every policy. |
| reference | A written pointer to an object: a map `{kind: <Kind>, name: <name>}`, or the name alone where the schema of the field fixes the kind (OBJ-006). `consumer` and `target` accept more than one kind and so are written with the kind (OBJ-013). A request names an object by a reference (LIF-007). |
| local reference | A reference that contains neither `#` nor `/`. It resolves only among the objects that exist, and is never looked up in a package (NAM-003). |
| qualified reference | A package's address, then `#`, then an export's name, such as `github.com/acme/conduit-history#shared-reviewer` (NAM-002). From outside a package it is the only way to reach what the package exports, and it never carries a version (NAM-005). A document does not use one for something in its own project or package (NAM-022). |
| `#name` | `#` followed by a name, with no address: a reference to the document of that name in the same project or package as the document that holds it. A document uses this form, and never a qualified reference, for something in its own project or package (NAM-022). It carries no kind, so no two documents of one project or package share a name (NAM-027). A selection may be written this way (NAM-018), and so may a `$ref` to a Schema contract (SCH-015). Only the core package's own documents write their own address in full. |
| reference to a Pool's entry | `{kind: Pool, name: <pool>, entry: <entry>}`, written outside the Pool's template. It resolves to the Resource of that entry that is not retired (POOL-014). |
| entry reference | `{entry: name}`: one entry's reference to another entry of the same Pool template, rejected anywhere else (POOL-005). It resolves to the other entry's Resource when the Pool is instantiated (POOL-006). |
| reference schema | One of the Schema contracts that the core package exports for references: `reference.v1`, `interface-reference.v1`, `agent-reference.v1`, `resource-reference.v1`, `pool-reference.v1`, `conduit-reference.v1`, `actor-reference.v1`, `consumer-reference.v1`, `target-reference.v1`, `entry-reference.v1` and `pool-entry-reference.v1`. A value inside `config` or `arguments` is a reference only where a reference schema governs its place, directly, as the `items` of a list, or as a branch of `anyOf` or `oneOf`; any other value is data and is never rewritten (OBJ-021). Where an input's reference schema fixes the kind, a bare name is accepted for it (INS-006). |

## Placement

| Term | Meaning |
| --- | --- |
| location | Where an Agent or a Resource runs. A location is a Resource instance of the location type, owned by the `system` Pool. It differs from other Resources in three ways: it is its own placement (PLC-008); no ordinary request retires or unregisters it while other instances are placed on it (PLC-011); and instances depend on it through their placement, with no Conduit (PLC-015). |
| location type | The type of which every location is an instance (PLC-008). It is the Type contract `location.v1` of the core package. |
| placement | The location of an Agent or a Resource, held as that location's ID. Each has exactly one, and no operation changes it (PLC-001, PLC-009). Interfaces, Pools and Conduits have none (PLC-006). Where a document gives none, PLC-002 to PLC-004 choose it. |
| host | The machine a location is on. It is one of the location's facts. |
| location fact | Something stored about a location, such as its host. A location's facts are stored configuration of the location, held under `facts` in its configuration. They are given when the location is instantiated, and afterwards only the control `set-facts.v1` changes them; nothing observed of the location does (PLC-008). The promises also call them its stored facts. |
| `set-facts.v1` | The control of the core package that sets a location's facts. It is in the capability `location-settings.v1`, which the location type offers, so it is requested through an open Conduit to the location that lists it. It takes the location's current `revision` and `facts`, and its result states `settings` (PLC-008). |
| default location | The location that the default-location setting of the `system` Pool names. Where several locations exist, an instance whose document gives no placement takes it (PLC-003). It is designated or cleared by `set-default-location.v1`, a control of the `system` Pool in the capability `system-settings.v1` (PLC-016). |
| requirement on placement | A requirement whose contract applies to placement: a schema that the stored facts of an instance's location must satisfy (CTR-005, PLC-007). A type states its own under `placement`, and an implementation states its own in its manifest item (CTR-004, PKG-019). |
| shared-host requirement | The requirement `shared-host.v1` of the core package, applied to a Conduit: that the stored facts of its consumer's location and of its target's location give the same host. The engine checks it when the Conduit is opened and on each request through it (PLC-013). It does not narrow requests, so it is not a filter (IFC-018). |

## Ownership

| Term | Meaning |
| --- | --- |
| owner | The Pool instance that owns a Resource. Every Resource has exactly one, fixed when it is instantiated, and no operation changes it (POOL-001). Ownership is a stored fact and permits nothing (ACC-028). No promise uses the word for a person. |
| entry | A named nested instantiation of a Resource template, written in the `entries` map of a Pool template (POOL-012). When the Pool is instantiated, one Resource is instantiated for each entry and is owned by that Pool (POOL-002). An entry never has more than one Resource that is not retired (POOL-003). In the promises the word is used only for this. |

## First start

| Term | Meaning |
| --- | --- |
| first start | The first time an engine is started. After it the first-start objects exist, each ready, with the two open Conduits of `root`, and nothing else (FST-001 to FST-003). |
| first-start object | One of the Pool `system`, the location `local`, the Resources `management` and `access`, and the Interface `root`. Each is an instance of a template that the core package exports, under the names `system`, `location`, `management`, `access` and `root` (FST-005), and none is retired or unregistered (LIF-022). |
| `system` | The Pool that exists from first start, also written "the system Pool". It owns the locations and the Resources `management` and `access`, and the default-location setting is one of its settings. |
| `management` | The Resource against which these requests are authorized: registering, replacing, renaming and unregistering any object; setting up, retiring and inspecting an Agent, an Interface or a Pool; inspecting a Conduit or a template; installing a package; and every operation reserved to the root Interface (ACC-033). Its capability is `object-management.v1`. The same promise names the Pool that owns a Resource, `access`, or the instance itself for the other requests. |
| `access` | The Resource whose operations open and close Conduits and apply and remove policies (ACC-006, ACC-012, ACC-031). It stores the policies in force, outside its configuration, and inspecting it lists them in its status. Its capability is `access-control.v1`. |
| root Interface | The Interface `root`, the only actor that has access at first start. Its Conduit to `access` has no bound (ACC-022), no request closes a Conduit of which it is the consumer (ACC-024), and no policy is applied to it (ACC-035). Some operations, such as declaring a location lost and purging records, are reserved to it: they are operations of `management` and are dispatched only when it asks (ACC-032, ACC-033, REC-004). |
| reserved to the root Interface | Said of an operation that is dispatched only when the root Interface asks (ACC-032). Its Operation contract carries the mark `reservedToRoot: true` (CTR-002). Only an Operation contract of the core package may carry the mark (CTR-009), and only a capability of the core package may list such an operation (CTR-003). The two are `declare-lost.v1` and `purge.v1`. |

## Relationships

| Term | Meaning |
| --- | --- |
| Conduit | One of the five families. A Conduit is a relationship between exactly one consumer and exactly one target for exactly one capability (CON-001). While it is open it permits exactly the operations it lists, on its target (ACC-003). It has no type and no placement. |
| consumer | The instance that makes requests through a Conduit. It is an Interface, an Agent or a Resource (OBJ-013). |
| target | The instance that receives them, and so the instance a request is made of. It is an Interface, an Agent, a Resource or a Pool (OBJ-013). In the promises the word is used only for this end of a Conduit. |
| capability | A Capability contract: a list of operations (CTR-003). A type's contract lists the capabilities its instances offer, and a Conduit names exactly one, which its target's type must offer (CON-021). Offering is not permission. |
| declared Conduit | A nested instantiation of a Conduit template, written in the `conduits` map of an Interface, Agent or Resource template (TPL-001). One Conduit is instantiated for it with each instance of that template, with the instance as its consumer (CON-002). It is a step of that instance's setup, which opens it (GRA-022, CON-004). |
| open | Set up a Conduit. An open Conduit is one whose state is ready. It is opened by an operation of `access` (ACC-006). |
| close | Retire a Conduit. A closed Conduit is one whose state is retired: all the access it carried has ended, and it is never opened again (ACC-011, LIF-006). CON-008 says what closes one. |
| composite, component | A composite Conduit template is one whose `spec` holds `components`, a map of nested instantiations of Conduit templates, in place of a capability (CON-016, CON-019). Each member is a component. Instantiating a composite makes one Conduit for each component, called a component Conduit, with the consumer and the target given to the composition, and makes no instance for the composition itself (CON-003). |

## Access

| Term | Meaning |
| --- | --- |
| actor | An Interface or an Agent (IFC-021). Only an actor makes a request on its own behalf. |
| caller | The actor on whose behalf a request is made. For a request an actor makes on its own behalf it is that actor. For an onward request it is the caller of the invocation being served (IFC-020, IFC-021). |
| onward request | A request that an instance makes of another target in order to serve an invocation it received. It names that invocation, is permitted only under the instance's own open Conduit to that target, and carries the caller and the restrictions of the invocation it serves (ACC-016, ACC-017). A step that names a declared Conduit under `conduit` is one (ACC-018). |
| serve | Do the work of. An instance serves an invocation that was dispatched to it and is in progress (IFC-028). An implementation serves each instance for which it is the selected implementation. |
| permitted | A request is permitted when an open Conduit from its consumer to its target lists the operation and every restriction that applies permits it (ACC-001, ACC-008). ACC-033 fixes the target for each management request, and says the request is authorized against that target. A request that is not permitted reports refused. |
| access decision | The engine's own decision whether a request is permitted. It is made before the target's implementation is invoked, and it is recorded (ACC-002, REC-001). |
| right to open | What an actor has when it is the consumer of an open Conduit to `access` that lists the operation that opens Conduits (ACC-006, ACC-023). It is limited by that Conduit's bound, and is passed on only as it is or narrower (ACC-030). |
| bound | The limit that a Conduit to `access` carries on which Conduits its consumer may open and close and which policies it may apply and remove. It is written as the core package's requirement `bound.v1`, applied in the `requirements` of the Conduit's template with the arguments `pools`, `targets` and `operations`. Naming a Pool covers the Pool itself and every Resource it owns, whenever instantiated (ACC-029). One bound is within another when every Pool and every target it names is covered by the other, and every operation it names is one the other names (ACC-030). A Conduit to `access` that applies no `bound.v1` has no bound, as the root Interface's has none (ACC-022). A Conduit to any other target that applies one, or a Conduit that applies it twice, is not instantiated (CON-028). |
| requirement | A Requirement contract: something that must hold for a relationship or its use. Its `appliesTo` says where it may be applied: to a Conduit, as a policy, or to placement (CTR-005). One requirement may be a composition of others, which are its `constituents`. |
| applied requirement | A requirement as it is written where it is applied: `requirement`, the selection of the contract, and `arguments`, which may be left out and are checked against the inputs the contract declares (NAM-019). |
| filter | A requirement applied to a Conduit that narrows what may pass through it. Its contract states `narrows: true`, and it is written in the Conduit template's `requirements` map (CTR-005, CON-018). It applies to requests through that Conduit only (ACC-009). |
| policy | A requirement applied to chosen actors or targets instead of to one Conduit. It limits every request of which one of them is the consumer or the target (ACC-009). It is applied by `apply-policy.v1`, which returns an ID for it, and removed by `remove-policy.v1`, which names that ID. `access` stores it, and it is not configuration: neither operation names a revision, and inspecting `access` lists the policies in its status (ACC-031). It is never written in a template. |
| restriction | A filter or a policy that applies to a request, including one that an onward request carries from the invocation it serves (IFC-018). A restriction only narrows: none permits what the Conduit does not (ACC-008). |
| mediated | Said of a restriction that applies to requests that pass through the engine. It is one of the two values that inspecting a Conduit reports for each restriction on it (ACC-027). |
| enforced | Said of a restriction that an isolation plugin also prevents a consumer from going around. It is the other value that inspecting a Conduit reports (ACC-027). |

## Requests and operations

| Term | Meaning |
| --- | --- |
| operation | A callable action with named inputs and outputs, defined by an Operation contract (CTR-002). Every management verb, such as register, set up, open, close, retire or install, is an operation of a capability that `management`, `access` or a Pool offers, and is an Operation contract of the core package, such as `register.v1` or `open.v1` (OPS-019). |
| effect | What an operation's contract says, under `effect`, that it changes outside itself. An operation whose contract states none changes nothing outside itself (CTR-002). |
| request | The asking for one operation on one target, by an actor on its own behalf or by an instance as an onward request. Every request ends in exactly one of the seven ways listed [below](#how-a-request-ends). |
| dispatch | The engine's passing of a request to the implementation selected for its operation, after the engine's own checks. A request that is not dispatched reaches no implementation. |
| invocation | What a dispatched request begins: one carrying-out of an operation on a target for a caller. It has an ID that the engine gives it, a state, and one or more attempts. |
| the engine's ID for an invocation | The ID the engine gives every invocation, whether or not the caller supplied an operation identity. It is sent to the implementation (IFC-006), is what an onward request names to say which invocation it serves (IFC-020), and is held in records (REC-002). |
| operation identity | A value a caller may attach to a request so that repeats of it are recognized. It is scoped to that caller and that target, and at most one invocation exists for one caller, one target and one identity (OPS-008). A request that carries none is always a new invocation (OPS-011). |
| repeat | A request with the same caller, target, operation identity and contents as an earlier one that began an invocation. The contents are the same when both name the same operation and their arguments are equal (OPS-009). A repeat is not a new request: it is answered with the invocation's state, or dispatched as a new attempt. A request that matches an earlier one that ended with a reason is not a repeat: it is a new request, decided afresh (OPS-025). |
| attempt, new attempt | An attempt is one dispatch of an invocation. A new attempt is a further dispatch of the same invocation, with the same operation identity. It is made only for a repeat: after the outcome failed (OPS-024), or after the outcome unknown where the operation's contract declares `deduplicates: true` and the implementation is not plain (OPS-012). Nothing is dispatched again without a repeat (IMP-007). |
| in progress | Said of an invocation that has been dispatched and has not ended (IFC-024). On the wire it is written `inProgress` (IFC-024, IFC-025). |
| the invocation's state | In progress, or the invocation's outcome. It is what is kept for an operation identity, and it is read from the records about the invocation (OPS-008). |
| outcome | How an invocation ended: succeeded, failed or unknown (OPS-001). |
| reason | Why a request ended with no outcome: refused, invalid, conflict or unavailable (OPS-005). |
| retained result | The result of an invocation, kept so that the original caller can ask for it later by operation identity and target. Only that caller is given it, and asking needs no open Conduit (OPS-015, OPS-016). |
| retrieval tool | The tool `retrieve-result.v1`, an operation of the core package that is in no capability. The engine presents it to every instance under that name alone, and a call to it needs no Conduit. A call names an invocation by `operationIdentity` and `target`, and its result holds `state`, `withheld` and `outputs` (IFC-024, OPS-016). |
| withheld | Said of the content of a result that exists and is not delivered, because the caller's access at that moment does not permit it. The caller is still told the outcome, and that content is withheld (OPS-017, ACC-034): the retrieval tool's result says `withheld: true`, and a direct answer carries no `structuredContent` and holds `io.github.shrav-k.conduit/withheld` in its `_meta` (IFC-024, IFC-025). |
| reconcile | Record, with what was checked, that the effect of an invocation whose outcome is unknown did occur or did not. The outcome then becomes succeeded or failed (OPS-014). It is the operation `reconcile.v1`, which names the invocation by the engine's ID for it. The noun is reconciliation. |
| model work | An Agent's model being run: a turn started or continued. It starts only as the effect of an operation whose contract is marked `startsModelWork: true`, dispatched through an open Conduit that lists that operation (IMP-001, IMP-003). Neither setup nor retirement starts it (IMP-005, IMP-009). |

### How a request ends

Every request ends in exactly one of these seven ways. No other word is used for them.

| It has | Word | Meaning |
| --- | --- | --- |
| An outcome | succeeded | The effect the operation's contract describes has occurred (OPS-002). |
| | failed | The effect has not occurred, in whole or in part (OPS-003). |
| | unknown | It cannot be confirmed that the effect occurred or that it did not, or it occurred only in part (OPS-004, OPS-007). |
| A reason, and no outcome | refused | The request is not permitted, whoever asks and whatever the cause: for example no open Conduit lists the operation, a restriction forbids it, a bound does not cover it, or the operation is reserved to the root Interface. |
| | invalid | The request cannot be read, or does not satisfy a schema or a rule for what it asks, such as a document that is rejected or a change that names no revision. |
| | conflict | The request does not match the current revision or state of what it addresses, or clashes with something that exists, such as a name or an operation identity already used with other contents. |
| | unavailable | The request could not be checked or carried out now: for example a restriction cannot be evaluated or applied, the target is not ready, or the shared-host requirement does not hold. |

A request that has an outcome was dispatched. A request that has a reason was not dispatched,
with one exception: an implementation that carries Conduit's keys may itself answer refused or
unavailable in place of an outcome, where a restriction it was sent forbids the request or
cannot be applied, and it has then performed nothing (IFC-026, IFC-027). Where more than one
reason applies to a request that is not dispatched, one is reported, in this order: invalid if
the request cannot be read; otherwise refused, then invalid, then conflict, then unavailable
(OPS-023). A result that carries both an outcome and a reason has the outcome unknown
(IFC-009). A request that matches an earlier one that ended with a reason is a new request and
is decided afresh (OPS-025). The answer to a repeat of an invocation that has not ended is not
one of the seven: it carries neither an outcome nor a reason, and says `inProgress` (OPS-009,
IFC-025).

## Implementations and the implementation interface

| Term | Meaning |
| --- | --- |
| implementation | What does the work behind an operation. A package's manifest lists each one with the operations it implements and how it is reached, which is by `command`, `url` or `graph` (PKG-019, MAN-009); a template may instead write a graph in place (TPL-003). In the core package's manifest alone, the implementation `engine` states none of the three, because the engine itself does that work (MAN-012). Otherwise the word is never used for an engine. |
| plain | Said of an implementation that is an existing server not written for this specification, marked `plain: true` in its manifest item (PKG-020). It receives none of Conduit's keys (IFC-007), supports no requirement and never counts as deduplicating, and its answers are turned into outcomes by the table of IFC-010. An implementation that is not plain is one that carries Conduit's keys. |
| implementation interface | The shared wire between an engine and an implementation, and between an engine and a consumer. It is the Model Context Protocol: an operation is one tool, and the engine presents to each consumer a server whose tools are the operations its open Conduits list (IFC-001, IFC-012). |
| baseline revision | The revision of the Model Context Protocol that every engine, and every implementation that carries Conduit's keys, supports. For this version it is 2026-07-28 (IFC-002). Either side may support others, and only a new version of this specification changes it (IFC-005). |
| Conduit's keys | The keys Conduit adds to `_meta`, each beginning `io.github.shrav-k.conduit/`. On a request they are `interfaceVersion`, which is `v0` for this version, `instance`, `operation`, `invocation`, `caller` and `restrictions`, with `operationIdentity` and `conduit` where they apply (IFC-006), and `serving` on an onward request (IFC-020). On a result they are `outcome`, or `reason` in its place (IFC-009, IFC-026); `restriction`, beside a reason that an implementation gives (IFC-026); `state`, with the value `inProgress`, on the answer to a repeat of an invocation that has not ended; and `withheld`, with the value true, where content is withheld (IFC-025). |
| tool | A tool of the Model Context Protocol. An implementation has one for each operation it implements (IFC-001). The engine presents one to a consumer for each operation of each of its open Conduits, beside the retrieval tool (IFC-012, IFC-024). |
| tool name | The name of a tool the engine presents to a consumer: the Conduit's local name, then the name of each component for a Conduit made from a composite template, then the operation's export name, joined by dots (IFC-012). An export name ends in the contract's version segment, so a tool name does too, as in `checkout.directory.read.v1`. The retrieval tool is named `retrieve-result.v1` alone (IFC-024). No two tools presented to one consumer share a name, and none is longer than 128 characters (IFC-014, CON-026). |
| local name | The name by which a consumer's tools name one of its Conduits. For a declared Conduit it is the name its consumer's template gives it, and for a Conduit instantiated on its own it is the Conduit's own name (IFC-012). |
| handle | A live value passed between operations, such as a running service. An Operation contract marks an input or an output as one with `handle: true`. On the wire it is an opaque string that its implementation issued, which the engine passes back unchanged; holding one permits nothing (IFC-011). |
| hold | Keep as part of what is stored for a template or an instance. An instance holds the handle of a runtime from the moment the runtime's start operation succeeds until the runtime is stopped (TPL-007). An instance holds the IDs its references resolved to, and a template and an instance hold the versions and digests of their packages, or for the core package its address and version alone (NAM-009, PKG-005). The word is not used for a Conduit or a right: an actor is the consumer of an open Conduit. |
| runtime | Something long-lived that an instance keeps, such as a running process. A template or a Type contract declares it as a named pair of operations: `start`, which returns one handle that the instance then holds, and `stop`, which takes that handle (TPL-006, TPL-007). It is not the engine. |
| restore (a runtime) | Said of an implementation that, as its type contract allows, keeps alive what is behind the handle an instance holds. The handle stays valid, and nothing is dispatched because of it (OBS-005). |

## Configuration and change

| Term | Meaning |
| --- | --- |
| definition | What a template or an instance declares apart from its configuration: its type, its `implementation` map, runtimes, `setup` and `retire` graphs, controls and nested instantiations, and for a Conduit its capability, operations and requirements. An instance's definition is resolved when the instance is made (INS-013), and no control changes it. |
| configuration | The stored settings of an instance. It is the value written under `config`, which the `schema` of the instance's type governs (CTR-004): a template writes it in its `spec`, inputs fill it when the template is instantiated, and afterwards controls change it. `config` is where configuration is written, and a setting is one place in it. A location's facts are configuration (PLC-008); a policy that `access` stores is not (ACC-031). |
| control | An operation whose Operation contract also states, under `changes`, the settings it changes (CTR-006, OPS-020). A type lists its controls, and a template chooses which of them its instances have and may narrow their inputs (TPL-010). It is requested through an open Conduit to the instance whose configuration it changes. `revision` is the one input it must always be given (REV-006), and the settings its result confirms, in its output `settings`, are the ones stored (REV-007). The core package has two: `set-default-location.v1` and `set-facts.v1`. |
| revision | The single value on a template or an instance that a change is checked against (REV-001). A request to change an object names the object's current revision, in the operation's input `revision` (REV-002, REV-011). The revision advances exactly when the stored definition or configuration changes, and never returns to an earlier value (REV-003 to REV-005). Setups, retirements and changes of one instance are applied one at a time, and setting up and retiring name no revision and advance none (REV-012). |

## Packages

| Term | Meaning |
| --- | --- |
| package | What an author ships: a manifest and the things it exports, distributed as one archive, which is a gzip-compressed tar file with the manifest at its root (PKG-022, MAN-013). It is identified by its address, and each release of it has a version. A package is a project that has been published, and its manifest states its own address and version (PKG-017). |
| address | A package's identity: a host part, which is a lowercase DNS name with a dot in it, no port and no trailing dot, and then a path, which an address may lack, such as `github.com/acme/conduit-history` (NAM-001). A release that breaks with those before it should take a new address (NAM-025). |
| version | Of a package: a Semantic Versioning 2.0.0 version (PKG-001). A release is one version of a package. Two versions are the same when neither precedes the other (PKG-003). A manifest names exactly one version of each package (NAM-006), and a reference never carries one (NAM-005). |
| manifest | The one file of a project that lists the packages it uses, named `conduit.yaml` or `conduit.json` (MAN-001). It carries `apiVersion` and is not one of the ten kinds of document: it has no `kind`, `metadata` or `spec` (MAN-003). Its other keys are `packages`, `exports`, `implementations`, and `address` and `version`, which a package's manifest states and the manifest of a project that is not published does not (MAN-004, PKG-017). `packages` is a map keyed by address, and each item gives exactly one `version`, that version's `digest`, and optionally `fetch` (NAM-006, MAN-005). `exports` is a list of names (PKG-018, MAN-007), and `implementations` is a map keyed by name (PKG-019, MAN-009). |
| `fetch` | The key of a manifest item that gives the place to fetch the package's archive from, where that differs from the item's address. Its value is a URI reference, and a relative one is resolved against the place of the manifest that holds it (NAM-012, MAN-006). The package is still identified by its address. |
| export | Something in a package that the package's manifest names so that others may refer to it: a document, which is a template or a contract, listed in `exports`, or an implementation, exported by its item under `implementations` (PKG-018, PKG-019, MAN-007). From outside the package it is reached only by a qualified reference. Its name is made of segments separated by dots and is at most 63 characters long (NAM-004). |
| re-export | An export that a package takes from another package. It is written in `exports` as a qualified reference and keeps the name the other package gives it (MAN-008). A selection of it resolves to the package that originally exports it (PKG-013). |
| core package | The package built into every engine, at the address `github.com/shrav-k/conduit/core`. It is written out as documents under [core/](core/README.md) and [schemas/](schemas/README.md). It exports the Operation contracts of the management verbs, of the retrieval tool and of its two controls; the capabilities that `management`, `access`, a Pool, the `system` Pool and a location offer; the plain Interface and Pool types and the types of the first-start objects; the requirements `bound.v1` and `shared-host.v1`; the reference schemas and the schemas of documents; and the templates of the first-start objects (OPS-019, IFC-024, ACC-029, OBJ-021, FST-005). Its one implementation, `engine`, is the engine itself (MAN-012). It needs no manifest item and cannot be replaced (NAM-020, NAM-021), and it has no digest. |
| unpublished package | What the engine keeps of the contracts submitted with a project that is not published. It is identified by a digest: the SHA-256 of the RFC 8785 form of one object that holds `documents`, the project's contract documents ordered by kind and then by name, and `manifest`, which is left out when there is none (SUB-008). A template registered with those contracts keeps that digest as it keeps the version and digest of any package (NAM-009). |
| selection | A document's choice of a type, a capability, an operation, a requirement or an implementation, written as a qualified reference or in the `#name` form (NAM-018). The keys of an `implementation`, `controls` or `tools` map are selections of operations (TPL-003, TPL-010, PKG-020). Implementations are selected in the one `implementation` map of a template's `spec`, keyed by operation. An operation that is not a key there takes the default its type states, or else the default the operation's own contract states (TPL-003, TPL-004). |
| digest | Of a package: the SHA-256 of its archive file exactly as distributed, written as `sha256:` followed by the hash in lowercase hexadecimal (PKG-004, PKG-022). The core package has none (MAN-012). The digest of an unpublished package is the one SUB-008 defines. |
| contract | A document of the kind `Type`, `Capability`, `Operation`, `Requirement` or `Schema`, which states what a type, a capability, an operation, a requirement or a schema is (CTR-001). Its name is its export name, and the last segment of the name, written `v` and a whole number from 1 as in `.v1`, is its compatibility version, with at least one segment before it (CTR-001): two names that differ only there are two contracts (NAM-015). Under one name a later release may only add optional things (NAM-024). A contract is not registered. |
| Schema contract | A contract whose whole `spec` is a schema under the schema profile (CTR-008). A `$ref` may name one by a qualified reference, or by `#name` within one project or package (SCH-015). The reference schemas, and the schemas of documents under [schemas/](schemas/README.md), are Schema contracts of the core package. |
| names a version assigns | The three names that a version of this specification states in [names.yaml](names.yaml): the address of the core package, the value of `apiVersion`, and the prefix of Conduit's keys (OBJ-024). Only a new version may change them. For this version they are `github.com/shrav-k/conduit/core`, `github.com/shrav-k/conduit/v0` and `io.github.shrav-k.conduit/`. |

## Graphs

| Term | Meaning |
| --- | --- |
| graph | A finite set of steps and the dependencies among them, written as a map that holds `steps` and, where its operation declares outputs, `outputs` (GRA-023). A graph implements exactly one operation (GRA-014). |
| step | One member of a graph's `steps`. It has exactly one of `operation` and `runtime`, which invokes the start operation of a runtime the template declares (GRA-017). A step with `operation` invokes the implementation selected for its own instance or, where it also has `conduit`, is a request through that declared Conduit to its target (GRA-024). |
| depends on | The relation that orders steps. A step depends on the steps its `dependsOn` lists, on any step whose output it uses, and on the declared Conduit its `conduit` names (GRA-001, GRA-018, GRA-024). An entry that refers to another entry depends on it in the same way (POOL-006). Order comes from this relation alone, never from the order of writing (GRA-012), and a step runs only after every step it depends on has succeeded (GRA-002). |
| value binding | How a step's argument, or a graph's output, gets its value. It is exactly one of `{literal: value}`, a value reference, `{object: {...}}` and `{list: [...]}` (GRA-019). |
| value reference | A value binding that takes its value from somewhere else. It is a map with `source`, and for some sources it may add `pointer`, a JSON Pointer into the value the source names (GRA-020). |
| `source` | The key of a value reference that says where the value comes from: `input`, `config`, `step`, `runtime` or `instance` (GRA-020). The word has no other use: where a record or an observation was made is its origin, and where a package is fetched from is `fetch`. |
| output | A named value that an operation produces, declared in its contract. A graph that implements the operation gives each output a `value` in its `outputs` map (GRA-021). |
| setup graph | The graph a template writes under `setup`, which implements the setup operation of its instances (GRA-014). The template's declared Conduits and, for a Pool, its entries are steps of it under their own names (GRA-022). |
| retire graph | The graph a template writes under `retire`, which implements the retire operation of its instances (TPL-008). It runs only for an instance for which a setup was ever dispatched (TPL-009). |

## Lifecycle

| Term | Meaning |
| --- | --- |
| lifecycle state | One of registered, ready and retired. Every instance of every family has exactly one, and it only moves forward (LIF-001, LIF-006). A template has none (LIF-012). |
| registered | The state an instance has when it is instantiated (LIF-002), and keeps until its setup succeeds or it is retired. As a description of an object the word names only this state. |
| ready | The state of an instance whose setup has succeeded (LIF-003). A Conduit that is ready is open. |
| retired | The state of an instance from the moment a request to retire it is dispatched, whatever the outcome (LIF-005). It is never brought back (LIF-006). A retired instance still exists until it is unregistered (LIF-007). |
| set up | Invoke an instance's setup operation: the steps of its `setup` graph, the opening of its declared Conduits and, for a Pool, the setting up of the Resources of its entries (TPL-012). The noun is setup. For a Conduit the verb is open. |
| retire | End an instance for good. Its `retire` graph runs, if it has one and a setup of it was ever dispatched, and then each runtime it still holds is stopped (TPL-009). For a Conduit the verb is close. |
| shutdown | The stopping and releasing of what a retired instance itself holds, as its type contract defines (LIF-016). The status reports it as confirmed, pending or unknown (LIF-014, LIF-024). |
| declare lost | An operation reserved to the root Interface, `declare-lost.v1`. It retires a location and every instance placed on it, without their shutdown being confirmed; PLC-017 says what the status of each then reports. |
| unregister | Remove a template or an instance, which then no longer exists. An instance can be unregistered only when it is retired and either its shutdown is confirmed or its location was declared lost (LIF-017). Its records are kept, and objects that referred to it keep its ID and report it as missing (LIF-019). |

## Inspection, observation and records

| Term | Meaning |
| --- | --- |
| inspect | Read an object. For an instance the result has three parts: what was declared, the status, and the records about it (LIF-013). `inspect.v1` returns them as `declared`, `status` and `records`, beside the object's `id`, `revision` and `packages`. Inspecting has no effect and gives no access (OBS-006). |
| declared | Said of the part of an inspection result that is the instance's resolved definition and configuration. It changes only by an operation that an actor requested, never because of an observation (OBS-001). |
| status | The part of an inspection result that the engine writes: the lifecycle state and what has been observed (LIF-013). A submitted document may not carry it (OBJ-004). |
| condition | Something further that a type's contract says it reports of its instances, such as running, paused or idle. It appears in the status beside the lifecycle state and never changes that state (LIF-011). |
| observation | Something seen of an instance, reported with the time it was made and its origin (OBS-002). What cannot be observed now is reported as unknown (OBS-003). A difference from what was declared is reported, and is not removed without a request (OBS-004). |
| origin | Where a record or an observation was made, such as one location. Records are in order within one origin, and no order is promised between origins (REC-005). |
| record | A log entry about a request or an event: who, what, when and how it ended (REC-002). REC-001 lists what is always recorded. A record is added and never altered, and only a purge removes one (REC-003). The word is used for nothing else: what is kept for an operation identity is the invocation's state. |
| purge | Remove the records made before a time the request names. It is an operation reserved to the root Interface, `purge.v1`, and itself leaves a record. It keeps, for every invocation, the operation identity, the outcome and a digest of the contents (REC-004). How that digest is computed is left to each engine, as entry DL2 of the implementation-defined list. |
| unknown | The one word for "cannot tell": for an outcome, for an observation, for a shutdown and for a setting that a control addressed. It is never a lifecycle state (LIF-001). |

## What is left to each engine

| Term | Meaning |
| --- | --- |
| implementation-defined | Left to each engine by this specification. [implementation-defined.yaml](implementation-defined.yaml) lists every such point, and an engine's choice for each must keep every promise. |
| runtime profile | An engine's document of its choice for every item of the implementation-defined list. |

## Words the promises do not use

| Term | Meaning |
| --- | --- |
| grant | Not used. The access an open Conduit carries is the operations it lists; it begins when the Conduit is opened and ends when the Conduit is closed (ACC-003, ACC-011). |
| declaration | Not used. What is submitted is a document, and an instantiation written inside a template is a nested instantiation. |
| create | Not used for an operation. An instance is instantiated, and that is done by registering its document. |
