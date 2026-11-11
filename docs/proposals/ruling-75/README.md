# Ruling 75: retries fenced, the folder as the grant, browser logins settled, S2 to S10 accepted, G5's scope

Status: in force (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 7. Operations, outcomes, revisions, records and observation](../../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation), [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed) (spec pending except where a promise is cited). This record says why.

The coordinator gathered the questions that waited for the owner on 2026-11-11, each with a
recommendation:

- **A**, the model-checking finding MC2-001 (work order 105);
- **B**, the durable design that [ruling 74](../ruling-74/README.md) asked for in place of the
  `ownTools` switch;
- **C**, seven follow-ups of [ruling 73](../ruling-73/README.md) found by the research on browser
  logins;
- **D**, the pre-registrations of S2 to S10 (work orders 101 to 103);
- **E**, the scope of G5.

The owner: "agree with all except the following / for C5 can't we just do browser through ssh as
well?". The coordinator answered that Codex's browser login can cross machines, because its callback
waits on a fixed port and checks only the path and the login's state. The owner: "do intent, also
it's fine about the API keys unless they're in git history somewhere", and then "also confirm C5 and
accept all other rulings".

## The decisions

**A. A retry after a reconciled failure** (MC2-001)

1. Model checking found that OPS-014 and OPS-024 together break OPS-018 on a plain implementation,
   in seven actions:
   - an attempt is marked unknown while it is still on its way;
   - an actor reconciles it as failed;
   - the Agent's retry dispatches a new attempt;
   - both land, so one keyed effect is applied twice.

   The decision: an actor may record that an attempt's effect did not occur only once the attempt
   is fenced. Either no copy of its dispatch remains in flight, or the implementation confirms that
   the attempt can no longer act. This is the fenced control of experiment 012.

**B. What an Agent reaches through its own tools**

1. **The folder is the grant.** `ownTools` is removed from the `.v2` Types before 0.2.0
   publishes. An Agent has its client's own tools exactly when its template names the directory it
   works in. An Agent that names none has `.v1`'s tools. S1's Agents name no directory, so they
   write nothing; forgetting the line is safe.
2. **Confinement limits live on the runtime provider.** That covers the hosts allowed and a
   read-only directory. The provider is chosen per Agent with `runtime:`, and its limits are not
   Agent inputs. The first confining provider is to wrap an existing whole-process sandbox, and is
   built later.
3. **A dedicated macOS user.** Before a coordinator Agent drives workers (A78's orders 4 and 5),
   the engine that runs Conduit's own work runs as a dedicated macOS user.

**C. Browser logins** (ruling 73's follow-ups)

1. **A Resource's login is done by a provider package.** The package keeps the token in its own
   state directory. The engine holds no token (SEC-020). Until Agents are confined, such a package
   runs where no unconfined Agent runs.
2. **The return goes to the Interface that received the sign-in.** That is PND-010's and ruling 66
   point 3's rule, and root in practice in v0.
3. **A Claude Agent signs in with a browser on its own machine** in v0. Conduit never carries
   Claude's code between machines, because Anthropic's terms forbid intermediating Claude.ai
   credentials. To be revisited in v1.
4. **`conduit` runs on the Mac in v0.** From the Windows computer or a phone, the person uses SSH
   and pastes, or a device code. A phone Interface is v1's. A root Interface on another machine
   waits until Agents are confined.
5. **Codex uses its browser login everywhere.** Across machines, its redirect comes back through the
   person's Interface (ruling 73) or an SSH forward. The device code stays an option where an
   account has it on. A probe confirms that a carried redirect completes the login.
6. **Logins are declared by the package.** Each package declares each of its logins:
   - the authorization origin and path;
   - the client ID;
   - the redirect or hosted page.

   The declarations are shown at install and in inspection, and pinned with the package's digest.
   The engine relays nothing else. A client ID that two installed Types claim needs root's
   recorded confirmation.
7. **The account check stays strict.** A Resource whose service cannot say who signed in gets no
   sign-in.

**D. S2 to S10** (experiments 013 to 021)

1. **Each scenario's pre-registration is accepted:** its pass conditions, measures, counting rules
   and planned vectors, with each scenario's own answers as the coordinator listed them. They
   include:
   - the Windows computer as a managed node with the engine fixed on the Mac (S3);
   - SQLite search behind counted adapters (S8);
   - twelve replays for S10 in place of twenty.
2. **Accounts.** S7 and S8 also use the second subscription accounts. Every account has its own
   label, and every Agent home signs in on its own.
3. **Authors.** S4, S5, S6 and S9 are each written twice, by the coordinator and by a fresh Codex
   author, with scripted clients.
4. **Real clients.** Beyond S1 and S2, the only real-client run is S4's one lost-response replay on
   a second account, which also gives A6 its real client's trace.

**E. G5's scope**

1. **G5 and A40's model checking account for every convention-layer safety promise, wherever it
   sits:** 217 in the safety catalogue, each held by a named model or listed with its reason. This
   reads ruling 39's "the safety promises of the seven conventions" by its intent. The promises
   about two machines, a partition and a lost machine lie outside the seven files.

On the API keys printed into one research agent's transcript on 2026-11-11: the coordinator
searched every commit of every branch, tag and pull-request ref for each key, and found none. The
owner accepted leaving them as they are.

## What it amends

- Ruling 39: G5's reach (E1).
- Ruling 70: points 1 and 3. The Agent's own tools are on where the template names a directory,
  not by default, and there is no switch (B1).
- Ruling 73: point 3, on who keeps a Resource's credential and which Interface receives the return
  (C1, C2); and Claude's and Codex's routes across machines (C3, C5).
- Ruling 74: points 2 and 3. S1 writes no own-tools line, and the switch is gone (B1).
