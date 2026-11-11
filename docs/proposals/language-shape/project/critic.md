Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

severity major

The skeleton holds up: one home per object, a computed inventory, audiences from a Pool to projects, and no nesting. It is not ready to put to the owner as written. It reverses three ruled decisions without citing them (ruling 30 OL2, ruling 44 point 2, and the v0 scope of rulings 15, 49 and 50). An existing conformance vector disproves its claim that no routing outcome flips. Its approvals are tangled into stages B and C, so the owner cannot drop them cleanly. Its engine-served approval page invents identity and notification infrastructure, against ruling 40. Severity: major. None of the gaps is fatal, and each has a concrete fix.

Gaps, most severe first. Each gives the severity, the problem, the evidence and what to do. All checks were made against <home> read only.

A. RULINGS CONTRADICTED OR NOT CITED

1. MAJOR. Ruling 30 OL2 is reversed without being named.
- The owner ruled "one authority for an object's whole life: its owning Pool for a Resource, management for everything else" (docs/proposals/conventions/README.md:52). Its own rationale cites "Azure and Kubernetes scope management rights by group or namespace", which is the same idea as a Project.
- ACC-033 builds on it: "at any moment one target authorizes a request".
- The recommendation's rows R5, R7 and R10 give two or three alternative authorizers (the Pool X, the Project P, or management). It says it replaces ACC-033's single authorizer but never cites OL2.
- Do: put the amendment to the owner as its own decision. Or keep one authority by making P the sole authority over its members, with root reaching each Project through a derived, visible, permanent Conduit (P4's option 4c). That option also fixes gap 23.

2. MAJOR. Ruling 44 point 2 (D03/D04) is reversed early and without comparison.
- The ruling says v0 states plainly that a bound does not limit filters, and that access to management cannot be narrowed to one template. Narrow provisioning goes through a broker.
- The ruling also says: "S1 and S2 measure whether that is good enough; if it is clumsy, a focused constraint, such as a list of allowed templates on access to management, comes back to the owner."
- `project-bound.v1`'s `templates` dimension and the audience `requirements` field reverse both points. S2 has not run, and the broker (the spec's own answer in spec/README.md, "What access does not narrow") is never compared.
- Do:
  - cite ruling 44 point 2;
  - add a per-project broker option to decisions 11 and 20;
  - say that the S2 evidence is missing;
  - define "carries" for required restrictions (exact requirement and arguments).

3. MAJOR. The v0 scope breaks rulings 15, 49 and 50.
- The purpose (ruling 15) says "Teams: not in v0".
- Ruling 49 point 2: "more granular in the future… fleshing out interfaces more will likely be a lot of the work for v1."
- Ruling 50: every pending action is "raised at the root Interface in v0".
- Stages B to D (collab, approvals routed to collab, a phone panel) are team and v1 features, but they are scheduled as v0 work orders. Decision 1 lacks the obvious option (d): stage A (memory, names, inventory, tombstones, `retire-with-members`) in v0, and B to D proposed for v1.
- The plan never places the stages against ROADMAP steps 3 to 7 (increment 4, the round-7 read, the S2–S9 fan-out, the readiness gate).
- Do: add option (d), place each stage in the roadmap, and ask the owner to amend rulings 15, 49 and 50 explicitly if B to D stay in v0.

4. MAJOR. The breach of preservation test 5 is wider than stated. The test is "every management action is available to an agent under a grant", and the purpose says "Agents can operate the system… through the same contract a human uses".
- Only approvers make envelopes, and Agents never hold `approve.v1`. So no Agent can ever open a Conduit to a Project: agents cannot delegate project authority at all.
- Fix 2 makes every open of a Conduit to a Pool, `management` or `access` with an Agent consumer wait for a person, at engine level too.
- "The actor's person" is undefined when the actor is an Agent.
- Do: list every exception, or scope person confirmation to Conduits to Projects and keep ACC-030 delegation among Agents intact.

5. MAJOR (verified). The claim that no routing outcome flips is false.
- conformance/conventions/access/bounds, observation 3: agent-a opens a Conduit to Pool …116 through its bounded `access`. Permitted today.
- Observations 9 and 10: agent-a opens a Conduit to `access` for itself. Permitted today.
- All three become pending or refused under fix 2.
- Two more rules change engine-level behaviour, contradicting "engine-level objects keep today's rules exactly":
  - "an Interface consumer's half is the Interface itself or an unbounded `access`" narrows ACC-030's "for any consumer";
  - "only root opens a Conduit whose consumer is root" is new.
- Do: scope these rules to opens that touch a Project, or list the flips, re-expect the vectors and reword the regression gate.

6. MAJOR. Ruling 40 and the current direction (identity is a later provider): the approval page is new infrastructure.
- Ruling 49's URL elicitation points to a third party's page, which does its own authentication. An engine-served approval page must instead:
  - authenticate a person;
  - bind the click to a `person.v1` Interface (DL13 issues tokens to instances, not to browsers);
  - resist forwarding and cross-site request forgery.
- Unauthenticated, the URL is a bearer capability: any model or transcript that sees it can approve.
- Delivery is missing too. An MCP elicitation reaches only a connected client during a request. Nothing pushes to an idle phone, and a laptop engine is unreachable from a phone without network exposure.
- Do: put person authentication (OIDC or WebAuthn user verification) and notification behind provider contracts, specify the binding, and count the cost. Or keep v0 approvals in the Interface's own client.

7. MAJOR. The approvals cannot be dropped "without unwinding stages A to C" (risk 4, decision 10).
- Stage B's delegation rule needs `open.v1` AND `approve.v1` on P.
- The roles `project-admin` and `project-approver`, `person.v1`, person-confirmed envelopes and the `approvers:` key all depend on `approve.v1`.
- Do: give stage B a delegation rule that stands alone (`open.v1` on P with a bound within the actor's, as ACC-030 does), and move everything about approval into stage D.

8. MAJOR. Ruling 53's open decision 2 is answered with a different question.
- The open decision is how a plan is confirmed without a terminal, and how that is counted for the S1 rerun. The simplification critic's gap 1: a non-interactive Codex author cannot answer y/N, and a plan ID is a counted line.
- Propose and approve do not address it: root's widenings never wait, and a fresh author cannot click an engine page.
- Do: present approvals as a new question, and keep decision 2's options (a) to (c) open or answer them separately.

9. MAJOR. Sequencing holds up the v0 bar.
- Decision 21(a) ties ruling 53's five accepted additions, which the S1 rerun needs, to Project stage A.
- Stage A waits on decisions 1 to 6 plus WO-1 to WO-4 (about 45 promises). That stacks a second dependency on the v0 bar, on top of the short-form investigation.
- Do: write the additions once in a form ready for scopes (keys carrying a scope ID that defaults to the engine), and let the rerun proceed whatever is decided on Projects.

B. ESCALATION PATHS LEFT OPEN

10. MEDIUM-HIGH. The sign-in phishing guard is weak.
- "Raised only for instances whose template root registered or approved" covers nearly every project instance, because by design root registers P's templates.
- So an operator can register, from a root template, an Agent whose expected-account argument names the owner's personal account, and root receives a sign-in that looks legitimate.
- Do: raise a sign-in only where the template fixes the expected account, or where a list of accounts that root declares for the project includes it.

11. MEDIUM. Secrets can still be reached through a confused deputy.
- Fix 13 guards templates, but not instance arguments or secret Resources. An operator can register an `op-item` that names any item in a store the engine reaches, and an instance whose declared `get-secret` Conduit targets it. Root's apply then opens the read.
- "Secret-store providers homed like any Resource" contradicts "the auth Pool lives in a root-only project".
- Do: make registering secret Resources and secret-store providers engine authority only, and show a get-secret line in the plan or evidence for root's setups.

12. MEDIUM. Widening never waits when it goes through instance arguments or controls.
- Reach flags (`accountServices` and `peerMessaging`, which S1's fixture holds as instance config), directory roots, and controls (REV-006) can all widen reach. The template cap fixes only runtime and placement.
- Do: require inputs that affect reach to be fixed by the template for registrations authorized through a Project, or say plainly in the mediation statement that operators widen freely through any input left open.

13. MEDIUM. A proposed control can never be carried out.
- The closed list of proposable requests includes controls. But R17 authorizes a control only through a Conduit to the instance, and an approve Conduit permits only "what a Conduit to P would", which never covers a control.
- A proposer who already holds the instance's Conduit is refused under "could make it all itself".
- Do: drop controls from the list, or add a row.

14. MEDIUM. One open is authorized by two halves, but records and `openedBy` name one Conduit.
- An approved action records the proposer and `approvedBy`, but not the approver Conduit that authorized it.
- Transitive `revoke.v1` and preservation test 7 (everything attributable) need both.
- Do: record both half Conduits, plus the ID of any audience entry used.

15. MEDIUM. One evidence digest cannot cover views that differ per approver. Each approver sees a different redacted view.
- Do: give each approver a digest of their own view, and have the engine recheck a full digest at the last approval.

16. MEDIUM. Who names tools across projects is unsettled.
- A Conduit homed in the target's project (for example coordinator -> worker-032, homed in packages) has its `as`, and so its tool names, chosen by the foreign project.
- Under IFC-012 and IFC-014 a colliding Conduit is not instantiated. A foreign project can therefore take the consumer's tool names, and the IFC-014 diagnostic leaks which local names the consumer already has.
- Do: give `as` to the consumer's side, or prefix the tool names of Conduits from another project.

C. FIRST START, REPAIR, PEERS, ENROLLMENT

17. MEDIUM. Root's Type is not `person.v1`.
- Root is `@core#interface.v1` (FST-001; conformance/fixtures/root-system-state.json is used by 197 cases). Yet `approve.v1` is limited to `person.v1` Interfaces, while root approves everywhere as the fallback. Root is also often driven by an AI admin session.
- Do: decide whether first start makes root a person (another FST change and fixture regeneration) or makes root an explicit exception.

18. MINOR-MEDIUM. Repair and state lifecycle are underspecified.
- Tombstones and pending actions "outlive the Project until root purges it", but `purge.v1` covers records, and no promise in stage A changes it. There is no retention rule.
- After an unshare cascade, it is unstated whether a later re-share stays blocked until each tombstoned Conduit is restored.
- `sameRegistration` breaks when a worker is recreated or replaced (`register-replacement.v1`, or the recreation pattern of finding 4-12).

19. MINOR. Peers and vocabulary.
- Marking an Interface "foreign" needs the trust domain of DL13 and ruling 51 (decision 2); say so.
- "Separate engines" gives an untrusted second person nothing safe in v0, because sharing across parties is v1 (ruling 51, decision 6).
- "Half" now has two meanings: in ruling 51 it is each engine's part of a relationship, here it is an authorization. That breaks ruling 42's one-meaning rule; rename one of them.

20. MINOR. Enrollment staying at engine level is right, but say that a project admin cannot bring compute: another party's machine is only ever a peer.

D. INTERPLAY WITH THE SHORT FORM AND RULING 53

21. MEDIUM. The concise form and the template cap conflict.
- Every `agents:` entry compiles to a template, so any concise-form author of a project needs `templates: all`. The cap then protects against Agents only.
- Non-admin granular Interfaces cannot use the concise form unless it supports `from:` an existing template.
- Collab also cannot run `conduit install`, which goes through `management`, so the pins in her file need root.

22. MINOR. Ruling 53 decision 6 is answered implicitly.
- Decision 6 (derived defaults) is open: the owner asked for more detail. The recommendation answers it without saying so: enclosure removes the operator Conduit, and a derived `default` Pool stays.
- `managers:`, `approvers:` and `audience:` add more objects the CLI derives.
- The S1 example is inconsistent: the concise plan has `s1/default`, the canonical form has `workspace`.
- S1's local name `project` (the tool `project.file.read.v1`, the tombstone "as project") becomes confusing beside `project: s1`.

23. MINOR. "Address bound to the first project that registers it" blocks reusing one submission in two projects, and is an existence oracle.

24. MEDIUM. Explicitness (ruling 52). With enclosure, a Project's inventory omits the holders of `management` and of an unbounded `access`.
- Do: list "engine authority: …" in every inventory and plan, and decide the trade-off with disclosure.

25. MINOR-MEDIUM. Agents shared across projects. The owner named agents. Audiences exist only for Pools, so a reviewer Agent shared with every project needs one consumer grant per project, with no equivalent of `laterProjects`.

E. COST AND CONCEPT COUNT UNDERSTATED

26. MEDIUM. The concept count is too low.
- "About 8" leaves out:
  - Project, scope and engine scope, qualified references, reserved names;
  - the entry fields `ceiling`, `requirements`, `requestable`, `laterProjects` and `laterResources`;
  - the bound's dimensions with revision pinning, `templates: all` and `sameRegistration`;
  - `person.v1` and foreign Interfaces;
  - pending actions, the evidence digest, expiry, `replaces`;
  - `includeShared`, `retire-with-members`, share and unshare;
  - envelopes, `--recreate` and `projectId`.
- By the simplification critic's counting rule this is about 25 to 30.
- The fresh author receives spec/ (rulings 48 and 53, item 9), which grows by about 100 promises.

27. MEDIUM. Costs left out of the estimate:
- spec/core documents: two Types, a capability with 17 operations, about 6 new Operation contracts with schemas, a requirement, 5 role templates, and reference schemas. 33 files under spec/core and spec/schemas mention "project".
- Document schemas and examples, which check_schemas.py validates.
- The web surface for approvals and its authentication.
- The round-7 cold read.
- Model checking of transitive revoke, two halves and approval races (roadmap step 6).
- The full cost again in each of the two independent builds (step 7).
- The rename estimate is fair: 55 of the 455 promises mention "project". But the rows of the ruling log are never rewritten, so ruling 37's row ("unique… within a project") changes meaning after the rename; it needs a note in AUDIT.md.
- Citation drift: reference-model/crates/shell/src/main.rs has `issue_client_token` at lines 178 and 234, not 157 and 212.