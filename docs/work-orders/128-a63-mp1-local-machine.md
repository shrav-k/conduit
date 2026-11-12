# Work order 128 (A63-MP1): the local machine provider

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
private key, even one that a test made.

## What and why

A63 builds two providers for each provider contract (ruling 40). The engine's own machine
provider reports the one machine it runs on and stays the built-in; the admin decided (option (b)
of order 120) that a packaged local machine provider is built as well, so that the machine
contract's suite runs over MCP on two implementations, this one and the second host's. This order
is A63-MP1 of [PROVIDERS.md](../implementation/PROVIDERS.md): read its row and its paragraph
"A63-MP1 / MP2". It serves the machine contract as it stands on `main`, `list-machines.v1` and
`place.v1`; the enrollment operations that order 038b staged land with order 049a-1 after the
rerun's pin, and MP2 takes them up then, once the admin has settled how a local-only provider is
qualified for enrollment (the admin's decision C: MP1 stays local).

Read first:

- `list-machines.v1` and `place.v1` in `spec/core/operations.yaml`, and the capability
  `machine-provider.v1` in `spec/core/capabilities.yaml`;
- PRV-002, PRV-005, PRV-013, PRV-014 and PRV-015 in `spec/promises/providers.yaml`; ENR-002 in
  `spec/promises/enrollment.yaml` (the machine `id`); PLC-007 and PLC-026 in
  `spec/promises/placement.yaml`; IFC-006, IFC-008, IFC-009, IFC-037 and IFC-047 in
  `spec/interface/implementation.yaml`; and OPS-007 in `spec/conventions/operations.yaml`;
- how the engine reports the local machine at first start, for its canonical hostname and its
  `kubernetes.io/os` and `kubernetes.io/arch` labels: `engine/crates/shell/src/lib.rs`, near lines
  667 to 690, as reference only;
- `packages/runtime-provider-kit/src/server.rs`, for how a provider's MCP server lists Conduit's
  extension, refuses a call without it (-32021) and answers with the `outcome` key as OPS-007
  reads it; and `packages/agent-codex/` for a package's form and its `package` binary.

## The decisions it applies

1. **Where it lives.** `packages/provider-local-machine/`, a crate with its own `Cargo.toml`,
   committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and
   `dist/`. It depends on no crate of this repository: the runtime kit is for runtimes. Write its
   small MCP server here, following the kit's server for discovery, the extension, the
   `outcome` key and -32021; report any code worth sharing later as a finding.
2. **The package.** `conduit.yaml`: address `example.org/conduit/provider-local-machine` (as
   agent-codex's is `example.org/...`; `set_names.py` refuses look-alikes of the core's address),
   version 0.1.0, exporting the Type `local-machine.v1`, with one implementation item,
   `provider`, whose `command` is the crate's binary `bin/provider-local-machine`, which
   `implements` the core's `list-machines.v1` and `place.v1` by their full references, is not
   plain, and lists no `tools` (PKG-020; the tool names are the operations' export names,
   IFC-001). `contracts.yaml` declares the Type: family Resource, offering
   `github.com/shrav-k/conduit/core#machine-provider.v1` and nothing else, its `implementation`
   mapping both operations to `#provider` (as the Type in
   `conformance/conventions/setup/review-relative-provider/documents.json` does), `records`
   `inputs: false` and `outputs: false`, and an empty configuration schema
   (`additionalProperties: false`): a machine's labels are reported status, never configuration. A `package` binary writes `dist/provider-local-machine.tar.gz` and its
   `.sha256` as agent-codex's does, deterministically.
3. **`list-machines.v1`** reports exactly one machine, the one the provider runs on:
   - `hostname`: the OS's hostname, trimmed and lowercased, as the engine's first start makes it;
     one longer than 63 characters is a failure the answer says, not a truncation;
   - `labels`: `kubernetes.io/hostname` with that hostname, and `kubernetes.io/os` and
     `kubernetes.io/arch` with the values Go gives `GOOS` and `GOARCH` (`darwin`, `linux`,
     `windows`; `amd64`, `arm64`), as the engine maps them; no other label;
   - `state`: `reachable`;
   - `id`: ENR-002's opaque, provider-scoped, stable ID, which PRV-014 and ENR-002 already
     require on `main` and which the operation's output schema admits, since its items are not
     closed. It is derived from a key the provider makes once and keeps in its state directory
     (the variable `spec/names.yaml` names `stateDirVariable`), so that it survives restarts and
     changes only with a new key. The key is written atomically (a temporary file, fsync, a rename
     that never replaces an existing key, fsync of the directory), so that a crash can never
     change the `id`; it never leaves the state directory, at 0600 in a 0700 directory. Without
     the state directory, the binary refuses to start, with a diagnostic that says so.
4. **`place.v1`** answers `hostname` with that machine where every label of `nodeSelector` equals
   the machine's (an empty selector matches), and answers with no `hostname` otherwise. It places
   nothing else. PRV-015 says the provider names the machine by its `id`, while the operation's
   output schema gives `hostname`: keep `hostname`, as the schema says, and report the conflict as
   a finding.
5. **Its contract text.** The Type's description and the README state which name of the machine it
   reports (PRV-014), how the `id` is derived, that it reports one machine and never `gone` or
   `pending`, that it is for Unix, that it needs its own state directory (PRV-029), and D15's four
   fields as [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures)
   defines them. Say plainly that enrollment is
   not served until MP2.
6. **Tests**, driving the binary with an `rmcp` client; the file-mode checks are for Unix and
   gated by platform:
   - discovery lists the extension; a call without it answers -32021 and does nothing;
   - `list-machines.v1`'s answer validates against the operation's output schema (copied, with a
     test that fails when the copy differs from `spec/`), with the canonical hostname and the
     three labels;
   - the `id` is the same across two runs on one state directory, differs on a fresh one, and the
     key file is 0600 in a 0700 directory;
   - `place.v1` with a matching selector, an empty selector, a selector naming another hostname,
     and an unknown label;
   - a `place.v1` argument that breaks its input schema answers `failed` (`list-machines.v1` has
     no input schema: say what the binary advertises for it, as your own choice);
   - `conduit.yaml` validates against the manifest schema and `contracts.yaml` against the
     document schema in `spec/schemas/`, in a test that is skipped, saying so, where `spec/` is
     absent: `tools/check_schemas.py` does not read `packages/`;
   - the `package` binary's archive is the same, byte for byte, when built twice.
7. **What this order does not do:** no enrollment, no evidence, no `pending` machines (MP2), and
   no change outside this crate.

## Scope

Change only `packages/provider-local-machine/` (new).

## Done when

In `packages/provider-local-machine/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`
- `cargo run --release --bin package`, twice, with the same digest

From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/provider-local-machine
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rln -E "PRIVATE KEY|BEGIN CERTIFICATE" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
git status --porcelain --ignored $D | grep -E "target/|dist/" | grep -v '^!!'
```

`target/` and `dist/` stay ignored and unstaged. Test keys are made at run time, never committed,
and no real hostname goes into a README or a snapshot. Report the key's algorithm and the `id`'s
derivation as your own choices, and, as a finding, that an engine whose hostname is set by
configuration (DL5) would not match the OS hostname this provider reports. Do not stage anything.

## Record

Run on 2026-11-12 on the Mac, from base b451e4a: the worker and a fresh verifier (358 s), then a
second worker round (234 s) and a fresh verifier (298 s). The result is
`packages/provider-local-machine/`: the Type `local-machine.v1`, its MCP server, `list-machines.v1`
reporting the one machine with its canonical hostname, its three labels and a stable `id` from a
key written atomically in its state directory, `place.v1` by selector, the package builder, and
tests that validate its YAML against `spec/schemas/`.

Round one's must (Rust's `loongarch64` reported in place of Go's `loong64`) was fixed in round
two. Round two's verifier left one must: little-endian MIPS hosts reported `mips` and `mips64`
in place of Go's `mipsle` and `mips64le` (PRV-014 asks for Go's GOARCH spellings). **The lane
coordinator fixed it, an admin-side fix at the admin's word (a small finding, ruling 71),** with
explicit tests for both endiannesses. A fresh reader checked the mapping against Go's list and
found two shoulds, which the coordinator also applied: big-endian ARM gives `armbe` and
`arm64be`, and `riscv32` and `wasm32`/`wasm64` give Go's `riscv` and `wasm`. The crate's fmt,
clippy and 12 tests pass.

The worker's findings, for the admin: the engine's first-start report
(`engine/crates/shell/src/lib.rs`, near lines 678 to 686) has the same mapping gaps (LoongArch,
little-endian MIPS, big-endian ARM); PRV-015 says the provider names a machine by its `id`, while
`place.v1`'s output schema gives `hostname`, which this provider keeps; and an engine whose
hostname is set by configuration (DL5) would not match the OS hostname this provider reports.
