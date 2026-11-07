# Ruling 43: log of the interface group

Date: 2026-11-06. This is the account of the editor who applied [ruling 43](README.md) to two
files: the [implementation interface](../../../spec/interface/implementation.yaml) and the
[implementation-defined list](../../../spec/implementation-defined.yaml). The ruling is the
owner's. The choices listed under [Choices beyond the record](#choices-beyond-the-record) are
this editor's, and each stands unless the coordinator or the owner strikes it.

What was applied: the ruling's acceptance of the
[proposal on work across machines](../multi-machine/README.md), with D5's second part as option
b and Q1 tightened, as the proposal's
[section 9, "In v0"](../multi-machine/README.md#9-the-promises-that-change) lists the changes for
these files (IFC-017, IFC-041, IFC-022, DL13 and a new item for Q2), with the detail of sections
[1.4](../multi-machine/README.md#14-one-way-to-prove-who-is-who),
[1.5](../multi-machine/README.md#15-the-engine-across-a-failover),
[3](../multi-machine/README.md#3-d5-who-may-reach-a-server-on-another-machine) and
[5](../multi-machine/README.md#5-the-smaller-decisions); IFC-042, which Q1 as the ruling
tightened it asks for; and the items of the
[integration log of ruling 42](../ruling-42/log-integration.md) that waited on this ruling and
touch these files. Not applied: the section
[In v1, for A65](../multi-machine/README.md#in-v1-for-a65), which is for the tracker. No record
was edited but this log, and nothing was committed.

## Checks

Run at the end of this pass, with every group's files as they stood then; other groups were
still writing, so a later run may differ.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `447 promises ok` |
| Every text field of the two files folded with `>-`; a line over 96 characters in them | Yes; none |
| An ID or a DL item cited in the two files that does not exist | None. PRV-025, PRV-026 and SEC-024 were written by their groups before the last run |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0`. The tool now also checks the names of what `identity` carries, which `spec/names.yaml` holds; these two files write none of them |
| `python3 tools/check_schemas.py` | `Errors: 0`; 24 rules from 23 promises, 26 sets under `examples/invalid-by-rule/` each rejected by its rule. An earlier run, while the providers group was writing, gave one error not in these files: `spec/promises/manifest.yaml` cited PRV-025 before it existed |
| `git diff --check` | Clean |

## Counts

In `spec/interface/implementation.yaml`, 5 promises changed and 2 added; none removed and none
moved. The file now holds 40 promises, 38 of the layer `interface` and 2, IFC-014 and IFC-043,
of the layer `language`. In `spec/implementation-defined.yaml`, 3 items changed and 1 added; the
list now holds 16 items. DL14 is not reused.

| File | Changed | Added |
| --- | --- | --- |
| `spec/interface/implementation.yaml` | IFC-017, IFC-022, IFC-030, IFC-032, IFC-041 | IFC-042, IFC-043 |
| `spec/implementation-defined.yaml` | DL5, DL13, DL16 | DL17 |

Each file has a paragraph in its head comment naming ruling 43, what changed and this log. The
sentence of the interface file's head comment on layers now names IFC-043 beside IFC-014.

## New IDs

| ID | Assigned | What it says |
| --- | --- | --- |
| IFC-042 | In advance, by the coordinator | A consumer on another machine may present, at the engine's address, a signed token in the form of a JWT-SVID as the protocol's own authorization, whose audience is the engine's address as its canonical `https` URI, whatever other form of proof DL13 offers. |
| DL17 | In advance, by the coordinator | Whether the engine can restart on another machine or run as several parts, and, where it can, how it meets Q2's outcomes. |
| IFC-043 | By this group, in its own file | An implementation item reached by `remote` that is not `plain` and names `Authorization` in its `headers` is rejected. See [choice 1](#choices-beyond-the-record). |

## What changed

| What | IDs | Why | Proposal |
| --- | --- | --- | --- |
| The engine reaches a run at the `url` that its runtime provider last reported, from `run.v1` or from the optional `url` of `inspect-run.v1` | IFC-017 (`then`) | The proposal's "latest endpoint reported". The output is named `url`, as `run.v1`'s is since ruling 42, and as PRV-009 of the providers group names it | 3, "And the engine's side"; 9, rows IFC-017 and `inspect-run.v1` |
| To a run's `url`, and to a `remote` item that is not `plain`, the engine sends nothing, `server/discover` among it, until the server's side has proved who it is under PRV-025, and it presents on every HTTP request the proof of an enforcement point, by DL13's mechanism | IFC-017 (`then`) | D5, A: the engine checks who answers before it sends, and only an enforcement point reaches a gated server | 3, "The gate, in substance", item 4, and "And the engine's side"; 9, row IFC-017 |
| How the server's side proves itself toward a run and toward a `remote` item; a token's audience is the instance's ID, never the `url`; where the engine cannot verify who answers, nothing is dispatched, the part or the request reports unavailable and its record says why; the engine is the only enforcement point in this version; the engine's own runner, over stdio, has no gate | IFC-017 (observable) | As above | 3, "The gate, in substance" and "And the engine's side"; 8, the row for the engine's own runner |
| An instance is established not from the address, machine, network, runtime provider or run that a request came from | IFC-022 (`then`) | Q1's last point, so that mutual TLS, which proves the sender once for each connection, still counts | 5, Q1; 9, row IFC-022 |
| What a request presents names an instance by its ID, and none once that instance is removed, even where another instance takes its name | IFC-022 (`then`) | "Named by ID" | 1.4; 9, row IFC-022 |
| The exception to "no consumer can present as another", stated plainly: a program that can read the credential of another instance's run, as root and every program of the operating-system user that the run runs as can | IFC-022 (`then`) | X1, settled under Q5: runs under one operating-system user can read each other's credential | 5, Q5; X1 of the [verification of ruling 42](../ruling-42/log-integration.md#each-finding) |
| The two forms of proof each count; under mutual TLS the certificate of the TLS connection is part of what each request on it presents; a run on the engine's own runner runs as the engine's user unless DL16 says otherwise, and a program of that user is inside the domain of trust, can present as any run under that user and can make the engine's proof to every gate; an instance not trusted that far is given a runtime provider that runs it apart | IFC-022 (observable) | Q1 and Q5 | 5, Q1 and Q5; 9, rows IFC-022 and `spec/README.md` |
| A run is given the engine's address and what it presents under the names that PRV-022 fixes | IFC-022 (observable) | Q1: the names of what `identity` carries are fixed once | 5, Q1 |
| The proof the engine presents to an implementation's gate is its own, as an enforcement point, and never what a consumer presented | IFC-030 (observable) | So that the gate does not read as a breach of IFC-030's rule on what the transport carries | 3, "What does not count" |
| A credential that names a removed instance establishes none, so a run left on a machine after its instance was removed reaches nothing through the engine | IFC-032 (observable) | Revocation is the engine's check against live state on every request | 1.4; 2, S7 |
| The headers of a connection take their values from the configuration as it stood when the connection was made | IFC-041 (`then`) | The connection is the engine's holding of those values (the row connection); and X11's recommendation, the setup's, applies to headers that carry no secret as item 6 of the ruling 42 log asked | X11; item 6 of [Still needs a decision](../ruling-42/log-integration.md#still-needs-a-decision) |
| To a `remote` item that is not `plain`, the engine connects only at an `https` URL, and every HTTP request carries the proof of an enforcement point, by mutual TLS or as the protocol's own authorization with the item's canonical URI as audience | IFC-041 (`then`) | D5, A, for a `remote` server that takes Conduit's keys | 3, "And the engine's side"; 9, row IFC-041 |
| A header that carries a secret goes only where SEC-024 lets it go: to `https`, or to an `http` loopback host on an engine that can neither restart on another machine nor run as several parts (DL17) | IFC-041 (observable) | D5's second part, option b, as the ruling revised it; SEC-024 is the secrets group's | 3, "D5, second part"; the ruling |
| After the engine has lost a connection's values, it reads the secrets again by the references that the setup which made the connection read, and sends every other header with the value that setup took: a restart changes nothing a connection carries | IFC-041 (observable) | X11, as settled; SEC-023 is the secrets group's | X11 |
| A `remote` item that is not `plain` is its own gate: its operator configures it to trust DL13's issuer; under the protocol's authorization its protected resource metadata names that issuer, or one the user runs that issues tokens to the engine under the client-credentials extension; one that cannot be so configured is used as `plain`; one at an `http` URL is not connected to, and the part reports unavailable | IFC-041 (observable) | D5, A | 2, S2; 3, "And the engine's side" and its costs |
| New: the engine's address accepts the token form from a consumer on another machine | IFC-042 | Q1 as the ruling tightened it: any client that supports the protocol's authorization, or a configured `Authorization` header, can connect as an Agent from another machine | The ruling, Q1; 5, Q1 |
| New: a `remote` item that is not `plain` names no `Authorization` header | IFC-043 | The proposal left to the writer of the promise whether such an item must use mutual TLS or may not name that header | 3, "And the engine's side", last sentence |
| The engine keeps its hostname across a move to another machine | DL5 | X7, settled under Q2 | 5, Q2 |
| DL13 rewritten: the proof of who is who in both directions under one root of trust; the two forms of proof, the token form always accepted at the engine's address; IDs by the instance's ID, SPIFFE's form recommended and not required; where the signing keys are held, which brings their holder into the domain of trust; where checking keys are published; the authorization server named at the engine's address; what goes in `identity` under the names PRV-022 fixes; a run's credential and its gate's certificate last as long as the instance, none renewed during a run; each root kept while a run that holds it lasts; the engine's own credential may be short-lived. Its subject is now "Proof of who is who" | DL13 | Q1, and X9, settled: in v0 a credential lasts as long as its instance and renewal is v1's, so the earlier "whether and when it is refreshed in place" and how a provider gets new content go | 1.4; 5, Q1 and Q2; 9, row DL13 |
| The servers of providers are run again when first needed, under PRV-026; a process of the engine's own operating-system user is inside the domain of trust and can get past every gate | DL16 | Q3 and Q5 | 5, Q3 and Q5 |
| New: the outcomes that an engine meets where it can restart on another machine or run as several parts: decisions read the state as it stands; at most one part acts on an invocation, as the store of the engine's own state enforces; the address in `identity` and the signing keys survive a move, and the stops a runtime provider owes (PRV-021); `local` keeps representing one hostname; such an engine rejects a secret header to `http`, under SEC-024 | DL17 | Q2, and X7 | 1.5; 2, S8; 5, Q2; 9, row "New item of the implementation-defined list" |

## Items of the ruling 42 log settled here

| Item | Settled as | Where |
| --- | --- | --- |
| X1, item 2 of [Still needs a decision](../ruling-42/log-integration.md#still-needs-a-decision) | Q5: the exception is stated plainly | IFC-022 (`then`, observable) and DL16 here; `spec/README.md` is the README group's |
| X3, item 1 of the same list | The names are fixed under Q1, so a template whose item binds one is rejected (MAN-018, the manifest group's) | Not in these files: IFC-022 and DL13 cite PRV-022 for the names |
| X7, item 4 of the same list | The engine keeps its hostname across a move, under Q2 | DL5, DL17 |
| X9 | In v0 a credential lasts as long as its instance; renewal is v1's | DL13 |
| X10, item 5 of the same list | `stop.v1` and `inspect-run.v1` take the machine | Not in these files (the providers and core groups) |
| X11, item 6 of the same list | SEC-023 reads the references that the setup read (the secrets group's); the headers that carry no secret keep the values the setup took | IFC-041 |
| Item 5 of [Needs a decision](../ruling-42/log-integration.md#needs-a-decision), a stop held when a provider is retired | The stops survive the provider's retirement and a move of the engine (PRV-020, PRV-021, the providers group's) | DL17 cites PRV-021 for a move |

## Coined names

- **The names of what `identity` carries** are not written in these two files. IFC-022, IFC-042
  and DL13 cite PRV-022, which fixes them, and `spec/names.yaml` holds them, so that one place
  changes them. This editor chose, independently and by the same precedents, the names that
  PRV-022 and `spec/names.yaml` hold when this log was written: `CONDUIT_URL` for the engine's
  address; `CONDUIT_IDENTITY_DIR` for the directory, as Nomad's `NOMAD_SECRETS_DIR` names a
  task's ([research, 1.2](../../research/MULTI_MACHINE.md#12-nomad)), since runs that share a
  machine cannot share Kubernetes' one fixed path; and the files `ca.crt` and `token`, as
  Kubernetes names them, and `tls.crt` and `tls.key`, as cert-manager's csi-driver-spiffe writes
  an SVID ([research, 1.4](../../research/MULTI_MACHINE.md#14-spiffe-and-spire)). An earlier
  draft of `spec/names.yaml` held `CONDUIT_ENGINE_URL` and spiffe-helper's file names; it now
  agrees with PRV-022.
- **DL13's subject**, "Proof of who is who", from the heading of the proposal's section 1.4; it
  was "Identifying consumers", and DL13 now also covers the engine's proof to a gate.
- **DL17's subject**, "An engine that restarts on another machine or runs as several parts".
  The ruling says "an engine that can fail over", and
  [definitions.md](../../../spec/definitions.md#words-the-promises-do-not-use) lists "fail
  over" as a word the promises do not use, so these files say what it means, as SEC-024 does.
- **Terms used and not coined here**: enforcement point and gate, from the ruling and the
  proposal's sections 1.2 and 1.3, for the definitions group to define. "Root of trust", in
  DL13, is the proposal's (1.4). "The token form" is not used as a term: the text says "a signed
  token in the form of a JWT-SVID".

## Choices beyond the record

1. **IFC-043, the `Authorization` header.** Under the token form the engine's proof toward a
   `remote` item is the protocol's authorization, in that header, so an item that also names it
   in `headers` collides. The proposal left the choice between "must use mutual TLS" and "may
   not name that header" to the writer. This takes the second, as a rule on documents, because
   it holds whichever engine reads the item: whether the engine offers mutual TLS is DL13's
   choice, so the first would make a document work on one engine and not another. The owner may
   strike it for mutual TLS, or for the engine's token taking the header's place.
2. **A `remote` item that is not `plain` at an `http` URL** is not connected to, and the part
   reports unavailable, as the proposal's engine side has it. It is not rejected when the
   document is registered, as SEC-024 rejects a secret header to `http`; the manifest's rules
   are not this group's. See [item 1](#needs-a-decision).
3. **IFC-041's values as the configuration stood when the connection was made.** This follows
   from the row connection and from X11's recommendation, and states what "a restart changes
   nothing a connection carries" needs.
4. **IFC-022's exception is worded for any runtime provider**: a program that can read another
   run's credential, as root and the run's own operating-system user can. Q5 names the engine's
   own user; X1 is about runs that share any user, so the wording covers both.
5. **IFC-042's `when`** is a consumer on another machine, as the coordinator's assignment states;
   its `then` holds "whatever other form of proof DL13 offers", which is the ruling's "always". On
   the engine's own machine DL13 may establish an instance by other means.
6. **IFC-042's observable** says that the engine's address, as reached from another machine, is
   an `https` URL with the protected resource metadata of RFC 9728, naming the authorization
   server that DL13 states: Q1 makes the token's audience the receiver's canonical `https` URI,
   and the protocol's authorization requires the metadata.
7. **IFC-030 and IFC-032** gained a sentence each in their observables, as the table says.
8. **IFC-017's "`server/discover` among it"**: the discovery that IFC-037 has the engine read
   first is also held back until the server's side has proved who it is.
9. **DL16** points to PRV-026 for the servers of providers, and repeats Q5's consequence for a
   process of the engine's own user.

## Needs a decision

1. **A local server that takes Conduit's keys at `http://localhost`.** The ruling's revision b
   keeps a local server with a secret header working over `http` on an engine that cannot move,
   but under D5, A, an item that is not `plain` is reached only over `https`, so a local server
   that takes Conduit's keys at `http://localhost` no longer connects, whatever the engine.
   Options: (a) as written: such a server runs as a `command` or `package` item on the engine's
   own runner, over stdio, or is used as `plain`; (b) extend b's loopback exception to Conduit's
   keys on an engine that can neither restart on another machine nor run as several parts.
   Recommendation: (a), since loopback admits every process on the machine, which the proposal's
   "What does not count" rejects as a gate. Under (b), whether such an item is then also rejected
   at registration on an engine that can move is the manifest group's to write.
2. **IFC-043 or mutual TLS** ([choice 1](#choices-beyond-the-record)), for the owner to confirm
   or strike.
3. **"Not from the run it came from" on one machine.** Item 2 of the ruling 42 log offered, as
   ways to bind a proof to a process on one machine, a socket for each run or a check of the
   peer's operating-system credentials. IFC-022 now says that an instance is not established from
   the run a request came from, as Q1 words it; read strictly, such a check is excluded. Under
   Q5 nothing requires it, so the text is applied as worded; the coordinator may confirm.

## For other jobs

- `spec/README.md`: the row for `interface/implementation.yaml` says 38 promises, now 40; the row
  for `implementation-defined.yaml` says 15 items, now 16. The paragraph that says the mechanism
  that proves which instance sent a request "stays each engine's choice for now" now has DL13's
  fixed forms and PRV-022's fixed names beside it. X1's statement is the README group's.
- `spec/definitions.md`: rows for enforcement point and gate; the row `identity`, whose names are
  now fixed (PRV-022); DL13's new subject where a row names it; the row "fail over, failover",
  which these files keep to.
- `spec/core/README.md`, the list of what is left open: "What the engine puts in `run.v1`'s
  `identity`: the name of the variable ... DL13" is now PRV-022's, apart from which files a
  run is given and their content (the core group's file).
