# Round 2: decisions for the fix pass

The three readers of `spec/` reported 125 findings: [reader 1](reader-1.md), 36;
[reader 2](reader-2.md), 49; [reader 3](reader-3.md), 40, of which 7 unsafe. The owner ruled
that findings are resolved without a ruling on each (plan-audit ruling 37). This file settles
every finding a reader marked as needing a decision, and the unsafe ones. Every other finding
is fixed as its reader proposes. Each row is the coordinator's choice and can be struck by the
owner.

The rules for editing promises and the vocabulary are in
[the consolidation instructions](../CONSOLIDATION.md).

## Unsafe findings (reader 3)

| Finding | Decision |
| --- | --- |
| R3-1 Filters skip graph steps | A step without `conduit` carries the restrictions of the invocation its graph is serving, and the implementation it invokes applies them as IFC-019 states. |
| R3-2 A retried step has no identity | The engine gives every step an operation identity derived from the invocation's ID and the step's name, and sends it. A new attempt of the step carries the same one. |
| R3-3 Retained results scoped to the caller alone | An operation identity, and the retrieval of a result, are scoped to the caller, the target and the instance that sent the request. |
| R3-4 No test for "access at that moment" | A retained result is bound to the Conduit its invocation was dispatched through. Its content is delivered only while that Conduit is open, and never under another one. |
| R3-5 Nested setup steps use the caller's rights | The steps of a setup are authorized exactly as the request to set up was: by the Conduits of the instance that sent it. For an onward request that is the sending instance, not the caller it serves. |
| R3-6 Closing stops one level only | The rule holds for every invocation that serves, directly or through others, one that was dispatched through the closed Conduit. |
| R3-7 Reconciling a management operation | Reconciling is authorized against the target that authorized the request being reconciled. `access-control.v1` therefore includes `reconcile.v1`. Recording that an unknown setup or open occurred makes the instance ready or the Conduit open; that a retirement or close occurred confirms the shutdown; that it did not occur gives the outcome failed. |
| R1-1.1 An unbounded right passes the "within" test | A Conduit to `access` that applies no bound is unlimited, and only an actor whose own Conduit to `access` applies no bound may open or close one. |

## Decisions on the other findings

| Finding | Decision |
| --- | --- |
| R1-2.1, R2-T1, R1-1.4 The setup and retire operations | An instance is set up by one invocation of `set-up.v1` and retired by one of `retire.v1`, requested of the target ACC-033 names, with the instance as an input. The engine implements both. A template's `setup` and `retire` graphs are the steps the engine runs for that instance; they are not implementations selected in the `implementation` map and have no inputs of their own. "The setup operation" and "the retire operation" mean these invocations. |
| R1-1.2 A rename and the revision | A rename advances the revision, like any stored change. |
| R1-1.3 A Conduit closed without a request | A Conduit's shutdown is confirmed when it is closed, however it was closed. It holds nothing to shut down. |
| R1-1.5 Retrieving under an unknown identity | An identity under which the sender has no invocation reports refused. OPS-015 becomes that rule. |
| R1-1.7 "Names a location" | PLC-005 covers placement only: a template that gives a placement, in an entry or as the default of an input whose destination is a placement, is rejected. A template may refer to a location in any other way. |
| R1-2.2, R2-C2 Replacing a component | A component of a composite that was instantiated by its own document cannot be replaced; a new Conduit is instantiated by a new document. A replacement asked for while the instance is not retired reports conflict, for a declared Conduit and a component as for an entry. |
| R1-2.3 A bound applied twice | A template that applies `bound.v1` twice is rejected when it is registered. A Conduit whose target is not `access` and that applies it is not instantiated. |
| R1-2.5, R2-C1 Every Pool offers its capability | A Type of the family Pool that does not offer `pool-management.v1` is rejected when its package is installed. |
| R1-2.6, R2-C4 Policies and bounds of unpublished contracts | A policy and a bound name published contracts only. |
| R1-3.1 Work that runs later | Any operation whose contract says so may arrange later work. A start of model work is arranged only by an operation marked as starting it. Both are bound to the Conduit through which they were arranged; once it is closed no start is dispatched, and the requests of work already started are refused. IMP-006 keeps the model-work half and ACC-019 the access half. |
| R1-5.2 A reason in a language promise | OBJ-002 keeps the rejection. A new convention promise says that a registration rejected for a clash of names reports conflict. |
| R1-1.11 An input with a default | Any input that states a default may be left out, whether or not it is marked optional. This holds for a template's, an Operation's and a Requirement's inputs, and OBJ-015 says so. |
| R2-T2 What a project is | The documents and the one manifest handed over together in one registration. A directory is one way to hand them over. |
| R2-S1 The value of `from` | A bare name or `{kind, name}` for a template that exists, a qualified reference for one a package exports, or `#name` for one of the same project or package. A promise states it. `from` is not a selection; PKG-013 names it beside selections. |
| R2-S3 Runtimes of a type | An instance has the runtimes its type declares as well as its template's. A template that declares a runtime under a name its type declares is rejected. A step may name either. |
| R2-S4 A left-out `arguments` map | Present and empty, as an unwritten `config` is. |
| R2-S6 The recorded packages | The key is `packages`: a list of items with `address`, `version` and `digest`. The core package's item has no `digest`; an unpublished package's has `digest` alone. OBJ-004 names the key. |
| R2-S9 `.` and `..` in an address | A path segment starts with a letter or a digit. |
| R2-G1 The form of a resolved reference | Where a value is a reference, the instance stores `{kind, id}` in its place. Every reference schema accepts that form, which only the engine writes. An implementation receives it, a graph reads it, and inspecting shows it. |
| R2-G2 Which input sets which setting | A control's `changes` is a map from the name of an input to the pointer it sets. "The settings a control addressed" are those of the inputs it was sent. |
| R2-G3 A composed requirement | An implementation supports a composition when its `supports` lists the composition or every one of its constituents. On the wire a composition is sent as one item for each constituent, with its arguments bound, and `restriction` names a constituent. A composition narrows if any constituent does, and may be applied only where every constituent may. |

## The points known before the round

These are settled here too, so that the next round starts with none.

| Point | Decision |
| --- | --- |
| One `package` key cannot name two unpublished packages | Each `#name` on the wire is written with its package's digest: `<digest>#<name>`. The separate `package` key is removed. |
| The reason for retrieving a result once the target is unregistered | Refused, as for any identity under which the sender can reach no invocation. |
| An object's default against its properties' defaults | The object's default is checked after its own properties' defaults are filled in. |
| A pointer that selects nothing in a graph's `outputs` | The invocation's outcome is unknown if a step had an effect, and failed otherwise, as OPS-022 decides any graph. |
| What `obliges` requires | It names the ends whose implementations must state the requirement in `supports`. With none, the target's must. Inspecting a Conduit shows, under each requirement, the qualified reference of the implementation that applies it. |
| The `settings` input of a reconciled control | The same shape as the control's `settings` output. |
| A repeat's "same contents" and defaults | Judged after defaults are filled in. |
| An implementation that restores a runtime its type does not allow | The engine treats the handle as no longer held; the runtime is stopped under its `stop` operation and reported as not held. |
| The record of a request from no known instance | It holds no caller, and says that none could be established. |
| How a value is marked as secret | Left for the owner. |

## Who edits what

Editor P owns every promise file under `spec/promises/` except `manifest.yaml`,
`spec/conventions/`, `spec/interface/`, `spec/implementation-defined.yaml`,
`spec/schema-profile.md`, `spec/definitions.md` and `spec/README.md`.

Editor S owns `spec/core/`, `spec/schemas/`, `spec/promises/manifest.yaml` and
`tools/check_schemas.py`.
