# Work order 132 (A63-FS): the file secret store

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
secret's value, even a synthetic one that a test made.

## What and why

A file store ships with the reference engine as a package, and the engine holds no store
(IN_FORCE.md topic 12). This order is A63-FS of [PROVIDERS.md](../implementation/PROVIDERS.md):
read its row and its paragraph "A63-FS". It is one of the two secret stores of S10's comparison
(the second store, if A73 or its approvals are not in time: the owner's answer to S10's Q2), and
it is the store that tests and labs use without a keychain.

Read first:

- `get-secret.v1` in `spec/core/operations.yaml` and the capability `secret-provider.v1` in
  `spec/core/capabilities.yaml`;
- SEC-005, SEC-006 and SEC-022 in `spec/promises/secrets.yaml` (no controls, the value only from
  the store the locator names, no other capability), and the rest of `secrets.yaml` for what the
  engine does around a read, which this package does not do; IFC-006, IFC-008, IFC-009, IFC-037
  and IFC-047 in `spec/interface/implementation.yaml`; and OPS-007 in
  `spec/conventions/operations.yaml`;
- `packages/provider-local-machine/`, order 128's package: its small MCP server, its package form
  and builder, and its test that validates its YAML against `spec/schemas/`. Follow it; this crate
  depends on no crate of this repository, so copy what you need and report the copy as a finding.
  But that server models provider calls, and `get-secret.v1` is not one (decision 3);
- the head of `spec/promises/secrets.yaml` (near lines 14 to 20), on what a store's contract
  states; IFC-019 and IFC-026 in `spec/interface/implementation.yaml`, on filters and a `reason`.

## The decisions it applies

1. **Where it lives.** `packages/provider-file/`, a crate with its own `Cargo.toml`, committed
   `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and `dist/`. A
   provider, not a scenario server. For Unix; say so.
2. **The package.** `conduit.yaml`: address `example.org/conduit/provider-file`, version 0.1.0,
   exporting the Type `file-secret.v1`, with one implementation item, `provider`, whose `command`
   is `bin/provider-file`, which `implements` the core's `get-secret.v1` by its full reference, is
   not plain, and lists no `tools`. `contracts.yaml` declares the Type: family Resource, offering
   `github.com/shrav-k/conduit/core#secret-provider.v1` and nothing else (SEC-022), its
   `implementation` mapping the operation to `#provider`, `records` `inputs: false` and
   `outputs: false`, no `controls` (SEC-005), and a configuration schema whose locator is `path`,
   an absolute path to one file, with `additionalProperties: false`. A `package` binary writes the
   archive and its digest as order 128's does.
3. **`get-secret.v1`.** It arrives as a request through the instance's own Conduit, not as a
   provider call (SEC-010, SEC-011; IFC-006): its `_meta` carries `instance`, `operation`,
   `invocation`, `caller`, `filters`, `config`, `revision` and `conduit` under Conduit's prefix.
   The locator is `config.path`. Where `filters` is not empty, it reads nothing and answers with
   the `reason` `unavailable` and the `restriction`, and no outcome (IFC-019, IFC-026). Otherwise
   it reads the value from that one file only (SEC-006): never from the configuration, the
   environment or another file.
   - It opens the file with `O_NOFOLLOW`, so a symbolic link as the final component is refused,
     and checks the opened descriptor with `fstat`: a regular file only, with
     `mode & 0o077 == 0`. Do not compare against a canonical path: macOS's temporary directories
     and `/tmp` are reached through links.
   - It gives the content as `value`, as UTF-8, with exactly one final `\n` stripped, if there is
     one (the order's choice; the store suite will test it).
   - It reads at most a stated size, and answers `failed` past it.
   - It answers `failed`, with no value, where the file is missing, unreadable, not a regular file,
     not UTF-8, larger than the bound, or readable or writable by group or others, each with an
     explicit message (ruling 52) that names the reason and never the value. No read buffer is
     ever printed or formatted with `Debug` (a UTF-8 error's `Debug` prints the bytes).
   - It keeps no copy and no cache, and logs no value.
4. **Its contract text.** The Type's description and the README state, as the head of
   `secrets.yaml` asks a store's contract to: the locator's form; the largest value it reads; where
   it must run (on the machine that holds the file, by the engine's own runner); the permissions
   it requires, and their consequence, that a secret mounted by Docker (0444) or by Kubernetes (a
   link, 0644) is refused; the newline rule; that it reads one file and nothing else; that it keeps
   nothing; and D15's four fields as
   [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them.
   SEC-020 binds the engine, not this package: do not claim it.
5. **Tests**, driving the binary with an `rmcp` client and the full request `_meta` of decision 3,
   with synthetic secret markers in files made at test time (markers that look like no real token
   format: no `ghp_`, `sk-` or `AKIA`). Capture the child's stderr, so that "no log line" can be
   checked, and compare with `assert!(a == b)`, never `assert_eq!`, so that a failing test prints
   no marker:
   - discovery lists the extension; a call without it answers -32021 and reads nothing;
   - a call with a filter answers the `reason` `unavailable` and reads nothing;
   - a private file's content comes back as `value`, with one final newline stripped and a second
     one kept;
   - a missing file, a file readable by others, a group-readable file, a group- or other-writable
     file, a symbolic link, a FIFO, a file past the size bound, a non-UTF-8 file and an unreadable
     file (skipped when the tests run as root, saying so) each answer `failed` with no value, and
     no answer, message or stderr line holds the marker;
   - a configuration with an extra key, and one without `path`, answer `failed`;
   - the answer validates against the operation's output schema (copied, with a drift test);
   - `conduit.yaml` and `contracts.yaml` validate against `spec/schemas/`, skipped, saying so,
     where `spec/` is absent;
   - the `package` binary's archive is the same, byte for byte, when built twice.
6. **What this order does not do:** no engine reading or delivery of secrets (that is the admin's
   SEC-D), no listing or writing of secrets, and no change outside this crate.

## Scope

Change only `packages/provider-file/` (new).

## Done when

In `packages/provider-file/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`
- `cargo run --release --bin package`, twice, with the same digest

From the repository root: `python3 tools/check_docs.py` and `python3 tools/set_names.py --check`.
Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/provider-file
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
git status --porcelain --ignored $D | grep -E "target/|dist/" | grep -v '^!!'
```

`target/` and `dist/` stay ignored and unstaged. Report the size bound as your own choice. Do not
stage anything.
