# The ruling on the core package and the schemas — log of the promise side

Written 2026-11-05 by Editor P, under [APPLY.md](APPLY.md), the [record](README.md) and the
rules of the [consolidation instructions](../reaudit/CONSOLIDATION.md). Editor P's files are
every promise file under `spec/promises/` except `manifest.yaml`, every file under
`spec/conventions/` and `spec/interface/`, `spec/implementation-defined.yaml`,
`spec/schema-profile.md`, `spec/definitions.md` and `spec/README.md`. No other file was edited,
and nothing was committed. `spec/core/`, `spec/schemas/`, `spec/promises/manifest.yaml` and
`tools/check_schemas.py` belong to Editor S and were only read.

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

"Item" names the row of APPLY.md, or the choice (C), name (N), point (P) or disagreement (D) of
the record, that the change applies. "(b)" marks a change made only so that a promise agrees
with the accepted batch, as the brief's part (b) asks.

## Counts

| File | Before | Changed | Removed | Added | After |
| --- | --- | --- | --- | --- | --- |
| `promises/objects.yaml` | 24 | 8 | 0 | 0 | 24 |
| `promises/schemas.yaml` | 18 | 2 | 0 | 0 | 18 |
| `promises/instantiation.yaml` | 22 | 3 | 0 | 0 | 22 |
| `promises/templates.yaml` | 12 | 4 | 0 | 0 | 12 |
| `promises/contracts.yaml` | 8 | 5 | 0 | 1 | 9 |
| `promises/naming.yaml` | 26 | 8 | 0 | 1 | 27 |
| `promises/packages.yaml` | 16 | 7 | 0 | 3 | 19 |
| `promises/submissions.yaml` | 8 | 2 | 0 | 0 | 8 |
| `promises/pools.yaml` | 14 | 2 | 0 | 0 | 14 |
| `promises/placement.yaml` | 17 | 2 | 0 | 0 | 17 |
| `promises/conduits.yaml` | 23 | 5 | 0 | 1 | 24 |
| `promises/graphs.yaml` | 22 | 1 | 0 | 0 | 22 |
| `promises/first-start.yaml` | 4 | 2 | 0 | 0 | 4 |
| `conventions/lifecycle.yaml` | 26 | 4 | 0 | 0 | 26 |
| `conventions/access.yaml` | 28 | 5 | 0 | 0 | 28 |
| `conventions/operations.yaml` | 24 | 5 | 0 | 1 | 25 |
| `conventions/revisions.yaml` | 13 | 5 | 0 | 0 | 13 |
| `conventions/records.yaml` | 6 | 1 | 0 | 0 | 6 |
| `conventions/observation.yaml` | 6 | 0 | 0 | 0 | 6 |
| `conventions/implicit.yaml` | 9 | 0 | 0 | 0 | 9 |
| `interface/implementation.yaml` | 29 | 9 | 0 | 0 | 29 |
| Total of Editor P's files | 355 | 80 | 0 | 7 | 362 |

With the 14 promises of `promises/manifest.yaml` the specification holds 376 promises.

New IDs: CTR-009, NAM-027, PKG-023, PKG-024, PKG-025, CON-028 and OPS-025. Each is the next
number never used in its prefix. No ID was removed, reused or moved between files.

## Where a row of APPLY.md named two homes

| Item | Home chosen | Why |
| --- | --- | --- |
| X4 | A new promise, CTR-009 | CTR-003 is about a Capability contract. The rule is about an Operation contract, and about a project as well as a package. CTR-002 now lists the key `reservedToRoot`, and CTR-003 says what a reserved operation is |
| P9 | IFC-009 | It is the promise that says which results are malformed and so unknown. IFC-027 is about a result that holds a reason alone; its `when` now says so, and OPS-007 and IFC-026 agree |
| D16 | INS-006 | The rule is about what a destination receives, which INS-006 states. OBJ-006 gained one sentence that points there |
| D28 | SUB-005 | SUB-005 says what a submission may contain. SUB-001 is about order |
| P5 | Three promises, PKG-023 to PKG-025 | APPLY.md asks for one new promise. Rule 6 of the consolidation instructions asks that a promise about a request that is not dispatched name exactly one reason, so each of invalid, unavailable and conflict has its own |

## objects.yaml

| ID | What changed | Item |
| --- | --- | --- |
| OBJ-003 | Observable: a component Conduit's name is given by CON-003, which keeps it within 63 characters | P7 |
| OBJ-006 | Observable: points to INS-006 for a bare name given to an input | D16 |
| OBJ-013 | `metadata` and `arguments` may be left out of an instance document | D4, D5 |
| OBJ-014 | `arguments` may be left out of each of the three nested instantiations | D4 |
| OBJ-018 | Observable: an instance document may leave out `metadata` altogether | D5 |
| OBJ-019 | Inside a schema only the names are open, and its keywords are closed by SCH-002; a key of an `implementation`, `controls` or `tools` map is a selection, `#name` included | D6, D9 |
| OBJ-021 | A reference schema governs a place directly, as the `items` of a list, or as a branch of `anyOf` or `oneOf` | D14 |
| OBJ-023 | Observable: the diagnostics are in the `diagnostics` of the result's `structuredContent` | D21 |

## schemas.yaml

| ID | What changed | Item |
| --- | --- | --- |
| SCH-008 | Whether a `default` is allowed is judged where it is written | D26 |
| SCH-015 | A `$ref` has three forms; `#name` reaches a Schema contract of the same project or package | P3 |

## instantiation.yaml

| ID | What changed | Item |
| --- | --- | --- |
| INS-006 | Observable: a bare name is accepted where the input's schema fixes the kind, and the destination receives the reference with its kind | D16 |
| INS-012 | Rewritten as a closed list of accepted destinations, with the whole of `config` and of each `arguments` map; the rejected places are examples | P4 |
| INS-014 | Observable: a template with no `config` is checked as an empty map | D17 |

## templates.yaml

| ID | What changed | Item |
| --- | --- | --- |
| TPL-002 | `type` is a selection | D9, D20 (b) |
| TPL-003 | Each key is the selection of an operation, `#name` included; each string value is the selection of an implementation | D9 |
| TPL-006 | `start` and `stop` are selections | D9, D20 (b) |
| TPL-010 | Each key is a selection; an input is narrowed as `inputs: {<name>: {schema: <schema>}}` | D9, C10 |

## contracts.yaml

| ID | What changed | Item |
| --- | --- | --- |
| CTR-001 | The version segment is `v` and a whole number from 1, and at least one segment comes before it | D27, C5 |
| CTR-002 | The default implementation is one selection; the `spec` may hold `reservedToRoot` | D20, X4, C19 |
| CTR-003 | `operations` is a list of selections; a reserved operation is one marked `reservedToRoot: true` | D9, X4 |
| CTR-004 | `controls` is a list of selections | D9, D20 (b) |
| CTR-008 | Observable: `#name` in a `$ref`; the schemas of documents are Schema contracts | P3, C1 |
| CTR-009 | **New.** An Operation contract outside the core package marked `reservedToRoot: true` is rejected, on installation or on submission | X4 |

## naming.yaml

| ID | What changed | Item |
| --- | --- | --- |
| NAM-001 | No trailing dot; a label of at most 63 characters; a last label of digits is not a host name; `xn--` labels; an address may have no path and no segment is empty | P6, C6 |
| NAM-006 | Observable: cites MAN-001 to MAN-010 for the file and its keys | (b) |
| NAM-009 | For the core package the address and version alone; the unpublished package's digest is the one SUB-008 defines | D2, P1 |
| NAM-012 | Observable: cites MAN-006 for the value of `fetch` | (b) |
| NAM-013 | An export is also reached as `#name` from a document of the package itself | D18 (b) |
| NAM-018 | Observable: lists the places where both forms of a selection are accepted | D9, D20 |
| NAM-019 | `requirement` is a selection; `arguments` may be left out | D9, D4 |
| NAM-022 | Observable: a document refers to its own project or package as `#name` and never by its own address; the core package's documents are the exception | D18 |
| NAM-027 | **New.** Within one project or package no two documents share a name, whatever their kinds | X5 |

## packages.yaml

| ID | What changed | Item |
| --- | --- | --- |
| PKG-005 | Observable: the core package is listed by address and version alone | D2 |
| PKG-013 | Observable: cites MAN-008 | (b) |
| PKG-017 | Observable: cites MAN-004 | (b) |
| PKG-018 | Observable: cites MAN-007 | (b) |
| PKG-019 | `supports` and `placement` may be left out, and what that means; cites MAN-009, MAN-010 and the exception of MAN-012 | D24, D1 |
| PKG-020 | The keys of `tools` are selections | D9 |
| PKG-022 | Observable: cites MAN-013 and MAN-014 | (b) |
| PKG-023 | **New.** A request to install a package that is rejected is not dispatched and reports invalid | P5 |
| PKG-024 | **New.** A request to install a package whose archive cannot be fetched now reports unavailable | P5 |
| PKG-025 | **New.** A request to install at an address and version already installed with another digest reports conflict | P5 |

## submissions.yaml

| ID | What changed | Item |
| --- | --- | --- |
| SUB-005 | Observable: a submitted directory is the `.yaml` and `.json` files in the directory itself | D28 |
| SUB-008 | The digest of an unpublished package is the SHA-256 of the RFC 8785 form of one object with `documents` and `manifest` | P1 |

## pools.yaml, placement.yaml, conduits.yaml, graphs.yaml, first-start.yaml

| ID | What changed | Item |
| --- | --- | --- |
| POOL-005 | Observable: `{entry: name}` anywhere other than in an entry of a Pool template is rejected | D15 |
| POOL-012 | `arguments` is optional in an entry | D4 |
| PLC-008 | The facts are held under `facts`; after instantiation only the control `set-facts.v1` changes them, in the capability `location-settings.v1`; a location's document that states a placement is rejected | X1, D31, C29 |
| PLC-016 | Observable: names `set-default-location.v1` and `system-settings.v1`; the value is a reference or null | C26, C40 (b) |
| CON-003 | Observable: a component Conduit's name is never longer than 63 characters; a generated part is shortened, and a given name that makes it longer is rejected | P7 |
| CON-016 | `capability` is required of a template that is not composed; `operations` unless an input fills it; `requirements` and `inputs` may be left out | D25 |
| CON-017 | `capability` and `operations` are selections | D9, D20 (b) |
| CON-018 | `requirement` is a selection; `arguments` may be left out; names `shared-host.v1` and points to ACC-029 for the bound | D4, D9, N31 |
| CON-026 | A rename that would make a tool name longer than 128 characters reports conflict | P8 |
| CON-028 | **New.** A Conduit that applies `bound.v1` and whose target is not `access`, or that applies it more than once, is not instantiated | X3 |
| GRA-017 | A step's `operation` is a selection | D9, D20 (b) |
| FST-002 | Observable: the two Conduits are the declared Conduits `management` and `access` of the template `root` | C28 |
| FST-005 | Observable: names the seven templates | N32 to N38 |

## lifecycle.yaml, access.yaml

| ID | What changed | Item |
| --- | --- | --- |
| LIF-007 | No longer "by its ID": a request names an object by a reference, and a retired instance's name still resolves until it is unregistered | X2 |
| LIF-009 | The request is `register-replacement.v1` and names the retired instance | D8, C37 |
| LIF-013 | Observable: the three parts are the outputs `declared`, `status` and `records` of `inspect.v1`, beside `id`, `revision` and `packages` | C38 (b) |
| LIF-015 | Observable: the request of LIF-009 is among those that address a retired instance and can be dispatched | C37 (b) |
| ACC-029 | Observable: the bound is the requirement `bound.v1` with `pools`, `targets` and `operations`; a Conduit to `access` that applies none has no bound | C32, N30 |
| ACC-031 | A policy is not configuration; applying returns an ID, removing names it, neither names a revision; inspecting `access` lists the policies in its status | D12 |
| ACC-032 | Observable: an operation is reserved where its contract is marked `reservedToRoot: true` | X4 |
| ACC-033 | Registering a Resource into a Pool that exists needs a Conduit to `management` and one to the Pool, each listing `register.v1`; a replacement needs the same for `register-replacement.v1` | D8 |
| ACC-034 | Observable: withheld content means the outcome, no `structuredContent`, and `io.github.shrav-k.conduit/withheld` | P13 |

## operations.yaml, revisions.yaml, records.yaml

| ID | What changed | Item |
| --- | --- | --- |
| OPS-007 | A result that states both an outcome and a reason is unknown | P9 |
| OPS-009 | Observable: the answer to a repeat of an invocation that has not ended carries `io.github.shrav-k.conduit/state` with `inProgress` | P12 |
| OPS-014 | Observable: the request is `reconcile.v1`, which names the invocation by the engine's ID | C39 (b) |
| OPS-016 | The caller asks by operation identity and target, with `retrieve-result.v1` | D22, C24 |
| OPS-019 | Observable: names the three capabilities of the core package and some of its operations | N1 to N22 (b) |
| OPS-025 | **New.** A repeat of a request that ended with a reason is a new request, decided afresh | P10 |
| REV-006 | `revision` is the one input of a control that must always be supplied | D13 |
| REV-007 | The confirmed settings are in the output `settings` | D13 |
| REV-010 | Observable: in `reconcile.v1` they are the inputs `revision` and `settings` | C39 (b) |
| REV-011 | Observable: the revision is named in the input `revision` | C20 (b) |
| REV-012 | One setup, retirement or change of an instance at a time; setting up and retiring name no revision and advance none | P14 |
| REC-004 | A purge removes the records made before a time the request names; how the digest is computed is DL2 | D23, P2 |

## interface/implementation.yaml

| ID | What changed | Item |
| --- | --- | --- |
| IFC-009 | Observable: a result that holds both keys is unknown | P9 |
| IFC-012 | Observable: a tool name ends in the operation's version segment, as in `checkout.directory.read.v1` | D3 |
| IFC-014 | Observable: CON-026 also covers a rename that makes a tool name too long | P8 |
| IFC-016 | Observable: a retained result is found by operation identity and target | D22 |
| IFC-019 | Observable: the restriction is named in `io.github.shrav-k.conduit/restriction` | P11 |
| IFC-024 | The tool is `retrieve-result.v1`, named by its export name alone; its inputs are `operationIdentity` and `target`; its result holds `state`, `withheld` and `outputs` | D22, C24, C25 |
| IFC-025 | `io.github.shrav-k.conduit/state` for a repeat of an invocation that has not ended; `io.github.shrav-k.conduit/withheld` where content is withheld; a result that ended with a reason may carry `diagnostics` | P12, P13, D21 |
| IFC-026 | The answer holds `io.github.shrav-k.conduit/restriction` and its `isError` is true | P11 |
| IFC-027 | `when`: the result holds a reason and no outcome | P9 |

## The other files

| File | What changed | Item |
| --- | --- | --- |
| `implementation-defined.yaml` | DL2 gains "how the digest of an invocation's contents is computed", and names RFC 8785 over the operation and its arguments as one way. DL7 gains the schemes of `fetch`, which MAN-006 leaves to it. DL9 covers a setup and a retirement as well as a change, and is entitled "Controls and busy instances" | P2, C47, P14 |
| `schema-profile.md` | The standards table gains RFC 8785, RFC 3986 for `fetch` and an implementation's `url`, and RFC 5890; the row for the host part follows NAM-001; a `$ref` has three forms; a schema is an object or a boolean; a pointer to `schema.v1` | P1, P3, P6, C3, C47 |
| `definitions.md` | Interface, Agent, Resource, Pool, model work, mediated, enforced, project and published take the accepted meanings. New rows: published, `set-facts.v1`, reserved to the root Interface, retrieval tool, `fetch`. Corrected rows: kind, document, submission, name, instance, nested instantiation, argument, destination, rejected, register, install, ID, reference, qualified reference, `#name`, entry reference, reference schema, location type, location fact, default location, shared-host requirement, first-start object, `management`, `access`, bound, applied requirement, policy, operation, repeat, in progress, retained result, withheld, reconcile, implementation, Conduit's keys, tool, tool name, hold, configuration, control, revision, package, address, manifest, export, re-export, core package, unpublished package, selection, digest, contract, Schema contract, declare lost, inspect and purge; and the paragraph under "How a request ends" | The accepted definitions; part (c) of the brief |
| `README.md` | Rows for `promises/manifest.yaml`, `core/` and its eight files, `schemas/` and its nine files, and the three directories of examples; every count recounted; "Still to come" is the conformance vectors and a reference model; a section on checking with the three tools; the reading order gains the manifest's promises and the core package | Part (d) of the brief |

## Left undone, and why

Each of these needs a decision that APPLY.md and the record do not hold, so the text was left
and nothing was chosen.

1. **OBJ-019 against `template-input.v1` and `requirement-spec.v1`.** OBJ-019 exempts "a key
   inside `config`, `annotations` or a literal value". The schemas leave the value of an
   input's `default` open, and `spec/schemas/README.md` lists "the value of `literal`, and of
   an input's `default`" as open "exactly where OBJ-019 exempts". Read strictly, OBJ-019
   rejects a key inside a `default` that is a map.
2. **IFC-006 and IFC-018 against SUB-008.** On the wire an operation is given "by its qualified
   reference", and a restriction holds "the qualified reference of a requirement". A contract
   of a project that is not published has no address, so it has no qualified reference.
3. **LIF-017 against `unregister.v1`.** LIF-017 lets an instance be unregistered when "its
   shutdown is confirmed or it was retired because its location was declared lost". The
   description of `unregister.v1` in `spec/core/operations.yaml` says "an instance that is
   retired and whose shutdown is confirmed".
4. **SUB-008, the order of `documents`.** "Ordered by `kind` and then by `metadata.name`" does
   not say how two kinds or two names are compared: by code point, or kinds in the order
   OBJ-020 lists them. Two engines could compute different digests.
5. **CON-003, a name that cannot fit.** Where components are nested, the component names alone
   can pass 63 characters, so shortening the generated part cannot make the whole fit. P7 says
   a name is never longer; no promise says that such a Conduit is not instantiated.
6. **INS-014 and INS-011, SCH-009.** A template with no `config` is checked as an empty map.
   Not said: whether an input may then have a destination inside `config`, whose parent
   INS-011 requires to be present, and whether the defaults of the type's schema then apply,
   since SCH-009 does not make an absent object.
7. **SCH-008 and SCH-009.** A `default` is judged where it is written. A property that states
   one inside a member of `$defs` is accepted there, and may be reached only through a `$ref`
   inside `anyOf`. Whether SCH-009 applies that default is not said.
8. **OPS-016, a target that is unregistered.** `retrieve-result.v1` names the target by a
   reference, and a reference to an unregistered object no longer resolves (D22 names this
   and the batch does not settle it).
9. **IFC-025, the restriction in the engine's answer.** IFC-026 puts
   `io.github.shrav-k.conduit/restriction` in the implementation's answer. No promise says
   whether the engine's own answer to a consumer carries it, though ACC-015 says that answer
   "names the policy as the cause".
10. **"Isolation plugin".** The accepted definition of enforced uses the words, and no promise
    or definition says what an isolation plugin is. The row was written as accepted.
11. **A control's `settings`.** REV-007 now says the confirmed settings are in the output
    `settings`. Its shape is "an object" in the core package, and no promise says how the
    object names a setting, or that a control's contract must declare `revision` and
    `settings`. CTR-006 was left as it is.
12. **Choices of the record that no promise states** and that contradict none were not written
    into promises: C8 (`optional: false`), C9 (a Requirement's input), C11 and C12 (a Type's
    `family`, `conditions` and lists), C13 (the forms of `from`), C15 (the form of an ID and
    of a revision) and C16. The schemas carry them. `plain: false` is stated by MAN-010.
13. **NAM-022 and implementations.** NAM-022 says that `#name` refers to "the document of that
    name", and NAM-018 lets the selection of an implementation be written so. An
    implementation is not a document, and NAM-027 and PKG-012 do not keep its name apart from
    the name of a document its package does not export. Editor S found this too and records it.
14. **`spec/README.md` against `AGENTS.md`.** The status paragraph of `spec/README.md` still
    says that "accepted behavior that the promises do not yet cover still lives in"
    `docs/protocol`. `AGENTS.md`, which another editor changed during this pass, now says that
    nothing under `docs/protocol/` binds and that a behavior missing from `spec/` is a gap in
    `spec/`. The sentence is older than this pass and APPLY.md does not touch it.

## Checks

Run from the repository's root after the last edit to Editor P's files.

- The validation command of the consolidation instructions, over every file under
  `spec/promises/`, `spec/conventions/` and `spec/interface/`, `promises/manifest.yaml`
  included: `376 promises ok`.
- Every promise ID and every entry of the implementation-defined list cited in Editor P's
  files and in this log exists: 0 missing. A removed promise is named only in the header
  comment of its file.
- No line of a YAML file of Editor P's is longer than 96 characters, and `git diff --check`
  reports nothing for them.
- `python3 tools/set_names.py --check`: `Errors: 0`.
- `python3 tools/check_schemas.py`, which ran itself again under `/usr/bin/python3`:
  `Errors: 0`, with 98 exports and 1 implementation in the core package, 50 invalid documents
  and 8 sets under `invalid-by-rule` each rejected.
- `python3 tools/check_docs.py`: `Checked 104 Markdown files and 1309 local links`, with 21
  errors. Every one is a link to `experiments/001-shared-access/`, a directory that another
  editor removed while this pass was running; the links are in `docs/VOCABULARY.md`, three
  files under `docs/archive/` and `experiments/README.md`. None is in a file of Editor P's
  or of Editor S's, and no link in `spec/` or in this directory is reported. The run before
  that removal reported no error in any of Editor P's files.
