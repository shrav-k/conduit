# Targeted review of the sign-in and home surface of the Agent packages 0.2.0, 2026-11-16

## 0. Front page

**The answer:** it turns on Q3 (decided section D item 9). Under Q3's recommended option a,
`codex.v2`'s `signIn` is not freeze-ready until the Codex account fact is in (answer 3), and the
rest is freeze-ready after the listed changes; under Q3's other options, freeze-ready after the
listed changes (answer 2). **For the owner:** five questions ([section 5](#5-the-owner-questions)):
Q1, the `.v2` descriptions; Q2, `apiKey` in 0.2.0; Q3, `codex.v2`'s `signIn` and the account fact;
Q4, when 0.2.0 merges; Q5, whether a later `.v2` release may add a login. Then one accept-or-strike
of the lines for o1 to o7 ([section 1](#1-the-open-line-clause-by-clause)). **What stays held:**
048a-1, 048a-2, 048b-join, 050 and 051b, until the owner's answers ([ruling
81](../ruling-81/README.md), point 1). A81 and A22 gate nothing. **This record binds nothing:**
[spec/](../../../spec/README.md) alone binds, and nothing here holds until the owner rules on it.

## About this record

| Part | What it holds |
| --- | --- |
| This file | The front page; the *Open* line, o1 to o7; the three questions the owner left to the review; the owner questions; the must-state lists; the parked list; how it ran |
| [verdicts.md](verdicts.md) | A verdict for every member, M01 to M91, with its A81 and A22 marks |
| [findings.md](findings.md) | Every finding that survived verification, in its surviving form; then those the skeptic's flags dropped, and the struck items |

The review asked: can the sign-in and home surface that the Agent packages 0.2.0 (`claude-code.v2`,
`codex.v2`) would freeze under NAM-024 (`spec/promises/naming.yaml:414-439`) be frozen, and if not,
exactly what must change before 048a-1 and 050 are written? Ruling 81, point 2, set it, on Fable,
for this one review. It read the repository at T, `0db2951c88879cfa70338f39775b6a96076486e8`, and
the three package branches by hash: `559ecd0b` (order 078, cited by default), `dfcef3ec` (order 076)
and `65a999f0` (order 050a). Sources are cited as `file:line` at T or `hash:path:line`. The run's
working files, its brief, its run design and every agent's output, are kept out of the repository;
[section 8](#8-how-it-ran) says how it ran. The IDs dec-N, known-N and K1 to K11, and "the decided
list" with its sections A to E, belong to the review's packet, which is kept with those files.

## 1. The *Open* line, clause by clause

`IN_FORCE.md:218` lists seven clauses this review owns. For each: the record's answer, what it
rests on, its carrier, and how it reaches the owner. The lines that are not among the questions
reach the owner in **one accept-or-strike** of this whole list.

- **o1, whether the hooks are keyed or plain.** *Answer:* the item that implements `sign-in-hook.v1`
  carries Conduit's keys, for `codex.v2` where it states `signIn` (Q3), and for `claude-code.v2`
  where it states more than one login, as it does if Q5's a and Q1 (vi) stand; the account, sign-out
  and retire hooks need no keys; an item's `plain` flag is a manifest member NAM-024 does not
  freeze, only the default's name is; the turn item's kind is 050's choice, and its turn
  implementation follows whichever of F84's halves matches it. *Rests on:* V16, V28 (S12), V18;
  `spec/interface/implementation.yaml:169-186`; `spec/promises/pending.yaml:204-205, 251-252`.
  *Carrier:* 050; 048a-2 for F84's text. *To the owner:* accept or strike. Not covered (G07):
  whether the turn item is keyed is left open, and #17's field carrier depends on it.
- **o2, the design of the connection Conduit manages for a Claude sign-in (80 point 8).** *Answer:*
  no Type or core member: conditions with tests, scoped to `carried: false`; the channel that
  carries the return terminates at each end in a program that is not Conduit's, and no process
  running Conduit's code decrypts, terminates or copies it (V07, S07); a forward opens only after
  this sign-in's URL is relayed, for the port that URL's redirect_uri names, to that node's
  loopback, and closes when `sign-in.v1` ends, never on another port (V08); the connection does not
  bind who completes the login (V22, S14); a Claude Agent's paste route is the person's own on-node
  delivery of the return, or the code-page login's code, held by no engine or Interface process
  (V12, S05). *Rests on:* V07, V08, V12, V22; `docs/proposals/ruling-80/README.md:147-156, 345-353`;
  `IN_FORCE.md:216`. *Carrier:* 050. *To the owner:* accept or strike. The rewrite of the three
  stale texts (M74 to M76) rests on V07's surviving fix, which states it as `IN_FORCE.md:216`
  already owes, with dec-8 and known-2 to known-4 (G10).
- **o3, what order 050 does about Codex's login opening a browser past the shim.** *Answer:* 050
  states the limit in the unfrozen texts it already writes, the limits section and the accounts
  guide: on the engine's own runner, Codex's browser login may open the default browser of the
  engine user's graphical session, where one exists; the hook still relays the printed URL under
  IFC-049, the account check (PND-011) stays the only guard, no probe gates the freeze, and a read
  of the opener library (P-R4-4) may narrow the guide's words later; FR-118 is not raised. *Rests
  on:* V06; V20 (S09); `experiments/027-a5-sign-in/results-2026-11-16.md:73-76`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:199-216, 258-263`. *Carrier:* 050.
  *To the owner:* accept or strike; making the device login Codex's default is parked (S16).
- **o4, the refusal reason for a second sign-in of one instance.** *Answer:* `conflict`: 048a-2
  writes one sentence after PND-010's fixed-port sentence, with a diagnostic naming PND-010 and,
  where the caller may see it, the other invocation's pending action, one vector, and MC9a
  registered with that reason before dispatch; the fixed-port sentence stays `unavailable`. Under
  OPS-023 conflict comes before unavailable, so a second sign-in.v1 of the same instance reports
  conflict even where it also shares a fixed port; the fixed-port sentence's unavailable then
  reaches only a sign-in of another instance. *Rests
  on:* V21 and V37, merged (S13); `spec/definitions.md:238-239`;
  `spec/promises/pending.yaml:179-183`; `spec/conventions/operations.yaml:505-515`. *Carrier:*
  048a-2. *To the owner:* accept or strike; B0's `unavailable` is the option not taken.
- **o5, F95 to F97.** *Answer:* F97, CTR-013 gains CTR-014's control sentence with one loader
  vector, carried by 048a-1; F96, a named spec fix before 048a-2 is drafted; F95, a named spec fix
  landed with 050's rewrite of the limits and IFC-049's observable; none bears on a 0.2.0 member's
  freeze. *Rests on:* V22 (S14), V38 (C06); `docs/proposals/full-review/decisions.md:276-278`.
  *Carrier:* 048a-1; two named spec fixes, F96's and F95's. *To the owner:* accept or strike.
- **o6, whether `codex.v2` states its device login now or later, 75 C5's option standing.**
  *Answer:* under Q5's recommended a, now: `codex.v2` states the device login, its literals read
  from the client, and until F83's code path and 051c's panel exist the hook asked for it fails
  with a diagnostic naming PND-010 and the unbuilt path and never chooses it unasked; the
  description names the browser login as the default. Under Q5's b, later, within `codex.v2`.
  *Rests on:* V11 (should under S04); under a, the skeptic's reading of NAM-024 (C01): an addition
  that changes what an unchanged request may do, a further login or a member added to an existing
  login, is stated at 0.2.0 or never in `.v2`, and an addition an unchanged request never
  exercises, a new operation with its implementation default, is an optional thing; under b,
  G02's Lens B, under which V33's defer holds. *Carrier:* 050. *To the owner:* as Q5. The critic
  marked o6 not answered, because V11 and V33 read NAM-024 both ways, and G02's verifiers split
  (Lens A confirmed the strict reading, Lens B narrowed to the open one), so neither reading is
  confirmed and the record asks it. The skeptic's case for asking it: "Two readers and one verifier
  read NAM-024 the other way; the device login is the owner's standing option (75 C5), and a
  wrong reading costs a codex.v3 either way." G02's Lens B, verified after the skeptic ran, read it
  the other way too: NAM-024's own example, "a further operation of a capability", is an element
  appended to a required list, and a later login is used only where asked for (G02 in
  [findings.md](findings.md#g02-one-reading-of-nam-024-for-a-login-added-later)).
- **o7, whether 0.2.0 can be frozen under NAM-024 (81).** *Answer:* it turns on Q3 (decided section
  D item 9). Under Q3's recommended option a, `codex.v2`'s `signIn` is not freeze-ready until the
  Codex account fact is in (answer 3), and the rest is freeze-ready after the listed changes; under
  Q3's other options, freeze-ready after the listed changes (answer 2) (S22). The listed changes:
  `signIn` with the required account place, the four hooks under `requires` with defaults,
  `toolsDirectory`, and `apiKey` as Q2 decides (V01, V13); the client homes and D53's refusal (V02,
  V15, V26); the descriptions as Q1 decides (V17); the page and device logins as Q5 decides (V12,
  V11); every literal read from the client at a stated version. A81 and A22 gate nothing. *Rests
  on:* the 35 change verdicts and the 3 defer verdicts in [verdicts.md](verdicts.md);
  `docs/proposals/ruling-81/README.md:43-49`; decided section D item 9 (a 0.2.0 that waits on A5
  stage 1 is stated as a question, under each of its answers). *Carrier:* 050, 048a-1, 048a-2 and
  two named spec fixes. *To the owner:* its parts as Q1 to Q5, and this line in the accept-or-strike
  list.

## 2. The three questions the owner left to the review

The owner left these to the review on 2026-11-16 (dec-35 to dec-37; [ruling 81's later
answers](../ruling-81/README.md#later-answers)).

- **(a) Can A5 stage 0's Codex failures (`experiments/027-a5-sign-in/results-2026-11-16.md:69-76`)
  be met with sign-in in the core?** For failure (b), the tab opened past the shim: **yes**, by a
  stated limit, guidance and the account check (V06; o3); FR-118 is not raised. For failure (a), the
  status naming no account: **yes only if a named probe passes**: the probe is Q3's account fact,
  under Q3's recommended option the package's account hook in a scratch home signed in through
  Conduit's Interface flow (stage 1's first step, pre-registered), reporting the signed-in account
  (V03, S03). On its fail, `codex.v2` states no `signIn` (CTR-013,
  `spec/promises/contracts.yaml:305-310`), Codex is configured by `apiKey` where Q2 keeps it (under
  Q2's a, only once batch 2 writes B13's text), and **FR-118 returns to the owner once the probe has
  run**, as the brief's output contract 2(a) has it and S03 and C03 apply it (G05, Lens A
  confirmed), with the recommendation that `codex.v2` state no `signIn` and the owner's choice among
  O3's two remaining answers. The case for keeping sign-in in the core even then, R1's and R4's: the
  fail is the client's, and moving sign-in out of the core changes nothing for it. Under Q3's option
  b, the owner chooses that no for Codex, which is also the owner's own answer to FR-118 for Codex
  (C03).
- **(b) The verdicts on M81 to M87, M16 and M17, together.** M81 to M85: freeze as is (no
  finding). M86: freeze as is; its sentence is copied into `claude-code.v2`'s description. M87,
  M16 and M17: change before 050 (V17): the descriptions name no client version, the README
  records the version read; both carry decision 25's sentence, with "On the engine's own runner"
  before the engine-user sentence; "ends the conversation" is kept with D55's un-refusing struck
  for `.v2`, or dropped (M68); the Codex browser limit stays out of the descriptions (V06); any
  sentence on the home names no store (V19). These changes reach the owner as Q1.
- **(c) For A81 and for A22, each member whose freeze a verifier confirmed is not safe without that
  probe.** **A81 bears on no member's freeze.** R3, R4 and B0 marked M41 not safe; Lens A narrowed
  the mark and Lens B refuted it: every A81 outcome leaves `carried: true` valid, a fail ends
  `returned: undelivered` and polling continues (`spec/promises/pending.yaml:278-282`), and the
  account check alone decides completion (`:208-210`); the workflow marked it `makes_gate: false`. A
  fail goes to the owner as a finding against rulings 73 and 75 C5
  (`docs/proposals/ruling-73/README.md:58-61`), and A81 stays owed before S3 (`IN_FORCE.md:215`).
  Lens A's case, not adopted: a fail of A81's carry leg would put `carried: false` to the owner
  (ruling 73, `:60-61`), which NAM-024 forbids once 0.2.0 holds `carried: true`; Lens B answers that
  the literal is permissive and no outcome requires changing it (S01). Not covered (G09): which lens
  governs M41 was left to the record; it follows Lens B, as the critic's handed table and the
  skeptic do. **A22 bears on no member's freeze,** provided no frozen text names the store of
  Claude's login (V19); neither the retire hook, nor the logout read, nor the port probe makes it a
  gate (C10, S21). A22 stays owed before any Claude Agent is a slot (D61).

## 5. The owner questions

Five questions: Q1 to Q4 in the skeptic's ranking, then Q5, which the record adds because G02's
verifiers split ([section 8](#8-how-it-ran)). Each needs the owner because the decided list does not
settle it. Q1 is the one dec-36 sends, and Q3 carries the answer to (a)'s conditional yes (decided
section D item 9); Q2 and Q4 are needed because decided section D items 1 and 4 leave conflicts
between decided items open. Q2 and Q3 should be answered together, Q3 with Q4, and Q5 before Q1 (v)
and (vi). Q3's a needs Conduit's Interface sign-in flow, which the engine stubs until the sign-in
orders land, so with Q4's b, 0.2.0 either waits for them or merges `codex.v2` with no `signIn`,
which `.v2` then never gains (V01). "Answer under each option" is o7's answer if that option is
chosen.

### Q1: the `.v2` descriptions as 0.2.0 freezes them

- **Stake:** every sentence of the two descriptions is frozen for the life of `.v2` and reaches
  every Agent that reads the tools (IFC-033).
- **Question:** which sentences do the two `.v2` descriptions gain or lose, each accept or strike:
  (i) no client version; (ii) decision 25's other-runtime sentence in both, with "On the engine's
  own runner"; (iii) the clause "ends the conversation" kept with D55's un-refusing struck for
  `.v2`, or dropped; (iv) D53's retire-hook sentence, with the login kept where the client keeps it
  for that directory and no store named; (v) `codex.v2` names the browser login as the unasked
  default and lists the device login; (vi) `claude-code.v2` lists Claude's code-page login. (v)'s
  device clause and (vi) stand only under Q5's a, and (v) only where `codex.v2` states `signIn`
  (Q3). Not in the descriptions: the browser-opens limit and the refusal of a home outside the state
  directory.
- **Options:** a) all six, the clause kept, D55's un-refusing struck for `.v2`; b) all six, the
  clause dropped, D55 left open; c) the branch descriptions as they stand plus only what CTR-013
  and D53 require.
- **Recommendation:** a.
- **Strongest case for b, against the recommendation:** D55 is a logged option deferred to after the
  rerun; freezing the clause closes it for `.v2`'s life while order 62 is unwritten; R4 froze M68 as
  right today.
- **Strongest case for c, against the recommendation:** the fewest frozen words; each added sentence
  can be made false later and reaches every Agent under IFC-033; the browser default is already
  038c's plan and PND-010's choice; a second Claude login doubles the literals that can drift (K8),
  and the redirect's return can be delivered by hand on the node.
- **What waits on it:** 050's description text; M16, M17, M68, M86, M87; the statements of V11
  and V12 (o6; M33, M43), which turn on Q5.
- **Answer under each option:** a, b, c: answer 2.

### Q2: `apiKey` in 0.2.0

- **Stake:** `apiKey` can enter `.v2` only in 0.2.0, and two of the owner's decided items collide
  on how its key would reach the client.
- **Question:** does `.v2` hold `apiKey` in 0.2.0? Finding 4-02 as accepted and 61 B13 cannot both
  be met by SEC-010's route, the specification's one route for a secret into a run, and the
  property can enter only now: MAN-017 binds only a required place, NAM-024 lets a later release
  add only an optional property without a default, and a later `oneOf` would make `account: null`
  alone invalid.
- **Options:** a) `apiKey` in the schema as the fixture shapes it, but no `.v2` Agent is given a
  `get-secret` Conduit until batch 2 writes B13's text; a template setting `apiKey` fails setup
  explicitly until then; b) `apiKey` in, with SEC-010's route live now, B13 read as not covering
  the engine's own read into the client's environment; c) no `apiKey` in `.v2`; 4-02's alternative
  waits for a `.v3` or another route; d) `apiKey` in, settable only on an Agent that no actor but
  root takes turns on, checked at registration.
- **Recommendation:** a: the only option that forecloses nothing and chooses between neither of
  the owner's decisions; the fixture follows 050's schema.
- **Strongest case for b, against the recommendation:** SEC-010 exists today; the key reaches only
  the client's environment, which an unconfined Agent could read anyway under ruling 70; a frozen
  unusable member invites root to open the Conduit against B13.
- **Strongest case for c, against the recommendation:** B13's letter is unconditional, and
  `IN_FORCE.md:293` and `:353` already bar API keys elsewhere; but c removes the owner's accepted
  alternative for `.v2`'s life and leaves Q3's fail branch with no Codex route.
- **Strongest case for d, against the recommendation:** it reads B13's own "each turn-holder other
  than root"; but no registration-time check of future turn-holders exists, so it is a new engine
  rule for 050's release.
- **What waits on it:** M44 to M47, M70; V01's `apiKey` clause; V34; Q3's fail branch; F143's line.
- **Answer under each option:** a, b, c, d: answer 2. Q3's fail branch exists only under Q2's a, b
  or d.

### Q3: `codex.v2`'s `signIn` and the account fact

- **Stake:** if `codex.v2` states `signIn` and Codex's account hook cannot learn an attested
  account, `signIn` cannot be removed, and every Codex slot moves to a `codex.v3`.
- **Question:** does 0.2.0 state `signIn` on `codex.v2` before any source shows that Codex's
  account hook can learn a service-attested account? CTR-013 lets a Type state `signIn` only where
  the hook reports such an account, and NAM-024 forbids removing it later; stage 0 found the status
  names no account, order 195 read the status only, and the only registered trial is stage 1, by
  the package's hook, on the first build with sign-in.
- **Options:** a) 050 is built; `codex.v2` states `signIn` only once the package's account hook, in
  a scratch home signed in through Conduit's Interface flow (stage 1's first step, pre-registered;
  the pre-registration names the Type, not an installed `codex.v2`, under which the scratch home is
  signed in), reports the signed-in account; no archive whose `codex.v2` states `signIn` is
  installed anywhere before; b) `codex.v2` states no `signIn` now; Codex on `apiKey` only (needs
  Q2's a, b or d; under Q2's a, usable only once batch 2 writes B13's text); `signIn` with a
  `codex.v3` later; c) state `signIn` now, as 038c plans, and accept a `codex.v3` and fresh
  registrations on a fail.
- **Recommendation:** a: the fact comes from the package's own hook, as D80 registers it, before
  the literal freezes, at the cost of one sign-in by the owner that stage 1 costs anyway; a
  no-cost read of the public source may run first and decides nothing.
- **Strongest case for b, against the recommendation:** nothing waits and nothing is frozen blind,
  as CTR-013's own text allows; but Codex is out of the subscription login for `.v2`'s life, against
  ruling 72 point 2 (dec-11) and ruling 75 C5, depends on Q2 keeping `apiKey`, and is the review's
  no to dec-35's (a) for Codex, returning FR-118 to the owner as this answer.
- **Strongest case for c, against the recommendation:** the literal reading of "A5 gates nothing"
  and of ruling 80 point 7; but a fail breaks a must, costs a `codex.v3` and re-registers every
  Codex slot, and 038c's plan predates stage 0's failure.
- **What waits on it:** M08, M21, M22, M23, M52; 050's Codex half; V01's `codex.v2` clause; the
  pre-registration of stage 1's first step in experiment 027's README; dec-35's (a) for failure (a).
- **On a fail, under a:** `codex.v2` states no `signIn` (`requires` holds `retire-hook.v1` alone),
  Codex is on `apiKey` where Q2 allows (under Q2's a, only once batch 2 writes B13's text), and
  FR-118 returns to the owner (dec-35). A81 is not part of this question: it bears on no member's
  freeze and stays owed before S3.
- **Answer under each option:** a: answer 3 for `codex.v2`'s `signIn` until the account fact is
  in; b: answer 2; c: answer 2.
- P-R4-2, the owner's one-bit read, is parked (line 2): it strikes D80 (dec-29), which the brief
  does not reopen; this departs from S03, as S16 and S17 drop options on the same ground.

### Q4: when 0.2.0 merges, relative to the sign-in orders

- **Stake:** an engine without 048a-2 runs no account check at setup and stubs `sign-in.v1`, so a
  0.2.0 merged earlier would have its slots signed in by hand.
- **Question:** does 0.2.0 merge only after 048b-join, with 050 written after the record beside
  048a-1 and 048a-2, or as soon as 050 is ready, under ruling 80 point 7's "next"? At T the engine
  checks `signIn`'s shape but runs no account hook at setup and stubs `sign-in.v1`
  (`engine/crates/core/src/requests.rs:1046-1058`), so an earlier 0.2.0 could sign its slots in
  only by hand, against ruling 72 (`docs/proposals/ruling-72/README.md:72-74`) and ruling 49 point
  3's check.
- **Options:** a) 050 written right after the record, beside 048a-1 and 048a-2; 0.2.0 merges after
  048b-join; each slot's sign-in follows 051b (ruling 78 point 1 part 2); b) 0.2.0 merges when 050
  is ready; the slots are signed in by hand until the sign-in orders land, then checked again;
  c) 050 written only after 048b-join is written.
- **Recommendation:** a: 050 stays next in the writing and merges onto an engine that keeps the
  promises 0.2.0 states.
- **Strongest case for b, against the recommendation:** rulings 80 point 7 and 81
  (`docs/proposals/ruling-81/README.md:95-97`) make 050 "next"; the Fleet's path waits on own tools,
  not sign-in; the dogfood store already runs a branch-built 0.2.0 with logins made by hand; but
  ruling 72's rule and ruling 49 point 3's check are skipped for the first round, and the store is
  re-initialized once more.
- **Strongest case for c, against the recommendation:** no drift between 050's hooks and 048a-2's
  text; but it is the longest path, and ruling 81 keeps 050 next.
- **What waits on it:** the order of the five held orders; known-14's re-initialization and D56's
  export; V31.
- **Answer under each option:** a, b, c: answer 2.

### Q5: may a later `.v2` release add a login (o6; M23, M33, M43)

- **Stake:** under one reading, a login that 0.2.0 does not state can never join `.v2`; under the
  other, stating the device and code-page logins now freezes two more sets of client literals for
  `.v2`'s life.
- **Question:** may a later release at the same address append a login to a Type's
  `signIn.logins` as one of NAM-024's "optional things" (`spec/promises/naming.yaml:420-425`)? A
  member added to a login that 0.2.0 states, such as `issuer`, is a change under either reading
  (M32, M42).
- **Options:** a) no (the skeptic's reading, C01): `codex.v2` states the device login now with
  V11's refusal, and `claude-code.v2` states the code-page login, as Q1 (v) and (vi) have them;
  b) yes (G02's Lens B): M43 and M33 defer under `naming.yaml:420-422`, o6 reads "later, within
  `codex.v2`", and Q1 loses (v)'s device clause and (vi).
- **Recommendation:** a.
- **Strongest case for b, against the recommendation:** G02's Lens B: NAM-024's own example, "a
  further operation of a capability", is an element appended to a required list; a named login is
  used only where asked for; a device login states no port.
- **Strongest case against b:** V11's Lens B: defer is not safe under NAM-024 as written; a later
  login changes what an unchanged unasked sign-in may start (`pending.yaml:202-204`).
- **What waits on it:** M23, M33, M43; Q1 (v) and (vi); 050 `claude-code.v2` item 2, `codex.v2`
  item 2 and the connection's item 3.
- **Answer under each option:** a, b: answer 2.

### Dropped or moved

The skeptic's dispositions of the readers' candidate questions:

- **R3's question 1, Claude's code-page login (M33):** moved to
  the accept-or-strike list as V12's change; it reaches the owner as Q1's sentence (vi), and
  whether it can join `.v2` later as Q5. The case
  for keeping it a question: it adds a second Claude login literal and a package-owned channel on
  the node for a code, close to the owner's words that Conduit itself should not carry a Claude
  sign-in (`docs/proposals/ruling-80/README.md:185-197`).
- **R3's question 2, a probe of Codex's listener before `carried: true` freezes:** dropped (S01,
  S18): its premise is the A81 gate, which Lens B refuted.
- **B0's question 1, which login the Codex hook starts unasked:** dropped; its device-default option
  is parked (S16), since it inverts ruling 75 C5; the review owns o3, which V06 answers, and the
  browser default reaches the owner as Q1's sentence (v).
- **B0's question 2 and R4's question 1, their A81 halves and R4's option b:** narrowed and merged
  into Q3 (S17, S18): A81 bears on no member's freeze, and option b reopens D18 (dec-30).
- **o6 as its own question:** asked, as Q5: G02's verifiers split on the NAM-024 reading, so
  neither reading is confirmed; the skeptic's reading (C01) is Q5's recommendation.
- **P-R4-2, the owner's one-bit read of the account claim (S03's option b):** parked (line 2): it
  strikes D80 (dec-29), which the brief does not reopen ([section 8](#8-how-it-ran)).

## 6. Must-state lists

For each held order, what its writer must state, each traced to a verdict or a surviving finding
and written as a done-when a test, vector or named check can show. Where a list waits on an owner
question, the item says so.

### 048a-1

1. CTR-013's `then` gains CTR-014's control sentence, adapted: "The Type lists no control whose
   changes holds the account pointer, a pointer inside it or a pointer over it, compared as CTR-006
   compares pointers." Done when one loader vector rejects a Type that states `signIn` and lists a
   control whose `changes` holds `/account`. Traces to M20; V38, V22 (F97); C06.
2. The fixture `conformance/fixtures/s1/packages/agents-0.2.0` follows 050's `.v2` schema as Q2's
   answer shapes it, not the reverse. Done when the fixture's `.v2` schemas equal the schema that
   Q2's answer and 050 `claude-code.v2` item 1 fix, and the case
   `conformance/language/packages/agents-v2-defaults/` passes against them. Traces to M70,
   M44 to M46; V13 (R1-09); G06.

### 048a-2

1. PND-010's `then`, after the fixed-port sentence: a `sign-in.v1` for an instance whose earlier
   `sign-in.v1`, setup or retirement has been dispatched and has not ended is not dispatched and
   reports `conflict`, with a diagnostic naming PND-010 and, where the caller may see it (PND-002),
   the other invocation's pending action; the same in `sign-in.v1`'s description; the fixed-port
   sentence stays `unavailable`. Done when one vector shows the refusal and MC9a is registered with
   `conflict` before 048a-2 is dispatched. Traces to M89; V21, V37 (S13).
2. PND-005 gains F84's two halves: a keyed implementation reports a lapsed login by ending the
   call failed or unknown with a diagnostic naming PND-005; a plain one, only by a URL request
   (IFC-016). Done when PND-005 states both and a vector with a scripted lapse shows a keyed call
   ending with that diagnostic. Traces to M56; V28 (S12).

048a-2 is drafted after F96's named spec fix ([below](#carriers-outside-the-held-orders)).

### 048b-join

Nothing from this review. (Whether 0.2.0 merges only after 048b-join is Q4.)

### 050, `claude-code.v2`

1. The Type states `signIn` with its required account place `/account`; `requires` lists
   `sign-in-hook.v1`, `account-hook.v1`, `sign-out-hook.v1` and `retire-hook.v1`, each with a
   default in `implementation`; `toolsDirectory: {directory: /directory}`; `account` is required,
   with no default: a string or null under the fixture's `oneOf` where Q2 puts `apiKey` in, a
   non-empty string where it does not; `apiKey` as Q2's answer allows. Done when a loader vector
   accepts the Type and a template binding `/account`, and `/apiKey` where Q2 puts it in, passes
   MAN-017. Traces to M07, M15, M20, M22, M44 to M46, M54, M69; V01, V13.
2. The logins: the redirect login and, if Q5's a and Q1 (vi) stand, the code-page login (`return:
   page`, `carried: false`), every literal read from the client at a version the README names;
   `redirect.ports` is `any` where the port varies across starts and the fixed list where it does
   not; the hook relays the redirect login unless `signInLogin` asks for the page login. Done when a
   package test with a fake client shows each login's URL passing IFC-049's match, and the README
   names the client version read. Traces to M23 to M33; V12 (S05), V19.
3. Where the Type states more than one login, the item that implements `sign-in-hook.v1` carries
   Conduit's keys. Done when `conduit.yaml` gives that item without `plain: true` and a package
   test shows `signInLogin` reaching the hook. Traces to M49; V16.
4. Every `.v2` launch and hook runs the client with `CLAUDE_CONFIG_DIR` inside
   `CONDUIT_STATE_DIR`, read from the environment alone, and refuses, with a diagnostic naming the
   cause, where it is unset or the home resolves outside it; the `.v1` launch's environment is
   unchanged from 0.1.1. Done when a marker in a synthetic default home stays untouched after an
   undo, a retirement and a turn, and a test shows the `.v1` launch's environment equal to 0.1.1's.
   Traces to M57, M60 to M62, M88; V02, V15, V26 (S11). Not covered (G11): whether the refusal is
   scoped to `.v2` launches.
5. No client output, URL, claim or token in any result, diagnostic, log or record; the hook relays
   the URL of the login it started. Done when two canary tests pass: a fake client that prints a
   canary URL beside the one it hands over, and a fake home with a canary token and claims driven
   through every error path, each canary absent from every result, diagnostic, record and log.
   Traces to M49, M52; V09, V25 (S20).
6. The description holds exactly the sentences Q1's answer accepts, names no client version and
   names no store for the login. Done when the description in `contracts.yaml` is checked against
   Q1's answer sentence by sentence. Traces to M16, M68, M86; V17, V19 (S15).
7. `turn.v1`'s schemas stay byte for byte as 0.1.1 published them; #17's class and turn are not
   outputs and no schema or description enumerates the classes; the survivors of order 16's stop
   are named where #17's fields live and, on TERM, in the wrapper's own log; the turn
   implementation follows the half of F84 that matches its item. Done when 050a's byte check
   passes and a vector shows the fields reach the caller under IFC-025 wherever 050 places them.
   Traces to M05, M56, M65 to M67; V18 (S06), V28 (S12). Not covered (G07): the turn item's kind.
8. Before the first undo or retirement of a real login, a desk read of Claude Code's documentation
   for any revocation `claude auth logout` makes, and its scope. Pass: no revocation, or one scoped
   to that home's token. Fail: the hook removes the home's credential without the client command,
   naming the keychain item or waiting on A22, and the accounts guide's vendor line says what stays
   at the vendor. A22 bears on no member's freeze through this item. Done when 050's report records
   the read and its outcome. Traces to M53, M54; V10 (S21).

### 050, `codex.v2`

Items 2 and 3, item 4's `codex login` and the undo in its done-when, and item 5's clauses on the
sign-in and account hooks, hold only where `codex.v2` states `signIn` under item 1.

1. Under Q3: on the account fact's pass, or under Q3's c, the Type as `claude-code.v2`'s item 1; on
   the fact's fail under Q3's a, or under Q3's b, no `signIn`, and `requires` holds `retire-hook.v1`
   alone with its default; `account` and `apiKey` as Q2 allows; under option a, no archive whose
   `codex.v2` states `signIn` is installed anywhere before the fact is in. Done, under Q3's a, when
   the fact's outcome is recorded, with no value, before any archive whose `codex.v2` states
   `signIn` is installed, and, under every option, when a loader vector accepts `codex.v2` as Q3's
   answer shapes it: with `signIn` and `/account` as in `claude-code.v2`'s item 1 on the fact's pass
   or under Q3's c, or with no `signIn` and `requires` holding `retire-hook.v1` alone on the fact's
   fail under Q3's a, or under Q3's b. Traces to M08, M21, M22, M44 to M46, M54, M69; V01, V03
   (S03), V13; C02.
2. The logins: the browser login, `carried: true`, its literals read from the client at a version
   the README names, `redirect.ports` the fixed list read from the client, `issuer` the value the
   source names as `DEFAULT_ISSUER`; and, if Q5's a and Q1 (v) stand, the device login, which the
   hook asked for fails with a diagnostic naming PND-010 and the unbuilt path until F83's code path
   and 051c's panel exist, and never chooses unasked. Done when a package test with a fake client
   shows the browser login's URL passing IFC-049's match, a request for the device login failing
   with that diagnostic, and an unasked sign-in starting the browser login. Traces to M23, M34 to
   M43; V11 (S04), V23 (S10), V24.
3. The item that implements `sign-in-hook.v1` carries Conduit's keys. Done when `conduit.yaml`
   gives that item without `plain: true` and a package test shows `signInLogin` and
   `signInReturn` reaching the hook. Traces to M49; V16.
4. As `claude-code.v2`'s item 4, with `CODEX_HOME`; and `-c cli_auth_credentials_store="file"` on
   `codex login` and on every launch, stated in the README. Done when the marker test passes and a
   test shows the setting in every launch's arguments. Traces to M51, M59 to M62, M88; V02, V15,
   V26, V27 (S11).
5. As `claude-code.v2`'s item 5, with the account hook's diagnostics naming only the step and the
   cause, never file content, a claim or the token; and where the client cannot bind its port, the
   hook's diagnostic names PND-010 and the bind failure, holding no URL value. Done when the two
   canary tests pass and a test with the port held shows that diagnostic. Traces to M49, M52;
   V09, V23, V25 (S20).
6. The description holds exactly the sentences Q1's answer accepts, and no client version under Q1's
   a or b; the browser-opens limit is in the limits section and the accounts guide, not the
   description: on the engine's own runner Codex's browser login may open the default browser of the
   engine user's graphical session, where one exists; the guide tells the person to close that
   window and keep that browser free of an OpenAI session. Done when the description is checked
   against Q1's answer sentence by sentence and the limits and the guide hold the limit. Traces to
   M17, M35, M50, M68, M87; V06, V17 (S15).
7. As `claude-code.v2`'s item 7. Traces to M05, M56, M65 to M67; V18, V28.
8. Before the first undo or retirement of a real login, a desk read of Codex's source and
   documentation, as order 195 read them, for any revocation `codex logout` makes, and its scope.
   Pass and fail as `claude-code.v2`'s item 8, the fail removing the home's credential without the
   client command. Done when 050's report records the read and its outcome. Traces to M53, M54;
   V10.

### 050, the connection Conduit manages for a Claude sign-in

1. Scope and endpoints: for logins stating `carried: false`, and nothing changes for
   `carried: true`; the channel that carries the return terminates at each end in a program that is
   not Conduit's, the system's own server for the route point 8 names on the node and that route's
   client on the person's side; no process running Conduit's code decrypts, terminates or copies
   it. Done when P-R3-d passes: a canary redirect sent over the managed connection appears in no
   engine, Interface or provider log, record or diagnostic, and the forwarded port's sockets belong
   to the route's own programs, not to a Conduit process. Traces to M78; V07 (S07).
2. A forward opens only after IFC-049 relayed this sign-in's URL, for the port that URL's
   redirect_uri names, to that node's loopback only; it closes when `sign-in.v1` ends, signed in,
   failed or at DL18; where the local port cannot be bound, the Interface says so, naming the port,
   and opens no forward on another port. Done when a scripted sign-in with a fake client on a fixed
   port shows the forward absent before the relay, present during the wait and gone after the end.
   Traces to M78, M27; V08.
3. If Q5's a and Q1 (vi) stand: no engine or Interface process holds the code-page login's code, and
   nothing records it, as PND-014 holds for a carried value. Done when a fake-client test shows a
   canary code in no engine or Interface result, record, diagnostic or log. Traces to M33, M79; V12
   (S05).
4. The three stale texts give way to point 8's words, as `IN_FORCE.md:216` owes: the second
   sentence of the core `carried` description, IFC-049's sentence for `carried: false`, and the
   limits' two route sentences. Done when no sentence in `spec/` says that a `carried: false` login
   completes only with a browser on the program's machine or that an SSH forward is only the
   person's own. Traces to M74 to M76; V07's surviving fix; dec-8; known-2 to known-4 (G10).

### 051b

Nothing from this review changes 051b: dec-34's command-line paste is the `carried: true` route
under PND-013 and serves no Claude Agent's sign-in, whose on-node delivery passes through no
Conduit process (G04, Lens A confirmed; V07, V12).

### Carriers outside the held orders

- **F96, a named spec fix before 048a-2 is drafted:** the request state is bound to the
  authenticated Interface and the originating invocation; "session" is dropped at
  `spec/interface/implementation.yaml:1393-1394` and `:1435`, `spec/promises/pending.yaml:207` and
  `spec/core/operations.yaml:1385`; the removed binding is logged. Done when none of the four
  places holds the word and the log row exists. Traces to M91; V22.
- **F95, a named spec fix landed with 050's rewrite of the limits that `IN_FORCE.md:216` owes:**
  the limits and IFC-049's observable gain F95's sentence: "The connection Conduit manages for a
  Claude sign-in does not bind the person who completes the login; the expected account (PND-011)
  remains the only binding." Done when both texts hold it. Traces to M90; V22 (S14).
- **The probes (G06, with S09 and S10):** stage 1's first step, Q3's option a, is pre-registered in
  [experiment 027's README](../../../experiments/027-a5-sign-in/README.md) by a small order of its
  own, as order 195 pre-registered stage 0b, written once the owner answers Q3 and before 050's
  Codex half is written, with stage 0's guards and the owner told first. P-R3-c (Codex's port)
  and the Claude two-start probe are optional and gate nothing; if run, they are registered the
  same way. P-R4-4 (the opener library) may run at no owner cost and gates nothing; no order is
  named for it.

## 7. The parked list

Ten lines, for the owner; 34 parked items in all, from the readers (28), the skeptic (2), the
triage's one N finding and the verifiers' outcomes (3). The 13 not listed are counted in
[section 8](#8-how-it-ran).

1. 75 C5's browser default for Codex: its login opens a browser unconditionally (S16).
2. D80, "tried only here, by the package": stage 1 needs the build it would gate; P-R4-2 (V03, R4).
3. D18, "Codex first" if 050 is cut: at T Codex failed stage 0 and Claude passed (R4).
4. B0-11: the engine's NAM-024 install check compares only defaults (R1; ruling 46 point 4).
5. NAM-024 could say in words which additions count: a login, an offers entry (V14, V33, G02).
6. The dogfood store's branch-built 0.2.0 shares 050's address and version (known-14; B0, R1, R4).
7. The accounts guide could say the configured account is the identifier the client attests (V29).
8. Stale texts, cited, not found again: `accounts.md:17-18, 39-40`, `operations.yaml:1376-1378`.
9. Section D item 1's SEC-010 reading (R2) and F143's line (R4): both now reach the owner in Q2.
10. dec-8 with 75 C4: "opens" read as from the engine's machine toward a node (R3).

## 8. How it ran

**T and the hashes.** T is `0db2951c88879cfa70338f39775b6a96076486e8`, `origin/main` after orders
171a, 171b and 195 merged (#334). The branches, by hash only: `65a999f0` (order 050a), `dfcef3ec`
(order 076) and `559ecd0b` (order 078, stacked on 076), all on base `50e7cc5c`. The members hash at
T: `cb1248b6217ce6c694dee20442d91729412e8445ef925e69a406251f9834cd4c`, over 91 rows. After T, only
order 193b merged to `main` (#335), before this record: the local gate's Rust stage and the
documents on how merges are checked. It changes no file the record cites, so no verdict is reopened
for it.

**How it ran.** As a workflow of Fable agents from the coordinator's session, the run mode the
owner chose ([ruling 81's later answers](../ruling-81/README.md#later-answers)). Opus did Phase 0
and this record. Phase 1: five readers (B0, blind; R1 to R4). Phase 2: the triage, seven Lens A
and three Lens B verifiers, and a third check of the three survivors that would have changed the
answer (V03, V20, V23). Phase 3: the completeness critic and the skeptic, then a Lens A and a
Lens B verifier for the critic's six costliest gaps. Every planned agent ran and finished; none
was rerun.

**Each agent's model, minutes and tokens** (from the run's journal; minutes from its start and end
times, which ran from 01:38 to 03:25 UTC on 2026-11-17, the evening of 2026-11-16 local time, about
1 hour 47 minutes of wall time):

| Agent | Phase | Model | Minutes | Input | Output | Cache write | Cache read |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| B0 | Readers | Fable | 21.7 | 1,032 | 78,076 | 309,520 | 3,390,446 |
| R1 | Readers | Fable | 32.0 | 682 | 140,302 | 1,043,514 | 3,394,053 |
| R2 | Readers | Fable | 21.4 | 551 | 93,215 | 847,182 | 2,342,064 |
| R3 | Readers | Fable | 24.0 | 454 | 105,676 | 775,388 | 1,767,935 |
| R4 | Readers | Fable | 26.4 | 422 | 115,027 | 853,063 | 1,725,430 |
| Triage | Triage | Fable | 22.7 | 356 | 122,478 | 415,532 | 1,077,787 |
| A1 | Verify | Fable | 9.3 | 585 | 34,677 | 177,023 | 1,839,560 |
| A2 | Verify | Fable | 9.8 | 452 | 38,447 | 144,642 | 1,346,732 |
| A3 | Verify | Fable | 9.6 | 675 | 35,445 | 142,907 | 1,980,694 |
| A4 | Verify | Fable | 11.8 | 649 | 44,857 | 188,625 | 2,452,823 |
| A5 | Verify | Fable | 9.3 | 612 | 34,377 | 136,789 | 1,679,001 |
| A6 | Verify | Fable | 12.0 | 615 | 45,306 | 176,082 | 2,223,720 |
| A7 | Verify | Fable | 5.7 | 323 | 21,391 | 75,457 | 573,258 |
| B1 | Verify | Fable | 12.7 | 713 | 46,448 | 200,167 | 2,784,458 |
| B2 | Verify | Fable | 14.1 | 809 | 51,527 | 367,267 | 2,978,779 |
| B3 | Verify | Fable | 13.2 | 775 | 47,079 | 149,159 | 2,337,116 |
| RA1-1 | Reserve | Fable | 7.9 | 323 | 32,389 | 136,182 | 877,785 |
| RB1-1 | Reserve | Fable | 8.2 | 777 | 26,392 | 143,232 | 2,293,404 |
| Critic | Critic and skeptic | Fable | 18.6 | 517 | 79,018 | 548,466 | 2,773,102 |
| Skeptic | Critic and skeptic | Fable | 18.0 | 487 | 70,205 | 465,067 | 2,306,720 |
| GA1 | Gaps | Fable | 8.1 | 579 | 27,097 | 137,491 | 1,478,552 |
| GB1 | Gaps | Fable | 10.9 | 713 | 38,226 | 173,260 | 2,488,108 |
| **22 agents** | | | **327.4** | **13,101** | **1,327,655** | **7,606,015** | **46,111,527** |

Phase 0, the drafting agent, the checkers and the fixers ran on Opus, outside the journal; tokens
not recorded. Their wall-clock minutes were not recorded either; from the run's file times, Phase
0's output was written at 01:38 UTC, its start not recorded; the drafting agent ran after 03:25 UTC
and wrote its last file at 03:45 UTC, at most about 20 minutes; the first checker returned at 03:58
UTC, about 13 minutes later. The first fixer wrote its account at 04:06 UTC, about 8 minutes after
the check, a second Opus checker read those fixes at 04:11 UTC, a second fixer applied its findings
by 04:13 UTC, a third Opus checker read those fixes at 04:15 UTC, a third fixer applied its findings
by 04:17 UTC, and a fourth Opus checker read those fixes at 04:19 UTC and found three shoulds. The
coordinator applied them with three edits of its own by 04:20 UTC, and a fifth Opus checker read the
result at 04:31 UTC and found eight shoulds; the coordinator applied them by 04:33 UTC; a sixth Opus
checker read those fixes at 04:41 UTC and found six shoulds; the coordinator applied them by 04:44
UTC, and a seventh fresh Opus reader read those fixes before the merge, its outcome and any last fix
recorded with the run's working files.

**The owner's time.** Phase 0's notice: about a minute, with nothing to answer. The run mode and
the three up-front questions were answered on 2026-11-16 through the question tool, minutes not
measured. Nothing else: no sign-in, no browser, nothing to watch.

**Findings raised, refuted and dropped.** The five readers raised 57 findings (B0 12, R1 11, R2 12,
R3 12, R4 10). The triage merged them into 30: 29 marked V and 1 marked N (B0-11, not verified; not
a member). It added 9 V items for marks, defers and changes with no finding behind them, 38 V items
in all, verified in 7 Lens A and 3 Lens B batches. 34 survived and 4 were struck (V29, V30, V35,
V36). The reserve: 4 of about 4 agents, two for the third check of V03, V20 and V23 (all narrowed)
and two for the critic's gaps G01 to G06 (G03 refuted; the rest survived). The skeptic's flags then
dropped 3 survivors from the record (V04, V05, V14) and merged 2 into others (V33, V37). The
answer's tier as the workflow counted it was 3.

**Phase 0's flags.** F1: order 171b, at the coordinator's choice open to the owner's strike, gives
every held request a diagnostic naming PND-005 (`spec/promises/pending.yaml:155-156`); F84's
trigger keeps its words, now listed fourth. F2: order 172b changed PRV-008, PRV-030 and DL16 (a
stop ends only the processes in the unit by which the provider stops the run), which govern order
16's stop (M64). F3: order 195's results cite seven URLs of four public source and documentation
pages, as order 195 requires; Phase 0 read its stop on "a URL" as a login's URL. F4: order 172a
changed IFC-003 and IFC-046, so that after a version error a call is resent only for an operation
with no effect that is not a control and starts no model work; the sign-in and sign-out hook parts
of PND-010, PND-011's undo and PND-013's retry now end unknown instead of being resent.

**Deviations and departures.**

1. **The answer's tier.** The workflow counted tier 3 mechanically, from V03, V20 and V23. The
   skeptic's S09 and S10 narrow V20 and V23 to tier 2, so the one wait on a fact is Q3's, and
   decided section D item 9 has the answer stated under each of Q3's options (S22). The front page
   and o7 say so.
2. **The saved files.** Four agents' own saved files (R3, the triage, the critic and the skeptic)
   differ from the returns the workflow kept. The record uses the returns, which the later phases
   used; it read the skeptic's saved file only for fields its return lacks (the dispositions of
   dropped questions, the parked additions and what it did not review).
3. **Who drafted.** An Opus agent drafted this record from the run's working files; the coordinator
   read it and committed it under the work-order identity, and a fresh Opus checker read it against
   the evidence (ruling 79), fresh Opus readers read each round of fixes, and a last fresh reader
   read the coordinator's own fixes.
4. **G02, one reading of NAM-024.** G02's verifiers split: Lens A confirmed the critic's fix
   (the strict reading, with one owner question), and Lens B narrowed it to the open reading (a
   login added later is an allowed addition; M43 and M33 defer). Neither reading is confirmed, so
   the reading goes to the owner as Q5, with the skeptic's strict reading (C01, S04) as option a
   and its recommendation, and Lens B's as option b. Q5 is not in the skeptic's ranking, which
   ran before G02 was verified; the record adds it after Q1 to Q4 for this reason.
5. **V05 and A81.** run.md lets only a not-safe mark that Lens A confirmed or narrowed make a gate,
   and carries a must-level item that Lens B refutes forward as should, changing no verdict; Lens A
   narrowed V05 and Lens B refuted it. The workflow marked it `makes_gate: false` after Lens B
   refuted it; the record follows the workflow, the critic's handed table and the skeptic (S01), so
   A81 gates nothing (G09, not covered).
6. **Skeptic's drops and merges.** V04, V05 and V14 survived Phase 2 but leave the findings (S01,
   S02), each listed in one line; V33 falls into V11 (S04) and V37 into V21 (S13); V11 is carried
   at should, as S04 states.
7. **Verdicts and their grounds.** M74 to M76 change as V07's surviving fix states, 050
   rewriting the three stale texts under `IN_FORCE.md:216` as already owed (dec-8; known-2 to
   known-4), so they need no departure (G10). B0-11 gets no verdict: it is a finding about the
   engine, not a member (the critic's table). M47's defer is conditional on Q2, and M23 changes
   or freezes as is, and M33 and M43 change or defer, as Q5 decides.
8. **Choices among the survivors' options.** The record picks `conflict` for o4 (S13); 048a-1 for
   the fixture's realignment, of G06's two carriers; and no order for P-R4-4 (S09). The managed
   forward is scoped to the port that the relayed URL's redirect_uri names, not V08's "port and
   path": a forward of a port carries any path, and limiting it by path would need a process that
   reads the request, which the connection's item 1 forbids.
9. **The critic's gaps.** Checked, with their verifiers' outcomes in
   [findings.md](findings.md#gap-findings-verified-in-the-reserve): G01 (Lens A confirmed, Lens B
   refuted: goes forward as should, changing nothing), G02 (as point 4), G03 (refuted), G04, G05
   and G06 (Lens A confirmed). Not covered, beyond the reserve, each in one line where it bears:
   - G07: whether the turn item carries Conduit's keys, on which #17's field carrier depends (o1;
     M18, M66; 050 item 7).
   - G08: V03's and V12's claims and fixes disagree on P-R4-2 and on M33's route to the owner;
     P-R4-2 is parked (point 13) and M33 turns on Q5 (Q3; M33).
   - G09: which lens governs M41's A81 mark (section 2 (c); M41).
   - G10: M74 to M76 change on V07's surviving fix, with dec-8 and known-2 to known-4 (o2; M74
     to M76; the connection's item 4).
   - G11: whether D53's refusal is scoped to `.v2` launches as the home variables are (M61; 050
     item 4).
10. **The flags of the critic and the skeptic** are applied throughout: the critic's member flags
    in [verdicts.md](verdicts.md), its *Open* and handed lines in sections 1 and 2, and S01 to S22
    and C01 to C12 where the findings and lists cite them. C12's merges keep each 050 list within
    eight items. Q4's rule to merge into Q3 is not used, since five questions fit the cap.
11. **The parked items not listed (13):** B0's notes on `toolsDirectory`, F95 and F96,
    `peerMessaging` and `wrapper-v2-path`, and the vendors' terms; R1's on `effort`'s pattern and
    D5's "v2 only"; R2's on known-21, the SSH runtime's `HOME` and decision 25 in
    `claude-code.v2`; R3's on Phase 0's F3 and part B's rule; R4's on decision 20's wording and the
    domain-of-trust status order.
12. **What the readers did not review**, in their own lines: none ran a client, a build or a test,
    and none read a credential, keychain, login file or the environment; each left the others'
    areas to them; B0 read nothing of the packet.
13. **P-R4-2 parked, against S03.** S03 keeps the owner's one-bit read of the Codex account claim
    as Q3's option b. It strikes D80 (dec-29), which the brief does not reopen, and V03's surviving
    claim calls it a parked note at most; the record parks it (line 2), as S16 and S17 drop
    options on the same ground.
14. **M90's carrier.** The record carries F95's sentence by a named spec fix landed with 050's
    rewrite of the limits, one of the output contract's engine-side carriers, where the critic's
    table named 050.
