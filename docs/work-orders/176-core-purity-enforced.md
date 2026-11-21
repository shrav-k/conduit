# Work order 176: the core's purity enforced

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only because the
shell's tests in `cargo test --release` bind a loopback port; use it for nothing else, and reach
no other machine. Run one cargo command at a time, and never print a whole large file or log.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 33 of [its orders](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/orders.md))
found that the core is pure but that nothing keeps it that way
([FR-355](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P07-engine-build.md#fr-355)). The core's module
comment (`engine/crates/core/src/lib.rs:1`) rules out the file system, the clock, the network and
processes. The review's grep found no access to a file, the network, a process, the environment,
a thread or a clock, and found `BTreeMap` used throughout. But the engine has no Clippy
configuration, no `forbid` attribute and no check that would refuse a change that breaks this.
Only each order's review keeps the core pure. FR-355's verifier found a blind spot in the grep
itself: it misses a grouped import such as `use std::{..., io::Read}`
(`engine/crates/core/src/archives.rs:9-12`). The verifier agreed that the fix is a Clippy list of
the methods the core may not call.

This order adds that list. The list also names the types and macros that reach the same places.
The order makes any exception to the list an error, and it forbids unsafe code.
[Order 152](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/152-ci-engine-shards-stand-alone.md) gave the engine a CI job of its own, and that
job's `cargo clippy --all-targets -- -D warnings` (`.github/workflows/checks.yml:278-280`) will
apply the list to every pull request. No behavior changes: no function of the core changes, and
the list refuses only what the core does not do today. Line numbers are those of `main` at
`ef980e64`.

## The decisions it applies

1. **A Clippy list for the core alone.** Create `engine/crates/core/clippy.toml`. Clippy uses the
   first `clippy.toml` or `.clippy.toml` it finds, searching from a crate's manifest directory
   upward. Neither exists anywhere in the repository, so this file applies to `conduit-core` and
   to no other crate. The file begins with this comment:

   ```toml
   # conduit-core is pure (src/lib.rs): it reaches no file system, environment, process, network,
   # thread, clock or terminal, and no result of it depends on a random seed. The shell passes in
   # what the core needs. Clippy reads this file for this crate alone, and src/lib.rs forbids
   # allowing these lints. The list names entry points: it cannot see input or output inside a
   # function of a dependency that it does not name.
   ```

   Three arrays follow, in this order: `disallowed-methods`, `disallowed-types` and
   `disallowed-macros`. Each entry is one line, `{ path = "PATH", reason = "REASON" }`, and its
   reason is the one given for its group below. In each array, each group's entries come after a
   one-line comment that names the group. Clippy prints the reason under each refusal, so each
   reason says what the core does instead ([ruling 52](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-52/README.md)). Where a
   line below gives a prefix and then names, each path is the prefix, then `::`, then the name.
   Within each array, the groups come in this order:

   - **File system**: "the core touches no file system: the shell reads files and passes their
     bytes in". Methods: under `std::fs`, `read`, `read_to_string`, `read_dir`, `read_link`,
     `metadata`, `symlink_metadata`, `canonicalize`, `exists`, `write`, `copy`, `rename`,
     `hard_link`, `create_dir`, `create_dir_all`, `remove_file`, `remove_dir`, `remove_dir_all`
     and `set_permissions`; under `std::path::Path`, `exists`, `try_exists`, `is_file`,
     `is_dir`, `is_symlink`, `metadata`, `symlink_metadata`, `canonicalize`, `read_dir` and
     `read_link`. Types: `std::fs::File`, `std::fs::OpenOptions` and `std::fs::DirBuilder`.
   - **The loader's file reads**: "this reads the file system: the core calls only the loader's
     pure functions, such as read_bytes and read_submitted_files". Methods:
     `conduit_loader::read_file`, `conduit_loader::read_project` and
     `conduit_loader::Specification::read` (`engine/crates/loader/src/read.rs:93`,
     `project.rs:79` and `schemas.rs:93`).
   - **Environment and process**: "the core neither reads nor changes its environment, and
     neither runs nor ends a process: the shell passes in what the core needs". Methods: under
     `std::env`, `var`, `var_os`, `vars`, `vars_os`, `args`, `args_os`, `current_dir`,
     `set_current_dir`, `current_exe`, `home_dir` and `temp_dir`; then `std::process::abort`,
     `std::process::exit` and `std::process::id`. Types: `std::process::Command`.
   - **Network**: "the core opens no connection and resolves no name". Methods:
     `std::net::ToSocketAddrs::to_socket_addrs`. Types: under `std::net`, `TcpStream`,
     `TcpListener` and `UdpSocket`.
   - **Threads**: "the core starts no thread, sleeps for nothing and reads no machine fact".
     Methods: under `std::thread`, `spawn`, `scope`, `sleep` and `available_parallelism`. Types:
     `std::thread::Builder`.
   - **Clock**: "the core reads no clock: the time arrives in Evidence.time". Methods:
     `std::time::SystemTime::now`, `std::time::Instant::now`, `chrono::Utc::now` and
     `chrono::Local::now`. Types: `std::time::SystemTime` and `std::time::Instant`.
   - **Terminal**: "the core neither prints nor reads a terminal: it returns its results and
     diagnostics to the shell". Methods: under `std::io`, `stdin`, `stdout` and `stderr`.
     Macros: under `std`, `print`, `println`, `eprint`, `eprintln` and `dbg`.
   - **Random order**: "a random seed changes the order from run to run: the core uses BTreeMap
     and BTreeSet". Types: `std::collections::HashMap`, `std::collections::HashSet` and
     `std::hash::RandomState`.

   That makes 57 methods, 13 types and 5 macros, 75 entries in all.

2. **The two chrono entries may not resolve.** `chrono::Utc::now` and `chrono::Local::now` exist
   only when chrono's `clock` feature is on. The core leaves it off
   (`engine/crates/core/Cargo.toml:23`), but the shell turns it on
   (`engine/crates/shell/Cargo.toml:19`). Cargo unifies features when it builds the whole
   workspace, so in that build the core could call either one. Each of the two entries carries a
   third key inside its braces, `{ path = "chrono::Utc::now", reason = "REASON", allow-invalid =
   true }`, and the same for `chrono::Local::now`, so that a check of the core alone, where
   neither path exists, prints no warning. They come last in the clock group of
   `disallowed-methods`, after this comment:

   ```toml
   # chrono's now exists only when another member of the workspace, the shell, turns on chrono's
   # clock feature; allow-invalid keeps a check of this crate alone free of warnings.
   ```

   No other entry has `allow-invalid`. If Clippy warns that some other listed path does not
   resolve, drop that entry, quote the warning in your report, and lower the count of 75 in Done
   when by one for each entry dropped.

3. **No exception, and no unsafe code.** In `engine/crates/core/src/lib.rs`, add these lines after
   the module comment (lines 1 to 3) and before the blank line above `pub mod access;`:

   ```rust
   //! Clippy enforces this: `clippy.toml`, beside this crate's `Cargo.toml`, names the functions,
   //! types and macros that reach outside the core, and the attributes below forbid allowing them.
   //! Unsafe code is forbidden too, so that no foreign function can be declared.
   #![forbid(unsafe_code)]
   #![forbid(clippy::disallowed_macros, clippy::disallowed_methods, clippy::disallowed_types)]
   ```

   With `forbid`, a listed use is an error even without `-D warnings`. An `allow` or an `expect`
   of any of the three lints anywhere in the core is then an error too (E0453). Edition 2024
   requires every `extern` block to be `unsafe extern`, and `unsafe_code` refuses that, so the
   core cannot declare a foreign function to reach the outside through. The core has no unsafe
   code today.

4. **What stays.**
   - No other line of the core changes. If Clippy refuses a line of the core as it is on `main`,
     change neither the line nor the list: report the line and the lint. FR-355's grep found no
     such line, and neither did the coordinator's.
   - These stay allowed, because none of them reaches outside the core:
     - `std::io::Read` on bytes held in memory (`archives.rs:9-12`);
     - `PathBuf` used for string work (`packages.rs:740`);
     - `url::Url::to_file_path` (`requests.rs:1143`);
     - `Cow` and `OnceCell` (`access.rs:6`);
     - the `OnceLock` that caches the names `include_bytes!` embeds from the specification at
       build time (`lib.rs:29-33`; `engine/CONFORMANCE.md:39` says a rename requires
       rebuilding).

     That `to_file_path` gives different results on different platforms is a caveat of FR-355's
     verifier, and this order does not take it up.
   - The list names entry points, not every route. Review still has to cover input or output
     inside a dependency's function that the list does not name, any new dependency of the core,
     a platform's module under `std::os`, and any file the core embeds when it is built, as
     `include_bytes!` embeds the names. Decision 5 says so in the engine's README.
   - The loader, the conformance runner and the shell get no list. `.github/workflows/checks.yml`,
     every `Cargo.toml` and every `Cargo.lock` stay as they are. CI does not build the core's
     `differential` feature; Done when checks it once.

5. **The engine's README says how the boundary is kept.** In `engine/README.md`, under "The
   core's boundary", add this paragraph after the one that ends "before exposing a management
   operation." (line 41), wrapped at no more than 100 characters a line:

   > `cargo clippy` enforces this boundary. The core's clippy.toml names the functions, types and
   > macros through which code reaches a file, the environment, a process, the network, a thread,
   > a clock or the terminal, or depends on a random seed; Clippy applies it to `conduit-core`
   > alone, and the core's `lib.rs` forbids allowing those lints. It forbids unsafe code too, so
   > that no foreign function can be declared. The list names entry points only: input or output
   > inside a dependency's function that it does not name, a new dependency of the core, a module
   > under `std::os`, and the files the core embeds when it is built are left to review.

   In that paragraph, "clippy.toml" is a Markdown link to `crates/core/clippy.toml`, relative to
   the README. Nothing else in the README changes.

## Scope

Change only these three files:

- `engine/crates/core/clippy.toml`, which is new;
- `engine/crates/core/src/lib.rs`, as decision 3 says;
- `engine/README.md`, as decision 5 says.

## Done when

From `engine/`, each of these passes. Run them one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo clippy -p conduit-core --all-targets -- -D warnings
cargo clippy -p conduit-core --all-targets --features differential -- -D warnings
cargo test --release
```

If the fourth fails with any error other than this order's own refusals ("use of a disallowed",
E0453 or `unsafe_code`), such as a lint or a compile error in code CI never builds, report the
first such error and its line, and change nothing for it. Then, from `engine/`, these four
commands print 75, 2, 2 and 0:

```bash
grep -c 'path = "' crates/core/clippy.toml
grep -c 'allow-invalid = true' crates/core/clippy.toml
grep -c -E '^#!\[forbid\((unsafe_code|clippy::disallowed_macros, clippy::disallowed_methods, clippy::disallowed_types)\)\]$' crates/core/src/lib.rs
cargo clippy --color never -p conduit-core --all-targets -- -D warnings 2>&1 | grep -c '^warning'
```

The last one shows that every listed path resolves.

Next, show three things: the list refuses what it names, no exception to it can be made, and
without it no other check refuses the same code. Write `engine/target/purity-probe.rs` with
exactly these lines. The probe is compiled but never run:

```rust
pub fn purity_probe() {
    let _ = std::fs::read("probe");
    let _ = conduit_loader::read_file(std::path::Path::new("probe"));
    let _ = std::path::Path::new("probe").exists();
    let _ = std::env::current_dir();
    let _ = std::process::Command::new("probe");
    let _ = std::net::TcpStream::connect("127.0.0.1:9");
    let _ = std::thread::spawn(|| ());
    let _ = std::time::SystemTime::now();
    let _ = std::collections::HashMap::<u8, u8>::new();
    println!("probe");
}
```

Then, from `engine/`, run these one at a time:

```bash
cp crates/core/src/lib.rs target/lib.rs.fixed
cp crates/core/clippy.toml target/clippy.toml.fixed
cat target/lib.rs.fixed target/purity-probe.rs > crates/core/src/lib.rs
cargo clippy --color never -p conduit-core --all-targets -- -D warnings > target/probe-listed.txt 2>&1; echo "exit $?"
grep -A1 '^error: use of a disallowed' target/probe-listed.txt | grep -o -E 'lib\.rs:[0-9]+' | sort -u | wc -l
grep -c 'note: the core touches no file system' target/probe-listed.txt
{ cat target/lib.rs.fixed; echo '#[allow(clippy::disallowed_methods)]'; cat target/purity-probe.rs; } > crates/core/src/lib.rs
cargo clippy --color never -p conduit-core --all-targets -- -D warnings > target/probe-allowed.txt 2>&1; echo "exit $?"
grep -c '^error\[E0453\]' target/probe-allowed.txt
rm crates/core/clippy.toml
cat target/lib.rs.fixed target/purity-probe.rs > crates/core/src/lib.rs
cargo clippy -p conduit-core --all-targets -- -D warnings
cp target/lib.rs.fixed crates/core/src/lib.rs
cp target/clippy.toml.fixed crates/core/clippy.toml
cargo clippy -p conduit-core --all-targets -- -D warnings
```

The expected results:

- **With the list:** Clippy exits non-zero. The count of refused lines is 10, one for each line of
  the probe, and the file system's reason is printed at least once. If decision 2 made you drop
  an entry, the count is lower by the probe lines that only that entry refused; name them.
- **With an `allow`:** Clippy exits non-zero, and E0453 is counted at least once.
- **With the list removed:** the probe passes, so no other lint refuses it.
- **After the restore:** the last command passes.

Restore with `cp`, not `mv`. A moved file keeps its older modification time, so cargo would not
recheck the crate. Git ignores `target/`. Report each exit status and each count, and the first
line of `target/probe-listed.txt` that begins `error: use of a disallowed`. After that,
`git status --short` must list only the three files in scope.

Then, from the repository root, each of these passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each of these prints 1. The commands that start with `tr` read the file with its line breaks
turned into spaces, so that rewrapping does not hide a phrase:

```bash
grep -c 'crates/core/clippy\.toml' engine/README.md
tr -s ' \n' '  ' < engine/README.md | grep -c '`cargo clippy` enforces this boundary\.'
tr -s ' \n' '  ' < engine/README.md | grep -c 'a new dependency of the core, a module under `std::os`, and the files the core embeds when it is built are left to review\.'
```

Before the pull request merges, the admin has a fresh read-only Opus reader read the diff
(ruling 79). The admin also shows the engine build, the four engine shards and the doctests
passing on the pull request, on Linux. Do not stage anything. Report each choice you make as your
own, and every check's last line.

## Record

Run on 2026-11-15 on the Mac from `main` `5f9fcbe8`: the worker (829 s) and a fresh Codex verifier
(204 s), which found nothing to change in one round but did not rerun the build or the tests; it
reran `cargo fmt --check` and the docs, private-path and whitespace checks, and checked the build
and probe results against the worker's saved logs. The new `engine/crates/core/clippy.toml` names 57
methods, 13 types and 5 macros in eight groups, each with the reason Clippy prints under its
refusal. Only the two chrono entries carry `allow-invalid`, and no entry was dropped. The core's
`lib.rs` forbids unsafe code and forbids allowing the three lints, and the engine's README says how
the boundary is kept and what is left to review. No other line of the core changed. The pinned
rustfmt wraps the second `forbid` over several lines, so the order's exact-line grep prints 1, not
2; both attributes are in force. The worker's probe was refused on all 10 of its lines, an `allow`
failed with E0453, and with the list removed the probe passed. A fresh read-only Opus reader then
verified the order (ruling 79, point 2). With a target directory of its own it reran `cargo fmt
--check`, both Clippy commands, the core with `--features differential`, the core's release tests
(22 unit tests passed; the core has no doctest) and `cargo check` with warnings denied, then the
docs, private-path and whitespace checks. It probed every entry and several ways around the list,
such as an alias, a function pointer and a module-level `allow`, and each was refused. It found no
must. Both of its shoulds were applied before merge. The Network and Threads reasons now also say
what the shell does instead: "the shell connects and passes in what it receives" and "the shell runs
the work and passes machine evidence in". Three routes that the reasons already ruled out but Clippy
let through are now listed: `std::thread::park_timeout` with the threads, and `env!` and
`option_env!`, which read the build machine's environment, in a new environment group at the head of
the macros. The list now has 58 methods, 13 types and 7 macros, 78 entries in all, so the order's
first count prints 78. In a scratch copy, a probe of each of the three was refused by its path, with
its reason. Clippy now refuses all three, so the README need not name them as left to review. The
one route the reader found that the list does not name, `std::os::unix::fs::symlink`, is under
`std::os`, which the README already leaves to review, so that list is unchanged. With the fix, both
Clippy commands passed with no warning, the core's 22 unit tests passed in a release build with two
test threads, and `cargo fmt --check` passed.

A second fresh Opus reader read the coordinator's fixes (ruling 79, point 2) and found no must. Its
two wording points on this Record were applied. Its third should, two more deprecated routes of the
same kind (`std::thread::sleep_ms` and `std::thread::park_timeout_ms`, which an `allow(deprecated)`
lets through), is left to a follow-up order with the counts it changes.
