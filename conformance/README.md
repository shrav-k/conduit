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
lists which choices this order implements and which belong to later orders. No package is
installed by this suite. Future package fixtures must distinguish preinstalled snapshots from
archive-install inputs with real bytes and hashes.

## Inputs and files

`files` maps aliases to paths relative to the case directory; shared fixtures may use relative
paths into the repository. Paths are never fetched. A submitted alias is the original file
name, including `conduit.yaml` when it is a manifest. Documents are ordinary Conduit files:
runner symbols never appear in them. Bytes that intentionally fail reading live in `bytes/`.
No size or count cap is imposed on files, submissions or archives by these cases; the nesting
limit is 128 and the portable floor is 64.

Each step has one `input` and an `expect`. Supported input kinds are:

- `first-start`: supplied `evidence` (time, UUID allocations, revisions and generated suffixes)
  and a local `machine` report. The seven object IDs precede the fourteen record IDs. A repeat
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

The protected cases retain unauthorized Resource submissions into the existing system Pool:
ACC-033 row 1 requires registration authority to both management and that Pool, and OPS-023
requires refused before any FST-006 diagnostic. Their replies assert diagnostics are absent.
The authorized counterpart uses [an established state](fixtures/root-system-state.json) with
an additional open root-to-system Conduit listing `register.v1` and `retire.v1`. Registration
there reaches FST-006. Resource retirements through that Pool Conduit still report refused
under LIF-034, ahead of lifecycle conflicts under OPS-023. The snapshot does not claim first
start made the extra Conduit. The empty-state first-start case retains its exact seven objects.

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
`invocation-ended`, `provider-call` and `object-change`. First start emits two `object-change`
records per instance: registration and registered-to-ready, with `cause: first-start` and no
invented caller or invocation. LIF-034 and protected-template requests emit an access decision
and an undispatched request end. This vocabulary and the additional event data are runner
conventions, as decision 7 allows; the required fields remain REC-001 and REC-002's.
Lifecycle records retain the named object in `data.object` separately from `data.target`,
which is the actual Conduit target REC-002 requires.

## Running

From `reference-model/`:

```sh
cargo run --release -p conduit-conformance
cargo test --release
```

The executable accepts an optional repository-root path. It reports each case's pass and
promise IDs, and fails with the case, step and mismatched expectation. Every discovered case
runs; there is no layer filter that could silently skip conventions. The workspace test runs
the same suite. Broken fixtures and unsupported inputs fail as harness errors, never invented
Conduit reasons. An undecidable language case is stopped and logged for the admin in
[the increment's log](../docs/proposals/first-build/log-increment-2.md).

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
Optional `artifacts` entries name a file alias, snapshot address and implementation item; the
runner checks its bytes against that item's Package `fileSha256`.

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
