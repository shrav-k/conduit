# Work order 143 (S4-SRV1): S4's partial-effect and deduplicating effect servers

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for the installation check, whose engine binds a loopback port; use it for nothing else, and
reach no real service. Run one cargo command at a time, and never print a whole large file or log.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never print a key or a token, even a synthetic one that a test made.

## What and why

Scenario S4 ([its README](../../experiments/015-s4-partial-setup/README.md)) asks whether a setup
that fails partway, is retried and loses a response can be understood and recovered without
repeating an effect. Its runs need servers that the packages supply, apart from author work
("The tasks", first paragraph): a plain server that writes the first of two external ledger
entries and answers -32021, directly and as a task's failed error (E01), for an operation that
states an effect and for one marked `startsModelWork: true` that states none; servers that
complete an effect and lose the reply, once before the engine receives it (unknown) and once after
(lost success); a non-plain deduplicating server; a durable ledger; and barriers. D02 (M1) runs
twelve cells over them: a key-preserving or unkeyed client, a plain or deduplicating
implementation, and lost success, unknown or an intended second action. P2 and P4 judge by
external effect counts, and Q5's answer keeps a genuinely partial effect unknown, with no
assertion about the whole effect.

This order builds them, as S4-SRV1 of [S4's plan](../../experiments/015-s4-partial-setup/plan.md)
(its row in "The orders" and the paragraph "S4-SRV1"), except E01's task variant. That variant is
cut from this order into a later order, S4-SRV1b: nothing can reach a task through Conduit before
S4-TASK2 relays tasks, so its mode, its barrier and its tests wait for that order. S4-H, lane 10's
harness, drives these servers in S4's runs, and S9-H uses the keyed effect and its barriers for
S9's T2. The vectors V2 and V3 belong to S4-TASK1 and S4-REC1, which use the engine's own scripted
fixtures, not this package. Order 144 (S4-SRV2) builds the setup hook in parallel with the same
extension of SCR1's barrier handshake (decision 5), which both orders state in full; the two
crates share no code. This order depends on no other.

Read first:

- S4's README: "The tasks", P2, P4, M1 (D02), E01, E09, and Q5 with its answer; S4's plan: the
  paragraphs "S4-SRV1", "S4-SRV2" and "S4-H", above all the last sentences of "S4-SRV2", which
  ask both server orders for package installation checks;
- [order 142](142-scr1-scripted-client.md) (SCR1), decision 7: the barrier handshake that
  decision 5 extends;
- IFC-006, IFC-007, IFC-008, IFC-009, IFC-010, IFC-018, IFC-026, IFC-037, IFC-045 and IFC-047 in
  `spec/interface/implementation.yaml`; OPS-007, OPS-008, OPS-011, OPS-012 and OPS-018 in
  `spec/conventions/operations.yaml`; CTR-002 in `spec/promises/contracts.yaml`; PKG-020 in
  `spec/promises/packages.yaml`; MAN-010 and MAN-017 in `spec/promises/manifest.yaml`; TPL-019 in
  `spec/promises/templates.yaml`; IMP-001 in `spec/conventions/implicit.yaml`; and a plain item's
  `tools`, near line 100 of `spec/schemas/examples/valid/files-package/conduit.yaml`;
- rmcp 3.9.0's `src/service.rs`, near lines 1508 and 1661: once its peer cancels a request with
  `notifications/cancelled`, rmcp drops the answer to it;
- [the engine's README](../../engine/README.md), "The shell's store, runner and command line" and
  "Change a serving project", and `engine/crates/shell/src/author.rs` (`install` and `submit`),
  for the installation check;
- the package form: [order 136](136-s2-l1-linear-aware.md) and its result,
  `packages/scenario-s2-linear/`, and `packages/provider-local-machine/`.

## The decisions it applies

1. **Where it lives.** `packages/scenario-s4-effect/`, a crate with its own `Cargo.toml`,
   committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and
   `dist/`, using `rmcp = "=3.9.0"` as the other packages do. It depends on nothing in this
   repository. Its README says first that it serves scenario S4 and is not for general use. It
   targets Unix, as S4's runs do, and its README says so.
2. **Its contracts**, in `contracts.yaml`, at the address `example.org/conduit/scenario-s4-effect`:
   - `apply-effect.v1`: input `label`, a required string of a lowercase letter or digit and up to
     62 lowercase letters, digits or hyphens; output `record`, a required string; an `effect`,
     that it appends one effect, two entries, to the scenario's ledger; and `deduplicates: true`;
   - `start-model-work.v1`: the same input and output, `startsModelWork: true` and no `effect`.
     No model runs: its entries stand for the model work that an Agent would have begun, and the
     README says so;
   - `read-effects.v1`: input `label`, optional, as above; its output is decision 7's readback;
     no `effect`;
   - the capabilities `effect-writing.v1` (`apply-effect.v1`), `model-work.v1`
     (`start-model-work.v1`) and `effect-reading.v1` (`read-effects.v1`);
   - three Types of the family Resource, each with one item, as TPL-019 allows:
     `effect-plain.v1` offers `effect-writing.v1` and `model-work.v1`, both mapped to `#plain`;
     `effect-keyed.v1` offers `effect-writing.v1`, mapped to `#keyed`; `effect-ledger.v1` offers
     `effect-reading.v1`, mapped to `#reader`. Each schema has `additionalProperties: false` and
     requires `ledger`, a string, in all three; `control`, an `anyOf` of a string and null, in the
     first two; and `mode`, `complete` or `partial-error`, in the first. S4-SRV1b will add
     `partial-task`.

   No other contract: no repair, confirmation or reset operation (decision 11).
3. **Its items**, in `conduit.yaml`, one binary given a first argument:
   - `plain`: `command: [bin/scenario-s4-effect, plain]`, `plain: true`, `tools` naming
     `apply-effect` and `start-model-work`, and no `supports` (PKG-020);
   - `keyed`: `[bin/scenario-s4-effect, keyed]`, Conduit-aware, implementing `apply-effect.v1`;
   - `reader`: `[bin/scenario-s4-effect, read]`, Conduit-aware, implementing `read-effects.v1`.

   A plain server learns its configuration only from its `environment` (PKG-020), so every item
   takes it that way, and none reads it from `_meta`: `S4_EFFECT_LEDGER` from `/ledger`, for all
   three; `S4_EFFECT_CONTROL` from `/control`, for `plain` and `keyed`; and `S4_EFFECT_MODE` from
   `/mode`, for `plain`. Each place is required and scalar (MAN-017). A null `control` leaves its
   variable out, and then there is no barrier. At start the server checks that the ledger's path
   is absolute and its directory exists, that a control directory exists, is owned by the user
   it runs as and is writable by no group or other user, and, for `plain`, that the mode is one
   of the two; otherwise it exits with a fixed message naming the variable.
4. **The ledger.** One JSON Lines file at the configured path, which is the package's own: only
   this binary writes it, the engine never reads it, and it lies outside the engine's store, in a
   directory that S4-H or the operator chooses. It is created with mode 0600. Every write takes
   an exclusive advisory lock on the file (`flock`), appends whole lines in one write, calls
   `fsync` and releases the lock, and the file's creation also syncs its directory. A line is
   committed once that `fsync` returns. No lock is held while a call waits at a barrier, and a
   call held at a barrier holds up no other call. Every line holds the format's version, its
   kind, the call's ID (lowercase letters, digits and hyphens, unique in the ledger: the
   process's ID, its start time and a counter will do), the mode, the operation, the `label`,
   and, from a Conduit-aware call, the instance, caller, idempotency key and invocation that its
   `_meta` carries (null from a plain call, which carries none). The kinds are at least
   `received`, written before the first barrier; `entry`, one part of an effect, with its
   record's ID and its part, 1 or 2 of 2; `deduplicated`; `reply`, with `released`, `dropped` or
   `cancelled` and what was answered or would have been; and `control-lost`. Each `received` line
   carries `attempt`, the server's own count of the calls it has received with the same instance,
   caller, key and invocation (keyed) or the same operation and label (plain). IFC-006 sends no
   attempt number: the count is trace evidence only (decision 6). The README freezes this format
   as v1.
5. **Barriers, released by explicit acknowledgement.** The servers extend the handshake of
   SCR1's barriers (order 142, decision 7), so that S4-H releases client and server barriers with
   one mechanism: `NAME.reached`, then `NAME.release`, then `NAME.acknowledged`, in one
   directory, with the server polling for the release. They differ from SCR1's in four ways: a
   server chooses its names, `CALL.BARRIER`, from the call's ID and the barrier's name, so S4-H
   finds server barriers by scanning the directory for `*.reached` files and matching the call
   details inside; its `.reached` file is not empty but holds one JSON object, the version (1),
   the package, the barrier, the call's ID and the call (operation, label, instance, invocation,
   caller, key and attempt, null where the call carries none); its release is not empty but holds
   a verdict, `proceed` or `drop`; and it never times out. `control` names the directory. The
   server creates `NAME.reached`, and later `NAME.acknowledged`, new with mode 0600, each by
   writing it under a temporary name that ends in none of `.reached`, `.release` and
   `.acknowledged`, since S4-H scans for `*.reached`, and renaming it into place. `NAME.reached`
   is the server's acknowledgement that it holds the call there with everything before it
   committed. The controller writes `NAME.release` the same way: under a temporary name
   in the same directory, one that does not end in `.release`, renamed into place once written,
   so that the server, which reads only `NAME.release`, reads it only after the rename. The
   READMEs say so. Once the server has committed any ledger line that records the release itself
   (the `reply` line at `after-commit`, or a `dropped` one at any barrier), it creates
   `NAME.acknowledged`, and only then goes on. `drop` means that the server performs nothing
   further for the call and never answers it, and writes a `reply` line saying `dropped`. A file
   it cannot write or read, or a release that holds anything else, acts as `drop` and writes
   `control-lost`. No timer orders anything: the poll is the only wait, the server never gives up
   waiting, and the engine's own timeout is what turns a held call unknown. The barriers:
   - `before-apply`, after `received` and before any entry;
   - `after-commit`, once the call's entries are committed and before its answer, in both modes
     and in the keyed server, a deduplicated attempt included.
6. **The servers.**
   - **Plain**, by `mode`, for both of its operations alike. `complete` writes entries 1 and 2
     under a new record and answers a complete result whose `structuredContent` is `{record}`,
     with a text block naming it. `partial-error` writes entry 1 and answers the JSON-RPC error
     -32021, whose `data.requiredCapabilities` names form-mode elicitation,
     `{"elicitation":{"form":{}}}`, which IFC-045 never lets the engine declare. The server lists
     no extension, neither Conduit's nor the tasks extension.
   - **Keyed** checks IFC-006 as order 136's server does: Conduit's extension at `v0`, or -32021
     naming it (IFC-008); a key that IFC-006 requires, missing or malformed, performs nothing
     and answers failed with a diagnostic naming the key; and a `filters` holding any item
     answers `unavailable` in IFC-026's form, since it supports no requirement. It deduplicates
     across every attempt of one scoped idempotency key (CTR-002, OPS-012). OPS-008 scopes a key
     to its caller, its target and the instance that sent it, and IFC-006 gives an
     implementation no sending instance, so the server finds a record by the instance, caller,
     idempotency key and invocation that the call's `_meta` carries: OPS-008 allows one
     invocation for each scoped key, and a new attempt keeps its invocation's ID. The attempt
     count, the Conduit and the time are never part of it, so an attempt through another Conduit
     finds the same record. Under the lock it writes each part of the record not yet written, and
     no other: a record with both parts is `deduplicated`, and one left with part 1 alone, as a
     process stopped between the two writes leaves it, gets part 2 only. It answers succeeded,
     with `isError` false and the record's `{record}`, the same for every attempt. A call with no
     idempotency key is new work (OPS-011) and gets its own record. The records are the ledger,
     so they hold across a restart of the server (OPS-018).
   - **Reader** answers `read-effects.v1` succeeded with decision 7's readback, filtered by
     `label` where it is given, after the same IFC-006 checks.
   - **Answers to a cancelled request.** rmcp drops the answer to a request that its peer has
     cancelled. So before it answers, each server checks the request's cancellation token
     (`context.ct`): where the request is cancelled, it writes a `reply` line saying `cancelled`
     in place of `released`, and the answer goes nowhere. A cancellation that arrives between that
     check and the send can still drop a `released` answer; the README states this limit.

   Every diagnostic names its cause (ruling 52). An unknown tool is the protocol error -32602,
   with nothing performed.
7. **Readback and receipts, independent of the engine.** One function reads the ledger, under a
   shared lock, and nothing else: no engine store, no record of the engine's, no server's answer.
   It serves `read-effects.v1` and the subcommand `scenario-s4-effect readback LEDGER`, which
   prints the same JSON on stdout, so that S4-H and the tests read the ledger with no engine.
   It reports:
   - each record, with its parts and its state, `whole` or `partial`. It never reports an effect
     as failed or absent: a record with part 1 alone is `partial` (Q5), and a label with no
     record is reported as only that;
   - each call, from its `received` line: the attempt count, the parts it wrote, whether it was
     deduplicated, its reply, `released`, `dropped`, `cancelled` or `none`, and what it answered.
     A call with no `reply` line is open, still held or stopped, and is never treated as ended;
   - by number, each line that does not parse, never dropped silently.

   The receipts tell lost success from unknown. A call whose entries are committed and whose
   success was `released` was answered: the engine kept success if it received the answer, and a
   client that lacks it has a lost success. A call whose reply was `dropped` or `cancelled`, or
   that is open, was never answered: the engine's outcome is unknown. The README states the
   readback's form as v1, and the output schema of `read-effects.v1` states it in full.
8. **Tests**, driving the built binary with an `rmcp` client and the full request `_meta`, with
   a test controller that waits for `.reached` and `.acknowledged` files in a temporary directory
   and writes each release under a temporary name before renaming it. No test waits a fixed time
   in place of a barrier's file, and no test drops or times out the future of a held call, which
   could make the client cancel it, apart from the one test of cancellation below. Compare markers
   with `assert!(a == b)`, never `assert_eq!`. The tests cover:
   - discovery: the plain server's two tools and no extension; the keyed server and the reader at
     `v0`; -32021 without the extension and at another revision, with the ledger unchanged; an
     unknown tool;
   - E01, for each of the two operations: `partial-error` commits entry 1, is held at
     `after-commit` while the readback shows a `partial` record, then answers -32021 with its
     `data.requiredCapabilities`;
   - deduplication: two attempts with the same key, invocation, caller and instance, and other
     attempt counts, apply one effect and answer the same record, and so does a second attempt
     that names another Conduit; two attempts held together at `before-apply` and released
     together apply one effect; the same key from another caller, or under another invocation's
     ID, applies a new effect; a call with no key always applies a new one; a ledger seeded with
     part 1 of a record gets part 2 alone; an attempt sent after the server was stopped at
     `after-commit` and started again is deduplicated;
   - plain `complete`: a second identical call is a second effect, OPS-011's new work;
   - barriers: `drop` at each barrier leaves the call unanswered while a later call to the same
     server is answered, and the readback shows the call open or dropped; an unusable control
     directory stops the server at start, and a release it cannot read acts as `drop` with
     `control-lost`;
   - cancellation: a call held at `after-commit` whose request the client cancels with
     `notifications/cancelled`, then released, writes `cancelled` and no `released`;
   - readback: `whole`, `partial`, a torn last line counted, an open call not treated as ended,
     and `read-effects.v1`'s output equal to the subcommand's; answers validate against the
     output schemas, and `isError` agrees with the outcome;
   - `conduit.yaml` and `contracts.yaml` validate against `spec/schemas/` (skipped, saying so,
     where `spec/` is absent), and the archive is the same, byte for byte, when built twice.
9. **The installation check**, the package installation check that S4's plan asks of every
   server order. A fixture project in `tests/fixtures/project/` holds a `conduit.yaml` that pins
   this package by its address, its version and the literal digest `sha256:PLACEHOLDER`, which
   the check replaces with the built archive's in a temporary copy, and a `templates.yaml` with one
   Resource template of each of the three Types, and no instance. So `submit` registers the
   templates and then opens and sets up nothing: nothing is dispatched to the package's items.
   The commands are in "Done when". Running the servers through Conduit is S4-H's qualification.
10. **Its README** gives the configuration's places, the modes, the barriers and their handshake
    with the atomic release, the cancellation limit, the ledger's and the readback's v1 forms,
    D15's four fields as
    [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them,
    and counts by category as order 136's README gives them, as package and fault preparation
    that M3 counts apart from author work. A `package` binary writes the deterministic archive
    `dist/scenario-s4-effect.tar.gz`, of `bin/scenario-s4-effect`, `conduit.yaml`,
    `contracts.yaml` and `README.md`, and `dist/scenario-s4-effect.tar.gz.sha256`, which holds its
    digest as bare lowercase hex and a newline, as order 136's `src/bin/package.rs` writes it. The
    installation check relies on both.
11. **What this order does not do:** no task variant, which is S4-SRV1b's (no `partial-task`, no
    tasks extension, no `tasks/cancel` and no barrier before a task's terminal error); no setup
    hook (order 144); no engine change and no vector; no generic repair or confirmation API,
    since an author checks the readback and reconciles through the engine's `reconcile.v1`; no
    output-expiry control, which is the engine's (DL10); no model; and no change to any other
    package.

## Scope

Change only `packages/scenario-s4-effect/` (new). The plan sized it L, 650 to 850 lines including
tests. Re-estimated from orders 136 and 139, whose results counted about 500 lines of code and
1,000 of tests each, and with the task variant cut: about 600 to 750 lines of code, 150 to 200 of
manifests, contracts and fixture, and 700 to 900 of tests, so 1,450 to 1,850 in all, beside the
README and the lockfile. Report the counts.

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release`, and `cargo run --release --bin package` twice, with the same digest. Then
the installation check, from the repository root, as one `bash` invocation (a script or
`bash -c`), so that its variables and the background `serve` hold throughout; it runs one cargo
command. It stops before `init` if the port already answers, and its wait ends if `serve` exits,
so that another program's listener cannot pass for the engine. Set `P` to another free loopback
port if 38143 answers, and say which:

```bash
D=packages/scenario-s4-effect
P=38143
(cd engine && cargo build --release)
E=$PWD/engine/target/release/conduit
H=$(cat $D/dist/scenario-s4-effect.tar.gz.sha256)
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
"$E" install --store "$L/store" "$PWD/$D/dist/scenario-s4-effect.tar.gz#$H"
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
D=packages/scenario-s4-effect
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" \
  $D --exclude-dir=target --exclude-dir=dist
grep -n -E "^  name: " $D/contracts.yaml | grep -v -E "name: (apply-effect|start-model-work|\
read-effects|effect-writing|model-work|effect-reading|effect-plain|effect-keyed|effect-ledger)\.v1$"
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

`target/` and `dist/` stay ignored and unstaged. If the sandbox refuses something a test or the
installation check needs, list what did not run, never as passed. Report each choice of
configuration, form or name that this order leaves open as your own, and these as findings: that
IFC-006 gives an implementation no sending instance, so the keyed server scopes a key by the
invocation's ID (decision 6); the loader's verdict on an operation marked `startsModelWork: true`
on a Resource, since model work is an Agent's (decision 2); and that a peer's cancellation can
still drop an answer after the server's check (decision 6). Do not stage anything.
