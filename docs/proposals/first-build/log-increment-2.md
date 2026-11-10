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
| 2-04 | Succeeded plain content on repeats (F1) | Gap | OPS-009, IFC-025, DL10 | Settled by the admin, decision 5 |
| 2-05 | A plain server's invalid-parameters error does not prove no effect | Contradiction | IFC-010, OPS-003, OPS-018 | Settled by the admin, decision 8a |
| 2-06 | A plain server's parameter-header error does not authorize repeating an effect | Contradiction | IFC-046, OPS-018 | Settled by the admin, decision 8a |
| 2-07 | How a contract declares an effectful plain operation safe to repeat | Gap | IFC-046, OPS-012 | Settled by the admin, narrowing decision 8a |
| 2-08 | Which Pool authorizes inspection of a missing Resource | Ambiguity | ACC-033, OPS-023, OPS-029, LIF-037 | Settled by the admin |

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

## Work order 009: package snapshots and project contexts

2026-11-09. I2-03, first part, applies decision 2 of the
[admin's decisions](increment-2/decisions.md): an immutable catalog of explicitly preinstalled
snapshots, with no fetching or installation. The core derives package/project name contexts
from the ordinary loaded documents, manifest pins and built-in package. It rejects missing or
mismatched pins (PKG-006, PKG-003, PKG-021), checks manifest identity and exports (PKG-017,
PKG-018, MAN-007), namespace uniqueness (NAM-027), local and external selections (NAM-007,
NAM-014, NAM-018, NAM-022, NAM-028), and Schema referent kinds (SCH-015). NAM-020 and NAM-021
protect the separately supplied core. First-start provenance stays unchanged (FST-005).

SUB-008's project digest uses the original manifest and only contract documents, with the
specified ordering and RFC 8785 canonicalization. Expansion preserves the relative-program
provenance required by SUB-010 and PRV-007. The cases compare digests across contract order,
additional templates and directory changes, and against independently computed expectations.
The supplementary digest unit test covers binary64 serialization and UTF-16 member ordering.
Implementation context checks cover MAN-010 bindings, MAN-017 required scalar places,
PKG-020 plain tool coverage and TPL-002 Type families.

The [S1 fixtures](../../../conformance/fixtures/s1/README.md) are promoted copies of the
study's drafts; the drafts remain untouched because they are outside this order's scope.
The fixtures retain real deterministic Conduit archive bytes and their computed pins. The
filesystem Package's npm artifact was available in the local cache; its retained bytes supply
the real `fileSha256`, without network access. These are snapshot and artifact evidence,
never successful installations. No archive-installation promise is claimed by these cases.

**Findings:** no new undecided language case was encountered in this part. No specification
change or settlement was made. Local submitted `from`, object resolution, registration and
instance validation remain the later orders' work; archive installation is I2-03's second
part. Catalogs with duplicate SemVer-equivalent releases, invalid snapshot metadata or invalid
boundary directory evidence are embedding errors, not invented Conduit outcomes.

**Worker choices:** scope keys are opaque runner identifiers; catalog storage is privately
owned and read-only. Replies expose the derived environment, references and frozen context
values without registering anything. Relative fetches use RFC 3986 path tokens, preserving
percent-encoded dots; relative command paths use filesystem tokens, preserving literal `?`
and `#`. The snapshot evidence archives and artifact bytes are retained so their hashes are
repeatable. Unsupported metadata fails the harness rather than being skipped.

**Validation:** `conduit-conformance` runs all 104 cases, including 58 package/project cases.
S1's missing-snapshot and mismatched-digest counterparts reject without exposing any partial
context. The duplicate-export case is rejected by the strict loader under SUB-005 before
context resolution, as `manifest.v1` encodes PKG-012. All workspace release tests, Clippy with
warnings denied, formatting and the repository's documentation, names, schemas, roadmap and
whitespace checks passed. No commit, branch change, push or pull request was made.

## Corrections after verification of I2-03's first part

2026-11-09. The admin accepted these three implementation defects against existing promises.
No new language finding or specification change was needed.

- MAN-017 requires every instance to have a value at each binding pointer. `required` and
  `properties` alone constrain only objects, so each intermediate place must also admit only
  objects in every schema branch. Binding validation now intersects the `type`, `enum` and
  `const` constraints at intermediate places as it already did at leaves. The
  [nullable-parent vector](../../../conformance/language/packages/binding-nullable-parent/case.yaml)
  rejects `/parent/leaf` where `parent` is required but admits null, even though `leaf` is a
  required string in an object parent. Before the fix the vector failed because resolution
  accepted the project and exposed its environment.
- TPL-004 applies a Type default only where the template supplies no implementation for the
  operation. NAM-022 resolves each operation key in the project or package that holds it;
  spelling alone is not its identity. The resolver now merges by the resolved scope and
  operation name, retaining the selecting scope for each implementation value. MAN-017 then
  checks only the selected item. The
  [qualified-override vector](../../../conformance/language/packages/qualified-implementation-override/case.yaml)
  selects a package Type whose `#op.v1` default has an invalid binding, and replaces it with a
  valid template selection keyed `example.org/test/p#op.v1`. Before the fix it failed with
  `accepted: false` instead of `true` because the replaced default was still checked.
- MAN-010 takes a relative program from the manifest's directory; SUB-010 resolves it against
  that supplied directory. Anchoring now preserves `..` for filesystem lookup instead of
  lexically removing the preceding component, which might be a symlink. The
  [parent-traversal vector](../../../conformance/language/packages/relative-program-parent-traversal/case.yaml)
  supplies `var/../bin/ls` from `file:///` and requires `/var/../bin/ls` in both the expanded
  manifest and held context, with relative-program provenance retained. Before the fix it
  failed because the held program was `/bin/ls`. The case asserts the preserved path without
  depending on the host's symlinks or running the command; the pure core reads no filesystem.

All three vectors were observed failing before their corresponding fixes, then passing in
the runner's full suite of 107 cases. The required-case test also names each new vector.

## I2-04: registration and resolving instances (work order 012)

2026-11-09. Applied the order's decisions against OBJ, INS, SUB, NAM and TPL, using I2-03's
snapshot contexts. No language finding needed a settlement and no specification or S1 fixture
was changed. S1's count agrees with the study: nine templates and six named instances, plus
four nested Conduits, **19 new objects**. The instances are registered and every new Conduit
is unopened. No setup, run, call, installation or external intent occurs.

**What the build did.** The core plans registration in private and publishes only after all
documents and nested instances resolve. Every decision cites promises. Existing-name clashes
report conflict; other defects report invalid, with refusal taking precedence when the
established registration path lacks authority. ID, revision and suffix evidence is supplied
at the boundary. Instance configuration, ownership, placement, Conduit ends and retained
nested declarations keep references by kind/ID, resolved under their schemas. A template
keeps its package identities, schema environment and selected implementation items; a later
project's contracts cannot change that context. Argument assignment replaces whole values,
fills defaults without coercion and preserves relative paths. Generated names retry both
existing and later-submitted named collisions under DL15.

**Worker choices.** Registration uses a new `register-project` input, leaving I2-01's
protected-submission compatibility input intact. Traversal sorts by family, name and document
value, then nested slot name, so allocation evidence maps to the same objects in every order.
The reply's nested projection gives parent ID, slot and declaration name. A successful
registration appends one `object-change` per new object, without inventing setup invocations.
Frozen contexts are owned values in the state. The case schema now permits POOL's four-letter
promise prefix. The runner's `registration-orders` compares entire transitions across three
orders, including reversed files, and publishes one result. Its assertions still compare
exact object/record arrays. DL15 collision retry is now implemented rather than reserved.

**Vectors.** The cases under
[registration](../../../conformance/language/registration/s1/case.yaml) cover accepted and
rejected arguments, defaults, configuration and references; template/Type family checks;
implementation defaults, overrides, engine selections, coverage and run counts; ownership,
placement, nested slots and generated-name collisions. S1 and its rejected counterpart are
compared in three orders, with rejection preserving every existing object. The frozen-template
case submits a changed Type contract at the same address and makes another instance from the
old template; it retains the old schema and leaves the old instance unchanged.

**Validation:** all 159 portable cases passed through `conduit-conformance`, including 52
new registration cases. Every registration case asserts exact object/record arrays or unchanged
objects on rejection. An additional boundary test exhausts IDs after an object-change record
was built privately, and repeats with a reused ID; both leave the caller's objects, records
and allocations untouched. All workspace release tests passed. Clippy with warnings denied
and formatting passed. The reference-patterns case applies both a property schema and a
matching pattern to one reference; both read the original value, preserving the stored ID
without treating another schema's rewrite as authored input. Repository documentation, names,
schemas, roadmap and whitespace checks passed. No commit, staging, branch change, push or
pull request was made.

### Verification corrections to I2-04 (work order 012, second round)

2026-11-09. The admin accepted ten verifier findings. The worker checked each against the
promises before applying it; all ten are decided by the current specification or, for vector
coverage, the order's decision 5. No specification change or "For the admin" settlement was
needed. The corrections are confined to the order's scope.

1. **Omitted Type schema — CTR-004.** A Type without `schema` puts no rule on configuration.
   Both template inspection and instance resolution use an unconstrained schema when absent.
   `registration-type-no-schema` accepts and preserves arbitrary configuration, including an
   ordinary map resembling a reference. It failed against the original binary.
2. **Defaults inside config — PLC-005, INS-012.** PLC-005 forbids defaults targeting an entry's
   placement/runtime; INS-012 permits destinations inside config. The prohibition now checks
   the `entries` root. `registration-config-standard-defaults` accepts defaults at both
   `/config/settings/placement` and `/config/settings/runtime`; it failed before the fix.
3. **Requirement arguments — NAM-019, OBJ-009, OBJ-021, SCH-009.** NAM-019 requires the same
   argument checks and default filling as instantiation. Every applied requirement now uses
   its own held contract context, validates arguments, fills input and schema defaults,
   rejects authored IDs and resolves governed references. The eight `registration-requirement-*`
   cases cover bound IDs/defaults, schema defaults, private schemas in the Requirement's
   package, unresolved references, authored IDs, invalid/unknown arguments and required inputs.
   All eight failed against the original binary and pass after correction.
4. **Retained packages — PKG-007, NAM-010.** Instantiation rejects an uninstalled package,
   naming its required version, while retaining the template's original context. Selection of
   a retained template now checks its package address/version/digest against the current
   catalog. `registration-retained-package-missing` registers a template with its snapshot,
   then omits that snapshot on an instance-only second submission; the rejection names the
   package and version and preserves existing objects. That second step was accepted before.
5. **Retained entry references — OBJ-009, POOL-009, POOL-016.** Every instance reference is
   stored by ID, and an input-supplied placement remains in the retained entry. Retained
   entries now copy supplied placement/runtime references from their resolved children;
   provider-chosen placement is not added to the declaration. The placement and runtime
   retained-entry vectors assert both parent and child, using plain string input schemas.
   Both failed before the fix. Standard references are resolved from their original supplied
   values, so an input schema's prior rewrite neither loses authorship checks nor rejects an
   engine-written ID.
6. **Retired entries — POOL-014.** An entry reference reaches the Resource that is not retired,
   and names no existing object when none qualifies. Lookup now filters by Resource kind and
   excludes retired entries. `registration-entry-retired-only` rejects; the active-replacement
   case selects the later active ID even with the retired Resource first. Both failed before.
7. **Template diagnostics — OBJ-008, OBJ-021, POOL-005.** Missing template references are
   diagnostics that permit registration. Inspection now covers Conduit input defaults,
   applied requirement arguments, nested arguments and standard targets, and missing nested
   template names. It continues after unresolved schema leaves. `registration-template-nested-notes`
   asserts five non-blocking named diagnostics; `registration-template-nested-from-note`
   asserts the missing template name. Both failed before. Two additional vectors preserve
   POOL-005's acceptance of a real sibling entry and rejection of a missing sibling; both
   already passed before these changes.
8. **Standard argument keys — OBJ-013.** Owner, placement, runtime, consumer and target are
   rejected under `arguments` independently of template input names. Five vectors declare
   each conflicting input and assert rejection with unchanged objects. All five were accepted
   by the original binary.
9. **Coverage — INS-023, PRV-006, SUB-005; order decision 5.** New accepted/rejected vectors
   cover input-supplied operations, an explicit registered runtime provider, a non-provider,
   a retired provider, and a wrong-family runtime rejected by the document schema. These six
   cases already passed the original binary: they close coverage gaps. INS-023's rejection
   now names the offending operation, and runtime rejection names the selected runtime.
10. **S1 assertions — OBJ-009, CON-017, CON-021, TPL-004, NAM-009.** The three accepted S1
    variants now assert the consumer, target, capability and exact ordered operations of
    all five Conduits, the selected filesystem implementation items, and the exact retained
    package identity arrays of all nineteen new objects. The existing resolver satisfies the
    strengthened assertions; no S1 fixture changed.

**Worker choices.** Template diagnostics reuse the reference-schema traversal with an
inspection-only collection of unresolved leaves; entry shorthand is checked against the Pool's
entry names in that mode. Standard fields are checked independently of input schemas. Retained
package checks compare the identities already recorded on the template and leave its indexes
unchanged. They reuse the catalog's PKG-003 comparison, which ignores build metadata. The
`registration-retained-package-build-metadata` safeguard passed the original binary, failed
the initial availability correction, then passed with that comparison reused. Wrong-family runtime coverage uses the loader because its document schema rejects
that form before registration. The required-case test names all thirty-one added vectors.

**Before-fix evidence.** A copy of the original release binary ran the final new fixtures in
isolated case inventories. Only the temporary baseline copies omitted `required_promises`
assertions, to distinguish actual behavior failures from newly added decision citations.
Twenty-two new behavior regressions failed there; the six coverage cases and the three
acceptance safeguards passed. The original source was saved separately for comparison and
was not committed. No fetch, installation, provider call or process run is claimed by a vector.

**S1 assertion probes.** Two temporary mutations to the resolver were compiled separately:
retargeting each new Conduit to its consumer, and appending an extra operation after the
capability check. Each mutation failed all three strengthened S1 variants, respectively at
`target/id` and the exact `operations` array length. The source was restored after the probes;
neither mutation is part of the change.

**Validation after correction.** All 190 portable cases, including 31 added cases and the
three strengthened S1 cases, passed through `conduit-conformance`. The workspace release tests,
including all five boundary tests, passed; Clippy with warnings denied and formatting passed.
The four Python repository checks and whitespace checks passed. New untracked registration
files were checked for trailing whitespace separately. No specification or S1 fixture was
changed, and no commit, staging, branch change, push or pull request was made.

## Work order 013 — installing supplied archives

Implemented on 2026-11-09. Applies the second part of I2-03, order decisions 1 to 5. No new
language finding was needed and no specification changed.

**What the build now shows.** The pure core takes an archive's compressed bytes, requested
address/version, expected digest and fetching completion. PKG-004/026 verify the exact
compressed bytes before parsing. PKG-022, MAN-013 and MAN-014 govern gzip/USTAR, root manifest
placement and member/link safety. MAN-011 selects only root documents, skipping dot-names
and directories, and rejects an instance. The existing loader supplies SUB-005's strict
reading and document/manifest validation. PKG-017 checks requested identity; the existing
catalog resolver checks exports, names, schemas and implementations, and unsatisfied package
dependencies reject under PKG-028. Every core decision cites its promises.

Only a fully accepted package enters state, marked `installed-archive`, retaining the virtual
unpacked files and regular-file modes. Later resolving and registration include it. Invalid
archives report PKG-023 without partial publication. Unavailable fetching reports PKG-024;
existing packages use PKG-025/027 without examining completion bytes. Installation creates
no instance (NAM-013), intent or record in this semantic input. This is archive admission and
catalog publication; general `install.v1` authorization, dispatch and record handling, and the
shell's physical storage, remain later convention/shell work. The core and runner fetch no
package and run no provider.

**Vectors.** Twenty-six archive cases add real-byte coverage. `archives-s1` first resolves
S1's project from snapshots, installs the repository-authored filesystem, history and Agent
archives in order, and resolves again with no supplied snapshots. Captured baseline values
check the same schema environment, resolved references, project digest, expanded manifest and
all context fields except provenance. Repeated/conflicting requests ignore unavailable
completion evidence. Other cases cover digest syntax and mismatch, version syntax, wrong
manifest address/version, an instance document, skipped `._` bytes, nested non-documents,
retained program bytes/mode, internal symbolic/hard links, dangling document links, invalid
documents, a nested manifest, malformed gzip/tar, missing dependencies, absolute/traversing
members, external symbolic/hard links and an indirect escape through another link.

Twenty-one deliberate variations are generated by the Rust `make_archives` example. Its
check mode compares every retained archive and `digests.json` byte for byte; recipes separately
state their expected compressed digest. A boundary test compares deterministic transitions
from identical inputs, round-trips installed state through serialization, and repeats against
the restored catalog with no archive bytes available.

**Third-party bytes.** The published npm filesystem tarball is removed. The three order-009
resolver cases now compare the Package pin with recorded SHA-256 data. Five registration
fixtures inherited the same tarball dependency from order 012; their aliases and artifact
recipes were converted too so removal leaves none broken. The S1 README records the hash,
the exact published package/version and `npm pack` reproduction command, its original cached
provenance, and that the core never fetches it. Installation archives contain only
repository-authored material.

**Worker choices.** DL7 reads POSIX USTAR with UTF-8 paths and regular files, directories,
symbolic links and hard links to earlier regular files; no GNU/PAX extensions. Links resolve
in memory with at most forty expansions. Repeated paths replace members except incompatible
nonempty-directory/parent layouts. All member paths and link targets are checked, including
ignored content. Programs and other non-document bytes remain in the virtual directory. The
profile and fixture README record these choices. Installed entries precede supplied snapshots
in the resolver catalog; legacy `snapshot-N` scope names remain stable, with explicit source
markers. The runner accepts only snapshot provenance in snapshot recipes. `stateUnchanged`
now checks installed entries too. Built-in schema file traversal is sorted. Snapshot artifact
recipes carry recorded hashes rather than file aliases. These are implementation and runner
choices, not changes to the promises.

**Validation.** All 216 portable cases passed, including the 26 archive cases. Workspace
release tests passed, including the new deterministic/restored-state boundary test. Clippy
with warnings denied and formatting passed. The generator's check mode reproduced all 21
archives and their digest index. The four repository Python checks, Git whitespace check and
a separate trailing-whitespace scan of changed text files passed. No staging, commit, branch
change, push or pull request was made.

### Work order 013 — corrections after verification

Corrected on 2026-11-09. All four accepted verifier findings are decided by existing promises;
none requires a specification change or a finding for the admin.

1. **An address held by a submitted project.** PKG-017 checks the archive's requested identity;
   NAM-024 concerns compatibility between releases. PKG-029 decides this finding: while an
   existing template or instance holds a submission's contracts or implementations, installing
   at its address reports conflict and installs nothing. MAN-016 explicitly cites that rule
   for this order of the operations. Installation now checks each existing object's selected
   `packages` against its retained submitted-project context before examining archive bytes.
   Four established-state vectors isolate templates and instances holding contracts and
   implementations, including a retired instance that still exists. Three safeguards accept
   installation with no holder, an unused project context, or a catalog-package selection.
   `archives-registered-project-holder` additionally registers a real project before attempting
   installation, checking the production representation and preserving both objects and catalog.
2. **Unreadable link paths.** PKG-022 delegates archive layout to MAN-013, which requires the
   unpacked project's directory; MAN-011 requires eligible entries to be readable as regular
   files and rejects unreadable ones under SUB-010. PKG-023 makes the installation invalid
   without publication. Document and hard-link reads now require every intermediate component
   to exist as a directory before continuing, including before `.` and `..`. Safety-only
   checks still allow forward and dangling symbolic targets, which cannot become readable
   through normalization. Real archives `manifest-file-parent` and `manifest-missing-parent`
   reject a root manifest link through a file or missing component before `..`; the matching
   `manifest-directory-parent` archive succeeds.
3. **The fortieth link expansion.** The profile's DL7 choice states "at most 40 link
   expansions"; PKG-022 delegates archive format choices to DL7. Link replacements are now
   counted separately from terminal lookup. Real archive vectors accept 39 and 40 replacements
   and reject 41 without publication, under PKG-022 and PKG-023.
4. **The unsafe member's name.** PKG-022 and MAN-013 delegate unsafe members to MAN-014,
   whose observable requires a diagnostic naming the member; PKG-023 requires invalid with
   diagnostics and no publication. Symbolic- and hard-link target resolution errors now retain
   the member name, as do errors resolving a member's parent or the final safety pass. The
   symbolic, hard and indirect escape vectors assert the full diagnostic, including the archive
   and member `escape`; the absolute-link diagnostic is asserted too.

**Before-fix evidence.** A saved original release binary ran the final fixtures in isolated
inventories. Only those temporary copies used empty `required_promises` arrays, avoiding
failures caused solely by new citations. Eleven behavior regressions failed: the five held
address cases, the file/missing manifest-parent cases, the 40-expansion boundary, and the
symbolic/hard/indirect escape diagnostics. Seven safeguards passed: the three unheld/catalog
address cases, the directory manifest-parent, 39 and 41 expansions, and the already-correct
absolute-link diagnostic. The final suite contains 230 vectors, with fourteen new cases and
four strengthened cases. Six new archives are reproduced by the existing Rust generator.

**Worker choices.** Kept safety checking separate from file readability so that forward and
unused dangling links remain representable. Used established-state snapshots to isolate each
kind of holder and a registration sequence to check actual generated state. Escape assertions
use exact diagnostic text, which also checks the archive and target. These are implementation
and vector choices; no language behavior was invented.

**Validation after correction.** All 230 portable cases passed. Workspace release tests,
including all six boundary tests, passed, as did Clippy with warnings denied and formatting.
The Rust generator verified all 27 archive fixtures and their digest index byte for byte.
All four Python repository checks passed, as did Git's whitespace check and a separate scan
including untracked text files. No specification, branch, staging, commit, push or pull request
was changed or made.

## I2-05: access decisions and current tools

Work order 014, 2026-11-09. No new language finding: the required cases have answers in the
promises, and no specification text changed. The core now evaluates attributed tool requests
and publishes exact private lists from registration's frozen contexts. ACC-033 routes management
operations, requiring the additional existing owning Pool's authority before registration
invalidity. Bounds cover Pools, targets and operation names, including narrower delegation;
root protection, secret restrictions, serving identity, current authority and carried-filter
support precede permission. Every decision cites promises and every access observation keeps
an access record without creating an invocation. A permit is an input to the following stage,
not a dispatch or a successful operation.

The [access vectors](../../../conformance/conventions/access/s1-exact-access/case.yaml)
register S1, then observe already established open/closed Conduits. They show A's five tools,
B's six, independent history access, guessed and stale calls refused, server tools adding no
implicit authority, identity/authority failures, bound boundaries and delegation, existing
Pool registration authority, and both forms of S1's plain-filter rejection. List comparison
produces notifications for only the changed consumer. This is evidence about the server's
presentation and decisions, not that either real client's model refreshes its cached catalog;
experiment 008's distinction remains.

The registration vectors `requirement-contract-context` and `requirement-schema-default`
had applied a non-core filter to `open.v1`, implemented by `engine`, which CON-022 forbids.
Their operation lists are now empty, preserving their tests of reference resolution and
default filling without requiring an implementation to support the filter. New registration
checks enforce CON-022, CON-028 and IFC-014, including unopened Conduits. These are fixes
against existing promises, not settlements or changes to language behavior.

### I2-05: correction after the verifier

Work order 014, returned once on 2026-11-09. The admin accepted five implementation defects.
Each is decided by existing promises; none requires a language settlement or a specification
change.

1. **Entry authority.** ACC-033 rows 4 and 6 authorize an entry Resource's setup, retirement
   and inspection against its owning Pool. POOL-001 fixes that owner, POOL-002 makes a Resource
   for each entry, and POOL-014 resolves the outside entry reference to its Resource that is
   not retired. Access resolution now selects that Resource before routing. The
   [entry-authority vector](../../../conformance/conventions/access/entry-authority/case.yaml)
   compares management-only with Pool-only authority for inspection, setup and retirement,
   both by the entry reference and by the Resource's name. A retired predecessor comes first
   in state; missing entries and entries with only retired Resources name no object.
2. **Local cannot be lost.** LIF-034 requires refused whoever asks, explicitly including root.
   ACC-032's permission for root to ask does not override it. Routing now resolves the `node`
   input, and protects `local` before permission. The
   [root-local-lost vector](../../../conformance/conventions/access/root-local-lost/case.yaml)
   uses root's existing management Conduit, so ACC-032 cannot mask LIF-034.
3. **The target-held Capability.** CON-017 governs the operation list resolved for a Conduit;
   NAM-026 separately requires unavailable when the target's held Capability omits a listed
   operation. Its observable distinguishes the absence of the Operation contract itself,
   which IFC-012 and IFC-033 omit from publication. Access now compares the operation with
   the members resolved in the target-held Capability's own scope. The
   [capability-skew vector](../../../conformance/conventions/access/capability-skew/case.yaml)
   keeps the Operation contract in an older context whose Capability omits it: the tool is
   presented but its call reports unavailable. The matching newer Capability permits the call;
   an entirely missing Operation contract still reports refused.
4. **Only published names collide.** IFC-014 compares names that IFC-012 presents or would
   present once opened. Publication and collision validation now share the same exclusions:
   `get-secret.v1`, a control absent from either the Type or the template, and an operation
   whose contract the target does not hold. Four acceptance vectors,
   [secret](../../../conformance/conventions/access/collision-secret/case.yaml),
   [Type control](../../../conformance/conventions/access/collision-control-type/case.yaml),
   [template control](../../../conformance/conventions/access/collision-control-template/case.yaml)
   and [missing contract](../../../conformance/conventions/access/collision-missing-contract/case.yaml),
   register two unopened Conduits with the same local name and no presented tool in common.
   The existing common-tool vector still rejects unopened Conduits.
5. **The diagnostic identifies both tools.** IFC-014's observable requires the tool name and
   the other Conduit. The diagnostic now keeps the matching name, the other Conduit's name
   and its ID. The
   [collision vector](../../../conformance/conventions/access/tool-collision/case.yaml)
   asserts the full messages in both directions, including `project.directory.list.v1` and
   the competing Conduit's identity.

**Search after the authority fix.** Registration's `References::resolve` already resolves
both outside entry references (POOL-014) and entry-local references (POOL-005/006) to the
Resource, not its owner. Inspection, setup, retirement, reconciliation and bound targets use
access's corrected resolver. Stored references select IDs; template `from` selects the named
template; the later owner lookups in routing intentionally implement ACC-033.

The search found another reference defect in access: bare names did not carry the kind fixed
by their schema (OBJ-006). OBJ-002 permits a Pool and a Resource to share a name. A bare node
name could therefore select its same-name owning Pool; a bare Resource `owner` could select
a Resource instead of the named Pool and bypass the extra registration authority check.
Fixed-kind lookups now apply to the `node`, `conduit` and registration `owner` fields. The
[fixed-kind vector](../../../conformance/conventions/access/fixed-kind-references/case.yaml)
puts the same-name objects first in state: bare `local` is still refused under LIF-034, its
explicit Resource form agrees, and management alone cannot register into `workspace`.
No additional form resolving to an owner was found.

**Before-fix evidence.** Saved the original release runner before changing the implementation.
It fails all seven new verifier vectors and the strengthened collision vector in isolated
inventories. Only the temporary copies of those vectors had `required_promises` cleared, so
these failures demonstrate behavior or diagnostic differences rather than missing new
citations. The first three failures have one permit record instead of the two rejection
records; the four collision acceptance cases have two rejection records instead of their
required registration records; the diagnostic assertion fails on the original generic text.
The fixed-kind vector also fails against the runner with the five verifier fixes but without
the fixed-kind correction: it permits root's bare `local` request.

**Worker choices.** Used established-state observations for access, a retired predecessor to
check current entry selection, and actual registration for collision acceptance. Selected the
first common tool name in sorted order and included the competing Conduit's name and ID in
the diagnostic. Shared publication exclusions rather than maintaining two lists. Supplied
Capability selections in earlier observation-only Conduits, matching registration's stored
shape. These choices change no language behavior.

**Validation after correction.** All 248 portable cases pass. Workspace release tests,
Clippy with warnings denied, formatting, all four Python repository checks and Git's
whitespace check pass. A separate scan of changed and untracked text files reports no trailing
whitespace. No specification, branch, staging, commit, push or pull request was changed or made.


## I2-06: request endings (work order 015)

The pure core now decides invocation dispatch, completions and repeats after evaluating
current access atomically. An invocation retains its caller, sender, target, operation,
filled arguments' canonical digest, selected contract and implementation, attempt number and
Conduit. It is independent of records (OPS-008). Every returned intent is a plan for the shell;
DL2 requires persistence of the state and records before it is carried out. No call is sent
here. No restart or record removal sends one.

### 2-04. Succeeded plain content on repeats

**State: Settled by the admin, decision 5. The owner can strike it.**

OPS-009 previously described only declared outputs, leaving a plain success's text undefined
on a repeat. OPS-009, IFC-025 and DL10 now explicitly keep that content with outputs, return it
unchanged while kept, and report conflict under OPS-034 after expiry. They never dispatch a
succeeded invocation again to recover it. The
[plain-content vector](../../../conformance/conventions/requests/plain-content/case.yaml)
observes text on the original result, an identical repeat, and conflict after explicit expiry.
The [declared-outputs vector](../../../conformance/conventions/requests/declared-outputs/case.yaml)
checks outputs and content together. OPS-034's existing rule is applied through OPS-009's
explicit statement that plain content counts as outputs for these promises; it is outside
this order's specification edit scope.

### 2-05. Invalid parameters from a plain server

**State: Settled by the admin, decision 8a. The owner can strike it.**

IFC-010's unconditional failed for -32602 could permit a second write after a server had
acted. The amended table treats it as an ordinary plain error: failed only for an operation
with no effect, no control and no model work, unknown otherwise. An engine rejection before
dispatch keeps its reason and has no outcome. The
[read](../../../conformance/conventions/requests/read-invalid-parameters/case.yaml) and
[write](../../../conformance/conventions/requests/write-invalid-parameters/case.yaml) vectors
exercise both outcomes. OPS-003 and OPS-018 did not repeat the old reading and need no edit.

### 2-06. Repeating after a parameter-header error

**State: Settled by the admin, decision 8a. The owner can strike it.**

IFC-046 no longer assumes that -32020 proves no effect. Only an operation with no effect,
no control and no model work may be sent again. Otherwise it ends unknown, as finding 2-07
narrows decision 8a. The
[read repair vector](../../../conformance/conventions/requests/repair-read-success/case.yaml)
observes one repair intent and the second response's result, with one invocation and attempt;
the [second-error vector](../../../conformance/conventions/requests/read-parameter-headers/case.yaml)
observes no third call. The
[unsafe write vector](../../../conformance/conventions/requests/unsafe-write-headers/case.yaml)
checks that neither deduplicates nor idempotentHint authorizes a plain write's repair.

### 2-07. Declaring safe repetition of a plain effect

**State: Settled by the admin, narrowing decision 8a. The owner can strike it.**
Class: gap. Promises: IFC-046, OPS-012.

No contract declares an effectful `plain` operation safe to repeat, and none is added.
IFC-046 now sends a call again after -32020 only for an operation with no declared `effect`,
that is not a control and starts no model work; any other ends unknown. Its amended text
contains no declaration exception. The previously stopped variant now expects unknown and
no further dispatch, including on a caller repeat, in the
[effectful plain headers vector](../../../conformance/conventions/requests/effectful-plain-headers/case.yaml).
The [aware headers vector](../../../conformance/conventions/requests/aware-headers-no-repair/case.yaml)
also prevents an effectful deduplicating operation from bypassing the narrowed condition.

### Implementation choices and vector boundaries

- `invoke` checks current access, fills defaults, validates, compares the scoped key and
  contents, checks ordinary availability, then dispatches. Conflicts precede ordinary
  unavailability; unreadable requests and unavailable authority precede both. A permit from
  the older access-only input is not accepted as reusable authority.
- DL2's digest is SHA-256 over RFC 8785 of operation and filled arguments. The index is in
  `State.invocations`, not reconstructed from records. It retains the target's frozen schema
  view for output validation. Completion inputs name both invocation and attempt, so an old
  attempt cannot overwrite a new one. Late completions cannot resolve unknown by themselves.
- A completion is a supplied result, protocol error or missing response. A header repair is
  a distinct intent within the same attempt, allowed once. An authorized reconciliation
  records its actor and checked evidence and never fabricates content or outputs.
- Cancellation is a trusted boundary notification for the running request, forwarded as an
  intent. A valid nonplain implementation confirmation of failed can accompany it;
  otherwise its outcome is unknown. Protocol ownership of cancellation belongs to the shell.
- `request-fixture` supplies ready/open observations on registered S1, optional contract
  variations and a Conduit-aware implementation variation. These are fixture observations,
  not setup or contract mutation operations. The runner checks that the actual request
  transition leaves objects and packages as observed. `restore-state` round-trips the entire
  serialized state and optionally observes records already purged; it does not implement
  `purge.v1`. It runs the core recovery transition: interrupted attempts end unknown under IMP-007, with
  supplied record evidence, and no dispatch intent. Settled invocations keep their state.
- `expire-outputs` is the explicit DL10 expiry boundary. It removes retained content and
  outputs, never the invocation. Time is supplied and no timeout reconciles an invocation.

The request vectors cover S1-plain-ends, S1-key-scope, S1-unknown-write and S1-reconcile,
including sender/caller/target changes, different operation and content, default equality,
no-key identities, in-progress conflict, succeeded repeats, unknown plain writes after
restart and record removal, both reconciliation outcomes, and cancellation. Reconciliation
of lifecycle state or controls remains the lifecycle order's work; unsupported core
management dispatches are embedding errors, never external calls to an engine implementation.


**Validation.** All 306 portable cases, including 58 request cases, pass in the workspace's
release tests. Clippy with warnings denied and formatting pass. The four Python repository
checks, Git whitespace check and a separate scan of changed and untracked text files are run
before the worker's report. No file is staged, and no branch, commit, push or pull request is
made. The return review below adds regression vectors and completes finding 2-07.


### Corrections after the return review of I2-06

These six findings are defects decided by existing promises. No additional settlement or
specification change is needed for them. Finding 2-07 above separately applies the admin's
narrowing of decision 8a.

1. **Metadata validity — IFC-009, OPS-007.** Presence is checked separately from value
   conversion. Both keys mean unknown, including null or nonstring values; no valid outcome
   or restriction reason rescues contradictory metadata. The ten `metadata-*` vectors test
   null, number, object and array reasons beside succeeded/failed, and null/number outcomes
   beside a refused reason with a matching restriction. Their non-deduplicating aware write
   repeats stay unknown and emit no dispatch. Each failed against the pre-fix executable:
   either it returned succeeded/failed or it reported the reason instead of unknown.
   Example: [failed with null reason](../../../conformance/conventions/requests/metadata-failed-reason-null/case.yaml).
2. **Aware capability errors — OPS-007, IFC-008, IFC-047.** Protocol completions preserve
   `data.requiredCapabilities`, with extensions under its `extensions` map. A nonempty
   capability map establishes failed for an aware
   implementation, distinguishing the extension from undeclared client capabilities; absent
   or malformed data still establishes nothing. Records of the extension error name the
   declared revision; a core test also checks absent, empty and malformed capability maps.
   The [extension](../../../conformance/conventions/requests/aware-capability-extension/case.yaml)
   and [undeclared capability](../../../conformance/conventions/requests/aware-capability-undeclared/case.yaml)
   vectors check failed and a caller-driven second attempt, beside the existing plain read
   and write cases. Both failed before the fix because the completion boundary rejected data;
   with that boundary retained and classification alone disabled, both returned unknown and
   failed their outcome assertions.
3. **Interrupted attempts — IMP-007, OPS-012, OPS-013, OPS-018.** `recover` is a pure core
   transition over restored state. Every in-progress attempt ends unknown, with records from
   supplied evidence and no dispatch intent. Settled invocations retain their states. The
   [interrupted restart](../../../conformance/conventions/requests/restart-interrupted/case.yaml)
   vector dispatches a write, restarts immediately without a completion, repeats it, supplies
   a late success, reconciles by an authorized actor with checked evidence, and restarts
   again. Recovery and the repeat send nothing, the late success is ignored, and
   reconciliation remains possible. Before the fix it failed with in-progress instead of
   unknown. The existing purge/restart vector still preserves the separate invocation index.
4. **Reconciliation precedence — OPS-031, OPS-023.** A forbidden non-control revision is
   checked before OPS-030's state conflict, after authority and schema validation. The
   [in-progress](../../../conformance/conventions/requests/reconcile-revision-in-progress/case.yaml)
   and [succeeded](../../../conformance/conventions/requests/reconcile-revision-succeeded/case.yaml)
   vectors combine that invalid revision with each conflicting state. Both failed before the
   fix with conflict rather than invalid, and assert the original invocation is unchanged.
5. **Recorded keys — REC-002.** Applicable access, dispatch, invocation-end and request-end
   records hold `idempotencyKey`, from the request or the durable invocation. Reconciliation
   records use the reconciled invocation's key and target as well. The
   [key retention](../../../conformance/conventions/requests/records-key/case.yaml) vector
   checks dispatch, completion and a succeeded repeat; key-scope and identity-scope assertions
   also distinguish keyed and keyless records. The retention vector failed before the fix
   because its first access record omitted the key.
6. **Recorded inputs — REC-002, SEC-004, SEC-015, PRV-024.** Record content follows the held
   target Type's `records` settings. Inputs are retained across access, dispatch, completion
   and repeats when enabled; outputs obey their separate setting. Schema-directed secret
   references retain only resolved Resource references; excluded values leave a path and
   rule, without their contents. Run secrets and identity, and secret-read outputs, are
   excluded regardless of retention settings. The
   [S1 inputs](../../../conformance/conventions/requests/records-inputs/case.yaml) vector
   failed before the fix because inputs were absent. The
   [rejected-input retention](../../../conformance/conventions/requests/records-inputs-invalid/case.yaml)
   vector also failed with absent inputs before readable rejections were given their raw
   arguments for retention and redaction. The
   [disabled inputs](../../../conformance/conventions/requests/records-disabled/case.yaml)
   vector is a negative control with the Type's inputs setting false, now selectable by
   contract kind in the fixture. Three pure-core tests additionally check run credentials,
   secret-read outputs, and secret references through an alias, including unresolved-value
   exclusion and resolved-value retention.

**Choices made by the worker.** Recovery is an explicit `recover` input with evidence for
records; the restore fixture calls it after deserialization. A completion preserves an optional
JSON `data` member, and treats absent, empty or malformed capability maps conservatively as
unknown. Record content is assembled in one helper using the held Type, with schema-directed
exclusions described by path and rule. Correcting reconciliation record enrichment also makes
its target the reconciled invocation's target. These implement the cited promises and add no
new language behavior.

## I2-07: setup and Conduit lifecycle (work order 016)

The core carries out ordered setups through the invocation path, including separately
authorized declared opens and Pool entry setups, explicit runner inspection/run reports,
remote connection parts and the final hook. Earlier successful parts survive a later
failure. Hook attempts retain a stable instance/operation key; succeeded and unknown hooks
are not dispatched again. Cancellation ends pending work and begins no later part.
No process or network connection is made by the core or by these vectors.

Closing records shutdown and removal, keeps the consumer ready, and preserves both records
and pending invocations. The original in-flight response still completes; subsequent calls
and onward requests use current authority. A missing declared slot is not recreated by setup.
Explicit replacement uses the existing registration resolver, frozen declaration references,
the current referenced template and a fresh identity, and leaves opening to a later request.

The 22 new cases under [setup](../../../conformance/conventions/setup/base/case.yaml)
cover the six S1 rows of the order, T7's independent A/B access, and the setup part boundaries.
Their IDs are in the coverage manifest and the required-case test. The base setup checks
part invocation/caller links and performs a complete second setup with fresh request keys;
runner reports and all UUID allocations are supplied. Findings: none newly established;
no promise text changed.

**Choices made by the worker.** Pending setup plans are stored within the durable invocation
index. A typed `setup-report` resumes only the outstanding part. The identity descriptor
contains required field names from the core contract and the subject ID, never credentials.
Preinstalled snapshots are retained in state so replacement can check the same immutable
package identities as registration. The runner adds explicit lifecycle observations,
ID-selected object/invocation assertions, ordered record projections and a development
case-prefix selector. These are core/runner representations, not new language rules.

### Return review of work order 016

The admin accepted eight verifier findings. Each is decided by the promises already in
force; none changes the specification or needs a "For the admin" settlement.

1. **Entry dependencies — POOL-006, POOL-013.** POOL-013 begins an entry "only once the
   parts for the entries it refers to have succeeded". Setup orders entries by their frozen
   reference dependencies and checks their outcomes before dispatch. The
   [succeeded dependency](../../../conformance/conventions/setup/review-dependencies-succeeded/case.yaml)
   and [failed dependency](../../../conformance/conventions/setup/review-dependencies-failed/case.yaml)
   cases put `z` before its dependent `a`, despite alphabetical order. A failed optional `z`
   leaves `a` undispatched and unsuccessful.
2. **Hook reasons — IFC-026, IFC-027, OPS-022.** IFC-026 covers "a request or a hook";
   OPS-022 expressly counts a part answered with a reason as unsuccessful without an effect.
   A hook's snapshot now supplies its reason to the parent before the parent settles, even
   when the hook invocation was removed. The
   [refused](../../../conformance/conventions/setup/review-hook-refused/case.yaml) and
   [unavailable](../../../conformance/conventions/setup/review-hook-unavailable/case.yaml)
   cases match the filter actually sent to the hook. Earlier successful opens make the parent
   unknown; the corresponding
   [refused without effects](../../../conformance/conventions/setup/review-hook-refused-no-effect/case.yaml)
   and [unavailable without effects](../../../conformance/conventions/setup/review-hook-unavailable-no-effect/case.yaml)
   cases end failed. The hook itself is answered with its reason, never a fabricated outcome.
3. **Cancelled entry effects — OPS-022, OPS-027, TPL-012.** OPS-022 requires unknown where
   an effectful part ended unknown. Cancellation now records each nested setup's outcome,
   effect and invocation in the Pool's parts before settlement. The
   [Pool cancellation](../../../conformance/conventions/setup/review-pool-cancel/case.yaml)
   case cancels while the first entry's run is pending, and asserts unknown for both setups,
   their part link and the cancellation intent.
4. **Retirement during setup — TPL-012, OPS-022, LIF-006.** TPL-012 requires that "the
   parts already dispatched end as they end" and prohibits any new part after retirement.
   Setup waits for pending entries, hooks, run work and connections; it records required work
   retirement prevented from starting. The five retirement cases cover
   [before a run](../../../conformance/conventions/setup/review-retire-before-run/case.yaml),
   connection-first completion with a
   [successful](../../../conformance/conventions/setup/review-retire-connection-succeeded/case.yaml)
   or [unknown run](../../../conformance/conventions/setup/review-retire-connection-unknown/case.yaml),
   and run-first completion with a
   [successful](../../../conformance/conventions/setup/review-retire-run-succeeded/case.yaml)
   or [unknown run](../../../conformance/conventions/setup/review-retire-run-unknown/case.yaml).
   No early completion settles outstanding work; no later hook or run is dispatched; the
   instance stays retired. An effect whose result is unknown remains unknown in setup.
5. **Replacement configuration — LIF-009, OBJ-009, INS-006.** LIF-009 uses the parent's
   resolved nested definition. Configuration resolution now respects replacement's trusted
   context for retained input references. The
   [reference replacement](../../../conformance/conventions/setup/review-replacement-reference/case.yaml)
   case registers an ordinary Resource entry with a reference input directed into its
   configuration, checks the parent's frozen argument, retires the entry and checks the
   replacement's configuration. Ordinary registration still rejects actor-authored IDs, as
   the existing authored-reference cases require.
6. **Running conditions — LIF-011, OBS-002, OBS-003, condition.v1.** LIF-011 requires the
   full condition schema; OBS-002 requires a URI-reference source and OBS-003 preserves the
   last successful heartbeat for an unknown observation. Inspection upserts Running beside
   other conditions, including reason, message and transition time, with a string source.
   The [built-in](../../../conformance/conventions/setup/review-conditions-builtin/case.yaml)
   and [selected runtime](../../../conformance/conventions/setup/review-conditions-provider/case.yaml)
   cases assert complete conditions through repeated stopped reports, running and unknown.
   Transition time stays fixed while status stays fixed. The runner also validates every
   condition against the actual core schema.
7. **Relative programs — PRV-007.** Its explicit prohibition applies even after SUB-010
   expands the manifest's relative path: the part ends failed and nothing runs. Before a
   provider run, setup now reads the selected implementation's retained relative-program
   provenance in its held scope. The
   [relative provider command](../../../conformance/conventions/setup/review-relative-provider/case.yaml)
   case registers an explicit runtime and a relative command from an ordinary manifest,
   asserts its expanded path, reports stopped and asserts failed with no run intent.
8. **Provider records — PRV-005, REC-002.** PRV-005 gives a provider call no Conduit,
   restriction or idempotency key; REC-002 gives its record no invocation served. Provider
   records now use the provider's own target and inputs with explicit enclosing-invocation
   correlation, without enriching them from the enclosing request's route. The
   [nested provider records](../../../conformance/conventions/setup/review-provider-records/case.yaml)
   case checks inspection and run within a Pool entry's setup, including correlation and
   absent request fields. Every provider-call record is also checked for the forbidden
   fields by the runner.

**Regression evidence.** Each of these seventeen cases was run with its corresponding fix
disabled, with the other fixes present, and failed. The failures respectively exposed the
wrong first entry, the missing hook part, the lost cancelled entry part, settlement before
outstanding work ended, rejection of the frozen replacement input, invalid conditions, a
forbidden provider run intent and inherited `serving` on a provider record. All cases are
required by the inventory test. No expected failure is presented as a successful check.

**Choices made by the worker.** Independent entries use deterministic name order as a tie
break; references determine their dependencies. A retirement-prevented required part is
represented as `retired-before-part` with no effect. Running conditions use `RunRunning`,
`RunStopped` and `RunUnknown` reasons, and a provider UUID URN or the built-in runner's URN
as their source. These are core and runner representations of the cited promises.


## I2-08: records, inspection and repair (work order 017)

The worker applied decision 7's six event types. Registration now records its access,
dispatch and ending, with an invocation entry, without changing earlier object allocations. The preparation boundary canonically orders
documents for its retained index so SUB-001 permutations remain equivalent.
Provider inspection/run reports now emit completion records with the run instance in the
CloudEvent subject. Each record is validated against `record.v1`; the runner additionally
checks applicable invocation, caller, route, key, serving and outcome fields and access before
dispatch. The existing provider-record vector now asserts the subject at the CloudEvent root
and both halves of the provider call.

The eleven `records-*` vectors cover inspection, unreadable ranges, purge, its root reservation,
strict cutoff and permanent certificates, unknown and succeeded repeats after purge, E11,
E12 and E08, including the broker's carried filter. The authority vector retains a known
caller when Conduit state is unreadable, omits an unreadable identity, and records no request
that is both unreadable and unidentified, under REC-001 and IFC-036. E11 resolves the replacement against the
new manifest and contracts, preserves the template ID, changes its revision only for changed
contents/context, keeps old instances frozen and resolves new instances against the new
context. E12 observes removal/name reuse and checks stale inspection/setup IDs with owning-Pool
authority, refusal without it, invalid ID addressing through management, lost credentials and
a successful follow-up with the current ID. It claims no retirement recovery. E08's approved
onward registration is supplied by a simulated trusted adapter, and creates no setup or run.
The adapter's approval logic remains separate provider evidence.

**Implementation choices.** The built-in project is supplied in Context for resolving
values-only management submissions. Replacement validation uses a private staging state and
supplied evidence, commits no staged allocations, and invokes no implementation. Purge
certificates use `object-change` with `action: records-purged`, cutoff and removed source/ID
pairs. Gaps retain source/ID metadata and are conservatively shown on every inspection until
more precise object association exists. Export's unreadable IDs describe a temporary read
failure; they do not delete data. Typed input references are associated with their object IDs when dispatched, using the
schema-directed walk also used for record redaction; inspection does not retarget them when
a name is reused. The input-reference vector checks unknown work naming another Resource,
including after purge and name reuse. The profile documents these representations. D19 counts
compact serialized UTF-8 JSON arrays: records, gap metadata and invocations separately,
including held contexts and retained outputs, with signed per-step growth and per-invocation
bytes, key status and outcome. No key expires and no index is compacted.

### 2-08. Inspection of a missing Resource through a Pool

**Class:** ambiguity. **Promises:** ACC-033, OPS-023, OPS-029, LIF-037.
**State:** Settled by the admin, on the return review of work order 017, 2026-11-09.
The owner can strike it.

**The ambiguity.** The description of `inspect.v1` and OPS-029 said a missing object reports
invalid through management or a Pool. ACC-033 requires the Resource's owning Pool, which
cannot be established for an absent object. OPS-023 puts authority before diagnostics, and
LIF-037 says an expected ID supplies no authority.

**The admin's decision.** Least disclosure: an object that does not exist has no owning Pool.
Authority to inspect it, or to act on it by a stale reference, is management's alone. A caller
with only a Pool's authority is refused, before any diagnostic, and learns nothing of whether
it existed. Where the object exists, ACC-033's owning-Pool rule applies. With management
authority, naming a missing object reports invalid. The worker amended `inspect.v1`, ACC-033
and LIF-037 in the extended scope.

**Vector.** [records-expected-id](../../../conformance/conventions/records/expected-id/case.yaml)
now pairs the formerly stopped Pool-only caller naming a missing Resource with the management
caller naming the same Resource, both with and without a stale expected ID. It asserts refused
without diagnostic, output or object/ID disclosure for the former, invalid for the latter,
no dispatch and unchanged objects. The supplied core's routing already implements this
least-disclosure rule; the new pairs passed before the edits.

**OPS-029, amended by the admin.** OPS-029's observable said: "So a request for `inspect.v1` that
names nothing reports invalid whether it is made through a Conduit to `management` or to a Pool",
on the reasoning that refused can be told only for a thing that exists. So the case was not
undecided, as 2-08 first read it: OPS-029 decided it the other way. The admin keeps its
settlement, as a choice against disclosure rather than the filling of a gap: names are global
per kind (OBJ-002), so under the old text a caller with a Pool's authority alone could tell a
name that exists nowhere (invalid) from one that exists in another Pool (refused). The admin
amended OPS-029's sentence to match, the same day. The owner can strike it, which would restore
OPS-029's text and reverse 2-08.

**Measured D19 sample, 2026-11-09.** In `records-purge`, record storage fell from 9,187 to
4,495 serialized bytes. The keyed unknown write's invocation entry stayed at 373,365 bytes.
The entire invocation array grew from 499,935 to 622,636 bytes because the purge itself is
another retained invocation. These counts include the representation's held contexts; they
are measurements of this vector, not a capacity projection or a proposed compaction policy.

**Verification before the return review.** The runner passed all 356 cases, including the eleven new records cases.
The workspace's release tests, clippy with warnings denied and formatting check passed, as
did the documentation, assigned-name, schema, roadmap and whitespace checks. The runner's
negative checks reject missing applicable record fields, dispatch before its access record,
and inspection that omits a record or an unknown invocation.

### Return review of work order 017

1. **Retained successful registration and replacement — OPS-009, ACC-033, OPS-023.** The
   records preflight now runs only for a new invocation or a dispatched new attempt, just as
   lifecycle preflight does. The current authority and input checks remain before the repeat
   answer. [records-replay-register](../../../conformance/conventions/records/replay-register/case.yaml)
   and [records-replay-replace](../../../conformance/conventions/records/replay-replace/case.yaml)
   compare the retained outputs and invocation ID, unchanged objects, and exactly an access
   decision and request ending, with no dispatch. Both failed on the supplied tree: OBJ-002
   and REV-002 conflicts respectively.
2. **Template inspection — LIF-012, LIF-013 and inspect.v1's outputSchema.** LIF-012 says,
   "It exists until it is unregistered, and has no lifecycle state", and "Inspecting a template
   returns no lifecycle state, and no operation gives it one." LIF-013's `when` is "An instance
   is inspected". The contract makes `status` optional and requires `state` whenever it is
   present. These decide the case: omit template status, retain all instance status, and invent
   no template state. [records-inspection](../../../conformance/conventions/records/inspection/case.yaml)
   now forbids template status and cites LIF-012; it failed before the fix. The runner validates
   generated and retained successful built-in outputs and successful repeat replies against
   the actual core contracts. Its negative schema test rejects status without state, and its
   inspection checks reject a lifecycle state on a template.
   This validation also exposed `register.v1` emitting `nested: null` for ordinary objects
   and internal parent/slot fields for nested ones. The `object.v1` contract and INS-016
   decide the result shape: omit `nested` for ordinary objects and return `instance`/`name`
   for nested ones. The [Pool-entry registration](../../../conformance/language/registration/pool-entries/case.yaml)
   vector now asserts those exact required fields and the absence of storage fields.
3. **Keys before invocation allocation — REC-002.** Its `then` requires the supplied key,
   and its observable says "Each of those is present in every record to which it applies."
   [records-keyed-refused](../../../conformance/conventions/records/keyed-refused/case.yaml),
   [records-keyed-invalid](../../../conformance/conventions/records/keyed-invalid/case.yaml)
   and [records-keyed-conflict](../../../conformance/conventions/records/keyed-conflict/case.yaml)
   assert the key on both records with no invocation allocated. All three already passed on
   the supplied tree: an earlier assignment put the key directly into the route. The worker
   consolidated propagation into the identity/key copy after route normalization, removing
   that earlier assignment. The runner now compares both outer records directly with the
   supplied request key, independently of any invocation entry; its negative test removes
   either key, both keys, and changes one key. No original-tree failure is claimed here.

**Choices made by the worker.** Centralize the supplied key with the request identity rather
than assign it twice. Validate built-in outputs in the runner against its actual core schema
environment, including newly retained successes and replies from earlier invocations. These
are implementation and verification choices, not new language semantics.

**Regression evidence.** With the other fixes present, disabling the repeat-preflight guard
failed each replay vector with its original conflict; disabling template-status omission
failed inspection's actual output-schema check; omitting the consolidated key copy failed
each of the three keyed ending vectors with a missing request key. Reverting registration's
result shape failed the Pool-entry vector under `object.v1`. Disabling either new runner
validator failed its corresponding negative unit test. These are nine expected failures,
not successful conformance results; all temporary mutations were restored. The supplied-tree
keyed rejection vectors passed before consolidation, as recorded above.

**Verification after the return review.** The full runner passed all 361 vectors, with D19's
separate record and invocation-index counts. After all temporary mutations were restored,
`cargo test --release`, clippy with warnings denied and `cargo fmt --check` passed, as did the
documentation, assigned-name, schema, roadmap, diff-whitespace and new-file trailing-whitespace
checks. OPS-029's contradictory sentence remains the out-of-scope admin follow-up above.

## I2-09: S1 whole and the increment 3 handoff (work order 019)

Written by the worker, 2026-11-09. This section reports the core, not a real-Agent run.
The [continuous case](../../../conformance/conventions/s1/whole/case.yaml), `s1-whole`, starts
from empty state, with no established-state fixture or lifecycle patch. Its 58 steps perform
first start, snapshot resolution, registration, setup of the Pool, both filesystem Resources
and both Agents, and the four declared opens. A reads/lists the project and is refused a
write; B reads/writes/reads. Both post/list/read history. Then root closes A's project
Conduit: its old keyed call is refused, B writes/reads, and both retain history access.
Explicit register-replacement makes a registered, unopened slot with a new ID; setup opens
it, A reads again and still cannot write. Exact private tool lists, unchanged B/history,
plain calls without Conduit metadata, final invocation outcomes and ordered request records
are asserted. Every external dispatch receives a scripted completion; no server, process,
Agent, file effect or model catalog is observed.

### Increment 2's done-when, item by item

| Item from the first build | Evidence and limit |
| --- | --- |
| S1's documents register and resolve | `s1-whole` resolves the actual filesystem/history/Agent snapshots and registers the ordinary S1 project after actual first start. It asserts the 19 new objects, package identities, endpoints, operation lists and lifecycle progress. `registration-s1-orders` additionally checks order independence; `archives-s1` checks real compressed bytes separately. |
| Every decision cites a promise | The runner rejects every emitted decision with no promise or an unknown ID, on all core steps. DL choices occupy their own field. This is executable citation coverage, not a proof that every citation is sufficient. |
| The safety vectors pass | The complete suite is required, including E05 standalone schema cases, access/refusal/bounds/filter boundaries, request endings/repeats/restart, partial setup/cancellation, E08's supplied trusted broker, E11 frozen contexts and E12 stale IDs. The run results below report actual checks, not expected passes. |
| Each case the core cannot decide is a logged finding | Findings 2-01 through 2-08 are settled in this log, including the return reviews. This integrated trace needed no new language answer. Unsupported implementation paths and missing coverage are recorded below for the admin; they are not silently answered or represented as language settlements. |

The [manifest](../../../conformance/manifest.yaml) now lists the study's complete inventory:
308 distinct promises including OBS-001/002/006 in its closing paragraph. 217 have vectors;
91 have explicit reasons. The runner validates the exact inventory and case references even
on a development subset; a regression test removes a row, removes a reason and supplies a
false case attribution, and each must fail. Case tags claim their asserted slice, not an
entire promise. Loader workspace tests not yet ported to portable cases, missing semantic
boundary pairs, release-skew/compatibility and shell-only evidence are distinguished in the
reasons. No promise S1 touches is omitted without explanation. This does not turn the 91
reasons into conformance results.

### For the admin: implementation and coverage work

**I2-09-implementation: implementation gap, not a new language finding.**
The existing promises decide these behaviors; no new ruling was needed or made here.
The [handoff](increment-3-handoff.md#current-implementation-limits-to-resolve-before-exposure)
records the unsupported public core paths: complete retirement/stop-run, unregister,
retire-with-resources, declare-lost, controls and the install.v1 invocation workflow.
Archive admission is already pure and tested, but lacks that workflow's access, fetch,
dispatch and records. Retirement/name-reuse safety observations do not execute a retirement.
The current dispatcher reports an embedding error for unsupported engine operations; it must
not be routed to an external server or implemented again by the shell. **The admin needs a
core follow-up before claiming or exposing the complete management tool contract.** This is
unfinished implementation, not a gap in what the promises decide. The manifest's reasons
record each affected S1-inventory promise; H03/H12 in the handoff are order-sized work items.

**I2-09-coverage: coverage gap, not an undecided language answer.**
Port the loader-only rows as portable cases, add the individually named semantic boundary
pairs (including capability/Type rejection, frozen placement, consumer crash, standalone
Conduit opens, missing revisions and expected retirement conflicts), and qualify release
compatibility before T5. Keep these as coverage work rather than relabelling broad happy-path
vectors as complete promise proofs. Credential secrecy, actual process environment, client
catalog behavior and around-engine reach require shell evidence; scripted core facts cannot
supply it. This order changes no promise and records no recommendation as an accepted ruling.

### Lean vectors: measured costs and proposed later order

Before this order, `conformance/` held **8,202,082 bytes in 1,269 files**. Its 361 case files
held **6,674,308 bytes**. The largest was setup-close, 92,353 bytes; setup-base was 91,376.
These are case scripts containing repeated traces and expectations, not 361 independent
90 KB state snapshots: 224 cases used an initial snapshot recipe, mostly referencing the
same 10,700-byte root/system state. There were 207 repeated register-project inputs and
129 expectations containing whole object arrays. Repeated supplied UUID pools, expanded
registration records, package/contract contexts and pretty-printed expected states account
for much of the size. For scale, compact JSON encoding of the legacy input evidence fields
alone takes 980,757 bytes, expected record fields 1,083,521, and expected state fields
1,236,657; these logical field counts are not additive measurements of disk space.

The new continuous case keeps ordinary documents shared by file alias and uses existing fact
projections for most later steps; it deliberately carries its own full first-start and S1
registration assertions and supplied evidence. Its length is a reason to improve the runner
format rather than copy this full trace into every future case. **No legacy vector is
rewritten here.** A later order can:

1. Add versioned runner-only shared scenario fixtures for common first start, registration,
   setup traces and allocation evidence. Make prerequisites explicit, acyclic and immutable;
   include no fixture syntax in Conduit documents. Avoid bootstrapping a test of setup from
   a silently ready snapshot. Keep one continuous empty-state S1 case.
2. Reuse ordinary package/project files and small declared variants, and deduplicate repeated
   fixture documents by content. Preserve original manifest bytes/values and package digests;
   never hide a changed submission behind a fixture merge.
3. Replace large repeated expected states with exact identity/cardinality sets plus required
   field projections and explicit absences. For records, keep exact event counts/order and
   correlation, optional data only where its rule is under test (decision 7, REC-001/002).
   Extend the existing record_facts matcher with exact cardinality/set checks before dropping
   legacy exact record arrays; a subsequence alone must not permit extra effects.
4. Offer named deterministic allocation blocks/captures at the runner boundary, expanded to
   explicit Evidence before step. Keep uniqueness/exhaustion checks and report expanded input
   on failure. Never let shorter fixtures supply time or entropy from ambient sources.
5. Migrate by family, run both representations and compare transitions/verdicts, and retain
   omission/extra-object, missing-record and wrong-correlation regression checks. Measure
   bytes before/after and preserve case IDs, manifest attributions and readable failure paths.
   Invocation-context compaction is a separate DL2 storage task, not a fixture rewrite.

### Conformance choices, handoff and validation

[CONFORMANCE.md](../../../reference-model/CONFORMANCE.md) consolidates the profile and
orders 004-017: implemented portions of DL2-10/12/13/15, their sources/evidence, and the
remaining DL1/3/5/7/8/10/12/13/16/17 choices. Removed DL11/14 are not reused. It makes no
claim of a persistent shell or cryptographic identity implementation.
[The handoff](increment-3-handoff.md) enumerates the actual core API, every current intent
and completion shape, consumer contract, unsupported paths, the first build's ten needs and
experiments 008/009's observed limitations as H01-H12. No network was used except crate
fetching; no real client, credential directory or owner configuration was accessed.

**Actual validation, 2026-11-09.** The workspace release suite passed, including all 362
portable cases (the vector test took 148.86 seconds), record/schema/boundary regressions and
the new inventory rejection test. After tightening the new case's unchanged-revision
assertions and removing two inherited tags it did not independently demonstrate, the final
58-step `s1-whole` subset passed again, with complete manifest validation. Clippy with warnings
denied and formatting passed. The documentation, assigned-name, schema and roadmap checks
passed, as did Git's whitespace check and a separate scan of all nine changed/new text files.
No promise, legacy vector, owner configuration, branch, commit, Git index or remote was changed.
