# Work order 121 (PLAN-F): the split reviews and the admin's answers, applied to the plans

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count.

## What and why

Orders 117 to 120 wrote the order-level plans of S2 to S10 and of A63, the providers: nine
`plan.md` files under `experiments/013` to `021` and `docs/implementation/PROVIDERS.md`. Before
any order in a plan is dispatched, a fresh skeptic reviewed each plan's split. The lane
coordinator accepted the findings listed below, and the admin answered four questions the plans
put to it. This order writes both into the plans. The decisions of orders 117 to 120 still hold:
the plans' form, kinds, owners, sizes and shared IDs, and their scopes. Read the order that wrote
each plan before you change it.

Apply each change as written. Where one cannot be applied as written, apply the nearest sound
reading and say so in your report. Keep each plan's sections and their order, and split no
section into new files. A scenario plan may grow to about 260 lines, and PROVIDERS.md to about
360. Where an item below changes an owner, a kind or a shared ID that orders 117 to 120 set, the
item governs. A plan may cite a suffixed part of a shared ID (A63-RP1, FLT-D3) where a dependency
is on that part alone.

Read first: the ten plans; orders 117 to 120 under `docs/work-orders/`; order 079
(`079-shell-https-address.md`, its section on the `https` listener); order 049a1
(`049a1-build-enrollment-core.md`, the cases it defers to S3, near its end); and order 077
(`077-spec-reconcile-fenced.md`).

## The decisions it applies

### A. Rules for every plan

1. **D4, provisional.** The admin's reading, of 2026-11-11: ruling 75, D4 limits the optional
   real-client replays of S4, S5, S6 and S9 only. S7 and S8 use the second subscription accounts
   (ruling 75, D2), and S3's and S10's replays use them too (ruling 72, point 2). The owner is to
   confirm it. In each plan, replace the question about D4 with one line: "Planned on the admin's
   reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9
   only. Provisional until the owner confirms." Replace the runs' waits on the admin ("after ...
   A3 resolution", "after dependencies and admin reading" and the like) with one wait, "the owner
   confirms D4's reading", on the counted real-client runs only (S3's, S7's, S8's and S10's), and
   list it under "For the owner". Nothing but those runs waits on it. The places: S3's A3 and its box, rows S3-R1 to S3-R4, and its
   critical path; S7's A2; S8's A2; S10's A2; S4's C; PROVIDERS.md's E; and any line of S5, S6
   or S9 that defers D4 to the admin.
2. **Where a scenario vector goes.** A scenario vector goes in the core or shell order that makes
   it pass, when that order stays at most L. Otherwise it goes in an admin vectors order placed
   right after that order, which depends only on the core and shell orders its cases need: never
   on a lane package, an owner step or a run. A case that tests a provider's own contract goes in
   A63's suite for that contract (A63-VR, VM, VP or VS), which B.1 makes lane 8's. Say this once in each
   plan, at the head of "The orders", and apply it as the sections below say.
3. **M2-3 on runs that providers make.** Every run that a runtime provider makes, through DSP,
   receives the engine's `https` address, and order 079 leaves that listener's TLS termination
   to the owner's M2-3. Name M2-3 as a wait, without an answer, wherever a plan's run or
   qualification needs a runtime provider's run to reach the engine: S2's isolated runs (S2-R3 and
   S2-R4), S10's Docker-only and all-swapped replays (its T3 and T5 configurations, and S10-R2),
   and in PROVIDERS.md the engine's use of A63-RP and A63-DR. Add it to each plan's "For the
   owner" as a held choice the plan waits on.
4. **Order 075 is merged** (55cf747): an Agent Type states the reach of its client's own tools.
   Correct each line that calls it pending.
5. **The sister plans exist.** Remove each line saying that a sister plan or PROVIDERS.md is
   absent, and link the file where the line cites it.

6. **Engine orders do not depend on lane packages.** An order that changes `engine/` tests with
   scripted servers inside the engine's own tests, as its tests do today; a lane package or
   scenario server is qualified by its harness, not by an engine order. Remove each core or shell
   order's dependency on a lane package (S2-L, S4-SRV1, S4-SRV2, S5-SRV, S6-SRV, S6-STOP, S9-SRV
   and the like), and say which fixture the order uses instead.
7. **One proposed merge order for the hot files.** S4's plan holds it, under "For the admin", for
   every plan's core and shell orders; the other plans cite it and say only where their orders
   fall in it. Start from this one and adjust it to the splits below:
   - core: S4-TASK1, S4-SET, S5-C1, S6-ACCESS, S4-REC, S5-C2, S6-DUTY, S9-CORE, S5-C3b, with
     S5-C3a placed by its hot files against S9-CORE's (both edit `registration.rs`); the admin
     interleaves SEC-D1, FLT-D1, S3-L1, S3-L2, S3-RET1 and PSD1;
   - shell: DSP, S4-LATE, S4-TASK2, S5-WIRE, S9-CUT, S6-WIRE; the admin interleaves SEC-D2,
     SEC-D3, FLT-D2, PSD2, PSD3 and S3-RET2;
   - H.1's run-end order spans both: after 077 and before S6-WIRE. FLT-D0 and FLT-D3 edit
     manifest pieces: after 071 and SV1, FLT-D3 after FLT-D2.

### B. `docs/implementation/PROVIDERS.md`

1. **The admin's answers.** Record each as the admin's decision of 2026-11-11:
   - **A, where A63-H lives:** its own crate, outside `engine/`. It goes in
     `experiments/022-provider-contracts/`, a lane path; number 022 was never used in the tree.
     The contract suites and their own index live there too, in their own form (`provider.yaml`),
     so they share nothing with the conformance runner's case form and need no seam with it. The
     runner and manifest stay the admin's. A63-H1, A63-H2 and the four suites become lane 8's.
     The order that creates the directory also adds its row to `experiments/README.md` and to the
     roadmap's register, as A63's; say so in its scope. A63-H1 and A63-H2 are of kind *harness*;
     the four suites are of kind *vectors*, which for them means cases under that directory, not
     under `conformance/`. A suite's safety case carries a field naming its safety promise (ruling
     68, point 2), named as SV1 names it once SV1 merges. No other order of these plans takes 022.
   - **C, the local machine provider:** MP1 stays local; MP2's qualification is settled before
     MP2 is dispatched.
   - **D, SSH:** separate crates for SSH-R and SSH-M (ruling 40), SSH-R reusing the shared
     runtime library (item 3); both wait for the owner's M2-1 and M2-2.
   - **E:** D4 as A.1 says.
2. **The shared runtime library is a crate of its own:** A63-RK, `packages/runtime-provider-kit/`,
   lane 8, the first runtime order, depending on nothing. A63-RP, A63-DR and SSH-R depend on it
   by path; none depends on another provider's crate. Each provider crate keeps its own
   `Cargo.lock` and toolchain and stays outside the engine's workspace, which is how this plan
   reads "self-contained" (IN_FORCE.md topic 5). Rewrite "What A63-RP shares with SSH-R" as
   "What the runtimes share", with this interface:
   - **Backend calls:** start a run from its staged item and return a duplex byte channel to the
     run's stdio; stop the run's whole tree; report its state. Staging the run's files and the
     state store (the run ledger and owed stops) are backend interfaces too: SSH-R stages its
     files on the other machine.
   - **The run's lifetime follows SSH-R's model:** a run is tied to its channel and ends when the
     channel ends; the provider stops every run it does not hold at startup; a configured bound
     is the backstop. A63-RP uses process groups, not launchd or systemd supervision; drop the
     supervisor from its operator set-up and tests.
   - **One gate listener per run,** on its own port: DSP's bridge checks no host, and an IP URL
     carries no SNI, so one listener cannot choose among runs' certificates.
   - **The silent-cut bound** runs from the enforcement point's connection, as PRV-025 states it,
     checked by the gate's own liveness probe; with no HTTP/2 in DSP's client, that probe is TCP
     keepalive.
   - **Owed stops** survive a provider restart, the instance's removal and the provider's
     retirement (PRV-021). After retirement the provider's server never runs again, so each
     runtime names the mechanism outside its server that carries them out. A stop that succeeded
     stays true, cancels a start still in flight, and is followed by no restart (order 081).
3. **Dependencies.** A63-H2 builds its own fixture gate and does not test through A63-RP. A63-DR1
   and A63-DR2 depend on A63-RK. SSH-R depends on A63-RK, not on A63-H. A63-VM does not depend on
   A63-H2 (machine providers are reached over stdio, with no gate). A63-RP1 covers the running
   case of a repeated `run.v1`; its uncertain case waits for 081. The packages need not wait for
   A63-H1: each tests itself with its own scripted MCP client, as agent-codex does, and A63-H1
   gates only the qualification of a suite. A63-VR waits for 081 only for its repeated-run
   cases; A63-VP's and A63-VS's cases that are not safety cases need not wait for SV1; A63-GS's
   suite pass does not need PSD; PSD3 needs PSD1 and DSP's TLS, not PSD2.
4. **Sizes.** No order above L, by the sister plans' bands (L about 550 to 900 lines, M about 300
   to 600). The gate is A63-RK's: split A63-RK into its server and backend interfaces, and its
   gate, if it is above L. A63-RP2 connects A63-RP to A63-RK's gate and adds recovery and the OS
   principals; A63-DR2 splits into the stop with A63-RK's gate, then the Agent profile, after
   050, which it now depends on. Split A63-H2 (trust fixtures and refusals; then fault controls)
   and PSD1, and mark A63-RP1 and A63-DR1 L or split them.
5. **What the suites ask of a provider.** Only what `spec/` states:
   - the source suite asks for the exact bytes for a URL whose scheme the provider claims; a
     corrupt archive, an unsupported scheme and a claim's lifetime are the engine's checks, in
     PSD's cases;
   - a whole-tree stop is a parameter each runtime's contract declares, not a suite assertion;
   - A63-FS does not list SEC-020, which binds the engine; A63-VP does not list PKG-026;
   - in Docker, "no gate key is mounted" means the gate's own key; the run's `tls.key` is
     delivered to the run as PRV-022 says.
6. **Work no order covers.** Add, proposed to the admin, the engine's dispatch of `place.v1` and
   of `list-machines.v1` beyond order 051c's report loop (PRV-015, a safety promise in the
   catalogue). Add to A63-DR's scope, or to S3's, a Docker daemon or context on the second machine
   and PRV-011's channel to it. Add owner steps for the distinct OS users that A63-RP's and
   A63-H2's tests need, and for authorizing A63-KC's binary on the item's partition list
   (experiment 005).
7. **S3-A741.** A74's stand-ins and A63-H2 both build a gate check and silent cuts. Build the
   configurable silent cut once, in S3-A741, which starts first and exposes it for reuse, and let
   A63-H2 reuse it. S3's plan says so too.
8. **Smaller corrections.** A63-MP1 emits the machine `id` now (PRV-014 already requires
   ENR-002's id). PSD1's hot files include the loader's and the packages modules for PRV-016's
   Type check. A63-DR's contract states the limits of an Agent's own tools under it (PRV-006, as
   order 075 amended it).
9. **Every plan's Status** says that this order revised it, against the commit you read.

### C. S2's plan (`experiments/013-s2-shared-login/plan.md`)

1. **S2-R3 and S2-R4** wait on M2-3 (A.3), beside A63-DR, DSP and ADDR. "For the owner" names it.
2. **Vectors.** Drop S2-V as a separate order. V3 (`s2-rotate-replace-repair`) goes with the
   SEC-D part that makes it pass, or in an admin vectors order right after it (A.2). V6
   (`s2-cached-tool-refused`) goes with the order that makes it pass; if it passes at the base,
   it goes in FLT-D0 (C.4).
3. **Engine orders do not depend on a lane crate** (A.6). SEC-D1, FLT-D1 and FLT-D2 carry the
   restriction contract O3 settles as their own test fixture, written from S2's README, and depend
   on neither S2-F nor S2-L: conformance cases supply each implementation's completion
   (`conformance/README.md`), and engine tests use scripted servers.
4. **FLT-D's split.** The tree already re-sends a filtered read, checks filter support and
   rechecks closure, so FLT-D cannot be sized before its cases run. Make FLT-D0 an XS or S admin
   vectors order, after 071, SV1 and the pin, that writes V1, V2 and V5 and commits only the cases
   that pass at its base; a case that fails is reported and moves to FLT-D1, which fixes what
   those cases show and is sized when they have run. FLT-D2 stays the shell's wire work. FLT-D3
   is S8's V4 and V5, an admin vectors order after FLT-D1 and FLT-D2 only; S8's plan cites it.
5. **SEC-D3 is two jobs.** Split it: secret-bearing remote connections, which DL16 leaves
   unserved, apart from SEC-023's authorized reread after a value is lost. Keep each at most L.
6. **SCR or S2-H.** SCR, owned by S4's plan, carries the keyed replay after a filter's deadline
   and the forced stale calls; S2-H keeps the clock and the faults. Remove A3's question, and say
   which SCR order supplies them.
7. **S2-L's crates.** The adapter and the Conduit-aware implementation are separate crates under
   `packages/`, so that D01 counts each implementation apart; the local Linear stub stays with
   S2-L1. S2-L2 need not wait for all of S2-L1: say what it waits for.
8. **A63-KC** depends on S2-O's throwaway test keychain: say so in S2's plan and in
   PROVIDERS.md's A63-KC row.
9. **The hot files.** Replace the parallel lanes on core `requests.rs` with the merge order of
   A.7, and say where S2's and S3's orders fall in it.

### D. S3's plan (`experiments/014-s3-two-machines/plan.md`)

1. **SSH-R's chain.** SSH-R depends on A63-RK, the shared runtime library (B.2), and waits on
   M2-1 and M2-2. Correct "SSH-R depends only on main", and add A63-RK to the dependencies of
   S3-P, S3-H and the run gate, with A63-VR for SSH-R's qualification, and to the critical path.
2. **Vectors** (A.2). Drop S3-V1 and S3-V2 as separate orders. V1 and V2 go with S3-L2, or with
   an admin vectors order right after 077's build, whichever makes them pass. V5
   (`s3-gate-identity`, PRV-025) goes in A63-VR, the runtime suite. V6 goes with S3-RET1. V7 is a
   composite in an admin vectors order after 049b: it cites order 049a1's cases
   `enrollment/admit-fingerprint`, `enrollment/admit-not-admissible` and
   `nodes/no-node-without-admission`, adds only what they do not cover, and does not wait for
   S3-RET2.
3. **The cases 049a1 defers.** `nodes/declare-lost-removal` and `nodes/gone` go in S3-L1.
4. **One owner for the owed stop after removal:** S3-L2. S3-L1 no longer claims it.
5. **Split S3-O.** S3-O1 is the machine pair as stage 0 sets it up, which A74's real-machine
   half needs; it does not wait for M2-1 to M2-4. S3-O2 is the second accounts, Docker and the
   service on the pair, after the M2 answers. S3-A742 depends on S3-A741 and S3-O1, not on
   S3-O2.
6. **A81.** Its directory is its own, under `experiments/`, with the next unused number under the
   roadmap's rules other than 022, not under S3's. Run its route by an SSH forward done by hand before S3-RET1 is
   dispatched, and the route through the person's Interface after S3-RET2.
7. **Items that run on the second machine.** An archive item runs only on the engine's own runner
   (order 078). Add the orders that make S1's and S2's servers runnable there: one on lane 8 for
   PATH items for S2-L's binaries in their own manifests, and one proposed to the admin for S1's
   server items in the S1 example and fixtures. Size them and place them on the critical path.
8. **S3-M** depends on A63-VM, A63-MP2 and the admin's decision C (B below), and shrinks to
   running that suite on the selected second-host provider and recording the result.
9. **A74's unmeasured cells.** A2 reads "A74's dated H1–H10 observations must exist first" as
   allowing unmeasured cells, which narrows what the owner accepted. Move it to "For the owner",
   as a question with its options and a recommendation.
10. **Smaller corrections.** Reconciliation in `requests.rs` starts at line 937, not 919. The run
    gate needs S2's prerequisites, as S3's README says, not S2's passing runs.
    `s3-loss-cause-schema` is not a safety vector: PLC-024's safety part is the closing of
    Conduits (the safety catalogue), not the cause field.

### E. S7's plan (`experiments/018-s7-outside-package/plan.md`)

1. **The cycle between S7-H1 and S7-P.** S7-H1 builds the disposable upstream stub itself, under
   `experiments/018-s7-outside-package/`, and qualifies `mcp-remote` against it.
2. **S7-P.** The README's preparation asks for no reference plugin or adapter: the outside author
   builds them in T3. Drop them from S7-P; keep only what a pass condition needs, or drop S7-P.
3. **The author's repository and npm scope.** Add an owner step before S7-N that sets up the
   author's repository and npm scope (Q4: TypeScript on npm, in a repository the author owns).
   Any local stand-in for "on npm" is a question for the owner, not a plan's choice.
4. **FLT-D.** S7 depends on FLT-D only if the author's plugin declares filter support; D01 has no
   restriction cases. Say so, and drop the dependency otherwise.
5. **Waits that are not needed:** S7-V on S7-P, and S7-H2 on S2-L.

### F. S8's plan (`experiments/019-s8-filtered-search/plan.md`)

1. **V4 and V5** are FLT-D3 (C.4), after FLT-D1 and FLT-D2 only. FLT-D no longer depends on
   S8-H2.
2. **DSP.** FLT-D2 depends on DSP, so S8's critical path shows the DSP join.
3. **The oracle.** S8-A depends on S8-H1. Finalizing the oracle after S8-A is an order of its own
   (S8-H1b), or part of S8-H2: say which.
4. **SCR.** S8-H2's scripted checks use SCR.
5. **S8-C** is one contracts-only package that both adapters pin, as S2-F is; size it again
   (likely M).
6. **S8-V** need not wait for S8-H2.

### G. S10's plan (`experiments/021-s10-swapped-providers/plan.md`)

1. **Vectors** (A.2). V2 (`s10-runtime-equivalence`, PRV-025) becomes A63-VR's safety case, run
   on the runtime pair, and S10-V is dropped. V1's safety promise, SEC-007, is a refusal by the
   engine, and V3's, PKG-026, is the engine's archive check: their safety parts stay with SEC-D and
   PSD1, or an admin vectors order right after each, and A63-VS and A63-VP show only the pair's
   equivalence. No vector's safety promise changes; a change would be the owner's question. V5
   and V6 go in an admin vectors order after SEC-D, FLT-D and PSD1. V4's engine part stays in
   PSD1. PSD no longer depends on S10-H1 or A63-GS, and FLT-D no longer depends on S10-H2.
2. **Split S10-H2.** S10-H2a, now: the D15 and D16 ledgers and the comparison tool. S10-H2b,
   later: the freeze and the resets per configuration, after S2-H, S2-R1 and the provider
   packages. D09's rotation controls on the second store are part of S10-H2b, since S2 defines
   D09 on the keychain.
3. **M2-3** (A.3) on the Docker-only and all-swapped configurations, and on S10-R2.
4. **S10-R1's "P3 reports"** name their orders, as B.4 splits them: A63-RK, A63-H1 and A63-H2,
   A63-RP's and A63-DR's parts, A63-PS, A63-KC, A63-1P or A63-FS, A63-GS, A63-VR (whose
   repeated-run cases wait for 081), A63-VP and A63-VS. R1 also needs S2-H and SCR.
5. **A73.** Add a probe order for A73, the 1Password probe, on lane 11 (the probes, which this
   coordinator also runs), in its own directory with the next unused number other than 022, with
   the owner's approval before its desktop-app case. "If A73 is late" reads the owner's "or else":
   make it a question for the admin, marked provisional as A.1 marks D4.
6. **Git over `https`.** A63-GS claims only `git+https`, so S10-O also sets up a disposable
   HTTPS Git repository and its credential.
7. **Items that run in Docker.** The Agent packages (050) and S1's and S2's service packages need
   items that a container can run. Assign them: 050's owner, S2-L, and a proposal to the admin
   for S1's.
8. **Smaller corrections.** PSD1 is a core order, not a shell order. PRV-016 starts at line 487.

### H. S4's, S5's, S6's and S9's plans (`experiments/015`, `016`, `017` and `020`)

1. **Fences from a run's end** (S6). Order 077 leaves out the fences that come from a runtime
   provider's report and from setup parts, and the shell's findings it names (stopped runs
   reported only on drop; nothing after a restart); S4-LATE leaves them out too. Add to S6's plan
   a proposed admin order, after 077: the shell reports a run stopped on its own exit, after a
   stop part and after a restart, and the core fences on PRV-009's "stopped" report. S6-WIRE
   depends on it and on S4-LATE, and on S9-CUT if T4's restart case relies on it.
2. **S6-ACCESS depends on 081**, which rewrites the text its V2 (CON-008, LIF-024) and V3
   (ACC-039) test.
3. **S6-STOP** is built on A63-RK, the shared runtime library with its gate (B.2, B.4), with a
   fault-injecting backend, and depends on A63-RK; on A63-VR too if it runs the runtime suite.
   Size it again. Drop PRV-021 from it, and remove the line "while the instance exists". Reuse S3-L2's deterministic provider
   fixture where they overlap, or say why not. S6-STOP adds no case to A63's suites.
4. **S6's onward calls after a close.** FLT-D1 (S2's plan) owns the fix; S6-ACCESS cites it and
   adds only S6's cases.
5. **S9's V3** (`s9-interrupted-initialize`) passes on today's DL6 core for its empty and
   committed cases: it goes to S9-CORE, or to an admin vectors order after the pin and SV1. The
   kill and reopen tests stay in S9-CUT.
6. **S9's cuts in the author runs.** S9-CUT's barriers are test-only, but S9-H and S9-RUN need
   acknowledged cuts in the binary the authors run. S9-CUT owns a pinned lab build with those cuts
   behind a cargo feature that no release enables; S9-H and S9-RUN use it.
7. **SCR.** Split it into SCR1 (the core client) and SCR2 (the task drivers, and S2's extras:
   the exact keyed payload replayed after a controlled deadline, forced cached tool names and
   serving IDs, assertions that retained outputs are absent, and clock synchronization). Say how
   SCR runs as an Agent's run: through the Codex package's `scripted-client` binary, or a
   scripted Agent package of its own, and what changes when order 050 changes the Agent packages.
8. **A real client's lost response** (S4's E09). SCR's dropped replies do not make a real client
   lose one. Add, to S4-H or as E09's own preparation, a loopback proxy between the Agent and the
   engine that drops the response and records the wire trace for A6, and say what the Agent's
   engine URL needs for that: a question for the admin, as 050's owner.
9. **Splits.** S5-C3 into S5-C3a (E03 and E04 admission with V1, needing only the pin and SV1)
   and S5-C3b (V3 and V5). S9-CORE into V1, V2 and V5 (the pin, 042 and SV1) and V4 (047, 077
   and 081). S9-CUT into the initialization cuts, and repair with the declare-lost refusal. S4-REC
   into its parts, or its vectors into an admin vectors order. S6-DUTY into V4 (the duties) and V5
   (the cascade and PND-007's notice).
10. **Waits and overlaps.** S4-REC does not wait for S4-TASK1 (its V3 to V5 involve no task);
    S5-C2 does not wait for 082. The part of S9-CORE that touches hook retry and LIF-027 depends
    on S4-SET, which fixes them first; S9-CUT drops the hook regression S4-LATE already carries. S4-SRV2 and S9-SRV share one
    hook that loses its reply: one reuses the other.
11. **S9's T2** needs a keyed effect and an interrupted call: name S4-SRV1 for them, or add them
    to S9-SRV.
12. **Smaller corrections.** S4-LATE's scope states how long it keeps a reply's slot after a
    timeout, in CONFORMANCE.md's DL2 and DL10 rows. S9-CUT's scope updates CONFORMANCE.md's H03
    paragraph, since a local declare-lost will no longer answer -32603. S9's box lists S4-LATE
    once, as S4's. CONFORMANCE.md's paragraph that S9 cites starts at line 324.

## Scope

Change only the nine files `experiments/0NN-*/plan.md` (013 to 021) and
`docs/implementation/PROVIDERS.md`.

Do not change any other file: not a pre-registration README, not `spec/`, `conformance/`,
`engine/`, `packages/`, ROADMAP.md, `experiments/README.md`, IN_FORCE.md, docs/AUDIT.md, or any
order, record or ruling. Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
F="docs/implementation/PROVIDERS.md $(ls experiments/0*/plan.md)"
grep -n -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" $F
grep -n -E "[[:space:]]+$" $F
grep -n -i -E "sister plans.*(absent|not( yet)? present|unwritten)" $F
```

Report, for each lettered item, what you changed and where; each item you could not apply as
written, with the reading you applied; and each new order or ID you introduced, as one of your
own choices. Do not stage anything.

## Record

Run on 2026-11-11 on the Mac, from base 1b717c4: the worker in 20 minutes (1184 s) and a fresh
verifier in 7 minutes (433 s); then a second worker round in 2 minutes (104 s) and a fresh
verifier in 4 minutes (235 s), which found nothing to change. Round one's must: S8-V still
depended on S8-A, an owner step, against A.2; round two removed it, keeping S8-A before the
adapters. Its should: S3's run gate and critical path lacked A63-MDIS, the engine's dispatch of
`place.v1` and `list-machines.v1`; round two added it. Neither verifier's sandbox could run the
unit suite; the lane coordinator ran every check of "Done when" on the result, all clean.

The ten plans now carry the admin's four answers and the accepted fixes of the four split
reviews, as the decisions above list. The plans run from 175 to 253 lines, and PROVIDERS.md to
326.

**The lane coordinator's decision, accepted by the admin (2026-11-11):** the code the runtime
providers share is a crate of its own, A63-RK, `packages/runtime-provider-kit/`, built first on
lane 8 in two parts (its server and backend interfaces; its gate). A63-RP, A63-DR, SSH-R and
S6's stop provider depend on it by path, and no provider crate depends on another's. This
refines the admin's earlier answer that SSH-R reuse A63-RP's library.

**Added by the lane coordinator, at the admin's answer of 2026-11-11 on scenario servers:** a
package that serves a scenario goes under `packages/` with a directory named `scenario-sN-...`,
and its README says that it serves that scenario and is not a provider for general use. The
coordinator renamed the planned directories (S2's restriction contracts, its Linear aware
implementation and its adapter; S8's contracts and its two adapters; S6's stop provider) and
added the rule at the head of "The orders" in the plans of S2, S4, S5, S6, S8 and S9.

The admin accepted as proposals the rule for where a scenario vector goes, the rule that no
engine order depends on a lane package, and the merge order for the hot files kept in S4's plan.
It takes into its lane, after the pin, the three admin orders the reviews added: the engine's
dispatch of `place.v1` and `list-machines.v1` (A63-MDIS), the fences that follow a run's end, and
S1's servers on the second machine and in Docker.

Merged at the admin's word. Dispatch now follows the admin's order: A63-RK and A63-H1, then the
local providers, then the scenario servers.
