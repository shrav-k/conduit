Not binding.

# The safety catalogue

The classification is the coordinator’s, open to the owner to strike under
[IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed).
Written from `spec/` as of commit `e94c057`. The proof lane updates this catalogue when a promise changes.

Promises read: 502; safety: 394; for the coordinator: 0; others: 108. Safety by kind: access 193,
secret 39, effect 119, result 43. Safety by layer: language 133, convention 217, interface 44.
Safety in the seven convention files: 128 (G5’s list as ruling 39 worded it); safety of the
convention layer anywhere: 217 (the wider reading, for the owner).

Recount the classification rows from the repository root:

```bash
awk '/^## /{s=$0} /^\| [A-Z]+-[0-9]+ \|/{n[s]++} END{for(k in n) print n[k], k}' docs/implementation/SAFETY_CATALOGUE.md
```

A safety promise guards an unsafe outcome under the [tracker’s definition](../AUDIT.md#how-to-use-this-document).
The test includes what binds a gate, provider or implementation, and counts overlapping
safeguards separately: access that should not be granted, secret or identity material
disclosed, an effect lost or repeated or performed without authorization, or a wrong
result or account of who did what. An unknown outcome treated as known is an effect breach.
Document validity alone, refusal of valid input and diagnostic wording alone do not qualify.
Each row selects the kind of the unsafe slice quoted in its reason; other slices may guard
other kinds too. Quoted text is from the named promise field, with folded whitespace.
S1 associations name requiring promises, not proof of coverage; these are proposed traces,
not results. The [specification](../../spec/README.md) alone binds behavior.

## Safety promises

### spec/promises/objects.yaml

Read: 24 promises. Source: [spec/promises/objects.yaml](../../spec/promises/objects.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| OBJ-002 | language | must | access | `observable`: “The existing object, its definition and its revision are unchanged” — overwriting an existing Conduit can widen its consumer’s access. | S1-duplicate |
| OBJ-003 | language | must | result | `observable`: “The result of `register.v1` lists each instance the request made, with its name and its ID, and says of a nested instance which nested instantiation made it” — returning a wrong ID or nested-source attribution gives a wrong output for the dispatched registration. | — |
| OBJ-004 | language | must | access | `when`: “a field the engine owns: `id`, `revision`, `status`, or `packages`” — admitting forged engine identity or state can substitute an authorization target. | — |
| OBJ-009 | language | must | access | `then`: “the kind and the ID of the object the reference resolved to” — storing a different target can authorize access to the wrong object. | S1-register-set |
| OBJ-010 | language | must not | access | `observable`: “A stored resolved form is not resolved again” — rebinding a grant after name reuse can give access to its replacement. | E12-name-reuse |
| OBJ-011 | language | must | access | `then`: “no document rewrites the definition of an instance” — rewriting an existing Conduit can widen the access already granted. | — |
| OBJ-021 | language | must | access | `observable`: “A value governed by a reference schema is resolved under OBJ-009” — misidentifying a bound’s target reference can apply authority to the wrong object. | — |

### spec/promises/schemas.yaml

Read: 21 promises. Source: [spec/promises/schemas.yaml](../../spec/promises/schemas.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| SCH-001 | language | must | access | `observable`: “A value is valid against the schema exactly when” — weakening the meaning of a restriction’s schema can admit arguments its author excluded. | E05-local, E05-siblings |
| SCH-004 | language | must | effect | `observable`: “Checking a `uri` makes no request to it” — fetching a URI merely to validate it performs an unauthorized request. | — |
| SCH-007 | language | must | result | `then`: “They are equal exactly when JSON Schema's definition of equality says so” — treating unequal keyed contents as equal can return an earlier invocation’s output for a different request. | — |
| SCH-009 | language | must | access | `then`: “the filled value is what is stored or sent” — sending a different default in filter arguments can widen the caller’s permitted slice. | — |
| SCH-010 | language | must | access | `then`: “The defaults of that map's own properties then apply to it” — omitting a nested restriction default can weaken the filter sent to an implementation. | — |
| SCH-011 | language | must not | access | `observable`: “Neither is converted” — coercing restriction arguments into different values can admit access the written restriction excludes. | — |
| SCH-015 | language | must | access | `then`: “Each of the last two references the schema that is that contract's `spec`” — resolving a restriction schema to another contract can validate access against the wrong constraint. | E05-local, E05-recursive, E05-escaping |
| SCH-016 | language | must | access | `observable`: “Whether a string matches is what an ECMA-262 regular expression with the `u` flag gives” — a more permissive pattern match can allow arguments excluded by a restriction schema. | — |
| SCH-020 | language | must | result | `observable`: “That value is what a schema checks and what SCH-007 compares” — sending a different numeric value can produce an output for arguments other than the request’s. | — |

### spec/promises/instantiation.yaml

Read: 23 promises. Source: [spec/promises/instantiation.yaml](../../spec/promises/instantiation.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| INS-005 | language | must | access | `then`: “No instance is made, and the document that supplies the argument is rejected” — admitting an argument that fails its input schema can grant operations that schema excludes even within the selected capability. | — |
| INS-006 | language | must | access | `then`: “The whole value is assigned to each destination the input names” — substituting or merging a Conduit target or restriction argument can widen the granted access. | — |
| INS-007 | language | must | access | `then`: “The default is assigned to each of its destinations” — a different default for a Conduit’s restriction can grant a broader slice. | — |
| INS-008 | language | must | access | `then`: “Each of its destinations keeps the value the template gives it” — discarding an unsupplied restriction’s template value can broaden access. | — |
| INS-009 | language | must | access | `observable`: “each destination holds null and no default is applied” — substituting a default for an explicit null can select a different Conduit target or restriction. | — |
| INS-012 | language | must | access | `observable`: “the `requirement` of an applied requirement” — accepting an input that rewrites the requirement contract can replace a grant’s constraint with a weaker one. | — |
| INS-013 | language | must | access | `then`: “The instance holds the complete resolved definition and configuration” — losing the resolved restriction configuration can widen an existing Conduit. | — |
| INS-014 | language | must | access | `then`: “No instance is made, and the instance document is rejected” — admitting configuration that fails the Type’s schema can permit disclosure beyond its allowed read limit. | — |
| INS-016 | language | must | effect | `observable`: “no implementation has received a call” — starting an implementation during registration performs work that registration did not authorize. | S1-register-set |
| INS-018 | language | must not | result | `observable`: “The value reaches its destination exactly as supplied” — rebasing a requested path can make a dispatched operation read or change a different file. | — |
| INS-019 | language | must | access | `then`: “The first of those that exists is used, in that order” — overriding an explicit restriction argument with a weaker default can widen the grant. | — |
| INS-020 | convention | must | access | `then`: “Each of those instances keeps the definition and configuration it resolved when it was made” — changing an existing Conduit when its template changes can widen its grant. | E11-replace |
| INS-023 | language | must | access | `when`: “an operation that its capability does not include” — admitting that operation can grant a call outside the selected capability. | — |

### spec/promises/templates.yaml

Read: 14 promises. Source: [spec/promises/templates.yaml](../../spec/promises/templates.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| TPL-003 | language | must | access | `observable`: “an operation that `engine` lists under MAN-012” — allowing a replacement implementation of core access management can bypass its authorization semantics. | — |
| TPL-004 | language | must | effect | `observable`: “A default that a type states for a hook it does not require is not used” — dispatching that unselected hook performs work the template did not authorize. | — |
| TPL-008 | language | must | result | `then`: “Each setup of it is one invocation of `set-up.v1` that names it” — misidentifying the setup or retirement invocation can record the wrong operation or target. | — |
| TPL-009 | convention | must | effect | `observable`: “the invocation succeeds only if each of them that it has succeeded” — reporting success despite an unconfirmed hook or stop treats an unknown effect as known. | — |
| TPL-010 | language | must | access | `observable`: “A control the map does not list is not available on the template's instances” — exposing an excluded control can permit configuration changes outside the template’s grantable surface. | — |
| TPL-011 | convention | must | access | `then`: “A request whose value fails either is not dispatched” — dispatching a control outside the template’s narrowed limits grants an excluded change. | E05-narrowed |
| TPL-012 | convention | must | effect | `then`: “A part is begun only once every part that comes before it in that order, and is not marked optional, has succeeded” — running a later setup part after refusal or cancellation performs work that setup did not authorize. | S1-setup-sender |
| TPL-013 | convention | must | access | `then`: “The request is not dispatched” — dispatching an absent control grants an operation the instance excludes. | E05-narrowed |
| TPL-017 | convention | must | access | `observable`: “It carries the filters of the invocation of `set-up.v1` that it belongs to” — dropping a setup filter lets the hook exceed its caller’s restricted access. | — |
| TPL-018 | convention | must | effect | `then`: “where it reports the run's state `unknown`, the hook is not dispatched” — requesting another run in that case can duplicate the running effect. | — |
| TPL-019 | language | must | effect | `then`: “an Agent or a Resource has at most one item that is run” — admitting multiple run items can start an additional implementation the instance does not authorize. | — |

### spec/promises/contracts.yaml

Read: 11 promises. Source: [spec/promises/contracts.yaml](../../spec/promises/contracts.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| CTR-002 | language | must | effect | `then`: “applies the effect at most once for it” — an implementation that disregards its deduplication declaration can repeat an effect after a lost response. | — |
| CTR-003 | language | must | access | `then`: “A capability that is not a contract of the core package and that lists an operation reserved to the root Interface is rejected” — admitting the capability can expose root-only operations through another grantable surface. | — |
| CTR-004 | language | must | access | `observable`: “One that states no `offers` offers no capability” — inventing a capability for such a type makes otherwise unoffered operations reachable. | — |
| CTR-005 | language | must | access | `then`: “apply it to each call that they receive for a request through the Conduit” — omitting an applied filter lets a call exceed its Conduit’s restriction. | — |
| CTR-006 | language | must | result | `then`: “No two pointers of its `changes` are equal, and none lies inside another” — admitting overlapping changes can make the stored settings disagree with the change reported for a dispatched control. | — |
| CTR-013 | language | must | secret | `observable`: “The engine reads only what account-hook reports, never the instance's state directory” — reading the directory can disclose the client’s credential material outside its intended holder. | — |

### spec/promises/naming.yaml

Read: 28 promises. Source: [spec/promises/naming.yaml](../../spec/promises/naming.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| NAM-009 | language | must | effect | `observable`: “the instances made from each run that directory's programs” — substituting another directory’s implementation executes code the registered template did not select. | E11-replace |
| NAM-010 | language | must | access | `then`: “through the packages the template holds” — resolving a restriction against a later manifest can silently weaken an existing grant. | — |
| NAM-011 | language | must | access | `then`: “continues to hold, and to use, the version and digest it held before” — substituting a package on removal can change a live Conduit’s restrictions. | — |
| NAM-012 | language | must | effect | `observable`: “Reading or registering the manifest fetches nothing and installs nothing” — fetching on registration performs work that no install request authorized. | S1-no-fetch |
| NAM-013 | language | must | access | `observable`: “the objects that exist are exactly those that existed before it” — registering exported Conduits during installation can grant access without an object-registration request. | — |
| NAM-014 | language | must | access | `then`: “through that package's own manifest” — substituting a project’s version can bind an applied restriction to a weaker contract. | E05-two-contexts |
| NAM-015 | language | must | access | `observable`: “A Conduit that names the `.v1` capability is not satisfied by a target that offers only `.v2`” — treating incompatible capabilities as interchangeable can grant access the named contract does not cover. | — |
| NAM-019 | language | must | access | `observable`: “the argument is kept so filled” — losing a restriction’s normalized default can send a weaker filter than the document declares. | — |
| NAM-020 | language | must | access | `then`: “to the core package built into the engine” — resolving core access contracts to another package can substitute their authorization meaning. | — |
| NAM-021 | language | must | access | `then`: “The core package cannot be replaced” — admitting a replacement can substitute the built-in access contracts with weaker ones. | — |
| NAM-022 | language | must | access | `then`: “in the same project, or the same package, as the document that holds it” — looking elsewhere can substitute a different filter contract for the one a Conduit declares. | E11-replace |
| NAM-023 | convention | must | effect | `observable`: “A result whose outputs fail that version has the outcome unknown” — accepting an invalid output as confirmed treats an unverified invocation as known. | E05-two-contexts |
| NAM-024 | language | must | access | `then`: “A property that it adds to a Type's `schema`, at any depth, and an input that it adds to a Requirement contract, state no `default`” — adding a reach-enabling default can widen access when an unchanged document is registered after a pin moves. | — |
| NAM-025 | language | should | result | `observable`: “A contract of one name means the same, apart from the additions NAM-024 allows, in every release at one address” — changing a contract’s effect under the same identity can report a different effect as the requested one. | — |
| NAM-026 | convention | must | access | `then`: “The request is not dispatched” — dispatching an operation absent from the target’s capability grants an operation that capability does not offer. | — |

### spec/promises/sections.yaml

Read: 15 promises. Source: [spec/promises/sections.yaml](../../spec/promises/sections.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| SCT-003 | language | must | effect | `observable`: “Registration starts no work, and installation registers nothing” — running a translated project during registration performs unauthorized work. | — |
| SCT-004 | language | must | access | `observable`: “Nothing inside `config`, `arguments`, `implementation`, `controls`, canonical declared `conduits`, canonical `entries`, or requirement arguments is rewritten” — rewriting restriction arguments during translation can widen the resulting grant. | — |
| SCT-005 | language | must | access | `then`: “Its body is exactly the canonical template `spec` of that family” — a translation that changed the body, such as the operations a Conduit template lists, would grant access the author did not write. | — |
| SCT-006 | language | must | access | `then`: “go unchanged to the template's `spec`” — changing translated controls or declared Conduits can grant operations the authored definition excludes. | — |
| SCT-007 | language | must | access | `observable`: “Operations are never grouped or inferred” — inferring extra operations for a consumes item can grant calls the author did not list. | — |
| SCT-009 | language | must | access | `then`: “whose consumer is `{kind: Interface, name: root}`” — assigning the operator grant to another consumer can expose root’s intended access to it. | — |
| SCT-011 | language | must | access | `then`: “supplies X as the owner of every Agent instance defined anywhere in that file” — assigning an Agent to another Fleet gives that Fleet’s administrators authority over the wrong Agent. | — |
| SCT-012 | language | must | access | `observable`: “View lists only `list.v1` and `inspect.v1`” — adding management operations to view scope grants changes to a reader. | — |
| SCT-014 | language | must | secret | `observable`: “No value at a place governed by a secret reference is echoed” — echoing a submitted literal there can disclose a secret in a diagnostic. | — |
| SCT-015 | language | must | effect | `observable`: “It supplies no implicit setup, opening, retirement, repeat or change of a pin” — using registration equivalence to perform those actions implicitly can execute unauthorized effects. | — |
| SCT-016 | language | must | access | `then`: “with the written `consumer` and `target`” — changing either endpoint during translation can give the wrong party access to the wrong target. | — |

### spec/promises/packages.yaml

Read: 23 promises. Source: [spec/promises/packages.yaml](../../spec/promises/packages.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| PKG-003 | language | must | access | `observable`: “No newer or compatible version is substituted” — substituting a release can silently change the restriction contracts a grant uses. | — |
| PKG-004 | language | must | effect | `then`: “the SHA-256 of the package's archive file exactly as distributed” — hashing different bytes can let a substituted executable pass the pin and run without authorization. | — |
| PKG-005 | language | must | access | `then`: “the version and digest that the template holds under NAM-009” — using another package can change the restriction configuration a Conduit resolves. | E11-replace |
| PKG-006 | language | must | effect | `observable`: “A registration fetches no package and installs none” — fetching during registration performs an unauthorized installation request. | S1-no-fetch |
| PKG-016 | interface | must not | result | `observable`: “the same whether or not it has also served others” — cross-instance state contamination can return another instance’s output for the dispatched request. | — |
| PKG-017 | language | must | effect | `observable`: “an address or a version other than the one it was listed, asked for or fetched under is rejected” — admitting that substitution can later execute code outside the package identity requested. | — |
| PKG-019 | language | must | access | `then`: “An item that leaves out `supports` supports no requirement” — assuming filter support can dispatch restricted calls to an implementation that cannot enforce them. | — |
| PKG-020 | language | must | access | `then`: “It states no `supports`” — admitting filter support on a plain implementation can send a restricted call to a server that ignores its filter. | S1-plain-filter |
| PKG-021 | language | must | effect | `observable`: “The archive is not used” — using bytes with a different digest can execute an implementation the manifest did not authorize. | — |
| PKG-023 | convention | must | effect | `then`: “The request is not dispatched” — dispatching an installation whose unsafe archive was rejected can write outside the extraction boundary. | — |
| PKG-024 | convention | must | result | `observable`: “the packages that are installed are as they were” — changing installed packages after a failed fetch can report an installation state different from what occurred. | — |
| PKG-025 | convention | must | effect | `then`: “nothing is fetched for an address and version that is already installed” — fetching for this conflict performs a request excluded by the installation’s current state. | — |
| PKG-026 | language | must | effect | `observable`: “The archive is not used” — accepting different bytes can later run an implementation the install request did not authorize. | — |
| PKG-027 | convention | must | effect | `then`: “nothing is fetched for an address and version that is already installed” — fetching again for the completed installation performs an unnecessary unauthorized fetch. | — |
| PKG-028 | language | must | effect | `observable`: “No package is fetched or installed because another package lists it” — automatic dependency installation performs work no actor requested. | — |
| PKG-029 | convention | must | access | `observable`: “the contracts of the submitted project keep that address” — letting an installed package take that address can redirect an existing named bound to different operations. | — |
| PKG-031 | language | should | access | `observable`: “a plan prints and counts any widening of the resolved Conduit operations before apply” — silently applying a widened template can grant operations the author did not knowingly approve. | — |

### spec/promises/manifest.yaml

Read: 17 promises. Source: [spec/promises/manifest.yaml](../../spec/promises/manifest.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| MAN-006 | language | must | secret | `then`: “It carries no user information, the part of a URI where RFC 3986 places a credential” — admitting a credential-bearing fetch reference can expose the credential through ordinary manifest handling. | — |
| MAN-010 | language | must | effect | `observable`: “no shell reads the list” — interpreting command arguments as shell syntax can execute an additional effect the item never requested. | — |
| MAN-011 | language | must | access | `then`: “Nothing in a directory below it is read” — registering an unintended nested project can create Conduits outside the submitted project’s declared set. | — |
| MAN-012 | language | must | access | `then`: “Its implementations are the engine itself” — substituting an external implementation for core access operations can bypass their authorization semantics. | — |
| MAN-014 | language | must | effect | `observable`: “nothing is written outside the place where the engine keeps the package” — extracting an escaping archive member can overwrite a file outside the authorized installation boundary. | S1-archive-escape |
| MAN-016 | language | must | access | `observable`: “the project's contracts cannot be named by it as well” — admitting the address collision can make an existing bound cover another contract under the same name. | — |
| MAN-018 | language | must | secret | `observable`: “a value of its choosing could send the run's credential to an address of its own” — allowing configuration to override identity variables can disclose the run’s credential. | — |

### spec/promises/submissions.yaml

Read: 11 promises. Source: [spec/promises/submissions.yaml](../../spec/promises/submissions.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| SUB-002 | language | must | access | `then`: “The reference resolves to that object” — binding a submitted Conduit to another target grants access outside its declared relationship. | S1-register-set |
| SUB-003 | language | must | effect | `then`: “Nothing in the submission is registered” — partial registration performs a state-changing effect despite reporting rejection of the whole request. | S1-register-set, S1-duplicate, S1-target-type |
| SUB-006 | language | must | effect | `then`: “Nothing else happens” — starting setup or a run on accepted registration performs work the request did not authorize. | — |
| SUB-007 | language | must | access | `then`: “It never replaces a template” — replacement by registration bypasses revision-checked change and can alter future grants. | — |
| SUB-008 | language | must | access | `observable`: “a template and its instances reach only their own submission” — mixing submissions at one address can substitute a weaker restriction contract for the one resolved. | E11-replace |
| SUB-009 | convention | must | effect | `observable`: “The request replaces one template and registers nothing” — registering additional objects during replacement performs effects outside the requested single-template change. | E11-replace |
| SUB-010 | language | must | effect | `then`: “It installs no package” — automatic installation while registering a directory performs an unauthorized fetch or install. | S1-no-fetch |

### spec/promises/pools.yaml

Read: 15 promises. Source: [spec/promises/pools.yaml](../../spec/promises/pools.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| POOL-001 | language | must | access | `then`: “No operation changes it” — moving a Resource to another owner gives that Pool’s administrators authority they were not granted. | — |
| POOL-002 | language | must | effect | `observable`: “Registering the Pool's template instantiates no Resource” — instantiating entries on template registration creates objects the request did not ask to create. | — |
| POOL-003 | convention | must | effect | `observable`: “The Pool owns one Resource for the entry before and after the request” — creating another Resource despite conflict performs an unreported registration effect. | — |
| POOL-004 | language | must | access | `observable`: “A retired Pool gains no Resource” — admitting a new Resource under a retired owner can extend grants scoped to that Pool beyond its ended lifecycle. | — |
| POOL-006 | language | must | access | `then`: “The reference resolves to the Resource instantiated from the other entry” — resolving a Conduit target or bound to another entry can grant access to the wrong Resource. | — |
| POOL-007 | convention | must not | effect | `then`: “The Resource of any other entry is set up because of it” — implicitly setting up another entry starts work the individual Resource request did not authorize. | — |
| POOL-009 | language | must | access | `then`: “the Resource of that entry takes the placement, or the runtime provider, supplied for the input” — substituting another runtime or placement can give the Resource access outside the selected confinement. | — |
| POOL-013 | convention | must | effect | `then`: “For a Resource that is already ready nothing is dispatched” — dispatching its setup again can repeat a hook or other completed effect. | — |
| POOL-014 | language | must | access | `observable`: “with that Resource's ID” — binding the entry reference to another Resource can redirect a grant to an unintended target. | — |
| POOL-015 | convention | must | access | `then`: “by that instance's own open Conduit to the Pool” — borrowing the original caller’s or another instance’s authority permits unauthorized entry setup. | — |
| POOL-016 | convention | must | result | `then`: “The Resource of that entry is placed under PLC-002, by the machine provider under PRV-015” — an entry, or its replacement, placed on another machine would serve a dispatched request from the wrong machine's state. | — |
| POOL-017 | convention | must | access | `then`: “The request is not dispatched” — creating an entry after its owning Pool retires can extend Pool-scoped access past the owner’s lifecycle. | — |

### spec/promises/fleets.yaml

Read: 6 promises. Source: [spec/promises/fleets.yaml](../../spec/promises/fleets.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| FLEET-001 | language | must | effect | `observable`: “It has no config, placement, runtime, run, hooks, entries or declared Conduits” — starting a run or hook for a Fleet performs work that Fleet setup never authorizes. | — |
| FLEET-002 | language | must | access | `then`: “fixed for its life; no operation changes it” — moving an Agent to another Fleet gives that Fleet’s administrators unintended authority over it. | — |
| FLEET-003 | language | must | access | `observable`: “needs both `management` and that Fleet” — registering an Agent without its existing Fleet’s authority creates an administrable member without the required grant. | — |
| FLEET-004 | convention | must not | access | `observable`: “Running turns takes a Conduit to the Agent” — treating ownership or Fleet administration as permission to run a turn grants unlisted Agent access. | — |
| FLEET-005 | convention | must | access | `observable`: “names the other Conduit only to a caller permitted to inspect it” — an unfiltered local-name collision diagnostic can disclose another consumer’s Conduit. | — |

### spec/promises/placement.yaml

Read: 21 promises. Source: [spec/promises/placement.yaml](../../spec/promises/placement.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| PLC-001 | language | must | result | `then`: “with the node's ID” — storing another placement can make a dispatched path-based operation return data from the wrong machine. | — |
| PLC-002 | convention | must | result | `then`: “its placement is the node that represents the machine that the machine provider names for it under PRV-015” — another node substituted for the one named would serve a dispatched request from the wrong machine. | — |
| PLC-007 | language | must | access | `then`: “The instance is not instantiated” — admitting a placement excluded by the workload’s selector can disclose its requests to an unselected machine. | — |
| PLC-008 | language | must | access | `then`: “no instance document makes one” — admitting an actor-made node can introduce an unauthorized machine into placement and shared-host decisions. | — |
| PLC-009 | language | must not | result | `observable`: “No operation changes the placement of an existing instance” — moving an existing path-based service can return outputs from a different machine under the same instance identity. | — |
| PLC-012 | convention | must not | access | `observable`: “reports refused unless an open Conduit lists the operation” — granting access merely for co-location permits an unlisted request. | — |
| PLC-013 | language | must | access | `then`: “The Conduit is not instantiated” — admitting a cross-host Conduit despite shared-host grants a relationship its restriction excludes. | — |
| PLC-017 | convention | must | effect | `then`: “The status of each reports its shutdown as unknown” — reporting shutdown confirmed on loss treats a potentially running effect as stopped. | — |
| PLC-018 | language | must | effect | `then`: “The instance is not instantiated” — admitting a workload on a retired node can authorize new runs on a machine whose lifecycle ended. | — |
| PLC-019 | convention | must | effect | `observable`: “no implementation receives a call for its setup” — dispatching setup before its node is ready starts work outside the setup preconditions. | — |
| PLC-020 | convention | must not | result | `observable`: “never opens it, expands it or checks it against the machine the engine itself runs on” — interpreting an instance’s path on the engine machine can produce an output about the wrong file. | — |
| PLC-023 | convention | must | effect | `observable`: “what cannot be observed of it is reported as unknown” — treating an unreachable run as stopped can authorize a duplicate run. | — |
| PLC-024 | convention | must | access | `observable`: “The Conduits of which any of them is an end are closed under CON-008” — leaving grants usable after the machine is gone permits access past retirement. | — |
| PLC-026 | convention | must | access | `observable`: “a retired node represents none” — reusing a retired node for a returned machine can revive relationships whose original machine was lost. | — |
| PLC-027 | convention | must | effect | `then`: “the hook is not dispatched, no run is asked for and no secret is read for one” — starting a retirement hook on a lost node performs work beyond that node’s ended authority. | — |
| PLC-028 | language | must | access | `observable`: “no actor registers a node, through that template or through one of its own” — accepting a substitute node template lets an actor introduce a machine without the engine’s admission path. | — |

### spec/promises/conduits.yaml

Read: 22 promises. Source: [spec/promises/conduits.yaml](../../spec/promises/conduits.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| CON-001 | language | must | access | `then`: “exactly one consumer, exactly one target and exactly one capability” — adding an endpoint can give another party access outside the declared relationship. | — |
| CON-002 | language | must | access | `then`: “with the new instance as its consumer” — assigning declared grants to another consumer can give that party unintended access. | — |
| CON-004 | convention | must | access | `then`: “Setup instantiates no Conduit” — recreating a closed declaration during setup can restore access that was revoked. | S1-setup-sender, S1-setup-again, S1-restore |
| CON-006 | language | must | access | `then`: “The document is rejected” — accepting shared-host for an end that runs nowhere falsely permits a relationship the co-location restriction cannot establish. | — |
| CON-008 | convention | must | access | `then`: “From the moment the retirement of a target is dispatched” — delaying closure can dispatch requests after the target’s access ended. | — |
| CON-011 | convention | must | result | `observable`: “names the closed Conduit and the capability it carried as no longer reachable through it” — reporting the closed route as reachable gives a wrong inspection output. | — |
| CON-012 | language | must | access | `then`: “the consumer's definition is not changed” — rewriting the consumer’s declared relationships while adding a standalone grant can widen other access. | — |
| CON-013 | convention | must | effect | `then`: “without setting up the consumer again” — an implicit setup during opening can repeat a hook or start work outside the open request. | — |
| CON-014 | convention | must | effect | `then`: “The consumer is not retired or interrupted” — closing one relationship must not stop the consumer’s run or terminate unrelated work without a retirement request. | S1-close |
| CON-017 | language | must | access | `then`: “each of an operation that capability includes” — admitting an out-of-capability operation makes an unoffered call grantable. | S1-target-type, S1-exact-access |
| CON-018 | language | must | access | `observable`: “which the engine applies to each request through that Conduit” — failing to apply the bound permits grants outside the delegator’s declared limits. | — |
| CON-020 | language | must | access | `observable`: “the consumer of a declared Conduit is always the instance being instantiated” — accepting a declared consumer override can assign that instance’s intended access to another party. | — |
| CON-021 | language | must | access | `then`: “The Conduit is not instantiated” — admitting a capability the target does not offer can grant access outside its declared interface. | S1-target-type |
| CON-022 | language | must | access | `then`: “The Conduit is not instantiated” — admitting an unsupported filter can dispatch restricted calls to an implementation that cannot enforce it. | S1-plain-filter |
| CON-023 | convention | must | access | `observable`: “A Conduit whose target is retired is already closed under CON-008, and is never opened” — opening it can restore access after the target’s retirement. | S1-target-registered |
| CON-024 | convention | must | effect | `then`: “The request is not dispatched” — dispatching non-management work before setup succeeds performs work outside the target’s ready-state preconditions. | S1-target-registered |
| CON-025 | language | must | access | `then`: “The Conduit is not instantiated” — creating a grant for a retired endpoint can introduce access after its lifecycle ended. | — |
| CON-027 | language | must | access | `observable`: “A Pool or Fleet given as a consumer” — admitting such a consumer creates an authorization principal this version does not permit. | — |
| CON-031 | convention | must | access | `then`: “The request is not dispatched” — opening a registered grant after an endpoint retires revives access past its lifecycle. | — |

### spec/promises/first-start.yaml

Read: 6 promises. Source: [spec/promises/first-start.yaml](../../spec/promises/first-start.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| FST-001 | language | must | access | `observable`: “it means the object made at first start” — treating a later same-named object as root or access can confer the first-start object’s authority on another principal. | bootstrap |
| FST-002 | convention | must | access | `observable`: “with root as consumer” — assigning a bootstrap management or access Conduit to another consumer grants it root’s intended powers. | bootstrap |
| FST-003 | convention | must not | effect | `then`: “any implementation other than those behind the first-start objects has received a call” — calling another implementation at first start performs work no actor requested. | bootstrap |
| FST-005 | language | must | result | `observable`: “reports a `from` that is a qualified reference” — reporting a different source template gives a wrong inspection result for a first-start object. | bootstrap |
| FST-006 | language | must | access | `then`: “the engine alone makes nodes” — admitting an actor-created node bypasses the engine’s machine admission path. | — |
| FST-007 | convention | must | access | `observable`: “A generated domain is never a shared fixed name such as `conduit.local`” — sharing a trust namespace can make another engine’s identity appear to belong to this engine’s domain. | — |

### spec/promises/providers.yaml

Read: 29 promises. Source: [spec/promises/providers.yaml](../../spec/promises/providers.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| PRV-002 | convention | must | effect | `observable`: “that server ends when the engine stops” — leaving it running and running it again after restart can duplicate the provider’s run. | — |
| PRV-003 | convention | must | secret | `observable`: “delivers them as PRV-011 and PRV-022 have a runtime provider deliver them” — the built-in runner can disclose credentials if it ignores the providers’ delivery constraints. | — |
| PRV-004 | language | must | access | `then`: “no Conduit permits an operation of those capabilities” — exposing run or stop through a Conduit lets an instance control another instance’s execution. | — |
| PRV-005 | convention | must | result | `then`: “that request's `caller`” — sending the wrong caller misattributes a provider action made for a request. | — |
| PRV-006 | language | must | access | `then`: “for its whole life, and no operation changes it” — silently replacing an instance’s chosen confining runtime can expose its requests beyond that runtime’s boundaries. | — |
| PRV-007 | convention | must | effect | `then`: “Where the runtime provider reports the run `unknown`, the engine asks for no run” — asking again can duplicate a run that is still executing. | S1-setup-again |
| PRV-008 | convention | must | effect | `observable`: “Where the runtime provider cannot tell whether the run stopped, the stop ends unknown” — confirming shutdown without knowing it stopped treats a live effect as ended. | — |
| PRV-009 | convention | must | effect | `observable`: “Where the runtime provider has not reported the run's state, the condition's `status` is `Unknown`” — reporting stopped without evidence can authorize a duplicate run or a false reconciliation. | — |
| PRV-010 | convention | may | effect | `observable`: “dispatches nothing because of it” — dispatching an operation merely because a run restarts can repeat an effect without a caller request. | — |
| PRV-011 | convention | must | secret | `then`: “only over a channel that authenticates that machine and keeps the value confidential” — sending a secret without those checks discloses it to an unintended peer or observer. | — |
| PRV-012 | convention | must | effect | `then`: “The run ends failed, and nothing is run” — executing an unsupported artifact or transport performs work excluded by the runtime’s accepted execution contract. | — |
| PRV-013 | convention | must | access | `then`: “each is owned by `system`” — allowing another Pool to supply the active machine or package-source provider bypasses system-scoped authority over those services. | — |
| PRV-014 | convention | must | effect | `then`: “`gone` only where the provider knows that the machine no longer exists” — treating silence as disappearance can cause a replacement while the original run still acts. | — |
| PRV-015 | convention | must | access | `then`: “whose labels match the selector” — placing a workload on an excluded machine discloses its requests beyond the selected placement constraint. | — |
| PRV-016 | language | must | access | `observable`: “nothing changes the schemes that a provider claims, for its whole life” — mutable claims let a control redirect fetches to a provider outside the original scheme assignment. | — |
| PRV-017 | convention | must | effect | `then`: “Either way the archive is checked against the digest under PKG-026” — using unverified fetched bytes can run code the install request did not authorize. | — |
| PRV-018 | convention | must not | access | `observable`: “Only what the request presents establishes the instance” — trusting a network address as identity lets another process use an instance’s grants. | — |
| PRV-019 | convention | must | result | `then`: “on the machine given in `machine`, and on no other” — running a path-based service on another machine produces outputs or effects for a different file than requested. | — |
| PRV-021 | convention | must | effect | `then`: “neither a restart of the engine, on its machine or on another, nor the provider's own retirement loses” — losing the pending stop leaves its requested shutdown effect unperformed. | — |
| PRV-022 | convention | must | secret | `then`: “It puts them nowhere else” — disclosing identity files to another party lets that party present as the run’s instance. | — |
| PRV-023 | convention | must | secret | `observable`: “passes no token exported in the shell that started the engine” — inheriting an unlisted token exposes the engine user’s credential to the run. | — |
| PRV-024 | convention | must not | secret | `then`: “in a record, an inspection result, an observation, a diagnostic, any result” — publishing identity material through those surfaces discloses credentials to readers. | S1-records |
| PRV-025 | convention | must | access | `then`: “only where the request carries cryptographic proof that an enforcement point sent it” — admitting an unproved request lets a caller bypass every Conduit check. | — |
| PRV-026 | convention | must | access | `then`: “the filters on that Conduit as they stand when the read is made” — reusing former secret-read authority after restart can deliver a secret after its access was revoked. | — |
| PRV-027 | convention | must | effect | `then`: “in `instance` the instance's ID and in `machine` the machine of the instance's node” — misaddressing a stop can leave the requested run alive while stopping a different run. | — |
| PRV-028 | convention | must | secret | `then`: “before it is sent anything, that it serves the instance” — sending first can disclose request contents or credentials to a process that took the run’s URL. | — |
| PRV-029 | convention | must | secret | `observable`: “passes it to no other run” — reusing or handing over an instance’s state directory can disclose its client login to another instance. | — |

### spec/promises/pending.yaml

Read: 11 promises. Source: [spec/promises/pending.yaml](../../spec/promises/pending.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| PND-001 | convention | must | access | `then`: “Raising one permits nothing” — treating a pending action as a grant allows an operation before any authorized answer. | — |
| PND-002 | convention | must | access | `observable`: “with no count of the rest” — showing hidden subjects or their counts discloses objects outside the Interface’s inspection scope. | — |
| PND-003 | convention | must | effect | `observable`: “Expiry starts no hook, setup or model work” — resuming work on expiry executes an effect no actor requested. | — |
| PND-004 | interface | must | secret | `then`: “Values of kind code or fingerprint appear in no elicitation message, tool result, _meta, notification or record of a request” — publishing a device code on those surfaces can disclose sign-in material to an unintended reader. | — |
| PND-009 | convention | must | access | `observable`: “An action not raised at the caller reports refused before invalid” — disclosing or answering another Interface’s action bypasses its inspection scope. | — |
| PND-012 | convention | must | secret | `then`: “a request record holds no code or fingerprint value” — recording a device code exposes sign-in material to ordinary request-record readers. | — |
| PND-005 | convention | must | effect | `observable`: “Nothing is queued, resumed or dispatched after it ends” — resuming a held model request after sign-in starts work without a new request. | — |
| PND-010 | convention | must | access | `then`: “A caller other than root signs in only an Agent of a Fleet in its scope; Resources remain root's” — allowing a broader sign-in lets an Interface alter another party’s account access. | — |
| PND-011 | convention | must | effect | `observable`: “holds back all model work” — running model work after an unsuccessful account undo spends work under an account the instance did not authorize. | — |
| PND-006 | convention | must | effect | `observable`: “Computation adds no record and dispatches nothing” — dispatching setup while computing its pending action can repeat an effect implicitly. | — |
| PND-007 | convention | must | access | `observable`: “Evidence is disclosed under PND-002 and PND-004” — publishing unfiltered notice evidence discloses closed grants beyond the Interface’s scope. | — |

### spec/promises/secrets.yaml

Read: 24 promises. Source: [spec/promises/secrets.yaml](../../spec/promises/secrets.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| SEC-001 | language | must | secret | `then`: “any other value there, a literal or a bare name among them, makes the document that holds it rejected” — admitting a literal at a secret place can store and expose the secret as ordinary configuration. | — |
| SEC-002 | language | must | result | `then`: “The document that holds the reference is rejected” — accepting a non-secret provider as the secret target can report an unrelated value as the requested secret read. | — |
| SEC-003 | language | must not | secret | `then`: “The diagnostic holds the value, or any part of it” — echoing a rejected literal can disclose the secret to diagnostic readers. | — |
| SEC-004 | convention | must not | secret | `observable`: “holds no part of the string” — recording a rejected secret literal exposes it to request-record readers. | — |
| SEC-005 | language | must | secret | `observable`: “the locator that a Conduit to it leads to is fixed” — changing the locator behind an existing read grant can disclose a secret the reader was never granted. | — |
| SEC-006 | interface | must | secret | `then`: “never from any other store” — falling back to another store can disclose a secret outside the granted locator. | — |
| SEC-007 | convention | must | secret | `then`: “The request is not dispatched” — dispatching a consumer’s direct get-secret request delivers a value only the engine may receive. | — |
| SEC-008 | convention | must | secret | `then`: “only where the root Interface sent it on its own behalf” — allowing another actor to open secret reads gives it power to disclose secrets to a chosen consumer. | — |
| SEC-009 | language | must | secret | `then`: “The Conduit is not instantiated” — granting an Interface a secret-read relationship admits a consumer excluded from secret delivery. | — |
| SEC-010 | convention | must | secret | `then`: “through the instance's own open Conduit to that Resource that lists the operation” — borrowing another instance’s Conduit can deliver a secret to an unentitled run. | — |
| SEC-011 | convention | must | result | `then`: “It has that invocation's caller as its caller” — attributing a secret read to another caller falsely records who caused it. | — |
| SEC-012 | convention | must | access | `then`: “The engine asks for no run, makes no connection and sends no HTTP request on a connection” — proceeding after a revoked secret read bypasses that read’s current access decision. | — |
| SEC-013 | convention | must | secret | `then`: “only to the item's URL” — sending the header to another URL discloses the secret to an unintended recipient. | — |
| SEC-014 | convention | must | access | `observable`: “Closing the Conduit ends every later read through it” — refreshing a value after that closure delivers a new secret read beyond ended access. | — |
| SEC-015 | convention | must not | secret | `then`: “The record holds the value read, or any part of it” — including a secret value exposes it to readers of the read’s record. | — |
| SEC-016 | convention | must | access | `then`: “It is not dispatched” — reading despite unavailable dispatch recording bypasses the explicit precondition on secret access. | — |
| SEC-017 | convention | must not | access | `observable`: “each a request with its own access decision and record” — using a cached value instead of a new read can deliver a secret after its read access ended. | — |
| SEC-018 | convention | must not | secret | `observable`: “The result of the read reaches the engine and no instance” — returning that result to an instance exposes the raw value outside the permitted delivery path. | — |
| SEC-019 | interface | must not | secret | `then`: “into a result, a diagnostic, or the `arguments` or `_meta` of a request” — echoing a delivered value discloses it beyond the external service it was for. | — |
| SEC-020 | convention | must not | secret | `observable`: “The engine passes no token that the user has exported in the engine's environment to any run” — passing such a token in run inputs exposes a credential outside its explicit store-backed read path. | — |
| SEC-021 | language | must | secret | `observable`: “a token would be stored, shown on inspection and recorded as it was written” — admitting a declared secret binding to ordinary data discloses it through configuration surfaces. | — |
| SEC-022 | language | must | secret | `observable`: “no request that an actor sends is dispatched to the secret Resource's implementation” — offering another capability there lets an actor reach a store implementation outside the engine-only read path. | — |
| SEC-023 | convention | must | access | `then`: “the filters on that Conduit as they stand when the read is made” — recovering a connection using former authority can read a secret after revocation. | — |
| SEC-024 | convention | must | secret | `observable`: “cross only `https`, or `http` to the machine the engine runs on where the engine never runs on another” — using an insecure or rebound loopback destination can disclose a secret to an observer or another process. | — |

### spec/conventions/lifecycle.yaml

Read: 38 promises. Source: [spec/conventions/lifecycle.yaml](../../spec/conventions/lifecycle.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| LIF-001 | convention | must | result | `then`: “It reports exactly one lifecycle state: registered, ready or retired” — reporting another or inconsistent state gives a wrong inspection output. | — |
| LIF-002 | language | must | access | `then`: “Its state is registered” — making a new Conduit ready immediately grants access before an explicit open. | — |
| LIF-003 | convention | must | result | `observable`: “reports the state ready” — reporting another state after successful setup gives an incorrect inspection output. | S1-target-registered |
| LIF-004 | convention | must | effect | `then`: “Its state is unchanged” — marking an unsuccessful open ready treats an unknown or failed opening as confirmed. | S1-setup-sender |
| LIF-005 | convention | must | access | `then`: “Its state is retired from that moment” — delaying the transition permits new calls after retirement was dispatched. | — |
| LIF-006 | convention | must not | access | `observable`: “No operation moves a ready instance to registered, or a retired instance to ready or registered” — reviving a retired Conduit restores ended access. | — |
| LIF-007 | language | must | effect | `then`: “until it is removed: under LIF-035 once its shutdown is confirmed, or under LIF-017 by a request to unregister it” — removing it earlier can lose the state needed to stop or settle its still-running effects. | — |
| LIF-008 | convention | must not | access | `observable`: “the consumer of a closed declared Conduit has no access through it” — recreating the declaration implicitly restores revoked access. | S1-restore |
| LIF-009 | convention | must | access | `then`: “it sets nothing up and opens nothing” — opening the replacement during registration restores access before a separate authorized opening. | S1-restore |
| LIF-010 | convention | must | effect | `observable`: “what could not be observed reported as unknown” — turning silence into a known stopped state can permit duplicate work. | — |
| LIF-011 | convention | must | access | `then`: “They never change the lifecycle state” — letting a reported condition revive a retired consumer can restore ended access. | — |
| LIF-012 | convention | must | result | `observable`: “Inspecting a template returns no lifecycle state” — returning an invented lifecycle state gives a wrong inspection output. | — |
| LIF-013 | convention | must | access | `observable`: “with PND-002 disclosure and PND-004 evidence withholding” — unfiltered inspection can disclose pending subjects or evidence outside the caller’s permitted view. | — |
| LIF-014 | convention | must | effect | `observable`: “a confirmed shutdown removes the instance” — misreporting pending or unknown shutdown as confirmed treats an unresolved run as ended. | — |
| LIF-015 | convention | must | access | `then`: “The request is not dispatched” — setting up a retired instance can revive its ended relationships or execution authority. | — |
| LIF-016 | convention | must not | effect | `then`: “Anything is stopped or released other than what the instance itself holds” — retiring one instance can destroy another instance’s or a shared service’s effects. | — |
| LIF-017 | convention | must | effect | `then`: “only where the instance is retired” — unregistering an active instance can discard its lifecycle state before the required retirement and shutdown process. | — |
| LIF-018 | convention | must | effect | `observable`: “Nothing confirms the shutdown by the passing of time” — confirming by timeout treats an unknown run as stopped. | — |
| LIF-019 | convention | must not | effect | `then`: “Records, backing files or service data are erased” — erasing backing data on removal loses effects the instance was meant to leave in the service. | S1-close-in-flight |
| LIF-020 | convention | must | access | `then`: “The request is not dispatched and reports conflict” — retiring the owner before its live members end can prematurely transfer their inspection authority to management’s retired-owner fallback. | — |
| LIF-021 | convention | must | effect | `observable`: “Cancellation under OPS-027 passes to retirements in progress and dispatches no further retirement” — continuing a cancelled cascade performs later retirements outside its remaining authorization. | — |
| LIF-022 | convention | must not | effect | `observable`: “the object is as it was” — retiring a protected first-start object performs a state change no actor is permitted to cause. | — |
| LIF-023 | convention | must not | effect | `then`: “The instance is retired” — automatically retiring an unused instance performs a shutdown effect no actor requested. | — |
| LIF-024 | convention | must | effect | `then`: “once an invocation of `retire.v1` for it has succeeded” — confirming shutdown before successful retirement treats an unresolved run or hook as ended. | — |
| LIF-025 | convention | must | effect | `then`: “Its retire hook is not dispatched, no stop of a run is asked for” — dispatching retirement work for a never-set-up instance performs effects that this retirement does not authorize. | — |
| LIF-026 | convention | must | effect | `observable`: “Its retire hook is dispatched again only as OPS-021 and OPS-028 allow” — unconditionally repeating the hook can duplicate a completed or unknown effect. | — |
| LIF-027 | convention | must not | effect | `then`: “a run that it asked for is stopped, because of it” — automatic rollback adds a stop effect no retirement request authorized. | S1-setup-sender |
| LIF-028 | convention | must | access | `then`: “unless the instance, or the Conduit's consumer, has been retired” — reconciliation that revives a retired endpoint restores ended access. | — |
| LIF-029 | convention | must | access | `observable`: “The declared Conduit has one Conduit before and after the request” — admitting another live declaration slot can introduce an additional grant alongside the existing one. | S1-restore |
| LIF-034 | convention | must | effect | `then`: “The request is not dispatched” — dispatching retirement of a protected first-start object performs an explicitly forbidden effect. | bootstrap |
| LIF-035 | convention | must | effect | `observable`: “An instance whose shutdown is pending or unknown is not removed” — removing it prematurely loses state needed to settle or stop an unresolved effect. | S1-close |
| LIF-036 | convention | must | effect | `then`: “Before it removes the instance, the engine asks the instance's runtime provider once more to stop its run” — omitting the final stop can leave an untracked run executing after removal. | — |
| LIF-037 | convention | must | access | `observable`: “It is compared only after the access decision” — revealing expected-ID conflict details first discloses an object or its identity to an unauthorized caller. | E12-name-reuse |
| LIF-040 | convention | must | access | `observable`: “Inventory and `list.v1` use this same visibility and give no hidden counts” — exposing hidden tombstones or their counts discloses relationships outside inspection authority. | — |
| LIF-041 | convention | must | access | `then`: “the request is not dispatched and reports conflict; nothing changes” — ignoring a matching tombstone can recreate access that was deliberately closed. | — |

### spec/conventions/access.yaml

Read: 35 promises. Source: [spec/conventions/access.yaml](../../spec/conventions/access.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| ACC-001 | convention | must | access | `then`: “The request is not dispatched” — dispatching without an open listed grant permits unauthorized access. | S1-exact-access |
| ACC-002 | convention | must | access | `then`: “before its implementation is invoked” — calling first exposes the request or its effects before authorization. | S1-exact-access, S1-records |
| ACC-003 | convention | must | access | `then`: “exactly the operations it lists, on its target, and no others” — allowing an unlisted operation widens the relationship’s grant. | S1-exact-access |
| ACC-005 | convention | must | access | `then`: “by that instance's own Conduit to `access`” — borrowing the setup caller’s or target’s authority opens a grant the sender cannot delegate. | S1-setup-sender, S1-restore |
| ACC-006 | convention | must | access | `then`: “whose bound covers the new Conduit's target” — opening outside the sender’s bound grants access the sender cannot pass on. | S1-setup-again |
| ACC-008 | convention | must | access | `then`: “every restriction that applies permits it” — ignoring any applicable restriction admits a forbidden request or output. | — |
| ACC-009 | convention | must | access | `then`: “for an onward request, by the filters carried under ACC-017” — dropping carried filters while separating Conduit scopes lets onward access exceed the caller’s limits. | — |
| ACC-011 | convention | must | access | `then`: “All the access it carried ends” — reusing the closed route dispatches or delivers beyond revoked access. | S1-close |
| ACC-012 | convention | must | access | `then`: “whose bound covers the target of the Conduit to be closed and every operation that Conduit lists” — closing outside that bound grants an unauthorized access-management operation. | — |
| ACC-014 | convention | must | access | `then`: “nothing of it is performed” — performing work despite an unevaluable restriction bypasses that restriction. | — |
| ACC-015 | convention | must | access | `then`: “The request is not dispatched” — sending a carried filter to an unsupported implementation permits work without its required limit. | S1-plain-filter, E08-broker |
| ACC-016 | convention | must | access | `then`: “only under the instance's own open Conduit to that target” — borrowing the caller’s Conduit permits an onward target the sender cannot reach. | E08-broker |
| ACC-017 | convention | must | access | `then`: “Its effects and returned content are limited by those filters” — dropping a carried filter permits onward effects or disclosure forbidden to the original caller. | S1-close-in-flight, E08-broker |
| ACC-018 | convention | must | access | `then`: “It carries the filters of the invocation” — dispatching a hook without its parent’s filters widens that parent’s permitted effects. | — |
| ACC-019 | convention | must not | effect | `observable`: “The engine itself dispatches no request for the invocation or for its caller afterwards” — dispatching later work uses an ended invocation to authorize a new effect. | — |
| ACC-023 | convention | must | access | `then`: “It has no access until a Conduit of which it is the consumer is opened” — giving a new Interface implicit authority permits ungranted operations. | — |
| ACC-024 | convention | must not | access | `observable`: “The request is not dispatched” — allowing another actor to close root’s protected grant gives it a forbidden access-management operation. | — |
| ACC-027 | convention | must | result | `then`: “In this version every restriction is reported as mediated” — reporting enforced gives a wrong inspection output about the protection actually promised. | — |
| ACC-028 | convention | must not | access | `observable`: “reports refused on every operation of the Resources the Pool owns” — treating Pool ownership as service permission grants unlisted Resource access. | — |
| ACC-029 | convention | must | access | `observable`: “It covers no later object under that name” — rebinding a bound item by name lets it authorize grants to an unapproved replacement. | — |
| ACC-030 | convention | must | access | `then`: “A right to open is passed on, or opened for oneself, only as it is or narrower” — delegating a wider bound escalates access beyond the sender’s authority. | E08-broker |
| ACC-032 | convention | must | access | `then`: “only if the actor is the root Interface” — dispatching a reserved operation for another actor grants root-only power. | S1-purge |
| ACC-033 | convention | must | access | `then`: “by an open Conduit from the actor listing the operation to the target named by the first fitting row” — using another authorizing target permits management of objects outside the actor’s authority. | S1-pool-authority, S1-reconcile, E12-name-reuse |
| ACC-034 | convention | must | access | `then`: “An onward request made after the close” — dispatching that onward request extends access after a Conduit in its chain closed. | S1-close-in-flight |
| ACC-036 | convention | must | access | `then`: “No earlier decision is reused” — using a cached permission can dispatch a request after revocation. | S1-close |
| ACC-037 | convention | must | access | `then`: “Nothing else about the request is examined or reported” — reporting object details while authority is unreadable discloses data before permission is established. | S1-authority-unreadable |
| ACC-038 | convention | must | result | `then`: “the result names, under that requirement, the implementation that applies it” — naming another implementation gives a false inspection account of who enforces the restriction. | — |
| ACC-039 | convention | must | access | `then`: “Every other such request is not dispatched” — allowing ordinary or onward work for a retired consumer grants access after its authority ended. | — |
| ACC-044 | convention | must | result | `then`: “`actor`, the caller; `sentBy`, the sending instance” — keeping different provenance falsely attributes who opened the grant. | — |
| ACC-053 | convention | must | access | `then`: “Coverage is checked against the kept IDs, not any later objects under the names” — checking replacement objects can let an actor lift a tombstone outside its bound. | — |
| ACC-054 | convention | must | access | `then`: “only where root sent it on its own behalf” — opening root-consumed grants for a delegate gives it a root-only access-management operation. | — |
| ACC-055 | convention | must | access | `then`: “Through a Pool, the invocation must have been sent by that consumer” — allowing another consumer’s reconciliation lets it settle effects outside its authority. | — |
| ACC-057 | convention | must | access | `then`: “only where root sends it on its own behalf” — letting another actor register Agents grants an operation explicitly reserved to root’s own requests. | — |
| ACC-056 | convention | must | access | `then`: “Nothing in tier 1 depends on whether an object exists, who owns it or its state” — including resolved hidden facts in an ordinary refusal discloses objects the caller may not inspect. | — |

### spec/conventions/operations.yaml

Read: 32 promises. Source: [spec/conventions/operations.yaml](../../spec/conventions/operations.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| OPS-001 | convention | must | result | `then`: “exactly one outcome: succeeded, failed or unknown” — reporting another or multiple outcomes gives a wrong result for the ended invocation. | S1-plain-ends |
| OPS-002 | convention | must | result | `then`: “The effect the operation's contract describes has occurred” — reporting success without the effect gives a false outcome. | — |
| OPS-003 | convention | must | effect | `then`: “has not occurred, in whole or in part” — reporting failed after a partial effect makes a repeat able to apply that effect again. | — |
| OPS-004 | convention | must | effect | `then`: “It reports the outcome unknown” — reporting a known outcome without confirmation can lose or repeat the effect on retry. | — |
| OPS-005 | convention | must | result | `observable`: “carry the same one of those four reasons and no outcome” — returning an outcome for an undispatched non-repeat falsely reports an invocation’s execution. | S1-plain-ends |
| OPS-006 | convention | must | result | `then`: “The result states its outcome” — misstating or omitting the outcome prevents the response from accurately reporting the dispatched request. | — |
| OPS-007 | convention | must | effect | `then`: “The invocation's outcome is unknown” — treating a missing or inconsistent result as failed can repeat an effect that occurred. | — |
| OPS-008 | convention | must | effect | `then`: “At most one invocation exists for one caller, one sending instance, one target and one idempotency key” — losing or duplicating that identity allows the same keyed effect to run twice. | S1-key-scope, S1-purge |
| OPS-009 | convention | must | effect | `then`: “Nothing is dispatched” — dispatching a completed keyed invocation again can repeat its effect. | S1-key-scope, S1-reconcile, S1-purge |
| OPS-010 | convention | must | effect | `then`: “It is not dispatched” — dispatching changed contents under a used key can perform a second effect under one invocation identity. | S1-key-scope |
| OPS-011 | convention | must | effect | `then`: “It is not treated as a repeat of an earlier request because its contents match” — answering a new unkeyed request from an earlier success can lose the separately requested effect. | S1-key-scope |
| OPS-012 | convention | must | effect | `then`: “only if the operation's contract declares `deduplicates: true` and the implementation is not marked `plain`” — retrying an unknown effect without deduplication can apply it twice. | S1-unknown-write |
| OPS-013 | convention | must | effect | `then`: “The outcome stays unknown until it is reconciled” — changing uncertainty into failed or succeeded without reconciliation can lose or repeat an effect. | S1-unknown-write |
| OPS-014 | convention | must | effect | `then`: “only once the invocation is fenced” — recording failed before all attempts are fenced lets an earlier dispatch meet a retry and repeat its effect. | S1-reconcile |
| OPS-018 | convention | must not | effect | `then`: “The effect is applied more than once” — duplicating a keyed effect under loss or restart is an unsafe repeat. | S1-unknown-write |
| OPS-019 | convention | must | access | `then`: “every promise about operations and about access applies to it as to any other” — exempting management verbs from access checks permits unauthorized management. | — |
| OPS-020 | convention | must | access | `then`: “authorized by an open Conduit to the instance whose configuration it changes” — using another target’s authority permits an unauthorized configuration change. | — |
| OPS-021 | convention | must | effect | `then`: “That hook is not dispatched again” — redispatching an unknown hook can duplicate its effect. | — |
| OPS-022 | convention | must | effect | `then`: “failed only if no part had an effect, and otherwise unknown” — calling a partial setup failed allows its already-applied effect to be retried as absent. | S1-setup-sender |
| OPS-023 | convention | must | access | `observable`: “nothing about any other reason that applies” — disclosing invalid or conflict details before refusal can reveal a hidden object. | S1-pool-authority, S1-authority-unreadable, S1-reason-order, E12-name-reuse |
| OPS-024 | convention | must | access | `observable`: “the filters that apply to the repeat decide afresh” — replaying stored read output instead can disclose content a current filter forbids. | — |
| OPS-026 | convention | must | result | `then`: “The value so filled is what is sent” — dispatching a different default performs the request with arguments other than its normalized contents. | S1-key-scope |
| OPS-027 | convention | must | effect | `then`: “The invocation's outcome is unknown, unless its implementation confirms that nothing was done” — treating cancellation as failed without confirmation can repeat a partially applied effect. | — |
| OPS-028 | convention | must | effect | `then`: “a hook that succeeded is not dispatched again” — changing the hook’s key across setups or callers can run a completed effect twice. | — |
| OPS-029 | convention | must | access | `observable`: “a caller with only a Pool's authority learns nothing of whether a name exists outside that Pool” — reporting missing-object details before authority discloses names beyond the caller’s scope. | S1-close |
| OPS-030 | convention | must | effect | `then`: “The request is not dispatched” — reconciling a known or in-progress invocation, or recording failed before its fence, can authorize an unsafe repeat. | S1-reconcile |
| OPS-032 | convention | must | effect | `then`: “Nothing is dispatched” — dispatching an in-progress repeat creates a concurrent duplicate effect. | S1-key-scope |
| OPS-033 | convention | must | secret | `observable`: “the result holds no part of it” — echoing an invalid literal secret input discloses the value in the request’s result. | S1-reason-order |
| OPS-034 | convention | must | access | `observable`: “Where a filter applies to the repeat, the outputs are not given” — returning stored outputs can disclose content the current filter would forbid. | — |
| OPS-036 | convention | must | access | `then`: “only items that the actor may inspect” — listing hidden objects or counts discloses data outside the actor’s inspection authority. | — |

### spec/conventions/revisions.yaml

Read: 14 promises. Source: [spec/conventions/revisions.yaml](../../spec/conventions/revisions.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| REV-001 | convention | must | effect | `observable`: “compared only for equality” — ordering opaque revisions can admit a stale change that overwrites a later requested effect. | — |
| REV-002 | convention | must | effect | `then`: “The change is not dispatched” — dispatching a stale change can overwrite a newer effect the caller did not authorize replacing. | E11-replace |
| REV-003 | convention | must | effect | `observable`: “The revision after the change differs from the revision before it” — keeping the old revision lets another stale request overwrite the completed change. | — |
| REV-005 | convention | must not | effect | `observable`: “three distinct revisions” — reusing a revision after a value changes back can admit a stale change and lose an intervening effect. | — |
| REV-006 | convention | must | result | `then`: “No default is applied to it” — changing an omitted setting reports a control success for a change the request did not make. | — |
| REV-007 | convention | must | result | `then`: “It has applied exactly the values it was sent” — silently applying another setting reports an incorrect effect as success. | — |
| REV-009 | convention | must | effect | `then`: “the settings the control addressed are reported as unknown” — reporting uncertain settings as confirmed treats an unknown configuration effect as known. | — |
| REV-010 | convention | must | result | `observable`: “The values are not stated in the request: the engine stores those it sent” — accepting substituted reconciliation values reports another configuration as the original control’s effect. | — |
| REV-011 | convention | must | effect | `observable`: “No change is applied to whatever revision is current” — accepting an unconditioned change can overwrite an intervening effect the caller never observed. | — |
| REV-012 | convention | must not | effect | `observable`: “Of several requests to change one object that name the same revision, at most one is stored” — concurrent storage of both loses one authorized update. | — |
| REV-013 | convention | must | access | `then`: “The reconciliation is not dispatched” — storing a reconciled configuration outside the Type’s limits can enable an excluded operation setting. | — |
| REV-014 | convention | must | effect | `then`: “It is not dispatched” — dispatching a concurrent setup or change can duplicate work or overwrite a completed effect. | — |
| REV-015 | convention | must | access | `then`: “It is not dispatched” — applying a control outside the Type’s schema admits a configuration change beyond its allowed limits. | — |

### spec/conventions/observation.yaml

Read: 5 promises. Source: [spec/conventions/observation.yaml](../../spec/conventions/observation.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| OBS-001 | convention | must not | access | `then`: “The instance's declared definition, configuration or revision changes because of it” — letting an observation rewrite a Conduit’s declaration can widen access without an actor’s change request. | — |
| OBS-002 | convention | must | result | `then`: “the time it was made and its source” — reporting another source or time falsely describes the observation returned by inspection. | — |
| OBS-003 | convention | must | effect | `observable`: “The earlier value is not shown as current” — showing a stale run state as current treats uncertainty about an executing effect as knowledge. | — |
| OBS-004 | convention | must not | effect | `observable`: “No setup, control, open or retire is dispatched because of it” — repairing observed drift implicitly starts an effect no actor requested. | — |
| OBS-006 | convention | must not | effect | `then`: “An effect occurs, or the actor that inspects it gains access to anything” — performing work during inspection adds an effect the read did not authorize. | — |

### spec/conventions/records.yaml

Read: 6 promises. Source: [spec/conventions/records.yaml](../../spec/conventions/records.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| REC-001 | convention | must | result | `observable`: “states that the request was permitted, or that it was not dispatched and the reason” — recording the wrong access verdict falsely states what was decided and dispatched. | S1-records |
| REC-002 | convention | must | secret | `observable`: “no part of its `identity`” — including run identity in a record discloses credentials to record readers. | S1-close-in-flight, S1-authority-unreadable, S1-records |
| REC-003 | convention | must not | result | `then`: “The engine removes or alters it other than by a purge under REC-004” — altering an existing record can falsely attribute who performed an operation. | S1-purge |
| REC-004 | convention | must | effect | `observable`: “no effect is applied a second time” — purging invocation state with records lets an old keyed request repeat its effect. | S1-purge |
| REC-005 | convention | must | result | `then`: “in the order in which they were made at that source” — returning another order gives a false account of the recorded sequence of decisions and actions. | S1-records |
| REC-006 | convention | must | result | `observable`: “It is never presented as complete” — presenting an incomplete history as complete gives a wrong output about the requests and effects it contains. | S1-records |

### spec/conventions/implicit.yaml

Read: 5 promises. Source: [spec/conventions/implicit.yaml](../../spec/conventions/implicit.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| IMP-001 | convention | must | effect | `then`: “dispatched through an open Conduit that lists that operation” — starting model work without that invocation spends work no actor authorized. | — |
| IMP-002 | convention | must not | effect | `then`: “Model work starts because of it” — starting a turn as a side effect of setup or observation performs unrequested model work. | — |
| IMP-003 | language | must | effect | `then`: “An operation without the mark does not start model work” — starting a turn from an unmarked operation spends model work its contract did not authorize. | — |
| IMP-007 | convention | must not | effect | `then`: “It is dispatched again without a repeat of the request from its caller” — automatic redispatch after an end or restart can duplicate an effect. | S1-unknown-write |
| IMP-008 | convention | must | effect | `then`: “The request is not dispatched” — allowing an unmarked invocation to start a marked onward operation performs model work outside the original authorization. | — |

### spec/interface/implementation.yaml

Read: 46 promises. Source: [spec/interface/implementation.yaml](../../spec/interface/implementation.yaml).

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| IFC-001 | interface | must | result | `observable`: “carries exactly the operation's inputs as its `arguments`” — changing the inputs on the wire can report an output for a different request. | — |
| IFC-003 | interface | may | effect | `then`: “The meaning of Conduit's keys and of an outcome is the same at every revision” — changing outcome meaning during negotiation can treat an unknown effect as failed and permit a duplicate. | — |
| IFC-004 | interface | must | access | `observable`: “A revision that lacks any of what a call relies on is not used for that call” — using a revision unable to carry filters can dispatch a call without its required restrictions. | — |
| IFC-006 | interface | must | secret | `observable`: “only the reference, never a secret” — sending secret values in ordinary configuration exposes them outside their permitted run or header delivery path. | — |
| IFC-008 | interface | must | effect | `then`: “It performs nothing” — acting before a failed extension-version reply lets a retry duplicate the effect. | — |
| IFC-009 | interface | must | effect | `observable`: “a result that contradicts itself is trusted in neither part” — trusting a contradictory failed result can repeat an effect that occurred. | — |
| IFC-010 | interface | must | effect | `observable`: “That code gives no guarantee that a plain server has not acted” — treating an effectful invalid-parameters error as failed can repeat a partial effect. | S1-plain-ends |
| IFC-011 | interface | must | access | `then`: “A handle permits nothing” — treating a handle as permission grants a caller access without an authorization check. | E05-handle |
| IFC-012 | interface | must | access | `observable`: “the engine presents no tool that belongs to no Conduit” — publishing or dispatching a tool outside the consumer’s grants exposes an unauthorized operation. | S1-exact-access |
| IFC-013 | interface | must | access | `then`: “Its tools are no longer presented to its consumer” — continuing to list the closed route discloses an operation after its presented access ended. | — |
| IFC-014 | language | must | result | `observable`: “No tools of one consumer share a name” — admitting a collision can dispatch the consumer’s requested tool to the wrong target or operation. | — |
| IFC-015 | interface | must | secret | `observable`: “no `identity` and no credential that names an instance” — handing instance credentials to a shared remote implementation lets it impersonate those instances. | — |
| IFC-016 | interface | must | access | `then`: “from any other instance reports refused” — using a task ID as permission can disclose another instance’s result or let a caller control its task. | — |
| IFC-017 | interface | must | secret | `then`: “it follows no HTTP redirect from either” — following a redirect can disclose credentials or request contents to a server that proved nothing for the instance. | — |
| IFC-018 | interface | must | access | `then`: “The list holds every filter that applies to the request” — omitting a filter on the wire lets the implementation exceed the caller’s permitted slice. | — |
| IFC-019 | interface | must | access | `then`: “It applies every filter in the list to what the call asks it to do and to what it returns” — ignoring one filter permits a forbidden effect or disclosure. | — |
| IFC-020 | interface | must | access | `then`: “from the served invocation” — trusting the sender’s substituted caller or filters can widen onward authority beyond its actual invocation. | E08-broker |
| IFC-021 | interface | must | effect | `then`: “The request is not dispatched” — dispatching a Resource’s request without a served invocation performs work no actor invocation authorized. | — |
| IFC-022 | interface | must | access | `then`: “none once it is removed, even where another instance has since taken its name” — accepting a removed instance’s credential revives access or impersonates its replacement. | E12-name-reuse |
| IFC-023 | interface | may | effect | `observable`: “The engine treats that value as the request's idempotency key” — ignoring or changing a supplied key can dispatch the same keyed effect twice. | — |
| IFC-025 | interface | must | result | `then`: “with the value succeeded, failed or unknown” — returning another outcome than the invocation’s gives the consumer a wrong result. | S1-pool-authority, S1-reason-order, S1-plain-ends |
| IFC-026 | interface | must | effect | `observable`: “never after it has performed any part of what the call asks” — returning a reason after a partial effect makes a later request repeat work reported as unperformed. | — |
| IFC-027 | interface | must | result | `then`: “The request ends with that reason and has no outcome” — reporting an outcome instead falsely describes the implementation’s refusal as an executed invocation. | — |
| IFC-028 | interface | must | access | `then`: “The request is not dispatched” — accepting another instance’s or an ended invocation’s serving ID borrows authority the sender does not hold. | — |
| IFC-029 | interface | must | access | `then`: “read at the revision of Conduit's extension that it declares” — reading access metadata under another revision can apply the wrong caller or restriction semantics. | — |
| IFC-030 | interface | must not | secret | `then`: “what a consumer presented to the engine to identify itself” — forwarding consumer proof discloses a credential to the implementation. | — |
| IFC-031 | interface | must not | access | `observable`: “holds no key that begins `io.github.shrav-k.conduit/`” — forwarding caller, filter or configuration metadata discloses it to a service outside the Conduit relationship. | — |
| IFC-032 | interface | must | access | `then`: “The request is not dispatched” — letting an unauthenticated or removed instance use a tool bypasses instance-based authority. | — |
| IFC-033 | interface | must | access | `observable`: “It includes no configuration, owner or placement” — adding those facts to tool descriptions discloses target details the consumer’s use grant does not authorize it to inspect. | E05-local, E05-two-contexts, E05-recursive, E05-siblings, E05-escaping, E05-core, E05-handle, E05-narrowed |
| IFC-036 | interface | must | access | `when`: “whose `Origin` header is present and is not one that the engine accepts” — dispatching that rejected-origin request bypasses the protocol’s origin admission boundary. | — |
| IFC-037 | interface | must | access | `observable`: “The engine never falls back to sending such a call as it would to a plain implementation” — fallback without Conduit metadata drops the restrictions the implementation must apply. | — |
| IFC-038 | interface | must | access | `then`: “Every tool list that the server serves carries `cacheScope: private`” — allowing shared caching can disclose one consumer’s tools to another authorization context. | — |
| IFC-039 | interface | must | access | `observable`: “What trace context a request carries changes nothing that the engine decides” — using trace metadata to alter permission allows a sender to supply authority outside its proof and grants. | — |
| IFC-040 | interface | must not | access | `then`: “the request's caller, a restriction, or any part of an access decision” — trusting trace context lets a caller forge authorization inputs. | — |
| IFC-041 | interface | must | secret | `observable`: “no header's value reaches a request for another instance” — sharing a connection’s headers across instances discloses one instance’s secret to another service request. | — |
| IFC-042 | interface | must | access | `observable`: “A token whose audience is another receiver, or that names an instance that has been removed, establishes no instance” — accepting such a token grants access outside its audience or ended identity. | — |
| IFC-043 | language | must | access | `observable`: “a value from configuration there would be the same for every request, and proves nothing of who sent it” — accepting configured Authorization in place of enforcement-point proof can bypass the gate’s authority check. | — |
| IFC-044 | language | must | secret | `then`: “The item is rejected” — admitting a non-HTTPS Conduit-aware endpoint can expose enforcement-point proof or secret-bearing requests without authenticated confidential transport. | — |
| IFC-045 | interface | must | effect | `then`: “it declares no capability but” — advertising additional capabilities can enable server-initiated model work or other work the call did not authorize. | — |
| IFC-046 | interface | must | effect | `then`: “only for an operation whose contract states no `effect`, that is not a control and starts no model work” — resending an effectful call after a header error can duplicate an effect already begun. | — |
| IFC-047 | interface | must | effect | `then`: “only where it has performed nothing for the call” — replying with an undeclared-capability error after acting makes the failed outcome authorize a duplicate attempt. | — |
| IFC-050 | interface | must | access | `then`: “It leaves out only what ACC-056, LIF-013 and PND-004 withhold from that caller” — including withheld diagnostic or notice facts discloses information outside the caller’s view. | — |
| IFC-049 | interface | must | secret | `then`: “The URL is HTTPS only, contains no credential or personal data and is not pre-authenticated” — relaying a credential-bearing URL exposes sign-in material through the client’s URL flow. | — |

## For the coordinator

None open. IFC-007 was decided by the admin on 2026-11-11, open to the owner to strike: it is
wire shape, not an access-safety promise, and is listed with the others.

## The others

| ID | reason |
| --- | --- |
| OBJ-001 | naming |
| OBJ-006 | form |
| OBJ-007 | form |
| OBJ-008 | form |
| OBJ-012 | form |
| OBJ-013 | form |
| OBJ-014 | form |
| OBJ-015 | form |
| OBJ-016 | form |
| OBJ-017 | form |
| OBJ-018 | form |
| OBJ-019 | form |
| OBJ-020 | form |
| OBJ-022 | naming |
| OBJ-023 | form |
| OBJ-024 | naming |
| OBJ-025 | form |
| SCH-002 | form |
| SCH-003 | form |
| SCH-005 | form |
| SCH-006 | form |
| SCH-008 | form |
| SCH-012 | form |
| SCH-013 | form |
| SCH-014 | form |
| SCH-017 | form |
| SCH-018 | form |
| SCH-019 | form |
| SCH-021 | form |
| INS-001 | form |
| INS-002 | form |
| INS-003 | form |
| INS-004 | form |
| INS-010 | form |
| INS-011 | form |
| INS-017 | form |
| INS-021 | form |
| INS-024 | form |
| INS-025 | naming |
| TPL-001 | form |
| TPL-002 | form |
| TPL-005 | form |
| CTR-001 | naming |
| CTR-007 | naming |
| CTR-008 | form |
| CTR-009 | form |
| CTR-012 | form |
| NAM-001 | naming |
| NAM-002 | naming |
| NAM-003 | naming |
| NAM-004 | naming |
| NAM-005 | naming |
| NAM-006 | form |
| NAM-007 | form |
| NAM-008 | diagnostic wording |
| NAM-016 | progress or timing |
| NAM-017 | progress or timing |
| NAM-018 | naming |
| NAM-027 | naming |
| NAM-028 | form |
| SCT-001 | form |
| SCT-002 | form |
| SCT-008 | form |
| SCT-013 | naming |
| PKG-001 | naming |
| PKG-007 | form |
| PKG-012 | naming |
| PKG-018 | naming |
| PKG-022 | form |
| PKG-030 | provider choice |
| MAN-001 | form |
| MAN-002 | form |
| MAN-003 | form |
| MAN-004 | form |
| MAN-005 | form |
| MAN-007 | form |
| MAN-009 | form |
| MAN-013 | form |
| MAN-015 | naming |
| MAN-017 | form |
| SUB-001 | form |
| SUB-004 | form |
| SUB-005 | form |
| SUB-011 | implementation-defined |
| POOL-005 | naming |
| POOL-011 | form |
| POOL-012 | form |
| FLEET-006 | naming |
| PLC-005 | provider choice |
| PLC-006 | form |
| PLC-011 | other: rejects removal of an occupied node; violating this lifecycle admission rule alone need not misreport effects or grant access |
| PLC-015 | form |
| PLC-025 | form |
| CON-007 | progress or timing |
| CON-016 | form |
| CON-028 | form |
| PRV-001 | provider choice |
| PRV-020 | other: postpones provider retirement while dependants exist; violating the admission rule alone can leave work unavailable without misreporting its outcome |
| LIF-031 | form |
| LIF-032 | form |
| LIF-033 | form |
| ACC-022 | progress or timing |
| OPS-025 | progress or timing |
| OPS-031 | form |
| REV-004 | progress or timing |
| IFC-002 | progress or timing |
| IFC-005 | naming |
| IFC-007 | other: wire shape, by the admin's decision of 2026-11-11, open to the owner to strike. The selected implementation is granted the Resource's configuration and the call; IFC-030 withholds the consumer's proof, and the secrets promises withhold secret material. |

## S1 traces

Source: [the proposed S1 traces](../../conformance/s1-safety-cases.md), in table order,
followed by the two adjacent publication checks. There are 34 traces: 27 safety traces and
7 flagged publication-only traces. “Safety” concerns the trace’s substantive unsafe slice;
checking formatting or diagnostic wording alongside it supplies no safety coverage.
The promise column intersects the trace’s requiring list with the safety tables above;
a named safety promise need not be tested in its unsafe slice by this trace.

The seven publication-only checks compare standalone schema or wire representation, rather
than deciding or dispatching a Conduit request. IFC-033 explicitly says the definition changes
nothing the engine accepts: it still checks against the contract and template. A malformed
published schema alone therefore need not yield an unsafe outcome under this catalogue’s test.
E05-local and E05-core’s no-network checks concern standalone validation without a package
resolver; an actual unauthorized engine fetch would need its own safety assertion.
E05-narrowed also checks a call to an excluded control, so it does contain an access-safety
slice under TPL-013; correct publication of the narrowing alone would not supply that slice.

| trace | safety promises among requiring promises (kind) | safety trace? | flag |
| --- | --- | --- | --- |
| E05-local | IFC-033 (access), SCH-001 (access), SCH-015 (access) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| E05-two-contexts | IFC-033 (access), NAM-014 (access), NAM-023 (effect) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| E05-recursive | IFC-033 (access), SCH-015 (access) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| E05-siblings | IFC-033 (access), SCH-001 (access) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| E05-escaping | IFC-033 (access), SCH-015 (access) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| E05-core | IFC-033 (access) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| bootstrap | FST-001 (access), FST-002 (access), FST-003 (effect), FST-005 (result), LIF-034 (effect) | yes | — |
| S1-register-set | SUB-002 (access), SUB-003 (effect), OBJ-009 (access), INS-016 (effect) | yes | — |
| S1-duplicate | OBJ-002 (access), SUB-003 (effect) | yes | — |
| S1-archive-escape | MAN-014 (effect) | yes | — |
| S1-no-fetch | PKG-006 (effect), SUB-010 (effect), NAM-012 (effect) | yes | — |
| S1-target-type | CON-021 (access), CON-017 (access), SUB-003 (effect) | yes | — |
| S1-plain-filter | PKG-020 (access), CON-022 (access), ACC-015 (access) | yes | — |
| S1-pool-authority | ACC-033 (access), OPS-023 (access), IFC-025 (result) | yes | — |
| S1-setup-sender | ACC-005 (access), CON-004 (access), TPL-012 (effect), OPS-022 (effect), LIF-004 (effect), LIF-027 (effect) | yes | — |
| S1-setup-again | CON-004 (access), ACC-006 (access), PRV-007 (effect) | yes | — |
| S1-target-registered | CON-023 (access), CON-024 (effect), LIF-003 (result) | yes | — |
| S1-exact-access | ACC-001 (access), ACC-003 (access), ACC-002 (access), IFC-012 (access), CON-017 (access) | yes | — |
| S1-close | ACC-011 (access), ACC-036 (access), CON-014 (effect), LIF-035 (effect), OPS-029 (access) | yes | — |
| S1-close-in-flight | ACC-034 (access), ACC-017 (access), REC-002 (secret), LIF-019 (effect) | yes | — |
| S1-restore | LIF-008 (access), LIF-009 (access), LIF-029 (access), CON-004 (access), ACC-005 (access) | yes | — |
| S1-authority-unreadable | ACC-037 (access), OPS-023 (access), REC-002 (secret) | yes | — |
| S1-reason-order | OPS-023 (access), OPS-033 (secret), IFC-025 (result) | yes | — |
| S1-plain-ends | IFC-010 (effect), IFC-025 (result), OPS-001 (result), OPS-005 (result) | yes | — |
| S1-key-scope | OPS-008 (effect), OPS-009 (effect), OPS-010 (effect), OPS-011 (effect), OPS-026 (result), OPS-032 (effect) | yes | — |
| S1-unknown-write | OPS-012 (effect), OPS-013 (effect), OPS-018 (effect), IMP-007 (effect) | yes | — |
| S1-reconcile | OPS-014 (effect), OPS-030 (effect), ACC-033 (access), OPS-009 (effect) | yes | — |
| S1-purge | REC-004 (effect), REC-003 (result), OPS-008 (effect), OPS-009 (effect), ACC-032 (access) | yes | — |
| S1-records | REC-001 (result), REC-002 (secret), REC-005 (result), REC-006 (result), ACC-002 (access), PRV-024 (secret) | yes | — |
| E08-broker | ACC-030 (access), ACC-016 (access), ACC-017 (access), ACC-015 (access), IFC-020 (access) | yes | — |
| E11-replace | SUB-008 (access), SUB-009 (effect), NAM-009 (effect), NAM-022 (access), INS-020 (access), PKG-005 (access), REV-002 (effect) | yes | — |
| E12-name-reuse | ACC-033 (access), LIF-037 (access), OBJ-010 (access), IFC-022 (access), OPS-023 (access) | yes | — |
| E05-handle | IFC-011 (access), IFC-033 (access) | no | Flag: publication only; no decided or dispatched Conduit request has a wrong outcome or output merely from this schema or representation mismatch. |
| E05-narrowed | IFC-033 (access), TPL-011 (access), TPL-013 (access) | yes | — |

The three filtered replay variants of finding 6-1-02 are adjacent boundaries, not additional
traces: read-only success is redispatched under OPS-024, success with an effect and outputs
conflicts under OPS-034, as does success with an effect and plain content even without declared
outputs; a success with neither outputs nor plain content returns its recorded outcome under
OPS-009. A plain implementation supports no filter: an onward request alone can carry one,
and OPS-023 puts OPS-034 conflict before ACC-015 unavailability. None is counted again here.


## Not yet written

These are prospective additions or changes, not classifications of text outside `spec/`.
References identify the pending lines by [IN_FORCE.md](../../IN_FORCE.md) topic and their
ruling citations. Lines already marked **In spec/**, including those below older pending
headings, are not counted again.

- Topic 5, batch 2 (61 B3, B8, B10, B11): adding `make.v1`, keep, withhold and acting for a
  Fleet will require safety promises for their authorization, limits and recorded identities.
- Topic 6, making a Conduit (61 B8): `make.v1` must preserve every opening condition and the
  actor’s bound, or it could introduce a grant the actor cannot delegate.
- Topic 6, grant review (61 B9, B10, replacing 54 points 2 to 4): limits on who keeps a grant
  and the disclosure and attribution of grants made under closed authority guard unauthorized
  access and false accountability.
- Topic 6, withhold (61 B11): refusing the same Fleet’s re-making until a permitted lift will
  add a safeguard against restoring explicitly withheld access.
- Topic 9, Fleet authority (61 B1 to B7, B13; 71 point 2): forwarding, appointment, bounded
  grants, protected targets and operations, and linked records will change both authorization
  and attribution safety promises.
- Topic 9, Fleet-held Pool scopes (66 point 1, with 61 B2): completing `manages` for Fleet-held
  Pools must retain the listed exclusions or grant administration beyond the scoped authority.
- Topic 9, grant answers and peer delivery (61 B9; 66 point 3; 68 point 3 R3): completing these
  pending-action paths must keep answering authority, inspection limits and peer-root standing
  from widening delivery or permission.
- Topic 9, Agent client tools (70, provider order 050): the promised inspection of reach beyond
  Conduit must describe enabled tools accurately, or return a wrong account of the Agent’s reach.
- Topics 9 and 12, secret-reading Agents that others drive (61 B13): preventing turns on a
  secret-reading Agent and secret grants to a driven Agent will guard indirect secret disclosure.
- Topic 10, managed enrollment (50; 51 point 8; 52 point 4; 56; 68 point 3 R2): root admission,
  stable machine identity, evidence comparison and join-secret withholding will add safeguards
  against an unintended machine’s admission or credential disclosure, changing PLC-025/PLC-026.
- Topic 10, peer engines (51 caveat; 62 C4; 68 point 3 R3): the admitted Interface’s specific
  root standing and exclusions must prevent cross-engine authority from becoming unrestricted.
- Topic 7, metrics (68 point 1 as amended by 69; 72 point 1): attribution, attribute-preserving
  aggregation and Interface inspection limits will guard wrong metric outputs and disclosure
  beyond inspection authority, although numbers decide no operations and metrics do not gate v0.

Topic 7’s portable cause-of-loss field (44, open point 3) and topic 13’s safety-vector field
(68 point 2) concern vector representation rather than adding runtime safety behavior.
Topic 11’s pending tool-description line (53 point 3) is already written in IFC-033.
No pending line is treated as supplying behavior missing from the specification.
