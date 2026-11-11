# Ruling 41: the review of the whole design, and the secret type

Status: in force; amended by 42 (D6, D8, D12, names); amended by 43 (item 4c, Q3); amended by 44 (D14); amended by 58 (M22, M41) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [PRV-001 to PRV-026](../../../spec/promises/providers.yaml), [SEC-001 to SEC-023](../../../spec/promises/secrets.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

After ruling 40 the coordinator had the whole design reviewed against the owner's principles
(Conduit innovates only on connection and access; infrastructure is swappable behind contracts;
nothing is overcomplicated). The [review](../principles-review/README.md) put 31 decisions to the
owner, listed the alignments the coordinator would make and three changes to choices that rulings
had accepted, and reduced the secret type to a contract that any store implements (its section
4). The coordinator recommended accepting all of it, and noted that items 3a (removing policies),
5 (removing composite Conduits) and 6a (a result delivered after its Conduit closed) remove or
weaken parts of access, which is Conduit's own ground.

The owner's ruling: "Accept all, though I don't necessarily like the name secret reveal for the
contract, we should do another vocab audit as well before we finish up pr 7".

## What follows

- Every decision of section 2 of the review is accepted as recommended, with each lettered line
  as recommended: items 1 to 31.
- The alignments of section 3 are accepted: C33, C51, C52 and C58, and the three changes to
  choices that rulings accepted, C14, C20 and C21.
- The secret type is ruled as section 4 of the review reduces it. The
  [secrets proposal](../secrets/README.md) is the reasoning and the record of the options; where
  it and section 4 differ, section 4 governs. This settles point 1 of ruling 39.
- Removing policies (item 3a) is recorded as a candidate access feature to design for a later
  version, as Conduit's own, rather than only as an outside decision point.
- The name of the shared contract of secret stores, `secret-reveal.v1`, and of its operation,
  `reveal-secret.v1`, are not final. A vocabulary audit of the whole specification, as it stands
  after this ruling is applied, proposes names before pull request 7 is finished, and the owner
  rules on them.
- The specification is rewritten around provider contracts as rulings 40 and 41 say, then read
  by the vocabulary audit and by a fifth round of readers, before pull request 7 is merged.

## After the ruling

The planner of the rewrite ([PLAN.md](README.md#planners-choices), "Planner's choices") had to fill twenty gaps
that the review left open, P1 to P20, and the coordinator put them to the owner while the
editors worked. The owner's answer, on 2026-11-06: "Accept all with p2 changed".

- P1 to P20 are accepted as the plan states them, except P2.
- P2 is changed: `spec/promises/graphs.yaml` is not kept as a file with no promise. It is
  deleted, and `spec/README.md` lists the promise files that rulings removed, with the IDs each
  held, so that no removed ID can be reused.
- The names that P1 coins are not final: the vocabulary audit reviews them with
  `secret-reveal.v1` and `reveal-secret.v1`.

## Planner's choices

The review decides the direction; these fill what it leaves open. Each is the smallest reading
the planner found. The coordinator confirms or strikes each; the vocabulary audit renames.

| # | Choice | Why it is needed | Alternative |
| --- | --- | --- | --- |
| P1 | New names: operations `setup-hook.v1`, `retire-hook.v1`, `run.v1`, `stop.v1`, `inspect-run.v1`, `list-machines.v1`, `place.v1`, `fetch-package.v1`; capabilities `runtime-provider.v1`, `machine-provider.v1`, `package-source.v1`; schemas `handle.v1`, `record.v1`, `condition.v1`, `label-selector.v1`, `server-package.v1`, `server-remote.v1`; the standard argument `runtime`; item keys `package`, `remote`, `digest`, `environment`, `arguments`, `headers`; `_meta` keys `config` and `revision`; the extension `io.github.shrav-k.conduit/interface`; the Operation `spec` keys `inputSchema`, `outputSchema`, `idempotentHint`, `destructiveHint`, `openWorldHint`; the input `nested` of `register-replacement.v1`; the words "part" (of a setup or retirement), "remove" (an instance), "provider call" | The review names `secret-reveal.v1`, `reveal-secret.v1` and `secret-reference.v1` only | Names the vocabulary audit proposes |
| P2 | `spec/promises/graphs.yaml` stays as a file that holds no promise (`[]`), with a head comment naming its 26 removed IDs | The rule says a removed ID is named in its file's head comment | Delete the file and name the IDs in `spec/README.md` |
| P3 | One run per instance (2a) means a template whose selected implementations include two different items that are run (`command` or `package`) is rejected (TPL-019) | 2a names a run by the instance's ID, so an instance can have at most one | Name a run by the instance's ID and the item's name |
| P4 | A setup asks for the run only where the provider does not report it running (1c); a retirement asks for one only where a retire hook is selected and the run is not running (4c) | 1c leaves a running runtime as it is; 4c names a retirement that runs it for its hook | Every setup asks for a new run |
| P5 | The engine's invocation of `run.v1`, `stop.v1`, `inspect-run.v1`, `list-machines.v1`, `place.v1` and `fetch-package.v1` is a "provider call", not a request: no Conduit, no access decision; it is recorded; and no Conduit may name those three capabilities (PRV-004, PRV-005). A secret read is a request (S4.4) | Otherwise any Conduit to a runtime provider could run or stop another instance's run, and a call with no caller would return, against 4c | Make them ordinary operations that Conduits may list |
| P6 | Providers of runs, machines and packages run on the engine's own runner at `local` (PRV-002). The machines provider and package-source providers are owned by `system`, at most one machines provider, one package-source provider per URL scheme, with the schemes in its configuration under `schemes` (PRV-013, PRV-016) | 31b: "the machines provider is the one the root Interface registers", and a package source "is picked by the scheme of `fetch`"; something must record which scheme | A key in the Type contract for schemes; registration reserved to the root Interface |
| P7 | A location for a reported machine is named by the first label of its hostname, lowercased, where that satisfies OBJ-001 and is free; otherwise by OBJ-003 (PLC-026). The hostname itself is its label `kubernetes.io/hostname` | 8d names a location "by its hostname", which may hold dots | Always a generated name |
| P8 | A repeat of an invocation that succeeded, and whose outputs the engine no longer keeps (DL10), reports conflict with the engine's ID, as a repeat of one in progress does (OPS-032) | 6b makes `isError` false only with outputs, and the protocol requires them where an `outputSchema` is declared | Keep outputs as long as the invocation's state |
| P9 | `register-replacement.v1` names the instance that holds the nested instantiation and, in `nested`, its name (LIF-009) | 17 removes a closed Conduit at once, so the retired instance can no longer be named | Keep removed Conduits until replaced |
| P10 | A request to install a package at an address that a submitted project's manifest states reports conflict (PKG-029) | 11 forbids the other order only | Reject neither, and let the package shadow the project |
| P11 | A submitted project's contracts keep SUB-008's digest as the identity of one submission, which a template keeps (NAM-009); outside the project's own documents they are named `address#name` (11) | Templates must keep what they resolved after the project is edited | Drop the digest and keep only the latest submission |
| P12 | A Requirement contract loses `appliesTo` (C26, with 3a and 8a), and a Type's `placement` becomes optional | `appliesTo` would admit only `conduit`; an empty selector means nothing | Keep `appliesTo: [conduit]` |
| P13 | Inspection reports a run's state as the condition `Running` (PRV-009) | 2a has an actor settle a run of unknown outcome by reading its status | A member `run` in the status |
| P14 | An entry's Resource takes `runtime` only from an input whose destination is `/entries/<name>/runtime`, as its placement (POOL-009, INS-012) | 31b gives every Resource the argument; entries write no standard argument | Entries always run on the engine's own runner |
| P15 | A `package` item states `digest` (the SHA-256 of the artifact, written as PKG-004) beside the server.json Package; `environment`, `arguments` and `headers` map the names that the Package or Remote declares (and, for `command`, any variable name) to JSON Pointers into the configuration | 25a requires a digest, 25c and 25d bind declared names to places | Put the pointers inside the Package |
| P16 | `diagnostic.v1` is SARIF 2.1.0's `result` reduced to `ruleId`, `level`, `message.text` and `locations[].logicalLocations[].fullyQualifiedName`, which holds the JSON Pointer | 24a names a JSON Pointer as a logical location | `physicalLocation` with a region |
| P17 | `fetch-package.v1` returns the archive in `archive`, base64 (RFC 4648) | A tool's outputs are JSON | A `file:` URL the provider writes to |
| P18 | `run.v1` returns `endpoint`, a Streamable HTTP URL; the engine's own runner gives none and reaches its process over stdio | 2 says the run returns the endpoint | A local socket path |
| P19 | `inspect.v1`'s output `records` is a list of `record.v1` (24c) | 24c gives `record.v1` no other place of use | Export only |
| P20 | A setup or retire hook is dispatched through no Conduit, for the caller of the setup or retirement, with an operation identity scoped to the instance alone (OPS-028), so a further setup by another actor does not repeat a hook that succeeded | 1c: the hook repeats "only as its operation identity allows" | Scope the identity to the caller too, as OPS-008 does |

P2 was changed by the owner as [After the ruling](#after-the-ruling) records: no removed ID
can be reused. P1's names were replaced by N1 to N20 of [ruling 42](../ruling-42/README.md).

## Choices that set behavior

Copied from the editor logs before their removal under ruling 58. These are the choices at
the time of each pass, including readings and alternatives; later rulings and the specification
govern. The source labels retain the log and its choice number.

### log-G1

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G1.md). 26 choices.

- log-G1, 1: `graphs.yaml`; The first two lines of the head comment now say that the file holds no promise and why it is kept; the history below them is kept as it was.
- log-G1, 2: `templates.yaml` head; The sentence on runtimes as pairs of operations that pass a handle is replaced by one on the hooks, citing the standards decision 1 names (Kubernetes PostStart and PreStop, Compose `post_start` and `pre_stop`).
- log-G1, 3: `lifecycle.yaml` head; "retired and unregistered" → "retired and removed" in its first line. The section comment "Retirement and unregistration" is kept as a record.
- log-G1, 4: TPL-003; Of the observable sentence "A step, a runtime or a member of a template's `controls` map that carries its own `implementation` key is rejected", only the step and the runtime are cut; the `controls` case stays. Beside the plan's hook sentence, "`set-up.v1` and `retire.v1` are not [selected here]: the engine implements them, as TPL-008 states" is kept from the old sentence.
- log-G1, 5: TPL-008; The plan's "the one invocation of `set-up.v1`" is written "each setup of it is one invocation", since an instance can be set up, and retired, more than once. The Conduit sentence points at LIF-032.
- log-G1, 6: TPL-009, TPL-018; The stop is asked for "where an item that is run is selected for it", as PRV-007 to PRV-009 say it; with no hook and no such item the retirement dispatches nothing and succeeds at once, as the old text said for no graph and no runtime. The observable says the stop is asked for whether or not the run is reported running, which is why the old rules on a start of unknown outcome have no successor.
- log-G1, 7: TPL-012; "asking for its run, under PRV-007, or connecting to its `remote` item" is written "asking for its run ... and connecting to each `remote` item among its selected implementations": TPL-019 limits the items that are run to one, but not the `remote` items, so an instance can have both. The order: a part is begun only once every part before it in the order, not marked optional, has succeeded; among entries, POOL-013's order. "Nothing else is dispatched" excepts the reads of SEC-010 and SEC-013, since a read is a request (P5).
- log-G1, 8: TPL-017, TPL-018; "Selects an implementation" covers a selection by default under TPL-004 as well as one in the `implementation` map. Model work: the hook starts none under IMP-001 and its onward requests none under IMP-008.
- log-G1, 9: TPL-018; Where the run cannot be had, as where a read fails, the hook is not dispatched and reports unavailable, as SEC-012 says for any request that was to go to that implementation.
- log-G1, 10: TPL-019; `then` states the rule ("an instance has at most one item that is run"); the observable says what an item that is run is, and that one item implementing several operations is one run.
- log-G1, 11: LIF-008; "has only the retired instance" → "has no instance that is not retired", since the retired one may have been removed (17); this is CON-004's and POOL-013's wording.
- log-G1, 12: LIF-009; `when` adds "or has been removed", the reason P9 gives. "Two instances" → a later one and the earlier one until it is removed; "has been unregistered, under OBJ-009" → "no longer exists"; the CON-003 citation goes with CON-003, and naming is OBJ-003's alone.
- log-G1, 13: LIF-011; Cites the core schema `condition.v1` (plan section 4).
- log-G1, 14: LIF-015; "would store settings" → "would store ... the values the control was sent", following REV-010 under decision 15.
- log-G1, 15: LIF-017; The observable says that a location whose machine the machines provider reported gone is declared lost in this sense, under PLC-024, which is the plan's term "declare lost" (8e). It points at LIF-012 for a template.
- log-G1, 16: LIF-018; The observable says that the actor tells whether the run was stopped from the condition `Running` (PRV-009), which replaces the route through TPL-015.
- log-G1, 17: LIF-021; The LIF-030 sentence becomes "A Resource whose shutdown is confirmed has been removed, under LIF-035, and gets no invocation of `retire.v1`". The REV-014 sentence says only that the retirement of a busy Resource reports conflict, so the Pool is not retired by the request; see "Still needs a decision".
- log-G1, 18: LIF-024; "unless an earlier one succeeded" goes: an instance whose retirement succeeded has been removed.
- log-G1, 19: LIF-026; "only as OPS-021 allows" → "only as OPS-021 and OPS-028 allow": OPS-028's identity is what makes the repeat.
- log-G1, 20: LIF-027; "A run that it asked for is stopped" is added to `then` as well as to the observable, so that the promise states its rule in `then`.
- log-G1, 21: LIF-028; "which is pending unless a start of unknown outcome keeps it unknown" goes; it points at LIF-024 alone.
- log-G1, 22: LIF-033; The plan's "or names a first-start object" is read as naming one in `instance`.
- log-G1, 23: LIF-035; The observable lists what follows from removal: recorded under REC-001, the name free again under OBJ-002, holders report it missing under LIF-019, and pending or unknown shutdowns not removed.
- log-G1, 24: OBS-002; The `then` is kept; the condition fields are in the observable, as the plan says.
- log-G1, 25: OBS-004; The observable calls a provider's restart "the provider's own setting", so the `must not` binds the engine and not the provider.
- log-G1, 26: IMP-002; Exactly the plan's changes; "removed" is not added to the list of events.

### log-G2

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G2.md). 28 choices.

- log-G2, 1: **Entries and documents alike.** PLC-002, PRV-006 and PRV-015 speak of an Agent or a Resource "given" no placement or a `runtime`, by its document or, for an entry's Resource, by an input of the Pool's template, so that POOL-009 and POOL-016 can point at them. The plan's wording named instance documents only.
- log-G2, 2: **"Stands for".** A location stands for the machine it was registered for, which its status names by its `kubernetes.io/hostname` label, until it is retired; a retired location stands for none (PLC-026). So a machine reported gone that comes back, or one whose location an actor retired, is given a new location at the next report (PLC-024, PLC-026): with a forward-only lifecycle (LIF-006) the only other reading would be that such a machine never gets a location again.
- log-G2, 3: **The name of a reported machine's location (P7).** "The first label of its hostname" is written "the part of the machine's hostname before the first dot", so that "label" keeps the one meaning of section 1. "Free" is written as "a valid name under OBJ-001 that no Resource that exists has under OBJ-002".
- log-G2, 4: **Who registers a location.** The engine itself, as at first start: no actor asks, no request is made, and the registration and its lifecycle states are recorded under REC-001 (PLC-026). A machine reported gone gets no location.
- log-G2, 5: **Gone counts as declared lost.** PLC-024 says the location "counts from then on as declared lost", so LIF-017, CON-008 and LIF-024 apply without a second clause each, as the term table's row for "declare lost" says. G1's LIF-017 and G3's CON-008 already read so.
- log-G2, 6: **PLC-011's `then`** now names the report of a gone machine beside `declare-lost.v1` as a way a location with instances is retired; otherwise "only the declare-lost operation" would contradict PLC-024.
- log-G2, 7: **PLC-017** loses "as a request to close a closed Conduit does under ACC-012": G3 rewrites ACC-012 so that such a request reports invalid.
- log-G2, 8: **PLC-007** with several selected items: the selector to match is the union of the type's and every selected item's (the plan said "the item's").
- log-G2, 9: **PLC-013** points an end with no placement at CON-006, which already rejects it, and says that the engine applies `shared-host.v1` itself and never sends it (C20).
- log-G2, 10: **PLC-023** covers a setup or retire hook as well as a request, since a hook is dispatched to the instance's implementation through no Conduit.
- log-G2, 11: **PLC-025** reads "selects" as TPL-019 does: by the `implementation` map or by default.
- log-G2, 12: **PLC-008** says that `location.v1` offers no capability and lists no control, as section 4 of the plan makes it, and that `local` is made at first start and the others under PLC-026.
- log-G2, 13: **POOL-013** carries GRA-002's rule (a part begins only once those for the entries it refers to have succeeded) and GRA-012's (order from the references alone, not from the order of writing); a part with no Resource that is not retired "is not dispatched and reports conflict".
- log-G2, 14: **PRV-002** also rejects an input of a Pool's template that supplies a placement or a `runtime` for a provider's entry.
- log-G2, 15: **PRV-003**: the built-in implementations are part of the engine; no package provides them, no Resource stands for them, no Conduit reaches them.
- log-G2, 16: **PRV-004 also covers a Conduit that lists an operation of the three capabilities**, not only one whose capability is one of them. Without it, a capability of one's own that lists `run.v1`, offered by a runtime provider's type, would open the hole that P5 closes. The coordinator may strike this and keep the plan's narrower rule.
- log-G2, 17: **PRV-005 carries the provider's own `instance`, `config` and `revision`**, beside the plan's `operation`, `invocation` and `caller`. Under 31b a provider is configured as it needs (a Docker context), and under 1b configuration travels in `_meta`; with the plan's list a provider could not learn its own configuration. The coordinator may strike this.
- log-G2, 18: **PRV-005, calls outside an invocation.** `place.v1` and `fetch-package.v1` are made while the engine decides a request, before it is dispatched (PRV-015 rejects the document and PKG-024 reports unavailable, both reasons and not outcomes), so they carry the request's `caller` and no `invocation`. `list-machines.v1`, and `inspect-run.v1` when the engine asks of its own accord for PRV-009, carry neither. The plan's "made within the invocation it serves" holds for `run.v1`, `stop.v1` and `inspect-run.v1` in a setup or retirement.
- log-G2, 19: **PRV-007** learns whether the run is running by asking `inspect-run.v1` at that part, and gives in `location` the hostname and labels of the machine that the location stands for. It covers the run asked for to serve a retire hook (TPL-018) as well as a setup's.
- log-G2, 20: **PRV-008** says a stop is not dispatched and reports unavailable where the runtime provider is not ready, as PRV-007 does for the run.
- log-G2, 21: **PRV-009** maps `running`, `stopped` and `unknown` to the condition's `status` `True`, `False` and `Unknown`; the report's time is `lastHeartbeatTime` and its `origin` the runtime provider.
- log-G2, 22: **PRV-010** keeps the level `may`; what the engine must not do because of a restart is in its observable, citing SEC-014.
- log-G2, 23: **PRV-012**: "its contract says it runs" is the prose of the provider's contract; no key of the Type contract lists registry types.
- log-G2, 24: **PRV-013** reports invalid (a rejected document under OBJ-023), not conflict.
- log-G2, 25: **PRV-015**: the provider names a machine for which a location already stands; one it names with no location makes the instance not instantiated, so that deciding a request registers no location. Alternative: register the location first.
- log-G2, 26: **PRV-016** compares a scheme without regard to case, as RFC 3986 says of schemes.
- log-G2, 27: **PRV-017**: a package-source provider that is not ready makes the request report unavailable under PKG-024, because fetching comes before dispatch.
- log-G2, 28: **Words that go** are avoided in new text: "host" (PRV-011 says "the machine that the engine runs on"; PRV-017 "server"), "policy" (PRV-010 says "its own setting for restarts"), "a handle is held" and "declaration" (a word the promises do not use).

### log-G3

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G3.md). 15 choices.

- log-G3, 1: **A bound is a restriction, not a filter.** Section 1 of the work order now defines a filter as a requirement other than the core package's, and a restriction as "a filter or a bound". ACC-029 said "The bound is a filter"; it now says "The bound is a restriction, which the engine applies itself". ACC-009 said "another Conduit that carries a filter", which under the old terms covered a bound; it now says "a restriction: a filter or a bound", so that its rule is unchanged.
- log-G3, 2: **"Part" is kept for its new meaning.** Where a rewritten promise used "part" in its ordinary sense, it was reworded: ACC-006 "not opened for the part that was within the bound" → "not opened, not even with only the operations that the bound names"; ACC-011 "No part of it remains in force" → "None of it remains in force"; CON-022 "plays no part" → "does not count". "Policy" was kept out of ACC-019 too: a provider "restarts a run of its own accord" (PRV-010 itself says "restart policy").
- log-G3, 3: **The rule in `then`.** Where the work order put a rule only in an observable, it was also stated in `then`: CON-018 states the C20 rule there; ACC-003 states the `reveal-secret.v1` exception there; ACC-015 states what a hook's part reports.
- log-G3, 4: **A machine reported gone (CON-008).** Decision 8e gives a gone machine PLC-017's effect, and PLC-024 now says the location then counts as declared lost. Without a clause, a Conduit whose consumer is on a gone machine would never close, since that consumer's shutdown stays unknown (LIF-024). CON-008's `then` names "placed on a machine that is reported gone, under PLC-024" beside "declared lost". The integrator may drop the clause if PLC-024's wording is taken to cover it.
- log-G3, 5: **Inspecting a closed Conduit (CON-008).** Its observable said that inspecting a closed Conduit reports which event closed it. A closed Conduit is now removed at once (LIF-035), so that sentence became: a request that names it reports invalid (OPS-029), and its records are kept (LIF-019). See "Still needs a decision", item 1.
- log-G3, 6: **Hooks in ACC-015 and ACC-018.** A hook is not a request (section 1: three kinds of request), so ACC-015's `when` says "a request or a hook", and a hook that cannot be dispatched has "its part" report the reason. ACC-018's `then` puts the hook's own onward requests under ACC-016.
- log-G3, 7: **Reconciling a hook (ACC-033).** "A step that invokes the implementation of its own instance was dispatched to that instance" became "a setup hook or a retire hook is dispatched to its own instance, under ACC-018, and is reconciled in the same way", that is, against the Pool that owns a Resource and against `management` otherwise.
- log-G3, 8: **Management verb (ACC-033, CON-024).** Defined by the three capabilities, since the core package now exports provider and hook operations that are not management verbs.
- log-G3, 9: **ACC-015's core-operation sentence** names `management`, `access` and a Pool, not a location: `location.v1` offers no capability now. `engine` is described as implementing the management verbs, as the core manifest of section 4 has it.
- log-G3, 10: **ACC-019** gained one sentence on the engine's read of a secret (only within the setup or retirement that asks for a run or a connection, SEC-011; none on a provider's own restart, PRV-010), so that the promise that no request follows an ended invocation covers the third kind of request.
- log-G3, 11: **ACC-030's trust statement** (decision 28) names the exceptions that still hold for a delegate: the reserved operations of ACC-032 and `reveal-secret.v1` under SEC-008.
- log-G3, 12: **ACC-034** says the delivery of a result is not decided again, and that a later repeat or a `tasks/get` is decided anew; through the closed Conduit it reports refused, as the work order says. See "Still needs a decision", item 2.
- log-G3, 13: **ACC-039** permits the read of a secret only "for a run that the retirement asks for to serve that hook", as SEC-011 limits a retirement's reads to the run; it also cites PRV-004 for the stop of a run, which no Conduit permits.
- log-G3, 14: **ACC-012**: a repeat of the request that closed a Conduit is answered "as OPS-009 and OPS-032 state", since OPS-009 now answers only where the engine still keeps the outputs.
- log-G3, 15: **CON-022** adds that no Conduit is instantiated that applies a filter and lists an operation for which its target selected `engine`, since `engine` states no `supports` (MAN-012). This was already so when `engine` supported `bound.v1` alone; the sentence makes it visible.

### log-G4

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G4.md). 21 choices.

- log-G4, 1: E1; OPS-009, OPS-032; OPS-009's `when` requires that the engine still keeps the outputs, so that OPS-009 and OPS-032 never both apply.; The plan puts DL10's limit in OPS-009's `then` and the case of outputs no longer kept in OPS-032; stated once in `when`, the two promises do not overlap.
- log-G4, 2: E2; OPS-009; An operation that declares no outputs has none to lose: a repeat of a succeeded invocation of it is always answered with the outcome and an empty `structuredContent`.; Carried from the current IFC-025 ("content to keep only where its operation declares outputs"); without it, OPS-032 could report conflict for an operation with nothing to keep.
- log-G4, 3: E3; OPS-008; The engine's state is kept "for every invocation", the operation identity "where it carries one" and the Conduit "where it went through one".; `reconcile.v1` names any invocation by the engine's ID, with or without an identity, as REC-004 had it; a hook goes through no Conduit.
- log-G4, 4: E4; OPS-008; "Asking by that caller, target and identity returns the state" became a repeat being answered from the one invocation (OPS-009, OPS-012, OPS-024, OPS-032).; Retrieval was the only other way to ask, and it goes.
- log-G4, 5: E5; OPS-021, OPS-028; A later dispatch of a hook is matched against the earlier invocation as OPS-009 matches a repeat, whoever is the caller of the setup or retirement; a hook that succeeded is not run again, one of unknown outcome only as OPS-012 allows, one that failed as a new attempt under OPS-024.; P20: the hook's identity is scoped to the instance alone. The caller of such a new attempt is left open (D1).
- log-G4, 6: E6; OPS-022; The parts of a retirement are those of TPL-009 and TPL-018, so the run that a retire hook needs (P4) counts; only a declared Conduit or an entry can be marked optional, and the stop of a run is not.; The old text made each stop of a runtime a step that is not optional; the plan's term "optional" names only those four things, two of which are parts.
- log-G4, 7: E7; OPS-022; "A step that GRA-003 leaves as succeeded" became "a part that CON-004 or POOL-013 counts as succeeded because what it does is done already".; GRA-003's rule moved into CON-004 and POOL-013 (plan, G1 and G3).
- log-G4, 8: E8; OPS-027; The observable says that the protocol has a receiver that stops send no response, so a cancelled invocation is usually unknown until reconciled; a `tasks/cancel` is decided under IFC-016; how long the engine waits is DL3 and DL10.; The baseline's words, as [the round-2 citations](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/mcp-citations.md#what-the-file-does-not-claim) quote them ("Closing the SSE response stream is the cancellation signal"; a server should "Not send a response for the cancelled request"); the review's decision 22 names DL3 and DL10.
- log-G4, 9: E9; OPS-014; The ID is learnt from the `_meta` of the request's result or of a repeat's (IFC-025 holds it for a request with an outcome and for OPS-032), or from the records (REC-002). To reconcile a setup or retirement that asked for a run or its stop, the actor checks the run's state under PRV-009.; The plan's "the result's `_meta` or the records", and its "checked by PRV-009". OPS-014 does not say whether a provider call can be reconciled by itself (see I3).
- log-G4, 10: E10; OPS-005; Added that a repeat under OPS-032 is not dispatched and reports conflict.; Its `when` now admits that repeat, and saying so keeps the reader from looking for its outcome.
- log-G4, 11: E11; OPS-029; The LIF-032 sentence, which hung on the removed PLC-022 sentence ("So is ..."), now stands alone; a `nested` that names nothing the holder holds is LIF-033's.; The removed sentence was its antecedent.
- log-G4, 12: E12; REV-001; "There is no second counter" → "There is no second revision"; the comparison follows an HTTP entity tag, as C21 says: nothing is read from a revision's form.; An opaque string is not a counter.
- log-G4, 13: E13; REV-007; The example of a clamp (20 asked, 15 confirmed) became the rule against one: 20 is stored, or the control does not succeed.; Decision 15 forbids clamping.
- log-G4, 14: E14; REV-010; "The values are not stated in the request: the engine stores those it sent."; Follows from removing the `settings` input; the engine keeps the values under OPS-008.
- log-G4, 15: E15; REV-013; Observable says when it arises: another change was stored after the control was dispatched, since REV-015 checked the same values against the configuration then.; It explains why the promise survives decision 15.
- log-G4, 16: E16; REV-014; The exception is "a repeat that OPS-009 or OPS-012 answers without a dispatch", where the review and the plan say "a repeat under OPS-009". A repeat that OPS-012 or OPS-024 dispatches as a new attempt is subject to REV-014.; After 6b, OPS-009 no longer covers the repeat answered with the outcome unknown; the exception's purpose, not reporting a busy conflict for what dispatches nothing, covers it too. Strike to keep the literal text.
- log-G4, 17: E17; REV-014; "A control is a change, and so is a reconciliation that stores the values of a control (REV-010)." The engine neither queues nor waits; the caller asks again later.; Names what "change" covers, from REV-010; restates "at once" as observable behavior.
- log-G4, 18: E18; REV-015; The check is made after the defaults of supplied values are filled (OPS-026), with an unsupplied input's setting checked as stored (REV-006); the example uses `maximum`.; The plan's text with what it implies from OPS-026 and REV-006.
- log-G4, 19: E19; REC-001; A provider call is recorded when it is made and when it ends, "with how it ended".; PRV-005 says it is recorded under REC-001, and REC-001 lists events.
- log-G4, 20: E20; REC-002; The removed OBS-007 sentence is replaced by "every other request has a caller": a read of a secret has the caller of the request that asked for the run (SEC-011), and a provider call's record holds the caller of the invocation it serves where it has one (PRV-005). `inspect.v1` returns records as `record.v1` (P19).; Decision 2 and 4c remove the request for no actor; the plan's terms say every request has a caller.
- log-G4, 21: E21; REC-004; The sentence "What a purge keeps of an invocation is read as records of that instance" was cut.; Under C33 a purge keeps nothing as records. See D3.

### log-G5

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G5.md). 18 choices.


#### Choices

- log-G5, 1: IFC-006; The step sentence is pointed at hooks rather than only cut (plan rule 8): `conduit` is absent for a hook, which carries the restrictions of its setup or retirement under ACC-018. Added pointers to SEC-011 (whose read it is) and PRV-005 (what a provider call carries). "Every request has a caller" is limited to requests the engine sends, so it does not contradict IFC-032's record with no caller.
- log-G5, 2: IFC-008; The outcome stays failed, as before: IFC-008 obliges the implementation to perform nothing, so its -32021 error says nothing was done. The record names the revision declared.
- log-G5, 3: IFC-004, IFC-008, IFC-037; -32021 is described as the error a server gives a request that does not declare a capability it requires, as the second check against the protocol (`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/mcp-recheck.md`, row 26) reads it. The declaration goes in `io.modelcontextprotocol/clientCapabilities`.
- log-G5, 4: IFC-012, IFC-014; The length sentences go from IFC-012, as the plan says; IFC-014's observable keeps the 128-character statement with its reason (63 + 1 + 63).
- log-G5, 5: IFC-016; `when` kept as it was. The observable says how a caller with no task learns the result, which replaces retrieval (6c).
- log-G5, 6: IFC-017; `when` names the items reached by `command`, `package` or `remote`. Where the engine's own runner runs an item, how it starts the process is the engine's own.
- log-G5, 7: IFC-025; `invocation` names the invocation the request began or repeats. `isError` is said to keep the protocol's meaning. The ACC-034 sentence is kept in its new sense (a result whose Conduit closed holds its outputs). Diagnostics are in the form of `diagnostic.v1`; an error diagnostic is one whose `level` is `error`.
- log-G5, 8: IFC-028; Adds that a hook is an invocation dispatched to the instance, and its onward requests name it.
- log-G5, 9: IFC-029; Requests to the engine and results "are read by the prefix of their keys", rather than "declare no revision", so that a consumer is not forbidden to declare the extension. The extension's name is the prefix followed by `interface`.
- log-G5, 10: IFC-033; A hint the contract does not state, and `readOnlyHint` where it is not true, is left out of `annotations`, so the protocol's default holds.
- log-G5, 11: IFC-037; No extension of Conduit's is declared to a plain implementation.
- log-G5, 12: IFC-038; The notification is named generically ("the protocol's notification that the tool list changed"); the streams notified are those on which a request of the instance asked.
- log-G5, 13: IFC-039; Binds the engine, a consumer and an implementation. The reserved trace keys are not named: no check against the protocol in this repository quotes them.
- log-G5, 14: IFC-040; The observable extends to the instance that IFC-022 establishes, since it is part of every access decision.
- log-G5, 15: IFC-041; The connection is the part of the setup that TPL-012 names; the observable says it serves one instance, that a `remote` with no `headers` is sent none, and points at IFC-030.
- log-G5, 16: DL8; "unregistered" became "unregistered or removed", to match decision 17 (REC-003 is rewritten the same way).
- log-G5, 17: Head comments; Each file gains one paragraph naming the removed IDs, where their content went, the changed and added IDs, and this log. The interface file names the removed keys without the prefix, so that `tools/set_names.py`, which reads the keys from that file, does not count them as defined.

#### Outside the plan

- log-G5, 1: IFC-009 is listed as unchanged in the plan, but its observable said that an implementation sets `isError` "as IFC-008 states for one such result". After IFC-008's rewrite, IFC-008 gives no result. The clause was cut and nothing else changed. The coordinator confirms or reverts this.

### log-G6

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G6.md). 19 choices.

- log-G6, 1: OBJ-010: "or to none" is kept, now explained by "because the object it resolved to no longer exists", and "whatever the objects are now named" is cut, since names no longer change.
- log-G6, 2: OBJ-011: the observable says that an object is known by another name only where another object is made under it, as decision 10's text says, citing Kubernetes `metadata.name`.
- log-G6, 3: OBJ-013: the example "`runtime` on a Pool" is added, matching the planned example `OBJ-013-runtime-on-a-pool`.
- log-G6, 4: OBJ-015: the observable says that a property of `inputSchema` that states a `default` may be left out even where `required` lists it, because OPS-026 fills it before the request is checked against `inputSchema`; and that `optional: true` inside a schema is outside the profile (SCH-002).
- log-G6, 5: OBJ-019: the exemption "a label that keys a member of a label selector under `placement`" is added. The plan does not list it, but labels are not names (OBJ-022), so without it OBJ-019 would reject every label selector. `setup` and `retire` join `runtimes` in the example, as TPL-001 has a template that writes them rejected under OBJ-019.
- log-G6, 6: CTR-002: an empty property name is still rejected ("any non-empty string", as the plan says), and `inputs` and `outputs` are named as keys that are now rejected.
- log-G6, 7: CTR-005: `schema` is named among the removed keys, since a requirement on placement, which stated one, goes; the two core requirements are named with their promises.
- log-G6, 8: CTR-012: `type: object` is required at the root, not only allowed, because the Model Context Protocol's tool definition requires it. The plan's "anything but `type: object`, ..." was read this way.
- log-G6, 9: NAM-012: `when` drops "that differs from the item's address", since an address is no longer a place to fetch from (decision 21). `then` says every engine "accepts" `https` and `file`, and points at PRV-017 for which of the engine and a provider fetches, because PRV-017 lets a provider claim `https` too. A credential is "user information", the part of a URI where RFC 3986 places one; MAN-006 and PKG-023 use the same words.
- log-G6, 10: NAM-022 and SUB-009: the plan's "the project contracts that the template being replaced holds" is written "the contracts and implementations of the submitted project that the template being replaced holds", in the plan's term "the contracts of a submitted project", since `#name` reaches implementations too.
- log-G6, 11: PKG-006 and PKG-028: "not installed at the version and digest that the manifest gives", as decision 12 words it. NAM-008 was narrowed to match (see the table).
- log-G6, 12: PKG-019: "A `package` item also states `digest`" and a pointer to PRV-012 are added, as MAN-010 requires the digest.
- log-G6, 13: PKG-023: a `fetch` whose scheme nobody accepts reports invalid with no qualifier, as a relative one does; a request for a package already installed needs no `fetch`, since PKG-025 and PKG-027 fetch nothing.
- log-G6, 14: MAN-010: which keys go with which way of reaching an item (`environment` with `command` or `package`, `arguments` with `package`, `headers` with `remote`, `digest` with `package`) is stated in its observable, as PKG-019's plan line gives it. For a `package` item, `environment` names only variables the Package declares (P15); for a `command` item, any.
- log-G6, 15: MAN-012: "may state `placement`" is kept for `engine`, since the plan does not drop it and the core manifest writes `placement: {}`.
- log-G6, 16: MAN-015: a project with contracts and no manifest at all is rejected too.
- log-G6, 17: MAN-017: a type that states no `schema` requires nothing, so any pointer is rejected.
- log-G6, 18: SUB-008: the digest's computation is kept word for word, including "left out when the project has none", although MAN-015 now makes that case unreachable for a project with contracts, so that the computation does not change.
- log-G6, 19: `spec/schema-profile.md`: the whole keyword list stays in the first paragraph of its section, because `tools/check_schemas.py` reads only that paragraph; `$anchor`'s exclusion gives the review's reason. The Digests, Canonical form and Pointers rows were reworded too, since "unpublished package" and "value reference" are terms that go. In `manifest.yaml`'s head comment, the list of practices drops the re-export and adds `.yml` and server.json.

### log-G7

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G7.md). 16 choices.

- log-G7, 1: C1; SEC-001; "Under an applicator beside any branch other than `{type: null}`" is read as: `secret-reference.v1` may stand as the schema of a place itself (so under `properties`, `patternProperties`, `additionalProperties` or `prefixItems`), as `items`, or as a branch of `anyOf` or `oneOf` whose every other branch is `{type: null}`. Under `allOf`, `not`, `if`, `then`, `else`, `dependentSchemas` or `contains`, or beside any other branch, the schema is rejected.; The review's table says that once decision 20 admits `if`, `not` and the other applicators, "the rule covers them too". `contains` is listed because a list under it may hold literals beside one reference.
- log-G7, 2: C2; SEC-001; A bare name is rejected at such a place, as a literal is; a reference to a Pool's entry and an entry reference are accepted.; The work order's export: "a reference schema of the kind Resource, no bare name, accepting the resolved form"; the draft SEC-1.
- log-G7, 3: C3; SEC-004; The `when` covers any record of a request whose kept inputs hold a value at such a place, not only the inputs of an operation whose `inputSchema` names the schema (the draft SEC-22's scope).; The work order states the rule without that limit, and S4.1 says no record holds a literal. The core package's types keep no inputs, so the case arises only for a type whose `records` keeps inputs. See D5.
- log-G7, 4: C4; SEC-006; The observable says a plain implementation of `reveal-secret.v1` learns its locator through its item's `environment` or `arguments` (MAN-010), since IFC-007 sends it no keys.; It states what follows from 1b and 25d, and adds no rule.
- log-G7, 5: C5; SEC-007; Written as a request that "an instance sends: an actor on its own behalf, or any instance as an onward request", so that a Resource's onward request for the operation is refused too.; 4b: "Only the engine's read under 4a is dispatched." The work order's words "an actor's request ... or as an onward request" read the same where an actor's request means one whose caller is an actor.
- log-G7, 6: C6; SEC-008; "The actor is the root Interface" is written "the root Interface sent it on its own behalf": an onward request that serves the root Interface is refused, and the part of a setup that opens such a declared Conduit is the root Interface's request only where it sent the setup (ACC-005, POOL-015).; S4.4 ("a Conduit that only the root Interface opens") and the draft SEC-7's observable.
- log-G7, 7: C7; SEC-009; Kept the work order's "is not instantiated", in CON-021's form, and not the draft SEC-6's "the template is rejected".; The work order fixes the rule.
- log-G7, 8: C8; SEC-010; The read goes through exactly one open Conduit of the instance to the secret Resource that lists `reveal-secret.v1`; with none, or more than one, it reports refused.; The draft SEC-16's observable, which the review's table keeps under the proposal's line 8a; with two, choosing one would bypass the other's filters.
- log-G7, 9: C9; SEC-010; The value read is passed in `secrets` "in place of the resolved form that the configuration holds at that place"; a place that holds null under `{type: null}` reads nothing.; PRV-007's "the values of SEC-010 apart in `secrets`"; null is not a reference under OBJ-021. See D4.
- log-G7, 10: C10; SEC-011; Where the instance is set up as a part of a Pool's setup (POOL-013), the read belongs to that setup.; 4c: the read belongs to the request that asked for the run.
- log-G7, 11: C11; SEC-012; The part of the setup or the retirement that would ask for the run or make the connection reports unavailable, and no later part is begun.; PRV-007 has the part report unavailable where the runtime provider is not ready; TPL-012 begins a part only after the ones before it succeeded.
- log-G7, 12: C12; SEC-013; "Sends the value only to that item's URL" is taken to exclude an HTTP redirect to another URL.; A redirect is a request to another URL.
- log-G7, 13: C13; SEC-015; The record names, as REC-002's "instance that acted for the caller", the instance whose Conduit the read went through.; REC-002's fields, with the caller of SEC-011.
- log-G7, 14: C14; SEC-018; The observable says the record of the provider call to `run.v1` holds no value of `secrets`, whatever the runtime provider's type states under `records`.; PRV-005 records every provider call, and S4.7 puts the value in no record.
- log-G7, 15: C15; SEC-020; "Gives a run any variable of its own environment" is written as what the engine passes to a run in `environment`, `arguments` or `secrets`, matching PRV-007's observable; what else a process receives is the provider's.; The review's table, line 18: "what else a run receives is the provider's (2c, 13b)". See D3.
- log-G7, 16: C16; All; "Store" means the system outside the engine that a locator names (a keychain, a vault, a file); the object is always "secret Resource"; the plan's term "secret store" is not used in the promises.; See the terms below: the plan's rows for secret store and secret Resource name one object twice.

### log-core

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-core.md). 16 choices.

- log-core, 1: The plan's rule `location-from` is named `first-start-from` and checks all five templates that FST-006 names, not `location` alone: the check is the same, and FST-006 rejects all five. Because of it, `examples/valid/first-start/` is exempt from that one rule: it writes out the documents that first start makes, which no submission may hold, and the tool names the directory in `FIRST_START_DIRECTORY`.
- log-core, 2: `object.v1`'s `nested` is `{instance, name}`, both required, where it was `{instance, path}` with `path` a list for components: with no component, the list had one name, and LIF-009 names a nested instantiation by those two. Not in the plan's list.
- log-core, 3: `run.v1`'s `secrets` is keyed by the JSON Pointer of the place each value was read for, which is how PRV-011 words the delivery ("the variable or the argument that the item binds to the place it was read for"). G7's question D1 is still open; the key is the one line to change.
- log-core, 4: An operation with no inputs states no `inputSchema`, and one with no outputs no `outputSchema`. IFC-033 gives the tool of an operation with no outputs an `outputSchema` that describes a map with no properties; read the same way, the tool of one with no inputs has the `inputSchema` `{type: object}`, which IFC-033 does not state.
- log-core, 5: The core operations' `inputSchema` states no `additionalProperties: false`: IFC-001 already makes a request's `arguments` exactly the operation's inputs.
- log-core, 6: `server-package.v1` and `server-remote.v1` leave out server.json's descriptions, examples and defaults, and are as open as server.json is. Its defaults are annotations there, and one stands inside `allOf`, where SCH-008 rejects a `default`. `server-remote.v1`'s `url` also carries `format: uri`, beside server.json's pattern, since MAN-010 requires a URI.
- log-core, 7: `diagnostic.v1`'s `ruleId` must have the form of a promise ID, and the schema stays open, as SARIF's `result` is, so that an engine may add what else SARIF holds. A diagnostic names the document it is about by no fixed member.
- log-core, 8: `record.v1` requires `specversion`, `id`, `source`, `type`, `time` and `data`; `data` holds REC-002's fields under the names IFC-006 gives Conduit's keys (`caller`, `instance`, `conduit`, `operation`, `invocation`, `operationIdentity`) with `target`, `outcome`, `reason`, `inputs` and `outputs`, none required, since each is present only where it applies. The values of `source` and `type` are left to the engine, so that no new name is coined before the vocabulary audit.
- log-core, 9: `condition.v1` keeps the patterns, lengths and `required` of Kubernetes `metav1.Condition` (`type`, `status`, `reason`, `message`, `lastTransitionTime`). `origin` is a non-empty string.
- log-core, 10: `label-selector.v1` gives a value the syntax of a Kubernetes label value, beside the plan's "values strings", since labels are Kubernetes's (C58).
- log-core, 11: MAN-006's "no user information" is stated in `manifest.v1` with `not` and a pattern, as decision 20 allows; `install.v1`'s `fetch` has the format only.
- log-core, 12: The keys of an item's `environment`, `arguments` and `headers`, of a control's `changes` and of a template's control `inputs` are non-empty strings by `propertyNames`; that a name is one the Package or Remote declares (MAN-010) is left to the engine.
- log-core, 13: The core manifest's `engine` states no `placement`, where it stated `placement: {}`; MAN-012 still allows one.
- log-core, 14: `type-spec.v1` does not state PRV-001, PRV-016 or SEC-005, which `if` and `contains` could express: the plan gives PRV-001 to the tool, with an example that the schemas must accept, and gives the other two to no one. The schemas' README lists all three.
- log-core, 15: `secret-branch` does not follow a `$ref` to a member of `$defs` that holds `secret-reference.v1`; it treats `items`, a member of `properties`, `additionalProperties` and a member of `$defs` as the places SEC-001 allows.
- log-core, 16: `item-place-required` follows `properties` and `required` and stops, judging nothing, at a `$ref`; it takes the empty pointer, the whole configuration, as always present.

### log-docs

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-docs.md). 8 choices.

- log-docs, 1: **secret store.** The work order defines it as a provider whose type offers `secret-reveal.v1`, which is also its definition of secret Resource: two terms for one object. The promises use "secret Resource" for the object and "store" for the system outside the engine that its locator names (log of G7, C16; integration, PRV-001 and PRV-002). So "secret Resource", "store" and "locator" have rows, and "secret store" is listed under the words the promises do not use, with what the core package's descriptions mean by it.
- log-docs, 2: **read.** The promises say "the engine's read of a secret", and "a read" for short (log of G3); the row has that name and says so.
- log-docs, 3: **label.** NAM-001 uses the word in its DNS sense, one part of a host name, beside the new sense of a machine's label. The row gives the new sense and names that one other use, so that a reader is not misled; the vocabulary audit may rename one.
- log-docs, 4: **depends on.** The work order keeps it as "only an entry on the entries it refers to". The row moved from the section on graphs to "Ownership", beside entry, and says also that an instance depends on its location through its placement (PLC-015).
- log-docs, 5: **output.** The work order's table removes "output (of a graph)" and keeps an operation's outputs. The old row mixed the two; the new row keeps the second only, in the `outputSchema` form.
- log-docs, 6: **resolving.** Its row still says that resolving runs nothing. It now adds that placing an instance that is given no placement is not resolving (PLC-002, PRV-015). PLC-007, PLC-013 and POOL-016 are still of the layer `language` while they read labels that a provider reported; that is question R14 of the integration log, which this pass did not settle.
- log-docs, 7: **hook and request.** A hook is not a request in the sense of the row for request (three kinds), as the work order and the log of G3 have it, and the row says so. IFC-006 still speaks of "the request" that is a setup hook or a retire hook, meaning the protocol's request: a word for the vocabulary audit.
- log-docs, 8: **user** and **trust domain** are new rows for words the promises now use (PRV-001, PRV-003, PRV-012, NAM-012; PRV-011).

### log-integration

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md). 6 choices.

- log-integration, 1: PRV-004 also refuses a Conduit that lists an operation of `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1` under another capability (G2, choice 16).
- log-integration, 2: PRV-005 has a provider call carry the provider's own `instance`, `config` and `revision`, so that a provider learns its configuration as any implementation does under IFC-006 (G2, choice 17).
- log-integration, 3: REV-014 excepts a repeat that OPS-009 or OPS-012 answers without a dispatch, where the plan says "a repeat under OPS-009" (G4, E16).
- log-integration, 4: TPL-012 connects to each `remote` item among the selected implementations, where the plan says "or connecting to its `remote` item" (G1).
- log-integration, 5: IFC-009's clause "as IFC-008 states for one such result" was cut (G5); NAM-008, NAM-021 and PKG-017 were changed beyond the plan's list to agree with PKG-006, MAN-016 and MAN-004 (G6).
- log-integration, 6: OBJ-019 exempts the labels of a label selector, and CTR-012 requires `type: object` at the root of `inputSchema` and `outputSchema` (G6, choices 5 and 8).
