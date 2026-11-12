# Handover: the coordinator's work, for the session that takes it over

Not binding; [spec/](../../spec/README.md) alone binds, and [IN_FORCE.md](../../IN_FORCE.md) holds
every other decision in force (ruling 67). The coordinator's work on 2026-11-12, when the admin
session handed it to a new coordinator session because the owner's usage was near its limit. The
new session's first task is the owner's [full review](../guides/full-review.md). Rulings, orders
and pull requests are cited by number; a fact found only in the coordinator's private notes is
marked "(log)" and is still to be recorded. Machine details (paths, hosts, accounts, the runners)
stay outside the repository, in the private handoff note: `.wo/notes/HANDOFF-local.md` beside the
main checkout. Read with it [IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed),
[work orders](WORK_ORDERS.md) and [what waits for the owner](../AUDIT.md#waiting-for-the-owner).

## Roles

| Who | Does | Rulings |
| --- | --- | --- |
| The owner | Rules on behavior, on what Conduit is for and on the order of the work. May strike any coordinator decision logged since ruling 57. Answers numbered questions in batches ("1a 2 3c") | 37, 38, 46 point 4, 57, 58 point 1 |
| The admin session: the coordinator and merge authority | Plans; writes each order and the decisions it applies; reviews; runs the checks; commits, opens and merges pull requests; records rulings. Sole merge authority for the engine's core, the conformance runner and manifest, `spec/`, IN_FORCE.md and the ledger. Its orders are numbered below 100 (037 to 082 so far) | 48, 71 point 1 |
| The lane coordinator | One lane, scenarios, providers and proof, in its own worktree, with orders 100 to 199. Merges only pull requests confined to its lane's paths (`packages/`, `experiments/`, a provider's crate), once CI is green and a fresh verifier is clean. A conflict between lanes goes to the admin; a behavior question to the owner. It was a subagent of the outgoing session: [restart it](#restarting-the-lane-coordinator) | 71 point 1 |
| Codex workers and verifiers | A worker carries out one order in a tree of its own, commits nothing and answers in the order's JSON schema. A fresh read-only Codex verifier checks each round | 48 ([how the work is done from now on](../proposals/ruling-48/README.md#how-the-work-is-done-from-now-on)); [work orders](WORK_ORDERS.md) |

How the rulings shape the work:

- **48.** The coordinator spends its session on decisions, not execution. Codex runs on the owner's
  ChatGPT subscription. Fresh Codex reads test what a reader without context can do. A78 sends the
  orders through Conduit.
- **58.** Decisions logged before ruling 57 are accepted; later ones stay open to strike (point 1).
  When a batch or an increment closes, its papers condense into the registers, and the work-order
  record replaces merged orders (points 3, 4 and 10; A80).
- **67.** Four documents, one question each: `spec/` binds, IN_FORCE.md holds other decisions, the
  ledger what was ruled, a record why. A number only for a new decision; a clarification is a dated
  later answer in the ruling it clarifies (42, 52, 67, 68, 71, 72, 75 and 76 have them). One
  change carries a ruling everywhere.
- **70.** Agents run code, unconfined in v0. Conduit building Conduit (A78) is the first priority;
  every friction of dogfooding is a finding of the first build.
- **71.** Two or three coordinators, one merge authority (point 1). Batch 2 on a branch, merged
  after the owner judges S1's bar (point 2). The bar's rule is fixed (point 3). Small findings that
  add a feature are delegated (point 4). When the ChatGPT Codex allowance runs out, the workers
  pause and the owner is told, with no API key ([later answer of 2026-11-12](../proposals/ruling-71/README.md#later-answers)).
- **74.** `main` is protected before Conduit runs its own orders: pull requests only, for
  administrators too, and no force-push or deletion (point 1).

## The work-order pipeline

[Work orders](WORK_ORDERS.md) gives the design: an order's parts, the dispatch flags, the verifier
and [the lessons](WORK_ORDERS.md#what-this-round-taught). As practised on 2026-11-12:

1. **Write.** An order is `docs/work-orders/NNN-name.md`: a preamble (read AGENTS.md; the report
   schema; what the network is for; never print the environment or a whole large file; one cargo
   command at a time), then What and why, The decisions it applies (each with its source), What to
   do, Scope, Done when (commands) and, at merge, its Record. Its schema is its own
   `NNN-name.schema.json` or the shared `004-i2-01-vectors-and-first-start.schema.json`, named at
   dispatch with `SCHEMA=`. The order's file merges to `main` first, on a branch `orders/NNN`
   (#245, #258): the runners read order files from `origin/main`, whatever the base.
2. **Dispatch.** A runner kept outside the repository makes a worktree on `wo/NNN-name` from
   `BASE` (default `origin/main`; a stacked order names the unfinished branch). It starts the worker
   with `codex exec --ignore-user-config --strict-config --disable apps -s workspace-write`, with the
   network only for `net` orders (crates, loopback ports), and with secret-like environment
   variables removed by name. It checkpoints the worker's output as a local commit. `EXTRA` appends
   a section "From the admin session" (080, 081). `JOBS` and `TT` cap cargo's jobs and test threads.
3. **Verify and rounds.** A fresh read-only Codex session reads [verify.md](../work-orders/verify.md),
   the order and the report, and answers must, should or nit in `verify.schema.json`. A must or a
   should starts another worker round and a fresh verifier, up to `ROUNDS` (default 2).
4. **Rounds after the fact.** A second runner continues from a verifier report (`START_ROUND=N`;
   `VERIFY_FIRST=1` when a worker finished but its verifier never ran, as 079's did). The admin may
   write that report itself to drive a round: to continue a checkpoint (070, 077 and 081 after the
   Windows pause) or to fix a merge (042 after its rebase).
5. **Finishing orders.** When the rounds end with musts left, a finishing order is stacked on the
   branch and the pair merges together: 124 on 122, 125 on 123, 130 on 129, 148 on 142; 149 and
   150 on 146, the three as one (#260).
6. **Review and merge.** The admin reads the diff, the report and each choice the worker made
   itself; runs every check of AGENTS.md and, for a Rust directory, its tests, fmt and clippy; scans
   for private paths. It squashes with the guard (below), checks that the squash's files stay in the
   order's scope, pushes `wo/NNN-name`, opens a pull request, waits for CI and squash-merges with a
   message written by hand. The Record goes at the end of the order's file: dates, rounds and
   times, verifier findings, the admin's changes and own decisions, and the worker's findings kept
   for later ([081's Record](../work-orders/081-spec-one-run-per-instance.md#record) is the model).
   Orders 037 to 045, 049a1, 050a and 067 onward still sit in `docs/work-orders/`; moving them
   into [the record](WORK_ORDERS.md#the-record) is owed (A80). Order 067 moved 001 to 036k and 060
   to 066 into it.
7. **Orders built on Windows** pass the changed crate's tests, fmt and clippy on the Mac before
   merging. Order 131 did not compile on macOS (rustix's `mknodat`) and expected Linux's error
   message; 137 and 140 used `chmod --` and `find -printf` (fixed in #241). Windows' sandbox could
   not use the pinned toolchain for 075's spec suites, and timed out 072's S1 HTTP test (log).
8. **CI** ([checks.yml](../../.github/workflows/checks.yml)) runs on every pull request and every
   push to `main`: the docs job (the six Python checks, the tools' unit tests, `git diff --check`);
   fmt, clippy and tests per Rust directory; the engine's tests as a nextest archive in four
   partitions; doctests. A nightly job runs the engine with npm. Order 068 built it (#198); since
   070 (#240) an engine shard takes 1 to 8 minutes, from 12 to 38.
9. **Small findings (71 point 4).** Instead of another round, a coordinator may fix a small last
   finding and log it in the Record: the last race of 122 and 124 (#207), 128's mips byte order,
   136's redaction of a library-built error, 137 and 140's macOS fixes (#241), 141's extension
   (#249). Until the S1 rerun, nothing that could move S1's author counts: CLI flags, short-form
   keys, templates and defaults.

## Restarting the lane coordinator

The lane coordinator was a subagent of the outgoing session, which a new session cannot reach. If
the lane is to go on, start a new lane coordinator with this brief; the private handoff note gives
its worktree, its logs and the machines.

- **Its role** (ruling 71, point 1): the lane of scenarios, providers and proof. It writes and
  dispatches its own orders, numbered 100 to 199, through the same runners, and logs each decision
  for the owner to strike. A conflict between lanes goes to the admin, a behavior question to the
  owner.
- **What it merges**: only pull requests confined to `packages/`, `experiments/` and its own order
  files (100 to 199) under `docs/work-orders/`, with [PROVIDERS.md](PROVIDERS.md) and its plans.
  Anything else, `engine/`, `spec/`, `conformance/`, IN_FORCE.md and the ledger among it, goes to
  the admin.
- **Each merge** needs CI green and a clean fresh verifier; for an order built on Windows, the
  changed crates' tests, fmt and clippy run on the Mac (item 7 above); and the guarded squash
  ([the squash guard](#standing-rules-learned-this-session)).
- **Its queue** is row 14 of [the queue](#the-queue). Tell it when 071 merges (SV1 unblocked) and
  when the checkpoint ends.

## Dogfooding: Conduit runs its own orders (A78)

Stage 1 (ruling 70; ruling 61 B12): root sends one order at a time through turns on two `codex.v2`
Agents of the `conduit-core` Fleet, `worker-1` (4-hour deadline) and `verifier-1` (1 hour), in a
separate clone that cannot push. A harness outside the repository keeps the runner's interface and
log lines, restarts the engine for each fresh conversation, validates each report against the
order's schema (one repair turn), and checks guards after each turn: the slot's HEAD unmoved, the
verifier changed nothing, nothing pushed, the main checkout untouched. The Agent packages come from
076's branch: the packages of 050a, 076 and 078 stay unmerged until 050, because NAM-024 forbids
adding a default once `.v2` is published ([batch 1, order 050](../proposals/spec-batch-1/README.md#the-work-orders)).

What worked (log):

- The smoke test, on an engine with 074: six of six checks pass, after two of the harness's
  matchers were fixed. The Agent's own tools work in the slot; `codex.v1`'s tools write nothing;
  `claude-code.v2` runs with no prompt; a 1273-second turn keeps its session (074); a 60-second
  deadline names itself; a background job does not hold the turn.
- 042 is the first order run by Conduit itself: four rounds, every completed turn a valid report,
  no guard tripped except once, in the outage's failed turn, when GitHub's refs could not be read.
  Its first worker turn took 11966 s, inside the deadline; round 4 was clean.

What failed or cost (log):

- A brief network outage on the Mac failed 042's round-3 worker turn after 55 s: "client turn 1
  failed: client failure: other; session cannot restart implicitly", outcome unknown. Plain
  `codex exec` workers reconnected by themselves. The Agent's turn should survive a transient
  network loss, or name it.
- `claude-code.v2` wrote inside the slot's `.git` with no prompt: unconfined, as ruling 70 allows.
- Each fresh conversation costs an engine restart: 171 to 191 s in 042's round 4.
- In the smoke test, Codex's terminal (PTY) children escaped the run's process-group kill; the
  harness lists and stops what a turn leaves behind.

None is yet a finding in [increment 4's log](../proposals/first-build/log-increment-4.md); ruling
70 asks that each be. Next: rebuild the dogfood engine on `main` (070 is in), then run 046 and 047
through it; orders 4 and 5 need appointment (batch 2). The plan's follow-ups: a fresh conversation
without a restart (`newConversation` on `turn.v1`, on 050a's branch), and cancelling the provider
call when a dispatch deadline fires, a fault fix before the rerun. After 050, the store is
initialized again and the owner signs each slot Agent in once.

## The second machine

The owner's Windows computer runs Ubuntu-24.04 under WSL2, reached over Tailscale and SSH (72
point 3; 76 point 6; 77).

- Until it is enrolled as a node, ordinary workers run there over SSH as on the Mac: a runner
  starts the remote copy, follows its log and fetches `wo/NNN-name` back (76 point 6). Conduit-run
  orders stay on the Mac until enrollment exists.
- **On demand** ([72's later answers](../proposals/ruling-72/README.md#later-answers)): the computer
  is Conduit's whenever it is on and the owner has not said "pause". On "pause", commit each
  order's partial work on its branch, fetch it to the Mac, stop everything there, and dispatch
  nothing until "resume"; then check SSH first. Once interop is off, the owner stops the distro with
  `wsl --terminate Ubuntu-24.04`. On the pause of 2026-11-11, 070, 077 and 081 were checkpointed
  and continued on the Mac.
- **Ruling 77's setup**: the SSH providers SSH-M, not built, and SSH-R (merged by 137, 140 and
  145, and its drainer by 146, 149 and 150 in #260; its key sweep, 147, is open); one run user,
  `conduit-run`, with no sudo, PRV-025's isolation stated unmet; the engine's certificate through
  `tailscale serve` once the Mac is renamed in the tailnet; interop, `appendWindowsPath` and
  automount off. The owner runs the steps that need sudo, and the coordinator signs nothing in.
  Part 1 is done: `conduit-run` has its tools, the owner signed its Codex in with the primary
  account, and the coordinator reaches it over SSH. Part 2, bubblewrap if it is missing, the
  distro's restart that applies the lockdown, and taking the coordinator's key out of the owner's
  account, waits until 080 and 071 finish; orders then run as `conduit-run` only. Ask the owner
  before part 2 (log).
- **Capacity**: WSL keeps its default 15 GB (M2-7). Five orders reached 12 GB; before 070, shell
  test binaries took 5 to 8 GB each and ran it out of memory once (log).
- [The second-machine guide](../guides/second-machine.md) gives ruling 77's setup and S3-OF's
  options.
- Its Tailscale name and address, user names, host-key fingerprint and accounts never enter the
  repository.

## The queue

The order is the roadmap's step 3: batch 1, then the S1 rerun, then the bar; the lanes of steps 4 to
6 run beside it. State at the handover, `main` at `7e963b1a`; the tracker's
[at the handover](../AUDIT.md#at-the-handover-of-2026-11-12) gives it in short. Once the running
items finish, the work stops for the checkpoint.

| # | Item | State | Next step |
| --- | --- | --- | --- |
| 1 | 042 (E4): tombstones, lift, inventory, provenance | Four rounds through Conduit, round 4 clean. The admin rebased it onto `main` with 070 and 081, resolving conflicts in the core's `access.rs` and `lib.rs`; one vector then fails, `e4-tombstone-register-replacement` step 6 (LIF-041 in register-replacement). A fix round runs (`ROUNDS=2`) | Once clean: guarded squash, every check and the engine's tests, pull request, merge, Record with A78's measures |
| 2 | 077: fenced reconciliation, and the catalogue's fixes (75 A1) | Continued on the Mac from its Windows checkpoint. Round 2's one must: an existing Fleet vector outside the order's scope expects a reconciliation to succeed without the fence's evidence (443 of 444 vectors pass). Round 3, the last, runs | Review; recount `spec/README.md`'s totals (519 promises and 443 vectors since 081); merge |
| 3 | 049a1: enrollment in the core, on the side branch `s3/m2` | Round 5 runs. Every round finds new musts: request attribution, keyed repeats, stale reports, overlapping admissions; in round 4, setup refused for a mere external implementation, and provider outputs through the generic tool path. Its rounds run from `main`'s `c8e9aa20`, not from `s3/m2`: rebasing onto `s3/m2` conflicts in `conformance/manifest.yaml` and the shell's `server.rs`, left for the merge | Read round 5's findings before any finishing order (rules below); merge into `s3/m2`; then SSH-M |
| 4 | 080 (DSP): the shell's runtime-provider calls | Runs on Windows, stacked on 079's branch, with the gate's session rules as `EXTRA` | At merge, rebase onto `s3/m2` (079 drops out); Mac tests |
| 5 | 071 (S0a): the conformance manifest in pieces | Runs on Windows | Once merged, tell the lane that SV1 is unblocked. Once 080 and 071 finish, part 2 of ruling 77's setup ([the second machine](#the-second-machine)) |
| 6 | Lane: 144 (S4-SRV2, S4's hook server) | The lane coordinator's. Queued, not started: its dispatch waits for the Mac's load to fall under 8, so it may start after the checkpoint | Merged in the lane |
| 7 | The admin's next spec order | Not written. PRV-021's owed stop in three cases, matching kit order 141; PRV-015 names a machine by ID where `place.v1` gives a hostname; "labels last reported" (PRV-027, LIF-036, `stop.v1`); one live node per machine (049a1); what an engine owes when it cannot store its state, and races across addresses ([082](../AUDIT.md#known-to-be-open)); 081's Record's leftovers | Write after 077 merges |
| 8 | The admin's small engine order | Not written. The first-start report maps Rust's architecture to Go's `GOARCH` wrongly (`loongarch64`, little-endian `mips` and `mips64`, `armbe`, `arm64be`, `riscv32`, `wasm`); mirror order 128's mapping (PRV-014) (log) | A small order, or a fix under 71 point 4 |
| 9 | Batch 1's rest, on the rerun path | Not written: 046, 047, 048a, 048b, 050, 053; joining them, 043a, 043b, 052a, 052b, 051a, 051b ([the plan](../proposals/spec-batch-1/README.md#the-work-orders)) | 046 after 042, through Conduit |
| 10 | The sign-in chain | Not written: 050 (each Agent's own home, the hooks, the account check; merges with 050a, 076 and 078 as one 0.2.0), 048a, 048b, 051b; then A81 (a carried Codex login) and [A82](../../ROADMAP.md#experiments-and-probes) (Linear's OAuth through the Interface) | The second accounts wait for this: each Agent signed in through the Interface, as the test of its own subscription and of the model ([72's later answers](../proposals/ruling-72/README.md#later-answers)) |
| 11 | Off the rerun path | 049b, 051c, SSH-M; `s3/m2` merges after the rerun's pin | After 049a1 |
| 12 | The S1 rerun and the bar | 053, a pinned tag, the rerun with the owner present, the owner's judgment (71 points 2 and 3) | After batch 1 |
| 13 | Batch 2 | Not begun. On its own branch, merged after the bar (71 point 2). Holds appointment (A78's orders 4 and 5), MC9 ([G5](../../experiments/012-model-checking/G5.md#what-g5-still-needs)), and apply calling the control a Type offers for a changed control value (OPS-020; from 069b's reference, for S4's T1) | After the rerun's pin |
| 14 | The lane's queue | Merged: S2 to S10's plans (117 to 121, 133, 134); A63's kit, contract harness and local providers ([PROVIDERS.md](PROVIDERS.md)); S2's stub, Conduit-aware implementation and adapter; S4-SRV1; SCR1; SSH-R's drainer (146, 149 and 150, #260). Open: SSH-R's key sweep (147), S4-SRV2 (144) | The lane coordinator; S2-PL after the owner's choice, S3-H after S3-OF |
| 15 | Records owed | Orders 037 to 045, 049a1, 050a and 067 onward into the record (A80); review-list items 30 onward (below); A78's findings in increment 4's log | A docs pass |

## The owner's open items

- **Ruling 77's setup** ([what follows](../proposals/ruling-77/README.md#what-follows)). Part 1 is
  done: `conduit-run`, its tools, and its Codex signed in by the owner. Part 2, after 080 and 071:
  bubblewrap if missing, the restart that applies the lockdown, and the coordinator's key taken out
  of the owner's account. Before the end-to-end run: the Mac renamed in the tailnet, then HTTPS
  certificates turned on.
- **S3-OF**, S3's cut, heal and loss: A, the owner's hand at each barrier, 18 actions a run (9 to
  18 extra owner minutes; 72 actions for the four runs), recommended; B, a Windows SSH account
  limited to those commands; C, a fault user with one sudo rule. It gates S3's fault harness and
  runs ([S3's plan](../../experiments/014-s3-two-machines/plan.md#for-the-owner)).
- **S2's plain server**, D01's third row, before S2's runs: no open-source Linear MCP server runs
  unchanged against the stub ([S2's plan](../../experiments/013-s2-shared-login/plan.md)).
- **S1's bar** after the rerun, with findings 4-01 and 4-11 and C6 to C12 ([the tracker](../AUDIT.md#waiting-for-the-owner)).
- **A82's application**: a Linear OAuth application the owner registers (S2-LOG-O).
- Later, not urgent: G5's 11 exclusions, accepted or modelled; a repository and an npm scope for
  S7's outside author ([S7's plan](../../experiments/018-s7-outside-package/plan.md)); S8-A,
  accepting S8's concrete Requirement and contracts before any code
  ([S8's plan](../../experiments/019-s8-filtered-search/plan.md)); A74's unmeasured cells, and the
  reading of ruling 75 D4 that counted runs wait on
  ([S3's plan](../../experiments/014-s3-two-machines/plan.md#for-the-owner)); A20 after A7; G8 and
  G11; whether runtime providers that use the kit by path are "self-contained"
  ([PROVIDERS.md](PROVIDERS.md), IN_FORCE topic 5). The tracker's
  [list](../AUDIT.md#waiting-for-the-owner) is the whole of it.

## Decisions the owner may strike

Items 1 to 29 of [the review list](../proposals/first-build/to-review-with-the-owner.md) stand: the
owner confirmed them on 2026-11-10 and 2026-11-12. Logged since, and still to be added to it:

- Order 038c: four login kinds (`device`, `redirect`, `page`, `service`); `clientId` on every login;
  no return through `tasks/update` in v0; the client-ID exemption only for one package's own Types
  and versions.
- Order 081: MC4-009 and MC4-001 fixed as defects; MC6-001 follows OPS-022 and LIF-024; LIF-019
  narrowed for removal (MC6-002).
- Order 082: concurrent first installs of one address and version decided one at a time.
- Order 070: lazy publication is right; the differential reference was fixed, not the lazy path.
- 049a1: one live node per machine; acceptances decided one at a time; a second refused with a
  conflict naming the node (log).
- Order 145: SSH-R's drainer keeps its state in an operator-set `durableDir` outside
  `CONDUIT_STATE_DIR`; the SSH key is deleted once nothing is owed.
- Providers: option (b), packaged local providers built and passing the contract suites
  ([PROVIDERS.md](PROVIDERS.md)); "self-contained" read as a crate's own lockfile and toolchain.
- S2: the "small adapter" is the glue that puts an existing tool behind Conduit, the same for S7;
  the plain row runs only in the owner's S2 runs ([S2's plan](../../experiments/013-s2-shared-login/plan.md)).
- S4's T1 repair: the author corrects a `blockedBy` configuration place through Conduit
  ([S4's plan](../../experiments/015-s4-partial-setup/plan.md)).
- A6's traces come from S2's and S4's runs ([the register](../../ROADMAP.md#experiments-and-probes)).
- The side branch `s3/m2` for the Windows engine work, merged after the rerun's pin (log).
- "Windows computer is up again" read as the resume after the pause of 2026-11-11 (72's later answers).

## Standing rules learned this session

- **No environment dumps.** On 2026-11-11 an agent printed a session's environment, which held
  keys (log). Every brief that may run a command says: never run `env`, `printenv`, `set` or
  `export -p`, nor print a variable's value. Probes start clients under `env -i` with fake keys.
- **Credentials are never read**: no key, token or login file, the owner's or Codex's. Workers never
  edit the owner's configuration (AGENTS.md).
- **Private details never enter the repository**: no local path, user name, address, host name or
  account. [check_private_paths.py](../../tools/check_private_paths.py) checks the tracked files;
  a literal scratch path from 122 and 123 failed CI once (#205). History keeps old versions (71
  point 5).
- **The squash guard.** Worktrees share the remote-tracking refs, and another session's fetch can
  move `origin/main` between two commands; the squash of 041b (1091a71) undid MC8's merge, restored
  in #190. Squash only in one command, `git fetch origin && git rebase origin/main && git merge-base
  --is-ancestor origin/main HEAD && git reset --soft origin/main && git commit`, then check
  `git diff --name-only HEAD^ HEAD` against the order's scope.
- **zsh**: brace a variable before a colon in a refspec, `${NAME}:refs/...`; `$NAME:r` is a
  modifier. An unmatched glob aborts the whole command.
- **Load.** The lane coordinator runs one heavy (engine-building) order on each machine; the admin
  ran at most two engine orders on Windows until 070 merged. On the Mac, dispatch only while the
  load is under 8, and cap `JOBS` and `TT`. Under load, rerun timing-sensitive shell tests with
  `RUST_TEST_THREADS=2` (log).
- **Don't start finishing orders blindly.** Read the last verifier's findings and the branch first.
  When each round finds a new fault of one kind, the design is wrong: 149's drainer handover raced
  in every round, and [150](../work-orders/150-ssh-r-one-drainer.md) replaced it with one drainer.
  Two of 081's dispatches failed at once, for a missing `SCHEMA` and a leftover branch (log).
- **Keep work out of session scratch space** (order 019's run was lost) and never print a whole
  large file (order 028) ([lessons](WORK_ORDERS.md#what-this-round-taught)).
