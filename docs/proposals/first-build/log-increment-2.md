# Increment 2: the core on S1

Not binding. The specification in [spec/](../../../spec/README.md) alone binds. This log records
findings of increment 2 under [ruling 46](../ruling-46/README.md), point 4; the
[README](README.md) says what a finding is and who settles it.

Work order 004 (I2-01), 2026-11-09, establishes the portable vector format, the pure
`conduit-core`, the `conduit-conformance` file boundary and first start. It applies the admin's
[decisions](increment-2/decisions.md), particularly 3, 4 and 7. Its
[cases](../../../conformance/README.md) cover first start, each protected core template,
LIF-034's refusals, repeated first start and the four original loader settlements, with the
review amendments carried too. It claims no general registration, access evaluation, shell,
package installation or execution of S1.

## Findings

The required I2-01 cases are decided by the promises. The admin settled the supplemental YAML
BOM boundary below. The core returns an embedding error for unsupported inputs rather than
selecting a Conduit answer. No specification change was made by this order.

| # | Finding | Class | Promises | State |
| --- | --- | --- | --- | --- |
| 2-01 | Whether two initial YAML BOMs must be rejected | Ambiguity | SCH-013, SUB-005; schema profile, Encoding | Settled by the admin |

## 2-01. Two initial YAML byte order marks

**Class:** ambiguity. **Promises:** SCH-013, SUB-005; the schema profile's Encoding row.
**State:** Settled by the admin. The owner can strike it.

The Encoding row allows one leading byte order mark, ignored in either syntax. The loader
previously accepted two initial marks in YAML and ignored both, while its JSON reader rejected
two. The worker stopped the supplemental YAML case pending a decision.

**The admin's settlement.** A file beginning with two byte order marks is not YAML and is
rejected under SUB-005, as it is in JSON. YAML 1.2.2's document prefix holds one optional mark;
the second is content outside that prefix, where it may not stand. A mark that YAML permits
before a later document remains permitted.

**What the build did.** The YAML reader now rejects the second leading mark under SUB-005.
The [BOM vector](../../../conformance/language/loader/bom/case.yaml), `loader-leading-bom`, gives
[bom-2.yaml](../../../conformance/language/loader/bom/bom-2.yaml) its rejected expectation and
checks [a mark before a later document](../../../conformance/language/loader/bom/later-document.yaml)
is accepted with both documents intact. It retains the one-mark checks in each syntax and the
two-mark JSON rejection. This applies the admin's decision without changing the specification.

## Corrections after verification of I2-01

These are implementation defects against existing promises, not new language findings.

- **Registration authority and diagnostic precedence.** ACC-033 row 1 requires open Conduits
  listing `register.v1` to both `management` and an existing, non-retired owning Pool.
  OPS-023 gives refusal precedence over document invalidity and forbids object diagnostics in
  that reply. The [original protected case](../../../conformance/language/first-start/protected/case.yaml)
  retains the three Resource submissions naming `system` as owner, now expecting refused
  with no diagnostics. The [authorized case](../../../conformance/language/first-start/protected-authorized/case.yaml)
  supplies an established state with root's additional open Conduit to `system`, listing
  `register.v1`, so all five templates reach FST-006 and return invalid with its diagnostic.
  The core checks both authorizations before producing FST-006 diagnostics. Neither case
  publishes an object or dispatches anything.
- **The named object and the target.** The lifecycle input now carries a resolved `object`
  separately from the request's actual Conduit `target`. LIF-034 applies to the named object;
  its refusal precedes lifecycle conflicts under OPS-023, including attempts to unregister a
  ready first-start object or retire the occupied system Pool. REC-002's target in both
  records is the Conduit target, with the named object retained in a separate `object` field.
  The [lifecycle case](../../../conformance/conventions/first-start/lifecycle/case.yaml)
  checks `management` as the target of root's management Conduit, including unregistering
  `system`. The [Pool-target case](../../../conformance/conventions/first-start/lifecycle-pool-target/case.yaml)
  retires `local`, `management` and `access` through root's supplied Conduit to `system`,
  the Resource-retirement target ACC-033 row 4 requires; both records keep `system` as target
  and the Resource as the named object. LIF-034 refuses these too, despite the supplied access.

## Implementation choices of I2-01

- Runner control files use JSON syntax, which is valid YAML; Conduit documents remain ordinary
  files with names from `spec/names.yaml`.
- First-start allocations supply seven object UUIDs, then fourteen record UUIDs, seven
  revisions and two generated-name suffixes. Registration and registered-to-ready changes
  have separate `object-change` records, without fabricated invocations. The atomic transition
  is the persistence boundary; no crash recovery shell is claimed.
- Root's two declared Conduits use their declaration names locally and generated names
  globally. Their provenance, capability and exact operation lists come from the built-in
  templates supplied by the runner, not a second list in the core.
- The runner implements exact arrays and required-field map projections, explicit captures,
  and a profile that marks choices reserved for later orders. A known schema absolute keyword
  location is preserved. Output retention and contents hashing are stated profile choices,
  not claims of an implemented invocation index.
- The runner's `snapshot` initial recipe reads an explicitly supplied established state, with
  no transition or publication. The shared fixture adds one open Conduit from root to system
  listing registration and retirement. It does not claim first start made that Conduit or
  implement general registration or opening. The original empty-state first-start vector
  still checks exactly seven instances. The lifecycle input's separate `object` is an already
  resolved ID, not an operation's wire input schema.
