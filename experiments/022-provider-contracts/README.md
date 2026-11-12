# 022 — provider-contract vectors

A63: the harness and the suites that show each provider passing its contract, under ruling 40.
Built on 2026-11-11 by work order 123: the harness, with no suite yet. This standalone Rust
crate depends on no repository crate and belongs to no workspace. It does not use or change
the core conformance runner, its case form or its manifest.

Before running, the measure is: each selected case is passed only if launch, discovery and all
assertions succeed; failed at its first differing assertion or infrastructure failure; or
untested with a stated reason when a required feature is absent or the selected item is a
registry package or remote server outside H1. Totals count cases, including invalid cases.
Passing these synthetic fixtures establishes the harness's behavior, not any provider's contract.
A63 is done only with both implementations of every suite and S10's provider swaps.

Run from this directory, using Rust 1.97.1:

```sh
cargo run --release --bin provider-contracts -- suites/runtime/index.yaml descriptor.yaml lab report.json report.md
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
```

The binary takes exactly five positional arguments: suite index, descriptor, lab directory,
JSON report and Markdown report. It exits 0 when there are no failures (including an empty suite
or untested cases), 1 for case failures, and 2 for invalid run inputs or report-write failures.
Always inspect totals; an empty suite or an entirely untested run establishes no contract claim.
Report directories must already exist. Inputs and lab paths are local; the harness fetches no
archive or backing tool. The two report paths are explicit output destinations, outside any
provider output. Keep the lab private and use synthetic data only.

See [the case and descriptor form](docs/provider-yaml.md). The empty suite indexes are
[runtime](suites/runtime/index.yaml), [machines](suites/machines/index.yaml),
[package source](suites/package-source/index.yaml) and [secrets](suites/secrets/index.yaml).
The output schema copies live in [src/output-schemas.json](src/output-schemas.json); tests
compare them and their reachable definitions with the specification. Assigned names live only
in [src/names.rs](src/names.rs), also checked against the specification.

H1 starts a command item over stdio, with a ten-second initialization/discovery/call/version
limit and pauses of at most sixty seconds. Each provider process group is killed and reaped on
restart and after each case. A parameter-header error (-32020) refreshes tools and retries once
only for read-only operations (IFC-046). A protocol-revision rejection (-32022) is the exception
that permits even an effectful operation to repeat once: the harness uses the newest shared
revision other than the rejected one, from `data.supported` or, when absent, `server/discover`.
Both errors share one resend limit. With no shared alternative, or after a rejected retry, the
error is read under OPS-007 as unknown. Package files are re-extracted
for each invocation of the harness; state persists beneath the lab using the descriptor digest
and instance ID. A changed descriptor gets a new state directory. Nothing removes that state
automatically. Concurrent harness invocations must use separate labs.

Archives are tar or gzip-compressed tar, rooted at `conduit.yaml`. H1 accepts contained regular
files and directories and rejects links and special entries. A relative command resolves within
the extracted archive; an absolute command or bare executable follows the engine runner's rule.
A missing command or failed launch fails the case. Registry package items are untested. The
selected Type is recorded by the descriptor digest; H1 is not a package loader or Type checker.

The harness observes incoming MCP bytes (including malformed replies and startup), decoded
answers and stderr in memory, bounded at 16 MiB per run. Observation overflow fails the case.
No raw provider reply, stderr, command output, capture, configuration, identity, parameter value
or absolute path is included in reports. Failure details name only step and assertion positions
and fixed diagnostic categories. The marker assertion checks all observed runs in the case,
including later steps and output drained during shutdown. H1 makes no claim about files or
external logging systems that a backing tool does not expose on these streams; it reads none
of the provider's private state directory. H2 adds trust fixtures, run certificates, a gate and
fault controls. SV1 will supply the safety-case field; H1 invents none.

## Results

Validation on 2026-11-11 after the admin's verifier round: `cargo test --release` passed
**41 tests, 0 failed**;
`cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed. The fixtures use
five-second operation limits; the executable defaults to ten seconds. Cargo commands ran with
`RUSTC_WRAPPER=` because the configured compilation cache could not write inside the sandbox.

The verifier regressions check export tool names separately from qualified metadata,
extension declarations at MCP revision `2026-07-28`, incomplete replies without automatic
continuation (including `run.v1`), extra fields on restart and marker assertions, rejection
of dot-component instance IDs and distinct descriptor state directories, and redaction using
every case's markers and fixture values in both reports.

Work order 125 validation on 2026-11-11: `cargo test --release` passed **51 tests, 0 failed**;
`cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed. Ten added regression
tests cover literal identity/secrets arguments and secret equality assertions in both reports,
placeholder names remaining visible, empty label keys in configuration/parameters/fixtures
without erasing `machine` case IDs, and protocol-revision fallback. The fallback fixtures check
the retry's revision on the wire for legacy and modern sessions, including effectful calls;
they also cover discovery fallback, no shared alternative, rejected retries and both orders
of revision/header rejection without a third send. Before the fixes, seven of the ten tests
failed against order 123's implementation; the other three preserve existing limits.
Documentation and roadmap checks and all three Bash scans passed, with no scan matches.
`target/` remains ignored and unstaged.

Work order 125's follow-up verifier round on 2026-11-11 passed **53 tests, 0 failed**,
plus formatting and Clippy. Both added regressions failed before their fixes and passed after:
`regression_125_verifier_revision_retry_rejects_malformed_content` checks that malformed MCP
content gives unknown on both the first send and a revision retry, despite valid success
fields; `regression_125_verifier_mixed_supported_list_sends_no_retry` checks that a mixed-type
supported-revision list sends no retry. The retry now accepts exactly the response variants
that `call_tool_once` accepts, and revision selection validates every advertised list entry.

Repository documentation, roadmap, assigned-name and whitespace checks passed. The private-path
checker reported two existing scratch-directory examples in work orders 122 and 123, outside
this order's scope; they remain unchanged. All three required Bash scans of this new directory
produced no matches, and `target/` remains ignored and unstaged. A grep exit status of 1 denotes
no matches in these scans.

No real provider has been tested and there are no contract cases yet. The Rust fixture source in
[tests/fixtures/provider.rs](tests/fixtures/provider.rs) is built by Cargo; the integration tests
pack it into temporary per-contract package archives. No built executable or machine path is
committed.
