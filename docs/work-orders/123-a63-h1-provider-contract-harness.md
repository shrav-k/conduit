# Work order 123 (A63-H1): the harness for provider-contract vectors

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty: the cases this order writes are the harness's own
fixtures, not conformance vectors. This order has network access, to fetch crates and for tests
that bind a loopback port; use it for nothing else. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count. Run one cargo command at a time. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a token, a
private key or a secret value, even one that a test made.

## What and why

A63 is done when each provider passes the vectors of its contract and S10 passes with every
provider swapped (A63's row in [docs/AUDIT.md](../AUDIT.md); ruling 40). Today's conformance
runner drives only the core; nothing can show a provider package passing its contract. The plan,
[PROVIDERS.md](../implementation/PROVIDERS.md), puts that harness first: A63-H1, a crate of its own
in `experiments/022-provider-contracts/`, outside `engine/`, owned by the providers lane, with the
contract suites beside it in their own form. Read its paragraphs "A63-H1", "A63-H2a / H2b" and
"A63-VR / VM / VP / VS" before you start. This order is H1: the launcher, the MCP driver, the case
form and runner, and the reports. A63-H2 later adds trust roots, run certificates, the fixture gate
and faults; the four suites come in their own orders.

Decided by the admin (2026-11-11, recorded in PROVIDERS.md): the harness is its own crate outside
`engine/`; the suites use their own form, `provider.yaml`, and share nothing with the conformance
runner's case form, so no seam with the runner or the manifest is needed, and neither changes.

Read first:

- the four provider capabilities in `spec/core/capabilities.yaml`, and their operations in
  `spec/core/operations.yaml`: `run.v1`, `stop.v1` and `inspect-run.v1` (from line 864);
  `list-machines.v1` and `place.v1`; `fetch-package.v1`; and `get-secret.v1`;
- PRV-002, PRV-005, PRV-017, PRV-023 and PRV-029 in `spec/promises/providers.yaml`; SEC-005 to
  SEC-007 and SEC-010 in `spec/promises/secrets.yaml`; IFC-006, IFC-008, IFC-009, IFC-026, IFC-037
  and IFC-047 in `spec/interface/implementation.yaml`; OPS-007 in
  `spec/conventions/operations.yaml`; MAN-009 and MAN-010 in `spec/promises/manifest.yaml`, for an
  implementation item's `command`, `package`, bindings and environment;
- `packages/agent-codex/`: a package's `conduit.yaml` (its `implementations` and items) and its
  crate form. Its `tests/fixtures/client.rs` is a scripted stand-in for a native client, not an
  MCP client; its `tests/wrapper.rs` drives an MCP server with an `rmcp` client. The engine's
  shell pins `rmcp` at `=3.9.0` (`engine/crates/shell/Cargo.toml`), and the engine's runner
  launches items (`engine/crates/shell/src/runner.rs`); read both, change neither;
- [conformance/README.md](../../conformance/README.md), to see the core's case form, which this
  harness does not use, and to keep the two apart;
- [the roadmap's rules for an experiment](../../ROADMAP.md#proposing-an-experiment) and the
  [experiments index](../../experiments/README.md).

## The decisions it applies

1. **Where it lives.** `experiments/022-provider-contracts/`, new: a crate (name it
   `provider-contracts`) with its own `Cargo.toml`, committed `Cargo.lock`, `rust-toolchain.toml`
   at channel 1.97.1 and a `.gitignore` for `target/`; a README; `docs/provider-yaml.md`, the case
   form; `suites/`, with one directory per contract (`runtime/`, `machines/`, `package-source/`,
   `secrets/`), each holding only an index file that lists no case yet, so that git keeps it; and
   `tests/`. It is in no workspace and depends on no crate of this repository.
2. **The launcher.** It starts a provider package's implementation item as the engine's own
   runner starts a provider's own run (PRV-002; `engine/crates/shell/src/runner.rs`, near lines
   556 to 626, which you may read but not change).
   - A package's command is relative to its archive (`bin/...` in agent-codex's `conduit.yaml`),
     so the descriptor names the package by `ARCHIVE#SHA256`: the launcher checks the digest,
     extracts the archive under `<lab>`, and the report records the digest. A `package` item
     reached through a registry (npm) is out of scope: report it untested.
   - Its environment is as PRV-023 gives a run on the engine's own runner: the variables PRV-023
     names, where they are set, plus the descriptor's list of names to pass, plus a private
     `CONDUIT_STATE_DIR` that persists across relaunches of the same descriptor (PRV-029), plus
     the item's own bindings filled from the descriptor's configuration.
   - A launch that fails ends the case failed with its reason, never as a pass.
3. **The driver.** Over the run's stdio, with an `rmcp` 3.9.0 client (agent-codex's
   `tests/wrapper.rs`, near lines 286 to 341, drives an MCP server with one):
   - initialize, read `server/discover`, and go no further unless Conduit's extension is listed
     at revision `v0` (IFC-037). The engine's own server shows the wire form:
     `engine/crates/shell/src/server.rs` near lines 271 and 329, and
     `engine/crates/shell/src/server_tests.rs` near line 411 for a call that declares it;
   - each provider call declares the extension and carries the keys PRV-005 names: `instance`,
     `config`, `revision` and `operation`, which is the operation's qualified reference (IFC-006),
     with `invocation` where a case gives one and `caller` where a case gives one (`place.v1` and
     `fetch-package.v1` carry `caller` and no `invocation`). `engine/crates/core/src/requests.rs`,
     near lines 581 to 599, shows the `_meta` keys;
   - `get-secret.v1` is not a provider call: a secrets case calls it as the engine reads a secret
     (SEC-010), with the keys IFC-006 gives a request (`conduit`, `caller`, `invocation`,
     `filters`, as a case supplies them), and the case form says so;
   - the names `spec/names.yaml` assigns (the `_meta` prefix, the core package's address, the
     identity and state-directory variables) are written once, in one module, with a test that
     reads `spec/names.yaml` and fails when they differ: `tools/set_names.py` does not cover
     `experiments/`. A case never writes the prefix: it names a key by a placeholder that the
     runner expands.
4. **Outcomes and outputs.** The runner reads each answer as the engine does (IFC-009, IFC-026,
   OPS-007): `succeeded`, `failed` or `unknown` from the `outcome` key, a `reason` where IFC-026
   gives one in its place, or the protocol error -32021, and anything else as OPS-007 reads it.
   It validates an output against the operation's output schema only where the outcome is
   `succeeded`. The output schemas are copied from `spec/core/operations.yaml` with every
   definition they reach, and a test fails when a copy differs.
5. **The case form, `provider.yaml`.** Document it in `docs/provider-yaml.md` and validate it
   strictly; an unknown key, step or assertion fails the case.
   - A case has a stable `id`, a `title`, the `contract` (one of the four capabilities), the
     `promises` it tests, the tool features it needs, named fixtures and ordered steps. A safety
     case will carry the field that SV1 names once SV1 merges; this order invents no field for it,
     and the document says so.
   - A step is a provider call with its arguments, a pause, or a restart of the provider (which
     keeps its `CONDUIT_STATE_DIR`). A step may capture a value from an answer by a JSON pointer
     for later steps.
   - Arguments and expected values may name a descriptor's parameter or a fixture, such as a lab
     machine's hostname and labels, which the runner substitutes, so that expected results are
     written once per contract, never per implementation.
   - A step asserts exactly: the outcome or reason of decision 4; listed JSON pointers equal to
     given values or matching a stated pattern; and "no marker anywhere": that none of a case's
     synthetic secret markers appears in any answer, record or log line.
6. **The descriptor.** A separate file per implementation under test names the package archive
   and item, its Type and configuration, its parameters, the tool features it has, the command
   that prints the backing tool's name and version, the names to pass, and `<lab>` paths for
   fixtures. Nothing in a descriptor is a secret.
7. **The runner and the report.** A binary that runs one suite against one descriptor and writes a
   report, as JSON and as a short Markdown table: the repository revision, the suite and its
   cases' digests, the package archive's digest, the tool and its version, each case's verdict
   (passed; failed, with the first differing assertion; or untested, with why) and totals. A case
   whose tool feature the descriptor lacks is untested. A report holds no secret, token, key or
   identity value, and no absolute local path, hostname, user name or lab label: write `<lab>`
   and the descriptor's parameter names in their place.
8. **Its own tests.** Scripted provider packages in `tests/fixtures/`, Rust binaries the tests
   build and pack into archives at test time, so that no absolute path is committed. Each is a
   small MCP server that answers one contract's operation well or badly. Cover: a launch that
   fails; an archive whose digest differs; discovery without the extension; a malformed reply; a
   reply that breaks the output schema; each outcome of decision 4; an exact assertion passing and
   failing; a capture used by a later step; a substituted parameter; a restart step that keeps the
   state directory; an unknown key in a case; an untested case for a missing tool feature; and
   "no marker anywhere" failing on a fixture that echoes its synthetic secret.
9. **The index and the roadmap.** Leave granted by the admin, limited to these two rows:
   - in `experiments/README.md`, in numeric order: "022 — provider-contract vectors", linked to the
     new README, its purpose (A63: the harness and the suites that show each provider passing its contract)
     and its state ("Built on DATE by work order 123: the harness, with no suite yet");
   - in ROADMAP.md's register, one row in the form of experiment 012's: first cell "Experiment
     022", linked to the new README as 012's row links its own; the question (the harness
     for A63's provider-contract vectors: launching a provider package's item, driving its
     operations over MCP and reporting per implementation); needs ("No engine"); feeds (A63, and
     S10's P3); done when (the harness passes its own tests, and each suite runs on both of its
     contract's implementations with a dated report); state (`in progress: work order 123, in
     the providers lane of ruling 71`); and source ("Ruling 40; A63"). A63's own row stays in
     docs/AUDIT.md. Run `python3 tools/check_roadmap.py`.
10. **What this order does not do:** no suite's cases (they are their own orders), no trust
    roots, certificates, gate or fault controls (A63-H2), no provider package, and no change to
    `engine/`, `spec/`, `conformance/`, `packages/`, any other experiment, or any order, record or
    ruling.

## Scope

Change only `experiments/022-provider-contracts/` (new), its one row in `experiments/README.md`,
and A63's one row in ROADMAP.md's register.

## Done when

In `experiments/022-provider-contracts/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`

Report the totals passed and failed. From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_private_paths.py`
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
D=experiments/022-provider-contracts
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep target/ | grep -v '^!!'
```

`tools/check_private_paths.py` reads only tracked files, so these scans cover the new ones.

`target/` stays ignored and unstaged. Report each choice of the case form and of the report as one
of your own choices, and each behavior `spec/` leaves open as a finding. Do not stage anything.

## Record

Run on 2026-11-11 on the Mac, from base bb84a46, beside order 122: the worker in 24 minutes
(1453 s) and a fresh verifier in 7 minutes (403 s); then a second worker round in 7 minutes
(431 s) and a fresh verifier in 7 minutes (427 s). The result is
`experiments/022-provider-contracts/`: the launcher (a package archive by its digest, extracted
under `<lab>`, with PRV-023's environment and a persistent state directory), the MCP driver
(discovery, the extension declared by the client itself, PRV-005's keys, one send per call), the
`provider.yaml` case form and its strict validation, descriptors, per-implementation reports with
redaction, the four empty suites, and its own tests; with its row in `experiments/README.md` and
its register row in ROADMAP.md, by the admin's leave.

Round one's six musts were fixed in round two: the tool name is the export name, not the
qualified reference; the client declares the extension itself, since `rmcp` overwrote a manual
declaration; calls use `call_tool_once`, so nothing is resent; restart steps and
`no_marker_anywhere` reject unknown fields; the instance IDs `.` and `..` are refused; and the
tool name, its version and case IDs are redacted. Round two's verifier left three musts (two in
redaction, one in IFC-046's revision retry); order 125 closed them, and this order merged with it
as one change.
