# Work order 144 (S4-SRV2): S4's setup hook and its ledger reader

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for the installation check, whose engine binds a loopback port; use it for nothing else, and
reach no real service. Run one cargo command at a time, and never print a whole large file or log.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never print a key or a token, even a synthetic one that a test made.

## What and why

Scenario S4's T1 and T6 ([its README](../../experiments/015-s4-partial-setup/README.md), "The
tasks") set up a Pool with dependent entries and a setup hook: in T1 a required part is blocked
after earlier parts succeeded, and the author repairs the cause and asks for setup again; in T6
the hook's reply is lost, the hook is reconciled as failed only after independent no-effect
verification, and B asks for setup while A's delayed onward call arrives during B's attempt
(E20). The preparation that the packages supply includes a hook server that loses its reply, can
independently prove no effect for reconciliation as failed, and then holds A's onward call until
B's new attempt runs. P3 has inspection and the external ledger agree on completed parts, P5 has
E20 report the latest hook caller and its attribution limit (OPS-028), and M3 counts, at each
boundary, attempts per hook invocation, effects and the callers that records name.

This order builds the hook, its barriers, its ledger and the ledger's reader, as S4-SRV2 of
[S4's plan](../../experiments/015-s4-partial-setup/plan.md) (its row in "The orders" and the
paragraph "S4-SRV2"), and gives T1 a cause that an author can repair (decision 6). For E20 the
plan has the server hold the ended call's terminal response until the engine has marked it
unknown, then release it: S4-LATE forwards it, and order 077's core records the fence without
settling unknown, before an explicit no-effect reconciliation. This order provides that control,
`before-reply`. Today's shell drops an answer that arrives after its timeout
(`engine/crates/shell/src/runner.rs`, lines 1031 to 1047), S4-LATE qualifies the forwarding with
its own scripted servers, and the vectors V1 and V6 belong to S4-SET, with the engine's own hook
fixtures.

The hook's onward request, which E20 holds until B's attempt, is cut from this order into a later
order, S4-SRV2b, with the run's token, the stand-in for the engine's server and E20's test of it:
E20 cannot run before S4-LATE and S4-SET in any case.

S9-SRV ([S9's plan](../../experiments/020-s9-start-and-repair/plan.md), its row and paragraph)
reuses this hook and its lost-reply control, and specializes only a backing precondition, a repair
Resource and readback; it starts once this order's hook interface exists. Build the hook once,
here: the crate is a library as well as a binary, and its README freezes the interface
(decision 8). Order 143 (S4-SRV1) builds S4's effect servers in parallel with the same extension
of SCR1's barrier handshake (decision 5), which both orders state in full; the two crates share
no code. This order depends on no other.

Read first:

- S4's README: "The tasks" (the preparation, T1 and T6), P3, P5, M2, M3, V1 and V6; S4's plan:
  the paragraphs "S4-SRV2", "S4-LATE", "S4-SET" and "S4-H", above all the last sentences of
  "S4-SRV2", which ask both server orders for package installation checks; S9's plan: the row and
  the paragraph "S9-SRV"; and
  [S9's README](../../experiments/020-s9-start-and-repair/README.md), "The tasks" (its preparation
  and T4);
- [order 142](142-scr1-scripted-client.md) (SCR1), decision 7: the barrier handshake that
  decision 5 extends;
- TPL-004, TPL-012, TPL-017 and TPL-019 in `spec/promises/templates.yaml`; OPS-007, OPS-008,
  OPS-012 to OPS-014, OPS-020 to OPS-022 and OPS-028 in `spec/conventions/operations.yaml`;
  ACC-018 in `spec/conventions/access.yaml`; IFC-006, IFC-008, IFC-009, IFC-018 and IFC-026 in
  `spec/interface/implementation.yaml`; CTR-006 in `spec/promises/contracts.yaml`; CON-017 in
  `spec/promises/conduits.yaml`; REV-007 in `spec/conventions/revisions.yaml`; "configuration"
  in `spec/definitions.md`; and `setup-hook.v1`, near line 820 of `spec/core/operations.yaml`;
- [order 077](077-spec-reconcile-fenced.md), decisions 1 and 2: what fences an attempt, and what
  the fence leaves out;
- rmcp 3.9.0's `src/service.rs`, near lines 1508 and 1661: once its peer cancels a request with
  `notifications/cancelled`, rmcp drops the answer to it;
- [the engine's README](../../engine/README.md), "The shell's store, runner and command line" and
  "Change a serving project", and `engine/crates/shell/src/author.rs` (`install` and `submit`),
  for the installation check;
- the package form: [order 136](136-s2-l1-linear-aware.md) and its result,
  `packages/scenario-s2-linear/`, and `packages/provider-local-machine/`.

## The decisions it applies

1. **Where it lives.** `packages/scenario-s4-hook/`, a crate with its own `Cargo.toml`, committed
   `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and `dist/`: a
   library, `scenario_s4_hook`, and a binary that is its thin `main`, using `rmcp = "=3.9.0"` as
   the other packages do. It depends on nothing in this repository. Its README says first that it
   serves scenario S4 and is not for general use. It targets Unix, as S4's runs do, and its README
   says so.
2. **Its contracts**, in `contracts.yaml`, at the address `example.org/conduit/scenario-s4-hook`:
   - `read-hook-ledger.v1`: input `instance`, an optional string; its output is decision 7's
     readback; no `effect`;
   - `set-blocked-by.v1`, a control (CTR-006, OPS-020): inputs `revision`, the core package's
     `revision.v1`, and `blockedBy`, both required, `blockedBy` with the schema of the place
     below; `changes: {blockedBy: /blockedBy}`; an `effect`, that it changes the path at which the
     hook looks for a file that blocks its attempts; and `deduplicates: true`. Write it as
     `set-read-limit.v1` is written, near line 160 of
     `spec/schemas/examples/valid/files-package/contracts.yaml`, and check its form against the
     schema `operation-spec.v1` in `spec/schemas/contracts.yaml`;
   - the capabilities `hook-ledger-reading.v1` (`read-hook-ledger.v1`) and `hook-settings.v1`
     (`set-blocked-by.v1`): a Conduit lists a control only where its capability includes it
     (CON-017);
   - `s4-hook.v1`, a Type of the family Resource that `requires` the core package's
     `setup-hook.v1`, offers `hook-settings.v1`, lists `set-blocked-by.v1` under `controls`, and
     maps both operations to `#hook` (TPL-004). Its schema, with `additionalProperties: false`,
     requires `ledger`, a string, and allows `control`, a string, and `blockedBy`, an `anyOf` of
     a string that begins with `/` and null, so that setting null removes the block. Its one item
     is the instance's run (TPL-019), so the hook belongs to Resources, such as a Pool's entries
     in T1, and never to a Pool, which TPL-019 gives no run;
   - `s4-hook-ledger.v1`, a Type of the family Resource offering `hook-ledger-reading.v1`, mapped
     to `#reader`, whose schema requires `ledger`, a string.

   No contract but these six: no repair, precondition or confirmation operation, and
   `set-blocked-by.v1` sets one place of the hook's own configuration and nothing else. S9-SRV's
   repair Resource is S9's.
3. **Its items**, in `conduit.yaml`: `hook`, `command: [bin/scenario-s4-hook, hook]`,
   implementing `github.com/shrav-k/conduit/core#setup-hook.v1` and `#set-blocked-by.v1`; and
   `reader`, `[bin/scenario-s4-hook, read]`, implementing `#read-hook-ledger.v1`. Both are
   Conduit-aware and state no `supports`. Both read their configuration from each call's
   `io.github.shrav-k.conduit/config` (IFC-006), so neither binds `environment`, and a value that
   the control sets reaches the hook's next attempt with no new run. A `ledger` that is not an
   absolute path or whose directory does not exist, a `blockedBy` that is neither null nor an
   absolute path, and a `control` that is not a directory owned by the user the server runs as
   and writable by no group or other user, make a call perform nothing and answer failed, with a
   diagnostic naming the place.
4. **The ledger**, kept as order 143 keeps its own. One JSON Lines file at `ledger`, which is the
   package's own: only this binary writes it, the engine never reads it, and it lies outside the
   engine's store. It is created with mode 0600. Every write takes an exclusive advisory lock on
   the file (`flock`), appends whole lines in one write, calls `fsync` and releases the lock, and
   the file's creation also syncs its directory. A line is committed once that `fsync` returns. No
   lock is held while a call waits at a barrier, and a call held at a barrier holds up no other
   call. Several instances may share one ledger, as a Pool's entries do. Every line holds the
   format's version, its kind, the call's ID (lowercase letters, digits and hyphens, unique in the
   ledger: the process's ID, its start time and a counter will do), and the instance, invocation,
   caller, idempotency key and revision that the call's `_meta` carries. The kinds are at least:
   `received`, with `attempt`, the server's own count of the calls it has received for that
   instance and key, which is trace evidence only, since IFC-006 sends no attempt number;
   `effect`, the hook's setup work, one whole entry with its own ID; `ended`, the attempt's own
   work ended, with its effect's ID or null, the cause where it had none (`blockedBy`, the
   controller or the gate) and a diagnostic; `reply`, `released`, `dropped` or `cancelled`, with
   the outcome; and `control-lost`. The README freezes this format as v1.
5. **Barriers, released by explicit acknowledgement.** The server extends the handshake of
   SCR1's barriers (order 142, decision 7), so that S4-H releases client and server barriers with
   one mechanism: `NAME.reached`, then `NAME.release`, then `NAME.acknowledged`, in one
   directory, with the server polling for the release. It differs from SCR1's in four ways: the
   server chooses its names, `CALL.BARRIER`, from the call's ID and the barrier's name, so S4-H
   finds server barriers by scanning the directory for `*.reached` files and matching the call
   details inside; its `.reached` file is not empty but holds one JSON object, the version (1),
   the package's name (decision 8), the barrier, the call's ID and the call (instance,
   invocation, caller, key and attempt); its release is not empty but holds a verdict, `proceed`,
   `drop` or, at `before-apply` alone, `fail`; and it never times out. `control` names the
   directory. The server creates `NAME.reached`, and later `NAME.acknowledged`, new with mode
   0600, each by writing it under a temporary name that ends in none of `.reached`, `.release` and
   `.acknowledged`, since S4-H scans for `*.reached`, and renaming it into place. `NAME.reached` is
   the server's acknowledgement that it holds the call there with everything before it
   committed. The controller writes `NAME.release` the same way: under a
   temporary name in the same directory, one that does not end in `.release`, renamed into place
   once written, so that the server, which reads only `NAME.release`, reads it only after the
   rename. The READMEs say so. Once the server has committed any ledger line that records the
   release itself, it creates `NAME.acknowledged`, and only then goes on. `drop` means that the
   server performs nothing further for the call and never answers it. A file it cannot write or
   read, or a release that holds anything else, acts as `drop` and writes `control-lost`. No timer
   orders anything: the poll is the only wait, the server never gives up waiting, and the engine's
   own timeout is what turns a held call unknown. With no `control` there is no barrier. The
   barriers:
   - `before-apply`, after `received` and before any work;
   - `before-reply`, after `ended` and before the answer: the lost-reply control.
6. **The hook.**
   - **Checks.** Conduit's extension at `v0`, or -32021 naming it (IFC-008); an `operation` that
     is neither the core package's `setup-hook.v1` nor `set-blocked-by.v1`, or a key that IFC-006
     requires, missing or malformed, `idempotencyKey` among them for `setup-hook.v1` since
     OPS-028 gives every hook one, performs nothing and answers failed with a diagnostic naming
     it; and a `filters` holding any item answers `unavailable` in IFC-026's form, since the hook
     supports no requirement and S4's setups carry none.
   - **Each attempt** of `setup-hook.v1` writes `received` and waits at `before-apply`. On
     `proceed` it checks `blockedBy`, then asks the gate where the library was given one
     (decision 8), then writes its `effect` and an `ended` line naming it, and answers succeeded.
     Where `blockedBy` blocks it, on `fail`, or on the gate's fail, it performs nothing, writes
     `ended` with no effect, its cause and a diagnostic, and answers failed. On `drop` it performs
     nothing, writes `ended` with no effect and `reply` `dropped`, and never answers. With no
     controller every attempt goes straight to `blockedBy` and the gate.
   - **`blockedBy`, T1's repairable cause** (the admin's decision, 2026-11-12, logged for the
     owner to strike). Where `blockedBy` names a file that exists, the attempt performs nothing,
     writes no effect, and answers failed with a diagnostic naming the place `blockedBy` and that
     path (ruling 52). A file here is a regular file, or a link to one, which the hook learns from
     the path's metadata alone: it never opens the file, so no diagnostic, answer or ledger line
     ever holds what the file contains. Null, a path at which nothing exists and a directory block
     nothing. T1 tests whether an author can diagnose and repair a failed setup through Conduit,
     and an instance's configuration changes only through a control (OPS-020, and
     "configuration" in `spec/definitions.md`). So the repair that S4's M2 counts is
     the author's call of `set-blocked-by.v1` through a Conduit to the hook's instance, setting
     `blockedBy` to null or to a corrected path; removing the file outside Conduit is allowed, but
     M2 records it as an environment step, not an author repair. The README says which the
     pre-registration counts, as this paragraph states it.
   - **The control.** `set-blocked-by.v1`, after the checks, performs nothing, writes nothing to
     the ledger and answers succeeded, with `isError` false: the engine stores the value it sent
     (REV-007), and the hook reads it from the configuration of each later call.
   - **No deduplication.** Every attempt that proceeds applies its effect. `setup-hook.v1` does not
     declare `deduplicates`, and the engine never dispatches a hook that succeeded again (OPS-021,
     OPS-028), so a second effect under one key is evidence about the engine, which M3 counts, not
     the server's to hide.
   - **Every answer.** rmcp drops the answer to a request that its peer has cancelled. So before
     any answer, the hook checks the request's cancellation token (`context.ct`): where the
     request is cancelled, an attempt writes `reply` `cancelled` in place of `released`, and the
     answer goes nowhere. A cancellation that arrives between that check and the send can still
     drop an answer; the README states this limit.
   - **An attempt's answer** waits at `before-reply`. On `proceed` the hook writes `reply`
     `released`, then answers with the outcome, succeeded or failed, in `_meta`, `isError`
     agreeing (IFC-009), no `structuredContent`, since `setup-hook.v1` declares no outputs, and a
     text block naming the attempt and its effect, or why it had none. On `drop` it writes `reply`
     `dropped` and never answers.
   - **E20's lost-reply control.** An attempt that ended without an effect is held at
     `before-reply` while S4-H waits for the engine to mark it unknown, and is then released: its
     late failed answer is the terminal response that S4-LATE forwards, which order 077 counts as
     fencing the attempt. The server knows nothing of the engine's state; S4-H decides when.
7. **The reader and its readback, independent of the engine.** One function reads the ledger,
   under a shared lock, and nothing else. It serves `read-hook-ledger.v1`, filtered by `instance`
   where it is given, after decision 6's checks of IFC-006, and the subcommand
   `scenario-s4-hook readback LEDGER`, which prints the same JSON on stdout for S4-H, S9-H and the
   tests. For each attempt it reports the instance, invocation, caller, key, revision and attempt
   count; `work`, `ended` or `open`; `effect`, `applied` with its ID, `none` with its cause only
   where the attempt's `ended` line says that it had none, or `not-known` while the attempt is
   open; and `reply`, `released`, `dropped`, `cancelled` or `none`. It lists by number each line
   that does not parse, never dropped silently. It never infers that an effect is absent from a
   missing line, it states no outcome for a hook's invocation or a setup as a whole, and it never
   calls a setup failed because some instance or part has no effect: no partial ledger is called
   a whole-effect failure (OPS-022, Q5). The README states the readback's form as v1, and the
   output schema of `read-hook-ledger.v1` states it in full.
8. **The interface for S9-SRV**, which the README freezes as v1. The library exposes the barrier
   handshake of decision 5, given a directory and a package's name; the ledger's append and its
   readback (decisions 4 and 7), the readback a value that S9-SRV can extend with fields of its
   own; and a function that serves the hook over stdio, given the package's name for its barrier
   files and ledger lines, and an optional gate. The gate is called after `blockedBy` and before
   any work, is given the call (instance, invocation, caller, key, attempt and configuration),
   and answers proceed, or fail with a diagnostic, which the hook answers as it answers the
   controller's `fail`. S9-SRV supplies its backing precondition there, and adds its repair
   Resource and readback fields in its own crate, which depends on this one by path; it changes
   nothing here. The Rust names are your choice; report them.
9. **Tests**, driving the built binary with an `rmcp` client and the full request `_meta`, with a
   test controller that waits for `.reached` and `.acknowledged` files in a temporary directory
   and writes each release under a temporary name before renaming it. No test waits a fixed time
   in place of a barrier's file, and no test drops or times out the future of a held call, which
   could make the client cancel it, apart from the one test of cancellation below. Compare markers
   with `assert!(a == b)`, never `assert_eq!`. The tests cover:
   - discovery at `v0` with the two tools; -32021 without the extension and at another revision;
     a hook call missing `idempotencyKey`, a relative `ledger`, a relative `blockedBy` and an
     unsafe `control` each answer failed with the ledger unchanged; a `filters` item answers
     `unavailable`;
   - with no controller: succeeded, with `effect`, `ended` and `reply` `released`; a second call
     with the same key and another caller is counted attempt 2, with its own caller and a second
     effect;
   - `blockedBy`: with a file at its path that holds a marker, an attempt answers failed with no
     effect, its diagnostic names the place and the path, and the marker is in no answer, ledger
     line, stdout or stderr; once the file is removed, the next attempt succeeds; a directory at
     that path blocks nothing; `set-blocked-by.v1` answers succeeded, with `isError` false, and
     leaves the ledger unchanged; and an attempt whose configuration holds `blockedBy` null, as
     after that control, succeeds while the file still exists;
   - with a controller: `fail` answers failed with `ended` and no effect; `drop` at `before-apply`
     and at `before-reply` each leave the call unanswered while a later call is answered; an
     unreadable release writes `control-lost`;
   - the lost-reply control, in one server process: an attempt released `fail` is held at
     `before-reply` while the readback shows its work ended, its effect `none` and its reply
     `none`; once released, the late failed answer arrives and `reply` is `released`; a new
     attempt with that invocation and key and another caller then proceeds, and its effect is
     recorded with that caller;
   - cancellation: a call held at `before-reply` whose request the client cancels with
     `notifications/cancelled`, then released, writes `cancelled` and no `released`;
   - a gate given through the library that fails answers failed with no effect;
   - the readback: an open attempt's effect `not-known`, a torn last line counted, and
     `read-hook-ledger.v1`'s output equal to the subcommand's; answers validate against the output
     schema, and `isError` agrees with the outcome;
   - `conduit.yaml` and `contracts.yaml` validate against `spec/schemas/` (skipped, saying so,
     where `spec/` is absent), and the archive is the same, byte for byte, when built twice.

   These are the package checks that the plan asks of a server order. The engine's part, which
   marks the hook unknown, forwards the late answer, records the fence and dispatches B's new
   attempt, is qualified by S4-SET's V1 and V6, by S4-LATE and by S4-H, not by this order.
10. **The installation check**, the package installation check that S4's plan asks of every
    server order. A fixture project in `tests/fixtures/project/` holds a `conduit.yaml` that pins
    this package by its address, its version and the literal digest `sha256:PLACEHOLDER`, which
    the check replaces with the built archive's in a temporary copy, and a `templates.yaml` with
    one Resource template of each of the two Types, a Pool template whose two entries are hook
    Resources, a Conduit template listing `read-hook-ledger.v1`, as an author's Conduit to a
    reader would, and one listing `set-blocked-by.v1`, as an author's Conduit to a hook's
    instance would. It holds no instance, so `submit` registers the templates and then opens and
    sets up nothing: nothing is dispatched to the package's items. The commands are in "Done
    when".
11. **Its README** gives the configuration's places, `blockedBy`, its control and the repair
    that M2 counts, the barriers and their handshake with the atomic release, the cancellation
    limit, the ledger's and the readback's v1 forms, the interface for S9-SRV, D15's four fields
    as [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them,
    and counts by category as order 136's README gives them, as package and fault preparation
    that M3 counts apart from author work. A `package` binary writes the deterministic archive
    `dist/scenario-s4-hook.tar.gz`, of `bin/scenario-s4-hook`, `conduit.yaml`, `contracts.yaml`
    and `README.md`, and `dist/scenario-s4-hook.tar.gz.sha256`, which holds its digest as bare
    lowercase hex and a newline, as order 136's `src/bin/package.rs` writes it. The installation
    check relies on both.
12. **What this order does not do:** no onward request, which is S4-SRV2b's (with the run's
    token, the stand-in for the engine's server and E20's test of A's onward call); no retire
    hook, which S4's tasks do not need and S4-LATE replaces with its own servers; no
    precondition, repair Resource or readback field of S9's; no effect server (order 143); no
    engine change and no vector; and no change to any other package.

## Scope

Change only `packages/scenario-s4-hook/` (new). The plan sized it M, 350 to 500 lines including
tests. Re-estimated from orders 136 and 139, whose results counted about 500 lines of code and
1,000 of tests each, and with the onward request cut: about 450 to 600 lines of code, 120 to 170
of manifests, contracts and fixture, and 550 to 750 of tests, so 1,100 to 1,500 in all, beside
the README and the lockfile. Report the counts.

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release`, and `cargo run --release --bin package` twice, with the same digest. Then
the installation check, from the repository root, as one `bash` invocation (a script or
`bash -c`), so that its variables and the background `serve` hold throughout; it runs one cargo
command. It stops before `init` if the port already answers, and its wait ends if `serve` exits,
so that another program's listener cannot pass for the engine. Set `P` to another free loopback
port if 38144 answers, and say which:

```bash
D=packages/scenario-s4-hook
P=38144
(cd engine && cargo build --release)
E=$PWD/engine/target/release/conduit
H=$(cat $D/dist/scenario-s4-hook.tar.gz.sha256)
L=$(mktemp -d)
mkdir "$L/project" && cp $D/tests/fixtures/project/*.yaml "$L/project/"
sed -i.orig "s/sha256:PLACEHOLDER/sha256:$H/" "$L/project/conduit.yaml"
rm "$L/project/conduit.yaml.orig"
if (exec 3<>/dev/tcp/127.0.0.1/$P) 2>/dev/null; then echo "port $P already answers"; exit 1; fi
"$E" init --store "$L/store" --url "http://127.0.0.1:$P/mcp"
"$E" serve --store "$L/store" > "$L/serve.log" 2>&1 &
S=$!
for i in $(seq 30); do
  kill -0 "$S" 2>/dev/null || break
  (exec 3<>/dev/tcp/127.0.0.1/$P) 2>/dev/null && break
  sleep 1
done
"$E" install --store "$L/store" "$PWD/$D/dist/scenario-s4-hook.tar.gz#$H"
"$E" submit --store "$L/store" "$L/project"
kill "$S"; wait "$S"
```

Both `install` and `submit` must succeed. Never print `serve.log` whole; where a step fails, show
its last 20 lines. Report each command's last line and every diagnostic of the loader or the
engine as evidence. Where either rejects the package or the fixture, report the diagnostic as a
finding, change no engine code, and change the package only where the diagnostic shows a defect
in it. Then, from the repository root: `python3 tools/check_docs.py` and
`python3 tools/set_names.py --check`. Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/scenario-s4-hook
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" \
  $D --exclude-dir=target --exclude-dir=dist
grep -n -E "^  name: " $D/contracts.yaml | grep -v -E "name: (read-hook-ledger|\
hook-ledger-reading|set-blocked-by|hook-settings|s4-hook|s4-hook-ledger)\.v1$"
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

`target/` and `dist/` stay ignored and unstaged. If the sandbox refuses something a test or the
installation check needs, list what did not run, never as passed. Report each choice of
configuration, form or name that this order leaves open as your own, and these as findings: that
rmcp drops an answer to a request its peer has cancelled, so where the engine cancels a hook call
that it stopped waiting for, the lost-reply control's late answer never leaves the server and the
ledger shows `cancelled`, which S4-LATE must account for; that a cancellation can still drop an
answer after the server's check; and that the hook answers `unavailable` to any filter, so a
setup made through a filtered Conduit cannot run it. Do not stage anything.
