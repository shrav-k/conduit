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
| 2-02 | An authored root `$defs` key in the form of a contract copy | Gap | IFC-033 | Settled by the admin, decision 6 |
| 2-03 | Different defaults on one place through unconditional `$ref` siblings or `allOf` | Ambiguity | SCH-009, SCH-015, SCH-019 | Settled by the admin |

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

## 2-02. Reserved root copy keys

**Class:** gap. **Promise:** IFC-033. **State:** Settled by the admin, decision 6.
The owner can strike it.

Work order 006 (I2-02), 2026-11-09, applies
[decision 6](increment-2/decisions.md): each authored schema's root `$defs` reserves the form
of a qualified contract-copy key, with `@version` where stated. IFC-033 now rejects a document
using one when installed or submitted, with the key and its position in the diagnostic,
whether or not publication would reach that key. Nested `$defs` are outside this reservation.

The core's semantic admission check reports IFC-033. The
[root-key vector](../../../conformance/interface/schemas/e05-root-key/case.yaml) checks the
rule after strict loading, and the Python checker applies it to a new
[example](../../../spec/schemas/examples/invalid-by-rule/IFC-033-reserved-root-defs.yaml).
General installation and registration remain later orders; this is their shared schema check.

## 2-03. Conflicting unconditional defaults

**Class:** ambiguity. **Promises:** SCH-009, SCH-015, SCH-019.
**State:** Settled by the admin. The owner can strike it.

**The admin's settlement.** When schemas that apply to one place unconditionally, through
`$ref` and its siblings or `allOf`, give different defaults, reject the document under SCH-009
when installed or submitted. The diagnostic names the place and the defaults. Equal defaults
are one default. This does not change SCH-009's exclusions from filling through applicators.

**What the build did.** SCH-009 now states that rejection. The core's semantic admission
check returns its diagnostic instead of the former embedding error. The
[conflicting-defaults vector](../../../conformance/interface/schemas/schema-conflicting-defaults/case.yaml)
checks rejection and diagnostic facts; the
[equal-defaults vector](../../../conformance/interface/schemas/schema-equal-defaults/case.yaml)
checks admission and a single filled property. The Python checker applies the same rule to
[the reference-sibling example](../../../spec/schemas/examples/invalid-by-rule/SCH-009-conflicting-defaults.yaml)
and [the allOf example](../../../spec/schemas/examples/invalid-by-rule/SCH-009-allof-defaults.yaml).

## Corrections after verification of I2-02

These are implementation defects against existing promises, not new language findings.

- Control narrowing relocates references to original contract inputs and descendants under
  `allOf/0`, preserving both self references and references from elsewhere in the schema.
  The [control vector](../../../conformance/interface/schemas/e05-control-local-targets/case.yaml)
  compares authored and published verdicts for nested definitions and property references.
- Pattern assertions apply separately to the containing map, preserving references to an
  explicitly declared property. The
  [property-reference vector](../../../conformance/interface/schemas/schema-pattern-property-ref/case.yaml)
  checks both schema directions against explicit expected verdicts as well as each other.
- Default filling traverses existing map members through matching `patternProperties` and
  applicable `additionalProperties`, using the loader's pattern engine and retaining SCH-009's
  exclusions. The [applicator vector](../../../conformance/interface/schemas/schema-applicator-defaults/case.yaml)
  asserts filled values and verdicts, with output values left unfilled.
- Schema fixture calls compare returned objects to the actual input snapshot, including the
  injected publication environment. A harness regression injects an unexpected object mutation
  into all three schema input kinds and requires `stateUnchanged` to fail. After this correction
  alone, all 41 existing vectors passed on a rebuilt runner.

## Implementation choices of I2-02

- Package/project scope IDs are runner metadata. Address/version copy keys follow IFC-033;
  each scope's exact manifest pin selects dependencies. Internal URNs in authored validation
  preserve original resource roots and are never published. The target supplies the held
  operation and schemas (NAM-014, NAM-023, SCH-015).
- The publisher inlines nothing optional (DL12). A visited-contract set makes recursive copies
  finite. Local pointers escape JSON Pointer tokens before URI-fragment encoding; schema
  positions alone are traversed, leaving `$ref` data under annotations and literal values.
- The loader exposes the validators and pattern engine already; no loader changes are needed.
  `patternProperties` uses that engine to match actual property names, then delegates matching
  property schemas and additional properties to the existing validator. Original pattern
  schema locations remain addressable in the temporary validation form. The wire definition
  keeps authored patterns (SCH-001, SCH-016).
- Schema admission is a pure boundary following strict loading, reusable by later installation
  and submission transitions. Default diagnostics name the default position and failing schema
  keyword. Output evaluation fills no default; operation inputs fill only their root's own
  properties, as SCH-009 and OPS-026 require.
- E05 fixtures supply established targets and open Conduits. They prove no installation,
  registration, opening, implementation dispatch or execution of S1. Actual core registration
  and replacement contracts are read at the runner boundary. Both schema directions compare
  authored-resource and resolver-free publication verdicts, including copies, handles, control
  narrowing and omitted controls. Every decision cites its requiring promises.
