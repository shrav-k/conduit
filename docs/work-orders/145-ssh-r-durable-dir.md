# Work order 145 (SSH-R): a durable directory, outside the engine's lifecycle

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

The SSH runtime provider, `packages/provider-ssh-runtime/` (orders 137 and 140), keeps its ledger,
owed stops, markers, part, `known_hosts` and private key in `CONDUIT_STATE_DIR`. PRV-021 asks for a
place that "neither a restart of the engine nor the provider's own retirement loses"; but retiring
the provider removes its instance, and PRV-029 (LIF-035) then removes that directory with
everything in it, so no stop still owed can be carried out after retirement.
[PROVIDERS.md](../implementation/PROVIDERS.md)'s decision D settles an external stop-drainer that
survives retirement; it needs a place that survives too. **The admin decided (2026-11-12, under
PRV-021)** that the provider keeps that state in a directory the operator configures outside the
engine's store. This order moves the state there; [order 146](146-ssh-r-drainer.md) builds the
drainer on it. Read PRV-021 and PRV-029 in `spec/promises/providers.yaml`, the crate's README,
`src/main.rs`, `src/backend.rs` (its root and owner lock, near lines 108 to 140), `src/storage.rs`
and `src/config.rs`, and the kit's README on exclusive ownership.

## The decisions it applies

1. **The place.** A new required configuration place, `durableDir`, bound to
   `SSH_RUNTIME_DURABLE_DIR`: an absolute path (`^/`), with no default, which the item's
   environment binds and the schema requires (MAN-017; the crate's package test enforces it).
2. **One subdirectory per provider instance.** Under `durableDir`, the provider keeps a
   subdirectory at mode 0700 named by the lowercase hex SHA-256 of the bytes of the
   `CONDUIT_STATE_DIR` value it was given, as given, with no trailing newline: PRV-029 makes that
   path unique to the instance and never reuses it, and this engine never canonicalizes it. Two
   providers never share a subdirectory, a ledger or a lock. The provider is not given its own
   instance ID directly: report that as a finding. Hash the raw value; compare paths for
   containment (decision 4) by their canonical forms, since macOS reaches `/var` through
   `/private/var`.
   **At startup, in this order:** make or open the subdirectory; take the owner lock; check the
   file `state-dir`. At the first start there, with no `state-dir` file, write the empty ledger
   (`[]`) atomically and then `state-dir`, holding the `CONDUIT_STATE_DIR` value. Refuse to start
   where `state-dir` holds another value, or exists while the ledger does not. `FileStore`
   answers an error, not an empty ledger, where the ledger is missing (`src/storage.rs`, near line
   54), so that a missing ledger always means a removed directory, never "nothing owed".
3. **What goes where.** In the subdirectory: the owner lock, the ledger, the markers, the
   generation reservations, the part, `known_hosts`, the private key (the configured
   `privateKey` must lie in it), `diagnostics.log` and the drainer's log (order 146). In
   `CONDUIT_STATE_DIR`: the runs' standard-error files under `stderr/`, which can hold what a run
   printed and so must go when the instance is removed, and nothing the drainer needs. Name each
   file's place in the README. The subdirectory's path is too long for an SSH control socket, so
   the sockets always go to the private temporary directory that order 137 falls back to
   (`src/backend.rs`, near line 164): say so, and correct the README's sentence that they go in
   the state directory where they fit.
4. **Refusals, each naming the place and, where it helps the operator, the expected path.** The
   provider refuses to start where `durableDir` is missing, relative, a path that the crate's
   `-o` path writer refuses (`codec::option_path`), not a directory, not owned by the provider's
   user, not at mode 0700, inside `CONDUIT_STATE_DIR`, or inside that directory's parent (another
   instance's removal could delete it). A refusal about the private
   key names the subdirectory where the key must lie. The README gives the hash's input exactly,
   so an operator can compute the subdirectory and place the key before the first start.
5. **No silent migration.** Where `CONDUIT_STATE_DIR` holds a ledger, markers, generation
   reservations, a part or `known_hosts` from orders 137 and 140's layout, the provider refuses to
   start, naming each, and the README says to move them into the subdirectory: owed stops must never
   be dropped, and an old part must still be swept.
6. **The constructor, for order 146.** The backend's constructor takes the subdirectory and, as
   an option, the state directory. Only when the state directory is given does it make
   `stderr/` there and run decision 4's placement checks and decision 5's old-layout check;
   without it, it never reads, creates or canonicalizes anything under `CONDUIT_STATE_DIR`, so
   that the drainer can build the backend after the instance was removed. It accepts an owner
   lock that its caller already holds, as well as taking the lock itself.
7. **Its contract text,** in `contracts.yaml` and the README: the durable directory, that it is
   kept outside the engine's lifecycle and why (PRV-021, the admin's decision), that the operator
   creates and removes it, what each subdirectory holds and what stays in `CONDUIT_STATE_DIR`,
   and the refusals; the README's `privateKey` row says the key lies in the subdirectory. The
   drainer's text is order 146's.
8. **Tests.** Move the tests to the new layout without losing an assertion: those that build a
   `FileStore` or a backend on the state directory (`tests/support/mod.rs`, near line 38;
   `tests/runtime.rs`, near lines 192, 286 and 296; `tests/configuration.rs`, near line 129) use the
   subdirectory. The fixture's state directory sits at `<temporary directory>/instances/<UUID>`, as
   the engine makes it (`engine/crates/shell/src/lib.rs`, `state_directory`), with `durableDir`
   elsewhere, so that no fixture is refused for sharing a parent; keep a path with a space and a `%`
   on the durable side, so the escaping assertions survive (`tests/runtime.rs`, near lines 189 to
   200). Add: the subdirectory's name and mode, from a given `CONDUIT_STATE_DIR`; the constructor
   without a state directory, with the state directory removed, creating nothing there; two state
   directories under one `durableDir` giving two subdirectories, never sharing a lock; each refusal
   of decision 4; the old layout refused, naming its files; the empty ledger written at first start,
   before `state-dir`; a missing ledger read as an error; `stderr/` in the state directory and
   nothing else there; and `state-dir` holding another value, or present without a ledger, refused.
9. **What this order does not do:** build the drainer (order 146), change the kit, test over a
   real `sshd` or `systemd`, or reach any machine.

## Scope

Change only `packages/provider-ssh-runtime/`.

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release` (a skipped test is untested, never a pass), and
`cargo run --release --bin package` twice, with the same digest. From the repository root:
`python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run these with `bash`.
Each prints nothing when it passes:

```bash
D=packages/provider-ssh-runtime
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rln -E "PRIVATE KEY|BEGIN CERTIFICATE" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

The provider's shell scripts stay POSIX, since the tests run them on macOS too (order 140's
Record). `target/` and `dist/` stay ignored and unstaged. Report each choice you make as your own.
Do not stage anything.

## Record

Run on 2026-11-12 on the Windows computer (WSL), from base 0e95028: the worker (818 s) and a fresh
verifier (135 s), which found nothing to change in one round. The provider's ledger, owner lock,
markers, generation reservations, part, `known_hosts`, key and logs now live in a 0700
subdirectory per instance of the operator's `durableDir`, named by the SHA-256 of the
`CONDUIT_STATE_DIR` value, with `state-dir` recorded at the first start after an empty ledger;
the runs' `stderr/` stays in `CONDUIT_STATE_DIR`. The refusals name each place and the expected
path; the old layout is refused; a missing ledger reads as an error; and the constructor, given
no state directory, touches nothing under it, for order 146's drainer. On the Mac, with two test
threads: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` clean, all 42 tests
pass, and two archive builds give the same digest
(`sha256:0e83b0088edaa00bd74dedd5a0f280a8bc4b48f764630ac7c8634393299899d3`). The lane
coordinator's review found nothing to change.

The worker's findings: the provider is not given its own instance ID, so the subdirectory is named
from `CONDUIT_STATE_DIR`'s value, which PRV-029 makes unique to the instance; and writing an empty
ledger wherever `state-dir` is absent would erase a ledger an operator moved in from the old
layout, so an existing valid ledger is kept and an invalid one refused. Its own choices: a
`src/durable.rs` module with a non-cloneable owner lock for order 146; symbolic-link durable
directories and special permission bits refused; unreadable old-layout entries refused.
