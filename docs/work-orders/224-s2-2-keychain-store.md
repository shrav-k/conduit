# Work order 224: S2-2, the macOS keychain secret store as a package, and the engine's check of a secret store's Type

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each case you add, with its promises and `passes`.
This order needs no network: it fetches nothing, and opens only the loopback ports the engine's
tests bind. Run one cargo command at a time. Never print a whole large file, case, fixture or
log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Every secret in this order is
a synthetic marker that a test makes at run time and never prints. Do not run `conduit`,
`codex`, `claude` or the dogfood harness, and sign in nowhere.

**The keychain rule, the owner's, without exception.** Nothing in this order touches the login
keychain or any keychain the user has. Every keychain a test uses is a throwaway it creates under
its own temporary directory and removes; a test that would touch another must not exist. Run
`/usr/bin/security` only through the tests, never by hand; never open Keychain Access. If a
dialog ever appears, or the user's keychain search list or default keychain changes, stop at
once, change nothing more and report. A `security` or `codesign` call that reaches its 30-second
limit counts as a dialog. Never run `security` on a throwaway while it is locked.

## What and why

S2's stream, step S2-2. Order 218 (S2-1) built the engine's read of a secret through the
instance's own Conduit, with the file store `packages/provider-file`; the read path is not
store-specific. S2's store is the macOS keychain
([experiment 013](../../experiments/013-s2-shared-login/README.md), O4 "the keychain alone" and
O5 "the owner's logged-in session only, with no claim about a daemon";
[IN_FORCE.md, topic 12](../../IN_FORCE.md#12-secrets), 46 point 5), and no package serves it.
It runs ahead of S2-O and A63-OKC, on throwaway keychains of its own, and leaves the
rebuilt-binary test (the coordinator's reading, open to the owner's strike).

Order 218's "What this leaves" names this step: SEC-006 for the keychain store, and SEC-005 and
SEC-022 checked at installation, which the engine does not do today.

The package is A63-KC of [the provider plan](../implementation/PROVIDERS.md); this order applies
the findings and [safety rules](../../experiments/005-macos-keychain/README.md#safety) of
[experiment 005](../../experiments/005-macos-keychain/README.md).

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
3, a shared thing is owned once and never copied to an Agent (S2's service key stays in one
keychain item, and no Agent home holds it); and 2, every access is a relationship that can be
inspected and revoked (a secret store's Type with no control and no other operation gives a
value only through one Conduit that can be closed).

**Should this exist** (point 3): the store is Apple's; the owner stores and authorizes the item
with Apple's own tools. The package adds no store: it reads one generic password by service and
account, the item `security find-generic-password -s <service> -a <account>` names, but with
interaction off, which that tool cannot do (005), behind the existing `secret-provider.v1`
(ruling 40).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for
under 2 hours), a Codex verifier and a Claude reader, then the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how is yours.

**Stop rule.** Change nothing under `spec/`. Where a promise needs a choice `spec/` does not
make, stop, change nothing further, and report the promise, the choice and the options. Stop and
report also where a crate is missing; where a throwaway keychain's read cannot succeed with
interaction disabled and a cleared environment (give the status codes); where a line of
`by-rule-outcomes.txt` other than SEC-022's changes; and under the keychain rule. A choice no
promise governs is yours: list it under `own_choices`.

## The decisions

1. **The package.** `packages/provider-keychain/`, in the form of `packages/provider-file/`: its
   own `Cargo.lock`, `rust-toolchain.toml` at 1.97.1, `.gitignore`, `conduit.yaml`,
   `contracts.yaml`, `README.md`, `src/bin/package.rs` building `dist/provider-keychain.tar.gz`
   and its digest, `tests/package.rs` and `tests/schema_drift.rs`. Address
   `example.org/conduit/provider-keychain`, version 0.1.0, one Resource Type
   `keychain-secret.v1` that offers only `github.com/shrav-k/conduit/core#secret-provider.v1`,
   lists no control, implements `get-secret.v1` by `bin/provider-keychain`, and records neither
   inputs nor outputs. Copy and adapt provider-file's MCP surface (discovery, the extension
   check, the `_meta` checks and configuration, the filter refusal), which order 218 checked
   against the engine. Start the lock from a copy of provider-file's; resolving offline then adds
   only `security-framework` 3.7.0, `security-framework-sys` 2.17.0 and `core-foundation` 0.10.1.
2. **The locator.** The configuration is `{keychain, service, account}`, all required, no other
   key: `keychain` an absolute path to one keychain file; `service` and `account` non-empty
   strings, the generic password item's own attributes. A read takes the one generic password
   item with that service and account from that file alone: never the default keychain, the
   search list, another item class or another file, and nothing from the configuration's values
   or the environment. Where the path names no regular file, the read fails before any keychain
   is opened.
3. **How it reads.** Through Apple's Security framework, with `security-framework` =3.7.0 and
   `core-foundation` 0.10.1 (experiment 005's crates), as macOS-only dependencies; not through
   `/usr/bin/security`, which cannot disable user interaction (005, "Not tested, and why"). For
   each read: disable user interaction and confirm it is off, or fail having opened nothing; open
   the file with `SecKeychain::open`; read with the opened keychain's own
   `find_generic_password`; where the read is refused, ask `SecKeychainGetStatus` (declared as
   005's `src/lib.rs` does) whether the keychain is locked. Nothing is cached: each read opens
   the file again. The value is the item's bytes, UTF-8, at most 65,536 bytes, nothing trimmed.
4. **Failed reads** (SEC-006). A missing or non-regular file, a locked keychain, a missing item,
   a refused read (the binary not trusted), a non-UTF-8 or oversized value, or user interaction
   that cannot be disabled: each gives the outcome failed, no value, and a fixed message saying
   the keychain store could not be read and why, with no locator and never the system's own text
   (005: -25293's misleads). No other store or item is tried. Off macOS the binary builds, serves
   discovery, and answers every read failed, saying the store runs only on macOS.
5. **The README**, in provider-file's form with its D15 inventory, states: the locator; the
   logged-in session only (O5), no daemon claim (A22 is open); interaction off, so no read waits
   for a person; no cache, no logged value; the owner's authorization, from 005 (the item added
   with `-T <the binary>` and, in the login keychain's format, the binary's `cdhash:` on its
   partition list); that a rebuild changing the code-directory hash loses access (005, case 4),
   so each upgrade repeats the owner's step A63-OKC; and that authorizing the copy the engine
   installs, at its own path, is not tested here.
6. **The engine checks SEC-005 and SEC-022** where it checks CTR-013, in `resolve` in
   `engine/crates/core/src/packages.rs`, which runs both when a package is installed
   (`archives.rs`) and when a project is submitted. A Type whose `offers` resolves to the core
   package's `secret-provider.v1`, compared as names resolve and not as text, is rejected with a
   diagnostic of `ruleId` SEC-005 naming the type and each control it lists, and with one of
   `ruleId` SEC-022 naming the type and its capabilities where it offers another; a unit test
   shows the messages name them. The by-rule line of
   `SEC-022-type-of-a-secret-resource-that-offers-another-capability.yaml` becomes
   `named invalid SEC-022`, and no other line changes.

## The tests

**The keychain tests**, `packages/provider-keychain/tests/keychain.rs`, compiled on macOS alone.
They drive `bin/provider-keychain` over stdio with a cleared environment, as the engine's runner
does, with the `_meta` the engine sends, as provider-file's `tests/mcp.rs` does. Every test
keeps these rules, through shared helpers:

- The test process disables user interaction once, before any keychain call, and holds that
  guard until it exits (never dropped). It creates each throwaway with `CreateOptions`
  (`SecKeychainCreate`, which adds nothing to the search list; 005, "Safety"), with a password
  generated at run time, under its own `tempfile` directory, and automatic locking off. It locks
  and unlocks a throwaway only through its own handle, unlocking only with its password
  (`unlock(Some(..))`); at the end it locks it and removes the file, never with
  `SecKeychainDelete`.
- It runs `/usr/bin/security` and `/usr/bin/codesign` through one function, with stdin closed,
  a 30-second limit and no argument ever printed, which runs `security` only as
  `add-generic-password` or `set-generic-password-partition-list` whose last argument is the
  path of a throwaway this test created, or as exactly `list-keychains -d user` or
  `default-keychain -d user`, and `codesign` only as `-dv --verbose=4 <the provider binary>`,
  and refuses anything else. No throwaway's file name contains `login`.
- It records those two commands' output before it creates anything and requires each to be the
  same, byte for byte, at the end.
- No marker is printed anywhere; values are compared by digest. Each provider's stderr is
  empty, and each call answers within 5 seconds.

At least these tests, by these names:

- `reads_the_named_item_and_reads_again_each_time`: a keychain in the older format (its path
  does not contain `/Library/Keychains`), an item added with `-T` naming the provider binary
  and `/usr/bin/security`: the read succeeds with the marker; with the item changed in place
  (`add-generic-password -U`, with no `-T`), the next read gets the new value.
- `partition_format_needs_the_binary_on_the_partition_list`: a keychain under
  `<temporary>/Library/Keychains/`, the login keychain's format. With `-T` alone the read fails;
  with the binary's `cdhash:` (from `codesign -dv --verbose=4`) added to the item's partition
  list with the throwaway's password, it succeeds. This rehearses A63-OKC.
- `a_locked_keychain_fails_at_once_and_tries_no_other_store`: a second throwaway, unlocked,
  holds an item with the same service and account and another marker. The named one locked:
  failed, its message saying the keychain is locked and holding neither marker. Unlocked again,
  the read gives the first marker.
- `missing_untrusted_and_unreadable_reads_fail_with_no_value`: a path to no file, a directory, a
  missing item, an item added with no `-T` naming the binary, a non-UTF-8 value, and a value
  of 65,537 bytes (both by `-X`): each failed, no value, neither marker in any message, and the
  other throwaway's item never returned.

**The tests on every system**, in `tests/mcp.rs`: discovery and the extension refusal; the
filter refusal before any read; a configuration that is a string, lacks or adds a key, or has a
relative path or an empty service or account: failed, no value, no keychain opened; and, off
macOS, a well-formed read failed. `tests/schema_drift.rs` also asserts that the Type offers exactly
`secret-provider.v1` and lists no control. `tests/package.rs` checks the archive as
provider-file's does.

**Portable cases** under `conformance/language/contracts/`, named by you, with entries in its
`index.yaml`, by `resolve-project`, none an expected failure: a Type offering
`secret-provider.v1` and listing a control, refused with `ruleId` SEC-005; one offering it and
another capability, refused with `ruleId` SEC-022; one offering it alone, accepted. Order 218's
synthetic secret Type (`conformance/conventions/secret-reads/`) is a model. Write
`docs/work-orders/224-declared.yaml` in the form of [218's](218-declared.yaml), declaring each
case added.

**What follows.** In `engine/CONFORMANCE.md`, after "and other stores have no qualification
here.", one sentence beginning "From order 224," saying that installation and submission refuse a
secret store's Type that lists a control or offers another capability (SEC-005, SEC-022), and
that `packages/provider-keychain` is tested as a package alone, not yet through the engine. Add
this row to `RUST_READS` in `tools/gate.py`, as `tools/tests/test_gate.py` requires of a package
whose tests read `spec/`:
`('packages/provider-keychain', 'spec/', 'tests/schema_drift.rs', '"../../spec'),`. Run
`python3 tools/promise_standing.py --write`.

## What this leaves

Report this list, corrected by what you built: the engine's reads through the keychain store,
with the owner's throwaway keychain and authorization (S2-O, A63-OKC), in S2-3's dry run; the
installed copy authorized at its own path; a rebuilt binary's loss of access, tested; a daemon
session (A22); S2's V3 and V4 (S2-3); the store suite A63-VS.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor d92767dd HEAD; echo "main $?"
test -e packages/provider-keychain; echo "package $?"
grep -c '^SEC-022-type-of-a-secret-resource-that-offers-another-capability.yaml accepted succeeded -$' engine/crates/shell/tests/expected/by-rule-outcomes.txt
grep -rlE 'SEC-005|SEC-022' engine/crates/core/src engine/crates/loader/src | wc -l | tr -d ' '
grep -E '^\| SEC-0(05|06|22) \|' engine/STANDING.md | grep -c '| no case, no reason |$'
grep -c 'provider-keychain' tools/gate.py
grep -c 'and other stores have no qualification here' engine/CONFORMANCE.md
d=$(mktemp -d) && cp packages/provider-file/Cargo.toml packages/provider-file/Cargo.lock packages/provider-file/rust-toolchain.toml "$d"/ && mkdir "$d/src" && echo 'fn main() {}' > "$d/src/main.rs" && printf '\n[target.%s.dependencies]\ncore-foundation = "0.10.1"\nsecurity-framework = "=3.7.0"\n' "'cfg(target_os = \"macos\")'" >> "$d/Cargo.toml" && (cd "$d" && cargo metadata --offline --format-version 1 >/dev/null 2>&1); echo "offline $?"; rm -rf "$d"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `package 1`, `1`, `0`, `3`, `0`, `1`, `offline 0` and `0`; `offline`
printing otherwise means a crate is missing from the cache.

## Scope

Create `packages/provider-keychain/` and `docs/work-orders/224-declared.yaml`. Change
`engine/crates/core/src/packages.rs` (or a core module of your own that it calls), the new cases
and their entries under `conformance/language/contracts/`, the SEC-022 line of
`engine/crates/shell/tests/expected/by-rule-outcomes.txt`, `engine/CONFORMANCE.md` as above,
`engine/STANDING.md` only by the tool, and `RUST_READS` in `tools/gate.py`. Change nothing
under `spec/`, `experiments/`, `packages/provider-file/` or any other package, `docs/` outside
the declaration, `.github/`, `IN_FORCE.md`, `ROADMAP.md`, `engine/Cargo.lock` or any
`Cargo.toml` outside the new package. Anything else that needs changing goes in your report.

## Done when

Each line marked "fails today" prints otherwise on `main` at `d92767dd`; all pass after.

```sh
(cd packages/provider-keychain && cargo metadata --offline --locked --format-version 1 >/dev/null); echo "offline $?"  # offline 0; fails today (1)
(cd packages/provider-keychain && cargo fmt --check)  # passes; fails today
(cd packages/provider-keychain && cargo clippy --all-targets -- -D warnings)  # passes; fails today
(cd packages/provider-keychain && cargo clippy --all-targets --target x86_64-unknown-linux-gnu -- -D warnings)  # passes, as CI's ubuntu job builds it; fails today
(cd packages/provider-keychain && cargo test --release)  # passes, the keychain tests among them; fails today
(cd packages/provider-keychain && cargo test --release --test keychain -- --list 2>/dev/null | grep -c ': test$')  # 4 or more; fails today (prints nothing)
(cd packages/provider-keychain && grep -rlE 'passwords::|SecKeychain::default|ItemSearchOptions|SecKeychain(CopyDefault|SetDefault|SetSearchList|Delete|LockAll)|login\.keychain|(find|delete)-generic-password|(create|delete|unlock)-keychain|dump-keychain|(list-keychains|default-keychain).*"-s"' src tests | wc -l | tr -d ' ')  # 0; fails today (prints nothing)
(cd packages/provider-keychain && grep -rl '"list-keychains", "-d", "user"' tests | wc -l | tr -d ' ')  # 1 or more; fails today (prints nothing)
(cd packages/provider-keychain && grep -rhoE '"(add|set|find|delete|create|unlock|lock|dump|list|default|show|import|export|login)-[a-z-]+"' tests | LC_ALL=C sort -u | tr '\n' ' ')  # "add-generic-password" "default-keychain" "list-keychains" "set-generic-password-partition-list" ; fails today (prints nothing)
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, once, at the end; by_rule among it
git add -N packages/provider-keychain conformance docs/work-orders/224-declared.yaml  # so the checks see new files
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_rulings.py  # passes
python3 tools/set_names.py --check  # passes
python3 tools/check_schemas.py  # Errors: 0
python3 tools/check_roadmap.py  # passes
python3 tools/check_model_account.py  # passes
python3 tools/promise_standing.py --check  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 -m unittest discover -s tools/tests  # OK
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/224-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; exit 0
git diff --check  # prints nothing
git diff --name-only HEAD -- spec experiments packages ':(exclude)packages/provider-keychain' docs ':(exclude)docs/work-orders/224-declared.yaml' .github IN_FORCE.md ROADMAP.md engine/Cargo.lock | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- '*Cargo.toml' ':(exclude)packages/provider-keychain' | wc -l | tr -d ' '  # 0
grep -c '^SEC-022-type-of-a-secret-resource-that-offers-another-capability.yaml named invalid SEC-022$' engine/crates/shell/tests/expected/by-rule-outcomes.txt  # 1; fails today (0)
git diff -U0 HEAD -- engine/crates/shell/tests/expected/by-rule-outcomes.txt | grep -c '^[-+][A-Za-z]'  # 2, that line alone; fails today (0)
grep -E '^\| SEC-0(05|22) \|' engine/STANDING.md | grep -c '| case passes |$'  # 2; fails today (0)
grep -c "('packages/provider-keychain', 'spec/'" tools/gate.py  # 1; fails today (0)
grep -c 'From order 224,' engine/CONFORMANCE.md  # 1; fails today (0)
git reset -q -- packages/provider-keychain conformance docs/work-orders/224-declared.yaml  # stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives, per decision, what
you built and the tests and cases that show it; the message of each failed read; your own choices
under `own_choices`; the vector delta's counts; the corrected list of what this leaves; and every
check's last line. Stage nothing. The coordinator runs the gate, has the diff read, and merges.

## Record

Not yet run.
