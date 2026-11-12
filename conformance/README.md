# Conformance cases

Not binding. The [specification](../spec/README.md) alone binds. This is a portable convention
of the first build's runner, adopted by the admin's [decisions 3, 4 and 7](../docs/proposals/first-build/increment-2/decisions.md).
The [case schema](case.schema.json) describes the current executable subset. Later orders
extend it alongside their cases; an unknown input or assertion fails rather than being skipped.

Each `case.yaml` has a stable `id`, `layers` (`language`, `convention`, `interface`), promise
IDs, a title, the `first-build` profile, an `initial` recipe, named files and ordered `steps`.
An initial recipe is `empty` or `snapshot`, whose `file` names an alias holding an explicitly
supplied established core state. A snapshot is runner metadata, not submitted Conduit
documents, and makes no transition. Cases may use JSON syntax in their YAML files;
Conduit's strict reader reads both the runner metadata and the fixtures. [manifest.yaml](manifest.yaml)
is the coverage index. The four original loader settlements are named in their case titles;
the review's BOM, pattern-token and artifact-pin amendments have cases too. The artifact-pin
case checks a document and claims no execution or dependency-graph verification.

The [profile](profiles/first-build.yaml) states the admin's implementation choices. It also
lists which choices this order implements and which belong to later orders. The suite distinguishes preinstalled snapshots from
archive-install inputs with real bytes, expected hashes and fetching completions. Archive
installation implements DL7 in the profile.

## Inputs and files

`files` maps aliases to paths relative to the case directory; shared fixtures may use relative
paths into the repository. Paths are never fetched. A submitted alias is the original file
name, including `conduit.yaml` when it is a manifest. Documents are ordinary Conduit files:
runner symbols never appear in them. Bytes that intentionally fail reading live in `bytes/`.
No size or count cap is imposed on files, submissions or archives by these cases; the nesting
limit is 128 and the portable floor is 64.

Each step has one `input` and an `expect`. Supported input kinds are:

- `first-start`: supplied `evidence` (time, UUID allocations, revisions and generated suffixes)
  and a local `machine` report. The eight instance IDs precede the sixteen record IDs. `trustDomain` supplies the FST-007 namespace. A repeat
  consumes no allocations. No ambient time, hostname or entropy enters the core.
- `submit-files`: loader-checked files and a supplied established management request. This
  increment supports only submissions instantiating protected core templates. It tests their
  rejection, including absence of atomic publication, and resolves core selections without a
  manifest package entry. General registration comes in a later order.
- `request`: supplied evidence, an established root request and a separate `object` ID naming
  the already-resolved object the lifecycle operation acts on. The request's `target` must
  match its Conduit's actual target. This increment supports only LIF-034's refusals; general
  access and lifecycle processing come later. These are semantic inputs, not wire schemas.
- `load`: files checked together by the strict loader, without core registration.
- `read`: raw bytes, with explicit `yaml` or `json` syntax, checked by the strict reader only.
- `validate-values`: a JSON file's values checked against the actual supplied schema using the
  loader's asserted formats. This is schema validation, not a claim of core configuration
  resolution or operation-input processing.

`evidence.ids` are unique UUIDs supplied in consumption order. The core retains every consumed
ID, including record IDs; allocation exhaustion or reuse is an embedding error. Revisions and
five-character suffixes are supplied by the boundary too. For this initial transition the
suffixes need no collision retry because the object store is empty. The transition is atomic:
a caller persists its state and records together before carrying out any external intent.

A step's optional `capture` maps symbol names to JSON Pointers into the result. Subsequent
inputs and expectations may use an entire string `$symbol`; unresolved or rebound symbols
fail. `@core#export` expands the address read from [names.yaml](../spec/names.yaml), in runner
inputs and assertions only. Neither is a Conduit reference syntax. Requests carry established
sender/caller UUIDs and their authorizing Conduit's UUID; no credential bytes enter the model.

The protected case reaches FST-006 for root, whose declared `system` default supplies Pool
registration authority. A separate [Interface with management alone](language/first-start/protected-without-pool/case.yaml)
retains ACC-033's refused-before-diagnostics coverage. The [root-system snapshot](fixtures/root-system-state.json)
is a supplied fixture state, not a state the engine can reach: its extra root-to-system Conduit
is independent of first start. Its final management and access operation lists follow FST-002.
The [real first-start list case](conventions/list/from-first-start/case.yaml) starts empty and lists
all eight instances through root's three declared Conduits (OPS-036).

## Assertions

Maps are required-field projections: every specified member must exist and match. An empty
map asserts an empty map. Arrays have exact cardinality and order; in particular an expected
object array lists every object, including both declared Conduits and their complete operation
lists. Object order in this runner follows the supplied first-start allocations; it is not an
inspection ordering promise. Records follow the one source's order. Projection never permits
an omitted required field or an extra array member. `records`, `reply`, `external_intents` and
`required_promises`, plus `state` or `stateUnchanged`, are mandatory for a core step. Every
emitted decision must cite known promise IDs; implementation choices are in a separate field.
An empty intent list rejects every dispatch. Every emitted record is validated against
`record.v1`; the fixtures additionally assert IDs, supplied times, causes and applicable causal
fields. `stateUnchanged` compares object definitions and lifecycle, allowing required records
and allocation bookkeeping to grow. `absent` asserts that JSON Pointers do not exist, for
example `/reply/outcome` on an undispatched request.

Loader diagnostics compare exact arrays of required-field projections. `diagnostic_facts`
checks message facts and schema keyword attribution by following the evaluation pointer from a pinned root Schema contract
to the named contract's failing keyword, without pinning the bundler's traversal path. The SARIF adapter preserves logical and physical locations and a known absolute
keyword location. Library prose is not a portable expectation. Raw-reader diagnostic locations
are absent where no document was established. Schema-invalid documents and manifests check
SUB-005, identified file/document, instance pointer, named contract and keyword location.

The first-build record vocabulary is `access-decision`, `request-ended`, `invocation-dispatched`,
`invocation-ended`, `provider-call`, `object-change` and `enrollment-change`. First start emits two `object-change`
records per instance: registration and registered-to-ready, with `cause: first-start` and no
invented caller or invocation. LIF-034 and protected-template requests emit an access decision
and an undispatched request end. This vocabulary and the additional event data are runner
conventions, as decision 7 allows; the required fields remain REC-001 and REC-002's.
Lifecycle records retain the named object in `data.object` separately from `data.target`,
which is the actual Conduit target REC-002 requires.

## Running

From `engine/`:

```sh
cargo run --release -p conduit-conformance
cargo test --release
```

The executable accepts an optional repository-root path. It reports each case's pass and
promise IDs, and fails with the case, step and mismatched expectation. Every discovered case
runs; there is no layer filter that could silently skip conventions. The workspace test runs
the same suite. Broken fixtures and unsupported inputs fail as harness errors, never invented
Conduit reasons. An undecidable language case is stopped and logged for the admin in
[the open increment's log](../docs/proposals/first-build/log-increment-4.md).

## Reviewing vector changes

From the repository root, `python3 tools/vector_delta.py [BEFORE [AFTER]]` compares
`origin/main` with `WORKTREE` by default; either side may be a Git revision or `WORKTREE`.
It reads authored cases and fixture bytes, without running an engine or generating expectations.
JSON is read first; YAML cases and declarations require an interpreter with PyYAML.
Cases (`case.yaml`, `case.yml`, `case.json`) are paired by stable `id`, so moves are reported
as renames; changed IDs are removals and additions. Steps are paired by array index.
Each change is a JSON output line followed by a count summary. Case paths are JSON Pointers
with zero-based step indices; the displayed `step` starts at 1. Fixture paths are relative to
the repository, including fixtures outside this directory. All declared file aliases are checked
as bytes, with hashes and byte counts for changed contents. Formatting-only case changes are
ignored; metadata, initial state, inputs and captures are included. Unreadable files, duplicate
IDs and unsupported case/fixture symlinks are reported as errors.

Flips show before and after for `expect.accepted`, `expect.stateUnchanged` and
`expect.reply`'s `reason`, `outcome`, `accepted` and whole `tool` value, including additions or
removals of these assertions in existing cases. An absent `before` or `after` means a missing
value, distinct from JSON `null`. Added and removed cases get one case-level entry each.
Other expectation changes are reported by JSON path.

An order may supply `--declared FILE`, a YAML or JSON list of intended shapes:

```yaml
- name: intended refusal change
  kind: flip
  cases: ['s1-*']
  paths: ['/steps/*/expect/reply/reason']
  before: refused
  after: conflict
- name: registration gains owner
  kind: expectation
  cases: ['s1-register-*']
  paths: ['/steps/*/expect/reply/objects/*/owner']
  operation: add
  after: example-fleet
```

`name`, `kind` and `cases` are required. Kinds are `added`, `removed`, `renamed`, `flip`,
`expectation`, `input`, `fixture` and `metadata`. Case globs match IDs; optional `paths` globs
match report paths and default to `['*']`. Both use case-sensitive `fnmatch` syntax, where `*`
also matches `/`. Optional `before`/`after` constrain exact JSON values; `operation` constrains
`add`, `remove` or `replace`. Omitted constraints match any value; unknown keys are errors.
The first matching shape names the declaration on the change. Errors cannot be declared.
Exit status is 0 with no flips, undeclared changes or errors; 1 otherwise, **even for declared
flips**, which still need review. Invalid arguments, revisions or declarations exit 2.
`python3 tools/vector_delta.py --help` includes the declaration format.

## Authored schemas and E05

I2-02 adds `schema-case`, `schema-admission` and `schema-evaluation` runner inputs. Each names
one declared fixture file; unsupported metadata fields at the case/step/input level fail the
case schema. A schema fixture holds an `environment` of exact supplied scopes (`address`,
optional `version`, `manifest` address-to-scope pins, and ordinary `contracts` documents).
These are preinstalled snapshots, never successful installations. The runner adds the actual
core contracts read from `spec/`, and checks every document with the strict loader. The string
`@apiVersion` expands from the names file, as `@core#name` does for qualified references.

A publication fixture additionally supplies an established `state`, consumer, Conduit, target,
operation and authored scope. The runner injects the supplied schema environment into the
target and calls the core's `tool-definition` input. `samples.inputSchema` and
`samples.outputSchema` each list `{value, valid}` samples. Every sample is checked independently
in its original resource context and against the standalone publication. Unequal verdicts or
one that differs from `valid` fail immediately. Every published reference must resolve locally;
no package resolver is available on that path. Optional `copy_count` asserts the exact number
of contract copies at each root, without counting preserved local definitions. Control
fixtures give separate `narrowings` and `template_scope` for the authored-side intersection;
the core reads narrowing from the target's stored controls. Omitted controls report invalid
for the would-be tool through that open Conduit, and closed Conduits report refused.

Admission fixtures hold `scope` and `documents`; they run strict document validation then the
core's schema admission checks, including IFC-033's reserved root key. Evaluation fixtures
hold `scope`, `schema` and samples with `value` and `mode`: `supplied`, `operation-inputs` or
`output`. Their filled values and validation verdicts are asserted explicitly. Neither input
registers an object or installs an archive. The conformance suite claims these schema checks,
not the later transitions that call them.

For schema inputs, `stateUnchanged` is checked at every core call against that call's actual
snapshot (including publication's injected schema environment), before replies are combined.
The regression tests inject a returned object mutation into each schema input kind and require
failure. Conflicting unconditional defaults are rejected at admission under SCH-009; equal
defaults are accepted. The added vectors cover original control-reference targets, references
to explicit properties also matched by patterns, and defaults in existing map members reached
through patterns or additional properties.

## Package and project contexts

I2-03's first part adds `resolve-project`, naming a fixture file. It runs the strict loader
then the core's context resolution, without registration, instantiation, installation, setup
or execution. Each fixture has `snapshots` and `project`; a project recipe maps original file
names to aliases in the case's `files`, and can supply an absolute `file:` directory URL.
Each snapshot states `source: preinstalled-snapshot`, address, version, digest and a project
recipe. An optional `archive` alias supplies retained digest evidence: the runner hashes those
bytes but neither unpacks nor installs them. This is not evidence for PKG-022 or PKG-026.
Optional `artifacts` entries name a recorded `sha256`, snapshot address and implementation
item; the runner compares that recorded value with the item's Package `fileSha256`. No
third-party artifact bytes are retained or fetched.

The core owns its catalog's snapshots and exposes no mutation interface. It checks the pins
of every manifest, preserving each package's own dependency context, export visibility,
local selections, qualified template selections and Schema references. Duplicate names,
missing exports, mismatched releases and digests, and attempts to claim or list the core
address are rejected. Satisfied unused pins produce NAM-008 notes. The built-in documents and
manifest come separately from `spec/`, including its Schema contracts. Snapshot validation
checks the supplied installed facts; it does not represent an installation transition.

A successful reply contains the derived schema `environment`, resolved `references`, and
`contexts` retaining package identities and implementation items. The core context has no
digest. Relative programs are expanded in those items and listed under `relative_programs`,
retaining PRV-007's provenance. The project `digest` hashes only contract documents and the
original manifest under SUB-008, before expansion. Reordered contracts, added templates and
changed source directories leave that digest unchanged; changed contracts or manifest values
do change it. A rejection exposes no partial environment, contexts or digest. Every case
asserts no intents, no records and unchanged objects.

The [S1 fixtures](fixtures/s1/README.md) retain the three packages and project. The other
package cases use small ordinary documents and deterministic archive evidence. These cases
also cover implementation bindings (MAN-010, MAN-017), plain tool mappings and type-family
checks. They do not claim full registration or instance argument/configuration validation;
local submitted `from` and object references remain the later registration order's work.

## Registration vectors

I2-04 adds `register-project`: the same ordinary-file fixture as `resolve-project`, plus a
request and allocation evidence. It runs the loader, then atomically registers and resolves
on the core. Evidence lists object IDs followed by record IDs, one revision per object and
five-character suffix candidates. `reverse: true` reverses the loaded document sequence.
`registration-orders` runs the same input from the same state three times: ordinary order,
reversed documents, and reversed files with rotated documents. It requires exact equality of
the complete transitions, including decisions, allocations, diagnostics and records, then
reports `orders_compared: 3`. It commits one resulting transition to the case's state.

The [S1 registration case](language/registration/s1/case.yaml) checks all 19 new objects:
nine templates, six named instances and four nested Conduits. Exact object and record arrays
prevent extra publication or activity from being hidden by projections. The cases check
configuration, owner and placement IDs, unopened nested instances, missing/invalid arguments,
default precedence, null, relative paths, reference-schema chains, ordinary data, authored
IDs, nested targets, implementation selections and name collisions. A later submission with
a changed contract demonstrates that an existing template still instantiates against its
own held project. Accepted and rejected S1 variants compare across file/document orders.
These vectors exercise registration only; they start no implementation or provider.

The verification regressions include omitted Type schemas, defaults inside config, requirement
arguments resolved through their own contract context, retained package availability on a later
submission, retained entry placement/runtime IDs, active entry replacement lookup, unresolved
nested template names and defaults, and standard argument names conflicting with declared inputs.
Input-supplied operations and explicit runtime selection have accepted and rejected cases,
including a wrong-family runtime rejected by the document schema. The three S1 cases assert
every Conduit's endpoints, capability and exact operations, selected filesystem implementations
and exact retained package identities.

## Archive installation inputs

I2-03's second part adds `install-archive`, naming an ordinary runner recipe. It supplies the
archive's `name`, requested `address`, `version`, expected `digest`, optional absolute directory
URL, and `completion`. A complete fetching result is `{status: complete, file: <archive alias>}`;
the runner reads that alias and supplies the exact compressed bytes to the core. An unavailable
result is `{status: unavailable}`. The boundary supplies the built-in schemas; the core
constructs the strict loader in memory. No package source or provider is invoked.

The core verifies PKG-004/026 before decompressing. It unpacks the profile's USTAR dialect
in memory, checks all member paths and link targets, selects root documents under MAN-011,
and uses the loader's pure project reader. Root manifest placement, requested address/version,
instance prohibition, exports, dependencies and all existing package-resolution checks must
pass before the package is published. Failures report PKG-023's invalid reason and preserve
the installed catalog. An unavailable completion reports PKG-024; an existing address/version
is decided without inspecting archive bytes under PKG-025/027.

An installed entry has `source: installed-archive`, its exact identity, loaded documents and a
virtual directory retaining every file's bytes and mode. It survives state serialization and
is included in later resolution and registration. Snapshot recipes accept only
`preinstalled-snapshot`; they cannot claim installation. The legacy `snapshot-N` scope names
remain catalog positions for compatibility, while each context's `source` distinguishes its
provenance. `stateUnchanged` checks installed entries as well as objects.

The [S1 archive case](language/archives/s1/case.yaml) first resolves from snapshots, installs
filesystem, history and Agent archives in order, then resolves with no supplied snapshots.
It compares the environment, resolved references, original project digest, expanded manifest
and every context field with the snapshot result, apart from the explicit source marker.
The [generated variations](fixtures/install-archives/README.md) prove rejection and skipping
rules with retained bytes and computed hashes. The installation input exercises archive
admission and catalog publication; the general `install.v1` access/dispatch/record workflow
remains a later convention input, as does the shell's persistence of the virtual directory.

## Access and current tools (I2-05)

The [access cases](conventions/access/s1-exact-access/case.yaml) register S1 from its real
snapshots before querying access. `access-fixture` names a document with an `observations` array and selects its zero-based
`index`: `patches` supply
already established lifecycle facts on that registered state, `invocations` optionally supply
trusted in-progress serving state, and `call` is the core's `access` or `list-tools` input.
These observations do not claim setup, opening, closing, an invocation or dispatch. A patch
identifies an instance by `id` or `name` and sets supplied members with `set`, or appends a
supplied object with `append`. An optional `after` supplies a second observation; `changes`
then compares the two private tool lists and names the consumers whose subscribed streams the
shell must notify. The runner checks that each call preserves its actual supplied objects,
invocations, installed packages and start state, produces no intent or invocation record, and
that refused/unavailable replies contain only a reason and an optional restriction. Only its
records and consumed record IDs continue into the outer sequence.

The public core inputs take the established identity (`instance`, `none` or `unreadable`),
the tool name, arguments, and optional serving ID. `readable` and `authority_readable` are
boundary facts, never authorization supplied by a consumer. Registration's documents enter
as the operation's ordinary `arguments.documents`. The core checks current operation lists,
ACC-033's routing, bounds and carried filters, fills input defaults using target-held
contracts, and records every decision. A permit returns the attributed request for the next
stage; it creates no invocation and cannot be reused after the state changes. Dispatch and
idempotency are the next order's work. The shell also owns protocol envelopes: these vectors
check the core's reason and diagnostic projections, not a network JSON-RPC exchange.

Unknown identity or tool means there may be no target or caller to record. Access projections
therefore require those fields where applicable, rather than inventing an ID; every other
transition's record projections continue to require a target. Lists are exact sorted arrays,
private, and declare `listChanged`. The client experiments do not establish that a model's
cached tool catalog follows these lists: even a cached or guessed call is decided anew here.
The bounds case checks Pools, individual targets, operations, omitted/empty bounds and
narrower/wider delegated bounds. The two plain-filter cases test both registration rejection
and an onward call carrying a filter to a plain filesystem implementation.


## Request endings (I2-06)

The [request cases](conventions/requests/plain-content/case.yaml) begin by registering S1.
`request-fixture` names a control document holding a `requests` array and selects its
zero-based `index`. Each observation holds a `call` (`invoke`) and supplied ready/open
`patches`. Optional `contract_updates` select a target, contract `kind` (Operation by default) and name and replace spec
members in its held schema environment; `aware` and `target` select the corresponding
nonplain implementation variation. They describe independent established contexts, not
language operations. The runner checks that the core transition changes no objects or
packages after those observations.

`invoke` takes current access inputs and an optional idempotency `key`. It returns a pending
invocation and dispatch intent, or an IFC-025 result (a baseline protocol error for an
unpresented tool). `complete` supplies an invocation ID, attempt number and `result`,
`protocol-error` (including optional `data`) or `missing` completion. A safe -32020 repair yields a
`refresh-tools-and-call` intent within the same attempt, at most once. `cancel` is a trusted
boundary notification and can carry an implementation result as `confirmation`; without
valid no-effect confirmation its outcome is unknown. No intent executes a call.

`restore-state` serializes and restores all state, then calls the core’s `recover` transition.
Interrupted attempts end unknown under IMP-007, with no dispatch intent. Recovery of an
interrupted attempt requires supplied `evidence` for its records; settled state needs none. Its optional `records_purged`
observes an empty record store left by an
external purge; it is not a `purge.v1` implementation. `expire-outputs` explicitly exercises
DL10, removing retained outputs and plain content, while preserving the invocation.
The [finding log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md#i2-06-request-endings-work-order-015)
records the admin’s narrowing of header repair and the return-review regressions.

## Setup and Conduit lifecycle (I2-07)

The [setup cases](conventions/setup/base/case.yaml) register the ordinary S1 project
before making lifecycle requests. They cover the base setup, a complete second setup with
fresh keys, per-open sender authority (including an onward setup), opening before a target
is ready, independent A/B closing, a result arriving after close, and explicit restoration.
Additional cases cover optional declarations, ordered Pool entry setup, run reports,
partial setup without rollback, cancellation, hook keys and outcomes, concurrent run and
remote-connection parts, and replacement against a changed or missing template or retired target.
The registration-only Agent types still launch no wrappers or model work.

`setup-report` carries the setup invocation ID and a report: `inspect-run` with `state`,
`run` with `outcome` and an optional provider `url`, or `connected` with the selected `part`
and its `outcome`. It supplies evidence rather than performing a runner or network action.
A `lifecycle-fixture` selects an indexed request from a declared file, just as a
`request-fixture` does, while allowing lifecycle changes to persist. Its optional patches,
implementation updates and supplied serving invocations describe small explicit variants;
the base case uses the real lifecycle transitions throughout. An observation may remove an
object to represent an unregistered template. `@core#` also expands in fixture map keys.

Assertions `object_count` and `absent_objects` check exact cardinality and removal;
`object_facts` and `invocation_facts` select objects by ID (or instance name) and compare
required fields. `record_facts` requires matching records in the stated causal order.
`stored_record_facts` applies that assertion to all records in the resulting state, including
earlier transitions, so a removal case can prove that earlier records remain.
A lifecycle fixture may name `archive_file`, an alias in its case's `files`; the runner supplies
those compressed bytes as the call's archive completion, so public-install cases share the retained
archive instead of embedding a second copy.
These supplement the existing exact arrays and promise checks. The base case checks both
open invocations' sender, original caller and parent links, stable nested IDs and revisions;
the close case checks the exact remaining private tool list. For a development subset,
`cargo run --release -p conduit-conformance -- .. setup-` runs IDs with that prefix. Omitting
the prefix runs every case; the coverage inventory is validated in either mode.

The `setup-review-` cases exercise the return review of order 016: dependency order
and a failed optional dependency, hook filter reasons with and without earlier effects,
a cancelled Pool entry run, retirement before a run and during concurrent parts in both
completion orders, frozen reference inputs during replacement, complete Running conditions,
an expanded relative command with an explicit runtime, and provider calls within entry
setup. The runner validates every condition against `condition.v1` and rejects request route
fields on every provider-call record, in addition to the vectors' positive and absent-field
assertions. The condition cases check unchanged transition times, changed statuses, preserved
unrelated conditions and the last successful heartbeat when observation becomes unknown.

Work order 017 adds the `records-*` cases for inspection, explicit export, unreadable ranges,
root-only purge and its permanent certificates, E11 replacement and old contexts, E12 stale
expected IDs after observed name reuse, and E08's trusted broker and carried-filter boundary.
The E12 observation supplies the state after removal/name reuse; it does not claim retirement
recovery. The broker fixture supplies the trusted adapter's approved onward request; it does
not test that adapter's approval implementation. New registration occurrences use additional
supplied IDs, preserving the earlier vectors' object allocations.

`export-records` takes optional `unreadable` record IDs as read-availability evidence. The
profile defines its gap fields and event actions. Every transition validates all retained and
new records, checks applicable correlation and provider fields, and compares inspection's
records and unknown invocation IDs as exact ordered lists. `D19` output measures record,
record-gap and invocation-index bytes separately and reports the growth per step and each
invocation's key status and outcome. Projection of other implementation-defined fields never
makes a required portable field optional.

The return review adds keyed successful registration/replacement replays and keyed refused,
invalid and preflight-conflict endings. The runner compares each supplied key directly with
both outer request records, even without an invocation entry, and validates generated,
retained and replayed successful built-in outputs against their core Operation contracts.
Template inspection omits status under LIF-012; instance inspection keeps LIF-013's complete
status. The missing-Resource pairs assert management's invalid diagnostic and the Pool-only
caller's refusal without any object disclosure, as finding 2-08's admin settlement requires.

## S1 whole and promise inventory (I2-09)

[s1-whole](conventions/s1/whole/case.yaml) starts empty and runs 58 steps: first start,
resolving real snapshots, registration, Pool/Resource/Agent setup, four declared opens,
T1/T2 requests with scripted server results, T7's independent close, refused old keyed call,
B still writing/reading, history still working, and explicit nested replacement before setup
restores A. No fixture patch supplies lifecycle progress. Every dispatched external request
ends before the trace finishes; plain intents contain no Conduit metadata. Scripted text is
not evidence of real file effects, Agent behavior or confinement.

The manifest's `s1_coverage` contains exactly the 308 promise IDs in the
[frozen S1 inventory](s1-promises.yaml), copied from increment 2's study, section 2, including
its observation paragraph. The [safety traces proposed for S1 in increment 2](s1-safety-cases.md)
are preserved separately; they do not define the manifest's `safety_rows` tags.
Each row has vectors declaring that promise, or a specific reason
why no vector does. A case tag demonstrates only its asserted slice; neither a tag nor an
explicit reason proves a whole promise. The runner checks the inventory even with an ID
prefix: missing/extra/duplicate rows, empty reasons, unknown vectors and vectors that do not
declare the promise fail. The inventory currently has 217 rows with vectors and 91 with
reasons, including loader tests not yet ported, missing boundary cases and shell work. Those
reasons remain visible coverage debt, logged in the increment's I2-09 section.

The first build's [conformance choices](../engine/CONFORMANCE.md) and the
[increment 3 handoff](../engine/README.md#the-cores-boundary) state the current
interface, outstanding core work and shell evidence. The increment log also proposes how a
later order can reduce repeated fixtures and state/record assertions; this order does not
rewrite the legacy vectors.

## Planned work

These are coverage and representation proposals under A39, not new language promises.

### Lean vectors

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


At the condensation plan’s measurement, conformance/ was about 12 MB and 414k words;
these are historical sizes, not measurements of this edit. Keep case IDs and attributions
while measuring any later migration; changing storage for invocation contexts is separate.

### Coverage and safety tagging

Port loader-only rows as portable vectors and add semantic boundary pairs: capability/Type
rejection, frozen placement, consumer crash, standalone opens, missing revisions and expected
retirement conflicts. Qualify release compatibility: the old “before T5” requirement was
coverage debt, and T5 having run does not qualify that semantic boundary. Credential secrecy,
actual process environment, client catalogs and around-engine reach need shell evidence;
scripted core facts cannot supply it. The manifest records unvectorized reasons, not passes.

Carry D08’s nested Resource variant: put shared in a Pool entry, retire it explicitly, use
register-replacement.v1 on that entry, then restore access. Measure its extra structure;
a top-level Resource replacement does not establish nested replacement. T4/T5/T6 preserve
Agent/history identities only through explicit transitions; old references do not retarget.

Reconcile the historical 74 cases without safety_rows and review the 38 empty lists against
[s1-safety-cases.md](s1-safety-cases.md). Those traces are proposed safety work; safety_rows
currently has its own tag vocabulary. Neither the counts nor the tags define a safety-promise
list or establish coverage. Port the scenarios of the 16 “vector pending” and 29 DL-mapped
runtime rows in [LANGUAGE_DEFECTS](../docs/LANGUAGE_DEFECTS.md), with their actual evidence limits.

### Evidence of vector strength so far

Historical evidence copied verbatim from increment 2; this is the evidence toward A44 and G6,
not a fresh run or a claim beyond the assertions and limits below.

**S1 assertion probes.** Two temporary mutations to the resolver were compiled separately:
retargeting each new Conduit to its consumer, and appending an extra operation after the
capability check. Each mutation failed all three strengthened S1 variants, respectively at
`target/id` and the exact `operations` array length. The source was restored after the probes;
neither mutation is part of the change.

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

**Vector.** [records-expected-id](conventions/records/expected-id/case.yaml)
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
   answer. [records-replay-register](conventions/records/replay-register/case.yaml)
   and [records-replay-replace](conventions/records/replay-replace/case.yaml)
   compare the retained outputs and invocation ID, unchanged objects, and exactly an access
   decision and request ending, with no dispatch. Both failed on the supplied tree: OBJ-002
   and REV-002 conflicts respectively.
2. **Template inspection — LIF-012, LIF-013 and inspect.v1's outputSchema.** LIF-012 says,
   "It exists until it is unregistered, and has no lifecycle state", and "Inspecting a template
   returns no lifecycle state, and no operation gives it one." LIF-013's `when` is "An instance
   is inspected". The contract makes `status` optional and requires `state` whenever it is
   present. These decide the case: omit template status, retain all instance status, and invent
   no template state. [records-inspection](conventions/records/inspection/case.yaml)
   now forbids template status and cites LIF-012; it failed before the fix. The runner validates
   generated and retained successful built-in outputs and successful repeat replies against
   the actual core contracts. Its negative schema test rejects status without state, and its
   inspection checks reject a lifecycle state on a template.
   This validation also exposed `register.v1` emitting `nested: null` for ordinary objects
   and internal parent/slot fields for nested ones. The `object.v1` contract and INS-016
   decide the result shape: omit `nested` for ordinary objects and return `instance`/`name`
   for nested ones. The [Pool-entry registration](language/registration/pool-entries/case.yaml)
   vector now asserts those exact required fields and the absence of storage fields.
3. **Keys before invocation allocation — REC-002.** Its `then` requires the supplied key,
   and its observable says "Each of those is present in every record to which it applies."
   [records-keyed-refused](conventions/records/keyed-refused/case.yaml),
   [records-keyed-invalid](conventions/records/keyed-invalid/case.yaml)
   and [records-keyed-conflict](conventions/records/keyed-conflict/case.yaml)
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

### Increment 2's done-when, item by item

| Item from the first build | Evidence and limit |
| --- | --- |
| S1's documents register and resolve | `s1-whole` resolves the actual filesystem/history/Agent snapshots and registers the ordinary S1 project after actual first start. It asserts the 19 new objects, package identities, endpoints, operation lists and lifecycle progress. `registration-s1-orders` additionally checks order independence; `archives-s1` checks real compressed bytes separately. |
| Every decision cites a promise | The runner rejects every emitted decision with no promise or an unknown ID, on all core steps. DL choices occupy their own field. This is executable citation coverage, not a proof that every citation is sufficient. |
| The safety vectors pass | The complete suite is required, including E05 standalone schema cases, access/refusal/bounds/filter boundaries, request endings/repeats/restart, partial setup/cancellation, E08's supplied trusted broker, E11 frozen contexts and E12 stale IDs. The run results below report actual checks, not expected passes. |
| Each case the core cannot decide is a logged finding | Findings 2-01 through 2-08 are settled in this log, including the return reviews. This integrated trace needed no new language answer. Unsupported implementation paths and missing coverage are recorded below for the admin; they are not silently answered or represented as language settlements. |

The [manifest](manifest.yaml) now lists the study's complete inventory:
308 distinct promises including OBS-001/002/006 in its closing paragraph. 217 have vectors;
91 have explicit reasons. The runner validates the exact inventory and case references even
on a development subset; a regression test removes a row, removes a reason and supplies a
false case attribution, and each must fail. Case tags claim their asserted slice, not an
entire promise. Loader workspace tests not yet ported to portable cases, missing semantic
boundary pairs, release-skew/compatibility and shell-only evidence are distinguished in the
reasons. No promise S1 touches is omitted without explanation. This does not turn the 91
reasons into conformance results.

The continuous S1 case now asserts exact ordered record arrays for the twelve previously
unchecked management/setup steps: the initial open, Pool setup, each Resource's setup and
reports, B's setup, close, replacement registration and replacement setup. A's existing
three-record dispatch subsequence is also replaced with its complete fifteen-record sequence,
so both Agents' nested opens and all their request endings are required. These thirteen
steps assert 75 records, including caller/consumer/Conduit/operation/invocation correlation,
applicable serving and idempotency keys, lifecycle changes, and invocation/request endings.
Exact arrays also reject additional or reordered records. No core behavior changed.

**Return-review verification.** A temporary runner harness first ran `s1-whole` successfully,
then reran the case 75 times, removing one of these records from the observed transition
immediately before expectation matching on each run. This deliberately bypassed the global
record invariants, so the vector's own expectations had to catch every omission. All 75 runs
failed at the affected step's `/records` expectation with one fewer record than required.
The runner source was restored; no fault injection remains. The release
workspace tests passed on the restored source, including all 362 portable cases; clippy
with warnings denied and formatting passed too.

### Enrollment inputs and assertions

`machine-report` carries evidence (including time and allocation IDs), provider, outcome and,
for succeeded, list-machines.v1 outputs. `clock` carries evidence and closes expired
enrollments. The shell does not yet commit either autonomously (order 051c).
`setup-report` additionally accepts `{kind: enroll, outcome, outputs}` (outputs only for
succeeded), `{kind: accept-machine, outcome}` and `{kind: remove-machine, id, outcome}`.
Each case supplies every provider ending; no fixture provider program runs.

`expect.panel` is a required-field projection of the pure root panel view at the input's time.
`expect.undisclosed` is an array of `{text, whole_state?}` assertions: each string must occur
nowhere in the reply, decisions, intents, new or retained records, invocations, objects or
exported records. By default it excludes the private enrollment and latest-report fields;
whole_state true also searches those. It searches decoded strings and keys. Arrays in panel
projections retain the ordinary exact sequence semantics. A fixture input may supply its time
inside its call. These assertions expose no command or MCP tool.

The seventh CloudEvent record type, enrollment-change, records an enrollment transition with
the provider Resource as subject. Machine provider calls also use that Resource as subject.
Autonomous list-machines.v1 reports record made and ended without invocation/caller and
without outputs. Enroll endings never record the join action or its message. See DL8 in
[engine conformance](../engine/CONFORMANCE.md#enrollment) for the two omissions beyond the
type's records policy and the work left to orders 049a2, 049b and 051c.
