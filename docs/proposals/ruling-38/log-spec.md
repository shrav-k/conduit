# Ruling 38: log of the changes to `spec/`

Date: 2026-11-06. This log is the account of how the work order in
[the record of ruling 38](README.md#work-order-for-the-editor-of-spec) was applied. The ruling
is the owner's. The wording that carries it out is the editor's and the coordinator's, and
stands unless the owner strikes it.

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

An editor session began the work and stopped partway, at a usage limit, with no log. The
coordinator finished it and wrote this log from the difference between the specification
before the ruling (commit `ff91aa5`) and after it.

| | Before | After |
| --- | --- | --- |
| Promises | 442 (223 language, 183 convention, 36 interface) | 434 (220 language, 179 convention, 35 interface) |
| Core package exports | 99 | 98 |
| Operation contracts of the core package | 21 | 20 |
| Invalid examples | 62 | 63 |
| Rules of `tools/check_schemas.py` | 23 from 21 promises | 23 from 21 promises |

## By item of the work order

| Item | Promises | Documents |
| --- | --- | --- |
| Later work is dropped from v0 (point 6) | Removed: ACC-040, ACC-041, IMP-006, IMP-010, IMP-011, IMP-012, IFC-035, TPL-014. ACC-019 keeps its ID and is rewritten: once an invocation has ended, the engine dispatches no request on its behalf, and work that is to happen later is asked for later by an actor. Changed so that nothing speaks of later work: ACC-001, ACC-017, ACC-033, ACC-039, IMP-008, LIF-031, OPS-024, OPS-029, REC-004, IFC-006, IFC-012 (one tool needs no Conduit, not two), IFC-014, IFC-016, IFC-018, IFC-021, IFC-025, IFC-032, IFC-034, CON-024, CTR-002, PKG-020, TPL-004 | `start-later-work.v1` removed from `core/operations.yaml`, `core/conduit.yaml` and the core README; `arrangesLaterWork` removed from the schema of an Operation contract, which now rejects it; the two sweep operations removed from the example package; a new invalid example, `CTR-002-operation-arranges-later-work.yaml`; the definitions' row for later work says it is not part of this version, and the rows for request and caller lose the third case; the schemas README, the head comments and the overview follow |
| The warning of `apply-policy.v1` (point 7) | ACC-031 states the diagnostic that does not prevent the policy; ACC-015 and ACC-026 point to it | The description of `apply-policy.v1`; the overview |
| A directory installs nothing (point 8) | SUB-010 | DL7 of the implementation-defined list loses "whether it installs the packages" |
| The wording on a composite Conduit (point 3) | CON-004, and ACC-005, LIF-028 and OPS-022 where they repeated it: each component is opened by a request of its own, after the opening of every one has been decided, and after setup each component is a Conduit like any other | — |
| Standing | — | `spec/README.md` says what ruling 38 accepted and what it changed, and that later work is left out of this version; each changed promise file has a note in its head that names this log and the IDs it removed; the "Standing" paragraph of every promise file and of the implementation-defined list, and the heads of the core documents, say that ruling 38 accepted some of the coordinator's decisions |

## Choices

| Where | Choice |
| --- | --- |
| ACC-019 | Its `level` became `must not`, so that the promise states what the engine does not do. Its layer is unchanged. |
| The composite Conduit | Each component's opening is an invocation of its own, with its own outcome and its own reconciliation. The step succeeds when every one has succeeded. Where one fails or is unknown, those that opened stay open, under LIF-027. This keeps "nothing is closed" from round 4. It gives up something round 4 had: one outcome for the whole opening, and a reconciliation that records every component as open or every one as not open. What is still decided as a whole is whether any component is opened at all. The reason: `open.v1` names one Conduit, and the owner ruled that after setup each component is a Conduit like any other; a single outcome for several invocations would need a new operation. Put to the owner in the summary of this ruling's work. |
| Removed IDs | Named in the head of the file that held them, with the ruling, and not reused. |

## Checks

Run from the repository root after the last edit:

```text
$ /usr/bin/python3 (the validation script of CONSOLIDATION.md)
434 promises ok: language 220, convention 179, interface 35

$ python3 tools/check_schemas.py
Errors: 0

$ python3 tools/set_names.py --check
Errors: 0

$ python3 tools/check_docs.py
Errors: 0
```

After the fixes of [the check](check.md), no YAML line under `spec/` is longer than 96
characters. A search under `spec/`, `tools/` and
`docs/OVERVIEW.md` for `arrangesLaterWork`, `start-later-work` and the removed IDs finds them
only in head comments that record their removal, in the schema comment that says the key is
rejected, and in the invalid example.

## The check of the changes

A reviewer read these changes before the merge and reported fourteen findings, none unsafe
([report](check.md)). Each was fixed by the coordinator:

| Finding | What was done |
| --- | --- |
| R38-1 The overview's "two tools" | One tool, `retrieve-result.v1`. |
| R38-2 "Later work" in the schemas README | Removed. |
| R38-3 GRA-003 and `set-up.v1` said "whole or not at all" | Both say the opening is decided as a whole, and that each opened component has its own invocation and outcome. |
| R38-4 The single outcome of a composite's opening was given up | Kept as the choice above, which now says what was given up, and put to the owner. |
| R38-5 OPS-028 and one request per component | OPS-028 says how each request's identity is derived. |
| R38-6 ACC-031's list of targets for a policy with no `targets` | It names every target that cannot apply the policy: always `management`, `access`, every Pool and every location, and every instance whose implementation is plain. |
| R38-7 ACC-019 against a retrieval | ACC-019 excepts a call to `retrieve-result.v1` in its `then` and its observable. |
| R38-8 The "Standing" paragraphs | Updated in every promise file, the implementation-defined list and the core documents. |
| R38-9 Point 11 listed as accepted | The work order of the ruling now names points 9 to 12, as `spec/README.md` does. |
| R38-10 Later work not said where the layers are | `spec/README.md` says it after the two things outside the specification. |
| R38-11 A line of 98 characters | Wrapped, with two more that the fixes made long. |
| R38-12 This log | Corrected: OPS-029 added, OPS-022 moved, the claim on line length made true, the standing row widened. |
| R38-13 "No promise of this file has been removed" | "Was removed before ruling 38". |
| R38-14 "Two changes" in the core README | Four, each named. |

## The check of the handoff

A second reviewer read the repository at commit `96ed241` as the next reviewer will, and
reported twelve findings ([report](handoff-check.md)). Each was fixed by the coordinator:

| Finding | What was done |
| --- | --- |
| H-1 The composite Conduit is not in the open lists | Listed in the tracker's "Waiting for the owner" and "Known to be open"; the record of ruling 38 and the round-4 record point to it. |
| H-2 "Read by their editors alone" | The tracker says the changes of ruling 38 were checked against the ruling and not read cold. |
| H-3 The done-when of A42 | Restated as ruling 38 restated G7. |
| H-4 The schemas' heads and README | Say what ruling 38 accepted or made. |
| H-5 "Eight changes listed under later amendments" | The ruling's row says which are there. |
| H-6 "All rulings given on 2026-11-05" | Ruling 38 is dated 2026-11-06. |
| H-7 The reading of "Understood" | The record says where the gate is stated. |
| H-8 A private name in `AGENTS.md`; "three readers' reports" | Removed; two. The record of the ruling says where the names remain. |
| H-9 The squash message | Written by hand, with no session link and the owner's no-reply address. |
| H-10 Links pinned to `7927d68` | Listed for the owner: they work while the branch or the pull request exists. |
| H-11 "Round 4 is next" | "Followed". |
| H-12 An empty heading and a stale check line | Removed and corrected. |
