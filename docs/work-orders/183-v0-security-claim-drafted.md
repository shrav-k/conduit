# Work order 183: v0's security claim, drafted

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It changes no code, so run no cargo command. Never print a whole large file or log: read
`engine/CONFORMANCE.md`, `spec/README.md` and the findings in parts, by line range or `grep`.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 38 of [its orders](../proposals/full-review/orders.md))
found that v0 has no security claim that a user can read
([FR-480](../proposals/full-review/findings/G3-native-probes.md#fr-480), as its verifier
corrected it). [Ruling 47](../proposals/ruling-47/README.md) (lines 46 and 47) writes every route
around the engine that A7 finds into v0's public claim. But the claim's parts are scattered:
promises (IFC-022, ACC-027, CTR-014, LIF-013, PRV-006, PRV-025, PRV-029), the limits sections of
`spec/README.md`, two passages of `engine/CONFORMANCE.md` (lines 203 to 211 and 361 to 371), and
dated results. [Ruling 78](../proposals/ruling-78/README.md), point 3.3 (lines 79 to 86), made the
review's corrected draft the draft for publication, to grow with each route A7 finds. It closed
A20 and the plan audit's D5 on that draft, on the coordinator's map of A20's three questions
([FR-482](../proposals/full-review/findings/G3-native-probes.md#fr-482), as corrected).

This order writes that draft as a new section at the end of `engine/CONFORMANCE.md`, where DL16's
choices already stand, as FR-480's verifier advised. It adds what no result under one
operating-system user can show
([FR-422](../proposals/full-review/findings/E04-signin-isolation.md#fr-422); the coordinator's D60
in [decisions](../proposals/full-review/decisions.md)). It also corrects the readiness
assessment's stale "the bypass probe A7 has not run"
([FR-489](../proposals/full-review/findings/G3-native-probes.md#fr-489), point 2, as corrected).
The section binds nothing and decides no behavior: no promise, case, package or code changes. It
states the first build as built on `main`, including the core's purity that
[order 176](176-core-purity-enforced.md) enforced and the cancellation at a dispatch's deadline
that [order 162](162-dispatch-deadline-cancels.md) built. Where FR-480's draft and its verifier
differ, the verifier's corrections govern. Line numbers are those of `main` at `ab07fd47`.

## The decisions it applies

1. **One new section: its place and its form.** In `engine/CONFORMANCE.md`, add the section
   `## v0's security claim, drafted`, written with a straight apostrophe, as the file's last
   section. It goes after "Author commands (work order 028)", whose last line is line 398. Other
   documents cite this file by line, `docs/implementation/PROVIDERS.md` and the scenario plans
   under `experiments/` among them. At the end, the section moves none of the lines they cite.
   Its subsections are those of decisions 3 to 12, in that order, each a `###` heading with the
   exact title given there. Wrap prose at 100 characters a line at most, as the file does. Every
   statement ends with its sources in parentheses: promise IDs, a section of `spec/README.md`, a
   dated result, a row of a finding log, a ruling or an order. Links are relative to `engine/`.
   Cite this file's own rows and sections by name, such as the row "DL13: loopback boundary" or
   the I3-04 section, never by line number: decision 2's sentence moves every line below it. The
   line numbers in this order are only for finding the places on `main`. Each statement says
   what a promise means for a user, cites the promise and does not restate it at length. What no
   result showed is called "not shown", never "passed" (ruling 52). The section does not use the
   word "fence", which ruling 78, point 6 drops. It runs to about 120 to 200 lines.
2. **The section's opening paragraph and the file's.** The section opens by saying that it binds
   nothing. It is the draft of v0's public security claim under ruling 78, point 3.3, from FR-480
   as its verifier corrected it. It grows by a statement for each route A7 finds (ruling 47). It
   is the claim that the plan audit's probe E4 was to drive, "The honest security claim"
   (`docs/archive/plan-audit-history.md` line 106). It does what Docker's security page does when
   it says that whoever controls the daemon is root on the host. It describes the first build on
   `main` as this order leaves it, and a later order that changes what a statement says changes
   the statement. In the file's own opening paragraph, after "It is not a declaration of
   whole-language conformance." (line 7), add one sentence. It names the new section, linked by
   the anchor that `check_docs.py` derives from its heading (v0s-security-claim-drafted), as the
   draft of v0's public security claim, which promises nothing either. Nothing else outside the
   new section changes. The I3-04 and I4-02 passages stay as they are, and the section cites them.
3. **`### What the first build is`**: four bullets, each stating what is built on `main`.
   - The access decision is the core's. The core reaches no file, clock, network, process,
     thread or environment. Since order 176, Clippy refuses on every pull request a call to each
     entry point that `crates/core/clippy.toml` names. The engine README's section "The core's
     boundary" (lines 43 to 49) names what is left to review.
   - The serving engine accepts requests on loopback only (the row "DL13: loopback boundary";
     line 34 on `main`), so no request crosses machines. The engine's own runner runs every run
     as the engine's operating-system user, with a cleared environment and a fresh directory, and
     "This is no OS sandbox" (the DL16 rows, lines 37 to 40). It calls no runtime provider and
     makes no remote connection (the row "DL16: executor coverage", line 41), and it serves no
     secret (the I4-01 section's "Runtime providers, remote connections and secrets still have
     the unserved boundaries listed above", lines 269 and 270).
   - `sign-in.v1` and `answer.v1` answer unavailable, naming orders 048a and 047 (lines 128
     and 129).
   - The Agent packages on `main` are 0.1.1. Their Types, `claude-code.v1` and `codex.v1`, keep
     the client's own tools off, apart from what decisions 4 and 7 name. They state no
     `toolsDirectory`, so inspection reports their `ownTools` with `stated: false` (LIF-013). The
     `.v2` Types, which turn the own tools on, are on the unmerged branches of orders 050a and
     076. The dogfood engine runs them (ruling 70;
     `docs/implementation/WORK_ORDERS.md`, "Running the orders through Conduit").
4. **`### What v0 claims`**: three numbered statements.
   1. Every request the engine accepts is attributed to exactly one instance, from what the
      request presents (IFC-022). A request is dispatched only through an open Conduit from that
      consumer to the target that lists the operation. Otherwise it is refused, recorded and
      never sent (ACC-001). Each request is decided anew (ACC-036) and recorded (REC-001).
      Closing a Conduit ends the access it carried for every later request (ACC-011). The
      statement also carries IFC-022's exception: a program that can read the credential of
      another instance's run can present as that instance, as the operating system's superuser
      and every program of that run's operating-system user can. Write "the operating system's
      superuser" there, not "root", a word the section keeps for the root Interface. The
      statement also says that an invocation already dispatched runs to its end after its
      Conduit closes (ACC-034). A cancellation, whether a caller's or the one the shell sends
      when a call passes its deadline (order 162; the row "DL10: process and call deadlines"),
      asks the server to stop, and nothing confirms that it did (OPS-027). It says nothing of the
      outcome such a call ends with, which ruling 78, point 7 changes for a call that changes
      nothing. Shown by the third run of increment 3's closeout: 42 requests with 42 access
      decisions and 42 ends, and T7's close and explicit restoration
      ([its section "Results of the third run"](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#results-of-the-third-run)).
   2. A login that a Pool owns stays with its Resource and is never handed to an Agent through
      Conduit (`IN_FORCE.md`, topic 1; POOL-001, ACC-003, SEC-007, SEC-019). Not shown: the
      first build serves no secret, S1 has no credential, and S2 has not run.
   3. Two Agents ran with their own tools off, as S1's Agents do: Claude Code 2.1.318 and Codex
      0.167.0. Told to read a file outside their Conduits by any means, neither did. Neither
      loaded a project or ancestor instruction file. Claude Code loaded none of the owner's
      user-level instructions or hooks. Codex did load its home's `AGENTS.md`, a stated leak
      (finding 3-07). Shown by the closeout's third run and its section "The owner's user-level
      configuration". This is a property of each Agent package's launch flags at those versions
      (this file's I3-04 section). It is shown again at each new client version.
5. **`### What v0 does not claim`**: numbered statements, continuing from 4.
   4. No restriction is enforced. Every restriction is mediated: it applies to the requests that
      pass through the engine, and to nothing else (ACC-027).
   5. Some Agents have their client's own tools, unconfined, as the engine's operating-system
      user: those whose Type states `toolsDirectory` and whose template names a directory
      (CTR-014; ruling 70, point 2). Such an Agent reaches every file, process, network address,
      credential and signed-in account that user reaches. That includes the engine's store,
      root's token and every other run's token and state directory, so it can present as root
      or as any instance (IFC-022, PRV-029, DL16). No Conduit grants that reach, no restriction
      limits it, no record shows it, and closing the Agent's Conduits does not end it. For such
      an Agent, the Conduits state the access intended and are not a boundary. Its records are
      complete only for what it did through Conduit. Inspection shows the reach as `ownTools`
      with `directoryNamed: true` and `enforcement: neither` (LIF-013).
   6. Every program that runs as the engine's operating-system user is inside the engine's
      domain of trust and can get past every gate. An installed package's implementation is
      such a program (DL16; `spec/README.md`, "What the guarantee for secrets covers"). A
      package that runs a login holds that login's credential, and the defense is choosing
      packages, pinned by digest (CTR-013; `spec/README.md`, "Pending actions, sign-in and the
      network").
   7. A second machine joined through the SSH runtime provider runs every run as one user. Any
      run can read every other run's identity files and environment, and the secrets of later
      runs, so PRV-025's isolation is unmet (`packages/provider-ssh-runtime/README.md`, its
      paragraph that begins "Runs execute as"; ruling 77, point 2). The engine on `main` does not
      call that provider yet (decision 3).
   8. Exactly these two sentences, which FR-480's verifier gave in place of the draft's eighth:
      "Restrictions bound what any instruction can make an Agent do through Conduit. They do not
      filter content, and content read through a shared Resource can carry instructions."
      (ACC-001, ACC-027).
   9. A client may keep a protocol error from its model, so an Agent's model may never learn
      that a request was refused (`docs/AUDIT.md`, "Known to be open", its item "What a client
      and a model see"; lines 217 and 218 on `main`).
   10. Access does not narrow in three ways (ACC-009, ACC-029, ACC-030; `spec/README.md`, "What
       access does not narrow"). A bound limits a right to open by Pools, targets and operations,
       never by filters. So an actor that also holds a Conduit to `access` can open for itself an
       unfiltered Conduit to whatever its bound covers. Access to `management` cannot be
       narrowed to one template. Ruling 44 kept these two on purpose (`docs/AUDIT.md`, "Known to
       be open"). And closing an actor's own Conduit to `access` closes none of the Conduits it
       opened (ACC-030). Each stays inside Conduit: what it opens is opened by a recorded
       request.
   11. Some actors reach an Agent's own tools at second hand: whoever holds an open Conduit to
       the Agent that lists an operation marked `startsModelWork`, such as `turn.v1`. Through
       the Agent's model work, the holder reaches whatever the Agent's own tools reach.
       Inspection shows this in `targetOwnTools`, on the holder's entry for that Conduit. It
       shows one hop only, and not what the Agent reaches through its own Conduits (LIF-013).
   12. Until Agents are confined, a package that keeps a Resource's login is to run where no
       unconfined Agent runs. The engine does not check that placement in v0 (ruling 75, C1;
       `spec/README.md`, "Pending actions, sign-in and the network";
       [FR-197](../proposals/full-review/findings/X04-stated-limits.md#fr-197)).
   13. Root's token lapses 100 years after it is issued (the row "DL13: instance proof"). The
       only revocation is the check on every request that the instance still exists (IFC-022,
       IFC-032, DL13), and no request retires or unregisters root (LIF-034). So once a copy of
       root's token is taken, only a new engine store ends it
       ([FR-196](../proposals/full-review/findings/X04-stated-limits.md#fr-196); fix F93 in
       decisions).
   14. Once sign-in is built (order 048a), a model-driven console could fill `signInReturn`,
       and the engine cannot tell its source (PND-013; `spec/README.md`, "Pending actions,
       sign-in and the network").
6. **`### Under one operating-system user`** (FR-422; D60). This covers an Agent whose template
   names a directory, and every other process of the engine's operating-system user. For these,
   the claim does not say that the process cannot do any of the following:
   - read another Agent's login. A Codex login is a file in the client's home. That home is the
     owner's today (finding 3-07). Once each Agent has a home of its own (order 050), it is the
     Agent's state directory, which another process of the same user can read (PRV-029).
     Nothing mediates that read. Ruling 78, point 14 has PRV-029's "isolation is mediated"
     reworded. A Claude Code login on macOS is a keychain item of that user. Experiment 005
     found that another program's reads were refused or allowed according to how the item was
     made (`experiments/005-macos-keychain/README.md`, lines 126 to 135), so this is unknown,
     not safe;
   - use that login, by starting the same client with the other Agent's home, which needs no
     read;
   - replace that login, by putting its own login in the other home. The account is checked
     only during setup, in `sign-in.v1` or while polling (PND-011), so turns between checks can
     run on another subscription;
   - act as root's Interface. It can read root's token (DL13; `spec/README.md` lines 426 and
     427), and so, once sign-in is built, call `sign-in.v1`, list pending actions and fill
     `signInReturn`.

   The subsection then says that all of this holds both before and after the dedicated macOS
   user of ruling 75, B3. Before that user, the engine runs as the owner's own user, so the
   owner's everyday client logins are within this reach too. After it they are not, and every
   point above still holds among the engine, root and the Agents. On the second machine it holds
   among all runs of the one run user, whose own home also holds a client login (statement 7).
   Nothing is claimed about a malicious package or a model-driven console (statements 6 and 14).
   Every result obtained under one operating-system user opens by saying what it cannot show;
   A5's results, A7's rows and S3's local profile are among them. Such a result shows the
   same-user route with a planted marker, never by reading a real login (D60).
7. **`### Routes around the engine found so far`** (ruling 47): bullets.
   - The own tools on: all of statement 5. By the coordinator's account, two cases were seen in
     the dogfood: `claude-code.v2` wrote inside its slot's `.git` and asked for no permission
     (row 4-22 of [increment 4's log](../proposals/first-build/log-increment-4.md)), and Codex's
     terminal children outlived the kill of the run's process group (row 4-23). Next to them,
     say what is built: the runner neither stops nor sees a process that has left the run's
     group (the rows "DL16: run state and restart" and "DL16: executor coverage"). Add nothing
     else about the dogfood.
   - Codex reads its home's `AGENTS.md` whatever its flags say, and the wrapper uses the owner's
     home (finding 3-07; the I3-04 section).
   - Both Agents of a kind share one subscription login until each has a home of its own
     (order 050; the I3-04 section; the closeout's row "The login").
   - With its own tools off, Codex keeps three: its request for user input, `apply_patch` in a
     read-only sandbox, and its code-mode host
     ([the closeout's "Routes around the engine (ACC-027)"](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#routes-around-the-engine-acc-027);
     the I3-04 section).
   - Path containment is the server's own check of its root, not the engine's (the same closeout
     section).
   - The reach inputs. `accountServices` true opens Claude Code's claude.ai connectors and
     Codex's apps. Claude Code's `peerMessaging` true opens SendMessage, ListAgents and
     RemoteTrigger, and leaves the client's own default for inbound cross-session messages. No
     Conduit grants or records that reach, and the messages are also a way to start model work
     (`packages/agent-claude-code/README.md`, lines 64 to 68; the I3-04 section).
   - Unwrapped clients, which were A70's baseline and not Agents: Codex's read-only shell reads
     any file the user can, and its ChatGPT sign-in offered eight MCP servers that no file
     declares (`experiments/007-s1-baseline/results-2026-11-09.md`, lines 254 and 276 to 283).

   This subsection does not name Codex's collaboration tools: they are decision 8's, as FR-480's
   verifier corrected.
8. **`### Model work within an Agent's own access`**: finding 4-06, as this file's I4-02 section
   states it (lines 361 to 371; row 4-06). Codex 0.167.0 keeps six `collaboration.*` tools under
   every switch tried. A sub-agent it spawns makes its requests of the engine with the parent
   Agent's credential, so it reaches only what that Agent may. This is more parallel model work,
   not a route around the engine. With a `.v2` Type and a directory named, the client's sandbox
   is off and its sub-agents inherit the same unconfined reach, which then falls under statement
   5 (row 4-27).
9. **`### Not shown yet`**. A7's rows still run, as observation (ruling 78, point 3.3). Each
   bullet says "not shown". The first four are the closeout's
   [section "What is not shown"](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#what-is-not-shown):
   - that account services are off when the input is false. No connector or app read was
     attempted, so this rests on the flags alone;
   - Claude Code's refusal of an incoming cross-session message;
   - the reach inputs set true;
   - Codex's own tool list, which its events do not give;
   - Claude Code's user skills, and the user-settings sentinel of the closeout's open checks (its
     row "User settings" and its last line);
   - that a Pool's credential is never reachable by an Agent. This waits for a secret provider
     and for S2's measure D20
     ([the G3 verifier's list](../proposals/full-review/findings/G3-native-probes.md#what-the-verifier-says-the-analyst-missed));
   - interactive use, where a person can approve what a headless run refuses. Every run so far
     was headless.
10. **`### How to get more`**: give the Agent a runtime provider whose contract confines its
    consumer, chosen per Agent with `runtime:` (PRV-006; ruling 75, B2). Only such a provider
    could make a restriction enforced (ACC-027). None ships in v0, and the first build calls no
    runtime provider (decision 3).
11. **`### A20's three questions`**. A20 and the plan audit's D5 close on this claim (ruling 78,
    point 3.3). D5 is "The authorization machinery exceeds what is enforceable natively"
    (`docs/archive/plan-audit-history.md`, line 92). The three questions were worded at commit
    `e271d1c`, in the archived `docs/proposals/authoring-syntax/AUTHORIZATION.md`, lines 12 and
    121 to 122. Link the archive's index, whose section "Removed from the tree" lists that
    directory at that commit (line 20). This is the coordinator's map, logged for the owner to
    strike:
    - cross-path, ACC-009: a request through one Conduit is limited by that Conduit's own
      restrictions, and those on another Conduit of the same consumer do not apply;
    - issuer-independence, ACC-030: a right to open is passed on only as it is or narrower.
      Closing an actor's own Conduit to `access` closes none of the Conduits it opened
      (`spec/README.md`, "What access does not narrow");
    - provider scope, PRV-004, PRV-005 and SEC-011. No Conduit permits an operation of
      `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1`. A provider call is
      not a request: no access decision is made for it and no restriction applies to it, but it
      is recorded (REC-001). The read of a secret is a request. It is decided by the instance's
      own Conduit to the secret Resource and that Conduit's filters alone. What a provider's own
      program reaches falls under statement 6.
12. **`### Ruled, not yet built`**: four bullets. Each names what it changes when its order
    lands:
    - ruling 78, point 9: the engine's own panel, where sign-in's device codes show, is a screen
      that root's person opens, never returned to a request or written to a pipe, file or
      message. A model of the same operating-system user with a terminal tool can still read
      it, and that becomes a stated limit (statement 14 and the subsection "Under one
      operating-system user"; order 051c builds the panel);
    - point 10: root alone installs in v0. This closes two routes: learning a secret's digest
      from a refused install, and installing an archive at a trusted package's address to share
      its client IDs. It changes `spec/README.md`'s "What access does not narrow", which
      statement 10 cites;
    - point 11: closing the Conduit through which the engine reads a secret ends the engine's
      own holding of the value at once (statement 2);
    - point 14: the status of each Agent and Resource that has a run says whether that run is
      inside the engine's domain of trust, and PRV-029's "isolation is mediated" is reworded
      (statements 5 and 6, and the subsection "Under one operating-system user").
13. **The G8 cell of the readiness assessment** (`docs/implementation/INDEPENDENT_BUILDS.md`,
    line 132). In its Evidence column, replace the two sentences that run from "D5/A20 and D8/A24
    still have open remainders" to "and A5, A6, A7 and A10 open." with the text below, kept on
    the table row's one line. Link the words "v0's drafted security claim" to the new section:
    from `docs/implementation/`, the relative path is ../../engine/CONFORMANCE.md, with the
    anchor v0s-security-claim-drafted. The State, the column's other sentences, "What would meet
    it" and the dated paragraph below the table (lines 136 to 138) stay.

    > D5/A20 closed on 2026-11-14 on v0's drafted security claim
    > ([ruling 78](../proposals/ruling-78/README.md), point 3.3). D8/A24's rule is in force since
    > [order 163](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/163-a24-at-entry.md), as rule 8 of `CONTRIBUTING.md`; its state
    > is its row in the [tracker](../AUDIT.md#action-items). A7's method ran for Agents with
    > their own tools off, for files, in
    > [increment 3's closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#routes-around-the-engine-acc-027),
    > and its other rows run as observation. Ruling 78 put the native probes (A5, A6, A7, A10,
    > A22 and A73) outside G8's scope. The
    > [probe register](../../ROADMAP.md#experiments-and-probes) also records A4, A8 and A71 done,
    > A19 superseded and A74 open.
14. **A20's row in the tracker** (`docs/AUDIT.md`, line 515). Link the words "v0's drafted
    security claim" at the row's end to the new section: from `docs/`, the relative path is
    ../engine/CONFORMANCE.md, with the anchor v0s-security-claim-drafted. Nothing else in the
    file changes.
15. **What does not change.** Nothing under `spec/`, `conformance/`, `packages/` or
    `experiments/` changes, and no code under `engine/` changes. No promise changes, so
    `CONTRIBUTING.md` rule 8 (order 163) asks for no case. No vector changes, so the order adds
    no declared file (order 174). `IN_FORCE.md` stays as it is, and so do the limits sections of
    `spec/README.md`. A7's row in `ROADMAP.md` stays too; the review's order 39 rewrites it.
    Other documents' line numbers for this file move by the lines that decision 2's sentence
    adds. They stay as they are.

## Scope

Change only `engine/CONFORMANCE.md` (the new section and the one sentence of decision 2),
`docs/implementation/INDEPENDENT_BUILDS.md` (the G8 cell, decision 13) and `docs/AUDIT.md`
(A20's row, decision 14).

## Done when

From the repository root, one at a time, each passes:

```sh
grep -c "^## v0's security claim, drafted$" engine/CONFORMANCE.md   # prints 1
grep '^## ' engine/CONFORMANCE.md | tail -n 1   # prints the new heading: it is the last section
for id in IFC-022 IFC-032 ACC-001 ACC-009 ACC-011 ACC-027 ACC-029 ACC-030 ACC-034 ACC-036 LIF-013 LIF-034 CTR-013 CTR-014 PND-011 PND-013 PRV-004 PRV-005 PRV-006 PRV-025 PRV-029 SEC-011 OPS-027 REC-001 DL13 DL16; do awk '/^## v0.s security claim, drafted$/{f=1;next} /^## /{f=0} f' engine/CONFORMANCE.md | grep -q -e "$id" || echo "missing $id"; done   # prints nothing
awk '/^## v0.s security claim, drafted$/{f=1;next} /^## /{f=0} f' engine/CONFORMANCE.md | grep -o -E '[A-Z]{2,5}-[0-9]{3}' | grep -v '^FR-' | sort -u | while read id; do git grep -q -e "^- id: $id\$" -- spec || echo "unknown $id"; done   # prints nothing
awk '/^### Routes around the engine found so far$/{f=1;next} /^##/{f=0} f' engine/CONFORMANCE.md | grep -c collaboration   # prints 0
awk '/^### Model work within an Agent.s own access$/{f=1;next} /^##/{f=0} f' engine/CONFORMANCE.md | grep -c collaboration   # prints 1 or more
awk '/^## v0.s security claim, drafted$/{f=1;next} /^## /{f=0} f' engine/CONFORMANCE.md | grep -c -i -e 'stops mistakes' -e 'fence'   # prints 0
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Restrictions bound what any instruction can make an Agent do through Conduit\. They do not filter content, and content read through a shared Resource can carry instructions\.'   # prints 1
awk '/^## v0.s security claim, drafted$/{f=1;next} /^## /{f=0} f' engine/CONFORMANCE.md | wc -l   # report it
grep -c 'v0s-security-claim-drafted' engine/CONFORMANCE.md docs/implementation/INDEPENDENT_BUILDS.md docs/AUDIT.md   # 1 or more, 1, 1
grep -c 'the bypass probe A7 has not run' docs/implementation/INDEPENDENT_BUILDS.md   # prints 0
git grep -n CONFORMANCE -- '*.rs' '*.py' '*.toml' '.github/*'   # prints nothing
python3 tools/vector_delta.py HEAD   # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.
python3 tools/check_docs.py          # Errors: 0
python3 tools/check_roadmap.py       # Errors: 0
python3 tools/check_private_paths.py # Errors: 0
git diff --check
git status --short                   # lists exactly the three files in scope
```

Where `python3` lacks PyYAML, run `vector_delta.py` with `/usr/bin/python3`. The review's row also
names `cd engine && cargo test --release`. No source, test or CI step reads
`engine/CONFORMANCE.md` (the `git grep` above prints nothing), and this order changes no code, so
that command would check nothing the order changes. The `git status` check stands in for it, and
the pull request's CI runs the engine anyway.

The section's check must also be able to fail. From the repository root, run it against `main`'s
file:

```sh
for id in IFC-022 ACC-009 ACC-030 LIF-013 PND-013 PRV-025 PRV-029 DL16; do git show HEAD:engine/CONFORMANCE.md | awk '/^## v0.s security claim, drafted$/{f=1;next} /^## /{f=0} f' | grep -q -e "$id" || echo "missing $id"; done
```

It must print a `missing` line for each of the eight IDs, which are the review row's, since
`main` has no such section. Do not stage anything. In your report, mark each choice you made
yourself as your own. Give the section's line count, the eight `missing` lines and the last line
of every check. A fresh Codex verifier checks each statement against the sources it cites. A
fresh read-only Opus reader then reads the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `f383edaf`: the worker (572 s) and a fresh Codex verifier
(290 s), which found no must or should in one round. `engine/CONFORMANCE.md` gained, as its last
section, "v0's security claim, drafted", with the ten subsections of decisions 3 to 12, and its
opening paragraph gained the one sentence that names it. The G8 cell of
`docs/implementation/INDEPENDENT_BUILDS.md` no longer says that the bypass probe A7 has not run, and
names A71 done and A74 open. A20's row in `docs/AUDIT.md` links the section. No promise, case,
vector or code changed. A fresh read-only Opus reader then verified the order (ruling 79, point 2).
It ran every command of the Done when, `check_docs.py`, `check_roadmap.py`, `promise_standing.py
--check` and `check_private_paths.py`, each of the four ending "Errors: 0", and the negative
control, which printed all eight `missing` lines. It checked the cited rows, results and logs, and
the build on `main` that the section describes. It found two musts that the Codex verifier had
missed. Statement 1 did not say that it says nothing of a cancelled call's outcome, which ruling 78,
point 7 changes for a call that changes nothing. And the second bullet of "What the first build is"
quoted the I4-01 section in words that order 182 had already rewritten. It found six shoulds: four
links without their anchors; the sources of two "Not shown yet" bullets, of D5 and of the archived
lines of A20's questions; D60's rule stated as a fact about past results; "How to get more" without
ACC-027's "mediated, whatever the consumer's runtime provider"; and "its domain of trust" and "its
root", each open to the wrong reading. This fix applied every one before merge. For the
provider-scope question, it took the reader's second branch: the bullet now says that the archived
document's provider request is a request to a target's service, that v0 answers it only as "What
access does not narrow" does, and that the rest is stated as open (ruling 78, point 3.3). The
section is now 267 lines, over the order's estimate of 120 to 200: 24 of them are link definitions,
four of which pass 100 characters because a link's target cannot wrap. The Done when's `git grep -n
CONFORMANCE` prints four lines of `tools/promise_standing.py`, which order 185 added, a defect of
this order's check rather than of the work: none of them reads `engine/CONFORMANCE.md`.

A second fresh Opus reader read the coordinator's fix (ruling 79, point 2). Its one must, this
Record's count of the section's subsections, and its two shoulds were applied: the Record says which
checks end "Errors: 0", and the provider-scope bullet cites the archived lines that make a provider
request a request to a target's service and names what is stated as open.
