# Each promise's standing in the first build

Not binding. The [specification](../spec/README.md) alone binds. This file is generated: do not
edit it by hand. `python3 tools/promise_standing.py --write` writes it from the promise files
under `spec/`, the coverage index under `conformance/` (its `index.yaml` pieces and the S1 rows
of its `manifest.yaml`), [the expected failures](../conformance/README.md#expected-failures) and
the sources of the crates `conduit-loader`, `conduit-core` and `conduit-shell`. CI runs
`python3 tools/promise_standing.py --check`, which fails when this file differs from what those
inputs give.

It says what evidence the tree holds for each promise, not whether the first build keeps the
promise. A case shows only the slice of a promise that its steps assert, and a reason is not a
pass ([the S1 inventory](../conformance/README.md#s1-whole-and-promise-inventory-i2-09)). A
source file that names a promise may keep it only in part, and code that names no promise may
still keep one. The first build states its choices and limits in
[its conformance choices](CONFORMANCE.md).

## The columns

- **ID**, **Layer** and **Level**: the promise's own. The rows are in byte order of ID.
- **Cases**: how many entries of the coverage index list the promise among their `promises`.
- **Expected failures**: how many of those cases `conformance/expected-failures.yaml` lists.
- **S1**: `cases` where the promise's row in the S1 inventory (`s1_coverage` in
  `conformance/manifest.yaml`) names cases, `reason` where it gives a reason instead, and
  `outside` where the promise has no row there.
- **Engine source**: which of the crates `loader`, `core` and `shell` have a `.rs` file under
  their `src/` directory that names the promise's ID, in code, a string, a comment or a unit
  test alike; `none` where no such file does. The conformance runner's crate and the crates'
  `tests/` and `examples/` directories are not read.
- **Standing**, from Cases, Expected failures and S1 alone:
  - `case passes`: at least one case lists the promise and is not an expected failure. The
    runner fails a run in which a case fails that the expected failures do not list, and CI
    runs every case on each pull request and each push to `main`.
  - `expected failure`: cases list the promise, and the expected failures list each of them.
  - `reason, no case`: no case lists the promise, and its S1 row gives a reason.
  - `no case, no reason`: no case lists the promise, and no reason is recorded.

## Totals

Each number counts promises. The last row counts those whose Engine source is not `none`.

| Promises | language | convention | interface | all |
| --- | ---: | ---: | ---: | ---: |
| case passes | 128 | 150 | 27 | 305 |
| expected failure | 0 | 0 | 0 | 0 |
| reason, no case | 58 | 17 | 1 | 76 |
| no case, no reason | 39 | 77 | 19 | 135 |
| all | 225 | 244 | 47 | 516 |
| named in engine source | 141 | 164 | 27 | 332 |

## The promises

| ID | Layer | Level | Cases | Expected failures | S1 | Engine source | Standing |
| --- | --- | --- | ---: | ---: | --- | --- | --- |
| ACC-001 | convention | must | 8 | 0 | cases | core | case passes |
| ACC-002 | convention | must | 1 | 0 | cases | core | case passes |
| ACC-003 | convention | must | 2 | 0 | cases | core | case passes |
| ACC-005 | convention | must | 9 | 0 | cases | core | case passes |
| ACC-006 | convention | must | 1 | 0 | cases | core | case passes |
| ACC-008 | convention | must | 1 | 0 | cases | core | case passes |
| ACC-009 | convention | must | 1 | 0 | cases | core | case passes |
| ACC-011 | convention | must | 3 | 0 | cases | none | case passes |
| ACC-012 | convention | must | 1 | 0 | cases | none | case passes |
| ACC-014 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ACC-015 | convention | must | 3 | 0 | cases | core | case passes |
| ACC-016 | convention | must | 4 | 0 | cases | core | case passes |
| ACC-017 | convention | must | 4 | 0 | cases | core | case passes |
| ACC-018 | convention | must | 0 | 0 | outside | core | no case, no reason |
| ACC-019 | convention | must not | 0 | 0 | reason | core | reason, no case |
| ACC-022 | convention | must | 0 | 0 | reason | core | reason, no case |
| ACC-023 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ACC-024 | convention | must not | 4 | 0 | cases | core | case passes |
| ACC-027 | convention | must | 0 | 0 | reason | core | reason, no case |
| ACC-028 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| ACC-029 | convention | must | 8 | 0 | cases | core | case passes |
| ACC-030 | convention | must | 9 | 0 | cases | core | case passes |
| ACC-032 | convention | must | 3 | 0 | cases | core | case passes |
| ACC-033 | convention | must | 44 | 0 | cases | core, shell | case passes |
| ACC-034 | convention | must | 2 | 0 | cases | core | case passes |
| ACC-036 | convention | must | 4 | 0 | cases | core | case passes |
| ACC-037 | convention | must | 2 | 0 | cases | core | case passes |
| ACC-038 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ACC-039 | convention | must | 0 | 0 | outside | core | no case, no reason |
| ACC-044 | convention | must | 18 | 1 | outside | core | case passes |
| ACC-053 | convention | must | 24 | 0 | outside | core, shell | case passes |
| ACC-054 | convention | must | 7 | 0 | outside | core | case passes |
| ACC-055 | convention | must | 12 | 0 | outside | core | case passes |
| ACC-056 | convention | must | 4 | 0 | outside | core | case passes |
| ACC-057 | convention | must | 1 | 0 | outside | core | case passes |
| CON-001 | language | must | 1 | 0 | cases | core | case passes |
| CON-002 | language | must | 2 | 0 | cases | core | case passes |
| CON-004 | convention | must | 8 | 0 | cases | core | case passes |
| CON-006 | language | must | 0 | 0 | outside | none | no case, no reason |
| CON-007 | convention | must | 0 | 0 | reason | none | reason, no case |
| CON-008 | convention | must | 2 | 0 | cases | core | case passes |
| CON-011 | convention | must | 1 | 0 | cases | core | case passes |
| CON-012 | language | must | 0 | 0 | reason | none | reason, no case |
| CON-013 | convention | must | 0 | 0 | reason | none | reason, no case |
| CON-014 | convention | must | 2 | 0 | cases | core | case passes |
| CON-016 | language | must | 1 | 0 | cases | none | case passes |
| CON-017 | language | must | 2 | 0 | cases | core | case passes |
| CON-018 | language | must | 0 | 0 | outside | none | no case, no reason |
| CON-020 | language | must | 0 | 0 | reason | none | reason, no case |
| CON-021 | language | must | 0 | 0 | reason | core | reason, no case |
| CON-022 | language | must | 1 | 0 | cases | core | case passes |
| CON-023 | convention | must | 1 | 0 | cases | core | case passes |
| CON-024 | convention | must | 3 | 0 | cases | core | case passes |
| CON-025 | language | must | 3 | 0 | cases | core | case passes |
| CON-027 | language | must | 0 | 0 | reason | none | reason, no case |
| CON-028 | language | must | 1 | 0 | outside | core | case passes |
| CON-031 | convention | must | 1 | 0 | reason | core | case passes |
| CTR-001 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-002 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-003 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-004 | language | must | 1 | 0 | cases | core | case passes |
| CTR-005 | language | must | 0 | 0 | outside | none | no case, no reason |
| CTR-006 | language | must | 1 | 0 | outside | core | case passes |
| CTR-007 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-008 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-009 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-012 | language | must | 0 | 0 | reason | none | reason, no case |
| CTR-013 | language | must | 4 | 0 | outside | core | case passes |
| CTR-014 | language | must | 6 | 0 | outside | core | case passes |
| ENR-001 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-002 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-003 | interface | must not | 0 | 0 | outside | none | no case, no reason |
| ENR-004 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-005 | language | must | 0 | 0 | outside | none | no case, no reason |
| ENR-006 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-007 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-008 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-009 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-010 | convention | must | 0 | 0 | outside | none | no case, no reason |
| ENR-011 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| ENR-012 | convention | must | 0 | 0 | outside | none | no case, no reason |
| FLEET-001 | language | must | 2 | 0 | outside | core | case passes |
| FLEET-002 | language | must | 1 | 0 | outside | core | case passes |
| FLEET-003 | language | must | 2 | 0 | outside | core | case passes |
| FLEET-004 | convention | must not | 2 | 0 | outside | core | case passes |
| FLEET-005 | convention | must | 11 | 0 | outside | core, shell | case passes |
| FLEET-006 | language | must | 2 | 0 | outside | core | case passes |
| FST-001 | language | must | 4 | 0 | cases | core | case passes |
| FST-002 | convention | must | 9 | 0 | cases | core | case passes |
| FST-003 | convention | must not | 2 | 0 | cases | core | case passes |
| FST-005 | language | must | 4 | 0 | cases | core | case passes |
| FST-006 | language | must | 4 | 0 | cases | core | case passes |
| FST-007 | convention | must | 7 | 0 | outside | core, shell | case passes |
| IFC-001 | interface | must | 2 | 0 | cases | core | case passes |
| IFC-002 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-003 | interface | may | 0 | 0 | outside | none | no case, no reason |
| IFC-004 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-005 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-006 | interface | must | 4 | 1 | outside | core, shell | case passes |
| IFC-007 | interface | must not | 4 | 0 | cases | core, shell | case passes |
| IFC-008 | interface | must | 2 | 0 | outside | core | case passes |
| IFC-009 | interface | must | 11 | 0 | outside | core | case passes |
| IFC-010 | interface | must | 27 | 0 | cases | core | case passes |
| IFC-012 | interface | must | 21 | 0 | cases | core, shell | case passes |
| IFC-013 | interface | must | 3 | 0 | cases | core | case passes |
| IFC-014 | language | must | 7 | 0 | cases | core | case passes |
| IFC-015 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-016 | interface | must | 1 | 0 | outside | core | case passes |
| IFC-017 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-018 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-019 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-020 | interface | must | 8 | 0 | cases | core | case passes |
| IFC-021 | interface | must | 2 | 0 | cases | core | case passes |
| IFC-022 | interface | must | 3 | 0 | cases | core | case passes |
| IFC-023 | interface | may | 0 | 0 | reason | none | reason, no case |
| IFC-025 | interface | must | 36 | 0 | cases | loader, core | case passes |
| IFC-026 | interface | must | 4 | 0 | outside | core, shell | case passes |
| IFC-027 | interface | must | 4 | 0 | outside | none | case passes |
| IFC-028 | interface | must | 4 | 0 | cases | core | case passes |
| IFC-029 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-030 | interface | must not | 0 | 0 | outside | none | no case, no reason |
| IFC-031 | interface | must not | 0 | 0 | outside | none | no case, no reason |
| IFC-032 | interface | must | 1 | 0 | cases | core | case passes |
| IFC-033 | interface | must | 28 | 0 | cases | core, shell | case passes |
| IFC-036 | interface | must | 1 | 0 | outside | core, shell | case passes |
| IFC-037 | interface | must | 1 | 0 | outside | core | case passes |
| IFC-038 | interface | must | 2 | 0 | cases | core, shell | case passes |
| IFC-039 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-040 | interface | must not | 0 | 0 | outside | none | no case, no reason |
| IFC-041 | interface | must | 1 | 0 | outside | core | case passes |
| IFC-042 | interface | must | 0 | 0 | outside | none | no case, no reason |
| IFC-043 | language | must | 0 | 0 | outside | none | no case, no reason |
| IFC-044 | language | must | 0 | 0 | outside | none | no case, no reason |
| IFC-045 | interface | must | 1 | 0 | reason | core, shell | case passes |
| IFC-046 | interface | must | 21 | 0 | cases | core | case passes |
| IFC-047 | interface | must | 2 | 0 | outside | core | case passes |
| IFC-049 | interface | must | 7 | 0 | outside | core, shell | case passes |
| IFC-050 | interface | must | 4 | 0 | outside | core | case passes |
| IMP-001 | convention | must | 0 | 0 | reason | none | reason, no case |
| IMP-002 | convention | must not | 5 | 0 | cases | core | case passes |
| IMP-003 | language | must | 0 | 0 | reason | none | reason, no case |
| IMP-007 | convention | must not | 5 | 0 | cases | core, shell | case passes |
| IMP-008 | convention | must | 0 | 0 | outside | none | no case, no reason |
| INS-001 | language | must | 0 | 0 | reason | none | reason, no case |
| INS-002 | language | must | 6 | 0 | cases | core | case passes |
| INS-003 | language | must | 2 | 0 | cases | core | case passes |
| INS-004 | language | must | 4 | 0 | cases | core | case passes |
| INS-005 | language | must | 3 | 0 | cases | core | case passes |
| INS-006 | language | must | 5 | 0 | cases | core | case passes |
| INS-007 | language | must | 2 | 0 | cases | core | case passes |
| INS-008 | language | must | 2 | 0 | cases | core | case passes |
| INS-009 | language | must | 2 | 0 | cases | core | case passes |
| INS-010 | language | must | 1 | 0 | cases | core | case passes |
| INS-011 | language | must | 1 | 0 | cases | core | case passes |
| INS-012 | language | must | 3 | 0 | cases | core | case passes |
| INS-013 | language | must | 5 | 0 | cases | core | case passes |
| INS-014 | language | must | 1 | 0 | cases | core | case passes |
| INS-016 | language | must | 11 | 0 | cases | core | case passes |
| INS-017 | language | must | 0 | 0 | reason | core | reason, no case |
| INS-018 | language | must not | 1 | 0 | cases | core | case passes |
| INS-019 | language | must | 5 | 0 | cases | core | case passes |
| INS-020 | convention | must | 5 | 0 | cases | core | case passes |
| INS-021 | language | must | 0 | 0 | reason | none | reason, no case |
| INS-023 | language | must | 2 | 0 | cases | core | case passes |
| INS-024 | language | must | 1 | 0 | cases | core | case passes |
| INS-025 | language | must | 2 | 0 | cases | core | case passes |
| LIF-001 | convention | must | 1 | 0 | cases | none | case passes |
| LIF-002 | language | must | 1 | 0 | cases | none | case passes |
| LIF-003 | convention | must | 4 | 0 | cases | core | case passes |
| LIF-004 | convention | must | 8 | 0 | cases | core | case passes |
| LIF-005 | convention | must | 0 | 0 | reason | core | reason, no case |
| LIF-006 | convention | must not | 4 | 0 | cases | core | case passes |
| LIF-007 | language | must | 0 | 0 | outside | none | no case, no reason |
| LIF-008 | convention | must not | 2 | 0 | cases | core | case passes |
| LIF-009 | convention | must | 9 | 0 | cases | core | case passes |
| LIF-010 | convention | must | 0 | 0 | outside | none | no case, no reason |
| LIF-011 | convention | must | 2 | 0 | outside | core | case passes |
| LIF-012 | convention | must | 3 | 0 | cases | core | case passes |
| LIF-013 | convention | must | 16 | 0 | cases | core, shell | case passes |
| LIF-014 | convention | must | 0 | 0 | outside | none | no case, no reason |
| LIF-015 | convention | must | 0 | 0 | outside | core | no case, no reason |
| LIF-016 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| LIF-017 | convention | must | 0 | 0 | outside | core | no case, no reason |
| LIF-018 | convention | must | 0 | 0 | outside | none | no case, no reason |
| LIF-019 | convention | must not | 2 | 0 | cases | core | case passes |
| LIF-020 | convention | must | 2 | 0 | outside | core | case passes |
| LIF-021 | convention | must | 11 | 0 | outside | core | case passes |
| LIF-022 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| LIF-023 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| LIF-024 | convention | must | 7 | 0 | cases | core | case passes |
| LIF-025 | convention | must | 0 | 0 | outside | core | no case, no reason |
| LIF-026 | convention | must | 0 | 0 | outside | core | no case, no reason |
| LIF-027 | convention | must not | 8 | 0 | cases | core | case passes |
| LIF-028 | convention | must | 4 | 1 | outside | core | case passes |
| LIF-029 | convention | must | 1 | 0 | cases | core | case passes |
| LIF-031 | convention | must | 1 | 0 | cases | core | case passes |
| LIF-032 | convention | must | 1 | 0 | reason | core | case passes |
| LIF-033 | convention | must | 0 | 0 | outside | none | no case, no reason |
| LIF-034 | convention | must | 7 | 0 | cases | core | case passes |
| LIF-035 | convention | must | 4 | 0 | cases | core, shell | case passes |
| LIF-036 | convention | must | 1 | 0 | outside | core | case passes |
| LIF-037 | convention | must | 4 | 0 | cases | core | case passes |
| LIF-040 | convention | must | 22 | 0 | outside | core, shell | case passes |
| LIF-041 | convention | must | 10 | 0 | outside | core, shell | case passes |
| MAN-001 | language | must | 0 | 0 | reason | loader | reason, no case |
| MAN-002 | language | must | 0 | 0 | reason | loader | reason, no case |
| MAN-003 | language | must | 0 | 0 | reason | none | reason, no case |
| MAN-004 | language | must | 0 | 0 | reason | none | reason, no case |
| MAN-005 | language | must | 0 | 0 | reason | none | reason, no case |
| MAN-006 | language | must | 1 | 0 | cases | loader, core | case passes |
| MAN-007 | language | must | 1 | 0 | cases | core | case passes |
| MAN-009 | language | must | 0 | 0 | reason | none | reason, no case |
| MAN-010 | language | must | 8 | 0 | cases | loader, core | case passes |
| MAN-011 | language | must | 14 | 0 | cases | loader, core, shell | case passes |
| MAN-012 | language | must | 0 | 0 | reason | shell | reason, no case |
| MAN-013 | language | must | 8 | 0 | cases | core | case passes |
| MAN-014 | language | must | 7 | 0 | cases | core, shell | case passes |
| MAN-015 | language | must | 1 | 0 | cases | core | case passes |
| MAN-016 | language | must | 6 | 0 | cases | core | case passes |
| MAN-017 | language | must | 14 | 0 | cases | core | case passes |
| MAN-018 | language | must | 1 | 0 | outside | core | case passes |
| NAM-001 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-002 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-003 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-004 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-005 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-006 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-007 | language | must | 2 | 0 | cases | core | case passes |
| NAM-008 | language | must | 1 | 0 | cases | core | case passes |
| NAM-009 | language | must | 9 | 0 | cases | core | case passes |
| NAM-010 | language | must | 3 | 0 | cases | core | case passes |
| NAM-011 | language | must | 1 | 0 | cases | none | case passes |
| NAM-012 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-013 | language | must | 1 | 0 | cases | core | case passes |
| NAM-014 | language | must | 7 | 0 | cases | core | case passes |
| NAM-015 | language | must | 0 | 0 | reason | none | reason, no case |
| NAM-016 | convention | must | 0 | 0 | outside | none | no case, no reason |
| NAM-017 | convention | must not | 0 | 0 | reason | none | reason, no case |
| NAM-018 | language | must | 5 | 0 | cases | core | case passes |
| NAM-019 | language | must | 8 | 0 | outside | core | case passes |
| NAM-020 | language | must | 7 | 0 | cases | core | case passes |
| NAM-021 | language | must | 3 | 0 | cases | core | case passes |
| NAM-022 | language | must | 6 | 0 | cases | core | case passes |
| NAM-023 | convention | must | 3 | 0 | cases | core | case passes |
| NAM-024 | language | must | 11 | 0 | cases | core | case passes |
| NAM-025 | language | should | 0 | 0 | reason | none | reason, no case |
| NAM-026 | convention | must | 1 | 0 | cases | core | case passes |
| NAM-027 | language | must | 2 | 0 | cases | loader, core | case passes |
| NAM-028 | language | must | 6 | 0 | cases | core | case passes |
| OBJ-001 | language | must | 1 | 0 | reason | none | case passes |
| OBJ-002 | language | must | 2 | 0 | cases | core | case passes |
| OBJ-003 | language | must | 8 | 0 | cases | core | case passes |
| OBJ-004 | language | must | 0 | 0 | reason | none | reason, no case |
| OBJ-006 | language | must | 1 | 0 | cases | core | case passes |
| OBJ-007 | language | must | 1 | 0 | cases | core | case passes |
| OBJ-008 | language | must | 5 | 0 | cases | core | case passes |
| OBJ-009 | language | must | 35 | 0 | cases | core | case passes |
| OBJ-010 | language | must not | 2 | 0 | cases | core | case passes |
| OBJ-011 | language | must | 0 | 0 | reason | core | reason, no case |
| OBJ-012 | language | must | 0 | 0 | reason | none | reason, no case |
| OBJ-013 | language | must | 11 | 0 | cases | core | case passes |
| OBJ-014 | language | must | 0 | 0 | reason | none | reason, no case |
| OBJ-015 | language | must | 0 | 0 | reason | none | reason, no case |
| OBJ-016 | language | must | 0 | 0 | reason | core | reason, no case |
| OBJ-017 | language | must | 0 | 0 | reason | none | reason, no case |
| OBJ-018 | language | must | 0 | 0 | reason | core | reason, no case |
| OBJ-019 | language | must | 3 | 0 | reason | none | case passes |
| OBJ-020 | language | must | 0 | 0 | reason | loader | reason, no case |
| OBJ-021 | language | must | 9 | 0 | cases | core | case passes |
| OBJ-022 | language | must | 1 | 0 | reason | none | case passes |
| OBJ-023 | convention | must | 1 | 0 | reason | core | case passes |
| OBJ-024 | language | must | 0 | 0 | reason | loader | reason, no case |
| OBJ-025 | convention | must | 3 | 0 | cases | core | case passes |
| OBS-001 | convention | must not | 0 | 0 | reason | none | reason, no case |
| OBS-002 | convention | must | 2 | 0 | cases | core | case passes |
| OBS-003 | convention | must | 2 | 0 | outside | core | case passes |
| OBS-004 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| OBS-006 | convention | must not | 0 | 0 | reason | none | reason, no case |
| OPS-001 | convention | must | 17 | 0 | cases | core | case passes |
| OPS-002 | convention | must | 1 | 0 | cases | core | case passes |
| OPS-003 | convention | must | 1 | 0 | cases | core | case passes |
| OPS-004 | convention | must | 0 | 0 | reason | core | reason, no case |
| OPS-005 | convention | must | 21 | 0 | cases | core, shell | case passes |
| OPS-006 | convention | must | 3 | 0 | outside | none | case passes |
| OPS-007 | convention | must | 17 | 0 | cases | core | case passes |
| OPS-008 | convention | must | 10 | 0 | cases | core | case passes |
| OPS-009 | convention | must | 11 | 0 | cases | core | case passes |
| OPS-010 | convention | must | 3 | 0 | cases | core | case passes |
| OPS-011 | convention | must | 1 | 0 | cases | core | case passes |
| OPS-012 | convention | must | 26 | 2 | cases | core | case passes |
| OPS-013 | convention | must | 5 | 0 | cases | core, shell | case passes |
| OPS-014 | convention | must | 7 | 0 | cases | core | case passes |
| OPS-018 | convention | must not | 4 | 0 | cases | core | case passes |
| OPS-019 | convention | must | 1 | 0 | cases | shell | case passes |
| OPS-020 | convention | must | 0 | 0 | outside | none | no case, no reason |
| OPS-021 | convention | must | 3 | 0 | outside | core | case passes |
| OPS-022 | convention | must | 31 | 0 | cases | core | case passes |
| OPS-023 | convention | must | 25 | 0 | cases | core | case passes |
| OPS-024 | convention | must | 7 | 0 | cases | core | case passes |
| OPS-025 | convention | must | 1 | 0 | cases | core | case passes |
| OPS-026 | convention | must | 3 | 0 | cases | core | case passes |
| OPS-027 | convention | must | 5 | 0 | cases | core | case passes |
| OPS-028 | convention | must | 3 | 0 | cases | core | case passes |
| OPS-029 | convention | must | 9 | 0 | cases | core, shell | case passes |
| OPS-030 | convention | must | 7 | 0 | cases | core | case passes |
| OPS-032 | convention | must | 2 | 0 | cases | core | case passes |
| OPS-033 | convention | must | 4 | 0 | cases | core | case passes |
| OPS-034 | convention | must | 2 | 0 | cases | core | case passes |
| OPS-036 | convention | must | 10 | 0 | outside | core, shell | case passes |
| PKG-001 | language | must | 1 | 0 | cases | core | case passes |
| PKG-003 | language | must | 9 | 0 | cases | core | case passes |
| PKG-004 | language | must | 7 | 0 | cases | core | case passes |
| PKG-005 | language | must | 2 | 0 | cases | none | case passes |
| PKG-006 | language | must | 12 | 0 | cases | core | case passes |
| PKG-007 | language | must | 2 | 0 | cases | core | case passes |
| PKG-012 | language | must | 1 | 0 | cases | core | case passes |
| PKG-016 | interface | must not | 0 | 0 | outside | none | no case, no reason |
| PKG-017 | language | must | 14 | 0 | cases | core | case passes |
| PKG-018 | language | must | 1 | 0 | cases | core | case passes |
| PKG-019 | language | must | 0 | 0 | reason | core | reason, no case |
| PKG-020 | language | must | 9 | 0 | cases | core | case passes |
| PKG-021 | language | must | 6 | 0 | cases | core | case passes |
| PKG-022 | language | must | 15 | 0 | cases | core, shell | case passes |
| PKG-023 | convention | must | 39 | 0 | cases | core | case passes |
| PKG-024 | convention | must | 2 | 0 | cases | core | case passes |
| PKG-025 | convention | must | 2 | 0 | cases | core | case passes |
| PKG-026 | language | must | 7 | 0 | cases | core | case passes |
| PKG-027 | convention | must | 1 | 0 | cases | core | case passes |
| PKG-028 | language | must | 2 | 0 | cases | core | case passes |
| PKG-029 | convention | must | 5 | 0 | cases | core | case passes |
| PKG-030 | language | must | 0 | 0 | outside | core | no case, no reason |
| PKG-031 | language | should | 3 | 0 | outside | none | case passes |
| PKG-032 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PLC-001 | language | must | 7 | 0 | cases | core | case passes |
| PLC-002 | convention | must | 5 | 0 | cases | core | case passes |
| PLC-005 | language | must | 1 | 0 | outside | core | case passes |
| PLC-006 | language | must | 0 | 0 | outside | none | no case, no reason |
| PLC-007 | language | must | 2 | 0 | outside | core | case passes |
| PLC-008 | language | must | 2 | 0 | cases | core | case passes |
| PLC-009 | language | must not | 0 | 0 | reason | none | reason, no case |
| PLC-011 | convention | must | 0 | 0 | outside | core | no case, no reason |
| PLC-012 | convention | must not | 0 | 0 | reason | none | reason, no case |
| PLC-013 | language | must | 0 | 0 | outside | none | no case, no reason |
| PLC-015 | language | must | 0 | 0 | reason | none | reason, no case |
| PLC-017 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PLC-018 | language | must | 1 | 0 | outside | core | case passes |
| PLC-019 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PLC-020 | convention | must not | 0 | 0 | reason | none | reason, no case |
| PLC-023 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PLC-024 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PLC-025 | language | must | 0 | 0 | outside | core | no case, no reason |
| PLC-026 | convention | must | 2 | 0 | cases | core | case passes |
| PLC-027 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PLC-028 | language | must | 0 | 0 | outside | none | no case, no reason |
| PND-001 | convention | must | 4 | 0 | outside | core | case passes |
| PND-002 | convention | must | 3 | 0 | outside | core | case passes |
| PND-003 | convention | must | 5 | 0 | outside | core | case passes |
| PND-004 | interface | must | 0 | 0 | outside | core | no case, no reason |
| PND-005 | convention | must | 7 | 0 | outside | core, shell | case passes |
| PND-006 | convention | must | 3 | 0 | outside | core | case passes |
| PND-007 | convention | must | 1 | 0 | outside | core | case passes |
| PND-008 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PND-009 | convention | must | 2 | 0 | outside | core, shell | case passes |
| PND-010 | convention | must | 15 | 0 | outside | core, shell | case passes |
| PND-011 | convention | must | 8 | 0 | outside | core, shell | case passes |
| PND-012 | convention | must | 1 | 0 | outside | core | case passes |
| PND-013 | convention | must | 4 | 0 | outside | core, shell | case passes |
| PND-014 | convention | must not | 2 | 0 | outside | core, shell | case passes |
| POOL-001 | language | must | 7 | 0 | cases | core | case passes |
| POOL-002 | language | must | 2 | 0 | outside | core | case passes |
| POOL-003 | convention | must | 0 | 0 | outside | core | no case, no reason |
| POOL-004 | language | must | 2 | 0 | cases | core | case passes |
| POOL-005 | language | must | 2 | 0 | outside | core | case passes |
| POOL-006 | language | must | 2 | 0 | outside | core | case passes |
| POOL-007 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| POOL-009 | language | must | 1 | 0 | outside | core | case passes |
| POOL-011 | language | must | 0 | 0 | outside | core | no case, no reason |
| POOL-012 | language | must | 0 | 0 | outside | none | no case, no reason |
| POOL-013 | convention | must | 5 | 0 | outside | core | case passes |
| POOL-014 | language | must | 3 | 0 | outside | core | case passes |
| POOL-015 | convention | must | 1 | 0 | outside | core | case passes |
| POOL-016 | convention | must | 1 | 0 | outside | core | case passes |
| POOL-017 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-001 | language | must | 0 | 0 | outside | none | no case, no reason |
| PRV-002 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-003 | convention | must | 3 | 0 | cases | core | case passes |
| PRV-004 | language | must | 0 | 0 | outside | none | no case, no reason |
| PRV-005 | convention | must | 1 | 0 | cases | core | case passes |
| PRV-006 | language | must | 5 | 0 | cases | core | case passes |
| PRV-007 | convention | must | 10 | 0 | cases | core | case passes |
| PRV-008 | convention | must | 1 | 0 | cases | core | case passes |
| PRV-009 | convention | must | 5 | 0 | cases | core | case passes |
| PRV-010 | convention | may | 0 | 0 | outside | none | no case, no reason |
| PRV-011 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-012 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-013 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-014 | convention | must | 2 | 0 | cases | core | case passes |
| PRV-015 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-016 | language | must | 0 | 0 | outside | none | no case, no reason |
| PRV-017 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-018 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| PRV-019 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-020 | convention | must | 1 | 0 | outside | core | case passes |
| PRV-021 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-022 | convention | must | 1 | 0 | cases | core | case passes |
| PRV-023 | convention | must | 0 | 0 | outside | shell | no case, no reason |
| PRV-024 | convention | must not | 0 | 0 | reason | none | reason, no case |
| PRV-025 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-026 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-027 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-028 | convention | must | 0 | 0 | outside | none | no case, no reason |
| PRV-029 | convention | must | 0 | 0 | outside | shell | no case, no reason |
| PRV-030 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| REC-001 | convention | must | 12 | 0 | cases | core | case passes |
| REC-002 | convention | must | 23 | 0 | cases | core | case passes |
| REC-003 | convention | must not | 8 | 0 | cases | core, shell | case passes |
| REC-004 | convention | must | 5 | 0 | cases | core | case passes |
| REC-005 | convention | must | 2 | 0 | cases | core | case passes |
| REC-006 | convention | must | 4 | 0 | cases | core | case passes |
| REV-001 | convention | must | 1 | 0 | cases | core | case passes |
| REV-002 | convention | must | 1 | 0 | cases | core | case passes |
| REV-003 | convention | must | 0 | 0 | reason | none | reason, no case |
| REV-004 | convention | must not | 3 | 0 | cases | core | case passes |
| REV-005 | convention | must not | 0 | 0 | reason | none | reason, no case |
| REV-006 | convention | must | 0 | 0 | outside | none | no case, no reason |
| REV-007 | convention | must | 0 | 0 | outside | none | no case, no reason |
| REV-009 | convention | must | 0 | 0 | outside | none | no case, no reason |
| REV-010 | convention | must | 3 | 0 | outside | core | case passes |
| REV-011 | convention | must | 0 | 0 | reason | core | reason, no case |
| REV-012 | convention | must not | 2 | 0 | outside | core | case passes |
| REV-014 | convention | must | 8 | 0 | cases | core | case passes |
| REV-015 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SCH-001 | language | must | 10 | 0 | cases | loader, core | case passes |
| SCH-002 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCH-003 | language | must | 0 | 0 | reason | none | reason, no case |
| SCH-004 | language | must | 2 | 0 | cases | loader | case passes |
| SCH-005 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCH-006 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCH-007 | language | must | 7 | 0 | cases | loader, core | case passes |
| SCH-008 | language | must | 0 | 0 | reason | none | reason, no case |
| SCH-009 | language | must | 17 | 0 | cases | core | case passes |
| SCH-010 | language | must | 4 | 0 | cases | none | case passes |
| SCH-011 | language | must not | 0 | 0 | reason | none | reason, no case |
| SCH-012 | language | must | 1 | 0 | cases | loader | case passes |
| SCH-013 | language | must | 2 | 0 | cases | loader | case passes |
| SCH-014 | language | must not | 0 | 0 | reason | none | reason, no case |
| SCH-015 | language | must | 24 | 0 | cases | loader, core | case passes |
| SCH-016 | language | must | 4 | 0 | cases | loader, core | case passes |
| SCH-017 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCH-018 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCH-019 | language | must | 2 | 0 | cases | core | case passes |
| SCH-020 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCH-021 | language | must | 0 | 0 | reason | loader | reason, no case |
| SCT-001 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-002 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-003 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-004 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-005 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-006 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-007 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-008 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-009 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-011 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-013 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-014 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-015 | language | must | 0 | 0 | outside | none | no case, no reason |
| SCT-016 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-001 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-002 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-003 | language | must not | 0 | 0 | outside | none | no case, no reason |
| SEC-004 | convention | must not | 0 | 0 | outside | core | no case, no reason |
| SEC-005 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-006 | interface | must | 0 | 0 | outside | none | no case, no reason |
| SEC-007 | convention | must | 1 | 0 | outside | core | case passes |
| SEC-008 | convention | must | 4 | 0 | outside | core | case passes |
| SEC-009 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-010 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-011 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-012 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-013 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-014 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-015 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| SEC-016 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-017 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| SEC-018 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| SEC-019 | interface | must not | 0 | 0 | outside | none | no case, no reason |
| SEC-020 | convention | must not | 0 | 0 | outside | none | no case, no reason |
| SEC-021 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-022 | language | must | 0 | 0 | outside | none | no case, no reason |
| SEC-023 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SEC-024 | convention | must | 0 | 0 | outside | none | no case, no reason |
| SUB-001 | language | must | 6 | 0 | cases | loader, core | case passes |
| SUB-002 | language | must | 8 | 0 | cases | core | case passes |
| SUB-003 | language | must | 34 | 0 | cases | core | case passes |
| SUB-004 | language | must | 2 | 0 | cases | core | case passes |
| SUB-005 | language | must | 10 | 0 | cases | loader, core | case passes |
| SUB-006 | language | must | 5 | 0 | cases | core | case passes |
| SUB-007 | language | must | 0 | 0 | reason | none | reason, no case |
| SUB-008 | language | must | 13 | 0 | cases | core | case passes |
| SUB-009 | convention | must | 2 | 0 | cases | core | case passes |
| SUB-010 | language | must | 10 | 0 | cases | loader, core, shell | case passes |
| SUB-011 | language | must | 1 | 0 | cases | loader, core | case passes |
| TPL-001 | language | must | 0 | 0 | reason | none | reason, no case |
| TPL-002 | language | must | 2 | 0 | cases | core | case passes |
| TPL-003 | language | must | 1 | 0 | cases | core | case passes |
| TPL-004 | language | must | 10 | 0 | cases | core | case passes |
| TPL-005 | language | must | 2 | 0 | cases | core | case passes |
| TPL-008 | language | must | 0 | 0 | reason | core | reason, no case |
| TPL-009 | convention | must | 2 | 0 | cases | core | case passes |
| TPL-010 | language | must | 0 | 0 | outside | none | no case, no reason |
| TPL-011 | convention | must | 2 | 0 | outside | core | case passes |
| TPL-012 | convention | must | 29 | 0 | cases | core | case passes |
| TPL-013 | convention | must | 3 | 0 | outside | core | case passes |
| TPL-017 | convention | must | 5 | 0 | cases | core | case passes |
| TPL-018 | convention | must | 0 | 0 | outside | core | no case, no reason |
| TPL-019 | language | must | 2 | 0 | cases | core | case passes |
