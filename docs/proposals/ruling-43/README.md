# Ruling 43: work across machines

Status: **ruled by the owner on 2026-11-06** (plan-audit ruling 43). This is the record of a
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
