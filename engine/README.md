# The engine

Not binding. The specification in [spec/](../spec/README.md) alone binds, and this code follows
it: where the two differ, the code is wrong, or the difference is a finding of the first build.

This is the codebase of the first build, as [ruling 46](../docs/proposals/ruling-46/README.md)
shaped it: one Cargo workspace in Rust holding the pure core (A38),
in which every decision cites a promise, and the shell that makes it runnable, the first
slice of the reference engine (A50). [The first build](../docs/implementation/FIRST_BUILD.md)
pre-registers its increments and what each must show, and the
[roadmap](../ROADMAP.md#the-order-of-the-work) says where it stands. Each increment logs what it
finds under [docs/proposals/first-build/](../docs/proposals/first-build/README.md).

It holds the strict loader of increment 1 (A43), in [crates/loader](crates/loader), whose
package is `conduit-loader`; the pure `conduit-core` in [crates/core](crates/core); and the
`conduit-conformance` runner in [crates/conformance](crates/conformance); and the
`conduit-shell` library with its `conduit` binary in [crates/shell](crates/shell).

State on 2026-11-10: increment 1, the loader, was done on 2026-11-09; increment 2, S1 whole on
the pure core by vectors, and increment 3, the shell, were done on 2026-11-10. The shell ran
real Claude Code and Codex as Agents through T1, T2 and T7; the
[closeout results](../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)
separate what was shown from what was not. Increment 4 is in progress: both the
[coordinator](../docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md) and
[fresh author](../docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md)
completed T1 to T7 with real clients, writing 332 and 646 lines against the baseline's 64.
Both failed the owner's bar for authoring work. Orders 025 to 028 supplied change operations and
command fixes; the [log](../docs/proposals/first-build/log-increment-4.md) records their limits.
Rulings 49 to 54 await specification changes while the language's shape and Projects are
settled, before simplification is built and S1 rerun. The sections below describe the
implemented slice as of 2026-11-10, distinguishing
each order's tests from that later closeout; they do not claim whole-language conformance.

## The core's boundary

This describes the current code, including the retirement and install workflows added after
increment 2. The API is [`conduit_core::step`](crates/core/src/lib.rs):
`step(&State, Input, &Context) -> Result<Transition, String>`. No filesystem, clock, network,
process or ambient allocation runs inside it. Unsupported paths and malformed embedding
facts are errors, not invented Conduit reasons. Consult [CONFORMANCE.md](CONFORMANCE.md) for
remaining implementation limits before exposing a management operation.

`State` holds started, objects, records, allocated_ids, record_gaps, invocations,
installed_packages and preinstalled_snapshots. The invocation index includes held contexts,
setup/retirement progress and retained replies; records alone cannot reconstruct it.
`Context` supplies builtin, core_package, meta_prefix, templates, record_source and
core_environment from the names file and core documents. `Evidence` supplies time, ids,
revisions and suffixes; allocation exhaustion/invalidity fails the embedding input.

### Inputs

The kebab-case tagged variants of `Input` are:

| Input | Values besides kind | Purpose |
| --- | --- | --- |
| first-start | evidence, machine | Bootstrap once from supplied hostname/labels/state |
| resolve-project | snapshots, builtin, project | Resolve documents without registration/effects |
| register-project | snapshots, builtin, project, evidence, request | Resolve and register atomically |
| install-archive | archive, snapshots, builtin | Pure byte/hash/archive admission |
| invoke | evidence, request, key?, archive? | Current-authority invocation, including supported engine workflows |
| complete | evidence, invocation, attempt, completion | Settle the identified dispatch attempt |
| cancel | evidence, invocation, confirmation? | Trusted cancellation and optional implementation confirmation |
| setup-report | evidence, invocation, report | Resume outstanding setup or retirement phase |
| recover | evidence | End interrupted work unknown without implicit replay |
| builtin-run-stopped | evidence, object | Record an observed built-in process exit and stopped state |
| expire-outputs | invocation | Explicit retention boundary, preserving invocation state |
| access | evidence, request | Access-only verdict, no invocation; a permit cannot outlive a state change |
| list-tools | identity | Current consumer-private tool list |
| tool-definition | consumer, conduit, operation | Standalone tool schema publication |
| export-records | unreadable? | Records and read-availability gaps |
| check-authored | environment, scope, documents | Authored schema admission |
| evaluate-schema | environment, scope, schema, value, mode | Schema validation/defaulting |
| submit | evidence, request, documents | Lower-level protected-template test boundary |
| request | evidence, request, object | Lower-level resolved lifecycle test boundary |

Invoke/Access use [`AccessRequest`](crates/core/src/access.rs): identity, tool, arguments,
serving?, readable? and authority_readable?. Identity is instance/id, none or unreadable;
the availability booleans default true. The shell authenticates and supplies these trusted
facts; a consumer cannot assert its identity or original caller. The lower-level `Request`
contains resolved sender, caller, conduit, target and qualified operation. Its target is the
actual Conduit target, distinct from an object named in arguments.

A [`Project`](crates/core/src/packages.rs) retains manifest, ordinary loaded documents and
directory; preserve the original manifest for SUB-008. Snapshots carry source, address,
version, digest and project. An [`ArchiveInput`](crates/core/src/archives.rs) carries name,
address, version, digest, directory and completion (complete/bytes or unavailable). Installed
archives retain virtual file bytes/modes and directory/link entries. Case aliases, captures,
observation recipes and fixture patches belong to the runner, not the core or consumer wire.

### Transitions, intents and completions

A `Transition` returns state, decisions, new records, external_intents, reply and
request_replies. Decisions carry promises, separate implementation_choices, facts and
affected IDs. A request reply carries invocation, attempt and its reply even if invocation
state was removed or restored. A pending reply is internal until completion. The shell must
persist the transition and records before performing any intent or returning a success.

The current intent producers are [requests.rs](crates/core/src/requests.rs),
[lifecycle.rs](crates/core/src/lifecycle.rs) and [retirement.rs](crates/core/src/retirement.rs):

| kind | Correlation and action | Returning input |
| --- | --- | --- |
| dispatch | invocation, attempt, target, held implementation and tools/call | complete with same invocation/attempt |
| refresh-tools-and-call | Same fields; one permitted header/tool refresh and safe resend within the attempt | complete |
| builtin-runner | operation, invocation, caller, subject, inputs; inspect, run or stop | setup-report |
| provider-call | Same fields plus provider, config and revision; selected runtime provider | setup-report |
| connect | part, invocation, subject and selected implementation | setup-report connected with the matching part |
| cancel | invocation and attempt; notify an already-ended invocation | No settling completion |
| cancel-setup-parts | invocation and subject; cancel outstanding parts, begin none later | No settling completion |

Use held implementation selection, not a new name lookup. A plain dispatch carries only the
operation arguments; no Conduit metadata/config/revision/identity goes to that server.
Conduit-aware calls carry the names-file-prefixed metadata; a setup hook omits conduit.
The consumer’s serving key is interpreted by the engine and is not forwarded.

[`Completion`](crates/core/src/requests.rs) is result/result, protocol-error/code/data?
or missing. Late/duplicate completions do not change a settled or cancelled outcome.
Plain content survives successful replay; nonplain envelopes/outputs are validated. A plain
-32602 is failed only with no effect, control or model work, unknown otherwise. Only the safe
category gets one -32020 repair; an effectful call is not implicitly sent again.
[`Report`](crates/core/src/lifecycle.rs) is inspect-run/state, run/outcome/url?, stop/outcome
or connected/part/outcome. Correlate it with the outstanding phase; ended plans ignore late
reports. Run success needs a provider URL where required. Cancellation confirmation is
trusted evidence, not the caller’s assertion that an effect failed.

### Identity and the shell

A run intent’s identity is a descriptor, with materializeFor, environmentNames and fileNames
read from the current run contract. The core creates no credential or directory. The shell’s
[`materialize_identity`](crates/shell/src/lib.rs) constructs the actual identity environment
and files through its authority, for the named instance, using the store and engine URL.
Keep credentials out of Decisions/records; names come from the contract, not copied literals.

The shell loads bytes, supplies time/IDs and provider/process observations, owns durable
state/record publication, timers, output expiry, restart recovery, process supervision,
artifact installation and cancellation. Its Executor implements each intent and submits
correlated completions/reports; it supplies no second interpretation of access or lifecycle.
It presents authenticated MCP, current private lists and version-appropriate results/errors,
checks cancellation ownership, and owns transport/Origin/authentication behavior. After a
committed change, `access::tool_list_changes` computes consumer-specific list notifications;
it is a helper, not an intent from step. Client model catalogs and routes around the engine
need observed shell evidence; the core cannot establish confinement. Future shell evidence
and unfinished choices belong in [CONFORMANCE.md](CONFORMANCE.md) and the open increment’s
[log](../docs/proposals/first-build/log-increment-4.md); vector work belongs in
[Planned work](../conformance/README.md#planned-work).

## Running it

The toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml), Rust 1.97.1, which
rustup installs on first use. Run the commands from this directory, so that the pin applies:

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release -p conduit-conformance
```

`Cargo.lock` is committed, and the build output, `target/`, is ignored. The tests read the
specification from `../spec/`, so they run inside a checkout of this repository.

For real Agents, follow [live S1](examples/s1/README.md): build and pack both Agent packages,
set their host archive digests in a copy of the project, prepare the shared and history
directories, start `serve`, then explicitly install the dependencies and submit the project.
Claude Code and Codex must be installed, on the runner's PATH and signed in. As of the 2026-11-10 closeout they use the owner's shared logins;
setup starts wrappers only, and each `conduit call` to a turn tool starts model work explicitly.
The owner's [setup guides](../docs/guides/README.md) prepare later experiments. The separate
homes and engine-mediated sign-in of [ruling 49](../docs/proposals/ruling-49/README.md) are
pending as of 2026-11-10, not prerequisites already supplied by this shell.

## The shell's store, runner and command line (I3-01/02/03)

From this directory, with an existing parent directory for the store:

```sh
cargo run --release -p conduit-shell -- init --store /tmp/conduit-store
mkdir -p /tmp/conduit-s1/shared /tmp/conduit-s1/history
cargo run --release -p conduit-shell -- serve --store /tmp/conduit-store
# In another terminal, install filesystem, history and agents in this order.
# Use the SHA-256 of each archive (the checked-in project's manifest pins it).
cargo run --release -p conduit-shell -- install --store /tmp/conduit-store ../conformance/fixtures/s1/archives/filesystem.tar.gz#SHA256
cargo run --release -p conduit-shell -- install --store /tmp/conduit-store ../conformance/fixtures/s1/archives/history.tar.gz#SHA256
cargo run --release -p conduit-shell -- install --store /tmp/conduit-store ../conformance/fixtures/s1/archives/agents.tar.gz#SHA256
cargo run --release -p conduit-shell -- submit --store /tmp/conduit-store ../conformance/fixtures/s1/project
cargo run --release -p conduit-shell -- inspect --store /tmp/conduit-store
# Stop serve with Ctrl-C before exporting records.
cargo run --release -p conduit-shell -- records --store /tmp/conduit-store
```

First start uses the machine's hostname, OS and architecture, plus no custom labels unless
`--label KEY=VALUE` is supplied. Install every required package explicitly, including S1's
filesystem, history and registration-only agents packages. Neither registration nor first
start installs them. `submit` reads the directory through the strict loader, registers it,
then explicitly requests opening root's submitted Conduits and setup. The built-in runner now starts S1's unchanged npm filesystem
server for each held directory; those directories must already exist. `register` waits for
initialization and setup to end. This local command owns its runs and ends them when it exits;
`serve` owns the persistent runner and consumer HTTP server. Every open of a store
first commits recovery, ending interrupted invocations unknown without replay. Inspection makes
core inspect requests as root, one JSON line per object, including authority refusals.
Records exports ordered CloudEvents as JSON lines and reports gaps on stderr; stop `serve`
before exporting records. Any printed error result, rejection, or outcome other than
`succeeded` gives a nonzero exit. Inspection still prints every object's result when some
inspections are refused. `submit` stops at its first failed step and prints JSON summary
lines with `step` and `outcome` after the result lines, including the failed step. Earlier
successful steps persist; inspect and resume explicitly through `call`.

`conduit --help` and `conduit COMMAND --help` describe arguments and examples without a store.
`tools --as NAME` also accepts `KIND/NAME`, distinguishes absent instances from instances
without a client identity, and explains setup/opening when no tools are presented.
Client initialization waits up to 10 seconds per attempt, with three attempts. Errors name
the initialization step and elapsed wait. Authentication runs its blocking store read off
the async workers. A sent request is never automatically retried, including after an expired
session. Run observations are updated when startup succeeds; CLI inspection shows ended
observations separately as `pastConditions` with `endedAt`, while current `conditions` holds
the latest observation. The specification has no defined ended-condition form; this history
is a CLI supplement, like `consumedConduits`.

The shell holds an exclusive store lease. One mutex serializes read/step/commit; executors
run outside it, and their completions reenter it. The whole State and records are committed
in one atomic, fsynced file before work or final replies. Pending requests have an internal
Ticket that waits for a core ending. Transition.request_replies carries the core's exact
final replies, correlated by invocation and attempt, so a refusal is delivered even if the
core removes the invocation or restores its previous success. The [runner](crates/shell/src/runner.rs)
executes committed run, dispatch, refresh and cancellation intents outside that mutex, returning
completions through the shell. Runtime providers and remote connections remain unserved;
missing/failed reports go through the core. The base S1 fixture used above has no executable
Agent; [live S1](examples/s1/README.md) supplies the real-client wrappers instead.

Config sets the engine address and output retention. CLI equivalents are `--url`,
`--output-cap-bytes` and `--output-age-seconds`; defaults are a loopback address at port 3000
with path /mcp, 64 MiB and seven days. Runner settings are `initialize_timeout` (30 seconds),
`call_timeout` (120 seconds) and `pass_environment` (empty); CLI equivalents are
`--initialize-timeout-seconds`, `--call-timeout-seconds` and repeated `--pass-environment NAME`.
The runner supports npm Packages over stdio, verifies their artifact SHA-256 before unpacking,
installs dependencies without scripts, and keeps a hash-keyed cache under the store. This
pins the artifact, not its dependency graph. Each run has a private empty working directory,
a process group, a lifetime guard and one MCP session with no Roots capability. Dropping the
shell or runner, or killing the engine, ends the group and its descendants. Commands at
relative paths use retained installed-package files; absolute commands run as given and bare
program names use the permitted PATH.

Default tests stay offline. Run the real pinned filesystem server tests, including CLI
registration, a hash mismatch before extraction, and the S1 HTTP subprocess test, with:

```sh
CONDUIT_TEST_NPM=1 cargo test --release
```

Credential directories and authority files are private,
and actual credentials never enter core inputs or output. The same-key CA file checks
nothing at the loopback HTTP address. The supported choices and remaining boundaries are in
[CONFORMANCE.md](CONFORMANCE.md); implementation details and tests are in
[the increment 3 log](../docs/proposals/first-build/README.md#settlements-register).

## What increment 2 covers

I2-01 implements deterministic first start: exactly five ready objects and root's two open
declared Conduits, retaining their built-in template provenance, ownership, placement and
machine labels. `conduit-core::step` takes immutable state, an input and a supplied context,
and returns state, promise-citing decisions, records, external intents, a semantic reply and
correlated final replies for dispatched request attempts.
Time, UUID allocations, revisions, generated suffixes and the local machine report enter as
values. No core code reads a file, clock, network or process. Allocation errors leave the
caller's state untouched. Restart uses retained state and makes no new identities.

The [portable cases](../conformance/README.md) run through `conduit-conformance`, the file
boundary. They include protected-template rejection, root's LIF-034 lifecycle refusals and
the four original loader settlements. Every decision cites known promise IDs, each record
is checked against `record.v1`, and exact object and record arrays detect omissions and extra
effects. The runner validates cases against their JSON Schema and checks the coverage index.
The loader adds a public compiler for a built-in Schema contract with resolved references and
rejects two initial YAML byte order marks under SUB-005, applying the admin's settlement 2-01.
A permitted byte order mark before a later YAML document remains accepted.

I2-02 adds [authored schema evaluation](crates/core/src/schemas.rs) and E05 tool publication.
Supplied exact package/project scopes keep each manifest's release pins and each Schema
contract's local root. Authored validation uses separate in-memory resources; publication
copies each reached contract once into the tool root and rewrites its references, preserving
siblings, local definitions and recursion. Both paths reuse the loader's validator, formats
and ECMA pattern engine. `patternProperties` is specialized to the finite set of instance keys,
with original schema positions retained for references and `additionalProperties` handled by
the validator. The published schema itself contains the original patterns.

The core exposes `evaluate-schema`, `check-authored` and `tool-definition` inputs. The first
fills supplied values (with an operation-input mode and an output mode that fills nothing).
The second is the semantic admission check for schemas after strict loading, shared by
archive admission and registration; it checks default validity and reserved root copy keys.
The third reads the operation and schemas held by an explicitly supplied target and open
Conduit. It handles control narrowing and omission, handles, descriptions and hints. These
vectors start with established snapshots and claim no package installation, registration or
opening. The runner checks every snapshot contract through the loader, uses the actual core
contracts, and compares authored and standalone verdicts for every E05 sample in both schema
directions. Standalone validation checks all references are local and resolve before compiling
with no package resolver. As of 2026-11-10, conflicting unconditional defaults are rejected
under SCH-009 when installed or submitted; equal defaults are one default. Finding
[2-03](../docs/proposals/first-build/README.md#2-03)
records the admin's settlement and its vectors; the owner can strike it.

These I2-02 cases do not exercise registration, access, invocation handling or the shell.
Unsupported inputs are embedding errors, not invented Conduit reasons. The
[increment's log](../docs/proposals/first-build/README.md#settlements-register) records the settled BOM case
and the verification corrections: registration checks the existing owning Pool's authority
before exposing protected-template diagnostics, and lifecycle records keep the Conduit's
target distinct from the named object. Authorized vectors start from an explicit snapshot
containing root's additional open Conduit to system; they claim no general registration or
opening implementation.

## What increment 1 covers

The loader reads Conduit's documents as the specification says, and rejects what it must
reject. Every rejection is a value, a `Diagnostic`, which names the promise it rests on, the
file, the document within the file and the place within the document, as a JSON Pointer and as
a line and a column, and nothing panics on any input. A file that is not YAML or JSON is
rejected for that alone; otherwise every fault of every document is reported, so that the same
documents in any order give the same diagnostics (SUB-001), and a fault is named alike in the
YAML and the JSON form of a document (SCH-013). Reading takes time in proportion to the size of
a file. What reads the file system is at the edge: `read_file`, `read_project` and
`Specification::read`. Everything else takes what was read, so the core can use it unchanged.

- **One document, strictly** (`yaml.rs`, `json.rs`, `number.rs`). YAML 1.2.2 with its core
  schema, narrowed as the [schema profile](../spec/schema-profile.md#yaml-features-not-accepted)
  says: a duplicate key, an anchor, an alias, a merge key, a tag and a key that is not a string
  are rejected (SCH-012), and so are `.inf`, `.nan`, `0x1F` and `0o17` (SCH-017) and numbers out
  of range (SCH-006, SCH-021), and a character that YAML 1.2.2 does not allow where it stands.
  A number with no fractional part is an integer however it is written (SCH-005); one with a
  fractional part is binary64 (SCH-020), and an integer where that value is whole, held to the
  range of SCH-006, so `9007199254740993.5` is rejected. JSON as RFC 8259 writes it, with the
  same narrowings, so that it means the same as YAML (SCH-013); a JSON file whose value is an
  array holds its items as documents, and any other holds its value as one (SCH-018). Files are
  UTF-8, with one leading byte order mark accepted and ignored in either syntax; a noncharacter may stand in a
  string, and a surrogate that is not paired may not, nor, in YAML, a pair written as two
  escapes. Values nest at most 128 levels deep, a limit that DL10 leaves to an engine, past
  which a document is rejected under SUB-011.
- **A project's directory** (`project.rs`). The manifest is the one file named `conduit.yaml`,
  `conduit.yml` or `conduit.json`, and it holds one document (MAN-001); more than one is
  rejected (MAN-002). The documents are the other files of the directory itself whose names
  end in `.yaml`, `.yml` or `.json` and do not begin with a dot, names and endings compared
  exactly; nothing below it is read, and no other file (MAN-011, SUB-010). A link is followed,
  and one to a directory is not entered; one that leads nowhere, or an entry that cannot be
  read, under such a name, is rejected under SUB-010 and never skipped. Files handed over
  without a directory are read by the same rules.
- **Validation** (`schemas.rs`, `formats.rs`). Each document is validated against `document.v1`
  and the manifest against `manifest.v1`, after the Schema contracts of `spec/schemas/` and
  `spec/core/` are bundled exactly as `bundle()` in `tools/check_schemas.py` bundles them. The
  validator is JSON Schema 2020-12 with the profile's formats asserted (SCH-001), `uuid` and
  `duration` checked by the grammars that the profile cites (SCH-004), and `pattern` matched as
  ECMA-262 with the `u` flag, in the edition that JSON Schema 2020-12 cites (SCH-016). The loader
  also rejects a document whose own schemas hold a pattern outside the interoperable tokens
  of JSON Schema 2020-12, core, section 6.4, or one that is not ECMA-262 in Unicode mode
  (`authored.rs`), as SCH-016 says. `Specification::check` gives that verdict beside the
  schemas', as reading a project does; `Specification::validate` applies the schemas alone. A
  rejection by a schema rests on SUB-005, and names the Schema contract, the instance's place
  and the failing keyword's location (`keywordLocation`, and `absoluteKeywordLocation` where
  known, to be carried in the SARIF result's `properties`).

The loader does not resolve references, instantiate templates or apply the rules of
`tools/check_schemas.py` that no schema can express: the core implements those in increment 2.

## The tests, and the done-when

The tests are in [crates/loader/tests](crates/loader/tests). `examples.rs` and `formats.rs` hold
increment 1's done-when; the others test the rest of what it covers.

| Test | What it shows |
| --- | --- |
| `tests/examples.rs` | Every directory under `spec/schemas/examples/valid/` is read as a project and accepted, and the file there whose name begins with a dot is not read; every file under `invalid/` is rejected by the step that `check_schemas.py` rejects it by, when read or by a schema, as the fixture `tests/expected/invalid-verdicts.txt` records, a rejection by a schema always under SUB-005; every document under `invalid-by-rule/` passes the schemas, and `Specification::check` rejects those named for SCH-016 and no other; the core package's own documents are valid; every example means the same written again as JSON (SCH-013); and the trees of examples are flat, so these tests read every file that the tool reads |
| `tests/reading.rs` | The syntax rules, case by case, in YAML and in JSON: the characters YAML allows; a number read as a whole binary64 value, held to the range of SCH-006; I-JSON's two rules, and a surrogate that is not paired; every fault of a file reported, in any order of its documents and of their members (SUB-001); each fault with the same rule and place in YAML and in JSON (SCH-013); a document marker that the parser would read as content; and a file of some megabytes read in linear time |
| `tests/pattern_rewrites.rs` | All eight historical patterns compared with their rewrites, on example strings and generated and boundary strings, using `regress` for both |
| `tests/formats.rs` | The grammars of `uuid`, with its edges of version and variant, and of `duration`; each format asserted; patterns matched as ECMA-262 where Python's `re` and Rust's `regex` read them otherwise; a pattern that is not ECMA-262 or uses tokens outside the interoperable subset; and the verdict of `Specification::check` on one |
| `tests/project.rs` | Two manifests; a manifest that is not one document; documents in a directory below, not read; a file with another ending, or whose name begins with a dot, ignored; a link followed, and one that leads nowhere rejected; the order of documents, within a file and across files, which means nothing (SUB-001) |
| `tests/rules.rs` | Every promise the loader cites is one that `spec/` defines, with an ID in the form of a diagnostic's `ruleId` |

## Choices

**The schemas are read from `spec/` when the loader runs**, not embedded when it is built. The
specification is the source of truth, and the first build's findings change it: read at run
time, the tests always run against the specification as it stands, and nothing is copied. The
names that [spec/names.yaml](../spec/names.yaml) defines, the core package's address among
them, are read from that file too, and never written here: `tools/set_names.py --check` scans
this directory, and a test writes its documents with the names it reads. As of 2026-11-10 the
shell CLI also reads `spec/` from this checkout; a standalone distribution carrying the core
package (NAM-020) remains work for later.

The crate choices below record the loader work of 2026-11-09; their upstream status is as
recorded then.

**YAML: [saphyr-parser](https://crates.io/crates/saphyr-parser) 0.3.3.** `serde_yaml` is no
longer maintained: its last release is `0.9.34+deprecated`. `serde_yml`, a fork of it, is
deprecated too. `yaml-rust2` and `saphyr` come from the same maintainers, both from `yaml-rust`
with fixes toward the YAML test suite, and `yaml-rust2` says that it will have only basic
maintenance and that new work goes into `saphyr`. The loader uses `saphyr`'s parser alone, which
gives every event with its anchor, its tag, the style of a scalar and its place, and builds the
values itself, so every narrowing of the profile is a decision written in `yaml.rs` and none is
a default of a crate. Where the parser does not do what YAML 1.2.2 says, the loader does,
in its own code, and none of it is left to chance:

- It does not check the characters of the stream, `c-printable`: it takes a C0 control
  character anywhere, and DEL, a C1 control character, U+FFFE and U+FFFF outside a quoted
  scalar. The loader rejects each as not YAML.
- It does not check the version of a `%YAML` directive, which YAML 1.2.2 has a reader reject
  when it is not 1.x. The loader does, for a directive named `YAML` exactly.
- It reads a line `---` into a block scalar at the root as content, where YAML 1.2.2 has it
  begin a document. The loader rejects such a text as not YAML rather than read it otherwise
  than YAML does; the fault is to be reported to the parser's maintainers.
- Its lookahead stops at 255 levels of flow collections, which the loader reports, under
  SUB-011, as the limit that DL10 leaves to an engine. The loader's own limit is 128 levels.

**JSON: [serde_json](https://crates.io/crates/serde_json)**, with its `raw_value` feature. It
checks the syntax. The values are built from the raw text of each, so that two members with one
key are found, which `serde_json` would otherwise keep one of silently, and so that a number is
read from the digits that write it.

**The validator: [jsonschema](https://crates.io/crates/jsonschema) 0.61.0**, without its default
features, so it fetches nothing. It asserts formats when asked, and checks `date-time` and
`date` by RFC 3339 and `uri` and `uri-reference` by RFC 3986. Three things it does otherwise
than the profile asks, the loader gives it. Its `uuid` is replaced by a check written from the
grammar of RFC 9562 that SCH-004 cites, and its `duration`, which takes the letters as capitals
only, by one written from the grammar of RFC 3339, appendix A, whose letters match either case,
as ABNF has it (finding 1-13). Its `pattern`, which translates a pattern into the syntax of
another engine as far as it can, is replaced by [regress](https://crates.io/crates/regress), an
engine of ECMA-262's regular expressions, in Unicode mode. `regress` follows the current
edition of ECMA-262. The loader tokenizes every pattern against the interoperable tokens of
JSON Schema 2020-12, core, section 6.4, plus escaped pattern punctuation, before compiling it.
The old patterns and their rewrites are compared with `regress`'s PikeVM backend in
`tests/pattern_rewrites.rs`, avoiding costly backtracking in the historical patterns. The crate cannot be given another engine for `patternProperties`; no
schema of the specification uses it, and the core matches it itself when it validates the
schemas that authors write, from increment 2, as the
[log of increment 1](../docs/proposals/first-build/README.md#settlements-register) records.

## Snapshot catalogs and project contexts (I2-03, first part)

[The package module](crates/core/src/packages.rs) owns an immutable catalog of explicit
preinstalled snapshots plus a separately supplied built-in package. Its `resolve-project`
input derives schema scopes and resolves selections through each manifest's exact pins,
without I/O. SemVer comparison ignores build metadata; digests must match exactly. The core
address cannot be supplied as a snapshot, claimed by a submitted project, or listed as a pin.
External references require exports; local selections can reach private documents/items.
Missing or mismatched pins return invalid diagnostics without any fetch or installation.
The reply retains package identities and implementation items, including the provenance of
programs supplied as relative paths. No first-start object or its provenance is changed.

`project_digest` uses SHA-256 of RFC 8785 JSON, sorting only the contract documents by kind
and name in UTF-8 order, with the original submitted manifest. Runtime path expansion and
fixture metadata never enter it. The canonicalizer is pinned, and an independent digest test
covers number serialization and UTF-16 map-member ordering. Resolving also checks manifest
bindings, selected required scalar configuration places and plain tool coverage.

[The runner boundary](crates/conformance/src/packages.rs) strictly loads ordinary files,
adds the actual built-in package, and supplies directory URLs as evidence. Retained archive
and Package artifact bytes are hashed only as fixture evidence. The
[S1 vector](../conformance/language/packages/s1/case.yaml) resolves the filesystem, history,
and Agent packages and S1's project, with real computed pins and no state changes or intents.
Other vectors exercise accepted and rejected names, pins, exports, path expansion and digest
invariance. This input performs context resolution only; registration, instance arguments,
object references and execution are described in the later sections, as is archive admission.

## Registration and resolving instances (I2-04)

[The registration module](crates/core/src/registration.rs) adds `register-project`, carrying
loaded documents, their manifest, installed snapshots, the separately supplied built-in
package, a request and allocation evidence. It checks root's established registration path
and the additional authority for an existing owning Pool before returning object diagnostics.
A new Pool in the submission requires management authority alone (ACC-033).

Registration plans templates and instances privately and publishes them together only after
all checks succeed (SUB-003). Traversal is deterministic by family, name and document value;
nested slots use their names. Evidence supplies object IDs, revisions, record IDs and DL15
suffix candidates. Generated names retry collisions, including collisions with named objects
later in the submission. The reply lists every new object and each nested instance's parent,
slot and declaration name. Rejections preserve all existing objects; existing-name clashes
report conflict, other defects invalid, after authorization (OBJ-023, OBJ-025, OPS-023).

Templates retain their schema environment, exact package identities and resolved implementation
items. Instances use that frozen context (NAM-009/010), assign whole argument/default values,
validate configuration, and resolve references only where reference schemas govern them.
References in configuration, standard fields and retained nested declarations hold kind/ID.
Ordinary maps with `kind` and `name` remain data. Inputs are removed from the stored instance;
nested declarations retain their filled arguments for a future replacement (INS-013).
Missing instance references reject; missing template references are reported as notes (OBJ-008).
An omitted Type schema is unconstrained (CTR-004). Applied requirement arguments are checked,
default-filled and resolved through the Requirement's own context (NAM-019, OBJ-009/021).
Instantiation checks retained package identities against the supplied installed catalog without
changing the frozen context (PKG-007). Supplied entry placement/runtime references are retained
by ID; entry lookup excludes retired Resources (POOL-014/016). Standard argument names are
rejected under `arguments` even if the template declares an input of that name (OBJ-013).

S1 registers nine templates, six named instances and four nested Conduits. Instances are
registered, Conduits unopened, and no setup, run, call or external intent is made (INS-016,
SUB-006). Placement vectors use the established local node and the engine's own runner;
later sections describe the access, invocation and setup transitions. The earlier
`submit` input remains the protected-first-start compatibility path.

## Archive installation (I2-03, second part)

[Archive admission](crates/core/src/archives.rs) consumes requested package identity, expected
SHA-256 and a fetching completion containing compressed bytes. It performs no I/O. The core
verifies the digest before decoding gzip/USTAR, checks member and link safety, and reads root
manifest/documents through the loader's pure interface using supplied built-in Schema contracts.
It rejects instances and mismatched manifest identity, then reuses the catalog resolver for
dependency, export, contract and implementation checks. Only success publishes an
`installed-archive` entry, retaining the virtual directory and regular-file modes in state.
Subsequent resolving and registration include these entries alongside supplied snapshots.

The profile's DL7 choice and [archive fixtures](../conformance/fixtures/install-archives/README.md)
document the exact dialect and reproduction command. The S1 vector compares all resolved
context fields before and after installing its three archives. Failed installs leave objects
and installed entries unchanged. Repeats and conflicting digests use the existing catalog,
even with an unavailable completion. The input represents archive admission; general
management request authorization, dispatch records and physical package storage belong to
the convention and shell inputs. As of 2026-11-10, public `install.v1` is implemented as
[Change a serving project](#change-a-serving-project) describes. No vector fetches a package
or runs a provider.

I2-05 adds the pure `access` and `list-tools` inputs and `access::tool_list_changes`.
Identity and whether authority can be read are supplied boundary facts. Each access reply
is a reason or a permit carrying the resolved consumer, caller, Conduit, target, operation,
default-filled arguments and filters. The access record is kept before the permit is returned;
no invocation is created or dispatched by this stage. The following invocation stage must use
it atomically with the same state. A later request always decides access again.

Tool publication reads registration's frozen contract contexts and resolved references, and
first-start objects use the supplied core contract environment. Each instance's current list
is private and comes only from its open Conduits. The list-change helper compares established
states and returns notifications addressed to consumers; the shell delivers them to subscribed
streams. Supplied serving invocations retain `id`, `target`, `caller`, `conduit`, `state`,
optional `serving` and carried `filters`; `retirement_of` identifies retirement work for
ACC-039. They are trusted state, never caller-supplied trace metadata. The invocation stage
produces that state itself. These observations exercise access without claiming lifecycle
transitions or provider execution.


I2-06 adds `invoke`, `complete`, `cancel`, `recover` and `expire-outputs` in the pure core's `requests`
module. Access and invocation dispatch are one transition. The invocation index in `State`
is independent of records and survives serialization. Recovery ends interrupted attempts
unknown and emits no dispatch intent.
Results and reasons use IFC-025's envelope, with the prefix read from `spec/names.yaml` at
the boundary. Selected contracts are retained for output validation; attempt numbers reject
stale completions. Succeeded plain content is retained with outputs until explicit DL10
expiry. Reconciliation uses current ACC-033 authority and records checked evidence.

Dispatch, parameter-header repair and cancellation return intents only; the shell must
persist state and records before carrying them out. Control reconciliation now uses REV-010 and current Pool/Fleet authority (ACC-033);
setup/open reconciliation remains unfinished. Pool and Fleet retirement cascades use
[cascade.rs](crates/core/src/cascade.rs), under LIF-021 and REV-014; records purge is described under I2-08 below. The
[request vectors](../conformance/README.md#request-endings-i2-06) describe their observations,
and the [finding log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md#i2-06-request-endings-work-order-015)
records the admin’s narrowing of header repair and the return-review regressions.

I2-07 adds the pure `lifecycle` module. `invoke` now carries out `set-up.v1`,
`open.v1`, `close.v1` and `register-replacement.v1` through the same access and
invocation index. Setup stores its parts with its invocation. Declared opens and Pool
entry setups are separate invocations, with the sending instance's authority and the
original caller; runner actions are intents, not requests through invented Conduits.
A failed required part stops later parts and leaves successful earlier parts in place.

`setup-report` supplies an outstanding inspection, run or connection completion. The
built-in runner uses `builtin-runner` intents; a selected runtime uses `provider-call`
intents. Run inspection precedes a new run; a running report asks for none and an unknown
report makes the part unavailable. Runs and remote connections occupy the same setup
position, and the hook waits for all of them. Run inputs contain configuration bindings
and an opaque identity materialization descriptor whose required names come from the
supplied core contract. Credential materialization, authentication, connections and processes
belong to the shell. Hook invocations have no Conduit and use a stable per-instance key.

Closing removes the Conduit immediately and records the consumer's lost reach without
changing its lifecycle or revision. Pending results still complete through their retained
invocation; later requests check access again. An explicit nested replacement reuses the
registration resolver with trusted frozen references and the referenced template's current
definition. Preinstalled catalog snapshots are retained in state for that validation;
actor-authored resolved references remain invalid. Replacement registers a new identity
and opens nothing. Full inspection is described under I2-08 below. General retirement and
lifecycle reconciliation were outside I2-07's setup/open/close slice; as of 2026-11-10,
[Change a serving project](#change-a-serving-project) describes the public retirement paths
added by order 025. Lifecycle reconciliation remains unfinished.

Order 016's return review adds reference dependency scheduling for Pool entries, propagation
of hook reasons and cancelled entry outcomes, and collection of every dispatched setup part
after retirement. Required parts prevented by retirement count as unsuccessful; no later
part starts. Running observations upsert complete `condition.v1` values and retain transition
times until status changes. Selected runtime calls reject retained relative-program
provenance before dispatching `run.v1`, and their records use the provider's own projection
without request route metadata. Replacement configuration accepts references frozen by
registration while actor-authored IDs remain rejected.

I2-08 completes the records of registration and provider completion, adds `inspect.v1`,
root-only `purge.v1`, revision-checked `replace.v1` and values-only `register.v1` through the
invocation boundary. Replacement uses registration's resolver with the new project's
contracts and manifest; it preserves the template ID and every old instance's held context.
An equal replacement preserves its revision. Expected IDs are checked after authority and
input validity, with both IDs in a conflict diagnostic. The E08 fixture explicitly models a
trusted broker's onward request; its approval logic is provider evidence, not an engine policy.

`export-records` is the profile's explicit record export boundary. It returns the exact ordered
CloudEvents, `gaps` and `complete`. Its optional `unreadable` IDs simulate a temporarily
unreadable range without deleting it. Purge keeps a permanent `object-change` certificate with
`action: records-purged`, the cutoff and the IDs and sources removed. Later purges preserve
certificates. Inspection includes the resolved declaration, packages, lifecycle and observed
status, conditions, the complete `unreconciled` list from invocation state, and all matching
records in source order. `recordGaps` and `recordsComplete` report omissions conservatively:
a known gap in the single record source is shown even if its object association was lost.
Neither inspection nor export initiates repair or provider work.

The runner validates every emitted and retained record against the actual `record.v1`, then
checks applicable invocation fields, provider subjects and access before dispatch. Inspection
records and unknown invocations are checked as exact lists, including after purge. Each `D19`
line contains a case and step, serialized UTF-8 JSON bytes of the record array and invocation
index separately, their signed growth since the preceding step, and each invocation's bytes,
keyed status and outcome. Record-gap metadata is reported separately. These are measurements
of this representation, including retained outputs and held contexts, not estimates of a
storage backend's cost. An empty array occupies two bytes. Purge leaves prior invocation
entries and retained results intact; there is no compaction or key expiry.

The return review validates successful built-in outputs against their core contracts and
checks supplied keys on outer access/ending records independently of invocation correlation.
Template inspection omits status, as LIF-012 gives a template no lifecycle state. Keyed
successful registration/replacement repeats return their retained result without reapplying
state-changing preconditions, after current authority checks. Registration results use
`object.v1`'s portable nested `instance`/`name` fields rather than internal storage fields.

## Integrated S1 and the shell boundary (I2-09)

The [continuous S1 vector](../conformance/conventions/s1/whole/case.yaml) now takes an empty
core through first start, resolution, registration and setup, then T1/T2 requests and T7's
close, independent B access and explicit restoration. Its 58 steps use scripted completions;
no real Agent or filesystem server runs. All requests it dispatches end with validated
records. The [manifest](../conformance/manifest.yaml) inventories every study-section-2
promise with vector IDs or an explicit reason for missing coverage, validated by the runner.

[CONFORMANCE.md](CONFORMANCE.md) documents implemented and pending DL choices. The
[increment 3 handoff](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md) lists the inputs,
intents, completions, consumer tool contract and shell work items as increment 2 left them.
At that handoff, archive admission lacked the complete `install.v1` request workflow, and
public retirement/stop-run paths still needed core work. Orders 025 and 027 supplied those
paths on 2026-11-10; [Change a serving project](#change-a-serving-project) gives the commands,
and CONFORMANCE.md states what remains unfinished. The handoff's limits and the increment's
done-when are recorded in
[the log](../conformance/README.md#evidence-of-vector-strength-so-far).

## Consumer HTTP adapter (I3-03)

Select the port at initialization; the token audience is then fixed for that store:

```sh
cargo run --release -p conduit-shell -- init --store /tmp/conduit-http --url http://127.0.0.1:3000/mcp
cargo run --release -p conduit-shell -- serve --store /tmp/conduit-http
```

In another terminal, the store owner can use the root Interface through MCP:

```sh
cargo run --release -p conduit-shell -- tools --store /tmp/conduit-http
cargo run --release -p conduit-shell -- call --store /tmp/conduit-http management.inspect.v1 '{"object":{"kind":"Interface","name":"root"}}'
```

`tools --as INSTANCE` uses that materialized instance's private helper token. Commands never
print token files or recover the running store. SIGINT or SIGTERM stops the server.
The adapter verifies Bearer tokens on every HTTP request, enforces its own Origin, presents
exactly the core's private tools and forwards current-authority invokes. Pending calls wait
for committed completion; cancellation reaches the core. Management operations still missing
from the core answer internal error naming the unfinished operation and commit nothing.

`serve` starts the built-in runner and attempts outstanding setup before listening. Since
order 027 on 2026-11-10, a failed setup is reported as the instance's state and leaves the
server listening, so the author can repair it through `submit` and `call`.
It inspects ready instances with runnable implementations too, so runs stopped after registration
are started again through explicit setup. On shutdown it ends every run. The default scripted
HTTP tests cover A/B tools and calls, durable records, cancellation and T7's close and explicit
replacement followed by setup. The opt-in real S1 test starts `serve` as a process, connects
with A/B run-directory tokens, exercises the filesystem and T7 through `conduit call`, checks
private notifications and verifies both filesystem process groups disappear on SIGTERM.
Base S1 Agents have no executable; the test materializes their identities through the same
run-contract API as the runner, without executing Agents or models.
See [CONFORMANCE.md](CONFORMANCE.md#consumer-server-order-022) for these boundaries.

## Runnable Agent packages (work order 023)

[Live S1](examples/s1/README.md) replaces the registration-only Agent fixtures with the
standalone [Claude Code](../packages/agent-claude-code/README.md) and
[Codex](../packages/agent-codex/README.md) packages. Each crate builds and packs on its own.
`conduit install --store DIR ARCHIVE#SHA256` admits each archive through the serving engine;
`conduit submit --store DIR PROJECT_DIR` then registers and sets up the project. The CLI checks each
compressed archive's digest and leaves package admission to the core; it does not rewrite
project pins. Pins in a live working copy must match the generated host archives. The checked-in
example instead pins portable metadata-only archives for the registration vector.

Setup starts wrappers only. `conduit call --store DIR turn-a.turn.v1 '{"message":"..."}'`
and the corresponding turn-b tool are explicit model-work requests through authorizing
Conduits. No real client was run by order 023. Its default release tests build both standalone
packages, exercise unmodified archives through the CLI, then install test archives with the
explicit scripted-client argument and run S1's two Agents through the actual runner/HTTP
server. A scripted filesystem replaces npm in this test; each Agent makes an engine-recorded
request with its own run identity. This needs only crates and loopback networking.

The admin subsequently ran the real clients on 2026-11-10; the
[closeout results](../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md)
record T1, T2 and T7, project-instruction exclusion, refused outside-file reads, request
records and shutdown observations. Access remains mediated, with both Agents and the engine
running as one operating-system user. Codex still reads its home's `AGENTS.md` (finding 3-07).
Account-service refusal, incoming peer-message refusal, reach inputs set true, Codex's own
tool list and independent proof of conversation continuity were not shown in that closeout.
On 2026-11-10, the admin settled finding 4-06: Codex's six `collaboration.*` tools remain,
but a spawned sub-agent in its trial had no shell, could not read outside its sandbox and
was refused a write by the read-only sandbox. Engine requests carry the parent Agent's
credential. This permits parallel model work within that Agent's access; see
[CONFORMANCE.md](CONFORMANCE.md#i4-02-startup-repair-retirement-parts-and-inspection).
The other unshown checks carry forward.

## Change a serving project

As of 2026-11-10, orders 025 to 028 supply the commands below, help for every command and
nonzero exit status for any unsuccessful result. Registration preinstalls no S1 package;
install every dependency explicitly, as in the example above. `submit` requires absolute
fetch and program values: a request carrying documents has no submitted directory against
which to resolve them (SUB-010, finding 4-03, ruling 52).

These commands are requests of `conduit serve`, as the root Interface. Keep it running in
another terminal. `register` remains the command for first registration; `submit` adds a
project to the serving store and explicitly opens its new root Conduits and sets up its new
instances. Each request prints one JSON result line, including revision inspections.
A failed startup setup leaves `serve` listening so a missing Conduit can be submitted and the
instance set up through it with `call`.

`conduit inspect --store DIR` works while serving as root MCP inspection requests, and locally
when the store is not held. Its instance rows include a CLI-only `consumedConduits` list beside
`result`: declared and standalone Conduits with name, target and open boolean, projected from
the authorized `inspect.v1` result's `inventory.consumes` (LIF-013). The inventory also returns
incoming Conduits, opening provenance and visible Conduit tombstones, with scope for Interfaces
and Agents and administrators for Pools. Templates and Conduits have no inventory. Enumeration
and route selection use a read-only snapshot, and each request rechecks authority; inspection
across multiple objects is not atomic with concurrent changes.

```sh
engine/target/release/conduit install --store /tmp/conduit-live-store \
  conformance/fixtures/s1/archives/filesystem-0.2.0.tar.gz#cbb831f07d2b251c84fcd768a03353203a8a1b2215e862881fef47f866d84436
engine/target/release/conduit submit --store /tmp/conduit-live-store /absolute/new-project
engine/target/release/conduit replace --store /tmp/conduit-live-store \
  /absolute/edited-project Conduit/directory-read
engine/target/release/conduit retire --store /tmp/conduit-live-store Resource/old-directory
engine/target/release/conduit unregister --store /tmp/conduit-live-store Resource/old-template
```

`install` reads the archive's manifest for its address/version, verifies the supplied digest,
and requests public `install.v1` with an absolute file URL. Filesystem 0.2.0 runs npm server
2.2.0 and still exposes only its three contracted operations. Its
[fixture README](../conformance/fixtures/s1/README.md) gives the measured pins and rebuild
command. Existing instances keep the versions they resolved; history keeps 0.1.0.

`replace` reads the selected template from the directory and submits it with that project's
manifest/contracts at its inspected revision. To apply a changed declared Conduit to an
existing consumer, explicitly close its old nested Conduit, request `register-replacement.v1`
and open the new Conduit (or set up the consumer), using `call`. The local tool names stay the
same. Other holders keep their old definitions until their own replacement.

`retire` closes target Conduits at once and stops the run; successful shutdown removes the
instance automatically. `unregister` removes templates and retired instances whose shutdown
is still unconfirmed, at their current revision. It reports conflict for a live instance.
Both preserve records. A missing serving engine makes every command above exit nonzero.
