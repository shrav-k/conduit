# Round 5, reader 1, contradictions

Read cold at commit d0c425f on `docs/pre-codex`, opening nothing outside `spec/`: README.md,
definitions.md, names.yaml, schema-profile.md, implementation-defined.yaml, the 15 files under
`promises/`, the 7 under `conventions/`, `interface/implementation.yaml`, the 8 files of `core/`,
the 9 of `schemas/`, and all 91 examples (7 valid, 56 invalid, 28 invalid-by-rule). The counts
that the READMEs state hold: 449 promises (202 language, 206 convention, 41 interface), 16
implementation-defined items, 102 exports of the core package, 58 of them Schema contracts. No
promise ID is cited outside a head comment that no longer exists, and the words that rulings 41
to 43 retired do not survive in the normative text.

24 findings: 1 unsafe, 4 contradiction, 4 undetermined, 6 wrong, 2 misleading, 7 minor.

Judgment: the text agrees with itself far more than it disagrees. What does not agree clusters in
three places: a version rule (NAM-024) that was written with operations in mind and lets a
filter be ignored in part; behavior that ruling 43 added (the gate, SEC-024, DL17, PRV-026) after
descriptions and README bullets that still speak absolutely; and implementations whose shape the
promises assume without saying it (an Interface's or a Pool's run, a provider reached by
`remote`, which packages the engine's own runner runs).

---

## 5-1-01 — unsafe — a filter's optional input, added by a later release, can be ignored

Promises: CON-022, NAM-024, IFC-019, IFC-018, ACC-015, NAM-019.

Quotes: CON-022: "the version of the package does not count, because a later release may only add
optional inputs under NAM-024". NAM-024: "The later contract may add optional things, such as an
optional input of an operation". IFC-019: "If one of them forbids the call, or it cannot apply
one of them, it performs nothing". ACC-015: "whatever version of its package the implementation
holds".

Situation: `example.org/files` 0.3.0 exports the Requirement `rate-limit.v1` with the input
`limit`; 0.4.0 adds an optional input `paths` that narrows the filter, as NAM-024 allows. A Conduit
template registered against 0.4.0 applies `rate-limit.v1` with `{limit: 20, paths: [/public]}`,
which NAM-019 checks against 0.4.0. The target's selected implementation comes from 0.3.0 and
states `rate-limit.v1` in `supports`, so CON-022 instantiates the Conduit and IFC-018 sends both
arguments. The 0.3.0 server applies the one input its contract declares. Nothing makes an argument
that its version does not declare a filter it "cannot apply", and ignoring an added optional input
is what compatibility under NAM-024 ordinarily means. Requests outside `/public` are performed, and
ACC-038 reports that server as applying the filter.

Least change: in IFC-019, an implementation that is sent a filter argument that its version of the
requirement does not declare cannot apply that filter, and answers unavailable; or in CON-022,
match a filter also against the version that declares every argument the Conduit applies it with.

## 5-1-02 — contradiction — SEC-024 is a language promise whose result depends on the engine's configuration

Promises: SEC-024, DL17; README (layers, conformance); definitions (resolving, conformance document).

Quotes: SEC-024: "on an engine that, as configured, can neither restart on another machine nor run
as several parts, as its conformance document states under DL17". README: "Everything here is
answered by resolving, which reads the documents against what already exists, the lifecycle state
and the status of the objects that exist among it". Definitions, resolving: "The function from a
set of documents, read against what already exists". Conformance: the Language level is "documents
in, resolved objects or diagnostics out", and only the Engine level has a conformance document.

Situation: a template whose plain `remote` item at `http://127.0.0.1:8080/mcp` binds a header to a
secret. Whether resolving rejects it depends on how the engine is configured, which is neither a
document nor an object that exists. An engine that claims the Language level has no conformance
document, so the condition that SEC-024 reads is undefined for it. Two engines given the same
documents and objects resolve them differently.

Least change: give SEC-024 the layer `convention`, or add the engine's configuration that DL17
states to what resolving reads, in definitions.md and in the README's `language` row.

## 5-1-03 — contradiction — LIF-023 forbids the retirements that PLC-024 requires

Promises: LIF-023, PLC-024, CON-008, LIF-021.

Quotes: LIF-023 (`must not`): when "No actor has asked for it.", then "An instance is retired."
PLC-024: "The engine declares the node that represents it lost ... the node and every instance
placed on it are retired." Its observable: "No actor asks for it, and no request is made."

Situation: the machine provider reports a machine gone. PLC-024 has the engine retire the node and
every instance on it with no actor's request; LIF-023's rule, as its `when` and `then` state it,
forbids exactly that. Only LIF-023's observable lists the exception, together with CON-008's
closings and LIF-021's retirements.

Least change: move the exceptions into LIF-023's `when`: no actor asked for it, and it is not a
Conduit that CON-008 closes, an instance that PLC-017 or PLC-024 retires, or a Resource that
LIF-021 retires.

## 5-1-04 — contradiction — the example for PRV-004 rejects a template that PRV-004 lets be registered

Promises: PRV-004, CON-028; examples/invalid-by-rule/PRV-004-conduit-to-a-runtime-provider.yaml;
schemas/README.md (the PRV-004 row).

Quotes: PRV-004: "The Conduit is not instantiated: no Conduit permits an operation of those
capabilities." The example: "One document, valid against the schemas, that must be rejected"; it is
a Conduit template alone. The schemas README applies the rule "PRV-004, for a Conduit template".
CON-028 by contrast says both: "A template that applies it more than once is rejected when it is
registered. A Conduit that applies it ... is not instantiated."

Situation: a Conduit template whose capability is `runtime-provider.v1`. Under PRV-004 it is
registered, and no Conduit is ever instantiated from it. Under the example and the tool, the
template itself is rejected. An engine that follows the promise and one that follows the example
differ on the same submission.

Least change: add to PRV-004 that a Conduit template whose capability is one of the three, or that
lists one of their operations, is rejected when it is registered; or turn the example into a set
with a Conduit instance document, and change the tool's column to say so.

## 5-1-05 — contradiction — the example project's "existing server" is not marked plain

Promises: PKG-020, IFC-037, IFC-007; examples/valid/project/conduit.yaml (`git-mirror-server`).

Quotes: PKG-020: when "An implementation is an existing server that was not written for this
specification", then "Its implementation item in the manifest is marked `plain: true` and gives
`tools`". The example: "An existing server, found by name, that mirrors a Git repository." The item
`git-mirror-server` states neither `plain` nor `tools`.

Situation: read as PKG-020's words read, this server knows nothing of Conduit, so its discovery
result does not list Conduit's extension. Under IFC-037 every request for `mirror.sync.v1` on the
example's Resource `mirror` is then not dispatched and reports unavailable. The valid example shows
a pattern that a reader copies, and which does not work.

Least change: mark `git-mirror-server` `plain: true` with `tools: {'#mirror.sync.v1': <name>}`, or
describe it as a server written for this specification.

## 5-1-06 — undetermined — where an Interface's or a Pool's run runs, and what `machine` it is given

Promises: PRV-007, PRV-019, PRV-003, PRV-027, TPL-001, TPL-012, PLC-006, OBJ-013, IFC-017; `run.v1`.

Quotes: PRV-007: "in `machine`, the hostname and the labels of the machine that the instance's node
represents". PRV-019: "It runs the item on the machine given in `machine`, and on no other." Its
observable: "so `machine` always holds the machine of a node that is not retired". TPL-001 lets an
Interface or a Pool template hold `implementation`. PLC-006 and OBJ-013 give neither a placement or
a `runtime`. `run.v1` requires `machine`.

Situation: an Interface whose type offers a capability with an operation implemented by a `command`
item, such as a notifier that asks a person, or a Pool type that offers a further capability. Its
setup comes to the run (TPL-012), and PRV-003 has the engine's own runner serve it, since it names
no `runtime`. But it has no node, so nothing gives `machine`, PRV-019 has no machine to run on, and
the own runner "ends failed the run of one placed on any other node". One engine runs it on the
machine that `local` represents, another ends the run failed, and a third rejects the template.

Least change: state that an Interface's or a Pool's item that is run runs on the engine's own
runner on the machine that `local` represents, with that machine in `machine`; or reject a template
of those families whose selected implementations include an item that is run.

## 5-1-07 — undetermined — which packages and transports the engine's own runner runs

Promises: PRV-012, PRV-003, DL16, IFC-017, MAN-010.

Quotes: PRV-012: a run of a `package` item "whose `registryType` is not one that its contract says
it runs, or whose `transport` is of a `type` that its contract does not say it runs" "ends failed".
PRV-003: the own runner "keeps every promise that this file makes of a provider of that kind".
DL16: "this item is the contract that PRV-011 and SEC-020 leave to a runtime provider". IFC-017:
"the engine reaches the process it starts over stdio".

Situation: the valid package's item `listing` (`npm`, `stdio`) is selected for an instance that
names no `runtime`. DL16, the own runner's contract, says nothing of registry types or transports,
so one engine runs it with `npx` and another ends the run failed under PRV-012, and neither need
say so. A `package` whose transport is `streamable-http` cannot be reached over stdio at all, and
nothing says what the own runner does with one.

Least change: add to DL16 which registry types and transports the engine's own runner runs, and
that it ends the run of any other failed under PRV-012.

## 5-1-08 — undetermined — a provider whose implementation is reached by `remote`

Promises: PRV-002, PRV-026, PRV-025, PKG-020, IFC-037, IFC-041; definitions (provider).

Quotes: PRV-002: "Its placement is `local` ... and the engine's own runner runs it." Its
observable: "The engine reaches the provider's server over stdio, so no gate stands before it".
Definitions: "The server of a runtime provider, the machine provider or a package-source provider
is run by the engine's own runner on `local` and reached over stdio". PKG-020 rules out only a
plain item for those operations, and IFC-037 speaks of "a provider call ... to a provider's
implementation item" and of "an item reached by `remote` at each connection".

Situation: a machine-provider type whose default implementation is a non-plain `remote` item at an
`https` URL, a cloud API wrapped as a server. Nothing runs it, the engine cannot reach it over
stdio, PRV-026 has no run to make again, and PRV-025 and IFC-041 would put a gate before it where
PRV-002 says there is none. No promise rejects it, so one engine rejects it, under a rule it
chooses, and another makes provider calls to it over Streamable HTTP.

Least change: reject an item reached by `remote` that implements an operation of
`runtime-provider.v1`, `machine-provider.v1` or `package-source.v1`, beside PKG-020's rule; or say
in PRV-002 how such a provider is reached.

## 5-1-09 — undetermined — DL16 and PRV-026 each leave a secret Resource's process to the other

Promises: DL16, PRV-026, PRV-002, SEC-010.

Quotes: DL16: "what becomes of the processes it started when the engine stops and when it starts
again, apart from the servers of the runtime, machine and package-source providers, and the
processes of the secret Resources that it runs, which PRV-026 has the engine run again". PRV-026:
"and so does the process of a secret Resource that names no `runtime`, under PRV-003, where DL16 has
it end with the engine; DL16 leaves each to this promise". PRV-002 says that a provider's server
ends when the engine stops, and says no such thing of a secret Resource.

Situation: the engine restarts while a secret Resource runs on its own runner. PRV-026 applies only
if that process "ended when the engine stopped", and whether it ends is left to DL16, which leaves
it to PRV-026. Engine A ends the process, and before the next read through it runs it again, with
reads for the caller of its last setup that its records show. Engine B keeps the process and makes
no such reads. Neither states its choice.

Least change: state in PRV-002 that the process of a secret Resource on the engine's own runner
ends when the engine stops, as a provider's server does; or drop the secret Resources from DL16's
exception and keep PRV-026 conditional on DL16.

## 5-1-10 — wrong — `register.v1` says when a registration is rejected for want of a machine, and PRV-015 says otherwise

Promises: PRV-015, LIF-009; core/operations.yaml (`register.v1`).

Quotes: `register.v1`: "for want of a machine it is rejected only where no node matches". PRV-015:
"Where the provider names none, the instance is not instantiated, unless a node that is not retired,
other than `local`, has labels that match the selector and its machine is reported unreachable or
omitted."

Situation: the selector matches a node whose machine is reported reachable, or matches `local`
alone, and the machine provider names no machine. Under PRV-015 the document is rejected; the
description says a registration is rejected only where no node matches. The core README says that
where a document there and a promise differ, the document is wrong.

Least change: "for want of a machine it is rejected where the provider names none, unless a node
other than `local` that matches represents a machine reported unreachable or omitted".

## 5-1-11 — wrong — two descriptions say a secret Resource's server is reached only with the engine's proof

Promises: PKG-020, SEC-006, PRV-001, PRV-025, IFC-041; core/capabilities.yaml (`secret-provider.v1`);
core/operations.yaml (`get-secret.v1`).

Quotes: `secret-provider.v1`: "A secret Resource's server is reached only with the engine's proof,
as any server that the engine does not run itself is." It then says that a plain store reached by
`remote` "is then reached as before, with no proof". `get-secret.v1`: "The server that gives the
value is reached only with the engine's proof". PKG-020: "a plain server reached by `remote` stands
behind no gate and is reached as before".

Situation: a secret Resource whose implementation is a plain store reached by `remote`, which
PRV-001 and SEC-006 allow. Anything that can reach its URL can call it. The first description
contradicts itself, and the second says nothing of plain stores, or of a process on the engine's own
runner, which is reached over stdio with no proof.

Least change: say that a secret Resource's server that a runtime provider other than the engine's
own runner runs, or that is reached by `remote` and not plain, is reached only with the engine's
proof, and that a plain store reached by `remote` stands behind no gate.

## 5-1-12 — wrong — `declare-lost.v1` says the instances on a lost node can never confirm shutdown

Promises: PLC-017, PLC-027, LIF-024, LIF-025; core/operations.yaml (`declare-lost.v1`).

Quotes: `declare-lost.v1`: "Record that a node, and the instances placed on it, can never confirm
shutdown." PLC-017: "The shutdown of an instance for which no setup was ever dispatched is
confirmed". PLC-027: a further retirement with no hook to dispatch "succeeds once the stop has
succeeded, and the instance is then removed, under LIF-035".

Situation: an instance on a declared-lost node whose template selects no retire hook. A further
retirement asks for the stop, which succeeds, so its shutdown is confirmed and it is removed. The
description says that cannot happen.

Least change: "Retire a node and every instance placed on it, without their shutdown being
confirmed", as the definitions row "declare lost" says.

## 5-1-13 — wrong — DL17 says that replacing the template is enough

Promises: DL17, SEC-013, SEC-024, INS-020, NAM-009, PKG-005.

Quotes: DL17: "sends no such header for one registered before it was so configured, whose part
reports unavailable, under SEC-013, until the template is replaced; no other document needs to
change." INS-020: "Each of those instances keeps the definition and configuration it resolved when
it was made."

Situation: before the engine is made able to move, an instance is made from a template whose plain
item at `http://localhost` binds a secret header. Afterwards its setup's connection part reports
unavailable. Replacing the template changes nothing in the instance, which keeps its selected item,
so every further setup still reports unavailable. Only an instance made after the replacement
works, so the instance documents must change too.

Least change: "until the template is replaced and each instance made from it before is replaced by
a new one".

## 5-1-14 — wrong — the README's rotation bullet ignores SEC-023 and PRV-026

Promises: SEC-014, SEC-013, SEC-023, PRV-026, SEC-017; README ("What the guarantee for secrets covers").

Quotes: README: "A value rotated in its store reaches an instance's implementation only in a later
run, which a further setup asks for once the run has ended, and a `remote` item only at the next
setup (SEC-014, SEC-013)." SEC-023: "Before it sends an HTTP request on the connection, the engine
reads again". SEC-014's observable: "and so does the engine for a provider's run that ended with
the engine, under PRV-026".

Situation: a secret is rotated in its store, and then the engine restarts. Its next HTTP request on
a lasting `remote` connection carries the new value with no new setup (SEC-023), and a provider that
its own runner runs again gets the new value in a run that no setup asked for (PRV-026). The same
README states both reads two bullets later.

Least change: add "or, for a `remote` item, at the read again of SEC-023 once the engine has lost a
connection's values, and for a provider's run, in the run that PRV-026 makes again after a restart".

## 5-1-15 — wrong — the README says that SEC-019 binds an implementation of `get-secret.v1`

Promises: SEC-019, SEC-006; README ("What this is", the layers of the provider contracts).

Quotes: README: "what an implementation of `get-secret.v1` must do with a secret is `interface`, as
in SEC-006 and SEC-019." SEC-019: when "An implementation that carries Conduit's keys has been
given a value read under SEC-010, SEC-013 or SEC-023, as a variable, an argument or a header."

Situation: SEC-019 binds every implementation that receives a secret, such as the example's
`git-mirror-server`, and not the store. A reader who writes an ordinary server takes from the README
that SEC-019 is not theirs.

Least change: "what an implementation of `get-secret.v1` must do with a secret (SEC-006), and what
any implementation given one must not do with it (SEC-019), are `interface`".

## 5-1-16 — misleading — DL11 leaves to each engine what the access convention already decides

Promises: DL11, LIF-027, ACC-001, ACC-003, ACC-036, CON-013.

Quotes: DL11: "What a consumer with only some of its Conduits open may do." LIF-027: "What the
consumer of only some of its Conduits may do meanwhile is DL11". ACC-003: "It permits exactly the
operations it lists, on its target, and no others". ACC-036: "from the Conduits as they stand".

Situation: an Agent whose setup opened one declared Conduit and failed on the next. ACC-001, ACC-003
and ACC-036 already decide that requests through the open Conduit are permitted and every other is
refused. The only choice for DL11 that keeps every promise is to restate them, and an engine that
documents another choice there, such as holding requests until every Conduit is open, breaks
ACC-003.

Least change: remove DL11, and have LIF-027 say that each open Conduit permits what it lists
(ACC-003); or state what, if anything, is left to the engine.

## 5-1-17 — misleading — whether a restriction is "enforced" is said to be picked, and is never reported

Promises: ACC-027, PRV-006; README ("What this is"); definitions (enforced).

Quotes: README: "Whether a restriction is enforced is picked with the runtime provider (ACC-027)."
PRV-006: such a provider "is picked in the same way, and only it makes a restriction enforced".
ACC-027: "the inspection result states whether the restriction is enforced or only mediated ... In
this version every restriction is reported as mediated."

Situation: a user gives an Agent a runtime provider whose contract confines what it runs, as the
README and PRV-006 invite. Inspecting the Agent's Conduits still reports every restriction as only
mediated, which ACC-027's own definition makes false for that consumer. An engine that reports
"enforced" here keeps PRV-006's reading and breaks ACC-027.

Least change: README: "Whether a restriction can be enforced depends on the runtime provider; this
version reports every restriction as mediated (ACC-027)". In PRV-006: "only it could make a
restriction enforced".

## 5-1-18 — minor — LIF-029 repeats POOL-003 for an entry

Promises: LIF-029, POOL-003.

Quotes: LIF-029: when "An actor asks under LIF-009 for a new instance for a nested instantiation
while the instance made for it is not retired", then "conflict"; its observable: "This holds for a
declared Conduit as it does for a Pool's entry, which is POOL-003." POOL-003 states the same rule
for an entry.

Situation: two promises state one obligation for an entry's Resource, so a change to one can leave
the other behind.

Least change: limit LIF-029 to a declared Conduit, or drop POOL-003's rule and point to LIF-029.

## 5-1-19 — minor — SUB-008 provides for a manifest that MAN-015 requires

Promises: SUB-008, MAN-015.

Quotes: SUB-008: "`manifest`, the project's manifest as it was handed over ... which is left out
when the project has none." MAN-015 rejects a submission that holds a contract or lists an
implementation and "has no manifest or its manifest states no `address`".

Situation: SUB-008 applies only where a submission holds contracts or its manifest lists
implementations, and every such submission without a manifest is rejected. The clause is left over
from before ruling 41.

Least change: strike "which is left out when the project has none".

## 5-1-20 — minor — a hostname longer than 63 characters cannot be a label

Promises: PRV-014, PLC-026, PLC-007; `labels.v1`; `list-machines.v1`.

Quotes: PRV-014: "its `labels`, which hold `kubernetes.io/hostname`, whose value is the hostname".
`labels.v1`: a value is `maxLength: 63`. `list-machines.v1` limits `hostname` only to `minLength: 1`.

Situation: a machine whose canonical hostname has 70 characters, a long FQDN. No report can satisfy
both PRV-014 and the output schema, so the report fails validation and every machine counts as
omitted (PLC-023). Where the engine's own hostname is that long, PLC-007 cannot add `local`'s
hostname label to a selector.

Least change: state in PRV-014 what a provider reports for such a machine, such as a name that fits,
as its contract states, the way Kubernetes keeps the label to 63 characters.

## 5-1-21 — minor — "names a version assigns" is three names in one place and eleven in another

Promises: OBJ-024, PRV-022; definitions ("names a version assigns"); README (Contents); names.yaml.

Quotes: definitions: "The three names that a version of this specification states in names.yaml".
README Contents: "The names this version assigns: the core package's address, `apiVersion`, the
prefix of its `_meta` keys, and the names of what `identity` carries". names.yaml: "The names this
version of the specification assigns."

Situation: OBJ-024's rules, that only a new version changes them and that a document's `apiVersion`
says which version's names apply, are stated for the three. Whether they hold for `CONDUIT_URL` and
the files of `identity`, as on an engine that supports two versions, depends on which definition a
reader takes.

Least change: count the names of `identity` among the names a version assigns in definitions.md and
OBJ-024, or have the README and names.yaml name them apart.

## 5-1-22 — minor — the interface file's head comment misses IFC-044

Promises: IFC-044, IFC-014, IFC-043.

Quotes: interface/implementation.yaml: "Promises here have the layer `interface`, but IFC-014 and
IFC-043, rules on documents, have the layer `language`." IFC-044 also has the layer `language`.

Situation: a reader counting by layer from the head comment gets 39 interface promises in the file
where there are 38.

Least change: "IFC-014, IFC-043 and IFC-044".

## 5-1-23 — minor — DL4's subject names a case that cannot arise

Promises: DL4, ACC-037, MAN-012.

Quotes: DL4's subject: "Unreachable `access` or `management`". ACC-037, which points to DL4: "the
state of the Conduits that the decision needs cannot be established".

Situation: `access` and `management` are implemented by the engine itself (MAN-012), so they are
never unreachable apart from it. What DL4 now governs is ACC-037's case.

Least change: subject "Access decisions that cannot be made", and in the item "a request whose
access decision cannot be made under ACC-037".

## 5-1-24 — minor — the core README still leaves open which files of `identity` a run is given

Promises: PRV-007, PRV-022, DL13; core/README.md ("What this package does not settle"); `run.v1`.

Quotes: core README: "What the engine puts in `run.v1`'s `identity` beyond the names ...: which of
the files a run is given, under the forms of proof the engine offers". `run.v1` requires `ca.crt`,
`jwks.json`, `engine-id` and `token`, and PRV-022 says "The engine gives every file but `tls.crt` and
`tls.key` to every run".

Situation: the verification of ruling 43 closed and required the files, so only `tls.crt` and
`tls.key` on the engine's own runner are left to DL13.

Least change: "whether it gives `tls.crt` and `tls.key` to a run on its own runner".
