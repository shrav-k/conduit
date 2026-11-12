# Work order 127 (A63-RP1): the local-process runtime provider

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token, a private key or a secret value, even one that a test made.

## What and why

A63 builds two providers for each provider contract (ruling 40). The engine's own runner is the
built-in local-process runtime and stays S10's first implementation; the admin decided (option (b)
of order 120) that a packaged local-process provider is built as well, so that the runtime
contract's suite runs over MCP on two real implementations, this one and Docker. This order is
A63-RP1 of [PROVIDERS.md](../implementation/PROVIDERS.md). It starts only once order 129, which
makes the kit refuse a run on a machine that still owes a stop, is merged. Read its row and its
paragraph
"A63-RP1 / RP2". It builds the package on the runtime-provider kit,
`packages/runtime-provider-kit/`, which orders 122, 124 and 126 built: the kit holds the MCP
server and the gate; this provider implements the kit's backend for a local process. RP2, later,
adds recovery across restarts, the external drainer of owed stops and separate operating-system
users for runs and the gate.

Read first:

- the kit: its `README.md`, `src/backend.rs` (the traits), `src/server.rs` (`RuntimeServer`) and
  `src/gate` (the gate and its configuration);
- `run.v1`, `stop.v1` and `inspect-run.v1` in `spec/core/operations.yaml` (from line 864);
  PRV-006 to PRV-012, PRV-019, PRV-022, PRV-023, PRV-027 and PRV-029 in
  `spec/promises/providers.yaml`; MAN-009 and MAN-010 in `spec/promises/manifest.yaml`;
- `packages/agent-codex/`: the form of a package (its `conduit.yaml`, `contracts.yaml`,
  `Cargo.toml`, the `package` binary that writes a deterministic archive and its digest, its
  README and tests); and a Type that offers a core capability, as
  `conformance/conventions/setup/review-relative-provider/documents.json` shows one offering
  `github.com/shrav-k/conduit/core#runtime-provider.v1`;
- the engine's own runner, which this provider follows for a local process, as reference only:
  `engine/crates/shell/src/runner.rs`, where it builds a run's environment and process group, and
  DL16's rows in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **Where it lives.** `packages/provider-local-process/`, a crate with its own `Cargo.toml`,
   committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and
   `dist/`, depending on the kit by path (`../runtime-provider-kit`) and on no other crate of this
   repository. It is a provider, not a scenario server: no `scenario-` prefix. It is for Unix:
   process groups and file modes are Unix's; say so in the README, and gate the tests by platform.
2. **The package.** `conduit.yaml`: address `example.org/conduit/provider-local-process` (as
   agent-codex's is `example.org/...`; `set_names.py` refuses look-alikes of the core's address),
   version 0.1.0, exporting the Type `local-process.v1`, with one implementation item, `provider`,
   whose `command` is the crate's binary `bin/provider-local-process`, which `implements` the
   core's `run.v1`, `stop.v1` and `inspect-run.v1` by their full references, is not plain, and
   lists no `tools` (PKG-020; the tool names are the operations' export names, IFC-001).
   `contracts.yaml` declares the Type: family Resource, offering
   `github.com/shrav-k/conduit/core#runtime-provider.v1` and nothing else, its `implementation`
   mapping each of the three operations to `#provider` (as the Type in
   `conformance/conventions/setup/review-relative-provider/documents.json` does), `records`
   `inputs: false` and `outputs: false`, and a configuration schema for the gate's bind address
   and advertised host, its keepalive idle time, interval and count, and a run's lifetime bound,
   each with a default, with `additionalProperties: false`. A `package` binary writes
   `dist/provider-local-process.tar.gz` and its `.sha256` as agent-codex's does, deterministically.
3. **Configuration reaches the provider at its start.** The kit fixes its gate's configuration and
   its lifetime bound when they are built, and a provider call's configuration arrives only with
   each call. So the item's `environment` binds each configuration value to a variable of the
   provider's process, as agent-codex's item does, and the binary reads them once at startup. No
   variable name begins with `CONDUIT_` (`set_names.py --check` refuses one). The contract says
   that a change of configuration takes effect at the provider's next start.
4. **The binary** serves the kit's `RuntimeServer` over stdio with this provider's backend, its
   state store and the kit's TLS gate. Everything it keeps lives under its state directory, the
   variable that `spec/names.yaml` names `stateDirVariable` (PRV-029). Without it, the binary
   refuses to start, with a diagnostic that says so.
5. **Runs end with the provider.** The engine's guard kills the provider's process group when the
   engine ends (`engine/crates/shell/src/runner.rs`, near lines 38 and 105 to 113), and a run in
   a group of its own would escape it. So each run's process group is led by a guard: the
   provider's own binary in a guard mode, which holds a pipe from the provider, starts the run's
   program in its group, kills the whole group when the pipe closes, and kills it at the
   configured lifetime bound. The guard holds a lock on a per-run marker file in the state
   directory for as long as any member of the group may live, so that after a restart the
   provider can tell, without trusting a saved process ID:
   - **inventory:** the runs whose marker exists;
   - **state:** `running` while the marker's lock is held, `stopped` once it is free, `unknown`
     where it cannot tell;
   - **stop:** signal the group, wait a short grace, kill it, and confirm that the marker's lock is
     free; report `unknown` only where it cannot tell.
6. **The backend.**
   - **Declaration:** `command` items whose program is an absolute path, on this machine only (its
     hostname, as `run.v1`'s `machine` gives it). The kit accepts a bare program name, which
     MAN-010 has a starter look up; this provider refuses it in staging, ending `failed`, as it
     refuses a relative path. A `package` item of any registry or transport, and another machine,
     end `failed` and run nothing. Registry packages (npm) wait for RP2: report them as a finding.
   - **Staging:** a private directory per run under the state directory, named by a hex SHA-256
     of the instance's ID, never the raw ID (`id.v1` is any non-empty string), mode 0700, with the
     files of `identity` at 0600 under a directory that `CONDUIT_IDENTITY_DIR` names, and a
     persistent per-instance directory, named the same way, that the run receives under
     `stateDirVariable` (PRV-029) and that the provider never removes, since it never learns of an
     unregistration. Secrets go only into the variables and arguments their names give (PRV-011);
     the item's own `environment` and `packageArguments` bind as MAN-009 and MAN-010 say. Once a
     stop is confirmed, the run's private directory, its identity files with it, is removed.
   - **Start:** the guard starts the program with stdin and stdout piped as the kit's duplex
     channel, stderr to a private file in the run's directory, and an environment built as PRV-023
     builds one for the engine's runner: cleared, then the names PRV-023 passes, then the item's
     environment, the secrets, the identity variables and the state-directory variable.
   - **State store:** the kit's ledger and owed stops in one file under the state directory,
     written atomically (write, fsync, rename, fsync the directory), acknowledged only after that.
   - **Gate:** the kit's TLS gate, one listener per run, configured as decision 3 says.
7. **Its contract text.** The Type's description and the README state what PRV-021, PRV-025,
   PRV-028 and the others ask a runtime's contract to state:
   - its gate, and the silent-cut bound its configuration gives, with the kit's caveat on macOS
     (no `TCP_USER_TIMEOUT`);
   - that it runs `command` items with absolute programs on this machine only, no registry type
     and no transport (PRV-012), and as the provider's own operating-system user until RP2
     separates them;
   - where it keeps secrets and identity while a run lives (the process environment, the identity
     files, `tls.key` among them) and when it deletes them (PRV-011, PRV-022);
   - the state directory it supplies to each run, where, at what mode, and that it is never
     removed (PRV-029); and that it needs its own state directory to start;
   - no restart of a run (PRV-010); a stop's reach, the process group, which a descendant that
     calls `setsid` escapes; and where owed stops are kept (PRV-021);
   - the owner's rule of 2026-11-12 (MC4-002): a held stop is carried out before any new run on
     that machine starts, and a start refused for this names the owed stop. The kit enforces it
     since order 129. On this provider, which runs on one machine, a single owed stop blocks every
     new run until it is carried out;
   - D15's four fields as [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures)
     defines them. Say plainly what RP1 does not yet do.
8. **Tests**, with a small MCP server built by the tests as the run's program, and a TLS client in
   the tests that checks the chain and the instance's URI, copied from the kit's gate tests, which
   it does not export:
   - run, inspect and stop of a real process, reached through the gate with the engine's token;
   - a repeated `run.v1` on a running run answers the same `url` and starts no second process;
   - a stop ends the whole group: the program starts a child in its group, and neither survives;
   - the provider killed with SIGKILL while a run and its child live: neither survives, the
     restarted provider reports the run `stopped`, and a new `run.v1` succeeds;
   - a relative program, a bare program name, a `package` item and another machine's hostname
     each end `failed`; an instance ID of `../x` stays inside the state directory;
   - the run's environment: launch the provider with a cleared environment and synthetic values
     for the names PRV-023 passes, and compare the run's variable names and those synthetic values
     only, never the real environment, so that a failure prints nothing real; its identity files
     at 0600 in a 0700 directory, removed after a confirmed stop;
   - no secret or identity value in the provider's own logs, answers or state file;
   - the state file survives a restart with its contents;
   - a start that a held stop blocks: instance A's stop cannot be carried out, so `run.v1` for
     instance B is refused (`failed`, its message naming A's ID and the machine); once A's stop is
     carried out, B starts. Make A's stop fail through an injectable signaller in the backend
     library, which the tests replace, never through an environment variable or flag of the
     shipped binary (`cfg(test)` does not reach a binary that integration tests run). A signal
     that fails with the marker's lock still held gives `unknown` and an owed stop (decision 5);
   - `conduit.yaml` validates against the manifest schema and `contracts.yaml` against the
     document schema in `spec/schemas/`, in a test that is skipped, saying so, where `spec/` is
     absent: `tools/check_schemas.py` does not read `packages/`;
   - the `package` binary's archive is the same, byte for byte, when built twice.
9. **What this order does not do:** no external drainer of owed stops after the provider is
   retired, no separate operating-system users (both RP2), no registry packages, no change to the
   kit (report anything the kit lacks as a finding), and no change outside this crate. The work is
   at the top of L; keep the copied test helpers to what the tests need.

## Scope

Change only `packages/provider-local-process/` (new).

## Done when

In `packages/provider-local-process/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`
- `cargo run --release --bin package`, twice, with the same digest

From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/provider-local-process
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rln -E "PRIVATE KEY|BEGIN CERTIFICATE" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
git status --porcelain --ignored $D | grep -E "target/|dist/" | grep -v '^!!'
```

`target/` and `dist/` stay ignored and unstaged. Test keys and certificates are made at run time,
never committed, and no real hostname goes into a README or a snapshot. Report each default of
the configuration, the stop's grace, the kit's drain interval and the variable names as your own
choices; and as findings, each thing the kit lacks, and that an engine whose hostname is set by
configuration (DL5) would not match this provider's OS hostname. Do not stage anything.
