# Round 3, reader 2: terms and shapes

Cold read of `spec/` alone, for terms and document shapes. I read all 129 files under `spec/`
(124 YAML, 5 Markdown): the 424 promises in 22 files, `definitions.md`, `names.yaml`,
`schema-profile.md`, the 15 items of `implementation-defined.yaml`, the 99 documents and the
manifest of the core package (54 under `spec/core/`, 45 under `spec/schemas/`), the three
READMEs, and every example: 37 documents in 7 valid files, 59 invalid files and 30 documents in
17 invalid-by-rule files. I loaded every YAML file with a loader that refuses duplicate keys,
rebuilt the schemas under one `$defs`, validated the core package and the valid examples,
checked each invalid example for the one error it claims, and probed the schemas with about 90
small documents.

Thirteen findings: 1 `unsafe`, 3 `mismatch`, 6 `term`, 1 `citation`, 2 `minor`. The schemas,
the core package, the counts and the `_meta` keys agree with the promises almost everywhere;
what is left is one access gap around `bound.v1`, two places where a shape and a promise part,
one example that stops short of the promise it shows, and vocabulary.

## Findings

### 3B-1 (`unsafe`) `bound.v1` as a constituent of a composition is not covered

Files: `spec/conventions/access.yaml` (ACC-029, ACC-022, ACC-030), `spec/promises/conduits.yaml`
(CON-028, CON-030), `spec/promises/contracts.yaml` (CTR-010), `spec/core/requirements.yaml`.

CTR-010 lets a composition take any requirement as a constituent:

> The constituent holds `requirement`, the selection of a requirement

CON-030 says what holds for the other core requirement in that position:

> `shared-host.v1` may be applied itself, and may be a constituent of any composition.

Nothing says the same, or the opposite, for `bound.v1`. Every rule on a bound is written for a
template that applies `bound.v1` itself. CON-028:

> A Conduit template applies the core package's requirement `bound.v1` more than once, or a
> Conduit is instantiated that applies `bound.v1` and whose target is not the `access` Resource.

and its observable counts "each item of its `requirements` that applies
`github.com/shrav-k/conduit/core#bound.v1`". ACC-029:

> A Conduit to `access` that applies no `bound.v1` has no bound and is unlimited: it covers
> every target and names every operation.

Take a Conduit template to `access` whose one requirement is a composition, of a project or a
package, with `bound.v1` as its constituent. It is accepted: the composition narrows because its
constituent does (CTR-005), and `engine` supports it because it states the constituent
(PKG-019: "lists the composition or every one of its constituents"). One engine reads ACC-029 by
the letter: no item of `requirements` applies `bound.v1`, so the Conduit "has no bound and is
unlimited". Another treats the constituent as the bound. The author wrote a limit, and the first
engine opens a right to open with none. The at-most-once rule and the target rule of CON-028 are
passed in the same way: one direct `bound.v1` beside a composition that holds another, or a
composition with `bound.v1` on a Conduit whose target is not `access`.

Least-change fix: one sentence in CON-028 (and a clause in the `bound` row of
`spec/definitions.md`): a Requirement contract that has `bound.v1` among its constituents,
directly or through other compositions, is rejected when its package is installed or its
project is submitted. That keeps "applies `bound.v1`" meaning one thing. The alternative, to
say that a constituent counts as the bound, needs ACC-029, ACC-030 and CON-028 reworded.

### 3B-2 (`mismatch`) No promise says what a request that writes the resolved form reports

Files: `spec/conventions/lifecycle.yaml` (LIF-007), `spec/promises/objects.yaml` (OBJ-009,
OBJ-021), `spec/conventions/operations.yaml` (OPS-029), `spec/core/references.yaml`,
`spec/core/operations.yaml`, `spec/schemas/README.md`.

The inputs of the core operations that name an object use the reference schemas
(`reference.v1`, `conduit-reference.v1`, `resource-reference.v1`, `actor-reference.v1`,
`target-reference.v1`), and each accepts `{kind, id}`. For a document the text names the rule
that rejects it (OBJ-009: "a submitted document that writes it is rejected"). For a request it
only states a fact. LIF-007:

> A request names an object by a reference and never by an ID

`spec/schemas/README.md`:

> A document or a request never writes it: the engine rejects a submitted document that does
> (OBJ-009), and a request names an object by a reference and never by an ID (LIF-007).

No promise says that a request which writes `{kind: Conduit, id: ...}` in `open.v1`, or
`{kind: Agent, id: ...}` in the `actors` of `apply-policy.v1`, is not dispatched, or which
reason it reports. OPS-029 covers "a reference that resolves to nothing", which this is not.

The same text makes the form unavoidable in one place. OBJ-021 says the stored form is

> the one a value reference with the source `config` yields

so a step whose argument is `{source: config, pointer: /cache}` hands `{kind: Resource, id}` to
its operation, and an implementation that receives a reference in its `arguments` or in a
restriction has only the ID, not the name. If the operation is one of the core package's, or
any operation whose input a reference schema governs, the step is a request that names an
object by an ID.

Least-change fix: decide it in one place. Either add to OPS-029 (or LIF-007) that a request
which writes the resolved form in an input is not dispatched and reports invalid, and say in
GRA-025 that a step is such a request; or say that the engine accepts the resolved form in a
request and in a step, and drop "never" from LIF-007 and from the two READMEs. This changes
behavior, so it is the owner's to rule.

### 3B-3 (`mismatch`) The example of later work has no operation that a start could invoke

Files: `spec/schemas/examples/valid/files-package/contracts.yaml`,
`spec/schemas/examples/valid/files-package/conduit.yaml`, ACC-040, ACC-019.

`directory.sweep.v1` is the example of `arrangesLaterWork: true`. Its comment says:

> A sweep is a start that the consumer did not request, so `retrieve-result.v1` does not return
> its result

and its effect is "Arranges a sweep, which removes the stale entries of the directory when it
runs." Under ACC-040 a start is an invocation of an operation that the asking implementation
names, dispatched only if

> that Conduit lists the operation to be started

The package declares no operation that removes stale entries. The capability
`directory-access.v1` holds `directory.read.v1`, `directory.list.v1`, `directory.sweep.v1` and
`set-read-limit.v1`, and `files-server` implements nothing else that a start could name. As
written, the only operation a sweep's start could invoke is `directory.sweep.v1` itself, which
arranges another sweep and removes nothing. The example shows the mark and not the feature.

Least-change fix: add one Operation contract to the package (for example
`directory.sweep-now.v1`, with the effect of removing the stale entries), list it in
`directory-access.v1`, in the `requires` of `local-directory.v1`, in `exports` and in the
`implements` of `files-server`, and say in the comment that each start of a sweep is an
invocation of it, asked for with `start-later-work.v1`.

### 3B-4 (`mismatch`) The schema rejects a `graph` item marked `plain: true`; no promise does

Files: `spec/schemas/manifest.yaml` (`implementation-item.v1`), `spec/promises/manifest.yaml`
(MAN-009, MAN-010), `spec/promises/packages.yaml` (PKG-020), `spec/schemas/README.md`.

MAN-009 gives an item:

> `implements`; exactly one of `command`, `url` and `graph`; and, where they are given, `plain`,
> `tools`, `supports` and `placement`.

PKG-020 rejects three things: an operation missing from `tools`, a plain item that states
`supports`, and an item that is not plain and states `tools`. The schema has two branches with
`plain: {const: true}`, one for `command` and one for `url`, and none for `graph`. An item with
`graph`, `plain: true` and `tools` is rejected by the schema (I checked it with a validator),
and by no promise. The list of choices in `spec/schemas/README.md` does not name this one.

Least-change fix: one clause in PKG-020: a plain implementation is reached by `command` or
`url`, and an item that states `graph` and `plain: true` is rejected. It follows from "an
existing server", and IFC-001 already speaks only of an implementation "that is reached by
`command` or `url`".

### 3B-5 (`term`) The rows for request and caller leave out the request for a start of later work

Files: `spec/definitions.md` (rows request, caller, onward request, the instance that sent a
request), IFC-021, ACC-040.

The rows give two cases and no third. Request:

> The asking for one operation on one target, by an actor on its own behalf or by an instance as
> an onward request.

Caller:

> For a request an actor makes on its own behalf it is that actor. For an onward request it is
> the caller of the invocation being served (IFC-020, IFC-021).

IFC-021, which the caller row cites, now states a third case:

> A request for `start-later-work.v1` names no invocation in that key, whichever instance makes
> it: its caller is that of the invocation that arranged the work

and ACC-040 scopes its operation identity "to the instance that asks, to that caller and to
that instance as the target". Such a request is made by any instance, a Resource included, is
not on its own behalf and serves no invocation. The row for `start-later-work.v1` avoids the
question by saying "call".

Least-change fix: add the third case to the three rows, each in one clause: a request for
`start-later-work.v1` is made by the instance that holds the work, of itself, for the caller of
the invocation that arranged the work (IFC-021, ACC-040).

### 3B-6 (`term`) `deduplicates` is used and never given a meaning

Files: `spec/definitions.md` (rows attempt, plain), CTR-002, OPS-012, OPS-021, OPS-028.

Of the four marks of an Operation contract, three have their meaning in `definitions.md`
(model work, later work, reserved to the root Interface). `deduplicates` appears there only as a
condition ("where the operation's contract declares `deduplicates: true`", "never counts as
deduplicating"). CTR-002 lists it bare and says that "IMP-003, ACC-019, OPS-012 and ACC-032
state what each mark means". OPS-012 states what the engine does when the mark is present, not
what the mark asserts. The assertion is in an observable of OPS-028 only:

> An implementation whose operation declares `deduplicates: true` therefore recognizes the new
> attempt that OPS-021 dispatches

Least-change fix: one row in `definitions.md`, or one clause in the attempt row: an operation
whose contract declares `deduplicates: true` is one whose implementation recognizes an
operation identity it has already been sent and applies the effect at most once for it.

### 3B-7 (`term`) "origin" is defined for records and used for the package that first exports

Files: `spec/definitions.md` (rows bound, `source`, origin), `spec/core/requirements.yaml`.

The `source` row reserves the word:

> where a record or an observation was made is its origin

The bound row, in the same file, uses it for something else:

> naming an operation that a package re-exports, by the re-export or by its origin, names both

and so do the head comment and the description of `bound.v1` in `spec/core/requirements.yaml`
("by the re-export or by its origin, covers both"). ACC-029 and PKG-013 write "the package that
originally exports it". The core document also says "covers both" where ACC-029 and the bound
row say "names both": in the promises a bound covers targets and names operations.

Least-change fix: write "or by the reference to the package that originally exports it" in the
three places, and "names both" in `spec/core/requirements.yaml`.

The same holds, less sharply, for "definition". Its row gives one meaning ("What a template or
an instance declares apart from its configuration"), and IFC-033, IFC-034 and the rows for tool
and Conduit's keys use "the tool's definition" for the protocol's tool definition. One clause
in the definition row, as the row for state has for its two uses, would settle it.

### 3B-8 (`term`) "A rejected installation" is given three reasons; by its definition it has one

Files: `spec/core/README.md`, `spec/definitions.md` (row rejected), PKG-023 to PKG-025.

`spec/core/README.md`, under Diagnostics:

> A rejected installation reports invalid, unavailable or conflict as PKG-023 to PKG-025 state.

The row for rejected:

> A package is rejected when it is installed, and is then not available; the request to install
> it reports invalid (PKG-023).

PKG-024 (the archive cannot be fetched now) and PKG-025 (already installed with another digest)
reject no package. The install row of `definitions.md` has the right words: "what a request
that installs nothing reports".

Least-change fix: in `spec/core/README.md`, "A request to install that installs nothing reports
invalid, unavailable or conflict as PKG-023 to PKG-025 state."

### 3B-9 (`term`) Three second words for things that have a word

Files: `spec/interface/implementation.yaml` (IFC-021), `spec/core/operations.yaml`,
`spec/core/README.md`, `spec/conventions/access.yaml` (ACC-027),
`spec/conventions/observation.yaml` (OBS-003),
`spec/schemas/examples/invalid/INS-012-destination-operations-in-a-composite.yaml`.

- "sender". The definitions allow "the instance that sent a request" and "the sending
  instance". IFC-021 writes "an invocation its sender does not serve"; the description of
  `retrieve-result.v1` writes "An identity under which the sender has no invocation" and "the
  sender can reach no invocation"; `spec/core/README.md` repeats both.
- "composed", of a Conduit template. The head comments of `spec/promises/conduits.yaml` and
  `spec/promises/instantiation.yaml` say the word is "composite" and that "composition" is kept
  for a requirement. The header of the invalid example still says "`operations` is a
  destination only in a template that is not composed".
- "the report", for an inspection result. ACC-027: "the report states whether the restriction
  is enforced or only mediated". OBS-003: "the report says unknown with no time". Everywhere
  else it is "the inspection result" or "the result".

Least-change fix: "the sending instance", "that is not a composite", and "the inspection
result".

### 3B-10 (`term`) TPL-003 says "a control" where it means a member of a template's `controls` map

Files: `spec/promises/templates.yaml` (TPL-003), `spec/definitions.md` (row control), CTR-002,
`spec/core/operations.yaml`.

TPL-003:

> A step, a runtime or a control that carries its own `implementation` key is rejected with a
> diagnostic naming it.

A control is defined as "An operation whose Operation contract also states, under `changes`,
the settings it changes", and CTR-002 lets an Operation contract hold `implementation`, "its
default implementation". The core package's two controls both carry it
(`implementation: github.com/shrav-k/conduit/core#engine`). Read with the definition, TPL-003
rejects them.

Least-change fix: "A step, a runtime or a member of a template's `controls` map that carries
its own `implementation` key".

### 3B-11 (`citation`) The lists of what a schema cannot express leave out IMP-012 and CON-029

Files: `spec/schemas/README.md` ("What a schema cannot express"), `spec/schemas/graphs.yaml`
(head comment).

The row "The meaning of a graph" names later work and cites "IMP-004, IMP-005, IMP-009,
IMP-010". IMP-012, which rejects a `setup` or `retire` graph with a step whose operation is
marked `arrangesLaterWork: true`, is cited nowhere under `spec/schemas/`. A template that
breaks it is valid against `template.v1`. The head comment of `spec/schemas/graphs.yaml` ends
its list with "and model work (IMP-004, IMP-005, IMP-009)" and names neither IMP-010 nor
IMP-012. The same row says "which source is allowed in which graph" and cites GRA-010 without
GRA-020, which is the promise that rejects `input` in a `setup` or `retire` graph. CON-029, the
component Conduit whose name would pass 63 characters, is cited nowhere under `spec/schemas/`
either; IFC-014, its sibling for tool names, is in the row "Other uniqueness across documents".

Least-change fix: add IMP-012 and GRA-020 to the graph row and IMP-010 and IMP-012 to the head
comment of `spec/schemas/graphs.yaml`, and CON-029 beside IFC-014.

### 3B-12 (`minor`) An instance document in a package, and an export that names one

Files: `spec/definitions.md` (row export), `spec/promises/manifest.yaml` (MAN-007, MAN-011),
`spec/promises/packages.yaml` (PKG-018), NAM-013.

The export row says an exported document "is a template or a contract". MAN-007 says only:

> each item is either the name of a document of the package, which PKG-018 makes an export, or a
> qualified reference

and MAN-011 lets a package's directory hold instance documents ("holds something other than
templates, instances or contracts is rejected"). NAM-013 says installing registers nothing. So
an instance document in an archive is read, counted for NAM-027 and never used, and an
`exports` item that names it is accepted by MAN-007 and excluded by the definition. No promise
says which.

Least-change fix: one sentence in MAN-007: an item that names an instance document is rejected
(or, if instance documents have no place in a package at all, one sentence in MAN-011 or
MAN-013 that a package which holds one is rejected when it is installed).

### 3B-13 (`minor`) No rule for the keys of an implementation item's `placement`

Files: `spec/promises/objects.yaml` (OBJ-022), `spec/promises/manifest.yaml` (MAN-010),
`spec/schemas/manifest.yaml`, `spec/schemas/README.md`.

OBJ-022 gives the name rule to "the `runtimes` or `placement` of a Type contract". A manifest
is not a document in that sense, and MAN-010 says only:

> `placement` is a map of applied requirements, as CTR-004 gives a Type.

The head comment of `spec/schemas/manifest.yaml` lists "the rule for a key of `packages` and of
`implementations` (NAM-001, NAM-004)" as the engine's to check, and the first row of "What a
schema cannot express" lists "a member of `placement` in a Type". Whether
`placement: {Local_Paths: {...}}` in an implementation item is rejected is not stated.

Least-change fix: in MAN-010, "`placement` is a map of applied requirements keyed by name, as
CTR-004 and OBJ-022 give a Type".

## What I checked and found consistent

Counts and lists

- Promises per file against the Contents table of `spec/README.md`: all 22 files match, 424 in
  all, 15 implementation-defined items. Every promise has exactly the seven fields, a `layer`
  and a `level` from the stated lists, and `vectors: []`. The layer exceptions the READMEs and
  head comments state (PKG-016, IFC-014, the six of `implicit.yaml`, LIF-002, LIF-007, and the
  convention promises in topic files) are as stated.
- Every promise ID cited anywhere under `spec/` exists, except the 25 removed IDs, which occur
  only in head comments that say where each went. No `DL` item past DL15 is cited.
- The core package: 99 documents, each name unique; `spec/core/conduit.yaml` exports exactly
  those 99, in the order of the "Every export" table of `spec/core/README.md`, which has 100
  rows with `engine`; `engine` implements exactly the 21 Operation contracts. Counts by kind
  (7 templates, 6 types, 5 capabilities, 21 operations, 2 requirements, 11 reference schemas,
  2 result schemas, 45 schemas of documents, 58 Schema contracts in all) match all three
  READMEs, and the files table of `spec/schemas/README.md` matches file by file.
- The operations table of `spec/core/README.md` against the documents: capability membership,
  the `revision` input, `deduplicates`, `effect`, `reservedToRoot` and the default
  implementation of all 21. Each core type's `requires` is the union of the operations of the
  capabilities it offers. `root-management` and `root-access` list every operation of their
  capability, and `root-access` applies no requirement.
- Example counts: 7 valid files in 3 directories, 59 invalid, 17 invalid-by-rule.

Core package against the promises

- Every input, output and mark that a promise names for a core operation exists with that name
  and optionality: `documents` and `manifest` (MAN-001, SUB-005, SUB-009, SUB-010); `instance`
  (TPL-008, ACC-033, LIF-009); `revision` (REV-011, OBJ-026, SUB-009, LIF-017); `revision` and
  `settings` of `reconcile.v1` (REV-010) and `invocation` (OPS-014); `fetch` (NAM-012,
  MAN-006); `before` (REC-004); `actors`, `targets`, `arguments` and the returned ID
  (ACC-031); `operationIdentity`, `target`, `state` with its four values, `invocation`,
  `withheld`, `outputs` (IFC-024); `invocation`, `operation`, `arguments`, `outputs`
  (IFC-035); `location` or null (PLC-016); `facts` (PLC-021); `id`, `revision`, `packages`,
  `declared`, `status`, `records`, and the three shapes of a `packages` item (LIF-013,
  NAM-009, PKG-005); `diagnostics` on the four operations the README names (IFC-025).
- The capabilities include what ACC-033, OPS-019, LIF-021, LIF-009, PLC-016, PLC-021, ACC-006,
  ACC-012 and ACC-031 say, `reconcile.v1` is in the three that hold management verbs and in no
  other, and the two always-present tools are in none. Only `declare-lost.v1` and `purge.v1`
  carry `reservedToRoot`; no core operation carries `startsModelWork` or `arrangesLaterWork`;
  no core input states a `default`; both controls state `changes`, `revision` and `settings`,
  and each pointer is a place its type's schema describes (CTR-006).
- `bound.v1` and `shared-host.v1` against ACC-029, CTR-005, PLC-013 and SCH-019; the 11
  reference schemas against OBJ-006, OBJ-013, POOL-005, POOL-014 and the resolved form of
  OBJ-009; `object.v1` against OBJ-003; the seven templates against FST-001, FST-002, FST-005,
  PLC-008, INS-021 and INS-024.

Schemas against the promises

- `template.v1` and the three `spec` schemas against OBJ-012, TPL-001, TPL-002, CON-016 to
  CON-020 and the closed list of INS-012 (each destination pattern, with the examples INS-012
  gives as accepted and as rejected); `template-input.v1` against INS-021 and OBJ-015;
  `implementation-map.v1`, `runtime.v1` and `template-controls.v1` against TPL-003, TPL-006 and
  TPL-010.
- `instance.v1` against OBJ-013, PLC-006, POOL-001 and CON-027; `template-reference.v1`
  against the four forms of INS-025; the three nested instantiations against OBJ-014.
- `graph.v1`, `lifecycle-graph.v1`, `step.v1`, `value-binding.v1` and `value-reference.v1`
  against GRA-015, GRA-017 to GRA-021 and GRA-023.
- `contract.v1` and the four `spec` schemas against CTR-001 to CTR-006 and CTR-010, key by key;
  `applied-requirement.v1` against NAM-019 and CON-018.
- `manifest.v1` against MAN-003 to MAN-012, NAM-006 and PKG-017 to PKG-020, apart from 3B-4.
- `schema.v1` against the keyword and format lists of `schema-profile.md`, SCH-008 (where
  `default` is and is not accepted, including a property inside `items`, a member of `$defs`
  and anything inside `allOf`, `anyOf`, `oneOf`) and the three forms of `$ref` of SCH-015.
- `values.yaml` against OBJ-001, NAM-001, NAM-002, NAM-004, NAM-005, CTR-001, OBJ-016, PKG-001
  and PKG-004, with the accepted and rejected addresses NAM-001 lists.
- Every key a schema accepts has a promise that gives it a meaning, or is one of the choices
  `spec/schemas/README.md` lists. The only maps left open are the ones that README names.

Examples

- All 99 core documents, the core manifest and all 37 valid example documents validate.
- Each of the 59 invalid examples is rejected at exactly one place, and for the reason its name
  and header claim (`SCH-012-duplicate-key.yaml` when its file is read).
- All 30 documents of the 17 invalid-by-rule sets validate against the schemas, each set breaks
  the rule it is named for, and I found none that breaks another rule that is not a consequence
  of the same defect.
- The valid project and package against the promises, by hand: NAM-022 and NAM-027 (names and
  the `#name` form), TPL-004, TPL-005 and TPL-014 (an implementation for every operation, none
  plain for the marked one), TPL-006 (both runtimes), CTR-005 and CTR-010 (the composition),
  CTR-006 (the control), SCH-019 and SCH-010 (the defaults), POOL-005 and POOL-014 (the two
  entry references), CON-003 and IFC-012 (component names and tool names, no collision),
  CON-021, CON-022, CON-028 and CON-030, and PLC-002 for the instances with no placement. Only
  3B-3 came of it.
- A note, not a finding: `OBJ-015-input-marked-required.yaml` and
  `INS-012-destination-under-inputs.yaml` give a template of the type `interface.v1` an input
  with the destination `/config/title`. That type's schema closes `config`, so with the named
  defect fixed the template would register and every instance of it would be rejected under
  INS-014. No rule is broken at registration.

Terms and the interface

- Every row of `definitions.md` against the promises it cites: apart from 3B-5 and 3B-7 the
  rows agree with what they cite, and every citation points at a promise that says the thing.
- The vocabulary rules of `definitions.md`, searched across `spec/`: no "grant", "declaration"
  or "manifest item"; "create" only in the effect of an example operation, which is not an
  operation on an instance; "answer" only as a verb; "source" in the promises only as the key;
  "entry" in the promises only for a Pool's entry; "hold" never of a Conduit or a right;
  "implementation" never of the engine except `engine` in the core manifest; no key `required`
  outside a schema; no word other than the seven for how a request ends; "registered" of an
  instance only as the state.
- The 14 keys under `io.github.shrav-k.conduit/` that the promises use (`interfaceVersion`,
  `instance`, `operation`, `invocation`, `caller`, `restrictions`, `operationIdentity`,
  `conduit`, `serving`, `outcome`, `reason`, `restriction`, `state`, `withheld`) are all in
  the Conduit's keys row, each with one meaning, and the enumeration in `spec/core/README.md`
  agrees with IFC-006 and IFC-020.
- The three names of `names.yaml` are written the same in every file, and the retired name
  occurs only in `names.yaml`.
