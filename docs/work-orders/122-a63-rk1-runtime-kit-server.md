# Work order 122 (A63-RK1): the runtime-provider kit, its MCP server and backend interfaces

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Never print a whole large file: use
`head`, `tail`, `grep`, `sed -n` or a count. Run one cargo command at a time. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token or a private key, even one that a test made.

## What and why

A63 builds the providers as packages (ruling 40). Its plan,
[PROVIDERS.md](../implementation/PROVIDERS.md), puts first the code that every runtime provider
shares: A63-RK, a library crate of its own at `packages/runtime-provider-kit/`, which the local
process provider (A63-RP), the Docker provider (A63-DR) and the admin's SSH runtime (SSH-R) will
each depend on by path. Read the plan's paragraph "A63-RK1 / RK2 — What the runtimes share" and
its rows A63-RK1 and A63-RK2 before you start. This order is RK1: the provider's MCP server for
`runtime-provider.v1` and the interfaces a runtime implements beneath it. RK2, a later order, adds
the gate. No runtime exists yet; this order's tests use a fake backend and a fake gate.

The kit is not a provider and not an installable package: it has no `conduit.yaml`, no Type and
no archive. A runtime package wraps it.

Read first:

- the operations `run.v1`, `stop.v1` and `inspect-run.v1` in `spec/core/operations.yaml` (from
  line 864), with their schemas, and the capability `runtime-provider.v1` in
  `spec/core/capabilities.yaml`;
- in `spec/promises/providers.yaml`: PRV-005 (a provider call), PRV-007 (when the engine asks for
  a run), PRV-008 to PRV-012, PRV-019 (a machine the provider cannot run on), PRV-021 (owed
  stops), PRV-022 (identity), PRV-023, PRV-025 and PRV-028 (the gate, RK2's), PRV-027 and
  PRV-029 (the state directory);
- in `spec/interface/implementation.yaml`: IFC-001, IFC-006, IFC-008, IFC-009, IFC-037 and
  IFC-047; and OPS-007 in `spec/conventions/operations.yaml`, which says how the engine reads an
  answer and its `outcome` key;
- order 081 (`docs/work-orders/081-spec-one-run-per-instance.md`), its text for a repeated
  `run.v1` and for a stop. Its specification work is not merged (decision 5);
- order 080 (`docs/work-orders/080-shell-runtime-provider-calls.md`), its section "What and why"
  and its steps on provider calls, which say how the engine's shell will call a runtime provider
  and read its answer;
- the engine's own server, for the wire form of Conduit's extension in `rmcp` 3.9.0:
  `engine/crates/shell/src/server.rs`, near line 271 (listing the extension), its `discover` near
  line 329, and near lines 606 to 618 (refusing a call without the extension with -32021);
  `engine/crates/core/src/requests.rs`, near lines 581 to 599 (the `_meta` keys); and
  `engine/crates/shell/src/server_tests.rs`, near line 411 (a call declaring the extension). Read
  them; change none;
- `packages/agent-codex/`: the form of a crate under `packages/` (`Cargo.toml`,
  `rust-toolchain.toml` at 1.97.1, `Cargo.lock`, `.gitignore`, README, tests). Its
  `tests/wrapper.rs`, near lines 286 to 341, drives an MCP server with an `rmcp` client. Pin
  `rmcp` at `=3.9.0`, as the engine's shell does.

## The decisions it applies

1. **Where it lives.** `packages/runtime-provider-kit/`, a library crate named
   `runtime_provider_kit`, with its own `Cargo.toml`, committed `Cargo.lock`,
   `rust-toolchain.toml` at channel 1.97.1, a `.gitignore` for `target/`, a README and tests. It is
   not in the engine's workspace and depends on no other crate of this repository.
2. **The server.** A function or type that serves `runtime-provider.v1` over MCP on a given
   transport (stdio first: the engine's own runner reaches a provider's own run over stdio,
   PRV-002), for a value that implements the backend interfaces of decision 4.
   - It lists Conduit's extension at revision `v0` in `server/discover` (IFC-037), and offers
     exactly the tools `run.v1`, `stop.v1` and `inspect-run.v1`.
   - **Schemas.** It carries the three operations' input and output schemas, copied into the
     crate together with every definition they reach: `run.v1` reaches
     `core#implementation-item.v1`, and through it the server-package, server-remote, selection,
     pointer, labels and id definitions (`spec/schemas/manifest.yaml`, from about line 141, and
     `values.yaml`). Resolve each `core#name` reference to its copy, and say how. A test reads
     `spec/` and fails when any copy differs.
   - **Answers** (IFC-008, IFC-009, OPS-007). Every result carries the `outcome` key under the
     `_meta` prefix, with `isError` agreeing with it. An argument that breaks the input schema,
     and an item the runtime cannot run, answer `failed`, with a reason. A stop the provider owes
     answers `unknown`. `inspect-run.v1` answers `succeeded` with `{state}` and, where known,
     `url`, also when the state is `unknown`. The one protocol error is -32021, with
     `data.requiredCapabilities` naming the extension, for a call that does not declare the
     extension or declares another revision (IFC-008, IFC-047), and the backend is never called
     for it. Read OPS-007 for how the engine reads any other protocol error: as unknown.
   - **Names.** The names that `spec/names.yaml` assigns (the `_meta` prefix, the core package's
     address, the identity variables, the state-directory variable) are each written once, in one
     module. `tools/set_names.py` rewrites variables and the prefix in `.rs` files, but file names
     only in backticks: take the identity files' names from the copied schema of decision 2's
     "Schemas", which the drift test covers. A new `CONDUIT_*` variable name fails
     `set_names.py --check`; add none. The crate does not read `spec/` when it builds: it must
     build outside this repository.
3. **What every backend call receives:** the provider call's `config` and `revision`, the
   provider's only source of its configuration (PRV-005, IFC-006), and `machine` (PRV-027).
4. **The backend interfaces.** Rust traits that a runtime implements. Name them as you judge;
   they must cover, and the README must document:
   - **its declaration:** the `registryType`s and transports it runs, whether it runs `command`
     items, and the machines it can run on. A `run.v1` outside it ends `failed` and runs nothing
     (the operation's description; PRV-019 for a machine);
   - **staging:** given a run's `instance`, `item`, `packageArguments`, `environment`, `secrets`,
     `identity` and `machine`, prepare what the run needs, placing every file of `identity` and
     every secret where PRV-022 and PRV-011 say and nowhere else. A runtime may stage on another
     machine;
   - **start:** start the staged run and return a duplex byte channel to its stdio;
   - **stop:** stop the run's whole tree, as the runtime's contract declares a stop;
   - **state:** report `running`, `stopped` or `unknown`, as `inspect-run.v1` defines them;
   - **the state store:** a durable record, which the runtime places, of the runs the provider
     holds (the run ledger) and of the stops it owes (PRV-021);
   - **the gate:** given a run's channel and its identity, return a handle that gives the `url`
     at which the run is reached, that a stop closes, and that signals when it ends. RK2 supplies
     the real gate; this order defines the interface and a fake.
5. **What the server decides.** Build what `spec/` states today. Where it leaves a choice open,
   take the choice below, put it behind one function with a comment that order 081 settles it,
   and report it as a finding:
   - **One run per instance.** `run.v1` for an instance whose run the provider holds running
     answers that run's current `url` and starts no other: the operation says a provider "starts
     no run for an instance's ID while a run that it started for that ID may still be running".
     Where the state is `unknown`, it starts nothing (the kit's choice).
   - **A stop.** `stop.v1` for a run that is not running succeeds. Where the backend cannot stop
     the run now, the stop is written to the state store as owed, and the answer is `unknown`. A
     stop that succeeded stays true: it cancels a start still in flight, and no restart of that run
     follows it (the kit's choice, within PRV-010's latitude).
   - **The lifetime follows the plan's model.** A run is tied to its channel: when the channel
     ends, the server stops the run's tree and updates the ledger. A configured bound on a run's
     lifetime is the backstop. At startup the server stops every run the backend reports, whether
     the ledger holds it or not, since no run outlives the provider's channel to it; a run it
     cannot stop becomes an owed stop.
   - **Owed stops** are drained by a public function that needs no running server, so that a
     runtime's own mechanism can drain them after the provider is retired (PRV-021;
     PROVIDERS.md). The server calls it at startup, on each call, and on an interval the runtime
     sets. A stop is owed whether or not the instance still exists.
   - `inspect-run.v1` reports `stopped` only where no run the provider started for the ID can be
     running, and `unknown` where it cannot tell; it gives `url` where the gate gives one.
   - Nothing it records, logs or answers holds a value of `secrets` or anything of `identity`.
6. **What this order does not do:** no gate, TLS or certificate handling (RK2); no runtime and no
   Type (A63-RP, A63-DR); no change to `engine/`, `spec/`, `conformance/`, any other package, or
   any order, record or ruling. Where `spec/` leaves a behavior open beyond decision 5, leave it
   behind one function and report it as a finding; invent no rule.
7. **Tests.** With a fake backend and a fake gate, and an `rmcp` client in the crate's tests, cover
   at least:
   - discovery and the extension; -32021 for a call without the extension, with no backend call;
   - `failed` for an argument that breaks the schema, with no backend call, and for an item or a
     machine outside the declaration;
   - run, inspect and stop, each answer carrying the `outcome` key as decision 2 says;
   - a repeated `run.v1` on a running run answering the same `url` with one start;
   - a channel that ends stopping the run's tree and updating the ledger;
   - a stop the backend cannot carry out becoming owed and `unknown`, surviving a restart of the
     server over the same store, and carried out by the public drain function after the instance
     is gone, with no server running;
   - a successful stop cancelling an in-flight start, with no restart after it;
   - the startup sweep, of runs the ledger holds and of runs it does not;
   - the schema drift test;
   - no secret or identity value in any record, log line or answer, by synthetic markers.

## Scope

Change only `packages/runtime-provider-kit/` (new).

## Done when

In `packages/runtime-provider-kit/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`

Report the totals passed and failed. From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_private_paths.py`, which reads only tracked files, so also the scans below

Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep target/ | grep -v '^!!'
```

`target/` stays ignored and unstaged. Report each choice of decision 5 and each behavior left open
as a finding, and each interface name as one of your own choices. Do not stage anything.
