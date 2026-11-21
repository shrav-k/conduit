# Work order 236: two follow-ups: `provider-local-process` keeps its owed stops where its removal does not reach (PRV-021), and SEC-024's loopback hosts pinned as written

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports its tests bind. Every crate it builds is in the local cache already; if a build asks for one
that is not, stop and report, and fetch nothing. Run one cargo command at a time. Never print a
whole large file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential, even one a test made. Do not run `conduit` outside
a test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Every file
you write is public: write no absolute local path, no user or machine name and no private
repository name.

## What and why

**Two follow-ups, found on 2026-11-20.**

1. *PRV-021 past the provider's removal.* `packages/provider-local-process` keeps its ledger (run
   records and owed stops), its markers and its owner lock in the `CONDUIT_STATE_DIR` it starts
   with. The engine's own runner always runs it (PRV-002) and gives it that directory under
   PRV-029, which the removal of the provider's Resource removes with its contents (LIF-035). A
   stop still owed then, as where a run's guard died and its program survived (the README's
   "Processes, credentials and retained state"), is lost. PRV-021 asks for a place that "neither a
   restart of the engine, on its machine or on another, nor the provider's own retirement loses".
   Orders [221](221-engine-runtime-provider-local-process.md) and
   [226](226-engine-runtime-provider-local-process-qualified.md) carried it as a finding.
2. *SEC-024's loopback hosts.* `secret_transport` in `engine/crates/core/src/packages.rs` (order
   [227](227-s2-3-dry-run.md)) reads the host as the URL writes it, so `http://127.1:8080/` and
   `http://0x7f000001/` are refused, and its test lists `http://127.1/mcp` as refused. 227's reader
   called it "the safe side" and left it to a follow-up.

**What `spec/` settles.** Part 1: PRV-021 gives the place to the provider's contract and keeps the
engine out: "The engine keeps no memory of a run once its instance is removed", and a contract
that holds through retirement "keeps nothing on the engine's side". So the engine keeps nothing
and changes nothing. The SSH runtime provider made this choice under the admin's decision of
2026-11-12 under PRV-021 ([order 145](145-ssh-r-durable-dir.md)): a `durableDir` that its
configuration names, outside the engine's store. This order applies it to local-process.

Part 2: SEC-024 says "The scheme and the host are compared without regard to case, as RFC 3986
has them, and no other host counts, `localhost.example.com` and a name that resolves to a
loopback address among them. The URL is compared as the Remote writes it." `spec/schema-profile.md`'s
row for a `remote` URL says the same. RFC 3986's IPv4address is four decimal octets ("only the
dotted-decimal form of four octets is allowed by this grammar", its section 3.2.2); `127.1` and
`0x7f000001` are reg-names there. `url::Url` parses a host as the WHATWG URL Standard does, which
reads both as 127.0.0.1, so deciding on `Url::host()` would admit hosts that SEC-024 says do not
count. The check is right. This order pins its reading with a test and a comment.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests);
ruling 92, point 1): part 1 serves 6, nothing repeats an effect implicitly: the owed stop is no
longer lost with the state directory, but kept for the operator; whether a later Resource refuses
a new run beside it (the owner's rule MC4-002, in the provider's README) is the owner's choice,
under "What follows". Part 2:
none at risk; a secret header still crosses only `https`, or `http` to a loopback host written as
SEC-024 names it. No promise or case changes.

**Should this exist** (point 3).

- Part 1, yes, as existing practice: an agent on a machine keeps its durable state in a data
  directory its configuration names, as a Nomad client keeps `data_dir` and the kubelet its
  `--root-dir`, and as this repository's SSH runtime provider does already. Not taken: the engine
  keeping the stops (PRV-021 keeps it out); a fixed directory under the user's home (shared by
  every test and every Resource, and named in no document); a drainer that runs after retirement
  ([order 146](146-ssh-r-drainer.md) built SSH-R's; local-process's is RP2's, decision 4).
- Part 2: the change the finding asked for, no: it would contradict SEC-024. A test and a
  three-line comment, yes, so that no later order "fixes" a check that is right.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for under
2 hours), a Codex verifier and a Claude reader, who run only what settles a doubt, and the gate,
which runs everything (DEC-030, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The field's name, the test names and the cases are the
coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a promise's wording needs a choice that `spec/` does not make: report the promise, the choice
  and the options; do not choose;
- a correct result needs a change outside the scope: to the kit, `packages/provider-ssh-runtime`,
  another engine file, or a `Cargo.toml` or lockfile;
- decision 5's test cannot show the owed stop outliving the directory without a sleep or retry
  beyond the kit's own drain and the waits `tests/runtime.rs` already uses;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

### Part 1: local-process's durable directory

1. **The place** (PRV-021, as order 145's decisions 1 and 2 apply it). A new required
   configuration place, `durableDir`, bound to `LOCAL_PROCESS_DURABLE_DIR` in `conduit.yaml`: a
   string, an absolute path (`pattern: '^/'`), with no default, in the Type schema's `required`
   (MAN-017). Under it, one subdirectory per provider instance at mode 0700, named by the lowercase
   hex SHA-256 of the bytes of the `CONDUIT_STATE_DIR` value given, as given, with no trailing
   newline; PRV-029 makes that path unique to the instance and never reuses it. Follow order 145's
   decision 2 at startup: make or open the subdirectory, take the owner lock, then check its
   `state-dir` file (written after an empty ledger at the first start; another value, or the file
   without a ledger, refuses), and a missing ledger is an error, never "nothing owed".
2. **What goes where.** In the subdirectory: `owner.lock`, `state-dir`, `ledger.json` and
   `markers/`, everything an owed stop needs to be inventoried and carried out. In
   `CONDUIT_STATE_DIR`: `runs/` (identity files and `stderr.log`) and `instances/` (the runs' own
   state directories), which hold credentials and so go when the provider's Resource is removed;
   PRV-020 allows that only once no instance names the provider, so no run of theirs follows.
   The guard is given its marker in the subdirectory.
3. **Refusals at startup,** each exiting non-zero with one stderr line that names the place and
   the variable, never a value of a secret: `durableDir` missing, empty, relative, not a
   directory, not owned by the provider's user, not at mode 0700, inside `CONDUIT_STATE_DIR`, or
   inside that directory's parent (another instance's removal could delete it); and the old
   layout: where `CONDUIT_STATE_DIR` holds `ledger.json` or `markers/`, refuse, naming each, so
   no owed stop is dropped (order 145's decision 5). Hash the raw value; compare containment by
   canonical forms, since macOS reaches `/var` through `/private/var`.
4. **The contract,** in `contracts.yaml`'s description and the README (its "Start and
   configuration", the configuration table, "Processes, credentials and retained state", the D15
   inventory's rows (c) and (d) and its line counts): the durable directory, kept outside the
   engine's lifecycle and why (PRV-021, PRV-029); that the operator creates it and removes a
   subdirectory only once nothing is owed there; what each place holds; the refusals. Say what the
   provider does with a stop it still owes at its retirement: the stop stays in its subdirectory,
   nothing in RP1 carries it out (the external drainer is RP2's, as
   `docs/implementation/PROVIDERS.md` plans), and an operator ends the run's survivors and then
   removes the subdirectory. Replace "Start ... with `CONDUIT_STATE_DIR` naming its own state
   directory" and "retain it after retirement", which the engine's runner does not allow.
5. **Tests,** in `packages/provider-local-process/tests/`:
   - Move the existing tests to the new layout without losing an assertion: each provider starts
     with a `CONDUIT_STATE_DIR` at `<temporary directory>/instances/<name>` and a `durableDir` at
     `<temporary directory>/durable`, and reads markers and the ledger in the subdirectory.
     `tests/package.rs`'s template config gives `durableDir` a path, since it has no default, and
     its MAN-017 mutation now covers that binding too.
   - `owed_stop_outlives_the_state_directory`, in `tests/runtime.rs`. As
     `killed_guard_cannot_confirm_stop_or_admit_a_duplicate_after_restart` does, a run's guard
     alone is killed and `stop.v1` reports unknown, so a stop is owed; the provider is ended; the
     test removes that `CONDUIT_STATE_DIR` whole, as PRV-029 does at the Resource's removal. Then
     the subdirectory's ledger still holds the owed stop for that instance (by `FileStore::load`,
     a boolean check) and its marker is still there. A provider started with a new
     `CONDUIT_STATE_DIR` and the same `durableDir` starts, uses another subdirectory, and leaves
     the first's `ledger.json` byte for byte. On today's code the ledger goes with the directory.
     The test ends the survivors itself, as the guard test does.
   - `startup_refuses_a_misplaced_durable_directory`, in `tests/runtime.rs`: each refusal of
     decision 3 but ownership, one provider start apiece, exits non-zero with its stderr line
     naming `durableDir` or `LOCAL_PROCESS_DURABLE_DIR` (for the old layout, the file), and, for
     a misplaced `durableDir`, creates nothing under it. The ownership refusal is shown by a unit
     test with an injected uid, as SSH-R's `foreign_owned_durable_directory_is_refused` does.
6. **Order 226's tests, if merged.** Where `engine/crates/shell/tests/local_process.rs` exists
   (the start check prints `226 0`), its provider Resource's document gets `durableDir` in the
   test's temporary directory, outside the store; its `runs/` and `instances/` assertions stay on
   the provider's `CONDUIT_STATE_DIR`. Change nothing else in it. Where it does not exist, change
   no engine test. Where order 226 has merged, its two unit tests in `src/backend.rs`
   (`local_backend_accepts_the_canonical_spelling_of_its_hostname` and
   `local_backend_advertises_and_compares_canonical_hostnames`) move to the new layout as
   decision 5's tests do, without losing an assertion.

### Part 2: SEC-024's hosts as written

7. **The reading stays.** `secret_transport` keeps deciding on the host as the Remote writes it,
   as RFC 3986 has it: `localhost` without regard to case, an IPv4address of four decimal octets
   in 127.0.0.0/8, or `[::1]`, the IPv6 address `::1`, with `url::Url::parse` still required to
   succeed; not on `Url::host()`. What it decides today stays, but where decision 8 finds a
   departure. Above the function, add a comment of at most three lines that quotes, on one line,
   SEC-024's "The URL is compared as the Remote writes it" and says that `Url::host()` would
   read `127.1` as 127.0.0.1.
8. **The test,** `sec_024_reads_the_host_as_written`, beside
   `sec_024_allows_https_and_explicit_loopback_only`, which stays as it is. Refused:
   `http://127.1:8080/mcp`, `http://0x7f000001/mcp`, `http://2130706433/mcp`,
   `http://0177.0.0.1/mcp` and `http://127.000.000.001/mcp`. Allowed: `http://127.0.0.1:8080/mcp`,
   `http://127.255.255.254/mcp`, `http://localhost:8080/mcp` and `http://[::1]:8080/mcp`. Where a
   case fails on today's code, the code departs from RFC 3986's grammar: fix `secret_transport`,
   not the case, and report it.

## What to read

- PRV-002, PRV-020, PRV-021 and PRV-029 in `spec/promises/providers.yaml`; LIF-035 in
  `spec/conventions/lifecycle.yaml`; SEC-024 in `spec/promises/secrets.yaml` and the `remote`
  URL row of `spec/schema-profile.md`.
- [Order 145](145-ssh-r-durable-dir.md), decisions 1 to 5, 7 and 8, the model; and, to read
  only, `packages/provider-ssh-runtime`'s README sections "Startup configuration" and "Durable
  local layout", `src/config.rs` and `src/main.rs`.
- `packages/provider-local-process`: README, `contracts.yaml`, `conduit.yaml`, `src/main.rs`,
  `src/lib.rs`, `src/backend.rs` (`with_signaller`, `path`, the guard's launch), `src/storage.rs`,
  `tests/runtime.rs` (`Provider::start`, the guard test,
  `state_store_reopens_identical_contents_and_missing_state_directory_is_explicit`) and
  `tests/package.rs`.
- `engine/crates/core/src/packages.rs`: `secret_transport` and its test.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 2c56efac HEAD; echo "main $?"
grep -c 'names::STATE_DIR_VARIABLE' packages/provider-local-process/src/main.rs
grep -c 'durableDir' packages/provider-local-process/contracts.yaml
grep -c 'durableDir' packages/provider-ssh-runtime/contracts.yaml
grep -cF '"http://127.1/mcp",' engine/crates/core/src/packages.rs
grep -c 'sec_024_reads_the_host_as_written' engine/crates/core/src/packages.rs
test -e engine/crates/shell/tests/local_process.rs; echo "226 $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd packages/provider-local-process && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "provider $?"
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `0`, `4`, `1`, `0`, then `226 1` on `main` at `2c56efac` or `226 0`
once order 226 has merged (either goes on; decision 6 says what each means), `provider 0`,
`engine 0` and `0`. `provider` or `engine` printing otherwise means a crate is missing from the
cache: stop and report, and fetch nothing.

## Scope

Change `packages/provider-local-process/` (`src/`, `tests/`, `conduit.yaml`, `contracts.yaml`,
`README.md`), `engine/crates/core/src/packages.rs` for decisions 7 and 8 alone, and, only where
order 226 has merged, `engine/crates/shell/tests/local_process.rs` for decision 6. Change nothing
else: not `spec/`, `conformance/`, `docs/`, `experiments/`, `packages/runtime-provider-kit/` or
`packages/provider-ssh-runtime/`, no other engine file, no `Cargo.toml` and no `Cargo.lock`. Put
anything else that needs changing in your report.

## Done when

Write the tests of decisions 5 and 8 first and run them on the unchanged code: report each
failing line (decision 8's passes there, once written, unless a case departs from RFC 3986).
Then change the code until all pass. Each line marked "fails today" prints otherwise on `main`
at `2c56efac`; all pass after.

```sh
(cd packages/provider-local-process && cargo test --release --test runtime owed_stop_outlives_the_state_directory 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; fails today (0)
(cd packages/provider-local-process && cargo test --release --test runtime startup_refuses_a_misplaced_durable_directory 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; fails today (0)
grep -c 'LOCAL_PROCESS_DURABLE_DIR: /durableDir' packages/provider-local-process/conduit.yaml  # 1; fails today (0)
grep -c 'durableDir' packages/provider-local-process/contracts.yaml  # 3 or more; fails today (0)
(cd engine && cargo test --release -p conduit-core --lib sec_024_reads_the_host_as_written 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; fails today (0)
grep -c 'compared as the Remote writes it' engine/crates/core/src/packages.rs  # 1 or more; fails today (0)
```

These pass, as on `main`; run the provider's whole suite once, last:

```sh
(cd packages/provider-local-process && cargo fmt --check)
(cd packages/provider-local-process && cargo clippy --all-targets -- -D warnings)
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-core --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-core --lib sec_024)  # 2 passed
test ! -e engine/crates/shell/tests/local_process.rs || (cd engine && cargo test --release -p conduit-shell --test local_process)  # every test passes once 226 has merged
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^packages/provider-local-process/|^engine/crates/core/src/packages\.rs$|^engine/crates/shell/tests/local_process\.rs$' | wc -l | tr -d ' '  # 0
(cd packages/provider-local-process && cargo test --release)  # every test passes
```

Your report gives:

- per decision, what changed, and where each file of the provider's state now lives;
- each new test's name with its failing line on the unchanged code, then its passing line;
- each fault found in the provider or the kit, with the test that shows it;
- whether order 226's tests were present, and what decision 6 changed in them;
- every choice you made yourself, under `own_choices`;
- every check's last line.

Stage nothing. The verifier's turn follows; the coordinator then runs `tools/gate.py`, has a
fresh, read-only Claude reader read the diff (ruling 79;
ruling 92 point 4), and merges.

## What follows

Named here and not in this order:

- local-process's external drainer, RP2's, which carries out a stop still owed after the
  provider's retirement, as order 146's does for SSH-R; until then the contract says nothing does.
- Whether a later local-process Resource on the same machine should see an earlier one's owed
  stops (MC4-002 is per machine): the owner's choice; this order keeps one ledger per instance,
  as SSH-R does.
- The durable-directory code the two providers now share, a candidate for the kit.
- Should `127.1` and the like count as loopback, that is a change to SEC-024 and
  `spec/schema-profile.md`, by the owner's decision, not an engine fix.

## Record

Not yet run.
