# E04-signin-isolation: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** Sign-in is fully written in the specification (PND-005, PND-010, PND-011, PND-013, IFC-049, CTR-013) and not built at all: sign-in.v1 is a stub, none of the 494 conformance cases names a sign-in or pending-action promise, and both Agent packages still run on the owner's shared login. Six scenarios and the dogfood store's re-initialization after order 050 all sign in through it, and as planned the first real sign-in would happen there with no pass condition; so A5 should become the test, staged: two cheap probes of the real clients, a tracer slice to one checked sign-in, then four Agents on four subscriptions with the wrong-account, bystander and robustness cases, then a soak. What is working is the design's core: the account check, not the browser, keeps accounts apart, the engine holds nothing, and the rules follow OAuth's own RFCs and the protocol's URL elicitation. What is not is the word "isolation": under one operating-system user any Agent that runs code, and every dogfood slot is one, can use and replace another Agent's login and read root's token, and the specification says so; the test can show separation and what crosses Conduit, and must list the rest as not shown. The cheapest place to test the real claim is a Linux user per Agent on the Windows computer through SSH-R, whose own contract names it and whose configuration today refuses it; that amends ruling 77 point 2 and is the owner's. Two unknowns can sink the build and should be probed before any order is written: whether Codex can attest its account (without it a Codex Agent gets no sign-in), and whether a rebuilt redirect completes Codex's login (without it the return half of PND-013 is not worth building for Codex). The plan's serial path (046, 047, 048a, 048b, 050) puts those answers behind five to six thousand lines by its own low estimates; a tracer of about two thousand reaches them first. The chain as named also omits what a device code, a pasted URL and the Windows leg need (the panel, PND-013's return, the node path). Smaller findings on the way: no promise for a sign-out at retirement, nor for removing a home under any runtime provider but the engine's own runner; SEC-020 cited for something it does not say; no rule for two sign-ins of one instance at once. The owner has two things to rule: what A5 is and whether the second accounts may do Conduit's own work once signed in through it, and whether to add the user-per-Agent arm.

**What the analyst read.** Commit 2eb8f258 (2026-11-12), read-only, at the review checkout. Nothing was built or run, no client was started, no network was used, and no credential or environment was read.

Read in full: docs/guides/full-review.md (all but Focus 1 and its queued list); docs/implementation/HANDOVER.md; AGENTS.md "What holds" and "Where things go"; the records of rulings 49, 62, 66, 72, 73, 75 and 77; spec/promises/pending.yaml; SEC-019 and SEC-020; CTR-013 and CTR-014; IFC-006, IFC-007, IFC-045 and IFC-049, and the sign-in sentences of IFC-010 and IFC-016; PRV-006, PRV-011, PRV-025 and PRV-029; ACC-027; OPS-027; TPL-012's order; DL13, DL16, DL18 and DL19; sign-in.v1 and the three hooks in spec/core/operations.yaml; spec/README.md "Pending actions, sign-in and the network"; docs/work-orders/038c-staged (all four files); docs/proposals/sign-in/README.md; docs/guides/accounts.md; both Agent packages' READMEs and manifests; experiments/010 results; docs/AUDIT.md Purpose, preservation tests and the layered claim; ROADMAP.md's order of the work, rows A5, A6, A7, A74, A78, A81 and A82, the rules for proposing an experiment, the program and the scenarios.

Read in part: docs/proposals/spec-batch-1/README.md (lines 30-70, 115-186, 260-330, 395-410, 540-700); experiments/013-s2-shared-login/plan.md (status, what main has, lines 222-322); experiments/014-s3-two-machines README and plan (by search, and the rows for RET1, RET2, A81a, A81b); order 038c (lines 12-96) and order 050a (two excerpts); packages/provider-ssh-runtime README (lines 14-56), contracts.yaml and config.rs (the validation); engine/CONFORMANCE.md's DL16 rows; engine/crates/core/src/requests.rs (the stub only); experiments/005 (the results table) and 009 (section 7); docs/guides/second-machine.md and IN_FORCE.md topic 9 (by search); docs/implementation/PROVIDERS.md (the Docker rows).

Counted: 519 promises under spec/; 494 cases in conformance/manifest.yaml and the cases naming each sign-in and isolation promise.

Not read: the engine's code beyond the stub; any conformance case; the K1 protocol check; the plans of S4, S7, S8 and S10 beyond a count of mentions; the records of rulings 47 (beyond its required list), 48, 50, 56, 68, 70, 71 and 74; the first-build logs beyond two searches; the private handoff note, which is outside the tree. What the clients do (Claude's keychain entry keyed by its configuration directory, what each status command prints, Codex's port and callback) is as the repository's documents report it, not verified; I7 and I9 exist to verify it. I13 rests on a search for an in-progress rule, not on a reading of every operations promise.


## FR-421

**A5 becomes the test the second accounts are kept for: sign-in through the Interface and each Agent's own subscription, in four stages**

E04 I1 · experiment-change · verifier: partly · decides: owner

Register row: A5 (ROADMAP.md:62), rewritten in place. Results in experiments/024, the next unused number (S3's plan had pencilled it for A81, which becomes this test's cross-machine leg, I9).

Claim tested: the purpose's "An agent's subscription and context are its own; anything shared goes through a Resource", in the checked form of rulings 49, 73 and 75: claims 1 to 6, 9 and 11 of I3. It also states what it cannot show (I2).

Accounts, by label only: primary Claude, primary ChatGPT, second Claude, second ChatGPT. Linear is A82's leg (I10). Each `account:` value comes from a private overlay outside the repository; the harness writes labels, never addresses.
Machines: the Mac holds the engine, `conduit`, the browser and the engine's own runner. The Windows computer under `conduit-run` enters only in A81's leg.
Agents: one Fleet, four Agents that name no directory, so their clients' own tools are off: claude-primary, claude-second, codex-primary, codex-second.

Stages, with the owner's minutes:
0. Probes of the real clients, no build (I7): 10 to 15.
1. The first checked sign-in, on slice 1 of I6: codex-primary, or claude-primary if Codex cannot report its account: 5.
2. Four homes, 35 to 45: (a) register and set up the four; (b) send each one turn before any sign-in; (c) sign in claude-primary and codex-primary; then codex-second while the browser is still on the primary ChatGPT account (the wrong account); then codex-second correctly; then claude-second; (d) after every sign-in, undo and retirement, check every other Agent again (a repeated `conduit sign-in`, which ends at the account check and shows no URL) and the owner's two everyday logins by their status commands; (e) one turn per Agent, then S1's shared directory and message history between a primary and a second Agent; (f) the isolation matrix of I2; (g) the robustness cases of I8; (h) retire codex-second, check, register it again and sign in to the new home.
3. The soak, as the owner answers I4: 5 to 10.

Pass conditions, fixed before the run:
P1 Fresh home: each new Agent's first account check reports no account while the owner's everyday logins are signed in, and a sign-in action names the expected account.
P2 Checked: each sign-in ends succeeded once, with the expected account, and answers its action.
P3 Nothing before: the turn of (b) reports unavailable with the action's ID and is never dispatched later; a new turn after sign-in succeeds.
P4 Wrong account: ends failed naming both accounts; the next account check reports none; the action stays pending; no SignInUndone=False.
P5 Nothing crosses: zero hits for the relayed URL, `code=`, `state=`, a device code or a token-shaped string in every result the Interface received, every Agent's tool list and turn output, the exported records, the engine's log, and the store outside the instances' state directories. No Agent is presented a sign-in tool or a pending action.
P6 Reach: no tool presented to any Agent returns anything from another Agent's state directory; a tools-off Agent asked to read a canary file placed beside the other's login returns no canary.
P7 Bystanders: after each sign-in, undo and retirement, every other Agent's check and both everyday logins still pass.
P8 Each robustness case ends in its registered outcome, with a diagnostic naming its rule and cause.
A failed P1 for Claude on macOS, or a failed P7, is the answer A5's row already feeds: "Agents need separate OS users or containers".

Measures, each from records or the harness clock: owner seconds per sign-in (URL relayed to account seen), by login kind; the person's actions per sign-in, listed apart from authored lines (ruling 68, R1 b); `account:` lines authored; attempts per completed sign-in; each case's outcome, reason and ruleId; P5's counts; the isolation matrix (reached, not reached, not tested) by reader, login and route; P7's checks passed of those made; in the soak, checks passed, lapses, and minutes to sign in again.

Evidence, holding no code, token or address: record IDs, outcomes, reasons, ruleIds and action IDs; `signInLogin`'s name, return, node and returned; names, sizes, modes and modification times of the files in each home, never contents; counts from the scans, never the matches; canary strings the harness made. Before commit, a check searches the evidence for address, token and code shapes. No screenshots.

Costs: a harness of about 300 to 500 lines of Rust and 60 to 75 owner minutes on the Mac. Buys: sign-in's faults are found once, against pass conditions, instead of inside six scenarios' owner-attended runs and the dogfood store's re-initialization, where today the first real sign-in would happen with no pass condition at all.

**Evidence.** ROADMAP.md:62 (A5 today: "Four accounts for 48 hours: do two Claude and two Codex subscriptions coexist under one macOS user?"; done when "Refresh and logout behavior recorded per account"; feeds "If they collide, Agents need separate OS users or containers"). docs/AUDIT.md:281 ("An agent's subscription and context are its own; anything shared goes through a Resource"). docs/proposals/ruling-72/README.md:78-82 ("as a test of agent subscription isolation and our entire model"). docs/implementation/HANDOVER.md:157-158 ("After 050, the store is initialized again and the owner signs each slot Agent in once"). experiments/013-s2-shared-login/plan.md:228-234 (the table puts the first sign-ins inside S2-R1, S7-O and S3-O2). Sign-in appears in the plans or pre-registrations of S2, S3, S4, S7, S8 and S10 (grep -c -i 'second account\|second subscription\|sign-in\|signs in' over experiments/013, 014, 015, 018, 019, 021: every file nonzero). docs/proposals/spec-batch-1/README.md:155 ("A5 (your second accounts) does not gate the rerun"). experiments/014-s3-two-machines/plan.md:141 (024 pencilled for A81). docs/proposals/ruling-47/README.md:45 (A5 is observational, not in v0's gate).

**Waits.** I4's answer fixes stage 3. Stage 0 waits on nothing; stage 1 on slice 1 of I6; stage 2 on slice 2. S2-R1, S7-O, S3-O2, S10-O and the dogfood re-initialization then reuse a tested flow.

**Verifier.** partly. Reopened every cited source at 2eb8f258. Holds: ROADMAP.md:62 (A5's row as quoted, its Needs already naming the sign-in chain); docs/AUDIT.md:281-282; docs/proposals/ruling-72/README.md:76-82; docs/implementation/HANDOVER.md:157-158; experiments/013-s2-shared-login/plan.md:228-234; docs/proposals/spec-batch-1/README.md:155; experiments/014-s3-two-machines/plan.md:141; docs/proposals/ruling-47/README.md:45; the grep count is nonzero in every README and plan of 013, 014, 015, 018, 019, 021 (and also 016, 017, 020). Does not hold: (1) 'A5 becomes': docs/proposals/ruling-49/README.md:37-38 already says 'A5 runs on it with the owner's second accounts, including a sign-in to the wrong account, which must be refused'; docs/guides/accounts.md:42-46 and spec/README.md:415-416 say the same. (2) 'the first real sign-in would happen with no pass condition at all': S3's accepted pre-registration has P5 (experiments/014-s3-two-machines/README.md:60-63, accepted at :4-5), task T1 (:111), measure M2 (:156) and vector V6 (:172) for exactly this; S2 registers it at experiments/013-s2-shared-login/README.md:186-189. (3) The S1 rerun itself signs real Agents in before any of S2 to S10: docs/proposals/ruling-68/README.md:41-43 and docs/proposals/spec-batch-1/README.md:122-123 ('Each Agent also needs one sign-in'), with 048a, 048b and 050 on the rerun's critical path (:664; HANDOVER.md:217). (4) P5 fails on a conforming engine: IFC-049 puts the URL, with its state, in the InputRequiredResult the Interface receives (spec/interface/implementation.yaml:1363-1365, 1405-1406).

**Verifier's counterargument.** The staging and the fixed pass conditions are the real contribution; the framing is not. What is new is the order (before the dogfood store and the scenarios) and the conditions, not what A5 is, so the row's rewrite carries out ruling 49 and is the coordinator's; only placing it ahead of other work is the owner's. The item never says whether its stages come before the S1 rerun. If stage 2 does, it contradicts 'A5 ... does not gate the S1 rerun' (spec/README.md:415; spec-batch-1 README:155) without saying so; if it does not, the first measured real sign-in is still the rerun's. Better: stage 1 (one checked sign-in on a primary account) is the rerun's rehearsal and sits on its path anyway; stage 2 (second accounts) follows the pin. The isolation matrix duplicates S2's accepted measure D20 (experiments/013-s2-shared-login/README.md:146: attempted, readable, not-readable, not-tested by credential location, local and isolated); reuse D20's definition and add 'another Agent's home and login' as a location, instead of a second vocabulary. Nine real-account cases mean ten or more OAuth authorizations of the owner's working accounts in an hour, and several end only at DL18's wait; most are engine behavior and belong on the scripted fixture with the local OAuth stub that 048a already plans.

**Verifier's corrections.** P5 must exempt inputRequests[0].url of the InputRequiredResult returned to the Interface that asked, and must scan for exact known values, not 'token-shaped' strings: the store legitimately holds Conduit's own signed tokens and keys, and a real login's token cannot be matched without reading it, which the standing rule forbids (HANDOVER.md:285). P7 must add the third bystander: the primary ChatGPT login in conduit-run's home on the Windows computer, which the workers run on (HANDOVER.md:179-180) and which the wrong-account undo of the same account could touch. Owner minutes sum to 55 to 75, not 60 to 75, and exclude every DL18 wait. 'Six scenarios' is a floor of the real-client ones (75 D2, D4).


## FR-422

**Under one operating-system user the test cannot show that an Agent's login is safe from another Agent, and must say so in every result**

E04 I2 · assessment · verifier: supported · decides: coordinator

The repository already says it. PRV-029: "A process under the same operating-system user may still read it; isolation is mediated". spec/README: such a process "can read the root Interface's token (DL13) and every state directory, an unconfined Agent among them". Order 050a's Type description: an Agent with a directory reaches "the user's own client logins". SSH-R's contract: on the Windows computer "Any run can read every other run's identity files and environment".

So for an Agent whose template names a directory (every dogfood slot today), and for any other process of that user, the test must list as NOT SHOWN:
1. that it cannot read another Agent's login. A Codex login is a file in a directory of the same user. A Claude login on macOS is a keychain entry of that user; experiment 005 found reads by another binary refused or allowed according to how the item was made, so this one is unknown, not safe.
2. that it cannot use it: it can start the same client with the other home, which needs no read.
3. that it cannot replace it: it can put its own login in the other home. The account is checked only at setup and inside sign-in.v1, so turns between checks may run on the wrong subscription.
4. that it cannot act as root: it can read root's token, and so call sign-in.v1, list pending actions and fill `signInReturn`.
5. on the Windows computer, the same between all runs of `conduit-run`, whose own home also holds the primary ChatGPT login the owner made by hand.
6. anything about a malicious package or a model-driven console, which the specification's limits already name.

What the test can show: separation by construction (a home per instance, P1); the account check and the undo; that nothing of a login moves through Conduit; that Agents with their tools off do not reach another's login when asked (a property of each client's launch flags at one version: mediated, not enforced); and that one Agent's sign-in, undo or retirement leaves the rest working.

The matrix therefore has three routes for each ordered pair of Agents: through Conduit (expected: not reached); a tools-off Agent asked to read a canary beside the other's login (expected: not reached); and a plain process of the same user, standing for an Agent's shell (expected: reached, and recorded as the stated limit: a canary and file metadata, never the login; and the client's status command pointed at the other home, to show use without reading anything). A plain process is enough for the third route because it shows what is possible; it costs no owner minute and no subscription. Its rows feed A7 and v0's public security claim.

One hazard follows. The wrong-account undo and a retire hook sign a client out. If a package's home ever resolves to the user's default home, that undo signs the owner out of the everyday login that the coordinator's own session and the plain Codex workers run on. So the package refuses to start a client whose home is not inside its state directory, P1 runs before any real sign-in, and P7 follows every undo. The accounts guide already promises the owner "signing an Agent in never signs you out"; nothing tests it yet.

**Evidence.** spec/promises/providers.yaml:977-979. spec/README.md:413-414, 418-421, 424-427. spec/implementation-defined.yaml DL16 ("A process that runs as the engine's own operating-system user can read the engine's keys and what every run under that user presents, so it is inside the domain of trust and can get past every gate"). docs/work-orders/050a-agents-v2-own-tools.md:163-167. packages/provider-ssh-runtime/README.md:21-31. docs/guides/second-machine.md:108-110 (Codex signed in as `conduit-run` by the owner). docs/proposals/sign-in/README.md:28-30 (Claude's keychain entry keyed to CLAUDE_CONFIG_DIR, quoted from the vendor's page). experiments/005-macos-keychain/README.md:126-135 (reads by `other-reader` refused, -25293, except an any-app item). spec/promises/pending.yaml:200-206 (the account is checked "during setup, before sign-in-hook.v1 in sign-in.v1, or while polling"). spec/conventions/access.yaml ACC-027 ("In this version every restriction is reported as mediated"). docs/guides/accounts.md:24-26. docs/implementation/HANDOVER.md:24, 147 (Codex runs on the owner's subscription; `claude-code.v2` wrote in the slot's `.git` unconfined).

**Waits.** The 'not shown' list goes at the top of A5's results and into A7's row. I5 asks the owner whether to add the one arm where the claim can hold.

**Verifier.** supported. Each quotation reopened and found as given: spec/promises/providers.yaml:977-979 (PRV-029); spec/README.md:412-414, 418-420, 424-427; spec/implementation-defined.yaml:186-188 (DL16); docs/work-orders/050a-agents-v2-own-tools.md:163-167; packages/provider-ssh-runtime/README.md:21-31; docs/guides/second-machine.md:108-110; docs/proposals/sign-in/README.md:28-30; experiments/005-macos-keychain/README.md:126-135; spec/promises/pending.yaml:204-205; spec/conventions/access.yaml ACC-027 ('In this version every restriction is reported as mediated'); docs/guides/accounts.md:23-25; HANDOVER.md:24-25, 147. The six 'not shown' statements follow from these texts. The tools-off route matches the package's own closeout practice (packages/agent-codex/README.md:83-88: 'Closeout must ask Codex to read an outside sentinel').

**Verifier's counterargument.** Three things in the sources that the assessment leaves out change its edges, not its conclusion. (1) Ruling 75 B3 (docs/proposals/ruling-75/README.md:47-48; IN_FORCE.md:314): the engine that runs Conduit's own work moves to a dedicated macOS user before a coordinator Agent drives workers. That removes the owner's everyday logins and the coordinator's session from the Agents' reach, so the 'undo signs the owner out' hazard and the 'act as root's neighbour' rows are true only until B3; the matrix should be stated for both. (2) S2's accepted D20 already defines this measurement, with an isolated arm (experiments/013-s2-shared-login/README.md:116, 146). (3) The third route should not read a real keychain item: the status command pointed at the other home shows use without a read, and a read of Claude's own item by a probe binary is a credential read and may raise a prompt on the owner's screen.

**Verifier's corrections.** The hazard paragraph should name the third everyday login at risk, conduit-run's primary ChatGPT login on the Windows computer.


## FR-423

**The claims of 'agent subscription isolation', each with its promise or 'no promise'**

E04 I3 · fact · verifier: partly · decides: none

1. Each Agent has a home of its own. PRV-029 gives every instance a state directory (empty at first, the same after a restart, its path never reused, removed with the instance), on the engine's own runner only. That the client's home and login live inside it is left to the Agent Type's contract, which is order 050 and unwritten: no promise.
2. Each Agent is signed in to the account its configuration names. CTR-013 (the account place; only an account the service attests), PND-010, PND-011.
3. A sign-in to the wrong account is refused and undone. PND-011; a failed undo holds all model work (SignInUndone=False).
4. No model work before the expected account. PND-005.
5. The engine holds no token. CTR-013's observable ("starts no login and holds no token it yields") and PRV-029's ("reads none of its contents, records none"). SEC-020, which three texts cite for it, does not say it (I12).
6. Nothing of a sign-in reaches an Agent or a record: the URL (IFC-049, PND-002), a code (PND-004), the return (PND-013), the record (PND-012).
7. One Agent cannot use or read another's login. Through Conduit: only PRV-029's "passes it to no other run". Outside Conduit: no promise; the specification states the opposite for one operating-system user (I2).
8. A login carried across machines completes. PND-013 states the mechanism. That Codex's login completes from a carried redirect is no promise: IN_FORCE calls it "real-account evidence to obtain under ROADMAP.md's A81". A Claude login is never carried (carried: false).
9. Revoking one Agent's sign-in leaves the other working. No promise and no operation (I11). What exists: a lapsed login raises a sign-in action (PND-005); retirement removes the directory (LIF-035, PRV-029), on the engine's runner only.
10. An Agent's turns run on the account its configuration names. No promise between checks (PND-011 checks at setup and in sign-in only).
11. Signing an Agent in or out never touches the person's own logins. No promise; the accounts guide asserts it.

State of all of it: the text is in spec/, and none of it is built or tested. sign-in.v1 and answer.v1 answer "not built yet"; of the 494 cases in the conformance index none names PND-001 to 005, PND-009 to 013, IFC-016, IFC-045, IFC-049, PRV-025, PRV-029, SEC-020 or ACC-027 (CTR-013 has two, both about loading a contract); both Agent packages pass HOME through and say "The owner's subscription login is shared; a separate login home is future work".

**Evidence.** spec/promises/providers.yaml:965-984 (PRV-029; "their client-specific subdirectories and credential storage belong to their contracts"). spec/promises/contracts.yaml:282-330 (CTR-013; 303-306; 310-311). spec/promises/pending.yaml:137-157 (PND-005), 159-198 (PND-010), 200-224 (PND-011), 226-296 (PND-013), 33-34. spec/interface/implementation.yaml:1352-1422 (IFC-049; 1420-1421 "No Agent receives a sign-in's URL, login metadata or return"). IN_FORCE.md:195. engine/crates/core/src/requests.rs:1046-1058 (the stub). Count: /usr/bin/python3 over conformance/manifest.yaml: 494 cases; 0 for each listed ID; CTR-013: loader-own-tools, contracts-sign-in-logins. Count of promises: 519 (items with id and then under spec/). packages/agent-codex/README.md:40-41; packages/agent-claude-code/README.md:39-40; packages/agent-codex/src/lib.rs:23 (BASE holds HOME). docs/guides/accounts.md:24-26.

**Verifier.** partly. Recounted with /usr/bin/python3 over conformance/manifest.yaml: 494 cases; zero name PND-001 to 005, PND-009 to 013, IFC-016, IFC-045, IFC-049, PRV-025, PRV-029, SEC-020 or ACC-027; CTR-013 has two (loader-own-tools, contracts-sign-in-logins). Promises under spec/: 519 (245 convention, 226 language, 48 interface). Stub at engine/crates/core/src/requests.rs:1046-1058 as described. Promise texts as cited: pending.yaml:137-157, 159-198, 200-224, 226-294; contracts.yaml:282-329; implementation.yaml:1352-1422; IN_FORCE.md:195; packages/agent-codex/README.md:40-41; packages/agent-claude-code/README.md:39-40; packages/agent-codex/src/lib.rs:23. REV-012 (spec/conventions/revisions.yaml:182-190) confirms no between-check rule for sign-in. What does not hold: 'none of it is built or tested'. PRV-029's state directory is built and tested: order 045 merged as 99b641a3 (#134); engine/CONFORMANCE.md:39 states it; engine/crates/shell/src/tests.rs:82 (private_state_directory_is_empty_distinct_and_stable_after_restart) and engine/crates/shell/tests/changes.rs:364, 457 test it. The manifest indexes runner vectors only, so zero there is not 'untested'.

**Verifier's counterargument.** Claim 1's 'no promise' is right for spec/ and incomplete as a statement of what holds: IN_FORCE.md:187 and 191 keep 'each Agent its own home, made and removed with it, with its own subscription login' as a decision in force, spec pending in order 050 (ruling 49). By AGENTS.md's order that is the second-highest authority, and the review should class it as ruled and unwritten, not as a gap. The list also omits one claim the assigned rulings make: only root, or an Interface whose scope holds the Agent's Fleet, completes its sign-in (ruling 66 point 3; PND-010, pending.yaml:165-167). Real clients cannot show it until Interfaces other than root have tokens (docs/proposals/spec-batch-1/README.md:687), so it belongs on the 'not shown, vectors only' list.

**Verifier's corrections.** Replace 'none of it is built or tested' with: sign-in.v1, answer.v1, the pending actions, the hooks and the Agent homes are unbuilt; PRV-029's directory and CTR-013's contract rules in the loader are built. PND-013 ends at pending.yaml:294. Claim 10 should note PND-005's third trigger (pending.yaml:143), by which a package's contract may report a lapse between checks.


## FR-424

**What is A5, and may the second accounts do Conduit's own work once they are signed in through it?**

E04 I4 · question-for-owner · verifier: partly · decides: owner

In plain words: you kept the second accounts "within conduit as a test of agent subscription isolation and our entire model". The record then says they "give the work orders no capacity outside Conduit". Work orders that run through Conduit's own Agents (A78) are inside Conduit, so the record leaves open whether they count.

(a) A5 becomes the sign-in and isolation test of I1, run in stages before any scenario or the dogfood store signs anything in. Once stages 1 and 2 pass, its soak is Conduit's own work: each slot Agent in its own home on its own account, the second accounts among them, for as long as the work runs.
(b) A5 becomes the test, and the second accounts stay out of work orders. The soak is 48 hours of small scripted turns on the four Agents.
(c) A5 stays the 48-hour coexistence probe, and sign-in is first tested inside S2, S3 and S7, as S2's plan has it.

Recommendation: (a). Real work over days produces the refreshes, lapses and limits that a staged 48 hours of idle Agents never does; it is the model in daily use, which is what you asked the accounts to test; and after order 050 you sign each slot Agent in once anyway. Stages 1 and 2 come first so that the wrong-account undo and the bystander check are proven with you present before anything runs unattended. Choose (b) if you meant the accounts for testing only. (c) leaves the first real sign-in without a pass condition.

What it changes: A5's row; S2's plan table; the handover's queue row 10. Nothing in spec/ or IN_FORCE.md, and no new ruling number: a dated later answer under ruling 72 (ruling 67). A5 stays outside v0's gate, as ruling 47 has it.

**Evidence.** docs/proposals/ruling-72/README.md:76-82 (the owner's two sentences, and the coordinator's summary). docs/guides/full-review.md:42-46 ("never as capacity for work orders"). ROADMAP.md:62, 71 (A5; A78). docs/implementation/HANDOVER.md:120-128, 157-158 (two `codex.v2` slots; signed in once after 050). docs/proposals/ruling-70/README.md cited at HANDOVER.md:36 (Conduit building Conduit is the first priority). docs/proposals/ruling-47/README.md:45.

**Waits.** A5's done-when and stage 3; which account each dogfood slot is given when the store is initialized again after 050; I15's reading of the terms comes with it.

**Verifier.** partly. The owner's two sentences and the record's summary are as quoted (docs/proposals/ruling-72/README.md:76-82); the brief's paraphrase is wider ('never as capacity for work orders', docs/guides/full-review.md:41-43); IN_FORCE.md:264 has 'used only through Conduit'. So the gap between 'no capacity outside Conduit' and 'never as capacity' is real. HANDOVER.md:121-122 confirms both dogfood slots are codex.v2. Against the item: option (c) misstates the plan (ruling-49 README:37-38; accounts.md:42-46), and the same day the owner answered the allowance question 'just pause and let me know' (docs/proposals/ruling-71/README.md:65-69).

**Verifier's counterargument.** Strongest case against recommending (a): the owner's sentence was the answer to this very capacity question one day earlier ('I want to wait to use the second accounts as a test ... rather than using them now'), and (a) asks it again in other clothes. 'Our entire model' has a reading that needs no new answer: the scenarios S3, S7, S8 and S10 already run on the second accounts by ruling (72 point 2; 75 D2). (a) also has three holes: the dogfood has no Claude slot, so the second Claude account gets no soak from it; the slots name a directory and are unconfined, so the second accounts' logins sit inside the reach I2 lists; and ruling 75 B3 moves the dogfood engine to another macOS user mid-soak, with Claude's keychain under a user that has no login session still untested (ROADMAP.md:67, A22). And two subscriptions used to extend one person's working allowance is the case I15 says nobody has read. Better: do not ask 'what is A5' (ruling 49's record answers it); take (b) as the default without a question; put (a) to the owner only after stage 2 has passed and the terms have been read, as one line.

**Verifier's corrections.** Option (c) is not the status quo: A5 is already the sign-in test with the wrong-account attempt. The question reduces to one: after the test passes, may a second account be a dogfood slot's account. The decider is right (owner).


## FR-425

**Should the test get one arm where isolation can actually hold: a Linux user per Agent on the Windows computer?**

E04 I5 · question-for-owner · verifier: partly · decides: owner

In plain words: on one user account, one Agent's login cannot be kept from another Agent that runs code (I2). The test can only say "not shown" unless some Agents run as different users.

(a) No. The test reports it as not shown, which is v0's stated limit, and nothing more is built.
(b) Yes, at S3's time. Two more run users on the Windows computer (your sudo, about 10 minutes), one SSH-R provider Resource for each, and each Agent names its own with `runtime:`. The isolation matrix runs again there, and its same-user route is expected to turn from reached to not reached for files and process environment.
(c) Wait for the Docker provider's container Agents (A63-DR1, DR2a, DR2b: unbuilt, 1,600 to 2,250 lines by the providers' plan).

Recommendation: (b), with (a)'s statement in every result until it runs. Unix users are the existing tool, and SSH-R's own contract names them: "PRV-025's isolation is unmet until each Agent has a user of its own". The provider needs one change: its configuration refuses any user but `conduit-run`. It keeps confinement where ruling 75 B2 put it, on the runtime provider chosen per Agent, and adds nothing to the language. It separates Agents from each other; whether the provider's gate then also needs a principal apart from the runs, as its README says, is the lane's to settle when it builds it. Inspection still reports mediated (ACC-027), so no text in spec/ changes.

What it changes: ruling 77, point 2 ("One run user"), which you said you may revisit; a new register row (I10). Nothing on the S1 rerun's path.

**Evidence.** packages/provider-ssh-runtime/README.md:21-31 ("PRV-025's isolation is unmet until each Agent has a user of its own ... The kit gate also needs a separate OS principal from untrusted runs"). packages/provider-ssh-runtime/src/config.rs:117 (`|| m.user != "conduit-run"`). packages/provider-ssh-runtime/contracts.yaml:229. docs/proposals/ruling-77/README.md:18, 33-37. docs/proposals/ruling-75/README.md:42-45 (B2). docs/AUDIT.md:265-267 ("An author can opt in to a selected isolation framework through the language and plugins"). docs/implementation/PROVIDERS.md:138-140 (Docker's three orders and sizes); `ls packages` shows no provider-docker.

**Waits.** A new row A83 and S3's owner setup (S3-O2). Until answered, A5 runs with the 'not shown' list of I2.

**Verifier.** partly. Facts hold: packages/provider-ssh-runtime/README.md:21-31; packages/provider-ssh-runtime/src/config.rs:117 (m.user != "conduit-run"); packages/provider-ssh-runtime/contracts.yaml:229; docs/proposals/ruling-77/README.md:18-19, 33-36; docs/proposals/ruling-75/README.md:43-46; docs/AUDIT.md:265-267; docs/implementation/PROVIDERS.md:138-140 (650-850, 600-850, 350-550, so 1,600 to 2,250); ls packages shows no provider-docker. The premise does not: 'the test can only say not shown unless some Agents run as different users'. The accepted pre-registrations already hold an isolated arm on Docker with a credential-reach measure: S2-T7 and D20 (experiments/013-s2-shared-login/README.md:116, 146), S3's P6 and its two isolated-profile runs (experiments/014-s3-two-machines/README.md:64-66, 107, 184-190; plan.md:88-89), accepted under ruling 75 D1. PROVIDERS.md:131 and 148 already plan separate operating-system principals for the local-process provider (A63-RP2) with an owner step (A63-OUS).

**Verifier's counterargument.** Option (c) is presented as waiting; it is the design the owner accepted, and the purpose names it ('as with Docker', AUDIT.md:266-267), as does ruling 75 B2 (the first confining provider wraps an existing whole-process sandbox). The new arm would show a property of Unix (mode 0700 between users), not of Conduit, and it sits behind the longest chain in the repository: the Windows node path, a Linux build of the Agent package and a carried Codex login. 'One change' and 'about 10 minutes' understate it: each user needs an sshd Match block, a root-owned key file, lingering, a base directory and its own client install (README.md:292-306; docs/guides/second-machine.md:85-94), and SSH-R has never run against a real sshd (README.md:7-8). Better, with no ruling and no new row: add 'another Agent's home and login' to D20's locations so the isolated profile answers the question when A63-DR2b lands; state 'not shown' in A5 until then; raise users per Agent only inside S3-O2 if Docker on the second machine slips. That keeps the owner's questions at one.

**Verifier's corrections.** If asked at all, the question should list the accepted Docker profile as the default path, and say that it amends ruling 77 point 2 only under (b). Decider right (owner).


## FR-426

**Build a tracer to one real sign-in first: about two thousand lines, not the serial five to six thousand**

E04 I6 · recommendation · verifier: partly · decides: coordinator

The plan's path to a first real sign-in is serial: 046, 047, 048a, 048b and 050, with 051b joining. By its own estimates that is 4.8 to 6.3 thousand lines, and its findings say estimates run 1.5 to 2 times low. The two facts that can sink the design sit at the far end, in the real clients (I7). The roadmap's own method for S1 is a tracer; use it here.

Slice 0: the probes of I7. No code.

Slice 1, one checked sign-in on the Mac, about 1.8 to 2.5 thousand lines:
- From 048a: sign-in.v1 in place of its stub: the account check first, then the sign-in hook, IFC-049's match of the URL and the engine-written message, polling until the wait ends, a decline ending failed, a wrong account undone and failed naming both, one fixed-port login at a time; the scripted signIn fixture with a local OAuth stub; about ten of its vectors. Add the least of PND-005 that keeps it honest: no model work on a signIn instance until an account check has succeeded.
- 048b whole: input_required, requestState and the retry in the shell; URL elicitation declared only toward signIn Types.
- From 051b: `conduit sign-in` alone: show the message and the full URL, ask consent, open the browser, retry to the end.
- From 050, for Codex first: `account`, `signIn` with the literal logins that slice 0 read, the home inside CONDUIT_STATE_DIR with file storage, the three hooks on an implementation that carries Conduit's keys, the guard of I2, and a retire hook that signs out (I11).
Write its diagnostics to 046's rule from the start (the deciding promise and the cause), so 046 has nothing to redo.

Slice 2, after 046 and 047: pending actions for sign-in (raise, list, hold, expire, answer), `conduit pending`, Claude's Type, the rest of 048a's vectors, and the client-ID comparison across addresses. Stage 2 of A5 runs on it.

Slice 3, only if I9's replay passes: PND-013's return (paste first, then the listener), and the panel for sign-in evidence that a device code needs.

Costs: 048a and 050 are each cut in two, and slice 1 touches requests.rs and server.rs, which the plan lists as hot files, ahead of 046 and 047. Buys: the first real sign-in and the answers to I7 several orders earlier; and each dogfood slot gets its own home sooner, which is what lets 050a, 076 and 078 merge. The scenario plans already name an admin split (048a-1 and 048a-2, 048b-relay and 048b-join) that no admin document defines; this gives it content.

**Evidence.** docs/proposals/spec-batch-1/README.md:545-581, 622-640 (046 0.6-0.75k; 047 1-1.5k; 048a 0.8-1.1k; 048b 0.5-0.8k; 050 1.5k; 051b 0.4-0.6k), 664 (the critical path), 694 ("Estimates run 1.5 to 2 times low"). engine/crates/core/src/requests.rs:1046-1058. docs/work-orders/038c-staged/048a.md:14-15 ("No return implementation is needed for S1's local browser"). packages/agent-codex/conduit.yaml:16 and packages/agent-claude-code/conduit.yaml:16 (`plain: true`). spec/promises/pending.yaml:192-193, 235-236 (a plain hook gets no login name and no return). docs/implementation/HANDOVER.md:126-128 (050a, 076 and 078 stay unmerged until 050). experiments/013-s2-shared-login/plan.md:286-287 (048a-1/2, 048b-relay/join named; grep finds no definition outside the scenario plans).

**Waits.** A5's stage 1; the dogfood store's re-initialization; the merge of 050a, 076 and 078. Logged for the owner to strike, since it reorders batch 1 inside the roadmap's step 3.

**Verifier.** partly. Estimates recounted from docs/proposals/spec-batch-1/README.md:554, 564, 574, 581, 627, 640: 4.8k to 6.25k for 046, 047, 048a, 048b, 050 and 051b; :664 the critical path; :694 'Estimates run 1.5 to 2 times low'. docs/work-orders/038c-staged/048a.md:14-15 and both conduit.yaml:16 (plain: true) as cited; HANDOVER.md:126-128. Wrong: 'an admin split ... that no admin document defines (grep finds no definition outside the scenario plans)'. docs/work-orders/117-plan-s2-s3.md:74-93 (repeated in 118, 119 and 120) records the split 'as the admin gave it on 2026-11-11' with contents and dependencies: 048a-1 (CTR-013 in the loader, IFC-016's detection, the signIn fixture Type), with no dependency on 047; 048a-2 after 047 and 048a-1; 048b-relay after S0b only; 048b-join after 048a-2; 051b after 047 and 048b-join; 050 after 048b-join. Also wrong in detail: server.rs is not in the plan's hot-file list (:668).

**Verifier's counterargument.** The tracer's headline benefit is not the tracer's. The two facts 'that can sink the design' are answered by I7's stage 0 and I9's replay with no code at all, so 'a tracer of about two thousand reaches them first' double-counts. What the tracer alone buys is early proof of the round trip (hook, engine, Interface) and earlier homes for the two Codex slots. Its costs are understated: its own estimate is subject to the same 1.5 to 2 times (2.7k to 5k); it puts on main a sign-in.v1 that raises and answers no action, short of PND-005 and PND-010 until slice 2, or lives on a side branch and pays the rebase in requests.rs that 049a1 is paying now (HANDOVER.md:211); and it re-cuts 048a and 050 when the pipeline's cost is rounds, not lines. Better: keep the admin's split; dispatch 048a-1 and 048b-relay now, beside 046a and 047, as the split already allows; run I7 first; cut 050 by client (Codex first) so the dogfood slots get homes early. It touches the rerun's critical path, so 'logged to strike' is the least it needs.

**Verifier's corrections.** The summary's 'five to six thousand lines by its own low estimates' should read 4.4k to 5.7k for the five serial orders, 4.8k to 6.3k with 051b, low to high. Slices must name S0b (the core's seams, order 117:78-80), which precedes 048b-relay and is unwritten.


## FR-427

**A5, stage 0: probe the two facts about the real clients that decide the design, before order 050 is written**

E04 I7 · experiment-change · verifier: partly · decides: coordinator

Row: A5 (stage 0). Neither fact is known today.

(a) Can each client say which account it is signed in to, attested by the service? CTR-013 lets a Type state signIn only then, and ruling 75 C7 keeps the check strict. The repository records `claude auth status` as reporting the login in use, and `codex login status` as reporting only "the method in use"; batch 1's findings leave "Codex's account read" to a trial (O3). If Codex cannot, a Codex Agent gets no sign-in through Conduit at all.
(b) Can each client's login be started by a hook with no terminal, print its URL without opening a browser itself, and wait? Order 050 must write each login's origin, path, client ID and redirect as literal values; 038c's handoff says "none is guessed here".
A third fact is already known: the hook that is told a login's name, or handed a carried return, must carry Conduit's keys, and both wrappers are `plain: true`.

The probe: no build, the owner's hands, 10 to 15 minutes, primary accounts only, and no sign-in completed.
1. In each everyday home, run the client's status command. Record whether an account identity appears and in which field: yes or no, never the address.
2. In an empty scratch home set by CODEX_HOME or CLAUDE_CONFIG_DIR, run it again. Record that it reports no login. This is the first evidence for P1, and for Claude's keychain entry being keyed by the directory, which the repository has only as a quotation of the vendor's page.
3. Start each login in the scratch home. Record the URL's shape against IFC-049's checks (origin and path; a client ID present; the redirect's host, port and path; S256; the state's length), never its values, and whether a browser opened unasked. Cancel.

Pass: both clients give what CTR-013 and IFC-049 need. Measures: the yes or no of each step, per client and version. A failure of (b) changes order 050 before it is written. A failure of (a) for Codex goes to the owner, because C7 is the owner's.

This asks for the owner's time, not for a ruling: no second account is touched and nothing is signed in.

**Evidence.** spec/promises/contracts.yaml:303-307 ("A Type states signIn only where account-hook.v1 reports an account attested by the service ... A Type unable to learn it states no signIn and gets no sign-in or relayed URL"). docs/proposals/ruling-75/README.md:74-75. docs/proposals/sign-in/README.md:28-31, 39-45. docs/proposals/spec-batch-1/README.md:626, 677 ("Codex's account read waits on the trial (O3)"). docs/work-orders/038c-staged/return.md:53-55. experiments/009-native-launch/results-2026-11-09.md:238-254 (Claude reports `apiKeySource: "none"`; Codex's authentication `not_reported`; "No separate configuration home or separate model login was attempted"). packages/agent-codex/conduit.yaml:16.

**Waits.** Order 050's literal login values and whether its Codex Type may state signIn; slice 1 of I6.

**Verifier.** partly. Holds: spec/promises/contracts.yaml:303-307; docs/proposals/ruling-75/README.md:74-75; docs/proposals/sign-in/README.md:26-31, 39-45; docs/proposals/spec-batch-1/README.md:626, 677; docs/work-orders/038c-staged/return.md:53-55; experiments/009-native-launch/results-2026-11-09.md:238-254; packages/agent-codex/conduit.yaml:16; pending.yaml:192-193, 235-236. Overstated: 'If Codex cannot, a Codex Agent gets no sign-in through Conduit at all'. The batch plan already names the fallback and the owner's choices: docs/proposals/spec-batch-1/areas/refusals-and-identity.md:187 ('or, if that names no account, the email claim of the id_token in the home's auth.json') and :370 (O3: the owner then chooses between that read, API keys only for Codex, or an unchecked sign-in with a mark). CTR-013 admits that route by name ('returned at login, such as in an ID token', contracts.yaml:304-305). 'Neither fact is known today' is half right: docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md:54 records what both status commands printed on 2026-11-10 (a method, for Codex).

**Verifier's counterargument.** Worth doing first; it is the cheapest item here. Two changes. It need not cost the owner 10 to 15 minutes: the coordinator ran both status commands for the closeout, and a small Rust probe under env -i with scratch homes, recording yes or no and URL shapes only, starts and cancels a login without signing anything in; the owner is needed only if a browser opens unasked on the owner's screen, which is itself the finding. And the larger unknown is Claude's, not Codex's: when a hook with no terminal keeps the client from opening a browser, does `claude auth login` still print a loopback-redirect URL and listen, or fall back to the page that shows a code? The second is a page return, which carried: false cannot complete (implementation.yaml:1370). Step 3 records it; the pass condition should name it.

**Verifier's corrections.** State (a)'s fallback as O3 has it, and that the id_token read can only be tried by the package's hook in a signed-in scratch home, never by the coordinator. A failure of both routes for Codex is the owner's, as the item says.


## FR-428

**A5: the nine robustness cases, each with its registered outcome**

E04 I8 · experiment-change · verifier: partly · decides: coordinator

Row: A5 (stage 2, step g). Scripted means the fixture Type and a local OAuth stub, in vectors or shell tests, with no owner time. Real means the owner is present.

1. Device code (real, 3 minutes; Codex, only where the account has it on). Expected: the message says the code is in the engine's panel, and the code is in no result or request record. It needs the panel for sign-in evidence (051c), which is outside the named chain; without it the case is scripted only.
2. Pasted URL (real, 3 minutes; needs PND-013). The owner runs `conduit` on the Mac over SSH from the Windows desktop, opens the URL in that desktop's browser and pastes the final URL with echo off: ruling 75 C4's own case. Expected: `returned: delivered`, then the account check; zero hits for the value anywhere.
3. A pending sign-in expires and is asked again (scripted with the clock; real once if the two waits can be set short at `conduit serve`). Expected: the answer reads expired with no actor, nothing is dispatched, no new action arises until the next held turn or setup, and the new action has a new ID.
4. Declined (real, 1 minute). At `conduit`'s consent prompt: ends failed, no browser opens, the action stays pending. Abandoned at the vendor's page: ends unknown when the wait ends, with a diagnostic that says no account was seen.
5. Wrong account (real, 3 minutes): P4, then P7.
6. The Interface closed during the flow (real, 3 minutes): stop `conduit` after the URL opens. Expected: the invocation ends unknown (cancelled under OPS-027 if a retry was open, otherwise at the wait's end); turns stay unavailable; the next `conduit sign-in` either succeeds from the account check alone or relays a fresh URL; it never reports signed-in without a check.
7. Two at once (real, 3 minutes). codex-primary and codex-second: the second request reports unavailable, naming PND-010 and the port. claude-second and codex-second together: both proceed, are finished in the other order, and each ends on its own account. One instance twice: scripted, to record what happens (I13).
8. The engine restarted during the flow (real, 3 minutes): stop `conduit serve` after the URL shows. Expected: `conduit` says the engine went away; after the restart the action is still pending, the old requestState is rejected, nothing listens on the login's port, and a new sign-in completes. The port is the point: in the smoke test Codex's terminal children outlived the run's process-group stop.
9. Network loss (real, 3 minutes, the owner's hand on the Mac's network): before the page loads; and after approval, before the client redeems the code. Expected: no account seen, unknown at the wait's end with its cause, nothing half signed in, a second try succeeds. In the soak: an outage during a turn raises no sign-in action. Across machines, during a carried return: `returned: undelivered` with its cause; that one runs at S3, by S3-OF's means.

Pass, for every case: P8, and P5's counts stay zero after it. Measure: outcome, reason, ruleId, whether the diagnostic names the pending action, and the owner's seconds.

**Evidence.** spec/interface/implementation.yaml:1369-1370 ("for device it says the code is in the engine's panel"), 1379-1386. spec/promises/pending.yaml:153-157 (expiry; "never by expiry alone"), 172-176 (the fixed port), 181-190, 268-270 ("restart loses it"), 287-292. spec/conventions/operations.yaml:600-620 (OPS-027: a dropped stream cancels; unknown). docs/proposals/ruling-75/README.md:60-62 (C4). docs/work-orders/038c-spec-sign-in-return.md:264 (a second login on the same fixed port cancels the first, as Codex's does on 1455). docs/implementation/HANDOVER.md:142-145 (a short network loss failed a turn as unknown), 149-150 (Codex's children escaped the group stop). docs/proposals/spec-batch-1/README.md:642-643 (the panel is 051c).

**Waits.** Cases 3 to 9 need slice 2 of I6; cases 1 and 2 need slice 3. The 048a and 047 orders should make the two waits (DL18, DL19) settable at `conduit serve`, or case 3 stays scripted.

**Verifier.** partly. Expected outcomes checked against the promises: device code (implementation.yaml:1369-1370; PND-004, pending.yaml:77-79; panel is 051c, spec-batch-1 README:642-643); expiry (pending.yaml:59-64, 155-156); decline and wait's end (pending.yaml:187-188; DL18, implementation-defined.yaml:244-251); wrong account (pending.yaml:209-212); fixed port (pending.yaml:172-176; docs/work-orders/038c-spec-sign-in-return.md:264); restart (pending.yaml:270); OPS-027 (spec/conventions/operations.yaml:600-618); listener rule (pending.yaml:287-291); HANDOVER.md:143-146, 149-150. Those match. Not all nine can end 'in its registered outcome, with a diagnostic naming its rule'.

**Verifier's counterargument.** Case 9's second window, after approval and before the client redeems the code, is under a second wide; no hand on the network hits it, so it is a stub case, not a real one. Case 8's 'nothing listens on the login's port' rests on no promise: what becomes of a hook's processes at engine stop is DL16's, implementation-defined (implementation-defined.yaml:182-183), so P8 cannot name a rule for it; record it as an observation and a first-build finding. Cases 4 (abandoned), 6 and 9 end only at DL18, so the minutes hold only if the wait is settable, which before the rerun must not be a flag an author could use (HANDOVER.md:93-95). Case 6 misses its dangerous variant: with the Interface closed nothing polls, so a wrong account completed in the browser is not undone and stays in the home until the next setup (pending.yaml:182-184, 217-223); that is the case to run. Case 2 exposes an ambiguity to settle in the 051b order: PND-013 and return.md:63-65 say when the CLI opens a listener and that it pastes 'where uncertain', but not that it offers paste when it runs on the node's own machine, which is exactly ruling 75 C4's case. Keep on real accounts only what depends on the real client (5, 7, 8 and the device code); run the rest on the fixture.

**Verifier's corrections.** Mark case 9b and the 'one instance twice' case scripted; mark case 8's port check an observation; add the closed-Interface wrong-account variant; cases 1 and 2 are not in stage 2 (they wait for slice 3), so stage 2's minutes cover seven cases.


## FR-429

**A81: add a replay by hand before any return code is written, then the two routes through Conduit**

E04 I9 · experiment-change · verifier: partly · decides: coordinator

Row: A81 (ROADMAP.md:73). Its question stays: does a Codex browser login complete when its redirect is carried back from another machine, and is the account check then right?

Add stage 0, with no engine. Ruling 75 C5 rests on the coordinator's finding that Codex's callback checks only the path and the login's state, and asks for a probe. S3's plan builds the return first (S3-RET1 and RET2, 1,150 to 1,650 lines) and observes the Interface route after. Its earlier step, an SSH forward by hand, passes the browser's own request through and so does not test a carried one. A replay does, in about 10 owner minutes: as `conduit-run` on the Windows computer, in a scratch home, the owner starts Codex's login; opens the URL in the Mac's browser; a one-shot listener on the Mac takes the redirect; the owner sends a rebuilt request to Codex's port on the Windows computer, holding only code, state, iss and scope, as PND-013 says the engine rebuilds it, read with echo off. Then the status command, a sign-out, and the scratch home deleted. Account: primary ChatGPT, since the second accounts are kept for Conduit.
Pass: the login completes from a request the browser did not send. Measures: completed or not, the Codex version, the owner's minutes. If it fails, the redirect half of PND-013 does nothing for Codex, a device code or the person's own SSH forward remain, and RET1 and RET2 are not written for it.

Stages 1 and 2 are the plan's S3-A81a and A81b, through Conduit and on the second ChatGPT account: the SSH forward by hand, then the person's Interface. Add to each the fresh-home check P1: the Agent's first account check must report no account although `conduit-run`'s own home holds the primary login. If it reports the primary, the home is not separate.

Also record how the owner signed Codex in as `conduit-run` on 2026-11-12 (device code, the Windows desktop's browser, or a forward). It is the only Codex login made on that machine so far and it is written down nowhere.

Stage 0 completes one login of the primary account outside Conduit, in a home that is then removed. That is a request for the owner's hands and time; it becomes a question only if the owner reads "only through Conduit" as covering the primary accounts too.

**Evidence.** ROADMAP.md:73. docs/proposals/ruling-75/README.md:16-20 ("its callback waits on a fixed port and checks only the path and the login's state"), 63-65. experiments/014-s3-two-machines/plan.md:76-79 (RET1 550-800 lines, RET2 600-850; A81a before RET1, A81b after RET2), 141-146, 227. spec/promises/pending.yaml:249-252 ("The engine rebuilds a redirect value as the relayed redirect_uri with a query containing only code, state, iss and scope"). docs/work-orders/038c-staged/return.md:57-59 ("It remains evidence to obtain"). docs/guides/second-machine.md:108-110. IN_FORCE.md:195. docs/proposals/ruling-72/README.md:64-74 (every login tested through the Interface).

**Waits.** Whether S3-RET1 and RET2 are written; slice 3 of I6. Stages 1 and 2 wait on the Windows node path (049a1, SSH-M, 079, 080, the engine's certificate, a Linux build of the Agent package).

**Verifier.** partly. Holds: ROADMAP.md:73; docs/proposals/ruling-75/README.md:16-20, 63-65; experiments/014-s3-two-machines/plan.md:76-79 (RET1 550-800, RET2 600-850, so 1,150 to 1,650; RET1 'after manual-forward observation'), :141-146, :227; pending.yaml:250-252; return.md:57-59; second-machine.md:108-110; IN_FORCE.md:195. The reasoning is sound: an SSH forward passes the browser's own request, with every parameter and header, while PND-013 has the hook send a rebuilt one, so the plan's gate before RET1 tests the weaker thing. The method of the owner's Codex sign-in as conduit-run is recorded nowhere (grep over docs/ and packages/provider-ssh-runtime finds only that it was done: HANDOVER.md:179-180).

**Verifier's counterargument.** The stage itself is right and should be adopted. One clause is not the coordinator's: 'If it fails ... RET1 and RET2 are not written for it'. Ruling 73 point 1 has the Interface carry the redirect back in v0, 75 C5 has Codex's browser login everywhere, and ruling 73's record says a finding that would change these goes back to the owner (docs/proposals/ruling-73/README.md:58-61). PND-013 is also still needed for the pasted URL (75 C4) and for page returns. A simpler replay gives the same answer with no login on the Windows computer: on the Mac, a scratch home; open the URL in a browser on another device, where the redirect to localhost fails; copy the final URL (ruling 75 C4's paste, by hand) and send the rebuilt request on the Mac.

**Verifier's corrections.** On failure: the coordinator records the result and puts one question to the owner; it does not drop the return orders. Decider for the stage: coordinator.


## FR-430

**The other register rows: A82 reworded, A7 fed, S2's table replaced, and A83 if the owner chooses the user-per-Agent arm**

E04 I10 · experiment-change · verifier: partly · decides: coordinator

A82 (ROADMAP.md:74), Linear's login for a Resource. Two changes.
- Its question says "with the return carried to the Interface that received the sign-in". With `conduit`, the engine and the provider package all on the Mac, nothing is carried: PND-013 has the Interface open a listener only when it is not on the node's machine, and there the package's own listener takes the redirect. Reword to "its redirect taken by the package's own listener on the engine's machine"; a carried return is A5's case 2 and A81.
- Add to Needs: during the run, no unconfined Agent and no plain work-order worker runs under the package's operating-system user (ruling 75 C1), or the result states that placement as unmet. Today the dogfood slots on the Mac and the workers under `conduit-run` are exactly such processes.
Pass and measures stay as S2's plan gives them: one login, the account checked by one read, no code or token recorded, about 45 owner minutes with the application's registration.

A7 (ROADMAP.md:64), the bypass probe. Add to what it receives: the same-user rows of A5's isolation matrix (read, use, replace, act as root). Ruling 47 has every route A7 finds written into v0's public security claim.

S2's plan, the table "Logins through the Interface". Replace its first-use column with A5: all four subscription logins are first signed in under A5's stages 1 and 2, and S2-R1, S7-O, S3-O2 and S10-O sign their own homes in on a flow already tested. The table also misses that, by the handover, the dogfood slots are the first to sign in after 050.

A83, new, only if the owner answers I5 with (b). Question: with one Linux user per Agent on the Windows computer, each behind its own SSH-R provider Resource, can a process of one Agent read, use or replace another Agent's login? Needs: the owner's sudo for two users; SSH-R's user check relaxed; the Windows node path. Feeds: the isolation claim of v0's security statement; PRV-025's "unmet" in SSH-R's contract. Pass: every same-user row of A5's matrix that read "reached" reads "not reached" for files and process environment, with the canary method. Measures: the matrix; the lines of provider change; the owner's minutes. Done when: a dated matrix beside A5's.

**Evidence.** ROADMAP.md:64, 73-74. spec/promises/pending.yaml:287-292 ("It opens a listener only where it knows it does not run on the machine of the node signInLogin names: on that machine the program's own listener takes the redirect"). spec/README.md:424-427 ("a package keeping a Resource's login runs where no unconfined Agent runs; the engine does not check that placement in v0"). docs/proposals/ruling-75/README.md:52-54. experiments/013-s2-shared-login/plan.md:228-234, 236-252. docs/proposals/ruling-47/README.md:47-48. docs/implementation/HANDOVER.md:120-123, 157-158; docs/proposals/ruling-77/README.md:33-37 (the workers run as `conduit-run`). ROADMAP.md:89-99 (a new item takes the next A-number when accepted; the highest today is A82).

**Waits.** A83 waits on I5. The other three are edits the coordinator may make with A5's row.

**Verifier.** partly. A82's row as quoted (ROADMAP.md:74); PND-013's listener rule (pending.yaml:288-291); the placement limit (spec/README.md:426-427; ruling-75 README:52-54); A7's row and ruling 47 (ROADMAP.md:64; ruling-47 README:47-48); the next number is A83 (grep finds no A83 or higher; ROADMAP.md:98). The A82 placement point is correct and new: on the Mac today the dogfood slots and the coordinator's own session run under the package's operating-system user.

**Verifier's counterargument.** A82's rewording throws away the only test of a carried return for a Resource, which is what the owner asked for in ruling 73 ('Many resources may require browser login') and what 75 C2 rules on. Keep the question and add one leg of three minutes: `conduit` on the Mac over SSH, the browser on the other desktop, the final URL pasted; then the listener leg stands beside it. The S2 table's replacement depends on I1 and I4, and it too omits the S1 rerun, whose Agents sign in before S2-R1 (ruling-68 README:41-43). A83 depends on I5 and overlaps the isolated arm S2 and S3 already hold (D20). The placement note should say when it stops being unmet: ruling 75 B3's dedicated macOS user does not help, since the package and the slots would share that user too; only the confining provider does.

**Verifier's corrections.** A82: keep 'carried to the Interface that received the sign-in' for the paste leg, and add the listener-on-the-engine's-machine leg and the placement line. The A7 edit stands as written. Drop A83 unless I5 is asked and answered (b).


## FR-431

**No promise or operation revokes one Agent's sign-in, and a home is removed with its Agent only on the engine's own runner**

E04 I11 · fact · verifier: supported · decides: coordinator

Ruling 49 says each Agent's home is "made and removed with it". Three things fall short.
- No operation signs an Agent out. sign-out-hook.v1 runs only "as a part of sign-in or setup". The ways to end a login are to retire the Agent; to change its `account` and set it up again, when the old account is undone; or to revoke the session at the vendor, after which the client reports a lapsed login and a sign-in action is raised.
- The batch plan gives order 050 "the sign-out at retire". No promise states it. It can only be the package's own retire hook.
- Removal with the Agent is promised for the engine's own runner alone (PRV-029, LIF-035). SSH-R's contract keeps the remote directory "across starts and never removed", and the runtime contract has no call that tells a provider an instance is gone (run.v1, stop.v1 and inspect-run.v1 only). On macOS a Claude login is a keychain entry, outside the directory. So without a sign-out in the retire hook, a retired Agent leaves a live login behind on the Windows computer and in the keychain.

Least change, with no new operation and no new provider call: order 050's retire hook signs the client out and empties its home, as `docker logout` and `gh auth logout` remove one stored credential, and the Type's contract says so. A5's step (h) checks it: the status command in a directory made again at the old path reports no login, and the remote directory lists no login file. A person's sign-out operation would add a tool to root's defaults and would be the owner's to rule; the test gives no reason to ask for one yet.

**Evidence.** docs/proposals/ruling-49/README.md:32. spec/core/README.md:147 ("The engine, as a part of sign-in or setup"). spec/promises/pending.yaml:219 ("Hooks run only as parts of sign-in.v1 or setup"), 143, 220-223. spec/promises/templates.yaml TPL-012 (setup undoes a different account before the setup hook). docs/proposals/spec-batch-1/README.md:624. spec/promises/providers.yaml:975, 979-980 ("Another runtime provider supplies such a directory only where its contract states it does"). packages/provider-ssh-runtime/README.md:48-49 and contracts.yaml:43. spec/core/README.md:148-150 (the runtime contract's three operations). docs/proposals/sign-in/README.md:28-30.

**Waits.** Order 050's retire hook and Type contract; P7 and step (h) of A5.

**Verifier.** supported. docs/proposals/ruling-49/README.md:32; spec/core/README.md:145-150 (the three hooks 'as a part of sign-in or setup'; run.v1, stop.v1 and inspect-run.v1 the runtime contract's only operations); pending.yaml:143, 219-223; spec/promises/templates.yaml:214-216; docs/proposals/spec-batch-1/README.md:624 ('the three hooks plus the sign-out at retire'); providers.yaml:975, 979-980; packages/provider-ssh-runtime/README.md:48-49 and contracts.yaml:43 ('kept across starts and never removed'); spec/conventions/lifecycle.yaml:733-734. All as stated.

**Verifier's counterargument.** It is stronger than the item says: IN_FORCE.md:191 keeps 'made and removed with it' as a decision in force, so SSH-R's contract departs from a ruling, not only from a record, and the lane should hear it. The retire-hook fix is the existing paradigm and already in order 050's plan. Its limit should be written beside it: a retire hook cannot run on a machine that is lost or cut off (S3's own case), so there the login stays until it is revoked at the vendor; the accounts guide should say so. The test has no forced lapse: step (h) retires, but nothing signs one Agent out behind the engine's back (the client's own logout in that home) to show PND-005's third trigger and the bystanders, which is the claim 'revoking one leaves the other working'.

**Verifier's corrections.** Add a forced-lapse step to A5's stage 2. A person's sign-out operation stays unasked, as the item says.


## FR-432

**The chain the brief names is not the whole chain: 046 and 047 come first, and three of the nine cases need work outside it**

E04 I12 · fact · verifier: supported · decides: none

The brief and the handover name the sign-in chain as 050, 048a, 048b and 051b. By the batch plan 048a comes after 047, and 047 after 046; neither is written. Beyond those six:
- A device code needs the engine's own panel, because IFC-049 puts the code only there. The panel is 051c, "After 049a; off the rerun path".
- A pasted URL and a carried redirect need PND-013's return, which 038c handed to an order "after 048b, before S3": S3-RET1 and RET2 in S3's plan.
- An expiry, a held turn and a sign-in asked again need 047's pending-action core.
- An Agent on the Windows computer through Conduit needs the node path as well: 049a1 (checkpointed, one must left), SSH-M (not built), 079 and 080 (on the side branch `s3/m2`, 080 with three musts), the engine's certificate, and a Linux build of the Agent package, since its archive is "a host archive, not a portable executable for another OS/architecture".
So "A81 and A82 after the chain" understates the distance to the cross-machine leg, and the Mac legs are much nearer. That is the reason for I6's slices and for A81's stage 0.

**Evidence.** docs/guides/full-review.md:45-46; docs/implementation/HANDOVER.md:219 (queue row 10). docs/proposals/spec-batch-1/README.md:556, 566, 576, 622, 635, 642-643. spec/interface/implementation.yaml:1369-1370. docs/work-orders/038c-staged/README.md:9; return.md:3. experiments/014-s3-two-machines/plan.md:76-77, 187. docs/implementation/HANDOVER.md:212-213 (049a1 and 080 checkpointed), 176-177 (SSH-M not built). packages/agent-codex/README.md:33-34.

**Verifier.** supported. docs/proposals/spec-batch-1/README.md:556, 566, 576, 622, 635, 642-643; implementation.yaml:1369-1370; docs/work-orders/038c-staged/README.md:9 and return.md:3; experiments/014-s3-two-machines/plan.md:76-77; HANDOVER.md:174-177, 211-212, 218; packages/agent-codex/README.md:32-33. Each dependency is as stated.

**Verifier's counterargument.** The distance to the Windows leg is longer still: enrolling the machine through Conduit also needs 049b and 051c's machine commands (HANDOVER.md:219), and SSH-R has only fake-remote evidence (packages/provider-ssh-runtime/README.md:7-8). The item reads the unsplit plan; the admin's later split (docs/work-orders/117-plan-s2-s3.md:74-93) already takes 048a-1 and 048b-relay off the serial path, and adds S0b before the relay.

**Verifier's corrections.** Line drift: queue row 10 is HANDOVER.md:218; 049a1 and 080 are :211-212; SSH-M 'not built' is :174; the archive sentence is README.md:32-33.


## FR-433

**A gap: nothing decides two sign-ins of one instance at once when its logins state no fixed port**

E04 I13 · fact · verifier: supported · decides: coordinator

PND-010 refuses a second sign-in.v1 only where an in-progress one "may start a login stating a fixed port that a login this request may start also states". PND-001 allows one unanswered sign-in action per instance, which is about actions, not invocations. So for a Type whose logins use any port, as Claude Code's is expected to, two sign-in.v1 invocations for one Agent can run together: two login programs in one home. If one completes as the wrong account and is undone while the other completes as the right one, the undo can remove the right login. It fails closed (the account check then sees none and the wait ends unknown), so it is not unsafe, but two builders could differ and the person gets no explanation.

Least change: the existing refusal covers any in-progress sign-in.v1 for the same instance as well: unavailable, naming PND-010 and the other invocation's action where the caller may see it, in OPS-023's order. Setup, retirement and change of one instance already exclude each other in the same way. It adds a refusal and cannot move S1's counts, so it is a small finding for the coordinator (ruling 71, point 4), with one vector; A5's case 7 records what the engine does meanwhile.

Confidence is medium: I read PND-001, PND-010, PND-011 and sign-in.v1's description and searched the conventions for an in-progress rule, but did not read every operations promise.

**Evidence.** spec/promises/pending.yaml:17-19 ("An instance has at most one unanswered sign-in and one unanswered setup action"), 172-176. spec/core/operations.yaml sign-in.v1 description ("Before any part, a fixed port shared with a login an in-progress sign-in may start reports unavailable"). grep -rn -i 'in-progress\|in progress reports conflict' over spec/promises, spec/conventions and spec/interface: only pending.yaml:173 and operations.yaml:360. spec/conventions/lifecycle.yaml:443 ("A setup, retirement or change still in progress").

**Waits.** A sentence in PND-010 and one vector, with order 048a.

**Verifier.** supported. pending.yaml:18-19 (one unanswered sign-in action per instance), :172-176 (the refusal covers only a shared fixed port); spec/core/operations.yaml:1378-1380 (the same in sign-in.v1's description); spec/conventions/revisions.yaml:182-190 (REV-012 serializes only setups, retirements and changes of one instance); grep for 'in-progress' over spec/promises, spec/conventions and spec/interface finds only pending.yaml:173 and conventions/operations.yaml:360 (about recovery, not sign-in). No rule decides two sign-in.v1 invocations of one instance whose logins state no fixed port.

**Verifier's counterargument.** Narrower than it reads. Where the Type's login states a fixed port, as Codex's does, a second sign-in of the same instance is already refused, because both would start the same login. The gap is left for any-port, device and service logins, and it fails closed. The fix is small and the coordinator's under ruling 71 point 4. A reading that adds no refusal is also open: treat a second request for an instance with a sign-in in progress as REV-012 treats a second setup. Either way one sentence and one vector with 048a-2.

**Verifier's corrections.** The sign-in.v1 description is at spec/core/operations.yaml:1378-1380; 'operations.yaml:360' is the conventions file and says nothing of sign-in.


## FR-434

**A defect: SEC-020 is cited for 'the engine holds no token' and does not say it**

E04 I14 · fact · verifier: partly · decides: coordinator

Three texts cite SEC-020 for the engine holding no login token: CTR-013 ("The engine starts no login and holds no token it yields (SEC-020)"), ruling 75 C1 ("The engine holds no token (SEC-020)") and S2's plan. SEC-020's text is about something else: the engine does not serve `get-secret.v1` itself and passes no value of its own environment to a run. The statement that the engine keeps no credential a login yields exists only as a sentence in CTR-013's observable, a language-layer promise about what a Type's contract may state, and in PRV-029's "reads none of its contents, records none".

Least change, adding no behavior: CTR-013 cites PRV-029 in place of SEC-020, or SEC-020's observable gains one sentence saying the engine keeps no credential that a login it relays yields. The second is better: the safety catalogue and A5's scans (P5) then have one promise to name. A coordinator's fix under rulings 37 and 46, point 4.

**Evidence.** spec/promises/secrets.yaml:483-509 (SEC-020 whole: "The engine serves `get-secret.v1` itself, for any secret Resource, or passes to the run ... a value taken from a variable of the engine's own environment"). spec/promises/contracts.yaml:310-311. docs/proposals/ruling-75/README.md:53. experiments/013-s2-shared-login/plan.md:241. spec/promises/providers.yaml:977.

**Waits.** One sentence in spec/; P5 of A5 then names SEC-020.

**Verifier.** partly. spec/promises/secrets.yaml:483-507 (SEC-020 whole) says the engine does not serve get-secret.v1 and passes no value of its own environment to a run; its observable adds 'no store is built into the engine'. The three citations exist: spec/promises/contracts.yaml:310-311; docs/proposals/ruling-75/README.md:53; experiments/013-s2-shared-login/plan.md:241 (and :89). PRV-029's sentence is at providers.yaml:977. So the fact holds: no sentence of SEC-020 speaks of a login's token.

**Verifier's counterargument.** It is a loose cross-reference, not a behavior defect: the citations lean on 'no store is built into the engine', and CTR-013's own sentence states the rule. The preferred fix is the worse one. A sentence about login tokens in SEC-020 puts a second obligation in a promise whose `when` is a secret read or a run request, against the review's own test of one obligation per promise (docs/guides/full-review.md:116). Cite PRV-029 and the limits section (spec/README.md:418-419) in CTR-013 instead. The finding worth logging is the one mentioned in passing: an obligation on the engine ('starts no login and holds no token') sits in the observable of a language-layer promise about a contract's form, with no convention-layer promise of its own for the safety catalogue to hold.

**Verifier's corrections.** Class it as a wrong cross-reference, coordinator's under rulings 37 and 46 point 4; choose the first fix.


## FR-435

**Read the vendors' terms again for the case this test creates, before the soak**

E04 I15 · recommendation · verifier: supported · decides: coordinator

Experiment 010 read the terms for one owner with one login per client. It did not read the case A5 creates: one person holding two subscriptions with each vendor, and several Agents working on them for days (the soak of I4, option a). It quotes Anthropic's page that advertised limits "assume ordinary, individual usage", notes that many Agents at once on one plan "strains" that, and says OpenAI's terms of use "could not be fetched". Its own verdict asks for a second reading before v0 is published.

Before stage 3, the coordinator reads both vendors' current terms for this case and records the reading, dated, beside experiment 010's results; it goes to the owner with I4. Nothing in stages 0 to 2 waits on it: each is the owner signing in to an unmodified client through the vendor's own page, which the first reading covers. Cost: about an hour's desk check. It is not legal advice, as experiment 010 says of itself.

**Evidence.** experiments/010-subscription-terms/results-2026-11-10.md:18-19, 25-26, 31-34 (OpenAI's terms returned HTTP 403), 48 ("Many Agents at once on one plan strains 'ordinary, individual usage', a limit on volume, not a prohibition"), and its verdict ("To be read again before v0 is published, with OpenAI's terms of use read whole"). experiments/010-subscription-terms/README.md:9-12.

**Waits.** Stage 3 of A5 under I4's option (a).

**Verifier.** supported. experiments/010-subscription-terms/results-2026-11-10.md:14-19, 25-26, 31-34 (OpenAI's terms of use 'could not be fetched (HTTP 403)'), :48 ('strains "ordinary, individual usage", a limit on volume, not a prohibition'), :65-68 (the verdict: 'To be read again before v0 is published, with OpenAI's terms of use read whole'); experiments/010-subscription-terms/README.md:9-12. The first reading covers one owner with one login per client and does not mention two subscriptions held by one person.

**Verifier's counterargument.** It should come before I4 is put, not with it: (a) is the option that turns a test into added allowance, and whether a second subscription may extend one person's allowance is the precise sentence to look for in both vendors' terms. If the reading is unclear, (b) needs none of it. An hour's desk check is the right cost.


## What the verifier says the analyst missed

- **A5 is already this test by ruling 49's record; the brief's question has an answer** docs/proposals/ruling-49/README.md:37-38 ('A5 runs on it with the owner's second accounts, including a sign-in to the wrong account, which must be refused'), docs/guides/accounts.md:42-46 and spec/README.md:415-416 already define A5 as the second-accounts sign-in test. The analyst cited ruling 49 only for the homes (line 32) and built an owner question whose option (c) is not the status quo. What is stale is ROADMAP.md:62's question and done-when, a coordinator's edit. The owner's part is only the order and the soak.
- **The S1 rerun is the first measured consumer of real sign-in** Ruling 68 R1 (docs/proposals/ruling-68/README.md:41-43) and docs/proposals/spec-batch-1/README.md:122-123 put one sign-in per Agent and an `account:` line in the rerun's T1; 048a, 048b and 050 are on its critical path (spec-batch-1 README:664; HANDOVER.md:217). The analyst names six scenarios and the dogfood store and never the rerun, so the staging does not say whether A5 precedes it, and so whether it contradicts 'A5 does not gate the S1 rerun' (spec/README.md:415). A sign-in fault found in the rerun costs an owner-attended run and delays the bar; this is the strongest argument for stage 0 and stage 1, and it is absent.
- **Ruling 75 B3: the dedicated macOS user** docs/proposals/ruling-75/README.md:47-48 and IN_FORCE.md:314: before a coordinator Agent drives workers, the engine that runs Conduit's own work runs as a dedicated macOS user. It appears nowhere in the analysis. It changes the hazard of I2 and what 'one operating-system user' means for the owner's own logins; it interrupts a soak run as dogfood (I4 a); and it raises a probe the test should carry: does a Claude login, a keychain entry, work for a user with no login session? ROADMAP.md:67 (A22) says the daemon case is untested. If it does not, B3 is not feasible for Claude Agents as ruled, which the owner needs to hear before batch 2.
- **The accepted pre-registrations already hold the isolation measure and an isolated arm** S2-T7 and D20 (experiments/013-s2-shared-login/README.md:116, 146) and S3's P5, P6, M2, V6 and isolated-profile runs (experiments/014-s3-two-machines/README.md:60-66, 156, 172, 184-190), accepted under ruling 75 D1, measure which credentials each Agent process can read, locally and under Docker, and trace the second-account sign-in with the wrong-account undo. The analyst's matrix, I5's question and A83 were designed without them. The least change is one added location in D20 (another Agent's home and login) and A5 reusing D20's counting rule.
- **The admin's split of batch 1 exists in the tree** docs/work-orders/117-plan-s2-s3.md:74-93 (and 118 to 120) define 046a and 046b, S0b and S0c, 048a-1 and 048a-2, 048b-relay and 048b-join, with dependencies. I6 says no admin document defines them. The build-order answer should start from this split, which already lets 048a-1 and 048b-relay run beside 046a and 047. Separately, the handover and the batch plan still use the unsplit names (HANDOVER.md:217-218), a staleness for the coordinator to fix.
- **Scoped Interfaces completing sign-ins (ruling 66 point 3) is neither tested nor listed as not shown** The assignment named rulings 62 and 66. PND-010 (pending.yaml:165-167) lets an Interface other than root sign in only an Agent of a Fleet in its scope, and PND-002 raises the action there. Real clients cannot show it until Interfaces other than root have tokens (spec-batch-1 README:687: 'proven by vectors only'). The test should say so in its 'not shown' list and name the vector (048a's scoped-own-fleet-only).
- **Claude's headless login shape is the larger client unknown** The summary names two unknowns, both Codex's. For Claude, IFC-049 needs a loopback-redirect URL that the Interface opens after consent, and carried: false forbids a page code (implementation.yaml:1370; ruling 75 C3). Whether `claude auth login`, started by a hook with no terminal and kept from opening a browser itself, still listens and prints such a URL is unrecorded (docs/proposals/sign-in/README.md:26-27 describes the paste fallback). If it falls back to the code page, a Claude Agent cannot be signed in through the Interface as specified. I7's step 3 would see it; no pass condition or consequence names it.
- **No forced lapse, and one bystander missing** The claim 'revoking one Agent's sign-in leaves the other working' is tested only by retirement. A step that logs one Agent's client out in its own home, behind the engine, would show PND-005's lapse trigger (pending.yaml:143), the held turn and the bystanders in minutes, without waiting for a natural lapse in the soak. The bystander check omits conduit-run's primary ChatGPT login on the Windows computer (HANDOVER.md:179-180), which the workers depend on and which shares an account with the wrong-account case.

## The verifier's notes

Checked at 2eb8f258 (2026-11-12), read-only. Overall: the analyst's quotations and counts are accurate almost everywhere (I recounted 494 cases, 519 promises, the zero-case IDs, the stub, and every promise line cited), and three items stand as written or nearly (I2, I11, I12, I13, I15). The weaknesses are of one kind: the analysis read the specification and the rulings closely and the neighbouring plans thinly, so it proposes things the tree already holds (A5 as the sign-in test, by ruling 49's record; the credential-reach measure D20 and the Docker isolated arm in S2 and S3; the admin's split of 048a and 048b in orders 117 to 120; O3's fallback for Codex's account) and misses two rulings' effects (68 R1: the S1 rerun signs real Agents in; 75 B3: the dedicated macOS user). One pass condition (P5) would fail on a conforming engine, and one statement of fact is wrong ('none of it is built or tested': PRV-029's directory is, by order 045). Net advice to the coordinator: adopt stage 0 of I7 and the replay of I9 now, at little or no owner time; rewrite A5's row as ruling 49 already has it, with stage 1 as the rerun's rehearsal; reuse D20 instead of a new matrix; start from the existing split instead of a new tracer cut; put at most one question to the owner (may a second account be a dogfood slot's account, after the test passes and the terms are read), and none on A5's identity or on users per Agent.

Coverage. Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md 'What holds'; the records of rulings 49, 62, 66, 72, 73, 75 and 77; spec/promises/pending.yaml; CTR-013 and CTR-014; IFC-049; PRV-029; SEC-020; DL16, DL18, DL19; docs/work-orders/038c-staged/README.md, 048a.md and return.md; docs/guides/accounts.md; the register rows A4 to A8, A22, A78, A81, A82. Read in the cited parts: spec/README.md:400-436; PRV-025; OPS-027; LIF-035; REV-012; docs/proposals/spec-batch-1/README.md:118-155 and 540-700; both Agent package READMEs, manifests and lib.rs:15-35; packages/provider-ssh-runtime README (1-110, 292-320), config.rs:105-125 and contracts.yaml; S2's plan (218-290) and README (sign-in and D20 lines); S3's plan and README (sign-in, isolation, A81, RET lines); docs/implementation/PROVIDERS.md (Docker, RP2, OUS lines); order 117:56-96; experiments 005, 009 and 010 at the cited lines; rulings 47, 68 and 71 at the cited lines; IN_FORCE.md topic 9 and lines 264, 314; engine/CONFORMANCE.md:38-39, 126-131; requests.rs:1035-1065. Not read: vector-impact.md; the 038c order beyond :258-268; the plans of S4, S7, S8 and S10 in full; rulings 70 and 74; the safety catalogue; any code beyond the lines named. Nothing was built or run; no vendor behaviour was checked outside the repository, so statements about what the clients do rest on the repository's own records.
