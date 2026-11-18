# Ruling 83: the Agent packages 0.2.0 are subscription-only, and API access comes later as a declared route

Status: in force (ruled by the owner on 2026-11-18). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 9 (spec pending, order 050f, for point 1; a direction, not a design, for point 2). This record says why.

[Ruling 82](../ruling-82/README.md), point 2, put `apiKey` in `.v2` as an explicit setting only,
and had a template that sets it fail setup with an explicit error until batch 2 writes ruling 61
B13's text. Order 050f, which writes the `.v2` inputs and is held, not dispatched, found no place
in the specification as written where that refusal can end setup with its cause. The coordinator
put that one question to the owner through the session's question tool on 2026-11-18, with
options and a recommendation. The owner chose no option and asked to discuss a more durable
answer. The coordinator's discussion followed in the session, and the owner replied to it. The
question as put, its options, the owner's typed answer, the discussion and the reply follow,
verbatim.

## The question

As put, at 16:17:07 UTC: "You chose: a template that sets apiKey 'fails setup with a clear error'
until batch 2 settles B13. The spec as written has no place to put that refusal at setup (codex.v2
without sign-in runs no hook at setup, and a refusal by the client's own run isn't promised to fail
setup with its cause). How should it work?"

Its options:

- "Small spec rule (Recommended)": "Add one convention-layer sentence: when a package's run
  refuses its configuration at start, that setup part ends failed with the package's cause in the
  diagnostic. The .v2 wrapper then refuses a set apiKey at setup until batch 2. General and
  reusable for any package; one small spec order before 050n."
- "Refuse at the first turn": "No spec change: setup succeeds, and the wrapper refuses every turn
  while apiKey is set, with a clear error. Simpler, but the error comes at the first turn, not at
  setup."
- "Engine checks apiKey": "The engine itself refuses a .v2 setup whose apiKey is set. Works, but
  the engine would then know one package's input, which the language otherwise leaves to
  packages."
- "Secret-read route": "Bind apiKey through the spec's secret read: with no secret Conduit open to
  the Agent, setup's run part is refused as unavailable, naming the read rather than apiKey. Relies
  on root never opening such a Conduit until batch 2."

The owner chose no option and typed, at 16:18:04 UTC: "Let’s discuss what would be a more durable
long term solution to the apikey vs subscription issue in genrrwl".

## The coordinator's discussion

At 16:19:33 UTC, in the session, verbatim, with "you" and "your" read as the owner:

> Here's how I'd think about it.
>
> **Why this keeps coming up.** A client authenticates in one of two ways:
> - **Subscription login:** the client's own OAuth sign-in, which lives in the Agent's own home (050c).
> - **API key:** today the only spec route for a secret into a run is the run reading it itself (SEC-010). Ruling 61's B13 bars an Agent that takes turns from doing that, because an unconfined Agent (ruling 70) can read anything its client holds. So "API key as an Agent input" is in tension with B13 by design. That's why there's no clean place for the refusal: we'd be patching around it.
>
> **The durable pattern: never give the key to the Agent; give it a connection.** This is the standard industry answer, and it sits exactly where ruling 40 says Conduit innovates, on connection and access.
> - **How it works:** a provider holds the API key and runs a credential-injecting gateway; existing gateways can serve. The Agent's client is pointed at that gateway instead of the vendor. Both clients appear to support this: Claude Code documents an LLM-gateway setting, and Codex has custom model providers with a base URL. I'd want a source read at our pinned versions, like order 197's, to confirm.
> - **What Conduit does:** it opens, limits and closes that route per Agent, as it already will for the Claude sign-in connection (050o).
> - **What you get:**
>   - **Explicit by construction:** no route declared means no API use, so a silent fallback is impossible.
>   - **B13 holds:** the Agent never holds the key.
>   - **Rotation and revocation in one place:** close the route or rotate the key at the provider.
>   - **Per-Agent records:** usage, budgets and an audit trail come with Conduit's records and metrics.
>   - **Several keys or orgs:** each is just a separate provider Resource.
> - **Subscriptions stay as they are:** the client's own login, managed by sign-in, never read by Conduit. Each Agent then has exactly one declared form of model access, subscription or API route, and the engine never chooses between them.
>
> **The catch:**
> - **The route isn't a Conduit as Conduits work today.** It's an HTTP egress route, not MCP tool access, so it's a real language design: a new managed connection plus a provider type. That's batch-2-sized work, not a 0.2.0 patch.
> - **Not proven yet:** the gateway support in both clients and any terms-of-service limits still need checking. Experiment 010 has already covered subscription terms.
>
> **What it means for 0.2.0, which freezes now.** If the long-term answer is "a declared route, never a key in the Agent", then freezing `apiKey` as a secret the Agent's run reads is the wrong member to lock in. My recommendation:
> - **0.2.0 is subscription-only:** take `apiKey` out of `.v2`, keep `account` as it is, and drop the refusal problem entirely.
> - **API access comes later as the gateway route:** added as an optional member, which NAM-024 allows, so it can come later without a `.v3`.
> - **The cost:** no API-key Agents until that design lands. No current slot uses one.
> - **What it reverses:** your ruling 82 point 2, so it's your call. I'd fold it into a short ruling with the gateway direction recorded.
>
> Before you decide, I'd suggest a short grounded study: a workflow that checks each client's gateway support at our pinned versions, existing gateway providers, and how the route maps onto Conduit's model. It needs no client runs and none of your time.
>
> Meanwhile, I'm holding only 050f, the one order that depends on this. 048a-2 and 047d are being read, and the rest of the chain keeps moving.

None of the discussion's statements about the clients' gateway support, existing gateways or
the terms was checked at the versions Conduit pins. This record cites them as the coordinator's,
for point 3's study to check, not as facts.

## The owner's reply

At 16:20:59 UTC: "Agree with all your recommendations, continue".

The reply is this ruling, as ruling 67 has it. It agrees with the recommendations that the
discussion made: 0.2.0 subscription-only, with `apiKey` out of `.v2`, `account` kept as it is and
the refusal problem dropped; API access later as the gateway route, added as an optional member;
a short ruling with the direction recorded; and the grounded study. That "all your
recommendations" means the discussion's, and not the question's recommended option "Small spec
rule", is the coordinator's reading: the discussion came after the owner set the question's
options aside, and with `apiKey` out of `.v2` no set `apiKey` is left to refuse, so none of the
question's four options is chosen. That sorting is the coordinator's too. The discussion offered
the study before the owner decided. That the reply decides points 1 and 2 now, and that the study
comes before the route's design, is the coordinator's reading.

## The decisions

1. **The Agent packages 0.2.0 are subscription-only: `apiKey` is not in `.v2`** (the reply; the
   discussion's first recommendation). This amends [ruling 82](../ruling-82/README.md), point 2.
   Neither `.v2` Type has an `apiKey` member, and no item of either binds one. `account` stays as
   order 050f's text states it: required, with no default, a string or null. The `oneOf` of
   `account` and `apiKey`, which order 044 gave the conformance fixture of the Agent packages 0.2.0
   (conflict C12) and which 048a-1 kept when it made `apiKey` required under ruling 82, point 2,
   goes with `apiKey`. That "keep `account` as it is" means 050f's text, and that the `oneOf` goes
   with it, is the coordinator's reading. A null `account` means the Agent signs in to no
   subscription: no account check runs, since CTR-013 and PND-011 check only an account pointer that
   holds a string, and with no other route the Agent's turns fail with the class `authentication`
   ([order 050e](../../work-orders/050e-agents-v2-failed-turn-fields.md)). That is no fallback,
   since no key is used. Those two sentences are the coordinator's reading of what the
   subscription-only shape leaves. The refusal at setup that ruling 82, point 2, asked for, and that
   the question was about, is not needed: no member of `.v2` names a key. With it goes ruling 82
   point 2's term that no `.v2` Agent is given a `get-secret` Conduit before batch 2, which was
   there for `apiKey`; ruling 61 B13's own line, which bars such a Conduit to any Agent that an
   actor can take turns on, stands as batch 2's (the coordinator's reading). Ruling 82 point 2's
   settlement for 0.2.0 of the conflict between finding 4-02 and ruling 61 B13 now rests on there
   being no key in 0.2.0, and point 2 of this ruling gives the later answer that keeps a model
   service's API key out of every Agent; both are the coordinator's reading. The engine still never
   falls back from a sign-in to a key, and nothing switches a Codex Agent to an API key (ruling 82,
   point 1). This is [the targeted review's](../review-signin-2026-11-16/README.md#q2-apikey-in-020)
   Q2 option c, "no `apiKey` in `.v2`; 4-02's alternative waits for a `.v3` or another route", with
   point 2's route as that other route. The review's [section
   6](../review-signin-2026-11-16/README.md#6-must-state-lists) gave `account`, where `apiKey` is
   not in `.v2`, as "a non-empty string where it does not" put `apiKey` in. Keeping `account` a
   string or null departs from it, and the departure is the coordinator's reading, for this reason:
   a later declared route (point 2) can then serve an Agent with no subscription without a `.v3`,
   since NAM-024 lets a later release add an optional member but not widen `account` to admit null.
   The cost, in the discussion's words: "no API-key Agents until that design lands. No current slot
   uses one." The review recorded a further cost of option c, that it "leaves Q3's fail branch with
   no Codex route": on a fail of ruling 82 point 1's live check, Codex has no route in 0.2.0 until
   the route's design lands, and the question returns to the owner as before.
2. **The direction for API access: an Agent never holds a model service's API key** (the reply;
   the discussion's second recommendation). This is a direction, recorded so that later designs
   keep to it. It is not a design, and nothing in `spec/` or any order is to be written from it
   before point 3's study and a design that the owner rules on. An Agent never holds a model
   service's API key; model API access comes later as a declared route: a
   credential-injecting gateway that a provider runs, holding the key, to which the Agent's client
   is pointed instead of the vendor, and which Conduit opens, limits and closes per Agent. That
   places it in what [ruling 40](../ruling-40/README.md), point 1, makes Conduit's own, connection
   and access, with the gateway itself a provider, as ruling 40 has everything else. The route is
   added later as an optional member of the Types, which NAM-024 (`spec/promises/naming.yaml`)
   allows: its `then` lets a later contract add "an optional property of a Type's `schema`", which
   states no `default`, so the route needs no `.v3`. That the route fits that sentence is the
   coordinator's reading, which the route's design checks. Subscriptions stay the client's own
   login, managed by sign-in, which Conduit never reads. Each Agent has exactly one declared form
   of model access, a subscription or the API route, and the engine never chooses between them.
   How the route and `account` exclude each other is the design's, not this ruling's. The
   targeted review's Q2 bears on it: under NAM-024, "a later `oneOf` would make `account: null`
   alone invalid", so the route and `account` cannot be made to exclude each other by a `oneOf`
   added later, and the design must find another means or face a `.v3`. What the
   discussion lists as the route's gains (explicit by construction, B13 holding, rotation and
   revocation at the provider, per-Agent records, several keys or organizations as separate
   provider Resources) and its catch (an HTTP egress route, not tool access through a Conduit as
   Conduits work today, so a new managed connection and a provider type, work of batch 2's size)
   are the coordinator's, recorded as the reasons given, not as decided.
3. **A grounded study runs before the route is designed** (the reply; the discussion's last
   recommendation). It reads three things: each client's gateway support at the versions Conduit
   pins, from their documentation and source, as [order
   197](../../work-orders/197-a5-stage-1-step-1-codex-account-read.md) read Codex's source;
   existing gateways, and the providers that run them; and the route's place in Conduit's model,
   under ruling 40's connection and access and NAM-024. It needs no client run and none of the
   owner's time. The terms that the discussion lists as not yet checked are the study's to note
   where its sources state them, by the coordinator's reading; so is the reading of [experiment
   010](../../../experiments/010-subscription-terms/results-2026-11-10.md) that finding 4-02
   ([increment 4's log](../first-build/log-increment-4.md)) rests on, that a product running Claude
   Code may not remove or restrict an authentication method built into it, against packages that
   offer no key in 0.2.0. The study decides nothing; the route's design, with the study, comes to
   the owner (the coordinator's reading). The study gates nothing in 0.2.0. The discussion did not
   put that tension to the owner: it said "Experiment 010 has already covered subscription
   terms", and that experiment's reading is about removing an authentication method. So the
   tension waits for the owner in [the tracker](../../AUDIT.md#waiting-for-the-owner), not only
   as a note inside the study.

## What it amends

- **Ruling 82, point 2**: `apiKey` is not in `.v2` (point 1). The point's other line stands: the
  engine never falls back from a sign-in to a key. Its schema shape, `account` "a string or null
  under the fixture's `oneOf`, beside `apiKey`", becomes `account` alone, as 050f states it.
- **Ruling 82, point 6, o7**: 0.2.0's surface is frozen as points 2, 4 and 5 of ruling 82 resolve
  it, point 2 as this ruling amends it.
- **Ruling 82, point 1, its clause on `apiKey`**: "a key is used only where a template sets
  `apiKey` explicitly, under point 2" falls with point 2's `apiKey`. The point's sentence before
  it stands: nothing switches a Codex Agent to an API key.
- **Finding 4-02, as the owner accepted it on 2026-11-12**: "`.v2` offers an API key as an
  alternative to the subscription login, never the default". For `.v2` in 0.2.0 there is no such
  alternative, and the alternative comes later as point 2's route, never as a key in the Agent.
  That the owner's agreement reaches this acceptance, through ruling 82 point 2, which carried it
  into `.v2`, is the coordinator's reading.
- **The targeted review's parked list, line 9** ([section
  7](../review-signin-2026-11-16/README.md#7-the-parked-list)): ruling 82, point 2, answered it;
  it now stands answered by that point as this ruling amends it.

## What it does not change

- **Ruling 82, points 1 and 3 to 6, but for point 1's clause on `apiKey` under point 2 and o7's
  citation of point 2**: they stand. `codex.v2`'s account route, the live check, the merge order,
  the logins, the descriptions and o1 to o6 are unchanged.
- **Ruling 61 B13**: its text stays batch 2's. Point 2 keeps a model service's API key out of
  every Agent; B13 governs every other secret.
- **CTR-013 and PND-011**: the engine reads only what `account-hook.v1` reports, and the account
  check runs only where the account pointer holds a string.
- **NAM-024**: its text is unchanged; point 2 relies on it as written.
- **Ruling 40**: point 2 places the route within it.
- **Ruling 71, point 6, and its later answers**: when the ChatGPT Codex allowance runs out,
  workers pause and the owner is told, with no API key.
- **Order 050f's other content**: `toolsDirectory` and the description sentences (i) and (ii).

## Facts recorded with it

- The owner's time: about 4 minutes, on 2026-11-18. The question was put at 16:17:07 UTC and
  answered at 16:18:04; the coordinator's discussion followed at 16:19:33; the owner replied at
  16:20:59.

## What follows

- Order 050f is amended to drop `apiKey` before it is dispatched: its `.v2` schemas take
  `account` as it states it, with no `apiKey` and no `oneOf`; its section on where a set `apiKey`
  fails setup, and the options there, go; its statements that order 048a-1 gave the fixture
  `account` and shaped it are corrected to order 044, with 048a-1 adding `apiKey` to `required`;
  it starts from the fixture as order 202 realigns it, below, so its preflight, its account of
  the fixture and its list of what it leaves untouched expect no `apiKey` and no `oneOf` there;
  and its row in the work orders' index and order 117's entry for 050f follow.
- A small order, 202, realigns the conformance fixture of the Agent packages 0.2.0, which order 044
  shaped and order 048a-1 (its decision 6) made require `apiKey`, to point 1's: no `apiKey` and no
  `oneOf` in either `.v2` schema, `account` required with no default, a string or null, with the
  archive, its digest and the case `agents-v2-defaults` regenerated and checked. It realigns the
  fixture first and lands before 050f is dispatched, and 050f starts from the realigned fixture,
  since 050f states the packages' `.v2` schemas against it; that order is the coordinator's.
- Order 050f carries, before 0.2.0 merges, the change to each wrapper's refusal of a home holding
  a `.env`, whose text ends "since an API key is used only where a template sets apiKey": that
  text in both packages, its test constant and the Codex README's quote of it drop the `apiKey`
  reason (for example, "since the Agent holds no API key (ruling 83)").
- The study of point 3, before the route is designed. When its order is written, the study gains
  a planned row, with an A-number, in [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes)'s
  register, which holds one row for every experiment and probe, planned, running or done.
- [IN_FORCE.md](../../../IN_FORCE.md), topic 9: the line that held `apiKey` in `.v2` gives way to
  point 1's line, spec pending with order 050f; a line records point 2 as a direction, not spec
  pending; and the freeze line cites this ruling beside ruling 82, point 2.
- [The tracker](../../AUDIT.md#waiting-for-the-owner): the parked list's row notes that line 9 is
  answered by ruling 82, point 2, as this ruling amends it; and a row waits for the owner on the
  tension of point 3 between packages that offer no key in 0.2.0 and experiment 010's reading.
- [ROADMAP.md](../../../ROADMAP.md): no row names `apiKey` in 0.2.0, so none changes now; the
  study's row comes with its order.
- [The ledger](../ruling-log.md) gains row 83, and ruling 82's status, there and in its record,
  notes point 2, point 6's o7 and point 1's clause on `apiKey` amended by this ruling.
