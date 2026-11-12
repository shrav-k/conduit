# Work order 120 (PLAN-P): the order-level plan of the providers, A63

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count.

## What and why

An order of the providers lane under [ruling 71](../proposals/ruling-71/README.md). A63 builds the
providers as packages in Rust under `packages/`, one crate per provider, two for each provider
contract on different existing tools, and is done when each passes the vectors of its contract
and S10 passes with every provider swapped ([its row](../AUDIT.md), ruling 40). v0's provider
contracts are runtime, machines, package source and secrets (IN_FORCE.md topic 11). No provider
package exists yet: `packages/` holds only the Claude Code and Codex Agent packages. Today's
conformance runner drives only the core, so no provider can yet be shown to pass its contract's
vectors.

This order writes A63's order-level plan: the harness for provider-contract vectors, the first
and the second provider of each contract, their contract vectors, and the engine work each needs
before the engine can use it. The admin has asked that the first provider of each contract be its
own package under `packages/`: a local-process runtime, a local machine provider and a
local-directory package source, unless one already exists. Sister orders 117 (S2 and S3), 118
(S4, S5, S6 and S9) and 119 (S7, S8 and S10) write the scenarios' plans in the same form, at the
same time.

A plan is for the lane coordinators and the admin. It binds nothing, and it changes nothing the
owner accepted. Every order it names that changes `engine/` or `spec/` is a proposal to the admin,
who is the merge authority for those directories and writes those orders. A fresh skeptic reviews
the plan's split before any order in it is dispatched.

Read:

- A63's row in [docs/AUDIT.md](../AUDIT.md); [ROADMAP.md](../../ROADMAP.md):
  [the order of the work](../../ROADMAP.md#the-order-of-the-work) (steps 3 and 4) and the
  register's rows A22, A73 and A74;
- ruling [40](../proposals/ruling-40/README.md); [IN_FORCE.md](../../IN_FORCE.md), topics 1, 10
  to 13 and 15; rulings [46](../proposals/ruling-46/README.md) (point 5),
  [47](../proposals/ruling-47/README.md), [68](../proposals/ruling-68/README.md) (point 2),
  [72](../proposals/ruling-72/README.md) and [75](../proposals/ruling-75/README.md);
- the contracts: `spec/core/capabilities.yaml` (`runtime-provider.v1`, `machine-provider.v1`,
  `package-source.v1` and `secret-provider.v1`); their operations in `spec/core/operations.yaml`
  (`run.v1`, `stop.v1`, `inspect-run.v1`, `list-machines.v1`, `place.v1`, `fetch-package.v1` and
  `get-secret.v1`); the promises in `spec/promises/providers.yaml`, `secrets.yaml`,
  `enrollment.yaml`, `packages.yaml` and `manifest.yaml`; and the enrollment operations and
  contract edits that order 038b staged in `docs/work-orders/038b-staged/` (`049a.patch` and
  `049b.patch`). Order 049a-1 lands both (its step 1), so `machine-provider.v1` gains its
  enrollment operations only when 049a-1 merges, after the pin;
- the pre-registrations' sections on providers: [S10's](../../experiments/021-s10-swapped-providers/README.md)
  all of it; [S2's](../../experiments/013-s2-shared-login/README.md) "Runs" and "What it needs
  first"; [S3's](../../experiments/014-s3-two-machines/README.md) "Runs" and "What it needs
  first"; [S7's](../../experiments/018-s7-outside-package/README.md) D01;
- [experiment 005](../../experiments/005-macos-keychain/README.md), above all "What it means for
  a macOS keychain provider package" and "Safety";
- orders 079, 080 and 049a1 under `docs/work-orders/`: how the shell will call runtime and machine
  providers, the certificate each run gets, and the gate;
- [engine/README.md](../../engine/README.md) and [engine/CONFORMANCE.md](../../engine/CONFORMANCE.md):
  the DL7, DL13 and DL16 rows; [conformance/README.md](../../conformance/README.md) and the
  runner under `engine/crates/conformance/`, for the form of a vector and how the runner drives
  the core;
- [the safety catalogue](../implementation/SAFETY_CATALOGUE.md), for which provider and secret
  promises are safety promises;
- `packages/agent-codex/`: the form of a package crate (`Cargo.toml`, `conduit.yaml`,
  `contracts.yaml`, `README.md`, `tests/`); the guides [Docker](../guides/docker.md),
  [1Password and Linear](../guides/1password-and-linear.md) and
  [a second machine](../guides/second-machine.md).

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

1. **Where the plan lives.** `docs/implementation/PROVIDERS.md`, new, titled "The providers
   (A63): the order-level plan". Add one row for it to the table "Process for the builds" in
   `docs/README.md`, after the safety catalogue's row: "The first and second provider of each
   provider contract, their contract vectors and the engine work they need (A63; work order 120)."
2. **The form.** The plans of orders 117 to 120 have the same sections, so that the admin can
   read them together. Keep each under about 220 lines.
   1. A box, "In short", with one line for each order that changes `engine/` or `spec/`
      (proposed to the admin), one for each question for the admin, and one for each question for
      the owner.
   2. *Status*: "Written on DATE by work order 120, against `main` at COMMIT. A plan for the lane
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
   071, 072, 075, 038c, 043a, 043b and so on). Invent no other ID for shared work, and name this plan's own orders `A63-`
   followed by a short tag of your choice.
   - Batch 1, by its numbers and the splits the admin uses: 042, 046a, 046b, 047, 048a-1, 048a-2,
     048b-relay, 048b-join, 049a-1, 049a-2, 049b, 050 (the Agent packages 0.2.0, with order 050a),
     051a, 051b, 051c, 052a-1, 052a-2, 052b and 053; `PIN`, the rerun's pin.
   - The specification's fixes: 077 (fenced reconciliation), 081 (one run per instance), 082
     (storage loss and concurrent first installs). Each order file is on `main`; read its status
     from the tree. Batch 2: `B2-S`, `B2-C`, `B2-H`.
   - The Windows node: AGT, ADDR, DSP, SSH-R, SSH-M, as in point 4 above.
   - Safety vectors: SV1, SV2, SV3, SV4.
   - **Owned by S2's plan (order 117):** `SEC-D`, the engine's reading and delivery of a secret
     (SEC-006 to SEC-016, among others), which S3 and S10 need too; `S2-F`, the restriction
     contract of O3 (explicit issue IDs with a deadline, refused outside that scope) as a package
     of contracts; `FLT-D`, the engine's delivery of onward identity and filters (ACC-017,
     IFC-019, IFC-020) and its check that a target supports a filter (CON-022), which S8 needs
     too; `S2-L`, Linear's implementations (an unchanged plain server, a small adapter
     and a Conduit-aware implementation) and the local Linear stub its vectors use.
   - **Owned by this order:** `A63-H`, the harness for provider-contract vectors;
     `A63-RP`, `A63-MP` and `A63-PS`, the packaged local runtime, machine and package-source
     providers (a local process, the local machine, a local directory); `A63-FS`, the file secret
     store; `A63-KC`, the macOS keychain store (S2's store); `A63-DR`, the Docker runtime; `A63-GS`,
     the Git package source; `A63-1P`, the 1Password store, after A73; `PSD`, the engine's dispatch
     of package-source calls and its own `https` fetching (DL7), with the archive check (PRV-017,
     PKG-026) and scheme claims (PRV-013, PRV-016). `A63-DR`'s plan also says what an Agent
     package needs to run in a container (its home, its sign-in's browser callback). This plan decides each one's scope, and the scenarios' plans say
     what they need of each.
   - **Owned by order 118:** `SCR`, the scripted MCP client that the runs of S4, S5, S6 and S9 use
     in place of real clients (ruling 75, D3), and the `S4-`, `S5-`, `S6-` and `S9-` orders.
     S2's plan says whether S2's scripted protocol driver for E02 can be `SCR`.
   - **Owned by order 119:** the `S7-`, `S8-` and `S10-` orders.
5. **Hot files.** Name, for each core and shell order, which of these it edits, so that the admin
   can sequence them: `engine/crates/core/src/requests.rs`, `access.rs`, `registration.rs` and
   `lifecycle.rs`; `engine/crates/shell/src/server.rs`, `runner.rs`, `lib.rs` and
   `identity.rs`; the conformance runner's `lib.rs`; the manifest's pieces; the CLI's command
   tables.
6. **The form, for this plan.** Between "What `main` has now" and "The orders", add a section
   *The contracts*: for each of the four provider contracts, its first and second implementation,
   the existing tool each is built on, the scenario that exercises it, and what the engine must do
   before it can use it. "What `main` has now" is given per contract. The Status line names
   no README: write "it binds nothing, and changes nothing the owner accepted". This plan may
   run to about 280 lines.
7. **`A63-H`, the harness.** Plan what a provider-contract vector is and what drives it.
   - It starts a provider package's server as the engine would, from the package's item, and drives
     its operations over MCP. Each contract has one suite, which both of its implementations pass
     (ruling 40; IN_FORCE.md topic 13). Each run writes a report per implementation, with the tool
     and its version (S10's P3).
   - Propose the form of a contract vector, beside `conformance/`'s case form, and where the
     vectors live.
   - Give the options for where the harness lives, with a recommendation: a crate in the engine's
     workspace (then the admin's); the conformance runner extended (the admin's); or a crate of
     its own outside `engine/`. Say what each costs, and what the gate's checks (PRV-025, PRV-028,
     IFC-017) need of the harness, such as the store's root and a run's certificate.
   - Safety vectors carry SV1's field once SV1 merges.
8. **The built-ins and the packaged local providers.** The engine has its own runner and its own
   machine provider, and fetches `file` URLs itself; `spec/core/capabilities.yaml` gives it
   `https` too, but the build's `https` fetching is unfinished (CONFORMANCE.md's DL7 and DL16
   rows). Ruling 46, point 5, counts these built-ins as S1's first providers, and S10's accepted
   runs table names the runner and the local-directory fetching as S10's first implementations.
   None of them is reached through its contract's operations over MCP, so none can pass a suite
   that `A63-H` runs that way. The admin asked for the first provider of each contract as its own
   package, unless one already exists. Plan `A63-RP`, `A63-MP` and `A63-PS` as packages, and put
   these options under "For the admin", with what each costs and a recommendation:
   - (a) the built-ins stay the first implementations, `A63-H` also drives them through the
     shell, and no packaged local provider is built;
   - (b) the packaged local providers are built as well and pass the same suites over MCP, while
     S10's configurations stay as the owner accepted them;
   - (c) the packaged local providers become S10's first implementations, which changes what the
     owner accepted, and so is the owner's question.

   Do not decide it.
9. **Each provider package.** For each of `A63-RP`, `A63-MP`, `A63-PS`, `A63-FS`, `A63-KC`,
   `A63-DR`, `A63-GS` and `A63-1P`, give:
   - its crate (`packages/provider-<name>/`), its Type or Types, the capability it offers, and the
     existing tool it is built on;
   - D15's four fields, as S10's M1 defines them, as planned: what it implements itself, what it
     delegates, its persistent state, and what the operator sets up;
   - for a runtime, its gate (PRV-025) and how what answers at its `url` proves by TLS that it
     serves the instance (PRV-028), with the certificate the engine gives each run (order 080);
   - its tests, the contract vectors it must pass, and its size;
   - whether the engine can use it on `main` today, and if not, which order lets it: DSP for
     runtimes (on `s3/m2` until the pin); `PSD` for package sources (not yet written); 049a-1 and
     049b for the machine contract after enrollment; `SEC-D` for secrets.
10. **The contracts' pairs.**
    - **Secrets.** The file store ships with the reference engine as a package; the engine holds
      no store (IN_FORCE.md topic 12). The keychain store is S2's store (ruling 46, point 5), run
      in the owner's logged-in session only: A22 stays open (the admin's decision, option (c)),
      and the store does not wait for it. Follow experiment 005's safety rules for its tests.
      1Password runs after A73 and its approvals; the owner's answer to S10's Q2 is 1Password after
      A73, or else the file store.
    - **Runtimes.** A local process and Docker (`A63-DR`), which serves S2's D20 isolated profile,
      S3's isolated consumers and S10's T3. Say what an Agent package needs to run in a container:
      its home, and its sign-in's browser callback. The Windows node's SSH-R is a third runtime:
      say how it relates to the pair, and whether its gate and `A63-RP`'s can share code.
    - **Machines.** The local machine and the second host. The second host's provider is the one
      M2-1 selects, which the owner holds: SSH-M is the admin's candidate, and a Tailscale machine
      provider the other. Plan for either and name no answer. The machine suite must cover
      reports, placement and enrollment (order 038b's contracts), and say which cases wait for
      049a-1 and 049b. SSH-R and SSH-M are planned in one crate, `packages/provider-ssh`, while
      ruling 40 says one crate per provider: flag it for the admin.
    - **Package sources.** A local directory and Git (`A63-GS`). A provider claims its schemes for
      its whole life (PRV-016), and the engine fetches what no provider claims. Say which scheme
      each source claims. Plan `PSD` as a proposal to the admin: the engine's dispatch of
      `fetch-package.v1`, its own `https` fetching, the archive check (PRV-017, PKG-026) and the
      scheme claims at admission (PRV-013, PRV-016), following order 082 on concurrent first
      installs (MC7b-002).
11. **What starts now.** Order the packages that need no engine change for dispatch now, the
    harness first. Say, for each, how it is tested before the engine can call it.
12. **No new language and no answers for the owner.** A behavior a scenario needs that `spec/`
   does not state is a finding of class gap in your report and a line of "What `main` has now",
   with the spec order it would need, proposed to the admin. Name no answer to M2-1 to M2-4 or to
   any question the owner holds.
13. **No secrets, no local paths.** The plans name no machine name, network name, user name or
   local path. Write `<lab>` for a lab directory.

## Scope

Change only:

- `docs/implementation/PROVIDERS.md` (new);
- one row of the table "Process for the builds" in `docs/README.md` (decision 1).

Do not change any other file: not `spec/`, `conformance/`, `engine/`, `packages/`, ROADMAP.md,
`experiments/`, IN_FORCE.md, docs/AUDIT.md, or any record or ruling. Report anything else that
needs changing.

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
grep -n -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" docs/implementation/PROVIDERS.md docs/README.md
grep -n -E "[[:space:]]+$" docs/implementation/PROVIDERS.md
```

Report each gap as a finding, each order you propose and each estimate as one of your own
choices, and each difference from the state of the work above as a finding. Do not stage
anything.

## Record

Run on 2026-11-11 on the Mac, beside orders 117 to 119, from base 31acd73: the worker in 16
minutes (935 s) and a fresh verifier in 5 minutes (310 s); then a second worker round in 1.5
minutes (88 s) and a fresh verifier in 5 minutes (303 s), which found nothing to change. The
first verifier's one must: the runtime suite refused a repeated `run.v1`, against order 081,
under which a repeated `run.v1` for a running run answers with that run. Round two follows 081
and makes those cases wait for it. Neither verifier's sandbox could run the unit suite; the lane
coordinator ran every check of "Done when" on the result, all clean.

The result is `docs/implementation/PROVIDERS.md` and its row in `docs/README.md`. It plans:

- **the harness:** A63-H1 (the item launcher, MCP driver and reports, M) and A63-H2 (trust roots,
  run certificates, gate clients and faults, L), proposed as a crate in the engine's workspace
  and so the admin's; and four contract suites, A63-VR, VM, VP and VS, under
  `conformance/provider-contracts/`, whose index is the admin's;
- **the packages, lane 8:** A63-RP1 and RP2 (local process), MP1 and MP2 (local machine), PS
  (local directory, `file`), FS (file store), KC (keychain), DR1 and DR2 (Docker), GS (Git,
  `git+https`) and 1P (1Password, after A73), each with D15's four fields as planned;
- **the engine's package-source work, proposed to the admin:** PSD1 (admission, scheme claims
  and serialized first installs, after 082), PSD2 (source calls on DSP's path) and PSD3 (the
  engine's own `https` fetching);
- **questions for the admin:** where the harness lives (A), local-machine enrollment
  qualification (C), SSH's crate boundary (D), and the order of merging with ruling 75 D4's
  reading (E).

**Decision B, the admin's, logged as a coordinator decision (2026-11-11):** option (b). The
packaged local runtime, machine and package-source providers are built as well; each passes the
same contract suites over MCP as the second providers; S10's accepted configurations stay as the
owner accepted them, with the engine's built-ins as its first implementations. So no question
goes to the owner. The lane coordinator wrote the decision into the plan's box, its section B and
"For the owner".

**Added by the lane coordinator, at the admin's request:** a paragraph "What A63-RP shares with
SSH-R". A63-RP's crate exposes, as a library, the provider's MCP server (discovery, the three
runtime operations over a backend interface, the run ledger, owed stops and the staging of run
files) and the gate (the run's certificate, the engine's proof checked before any MCP bytes, the
byte bridge and the close after a silent cut). SSH-R and A63-DR depend on it by path; each
runtime keeps its own transport, principals, stop mechanics, state and contract text. Question
D's recommendation now points there.

Merged after the second verifier was clean, at the admin's word. A fresh skeptic reviews the
split before any order in it is dispatched.
