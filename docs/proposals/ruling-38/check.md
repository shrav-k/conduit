# Ruling 38: check of the changes to spec/

Date: 2026-11-06. A reviewer read the uncommitted changes to `spec/`, `tools/` and
`docs/OVERVIEW.md` against `HEAD` (the specification before the ruling). The work order is in
[the record of the ruling](README.md#work-order-for-the-editor-of-spec) and the account of how
it was applied is in [the log](log-spec.md). Every promise that the diff changed was read in
full, beside its text at `HEAD`, and so were the promises that cite them.

## Summary

| Severity | Count |
| --- | --- |
| unsafe | 0 |
| leftover | 2 |
| contradiction | 5 |
| lost | 1 |
| inaccurate-log | 1 |
| minor | 5 |
| **Total** | **14** |

No finding is unsafe. The removal of later work is complete in the promises, the core package,
the schemas, the examples and the checker. Two leftovers remain in prose (R38-1, R38-2). The
most serious point is the composite Conduit (R38-3, R38-4): the new wording of CON-004 gives
each component's opening its own outcome, so a composite is no longer opened "whole or not at
all", which the owner confirmed in point 3 and which GRA-003 and `set-up.v1` still state.

## Output of the checks

Run from the repository root on the working tree.

```text
$ /usr/bin/python3 (the validation script of docs/proposals/reaudit/CONSOLIDATION.md)
434 promises ok   (language 220, convention 179, interface 35)

$ python3 tools/check_schemas.py
Python 3.9.6; PyYAML 6.0.1; jsonschema 4.20.0
... rejected CTR-002-operation-arranges-later-work.yaml: at /spec: Additional properties are
    not allowed ('arrangesLaterWork' was unexpected)
6. Applied 23 rules of this tool, which come from 21 promises, ...
Errors: 0

$ python3 tools/set_names.py --check
Names: corePackage x435, apiVersion x241, metaPrefix x90. Errors: 0

$ python3 tools/check_docs.py
docs/AUDIT.md:98, 135, 211, 556: missing path proposals/secrets/README.md
docs/README.md:76: missing path proposals/secrets/README.md
docs/proposals/ruling-38/README.md:32: missing path ../secrets/README.md
docs/proposals/ruling-38/log-spec.md:64: missing path check.md   (this file; now written)
Errors: 7   (all expected: the secrets proposal is being written, and this file was missing)

$ git diff --check
(no output, status 0)
```

Further checks made by the reviewer:

- Promises removed against `HEAD`: ACC-040, ACC-041, IFC-035, IMP-006, IMP-010, IMP-011,
  IMP-012, TPL-014. None added. 31 changed: ACC-001, ACC-005, ACC-015, ACC-017, ACC-019,
  ACC-026, ACC-031, ACC-033, ACC-039, CON-004, CON-024, CTR-002, IFC-006, IFC-012, IFC-014,
  IFC-016, IFC-018, IFC-021, IFC-025, IFC-032, IFC-034, IMP-008, LIF-028, LIF-031, OPS-022,
  OPS-024, OPS-029, PKG-020, REC-004, SUB-010, TPL-004.
- Every promise ID cited under `spec/` and in `docs/OVERVIEW.md` exists, except IDs cited only
  in YAML head comments that record history or removal (the eight removed ones and older
  removed ones such as CON-005 or POOL-010). No removed ID is cited outside a comment.
- DL1 to DL15 exist, and every DL item cited exists.
- Every count in the contents tables of `spec/README.md`, `spec/core/README.md` and
  `spec/schemas/README.md` agrees with the files: 434 promises and each file's count; 98
  exports (20 Operation, 5 Capability, 6 Type, 2 Requirement, 7 templates, 58 Schema); 63
  invalid examples, 24 invalid-by-rule files, 7 valid files.
- One YAML line under `spec/` is longer than 96 characters: R38-11.
- A search under `spec/`, `tools/` and `docs/OVERVIEW.md` for `arrangesLaterWork`,
  `start-later-work`, "later work", "arrang", "a start", "two tools", "third case", "outlast"
  and "sweep" finds them only where reported below, in head comments, in the schema comment,
  in the invalid example, and in text that says later work is not part of this version. "A
  start" otherwise means only the start of a runtime.
- `tools/check_schemas.py` never had a rule on later work, at `HEAD` or now; the schema change
  alone rejects the mark, and the new invalid example shows it.

## Findings

### R38-1 leftover: the overview still speaks of two tools without a Conduit

- File: `docs/OVERVIEW.md`, line 330. Promise: IFC-012.
- Quote: "the engine presents to each consumer, as tools, the operations its open Conduits
  list and the two tools that need no Conduit (IFC-012)."
- The same page was fixed at line 98 ("One tool of the core package is the exception").
- Least change: "and the one tool that needs no Conduit, `retrieve-result.v1` (IFC-012,
  IFC-024)."

### R38-2 leftover: the schemas README still lists later work among the meanings of a graph

- File: `spec/schemas/README.md`, line 192 (the row "The meaning of a graph").
- Quote: "... handles, model work, later work | GRA-004, ... IMP-004, IMP-005, IMP-009".
- The promises IMP-010 and IMP-012 were taken out of the second column, but the words stayed
  in the first.
- Least change: end the first column at "handles, model work".

### R38-3 contradiction: "whole or not at all" survives beside the new CON-004

- Files: `spec/promises/graphs.yaml` (GRA-003, not changed by the diff),
  `spec/core/operations.yaml` (description of `set-up.v1`, line 326). Promises: CON-004,
  LIF-027, LIF-028.
- GRA-003: "A declared Conduit that is made from a composite template is opened as a whole or
  not at all, under CON-004."
- `set-up.v1`: "its component Conduits are opened all together or not at all", followed, in
  the same description, by the new "each an invocation of its own".
- CON-004 now: "Where the outcome of one of them is failed or unknown, the state of its Conduit
  is as it was, under LIF-004, a Conduit that another of them opened stays open, under
  LIF-027". So once the openings are dispatched, some components can be open and others not.
  Only the decision is all or none.
- Least change: depends on R38-4. If the owner accepts per-component outcomes, write in both
  places "the opening of every component is decided before any is opened, and where one would
  not be opened none is", and drop "as a whole" and "all together". If not, restore the single
  outcome in CON-004.

### R38-4 lost: a composite's opening at setup no longer has one outcome

- File: `spec/promises/conduits.yaml` (CON-004), `spec/conventions/lifecycle.yaml` (LIF-028),
  `spec/conventions/operations.yaml` (OPS-022). Record: [README.md](README.md), points 3 and
  "What follows from each", item 3.
- At `HEAD`, CON-004: "Otherwise all of them are opened, in one invocation with one outcome",
  "Where that outcome is unknown it is unknown for the whole", and "the reconciliation of the
  invocation under LIF-028 records every one of them as open or every one of them as not
  open". LIF-028 at `HEAD`: "recording that the opening did not occur leaves every one of them
  as it was."
- Now each component's opening is its own invocation with its own outcome and its own
  reconciliation. That is a change of rule, not of wording: a composite can end partly open
  after a failure or an unknown outcome, which it could not before. The record says of point 3
  "CON-004 is reworded. No rule changes", and the owner confirmed "A composite Conduit opens
  whole or not at all at setup".
- The work order's own wording ("one request for `open.v1` is made for each, or none") leads
  here, and the log names it as a choice ("Each component's opening is an invocation of its
  own, with its own outcome"). But neither the log nor the record says that the all-or-none
  outcome was given up.
- Least change: put the point to the owner, or at least say in the log, under "Choices", that
  the single outcome and the all-or-none reconciliation of round 4 are gone, and that "whole or
  not at all" now holds of the decision only. Then apply R38-3.

### R38-5 contradiction: CON-004 adds an identity rule that OPS-028 and OPS-021 do not know

- Files: `spec/promises/conduits.yaml` (CON-004), `spec/conventions/operations.yaml` (OPS-021,
  OPS-028).
- CON-004: "each carries an operation identity of its own, which the engine derives as OPS-028
  has it derive a step's, and from the component's name as well."
- OPS-028: the identity is "derived from the ID of the invocation in which the step is
  dispatched and from the step's name", and "A step of a setup ... keeps the identity that an
  earlier run gave it only while that run's invocation of the step is in progress or its
  outcome is unknown". One step now has several invocations, and OPS-028 and OPS-021 speak of
  "the step's" one invocation and one outcome.
- Least change: in OPS-028, add that a step that opens a composite declared Conduit has one
  invocation for each component, each with an identity derived also from the component's name,
  and that what OPS-021 and this promise say of the step's invocation holds of each.

### R38-6 contradiction: which targets the warning of `apply-policy.v1` names

- Files: `spec/conventions/access.yaml` (ACC-031, ACC-015, ACC-026), `spec/core/operations.yaml`
  (`apply-policy.v1`), `spec/core/README.md`, `spec/definitions.md` (row "policy").
- ACC-031 `then`: the result names "each target that the policy reaches and whose
  implementation does not support the policy's requirement." A policy that gives only `actors`
  reaches every target, Resources and Agents among them.
- ACC-031 `observable`: "For a policy that gives none, it names `management`, `access` and every
  Pool that exists when the policy is applied". A Resource whose selected implementation is
  `plain`, or does not state the requirement in `supports`, is reached and is not named.
- ACC-015 and ACC-026 name a location among the targets whose implementation is `engine`: "made
  of `management`, of `access`, of a Pool or of a location". A location is not named by the
  warning for a policy with no `targets`. ACC-026 now says "The result of the request that
  applied the policy names each such target", which is then not so for a location.
- The work order gives "`management`, `access` and every Pool", so the narrowing is the
  owner's; the `then` was not narrowed to match, and locations were left out.
- Least change: in ACC-031's `then`, say what is named for a policy that gives no `targets`;
  add "and every location" there, in the observable, in `apply-policy.v1`, in the core README
  and in the definitions; or, if every reached target is meant, drop the narrowing in the
  observable.

### R38-7 contradiction: ACC-019 forbids what IFC-024 permits

- Files: `spec/conventions/access.yaml` (ACC-019), `spec/interface/implementation.yaml`
  (IFC-021, IFC-024, IFC-028).
- ACC-019 is `must not`, `when` "An invocation has ended", `then` "The engine dispatches a
  request on its behalf: one that names it in `io.github.shrav-k.conduit/serving`, ...".
- IFC-024 and IFC-028 let a call to `retrieve-result.v1` name an ended invocation in that key,
  and its caller is that invocation's caller. ACC-019's own observable makes the exception ("The
  one call that may name it there is a retrieval"), so the rule and its observable disagree.
- The observable also says "An instance that is not an actor makes a request only to serve an
  invocation that is in progress, under IFC-021 and IFC-028", which IFC-021 contradicts for
  `retrieve-result.v1`.
- Least change: in the `then`, write "a request, other than a call to `retrieve-result.v1`, that
  names it in `io.github.shrav-k.conduit/serving`"; in the observable, add "a retrieval apart".

### R38-8 contradiction: the "Standing" paragraphs were not updated

- Files: every promise file's head comment, `spec/implementation-defined.yaml`, the core and
  schema files. Work order item "Standing".
- The work order asks that "the 'Standing' paragraph at the head of each file" say that ruling
  38 accepted the decisions of points 9, 10 and 12, made those of points 3, 6, 7 and 8, and that
  every other change since ruling 37 is still the coordinator's.
- What was done: files that the diff changed got the line "Ruling 38 of the owner ... changed
  this file". Their "Standing" paragraph still ends "the owner has not ruled on it", which is
  no longer true of, for example, ACC-039 (point 9) in `access.yaml` or LIF-031 (point 10) in
  `lifecycle.yaml`. Files with decisions accepted under point 12 but no change, such as
  `objects.yaml` (OBJ-026, a rename advances the revision), `schemas.yaml` (defaults of a
  schema's properties) and `graphs.yaml`, say nothing. `implementation-defined.yaml`, whose DL7
  changed under point 8, has no note of ruling 38 at all.
- `spec/README.md` does say it, so the heads disagree with it.
- Least change: append to each "Standing" paragraph one sentence: "Ruling 38 (2026-11-06)
  accepted the coordinator's decisions of its points 9, 10 and 12 and made those of points 3,
  6, 7 and 8; every other change since ruling 37 is still the coordinator's." Add the ruling-38
  line to `implementation-defined.yaml`.

### R38-9 minor: spec/README counts point 11 as accepted

- File: `spec/README.md`, lines 19 to 24.
- Quote: "That ruling accepted some of those decisions: a retired consumer's Conduits staying
  open ..., the resolved form ..., the interface following the Model Context Protocol on
  errors, and the eight changes of its point 12."
- The record lists points 9, 10 and 12 as accepted; point 11 is "A note for the next reader in
  the tracker".
- Least change: drop "the interface following the Model Context Protocol on errors", or say
  that point 11 was noted for the audit.

### R38-10 minor: spec/README does not say where it describes the conventions that later work is out

- File: `spec/README.md`, line 46 (the row `convention` of the layers table).
- The work order: "`spec/README.md` says, where it describes the conventions, that later work
  is not part of v0." It says so only in the paragraph on what "accepted" covers (line 24).
- Least change: add to the row `convention`: "Work that outlasts its invocation is not part of
  this version (ACC-019)."

### R38-11 minor: one YAML line is longer than 96 characters

- File: `spec/conventions/implicit.yaml`, line 26, 98 characters: "# IMP-011 and IMP-012, on
  later work, which is not part of this version. Those IDs are not reused."
- Least change: rewrap the comment.

### R38-12 inaccurate-log: lists and checks in the log

- File: `docs/proposals/ruling-38/log-spec.md`.
- OPS-029 changed (the sentence on ACC-040 and `start-later-work.v1` was removed) and is in no
  list of the log.
- OPS-022 is listed under point 6, "Changed so that nothing speaks of later work", but its only
  change is the composite Conduit of point 3: "where it makes a request for `open.v1` for each
  component Conduit, each of those invocations counts here as a step does".
- "No YAML line under `spec/` is longer than 96 characters" is not so (R38-11).
- The log says each changed promise file got a note; `implementation-defined.yaml` (DL7) did
  not, and the "Standing" item was applied only in part (R38-8). The counts in its table are
  correct.
- Least change: add OPS-029 to the list of point 6, move OPS-022 to point 3, correct the
  sentence on line length once R38-11 is fixed, and record R38-4 under "Choices".

### R38-13 minor: older head lines say no promise was removed

- Files: `spec/promises/templates.yaml` line 17, `spec/interface/implementation.yaml` line 30.
- Quote: "added TPL-014. No promise of this file has been removed." and "No promise of this file
  has been removed."
- Each was true when written, and the ruling-38 line below now says TPL-014 and IFC-035 were
  removed, so a reader meets both.
- Least change: "No promise of this file was removed in that pass."

### R38-14 minor: the core README undercounts what ruling 38 changed there

- File: `spec/core/README.md`, line 33.
- Quote: "Ruling 38 made two changes here", naming later work and the warning. The diff also
  adds the note on `open.v1` and a composite (point 3, line 105) and the directory under
  "Installing" (point 8).
- Least change: "Ruling 38 made changes here, which are the owner's decisions: ..." and name
  all four.

## Checked and found consistent

- Every removed promise is named in the head of its file, with the ruling, and the IDs are not
  reused.
- ACC-019 keeps its ID, is `must not`, and states the rule of the work order; IFC-016 stays,
  and "chain" is kept for onward requests (ACC-017, the definitions).
- IFC-012, IFC-014, IFC-021, IFC-025, IFC-032, IFC-034, ACC-033, CON-024, the definitions
  (management verb, tool, tool name, request, caller, the instance that sent a request, core
  package) and `core/capabilities.yaml` agree that one tool, `retrieve-result.v1`, needs no
  Conduit.
- CTR-002, `operation-spec.v1`, the new invalid example and the example package agree; no
  valid example holds the mark, and the example package's manifest, capability and type no
  longer list the two sweep operations.
- REC-004 and the description of `purge.v1` agree on what a purge keeps.
- IMP-008 lost only its case for a start; ACC-039 lost only its sentence on started work; no
  model-work rule mentions a later start.
- SUB-010, DL7, PKG-006, NAM-012, PKG-028, the definitions (project, install) and the core
  README agree that a directory installs nothing.
- ACC-005, LIF-027, LIF-028 and OPS-022 agree with the new CON-004, apart from R38-3 to
  R38-5.
- No rule that was not about later work disappeared, apart from R38-4.
