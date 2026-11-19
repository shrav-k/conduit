# Ruling 86: the Agent items stay plain in 0.2.0, the sign-in hook is keyed per operation (option D) in a later release with its engine half in 0.2.0, four changes to `spec/` are written now, and Codex's account check runs with refresh blocked

Status: in force (ruled by the owner on 2026-11-18, by the repository's local date; the coordinator's notes date the answers 2026-11-19, a date that, by the coordinator's reading, is in UTC). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 9 (points 1, 2, 3 (c) and (d), 4 and 5) and topic 11 (points 2, 3 (a) and (b), and 4), and A5's row of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes) (point 5's probe). This record says why.

[Ruling 82](../ruling-82/README.md), point 6, accepted the targeted review's line o1: "the item
that implements `sign-in-hook.v1` carries Conduit's keys, for `codex.v2` where it states `signIn`
and for `claude-code.v2` [...]; the turn item's kind stays 050's choice". TPL-019 allows a template
one run item, shared by the turn and every hook it runs (`spec/promises/templates.yaml`), and both
Agent packages' `.v2` items are one run item marked `plain`. So keying the sign-in hook keys the
turn. Order 050e had kept the turn plain, because IFC-025 passes a failed or unknown result's
`content` to the caller only from an implementation marked `plain`. And
[order 050l](../../work-orders/050l-codex-account-hook.md)'s Record, on the Codex lane, left R4
open: Codex's `account/read`, the route ruling 82, point 1, chose for `codex.v2`'s account check,
may refresh the login and rewrite the home's login file as its own upkeep, even with
`refreshToken` false, while `account-hook.v1` declares no effect, so that under CTR-002 it changes
nothing outside itself.

The coordinator put two questions to the owner on these, through the session's question tool, with
options and a recommendation. The owner asked for a discussion and research on the first, and for a
different route on the second. The coordinator's research (private) then compared the options; the
owner then answered two further questions, the first pointing to it. A spec check of options D and
E, part of the coordinator's research (private), then reported, and the owner answered a third
dialog on it. The questions as put, the options as the coordinator's notes hold them and the owner's
answers follow, verbatim. The notes date all three dialogs 2026-11-19, a date that, by the
coordinator's reading, is in UTC, and record no time for any; by the repository's local date, by
which the ledger dates rulings 83 to 85, all three fell on 2026-11-18, the day the research and the
spec check were written.

## The questions

### The first dialog

1. As put: "Ruling 82's o1 (keyed sign-in item) collides with the spec's one-run-item rule. Plain
   items in 0.2.0 still let Mac slots sign in through `conduit sign-in`; keying adds login choice,
   Codex's carried return from elsewhere, and hook-level refusals. How should 0.2.0 go?"

   The options: "Plain now, keyed next (Recommended)", "Keyed before 0.2.0", "Plain, keyed later,
   no spec now". The notes hold the options by label only.

   The owner typed: "Let’s discuss advantages and disadvantages and do your research"

2. As put, as the notes hold it, shortened where the ellipsis stands: "Codex's account/read ... can
   itself refresh and rewrite the login file as routine upkeep, even with refreshToken off. The
   account-hook contract says the check changes nothing. How should we treat it?"

   The owner chose "Find a different route", not the recommended option. The notes hold that label
   alone, with neither its text nor the other options.

### The second dialog, after the research

1. As put: "Keyed vs plain Agent items: which way for 0.2.0? (Full comparison in [the
   coordinator's research (private)].)" The bracket replaces the name of a private note.

   The owner chose "A now, check D/E (Recommended)", whose text read: "0.2.0 ships plain items
   (default-login sign-in on the Mac works). The carrier spec + engine half land now (~1 day) so
   later keyed packages need no new engine; IFC-010 is fixed and a login choice aimed at a plain
   hook is refused before any browser opens. A half-day spec check picks D (per-operation keys) or
   E (hook arguments) as the route for keying next. o1 amended for 0.2.0."

2. As put: "Codex account check: block refresh for the check (a probe confirms no write first).
   When an expired access token makes the check fail, what then?"

   The owner chose "(b) Proceed to sign-in (Recommended)", whose text read: "Amend PND-010 so a
   failed check before the hook still proceeds to the sign-in hook. Clean, and closes an existing
   gap where a rejected refresh token blocks re-sign-in."

For the second dialog the notes hold only the options the owner chose, and no other option.

### The third dialog, after the spec check

As put: "The spec check recommends D (sign-in keyed per operation, turn and other hooks plain) for
keying later. Since the turn then stays plain, the failed-turn 'carrier' you approved for 0.2.0
would never be used. What should 0.2.0's engine carry instead?"

The owner chose "D's engine in 0.2.0 (Recommended)", whose text read: "Rule D as the route (o1
read per operation). Order 207 writes D into the spec instead of the carrier, and 209 builds D's
engine half in 0.2.0, so the later keyed sign-in release is package-only and needs no newer engine,
your stated aim. About the same cost as the carrier (one spec + one engine order, off the chain);
the spec edit is wider (~56 lines in 17 files)."

The other options offered: "D later, drop the carrier"; "Keep the carrier as ruled". The notes hold
them by label only.

## What the research found

The research is the coordinator's and is not in the tree; these lines are the coordinator's
summary of it, read from `origin/main` and the two 050 lanes, with Codex's source at tag
`rust-v0.167.1`, not measures this tree holds. Its costs are estimates.

- **Plain and keyed.** A plain item is an ordinary MCP server, sent only the protocol's fields and
  read through IFC-007 and IFC-010's fixed table; a keyed item receives Conduit's `_meta` keys,
  such as `signInLogin`, and states its own outcome. The coordinator's plan for 050's remaining
  orders already keyed the sign-in item, in 050k's and 050m's item 3, and so keyed the turn with no
  carrier for 050e's fields.
- **What is at stake**, by item kind, as the research compared whole items:

  | | Plain item | Keyed item |
  | --- | --- | --- |
  | Default login | Works once 048a-5d and 048a-5e are built and IFC-010 is fixed | Works once 048a-5d and 048a-5e are built |
  | A login other than the default (Claude's code page, Codex's device login) | No: the hook gets no name | Yes, or a refusal naming PND-010 |
  | Codex's return carried through Conduit | No: only the person's own SSH forward | Yes, once 048a-7, 048a-8 and 051b-3 are built (048a-6 merged), and only if ruling 82 point 1's live check passes; for Claude nothing either way, its logins stating `carried: false` |
  | A failed turn's fields reach the caller | Yes, today | Lost until a carrier and an engine change land |
  | "Never started" told from "may have run" | No: every failed turn is unknown | Yes |
  | A login that lapses during a turn | The turn ends unknown, its class `authentication` in its content; no sign-in action is raised until the next account check | The engine raises the sign-in action and holds the instance |
  | Who is trusted for outcomes | The engine's table | The package's own word |
  | Upkeep | Unaffected by the extension's revisions | Follows each revision |

  Under option D, as ruled (point 2), the sign-in hook takes the keyed column's rows on sign-in,
  and the turn keeps the plain column's: a failed turn's fields still reach the caller, every
  failed turn stays unknown, and a lapse during a turn still raises no sign-in action until the
  next account check.
- **The options.** A: plain in 0.2.0, keyed in a later release, the carrier written now (about a
  day added, with the engine half). B: keyed before 0.2.0 (o1 as ruled; reverses 050e and puts six
  interface obligations on two packages with no Agent-side kit). C: a separate hook item beside the
  run item, changing TPL-019 (about twenty promises, ruling 41's one-run fence, 2 to 4 days). D:
  keys per operation on one item, plain for the turn and the other hooks and keyed for sign-in
  alone (changes what `plain` means, which is per item today; unchecked against NAM-024, OPS-012
  and IFC-025). E: the login and the return as `sign-in-hook.v1`'s arguments, the hook staying
  plain (closest to Codex's and ACP's own login methods; changes a core contract, which has no
  input schema today, and PND-013's first check, which requires a keyed hook). The research
  recommended A with the carrier, the IFC-010 fix and the PND-010 refusal of a login aimed at a
  plain hook, and D and E checked as the route for keying next.
- **The spec check of D and E**, read after the second dialog, compared the two promise by promise
  and recommended D, narrowed. D keeps the login and the return where `spec/` already fences them,
  in `_meta` (IFC-006, PND-014, REC-002), and changes no core contract; it meets o1's intent, the
  hook's own failed refusal naming PND-010 included, and its text where "item" is read per
  operation. E reopens PND-014, REC-002 and IFC-010, under which a plain hook's refusal of a named
  login ends unknown rather than failed naming PND-010, departs from MCP's retry by carrying the
  return in changed `arguments`, and fixes inputs on a core contract for the core address's life.
  D's costs: the defined term `plain`, and the words "carries Conduit's keys", change in at least
  56 lines in 17 files of `spec/`; and an engine without D's half rejects at install a manifest
  that holds D's new key, and E's route needs an engine that sends the hook's inputs, so neither
  route met the carrier's aim, a later keyed package that needs no newer engine, unless its own
  engine half lands in 0.2.0. The carrier serves only a keyed turn, which D leaves plain. The
  check proposed narrowing D to hook operations, so that the turn, OPS-012, IFC-025 and the gate
  stay per item for requests, and the term changes on the hook path alone.
- **Common to every option.** IFC-010's `then` reads a plain result's `input_required` as unknown,
  and its observable adds only that a URL request outside `sign-in.v1` also raises PND-005; it
  names none inside `sign-in.v1`, while IFC-007, IFC-016 and PND-010 have a plain sign-in hook's
  URL request relayed; a checker of the research found this a conflict to fix in `spec/` before
  048a-5d is dispatched. The engine on `main` reads a URL result as IFC-016's lapse trigger before
  it branches on plain or keyed (`engine/crates/core/src/requests.rs`, the `Completion::Result`
  arm that sets `sign_in::Trigger::UrlElicitation`, at 2036-2037 on fde1c615); the hook part
  inside `sign-in.v1` is 048a-5d's to build, IFC-049's URL match the part's that 048a-5's cut gives
  it, and the relayed output 048a-5e's.
- **The account check.** `refreshToken` false stops only the caller's refresh. Codex still
  refreshes proactively when it finds the access token near expiry, and recovers from a 401 of
  workspace discovery by refreshing, and saves a refresh that succeeds. A turn refreshes the same
  way; the check adds occasions with no turn (setup, the check before sign-in, polling) and a race
  between a check and a turn over a refresh token that may be used once. Of the routes compared,
  the research recommended `account/read` with refresh blocked for the check's process alone:
  Codex's `CODEX_REFRESH_TOKEN_URL_OVERRIDE` set, on that process only, to an `https` address
  under the `.invalid` domain, which no resolver answers, so nothing listens there directly; behind
  a system proxy only TLS guards the token
  ([RFC 6761, section 6.4](https://www.rfc-editor.org/rfc/rfc6761#section-6.4)). Codex saves only
  after the refresh endpoint answers, so no refresh completes, and a blocked refresh is transient
  and not remembered, so the block never yields a false "no account". Its serious weak point: with
  an expired access token, discovery gets a 401, recovery cannot refresh, and the check fails,
  where PND-010 reaches `sign-in-hook.v1` only after a check that succeeds. The variable is
  undocumented: Codex's login code reads it, and only Codex's own tests set it.

## The decisions

1. **The Agent packages' `.v2` items stay plain in 0.2.0** (the second dialog's answer 1). Ruling
   82, point 6, o1, is amended, dated 2026-11-18: the item implementing `sign-in-hook.v1` carries
   Conduit's keys in a later release, not in 0.2.0, and for that operation alone (point 2). So the
   coordinator's plan for 050k's and 050m's item 3 no longer keys the sign-in item, and order
   050e's plain turn stands, in 0.2.0 and after it. In 0.2.0, as the research found:
   - a dogfood slot on the Mac signs in through `conduit sign-in` with its client's default login
     ([ruling 85](../ruling-85/README.md), point 5), once 048a-5d and 048a-5e are built and
     IFC-010 is fixed (point 3 (b));
   - no login can be named: a request that carries `signInLogin`, the default's name included, is
     refused before any part runs (point 3 (c)); without one, the hook starts its client's default
     login;
   - no return is carried through Conduit, since PND-013's first check needs a keyed hook, so a
     Codex sign-in from another machine completes only through the person's own SSH forward, the
     account check alone deciding completion;
   - every failed turn is unknown, and a login that lapses during a turn ends that turn unknown and
     raises no sign-in action until the next account check.

   By the coordinator's reading:
   - "o1 amended for 0.2.0", in the second dialog's option, moves o1's taking effect to a later
     release and strikes nothing of it, and the third dialog's "o1 read per operation" says how it
     is read when it takes effect (point 2); and
   - [ruling 82](../ruling-82/README.md), point 4's logins are still stated now, and under 0.2.0's
     plain items the refusal that point 4 gives the hook, asked for the device login, comes instead
     from the engine before any part, under point 3 (c), with a diagnostic naming PND-010; the hook
     itself is never asked in 0.2.0, and point 4's words hold for the keyed release.
2. **Keying comes later by option D, keys per operation on one item, with ruling 82's o1 read per
   operation** (the third dialog's answer, after the spec check that the second dialog's answer 1
   asked for). An implementation item marked `plain` may list operations it implements whose calls
   carry Conduit's keys; a call for a listed operation carries them and is read as a keyed
   implementation's, and every other call to the item stays plain. That shape, a list on an item
   that stays marked `plain`, is the spec check's and the coordinator's, open to the owner's
   strike; the owner ruled per-operation keying with the turn and the other hooks plain. Ruling
   82's o1, "the item that implements `sign-in-hook.v1` carries Conduit's keys", is read as: that
   item carries them for its `sign-in-hook.v1` operation, and serves its turn and its account,
   sign-out and retire hooks plain. This ruling leaves E (the login and the return as
   `sign-in-hook.v1`'s arguments), B (keyed before 0.2.0) and C (a separate hook item, changing
   TPL-019) unchosen.
   - *Coordinator's*, open to the owner's strike: D is narrowed so that the list names hook
     operations only, as the spec check proposed. A request's operation, the turn among them, is
     never listed, so OPS-012, IFC-025 and the gate stay per item for requests, and the change to
     `plain` touches the hook path alone. The list's name and schema, and whether it is also
     limited to items that a command or a package runs, which the spec check proposed for an item
     behind a gate, are order 207's to put to the coordinator.
   - **NAM-024's rationale.** An item's `plain` flag is not a member NAM-024 freezes, and by the
     coordinator's reading neither is the list, an implementation item's member and not a
     contract's; so NAM-024's letter lets the same `.v2` be keyed later, as IN_FORCE.md already
     states of the flag. Keying turns point 3 (c)'s refusal of a named login into a started one,
     and the written answer to NAM-024's rationale, when a `.v2` package later lists its sign-in
     operation as keyed, is left for the owner. The coordinator's reading, open to the owner's
     strike, read from NAM-024 and
     [order 199](../../work-orders/199-nam-024-which-additions-count.md) on `main`: keying meets
     order 199's test as NAM-024's observable words it. Order 199 wrote the test, an addition that
     changes what an unchanged request may do, into NAM-024's observable only as the reason for the
     two classes that its `then` names, a login added to a Type and a member added to a login, and
     the observable applies it to "a request that was dispatched under the earlier release, made
     again with the same target and inputs". Keying adds neither: ruling 82, point 4, states the
     logins and their members at 0.2.0. A `sign-in.v1` that names no login starts its client's
     default login under 0.2.0's plain hook and, the hook choosing under PND-010 as NAM-024's
     observable says, may do no more under a keyed one. A `sign-in.v1` that names a login is refused
     in 0.2.0 before any part (point 3 (c)), so no hook ever receives it, as a request naming a
     further operation reaches no implementation under NAM-026, the case NAM-024's observable gives
     for an addition that changes nothing such a request may do. Whether NAM-024's "dispatched"
     excludes such a request, which the engine's own `sign-in.v1` refuses before any part, is what
     the owner's written answer settles; if the owner reads it otherwise, the keyed sign-in is made
     under Types with another version segment, as NAM-015 has any other change made.
3. **Four changes to `spec/` are written now** (the second dialog's answers 1 and 2, and the third
   dialog's answer). Order 207 writes them: the third dialog's option names it for (a), and for (b)
   to (d) it is the coordinator's.
   - (a) **D's spec**, in place of the carrier, which is dropped: the turn stays plain, so a keyed
     turn's `content` copy would never be used, and IFC-025 stays as it is, passing a failed or
     unknown result's `content` from a plain implementation. Order 207 writes point 2's list, under
     the coordinator's narrowing, into `spec/`: the term `plain` in `spec/definitions.md`, the
     manifest's schema, PKG-020, IFC-037's discovery before the first keyed call to a run, the
     promises that read `plain` or "carries Conduit's keys" (the spec check listed IFC-001, IFC-004,
     IFC-006, IFC-007, IFC-009, IFC-010, IFC-045 and IFC-047; the coordinator adds IFC-043 and
     IFC-044, whose `when` reads `plain` and which hold a keyed `remote` item to `https` and no
     `Authorization`), and PND-010's and PND-013's first check, read per operation; which lines
     change is order 207's.
   - (b) **IFC-010's `then`** is fixed so that a URL request inside a `sign-in.v1` part is relayed,
     as IFC-007, IFC-016 and PND-010 already say.
   - (c) **PND-010 refuses a `signInLogin` aimed at a sign-in hook whose operation is not keyed
     before any part runs**, so that no browser opens. In 0.2.0 no item lists an operation, so every
     named login is refused. That the refusal comes before any part, the account check included,
     and not only before the hook, is the coordinator's reading of "before any browser opens",
     matching PND-010's other request checks.
   - (d) **PND-010 is amended so that a failed account check before the hook still goes on to
     `sign-in-hook.v1`**; completion still needs a later check that names the account. PND-011's
     `then`, which lists the cases that go on to that hook, changes with it. A check that ends
     unknown is not named in the owner's option; whether it goes on alike is left to order 207 to
     put to the coordinator, and until then it does not (the coordinator's reading).

   The research asked whether the carrier was a later addition under ruling 78, point 3.1, which
   has a later addition reach the owner with its size. D's spec and engine half replace it, and the
   third dialog's option put them to the owner with their size, "About the same cost as the carrier
   (one spec + one engine order, off the chain); the spec edit is wider (~56 lines in 17 files)";
   that the owner's choice of that option meets point 3.1, which this ruling does not amend, is the
   coordinator's reading.
4. **D's engine half lands in 0.2.0** (the third dialog's answer). Order 209, which the option
   names, builds it as order 207's text states it: the loader accepts point 2's list, a hook call
   for a listed operation carries Conduit's keys and is read as a keyed implementation's, and the
   discovery IFC-037 asks for comes before the first keyed call to a run. No 0.2.0 package lists an
   operation, so the later keyed sign-in release is package-only and needs no newer engine (the
   option's words). Order 209 builds no keyed `content` copy, the carrier being dropped (point 3
   (a)). That order 209 also builds the refusal of point 3 (c) and the going on of point 3 (d) in
   0.2.0 is the coordinator's reading of the second dialog's "0.2.0 ships plain items" and of its
   answer 2. By the coordinator's reading, point 3 (b) adds nothing to order 209, and, so that
   default-login sign-in on the Mac works, the chain's Done-when lines must cover a plain hook's URL
   request: 048a-5d's, that the hook's answer is taken by the sign-in arm, plain or keyed alike, and
   not read as IFC-016's lapse trigger; the order that builds IFC-049's URL match, that its URL is
   matched; and 048a-5e's, that it is relayed, the hook part kept in progress and not ended unknown.
5. **`codex.v2`'s account check keeps asking Codex through `account/read`, with refresh blocked for
   the check's process alone** (the first dialog's answer 2 and the second dialog's question 2).
   Ruling 82, point 1's rules are unchanged: the hook never opens the login file, and asks Codex
   itself. Codex's `CODEX_REFRESH_TOKEN_URL_OVERRIDE` is set, on the check's child process only, to
   an `https` address under the reserved `.invalid` domain (`https://refresh.invalid/oauth/token`).
   A system proxy that receives the request can read the token only if it intercepts TLS with a
   certificate the system trusts, as it could for Codex's own refresh. No child of the wrapper but
   the check's receives `CODEX_REFRESH_TOKEN_URL_OVERRIDE`, and none receives
   `CODEX_REVOKE_TOKEN_URL_OVERRIDE`. The wrapper's environment allowlist keeps both out today, and
   the order that sets the variable tests that a value of either in the wrapper's own environment
   reaches no child, the check's included, and that the check's child receives
   `CODEX_REFRESH_TOKEN_URL_OVERRIDE` with the shipped value alone, so that nothing sends a refresh
   token to an address Conduit did not choose. A probe, order 208, first confirms that the check so
   run writes nothing of the login: the home's login file keeps its bytes, mtime and inode, and no
   lock file for it is created; the probe uses the file store on the Linux node, so whether a
   keychain item is read or written is left to stage 1's first step, on the Mac. It runs under
   `env -i` with a fresh scratch home, a fake login in the file store
   (`-c cli_auth_credentials_store="file"`), the override set as shipped, to
   `https://refresh.invalid/oauth/token`, the ChatGPT base URL pointed at a loopback stub of
   `accounts/check` that answers 401 in one case and 200 in another, a control run whose refresh
   reaches a loopback stub, through the login's own issuer or, in a second form, through the
   override, to show that a refresh is attempted, and every connection but loopback refused, so that
   no OpenAI host is reached. That any such write returns the choice to the owner is the
   coordinator's reading of "a probe confirms no write first". That "no write" means the login, and
   not the state files and helper directory that Codex's start writes in the home (order 197's R4),
   is the coordinator's reading of the owner's concern. The variable is undocumented, so it is
   re-read at every bump of Codex's pin. `account-hook.v1` keeps declaring no effect, so CTR-002
   stands: this answers 050l's open R4. An expired access token that makes the check fail no longer
   blocks signing in again, under point 3 (d). The second question named this route as the plan
   ("block refresh for the check (a probe confirms no write first)"), and the owner answered within
   it; that the owner thereby accepted the route, as "a different route" in the sense of the first
   dialog's answer 2, is the coordinator's reading.

## What it amends

- **Ruling 82, point 6, o1**: amended to D's per-operation reading, taking effect in a later
  release, not 0.2.0 (points 1 and 2): the item implementing `sign-in-hook.v1` carries Conduit's
  keys for that operation alone. The account, sign-out and retire hooks need no keys, PND-005's two
  halves stay in `spec/`, and the turn item is plain, in 0.2.0 and after it, as order 050e chose.
- **Ruling 82, point 1, its route refined, by the coordinator's reading of point 5**: `account/read`
  is asked with refresh blocked for the check's process alone, once the probe finds no write to
  the login (point 5). Its rules, its live check and its terms for `signIn` are unchanged.
- In `spec/`, by order 207 and spec pending until then: the meaning of `plain` and the promises
  that read it, per operation for hook operations (point 3 (a)); IFC-010 (point 3 (b)); PND-010
  (points 3 (c) and (d)) and PND-011's `then` (point 3 (d)).
- Not a ruling, and changed with it: the coordinator's plan for 050k's and 050m's item 3.

## What it does not change

- **Ruling 82, point 1's rules**: never open the login file; ask Codex itself; one live check,
  A5 stage 1's first step, after 048b-join and before 0.2.0 merges; `codex.v2` states `signIn`
  only if it passes; nothing switches a Codex Agent to an API key.
- **Ruling 82, point 4**: the logins 0.2.0 states, and NAM-024's rule that a login 0.2.0 does not
  state never joins `.v2`. **Ruling 82, point 6, o2 to o7**, as rulings 83 and 84 amend them.
- **TPL-019 and ruling 41's one run item**, since option C is not chosen.
- **IFC-025**, since the carrier is dropped; and **`sign-in-hook.v1`'s contract**, since option E
  is not chosen.
- **CTR-002 and `account-hook.v1`'s contract**: the check still declares no effect.
- **Ruling 85, point 5**: a dogfood slot on the Mac signs in after 051b-2.
- **Orders 048a-7, 048a-8 and 051b-3** stay on 0.2.0's path (048a-6 has merged). The research
  noted that under plain items no 0.2.0 package uses a carried return; moving them off the path is
  not decided here and would need the owner's ruling, under the scope guard.

## What is left for the owner

- The written answer to NAM-024's rationale when a `.v2` package later lists its sign-in operation
  as keyed, so that a named login that point 3 (c) refuses in 0.2.0 starts then (point 2; the
  coordinator's reading there is open to the owner's strike).
- If order 208's probe shows a write to the login, the route of `codex.v2`'s account check (point
  5).
- As before, if ruling 82 point 1's live check fails, the Codex question returns to the owner.

## Facts recorded with it

- The owner's time: three dialogs, the first of two questions, the second of two and the third of
  one, on 2026-11-18 by the repository's local date (the coordinator's notes date them 2026-11-19,
  a date that, by the coordinator's reading, is in UTC), with the research written between the first
  and the second, and the spec check between the second and the third. The notes record no times.
- The research was written on 2026-11-18, with an earlier analysis of o1 against TPL-019 and its
  checker's corrections, which it applied; the spec check of D and E was read on 2026-11-18 from
  `origin/main` at c704275f. Both are the coordinator's and not in the tree.

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md): topic 9's line on o1 says that the `.v2` items stay plain in
  0.2.0 and that the item implementing `sign-in-hook.v1` carries Conduit's keys for that operation
  alone in a later release, by option D, with the coordinator's reading of NAM-024's rationale; its
  line on `codex.v2`'s account route gains the refresh block, its probe and its re-reading at each
  pin bump; and it gains *spec pending* lines for points 3 (c) and (d), a line for D's engine half
  and an *Open* line for what is left to the owner. Topic 11 gains *spec pending* lines for points
  3 (a) and (b).
- [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes): A5's row says that order 208's probe
  comes before stage 1's first step.
- [The tracker](../../AUDIT.md#waiting-for-the-owner) gains a row for the owner: the written answer
  to NAM-024's rationale, and the account check's route if the probe shows a write to the login.
- [The ledger](../ruling-log.md) gains row 86, and ruling 82's status, there and in its record,
  notes the amendment of point 6, o1, and the refinement of point 1's route.
- Later, by orders, and not in this change: order 207 writes point 3 into `spec/`; order 209
  builds D's engine half, with points 3 (c) and (d); order 208 runs the probe, pre-registered
  before it runs; an order of 050's Codex half sets and strips the variable once the probe finds
  no write to the login, and the Codex package's README states its limit; 050k's and 050m's item 3
  are rewritten plain; a later release's packages list their sign-in operation as keyed; and 050l's
  Record gains a dated note that this ruling answers its R4.
  Where the variable's re-reading at each pin bump is listed is the coordinator's.
