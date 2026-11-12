# Work order 117 (PLAN-A): the order-level plans of S2 and S3

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count.

## What and why

An order of the scenarios lane under [ruling 71](../proposals/ruling-71/README.md). The owner
accepted the pre-registrations of S2 to S10 under [ruling 75](../proposals/ruling-75/README.md),
D. Each says what its scenario must show and, in its section "What it needs first", what must
exist before it runs. None says which work orders build that, in what order, or what can start
today. This order writes that plan for S2, one service login shared by two Agents, and for S3, S1
and S2 across two machines. Sister orders 118 (S4, S5, S6 and S9), 119 (S7, S8 and S10) and 120
(A63, the providers) write the others in the same form, at the same time.

A plan is for the lane coordinators and the admin. It binds nothing, and it changes nothing the
owner accepted. Every order it names that changes `engine/` or `spec/` is a proposal to the admin,
who is the merge authority for those directories and writes those orders. A fresh skeptic reviews
each plan's split before any order in it is dispatched.

Read:

- the two pre-registrations, [S2's](../../experiments/013-s2-shared-login/README.md) and
  [S3's](../../experiments/014-s3-two-machines/README.md), all of each, with the owner's answers
  at the end of "Open for the owner";
- [ROADMAP.md](../../ROADMAP.md): [the order of the work](../../ROADMAP.md#the-order-of-the-work),
  [the scenarios](../../ROADMAP.md#scenarios), and the register's rows A6, A22, A73, A74 and A81;
  A63's row in [docs/AUDIT.md](../AUDIT.md);
- [batch 1's plan](../proposals/spec-batch-1/README.md), its section "The work orders", which
  ends with the critical path: the numbers, contents, sizes and dependencies of orders 042 to 053;
- the order files under `docs/work-orders/` for work that is written but not all merged: 042,
  049a1, 050a, 070 to 072, 075 to 082 and 038c. Order 049a1's section "What and why" and order
  080's describe the side branch `s3/m2`;
- [engine/README.md](../../engine/README.md) and [engine/CONFORMANCE.md](../../engine/CONFORMANCE.md):
  what the engine does now, and its sections on choices still required. The DL16 row "executor
  coverage" and the DL7 rows say what is unserved;
- [conformance/README.md](../../conformance/README.md), for the form of a vector;
- [the safety catalogue](../implementation/SAFETY_CATALOGUE.md) and
  [G5's account](../../experiments/012-model-checking/G5.md), for which promises are safety
  promises and which are static (to be shown by vectors);
- [IN_FORCE.md](../../IN_FORCE.md), topics 9 to 13 and 15; rulings
  [46](../proposals/ruling-46/README.md), [47](../proposals/ruling-47/README.md),
  [61](../proposals/ruling-61/README.md) (batch 2), [68](../proposals/ruling-68/README.md),
  [72](../proposals/ruling-72/README.md), [75](../proposals/ruling-75/README.md) and
  [76](../proposals/ruling-76/README.md), with its later answers;
- `packages/agent-codex/`: the form of a package crate (`Cargo.toml`, `conduit.yaml`,
  `contracts.yaml`, `README.md`, `tests/`);
- the guides [1Password and Linear](../guides/1password-and-linear.md),
  [a second machine](../guides/second-machine.md), [accounts](../guides/accounts.md) and
  [Docker](../guides/docker.md).

## The state of the work, as the admin gave it on 2026-11-11

Not all of this is in the tree. Check what you can against it, and report any difference.

1. **What is merged.** Judge it from the tree at your base and `git log --oneline`. An order is
   merged when the tree holds its change, not when its order file is present: the files of
   orders 042, 049a1, 050a, 070 to 072, 075 to 082 and 038c reached `main` before their work did,
   or may have. Write at the top of each plan the commit you read and what you found.
   - Order 041b is merged: `retire-with-resources.v1` and `retire-with-agents.v1` run, DL1's
     ordering of container retirement is documented, and control reconciliation applies the held
     input values (REV-010). `declare-lost.v1` and control dispatch are still unfinished, and the
     engine fetches only `file` URLs (CONFORMANCE.md's DL7 row). The pre-registrations were
     written before 041b merged: plan only what CONFORMANCE.md still lists as unfinished, and say
     where a README's "What it needs first" is already met.
2. **The S1 rerun's pin.** Batch 1 runs to order 053; then the build is pinned to a tag and the
   rerun measures it. An order that changes the starting snapshot of the vectors, root's tools, or
   what S1's author does, waits for the pin unless the admin places it on a side branch. After
   the pin, the scenarios' core orders build on `main`, in two or three lanes at once.
   - **Batch 1's orders, as the coordinators split them** (sizes before the pipeline's fixes).
     Batch 1's plan has only 046, 048a, 048b, 050 and 052a; the splits are:
     - 070 (E1): the core stops copying the whole state per transition. M-L. 071 (S0a): the
       manifest in pieces, vectors in parallel. M-L. 072 (LST): `list.v1` in the shell. XS.
     - S0b: the core's seams: an operations registry, the coming State maps (tombstones, pending
       actions, machines) declared, the core's input-required output declared, and `step`'s arms
       in modules. M. After 070. S0c: the CLI's commands in a table. M. After 070.
     - 042: tombstones, lift, inventory, provenance. L+. 046a: refusal tiers, a diagnostic on
       every reason, the runner's assertions, the CLI's tier 2. L. 046b: closed Conduits from
       tombstones. M. After 042 and 046a.
     - 047: the pending-action core. L. After 046a. 048a-1: CTR-013 in the loader, IFC-016's
       detection, the signIn fixture Type. M. 048a-2: sign-in as a kind of pending action,
       PND-005, SignInUndone. L. After 047 and 048a-1.
     - 048b-relay: the shell relays `input_required` (`server.rs`, `runner.rs`). M. After S0b.
       048b-join: S-M. After 048a-2. 051a: shell commands (`list`, `fleet`, `retire
       --with-resources`). M. After 042 and S0c. 051b: the pending-action commands `pending`,
       `sign-in` and `answer`. M. After 047 and 048b-join.
     - 050: the Agent packages 0.2.0, with each Agent's home, the login flows and the sign-in
       hooks, built on orders 050a and 076 (the `.v2` Types and their clients' own tools), whose
       work is on their branches, not yet on `main`. M plus a run. After 048b-join.
     - 052a-1 and 052a-2: plan and apply. L each. After 069 (K3), 043b, 042 and 051a. 052b:
       replacement and authoring commands. 053: the rerun's preparation, last.
3. **Batch 2** (ruling 61) is drafted on its own branch and merges to `main` only after the owner
   judges the rerun's bar. Name its orders `B2-S` (specification), `B2-C` (core) and `B2-H` (shell), and
   say which behavior you need from it; it holds, among others, the rule on secrets for an Agent
   that others can drive (S2's G1).
4. **The Windows computer as a node** (the side branch `s3/m2`, merged to `main` only after the
   pin, under ruling 76's later answers). Its orders:
   - **AGT** (order 078): the Codex Agent item found on that machine's PATH. XS.
   - **ADDR** (order 079): the engine's second, `https` address, which admits only tokens of runs
     placed on another node, never root's. M.
   - **DSP** (order 080): the shell calls runtime providers, asks `server/discover` first, issues
     each provider run a certificate, and reaches the run through a bridge that checks TLS before
     sending anything (PRV-007, PRV-028, IFC-017). It also forwards other provider operations,
     which serves enrollment's machine calls. L+.
   - **049a-1** (order 049a1): enrollment in the core. L.
   - **SSH-R**, lane 9: an SSH runtime provider in `packages/provider-ssh`. Runs are systemd user
     units on the other machine; a gate on the engine's machine admits only the engine's token,
     and a stop kills the run's whole process tree; owed stops are kept and carried out at the next
     contact (PRV-021). Tested only where a test host is configured. L+. Depends on `main` only.
   - **SSH-M**, lane 9: an SSH machine provider in the same package: enrollment with a fresh key
     per enrollment, evidence that is the host key's fingerprint, `list-machines.v1` and a minimal
     `place.v1`. M. After 049a-1. It is the admin's recommended answer to M2-1, below, which the
     owner holds; the other candidate is a Tailscale machine provider. Plan for "the machine
     provider M2-1 selects", and name SSH-M only as the candidate.
   - **051c**, revised: the panel, `conduit machine enroll`, `admit` and `refuse`, and a loop that
     reports the machines every DL5 interval. M. After 049a-1 and DSP.
   - **049b**, revised: the fixture machine provider and its vectors. M. After 049a-1.
   - **049a-2**: PND-008's pending actions for enrollment. M. After 047 and 048a-2.
   - Four choices are held for the owner: which machine provider enrolls the Windows computer
     (M2-1), which operating-system user runs execute as there (M2-2), how TLS is terminated at the
     engine's `https` address (M2-3), and whether that machine's Linux is locked down (M2-4). Name
     no answer to any of them.
   - Its owner's set-up (stage 0) also yields A74's machine observations. It proves no part of
     S3: not partitions, loss, owed stops under faults, the Docker profile or second accounts.
5. **A22 stays open** (the admin's decision, option (c)): v0's engine runs in the owner's
   logged-in session, where experiment 005 showed that keychain reads work. S2 claims no daemon
   session. The keychain store does not wait for A22.
6. **A6's traces** come from S2's and S4's runs on the engine (the admin's decision). Discovery,
   invocation and refresh come from S2; the lost response from S4.
7. **Safety vectors.** SV1 adds the field naming a vector's safety promise, after order 071 (the
   manifest in pieces) merges; SV2 tags existing vectors; SV3 writes S1's missing safety vectors;
   SV4 makes every later order tag its own.
   A scenario's safety vector is written with its field once SV1 is merged, and not before.
8. **Real clients.** Ruling 75, D4 reads: "Beyond S1 and S2, the only real-client run is S4's one
   lost-response replay on a second account". It answered the optional replays of S4, S5, S6 and
   S9. Ruling 72, point 2, and ruling 75, D2, give S3, S7, S8 and S10 second subscription
   accounts for their Agents. Where a plan's runs depend on how D4 is read, say so under "For the
   admin"; do not settle it.

## The decisions it applies

1. **Where the plans live.** S2's is `experiments/013-s2-shared-login/plan.md` and S3's is
   `experiments/014-s3-two-machines/plan.md`, both new. In each README, append to the section
   "Status" one sentence: "Its order-level plan is plan.md, written by work order 117; the plan changes nothing accepted
   here.", with "plan.md" a link to the new file beside the README. Change nothing else in either README.
2. **The form.** The plans of orders 117 to 120 have the same sections, so that the admin can
   read them together. Keep each under about 220 lines.
   1. A box, "In short", with one line for each order that changes `engine/` or `spec/`
      (proposed to the admin), one for each question for the admin, and one for each question for
      the owner.
   2. *Status*: "Written on DATE by work order 117, against `main` at COMMIT. A plan for the lane
      coordinators and the admin: it binds nothing, and changes nothing the owner accepted in
      [README.md](README.md). Orders that change `engine/` or `spec/` are proposals to the admin.
      No order named here has been dispatched."
   3. *What `main` has now*: for each need in the README's "What it needs first", and each gap in
      "What `spec/` does not yet decide", what the tree already does, with evidence (a file and
      line, a test or a vector), and what is missing.
   4. *The orders*: a table with the columns ID, Kind, What it does, Promises, Depends on, Size,
      Who and Starts. Then, for each order, a short scope of three to eight lines: the files or
      crates it changes, the hot files among them (decision 5), what it must not change, the
      vectors it adds by the README's names, and its checks.
   5. *Shared with other plans*: each shared ID (decision 4) that this plan uses and does not own,
      and what it needs from it.
   6. *The order of the work*: the dependency chain; the critical path from today to the
      scenario's last run; what can start now on `main`, in the order to dispatch it; what waits
      for the pin, for batch 2, or for the owner.
   7. *For the admin*: each question on a split, a placement or the order of merging, with its
      options and a recommendation, marked as a recommendation.
   8. *For the owner*: only a question that the split raises and that ruling 75's answers do not
      settle, in the same form. If there is none, say so.
   9. *Limits*: what the plan does not settle.

   Link only to files that exist in your tree. Name a sister plan, `docs/implementation/PROVIDERS.md`,
   `experiments/006-across-machines/` or any other file not yet written in backticks, not as a
   link.
3. **Kinds, owners and sizes.**
   - **Kind** is one of: *spec* (`spec/`), *core* (`engine/crates/core` or `loader`), *shell*
     (`engine/crates/shell`), *provider* (a provider package under `packages/`), *server* (a
     package or stub that a scenario's tasks use, which is not a provider), *harness* (code that
     drives a scenario or injects its faults), *vectors* (cases under `conformance/` and nothing
     else), *probe* (an experiment of the register), *run* (a counted author run, which the
     coordinator carries out with the owner, not a Codex order), and *owner* (a step of the
     owner's, not an order).
   - **Who** is one of: *admin* (any order that changes `engine/` or `spec/`: proposed to the
     admin, never dispatched by a lane); *lane 6* (safety vectors); *lane 8* (providers and
     scenario servers); *lane 9* (A74 and S3's machine work); *lane 10* (scenario vectors,
     scenario harnesses and runs); *owner*. A vectors order whose cases pass only after a core
     order is part of that core order, and so the admin's. The admin is also the merge authority
     for the conformance runner and manifest (ruling 71, point 1): a vectors order that edits
     `conformance/manifest.yaml`, a manifest piece shared with other cases, or the runner is the
     admin's too.
   - **Size** is the class before the pipeline's fixes: XS under 45 minutes, S 1 to 1.5 hours, M 2
     to 2.5 hours, L 2.5 to 3.5 hours, L+ more than L; a run is about an hour of the owner's
     presence. Give an estimate of lines where you can, and split any order larger than L.
   - **Starts** is "now", when everything it depends on is in the tree at your base and it waits
     for neither the pin nor the owner; otherwise "after" and what it waits for.
4. **Shared IDs.** The four plans name shared work by these IDs, so that they agree. A plan may
   split an order it owns into parts with a numeric suffix (`S2-L1`, `S2-L2`); other plans cite
   the unsuffixed ID. Cite any other order by the number `main` or batch 1's plan gives it (070,
   071, 072, 075, 038c, 043a, 043b and so on). Invent no other ID for shared work, and name this plan's own orders `S2-` and
   `S3-` followed by a short tag of your choice.
   - Batch 1, by its numbers and the splits the admin uses: 042, 046a, 046b, 047, 048a-1, 048a-2,
     048b-relay, 048b-join, 049a-1, 049a-2, 049b, 050 (the Agent packages 0.2.0, with order 050a),
     051a, 051b, 051c, 052a-1, 052a-2, 052b and 053; `PIN`, the rerun's pin.
   - The specification's fixes: 077 (fenced reconciliation), 081 (one run per instance), 082
     (storage loss and concurrent first installs). Each order file is on `main`; read its status
     from the tree. Batch 2: `B2-S`, `B2-C`, `B2-H`.
   - The Windows node: AGT, ADDR, DSP, SSH-R, SSH-M, as in point 4 above.
   - Safety vectors: SV1, SV2, SV3, SV4.
   - **Owned by S2's plan (this order):** `SEC-D`, the engine's reading and delivery of a secret
     (SEC-006 to SEC-016, among others), which S3 and S10 need too; `S2-F`, the restriction
     contract of O3 (explicit issue IDs with a deadline, refused outside that scope) as a package
     of contracts; `FLT-D`, the engine's delivery of onward identity and filters (ACC-017,
     IFC-019, IFC-020) and its check that a target supports a filter (CON-022), which S8 needs
     too; `S2-L`, Linear's implementations (an unchanged plain server, a small adapter
     and a Conduit-aware implementation) and the local Linear stub its vectors use.
   - **Owned by A63's plan (order 120):** `A63-H`, the harness for provider-contract vectors;
     `A63-RP`, `A63-MP` and `A63-PS`, the packaged local runtime, machine and package-source
     providers (a local process, the local machine, a local directory); `A63-FS`, the file secret
     store; `A63-KC`, the macOS keychain store (S2's store); `A63-DR`, the Docker runtime; `A63-GS`,
     the Git package source; `A63-1P`, the 1Password store, after A73; `PSD`, the engine's dispatch
     of package-source calls and its own `https` fetching (DL7), with the archive check (PRV-017,
     PKG-026) and scheme claims (PRV-013, PRV-016). `A63-DR`'s plan also says what an Agent
     package needs to run in a container (its home, its sign-in's browser callback). Order 120 decides each one's scope; this plan says what S2 and
     S3 need of each.
   - **Owned by order 118:** `SCR`, the scripted MCP client that the runs of S4, S5, S6 and S9 use
     in place of real clients (ruling 75, D3), and the `S4-`, `S5-`, `S6-` and `S9-` orders.
     S2's plan says whether S2's scripted protocol driver for E02 can be `SCR`.
   - **Owned by order 119:** the `S7-`, `S8-` and `S10-` orders.
5. **Hot files.** Name, for each core and shell order, which of these it edits, so that the admin
   can sequence them: `engine/crates/core/src/requests.rs`, `access.rs`, `registration.rs` and
   `lifecycle.rs`; `engine/crates/shell/src/server.rs`, `runner.rs`, `lib.rs` and
   `identity.rs`; the conformance runner's `lib.rs`; the manifest's pieces; the CLI's command
   tables.
6. **S2's plan.**
   - Its Agents need the Agent packages 0.2.0 (050, built on 050a and 076) and sign-in end to end
     (048a-1, 048a-2, 048b-relay, 048b-join, 051b), from batch 1. Say which of S2's tasks need
     which.
   - Its engine needs, from the README, beyond `SEC-D` and `FLT-D`: filtered repeat dispatch
     (OPS-024), replacement and repair, and real clients' tool changes. Say which are met at your
     base.
   - The Docker runtime (`A63-DR`) serves D20's isolated profile and S2's runs, as S2's "Runs"
     says. Say what S2 needs of it and whether the runtime's dispatch (DSP) must be on `main`
     first.
   - Every vector V1 to V6 of S2's README is placed in exactly one order. A vector that needs the
     Linear stub depends on `S2-L`.
   - Say whether E02's scripted protocol driver can be `SCR`. Order 118 plans `SCR` to speak MCP to
     the engine as an Agent with the Agent's own token, send keyed repeats, hold a call at a
     barrier, and record each request and reply. Say what more E02's keyed replay after the
     deadline and its forced stale calls need, so that order 118's plan can include it.
   - Name the preparation the README asks for (the Rust harness that fixes the deadline clock,
     changes authorization data, pauses a dependent call and fails restoration partway) as a
     *harness* order, and say where its crate lives: under `experiments/013-s2-shared-login/`.
7. **S3's plan.**
   - Build on the Windows node's orders in point 4; do not plan them again. Say what S3 needs
     beyond them: partitions and loss, `declare-lost.v1` and PLC-024's cause (G5 of S3's README),
     owed stops under faults, the Docker profile on the second machine, the second accounts, and
     S2's packages on two machines. Name each as an `S3-` order or as a shared ID. The Docker
     runtime itself is `A63-DR` (order 120): say what S3 needs of it on the second machine, and
     plan no second Docker provider.
   - A74 runs first. Plan it as a probe in `experiments/006-across-machines/`, the directory its
     register row names, in at most two orders. S3's README already maps its tasks to A74's
     hypotheses H1 to H10: say which of A74's orders each task waits for.
   - S3 exercises the second implementation of the machines contract: the machine provider that
     M2-1 selects, of which SSH-M is the admin's candidate. Say which machine contract vectors it
     must pass, without naming M2-1's answer.
   - Place every vector V1 to V7 of S3's README in exactly one order, and say which safety promise
     each names. Order 038b's enrollment promises now meet part of S3's G4: say what is left.
8. **No new language and no answers for the owner.** A behavior a scenario needs that `spec/`
   does not state is a finding of class gap in your report and a line of "What `main` has now",
   with the spec order it would need, proposed to the admin. Name no answer to M2-1 to M2-4 or to
   any question the owner holds.
9. **No secrets, no local paths.** The plans name no machine name, network name, user name or
   local path. Write `<lab>` for a lab directory.

## Scope

Change only:

- `experiments/013-s2-shared-login/plan.md` and `experiments/014-s3-two-machines/plan.md` (new);
- one sentence appended to the section "Status" of each of their READMEs (decision 1).

Do not change any other file: not `spec/`, `conformance/`, `engine/`, `packages/`, ROADMAP.md,
`experiments/README.md`, IN_FORCE.md, docs/AUDIT.md, any other experiment, or any record or
ruling. Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" experiments/013-s2-shared-login experiments/014-s3-two-machines
grep -rn -E "[[:space:]]+$" experiments/013-s2-shared-login experiments/014-s3-two-machines
```

Report each gap as a finding, each order you propose and each estimate as one of your own
choices, and each difference from the state of the work above as a finding. Do not stage
anything.
