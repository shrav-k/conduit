# Ruling 80: the answers of 2026-11-16 on held orders, dispatch, local gates, sign-in and machines

Status: in force (ruled by the owner on 2026-11-16). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topics 6, 7, 9, 10, 13, 14 and 15 (spec pending wherever `spec/` must change). This record says why.

After [ruling 79](../ruling-79/README.md), the coordinator held some work orders for the owner's
answers. On 2026-11-16 it put the questions held since 2026-11-15 to the owner, each with options
and a recommendation:

- the drafts of SO-A and SO-B, the full review's orders 19 and 21
  ([its orders](../full-review/orders.md)), written as orders 171a, 171b, 172a and 172b;
- order 178, the store's growth, one line of measures per commit (the review's order 28), and
  order 179, A77's registration;
- the desk checks of [order 190](../../work-orders/190-desk-checks-terms-1password.md): how the
  coordinator runs the Codex workers, the second accounts, ruling 75 C3 and S10's secret store;
- orders [182](../../work-orders/182-stale-state-lines.md) and
  [184](../../work-orders/184-second-machine-docs-ruling-77.md): ruling 72's decision 3, and
  Docker Desktop on the Windows computer;
- [order 191](../../work-orders/191-log-marks-closed-settings-written.md)'s GitHub settings: the
  review's D11 and `CONDUIT_PRIVATE_NAMES` in CI;
- the old commits whose identity carries the Mac's user and host.

The owner answered through the session's question tool, by choosing an option or typing an
answer, and in chat. The owner's words, as given:

1. Order 171a, a bounded holder that repeats its own close after it succeeded: "Exempt own repeat
   (Recommended)".
2. Order 172a, F13 when an earlier attempt's late "succeeded" reaches the engine before a later
   attempt's "failed": "Unknown if any said success (Recommended)".
3. Order 172a, F13 and ruling 78 point 7's calls that change nothing: "No, exempt reads
   (Recommended)".
4. Order 172b, PND-006 after ruling 78 point 5: "Stays open (Recommended)".
5. Order 178, whether the file of commit measures is part of "its store" under ruling 78 point 17:
   "No, not the store (Recommended)".
6. Order 179, A77: "Pair on round 1 (Recommended)", an option whose description read "Up to ten
   extra Opus reads; paired reads stop before dispatch does if the allowance runs low."; "You
   decide then (Recommended)"; "Keep both, no spend compare (Recommended)".
7. Order 190, how the coordinator runs the Codex workers: "Can we dispatch it through conduit
   instead?"; then, told that it runs one order at a time, "Why can't they run in parallel as a
   fleet?"; then "Build toward the Fleet (Recommended)".
8. Order 190, the second accounts (rulings 49, 72 and 75): "Proceed as ruled".
9. Order 190, ruling 75 C3: "So I can't tunnel through remote control and sign in?"; then, told
   that remote desktop on the Agent's machine is C3 and that SSH by the owner is not Conduit
   carrying, the owner was asked whether C3 should also say plainly that the owner may sign
   Claude in over SSH, pasting the code or forwarding the client's local port, so long as
   Conduit relays nothing. The recommended option was "Yes, you may use SSH", the "you" being the
   owner. The owner selected no option and typed: "Yes, but also this is all a conduit should do,
   it should expose the login page via a conduit right?".
10. Order 190, S10's secret store ("1Password after A73, or else the file store"): "Need more
    clarification", then "Use the file store (Recommended)".
11. Order 191, D11's required status checks: "Do we really need CI or these checks at all right
    now at this stage of v0?"
12. Order 191, `CONDUIT_PRIVATE_NAMES` in CI: "For now, I'd like to understand what can be run as
    local scripts before merge rather than GitHub ci. Eventually we will have to move everything
    to ci, but for now I think it's best if we design and run our own gates and formalize them
    into GitHub processes later".
13. Then, on the local gate: "Yes, build tools/gate.py (Recommended)"; on GitHub's CI: "Keep it,
    don't wait (Recommended)"; on the Linux leg: "GitHub CI, non-blocking (Recommended)".
14. The old commits that carry the Mac's user and host: "Delete merged; keep the rest
    (Recommended)".
15. Order 182, ruling 72's decision 3 ("No rented machine stands in"): "Strike it".
16. Order 184, Docker Desktop's WSL integration for Ubuntu-24.04: "It's already turned off".
17. The owner also asked "We will still be logging in using the browser through the interface?
    What is this in reference to in regards to codex?" and "What is CONDUIT_PRIVATE_NAMES?".
18. During A5's stage 0 rerun, which the owner watched, the owner reported that a browser tab
    opened: "OpenAI / ChatGPT, closed".

Answers 1 to 7, 9 to 13 and 15 change decisions in force: a ruling's point, a line of
IN_FORCE.md, or a decision the full review logged for the owner to strike
([decisions.md](../full-review/decisions.md)) that an answer settles differently or makes the
owner's. They are this ruling, as ruling 67 has it (a number only for a new decision). Answer 8
changes no decision: it is a later answer in
[ruling 72's record](../ruling-72/README.md#later-answers), with a pointer from rulings 49 and 75.
In answer 14 the owner chose an action, which was carried out, and changed no decision in force.
Answers 16, 17 and 18 decide nothing. Answers 14, 16 and 18, and answer 17's question on
`CONDUIT_PRIVATE_NAMES`, are recorded below as facts, with three things the owner said later that
day; answer 17's question on sign-in is under "What it does not change". That sorting is the
coordinator's.

## The decisions

1. **A bounded holder's repeat of its own request is answered from its stored outcome** (answer
   1). The coordinator's fix F2, which order 171a writes, refuses under ACC-029 a request to open
   or to close that a holder of a bounded right makes naming a Conduit that does not exist. The
   coordinator found that a bounded holder that repeats its own close after the close succeeded
   would then be refused, since the closed Conduit no longer exists (LIF-035), where today the
   repeat is answered from the stored outcome (OPS-009, OPS-032). The owner chose the exemption:
   a repeat of the holder's own request is answered from its stored outcome, and F2's refusal
   does not reach it. The rest of F2 stays the coordinator's, open to the owner's strike. Order
   171a writes F2 with this exemption.
2. **F13 after a late "succeeded"** (answer 2). Ruling 78 point 6 ruled F13: where a later attempt
   is answered failed while an earlier attempt of the same invocation has no answer, the
   invocation stays unknown. While checking order 172a, the coordinator found a case F13 as ruled
   leaves: an earlier attempt's late answer "succeeded" reaches the engine before a later
   attempt's "failed", as two Streamable HTTP responses can, so every earlier attempt has an
   answer and failed would stand, although the engine holds an answer that says the effect
   occurred (OPS-003). Now "failed" stands only where no earlier attempt's answer said
   "succeeded"; otherwise the invocation stays unknown. A late answer still changes no outcome
   (OPS-013, OPS-014). Order 172a writes F13, so refined, into OPS-012 and OPS-024.
3. **F13 does not reach a call that changes nothing** (answer 3). Under ruling 78 point 7, a call
   that changes nothing ends failed when no answer comes, and a repeat is sent (OPS-024). Written
   without a scope, F13 would make such a call unknown when its repeat is answered failed while
   the first attempt has no answer. F13 does not apply to that class: the order that writes point
   7, SO-E (the review's order 52), adds the exception, so a call that changes nothing is never
   unknown, as point 7 says.
4. **A setup's pending action stays open after a reconciliation** (answer 4). Under ruling 78
   point 5, a reconciliation that records a setup as having occurred makes nothing ready, but
   PND-006 still has it answer the setup's pending action "ready". Now that action stays open
   until a later setup answers it "retried" or a retirement answers it "retired", and "ready" is
   dropped as an answer. PND-006's condition keeps holding for a setup whose outcome a
   reconciliation set, so the action is still computed. Order 172b writes it into PND-006.
5. **The store's measures file is not part of the store** (answer 5). Order 178 has the engine
   append one line of measures per commit to `commit-measures.jsonl` in the store's directory,
   once the commit is durable; the engine never reads the file back. Ruling 78 point 17 (F15) has
   every request refused when the engine cannot write or read its store. That file is not part of
   the store: a line that is not written is reported and refuses no request. In the coordinator's
   recommendation, the store is what the engine reads back, and a measure never refuses a request.
   Order 178 builds the file so, and the order that writes F15, SO-F (the review's order 53),
   keeps the file out of it.
6. **A77's registration** (answer 6). A77 comes from [ruling 48](../ruling-48/README.md), and order
   179 registers it as the full review set it out. That registration was the coordinator's; with
   these answers, the choices below are the owner's:
   - On A77's paired orders, at most ten, a fresh Opus reader reads round 1 beside the Codex
     verifier, as the review asks and order 179 registers it. Under ruling 79 the Opus reader
     reads only once the checks pass, so this costs one more Opus read for each paired order. As
     order 179 registered it and the option the owner chose stated, the paired reads stop before
     dispatch does when the Claude allowance runs low.
   - A77's result is a proposal to the owner, who decides when it comes in. It changes nothing by
     itself, and a Codex verifier still reads every round (ruling 79 point 2).
   - A77's question keeps its second part (on the same round-1 diff and brief, does a fresh Opus
     reader find the musts that the Codex verifier finds, and more?), and its done-when keeps the
     20-order rule beside "reviewed after five orders". The coordinator's spend is recorded per
     order only as a baseline, with no comparison with work the coordinator does itself.
7. **Dispatch builds toward a Fleet that runs in parallel** (answer 7). Order 190's desk check
   quoted OpenAI's documentation on ChatGPT-managed sign-in for automation: "Do not use this
   workflow for public or open-source repositories"
   ([its results](../../../experiments/010-subscription-terms/results-2026-11-16.md)). The
   coordinator runs the Codex workers on the owner's saved ChatGPT login on the Mac, not in CI,
   against this public repository; it had kept that method under rulings 48 and 70, and asked the
   owner how to run them.
   The option the owner chose: A78's orders 2 and 3 run through Conduit now, one at a time; the
   plain runner keeps the other orders meanwhile; and order 050 and the review's order 62 (a new
   conversation without restarting the engine) come next on the path, so that later orders run
   in parallel as a Fleet. This carries out [ruling 70](../ruling-70/README.md) point 5, that
   Conduit runs its own work orders as early as possible. Ruling 61 B12's slots and dispatchers
   stand, and [experiment 028](../../../experiments/028-a78-dogfood/README.md) holds A78's
   registration.
8. **C3 amended: Conduit manages the connection a Claude sign-in is completed over, and never
   carries the credential** (answer 9, and the owner's three replies below).
   [Ruling 75](../ruling-75/README.md) C3: "A Claude Agent signs in with a browser on its own
   machine" in v0, and Conduit never carries Claude's code between machines. Now Conduit manages
   the sign-in: it computes that the Agent must sign in, shows the person the login page through
   the person's Interface, and opens, limits and closes the connection, the tunnel or SSH
   session, over which the person completes it. It never reads, holds or relays the credential
   that the sign-in returns. The owner may complete a Claude Agent's sign-in by remote desktop or
   by SSH, pasting the code or forwarding the port, over a connection Conduit manages once order
   050 builds it. The managed connection is new behavior, and order 050 writes it.

   On answer 9, the recommended option, which the owner did not select, described signing in over
   SSH as "C3 stands, clarified"; the owner's typed "Yes" answered the same question. It is counted
   as an amendment because the limits in
   [`spec/README.md`](../../../spec/README.md#pending-actions-sign-in-and-the-network) change: they
   say today that those Agents sign in "only with a browser on their own machine", and that an SSH
   forward of a redirect's port "is the person's own and carries nothing through Conduit"; they are
   to say what this point says. That Conduit shows the login link through the person's Interface and
   never carries Claude's code was the existing design (75 C3; IN_FORCE.md, topic 9; for an Agent
   whose Type states carried: false, IFC-049 has the message say to open the link in a browser on
   the node's machine), not part of answer 9; the managed connection comes from the owner's replies
   below.

   No reply to the owner's question, "it should expose the login page via a conduit right?",
   reached the owner before the next question: the coordinator's next message was the dispatch
   question. Later on 2026-11-16, in its message reporting this ruling's third reading, the
   coordinator answered:

   > On C3 and 'expose the login page via a conduit': yes, that's already the design. Conduit's
   > engine passes the login link to your Interface, and never to an Agent (IN_FORCE topic 9).
   > C3 limits only the way back. After you sign in, Claude hands back a code that is itself a
   > credential, and carrying that code between machines is what Anthropic's terms call
   > intermediating. So Conduit shows you the page, and the code reaches the client either on
   > the Agent's own machine or through your own SSH session. If that isn't the reading you
   > meant, tell me and I'll correct the record.

   The owner replied three times in chat the same day. First:

   > I agree, conduit itself should not carry Claude sign in, it should only allow you to
   > complete the sign in process

   Then:

   > Conduit may still be responsible for managing the tunneling or ssh session

   The coordinator told the owner how it read this: Conduit may manage the connection, and
   whether and how it does is a design question for the sign-in work, not a promise yet. The
   owner replied:

   > No let's strengthen the wording, conduit does manage this connection that's the whole point
   > of the system for conduit to manage logins and these connections isn't it?

   That last reply sets the coordinator's "may" aside. The three replies are the owner's
   decision, as this point's first paragraph states it.
9. **S10's second store is the file store whenever A73 has not reported** (answer 10). S10's
   answer under ruling 75 D was "1Password after A73, or else the file store"
   ([S10's plan](../../../experiments/021-s10-swapped-providers/README.md#open-for-the-owner)).
   S10 uses the file store whenever A73 has not reported by the time it runs, and 1Password only
   if A73 has already reported that it works, as the option's description read: "S10 runs with
   the file store, so it never waits on A73; 1Password is used only if A73 has already reported
   it works." This settles the timing of "or else", which S10's plan read provisionally as the
   coordinator's ([its item A4](../../../experiments/021-s10-swapped-providers/plan.md#for-the-admin),
   and the review's D73).
10. **Local gates before each merge; GitHub's CI kept and not waited on** (answers 11 to 13).
    Answers 11 and 12 asked whether CI and required checks are needed at this stage of v0, and
    put local scripts before merge first, with GitHub's processes later. The coordinator then
    explained which checks exist and proposed a local gate. The decisions:
    - Before each merge, a local gate runs the checks on the coordinator's machine, and the merge
      waits for it. One work order builds it as `tools/gate.py`, with tests; as the option the
      owner chose said, the merge script and
      [METHOD_AS_RUN.md](../../implementation/METHOD_AS_RUN.md) switch to it, and later it
      becomes the GitHub workflow as is.
    - GitHub's CI keeps running on every pull request, as a free second opinion and the Linux
      leg. Merges no longer wait for it, and a run that fails after a merge is fixed in a
      follow-up. The Linux leg is GitHub's CI, and it does not block.
    - The owner did not answer order 191's two questions option by option; the coordinator
      applies these answers to them, and so settles both this way for now. D11's required status
      checks are not turned on, since a required check would make merges wait for CI; that
      follows from answer 13's "don't wait". On the private names, the coordinator had
      recommended that CI check them from a masked repository secret. Applying answers 12 and 13
      picks the other option, "No, local only", against that recommendation, and this choice is
      the coordinator's, open to the owner's strike: CI does not check the private names,
      `CONDUIT_PRIVATE_NAMES` stays unset in GitHub, and the local gate is to check the names
      before each merge once it holds them. The interim gate that the coordinator runs until
      `tools/gate.py` lands checks none yet.
11. **A rented machine may stand in** (answer 15). [Ruling 72](../ruling-72/README.md)'s decision
    3 ends "No rented machine stands in", and IN_FORCE.md topic 10 carried it. Order 182 logged
    that sentence among those the owner may strike, since the owner's words to the question on a
    fallback machine had been "We can use the windows machine whenever needed". That sentence is
    struck: a rented machine may stand in if the Windows computer is unavailable, and none is
    needed now. The rest of decision 3 stands.

## What it amends

- **Ruling 48**: A77, which its "What follows" registers (item 3): A77's question gains the
  paired-read part, its done-when the 20-order rule, and the coordinator's spend is recorded per
  order only as a baseline, with no comparison (point 6).
- **Ruling 71**: point 1, where a lane coordinator merges "once CI is green and a fresh verifier
  is clean": it merges once the local gate passes and a fresh verifier is clean (point 10).
- **Ruling 72**: decision 3's "No rented machine stands in", struck (point 11).
- **Ruling 75**: C3, amended: Conduit manages the connection a Claude sign-in is completed
  over, and never carries the credential; the owner may complete the sign-in over it by remote
  desktop or SSH (point 8); D1, for S10's answer to its question 2: the file store whenever A73
  has not reported (point 9).
- **Ruling 78**: point 5, for the setup's pending action that PND-006 computes (point 4); point
  6, F13, which keeps an invocation unknown after a late "succeeded" (point 2) and does not reach
  a call that changes nothing (point 3); point 17, F15's store, which does not include the
  measures file (point 5).
- **Ruling 79**: points 2 and 4, for A77's paired orders: an Opus read of round 1, and paired
  reads that stop before dispatch when the Claude allowance runs low (point 6).

The full review's logged decisions that these answers settle: F2, for a holder's own repeat
(point 1); D73's reading of A73's "or else" (point 9); D11, settled only by answer 13's "don't
wait", so that no status check is required on `main` (point 10); and A77's registration (point
6). The rest of each stays the coordinator's, and so do the private names in CI (point 10), open
to the owner's strike.

## What it does not change

- **The second accounts** (rulings 49, 72 point 2 and its later answers, and 75 D2 and D4): the
  owner's "Proceed as ruled" is a later answer, in
  [ruling 72's record](../ruling-72/README.md#later-answers).
- **The rest of ruling 79 point 2**: each order keeps its Opus reader once its checks pass, and
  its Codex verifier on each round, until the owner decides on A77's result.
- **Rulings 61 B12 and 70**: the path of the work orders through Conduit stands; point 7 says what
  comes next on it.
- **Ruling 64 point 3 and ruling 74 point 1**: CI keeps running on free runners, and `main` stays
  protected. Only the wait for CI goes.
- **Sign-in through the person's Interface**: asked "We will still be logging in using the
  browser through the interface? What is this in reference to in regards to codex?", the
  coordinator answered yes to the first part: Conduit's sign-in through the person's Interface
  does not change (point 8 adds the connection, which Conduit manages, over which the person
  completes a Claude Agent's sign-in). To the Codex part, the coordinator answered that order
  190's Codex question was about how the coordinator runs its Codex workers on the owner's saved
  ChatGPT login against this public repository (point 7), not about sign-in through the
  Interface, and that it does not affect Conduit's design.
- **Ruling 78 point 3.1**: in the coordinator's reading, these answers refine what was ruled and
  add no feature to v0.

## Facts recorded with it

- **The old commits.** The owner chose to delete the 17 merged branches whose commits carry the
  Mac's user and host, and to keep the rest (answer 14). They were deleted on 2026-11-16, and the
  6 unmerged branches and `main`'s history were kept. Later that day the owner asked that the six
  be rewritten too ("You can resume windows now and rewrite the branches actually"), and they were
  rewritten to the neutral work-order identity on 2026-11-16.
- **Docker Desktop.** Its WSL integration for Ubuntu-24.04 is already turned off on the Windows
  computer, the owner said.
- **`CONDUIT_PRIVATE_NAMES`.** The coordinator explained it: the optional list of private names,
  separated by commas, that `tools/check_private_paths.py` refuses in tracked files
  ([AGENTS.md](../../../AGENTS.md#checks)). The explanation also said the coordinator passed the
  list from the private notes when it ran the check locally, and the option "No, local only",
  which point 10's application picks, rested on the same claim. Later on 2026-11-16 the
  coordinator found that its local runs had used no names, only the generic patterns, and told
  the owner (point 10). The owner then asked: "Let's investigate to see whether it might be best
  to use this to test conduit secret stores etc. if not I can create this file." Whether the
  names can be held in one of Conduit's secret stores, as a test of them, is being
  investigated; if not, the owner creates the file of names.
- **The vendors' terms.** Asked to read OpenAI's Terms of Use, which order 190's desk check
  could not load (HTTP 403), and Anthropic's terms, the owner read them and said later on
  2026-11-16: "I did not find anything problematic" (point 7 and the second accounts).
- **A5's stage 0 rerun.** With the owner watching, a browser tab opened, which the owner reported
  as "OpenAI / ChatGPT, closed". The rerun's results merged in #317
  ([experiment 027's results](../../../experiments/027-a5-sign-in/results-2026-11-16.md)).

## What follows

- Order 171a goes out with point 1 written in, order 172a with point 2 and order 172b with point
  4; SO-E carries point 3, SO-F keeps to point 5, and order 050 writes point 8, when they are
  written. Orders 178 and 179 go out as registered, under points 5 and 6.
- Each change to `spec/` that this ruling decides is marked spec pending in IN_FORCE.md until the
  order that writes it lands: point 1 (order 171a), points 2 and 4 (orders 172a and 172b), point
  3 (SO-E) and point 8 (order 050).
- Order 193 builds `tools/gate.py`. Order 191's text records the local gate, the CI that does
  not block and the private names left out of CI for the local gate, before it merges.
- Three places are left out of this change on purpose, against ruling 67 point 6's one change
  for everything, because each is already in a work order's scope and two changes to the same
  lines would conflict.
  [CONTRIBUTING.md](../../../CONTRIBUTING.md#settings-this-repository-relies-on)'s "Settings this
  repository relies on" and [the tracker](../../AUDIT.md)'s D11 row are carried by
  [order 191](../../work-orders/191-log-marks-closed-settings-written.md), already written and
  read, which merges right after this ruling and cites it.
  [METHOD_AS_RUN.md](../../implementation/METHOD_AS_RUN.md)'s merge step is carried by order 193,
  which builds `tools/gate.py` and switches the merge step to it.

## Later answers

2026-11-16, later: the coordinator asked the owner, through the session's question tool, whether
point 8 should keep its wording, that Conduit "opens, limits and closes the connection", or say
only that Conduit manages it. The question as put: "Ruling 80 (now merged) records your sign-in
decision as: Conduit 'opens, limits and closes the connection' over which the person completes a
Claude sign-in. You said 'conduit does manage this connection'; 'opens, limits and closes' was my
wording. Which should the record say?" Its options: "'Manages'; how is 050's (Recommended)";
"Keep my wording". The owner chose no option and typed: "What does this wording distinction even
mean concretely? Your wording seems reasonable". Point 8's wording stands as the owner's, and no
decision changes. The owner's confirmation came in the same message as the owner's question,
before the coordinator explained anything, so the owner confirmed the wording only.

The coordinator then gave an illustration: a port forward over SSH that Conduit opens when a
sign-in is pending and closes when it ends, reaching only that Agent's machine and that login's
port. The illustration is the coordinator's and came after the owner's answer. It is not a
decision, it binds nothing, and it narrows none of point 8's routes. What "opens, limits and
closes" means concretely, for remote desktop, for pasting the code over SSH and for a forwarded
port alike, is the managed connection's design. Order 050 writes it, once the targeted review
that the owner chose the same day, which has that connection in its target, has its record and
the owner's answers ([ruling 81](../ruling-81/README.md), points 1 and 2). Point 8's routes,
remote desktop and SSH by pasting the code or forwarding the port, stand as point 8 states them.

2026-11-17: the coordinator asked the owner, in chat, three things: (1) a file of private
names, of mode 0600 and outside the repository, for the `--private-names-file` of
`tools/gate.py`; (2) whether the coordinator may set the neutral identity "Conduit work
order <work-orders@conduit.invalid>" in the main checkout's local Git configuration; (3)
optional strikes of the coordinator's calls; and it noted the live check to come. The owner
answered:
"1. Is this necessary if we do 2? 2. Agreed 3. Agreed  I can do the live session whenever you inform me. I suggest we find a durable way to proceed without a manually created private names file if possible, let me know if there are solutions".
The coordinator explained that the identity covers a commit's metadata and the names check
a file's content, proposed deriving the names on each run of the gate, and asked through
the session's question tool: "How should the merge gate check that private names (your
user name, machine names, private repo names) never land in the public repo, without a
hand-made names file?" Its options: "Gate derives them (Recommended)", described as "A
small order adds --derive-private-names to tools/gate.py: names read from this machine on
each run (OS account, host names, git identity, sibling repos, tailnet peers), kept in
memory, never printed, with already-public names and short words dropped. Tested with fake
values; works on any machine."; "Private script only"; and "Skip names check". The owner
chose "Gate derives them (Recommended)".

It settles where the private names are held, which the facts above left open: nowhere.
The gate derives them from the machine it runs on, at each run, and keeps them in memory
alone ([order 200](../../work-orders/200-gate-derived-private-names.md)). The owner makes
no file of names, and `--private-names-file` stays for a name that no source gives. That
the facts' investigation of a secret store for the names ends with it is the coordinator's
reading, open to the owner's strike. Point 10's choice that CI checks no private name
stands, and no decision of this ruling changes. The owner's "3. Agreed" and the live
session it offers change nothing here.

A fact recorded with it: on the owner's "2. Agreed", the coordinator set `user.name` to
"Conduit work order" and `user.email` to work-orders@conduit.invalid in the main
checkout's local Git configuration the same day; the global configuration was left as it
was. The method as run's squash still names the identity on its command line, and the
gate's `identity` gate still checks every commit a branch adds.
