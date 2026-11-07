# Round 5, reader 2, terms and shapes

Read cold, at commit d0c425f on the branch `docs/pre-codex`, and nothing outside `spec/`: the
README, `definitions.md`, `names.yaml`, `schema-profile.md`, the 23 promise files (449 promises:
202 `language`, 206 `convention`, 41 `interface`, which matches the README's counts file by file
and layer by layer), `implementation-defined.yaml` (16 items), the eight files of `core/` (102
exports and the implementation `engine`), the eight schema files of `schemas/` (41 Schema
contracts), the 7 valid example files, the 56 invalid examples and the 28 invalid-by-rule sets.

I built the table of keys for every document kind (the envelope and `metadata`, the `spec` of
each family's template, the instance of each family, the two nested instantiations, an input,
the `implementation` and `controls` maps, the five contract kinds, an applied requirement, the
manifest, an item of `packages`, an implementation item, a server.json Package and Remote) and
for the inputs and outputs of the 23 core operations, and checked each key against the promise
that names it, the definitions row that describes it and the examples that use it. Every
promise ID that a non-comment line cites exists, apart from the removed `GRA-` IDs that the
README's table lists. The key table agrees with the promises key by key; I found no key that a
promise requires and a schema lacks, or the reverse, except as the findings below say.

21 findings: 1 contradiction, 1 undetermined, 3 wrong, 4 misleading, 12 minor. None is unsafe.
Overall: the shapes are in good order; the defects are in the terms. `definitions.md` claims
that each term has one meaning and that several words are used for one thing only, and the
promises break a number of those claims; two rules on schemas (OBJ-019 and OBJ-021) do not
cover every form the profile admits.

---

## 5-2-01

- **Severity:** contradiction
- **Promises:** OBJ-019, SCH-001, SCH-002
- **Quotes:** OBJ-019: "A document contains a key that this specification does not define,
  other than: ... inside a schema, a name the author gives a member of `properties` or of
  `$defs`, and a key inside a value the schema states under `const`, `enum`, `default` or
  `examples` ... The document is rejected." `schema-profile.md`: the profile holds
  "`patternProperties`, `additionalProperties` and `propertyNames`" and "`required` and
  `dependentRequired`", and `dependentSchemas`. `schemas/schema.yaml` header: "OBJ-019 the
  names under `properties` and `$defs`, and the values of `const`, `enum`, `default` and
  `examples`, are the author's".
- **Situation:** A Type's `schema` writes `patternProperties: {"^x-": {type: string}}`, or
  `dependentRequired: {proxy: [port]}`. Each keyword is in the profile, `schema.v1` accepts it,
  and SCH-001 gives it JSON Schema's meaning; but its keys (`^x-`, `proxy`) are not keys this
  specification defines and are none of OBJ-019's exemptions, so OBJ-019 rejects the document.
  An engine that follows OBJ-019 rejects every use of these three keywords with an author's key;
  one that follows the profile accepts it. The core package escapes only because its own
  `dependentRequired` and `dependentSchemas` keys (`version`, `packageArguments`, `headers`,
  `environment`, `package`) happen to be keys the specification defines.
- **Least change:** Add to OBJ-019's exemptions "a key of `patternProperties`,
  `dependentRequired` or `dependentSchemas`", and say the same in the head comment of
  `schemas/schema.yaml` and in the row "A schema" of "Closed and open" in `schemas/README.md`.

## 5-2-02

- **Severity:** undetermined
- **Promises:** OBJ-021, SEC-001, MAN-017, SEC-010, INS-006
- **Quotes:** OBJ-021: "A reference schema governs a place where it is the schema of that place
  directly, where it is the `items` of a list that holds the value, or where it is a branch of
  `anyOf` or `oneOf` that the value satisfies." SEC-001: "`secret-reference.v1` governs a place
  only as the schema of the place itself ... and a schema that puts it under any other
  applicator, or beside any other branch, is rejected." `schemas/README.md`: "a `$ref` to a
  definition that holds it is not followed"; MAN-017's rule "and not a `$ref`".
- **Situation:** Every use of a reference schema is already one `$ref`. Nothing says whether a
  second hop still "governs" the place: a project's Schema contract `token.v1` whose `spec` is
  `{$ref: github.com/shrav-k/conduit/core#secret-reference.v1}`, used as
  `token: {$ref: '#token.v1'}` in a Type's schema, or a local `$defs` member that holds the
  `$ref`. `$ref` is not in the applicator vocabulary, so SEC-001 does not reject it. Engine A
  follows the `$ref`: it resolves `{kind: Resource, name: github-token}`, and an item that binds
  `GIT_TOKEN: /token` has the secret read and passed in `secrets`. Engine B does not: the value is
  data, kept as written, and MAN-017 rejects the template because the place admits an object.
  The same split decides, for `resource-reference.v1` behind a `$defs` member, whether a value
  is stored as `{kind, id}` or as written, and whether INS-006 accepts a bare name.
- **Least change:** Say in OBJ-021 that a reference schema governs a place only where the
  place's schema is `{$ref: <reference schema>}` itself, as `items` or as a branch, and that a
  schema that reaches a reference schema through any other `$ref` is rejected; or say that
  `$ref` chains are followed. Repeat the choice in SEC-001 and MAN-017.

## 5-2-03

- **Severity:** wrong
- **Promises:** PRV-018, PRV-025, IFC-042, PLC-017, PLC-024, SEC-013, IFC-036, IFC-037,
  IFC-017, IFC-041, IFC-044, PRV-022, SEC-024, DL13
- **Quotes:** `definitions.md`, "Words the promises do not use": "grant | Not used."; "declaration
  | Not used."; "\"Policy\" stays only in restart policy"; "\"Host\" stays only in the name of
  the shared-host requirement (PLC-013) and in the host part of an address (NAM-001)";
  "machines provider, the engine's own machine | Renamed by ruling 42 (N2) ... The machine the
  engine runs on is the computer."; "origin | Renamed by ruling 42 (N14): source". Against
  them: IFC-042 "by a grant that DL13 states"; PLC-017 "The declaration asks for no stop";
  IFC-036 "needs no declaration"; PRV-018 "neither does a VPN or a network policy"; SEC-024 "an
  `http` URL whose host is `localhost`"; PRV-022 "keeps the host of the engine's address";
  IFC-017 "by TLS for the host of its URL"; IFC-037 "for its origin on HTTP"; PRV-022 "On the
  engine's own machine `CONDUIT_URL` may be an `http` URL on loopback" (also IFC-042, DL13).
- **Situation:** Each row states a fact about the text that the text contradicts. The worst is
  "the engine's own machine": the rows retire it as the old name of the built-in machine
  provider, and three texts now use it for the computer the engine runs on, the very confusion
  the rename removed.
- **Least change:** Make each row say "not used in this sense", naming the senses kept (an
  OAuth grant; declaring a node lost or declaring an extension; a network policy; the host of
  a URL; a web origin), and write "the machine the engine runs on" in IFC-042, PRV-022 and DL13.

## 5-2-04

- **Severity:** wrong
- **Promises:** PRV-023, DL16
- **Quotes:** `core/README.md`, "Providers": "Its own runner passes a process the variables
  `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER` of the engine's environment, the
  further variables that a list set on the runner names, and what the item binds or `identity`
  gives, which wins, and nothing else of the engine's environment (PRV-023)." PRV-023: "or, on
  an operating system on which a process needs others in their place, such as Windows, those
  that DL16 names, in either case only where the value does not begin with `()`".
- **Situation:** On Windows the runner passes `SYSTEMROOT`, `TEMP` and the like, which the core
  README says it does not; and a `PATH` whose value begins with `()` is not passed, which the
  core README says it is. The page says that where it and a promise differ, it is wrong.
- **Least change:** Add "or, on an operating system such as Windows, those that DL16 names in
  their place, each only where its value does not begin with `()`".

## 5-2-05

- **Severity:** wrong
- **Promises:** SEC-006, PRV-025, PKG-020, IFC-041
- **Quotes:** `core/operations.yaml`, `get-secret.v1`: "The server that gives the value is
  reached only with the engine's proof: its run, where a runtime provider other than the
  engine's own runner makes it, stands behind that provider's gate, and an item reached by
  `remote` that is not `plain` checks that proof itself." `core/capabilities.yaml`,
  `secret-provider.v1`: "A secret Resource's server is reached only with the engine's proof ...
  A store reached by `remote` that cannot check that proof is marked `plain`: it is then reached
  as before, with no proof".
- **Situation:** A secret Resource on the engine's own runner is reached over stdio with no
  proof (PRV-003, PRV-025), and a plain `remote` store with none (PKG-020, SEC-006). The first
  sentence of each description says "only with the engine's proof"; the capability's own next
  sentence contradicts it. A package author reading the operation would think a plain store is
  not allowed, which SEC-006 allows.
- **Least change:** "The server that gives the value, where a runtime provider other than the
  engine's own runner runs it, or where it is reached by `remote` and is not plain, is reached
  only with the engine's proof", in both descriptions.

## 5-2-06

- **Severity:** misleading
- **Promises:** PRV-011, PRV-012, PRV-014, PRV-019, PRV-021, PRV-025, PRV-006, ACC-027, OPS-027,
  LIF-016, LIF-036, SEC-006, SEC-014, SEC-017, SEC-018, SEC-020, CTR-001, CTR-004, OBJ-018
- **Quotes:** `definitions.md`: "contract | A document of the kind `Type`, `Capability`,
  `Operation`, `Requirement` or `Schema`". PRV-021: "Its contract states what it does with that
  stop ... and where it keeps the stop meanwhile". PRV-011: "its contract states where it keeps
  the value, and that channel." PRV-025: "the runtime provider's contract states its gate, that
  user or group, and how long the gate takes to close a server's stream". SEC-017: "as its
  package's contract states". LIF-016: "as its type contract defines". OBJ-018: "Neither
  `description` nor `annotations` has any meaning to the language."
- **Situation:** In about twenty promises "contract" means prose that a provider, a type or a
  store package publishes about its behaviour. No contract document can hold it: `type-spec.v1`
  is closed and has no key for a gate, a channel, a held stop or a restart, and the only free
  text, `metadata.description`, has no meaning to the language. A reader looking for where "its
  contract states" a gate finds nothing in the shapes. The word names two things, and the
  second is not defined.
- **Least change:** Add a row, such as "a provider's contract, a type's contract: the
  documentation that a package publishes beside its Type, which states what the promises have
  it state; no engine reads it, and it is not a contract in the sense of CTR-001", or use
  another word for it.

## 5-2-07

- **Severity:** misleading
- **Promises:** PRV-022, PRV-018, IFC-022, IFC-042, DL13, NAM-001
- **Quotes:** `definitions.md`: "address | A package's identity: a host part ... and then a path".
  PRV-022: "`CONDUIT_URL`, the engine's address". IFC-042: "whose audience is the engine's
  address as its canonical `https` URI". DL13: "the engine's address that it gives a run, which
  on the engine's own machine may be an `http` URL on loopback". PRV-018: "It takes the address,
  the machine, the network ... that a request came from".
- **Situation:** "The engine's address", which sixteen lines of the promise files use and on which the
  token audience and `CONDUIT_URL` rest, is a URL, not an address in the defined sense (no
  scheme, NAM-001's form); PRV-018 uses "address" for a network address too. The phrase also
  varies with who reaches the engine (an `https` URL from another machine, possibly an `http`
  loopback URL on the engine's own), which no row says.
- **Least change:** Add a row "the engine's address: the URL at which a consumer or a run reaches
  the engine's server, given to a run as `CONDUIT_URL`; an `https` URL wherever a request
  crosses machines, and possibly an `http` URL on loopback for a run on the machine the engine
  runs on (IFC-042, DL13)", and qualify the row `address` as a package's or a project's.

## 5-2-08

- **Severity:** misleading
- **Promises:** OBJ-024, PRV-022, MAN-018, IFC-029
- **Quotes:** `definitions.md`: "names a version assigns | The three names that a version of this
  specification states in names.yaml ... Only a new version may change them ... The same file
  also fixes the names of what `identity` carries". OBJ-024: "It states its names in
  `names.yaml`: the address of the core package, the value of `apiVersion`, and the prefix of
  the keys". `names.yaml`: "The names this version of the specification assigns. This file is
  the one place they are defined." README, Contents: "The names this version assigns: the core
  package's address, `apiVersion`, the prefix of its `_meta` keys, and the names of what
  `identity` carries".
- **Situation:** One phrase names three names in `definitions.md` and OBJ-024, and eleven in
  `names.yaml` and the README. So whether "only a new version may change them" covers
  `CONDUIT_URL`, `ca.crt` and the rest is unsaid, although PRV-022 rests on their being fixed
  "so that a harness or a gate written once works under every engine".
- **Least change:** Either widen OBJ-024 and the row to the names of what `identity` carries,
  with "only a new version may change them", or have `names.yaml` and the README say "the names a
  version assigns, and the names of what `identity` carries".

## 5-2-09

- **Severity:** misleading
- **Promises:** LIF-009, LIF-031, OBJ-003, REC-002, IFC-006, IFC-020
- **Quotes:** `schemas/README.md`, "Choices the schemas encode": "`nested` in a result names the
  instance that holds the nested instantiation and its name, as `register-replacement.v1` takes
  them"; and "A record's `data` holds the fields of REC-002 under the names that IFC-006 gives
  Conduit's keys, with `consumer`, `target`, `outcome` and `reason` beside them". `object.v1`:
  `nested.instance: {$ref: ...#id.v1}`. `register-replacement.v1`: `instance: {$ref:
  ...#reference.v1}`. LIF-031: "A request that an actor makes on its own behalf names an object
  by a reference, and never by its ID."
- **Situation:** An actor that copies `nested.instance` from a result into
  `register-replacement.v1`, "as it takes them", sends an ID, and the request reports invalid
  under LIF-031. The record row leaves out `serving`, which is IFC-020's key and which IFC-006
  never sends, and `inputs` and `outputs`.
- **Least change:** "`nested` gives the ID of the instance that holds the nested instantiation
  and the instantiation's name; an actor names that instance in `register-replacement.v1` by
  its reference"; and add `serving`, `inputs` and `outputs` to the record row.

## 5-2-10

- **Severity:** minor
- **Promises:** PRV-005, IFC-026, PRV-022, SEC-010, IFC-044, PRV-028, MAN-017, CTR-012
- **Quotes:** `definitions.md`: "result ... and \"answer\" only as a verb"; "entry ... In the
  promises the word is used only for this"; "chain ... The promises use the word for nothing
  else"; "plain ... The word is used for nothing else"; "object ... A JSON object is called a
  map: `object` is otherwise only the JSON Schema type". Against them: PRV-005 "and an answer
  under IFC-008"; IFC-026 "the form of the answer"; PRV-022 "as a host entry or a tunnel";
  SEC-010 "A chain of such credentials ends at a secret Resource"; IFC-044 "which plain `http`
  cannot prove"; PRV-028 "one over plain `http` on loopback"; MAN-017 "admits an object or an
  array"; CTR-012 "a result that is not an object".
- **Situation:** Each word that `definitions.md` reserves to one sense is used in another, so
  the claims of exclusivity, on which a reader relies to read the word with no context, are
  false.
- **Least change:** Reword the promises: "the error it gives under IFC-008", "a line in a hosts
  file", "A sequence of such credentials", "unencrypted `http`", "admits a map or a list", "a
  result that is not a map"; or drop the claims of exclusivity.

## 5-2-11

- **Severity:** minor
- **Promises:** SEC-006, SEC-022, SEC-020
- **Quotes:** `definitions.md`: "store | The system outside the engine that holds a secret's
  value"; "secret store | Not used. ... an implementation of `get-secret.v1` for what reads the
  store". SEC-006: "a store reached by `remote` that cannot check an enforcement point's proof is
  used as plain". `core/capabilities.yaml`: "A store reached by `remote` that cannot check that
  proof is marked `plain`". SEC-022: "A store package that lists or writes secrets".
- **Situation:** A store is outside the engine and is never "reached by `remote`" or "marked
  `plain`": those are an implementation item. The word names both the system and the server
  that reads it, which the row "secret store" set out to stop.
- **Least change:** "an implementation of `get-secret.v1` reached by `remote` that cannot check
  ... is used as plain", in SEC-006 and in the description of `secret-provider.v1`.

## 5-2-12

- **Severity:** minor
- **Promises:** SEC-006, SEC-012, PRV-012, PRV-025, PKG-029, OPS-005
- **Quotes:** `definitions.md`: "refused | The request is not permitted, whoever asks and whatever
  the cause"; and in the row `install`: "PKG-029 refuses, with conflict, a package". SEC-006: "a
  refusal by the store gives the outcome failed". SEC-012: "where the store refuses". PRV-012:
  "and otherwise refuses it here" (the run ends failed).
- **Situation:** "Refuse" is one of the seven words for how a request ends, and here it also
  names a store's or a provider's declining, which ends in the outcome failed, and, in the row
  `install`, a request that reports conflict.
- **Least change:** "a store that declines", "and otherwise does not run it", and "PKG-029 has a
  request to install ... report conflict".

## 5-2-13

- **Severity:** minor
- **Promises:** SEC-013, SEC-023, IFC-012, IFC-037, OPS-027, PRV-018, IFC-041
- **Quotes:** `definitions.md`: "connection | The Model Context Protocol's connection of a client
  to a server ... The connection to an item reached by `remote` ... is what the promises mean
  where they speak of a connection with no more said ... It is the engine's holding of those
  headers' values". SEC-013: "a connection is the engine's holding of these values, and an HTTP
  request that fails does not end it". OPS-027: "A connection over Streamable HTTP that the
  engine sees drop ... closes its stream". IFC-012: "It depends neither on the connection nor on
  what was sent on it before ... on whatever connection it arrives". PRV-018: "A connection from
  the server at the `url` that a run returned".
- **Situation:** The word names a transport connection, which drops, and the engine's holding of
  header values, which outlives failed requests and restarts. IFC-012 and PRV-018 use it with no
  more said for the transport, which the row says means the second. IFC-037's "to an item
  reached by `remote` at each connection" can be read either way.
- **Least change:** Give the second sense its own name (such as "the held connection" of a
  `remote` item) and write "transport connection" in IFC-012 and PRV-018.

## 5-2-14

- **Severity:** minor
- **Promises:** SUB-006, SUB-009, LIF-009, PLC-026, OBJ-023
- **Quotes:** `definitions.md`: "register, registration | The one operation that takes
  documents, `register.v1` of the core package." LIF-009: "The request is a registration" (of
  `register-replacement.v1`, which takes no document). PLC-026: "The engine registers a node for
  it". `replace.v1`: input `template: {$ref: ...#template.v1}`.
- **Situation:** `replace.v1` also takes a document, so "the one operation that takes documents"
  is false; and "registration" also names a request for `register-replacement.v1` and the
  engine's own making of a node with no operation.
- **Least change:** "The operation that takes a submission"; and add to the row that a request
  for `register-replacement.v1` is authorized and reports as a registration does, and that the
  engine registers nodes and the first-start objects with no request.

## 5-2-15

- **Severity:** minor
- **Promises:** PRV-014, PLC-007, OBJ-022, CTR-004
- **Quotes:** `definitions.md`: "label | A key and a string value that the machine provider
  reports"; "label selector | A map from a label to a value". OBJ-022: "The keys of a label
  selector under PLC-007 are labels". PLC-007: "do not hold each label of the instance's label
  selector with the value it gives".
- **Situation:** The row makes a label the pair; the next row and the promises make it the key.
- **Least change:** "label | The key of a pair that the machine provider reports of a machine,
  with a string value".

## 5-2-16

- **Severity:** minor
- **Promises:** PRV-022, PRV-025, DL13, IFC-025, OPS-032
- **Quotes:** `definitions.md`: "ID | The stable identity of an object, which the engine gives
  it ... A credential of DL13 names the engine too, by an ID of the mechanism's, the engine's
  ID"; "the engine's ID for an invocation | The ID the engine gives every invocation". `id.v1`:
  "An ID the engine gave: of an object or of an invocation."
- **Situation:** "ID" is defined as what the engine gives an object, yet "the engine's ID" is an
  ID the mechanism gives the engine, which is no object; and "the engine's ID" and "the engine's
  ID for an invocation" differ only by a trailing phrase. A history line of
  `interface/implementation.yaml` already says "the engine's ID" for an invocation's.
- **Least change:** Call the mechanism's one "the engine's credential ID" (it is what `engine-id`
  holds), and leave "ID" for what the engine gives.

## 5-2-17

- **Severity:** minor
- **Promises:** ACC-018, IFC-006, PRV-005, TPL-018
- **Quotes:** `setup-hook.v1` and `retire-hook.v1`: "the implementation learns its instance's
  configuration from the request". `definitions.md`, hook: "A hook is not a request"; and
  configuration: "with every request to an implementation that carries Conduit's keys
  (IFC-006)". `definitions.md`, provider call: "The engine's invocation of an operation";
  invocation: "A provider call begins none". Hook row: "within a retirement of an instance that
  was ever set up"; TPL-018: "for which a setup was ever dispatched".
- **Situation:** Defined words used against their own rows: a hook's call is called a request,
  so the configuration row understates IFC-006, which covers "a call for a request or a hook";
  a provider call is called an invocation; and "was ever set up" reads as a setup that
  succeeded, where TPL-018 needs only one dispatched.
- **Least change:** "from its call"; "with every call for a request or a hook"; "The engine's
  call of an operation"; "for which a setup was ever dispatched".

## 5-2-18

- **Severity:** minor
- **Promises:** LIF-004, LIF-005, CON-011, PLC-017, PLC-024, LIF-013
- **Quotes:** `definitions.md`: "status | The part of an inspection result that the engine
  writes: the lifecycle state, the shutdown of a retired instance, the conditions, and
  `unreconciled` ... for a node, also the labels". LIF-004: "its status reports how that setup
  ended". LIF-005: "reports the state retired and what retired it". CON-011: "its status reports
  that the Conduit is closed". PLC-017: "states that it could not be confirmed because the node
  was declared lost".
- **Situation:** The row reads as the whole of a status, and leaves out four things that
  promises put there.
- **Least change:** Add "how its latest setup ended, what retired it, the Conduits closed while
  it is ready, and why its shutdown is unknown (LIF-004, LIF-005, CON-011, PLC-017, PLC-024)".

## 5-2-19

- **Severity:** minor
- **Promises:** PRV-022
- **Quotes:** `names.yaml`: "trustBundleFile: ca.crt" and "jwtBundleFile: jwks.json".
  `definitions.md`: "trust bundle | The files `ca.crt` and `jwks.json` of `identity`".
- **Situation:** The one place that defines the names calls `ca.crt` alone the trust-bundle file,
  where the term is both files; `tools/set_names.py --identity trustBundleFile=...` then renames
  half of what the term names.
- **Least change:** Rename the key, such as `rootCertificatesFile`, or say in its comment that the
  trust bundle is the two files.

## 5-2-20

- **Severity:** minor
- **Promises:** none (head comments)
- **Quotes:** `promises/placement.yaml`: "Since ruling 41, a location stands for a machine that the
  machines provider reports". `promises/providers.yaml`: "and `secret-reveal.v1`, offered by a
  secret Resource ... The engine's own runner, its own machine and its own fetching"; "its
  well-known labels for the machines provider".
- **Situation:** These sentences describe the present, not a past pass, and use five names that
  `definitions.md` lists as renamed. The row "secret store" excuses only lines "that record
  earlier passes".
- **Least change:** Write node, represents, the machine provider, `secret-provider.v1` and the
  engine's own machine provider in those sentences.

## 5-2-21

- **Severity:** minor
- **Promises:** IFC-044
- **Quotes:** `interface/implementation.yaml` head comment: "Promises here have the layer
  `interface`, but IFC-014 and IFC-043, rules on documents, have the layer `language`." IFC-044:
  "layer: language".
- **Situation:** The comment omits IFC-044, so it undercounts the file's `language` promises
  (three).
- **Least change:** "IFC-014, IFC-043 and IFC-044".
