# Handover: the coordinator's work, for the session that takes it over

Not binding; [spec/](../../spec/README.md) alone binds, and [IN_FORCE.md](../../IN_FORCE.md) holds
every other decision in force (ruling 67). The coordinator's work on 2026-11-12, when the admin
session handed it to a new coordinator session because the owner's usage was near its limit. The
new session's first task was the owner's [full review](../guides/full-review.md), carried out on
2026-11-14: [its record](../proposals/full-review/README.md) holds the questions, which the owner
answered the same day in [ruling 78](../proposals/ruling-78/README.md), and the work orders it
adds. Rulings, orders and pull requests are cited by number; a fact found only in the coordinator's private notes is
marked "(log)" and is still to be recorded. Machine details (paths, hosts, accounts, the runners'
scripts) stay outside the repository, in the private handoff note: `.wo/notes/HANDOFF-local.md`
beside the main checkout. [The method as run](METHOD_AS_RUN.md) gives the runners' steps, with each
private value a variable. Read with it
[IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed),
[work orders](WORK_ORDERS.md) and [what waits for the owner](../AUDIT.md#waiting-for-the-owner).

## Roles

| Who | Does | Rulings |
| --- | --- | --- |
| The owner | Rules on behavior, on what Conduit is for and on the order of the work. May strike any coordinator decision logged since ruling 57. Answers numbered questions in batches ("1a 2 3c") | 37, 38, 46 point 4, 57, 58 point 1 |
| The admin session: the coordinator and merge authority | Runs on Claude Opus (ruling 79). Plans; writes and dispatches each order and the decisions it applies; reviews each diff, beside the Opus reader's; runs the checks; commits, opens and merges pull requests; records rulings. Sole merge authority for the engine's core, the conformance runner and manifest, `spec/`, IN_FORCE.md and the ledger. Its orders are numbered below 100 (037 to 082 so far) | 48, 71 point 1 |
| The lane coordinator | One lane, scenarios, providers and proof, in its own worktree, with orders 100 to 199. Merges only pull requests confined to its lane's paths (`packages/`, `experiments/`, a provider's crate), once CI is green and a fresh verifier is clean. A conflict between lanes goes to the admin; a behavior question to the owner. It was a subagent of the outgoing session: [restart it](#restarting-the-lane-coordinator). Not run until the S1 rerun's pin (78 point 1) | 71 point 1, 78 point 1 |
| Workers and verifiers | A Codex worker (or Claude Opus) carries out one order in a tree of its own, commits nothing and answers in the order's JSON schema. A fresh read-only Codex verifier checks each round; a fresh Opus reader verifies the order before it merges, and when the Claude allowance runs low, dispatch stops and nothing merges unread. Fable is kept for the owner's reviews of the whole | 48 ([how the work is done from now on](../proposals/ruling-48/README.md#how-the-work-is-done-from-now-on)); [ruling 79](../proposals/ruling-79/README.md); [work orders](WORK_ORDERS.md) |

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
- **71.** Two or three coordinators, one merge authority (point 1), from the S1 rerun's pin under
  ruling 78. Batch 2 on a branch, merged after the owner judges S1's bar (point 2). The bar's rule
  is fixed (point 3), and ruling 78 scores it in ruling 45's two groups. Small findings that add a
  feature are delegated (point 4). When the ChatGPT Codex allowance runs out, the workers
  pause and the owner is told, with no API key ([later answer of 2026-11-12](../proposals/ruling-71/README.md#later-answers)).
- **74.** `main` is protected before Conduit runs its own orders: pull requests only, for
  administrators too, and no force-push or deletion (point 1).
- **78.** The full review's answers: one coordinator and no lane until the S1 rerun's pin, with
  sign-in before the pin (point 1); S1's bar in two groups (point 2); v0's feature list closed
  (point 3.1); a reconciliation only records (point 5); no fence and no "did not occur" (point 6).

## The work-order pipeline

[The method as run](METHOD_AS_RUN.md) gives every step, from writing an order to its Record, with
each command and each runner variable. [Work orders](WORK_ORDERS.md) gives the design and
[the lessons](WORK_ORDERS.md#what-this-round-taught).

## Restarting the lane coordinator

The lane coordinator was a subagent of the outgoing session, which a new session cannot reach.
Under ruling 78, point 1, the lane restarts only at the S1 rerun's pin; then start a new lane
coordinator with this brief. The private handoff note gives its worktree, its logs and the machines.

- **Its role** (ruling 71, point 1): the lane of scenarios, providers and proof. It writes and
  dispatches its own orders, numbered 100 to 199, through the same runners, and logs each decision
  for the owner to strike. A conflict between lanes goes to the admin, a behavior question to the
  owner.
- **What it merges**: only pull requests confined to `packages/`, `experiments/` and its own order
  files (100 to 199) under `docs/work-orders/`, with [PROVIDERS.md](PROVIDERS.md) and its plans.
  Anything else, `engine/`, `spec/`, `conformance/`, IN_FORCE.md and the ledger among it, goes to
  the admin.
- **Each merge** needs CI green and a clean fresh verifier; for an order built on Windows, the
  changed crates' tests, fmt and clippy run on the Mac
  ([the method as run](METHOD_AS_RUN.md#5-on-the-windows-computer)); and the guarded squash
  ([the squash guard](#standing-rules-learned-this-session)).
- **Its queue** is row 14 of [the queue](#the-queue): Lane SP checkpointed in #262, and
  [PROVIDERS.md's status](PROVIDERS.md#status) lists its queue in order. Tell it that 071 has merged
  (#281), so SV1 is unblocked, and when the checkpoint ends. Under [ruling
  78](../proposals/ruling-78/README.md), point 1, it restarts only at the S1 rerun's pin.

## Dogfooding: Conduit runs its own orders (A78)

Stage 1 (ruling 70; ruling 61 B12): root sends one order at a time through turns on two `codex.v2`
Agents of the `conduit-core` Fleet, `worker-1` (4-hour deadline) and `verifier-1` (1 hour), in a
separate clone that cannot push. A harness outside the repository keeps the runner's interface and
log lines, restarts the engine for each fresh conversation, validates each report against the
order's schema (one repair turn), and checks guards after each turn: the slot's HEAD unmoved, the
verifier changed nothing, nothing pushed, the main checkout untouched. The Agent packages come from
076's branch: the packages of 050a, 076 and 078 stay unmerged until 050, because NAM-024 forbids
adding a default once `.v2` is published ([batch 1, order 050](../proposals/spec-batch-1/README.md#the-work-orders)).

What worked, the smoke test's six checks and order 042's four rounds, is recorded in
[A78's results for order 042](../../experiments/028-a78-dogfood/results-2026-11-12.md), beside the
engine's own records of 042's turns and [the harness's method as run](../../experiments/028-a78-dogfood/README.md).

What failed or cost is logged in
[increment 4's log](../proposals/first-build/log-increment-4.md) as findings 4-21 to 4-27, each
with its settlement: the network outage (4-21), the write inside a slot's `.git` (4-22), the
leftover processes (4-23), the restart for each fresh conversation (4-24), the guard that
tripped (4-25), the turn of hours (4-26) and the sub-agents' reach (4-27).

Next: re-pin the dogfood engine to `main` (it is still pinned at `38c0b4f`,
before 070), then run 046 and 047 through it; orders 4 and 5 need appointment (batch 2).
After 050, the store is
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
  account, can run now: 080 is checkpointed and the computer is idle; orders then run as
  `conduit-run` only. Ask the owner before part 2 (log).
- **Capacity**: WSL keeps its default 15 GB (M2-7). Five orders reached 12 GB; before 070, shell
  test binaries took 5 to 8 GB each and ran it out of memory once (log).
- [The second-machine guide](../guides/second-machine.md) gives ruling 77's setup and S3-OF's
  options, which ruling 78 point 4 answered with option (E); its dated amendment comes with the
  review's orders.
- Its Tailscale name and address, user names, host-key fingerprint and accounts never enter the
  repository.

## The queue

The order is the roadmap's step 3: batch 1, then the S1 rerun, then the bar. Until the pin one
coordinator runs it with the small orders beside it, and the lanes of steps 4 to 6 restart at the
pin (ruling 78, point 1). State at the end of the outgoing session, `main` at `57813f06`; the tracker's
[at the handover](../AUDIT.md#at-the-handover-of-2026-11-12) gives it in short. The work has
stopped for the checkpoint; nothing runs, and the Windows computer is idle.

**`main`'s CI was fixed on 2026-11-14 (#266).** Three tests failed at times on CI's Linux runner,
not one, and none was a race in the code under test. In `packages/provider-ssh-runtime` an owner
lock taken with `flock` stays held while a child that another test thread forked still shares the
open file description; the tests now retry their single attempts for a bounded time. In
`packages/runtime-provider-kit` tracing-core cached a callsite's interest from the thread that first
reached it, so a scoped capture saw nothing; the test keeps a second dispatcher alive. In
`packages/provider-local-process` the backend proves a run's group gone only once its last member
is reaped; the test waits for the stopped state. Six CI runs in a row passed.

| # | Item | State | Next step |
| --- | --- | --- | --- |
| 1 | 042 (E4): tombstones, lift, inventory, provenance | Merged (#264, `57813f06`): the first order whose four rounds ran entirely through Conduit's own Agents (the fix round after its rebase ran on the plain runner), round 4 clean. Rebased onto `main` with 070, 081 and 077; a Codex round fixed the one vector the rebase broke (`e4-tombstone-register-replacement`, LIF-041), and the admin made Fleet reconciliation match a sender by ID only. 494 vectors pass on the normal and differential builds; the differential vector run takes about 2.4 hours on the Mac | Two vector coverage gaps, kept for later in [its Record](../work-orders/042-build-tombstones-and-inventory.md#record): the inventory-reading vector holds no Fleet holder; node-requests' readmission injects the expected node |
| 2 | 077: fenced reconciliation, and the catalogue's fixes (75 A1) | Merged (#263, `575a1bcd`) on local evidence: the engine's suite and 446 vectors passed on the Mac, and CI's engine shards were skipped (CI was fixed later, #266). An admin-side fix fenced 041b's vector `fleets/e3b/reconcile-control` before its reconcile ([its Record](../work-orders/077-spec-reconcile-fenced.md#record)) | The MC2 rerun, which checks the rule ruling 78 point 6 chose, in the sequence to the pin; MC4's re-registration |
| 3 | 049a1: enrollment in the core, on the side branch `s3/m2` | Checkpointed after five rounds, not merged; `wo/049a1-build-enrollment-core` pushed (`cdf146d5`). Left: one must, `conformance/manifest.yaml`'s PRV-014 coverage lists only the old cases and misses the new enrollment cases; one should, `enrollment/admit-accept-failed` does not assert the failing step's diagnostic. Its rounds ran from `main`'s `c8e9aa20`, not from `s3/m2`: rebasing onto `s3/m2` conflicts in `conformance/manifest.yaml` and the shell's `server.rs` | Fix both (71 point 4) with a fresh verifier; rebase and merge into `s3/m2`; its one live node per machine needs ENR text (row 7); then SSH-M |
| 4 | 080 (DSP): the shell's runtime-provider calls | Checkpointed after two rounds on Windows, not merged; `wo/080-shell-runtime-provider-calls` pushed (`80841bc7`), stacked on 079's branch. Three musts in the shell's `http_bridge.rs`: an empty SSE priming event ends the bridge before initialization; EOF after the initialization line, while its response is pending, can hang the bridge; mandatory baseline HTTP headers are omitted | Round 3 from round 2's verifier report, with the same `EXTRA` (the gate's session rules), on the Mac or on Windows as `conduit-run`; Mac tests; rebase onto `s3/m2` (079 drops out); merge into `s3/m2` |
| 5 | 071 (S0a): the conformance manifest in pieces | Merged on 2026-11-15 (#281, `2318ec59`) after a third round on the Mac; at the handover it was checkpointed after two rounds on Windows, with one must left ([its Record](../work-orders/071-conformance-manifest-pieces.md#record)) | Tell the lane, when it restarts at the pin, that SV1 is unblocked |
| 6 | Lane: 144 (S4-SRV2, S4's hook server) | Stopped unmerged at Lane SP's checkpoint (#262); `wo/144-s4-srv2-hook` pushed, with two musts named in its Record on that branch; the order's file on `main` has no Record | A finishing order stacked on it, in the lane, once it restarts at the pin |
| 7 | The admin's next spec order | Not written. PRV-021's owed stop in three cases, matching kit order 141; PRV-015 names a machine by ID where `place.v1` gives a hostname; "labels last reported" (PRV-027, LIF-036, `stop.v1`); one live node per machine (049a1); what an engine owes when it cannot store its state, and races across addresses ([082](../AUDIT.md#known-to-be-open)); 081's Record's leftovers | Write next (077 is in) |
| 8 | The admin's small engine order | Not written. The first-start report maps Rust's architecture to Go's `GOARCH` wrongly (`loongarch64`, little-endian `mips` and `mips64`, `armbe`, `arm64be`, `riscv32`, `wasm`); mirror order 128's mapping (PRV-014) (log) | A small order, or a fix under 71 point 4 |
| 9 | Batch 1's rest, on the rerun path | Not written: 046, 047, 048a, 048b, 050, 053; joining them, 043a, 043b, 052a, 052b, 051a, 051b ([the plan](../proposals/spec-batch-1/README.md#the-work-orders)) | 046 next, through Conduit, once the dogfood engine is re-pinned to `main` |
| 10 | The sign-in chain | Not written: 050 (each Agent's own home, the hooks, the account check; merges with 050a, 076 and 078 as one 0.2.0), 048a, 048b, 051b; then A81 (a carried Codex login) and [A82](../../ROADMAP.md#experiments-and-probes) (Linear's OAuth through the Interface) | The second accounts wait for this: each Agent signed in through the Interface, as the test of its own subscription and of the model ([72's later answers](../proposals/ruling-72/README.md#later-answers)) |
| 11 | Off the rerun path | 049a2, the second half of batch 1's order 049a (PND-008's pending actions for enrollment and ENR-012's removal question), after 047 and 048a; 049b, 051c, SSH-M; `s3/m2` merges after the rerun's pin | After 049a1 |
| 12 | The S1 rerun and the bar | 053, a pinned tag, the rerun with the owner present, the owner's judgment (71 points 2 and 3, as 78 point 2 amends point 3) | After batch 1 |
| 13 | Batch 2 | Not begun. On its own branch, merged after the bar (71 point 2). Holds appointment (A78's orders 4 and 5), MC9 ([G5](../../experiments/012-model-checking/G5.md#what-g5-still-needs)), and apply calling the control a Type offers for a changed control value (OPS-020; from 069b's reference, for S4's T1) | After the rerun's pin |
| 14 | The lane's queue | Merged: S2 to S10's plans (117 to 121, 133, 134); A63's kit, contract harness and local providers ([PROVIDERS.md](PROVIDERS.md)); S2's stub, Conduit-aware implementation and adapter; S4-SRV1; SCR1; SSH-R's drainer (146, 149 and 150, #260). Open: SSH-R's key sweep (147), S4-SRV2 (144). Checkpointed in #262; the queue in order is in [PROVIDERS.md's status](PROVIDERS.md#status) | A lane coordinator restarted at the S1 rerun's pin (ruling 78, point 1); S2-PL after the coordinator picks the plain server, S3-H after the owner adds S3's administrator rule (ruling 78, point 4) |
| 15 | Records owed | Orders 037 to 045, 049a1, 050a and 067 onward into the record (A80); review-list items 30 onward (below) | A docs pass; a check in `tools/check_docs.py` that `docs/work-orders/README.md` has one row for each order file (182's Record) |

## The owner's open items

- **Ruling 77's setup** ([what follows](../proposals/ruling-77/README.md#what-follows)). Part 1 is
  done: `conduit-run`, its tools, and its Codex signed in by the owner. Part 2, which can run now:
  bubblewrap if missing, the restart that applies the lockdown, and the coordinator's key taken out
  of the owner's account. Before the end-to-end run: the Mac renamed in the tailnet, then HTTPS
  certificates turned on.
- **Answered by [ruling 78](../proposals/ruling-78/README.md)** on 2026-11-14, every one as
  recommended: the full review's 17 questions; S3-OF, by option (E), the Mac's own firewall under
  one administrator rule the owner adds once; S2's plain server, which the coordinator now picks
  and logs to strike; and A20, closed on v0's drafted security claim.
- **S3's administrator rule** (ruling 78, point 4): one rule on the Mac, added once by the owner
  before S3's fault harness runs, under which the coordinator's locked-down script cuts the network.
- **S1's bar** after the rerun, scored in two groups (ruling 78, point 2), with findings 4-01 and
  4-11 and C6 to C12 ([the tracker](../AUDIT.md#waiting-for-the-owner)).
- **A82's application**: a Linear OAuth application the owner registers (S2-LOG-O).
- Later, not urgent: G5's 12 exclusions, accepted or modelled; a repository and an npm scope for
  S7's outside author ([S7's plan](../../experiments/018-s7-outside-package/plan.md)); S8-A,
  accepting S8's concrete Requirement and contracts before any code
  ([S8's plan](../../experiments/019-s8-filtered-search/plan.md)); A74's unmeasured cells
  ([S3's plan](../../experiments/014-s3-two-machines/plan.md#for-the-owner)); G8, whose scope
  leaves out the native probes (ruling 78, point 3.3), and G11; whether runtime providers that use
  the kit by path are "self-contained" ([PROVIDERS.md](PROVIDERS.md), IN_FORCE topic 5). The
  tracker's [list](../AUDIT.md#waiting-for-the-owner) is the whole of it.

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
- Order 077: 041b's vector `fleets/e3b/reconcile-control` keeps its purpose, with the fence
  established before its reconcile; 077 merged on local evidence while CI's engine shards were
  skipped.
- Order 042: Fleet reconciliation matches a sender by ID only, not through a reused name.
- 049a1: one live node per machine; acceptances decided one at a time; a second refused with a
  conflict naming the node (log).
- Order 145: SSH-R's drainer keeps its state in an operator-set `durableDir` outside
  `CONDUIT_STATE_DIR`; the SSH key is deleted once nothing is owed.
- Providers: option (b) of order 120, packaged local providers that are to pass the contract
  suites beside the second providers ([PROVIDERS.md](PROVIDERS.md), decision B), as
  [D45 of the full review's decisions](../proposals/full-review/decisions.md) changed it. No
  further work goes to `provider-local-machine`; `provider-local-process` stays as order 080's
  one-machine tracer and `provider-local-directory` only as a suite partner; no adapter wraps the
  built-in runner, whose built-ins are tested by running the suites' cases as data through the
  engine's own tests. The four contract suites hold no case yet
  ([experiment 022](../../experiments/022-provider-contracts/README.md)), so no provider has
  passed one. "Self-contained" is read as a crate's own lockfile and toolchain.
- S2: the "small adapter" is the glue that puts an existing tool behind Conduit, the same for S7;
  the plain row runs only in the owner's S2 runs ([S2's plan](../../experiments/013-s2-shared-login/plan.md)).
- S4's T1 repair: the author corrects a `blockedBy` configuration place through Conduit
  ([S4's plan](../../experiments/015-s4-partial-setup/plan.md)).
- A6's traces come from S2's and S4's runs ([the register](../../ROADMAP.md#experiments-and-probes)).
- The side branch `s3/m2` for the Windows engine work, merged after the rerun's pin (log).
- "Windows computer is up again" read as the resume after the pause of 2026-11-11 (72's later answers).
- Ruling 72, decision 3: "No rented machine stands in", in
  [IN_FORCE.md, topic 10](../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment),
  and in [the ruling's record](../proposals/ruling-72/README.md), is the coordinator's rendering.
  The owner's words, "We can use the windows machine whenever needed", answered a question about
  a fallback machine ([FR-330](../proposals/full-review/findings/P05-second-machine.md#fr-330),
  point 5). Struck, the sentence leaves topic 10 and decision 3.
- Order 154: a plan prints each `ownTools` member as `inventory.v1` gives it, except that an
  absolute-path `directory` is printed as `{"pathSha256":"HEX"}`, as the plan prints a root as
  `root-sha256=HEX`, so that plan text holds no local path
  ([the reference](plan-and-apply/README.md#text-grammar-and-statuses)). `spec/README.md`
  says a plan prints the member that inspection would show (CTR-014, LIF-013), and `spec/`
  does not change. Struck, the plan prints the directory as inspection shows it.

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
  in #190. [The method as run](METHOD_AS_RUN.md#7-merge-and-the-record) gives the one command and
  the check after it.
- **zsh**: brace a variable before a colon in a refspec, `${NAME}:refs/...`; `$NAME:r` is a
  modifier. An unmatched glob aborts the whole command.
- **Load.** [The method as run](METHOD_AS_RUN.md#load) gives the rule.
- **Don't start finishing orders blindly.** Read the last verifier's findings and the branch first.
  When each round finds a new fault of one kind, the design is wrong: 149's drainer handover raced
  in every round, and [150](../work-orders/150-ssh-r-one-drainer.md) replaced it with one drainer.
- **Keep work out of session scratch space** (order 019's run was lost) and never print a whole
  large file (order 028) ([lessons](WORK_ORDERS.md#what-this-round-taught)).
