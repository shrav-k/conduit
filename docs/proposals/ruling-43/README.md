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

The ruling was applied to `spec/` on 2026-11-06 and 2026-11-07. The logs are beside this record:
[providers](log-providers.md), [interface](log-interface.md), [secrets](log-secrets.md),
[core](log-core.md), [docs](log-docs.md) and the [integration](log-integration.md), whose
section "Verification" answers 43 findings of three verifiers: coverage of the ruling, cost on
one machine and for the Model Context Protocol, and scenarios across machines. The
specification then held 449 promises.

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

Coined names, for the owner to strike: the variables `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR`;
the files `ca.crt`, `jwks.json`, `engine-id`, `token`, `tls.crt` and `tls.key`; the terms gate,
enforcement point, decision point and the engine's ID. `spec/names.yaml` holds the names of
`identity`, and `tools/set_names.py` changes them together.
