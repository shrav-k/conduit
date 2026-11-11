Not binding. The critic of the Project model synthesis, kept as returned; the revision in [brief.md](brief.md) answers it.

severity major

Not ready to put to the owner as written. It fixes the substance of all six major gaps the earlier critic found: confinement now applies only to what flows through a Project, the owner's words are read literally, project authority ships in v0 as batch 2, five test-5 exceptions are named, `system` is in root's defaults, and setup through E3 uses the actor's own Conduit. But four new major gaps remain. (1) make.v1 slips past the appointment, deputy and secret rules, which are worded only for opening, so the H1 F3 and F5 escalations come back. (2) Confinement does not apply E1's exclusions to grants for a Project's own Agents, so the "packages retires skills" counterexample returns through a worker. (3) The standing turns that B12 gives the coordinator reach every Conduit a worker holds. That is the same "print the secret" path the recommendation uses to reject B7(b), and section 13 does not name it. (4) ACC-049's binding close has no lift operation, and it permanently blocks the fixed slot pairings after any review. There are also seven medium gaps and several minor ones. None is fatal; each has a contained fix.

Checked read-only against <home> HEAD is now a98ffe0, which adds rulings 56 and 57 after 22ce848. Ruling 56 calls a pending action's result its "answer". Ruling 57 does not bear on this. Nothing was modified or run apart from reads and scans of the vector files.

WHAT HOLDS (verified)
- Forwarding fits the spec as written:
  - ACC-016's "an instance that is not an actor and that names none reports refused" is met, because P names `serving`.
  - ACC-017 carries the act's caller.
  - ACC-034 and ACC-039 put E3 in the chain.
- access/bounds keeps all 17 outcomes. Observations 0-14 are access decisions only (stateUnchanged). Observations 15-16 close c611 through c610 on patched state, so neither ACC-044 nor B9 changes them.
- The flip scan agrees with the recommendation's counts. Exactly 35 vector steps set up an Agent through `management.set-up.v1` (agent-a 23, agent-b 10, agent-c 1, a 1). No vector inspects or retires an Agent through `management`.
- IFC-048 is free. The last ID is IFC-047, and the only removed IDs are IFC-024 and IFC-035.
- Three-part tool names fit IFC-014's limit: 31+1+31+1+63 = 127 characters.

MAJOR

M1. make.v1 gets past ACC-045, ACC-046, SEC-008 and ACC-032 (H1 F3 and F5 come back).
- The problem:
  - ACC-045 says "a request to open a Conduit that lists act.v1"; ACC-046 says "opening any Conduit whose target is" an actor. SEC-008 (spec/promises/secrets.yaml:290) and ACC-032 are also worded for open requests.
  - Section 9 authorizes make.v1 "by the bound (ACC-006, ACC-030), plus confinement where the sender is a Project".
  - The recommendation itself treats make and open as separate rule surfaces: "Only root ... makes a Conduit whose consumer is root. Applying the same rule to open.v1 is ruling C1". So a rule written for open does not reach make.
- Counterexample (H1 F3):
  1. Root gives ops-lead the bound {targets: [packages], operations: [core#act.v1]}, with make.v1 listed on the Conduit.
  2. ops-lead calls make.v1 {from: manage, consumer: ops-lead, target: packages}. That is "for itself", and not forwarded, so it is unconfined.
  3. ops-lead now acts for packages, and gains `auth` once root gives packages an E2 over it.
- The same works for F5: a delegate whose bound names the coordinator and cc#turn.v1 makes itself a turn on the coordinator and drives it.
- H1's own F1 fix said "ACC-045 checked at instantiation". That condition was lost in the synthesis.
- Fix:
  - word ACC-045, ACC-046, SEC-008 and ACC-032 as "open or make", and have ACC-048 state that every condition on opening applies to make;
  - add vectors for each.

M2. Confinement lets a Project's Agents receive what E1 forbids, so B2's counterexamples return through a worker.
- The problem:
  - PRJ-006 limits what an E1 lists (never register, register-replacement or retire-with-resources) and which operations of `access` an E2 lists. Nothing limits the operations an E2's bound names.
  - ACC-047(a) applies "only E1's operations" only when the consumer is P itself. For an Agent consumer, only (b) the target and (c) model work and get-secret apply.
- Counterexample:
  1. D administers `skills`. Its bound names skills and core#retire-with-resources.v1, and it lists make.v1.
  2. D opens an E2 for packages, as B2(a) and ACC-030 allow.
  3. packages' coordinator makes pkg-w1 a Conduit to the Pool `skills` listing retire-with-resources. Clauses (a) to (c) all pass.
  4. Through its standing turn on pkg-w1 (B12), the coordinator has pkg-w1 retire `skills`. That is the recommendation's own reason for excluding the operation ("packages would retire skills").
- The same holds for register.v1 wherever root has given one of P's Agents a Conduit to `management`, since row 1 needs both.
- Fix: ACC-047(c) forbids register.v1, register-replacement.v1 and retire-with-resources.v1 whatever the consumer, or PRJ-006 limits the operations an E2's bound may name.

M3. Standing turns give an actor every Conduit its workers hold (H1 F4), which contradicts the recommendation's reason against B7(b).
- The problem:
  - B7 rejects (b) with this counterexample: "the coordinator gives itself a turn on core-w2, which holds a root-given get-secret Conduit, and has it print the token (SEC-008 bypassed)".
  - Yet B12 has root give the coordinator standing turns on every worker.
- Concrete case: any Conduit root gives a worker is wielded by the coordinator. For example, a registry token by get-secret for pkg-w1 in packages would let packages' coordinator print it. Confinement does not touch root's own grants.
- What exists: inspection shows one hop (who can take turns on each Agent). Transitive reach is deferred to C10, and section 13 "Limits" does not name this.
- So "auth for a specific project" rests on root never giving a worker more than the Project's own bounds.
- Fix:
  - name it in section 13 and in the README limits;
  - either make opening a get-secret Conduit to an Agent that an actor can take turns on refused unless root accepts it with a named flag, or mark "reachable by <actor>" in inspection;
  - correct B7's rationale, since (a) does not prevent the attack in the recommended deployment.

M4. ACC-049 (root's close binds) has no lift operation, blocks the fixed slot pairings after any review, and leaves an exception unnamed.
- It triggers on every root close of a grant whose `sentBy` is a Project. That includes the review's own `conduit close --listed`.
- Effect on B12's fixed slots: after root closes a `trees` E2 and reviews it, the listed pair core-w1/tree-c1 stays refused even under a fresh E2 that root opens to restore the project.
- How it is lifted is unstated:
  - batch 2's core package adds act, make, keep and granted-under, but no lift;
  - if it is ruling 55 A7's operation, which a bound can withhold, then "until root lifts it" is wrong;
  - if only root may lift it, that is an unnamed test-5 exception.
- A list of refused pairs per Project is state held outside any Conduit, which rulings 27 to 29 never considered (the "list in disguise" the owner rejected for Pools, in reverse). Put it to the owner.
- The worked example misdescribes it: "while it is open, the coordinator cannot make it again".
- Fix:
  - bind only a close that root makes while the authority the grant was made through is still open, not closes made in the review;
  - name the lift operation and how it is granted;
  - show binds in the Project's inspection;
  - count the promises.

MEDIUM

D1. Under ACC-024, the "explicit cascade" root opens is permanent.
- ACC-024 says a Conduit whose consumer is root is never closed at any actor's request, "whoever makes it", root included. CON-008 closes it only when the target retires.
- LIF-021: if any shutdown is not confirmed, the Pool is not retired. A failed or abandoned cascade therefore leaves an unclosable standing retire-with-resources or retire-with-agents tool. A3(b) exists to prevent exactly that tool.
- ACC-046 makes retire-with-agents of any Project that owns an actor root's alone, so this path is the only one.
- Fix: let root close its own non-default Conduits (amend ACC-024), or make the cascade a named flag on the default operation (ruling 55 A4).

D2. Setting up Resources through an E1 is refused for any Resource with declared Conduits, and H1 F16 is not fixed.
- In step 3 of the flow (`conduit-core.work.set-up.v1`), the setup's parts are sent by P (ACC-005), so ACC-047 applies to them. Clause (a) refuses a Resource as consumer.
- The example's E2s also list no open.v1, which ACC-005 requires.
- "openedThrough = the earliest covering E2" is H1 F16, left as it was.
- Fix: name the limit (Resources with declared Conduits are set up by root), or write an exception for parts in confinement, and name the E2 used in the request.

D3. Owner's words narrowed without saying so: a non-root Interface "gives conduits to projects" only once.
- Section 9: a non-root actor whose request is not forwarded makes Conduits only for itself.
- So under B2(a), a non-root Interface can only open an E1 or E2 that `management` registered beforehand. A closed Conduit is removed (H1 F1), so each such grant can be given only once.
- Fix: allow make.v1 by a non-forwarded actor for a Project as consumer (E1 kind, and E2 under ACC-030), or state this narrowing in section 14.

D4. Actors cannot see what they granted.
- ACC-033 row 7 routes inspecting any Conduit to `management`. Ruling 53's inventory lists only what the caller may inspect. Section 11's inspection of a Project goes through `management` too.
- So the coordinator and pkg-console see no grants, provenance or marks. Closing at merge depends on remembering names returned by make.
- This is at odds with ruling 52's rule to be explicit and descriptive.
- Fix: add a row: a Conduit whose consumer is P or one of P's Agents is inspected through a Conduit to P listing inspect.v1.

D5. pkg-console in v0 depends on work no work order covers.
- The shell issues client tokens only for root (reference-model/crates/shell/src/main.rs:178, 234). It writes them to `<store>/client-tokens/<instance>` (lib.rs:280-299), which any program of the owner's OS user can read, so pkg-console's client can present as root.
- Ruling 49 leaves sign-in for anyone but root to v1.
- B4 lets one Interface act for several Projects. A model-driven console is then the same bridge B4 rejects for Agents.
- Fix: add credentials for non-root Interfaces to batch 2's shell order. Limit an Interface to one Project in v0, or name the bridge.

D6. Test-5 exceptions still unnamed:
- (6) an open whose outcome is unknown and whose `openedThrough` is closed "is reconciled only by root";
- (7) lifting an ACC-049 bind, if root alone may do it.
- Name both in section 15, or make each an operation a bound can withhold.

D7. Rulings amended without saying so:
- Ruling 51 decision 5 gives every pending action an expiry, but B9 has none. Section 15 lists only 51's peer mode.
- Ruling 30 OL5 makes "`root`'s two Conduits" permanent, and root-system is a third.
- Ruling 55 A7, if ACC-049's lift is root's alone.
- Ruling 56 point 8: a pending action's result is its "answer". Use that word.

D8. Bounds that name individual Resources break on a stable-name replacement.
- Bound items are resolved and stored as {kind, id} (spec/core/requirements.yaml:53-55, OBJ-021).
- Under ruling 55 A3, upgrading the `linear` or `main` package replaces the Resource under its name with a new ID. grant-linear and grant-shared then cover nothing, so "auth for a specific project" stops silently after the first upgrade.
- Re-making the E2 raises a B9 review each time.
- Fix: state how replacement re-binds bounds, or prefer bounds on Pools.

MINOR
- Name collisions in root's tools. Root's first-start Conduits have the local names `management` and `access`. A Pool or a Project of either name, or of the `as` of any of root's own Conduits (operator turns), collides under IFC-014, so the default is not made and the registration fails. Reserve those names, or state the rule.
- Generated names (`<consumer>--<as>` plus a number) can exceed OBJ-001's 63 characters. Shorten them as DL15 does.
- "Setup parts are the precedent" is wrong: parts are sent as the requester, never as the target. P's requests need IFC-015 ("as a client of the server the engine presents for that instance") and IFC-022 (who sent it, from what the request presents) amended for a sender inside the engine with no server or credential. Neither is in batch 2's list.
- OPS-035's reason is wrong: IFC-020 already scopes an idempotency key to the caller. The derived key is still fine.
- "Rows 5, 7 and 12 to 14 lose Agents" contradicts keeping row 12 (an instance that no longer exists goes to `management`). Only row 14 names Agents.
- The flow's "(re)setting up" of a tree is not a lifecycle operation. `manage` lists retire.v1, so one retire by the coordinator removes a fixed slot that P cannot register again. Drop retire from the coordinator's E3.
- list.v1 on `management` omits Conduits, packages, tombstones, binds and the Agents of a retired Project, which cannot be listed through a retired container. Discovery "over everything" has these gaps.
- B6 stops an actor from being retired, but any delegate can still lock an actor out by closing its Conduits (ACC-012).
- Cost: about 10 to 14 promises are uncounted: the M1 wording, the M2 clause, M4's lift and inspection, D1, the D2 exception, D4's row, the reserved names and name shortening, IFC-015 and IFC-022. So is the shell work for non-root credentials. Each S1-fixture registration step also gains default-Conduit records in its exact `records` list, beyond the ID shifts.
- Not checked: whether any vector closes or retires a delegate's Conduit to `access` while grants made through it are still open. B9's claim that "no existing outcome changes" depends on it.