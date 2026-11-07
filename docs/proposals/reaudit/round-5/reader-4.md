# Round 5, reader 4, behavior across machines

A cold read of `spec/` at commit d0c425f on the branch `docs/pre-codex`, and of nothing outside
it. I read in full `README.md`, `definitions.md`, `names.yaml`, `implementation-defined.yaml`,
`promises/providers.yaml`, `promises/secrets.yaml`, `promises/placement.yaml`,
`interface/implementation.yaml`, `conventions/access.yaml`, `conventions/lifecycle.yaml`,
`conventions/operations.yaml`, `conventions/implicit.yaml`, `conventions/observation.yaml`,
`conventions/records.yaml`, the operations of the provider capabilities and `get-secret.v1` in
`core/operations.yaml`, and the item and server.json schemas in `schemas/manifest.yaml`; and I
read the other files where they touch machines, nodes, runs, gates, connections, identity,
restarts or parts of an engine. I walked the twelve situations of the brief end to end, and
checked every place where the text describes decisions made by more than one part.

There are 23 findings: 7 unsafe, 0 contradiction, 11 undetermined, 1 wrong, 1 misleading and
3 minor.

Overall: the chain of proof holds in its shape. A run reaches the engine with a token over
`https` that it checks against `ca.crt` (PRV-022, IFC-042), the engine reaches a run only once
what answers proves with the run's certificate (PRV-028, IFC-017), and a gate admits only a
credential that names the engine (PRV-025). A run left on a machine whose node was declared lost
reaches nothing through the engine (CON-008, IFC-032), and the stop asked for once more on
unregistering (LIF-036, PRV-021, PRV-027) closes scenario 5 as far as the text can. The defects
are at the joins. What the engine presents to a gate, and how it names the instance there, are
the engine's choices while the gate is the provider's, so a gate written once is not portable
(5-4-09, 5-4-10). `http` on loopback is kept off a secret header but not off a run's
credential, a plain server's calls, or a store's answer (5-4-02 to 5-4-04). A move fences only
the engine's own runner (5-4-01). The engine of several parts is described by its reads, while
several promises still assume one process that stops and starts as a whole (5-4-08, 5-4-12,
5-4-15, 5-4-19, 5-4-20, 5-4-22).

Where each situation of the brief led: 1, an Agent on M2 through the engine to a tool on M3:
5-4-05, 5-4-09, 5-4-10, 5-4-19. 2, a store on M2 read before a run on M3, and a direct call:
5-4-03. 3, a partition with a setup and requests in flight: 5-4-07, 5-4-13, 5-4-15. 4, M2
unreachable then back: 5-4-16, 5-4-21. 5, M2 gone and back with a run on it: 5-4-21, and
otherwise nothing found. 6, restarts on M1 and on another machine: 5-4-01, 5-4-02, 5-4-08,
5-4-12, 5-4-16, 5-4-17. 7, a Conduit closed while M2 is cut off: 5-4-12. 8, a `remote` item
that takes Conduit's keys: 5-4-05, 5-4-06, 5-4-14, 5-4-18, 5-4-23. 9, a plain `remote` item at
`https` and at `http://localhost`: 5-4-03, 5-4-04, 5-4-18. 10, an off-the-shelf client with a
bearer token: 5-4-15, 5-4-21. 11, one machine, everything over stdio: 5-4-02, 5-4-11, 5-4-17.
12, a Docker-run Agent on M2: 5-4-07, 5-4-09, 5-4-10. Decisions made in more than one place:
5-4-04, 5-4-08, 5-4-12, 5-4-15, 5-4-19, 5-4-20, 5-4-22.

## 5-4-01. A move fences only the engine's own runner, so a run that another runtime provider made on `local` is left on the machine the engine left

- Severity: unsafe.
- Promises: DL17, PLC-026, PRV-006, PRV-008, PRV-019, PRV-026, LIF-024, LIF-035, TPL-012.
- Quotes: DL17, "before the runner runs anything on another machine, every process that it
  started on the machine the engine left has ended", and "the engine moves only to a machine
  that takes its hostname"; README, "An Agent that is not to be trusted that far is given a
  runtime provider that runs it apart, such as a container provider on `local` (PRV-006)."; PRV-008,
  "Stopping a run that is not running succeeds."
- Situation: The engine runs on M1 and is configured under DL17 to restart on M1', which takes
  the hostname. Agent A is placed on `local` with a container provider as its `runtime`, whose
  configuration names the local Docker socket, as the README advises for an Agent not to be
  trusted. The engine moves. A's container keeps running on M1 with A's secrets and a credential
  that lasts as long as A, and reaches the engine at `CONDUIT_URL`, which DL17 says keeps
  reaching it. The provider's server is run again on M1' (PRV-026) and asks M1''s daemon:
  `inspect-run.v1` says `stopped`, so a further setup asks `run.v1`, and a second run of A starts
  on M1'. Both present as A. A retirement's `stop.v1` reaches only M1', succeeds, the shutdown
  is reported confirmed (LIF-024) and A is removed (LIF-035), while the container on M1 still
  runs with A's secrets. The engine kept every promise and reported a confirmed shutdown that is
  false.
- Least change: In DL17, fence the machine and not only the runner: before anything is run,
  stopped or inspected for an instance on `local` on another machine, every run on the machine
  the engine left has ended, as the Kubernetes out-of-service taint presumes a machine shut down;
  or, on an engine that can restart on another machine, reject an instance on `local` whose
  `runtime` is not the engine's own runner.

## 5-4-02. A run's credential goes in clear to an `http` loopback address that anything may hold while the engine is not listening

- Severity: unsafe.
- Promises: DL13, PRV-022, IFC-022, IFC-042, PRV-028, SEC-024, DL16, DL17.
- Quotes: DL13, "the engine's address that it gives a run, which on the engine's own machine
  may be an `http` URL on loopback"; PRV-022, "On the engine's own machine `CONDUIT_URL` may be
  an `http` URL on loopback, as DL13 states, where `ca.crt` checks nothing"; PRV-028, "a program
  that has taken the address once the provider's process has ended, as after the engine
  restarts, is sent nothing"; SEC-024, "SEC-023 reads the secret again and would send it to
  whatever listens there"; README, "No consumer can present as another, except a program that
  can read the credential of another instance's run: root, and every program of the
  operating-system user that the run runs as".
- Situation: On M1 an Agent runs under a runtime provider placed on `local` that runs each run
  as its own operating-system user on the host's network, so the run outlives the engine; or on
  the engine's own runner, whose processes DL16 lets outlive it. Its `CONDUIT_URL` is
  `http://127.0.0.1:7777`. The engine restarts, or, under DL17, moves. While port 7777 is free,
  a process of an unrelated user binds it. The Agent's client retries and sends
  `Authorization: Bearer` with its `token`, and that process now holds a credential that lasts
  as long as the instance and presents as the Agent wherever the engine accepts it. PRV-028 keeps
  the engine from sending to such a squatter; nothing keeps the run from it, and SEC-024 forbids
  exactly this for a secret header on an engine that can move, but not for the credential, which
  is worth more. The README's list of who can present as another does not include this.
- Least change: In DL13 and PRV-022, allow an `http` loopback `CONDUIT_URL` only for a run of the
  engine's own runner whose process ends when the engine stops, on an engine that can neither
  restart on another machine nor run as several parts, and give every other run an `https`
  address that it checks against `ca.crt`; or at least state this limit in the README's list.

## 5-4-03. A secret Resource reached by a plain `remote` item stands behind no gate, and may stand at `http` on another machine

- Severity: unsafe.
- Promises: SEC-006, SEC-024, IFC-044, IFC-017, IFC-041, PRV-025, the contract `get-secret.v1`,
  the README's statement of what the guarantee covers.
- Quotes: README, "Only an enforcement point reaches the server of a secret Resource that the
  engine does not run itself, and the request and the value it returns cross between them
  confidential and unaltered", and "Every server that the engine does not run itself, the server
  of a run that a runtime provider makes and a `remote` server that takes Conduit's keys, stands
  behind a gate"; `get-secret.v1`, "The server that gives the value is reached only with the
  engine's proof"; SEC-006, "a store reached by `remote` that cannot check an enforcement point's
  proof is used as plain"; IFC-044, "A plain item may stand at an `http` URL".
- Situation: A store package's template selects, for `get-secret.v1`, a plain `remote` item at
  `http://secrets.m2.lan:8080/mcp`, which takes its locator from a header bound to the
  configuration. No rule rejects it: SEC-024 covers only a header that carries a secret, and
  IFC-044 only an item that is not plain. Before a run on M3, the engine reads; the value
  crosses from M2 to M1 in clear. Any process that reaches M2's port sends its own locator
  header and reads every secret that the store's credential reaches. On an engine that can move,
  the same item at `http://127.0.0.1` returns whatever the new machine's listener returns, and
  the engine passes it to the run as the secret. The engine kept every promise.
- Least change: Apply SEC-024's rule on the URL to the item that a template selects for
  `get-secret.v1`: a `remote` item that implements it stands at an `https` URL, or at an `http`
  loopback URL only where SEC-024 would let a secret header go. Add "that is not plain" to the
  README's two sentences and to the first sentence of `get-secret.v1`, and say that a plain store
  reached by `remote` is reached by whoever can reach its URL.

## 5-4-04. A plain `remote` item at an `http` loopback URL is accepted on an engine that moves or runs as several parts

- Severity: unsafe.
- Promises: SEC-024, IFC-044, DL17, IFC-041, IFC-010, PLC-007, PLC-020.
- Quotes: DL17, "Such an engine rejects a template whose item would send a header that carries a
  secret to an `http` URL", and "no other document needs to change"; IFC-044, "the `http` URL on
  loopback that SEC-024 allows, on an engine that, as configured, can neither restart on another
  machine nor run as several parts, serves a plain server alone"; SEC-024, "would send it to
  whatever listens there".
- Situation: Situation 9, on the engine that can move. A plain item at
  `http://localhost:8080/mcp`, a local database server with no secret header, is accepted. After
  a move to M1', or whenever the part that holds the connection runs on another machine, every
  call, with the caller's arguments, goes to whatever listens on that machine's loopback,
  another user's process among them, and what it answers is the outcome under IFC-010. The reason
  that SEC-024 gives for a secret header holds for the arguments and for the result. On any
  engine the URL is also read on the engine's machine, although the instance that selects the
  item may be placed on M2, and no selector ties it to `local`.
- Least change: Make SEC-024's condition on loopback hold for every `remote` item whose URL's
  host is a loopback address, whatever its headers, and have PLC-007 add `local`'s hostname
  label to the selector of an instance that selects one.

## 5-4-05. The engine may follow an HTTP redirect and present its certificate to a server that has not proved that it serves the instance

- Severity: unsafe.
- Promises: IFC-017, IFC-041, PRV-028, PRV-025, SEC-013, DL13, ACC-003.
- Quotes: IFC-017, "To a run's `url`, and to an item reached by `remote` that is not `plain`, the
  engine sends nothing, `server/discover` among it, until the server's side has proved who it
  is"; SEC-013, "The value goes in no request to any other URL, an HTTP redirect to another URL
  among them"; DL13, "Which of the two, one or both, the engine presents to a gate".
- Situation: The tool server A on M3 is compromised and answers a call with `307` to the `url`
  of instance B's run on M2. An engine that presents mutual TLS and follows redirects, as common
  HTTP clients do by default, sends A's request, with A's `config` and the caller's arguments, to
  B's gate with its client certificate; the gate admits the engine (PRV-025), and B performs an
  operation that no open Conduit lists on B. The checks of IFC-017 and PRV-028 attach to the
  `url` that the provider reported, not to where a redirect leads, and only a secret header is
  kept off a redirect. With the token form B's gate refuses, since the audience is A; an engine
  that does not follow redirects is unaffected.
- Least change: Add to IFC-017 and IFC-041 that the engine follows no HTTP redirect on a
  connection to a run or to a `remote` item that is not plain, and reads a redirect as no
  result.

## 5-4-06. The engine takes a token for one `remote` item from whatever authorization server that item names

- Severity: unsafe.
- Promises: IFC-041, DL13, PRV-025.
- Quotes: IFC-041, "issued to the engine, under the protocol's extension for client
  credentials, by the authorization server that the item's protected resource metadata names, to
  which the engine authenticates with its own credential of DL13"; its observable, "names as its
  authorization server the one that DL13 states, or one that the user runs"; PRV-025, "in a form
  that no server that receives that proof can present to another".
- Situation: The user runs an authorization server, AS-Y, that issues client-credentials tokens
  to the engine for a `remote` item Y. An installed package's `remote` item X, not plain,
  publishes protected resource metadata that names AS-Y. The engine authenticates to AS-Y with
  its own credential and asks for a token for X. AS-Y, as many do, ignores RFC 8707's `resource`,
  which that RFC only recommends honoring, and issues a token whose audience is Y. The engine
  sends it to X, which presents it to Y; Y admits it as the engine and acts on any `instance`,
  `caller` and `config` that X writes. An engine that takes tokens only from an authorization
  server configured for the item is safe; this one kept every promise.
- Least change: In IFC-041, obtain a token only from the authorization server that DL13 states
  or one that the user has configured for that item's URL, and never from one that the item's
  metadata alone names.

## 5-4-07. A run is asked for again where its provider reports `unknown`, and nothing keeps a provider from starting a second

- Severity: unsafe.
- Promises: PRV-007, TPL-018, PRV-009, `run.v1`, `inspect-run.v1`, DL17, TPL-019, PRV-014.
- Quotes: PRV-007, "its runtime provider, asked with `inspect-run.v1`, does not report the run
  running"; DL17, "one run serves one instance, under TPL-019"; TPL-019, "an instance has at most
  one item that is run"; PRV-014, "Kubernetes warns that deleting a Pod by force on silence can
  duplicate a Pod that is still running".
- Situation: Situation 3. M1 and M2 are cut apart while the machine provider, a cloud's API,
  still reports M2 reachable, so PLC-023 holds nothing back. Agent A's setup ended unknown, and
  an actor sets A up again. A's runtime provider cannot reach M2's daemon, so `inspect-run.v1`
  says `unknown`, and the engine asks `run.v1`. A provider that names its containers itself, or
  that reaches M2 again by then, starts a second run of A. Both present as A and both act, and a
  retirement's stop reaches one. "One run serves one instance" rests on TPL-019, which only
  keeps a template to one item that is run.
- Least change: In PRV-007 and TPL-018, ask for the run only where `inspect-run.v1` reports
  `stopped`; where it reports `unknown`, the part is not dispatched and reports unavailable. Say
  in `run.v1` that a provider starts no run for an instance's ID while one that it started may
  still run, and correct the citation in DL17.

## 5-4-08. When the part that holds a provider's server stops, nothing runs the provider again, and ACC-019 forbids the reads that would

- Severity: undetermined.
- Promises: PRV-026, PRV-002, ACC-019, SEC-011, REC-002, DL17, DL5, PLC-023.
- Quotes: DL17, "each server of a provider, and each connection to an item reached by `remote`,
  is held by one part at a time", and "An invocation in progress on a part that stops ends
  unknown, as under IMP-007, although the engine as a whole goes on."; PRV-026, "After the engine
  restarts, it is to make a provider call"; ACC-019, "the read of a secret for a provider's run
  that PRV-026 asks for again once the engine has restarted"; PRV-002, "that server ends when the
  engine stops, and the engine runs it again when it first needs it, under PRV-026".
- Situation: An engine of two parts, P1 on machine A and P2 on B. P1 holds the servers of the
  runtime provider and the machine provider. A dies; the engine as a whole goes on, so it has
  not restarted. The next provider call finds no server. Engine E1 reads "restarts" broadly and
  runs the providers again on B, reading their secrets for the caller of their last setup.
  Engine E2 keeps PRV-026 and ACC-019 as written and reads nothing: every provider call reports
  unavailable, the machine provider's reports stop, after DL5's wait every machine but `local`'s
  counts as omitted (PLC-023), and all work on other machines is unavailable until an actor sets
  each provider up again. The promises were written for an engine that stops and starts as a
  whole.
- Least change: In PRV-026, ACC-019, SEC-011, REC-002 and the definitions of provider, request
  and later work, put "once the provider's run on the engine's own runner has ended without the
  engine asking, as when the engine restarts or the part that held it stops" in place of "after
  the engine restarts".

## 5-4-09. Which proof a run's gate is given is the engine's choice, and which it admits is the provider's

- Severity: undetermined.
- Promises: DL13, PRV-025, PRV-022, IFC-017, PRV-028.
- Quotes: DL13, "Which of the two, one or both, the engine presents to a gate and to an item
  reached by `remote`"; PRV-022, "The names are fixed so that a harness or a gate written once
  works under every engine"; PRV-025, "ghostunnel admits a certificate by its URI", and "a proxy
  that checks a token whose audience is the instance's ID, as Envoy's `jwt_authn` checks a
  configured audience".
- Situation: Situations 1 and 12. A Docker runtime provider's contract states a ghostunnel gate,
  which admits a client certificate and nothing else. Under engine E1, which presents mutual TLS,
  the tool server on M3 serves; under E2, which presents only its token, the handshake fails and
  every part and request that needs a run of that provider reports unavailable (IFC-017). A
  provider whose gate is Envoy's `jwt_authn` fails the other way. Engines and provider keep every
  promise.
- Least change: In DL13, have every engine present both its certificate, to a gate that asks for
  one, and its token, to every run's gate; or fix one form for runs, and have PRV-025 name it.

## 5-4-10. Nothing fixes how the engine's token names the instance it is meant for, so a gate cannot know what audience to admit

- Severity: undetermined.
- Promises: DL13, IFC-017, PRV-025, PRV-022, IFC-022.
- Quotes: DL13, "and which toward a run names the instance's ID as its audience", "how an ID
  fits the characters that its form allows", and "SPIFFE's form is recommended and not
  required"; PRV-025, "a proxy that checks a token whose audience is the instance's ID, as
  Envoy's `jwt_authn` checks a configured audience".
- Situation: Situation 1. The provider sets its gate's audience from the `instance` that
  `run.v1` gave it, `3f2a…`. Engine E1 puts `3f2a…` in `aud` and is admitted; E2 puts the name
  its credentials use, `spiffe://example.org/conduit/instance/3f2a…`, and is answered 401.
  `identity` holds no file that says which name the engine will use. A gate built on a SPIFFE
  library also refuses E3, whose `sub` and URIs are not SPIFFE IDs, as DL13 allows, although
  the specifications of the X509-SVID and the JWT-SVID, whose forms the text names, require a
  SPIFFE ID there.
- Least change: In DL13, fix that the engine's token toward a run carries as `aud` exactly the
  name that the run's own `token` carries in `sub`, which is the URI of its `tls.crt`; and either
  require SPIFFE IDs or stop calling the forms SVIDs.

## 5-4-11. At an `http` loopback address no engine must accept the token, so a harness written once may be refused

- Severity: undetermined.
- Promises: IFC-042, IFC-022, DL13, PRV-022, PRV-018.
- Quotes: IFC-042, `when`, "A consumer sends a request to the engine's address at an `https`
  URL."; its observable, "on the engine's own machine DL13 may also give a run an address that
  is not `https`, such as an `http` URL on loopback, and establish an instance there by other
  means"; PRV-022, "The names are fixed so that a harness or a gate written once works under
  every engine".
- Situation: Situation 11: one machine, everything on the engine's own runner, and an Agent
  whose harness sends the file `token` from `CONDUIT_IDENTITY_DIR` as `Authorization: Bearer` to
  `CONDUIT_URL`, here `http://127.0.0.1:…`. Engine E1 accepts the token there; E2 establishes
  the instance there by other means, such as a header of its own, and answers the harness 401
  (IFC-032). PRV-018 rules out the peer's address and process, so the other means are something
  the request presents that no fixed name tells a harness to send.
- Least change: In IFC-042, accept the token form at every address that the engine gives a run
  in `CONDUIT_URL`, `http` loopback included.

## 5-4-12. Whether the engine keeps a connection's secret headers or reads them again is open, so closing the reading Conduit stops the header on one engine and not on another

- Severity: undetermined.
- Promises: SEC-013, SEC-023, SEC-017, SEC-014, SEC-012, ACC-011, DL17.
- Quotes: SEC-013, "holds it only until the connection ends: when a later setup's connection
  takes its place, when the instance's retirement ends, or when the instance's node is declared
  lost", and "closing the reading Conduit recalls nothing"; SEC-023, "the engine no longer holds
  the values read for the connection, as after the engine restarts, on its machine or on
  another".
- Situation: Situation 7. Instance X's `remote` item takes `X-Api-Key` from secret Resource S.
  Root closes X's Conduit to S. Engine E1 holds the value for the life of the connection and
  sends the key on every later request to X. Engine E2 holds it only for each request, since
  "only until" sets no lower bound, or, with several parts, loses it whenever the connection
  passes to another part; it reads again under SEC-023 before each request, the read is refused,
  and every request to X reports unavailable (SEC-012). Their records differ too: one read
  against one for each request.
- Least change: In SEC-013, say that the engine holds the values for as long as the connection
  lasts, and loses them only when it restarts or the part that holds the connection stops.

## 5-4-13. "Gone silent" is not defined, so whether a gate cancels a long call depends on the engine

- Severity: undetermined.
- Promises: PRV-025, OPS-027, the capability `runtime-provider.v1`.
- Quotes: PRV-025, "how long the gate takes to close a server's stream once the connection that
  carries requests to the server from the enforcement point has gone silent", and "which the
  gate then closes and the protocol has the server treat as a cancellation".
- Situation: A tool on M3 takes five minutes and answers with a stream on which it sends nothing
  meanwhile; the provider's contract gives the gate 60 seconds. Engine E1 sends HTTP/2 PINGs or
  TCP keepalives, so the connection is not silent and the call succeeds. Engine E2 sends nothing
  while it waits; the gate closes the stream at 60 seconds, the server cancels, and the
  invocation ends unknown. No promise asks the engine to keep an idle connection alive, or says
  what traffic counts.
- Least change: Say in PRV-025 that silence is the failure of the transport's own check of
  liveness, and in DL10 or DL13 that the engine keeps such a check, an HTTP/2 PING or a TCP
  keepalive, on every connection to a gate, at an interval that its conformance document states.

## 5-4-14. A `remote` item that takes Conduit's keys has no credential and no address of the engine for the onward requests that the text expects of it

- Severity: undetermined.
- Promises: IFC-015, IFC-022, TPL-017, DL13, IFC-042, PRV-022.
- Quotes: TPL-017, "so its onward requests, which are onward requests under ACC-016, go through
  Conduits that are open"; IFC-015, "It makes it as a client of the server the engine presents
  for that instance."; DL13, "the grants of the protocol's authorization that it supports for a
  consumer, and how it decides which instance a token that it issues names".
- Situation: Situation 8. A server at `https`, not plain, serves instances X and Y of one engine,
  and its setup hook makes onward requests. A run would present the `token` of its `identity`; a
  `remote` item is given no `identity`, no `CONDUIT_URL` and no credential. Engine E1's
  authorization server issues the server a token for each instance by some grant; engine E2's
  offers only a grant for people, so the server's onward requests report refused (IFC-032). A
  server written once works under one engine only, and under E1 it holds credentials that
  present as every instance it serves.
- Least change: Say in IFC-015 that an implementation reached by `remote` makes no onward
  request in this version; or fix one grant that every engine's authorization server offers,
  such as RFC 8693 token exchange of the engine's token on a call for one that names that call's
  instance.

## 5-4-15. Where the engine cannot read whether the instance that a valid credential names still exists, one engine refuses with 401 and another reports unavailable

- Severity: undetermined.
- Promises: IFC-022, IFC-032, ACC-037, OPS-023, DL17, DL4.
- Quotes: IFC-022, "exactly one instance or none: the instance whose ID it names, while that
  instance exists, and none once it is removed"; IFC-032, "which is 401 for a credential that
  is missing or invalid"; ACC-037, "the state of the Conduits that the decision needs cannot be
  established".
- Situation: Situation 10, with the part that receives an off-the-shelf client's request cut off
  from the store of the engine's own state (DL17). The token checks, but whether its instance
  still exists cannot be read. Engine E1 establishes none and answers 401 under IFC-032, which
  tells the client its credential is bad, so it drops the token or starts a sign-in. Engine E2
  takes it as ACC-037 and reports unavailable. A third that trusts the token breaks IFC-022 for
  a removed instance. The text treats checking the instance as always possible.
- Least change: In IFC-022 or ACC-037, say that where the existence of the instance that a valid
  credential names cannot be established, the request reports unavailable, without 401, as under
  ACC-037.

## 5-4-16. Whether a machine provider's report from before a restart still counts after it is open

- Severity: undetermined.
- Promises: PLC-023, DL5, PRV-026, PRV-014.
- Quotes: PLC-023, "or is omitted from its latest report while a node represents it", and "where
  no report arrives within the time that DL5 of the implementation-defined list gives, every
  machine but the one that `local` represents counts as omitted".
- Situation: Situations 4 and 6. M2 was reported unreachable, and the engine restarts. Until the
  machine provider, run again under PRV-026, gives its first report: engine E1 keeps the last
  report, so a request to an instance on M2 reports unavailable; engine E2 keeps none, and within
  DL5's wait M2 is neither reported unreachable nor omitted, so it dispatches. The reverse holds
  for a machine last reported reachable that went away during the restart.
- Least change: In PLC-023, say whether the latest report before a restart counts until the first
  one after it, or that every machine but `local`'s counts as omitted until then.

## 5-4-17. What a request reports when it is to a ready instance whose process on the engine's own runner has ended is not stated

- Severity: undetermined.
- Promises: IFC-017, PRV-026, DL16, OBS-004, OPS-007, PRV-028.
- Quotes: IFC-017, "only the engine's own runner returns none, and the engine reaches the process
  it starts over stdio", and "Where the engine cannot verify who answers, nothing is
  dispatched"; PRV-026, "An Agent's run on that runner is not run again so: a further setup asks
  for it".
- Situation: Situations 6 and 11. A tool server T runs on the engine's own runner and is ready;
  under DL16 its process ends when the engine stops, and the engine restarts. OBS-004 keeps the
  engine from running it again unasked. A request to T: engine E1 does not dispatch it and
  reports unavailable; engine E2 writes to the closed pipe, gets no result and reports unknown
  (OPS-007), which lands in T's `unreconciled` and waits for a reconciliation. For a run with a
  `url`, IFC-017 makes it unavailable; for stdio nothing does.
- Least change: In IFC-017, say that where the engine's own runner has no process for an
  instance's run, a request or a hook to it is not dispatched and reports unavailable.

## 5-4-18. Which roots the engine trusts when TLS proves a `remote` item's host is not stated

- Severity: undetermined.
- Promises: IFC-017, IFC-041, IFC-044, DL13.
- Quotes: IFC-017, "by TLS for the host of its URL", and "Toward an item reached by `remote`, TLS
  proves the host of its URL".
- Situation: Situations 8 and 9. A `remote` item stands at `https://tools.corp.internal` with a
  certificate from the user's private authority. Engine E1 trusts the operating system's store
  and connects; engine E2 trusts only a bundled public list and reports unavailable. For a run
  the roots are DL13's (PRV-028), and for the engine's address as a run checks it they are
  `ca.crt`; for a `remote` item nothing, DL13 included, names them.
- Least change: Add to DL13 the roots by which the engine checks the host of a `remote` item's
  URL.

## 5-4-19. A short-lived engine credential does not bound how late a paused part's request lands

- Severity: wrong.
- Promises: DL17, DL13, IFC-017, IFC-022, IFC-041.
- Quotes: DL17, "the engine's own credential, which DL13 makes short-lived on an engine of
  several parts, bounds how late"; IFC-022, "under mutual TLS, the certificate with which the
  request's TLS connection was made is part of what that request presents".
- Situation: Part P1 opens an HTTP/2 connection to a run's gate with a certificate that lives
  five minutes, pauses for thirty and resumes; it sends a call on the same connection. The gate
  checked the certificate at the handshake and admits the call, whose effect lands twenty-five
  minutes after the credential expired. With a token from a `remote` item's authorization server
  (IFC-041), the bound is that server's lifetime, not the engine's credential's.
- Least change: In DL13, on an engine of several parts, have the engine close every TLS
  connection to a gate or a `remote` item before the certificate it was made with expires, and
  ask an authorization server for tokens no longer-lived; or drop the sentence from DL17.

## 5-4-20. DL17 names linearizable reads and a compare-and-set on one invocation as the mechanism, which do not keep the rules that span invocations

- Severity: misleading.
- Promises: DL17, OBJ-002, OPS-008, REV-014, PRV-013.
- Quotes: DL17, "Every decision that a promise has the engine make reads the state as it stands,
  as a linearizable read of etcd does.", and "At most one part of the engine acts on any one
  invocation, and the store of the engine's own state enforces it".
- Situation: Two parts each take a `register.v1` for a document named `db`; each reads,
  linearizably, that no `db` exists, and each stores one: two objects named `db`, against
  OBJ-002. So too two requests that carry one idempotency key (two invocations, against
  OPS-008), two setups of one instance (REV-014) and two machine providers (PRV-013). The promises
  still bind the whole engine, so a careful engine conforms, but an implementer who builds what
  DL17 describes builds the race. This is the one place the text describes decisions across
  parts, and it describes reads alone.
- Least change: Add to DL17 that a decision that admits one of competing changes, such as a
  name, an idempotency key, a change of an instance or a provider, is committed against the state
  it read, by a compare-and-set or a transaction.

## 5-4-21. An instance with no item that is run is placed on an arbitrary machine, and is made unavailable and retired with that machine

- Severity: minor.
- Promises: PRV-015, PLC-002, PLC-007, PLC-023, PLC-024, PLC-027, CON-008, `shared-host.v1`.
- Quotes: PLC-007, "An instance with no item that is run, such as one reached only by `remote`,
  gets no such label."; PLC-027, "this holds for a hook whose implementation is a `remote` item
  too, since the instance's node decides".
- Situation: Situations 4, 5 and 10. A Resource reached only by `remote` at
  `https://api.example`, or an Agent that an off-the-shelf client drives from a laptop
  (IFC-042), is given no placement, and the machine provider places it on M2. While M2 is
  reported unreachable, every request to it reports unavailable although its server answers;
  when M2 is reported gone, as when an autoscaler deletes it, it is retired, its Conduits close
  at once and the Agent loses all its access. Nothing of it ran on M2, and `shared-host.v1`
  would call it running there.
- Least change: In PRV-015, place on `local` an Agent or a Resource for which no item that is run
  is selected and no placement is given, as PRV-002 places a provider.

## 5-4-22. A `file:` fetch on an engine of several parts reads the file of whichever part takes the request

- Severity: minor.
- Promises: PRV-003, PRV-017, NAM-012, MAN-006, DL17, PKG-024, PKG-026.
- Quotes: PRV-003, "it fetches the archive itself"; DL17, "What the promises say of the engine's
  own runner, of the servers of providers and of connections holds of the whole".
- Situation: An engine runs as parts on machines A and B. A request to install gives
  `fetch: file:///srv/pkgs/history-1.4.0.tar.gz`, which only A holds. Taken on A it installs;
  taken on B it reports unavailable (PKG-024), or invalid where B holds another file there
  (PKG-026). The digest keeps the wrong bytes out, but one request has two outcomes. DL17 says
  where the runner, the servers of providers and the connections are, and nothing of fetching.
- Least change: Add the engine's own fetching of `file:` URLs to DL17's list: every machine that
  may run the engine holds a file that an install names, or one part fetches them.

## 5-4-23. A Remote's `variables` are accepted but can never be used, and SEC-024 describes a case that cannot arise

- Severity: minor.
- Promises: MAN-010, SEC-024, the schema `server-remote.v1`, the schema profile's row for `uri`.
- Quotes: SEC-024, "The URL is compared as the Remote writes it, so a host written as a variable
  of the Remote is none of these"; MAN-010, "a `remote` whose `url` is not a URI".
- Situation: server.json writes a Remote's URL with variables, as
  `https://{tenant}.example.com/mcp` beside `variables`. `server-remote.v1` gives `url` the
  format `uri`, which the profile asserts by RFC 3986, whose grammar has no `{`; so MAN-010
  rejects every Remote that uses a variable, `variables` can never be used, and SEC-024's
  sentence, with the rule that the schemas' README describes for it, addresses a URL that is
  rejected before it.
- Least change: Say in MAN-010 that a Remote that declares `variables` is rejected, and drop the
  sentence from SEC-024 and the rule from the schemas' README; or bind the variables, and check
  SEC-024 against the URL once bound.
