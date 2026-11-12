# provider.yaml

This is the provider harness's case form, separate from the core conformance form. YAML is read
with strict typed deserialization: unknown fields, steps, assertions and duplicate mapping keys
are errors. No shell commands appear in cases. JSON is also accepted as YAML. Case filenames
are selected explicitly by a suite index; no globbing or recursive discovery occurs.

## Suite index

Each contract has an `index.yaml` with exactly `contract` and `cases`. `cases` is an ordered list
of relative paths to case files. Paths cannot traverse or resolve outside the suite directory.
Duplicate case IDs fail. Every case's contract must equal the index's contract. The indexes
shipped with H1 contain no cases.

```yaml
contract: runtime-provider.v1
cases: []
```

Contract names are `runtime-provider.v1`, `machine-provider.v1`, `package-source.v1` and
`secret-provider.v1`, the core capabilities. Operation names in cases are their unqualified
export names, also used as MCP tool names; only the operation metadata uses a qualified core
reference. Runtime cases
may call `run.v1`, `stop.v1`, `inspect-run.v1`; machine cases `list-machines.v1`, `place.v1`;
package-source cases `fetch-package.v1`; secrets cases `get-secret.v1`.

## A case

Every field shown below is required except `markers`; an empty list or map is explicit.
`id` is stable, 1–100 ASCII letters, digits, hyphens, underscores or dots. `title` is nonempty;
`promises` and `steps` are nonempty. A case with an absent tool feature is untested. A safety
case will carry the field that SV1 names once it merges; no such field is accepted yet.

A complete illustrative case (this is documentation, not a suite case):

```yaml
id: example-inspect
title: Inspect the selected run
contract: runtime-provider.v1
promises: [PRV-005, IFC-009]
features: []
fixtures:
  machine: {hostname: {$param: hostname}, labels: {$param: labels}}
markers: []
steps:
  - step: call
    operation: inspect-run.v1
    arguments:
      instance: {$param: subject}
      machine: {$fixture: machine}
    meta: {}
    capture: {observed: /structuredContent/state}
    assertions:
      - {assert: outcome, value: succeeded}
      - {assert: equal, pointer: /structuredContent/state, value: running}
      - {assert: pattern, pointer: '/_meta/{meta:outcome}', pattern: '^succeeded$'}
      - {assert: no_marker_anywhere}
  - {step: pause, milliseconds: 10}
  - {step: restart}
```

A call requires `operation`, an object `arguments`, and nonempty `assertions`. `meta` and
`capture` default to empty maps. A pause has exactly `step` and `milliseconds` (0–60000).
A restart has exactly `step`. It terminates the current run, initializes and discovers a new
one, keeping the state directory. A call after no launch starts the provider first; any launch
or discovery failure fails the case, even if a call would expect `unknown`.

The runner sets the provider's `instance`, `config`, `revision` from the descriptor and the
qualified `operation`. Cases never write the assigned metadata prefix: `meta` uses the bare
keys `caller` and `invocation` for provider calls. `place.v1` and `fetch-package.v1` require
`caller` and prohibit `invocation`. Autonomous list/inspect calls can omit both. No provider call
may carry a Conduit, filters or an idempotency key (PRV-005).

A secrets case represents an engine read under SEC-010, not a provider call and not an actor's
request. Its `meta` requires all four keys `conduit`, `caller`, `invocation`, `filters`; arguments
are `{}` for `get-secret.v1`. It receives the descriptor's secret Resource configuration through
metadata, as IFC-006 requires. H1 checks the server over stdio; it does not exercise authorization
by an engine. Every call declares the Conduit extension at `v0` in MCP client capabilities.
Every launch performs initialization and `server/discover`, which must advertise that revision.

## Substitutions, captures and assertions

A placeholder is an entire object with one key and a string name. It preserves the referenced
value's JSON type:

| Placeholder | Meaning |
| --- | --- |
| `{$param: name}` | Descriptor parameter `name` |
| `{$fixture: name}` | This case's named fixture, expanded recursively |
| `{$capture: name}` | An earlier call's captured answer value, preserved literally |
| `{$lab: name}` | Absolute runtime path of the descriptor's named relative lab path |

Unknown names, unknown dollar keys, mixed placeholder/object fields and recursion beyond 32
levels fail. Regular objects and arrays expand recursively. No string interpolation of
parameters occurs. The only string placeholder is `{meta:key}`, expanded to the current assigned
prefix and key. It is useful in answer JSON pointers, both assertions and captures; expansion escapes the assigned key
as one RFC 6901 pointer segment. JSON
Pointers use RFC 6901 escaping (`~0`, `~1`) and start with `/`, or are empty for the whole answer.
Captures are a map of name to pointer, evaluated after assertions; they become available to later
steps, and a missing pointer fails. Later captures may replace an earlier name.

A call asserts only these forms. Unknown fields or assertion names fail:

| Assertion | Fields and meaning |
| --- | --- |
| `outcome` | `value`: `succeeded`, `failed` or `unknown`, after reading and validating the answer |
| `reason` | `value`: `refused` or `unavailable`, only the IFC-026 form for a filter actually sent |
| `equal` | `pointer`, `value`: exact JSON equality; the expected value supports placeholders |
| `pattern` | `pointer`, `pattern`: the selected string matches a Rust regular expression; use anchors for a whole-string match |
| `no_marker_anywhere` | No additional fields; none of `markers` occurs in any observed answer, MCP stream or stderr, across the whole case |

An invalid regular expression or pointer fails. A missing pointer differs even from an expected
null. An empty marker is invalid. The marker assertion is checked again after all runs stop,
so it also covers output following the assertion. Markers must be synthetic. Observed records
here are the harness's protocol observations; no raw record is exported. External records and
logs that are not exposed on the observed streams need later fixture support.

`outcome` and `reason` are normalized decisions, separate from pointer assertions on the actual
answer. IFC-009 contradictions, absent or invalid keys, disagreement with `isError`, a missing or
malformed or incomplete result, and invalid required successful output all give `unknown`.
An `input_required` reply is observed once, without an automatic continuation or resend.
A failed or unknown
result needs no outputs. Only successful outputs are checked, against exact copies of the core
output schemas and reachable definitions. A protocol -32021 with identified required
capabilities gives `failed` (IFC-008/047); an unqualified error gives `unknown`. A -32020 on
a read-only operation refreshes the tool list and retries once (IFC-046); run and stop do not
retry after that error. A revision rejection (-32022) retries once, including run and stop, at
the newest revision both sides support other than the one rejected. Server revisions come from
`data.supported`, or discovery's `supportedVersions` when that field is absent; client revisions
are `rmcp`'s known versions. A present malformed list supplies no shared revision. Either error
uses the call's sole resend allowance. No shared alternative or a rejected retry gives unknown
under OPS-007. Reasons require
`isError: true`, no outcome, and a restriction equal to an actually sent filter. No filter is
sent on provider calls, so they cannot validly return this form. Assertions run in order and
reports name the first differing assertion without showing either value.

## Implementation descriptor

A separate YAML file selects one implementation. All fields below are required; unknown fields
fail the run. Nothing in this file is a secret. Paths may be local in the descriptor but never
appear in the report.

```yaml
archive: artifacts/provider.tar.gz#0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
item: provider
type: example.org/providers#runtime.v1
instance: 00000000-0000-4000-8000-000000000001
revision: revision-1
config: {endpoint: {$param: endpoint}}
parameters: {endpoint: example.invalid}
features: []
tool_version_command: [backing-tool-version-json]
pass_environment: []
lab_paths: {input: fixtures/input, record: observations/record.jsonl}
```

`archive` is a local path relative to the descriptor's directory (or absolute), followed by `#`
and the lowercase 64-digit SHA-256 of its bytes. Tar and gzip tar are accepted. `item` selects
`implementations[item]` from the archive-root `conduit.yaml`; the item's command and environment
bindings are read there, never copied into the descriptor. `type` is a qualified Type reference,
`instance` supplies the provider/secret Resource ID (using the case ID alphabet, except `.` and
`..`, which are rejected as path components), `revision` its opaque nonempty string revision
(REV-001), and `config` its
object configuration. H1 does not resolve this Type or validate a package as the engine loader
does. Descriptor content is pinned by its SHA-256 in the report.

`parameters` contains implementation-specific expected data (for example a fixture machine's
hostname and labels); parameter names use the case ID alphabet. `lab_paths` maps names to paths
of already provisioned fixtures beneath the runtime lab. The harness does not create fixture
content. Those paths must consist only of normal relative components, with no symlink escape.
`config` may use parameter and lab placeholders, but not case fixtures or captures.

`tool_version_command` is an argument vector, with no shell expansion. It must print one JSON
object with exactly `name` (the ID alphabet) and `version` (a numeric dotted version, optionally
with a prerelease/build suffix). Use a small wrapper if the backing tool has a different version
format. The command runs in the lab with the same ambient environment allowlist as the provider.
Its stderr and any other output are discarded. A failure or malformed output fails selected
cases. Name/version are the only command output fields eligible for the report; values containing
a known sensitive value are redacted before either report is rendered. The full suite supplies
that set: descriptor parameter/configuration values, instance ID, lab paths, local user/host
names, and every case's markers and fixture values, including invalid or untested cases.
Literal strings in call arguments `identity` and `secrets`, and in `equal` assertions for
`get-secret.v1`, are also collected; placeholder objects supply no literal values. Object keys
under `labels` in descriptor configuration, parameters and fixtures are collected too, except
keys beginning with `$`. Other object keys are not sensitive values.

`pass_environment` contains names only. PRV-023's six ambient variables, where set and not
beginning with `()`, plus this explicit list are inherited; all other ambient variables are
removed. Item bindings select scalar configuration by JSON Pointer: strings as-is, numbers in RFC 8785 form and
booleans as strings, null omitted. Objects, absent pointers and invalid pointers fail. Assigned
identity and state-directory variable names cannot be bound. H1 creates no identity fixtures;
H2 owns them. A private state directory is supplied for the same descriptor across relaunches.

## Reports

JSON includes repository revision (or `unavailable` outside a repository), contract, index digest,
every case digest, actual archive digest, descriptor digest, parameter names, tool name/version,
case verdicts/details and totals. The index digest pins case ordering and paths; individual case
digests pin case contents. The revision is the current Git HEAD and does not claim a clean tree.
The short Markdown table carries the same provenance and digests with case verdicts, fixed
failure categories and totals. Neither format includes input paths, titles, provider data or assertion values.
Case IDs that overlap that complete sensitive-value set become `case-N`. Report string fields
are checked against that set, so a matching generated digest or fixed diagnostic is also
redacted. Local locations use the documented `<lab>` abstraction; implementation data
is represented by descriptor parameter names. An invalid case gets `invalid-case-N` if its ID
cannot be safely used. Invalid run-level inputs give a fixed CLI error and no report.
