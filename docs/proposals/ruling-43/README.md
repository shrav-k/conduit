# Ruling 43: work across machines

Status: in force; amended by 58 (M13, M47, M66: later logged choices) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [IFC-041, IFC-042](../../../spec/interface/implementation.yaml), [PRV-022](../../../spec/promises/providers.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

Ruling 42 left D5 and D20 open and asked for more research on how Conduit works across
machines, looking ahead to what later versions need. The
[research packet](../../research/MULTI_MACHINE.md) and the
[multi-machine proposal](../multi-machine/README.md) answered it with D5, its second part, D20
and five smaller questions, Q1 to Q5. The owner asked for more explanation of each, and whether
any of it would add undue friction for systems on one machine or block the Model Context
Protocol. The coordinator explained each, and revised two recommendations in answer:

- D5's second part: option b in place of a, so that a local server reached at
  `http://localhost` with a secret header keeps working on an engine that cannot fail over.
- Q1: option a, tightened so that the engine's own address always accepts the token form, which
  is the protocol's own authorization, since many MCP clients cannot present a client
  certificate.

The coordinator also recommended keeping the name `identity`, which ruling 42's editors coined
for the input of `run.v1` that carries the engine's address and the credential files.

The owner's ruling: "Agree all, prepare for the codex audit".

## What follows

Every recommendation of the revised answer key is accepted:

- **D5, A.** Every server that the engine does not run itself sits behind a gate: the server of
  a run that any runtime provider other than the engine's own runner makes, and a server reached
  by `remote` that takes Conduit's keys. The gate admits a request only with proof that an
  enforcement point sent it, in a form that no receiver can replay to another, and nothing else
  on the server's machine reaches the server. The engine checks who answers before it sends. A
  provider's contract names its means, from existing tools. In v0 the engine is the only
  enforcement point. A server that is `plain` receives none of Conduit's keys and is reached as
  before.
- **D5, second part, b.** A header that carries a secret is sent only to an `https` URL, or to an
  `http` URL whose host is a loopback address on an engine that cannot fail over. An engine that
  can fail over rejects such an item when the document is registered.
- **D20, A.** A program shipped inside a package runs only on the engine's own runner in v0, as
  row 17 of the [open questions](../ruling-41/open-decisions.md) has it. A server meant for
  another machine is published as a `package` item pinned by its digest. In v1, fetching a
  package by digest is a feature that a runtime provider declares.
- **Q1, a, tightened.** Two standard forms of proof: mutual TLS with certificates in the form of
  an X509-SVID, and signed tokens in the form of a JWT-SVID, which toward a receiver at an
  `https` URL is the protocol's own authorization. The names of what `identity` carries are fixed
  once. Identifiers in SPIFFE's form are recommended and not required. The engine's own address
  always accepts the token form, so any MCP client that supports the protocol's authorization, or
  a configured `Authorization` header, can connect as an Agent from another machine.
- **Q2, a.** An engine that can fail over, or run as several parts, meets stated outcomes and
  chooses its own mechanism: every decision reads the state as it stands; at most one part acts on
  any one invocation, and the state store enforces it; the engine's address and signing keys
  survive a move; `local` keeps representing one hostname.
- **Q3, b.** After the engine restarts, a provider whose run ended with the engine is run again
  when the engine first needs it, with its secrets read for the caller of the setup that last ran
  it, decided by the Conduits as they stand then. This is a second exception to SEC-011, beside
  SEC-023.
- **Q4, a.** The shape of v1 is recorded in action item A65: an existing proxy on each machine
  enforces, Conduit's own decision point there decides from a copy of the Conduits held under a
  lease and refuses once the lease lapses, and the bound on a revocation is a value of the
  conformance document.
- **Q5, a.** `spec/README.md` says plainly that programs of the engine's own operating-system
  user are inside the engine's domain of trust and can get past every gate, and that an Agent not
  trusted that far runs under a runtime provider that isolates it.
- **The name `identity`** is kept.

"Prepare for the codex audit": the coordinator applies this ruling, runs the fifth round of
readers that ruling 41 ordered, fixes what they find, brings the repository's entry points up to
date for the next reader, and merges pull request 7.

## Earlier rulings this changes

| Ruling | What it said | Now |
| --- | --- | --- |
| 40 (point 3e) | A request that crosses machines carries proof of its sender. | It also binds what the engine sends to a server it does not run itself, and the server checks it (D5). |
| 41 (decision 4c), as ruling 42 amended it | Every read of a secret belongs to a request in progress, or is SEC-023's read again for a connection. | Also the read before a provider's run made again after a restart (Q3). |
| 42 | D5 and D20 wait on research. | Ruled here. |

## After the ruling

The coordinator's decisions on what the logs left open, under rulings 37 and 38, each as the
log recommends:

- A secret Resource on the engine's own runner is run again after a restart as a provider is
  (PRV-026), so that a single machine needs no setup by hand after a restart.
- Every run gets the trust bundle and `engine-id` (N1). The guard at send time alone keeps a
  secret header off `http` once the engine is able to move (N2). On an engine of several parts
  the engine's own credential is short-lived, its lifetime in the conformance document (N3,
  DL13). An engine address that a provider gives as the run sees it keeps the host of the
  engine's address (N4, PRV-022).
- Kept as written: IFC-043, under which an item that is not `plain` may not name
  `Authorization`, since the engine's token takes that header; "not from the run it came from"
  in IFC-022, beside DL13's identification over stdio by the channel the engine itself made;
  SEC-024's rejection of `http` to any host that is not a loopback address; MAN-018 rejecting
  the template; the IDs PRV-027, PRV-028, IFC-043 and IFC-044 that the editors added.

Accepted by [ruling 58 (point 1)](../ruling-58/README.md): these coined names and the logged editor choices.
Coined names, for the owner to strike: the variables `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR`;
the files `ca.crt`, `jwks.json`, `engine-id`, `token`, `tls.crt` and `tls.key`; the terms gate,
enforcement point, decision point and the engine's ID. `spec/names.yaml` holds the names of
`identity`, and `tools/set_names.py` changes them together.

## Choices that set behavior

Copied from the editor logs before their removal under ruling 58. These are the choices at
the time of each pass, including readings and alternatives; later rulings and the specification
govern. The source labels retain the log and its choice number.

### log-core

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-core.md). 10 choices.

- log-core, 1: **`identity` is required in `run.v1`, and closed.** PRV-007 and DL13 give every run `identity`, and PRV-022 lists its names "and no other", so the schema requires it and admits no other variable. The keys of `files` stay paths, as PRV-022 has them, so the schema cannot list the file names; `check_schemas.py` checks that the description names each.
- log-core, 2: **`url`, not `endpoint`,** for the new output of `inspect-run.v1`: N5 of ruling 42 renamed `run.v1`'s `endpoint` to `url`, and the providers and interface groups use `url` too.
- log-core, 3: **`machine` is required in `stop.v1` and `inspect-run.v1`,** in the same form as in `run.v1`, as PRV-027 has the engine always give it.
- log-core, 4: **IFC-043 in the schema.** IFC-043, the interface group's, rejects an item "wherever it is read, whichever engine reads it", so `implementation-item.v1` can state it, in its `else` branch for an item that is not `plain`, with a pattern that matches `Authorization` in any case. It is not among the items assigned to these files; if IFC-043 is struck, the schema's `propertyNames` under `else`, the example and the README rows go with it.
- log-core, 5: **MAN-018 and SEC-024 as rules on the template, not the manifest.** Each promise rejects the template, and an item that binds `CONDUIT_URL`, or a `remote` item at an `http` URL with a header bound to a secret, is valid alone, so neither is in `implementation-item.v1`, which would have rejected a package that no template uses that way. The rules look at the items that the tool has, as MAN-017's does.
- log-core, 6: **SEC-024's static part.** The rule rejects an `http` URL whose host is not `localhost`, an IPv4 address in `127.0.0.0/8` or the IPv6 address `::1`, compared without regard to case, as SEC-024 states; `127.1` and `foo.localhost` are not loopback hosts there. It does not look at a host written as a variable of the Remote (`{host}`), nor through a `$ref` to the place, and it does not reject a loopback host, which only an engine that can restart on another machine or run as several parts rejects.
- log-core, 7: **A by-rule set may hold its project's manifest.** The two rules need an item that no valid example has. Adding one to the valid examples would have shown an item that no template of theirs could use, so `check_schemas.py` now reads a document with no `kind` in a set under `examples/invalid-by-rule/` as the set's manifest, validates it against `manifest.v1`, and gives its items to the rules as the set's own, reached by `#name`.
- log-core, 8: **The example project's `workspace` loses its `runtime` input** rather than gaining an entry whose item is a pinned `package`: no item of the examples is a `package` that a Pool entry's template could select without more contracts, and D20's answer is that such an entry runs on the engine's own runner. The project no longer shows an input whose destination is `/entries/<name>/runtime`, which `pool-template-spec.v1` still allows.
- log-core, 9: **`spec/names.yaml` holds the names of `identity`,** and `set_names.py` changes them. A file name such as `token` is also a common word and Kubernetes' own file's name, so the tool counts and rewrites a file name only in backticks, never under `spec/schemas/examples/`, whose example project has an input `token`, and shows every line in which a file name would change before `--write`. The sentences of MAN-018 and PRV-022 that cite Kubernetes' own `token` and `ca.crt` would be rewritten too, and must be put back by hand after such a change.
- log-core, 10: **The description of `secret-provider.v1` says a store that cannot check the engine's proof is marked `plain` and sent no locator.** That follows IFC-041 and PRV-025, and means such a store learns its locator from no request: it can serve one fixed secret at most.

### log-docs

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-docs.md). 5 choices.

- log-docs, 1: **The D5 paragraph after the list goes.** The integration kept a paragraph that said D5 was a gap of this section until ruling 43. Section 9 says that the gap goes; the status paragraph of `spec/README.md` already says that ruling 43 ruled D5, and the substance is now a point of the list, which states limits and what holds rather than history.
- log-docs, 2: **A row for credential.** The promises give the word a precise sense (named by ID, as long as the instance, issued from one root), and use it in its ordinary sense for a fetch and a store; the row states the first and keeps the second.
- log-docs, 3: **`docs/OVERVIEW.md` does not write the names of `identity`.** `tools/set_names.py --write` rewrites them only under `spec/`, so a change of a coined name would leave the overview wrong. It links `spec/names.yaml` instead.
- log-docs, 4: **`docs/OVERVIEW.md` mentions a later version only by a link** to Q4 of the proposal, and says that none of it is part of this version, since ruling 43 accepted Q4 as a record for A65 and not as text.
- log-docs, 5: **`spec/schema-profile.md` repeats SEC-024's three loopback hosts**, which are the secrets group's choice 3, so that a reader of the profile sees which URLs a template may bind a secret header to. If that choice is struck, the row changes with SEC-024.

### log-integration

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md). 20 choices.

The [verification](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md#verification) reversed choice 3 (IFC-042 now names the engine's address at
an `https` URL, whichever machine a request comes from), choice 4 (PRV-026 now covers a secret
Resource) and the second half of choice 5 (DL17 now says what becomes of a template registered
before the engine was configured to move).


#### Choices of this pass

- log-integration, 1: **Decision point is defined although no promise uses it**, since section 9 asks for both terms and the row of enforcement point needs it.
- log-integration, 2: **IFC-017 carries point 4 of the gate in substance.** No group had written "between the engine and the side that checks the proof, the request and its result stay confidential and unaltered", and the secrets group's log relied on it.
- log-integration, 3: **IFC-042's scope governs.** IFC-022 and DL13 said that the engine's address accepts the token form always; IFC-042, as assigned, says from a consumer on another machine, and its observable leaves the engine's own machine to DL13. This pass made the three agree with IFC-042.
- log-integration, 4: **PRV-026 is left as written** (the three providers that provider calls reach), and DL16 and the core README were narrowed to match it, pending [item 1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md#needs-a-decision).
- log-integration, 5: **DL17 keeps SEC-024's rejection first**: "no other document needs to change". What becomes of a template registered before the engine is made able to move is [item 2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md#needs-a-decision).
- log-integration, 6: **`spec/README.md` states what ruling 43 leaves outside the guarantee for secrets**: a secret header over `http` on loopback, where an engine cannot move, reaches whatever listens there, a process of another user among them; and a gate admits root and the provider's own user or group on the run's machine.
- log-integration, 7: **The citation of Kubernetes in PRV-022 and in `spec/names.yaml`** no longer writes the file names, which settles the core group's open item 3. The groups' own choices beyond the record were checked against each other and against the ruling, and none contradicts another: the providers group's choices 1 to 9, the interface group's 1 to 9, the secrets group's 1 to 10 and the core group's 1 to 10, each in its log. Each still stands unless the coordinator or the owner strikes it; the ones that change behavior are in the next section.

#### Choices of the verification

- log-integration, 1: **The certificate is the one proof of who answers at a run's `url`** (PRV-028). The proposal's "or by being the provider's channel" stays as the way the request goes on from the provider's end, which presents the certificate.
- log-integration, 2: **IFC-044** rejects a `remote` item that is not plain at an `http` URL.
- log-integration, 3: **Every file of `identity` but `tls.crt` and `tls.key` goes to every run**, `token` and the trust bundle among them, and the schema requires them. See [N1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md#still-needs-a-decision).
- log-integration, 4: **Two files added to `identity`**, `jwks.json` and `engine-id`, under coined names.
- log-integration, 5: **The engine's address may present a certificate from other roots**, such as a public certification authority's (F8).
- log-integration, 6: **No run runs as the user or group that a gate excepts** (MX-02).
- log-integration, 7: **MAN-018 compares names without regard to case** (C9).
- log-integration, 8: **The engine authenticates to an authorization server that the user runs with its own credential** of DL13, by RFC 7523 or RFC 8705 (F6, MX-13). Changed in round 5 (M66, dated 2026-11-07): IFC-041 drops RFC 8705, since the protocol's extension for client credentials does not allow mutual TLS, so the engine authenticates by a JWT assertion under RFC 7523 alone, and DL13 states only its key and client identifier.
- log-integration, 9: **DL17's consequences**: one machine for the runner at a time, one part for each server of a provider and each connection, the machine left shut down before the runner runs elsewhere, and the invocations of a part that stops ended unknown (MX-04, MX-09, MX-10).
- log-integration, 10: **IFC-042 names the address at an `https` URL** (C8, MX-08). Changed in round 5 (M53, dated 2026-11-07): IFC-042 also accepts a run's `token` at the `http` loopback address that DL13 gives a run, under the condition that DL13 states (M47).
- log-integration, 11: **SEC-013's guard** (C1, MX-05): an engine configured to move sends a secret header to no `http` URL, whenever the template was registered.
- log-integration, 12: **A provider's own secret Resources belong on `local`**, stated as guidance (MX-11).
- log-integration, 13: Added in round 5 (M109), which found it missing here: **PRV-022 no longer gives leave to replace a file of `identity` in place** (MX-21). In this version a credential lasts as long as its instance (DL13), as the proposal has it, so the leave that ruling 42's D17 gave is left, in PRV-022's observable, for renewal on the run's machine in v1 (A65).

### log-interface

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-interface.md). 9 choices.

- log-interface, 1: **IFC-043, the `Authorization` header.** Under the token form the engine's proof toward a `remote` item is the protocol's authorization, in that header, so an item that also names it in `headers` collides. The proposal left the choice between "must use mutual TLS" and "may not name that header" to the writer. This takes the second, as a rule on documents, because it holds whichever engine reads the item: whether the engine offers mutual TLS is DL13's choice, so the first would make a document work on one engine and not another. The owner may strike it for mutual TLS, or for the engine's token taking the header's place.
- log-interface, 2: **A `remote` item that is not `plain` at an `http` URL** is not connected to, and the part reports unavailable, as the proposal's engine side has it. It is not rejected when the document is registered, as SEC-024 rejects a secret header to `http`; the manifest's rules are not this group's. See [item 1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-interface.md#needs-a-decision).
- log-interface, 3: **IFC-041's values as the configuration stood when the connection was made.** This follows from the row connection and from X11's recommendation, and states what "a restart changes nothing a connection carries" needs.
- log-interface, 4: **IFC-022's exception is worded for any runtime provider**: a program that can read another run's credential, as root and the run's own operating-system user can. Q5 names the engine's own user; X1 is about runs that share any user, so the wording covers both.
- log-interface, 5: **IFC-042's `when`** is a consumer on another machine, as the coordinator's assignment states; its `then` holds "whatever other form of proof DL13 offers", which is the ruling's "always". On the engine's own machine DL13 may establish an instance by other means.
- log-interface, 6: **IFC-042's observable** says that the engine's address, as reached from another machine, is an `https` URL with the protected resource metadata of RFC 9728, naming the authorization server that DL13 states: Q1 makes the token's audience the receiver's canonical `https` URI, and the protocol's authorization requires the metadata.
- log-interface, 7: **IFC-030 and IFC-032** gained a sentence each in their observables, as the table says.
- log-interface, 8: **IFC-017's "`server/discover` among it"**: the discovery that IFC-037 has the engine read first is also held back until the server's side has proved who it is.
- log-interface, 9: **DL16** points to PRV-026 for the servers of providers, and repeats Q5's consequence for a process of the engine's own user.

### log-providers

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-providers.md). 9 choices.

- log-providers, 1: **A directory, named by a variable.** Section 1.4 cites Kubernetes, which fixes absolute paths inside a container. The engine's own runner starts plain processes that share one machine, so fixed absolute paths would collide; a variable names a directory per run, and the files inside it have fixed names. This makes two variables where Q1 names one.
- log-providers, 2: **`url`, not `endpoint`.** The proposal names the new optional output of `inspect-run.v1` `endpoint`, the old name of `run.v1`'s output, which N5 of ruling 42 renamed to `url`. The core group and the interface group use `url` too.
- log-providers, 3: **A new ID for the machine of a stop or an inspection (PRV-027).** No promise describes the call to `inspect-run.v1`, so one promise covers both calls, and PRV-008's `then` points to it.
- log-providers, 4: **PRV-025 binds the server's side; the engine's side stays in IFC-017 and IFC-041.** Point 4 of section 3 is the engine's obligation, so it was left to those promises, one obligation per promise. PRV-025's `when` is a run by any runtime provider other than the engine's own runner, whether or not it has returned its `url`, so that the gate stands from the start.
- log-providers, 5: **`identity` wins over the list set on the engine's own runner** (PRV-023). MAN-018 rejects a template whose item binds `CONDUIT_URL` or `CONDUIT_IDENTITY_DIR`, but the list names variables of the engine's environment, which no template governs. Where the list names one of them, the run gets the value of `identity`, as ruling 42's integration recommended for the item case (its option a).
- log-providers, 6: **The names of `identity` are a closed list** (PRV-022: "under these names and no other"), so that MAN-018's check is exact. `token` is present where DL13's mechanism uses the token form, and `tls.crt` and `tls.key` where it uses mutual TLS or the run's server stands behind a gate, as DL13 now says.
- log-providers, 7: **"Restarts on another machine, or runs as several parts"** in place of "fails over", which `spec/definitions.md` lists among the words the promises do not use. DL17 and SEC-024 use the same words.
- log-providers, 8: **PRV-026 covers only the three provider capabilities that the engine reaches by provider calls**, as Q3 frames it, not a secret Resource, which is placed and run as any Resource is. See [open item 1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-providers.md#needs-a-decision).
- log-providers, 9: **PRV-011's channel** binds only what a provider carries to a machine other than the one its own server runs on, which is `local` under PRV-002; on one machine the provider's own means serve.

### log-secrets

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-secrets.md). 10 choices.

- log-secrets, 1: **SEC-024 rejects every other `http` URL at registration.** The ruling says that a secret header "is sent only to" `https` or an `http` loopback URL on an engine that cannot fail over, and that an engine that can fail over rejects the loopback item at registration. It does not say what becomes of an `http` URL to any other host on an engine that cannot fail over. Here it is rejected at registration too: the URL and whether the place is a secret's are known then, as SEC-021's check is, and a setup could never connect. The other reading is to accept the template and have the connection's part end failed at each setup.
- log-secrets, 2: **"Can fail over" read as DL17 reads it**: an engine that, as its conformance document states under DL17, can restart on another machine or run as several parts. `spec/definitions.md` lists "fail over" among the words the promises do not use, and DL17 and IFC-041 use the same words, as the providers group's choice 7 does.
- log-secrets, 3: **The loopback hosts.** `localhost`, an IPv4 address in `127.0.0.0/8`, and `[::1]`, compared without regard to case, as RFC 3986 compares a scheme and a host; no other name, even one that resolves to a loopback address. The Model Context Protocol's security best practices name `localhost`, `127.0.0.1` and `::1`; the whole of `127.0.0.0/8` is the IPv4 loopback block, and the ruling names `http://localhost`. IFC-041 says "a loopback address" and cites SEC-024 for it.
- log-secrets, 4: **SEC-024 is of the layer `language`**, although whether a template is rejected depends on the engine's statement under DL17, as the ruling has it ("An engine that can fail over rejects such an item when the document is registered").
- log-secrets, 5: **MAN-018 rejects the template**, as the coordinator's settlement of X3, the integration log's option (c) and PRV-022's observable put it, and not the item when its manifest is read. Both reject the same documents in the end; the template's check is made where MAN-017's and SEC-021's are.
- log-secrets, 6: **MAN-018 covers `environment` alone**, since `identity` carries variables and files, and an item binds no file; an argument or a header is not covered.
- log-secrets, 7: **SEC-023 keeps which secret Resource the setup read for each header**, as state that holds no value, as X11's recommendation has it. A control that has changed the place since is read at a later setup's connection, as it is where the engine still holds the values.
- log-secrets, 8: **ACC-027's observable says why a restriction behind a gate is still only mediated**: the gate keeps other callers from a target's server, but a consumer that no runtime provider confines may still reach by its own means what the target's implementation reaches. Section 8 of the proposal leaves "enforced" to v1, where a provider confines a run.
- log-secrets, 9: **SEC-006's observable names `headers`** among what a plain implementation learns its locator from, as PKG-020 already does, for a plain store reached by `remote`.
- log-secrets, 10: **SEC-012, SEC-014, OBS-004 and REC-002** each gained a sentence for PRV-026, which the proposal's section 9 does not list, so that each says of the read for a provider's run what it already said of SEC-023's read, or, for OBS-004, why running a provider again is not acting on a difference.
