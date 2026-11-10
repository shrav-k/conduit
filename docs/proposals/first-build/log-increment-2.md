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
