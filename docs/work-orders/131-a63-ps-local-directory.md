# Work order 131 (A63-PS): the local-directory package source

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, and never print an environment variable's value.

## What and why

A63 builds two providers for each provider contract (ruling 40). The engine fetches `file` URLs
itself and stays S10's first implementation; the admin decided (option (b) of order 120) that a
packaged local-directory source is built as well, so that the package-source contract's suite
runs over MCP on two real implementations, this one and Git. This order is A63-PS of
[PROVIDERS.md](../implementation/PROVIDERS.md): read its row and its paragraph "A63-PS".

Read first:

- `fetch-package.v1` in `spec/core/operations.yaml` and the capability `package-source.v1` in
  `spec/core/capabilities.yaml` (it claims the schemes its configuration lists under `schemes`,
  for its whole life);
- PRV-002, PRV-005, PRV-013, PRV-016 and PRV-017 in `spec/promises/providers.yaml`; PKG-026 in
  `spec/promises/packages.yaml`; IFC-006, IFC-008, IFC-009, IFC-037 and IFC-047 in
  `spec/interface/implementation.yaml`; and OPS-007 in `spec/conventions/operations.yaml`;
- `packages/provider-local-machine/`, order 128's package: its small MCP server (discovery, the
  extension, -32021, the `outcome` key), its package form and builder, and its test that validates
  `conduit.yaml` and `contracts.yaml` against `spec/schemas/`. Follow it; this crate depends on no
  crate of this repository, so copy what you need and report the copy as a finding;
- `engine/crates/core/src/packages.rs` and `archives.rs`, as reference only, for how the engine
  reads a `file` URL and checks an archive.

## The decisions it applies

1. **Where it lives.** `packages/provider-local-directory/`, a crate with its own `Cargo.toml`,
   committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and
   `dist/`. A provider, not a scenario server. For Unix; say so.
2. **The package.** `conduit.yaml`: address `example.org/conduit/provider-local-directory`,
   version 0.1.0, exporting the Type `local-directory.v1`, with one implementation item,
   `provider`, whose `command` is `bin/provider-local-directory`, which `implements` the core's
   `fetch-package.v1` by its full reference, is not plain, and lists no `tools`.
   `contracts.yaml` declares the Type: family Resource, offering
   `github.com/shrav-k/conduit/core#package-source.v1` and nothing else, its `implementation`
   mapping the operation to `#provider`, `records` `inputs: false` and `outputs: false`, no
   `controls` (PRV-016: no control reaches `schemes`), and a configuration schema with
   `required: [schemes, root]`, `schemes` as `{type: array, minItems: 1, items: {const: file}}`
   (the capability fixes the name, and PRV-016 rejects a Type whose schema does not require it;
   follow the example under `spec/schemas/examples/invalid-by-rule/` named for PRV-016), and
   `root`, an absolute directory, with `additionalProperties: false`. `check_schemas.py` checks
   only controls here, so the package's own YAML test must check this shape. A `package` binary writes the archive and its digest as order
   128's does.
3. **`fetch-package.v1`.** It reads the call's configuration (PRV-005). It maps a `fetch` URL to a
   path exactly as the engine does (`engine/crates/shell/src/lib.rs`, near lines 381 to 385:
   `Url::to_file_path`, with percent-decoding, a host that is empty or `localhost`, and the scheme
   compared without regard to case, PRV-016). The engine sends no relative `fetch`: MAN-006
   resolves one to a `file:` URL, and PKG-023 rejects relative and user-information fetches at
   install. It answers only for a `file` URL whose path, once resolved with every symbolic link
   followed, lies inside `root`, itself canonicalized, compared by path components (so a sibling
   `/r2` is not inside `/r`), and is a regular file. It returns the file's bytes unchanged, base64 as RFC
   4648 gives it, in `archive`, and checks nothing of them: the engine checks the digest
   (PKG-026), so corrupt bytes are returned as they are. A URL outside `root`, a scheme it does not
   claim, a relative reference, another host, a missing, unreadable or non-regular file, and a
   `fetch` that carries a credential (user information) each answer `failed`, with an explicit
   message (ruling 52) that names its own reason and never the bytes. It keeps no cache, and it
   states the largest archive it reads (it holds the whole file in memory, and base64 grows it by
   about four thirds), answering `failed` past it.
4. **Its contract text.** The Type's description and the README state the scheme it claims and
   that it claims it for its life; what claiming `file` takes over: once registered it is the
   engine's only fetcher of `file` URLs (PRV-013, PRV-017), so every `file` install, resolved
   relative fetches included, must lie under `root`, one root per engine while it stands, owned by
   `system` and placed `local` (PRV-002); the containment rule; the size bound; that it returns
   bytes unchecked; that it keeps nothing; and D15's four fields as
   [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them.
5. **Tests**, driving the binary with an `rmcp` client, on disposable directories made at test
   time:
   - discovery lists the extension; a call without it answers -32021 and reads nothing;
   - a file inside `root` comes back byte for byte; a file with corrupt bytes comes back unchanged;
   - a path outside `root`, by `..`, by `%2e%2e`, by a symbolic link that points out, and in a
     sibling directory whose name starts with root's, answers `failed`;
   - a scheme other than `file`, a relative reference, another host, user information, a missing
     file, a FIFO, a file past the size bound, and an unreadable file (skipped when the tests run
     as root, saying so) each answer `failed`;
   - the answer validates against the operation's output schema (copied, with a drift test);
   - `conduit.yaml` and `contracts.yaml` validate against `spec/schemas/`, skipped, saying so,
     where `spec/` is absent;
   - the `package` binary's archive is the same, byte for byte, when built twice.
6. **What this order does not do:** no engine dispatch of package sources (that is the admin's
   PSD), no other scheme, and no change outside this crate.

## Scope

Change only `packages/provider-local-directory/` (new).

## Done when

In `packages/provider-local-directory/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`
- `cargo run --release --bin package`, twice, with the same digest

From the repository root: `python3 tools/check_docs.py` and `python3 tools/set_names.py --check`.
Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/provider-local-directory
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
git status --porcelain --ignored $D | grep -E "target/|dist/" | grep -v '^!!'
```

`target/` and `dist/` stay ignored and unstaged. Do not stage anything.
