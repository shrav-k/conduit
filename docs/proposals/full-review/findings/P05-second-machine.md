# P05-second-machine: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** The Windows computer under WSL2 is a fair second machine for part of what S3 claims and not for the part that matters most. It gives a real second kernel and operating system, node-tied values, a real SSH path and a real total cut, and ruling 77's reason for an SSH probe is sound for this machine; SSH-R's design (no listener on the second machine, OpenSSH and systemd doing the work, PRV-025 stated unmet) is honest and follows existing tools. It cannot show a lost machine faithfully: stopping the distro kills every run, Windows keeps answering, and the disk and host key survive, so a lost machine that returns with live runs, the case the owed stop and MC4-002 are about, never occurs. It cannot show identity between runs (one user), and enrollment there is a ceremony on a machine already joined by hand. The lockdown also left every fault and every privileged step with the owner: S3-OF's option A counts 18 actions a run correctly but omits two restarts (20 a run, 80 in all), puts a person inside the timing cells and makes the runs unrepeatable; I recommend cutting on the Mac's side of the path, which needs no privilege and no owner, and C if the choice stays among the three offered. The SSH provider moves away from ruling 50's aim: the machine-provider contract is written for a network a machine joins, SSH has no join, approval, removal or discovery, and the 26 manual actions the owner took stay in front of the 4 that enrollment aims for. The order of the work is wrong: about 6,700 lines of SSH-R and 20,900 unmerged lines of enrollment exist, while A74, the engine-free probe that was to run first, has no result, SSH-R has never met a real sshd, no vector covers an unreachable, lost or admitted machine, and S3's counted runs sit behind 26 unmerged order groups and the S1 bar. Run the probe first, on what was built; let it decide between SSH-M and a Tailscale provider; develop enrollment against a disposable Linux node the coordinator controls; and keep the Windows computer for the counted evidence. Three questions go to the owner: S3's fault means, which join v0 proves, and whether a disposable node may be used.

**What the analyst read.** Root: the repository root at 2eb8f258 (2026-11-12); every citation is relative to it. Read in full: docs/guides/full-review.md (all but Focus 1, lines 112-143); docs/implementation/HANDOVER.md; AGENTS.md 'What holds'; the records of rulings 50, 56, 72, 76 and 77 and their ledger rows; docs/guides/second-machine.md; experiments/014-s3-two-machines/README.md and plan.md; IN_FORCE.md topic 10; spec/promises/enrollment.yaml and placement.yaml; PRV-013 to PRV-015, PRV-021 and PRV-025 in spec/promises/providers.yaml; the enrollment operations in spec/core/operations.yaml (lines 1461-1640); docs/proposals/enrollment/README.md sections 1 to 7; section 11 of docs/proposals/multi-machine/README.md (A74's H1-H10); docs/AUDIT.md Purpose, Layered claim and Waiting for the owner; ROADMAP.md rows for A74, A81 and S3 and 'Proposing an experiment'. Read in part: packages/provider-ssh-runtime (README lines 1-60, 84-94, 120-135, 186-235, 290-335; conduit.yaml; the Type's schema; greps of src), docs/implementation/PROVIDERS.md (status, shared table, decisions A-E, owner section), order 117's sketch of the Windows node, G5.md's MC4 rows, engine/CONFORMANCE.md by grep. Counted by command: promises and vector fields (519 promises), conformance/manifest.yaml (494 cases and which promises they cite), SSH-R's line counts, the diff stat of branch wo/049a1 against the commit. Not read: the rest of the multi-machine proposal, networking-paradigms.md and verification-methods.md, the records of rulings 51, 52 and 75, the bodies of orders 137-150 and 049a1, the staged patches 049a/049b, the engine's source, the plans of S2 and S10 beyond greps, the content of any side branch, and the private handoff note outside the tree. Nothing was built or run and no machine was reached: statements about how WSL2, Windows Firewall, Tailscale, OpenSSH and systemd behave that the repository does not record come from general knowledge, are marked as such, and are what the re-registered A74 should observe.


## FR-318

**A fair second machine for placement and a total cut; not for a lost machine, identity between runs, or enrollment as a check**

P05 I1 · assessment · verifier: partly · decides: none

Claim by claim, for the setup as ruled (a WSL2 distro on the owner's computer, Tailscale and SSH, one run user, interop off).

Placement by a provider: faithful in part. Real: a second kernel and operating system, node-tied values read on the second machine, a placement fixed for life, the engine's remote path. Not tested: any choice. With one non-local machine the provider names that machine or none, so selectors, labels bound at enrollment and 'unavailable while a matching node is unreachable' each get one trivial case.

Partition: faithful as a total, silent, two-way cut, provided the rule really covers WSL's mirrored traffic (the tree records no observation; A74 never ran). Not tested: a cut of one direction or of one of the two paths (the provider's SSH connection from the Mac, which carries every run's channel, and a run's calls back to the engine's https address). That is where a convention would need step-by-step behavior. Timing comes from barriers and a withheld reply, so the real network adds SSH's timeouts and little beyond the planned vectors.

Lost machine: not faithful. (1) Stopping the distro kills every run: SSH-R's runs are transient systemd user units, so on return there is no old run to refuse and no live unit for the owed stop to end. P4's 'old runs gain no access' and 'completes the stop despite removal' then pass with nothing to act on, and MC4-002's overlap of an old and a later run cannot appear. (2) Windows stays online in the tailnet (ruling 77's own reason for SSH), so the Mac meets a host that answers, not silence. (3) The disk and host key survive, so a rebuilt machine under an old hostname never occurs. (4) 'gone' has no real source, which the README admits.

Unknown outcomes across the network: faithful; they rest on barriers, as the vectors will.

Proof of identity: engine to machine (a pinned host key) and run to engine (a token and a public certificate) are real. Between runs on the second machine nothing holds: one user, PRV-025 stated unmet, so V5's safety promise cannot be shown there and D20 in the local profile has a known answer (every credential readable). The shared tailnet is no flaw: a flat network is the right adverse condition for ENR-011 and PLC-012, if the runs try, from the second machine and from a third tailnet device, to pass root's token at the remote-run address.

Enrollment evidence: not faithful as a check. The machine is already set up by hand, the coordinator already holds a key and the host key, and one machine cannot show a wrong machine, a reused hostname or a second join.

The owner as operator: the lockdown left every fault and every privileged step with the owner, so nothing can be rehearsed or repeated without the owner; the machine is also shared, under the one user, with work-order workers, which a loss phase would kill.

One hypervisor (general knowledge of WSL2, not recorded in the tree): all distros share one kernel and one network namespace, and in mirrored mode loopback is shared with Windows; any Windows process of the owner can enter the distro as root. So the lockdown works in one direction, and a loopback listener there is not private to the distro.

What is working: ruling 77's liveness argument; SSH-R leaving no listener on the second machine and leaning on OpenSSH and systemd; saying plainly that PRV-025 is unmet; the care over terminate against shutdown.

**Evidence.** experiments/014-s3-two-machines/README.md:55-59 (P4: 'On return, old runs gain no access from location or name reuse ... after return, verify that provider completes the stop despite removal'); :116 (T6 'After the prepared WSL shutdown'); :127-128 ('Use barriers and a withheld reply'); :263-264 (O4). packages/provider-ssh-runtime/src/backend.rs:1062-1078 ('--user', '--collect', '--unit=', 'RuntimeMaxSec='). packages/provider-ssh-runtime/README.md:12 ('per-run TLS gate on the engine's machine'), :21-31 ('PRV-025's isolation is unmet ... Any run can read every other run's identity files and environment'), :89-91 (sshd closes the run's stdin within 120 seconds of a cut). docs/proposals/ruling-77/README.md:27-29 ('when the owner stops Ubuntu, Windows stays online in Tailscale'), :48-52. spec/promises/providers.yaml:493 ('A provider of one machine ... name that machine or none'), :813-815 ('which none of its runs runs as or in'), :430 ('gone only where the provider knows'). experiments/012-model-checking/G5.md:61 (MC4-002 'Old/new-ID overlap despite retained stop custody'). experiments/014-s3-two-machines/README.md:78-80 and :171 list PRV-025 as exercised and as V5's safety promise.

**Waits.** Shapes I2, I3, I8 and I10.

**Verifier.** partly. Reopened at 2eb8f258. Holds: P4 and T6 as quoted (experiments/014-s3-two-machines/README.md:55-59, :116, :123-134, :263); runs are transient user units (packages/provider-ssh-runtime/src/backend.rs:1059-1081: systemd-run --user --pipe --wait --collect, RuntimeMaxSec, ExecStopPost), so no unit survives a stopped distro; 'A provider of one machine ... name that machine or none' (spec/promises/providers.yaml:493); 'gone only where the provider knows' (:430); PRV-025's 'which none of its runs runs as or in' (:813-815) against SSH-R's own statement (packages/provider-ssh-runtime/README.md:21-31); ruling 77's reason (docs/proposals/ruling-77/README.md:27-29); no silent cut has ever been tested (packages/runtime-provider-kit/README.md:131 'A silent packet-drop test needs privileged network controls and is not part of these tests'). Does not hold or is uncertain: (a) 'the Mac meets a host that answers, not silence' is not in the tree, and the analyst's own I5 O4 lists 'refused, reset or silence' as unknown; (b) 'pass with nothing to act on' overstates: the stop still removes the run directory (SSH-R README:79-80), identity files stay on disk after an abrupt kill because ExecStopPost never runs (:55-56), and T6 tests old identities by replay (README:116); (c) 'MC4-002's overlap cannot appear' is treated as a shortfall, but the owner answered MC4-002 on 2026-11-12: shipped providers carry out an owed stop on a machine before starting any run there (docs/proposals/ruling-42/README.md:275-285; IN_FORCE.md:244-245), so on one machine it is meant not to appear; (d) a rebuilt machine under an old hostname is not an S3 claim (P4 speaks of 'location or name reuse').

**Verifier's counterargument.** The verdict 'cannot show a lost machine faithfully' is true of the accepted means (stopping the distro), not of the computer. Windows sleep or hibernate freezes the WSL VM and returns it with live runs, and takes Windows off the tailnet as well; it is PLC-023's own example ('A machine that sleeps', spec/promises/placement.yaml:317). It needs no privilege and two owner actions (general knowledge of WSL2 and Modern Standby, to be observed first). On the partition: in v0 'the engine is the only enforcement point' (spec/promises/providers.yaml:821-822), so no cut can produce a dispatch on stale authority at a remote point; one-way cuts are beyond what S3 claims, and the analyst's own remark that the real network adds little beyond the vectors is the stronger finding.

**Verifier's corrections.** Say 'no live process to act on; leftover run files and a replayed credential remain'. Mark (2) as unobserved. Replace the MC4-002 remark with: plan.md:19 and :258 still call it an 'Owner hold', which is stale since IN_FORCE.md:244-245. Add: with SSH-M's evidence being the host key's fingerprint, reading it over SSH is circular; it must be read at the Windows console (ruling 68 R2, docs/proposals/ruling-68/README.md:44-45).


## FR-319

**S3-OF: how S3 cuts and heals the network and loses the machine**

P05 I2 · question-for-owner · verifier: partly · decides: owner

The plan's count for option A is right as arithmetic and understated as a cost. Each run has 8 partition cells and 2 loss phases: 8 cuts + 8 heals + 2 stops = 18, and 72 for four runs. Left out: the 2 restarts of the distro per run (the plan says so), which makes 20 actions at the Windows keyboard per run and 80 in all; the firewall commands need an administrator shell (general knowledge); an aborted cell leaves a rule the owner must remove; the 18 points are spread through tasks that real model clients carry out, so the owner is on call for the whole run, not for 9 to 18 minutes; and each 30 to 60 second wait sits inside a timing cell while client and dispatch deadlines run. A also drops the accepted README's 'a coordinator script restores both', so nothing can be rehearsed or repeated without the owner.

Options:
(A) The owner's hand at every barrier: 20 actions a run, 80 in all; installs and grants nothing.
(B) A Windows SSH account limited to the cut, heal and stop commands: no action per run after 30 to 60 minutes of setup; it opens an entry with firewall rights on the Windows host itself, outside the distro the owner gave to Conduit.
(C) A fault user in the distro with one sudo rule for one script: the coordinator drives every cut and heal with a kernel rule on the second machine, the method the README was accepted with; the owner only starts the distro again after each loss, 2 actions a run, 8 in all; 20 to 40 minutes of setup. The heal needs a timer or a control port left open, a detail for the harness.
(D) New: cut on the Mac's side of the path. SSH-R's machine table takes any host and port, so it can point at a small local proxy that forwards to the second machine's sshd, and tailscale serve can front the same proxy before the engine's https listener. The coordinator then drops bytes in either or both directions at exact barriers. No privilege on either machine, no setup by the owner, one-way cuts that a single firewall rule cannot make, and cells that can be repeated. The plan's first probe order already builds the silent-cut part; Toxiproxy is the existing tool of this kind, and A74's own H6 cuts on the first machine. The owner's hand remains only to stop and start the distro: 4 actions a run, 16 in all. Limit: bytes stop while TCP between the hosts stays up, so a new connection hangs after connecting where a real cut fails to connect; one cut by the owner's hand per runtime profile, 2 in all, checks that both look the same to the engine. Not tested: from SSH-R's README only.

Recommendation: (D); (C) if the owner wants one of the three offered. A's reason, that the owner is present anyway, holds for enrollment and sign-ins at the start, not for 20 interleaved actions. C restores the accepted method for one sudo rule that conduit-run does not hold. B is the widest grant. A, B and D change the accepted fault preparation; C keeps its cut.

**Evidence.** experiments/014-s3-two-machines/plan.md:259-298; :268-270 ('four cuts and four heals) and one loss phase (one shutdown); two workloads make 18 command-and-confirm actions per run, 72 for four runs'); :274-275 ('The two WSL restarts per run ... are not included in the 18 fault commands'); :294 ('Coordinator's recommendation: (A)'); :93 ('configurable silent cuts'). experiments/014-s3-two-machines/README.md:123-125 ('drop traffic to/from M1 using a Linux-side firewall rule; loss is wsl --shutdown. A coordinator script restores both'); :184-185 ('eight partition cells and two loss/return phases'). packages/provider-ssh-runtime/README.md:223-225 ('host is its SSH destination, port is 1-65535'). docs/proposals/multi-machine/README.md:1026 (H6: 'A packet filter drop on M1'). Arithmetic rechecked: 18 x 4 = 72; 18 x 30-60 s = 9-18 min; 60 + 9 = 69, 60 + 18 = 78.

**Waits.** S3-H's fault implementation and S3-R1 to R4 (plan.md:71, :82, :190). Changes README lines 123-125, the plan's S3-H and the last section of docs/guides/second-machine.md.

**Verifier.** partly. Count recounted from experiments/014-s3-two-machines/plan.md:268-275: per workload 4 cuts + 4 heals + 1 shutdown = 9; two workloads = 18 a run, 72 for four; the two restarts are excluded by the plan's own words (:274-275), so 20 a run and 80 in all. 69-78 minutes checked. 'A coordinator script restores both' is in the accepted README (:125) and dropped by A. Option D is technically possible: SSH-R pins by alias, not by address (packages/provider-ssh-runtime/src/ssh.rs:83-86: UserKnownHostsFile, HostKeyAlias=hostname, StrictHostKeyChecking=yes), and host and port are free (README:223-225). The plan already foresees a reusable silent-cut component (plan.md:93-94; docs/implementation/PROVIDERS.md:125). H6 names 'A packet filter drop on M1' (docs/proposals/multi-machine/README.md:1026). Not in the tree: that the firewall rule needs an administrator shell, and that the owner is 'on call for the whole run' (the cells are 'prepared ... apart from author tasks', README:123-126).

**Verifier's counterargument.** D cuts only the two paths the harness knows (the SSH master and the https address). A partition test should also catch a path nobody planned; a packet filter does, a proxy cannot. The existing paradigm, and A74's own registered means, is the Mac's packet filter (pf): a real silent drop, one direction or both, driven by the coordinator, nothing changed on Windows, for one sudo rule for one script on the Mac. That is option C's grant moved to the machine where the engine already runs as the owner, and it removes the doubt whether a Linux rule sees WSL's mirrored traffic. Offer it as option E. For the loss, Windows sleep or hibernate gives a return with live runs (see I1). D's human-latency argument against A is sound: a 30-60 second wait inside an in-flight request can let a dispatch deadline fire before the cut is noticed, which spoils D20's observed close times.

**Verifier's corrections.** State D as untested and limited to known paths. Add option E (pf on the Mac, H6's means). Recommendation order: E or D for the cells, with two cuts by hand as calibration; C if only the three offered. It changes accepted fault preparation (README:123-125), so the owner decides: correct.


## FR-320

**S3: add a loss that returns alive, and require a live unit for the owed stop**

P05 I3 · experiment-change · verifier: partly · decides: owner

Register rows: ROADMAP.md:141 (S3) and :174 (E15). Claim tested: P4, 'explicit loss retires the node and its placed instances and closes their Conduits ... On return, old runs gain no access ... verify that provider completes the stop despite removal'.

As accepted, T6 loses the machine by stopping the distro. Every run dies with it, so the claim is never put under strain (I1).

Change:
1. Add phase T6b, lost while cut. With a test instance running on the second machine: cut the network (do not stop the distro), root declares the node lost, unregister the retired instance so that its stop is owed, set up its replacement where the scenario has one, then heal. Two timings: a heal inside 120 seconds, before sshd closes the run's stdin, and a heal after it.
2. Pass condition for T6b. Short cut: the unit is active at heal (systemd's own report, keyed by instance and run ID); a call the old run makes to the engine's https address after the declaration is refused; the owed stop ends that live unit; any interval in which the old run and a later run both ran is reported with its length, which is MC4-002's overlap, observed and not hidden. Long cut: the unit's journal shows it ended by itself within the provider's stated bound, and the engine accepted nothing from it after the declaration.
3. In T6 (power-off), report a completion that finds no unit as 'nothing left to stop'. M1's rule 'zero pending stops cannot pass P4' gains: at least one owed stop in the run must end a live unit.
4. Lose the machine with wsl --terminate Ubuntu-24.04, the guide's own pause command, not wsl --shutdown, which stops every distro.
5. New measures under M1: the unit's state before and after each heal; the overlap interval; what the Mac saw at loss (refused, reset or silence) and the time to the provider's 'unreachable'.

Cost: one more phase per workload, driven by the coordinator under I2's C or D, with no owner action. It buys the only real-machine evidence for PRV-021, PRV-030 and the refusal of a retired instance's credential.

**Evidence.** experiments/014-s3-two-machines/README.md:55-59, :116, :155 ('zero pending stops cannot pass P4'). experiments/014-s3-two-machines/plan.md:175 ('Include 081's old/later-run overlap observation'), :16-19 ('Owner hold MC4-002'). packages/provider-ssh-runtime/README.md:89-91 ('remote sshd closes the run's stdin ... within 30 x 4 = 120 seconds ... RuntimeMaxSec is the backstop'). packages/provider-ssh-runtime/src/backend.rs:1062-1078. docs/guides/second-machine.md:50-51 ('wsl --terminate Ubuntu-24.04; wsl --shutdown would stop every distro, Docker Desktop's too') against experiments/014-s3-two-machines/README.md:125 ('loss is wsl --shutdown'). spec/promises/placement.yaml:233-236 ('a run left on a machine that comes back is asked to stop only by a further retirement ... or by the request to unregister').

**Waits.** The README was accepted under ruling 75 D, so the changed pass condition is the owner's; it can ride on I2's answer. S3-H and S3-L2's vectors V3 and V4 follow it.

**Verifier.** partly. Core holds: T6 loses the machine by stopping the distro (README:116, :125, :263) and M1 requires only a received stop, a pending obligation and a matched completion (:155), all of which pass when the unit is absent ('confirms inactive or absent', SSH-R README:79-80). PLC-017's observable says what the owed stop is for: 'a run left on a machine that comes back' (spec/promises/placement.yaml:233-236; spec/promises/providers.yaml:643-646). The 120 second and RuntimeMaxSec bounds are as quoted (SSH-R README:89-92). Rows ROADMAP.md:141 and :174 checked. Faults in the item: it cites plan.md:16-19 'Owner hold MC4-002', which the owner answered on 2026-11-12 (IN_FORCE.md:244-245; docs/proposals/ruling-42/README.md:275-285); PRV-030 evidence does not come from a declared-lost phase (the instance is retired, no run for that ID is asked again); after declare-lost the node is retired and a replacement on that machine needs a new admission (spec/promises/placement.yaml:400-402), which the item does not cost.

**Verifier's counterargument.** The same strain is available without any cut tooling: sleep or hibernate the Windows computer with a test run alive, declare lost, unregister, wake (two owner actions; PLC-023's own example). A cut keeps the old run executing during the loss, which is the harsher case, so keep both. The pass condition 'the owed stop ends that live unit' cannot be told apart from SSH-R's own best-effort stop at session end (SSH-R README:89) unless the witness is keyed by the ledger entry. Under option A the phase adds 8 owner actions a run (2 workloads x 2 timings x cut and heal).

**Verifier's corrections.** Pass condition should assert the rule in force: no run starts on the second machine before the stop it owes there is carried out (IN_FORCE.md:244), and report overlap across machines as the stated limit (IN_FORCE.md:245). Move PRV-030 to the undeclared cut cells (unit state before and after each heal). 'wsl --terminate' replaces the owner's accepted answer O4 (README:263), so that edit is the owner's with the rest. Owner decides: correct.


## FR-321

**The second machine has produced no experimental result, and its code ran ahead of its probe**

P05 I4 · fact · verifier: supported · decides: none

At this commit:
- A74, the engine-free probe that prepares S3, is 'open: no one has started it'; experiments/006-across-machines does not exist; its first order, S3-A741 ('Dispatch S3-A741 now'), was never written. It appears only in the planning orders 117 and 121.
- SSH-R is merged: 2,883 lines of source and 3,848 of tests, over orders 137, 140, 145, 146, 149 and 150, with 147 written. Its README says: 'This is fake-remote unit evidence, not qualification against sshd, user systemd or WSL. No test starts sshd or reaches another machine', and 'The owner's test host will confirm the systemd behavior; the fake remote only models it'. One of its tests is what turns main's CI red and skips the engine shards.
- SSH-M, the machine provider, is not built.
- Enrollment is text only on main: 12 promises, no engine code (no 'enroll' in engine/crates), no case. Its core, 049a1, is an unmerged branch of 76 files and 20,901 added lines after five rounds (about 3,600 in the engine's core, the rest mostly conformance cases).
- Of the 494 conformance cases, none cites PLC-011, PLC-013, PLC-017, PLC-019, PLC-023, PLC-024, PLC-025, PLC-027, PLC-028, PRV-013, PRV-015, PRV-021, PRV-025 or any ENR promise. Unreachable, declared lost, gone, placement by a provider, the owed stop, the gate and admission rest on text and bounded models only. The engine's own conformance statement says 'No distributed configuration or guarantee is demonstrated by this core' and that declare-lost is unfinished.
- S3's counted runs are gated on 31 named orders or order groups, of which 5 are on main (077, 081, 082, the runtime kit, SSH-R). The other 26 include batch 2's B2-S, B2-C and B2-H, which wait for the owner's judgment of S1's bar.
- So far the computer has served as work-order capacity (ruling 76, point 6); what it taught concerns the build (orders built there that failed on macOS), not Conduit's claims.

**Evidence.** ROADMAP.md:69 ('open: no one has started it'). Command: ls experiments (no 006). Command: grep -rln 'A741' docs/work-orders gives only 117 and 121. experiments/014-s3-two-machines/plan.md:209 ('Dispatch S3-A741 now'), :186-189 (the run gate; my count of its named IDs is 31). packages/provider-ssh-runtime/README.md:7-8, :87, :330 ('Untested on the Windows computer'). Command: wc -l packages/provider-ssh-runtime/src/*.rs src/bin/*.rs = 2883; tests = 3848. docs/implementation/HANDOVER.md:198-205 (CI red since #260). Command: grep -rlE 'enroll' engine/crates --include='*.rs' gives nothing. Command: git diff --shortstat 2eb8f258...wo/049a1-build-enrollment-core = '76 files changed, 20901 insertions(+), 75 deletions(-)'. Command: grep -c for each ID in conformance/manifest.yaml = 0; 494 cases by PyYAML. engine/CONFORMANCE.md:62, :130. IN_FORCE.md:212 ('No support for remote machines is claimed before a test across two machines').

**Waits.** Grounds I5 and the summary's judgment on the order of the work.

**Verifier.** supported. Recounted: ROADMAP.md:69 'open: no one has started it'; ls experiments shows no 006; grep -rln 'A741' gives only orders 117 and 121, PROVIDERS.md and plan.md, no order file; wc -l gives 2883 (src and src/bin) and 3848 (tests with support/mod.rs); SSH-R README:7-8, :87, :330, :350 as quoted; packages/ has no provider-ssh-machine; grep -rliE 'enroll' engine gives nothing; 12 'id: ENR-' in spec/promises/enrollment.yaml; git diff --shortstat 2eb8f258...cdf146d5 = 76 files, 20901 insertions, 75 deletions (engine/crates/core 3619, conformance 16707, spec 62); conformance/manifest.yaml has 494 cases by PyYAML and zero citations of each listed ID or any ENR (also zero across conformance/); engine/CONFORMANCE.md:62 and :130; run gate at plan.md:187-188 holds 31 named IDs when 'S2's prerequisites', 'placed vectors' and 'tagged safety cases' are left out, 5 on main (077, 081, 082, A63-RK, SSH-R); CI red at docs/implementation/HANDOVER.md:198-205.

**Verifier's counterargument.** Two things soften 'ran ahead of its probe': the lane has queued a loopback-sshd tier for SSH-R (docs/implementation/PROVIDERS.md:50; SSH-R README:350), and ADDR (079) is built on the side branch s3/m2, so 25 of the 26 are wholly unbuilt. Neither changes the finding. The plan itself ordered the probe first ('Dispatch S3-A741 now', plan.md:217); what failed is the execution, not the plan.

**Verifier's corrections.** 'Dispatch S3-A741 now' is plan.md:217, not :209; the run gate is :186-191. Add that the machine provider's capability bindings are still staged patches (IN_FORCE.md:224), so SSH-M has no live contract to build against.


## FR-322

**A74: register it again around what was built, and run it before any more SSH or enrollment code**

P05 I5 · experiment-change · verifier: partly · decides: owner

Register row: ROADMAP.md:69 (A74). Claim: 'do the mechanisms under S3 hold on real machines?'

H1 to H10 were written for gates on the second machine (Docker, ghostunnel, a Unix socket), an OCI registry, etcd and a host behind NAT. SSH-R keeps the gate on the engine's machine and leaves no listener on the second machine, so H1, H2, H8, H9 and H10 have no object on S3's path; the plan already asks the owner whether H5, H8, H9 and H10 may stay unmeasured.

Change: one engine-free session on the real pair, each observation dated with what was measured, no pass gate (as registered), a contradiction with a promise raised with its counterexample.
O1. SSH-R's three operations against the real machine as conduit-run: run, inspect, stop; the stop kills the whole process tree; closing stdin ends the run; the lifetime bound fires.
O2. A silent cut in mid-call: the time until each side notices, against the stated 30 second (ssh), 120 second (sshd) and lifetime bounds; the gate's 45 second bound watches a loopback connection on the Mac, not the path between the machines, so say which bound the cut actually meets (H6).
O3. The cut means chosen in I2: does it silence both directions of WSL's mirrored traffic, and what becomes of connections already open.
O4. Loss: what the Mac sees when the distro is terminated while Windows stays online (refused, reset or silence); on restart, whether sshd and the user manager return with nobody logged in.
O5. H3, H4 and H7 as written, with stand-ins: a wrong certificate, a removed instance's credential replayed, a forwarded port.
O6. Tailscale: whether tailscaled runs inside the distro, with Tailscale SSH on, beside the Windows host's under mirrored networking; which hostname and operating system each reports; whether both ends can show the node key (ruling 56, point 6, left this to A74; the enrollment proposal could not verify it). This decides I8.
O7. The clock offset between the two machines after the Windows computer sleeps and wakes, since tokens and enrollments carry expiries.

Order: before SSH-M, before further rounds of 049a1 and 080, before S3-H. Cost: the plan sizes the two probe orders at 600 to 850 and 200 to 350 lines plus a session; less with H8 to H10 dropped. It buys the first dated evidence from the second machine and can stop provider work that rests on a wrong assumption, as the Mac failures of orders 131, 137 and 140 did late.

**Evidence.** docs/proposals/multi-machine/README.md:1008-1030 (set-up and H1-H10). experiments/014-s3-two-machines/plan.md:68, :72, :91-98, :251-254 ('A74 unmeasured cells'). packages/provider-ssh-runtime/README.md:12-19 ('These are different connections'), :87. docs/proposals/ruling-56/README.md:25 ('that both ends can show the node key is to be confirmed in A74'). docs/proposals/enrollment/README.md:197 ('Whether WSL2 with networkingMode=mirrored uses the host's tailnet I could not verify'), :231-233. docs/implementation/HANDOVER.md:103-105 (orders built on Windows that failed on macOS).

**Waits.** The owner's acceptance can ride on the plan's open question about A74's unmeasured cells; no new ruling is needed if the answer is recorded there. SSH-M, S3-M and I8 wait on O6; S3-H on O2 to O4.

**Verifier.** partly. Holds: A74 unstarted (ROADMAP.md:69); H1-H10 and their set-up as described (docs/proposals/multi-machine/README.md:1008-1030); the plan's open question covers H5, H8, H9 and H10 only (plan.md:251-255); sizes at plan.md:68 and :72; ruling 56 point 6 leaves the node key to A74 (docs/proposals/ruling-56/README.md:25); the gate's 45 second bound sits on accepted sockets of a gate bound to 127.0.0.1 (packages/runtime-provider-kit/README.md:115, :127-131), so a cut between the machines meets SSH's 30 and 120 seconds, not the gate's bound. Does not hold: 'H1 and H2 have no object'. H1's substance moves to the kit's gate on the Mac (a call without the engine's token refused before the channel is used, kit README:120-124), and H2 ('Another run on M2 cannot reach the server') has an object in both profiles: a known contrary answer under one user, and the open question under Docker. The accepted README makes T2 to T6 depend on H1 and H2 (README:111-116). O6 leaves out that the tree records 'Tailscale recommends the Windows host only, not inside WSL2' and that WSL2 inside Windows is 'a second node with a clashing name' (docs/proposals/enrollment/README.md:194-196, :80). The citation HANDOVER.md:103-105 is wrong; the Mac failures are at :81-84.

**Verifier's counterargument.** 'Before further rounds of 049a1 and 080' is not carried by the evidence: 049a1 has one must and one should left after five rounds (HANDOVER.md, queue row 3) and is provider-neutral; 080 is needed by any remote runtime provider; no outcome of the probe changes either unless the contract is shrunk, which the analyst does not recommend. Stop SSH-M and S3-H, not those. The cheapest first evidence is not a new 600-850 line stand-in crate: drive the merged SSH-R through the provider-contract harness (experiment 022, ROADMAP register) against a loopback sshd, which the lane already queued (PROVIDERS.md:50), then against the real machine. O6 needs the owner's sudo and a second tailnet device and changes ruling 77's ruled setup, so it is a permission to ask, not an observation to add.

**Verifier's corrections.** Keep H1 (re-aimed at the gate on the Mac, with H7's second local user) and H2 (as a list of which stated openings work on the second machine, and as the hypothesis for the Docker profile). Order: probe before SSH-M and S3-H only. Dropping H1 and H2 is wider than the plan's question, so it cannot 'ride' on it. Owner decides: correct.


## FR-323

**Adding this machine took the owner 26 actions, against the 4 that enrollment aims for**

P05 I6 · fact · verifier: partly · decides: none

Counting each command, file edit, install, sign-in or console setting in the guide as one action:
- First setup (2026-11-11), 5 numbered steps, 15 actions: install WSL and restart (2); edit .wslconfig, shut WSL down, start Ubuntu (3); install Tailscale on both machines and sign both in (4); apt install, rustup, Node through nvm (3); add the coordinator's key, start sshd, turn off passwords (3).
- Ruling 77's setup, 6 numbered steps, 11 actions: create conduit-run, turn lingering on (2); the root-owned key file, the sshd Match block, the key taken out of the owner's account (3); system packages (1); wsl.conf, restart the distro (2); sign Codex in as conduit-run (1); rename the Mac, turn on HTTPS certificates (2).
- Total 26, not counting the optional desktop shortcuts or handing over the address and host key. Ten need sudo or an administrator shell. They ran over two days, and three parts were still open at the handover (bubblewrap, the restart, the key's removal).
- Of the 26, two are once per engine (the tailnet rename and certificates) and one serves the work-order workers (Codex's sign-in); 23 are per machine.

What enrollment aims for is 4 actions by the person: ask to add a machine with its labels; run one command or open one link on the new machine; read the fingerprint there; admit with it typed.

After all 26 the computer is still not a node. With SSH-M as sketched ('enrollment with a fresh key per enrollment, evidence that is the host key's fingerprint'), the join command covers only the admission of a key, so the 4 come on top of about 23, and the join command itself needs the owner's sudo, because the key file is root-owned.

**Evidence.** docs/guides/second-machine.md:55-81 (first setup), :83-112 (ruling 77's setup), :21-26 (three parts open), :114-118 (what to hand the coordinator). docs/proposals/enrollment/README.md:169-171 ('What the person does: asks the control panel to add a machine with chosen labels; runs one command, or opens one link, on the new machine; compares the code; admits'). spec/promises/enrollment.yaml:45 ('one action, a url to open or a command to run on the new machine'), :144 ('character for character'). docs/work-orders/117-plan-s2-s3.md:114-116 (SSH-M's sketch). packages/provider-ssh-runtime/README.md:302-303 ('A root-owned AuthorizedKeysFile ... keeps key admission outside runs' authority'). docs/proposals/ruling-50/README.md:9-12 (the owner's words).

**Waits.** Grounds I7, I9 and I11.

**Verifier.** partly. Recounted from docs/guides/second-machine.md:55-112: first setup 2+3+4+3+3 = 15, ruling 77's setup 2+3+1+2+1+2 = 11, total 26; ten privileged steps is reproducible if the Tailscale installers are not counted. Three parts open at the handover (:21-26). The aim of four person actions is in docs/proposals/enrollment/README.md:169-172, with the typed fingerprint from spec/promises/enrollment.yaml:143-144. SSH-M's sketch at docs/work-orders/117-plan-s2-s3.md:114-116; the root-owned key file at SSH-R README:302-303. What does not hold: this is a count of what the guide prescribes, not of what the owner did. Step 1 is conditional ('Where it is missing', :60), the owner called it 'that distro I use tor nothing else' (docs/proposals/ruling-72/README.md:45-46), and no time or mistake was recorded. '23 per machine' is too high: Tailscale on the Mac and its sign-in are per engine (2 more), rustup and the build tools serve work orders, and 5 steps exist only because of WSL (.wslconfig, shutdown, start, wsl.conf, restart).

**Verifier's counterargument.** The four that enrollment aims for exclude 'the agent on the machine', which the proposal leaves as 'the provider's job' (docs/proposals/enrollment/README.md:168-169), so 26 against 4 compares two scopes. Like for like, a plain Linux machine joined by SSH needs about 9 or 10 privileged steps (join the tailnet, sshd, the run user and lingering, the key file and Match block, passwords off) against one install-and-join command for Tailscale. The direction holds; the ratio does not.

**Verifier's corrections.** Title should read 'The guide prescribes 26 owner actions'. Per machine: about 21, of which 5 are WSL's. Add that SSH-R's own key is separate from SSH-M's 'fresh key per enrollment' (SSH-R README:225 'No key is generated by this provider'; src/ssh.rs:80), so two keys are admitted with sudo, not one.


## FR-324

**The SSH provider is sound for this computer and moves away from ruling 50's aim**

P05 I7 · assessment · verifier: partly · decides: none

What it buys: a liveness probe that tells the truth about Ubuntu; no API credential for the tailnet; a join that carries only a public key; evidence that is a real key's fingerprint.

What it costs ruling 50's 'adding machines should be much simpler with conduit':
1. No join. The machine-provider contract is written for a network that a machine joins: 'how it pins its coordination server', 'where the provider's network supports approval, it cannot send or receive traffic until accept-machine.v1', 'Remove the machine ... from this provider's network'. enroll.v1's input holds an enrollment ID, labels, an expiry, require and headless, and no address. An SSH machine joins nothing, so SSH-M must hold the machine's address in its own configuration: an inventory, as Ansible's is.
2. Two inventories. SSH-R already has one, typed by hand with the host's public key in it, read once at startup, with no control that changes it. An admission through SSH-M would not reach it.
3. Nothing behind approval and removal. A pending SSH machine is fully on the network, and 'remove' can only forget a key.
4. The laborious part stays in front of the provider (I6) and needs sudo each time.
5. One machine provider at a time (PRV-013). A Tailscale provider later replaces SSH-M, and replacement leaves every node omitted until its machine is enrolled again.
6. The join-action half of enrollment (the action, its secret, expiry, approval, removal: ENR-001 to ENR-003, ENR-008, ENR-009, ENR-012) then has no real provider in v0, only a fixture.

Ruling 77's reason concerns where tailscaled runs, not Tailscale. A Tailscale provider can define 'reachable' by a probe of the runtime's port, or tailscaled can run inside the distro, where it stops with Ubuntu. Neither is verified; the enrollment proposal says so, and A74 was to find out.

Which paradigm to follow. Where the provider has a network, Tailscale's own: a one-off tagged key, device approval, tags as labels, the node key as evidence, and Tailscale SSH as the channel, which needs no sshd configuration, key file or host-key pin. The promises are already this shape. Where it has none, Ansible's inventory with OpenSSH's own check of a typed fingerprint (Salt's accept-a-key), stated as that. Not k3s or Nomad agent join: both need a node agent of Conduit's own, which the enrollment proposal rejected as option D under ruling 40. What they do teach is one line that carries the address and a scoped, short-lived token. The mixture now planned, a contract shaped for Tailscale and a provider shaped for SSH, has the costs of both.

**Evidence.** spec/promises/enrollment.yaml:49-50, :55. spec/core/operations.yaml:1566 ('required: [enrollment, labels, expires, require, headless]'), :1615 ('Where the network supports approval, this permits machine traffic'), :1631. packages/provider-ssh-runtime/contracts.yaml:227-229 (machines: 'JSON array: hostname, host, port, user=conduit-run, host_key=ssh-ed25519 public key'; no controls key in the Type); packages/provider-ssh-runtime/README.md:193 ('Configuration is read once'), :223-225. spec/promises/providers.yaml:398-399 ('at any time at most one machine provider'), :409-411 ('each non-local node stays omitted until its machine is enrolled again'). docs/proposals/ruling-77/README.md:23-32. docs/proposals/enrollment/README.md:92, :186-197 (how Tailscale would implement it; 'I could not verify'), :203-206 ('Option D ... Rejected: it is a network and agent of Conduit's own, which ruling 40 leaves to existing tools').

**Waits.** Grounds I8 and I9.

**Verifier.** partly. Holds: the contract's network wording (spec/promises/enrollment.yaml:49-50, :55; spec/core/operations.yaml:1566, :1615, :1631); SSH-R's hand-typed machine table read once, with no control (packages/provider-ssh-runtime/contracts.yaml:227-229; README:193, :223-225); one machine provider at a time and re-enrollment on replacement (spec/promises/providers.yaml:398-411); ruling 77's single reason (docs/proposals/ruling-77/README.md:27-29); option D rejected (docs/proposals/enrollment/README.md:203-206). Point 2 is stronger than stated: run.v1 gives a runtime provider only 'the hostname and the labels' (spec/core/operations.yaml:916-917), so nothing in the language carries the admitted machine's key to the provider that must authenticate that machine (PRV-011, providers.yaml:351-354). Does not hold: 'SSH-M must hold the machine's address in its own configuration' (the join command can report the address and host key back, or SSH-M can list peers from the Mac's own tailscaled with no API credential); point 6 (SSH-M's command action, its expiry and attribution by a fresh key would be real; what lacks a real provider is a secret, approval that gates traffic, and a removal with effect).

**Verifier's counterargument.** ENR-002 is conditional ('where the provider's network supports approval'), so a provider without a network is inside the contract, only weaker; the mixture is not incoherent. The Tailscale path is not clean on this computer either: the tree records Tailscale's advice against running inside WSL2 and the name clash (docs/proposals/enrollment/README.md:194-196, :80), a device on the Windows host reports windows where ENR-010 wants linux (enrollment.yaml:254-255), and 'SSH-R may be reusable over Tailscale SSH' is untested. So this computer is the awkward case for both providers; ruling 50's aim is better judged on a plain Linux machine.

**Verifier's corrections.** Point 5 should say outright that ruling 77's 'not made a Resource for this computer' presumes a choice per machine, which PRV-013 does not allow. The paradigm answer belongs in a recommendation with a decider, not in an assessment marked 'decides: none'.


## FR-325

**Which join does v0 prove: SSH only, a Tailscale provider as well, or a smaller contract**

P05 I8 · question-for-owner · verifier: partly · decides: owner

Ruling 50 asked that Tailscale be a Resource in the pool and that adding machines be much simpler with Conduit. Ruling 77 chose SSH for this computer 'for now'. SSH has no join for the contract to carry (I7), so the part of enrollment that made it 'part of the language' would be proven in v0 only against a fixture.

Options:
(a) SSH only. S3 proves admission on a real machine: pending, the fingerprint typed, admitted. The join action, its secret, approval and removal are proven against a fixture, and the conformance statement says so. Cheapest. Ruling 50's 'much simpler' is left unmeasured.
(b) Probe first, then choose. Run A74's Tailscale observation (I5, O6: tailscaled inside the distro with Tailscale SSH on). If it works, build a Tailscale machine provider in place of SSH-M: every provider operation has an API behind it, the node goes offline when Ubuntu stops (ruling 77's reason is met), SSH-R may be reusable over Tailscale SSH, and joining is one command. If it fails, (a). Cost: one short owner-assisted session before any more code; then one provider crate and a tailnet API credential, or nothing.
(c) Shrink the contract to what v0 exercises: admission of a machine the provider reports (the enrollment proposal's option B). Fewer promises and operations; it reverses a design accepted in rulings 52 and 56 and work already done in 049a1.

Recommendation: (b). It costs the least before the facts are known, it is the only route that honours ruling 50's words, and it stops SSH-M being built if a better provider is one probe away. (c) is the right fallback only if the owner would rather have a smaller language than a simpler join.

**Evidence.** docs/proposals/ruling-50/README.md:9-12 ('tailscale should be a resource in the pool ... adding machines should be much simpler with conduit'). docs/proposals/ruling-77/README.md:15-19 ('Let's try your recommended setup for now, we can modify later if need be'), :30-32. docs/proposals/enrollment/README.md:126-140 (options A and B), :186-197. IN_FORCE.md:225, :227. docs/implementation/HANDOVER.md:172-173 (SSH-M not built). I7's evidence.

**Waits.** SSH-M (not built), S3-M, the scope of V7 and of 049b's fixture, and IN_FORCE.md:227. Nothing merged waits on it.

**Verifier.** partly. The tension is real: ruling 50's words (docs/proposals/ruling-50/README.md:9-12), ruling 77 'for now' (docs/proposals/ruling-77/README.md:16-19), SSH-M unbuilt (HANDOVER.md:174). Ruling 77 point 1 is in force (IN_FORCE.md:227) and the item says it would change it, so no silent contradiction. Faults: option (b) omits the tree's own record against tailscaled inside WSL2 (docs/proposals/enrollment/README.md:194-196); (c) is mislabelled: 'admission of a machine the provider reports' is the proposal's option A, 'confirmation only' (:126-131), not B; rulings 52 and 56 as the accepting rulings checked in the ledger (docs/proposals/ruling-log.md:60, :64).

**Verifier's counterargument.** Option (b) is 'probe first, then choose', which decides nothing now: the owner ruled on this the same day as the commit, with 'we can modify later if need be'. The only thing the owner must grant now is the probe's O6 (sudo, a second tailnet device, a change to the ruled setup), and that belongs in I5. Ask the join question only if the probe shows a Tailscale path that works. Missing option: keep SSH for this computer and let SSH-M discover machines from the Mac's local tailscaled peer list and probe them by SSH; that meets ruling 77's reason, needs no tailnet API credential, device approval or tags, and uses Tailscale as the inventory. A full Tailscale provider also costs the owner tailnet-wide device approval, tags in the policy file and an OAuth client secret (the proposal's own description, :186-193).

**Verifier's corrections.** Fold into I5 as a permission; drop as a separate question until there is evidence. Relabel (c) as option A. Name PRV-013's consequence in the question: whichever provider is chosen serves every remote machine.


## FR-326

**What would make adding a machine one command**

P05 I9 · recommendation · verifier: partly · decides: coordinator

Four changes, none to a promise.
1. Make the provider's join action a complete bootstrap. ENR-002 already allows 'a command to run on the new machine', one per operating system. For SSH, one idempotent script, run once with the person's own sudo, that creates the run user, turns lingering on, writes the root-owned key file and the sshd Match block, and prints the host key's fingerprint. This is what k3s (one curl line with the server's address and a token), Tailscale (its install script and one 'tailscale up' with a key) and Teleport (a generated install script bound to a join token) do. Today 'No setup script or independent supervisor is shipped', and the steps are prose in a guide.
2. One inventory. An admission must reach the runtime provider's machine table, or both providers must read one source; otherwise the host key is typed a second time by hand.
3. One command on the engine's side. 'conduit machine enroll' prints the join action, waits for the machine, shows the evidence, takes the typed fingerprint and admits, as OpenSSH's first-connection prompt accepts a typed fingerprint and compares it. ENR-006's comparison is unchanged.
4. The Agent's client program. An item found on the machine's PATH presumes a hand-installed client on every node; the bootstrap or a fetched item must carry it, or say that it is outside the count.

With a Tailscale provider, steps 1 and 2 become 'tailscale up' with a one-off tagged key and Tailscale SSH on.

Cost: one script per provider and operating system inside the provider's package (provider work, counted apart), and a change to how SSH-R learns its machines. It buys about 5 actions by the person where there are now 26, and a join that can be tested again and again (I10).

**Evidence.** spec/promises/enrollment.yaml:45-47. packages/provider-ssh-runtime/README.md:330 ('No setup script or independent supervisor is shipped'), :292-311 (the setup the script would hold). docs/guides/second-machine.md:83-107. experiments/014-s3-two-machines/plan.md:153 ('An archive item runs only on the engine's own runner'), :161 ('AGT's PATH item is an explicit unpinned-program shortcut'). docs/proposals/enrollment/README.md:71-72, :80 (kubeadm, k3s and Tailscale rows of the research). docs/work-orders/117-plan-s2-s3.md:119-120 (051c: 'conduit machine enroll, admit and refuse').

**Waits.** SSH-M's order, or the Tailscale provider's after I8; 051c's command.

**Verifier.** partly. Basis holds: ENR-002 allows a command per operating system (spec/promises/enrollment.yaml:45-46); 'No setup script or independent supervisor is shipped' (SSH-R README:330); the setup a script would hold (:292-311); 051c's commands (docs/work-orders/117-plan-s2-s3.md:119-120). 'None to a promise' does not hold for change 2: either the admitted key reaches the runtime provider through run.v1, which changes a contract in spec/core (operations.yaml:916-917), or two packages share a file outside their contracts, which couples providers that ruling 40 and S10 keep independent. Change 3 leaves out a binding limit: the join action and the evidence are shown only at root's own panel and never to a model (enrollment.yaml:73-81; spec/promises/pending.yaml:343-344, :353-355; spec/README.md:406-407), so the command must be run by the person, never in the coordinator's session, which is where conduit is usually run.

**Verifier's counterargument.** The Ansible form may be the cheaper one command: a bootstrap pushed from the engine's machine over the person's own SSH account, with the sudo password typed once, so nothing is pasted on the new machine; it strains ENR-002's words 'on the new machine'. 'About 5 actions' leaves out joining the tailnet (2) and WSL's own steps; for this computer it is nearer 8 to 10. A bootstrap that needs sudo each time also means each of S3's four fresh labs asks the owner again unless the key outlives the lab.

**Verifier's corrections.** Split the decider: the script and the CLI are provider and build work (coordinator); how an admitted machine's key reaches the runtime provider is a gap in the language (class it as a gap, owner if it changes run.v1). State that the command is the person's.


## FR-327

**May enrollment and faults be developed against a disposable Linux node the coordinator controls**

P05 I10 · question-for-owner · verifier: partly · decides: owner

On the Windows computer every enrollment needs the owner's sudo and presence, and every fault needs the owner's hand. That makes it the worst place to iterate on 'adding a machine in one command', and it cannot show a machine frozen and resumed, destroyed, or rebuilt under an old hostname with a new key.

Options:
(a) No. S3 shows loss as the power-off of one machine plus I3's cut, and its Limits say what was not shown.
(b) A second WSL distro on the same computer as a third node. It gives a second candidate for placement and a wrong machine for enrollment; it shares the kernel and network namespace, and the owner still operates it.
(c) A disposable Linux node on the Mac: a small VM, or a systemd container under the Docker the plans already need, as kind makes its nodes. The coordinator creates, freezes, cuts and destroys it itself. Used for developing and rehearsing the bootstrap and the fault harness, and for the cells the Windows computer cannot show, reported apart. It is never the second real machine: P2 keeps the Mac and the Windows computer, and no rented machine stands in.

Recommendation: (c). It needs no action from the owner, lets the join be tested dozens of times, and gives the faults with real content: a freeze is true silence and a return with live runs and a clock jump; a destroy is a real 'gone'; a rebuild is a new key under an old hostname. The owner's words in ruling 72, point 3, answered a question about a fallback ('We can use the windows machine whenever needed') and do not exclude it; ruling 68, point 4, replaced a simulated-network stage with model checking and real machines, which is why this is asked and not assumed.

**Evidence.** docs/proposals/ruling-72/README.md:13-18 (the question and the owner's words), :27-28 (decision 3). IN_FORCE.md:226. docs/proposals/ruling-68/README.md:50-53. docs/proposals/ruling-77/README.md:64-73 (the owner runs the steps that need sudo; S3's cut and loss need the owner's hand). spec/promises/placement.yaml:397-399 ('A report of a hostname under another id ... leaves the node omitted and adds a condition'); spec/promises/providers.yaml:430. docs/guides/docker.md:6 (Docker on the Mac).

**Waits.** Used only for development it changes no accepted experiment; counted as S3 evidence it changes README P2's wording. I9's bootstrap and I5's O1 can be rehearsed on it first.

**Verifier.** partly. The owner's words answered a fallback question (docs/proposals/ruling-72/README.md:13-18); IN_FORCE.md:226 and README P2 (:47-50) render them as 'no rented machine stands in'; ruling 68 point 4 replaced the simulated-network stage (docs/proposals/ruling-68/README.md:50-53). A local container or VM is not rented and stands in for nothing if it is reported apart. Not said in the item: the lane already queued a real-sshd tier on one machine (docs/implementation/PROVIDERS.md:50; SSH-R README:350). The citation spec/promises/placement.yaml:397-399 is off; the quoted text is at :402-404.

**Verifier's counterargument.** Used for development only, this is a test fixture and a build decision: the coordinator settles it and logs it to strike (rulings 37 and 71 point 4), and the item's own 'waits' line says it changes no accepted experiment. Asking the owner spends one of few questions on it. Only counting its cells as S3 evidence is the owner's, and the item does not ask for that. The fault with 'real content' it is wanted for, a freeze and a return with live runs and a clock jump, exists on the real computer through Windows sleep or hibernate.

**Verifier's corrections.** Reclass as a recommendation decided by the coordinator, joined to the queued loopback-sshd tier; keep for the owner only the question whether any cell from it may be reported in S3's results.


## FR-328

**S3: count what adding the machine really cost**

P05 I11 · experiment-change · verifier: partly · decides: owner

Register row: ROADMAP.md:141 (S3). Claim tested: ruling 50's 'adding machines should be much simpler with conduit', which no experiment measures today.

S3's measure M2 counts authored enrollment lines and panel actions (start, comparison, admission) with provider work apart. On that count this machine would report about 4 actions while 26 were taken before Conduit was involved (I6).

Change: add a measure, 'machine-add cost'. From a machine with only its operating system to the first run placed on it, count every action by the person: before Conduit, through Conduit (the panel, the join action) and after; mark those that need sudo or an administrator; record elapsed time and mistakes. Report it for each fresh lab's T1.

Baseline, on paper only, as experiment 023 counted S1: the same machine added with plain OpenSSH (a key copied, a host key accepted), with Tailscale SSH, and with a k3s agent join.

Pass condition: no number (ruling 46, point 3: observed time and mistakes come before any target). T1 passes only if every step a person took is in the count, and the README's line that the manual setup 'cannot pass T1' is read to mean that the manual steps are counted, not left out.

**Evidence.** experiments/014-s3-two-machines/README.md:156 (M2: 'Count authored account/enrollment lines under M0 and panel actions apart: enrollment start, evidence comparison, admission'), :138-141 ('Provider work apart'), :147-148 ('No numeric bar applies; observed time and mistakes precede any target'), :217-218 ('the guide's manual setup ... cannot pass T1 or replace it'). docs/proposals/ruling-50/README.md:9-12. experiments/023-paper-count (the paper-count precedent, listed by ls experiments).

**Waits.** An accepted measure changes, so it is the owner's; it can ride on I2's answer. S3-R1 to R4 report it.

**Verifier.** partly. M2 counts enrollment lines and panel actions only (README:156); 'cannot pass T1 or replace it' (:217-218); ruling 46.3 at :147-148; experiment 023 exists. 'No experiment measures today' needs a caveat: S10's D15 field (d) records 'operator setup, credentials, gate configuration and manual recovery' for each implementation (experiments/021-s10-swapped-providers/README.md:118-123), and SSH-R's README already fills it as 'Untested on the Windows computer' (:330).

**Verifier's counterargument.** The measure cannot be taken on this computer as specified. It was set up over two days with no time or mistake recorded, and each of the four fresh labs re-enrolls a prepared machine, so labs 2 to 4 would report about 4 or 5 actions and the first only a reconstruction from the guide (I6). A real count needs a fresh distro or the disposable node. Cheaper: extend D15 (d) to S3's machine provider with a count of person actions, and record I6's paper count once, as experiment 023 did.

**Verifier's corrections.** Say where the count is taken (a fresh distro or node, once) and that on the Windows computer it is retrospective. Reuse D15 (d) in place of a new measure. The added pass condition on T1 changes an accepted task, so the owner decides: correct.


## FR-329

**One run user: what S3 cannot claim on the second machine, and an unplanned conflict with Docker**

P05 I12 · assessment · verifier: partly · decides: coordinator

Under ruling 77 every run and every work-order worker on the second machine is conduit-run. SSH-R's README lists what follows: any run reads every other run's identity files and environment, writes into its input and output, and can leave units that escape the provider's stop.

Consequences for S3:
- V5 names PRV-025 as its safety promise and the README lists PRV-025 as exercised. On this machine, in the local profile, it is stated unmet. V5 belongs to the provider suite with a fixture; the README should say that the real runs do not show it.
- Codex B and the shared service both run there. The service's secret, delivered 'only through the authenticated runtime channel', then sits in a process the Agent can read, and the Agent's sign-in sits in a home the service can read. S2's 'no secret reaches an Agent's run', replayed on the second machine, is false by construction in the local profile, and D20 will list everything as readable. The owner's stated use of the second accounts, 'a test of agent subscription isolation and our entire model', cannot be shown between two runs on this machine under one user.
- This is no worse than the Mac, where Agents run as the engine's user, and it is better in one respect: the engine's signing key is not on the second machine.
- The isolated profile needs Docker there for a user with no sudo. The plans do not say how. Membership of the docker group is root in the distro, which undoes rulings 77's points 2 and 4 (root can probably turn interop back on; general knowledge, to verify). Rootless Docker or Podman is the existing answer and needs the owner's sudo once for the uid maps.
- SSH-R hard-codes the user ('user must be conduit-run'), so a user per Agent, the existing Unix answer that ruling 77 itself names, needs a provider change as well as the owner's sudo.

Recommendation: before S3-P, decide the isolated profile's daemon (rootless), let the bootstrap of I9 create one user per run kind where the owner allows it, and move SSH-R's user from a constant to the machine's entry. Until then S3's README states which claims the second machine does not carry.

**Evidence.** packages/provider-ssh-runtime/README.md:20-31, :224 ('user must be conduit-run'). docs/proposals/ruling-77/README.md:33-38 ('PRV-025's isolation is unmet until each Agent has a user of its own'). experiments/014-s3-two-machines/README.md:78-80, :103-107 ('Codex B and a shared service run on M2 ... deliver the service secret to M2 only through the authenticated runtime channel ... Repeat with consumer isolation using a Docker runtime'), :171, :276 ('Gates do not confine Agents; OS-user trust and readable credentials remain explicit'). docs/proposals/ruling-72/README.md:76-82 (the owner's words on the second accounts). spec/promises/providers.yaml:813-815. Command: grep -n -i 'rootless|docker group' over docs/guides/docker.md, docs/implementation/PROVIDERS.md and experiments/014-s3-two-machines gives nothing.

**Waits.** S3-P and A63-DR2b (the second daemon); the owner's sudo for uid maps or extra users; S3 README's list of promises exercised.

**Verifier.** partly. One-user consequences hold (SSH-R README:21-31; ruling 77 point 2, docs/proposals/ruling-77/README.md:33-38); the user is fixed in code (packages/provider-ssh-runtime/src/config.rs:117); Codex B and the service share the second machine (README:103-107). Wrong: 'The plans do not say how'. docs/guides/docker.md:9-17 says how: Docker Engine inside Ubuntu and 'sudo usermod -aG docker "$USER"', and S3's README tells the runs to follow that guide (:190-191). Applied to conduit-run it gives the run user root in the distro, so the conflict with ruling 77 is written into the tree, not merely unplanned; the analyst's grep missed it because the words 'docker group' do not occur. Also wrong: the quoted 'no secret reaches an Agent's run' is not in the sources. S2's P5 says 'no service secret in Agent inputs or results' (experiments/013-s2-shared-login/README.md:62), and D20 is defined to list readable credentials by location (:146), with the limit stated (S3 README:276). Nothing is 'false by construction'.

**Verifier's counterargument.** The README is already honest about readable credentials; the finding is the Docker conflict and the stale guide. Rootless Docker fits ruling 77 but its socket belongs to conduit-run, so any SSH-R run under that user controls every container; the isolated profile then isolates only if no local-profile run shares the machine during it. 'One user per run kind' changes ruling 77 point 2, which is the owner's, yet the item assigns the coordinator.

**Verifier's corrections.** Cite docs/guides/docker.md:9-17 and fix the guide before S3-O2. Replace the S2 quotation with P5's words. Decider: the coordinator for the daemon and the README's wording; the owner for more users. S3's topology has one Agent on the second machine, so 'between two runs' means the Agent and the service.


## FR-330

**Small fixes the coordinator can make now**

P05 I13 · recommendation · verifier: partly · decides: coordinator

1. The loss command. The guide pauses with wsl --terminate Ubuntu-24.04 because wsl --shutdown 'would stop every distro, Docker Desktop's too'; S3's README, plan option A and ruling 77's record still lose the machine with wsl --shutdown. Use terminate throughout (the README's wording goes with I3).
2. The owner's pause is a free fault. 'Pause' and 'resume' are a real stop and return of the machine without a declaration: exactly PLC-023's case ('A machine that sleeps, or whose network is cut, so loses no instance'). Once the computer is a node, record each real pause as an observation: what was unavailable, that nothing was retired, what a request did after resume.
3. Pause without new language. The ruling calls it 'Kubernetes' cordon and uncordon, with the owner as the operator', and the specification has no cordon. It needs none: a machine provider that reports the machine unreachable on the owner's word gets PLC-023's behavior whole. Put it in the machine provider's contract as a control, not in a promise.
4. During S3's runs the computer carries no work orders: they share the user, and a loss phase kills them. Say so in the plan's run gate.
5. Log, for the owner to strike, that 'No rented machine stands in' (IN_FORCE.md:226, the ruling's decision 3) is the coordinator's rendering of 'We can use the windows machine whenever needed'; the owner's words answered a question about a fallback.

**Evidence.** docs/guides/second-machine.md:50-51 against experiments/014-s3-two-machines/README.md:125, experiments/014-s3-two-machines/plan.md:267 and docs/proposals/ruling-77/README.md:69-71. spec/promises/placement.yaml:310 ('Nothing is retired because a machine is unreachable or omitted'), :317. docs/proposals/ruling-72/README.md:56-64 ('Kubernetes' cordon and uncordon, with the owner as the operator'), :16-18, :27-28. Command: grep -rn -i 'cordon|unschedulable|drain' spec/ gives nothing. packages/provider-ssh-runtime/README.md:20-21 ('shared by every run and the work-order workers').

**Waits.** Nothing waits; items 1 and 4 are edits to non-binding documents, item 5 an entry in the review list.

**Verifier.** partly. 1 holds as an inconsistency (docs/guides/second-machine.md:51 against README:125, :263, plan.md:267 and the guide's own :124, :132). 2 and 3: PLC-023 as quoted (spec/promises/placement.yaml:310, :317); grep for cordon, unschedulable and drain in spec/ gives nothing. 4 holds (SSH-R README:21; ruling 76 point 6 already limits ordinary workers to 'until' the computer is a node, docs/proposals/ruling-76/README.md:39-42). 5 holds: AGENTS.md 'What holds' point 2 makes a line that differs from the owner's words a defect, and IN_FORCE.md:226 goes beyond 'We can use the windows machine whenever needed'.

**Verifier's counterargument.** 1: README:263 is the owner's accepted answer O4, and docs/proposals/ruling-77/README.md:69-71 describes the pre-registration as it stood; 'throughout' would rewrite an accepted answer without the owner and falsify a record. 2: the ruled pause is a drain first ('checkpoints and stops the work running there', docs/proposals/ruling-72/README.md:59-60), so the runs are already stopped; the free fault with content is sleep. 3: PLC-023 stops nothing, and a provider that reports 'unreachable' on a person's word misstates what it observed (providers.yaml:428-430); Kubernetes' cordon is a field the operator sets on the Node, so a provider control that leaves a machine out of place.v1 is the closer form.

**Verifier's corrections.** 1: change README, plan and the guide's S3 section with the owner's answer to I2 and I3; leave ruling 77's record. 3: a 'do not place here' control, not a false state. Items 1 and 3 are therefore not 'high' confidence edits by the coordinator alone.


## What the verifier says the analyst missed

- **The admission's evidence never reaches the channel that carries secrets** The person compares a fingerprint at admission (ENR-006), but the runtime provider that must authenticate 'that machine' (PRV-011, spec/promises/providers.yaml:351-354) is told only 'the hostname and the labels' (spec/core/operations.yaml:916-917). SSH-R pins whatever host key was typed into its own table (packages/provider-ssh-runtime/contracts.yaml:229) and uses its own operator-installed key (README:225), while SSH-M is sketched with a fresh key per enrollment (docs/work-orders/117-plan-s2-s3.md:114-116). Hostname is the only join. This is a gap in the language, not only an SSH cost: class it, and let the owner decide whether run.v1 carries the admitted machine's id or key. The analyst saw the inconvenience (I7 point 2, I9 point 2) and not the break in the chain.
- **The Docker guide puts the run user in the docker group** docs/guides/docker.md:9-17 prescribes Docker Engine in Ubuntu and 'sudo usermod -aG docker "$USER"'; S3's README follows that guide (experiments/014-s3-two-machines/README.md:190-191). Under ruling 77 that makes conduit-run root in the distro. The guide predates the ruling and is stale; a coordinator's fix before S3-O2.
- **MC4-002 was answered; S3's plan still holds it** IN_FORCE.md:244-245 and docs/proposals/ruling-42/README.md:275-285 (2026-11-12): the overlap is a stated limit, and every shipped runtime provider carries out a stop it owes on a machine before starting any run there. experiments/014-s3-two-machines/plan.md:19 and :258 still say 'Owner hold'. S3 should test the shipped rule on the real machine (a start refused or delayed while a stop is owed), which no item proposes.
- **Sleep or hibernate as the loss that returns alive** Not considered as a fault means. It is PLC-023's own example (spec/promises/placement.yaml:317), happens on the real second machine, silences Windows in the tailnet, returns live runs with a clock jump, and needs two owner actions and no privilege. It answers most of I1's objection to the lost machine and removes the need for I10's node for that purpose. General knowledge of WSL2; observe it in the probe first.
- **The Mac's packet filter as the cut** A74's H6 registers 'A packet filter drop on M1' (docs/proposals/multi-machine/README.md:1026). pf on the Mac gives a real silent drop in one or both directions, driven by the coordinator, for one sudo rule on the Mac. The analyst cited H6 and then proposed a byte-dropping proxy, which cannot catch an unplanned path.
- **Who reads and types the fingerprint in S3** S3's authors are the coordinator and a fresh Codex author (README:184-186), both models. The join action and evidence may be shown only at root's own panel and never to a model (spec/promises/enrollment.yaml:73-81; spec/README.md:406-407), and the fingerprint must be read on the machine. With evidence that is an SSH host key, reading it over SSH is circular; T1 should say it is read at the Windows console and typed by the owner. No item states this.
- **PRV-025's stated bound does not cover the path that is cut** PRV-025 asks a contract to state how long the gate takes to close a stream once the connection from the enforcement point goes silent (spec/promises/providers.yaml:815-819). With the gate at the provider's end on the engine's machine that connection is loopback (packages/runtime-provider-kit/README.md:115, :127), so the 45 seconds never meets a cut between machines; SSH's 30 and 120 seconds do, and the promise does not ask for them. The analyst left this inside probe observation O2; it is a candidate gap for the coordinator's next spec order.
- **Existing means for the first real-sshd evidence** The lane queued a loopback-sshd tier for SSH-R (docs/implementation/PROVIDERS.md:50), and experiment 022's harness already drives a provider package over MCP without the engine (ROADMAP.md register). I5 prices the probe at the plan's 600-850 line stand-in crate and I10 asks the owner for a node, without using either.
- **The restricted key and the SSH-forward sign-in route** Ruling 77's setup admits the coordinator's key with 'restrict', which 'disables forwarding, PTYs and user rc' (SSH-R README:302-303), turns passwords off and takes the key out of the owner's account (docs/guides/second-machine.md:76-78, :86-89). S3's Codex sign-in and A81a rely on 'an SSH forward' (README:199-201; plan.md:143-144). Which account can forward is unstated. An inference from the tree, to settle before S3-A81a.
- **Tailscale's advice against running inside WSL2** docs/proposals/enrollment/README.md:194-196 records 'Tailscale recommends the Windows host only, not inside WSL2', and :80 that WSL2 inside Windows is 'a second node with a clashing name'. I5's O6 and I8's option (b) rest on doing exactly that and do not mention it.
- **Too many owner decisions for one area** Three questions and three owner-decided experiment changes. They reduce to two: S3's faults (means, the loss that returns alive, the terminate wording, the cost measure) as one answer; and permission for the probe's Tailscale observation. I8 needs no answer before the probe, and I10 is the coordinator's while it stays a development fixture.

## The verifier's notes

Verified at 2eb8f258 (2026-11-12), read-only, under the repository root. Read in full: the brief's named sections of docs/guides/full-review.md, docs/implementation/HANDOVER.md, AGENTS.md 'What holds', rulings 50, 72, 76 and 77, docs/guides/second-machine.md, docs/guides/docker.md, experiments/014-s3-two-machines/README.md and plan.md, IN_FORCE.md topic 10, spec/promises/enrollment.yaml, packages/provider-ssh-runtime/README.md. Read in part: placement.yaml (PLC-017, PLC-023, PLC-026), providers.yaml (PRV-011, PRV-013 to PRV-015, PRV-021, PRV-025, PRV-028), spec/core/operations.yaml (run.v1, enroll.v1, accept and remove), the enrollment proposal (sections 3 to 7), the multi-machine proposal's section 11, PROVIDERS.md status, the kit's gate section, SSH-R's backend.rs, ssh.rs and config.rs at the cited places, ruling 42's later answer, ruling 56's table, AUDIT.md Purpose and Layered claim. Recounted by command: SSH-R line counts, the 049a1 branch diff and its split, 494 manifest cases and the zero citations, the run gate's IDs, the guide's 26 actions, option A's arithmetic.

Not done: nothing was built or run; the 049a1 branch was measured, not read; rulings 51 and 52 and the private handoff note were not read; no claim about WSL2, Windows firewall, sleep, pf or Tailscale behaviour was tested, and each is marked as general knowledge where used.

Overall: the analyst's direction holds. I4 stands as written. The strongest findings survive checking: the accepted loss never leaves a live run for the owed stop, option A's count understates its cost, the probe that was to run first never ran, and the SSH pair leaves the admission unconnected to the runtime channel. Every other item needs a correction, and three matter: the Docker guide does say how Docker is installed and it conflicts with ruling 77 (I12 says the opposite); MC4-002 was answered by the owner and the items still treat it as held (I1, I3); and the count of 26 is the guide's prescription with dev tooling, WSL-only and per-engine steps in it, so '26 against 4' overstates a real gap (I6). Two recommendations overreach: holding 049a1 and 080 behind the probe (I5), and dropping H1 and H2. Two questions for the owner are not needed now (I8, I10). Two cheaper means were overlooked: sleep or hibernate for a loss that returns alive, and the Mac's packet filter for the cut.
