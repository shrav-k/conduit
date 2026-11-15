# GL5-stated-limits: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Commit 2eb8f258 (2026-11-12). Every stated limit in spec/README.md:331-618 is supported by the promises it cites, with three exceptions, and none of the twelve files X04 left unread contradicts one.

Verdicts by section:
- **Short form:** right for v0, except README:349, which says a closed Conduit can be reopened (F4).
- **Fleets, discovery and tombstones:** right; X04/F10 has the undercounted exceptions.
- **Pending actions and the network:** as X04 found.
- **Pool bound and closing a Conduit to `access`:** right for v0; grant review is batch 2.

**The two delegation limits** are right for v0 under ruling 44, but both are stated for a version that does not exist or with a way round that does not work:
- The filter example needs batch 2's `make.v1`.
- In v0 the real exposure is every registered Conduit within the bound, whoever its consumer.
- The broker is defined for `management` only and gives no usable way round the filter limit (F1).
- For `management`, the stakes are larger than written: on the engine's own runner, `register.v1` plus a Pool's `set-up.v1` amounts to root's authority. The broker, the only narrower route, cannot register an Agent under ACC-057, and no scenario plan measures it (F2).

**The secrets section:** all 17 points are supported and right for v0 as a domain-of-trust statement in the SPIFFE and Kubernetes mould. Its one fault: it puts the decision of which secret reaches which program in root's choice of Conduits. In fact, a setup that root sends opens whatever `get-secret.v1` Conduits a template's author declared, and reports none of them (F3, marked unsafe at medium confidence).

One constraint would close both delegation limits: a list of templates on a bound and on `management`. Ruling 44 already reserves it for the owner, and it should be asked now, since the planned measurement cannot reach the Agent case.

**What the reader read.** Nothing was built or run, and nothing was written or changed.

**Read in full:**
- spec/README.md:325-625
- docs/guides/full-review.md
- AGENTS.md, "What holds"
- the first pass's X04-stated-limits.md, with its verifier's text
- critic.json's missing-area entry
- spec/promises/secrets.yaml, pools.yaml and fleets.yaml
- spec/conventions/observation.yaml and implicit.yaml
- DL13 to DL19 of spec/implementation-defined.yaml
- docs/proposals/ruling-44/README.md

**Read in part:**
- spec/conventions/access.yaml: ACC-001 to ACC-019, ACC-027 to ACC-036, ACC-053 to ACC-057, and ACC-056's table.
- spec/promises/providers.yaml: PRV-002, 003, 006, 007, 008, 010, 011, 020 to 025, 028 and 029.
- conduits.yaml: CON-004, 021, 022, 025, 028 and 031.
- MAN-012, SUB-009, INS-013, NAM-010, NAM-011, REV-007 and PLC-017.
- lifecycle.yaml: LIF-035, LIF-040, LIF-041 and part of LIF-013.
- spec/interface/implementation.yaml: IFC-007, 021, 022, 042 and 044.
- spec/core/operations.yaml: `register.v1`, `replace.v1`, `set-up.v1`, `retire.v1`, `open.v1` and `close.v1`.
- spec/promises/pending.yaml: PND-009, and PND-010's opening rule.
- spec/definitions.md: the two broker rows.
- IN_FORCE.md: topics 6, 9 and 12.

**Searched only, by grep:**
- For filter, bound, secret, credential, `management`, closed, broker and reopen: spec/promises/objects.yaml, schemas.yaml, naming.yaml, instantiation.yaml, manifest.yaml, submissions.yaml and placement.yaml, and spec/conventions/revisions.yaml. These were not read in full, so a contradiction outside those terms could be missed.
- For broker and E08: experiments/, conformance/ and ROADMAP.md.
- For novelty: the first pass's INDEX.txt and its L01, L06, L11, X06 and X10 files; docs/AUDIT.md "Known to be open"; the cold readers' known-open list; the review list; docs/proposals/projects/brief.md.

**Not read:**
- spec/promises/enrollment.yaml, templates.yaml, contracts.yaml and packages.yaml.
- Most of spec/conventions/lifecycle.yaml and operations.yaml.
- The engine, and the ruling records other than ruling 44.

I did not re-verify X04's sign-in, panel, fence or left-behind-run findings. I took their verdicts as given for those limits.


## FR-566

**The filter limit's example assumes the bound holder can make a Conduit, but in v0 it can only open one already registered, and nothing offers a way round for filters**

GL5/F1 · defect · verifier: known, low · decides: coordinator · known-new-situation

**Promises.** ACC-029, ACC-030, ACC-006, ACC-033, ACC-054, SEC-008, ACC-032, PND-009, PND-010, ACC-015, ACC-016, ACC-017, IFC-021

**Where.** spec/README.md:458

**Quote.** "So an actor that reads a directory through a Conduit that applies a filter, and that is also the consumer of a Conduit to `access` whose bound covers the directory and names its read operation, can open for itself a Conduit that reads the directory with no filter."

**Situation.** Working the example. Actor A holds C1 (A to directory D, the read operation, filter F) and C2 (A to `access`, `open.v1`, bound {targets: [D], operations: [read]}). A sends `open.v1`.

What the promises allow:
- `open.v1` opens only "a Conduit whose state is registered" (spec/core/operations.yaml:755).
- Only `register.v1` through `management` registers a Conduit (ACC-033 row 2, spec/conventions/access.yaml:599-601). The other way is as a template's declared Conduit when a holder of `management` registers its instance.
- So A gets unfiltered reads only where a holder of `management` has already registered an unfiltered A-to-D Conduit. Otherwise A can open nothing new. The sentence describes batch 2's `make.v1` ("makes the Conduit from a registered template, within the actor's bound", IN_FORCE.md:124).

What v0 really allows, which the sentence hides: A may open, for any consumer, every Conduit within the bound that is registered and not open. That includes declared Conduits of instances not yet set up. For example, A can open the unfiltered declared Conduit of an Agent that A can drive, then drive that Agent.

The exceptions are incomplete. README:455 names only ACC-054. The promises also refuse a bound holder a Conduit that lists:
- `get-secret.v1` (SEC-008, spec/promises/secrets.yaml:198-200);
- an operation reserved to root (ACC-032, access.yaml:572-574);
- `answer.v1` or `sign-in.v1` for a consumer that is not an Interface (spec/promises/pending.yaml:104-105, 169-170).
ACC-030 itself says "the operations of ACC-032 and SEC-008 apart" (access.yaml:543).

Is there a way round? The broker paragraph (README:477-487) and both definitions (spec/definitions.md:33, 187) speak of `management` only. Applied to `access`, a broker would be a Resource that holds the wide Conduit to `access` and offers an operation that opens only filtered Conduits. In v0 that broker:
- can open only filtered Conduits that are already registered, or must also hold a Conduit to `management` that cannot be narrowed (CON-028, CON-022, MAN-012) in order to register them;
- must carry Conduit's keys, because a Resource's onward request names the invocation it serves (IFC-021, spec/interface/implementation.yaml:579-585), and a plain server is sent no keys (IFC-007);
- cannot be reached through a filtered Conduit (ACC-015, ACC-017).
So the broker paragraph gives an author no usable way round. It swaps the filter limit for a hand-written, Conduit-aware server that holds `management`.

A Conduit's filters are those of its template ("applies only the filters that its own template applies", access.yaml:502-505), and `make.v1` makes from a registered template. So a bound that names the Conduit templates its holder may open or make would close this limit. That is the same focused constraint README:486-487 keeps back for `management`.

**Counterargument.** The limit is ruling 44's option a, accepted by the owner, and the README holds statements, not promises. ACC-029 and ACC-030 already say "for itself or for another consumer". The sentence becomes literally true once `make.v1` lands in batch 2. In v0, root alone holds `management` in practice, so the set of registered Conduits is root's choice.

**Least change.** Rewrite README:454-461 as: "A bound limits a right to open by Pools, targets and operations, and never by filters (ACC-029, ACC-030). In this version `open.v1` opens only a Conduit already registered, and only a holder of `management` registers one (ACC-033). Within its bound, the consumer may open, for any consumer, any registered Conduit, including a declared Conduit of an instance not yet set up, whatever filters it applies, apart from those that ACC-032, ACC-054, SEC-008, PND-009 and PND-010 reserve. Batch 2's `make.v1` extends this to any registered template." Add to the broker bullet that the same pattern over `access` also needs `management` in v0. Put to the owner, in one question with the management constraint: should `bound.v1` take a list of the Conduit templates its holder may open or make?

**Precedent.** Kubernetes RBAC's escalation check: the `bind` verb restricted by `resourceNames` lets a delegate bind only the named Roles. AWS IAM permission boundaries cap whatever a delegated administrator grants.

**Who decides.** coordinator. The wording fix adds no behavior (rulings 37 and 46 point 4). A bound that names templates adds a feature, and ruling 44 point 2 says that constraint comes back to the owner.

**Verifier.** known, low. The quote is verbatim at spec/README.md:458-461. Its points are already recorded or ruled:
(1) The example needs a registrar. X04's assessment (first pass, X04-stated-limits.md item 8) says: "In v0 `open.v1` opens only a Conduit already registered (core/operations.yaml:755), and registering one needs `management`. So the section's example needs a registrar until batch 2's `make.v1` makes it fully real", and judges the limit consistent.
(2) The README sentence restates a promise. ACC-029's observable says the same: "may open, for itself or for another consumer, a Conduit to that target that applies none" (spec/conventions/access.yaml:504-505), and so does ACC-030 (access.yaml:540-543). Both are statements of authority. Whether a registered Conduit exists is the separate precondition in open.v1 (spec/core/operations.yaml:755).
(3) The "real exposure" is already stated. README:455 says the holder "may open, for any consumer, a Conduit that lists any operation that the bound names". A declared Conduit of an instance not yet set up is simply a registered Conduit.
(4) The missing reservations (ACC-032, SEC-008, PND-009, PND-010) are the set in X04/F10 and L13/F12. Their substance is recorded at IN_FORCE.md:21.
(5) Having no way round the filter limit is the owner's accepted option a (docs/proposals/ruling-44/README.md:26-31). The broker paragraph does not claim to be a way round it.
(6) The proposed owner question is already ruled: "a bound that names the templates it covers is for teams, later (41 item 28)" (IN_FORCE.md:115).

**Verifier's corrections.** What is left is one low wording point. README:455 cites only ACC-054 ("Subject to ACC-054 for root"), while the promise it restates says "the operations of ACC-032 and SEC-008 apart" (access.yaml:543), and PND-009 and PND-010 also refuse non-Interface consumers (spec/promises/pending.yaml:104-105, 169-170). Add those citations, the coordinator's choice. If the README wants it, it may add a pointer that in v0 open.v1 opens only registered Conduits. Drop the owner question, which ruling 41 item 28 already settles. Not a defect of behavior: the README overstates the v0 exposure rather than hiding it.


## FR-567

**The broker, the only way round the management limit, cannot register an Agent, and no scenario measures it**

GL5/F2 · gap · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** ACC-030, ACC-057, ACC-033, ACC-016, IFC-021, IFC-007, CON-028, CON-022, MAN-012, PRV-003

**Where.** spec/README.md:485

**Quote.** "The scenarios S1 and S2 measure whether this is good enough; where it is clumsy, a focused constraint, such as a list of the templates that a Conduit to `management` allows, comes back to the owner."

**Situation.** Verdict on the limit itself: right for v0 under ruling 44, and every clause is in its promises:
- CON-028 (spec/promises/conduits.yaml:374-379);
- CON-022 (conduits.yaml:292-295);
- MAN-012 (spec/promises/manifest.yaml:280-281);
- ACC-030 (spec/conventions/access.yaml:543-552);
- SUB-009.

The stakes are higher than the section says. A holder of `register.v1` and of a Pool's `set-up.v1` can register a Resource that names no runtime. The engine's own runner then runs its command as the engine's operating-system user (PRV-003, DL16), and such a program "can read the engine's keys and make its proof" (README:500-502). So delegating registration in v0 is delegating root, and the broker is the only narrower route.

That route fails where Conduit building Conduit needs it:
- (a) No Agents. ACC-057 refuses every registration that would make an Agent unless root sends it on its own behalf, "an onward request serving root" included (access.yaml:897-904). So no broker can register an Agent of an approved template. That is the case ROADMAP.md:172's E08 intends: "an agent may start only an approved reviewer, through a broker".
- (b) Every broker is hand-written. A Resource's onward request must name the invocation it serves (IFC-021, spec/interface/implementation.yaml:579-585), and a plain server is sent no Conduit keys (IFC-007). So every broker is a hand-written server that speaks Conduit.
- (c) The actor cannot manage what the broker made. Inspecting and unregistering what the broker registered go through `management` (ACC-033 rows 2 and 7), and "A registration is reconciled through `management`" (access.yaml:644). The broker must offer each of these again.

No measurement exists. No plan under experiments/ builds or counts a broker. `git grep -i broker -- experiments/` finds only experiments/013-s2-shared-login/plan.md:44, a pointer to a conformance case, and experiments/012-model-checking/README.md:1065, "no broker or unsupported-filter behavior". The conformance case E08 registers a Resource, and its onward registration "is supplied by a simulated trusted adapter" whose "approval logic remains separate provider evidence" (conformance/README.md:576-578).

So the trigger in ruling 44 ("if it is clumsy ... comes back to the owner") cannot fire from S1 or S2 as planned. An author who relies on a broker to provision Agents meets the refusal only at run time.

**Counterargument.** ACC-057 is ruling 59 A1 and is recorded (IN_FORCE.md:21). Registration through a Fleet is open as ruling 62 C7. README:470-474 already says `register.v1` trusts its consumer with programs, and X04's assessment judged that known. Plans bind nothing, and S1's rerun could still add a broker task.

**Least change.** In README:477-487 and spec/definitions.md:187, add: "A broker cannot register an Agent: ACC-057 refuses every onward registration that makes one. A broker carries Conduit's keys, since its onward request names the invocation it serves (IFC-021). What it registers is inspected, reconciled and unregistered through `management`, which the actor does not hold." Replace "The scenarios S1 and S2 measure" with the experiment row that does. Alternatively, add to S1's rerun a task fixed before it runs: a Resource broker that registers instances of one approved template, with its lines and trusted code counted under D22. Put the focused constraint to the owner now, together with ruling 62 C7, because the measurement cannot show the Agent case.

**Precedent.** Kubernetes ValidatingAdmissionPolicy (CEL) narrows what a holder of `create` may create by its content, with no custom server. Kubernetes RBAC good practice warns that the right to create Pods reaches every Secret and ServiceAccount in the namespace.

**Who decides.** coordinator. Rewording a statement and changing a non-binding experiment plan add no behavior (rulings 37 and 46 point 4). The focused constraint and registration through a Fleet stay the owner's (ruling 44 point 2; ruling 62 C7).

**Verifier.** weakened, low. The quote is verbatim at spec/README.md:485-487.
Already known:
- A broker cannot register an Agent: X04's assessment item 7 ("A broker cannot register an Agent (ACC-057)... which the section does not say"); X04/F10's situation; ACC-057 at spec/conventions/access.yaml:902-904 ("an onward request serving root"); IN_FORCE.md:21; ruling 59 A1 (docs/proposals/ruling-59/README.md:20).
- register.v1 with set-up.v1 amounts to code run as the engine's user: X04's assessment item 7, and ruling 59 A1's own reason ("whoever registers an Agent chooses the program that runs as the engine's user").
- A hand-written, Conduit-aware broker is ruling 44's design ("trusted operation of a Resource"). The conformance fixture says it "does not test that adapter's approval implementation" (conformance/README.md:415-416).
Refuted in part:
- "No scenario plan measures it" is wrong. ROADMAP.md:172 (S1 row) plans "E08: an agent may start only an approved reviewer, through a broker, with its trusted code and setup counted (D04)", and docs/proposals/reaudit/round-6/DECISIONS.md:108 says "E08 is in S1". The approach critic already notes that none of the E01 to E20 cases has run (approach/critic.md:82).
- Point (c) misreads ACC-033. Inspecting a Resource goes through its owning Pool (row 6, access.yaml:610), and so do setting it up and retiring it (row 4, access.yaml:605-606). Only unregistering (row 2) and reconciling the registration (access.yaml:644) need `management`.
- The owner already holds registration through a Fleet open until after the S1 rerun (IN_FORCE.md:178, 62 C7).

**Verifier's corrections.** What survives is a stale plan and one sentence:
- E08's case comes from D04's scenario, "Let the coding agent start a reviewer from this approved template" (docs/proposals/reaudit/round-6/design-critique.md:201-204). That is the Agent case, which ACC-057 (ruling 59 A1, after ruling 44) forbids to any broker. So the planned measurement can show only a Resource broker, which is what the conformance case registers (conformance/conventions/records/broker/case.yaml:330).
- Fix, the coordinator's: restate E08 at ROADMAP.md:172 as a Resource broker, or mark its Agent case as waiting on 62 C7. Add to README:477-487 that a broker cannot register an Agent (ACC-057).
- Strike point (c).
- Do not ask the owner the focused constraint now: ruling 44 point 2 and IN_FORCE.md:115, 178 set when it comes back.
- Class: a plan inconsistency and a statement omission, not a gap in the promises.


## FR-568

**Root's setup opens the secret Conduits a template's author declared, and its result names none of them**

GL5/F3 · gap · unsafe · verifier: weakened, medium · decides: coordinator · new

**Promises.** SEC-008, SEC-010, SEC-005, ACC-005, CON-004, CON-021, CON-025, POOL-013, POOL-015, ACC-033, ACC-030, ACC-057, IFC-025, LIF-013

**Where.** spec/README.md:513

**Quote.** "An Agent's model that can run commands can read its own credential, and only the root Interface's choice of which Conduits to a secret Resource it opens limits which secret reaches an Agent's process through the engine (SEC-008)."

**Situation.** Steps to the harm:
1. An actor holds Conduits to `management` and to Pool P that list `register.v1` (ACC-033 row 1, spec/conventions/access.yaml:595-596). It registers:
   - a Resource template whose declared Conduit, with the instance as consumer, lists `get-secret.v1` to an existing secret Resource S. Registering a Conduit checks only capability and retirement (CON-021, CON-025).
   - an item that runs a program of the actor's choosing and binds S to a variable (SEC-010, README:470-474).
   - Resource R into P.
2. The actor's own setup of R through P reports refused at that part, because "the Conduit is opened at setup only where the root Interface sent the setup" (spec/promises/secrets.yaml:205-207).
3. Root then does one of three things: sets up R; sets up P where R is an entry (POOL-013, POOL-015, spec/promises/pools.yaml:165-168, 202-206); or applies the project. The part is "a request for `open.v1` that names that Conduit, made by the instance that sent the request to set up" (CON-004, spec/promises/conduits.yaml:52-55). That instance is root, so the part opens the Conduit. The engine reads S and passes the value to the actor's program (SEC-010, secrets.yaml:239-244).
4. `set-up.v1` declares no outputs (spec/core/operations.yaml:433-446), so its result carries no structuredContent (IFC-025). Nothing in it names the Conduit opened, S, or the program that received S.

Result: the template's author chose which secret reaches which program. Root's only act was "set up R", told nothing.

The same holds for an Agent template written by a `management` holder that root instantiates (ACC-057) and sets up. Under ruling 70, root's apply sets up instances whose templates Agents wrote.

The secrets section describes the trust as root's choice of Conduits (README:513-515), and on the secret's side only, through its locator (README:529-530). It never says that opening or setting up also trusts the consumer's implementation, nor that a setup opens such Conduits without naming them.

A non-binding critique raised the same confused deputy for an earlier design (docs/proposals/projects/brief.md:1766-1768: "Root's apply then opens the read"). It reached no known-open list.

**Counterargument.** Root did send the setup. Inspection shows the instance's declared Conduits and their provenance, before and after (LIF-013, ACC-044). README:470-474 already says a holder of `register.v1` is trusted with programs, so this is that limit applied. In v0 root is one person who registers most templates.

**Least change.** (1) Under ruling 71 point 4 and ruling 52: `set-up.v1`'s result carries a diagnostic that does not reject for each declared Conduit the setup opened that lists `get-secret.v1`. It names the Conduit, its target, its consumer and the implementation item that will receive the value. Plans print the same line.
(2) Rewrite README:513-515 and 529-530 as: "Opening a Conduit that lists `get-secret.v1`, or sending the setup that opens a declared one, also trusts the consumer's implementation, which whoever registered the consumer or its template chose (ACC-030), and the secret Resource's locator (SEC-005)."
(3) Ask the owner whether a setup part may open a declared `get-secret.v1` Conduit at all, or whether root opens each one by name with `open.v1`.

**Precedent.** GitHub Actions environments with required reviewers: a job that uses an environment's secrets waits until a reviewer approves that specific job. Kubernetes RBAC good practices warn that the right to create a Pod grants use of every Secret in its namespace.

**Who decides.** coordinator. (1) and (2) add an output and a sentence (ruling 71 point 4; rulings 37 and 52), logged for the owner to strike. (3) changes behavior and is the owner's.

**Verifier.** weakened, medium. The quote is verbatim at spec/README.md:513-515. The mechanism is as stated:
- A declared Conduit's open is a part made "by the instance that sent the request to set up" (spec/promises/conduits.yaml:52-55).
- "the Conduit is opened at setup only where the root Interface sent the setup" (spec/promises/secrets.yaml:205-207); the same at spec/conventions/access.yaml:94-96.
- Registering a template checks only capability and retirement (conduits.yaml:263-271, 341-349).
- set-up.v1 declares no outputs (spec/core/operations.yaml:433-446), and IFC-025 gives such a result no structuredContent (spec/interface/implementation.yaml:669-670).
But the unsafe mark does not hold:
- SEC-008 makes root's setup the authorizing act, so the README's "root's choice" is literally true.
- Root can see what it approves before setup. The inventory holds "the Conduits the instance consumes ... each in full" (LIF-013, spec/conventions/lifecycle.yaml, inventory clause), and expectedId pins the instance.
- Each open part and each read is recorded with root as caller and the setup as the invocation it serves (spec/conventions/records.yaml:60-63, 78-80).
- In v0, whoever registers the template already chooses a program that runs as the engine's user and "can read the engine's keys and make its proof" (README:470-474, 500-502). So the secret route gives that actor nothing new; it matters only under a confining runtime, and none ships in v0 (IN_FORCE.md:26).
Two of the three routes in step 3 fail:
- A Resource registered into P under row 1 is not an entry; entries are declared in the Pool's template (spec/definitions.md:136).
- "No declaration sets up an instance" (README:342-343). Apply sets up only the project's own instances (docs/implementation/plan-and-apply/README.md:144-145).
Already raised, non-binding: brief.md:1765-1768, and brief.md:1370, whose answer is that "the approver must see exactly what it approves ... ruling 53's expectedId and inventory". The result's silence is L08/F4: a successful setup's result says nothing, for the same IFC-025 reason.

**Verifier's corrections.** Not unsafe. Class: a ruling 52 reporting gap. It is known-new-situation on L08/F4: there, an optional part that failed goes unreported; here, which secret Conduits the setup opened, and for which implementation, goes unreported.
Fix (1) can share L08/F4's change. If set-up.v1 gains `diagnostics`, add a non-rejecting diagnostic for each declared Conduit it opened that lists get-secret.v1. Check it against review item 11 (docs/proposals/first-build/to-review-with-the-owner.md:47), as the L08/F4 verifier asked. This is the coordinator's under ruling 71 point 4.
Fix (2) stays the coordinator's: one README sentence saying that root's setup opens every declared get-secret.v1 Conduit of the instance, whose consumer's implementation the template's registrant chose (README:470-474).
Question (3), whether a setup part may open such a Conduit at all, is the owner's. It is not needed to close the gap.
Severity medium, not high: nothing is released without root's own request, and the records show it.


## FR-569

**The short-form limit says a closed Conduit stays closed until reopened, but a closed Conduit no longer exists**

GL5/F4 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** LIF-035, ACC-011, ACC-006, LIF-041, ACC-053, ACC-033

**Where.** spec/README.md:349

**Quote.** "An existing closed Conduit stays closed until an explicit permitted opening."

**Situation.** The promises say a closed Conduit is gone:
- LIF-035: "A Conduit is so removed when it is closed" (spec/conventions/lifecycle.yaml:723-724).
- `open.v1`: "A closed Conduit has been removed, so a request that names one ... reports invalid" (spec/core/operations.yaml:763-765).
- ACC-011 (spec/conventions/access.yaml:195-197).
No opening ever reopens one. README:342 ("opens or reopens an authored Conduit") and the `--reopen` flag (README:353) rest on the same wrong picture.

The only route back takes three requests:
1. register a new Conduit through `management` (ACC-033 row 2);
2. lift the matching tombstone (LIF-041 matches the same consumer with the same local name, or the same target and capability, lifecycle.yaml:862-864; ACC-053);
3. send `open.v1`.

Two builders of plan and apply diverge. One plans `open.v1` on the closed name and gets invalid. The other plans registration, lift and open under `--reopen`, which also needs `management` and `lift.v1` beyond `open.v1`. The limit is unsupported by the promises it sits among.

**Counterargument.** The section holds statements, not promises. "Closed" may be meant loosely as "not open", and for a registered Conduit the sentence is true. Neighbouring wording is already found: X10/F17 (the cascade sequence) and L02/F9 (narrowing needs a lift).

**Least change.** Rewrite README:349 as: "A closed Conduit is removed (LIF-035) and is never opened again. Opening the same relationship again registers a new Conduit through `management`, lifts the matching tombstone (LIF-041, ACC-053) and opens it. A Conduit that is registered and not open stays so until an explicit permitted opening." Have `--reopen` print those three requests.

**Precedent.** Kubernetes: a deleted object is gone, and recreating it makes a new object with a new UID.

**Who decides.** coordinator. A wording fix that adds no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, low. The quote is verbatim at spec/README.md:349, and the promises do say a closed Conduit is removed:
- "A Conduit is so removed when it is closed" (spec/conventions/lifecycle.yaml:723-724).
- "A closed Conduit has been removed, so a request that names one ... reports invalid" (spec/core/operations.yaml:763-765).
- spec/conventions/access.yaml:195-196.
- IN_FORCE.md:107: "Closing, which never reopens".
The point is not in the first pass: no X05 or X10 finding covers README:349 or 342.
But the claimed builder divergence is refuted:
- open.v1's own description decides the case for any builder (operations.yaml:763-765).
- The only reference for plan and apply already does lift, then register, then open: "`--reopen` lifts an outside-apply Conduit tombstone before registering/opening its desired grant" (docs/implementation/plan-and-apply/README.md:109). Its golden case prints exactly the steps access.lift.v1, management.register.v1 and access.open.v1 (docs/implementation/plan-and-apply/cases/08-t7-restore.yaml:245-250).
- The sentence's intent, that apply never silently reopens, matches LIF-041's tombstones.

**Verifier's corrections.** A real but loose wording point in a section that binds nothing; low severity, not "unsupported by its promises" in effect. Reword README:349, and "reopens" at README:342, as the finding proposes: a closed Conduit is removed and is never opened again, and reopening the relationship means lift, register and open (LIF-035, LIF-041, ACC-053). This is the coordinator's under ruling 37. Drop the claim that two plan-and-apply builders diverge.


## Further points the verifier noticed

Not verified by a second reader.

- **ROADMAP's E08 measurement plans an Agent case that ACC-057 forbids** (ROADMAP.md:172 with spec/conventions/access.yaml:897-904). ROADMAP.md:172 reads "E08: an agent may start only an approved reviewer, through a broker", from D04's Agent-template scenario (docs/proposals/reaudit/round-6/design-critique.md:201-204). ACC-057 refuses that registration even when it comes as "an onward request serving root". So the measurement that ruling 44 point 2 relies on (README:485) can show only a Resource broker. This surfaced while checking F2 and belongs with F2's correction; it is a plan point, and plans bind nothing.

## The verifier's notes

Checked at commit 2eb8f258 in the read-only tree. Nothing was built, run or written.

**What I read**
- In full: spec/README.md:331-618; ACC-005, ACC-006, ACC-011, ACC-029, ACC-030, ACC-032, ACC-033 and ACC-057; SEC-008 to SEC-010; CON-004, CON-021, CON-022, CON-025 and CON-028; POOL-013 and POOL-015; open.v1 and set-up.v1; IFC-007, IFC-021 and IFC-025 (results part); OPS-022; LIF-035; REC-001's observable; PND-009 and PND-010 (the consumer clauses).
- The first pass's X04 file in full, and the INDEX searches for every finding.
- Ruling 44's record, and round 6's D04 critique and decision.
- IN_FORCE.md lines 18-27, 77, 107, 113-130 and 178.
- The plan-and-apply reference (lines 102-160) and the E08 conformance case.

**What I did not read**
- The rest of the promise files.
- The ruling 59 record beyond its A1 row.

**Summary of verdicts**
No finding survives as stated:
- F1 is known. X04's assessment already says the example needs a registrar, and ruling 41 item 28 (IN_FORCE.md:115) already puts a bound that names templates later, for teams.
- F2 is smaller than stated. A broker that cannot register an Agent is recorded. ROADMAP.md:172 does plan the measurement. Its point (c) misreads ACC-033 row 6. What survives is that the E08 case it plans is the Agent case that ACC-057 forbids.
- F3 is real as a ruling 52 gap, but it is not unsafe in v0:
  - root's setup is the authorizing act under SEC-008, and root can inspect before it sets up;
  - the template's registrant already runs code as the engine's user;
  - L08/F4 already holds that a successful setup's result says nothing.
- F4's wording fault is real and new, but the promises and the reference tool already decide the behavior.
