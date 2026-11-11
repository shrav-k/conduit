# The core package, the schemas of documents and the manifest's format

Status: in force; amended by 38 (points 10, 12); amended by 41 (items 6c, 8, 9, 24, C14, C21); amended by 44 (D06); amended by 55 (A6) (ruled by the owner on 2026-11-05). Holds now: [IN_FORCE.md: 2. The specification: form, layers and conformance](../../../IN_FORCE.md#2-the-specification-form-layers-and-conformance), [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas); [CTR-001 to CTR-012](../../../spec/promises/contracts.yaml), [LIF-031, LIF-037](../../../spec/conventions/lifecycle.yaml). This record says why.

Every choice C1 to C56,
every name N1 to N42 and every recommendation P1 to P14 below was accepted as written, with
the five changes X1 to X5 and the settlements of the open points and disagreements that
[APPLY.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/APPLY.md) lists. The section [Outcome](#outcome) says, for each item, how it was
settled and what it changed in the documents, and [log-documents.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-documents.md) logs
what was done to each file of the batch after the ruling.

This is the record of a ruling. Where it and the specification differ, the specification
governs. Changed since by the coordinator, under ruling 37 and not ruled on by the owner: the
owner's change X1 wrote a control's `changes` as a list of pointers, and it is now a map from an
input's name to a pointer (round 2 of the re-audit); requests name objects by reference only,
and a step or an onward request may now hold the resolved form (round 3); the core package
gained `start-later-work.v1`, and `reconcile.v1`, `apply-policy.v1` and `retrieve-result.v1`
gained inputs or outputs (rounds 2 and 3). The choices made while the ruling was applied, in
the section Outcome, are for the owner to strike or keep.
Accepted by [ruling 58 (point 1)](../ruling-58/README.md): the choices in Outcome, including "Choices made while applying the ruling".

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

[Sections (a) to (e), and the section after them](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md), are kept as they were proposed. They were
drafted on 2026-11-05 for decision G1 of the [re-audit](../reaudit/README.md), which asked for
three things to be written as normative parts of `spec/` and brought to the owner as one
batch. They describe the batch before the ruling: where they say "proposed", "recommendation"
or "guess", or give a count, read [Outcome](#outcome) for what holds now.

Two rules governed the drafting. Existing paradigms first: for anything that is not what
distinguishes Conduit, an existing standard or practice is adopted and named. And the promises
are the source of truth: where a promise leaves something open, the smallest choice is made and
listed in (b); where promises disagree, cannot be expressed or forced a guess, it is listed in
[(d)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) and not settled silently.

## (b) Choices that no promise states

Each row is a choice this batch made because no promise states it. "Follows" names the existing
practice.

### The schemas

| ID | Choice | Follows | Alternative |
| --- | --- | --- | --- |
| C1 | Every schema is a Schema contract that the core package exports, and one schema refers to another by a qualified reference | The specification's own rule, SCH-015 and CTR-008; a Kubernetes resource definition is itself a resource | One bundled schema file that uses only `#/$defs`, as the Compose specification publishes one file. It would sit outside the contract system, and an operation could not state an input with it |
| C2 | The schemas of documents are in `spec/schemas/` and the rest of the core package in `spec/core/`, although both are one package | The brief for this batch | Move them to `spec/core/schemas/`, so the package is one directory tree |
| C3 | A schema is an object or a boolean | JSON Schema 2020-12 | Objects only |
| C4 | `schema.v1` enforces SCH-008 by where `default` is written: it has one definition for a schema, one for a member of `properties`, and one for anything inside a combiner | Kubernetes structural schemas | Leave SCH-008 wholly to the engine |
| C5 | A contract's version segment is `v` and a whole number from 1, such as `v1` | Go's `/v2`; Kubernetes `v1` | Also accept `v1alpha1` and `v1beta1`, as Kubernetes does |
| C6 | An address may have no path, and a path segment is not empty: `example.org` and `example.org/a` are accepted, `example.org/` and `example.org//a` are not | Go module paths | Require at least one path segment |
| C7 | An instance document may leave out `metadata`, and any `arguments` map may be left out and then means no arguments | Kubernetes and Compose let an empty map be left out | Require both. [See D4 and D5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C8 | `optional` and `plain` are booleans, and `false` means the same as leaving the key out | CTR-002 says this of `startsModelWork` and `deduplicates` | Accept only `true` |
| C9 | A Requirement's input holds `schema` and may hold `default` and `optional` | A template's input without `to`; NAM-019 checks arguments "exactly as in instantiation" | `schema` only |
| C10 | A template narrows a control's input as `inputs: {<name>: {schema: <schema>}}` | The shape of every other input member | `inputs: {<name>: <schema>}` |
| C11 | A Type's `family` is `Interface`, `Agent`, `Resource` or `Pool` | TPL-002: a Conduit template has no type | Accept `Conduit` and reject it by rule |
| C12 | A Type's `conditions` is a list of distinct, non-empty strings; `offers`, `requires` and `controls` are lists of selections | CTR-004's wording | Apply the name rule of OBJ-022 to a condition |
| C13 | `from` accepts a bare name, `{kind, name}`, a qualified reference and `#name` | OBJ-006, NAM-002, NAM-022 | A bare name and a qualified reference only |
| C14 | Every reference schema that accepts a Resource also accepts `{kind: Pool, name, entry}` and `{entry: name}` | POOL-005, POOL-014 | Accept them only in `resource-reference.v1` |
| C15 | An ID is an opaque, non-empty string; a revision is a whole number from 0 | Kubernetes `uid` and `generation` | A UUID; an opaque string, as Kubernetes `resourceVersion` is |
| C16 | A diagnostic has `message` and whatever else an engine adds; a result names an object by `kind`, `name` and `id` | The smallest shape that lets a result be stated | Fix the whole format of a diagnostic now |

### The core package

| ID | Choice | Follows | Alternative |
| --- | --- | --- | --- |
| C17 | Export names: an operation is a verb, every other contract a noun, each ending in `.v1`; a template has the name of the first-start object made from it | `kubectl` and `docker` verbs; the names the promises already use | A namespace segment, such as `management.register.v1` |
| C18 | One Operation contract for one verb. `register`, `register-replacement`, `inspect`, `set-up`, `retire` and `reconcile` are in the capability of `management` and in the capability of a Pool | The smallest number of names; ACC-033 already fixes the target from the object named | Separate contracts for a Resource, such as `resource.set-up.v1`. [See D8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C19 | An operation reserved to the root Interface is marked `reservedToRoot: true` in its contract | The booleans `startsModelWork` and `deduplicates`; the Model Context Protocol's tool annotations | Mark it where a Capability lists it; or a fixed list in the specification |
| C20 | The revision a change names is an ordinary required input, `revision` | IFC-001: the arguments are exactly the inputs; Kubernetes sends `resourceVersion` in the object | A key in `_meta`, as HTTP sends `If-Match` in a header |
| C21 | A request names an object by a reference, never by an ID | `kubectl` names an object by kind and name | Also accept `{id: ...}`. [See D10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C22 | `deduplicates: true` on every operation whose whole effect is a change to what the engine stores; not on `set-up`, `retire` and `retire-with-resources`, which run other implementations' graphs | OPS-012 | Declare it nowhere, so that every unknown outcome waits for reconciliation |
| C23 | The engine is the default implementation of every core operation, exported as `engine`, and its manifest item states no `command`, `url` or `graph` | Go's standard library is built into the toolchain | No implementation item at all, with the core package excused from TPL-004 and CON-022. [See D1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C24 | The tool of IFC-024 is named by its export name alone, `retrieve-result.v1`. Its inputs are `operationIdentity` and `target`; its outputs are `state`, `withheld` and `outputs` | The Model Context Protocol's `tasks/result`; OPS-008 scopes an identity to a caller and a target | Name the invocation by the engine's ID |
| C25 | The state "in progress" is written `inProgress` | camelCase, as OBJ-019 requires of keys | `in-progress`; `in_progress` |
| C26 | A control is included in a capability, and the control of the `system` Pool is in a capability of its own, `system-settings.v1` | CON-017: a Conduit lists only operations its capability includes | Include it in `pool-management.v1` |
| C27 | The `system` Pool has its own type, `system-pool.v1`, which adds the default-location setting to the plain Pool type | PLC-003 speaks of the `system` Pool's setting | Give every Pool the setting |
| C28 | `management` and `access` are instances of their own, not entries of the `system` template; the two Conduits of `root` are declared Conduits of its template, with the local names `management` and `access` | FST-001 fixes the names, and an entry's Resource gets a generated one; CON-002 | Two Conduits instantiated on their own |
| C29 | A location's stored facts are the value of `facts` in its configuration. `host` and `localPaths` are named, and `facts` is open | Kubernetes node labels are an open map | The whole configuration is the facts; a closed list of facts |
| C30 | The plain Interface and Pool types, and the types of `management` and `access`, have an empty, closed configuration | "Plain" | An open configuration |
| C31 | A policy is stored by `access` outside its configuration. `apply-policy.v1` returns an ID, `remove-policy.v1` takes it, and neither names a revision | AWS IAM returns an identifier for a policy | A policy with a name the actor chooses; policies as configuration changed by controls. [See D12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C32 | `bound.v1` has three inputs, `pools`, `targets` and `operations`, each a list that defaults to empty. A Conduit to `access` with no bound requirement has no bound | ACC-029; a Kubernetes RBAC rule lists resources and verbs | One list of items, each with a target and its operations |
| C33 | `shared-host.v1` has no inputs and `narrows: false` | IFC-018 | — |
| C34 | `purge.v1` removes the records made before a time, `before` | Log retention by age, as `journalctl --vacuum-time` and CloudTrail retention | Purge by object, or purge everything. [See D23](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C35 | `install.v1` takes what a manifest item states: `address`, `version`, `digest` and `fetch` | `go get module@version` | Take the archive itself |
| C36 | `register.v1` takes `documents`, a list, and `manifest`; it returns `objects` and `diagnostics` | `kubectl apply` takes a list | Take files |
| C37 | `register-replacement.v1` names the retired instance, and the Conduits that authorize it list this operation | LIF-009 | Name the parent and the member; let `register.v1` on the Pool's Conduit suffice. [See D8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) |
| C38 | `inspect.v1` returns `id`, `revision`, `packages`, `declared`, `status` and `records`; a status has `state` and may have `shutdown` | LIF-013; Kubernetes `spec` and `status` | Fix the whole shape now |
| C39 | `reconcile.v1` names the invocation by the engine's ID, and takes `occurred`, `checked`, and for a control `revision` and `settings` | OPS-014, REV-010, IFC-006 | Name it by caller, target and operation identity |
| C40 | `set-default-location.v1` takes `location`, a reference to a Resource or null, which may be left out, and returns `settings` | REV-006, REV-007 | A second control that clears the setting |
| C41 | The core package's version is `0.1.0` | Semantic Versioning: a major version of 0 for something not yet stable | `0.0.0` |
| C42 | A core type lists the operations of its capabilities under `requires`, and the default implementation is stated by each Operation contract | TPL-004 | State the defaults in each Type's `implementation` map |

### The manifest

| ID | Choice | Follows | Alternative | Promise |
| --- | --- | --- | --- | --- |
| C43 | The manifest is the file `conduit.yaml`, or `conduit.json` | `go.mod`, `Cargo.toml`, `compose.yaml`; the name the naming record used as an illustration | `conduit.mod.yaml`; `manifest.yaml` | MAN-001, MAN-002 |
| C44 | It carries `apiVersion`, and no `kind`, `metadata` or `spec` | A Helm chart's `Chart.yaml` | An eleventh kind, `Manifest`; no version at all, as `package.json` | MAN-003 |
| C45 | `exports` and `implementations` may appear in the manifest of a project that is not published | A project is a package before it is published | Allow them only beside `address` and `version` | MAN-004 |
| C46 | `packages` is a map keyed by address | Cargo's `[dependencies]`; `package.json` | A list, as `go.mod` writes `require` lines | MAN-005 |
| C47 | `fetch` is a URI reference to the archive file; a relative one is resolved against the manifest | RFC 3986; npm's `file:` and URL dependencies; Cargo's `path` | Absolute URLs only | MAN-006 |
| C48 | `exports` is a list of names. An implementation is exported by its item under `implementations` and is not repeated | PKG-018, PKG-019 | A map from a name to a file | MAN-007 |
| C49 | A re-export is an item of `exports` that is a qualified reference, and keeps the name | JavaScript's `export ... from`; Rust's `pub use` | A map that can give a new name | MAN-008 |
| C50 | `implementations` is a map keyed by name; `supports` and `placement` may be left out | PKG-019 | Require both, as CTR-004 requires `placement` of a Type | MAN-009 |
| C51 | `command` is a list of strings, read by no shell, and a relative program is taken from the archive's root | The exec form of Docker and Kubernetes `command`; npm's `bin` | One string | MAN-010 |
| C52 | `url` is a URI; `graph` is a graph written in place and implements one operation | TPL-003, GRA-014 | `graph` names a file | MAN-010 |
| C53 | A project's documents are the `.yaml` and `.json` files in its directory itself. Nothing below is read | Terraform reads the files of one directory; `kubectl apply -f` does not recurse unless asked | Read every file below; accept `.yml`; list the files in the manifest | MAN-011 |
| C54 | The core package's implementations state no `command`, `url` or `graph`, and the core package has no digest | NAM-020: it is built in | See C23 | MAN-012 |
| C55 | The manifest lies at the root of the archive, with no directory around it | A Terraform module archive and a Lambda deployment package hold their files at the root | One directory around everything, as npm and Cargo do | MAN-013 |
| C56 | An archive with a member that would be written outside it is rejected | npm, Cargo and every careful unpacker | Leave it to each engine | MAN-014 |

### The names of the exports

| ID | Name | What | Alternative considered |
| --- | --- | --- | --- |
| N1 | `register.v1` | Register documents | `apply.v1` |
| N2 | `replace.v1` | Replace a template | `update.v1` |
| N3 | `rename.v1` | Rename an object | — |
| N4 | `unregister.v1` | Unregister an object | `delete.v1` |
| N5 | `inspect.v1` | Inspect an object | `get.v1`, `describe.v1` |
| N6 | `set-up.v1` | Set up an instance | `setup.v1`; the verb is two words in the definitions |
| N7 | `retire.v1` | Retire an instance | — |
| N8 | `retire-with-resources.v1` | Retire a Pool together with the Resources it owns | `retire-all.v1`, `retire-pool.v1` |
| N9 | `reconcile.v1` | Reconcile an invocation | — |
| N10 | `declare-lost.v1` | Declare a location lost | — |
| N11 | `purge.v1` | Purge records | — |
| N12 | `install.v1` | Install a package | — |
| N13 | `open.v1` | Open a Conduit | — |
| N14 | `close.v1` | Close a Conduit | — |
| N15 | `apply-policy.v1` | Apply a policy | `policy.apply.v1` |
| N16 | `remove-policy.v1` | Remove a policy | `policy.remove.v1` |
| N17 | `register-replacement.v1` | A new instance for a nested instantiation whose instance is retired | `reinstantiate.v1`, `new-instance.v1` |
| N18 | `retrieve-result.v1` | Retrieve one's own result | `result.v1`, `get-result.v1` |
| N19 | `set-default-location.v1` | The control that designates the default location | `default-location.set.v1` |
| N20 | `object-management.v1` | The capability of `management` | `management-operations.v1` |
| N21 | `access-control.v1` | The capability of `access` | `access-operations.v1` |
| N22 | `pool-management.v1` | The capability of every Pool | `pool-operations.v1` |
| N23 | `system-settings.v1` | The capability that holds the `system` Pool's control | `default-location.v1` |
| N24 | `interface.v1` | The plain Interface type | `plain-interface.v1` |
| N25 | `pool.v1` | The plain Pool type | `plain-pool.v1` |
| N26 | `system-pool.v1` | The type of `system` | `system.v1` |
| N27 | `location.v1` | The location type | — |
| N28 | `management.v1` | The type of `management` | — |
| N29 | `access.v1` | The type of `access` | — |
| N30 | `bound.v1` | The bound of a right to open | `open-bound.v1` |
| N31 | `shared-host.v1` | The shared-host requirement | — ; the accepted record on document schemas already wrote this name |
| N32 | `system` | The template of the `system` Pool | `system-pool` |
| N33 | `location` | The template of a location | — ; NAM-020 names it |
| N34 | `management` | The template of the `management` Resource | — |
| N35 | `access` | The template of the `access` Resource | — |
| N36 | `root` | The template of the root Interface | `root-interface` |
| N37 | `root-management` | The template of the root Interface's Conduit to `management` | `management-all` |
| N38 | `root-access` | The template of the root Interface's Conduit to `access` | `access-unbounded` |
| N39 | `engine` | The implementation that is the engine itself | `builtin` |
| N40 | `<kind>-reference.v1`, `reference.v1`, `actor-`, `consumer-`, `target-`, `entry-` and `pool-entry-reference.v1` | The 11 reference schemas | `reference.<kind>.v1`. The accepted record on document schemas wrote `resource-reference` |
| N41 | `object.v1`, `diagnostic.v1` | The 2 schemas of results | — |
| N42 | A noun for the thing described, such as `step.v1`, `template-spec.v1`, `manifest.v1` | The 45 schemas of documents | A prefix, such as `document.step.v1` |

## (c) The open points, each with a recommendation

These are the points the follow-up pass of the [re-audit](../reaudit/README.md#follow-up-pass)
left to this batch. "In the batch" says whether what was written already assumes the
recommendation.

| ID | Point | Recommendation | Follows | In the batch |
| --- | --- | --- | --- | --- |
| P1 | How the digest of an unpublished project is computed (SUB-008, NAM-009) | SHA-256, written as PKG-004 writes a digest, over the JSON Canonicalization Scheme form of one object with two members: `documents`, the project's contract documents ordered by `kind` and then by `metadata.name`, and `manifest`, the project's manifest, left out when there is none. The manifest is included because it fixes what the contracts' references to other packages mean | RFC 8785; SHA-256 is already in the profile. Go's `go.sum` hashes a sorted list in the same spirit | No. No file depends on it |
| P2 | How a purge's digest of an invocation's contents is computed (REC-004) | Leave it to the engine, as a line under entry DL2. The digest is never shown to anyone and never leaves the engine; REC-004 already states the one thing that matters, that contents equal under SCH-007 give the same digest. Name RFC 8785 over `{operation, arguments}` as one way that meets it, since it writes `1` and `1.0` alike | — | No |
| P3 | Whether `#name` in a `$ref` reaches a Schema contract of the same project or package (SCH-015, NAM-022) | Yes. The three forms cannot be confused: a JSON Pointer is empty or starts with `/`, and a name starts with a letter. Without it a project that is not published cannot share a schema between two of its documents, and a package must write its own address into its schemas | NAM-022 itself | No. Every schema here writes the qualified reference in full, and `schema.v1` rejects `#name` in a `$ref`. If accepted, `schema.v1` gains one branch |
| P4 | The places a destination may reach that INS-012 neither accepts nor forbids | Close the list. A destination is accepted only if it is a place INS-012 lists, or the whole of `config`, or the whole of one of the `arguments` maps INS-012 lists. Rejected: an item of `operations` such as `/operations/0`; a whole nested instantiation; `/requirements/<name>/requirement`; anything under `/inputs`; and any place not named | A closed list is what INS-012's forbidden list already implies for a template's fixed parts | No. `destination.v1` checks only the form. If accepted, three patterns can carry the list, one for each kind of `spec` |
| P5 | Which reason a rejected installation reports | Invalid, for every cause for which a promise says "the package is rejected", as OBJ-023 says of a rejected document. Unavailable where the archive cannot be fetched now. Conflict in one case only: a package is already installed at that address and version with another digest. Installing the core package's address is invalid (NAM-021) | Decision W4 | No |
| P6 | The host part of an address: a trailing dot, a numeric host, an internationalized name | No trailing dot. A host whose last label is all digits, such as `127.0.0.1`, is not a host name. An internationalized name is written in its ASCII form, with `xn--` labels; any other character is already refused by "lowercase letters, digits and hyphens". A label is at most 63 characters | RFC 1123 section 2.1, which NAM-001 and the profile already cite; RFC 5890 for the ASCII form | Yes, in `address.v1` and `qualified-reference.v1`, except the 63 characters, which the engine checks |
| P7 | A component Conduit's name that would be longer than 63 characters (CON-003) | Where the name before the hyphen was generated, shorten the template's part further, as OBJ-003 already does, so that the whole fits. Where a document gave it, the document is rejected. A name is never longer than 63 characters | OBJ-001; Kubernetes shortens the base of a generated name | No |
| P8 | A rename that would make a tool name longer than 128 characters | Not dispatched, and reports conflict, the reason CON-026 gives a rename that would make two tool names equal. One rule then covers both ways a rename can break IFC-014 | CON-026 | No |
| P9 | A result that carries both an outcome and a reason | The outcome is unknown. The implementation has contradicted itself, so nothing it said about having performed nothing can be relied on | IFC-009 and OPS-007 treat every malformed result so | No |
| P10 | A repeat of a request that ended with a reason | It is a new request and is decided afresh. A request that ended with a reason has no outcome and began no invocation that OPS-008 keeps, whether the engine did not dispatch it or the implementation answered with a reason, having performed nothing. Nothing can be applied twice | OPS-005 | No |
| P11 | How an implementation's answer names the restriction that forbade the request (IFC-019, ACC-014) | The result's `_meta` holds one more key under Conduit's prefix, `restriction`, whose value is the item of the list it was sent that forbade the request or could not be applied: the `requirement` and its `arguments`. The result's `isError` is true, as IFC-025 says of the engine's own answer | The shape IFC-018 already gives an item | No |
| P12 | "In progress" outside the retrieval tool: the answer to a repeat of an invocation that has not ended (OPS-009) | The result carries neither an outcome nor a reason. Its `_meta` holds a key under Conduit's prefix, `state`, with the value `inProgress`, the word `retrieve-result.v1` uses | The retrieval tool's own output | No, apart from the word |
| P13 | "Withheld" outside the retrieval tool: the direct answer of ACC-034 | The result carries its outcome and no `structuredContent`, and its `_meta` holds a key under Conduit's prefix, `withheld`, with the value true | The retrieval tool's own output | No, apart from the word |
| P14 | Whether setting up and retiring count as changes that are applied one at a time (REV-012) | One at a time, yes: while a setup, a retirement or a change of an instance has been dispatched and has not ended, no other of them is dispatched for that instance, and DL9 says what the later request reports. A revision, no: setting up and retiring name none, do not change the definition or configuration, and do not advance the revision | REV-012; a Kubernetes object's status changes without a new generation | Partly. `set-up.v1`, `retire.v1`, `open.v1` and `close.v1` have no `revision` input |

## Outcome

Owner ruling of 2026-11-05 (plan-audit ruling 37). The batch was accepted as drafted, with the
changes below. [APPLY.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/APPLY.md) is the instruction set for the ruling. "In the documents"
says what the ruling changed under `spec/core/`, `spec/schemas/` and in
`spec/promises/manifest.yaml`; "—" means nothing there had to change.

### The five changes

| ID | Change | Promise | In the documents |
| --- | --- | --- | --- |
| X1 | A location's facts change only by a control [(D11)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) | PLC-008 | The control `set-facts.v1`, with `changes: [/facts]`, the inputs `revision` and `facts` and the output `settings`; the capability `location-settings.v1`, which the type `location.v1` offers; both exported |
| X2 | Requests name an object by reference only [(D10)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) | LIF-007 | — ; every core operation already named an object by a reference |
| X3 | A bound in the wrong place [(D29)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) | CON-028 | Two examples under `examples/invalid-by-rule`, and a valid Conduit to `access` that applies `bound.v1` |
| X4 | The reserved mark [(D30)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) | CTR-009; CTR-002 names the key | `operation-spec.v1` documents `reservedToRoot`; one example under `examples/invalid-by-rule` |
| X5 | `#name` carries no kind [(D19)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) | NAM-027 | One example under `examples/invalid-by-rule` |

### The open points

Every recommendation in (c) was accepted as written.

| ID | Where it is promised | In the documents |
| --- | --- | --- |
| P1 | SUB-008, NAM-009; RFC 8785 in the schema profile | — |
| P2 | REC-004; entry DL2 | — |
| P3 | SCH-015 | `schema.v1` accepts `#name` in a `$ref`; both example packages use it; two invalid examples |
| P4 | INS-012 | Each of the three schemas of a template's `spec` carries the closed list as one pattern; four invalid examples |
| P5 | PKG-023 to PKG-025 | `install.v1` states the three reasons; MAN-013 and MAN-014 cite PKG-023 |
| P6 | NAM-001 | `address.v1` and `qualified-reference.v1` limit a label of the host to 63 characters; three invalid examples |
| P7 | CON-003, OBJ-003 | — |
| P8 | CON-026 | `rename.v1` states it |
| P9 | IFC-009 | — |
| P10 | OPS-025 | — |
| P11 | IFC-026 | — ; no schema and no output of a core operation describes a result's `_meta` |
| P12 | OPS-009, IFC-025 | — ; the core package's README names the key beside `retrieve-result.v1` |
| P13 | ACC-034, IFC-025 | — ; as P12 |
| P14 | REV-012 | Confirmed: `set-up.v1`, `retire.v1`, `retire-with-resources.v1`, `open.v1` and `close.v1` take no `revision`, and the checker now checks it |

### The disagreements

| ID | How it was settled | In the documents |
| --- | --- | --- |
| D1 | Accepted as the batch does: MAN-012, and the third form of `manifest.v1` for the core package only | — |
| D2 | NAM-009 and PKG-005: for the core package a template and an instance hold its address and version alone | MAN-012 cites the two and no longer says it itself |
| D3 | IFC-012's observable: a tool name ends in the operation's version segment, as in `checkout.directory.read.v1` | — |
| D4, D5 | OBJ-013, OBJ-014, POOL-012 and CON-018: `arguments` may be left out and then means none; an instance document may leave out `metadata` | — ; choice C7 |
| D6 | OBJ-019: inside a schema the names of properties are the author's, and the keywords are closed by SCH-002 | — |
| D7 | Accepted as the batch does: left to the engine, and listed in the schemas' README | — |
| D8 | ACC-033: registering a Resource into a Pool that exists needs a Conduit to `management` and a Conduit to that Pool, each listing `register.v1`; asking for a new instance for a nested instantiation needs the same two for `register-replacement.v1` | `register-replacement.v1` says so |
| D9, D20 | TPL-003, TPL-010, PKG-020, CTR-002, NAM-018 and OBJ-019: wherever a selection or such a key is "a qualified reference", `#name` is accepted for something in the same project or package | The descriptions of `implementation-map.v1` and `template-controls.v1`, and the READMEs, speak of selections |
| D10 | Change X2 | — |
| D11 | Change X1 | See X1 |
| D12 | ACC-031: a policy is not configuration. Applying one returns its ID, removing one names that ID, neither names a revision, and inspecting `access` lists the policies in its status | The status of `inspect.v1` may hold `policies`, each with an `id` |
| D13 | REV-006 and REV-007: a control's result states the settings it confirmed in the output `settings`; `revision` is the one input that must always be supplied | `set-facts.v1` follows it, and the checker checks every control |
| D14 | OBJ-021: a value is a reference where the schema that governs it is a reference schema, directly, as the `items` of a list, or as a branch of `anyOf` or `oneOf` | — |
| D15 | POOL-005: `{entry: name}` anywhere other than in an entry of a Pool template is rejected | Listed for the engine; one example under `examples/invalid-by-rule` |
| D16 | INS-006, which OBJ-006 cites: where an input's schema fixes the kind, a bare name is accepted for it, and the destination receives the reference with its kind | — |
| D17 | INS-014: a template with no `config` is checked against its type's schema as an empty map | — |
| D18 | NAM-022: a document refers to something in its own project or package by `#name`; its own full address is not in its manifest and is rejected under NAM-007. The core package's own documents are the exception | — |
| D19 | Change X5 | See X5 |
| D21 | IFC-025: a result that ends with a reason may carry `diagnostics` in its `structuredContent`, as a rejected registration does | `register.v1` and the core package's README say so |
| D22 | OPS-016 and IFC-024: a caller asks for its result by operation identity and target | — ; how to name a target that has been unregistered stays open |
| D23 | REC-004: a purge removes the records made before a time the request names | — |
| D24 | PKG-019: `supports` and `placement` may be left out, and then the implementation supports no requirement and has no requirement on placement | MAN-009 cites PKG-019 and no longer says it itself |
| D25 | CON-016: a Conduit template that is not composed requires `capability`; `operations` is required unless an input's destination fills it; `requirements` may be left out | `conduit-template-spec.v1` requires `operations` or `inputs`; one invalid example, and one under `examples/invalid-by-rule` |
| D26 | SCH-008: whether a `default` is allowed is judged where it is written, in the schema that holds it | — ; `schema.v1` already judged it so |
| D27 | CTR-001: a contract's name has at least one segment before its version segment | `contract-name.v1` requires it; one invalid example |
| D28 | SUB-005, with MAN-011: submitting a directory reads the `.yaml` and `.json` files in that directory itself and nothing below it | MAN-011 names `.yml` as an ending that is not read |
| D29 | Change X3 | See X3 |
| D30 | Change X4 | See X4 |
| D31 | PLC-008: a location's document that states a placement is rejected | Listed for the engine; one example under `examples/invalid-by-rule` |
| D32 | Accepted as the batch does: `resource-reference.v1`, and the engine checks that the Resource is a location | — |

### What the batch holds now

| Part | Where | What |
| --- | --- | --- |
| The schemas of documents | [spec/schemas](../../../spec/schemas/README.md) | 9 files holding 45 Schema contracts, as before |
| Examples | [spec/schemas/examples](../../../spec/schemas/examples) | 3 valid directories holding 7 files; 50 invalid documents that a schema rejects, or that are refused when read; and 8 sets of documents under `invalid-by-rule`, each valid against the schemas and rejected by a rule the checker applies itself |
| The core package | [spec/core](../../../spec/core/README.md) | 8 YAML files: its manifest; 20 Operation, 5 Capability, 6 Type and 2 Requirement contracts; 11 reference schemas and 2 schemas of results; and the 7 templates. 98 exports, 58 of them schemas, and one implementation |
| The manifest's format | [spec/promises/manifest.yaml](../../../spec/promises/manifest.yaml) | MAN-001 to MAN-014. None was added or removed |
| The checker | [tools/check_schemas.py](../../../tools/check_schemas.py) | Six steps. Step 6 is new: the rules no schema can express that the tool applies itself |

### Choices made while applying the ruling

Neither the record nor [APPLY.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/APPLY.md) states these. Each is the smallest reading found, and each is
for the owner to strike or keep.

| ID | Choice | Why |
| --- | --- | --- |
| A1 | `set-facts.v1` replaces the whole of `facts`, its input `facts` is optional, it declares `deduplicates: true`, and it is not reserved to the root Interface | `changes: [/facts]` is what X1 gives; [D13 makes](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) every input but `revision` optional; C22 for a change to what the engine stores |
| A2 | In the status that `inspect.v1` returns, `policies` is a list of objects, and only `id` is fixed | C38 fixed no more of an inspection result than it had to |
| A3 | The name of a declared Conduit, an entry, a requirement or a component inside a destination follows the name rule of OBJ-022; `target` and `placement` are reached whole and never inside | INS-011 needs the parent to be present, and INS-012 says "a declared Conduit's `target`" |
| A4 | `#name` in a `$ref` has the form of `#name` in a selection | NAM-022 gives one form |
| A5 | `conduit-template-spec.v1` rejects a template that is not composed and has neither `operations` nor `inputs`; that an input does fill `operations` is left to the engine | The profile cannot say that some member of a map holds a value |
| A6 | The rules the engine must check and that the checker also applies have their own examples, under `examples/invalid-by-rule`, apart from the documents a schema rejects | So that the report never says a schema rejected what a schema accepts |
| A7 | MAN-010 says that `plain: false` means the same as leaving `plain` out; MAN-012 says that the core package's manifest lists no `packages` | C8, and the third form of `manifest.v1`, were accepted and no promise said either |

### Found while applying it, and not settled

- **`#name` and implementations.** NAM-022 says `#name` "refers to the document of that name".
  An implementation is not a document, yet NAM-018 lets the selection of one be `#name`, and
  the example package writes `'#files-server'`. NAM-027 makes the names of documents differ,
  and PKG-012 the names of exports; nothing makes the name of an implementation differ from
  the name of a document its package does not export, so `#name` could still name two things.
  No promise was added for it. The checker refuses the clash in the files it reads and says
  that no promise covers it.
- **D22.** A caller names its invocation by operation identity and target. A target that has
  been unregistered can no longer be named, and no promise says how its result is retrieved.

### Verified after the ruling was applied

Run from the repository's root on 2026-11-05, after the last change to any file of the batch
and after Editor P's log of the promise side was written.

`python3 tools/check_schemas.py`, which ran itself again under `/usr/bin/python3` because the
first interpreter lacks PyYAML and jsonschema. The 58 lines that begin "rejected" are left
out here; each names a file and says why it was rejected:

```text
Python 3.9.6 at /Applications/Xcode.app/Contents/Developer/usr/bin/python3; PyYAML 6.0.1; jsonschema 4.20.0
Profile: 27 keywords, 5 formats. Format checkers available for all of them.
1. Read 82 files holding 194 documents as strict YAML.
2. Checked 58 Schema contracts against the profile, and every `$ref`.
3. Validated 134 documents under spec/schemas/, spec/core/ and examples/valid/.
4. 50 invalid documents in 50 files were each rejected by a schema, or refused when read.
5. Core package: 98 exports, 0 documents deliberately private and 1 implementation. Checked 392 references for the kind they name; 1 name packages that are not in this repository and were not checked. Checked 3 controls, the 5 operations that take no `revision`, the tables of the two READMEs, and 649 citations of promises in 85 files. The example packages keep 0 documents private.
6. Applied 8 rules of this tool, which come from 7 promises, to the core package and 3 example directories. 8 sets of documents in 8 files under examples/invalid-by-rule/ were each rejected by the rule they are named for; each of their 11 documents is valid against the schemas.
Errors: 0
```

`python3 tools/check_docs.py` and `python3 tools/set_names.py --check`:

```text
Checked 104 Markdown files and 1355 local links (offline).
Errors: 21
Names: corePackage x398, apiVersion x197, metaPrefix x51. Errors: 0
```

The docs check reported 21 errors in this run. None is in a file of this batch: every one is a
link to `experiments/001-shared-access`, a directory that another editor removed while this
pass was being finished, from `docs/VOCABULARY.md`,
`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/collaborator-proposals/PR-002-protocol.md`,
`https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/reviews/AUTHORING-2026-11-04.md`, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/archive/reviews/CONCEPTUAL.md`,
`experiments/README.md`. Earlier runs in this pass, before that removal, reported no error.
Other editors were changing the repository at the same time, so the counts of files, links and
names include their work.

A YAML load of `spec/promises/manifest.yaml`: 14 promises, each with exactly the keys `id`,
`layer`, `level`, `when`, `then`, `observable` and `vectors`, in that order, and no ID twice.
The validation command of the [consolidation instructions](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md) over
every promise file: 376 promises. Every promise ID that this section and
`spec/promises/manifest.yaml` cite exists.

What the lines mean, beyond what [(e)](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#e-what-was-verified-and-what-was-not) says of steps 1 to 4:

- **Step 4** counts only documents that a schema rejects, or that are refused when read. Each
  of the 10 new files was read in the report and is rejected for the reason it is named for.
- **Step 5** now also checks that every export exists and every document is exported; that
  the five operations of REV-012 take no `revision`; that each of the three controls in the
  repository has `changes` its type's schema describes, takes `revision`, has every other
  input optional and returns `settings`; that the tables of the two READMEs list exactly the
  files and the exports there are, each export with its kind; and that every promise cited
  under `spec/core/`, `spec/schemas/` and in `spec/promises/manifest.yaml` exists.
- **Step 6** applies eight rules of the tool's own. The core package and the three valid
  directories break none. Each of the 8 sets under `examples/invalid-by-rule` is rejected by
  the rule it is named for, and each of their 11 documents was first shown to be valid
  against the schemas, so no schema is what rejects them.

Also run, and not kept in the repository:

- 182 values were tried against single schemas: host labels of 63 and 64 characters, the
  forms of an address, of a contract's name and of a `$ref`, every accepted and rejected
  destination for each kind of template, and the forms of a Conduit template's `spec`. All
  behaved as intended.
- 16 deliberate faults were put, one at a time, into a copy of `spec/` and `tools/`: an
  export removed, a `revision` added to `open.v1`, a control's input made required, two
  documents given one name, a promise ID that does not exist, a wrong kind in a README, a
  document in `invalid-by-rule` made invalid against its schema, and others. The checker
  reported each. It reported nothing for a seventeenth change that is not a fault: a `$ref`
  of the core package rewritten as `#name`.
- The path without jsonschema was run by hiding the module. It ran steps 1, 2, 5 and 6, said
  that no document was validated against a schema, and exited with status 3.

Checked by reading, not by a tool: in the additions to the valid project, the operations
`team-access` lists are in its capability; the Resource `access` offers that capability; the
engine states `bound.v1` under `supports`; the bound's arguments fit its inputs; and the two
tools presented to `operator` have different names.

Not verified, as before: no engine exists; `pattern` was evaluated by Python's `re` and not
by an ECMA-262 engine; no archive was built; and the rules of step 6 are applied only as far
as the documents in front of the tool show, so they are not the engine's checks.

## Choices that set behavior

### Document log

Source: [log-documents](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-documents.md#choices-made-that-neither-the-record-nor-applymd-states).

The document log names these seven choices (A1 to A7); their reasons are preserved verbatim.

| ID | Choice | Why |
| --- | --- | --- |
| A1 | `set-facts.v1` replaces the whole of `facts`, its input `facts` is optional, it declares `deduplicates: true`, and it is not reserved to the root Interface | `changes: [/facts]` is what X1 gives; [D13 makes](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/README.md#d-where-the-promises-disagree-cannot-be-expressed-or-forced-a-guess) every input but `revision` optional; C22 for a change to what the engine stores |
| A2 | In the status that `inspect.v1` returns, `policies` is a list of objects, and only `id` is fixed | C38 fixed no more of an inspection result than it had to |
| A3 | The name of a declared Conduit, an entry, a requirement or a component inside a destination follows the name rule of OBJ-022; `target` and `placement` are reached whole and never inside | INS-011 needs the parent to be present, and INS-012 says "a declared Conduit's `target`" |
| A4 | `#name` in a `$ref` has the form of `#name` in a selection | NAM-022 gives one form |
| A5 | `conduit-template-spec.v1` rejects a template that is not composed and has neither `operations` nor `inputs`; that an input does fill `operations` is left to the engine | The profile cannot say that some member of a map holds a value |
| A6 | The rules the engine must check and that the checker also applies have their own examples, under `examples/invalid-by-rule`, apart from the documents a schema rejects | So that the report never says a schema rejected what a schema accepts |
| A7 | MAN-010 says that `plain: false` means the same as leaving `plain` out; MAN-012 says that the core package's manifest lists no `packages` | C8, and the third form of `manifest.v1`, were accepted and no promise said either |

### Promise log

Source: [log-promises](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-promises.md#where-a-row-of-applymd-named-two-homes).

The promise log chose these homes where the application instructions named two.

| Item | Home chosen | Why |
| --- | --- | --- |
| X4 | A new promise, CTR-009 | CTR-003 is about a Capability contract. The rule is about an Operation contract, and about a project as well as a package. CTR-002 now lists the key `reservedToRoot`, and CTR-003 says what a reserved operation is |
| P9 | IFC-009 | It is the promise that says which results are malformed and so unknown. IFC-027 is about a result that holds a reason alone; its `when` now says so, and OPS-007 and IFC-026 agree |
| D16 | INS-006 | The rule is about what a destination receives, which INS-006 states. OBJ-006 gained one sentence that points there |
| D28 | SUB-005 | SUB-005 says what a submission may contain. SUB-001 is about order |
| P5 | Three promises, PKG-023 to PKG-025 | [APPLY.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/APPLY.md) asks for one new promise. Rule 6 of the consolidation instructions asks that a promise about a request that is not dispatched name exactly one reason, so each of invalid, unavailable and conflict has its own |
