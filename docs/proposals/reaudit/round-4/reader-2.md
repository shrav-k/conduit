# Round 4, reader 2: terms and shapes

Cold read of `spec/` alone, for terms and document shapes. I read all 134 files under `spec/`:
`README.md`, `definitions.md`, `names.yaml`, `schema-profile.md`, `implementation-defined.yaml`
(15 items), the 22 promise files (437 promises), the 9 files of `spec/core/` (99 exported
documents and the manifest), the 10 files of `spec/schemas/` (45 Schema contracts and the
README), and the 88 example files (7 valid files holding 38 documents, 61 invalid files, 20
invalid-by-rule files holding 33 documents). I loaded every YAML document, rebuilt the schemas
as one JSON Schema 2020-12 document, validated the core package and the valid examples against
it, checked that every invalid example is rejected, and repaired each invalid example by the
one change its header names to confirm that nothing else in it is rejected by a schema.

Eleven findings: `unsafe` 0, `mismatch` 3, `term` 5, `citation` 0, `minor` 3. No finding in the
schemas of documents themselves: every key a promise names is in its schema with the same name,
type and optionality. The three mismatches are in a core description, in what `manifest.v1`
accepts from a registration, and in the input of `replace.v1`.

## Findings

### 4B-1. `object-management.v1` says `management` sets up and retires a Resource of a retired Pool

Severity: `mismatch` (a core document against a promise, on an access rule).
Involved: `spec/core/capabilities.yaml`, ACC-033, the `management` row of `definitions.md`.

The description of the capability says what `management` offers:

> inspecting, setting up and retiring what is not a Resource, or is a Resource whose Pool
> is retired;

ACC-033 moves two requests to `management` when a Pool is retired, and no other:

> Where a Pool is retired, two of the requests that this promise authorizes against that Pool
> are authorized against `management` in its place: inspecting a Resource that the Pool owns,
> and reconciling an invocation. No other request is authorized against a retired Pool.

and its observable names the two verbs the description adds: "one made through a Conduit to
`management` reports refused too: setting up or retiring a Resource that the Pool owns". The
head comment of the same file and the descriptions of `set-up.v1`, `retire.v1` and `inspect.v1`
in `spec/core/operations.yaml` all agree with ACC-033; only this description differs. A
description has no meaning to the language (OBJ-018), so no engine is bound by it, but it is
the one sentence that states what the capability is for.

Fix: "inspecting, setting up and retiring what is not a Resource; inspecting a Resource whose
Pool is retired;".

### 4B-2. `manifest.v1` accepts the core package's form from any registration

Severity: `mismatch` (a shape the schema accepts and no promise gives a meaning or rejects).
Involved: `spec/schemas/manifest.yaml` (`$defs/core`), NAM-021, MAN-009, MAN-012,
`register.v1`, the table "What a schema cannot express" in `spec/schemas/README.md`.

`manifest.v1` is `anyOf` of `project`, `package` and `core`. The `core` form is selected by the
address alone, `address: {const: github.com/shrav-k/conduit/core}`, and lets an implementation
item hold `implements` with none of `command`, `url` and `graph`. `register.v1` takes
`manifest` as `manifest.v1`. This manifest validates (checked with a validator), and the same
manifest under any other address does not:

```yaml
apiVersion: github.com/shrav-k/conduit/v0
address: github.com/shrav-k/conduit/core
version: 9.9.9
implementations:
  mine:
    implements: ['#directory.erase.v1']
```

The schema README and the head of `manifest.yaml` leave the case to NAM-021 ("That the core
package is not listed in a manifest"). NAM-021's `when` covers three cases:

> A package is installed whose address is `github.com/shrav-k/conduit/core`, or a manifest
> lists that address or gives `fetch` for it.

A manifest handed to `register.v1` that states the address as its own is not installed, does
not list the address under `packages`, and gives no `fetch`. MAN-012 then reads, by its own
`when` ("The manifest of the core package, whose address is ..., is read"), as if it applied:
"Its implementations are the engine itself". SUB-009 does reject `address` in a manifest handed
to `replace.v1`; nothing says so for `register.v1` or for a directory under SUB-010.

Fix: one clause in NAM-021's `when`, "or a manifest that is handed over states that address as
its own `address`", and the same words in the README row and in the head of `manifest.yaml`.
See 4B-4 for the wider question of a registration whose manifest states any `address`.

### 4B-3. "management verb" has two tests, and they differ for `purge.v1` and `inspect.v1`

Severity: `term`.
Involved: the `management verb` row of `definitions.md`, CON-024, OPS-019, ACC-033, OPS-014,
`spec/README.md`, `spec/core/README.md`, `spec/core/operations.yaml`.

The definition tests an operation by what it manages:

> An operation of the core package by which objects, Conduits, policies and packages are
> managed, such as `register.v1`, ... Each is an operation of one of the capabilities ...

CON-024 tests it by the capability that includes it: "A management verb is an operation of
`object-management.v1`, `access-control.v1` or `pool-management.v1`, under OPS-019". The two
READMEs follow CON-024: `core/operations.yaml` holds "every management verb", the two tools and
the two controls, which makes all 17 other operations management verbs. `purge.v1` manages
records, which the definition's list does not name, and `inspect.v1` manages nothing; under the
definition's first sentence a reader can leave both out.

The term decides a target. ACC-033: "Reconciling an invocation of a management verb is
authorized against the target that authorized the request being reconciled. Reconciling any
other invocation ... is authorized against ... the Pool that owns that instance where it is a
Resource". An invocation of `purge.v1` whose outcome is unknown (it declares `deduplicates`,
so the case is real) is reconciled through `management` under CON-024's test, and through the
`system` Pool, which owns the Resource `management`, under the definition's.

Fix: make the row state CON-024's test: "Every operation of `object-management.v1`,
`access-control.v1` and `pool-management.v1` is a management verb, `inspect.v1` and `purge.v1`
among them; a control is not one."

### 4B-4. Three phrases for one thing, and a definition of "published" that has two tests

Severity: `term`.
Involved: the `published`, `project` and `#name` rows of `definitions.md`, MAN-004, INS-025,
NAM-022, NAM-009, MAN-011.

The defined phrase is "a project that is not published". The promises also say "a submitted
project" (INS-025 observable, NAM-022 observable twice, MAN-011 observable) and "its own
unpublished project" (NAM-009 `then`). They are one thing only if every submitted project is
unpublished, and the `published` row gives two tests that can disagree:

> Made available as an archive under an address. A package is a project that has been
> published: its manifest states its own `address` and `version` ... A project that is not
> published states neither.

MAN-004 takes the second test: "A manifest that states `address` and `version` is the manifest
of a package". A directory that an author submits before publishing it, with its manifest
already stating both keys, is not published by the first test and is a package by the second.
Nothing rejects such a registration, and SUB-008 keeps its contracts "as an unpublished
package". INS-025 then gives `#name` written as `from` two meanings for it: "In a document of
a package, `#name` names a template of the same package", and "In a project that is not
published ... it resolves as the bare name does: among the templates that exist". They differ
for a name that only an earlier submission registered: NAM-022 says of the package form that
"it is never looked up among the objects that exist".

Fix: write "a project that is not published" in the five places, and add one sentence, to
MAN-004 or to the `published` row, that says which of the two a registration is whose manifest
states `address` and `version`: either it is rejected, or it is a project that is not
published whatever its manifest states. The choice changes behavior and is not mine to make.

### 4B-5. The input `manifest` of `replace.v1` accepts what SUB-009 rejects

Severity: `mismatch`.
Involved: `spec/core/operations.yaml` (`replace.v1`), SUB-009, `spec/schemas/README.md`,
the head of `spec/schemas/manifest.yaml`.

SUB-009: "A manifest handed over with the document may hold `packages` alone, beside its
`apiVersion`: one that holds any other member, such as `implementations`, `exports`, `address`
or `version`, is rejected." The input is declared as
`schema: {$ref: github.com/shrav-k/conduit/core#manifest.v1}`, which accepts all four, so the
tool definition that IFC-033 derives presents a full manifest as the input. The description of
`replace.v1` does state the rule, so an engine that reads it rejects the request. But the
table "What a schema cannot express" and the "Not expressed here" list at the head of
`manifest.yaml`, which are where a reader looks for the rules the schemas leave to the engine,
do not mention SUB-009 at all (it is cited nowhere under `spec/schemas/`).

Fix, least change: one row in the README table and one clause in the head of `manifest.yaml`:
"that a manifest handed to `replace.v1` holds `packages` alone (SUB-009)". The rule is also
expressible, as an inline schema of the input with `apiVersion` and `packages` and
`additionalProperties: false`, at the cost of repeating the shape of an item of `packages`.

### 4B-6. "The templates of the first-start objects" is used for seven templates

Severity: `term`.
Involved: `spec/README.md`, `spec/core/README.md`, `spec/core/conduit.yaml`,
`spec/core/templates.yaml`, FST-001, FST-002, the `first-start object` row of `definitions.md`.

The definition and FST-002 are firm that the two Conduits of `root` are outside the term:
"The two Conduits of `root` that exist from first start are not first-start objects", "They are
not first-start objects". Four places count their templates under it:

- `spec/README.md`: "The templates of the first-start objects | Draft; 7 templates"
- `spec/core/README.md`: "The 7 templates of the first-start objects"
- `spec/core/conduit.yaml`: "# Templates of the first-start objects (templates.yaml)." above
  seven exports
- `spec/core/templates.yaml`: "The templates of the first-start objects (FST-005)."

The `core package` row of `definitions.md` has the right wording.

Fix: use that wording in the four places: "the templates of the first-start objects and of the
two Conduits of `root`".

### 4B-7. "chain" has no row, and its members are invocations or Conduits by turns

Severity: `term`.
Involved: ACC-017, ACC-019, ACC-040, ACC-041, IMP-006, the `onward request` and `later work`
rows of `definitions.md`, rule 4 of "The form of a promise" in `spec/README.md`.

ACC-017 and ACC-041 make a chain a set of invocations: "The chain is the invocation it names
and every invocation that one served, directly or through others". ACC-019 and IMP-006 speak
of its members as Conduits: "to every other Conduit of its chain", "the Conduit, and the chain,
of the invocation that the step's graph serves"; ACC-040 has "once a Conduit of the chain is
closed". `definitions.md` uses the word in both ways without defining it: "the target of each
invocation in the chain" in one row, and "every Conduit of its chain, which are the Conduits
that the invocations the arranging request served came through" in another.

Fix: one row. "chain: the invocation a request names as served and every invocation that one
served, directly or through others, going on through the invocation that arranged the work
where one of them is a start of later work (ACC-017, ACC-041). The Conduits of a chain are the
Conduits those invocations were dispatched through (ACC-019)."

### 4B-8. Two rows of `definitions.md` are narrower than the promise they cite

Severity: `term`.
Involved: the `hold` row and TPL-007, TPL-015, OBS-007; the `supports` row and MAN-012.

`hold` gives two ends to the holding of a runtime: "until a stop of the runtime has succeeded
or has been reconciled as having occurred (TPL-007)". TPL-007's `then` has three: "until a
stop of the runtime has succeeded or has been reconciled as having occurred, or the engine
treats the handle as no longer held under OBS-007". The third is not a case of the first two:
TPL-015 says that after a stop under OBS-007 is reconciled as not having occurred, "where
OBS-007 had the engine treat the handle as no longer held, it stays so". By the row alone the
instance would still hold the runtime.

`supports` is "The key of an implementation item, of one that is reached by `command` or
`url`". MAN-012 gives the key to the one item that is reached by neither: "Each item states
`implements`, may state `supports` and `placement`", and `spec/core/conduit.yaml` uses it for
`bound.v1`.

Fix: add "or the engine treats the handle as no longer held (OBS-007)" to the `hold` row, and
"or of the core package's `engine` (MAN-012)" to the `supports` row.

### 4B-9. `replace.v1` declares an output `revision` that nothing describes

Severity: `minor`.
Involved: `spec/core/operations.yaml` (`replace.v1`, `rename.v1`), SUB-009, OBJ-026.

`replace.v1` declares `outputs: revision`, required. SUB-009 does not mention it, and neither
does the operation's own description. `rename.v1` advances the revision in the same way
(OBJ-026) and declares no output, so the two changes of a stored object disagree on whether the
result tells the new revision. For a replace that changes nothing, the text does not say which
value the required output holds.

Fix: one sentence in the description of `replace.v1`, "`revision` is the template's revision
after the request, which is the one it had where nothing changed", or drop the output so that
it matches `rename.v1`.

### 4B-10. `spec/core/README.md` gives the key `serving` one of its two uses

Severity: `minor`.
Involved: `spec/core/README.md`, IFC-020, IFC-024, OBS-007, the `Conduit's keys` row of
`definitions.md`.

All 14 keys that the promises use are in the `Conduit's keys` row, each with the meaning the
promises give it. The second place that enumerates keys is narrower: "The keys of a request's
`_meta` are those of IFC-006: ... with `operationIdentity` and `conduit` where they apply, and
`serving` on an onward request (IFC-020)". IFC-024 also puts `serving` on a call to
`retrieve-result.v1`, which the same README states three paragraphs earlier, and the sentence
reads `serving` as one of the keys of IFC-006, which IFC-020 says the engine "does not send
on". It also omits that `caller` is absent from the one request of OBS-007.

Fix: "... and, on a request that an instance sends to the engine, `serving`, on an onward
request (IFC-020) and on a retrieval of the result of one (IFC-024)".

### 4B-11. Status wording: `promises/manifest.yaml` marks only round 3 as the coordinator's

Severity: `minor`.
Involved: `spec/promises/manifest.yaml`, and the same pattern in the heads of the other
promise files, of `spec/implementation-defined.yaml`, and in the opening of `definitions.md`.

`spec/README.md`, both READMEs of the batch and the head of every file under `spec/core/` and
`spec/schemas/` say the whole of it: the owner accepted the part as it stood, and what the docs
audit and rounds 2 and 3 changed since is the coordinator's. The third part of the same batch
does not. Its head says "Owner ruling of 2026-11-05 (plan-audit ruling 37), which accepted
these promises as one batch", then lists what the docs audit and the three passes of round 2
changed in MAN-001, MAN-003, MAN-006, MAN-007 and MAN-010 with no word on their standing, and
attaches "which the owner has not ruled on" to the round-3 changes alone. Read on its own, the
contrast presents the earlier changes as part of what was accepted.

The other promise files open "Owner rulings of 2026-11-05 (...)" and follow the same pattern:
round-3 changes carry the note, docs-audit and round-2 changes do not. Three have no note at
all although round 2 changed or added promises in them: `promises/schemas.yaml`,
`promises/first-start.yaml` (FST-006 added) and `conventions/implicit.yaml` (IMP-010 to IMP-012
added). `spec/implementation-defined.yaml` marks only its last change. In `definitions.md`,
"The names of the core package's exports ... are those of the ruling on the core package and
the schemas" covers `start-later-work.v1`, which `spec/core/README.md` says the coordinator
added; the same paragraph attributes its row correctly further down.

Fix: in `promises/manifest.yaml`, the sentence the core files carry: "What the docs audit and
rounds 2 and 3 of the re-audit changed since rests on the coordinator's logged decisions,
which the owner has not ruled on." For the other heads, either the same sentence, or one
sentence in `spec/README.md` saying that a head comment marks the standing of round 3 only and
that the docs audit and round 2 have the same standing. In `definitions.md`, "except
`start-later-work.v1`".

## What I checked and found consistent

Terms and vocabulary rules:

- The words `definitions.md` rules out: "grant", "declaration" and "create" for an operation
  occur nowhere in a promise ("grant object" once in a head comment that explains the rule);
  "manifest item" nowhere; "answer" only as a verb; "implementation" never for an engine;
  "source" only as the key of a value reference in the promises; "entry" only for a Pool's
  entry in the promises; "target" only for that end of a Conduit; "owner" never for a person
  in a promise; "plain" only for an implementation; "hold" never for a Conduit or a right;
  `required` only as the schema keyword; no "composed" or "composition" for a Conduit template.
- No second word for a request's seven ends, for unknown, for unregister, for consumer or for
  caller.
- Every row of `definitions.md` against the promises it cites, apart from 4B-3, 4B-4 and 4B-8.
- Every promise ID cited anywhere under `spec/` exists, or is a removed promise that the head
  of its file names; all 15 `DL` items exist.

Counts:

- 437 promises, and the count of each of the 22 files as `spec/README.md` states it; 15
  implementation-defined items; the layers as the README describes them.
- Core package: 21 operations, 5 capabilities, 6 types, 2 requirements, 11 reference schemas,
  2 result schemas, 7 templates, 45 schemas in 9 files; 99 exports, 58 of them Schema
  contracts. `spec/core/conduit.yaml` lists exactly the documents that exist, `engine`
  implements exactly the 21 operations, and the table "Every export" has one row for each.
- 7 valid example files in 3 directories, 61 invalid, 20 invalid-by-rule; the file lists of
  `spec/schemas/README.md` name the contracts of each file in order.

Core package against the promises:

- Every operation, input and output a promise names exists with that name: `documents` and
  `manifest` of `register.v1`; `manifest` and `revision` of `replace.v1`; `name` of `rename.v1`
  with the rule for names as its schema (OBJ-028); `instance` of `set-up.v1` and `retire.v1`;
  `invocation`, `revision`, `settings` and the handle `handle` of `reconcile.v1` (OPS-014,
  REV-010, TPL-015, OPS-031); `address`, `version`, `digest` and `fetch` of `install.v1`;
  `before` of `purge.v1`; `actors`, `targets`, `arguments` and the optional `version` of
  `apply-policy.v1`, which returns an ID that `remove-policy.v1` names; `operationIdentity`,
  `target`, `state`, `invocation`, `withheld` and `outputs` of `retrieve-result.v1`;
  `invocation`, `operation`, `arguments` and `outputs` of `start-later-work.v1`; `location` of
  `set-default-location.v1` and `facts` of `set-facts.v1`; the six outputs of `inspect.v1`.
- `retire-with-resources.v1` has no input and no output, as ACC-033, ACC-034 and LIF-021 say;
  `retrieve-result.v1` and `start-later-work.v1` are in no capability.
- Marks: `reservedToRoot` on `declare-lost.v1` and `purge.v1` alone; no `startsModelWork` or
  `arrangesLaterWork`; no input states a `default`; every operation states `engine`; the
  columns of the operations table in `spec/core/README.md` match the contracts for all 21.
- The three capabilities that hold management verbs list what ACC-033 authorizes against each
  target, `reconcile.v1` in all three; the two settings capabilities hold one control each.
- Each type's `requires` is the operations of the capabilities it offers; each control states
  `changes` to a place its type's schema describes and declares `revision` and `settings`.
- `bound.v1` and `shared-host.v1` against ACC-029, CON-028, CTR-011, PLC-013; the 11 reference
  schemas against OBJ-006, OBJ-013, POOL-005, POOL-014 and the resolved form (tested with
  values of each form).
- The seven templates against FST-001, FST-002, FST-005 and ACC-022.

Schemas against the promises:

- `manifest.v1` and `implementation-item.v1` against MAN-003 to MAN-010 and PKG-017 to PKG-020,
  apart from 4B-2 and 4B-5: the keys, `address` and `version` together, the items of
  `packages`, `exports` and a re-export, exactly one of `command`, `url` and `graph`, `plain`
  with `tools` and without `supports`, and a `graph` item with no `supports`, `tools` or
  `plain: true`.
- The five contract kinds against CTR-001 to CTR-006, CTR-008 and CTR-010; the three template
  `spec` shapes against TPL-001, TPL-002, CON-016 and the closed list of INS-012 (each example
  destination of INS-012 tested); `template-input.v1`, `runtime.v1`, `template-controls.v1`.
- `instance.v1` against OBJ-013, PLC-006 and POOL-001; the three nested instantiations against
  OBJ-014; `template-reference.v1` against the four forms of INS-025.
- `graph.v1`, `lifecycle-graph.v1`, `step.v1`, `value-binding.v1` and `value-reference.v1`
  against GRA-017 to GRA-021 and GRA-023.
- `schema.v1` against the profile's keyword and format lists, the three forms of `$ref`, and
  the placement of `default` under SCH-008 (twelve cases tested).
- The value schemas against the examples the promises give: every accepted and rejected
  address of NAM-001, the names of CTR-001 and NAM-004, NAM-005, the pointers of OBJ-016,
  versions and digests.
- `object.v1` against OBJ-003. `diagnostic.v1` and the open parts of `inspect.v1` are the
  known open points.

Examples:

- Valid project: every template and instance against the rules a schema cannot express that
  the documents in front of me decide: destinations and their parents, selections and the
  capabilities' operations, the runtimes of type and template, the setup graph's sources,
  the entry reference and the reference to a Pool's entry, the composite and its component
  names, tool names, the bound on a Conduit to `access`, placement by PLC-002, unique names.
- Valid package: exports against documents, the three implementations, a default for every
  operation TPL-004 names, the composition and its constituent, the control, the type's
  runtime, and later work: `directory.sweep.v1` is marked, its implementation is not plain,
  its capability also includes the operation each start invokes, no graph step names it, and
  its contract says how the result is read, as ACC-019, ACC-040, TPL-014 and IMP-010 require.
- First-start documents against FST-001, FST-005 and PLC-008, with the header's statement that
  a submission of them is rejected.
- Each of the 61 invalid examples is rejected, and becomes valid after the one change its
  header names (59 checked that way; the duplicate key is rejected when the file is read, and
  the unknown kind has no repair). Each of the 33 documents under `invalid-by-rule` is valid
  against the schemas, and each set breaks the rule it is named for and no other rule that the
  documents in the set decide.

The implementation interface:

- The 14 `_meta` keys the promises use, each against the `Conduit's keys` row: one meaning
  each, with the two places `reason` and `restrictions` stand outside a request or a result.
