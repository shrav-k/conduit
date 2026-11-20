# Work order 218: S2-1, the engine reads a secret through a secret Resource of a Pool, with the file store, for one run, and puts its value in no result, record or log

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each case you add, remove or change, with its
promises and `passes`. This order needs no network: it reaches no host and fetches no crate, and
the only sockets it opens are the loopback ports the engine's own tests bind. Every crate it
builds is in the local cache already; if a build asks for one that is not, stop and report, and
fetch nothing. Run one cargo command at a time. Never print a whole large file, case, fixture or
log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Every secret in this order is a synthetic marker that a test makes at run time
and never prints. Do not run `conduit` against a store of your own outside the tests, `codex`,
`claude` or the dogfood harness, and sign in nowhere.

## What and why

S2's stream, step S2-1 (the coordinator's steps: S2-1 secrets through the file store, S2-2 the
keychain package, S2-3 the dry run, S2-4 the live run). Today the engine
serves no secret ([engine/CONFORMANCE.md](../../engine/CONFORMANCE.md), the paragraph that
begins "Runtime providers and remote connections still have the unserved boundaries", and its
statement 2, "the first build serves no secret"): a run whose `environment` points to a place
that `secret-reference.v1` governs fails, since `bindings` in
`engine/crates/core/src/lifecycle.rs` refuses the resolved reference as a "non-scalar run
binding", and no code under `engine/crates/` reaches the file store. Only SEC-007 and SEC-008 of
the 24 SEC promises have a case that passes ([engine/STANDING.md](../../engine/STANDING.md)).

S2's Linear server is a `command` item that binds `S2_LINEAR_API_KEY` to `/apiKey`, a place that
`secret-reference.v1` governs (`packages/scenario-s2-linear/conduit.yaml`). This order builds the
read that serves it: a setup of an instance whose run needs a secret makes the engine read the
secret through the instance's own Conduit to a secret Resource, an entry of a Pool, by a request
for `get-secret.v1`; the engine gives the value to that one run on its own runner, and puts it in
nothing else. The store is the file store that ships with the reference engine
([IN_FORCE.md, topic 12](../../IN_FORCE.md#12-secrets): 41 items 13a and 13b;
[packages/provider-file](../../packages/provider-file/README.md)). The read path is not
store-specific: S2-2's keychain store plugs into it unchanged.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
2, every access is a relationship that can be inspected and revoked (a read is a request through
one Conduit, refused where that Conduit is closed); 3, a shared thing is owned once and never
copied to an Agent (the value reaches the one run and no result, record or Agent); and 7,
everything is attributable (each read is recorded with its caller and the setup it serves).
**Should this exist** (point 3): delivering a stored secret to a process as a variable is the
existing practice, and this order invents none of it: Kubernetes `secretKeyRef` and Amazon ECS
`valueFrom` read at start and pass a variable, and the store is the existing package
`provider-file`. No existing tool decides each read through a relationship that can be closed
and records it for the caller of the setup it serves; SEC-008, SEC-010, SEC-011 and SEC-018 make
that Conduit's.

**What S2 needs from this step** ([experiment 013](../../experiments/013-s2-shared-login/README.md),
"What it needs first", and its V3 and V4): reads for runs on the engine's own runner and
fail-closed reads, recorded without the value (SEC-010, SEC-012, SEC-015); a new run reading the
new value after a rotation (SEC-014, SEC-017). Not this step: S2's vectors V1 to V6 (S2-3), the
keychain store (S2-2), runtime-provider runs (S2-R3 and S2-R4's Docker runtime), and the batch-2
rule on driven Agents (013's G1).

**When it runs.** Through Conduit's dogfood: a worker turn and a verifier turn by `conduit
call`, then the local gate and a fresh read. Keep out of `engine/crates/shell/src/main.rs` and
`server.rs` (order 219's) unless the read needs them, and say why. By
[ruling 89](../proposals/ruling-89/README.md), each item says what must hold; how you build it is
yours.

**Stop rule.** Change nothing under `spec/`. Where a promise named here needs a choice that
`spec/` does not make, stop there, change nothing further, and report the promise, the choice
and the options it leaves. Do not choose. An engine choice that no promise governs (how the
value travels from the read to the run inside the engine, a record's extra fields, a case's
layout) is yours: list it under `own_choices`.

**The owner's answer (2026-11-20).** Round 1 stopped here: SEC-010 refuses the read where more
than one open Conduit from the instance to the secret Resource lists `get-secret.v1`, but
ACC-056's order names no rule deciding it (REC-001). The owner ranked SEC-010 ("A now, C at the
review"). Make exactly these edits under `spec/`, and no other: (1) ACC-056's `then`: `(2)
ACC-001;` becomes `(2) ACC-001, then SEC-010 where the engine's read of a secret finds more than
one open Conduit from the instance to the secret Resource that lists `get-secret.v1`;`; (2) its
SEC-010 cause row becomes `The engine reads a secret only through exactly one open Conduit of
the instance's own that lists it; more than one is open (SEC-007).`; (3) SEC-010's
`observable`: `the read reports refused, and SEC-012 applies` becomes `the read reports refused,
decided by ACC-001 where none is open and by SEC-010 where more than one is, under ACC-056, and
SEC-012 applies`. A case shows it: two such Conduits, refused with `ruleId` SEC-010, nothing
started, no value anywhere. Lines marked "(answer)" below check it.

## What must hold

The promises' own words bind; each item names them and does not restate them (ruling 92 point 7).

1. **The read, before the run** (SEC-010, PRV-003, PRV-007, TPL-012). When the engine is to ask
   its own runner for a run of an item whose `environment` or `packageArguments` points to a
   place that `secret-reference.v1` governs, it first reads each secret Resource referred to
   there, by a request for `get-secret.v1` through the instance's own one open Conduit to that
   Resource that lists the operation, and the runner gives each value under its variable or
   argument name. A place that holds null is left out, and nothing is read for it. This holds in
   a setup, a part of a Pool's setup (POOL-013, POOL-015) among them, and in a retirement that
   starts a stopped run for its retire hook (TPL-018), so `run_inputs`' two callers,
   `lifecycle.rs` and `retirement.rs`, both read. The secret Resource is an entry of a Pool, and
   the consumer's reference to it is written as the Pool template's entry reference.
2. **Attribution** (SEC-011, SEC-015, REC-001, REC-002). The read belongs to the setup or the
   retirement it serves and has that invocation's caller; its record names the reading Conduit,
   its consumer, the secret Resource as target, `get-secret.v1`, the read's own invocation ID,
   the invocation it serves, the time and how it ended. It is decided by the reading Conduit and
   that Conduit's filters alone: the filters of the Conduit through which the setup came are
   neither applied to it nor sent with it.
3. **Fail closed** (SEC-012, SEC-008, SEC-010). Where the instance is the consumer of no such open
   Conduit, or of more than one, the read reports refused; where it ends with any reason, failed
   or unknown, no run is asked for, the part that would ask for it reports unavailable, no later
   part is begun, the values of reads that did succeed for that run go nowhere, and the record
   says how the read ended. A setup of the Pool sent by an actor other than root opens no
   declared Conduit that lists `get-secret.v1` (SEC-008). Where the consumer's declared Conduit
   to the secret entry is marked optional (CON-004), that part is not opened, the run part is
   begun, its read finds no open Conduit and reports refused (SEC-010), and the run part reports
   unavailable (SEC-012). Where it is not optional, the spec's own outcome holds: the part that
   opens it reports refused (SEC-008, ACC-005), or conflict where its Conduit was closed and none
   is left that is not retired (CON-004); no later part is begun (TPL-012), so no read record and
   no run intent exist.
4. **One run, read each time** (SEC-014, SEC-017, PRV-026). A value given to a run is fixed for
   that run: closing the reading Conduit afterwards recalls nothing, the instance stays ready,
   and the engine reads nothing more for it. Every later run is preceded by reads of its own,
   each with its own decision and record: with the stored value changed between two runs, the
   second run gets the new value, with no revision advanced and nothing changed in the secret
   Resource. Nothing the engine keeps, and nothing that survives its restart, stands in for a
   read. Where PRV-026's condition holds for the secret Resource's own run on the engine's
   runner, the engine runs it again before the read (PRV-026), and not because it saw the run
   stop (OBS-004).
5. **The value goes nowhere else** (SEC-018, SEC-020, PRV-011). The value is in the one run's
   process alone. It is in no State, journal entry, persisted intent, record, retained output
   (OPS-034), result (the read's own result reaches the engine and no instance), inspection,
   observation, diagnostic, `_meta`, log or output of a `conduit` command. The configuration
   sent under IFC-006 holds only the resolved reference. The engine answers no `get-secret.v1`
   itself: the read is dispatched to the secret Resource's implementation, and the engine's own
   environment supplies no value.

## The cases and the test

**Portable cases** under `conformance/`, run by the vectors with scripted completions and a
synthetic Type that offers `secret-provider.v1`, in a new directory of your naming with its own
`index.yaml` piece. Between them they list SEC-010, SEC-011, SEC-012, SEC-014, SEC-015, SEC-017,
SEC-018, SEC-020 and PRV-026, each under a step that asserts its slice, and none is an expected
failure. At least:

- a Pool's setup that reads before its consumer's run: the read's decision, dispatch and end
  records; its `_meta` naming the reading Conduit's filters alone; the run intent that follows;
  and the scripted value in none of `records`, `reply`, state or any intent the shell persists,
  under `absent` or a projection (items 1, 2 and 5);
- with the consumer's declared Conduit to the secret entry marked optional (CON-004): a read
  refused for no reading Conduit, for two, and for a Pool set up by an actor other than root, and
  a read that ends failed, each with no run intent and its run part unavailable (item 3);
- with that declared Conduit not optional: a Pool set up by an actor other than root, whose
  opening part reports refused, with no read record and no run intent (item 3);
- a second setup after the run ended, with its own read, and a close of the reading Conduit
  after the run started that leaves the instance ready and reads nothing (item 4);
- a read after the secret Resource's run ended under PRV-026's condition, preceded by running it
  again (item 4);
- a retirement whose retire hook needs a stopped run, whose read serves the retirement, and one
  whose read fails, so that the hook is not dispatched and the stop is still asked (item 1 and
  TPL-018).

Where the case schema cannot express a step, extend it and the runner
(`engine/crates/conformance/`) as [the case README](../../conformance/README.md) describes; an
unknown input still fails. Write `docs/work-orders/218-declared.yaml` in the form of
[209's](209-declared.yaml), declaring every case added.

**The engine test with the real store**, `engine/crates/shell/tests/secrets.rs`. It builds
`packages/provider-file` and its archive as `engine/crates/shell/tests/agents.rs`' `pack` builds
an Agent package (`cargo` in that directory, `--locked`), installs it, and runs a test server of
its own, as `engine/crates/shell/tests/runner.rs` runs `scripted_server` from the test binary.
The Pool's secret entry is `file-secret.v1` at a regular file that the test writes at mode 0600,
outside the engine's `--store` directory, with a synthetic marker made at run time; its other
entry is the test server's Type, whose `command`
item binds a variable to the secret entry's reference, with a declared Conduit to the secret
entry listing `get-secret.v1` and marked optional (CON-004), so that a refused read reaches the
run part. Root sets the Pool up through the shell as the S1 tests do. The
test shows, without printing the marker:

- the test server's process received the marker in its variable, shown by the server itself
  (for example, by writing a digest of what it received to a path outside the store that the
  test compares) and in no other way;
- the marker is in no byte of any file under the store directory, recursively, and in no stdout
  or stderr of the `conduit` commands the test runs, the setup's, `inspect`'s and `records`'
  among them;
- the store file rewritten with a second marker, and the engine stopped and started again, a
  further setup of the consumer alone, sent by root (POOL-015), runs the file store again
  (PRV-026) and passes the second marker: the server's digest is the second marker's and not the
  first's;
- the file refused by the store (mode 0644, then removed), the further setup's read ends failed, the
  test server is not started, and the record's message holds neither marker;
- root closes the reading Conduit: the running process keeps its value and the instance stays
  ready; after its run ends, the next setup's read is refused and its part reports unavailable.

If `provider-file` departs from a promise as the engine sends a read (IFC-006's `_meta`, its
discovery), fix the package with a test of its own and name the promise; otherwise leave it.
Since the engine's tests now read `packages/provider-file`, add `packages/provider-file/` to
`ENGINE_PREFIXES` and `ENGINE_WHY` in `tools/gate.py`, with its test in `tools/tests/test_gate.py`.

**What follows.** In `engine/CONFORMANCE.md`, the sentences that say the engine serves no secret
(the one beginning "Secrets are unserved too", its quotation under the statements, statement 2's
"the first build serves no secret" and "a Pool's credential is never reachable by an Agent; this
waits for a secret provider") say what is served now and what is not: runtime-provider runs and
remote connections (SEC-013, SEC-023), and the stores other than the file store. The row "DL16:
executor coverage" adds that secrets are read from the file store for the engine's own runner,
and runtime-provider runs and remote connections read none; `engine/README.md`'s line "Runtime
providers and remote connections remain unserved" adds that secrets are read from the file store
for the engine's own runner. Run
`python3 tools/promise_standing.py --write`.

## What this leaves, for S2-2 and later

Report the list round 1's report gave (S2-2: the keychain store's SEC-006, SEC-005 and SEC-022;
S2-3: S2's V3 and V4 with SEC-016, and SEC-019; later: SEC-013, SEC-023, SEC-024, SEC-010's
runtime-provider half, SEC-001 to SEC-004, SEC-009, SEC-021), corrected by what you built.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 55d889bc HEAD; echo "main $?"
grep -c '^| SEC-.*| case passes |$' engine/STANDING.md
grep -c 'non-scalar run binding' engine/crates/core/src/lifecycle.rs
grep -rl 'provider-file' engine/crates | wc -l | tr -d ' '
grep -c 'the first build serves no secret' engine/CONFORMANCE.md
test -f engine/crates/shell/tests/secrets.rs; echo "test $?"
(cd packages/provider-file && cargo metadata --offline --locked --format-version 1 >/dev/null); echo "offline $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `2`, `1`, `0`, `1`, `test 1`, `offline 0` and `0`. If `offline` prints
otherwise, a crate is missing from the cache: stop and report.

## Scope

Change `engine/crates/core/src/`, `engine/crates/shell/src/` (not `main.rs` or `server.rs`
unless you say why), `engine/crates/conformance/` where the case schema needs it,
`engine/crates/shell/tests/secrets.rs` (new), `conformance/` (the new cases, their fixtures and
index piece, `case.schema.json` and `case-schema/` where extended), `engine/CONFORMANCE.md`,
`engine/README.md`, `engine/STANDING.md` only by the tool, `tools/gate.py` and
`tools/tests/test_gate.py` as above, `packages/provider-file/` only as above, and create
`docs/work-orders/218-declared.yaml`. Change nothing under `spec/` beyond the owner's answer's three edits, `docs/` outside the
declaration, `experiments/`, any other `packages/` path, `IN_FORCE.md`, `ROADMAP.md`,
`docs/AUDIT.md`, `engine/Cargo.lock` or any `Cargo.toml`. Anything else that needs changing goes
in your report.

## Done when

Each line below marked "fails today" prints otherwise on `main` at `55d889bc`; all pass after.

```sh
(cd engine && cargo fmt --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite, the vectors and secrets.rs among it
(cd engine && cargo test --release -p conduit-shell --test secrets)  # passes; fails today (no such test)
git add -N conformance engine/crates/shell/tests/secrets.rs docs/work-orders/218-declared.yaml  # so the checks see new files
python3 tools/check_docs.py                     # Errors: 0
python3 tools/check_rulings.py                  # passes
python3 tools/set_names.py --check              # passes
python3 tools/check_schemas.py                  # Errors: 0
python3 tools/check_roadmap.py                  # passes
python3 tools/check_model_account.py            # passes
python3 tools/promise_standing.py --check       # Errors: 0
python3 tools/check_private_paths.py            # Errors: 0
python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/218-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; exit 0
git diff --check                                # prints nothing
git diff --name-only HEAD -- experiments IN_FORCE.md ROADMAP.md docs/AUDIT.md engine/Cargo.lock | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- spec | grep -v -x -e spec/conventions/access.yaml -e spec/promises/secrets.yaml | wc -l | tr -d ' '  # 0 (answer)
/usr/bin/python3 -c "import yaml; print(sum('(2) ACC-001, then SEC-010 where' in p.get('then','') for p in yaml.safe_load(open('spec/conventions/access.yaml')) if isinstance(p,dict)))"  # 1; fails today (0) (answer)
grep -rlE "ruleId: ?['\"]?SEC-010" conformance | wc -l | tr -d ' '  # 1 or more; fails today (0) (answer)
grep -E '^\| (SEC-0(10|11|12|14|15|17|18|20)|PRV-026) \|' engine/STANDING.md | grep -c '| case passes |$'  # 9; fails today (0)
grep -c '^| SEC-.*| case passes |$' engine/STANDING.md  # 10 or more; fails today (2)
grep -rl 'provider-file' engine/crates/shell/tests | wc -l | tr -d ' '  # 1 or more; fails today (0)
grep -c "'packages/provider-file/'" tools/gate.py  # 1 or more; fails today (0)
grep -c -e 'the first build serves no secret' -e 'Secrets are unserved too' engine/CONFORMANCE.md  # 0; fails today (3)
git reset -q -- conformance engine/crates/shell/tests/secrets.rs docs/work-orders/218-declared.yaml  # stage nothing
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives, per item above: what
you built, the cases and the tests that show it, and each choice you made yourself (under
`own_choices`), the route the value takes inside the engine among them; the vector delta's
counts; the corrected list of what this leaves; and every check's last line. Stage nothing. The
coordinator runs `tools/gate.py`, then has a fresh, read-only Opus reader read the diff
([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and merges.

## Record

Not yet run.
