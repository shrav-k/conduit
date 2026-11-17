# The verdicts: every member of the 0.2.0 surface

Section 3 of the [record](README.md). Not binding. One entry for each member M01 to M91 of the
hashed members list (91 rows; its hash at T is
`cb1248b6217ce6c694dee20442d91729412e8445ef925e69a406251f9834cd4c`); no reader added a member:
B0-11, the one item beyond M01 to M91, is a finding about the engine, not a member (the critic's
table). Each entry gives the verdict, the carrier of a change, any probe with the change under
each outcome, and the marks for A81 (`ROADMAP.md:73`) and A22 (`ROADMAP.md:67`): "safe" where some
outcome of the probe was weighed and every outcome leaves the member valid, "not bearing" where the
probe does not touch it. No member is not safe without either probe ([section 2
(c)](README.md#2-the-three-questions-the-owner-left-to-the-review)).

The verdicts are the completeness critic's supported verdicts, with the skeptic's flags applied
(S01, S02, S04, S05, S10, S13). "Vnn" and "Gnn" are the items in [findings.md](findings.md).
Sources are cited at T (`0db2951c`) or as `hash:path:line` on the branches. Where a verdict
waits on one of the owner's questions, the question is in
[section 5](README.md#5-the-owner-questions).

Counts: 91 members. Freeze as is: 53. Change before 050: 35. Defer: 3 (M06, M47, M48). M23, M33
and M43 change under Q5's a; under its b, M23 freezes as is and M33 and M43 defer.

## A. The release and the contracts it carries

- **M01**, the package addresses and version 0.2.0. **Freeze as is**: no finding touches it;
  NAM-024 governs a later release at the same address (`spec/promises/naming.yaml:414-440`).
  A81 not bearing; A22 not bearing.
- **M02**, the exports of 0.2.0 (both `.v1` and both `.v2` Types, `conversation.v1`,
  `turn.v1`). **Freeze as is**: V14 is dropped (S02) and M06 defers (V32), so the exports block
  no later addition. A81 not bearing; A22 not bearing.
- **M03**, the `.v1` Types and the `wrapper` item carried unchanged
  (`559ecd0b:packages/agent-codex/contracts.yaml:1-27`, `conduit.yaml:10-19`). **Freeze as is**:
  V26 keeps the `.v1` launch's environment as 0.1.1 gave it, with a package test (M88). A81 not
  bearing; A22 not bearing.
- **M04**, the capability `conversation.v1`, operations `['#turn.v1']`. **Freeze as is**: a
  further operation and its implementation defaults can land together in a later release (V32).
  A81 not bearing; A22 not bearing.
- **M05**, the operation `turn.v1`, its closed input and output shared with `.v1`
  (`559ecd0b:packages/agent-codex/contracts.yaml:36-75`). **Freeze as is**: orders 16 and 17 put
  nothing into `turn.v1`'s schemas (V18, narrowed by S06). A81 not bearing; A22 not bearing.
- **M06**, a slot for a further conversation operation (the full review's order 62).
  **Defer**, under NAM-024's "a further operation of a capability" (`naming.yaml:420-422`). Safe:
  the release that adds the operation adds the implementation default TPL-004 needs in every Type
  that offers `conversation.v1` (`templates.yaml:76-110`), which removes or changes nothing 0.2.0
  holds; order 62 is not held (`docs/proposals/ruling-81/README.md:95-97`). Residual reading: that
  an implementation entry is an "optional thing" reads NAM-024's "such as" as open (V32; parked).
  V14, the opposite reading, is dropped (S02). A81 not bearing; A22 not bearing.

## B. The `.v2` Types: heads, inputs, descriptions and turn wrappers

- **M07**, `claude-code.v2`'s head (`559ecd0b:packages/agent-claude-code/contracts.yaml:100-101,
  113-116`). **Change before 050**, carrier 050: the head states `signIn` with its required
  account place, and `requires` lists `sign-in-hook.v1`, `account-hook.v1`, `sign-out-hook.v1`
  and `retire-hook.v1`, each with a default in `implementation`; `records` stays
  `{inputs: false, outputs: false}` (V01; V09). A81 not bearing; A22 not bearing.
- **M08**, `codex.v2`'s head (`559ecd0b:packages/agent-codex/contracts.yaml:103-104, 115-118`).
  **Change before 050**, carrier 050, waiting on the Codex account fact (Q3): on the probe's pass or
  under Q3's c, as M07; on its fail under Q3's a, or under Q3's b, no `signIn`, and `requires` holds
  `retire-hook.v1` alone with its default (V01, V03, C02). A81 not bearing; A22 not bearing.
- **M09**, the input `directory`. **Freeze as is**: no finding. A81 not bearing; A22 not bearing.
- **M10**, the input `turnDeadlineSeconds`. **Freeze as is**: no finding. A81 not bearing; A22
  not bearing.
- **M11**, the input `effort`. **Freeze as is**: no finding (a reader's note on its pattern is
  parked). A81 not bearing; A22 not bearing.
- **M12**, the input `accountServices`. **Freeze as is**: no finding. A81 not bearing; A22 not
  bearing.
- **M13**, the input `model`. **Freeze as is**: no finding. A81 not bearing; A22 not bearing.
- **M14**, the input `peerMessaging` on `claude-code.v2`. **Freeze as is**: no finding. A81 not
  bearing; A22 not bearing.
- **M15**, the schema's closure (every property required, `additionalProperties: false`).
  **Freeze as is**: 050 adds `account`, and `apiKey` as Q2 answers, in the same closed form (V01,
  V13). A81 not bearing; A22 not bearing.
- **M16**, `claude-code.v2`'s description
  (`559ecd0b:packages/agent-claude-code/contracts.yaml:81-99`). **Change before 050**, carrier 050,
  as the owner answers Q1 (dec-36): it names no client version; it carries decision 25's sentence on
  another runtime provider with "On the engine's own runner" before the engine-user sentence; it
  keeps or drops "fails and ends the conversation" as Q1 decides; any sentence on the retire hook
  says the login is kept where the client keeps it for that directory and names no store; it lists
  the code-page login if Q1 (vi) stands (V17, V19, S15). A81 not bearing; A22 safe on that wording
  (V19).
- **M17**, `codex.v2`'s description (`559ecd0b:packages/agent-codex/contracts.yaml:81-102`).
  **Change before 050**, carrier 050, as the owner answers Q1: no client version (it names
  "Codex 0.167.0" at `:93-95`); the "ends the conversation" clause as Q1 decides; it names the
  browser login as the default and lists the device login if Q1 (v) stands; the browser-opens
  limit stays out of it (V06, V17, S15). A81 not bearing; A22 not bearing.
- **M18**, the `.v2` turn item `wrapper-v2`, `plain: true`. **Freeze as is**: `plain` is a
  manifest member that NAM-024 does not freeze; only the default's name is frozen. Whether the
  turn item carries Conduit's keys is 050's choice (V16, S12); G07, not covered, notes that #17's
  field carrier depends on it. A81 safe; A22 not bearing.
- **M19**, Codex's second turn item `wrapper-v2-path`. **Freeze as is**, as M18. A81 safe; A22
  not bearing.

## C. `signIn` and its logins

- **M20**, `claude-code.v2`'s `signIn.account`, planned as `/account`. **Freeze as is**: 050
  states the account place exactly as CTR-013 has it (`spec/promises/contracts.yaml:290-291,
  296-297`; V01). F97's control sentence is engine-side and goes to 048a-1 (V38, C06). A81 not
  bearing; A22 safe.
- **M21**, `codex.v2`'s `signIn.account`, and whether `codex.v2` states `signIn` at all. **Change
  before 050**, carrier 050, waiting on the Codex account fact (V03; Q3). The probe, as Q3's
  recommended option: the package's account hook, in a scratch home signed in through Conduit's
  Interface flow (stage 1's first step, pre-registered), reports the signed-in account, before any
  archive whose `codex.v2` states `signIn` is installed anywhere. Pass: the hook reports an account
  equal, ignoring ASCII case, to the signed-in account, no value recorded; `codex.v2` states
  `signIn` with `/account`. Fail: `codex.v2` states no `signIn` (CTR-013, `contracts.yaml:305-310`),
  Codex is configured by `apiKey` where Q2 keeps it (under Q2's a, only once batch 2 writes B13's
  text), and FR-118 returns to the owner (S03). Under Q3's b, `codex.v2` states no `signIn`, with no
  probe, and that choice is the owner's own answer to FR-118 for Codex (C03); under Q3's c, as on a
  pass, with no probe before the freeze, and a later fail costs a `codex.v3` (Q3's c). A81 safe; A22
  not bearing.
- **M22**, both `.v2` Types' `requires`. **Change before 050**, carrier 050: `requires` lists
  `sign-in-hook.v1`, `account-hook.v1`, `sign-out-hook.v1` and `retire-hook.v1`, each with a default
  (`spec/schemas/contracts.yaml:212-223`; `templates.yaml:104-110`); for `codex.v2` on the account
  fact's fail under Q3's a, or under Q3's b, `retire-hook.v1` alone (V01). A81 not bearing; A22 not
  bearing.
- **M23**, each Type's `logins` as a whole. **Change before 050 under Q5's a; Freeze as is under
  Q5's b**, carrier 050: under b, each Type's list holds its one login, the redirect login for
  `claude-code.v2` and the browser login for `codex.v2` where it states `signIn` (Q3), as CTR-013
  has it (`spec/promises/contracts.yaml:290-302`), and the page and device logins defer (M33, M43);
  under a, `claude-code.v2`'s list holds the redirect login and the code-page login (M33);
  `codex.v2`'s, where it states `signIn` (Q3), holds the browser login and the device login (M43);
  every value read from the client at a stated version. Q5's a is the skeptic's NAM-024 reading
  (C01; [section 1, o6](README.md#1-the-open-line-clause-by-clause)): a login added later changes
  what an unchanged sign-in may start, so it is stated now or not within `.v2` (V11, V12). A81 safe;
  A22 not bearing.
- **M24** to **M30**, Claude's redirect login: `name`, `return: redirect`, `redirect.host`,
  `redirect.ports`, `redirect.path`, `authorization`, `clientId`. **Freeze as is**: 050 states
  each value it reads from the client at a stated version, within the cited shape
  (`spec/schemas/contracts.yaml:378-458`). For M27, the ports: `any` where the port varies across
  starts, the fixed list where it does not; a two-start probe recording only same or different,
  no value, is the least way to learn which, and `any` leaves FR-433's any-port case to M89 (V19).
  V08 bounds the forward over that port, engine-side (M78). A81 not bearing; A22 not bearing.
- **M31**, Claude's redirect login, `carried: false`. **Freeze as is**: every reader agrees; V07
  scopes the managed connection's condition to it. A81 not bearing; A22 not bearing.
- **M32**, Claude's redirect login, `issuer`. **Freeze as is**: optional; nothing turns on it for
  `carried: false` (PND-013's first check, `pending.yaml:251-252`; V24). A81 not bearing; A22 not
  bearing.
- **M33**, Claude's code-page login. **Change before 050 under Q5's a; Defer under Q5's b
  (`naming.yaml:420-422`, safe as G02's Lens B states)**, carrier 050, open to the owner's strike as
  Q1's sentence (vi): `claude-code.v2` states Claude's code-page login, `return: page`, `carried:
  false`, its page URL and `clientId` read from the client at a stated version, as a second route
  for point 8's paste; the hook relays the redirect login unless `signInLogin` asks for the page
  login (V12, narrowed by S05). Under Q5's a it is stated now or not within `.v2`. A81 not bearing;
  A22 not bearing.
- **M34**, Codex's browser login, `name`. **Freeze as is**: read from the client at a stated
  version. A81 not bearing; A22 not bearing.
- **M35**, Codex's browser login, `return: redirect`. **Freeze as is**: the browser that Codex
  opens is a stated limit in unfrozen text, not a literal (V06). A81 not bearing; A22 not bearing.
- **M36**, Codex's browser login, `redirect.host`. **Freeze as is**: read from the client. A81
  not bearing; A22 not bearing.
- **M37**, Codex's browser login, `redirect.ports`. **Freeze as is**: 050 states the fixed list
  it reads from the client, whose source sets the port from a named constant
  (`experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:207-211`); a fallback bind shows
  as an explicit failure, not a wrong literal; P-R3-c is optional and gates nothing (V23, narrowed
  by S10). A81 not bearing; A22 not bearing.
- **M38** to **M40**, Codex's browser login: `redirect.path`, `authorization`, `clientId`.
  **Freeze as is**: read from the client at a stated version. A81 not bearing; A22 not bearing.
- **M41**, Codex's browser login, `carried: true`. **Freeze as is**: every outcome of A81 leaves
  `carried: true` valid; a fail ends `returned: undelivered` and polling continues
  (`pending.yaml:278-282`), the account check alone decides completion (`:208-210`), and a fail goes
  to the owner as a finding against rulings 73 and 75 C5
  (`docs/proposals/ruling-73/README.md:58-61`). A81 stays owed before S3 (`IN_FORCE.md:215`). The
  not-safe marks of R3, R4 and B0 fell with V04 and V05 (S01; Lens A's case, not adopted, is in
  [section 2 (c)](README.md#2-the-three-questions-the-owner-left-to-the-review)). A81 safe; A22 not
  bearing.
- **M42**, Codex's browser login, `issuer`. **Freeze as is**: 050 states `issuer` from the value
  the client's source names as `DEFAULT_ISSUER`, read at the stated version; it is harmless where
  the server sends no `iss` (`pending.yaml:259-260`), and it is stated at 0.2.0 or not within
  `.v2` under either reading of NAM-024 (V24; G02). A81 not bearing; A22 not bearing.
- **M43**, `codex.v2`'s device login, 75 C5's option. **Change before 050 under Q5's a; Defer under
  Q5's b (`naming.yaml:420-422`, safe as G02's Lens B states)**, carrier 050, reaching the owner as
  Q5, and its description sentence as Q1's sentence (v): `codex.v2` states the device login now, its
  literals read from the client at a stated version; until F83's code path and 051c's panel exist,
  the hook asked for it fails with a diagnostic naming PND-010 and the unbuilt path, and never
  chooses it unasked; the description names the browser login as the default (V11; S04 and C01,
  under which V33's defer falls; under Q5's b, V33's defer holds). A81 not bearing; A22 not bearing.

## D. `account` and `apiKey`

- **M44**, the input `account`. **Change before 050**, carrier 050: `account` is required, with
  no default; a string or null under the fixture's `oneOf` where Q2 puts `apiKey` in, and a
  non-empty string where Q2 leaves it out (V01; V13). A81 not bearing; A22 not
  bearing.
- **M45**, the input `apiKey`. **Change before 050**, carrier 050, waiting on Q2 (an owner's
  answer, not a fact): under Q2's a, b or d, `apiKey` is a required nullable place; under c, none
  (V13). It can enter only now: MAN-017 binds only a required place
  (`spec/promises/manifest.yaml:363-372`) and NAM-024 lets a later release add only an optional
  property without default. A81 not bearing; A22 not bearing.
- **M46**, how one of `account` and `apiKey` is chosen. **Change before 050**, carrier 050,
  waiting on Q2: where `apiKey` is in, both are required with the fixture's `oneOf`; where it is
  out, no `oneOf` and `account` is a non-empty string (V13). A81 not bearing; A22 not bearing.
- **M47**, the binding `AGENT_API_KEY: /apiKey`. **Defer**, conditionally: a manifest binding is
  outside NAM-024's `when` (`naming.yaml:417-419`), and MAN-017 accepts it later only where 0.2.0
  already holds `apiKey` as a required place; so it defers safely under Q2's a, b or d, and under
  c there is no `apiKey` to bind within `.v2` (V34, V13). A81 not bearing; A22 not bearing.
- **M48**, the binding `AGENT_ACCOUNT: /account`. **Defer**: a manifest binding outside NAM-024's
  `when` (`naming.yaml:417-419`), pointing at a place 0.2.0 holds required, so MAN-017 accepts it
  when a later release adds it; PND-011 compares the attested account itself, so the hint changes
  no meaning (V34). A81 not bearing; A22 not bearing.

## E. The hooks

- **M49**, each `.v2` Type's `sign-in-hook.v1` implementation. **Change before 050**, carrier
  050: the item that implements `codex.v2`'s sign-in hook carries Conduit's keys, and so does
  `claude-code.v2`'s where it states more than one login (V16); the hook takes its URL from the
  login it started, writes no client output into a result, diagnostic, log or record, and names a
  failure to bind the client's port as its cause, holding no URL value (V25, V23, S20). A81 safe;
  A22 not bearing.
- **M50**, how the sign-in hook gets each client's URL without a browser opening. **Change before
  050**, carrier 050: for Codex, the limits section 050 rewrites and the accounts guide say that
  on the engine's own runner Codex's browser login may open the default browser of the engine
  user's graphical session, where one exists; the guide tells the person to close that window and
  keep that browser free of an OpenAI session; the hook still relays the printed URL under IFC-049
  and the account check stays the only guard (V06). For Claude, the hook takes the URL of the
  login it started (V25). The clients' homes follow V02. A read of the opener library (P-R4-4) may
  run at no owner cost and narrows the guide's words where it passes; it gates nothing (V20,
  narrowed by S09). A81 not bearing; A22 not bearing.
- **M51**, Codex's credential store. **Change before 050**, carrier 050: the wrapper passes
  `-c cli_auth_credentials_store="file"` on `codex login` and on every launch, and the README
  says so (V27). A81 not bearing; A22 not bearing.
- **M52**, each `.v2` Type's `account-hook.v1` implementation. **Change before 050**, carrier
  050: its diagnostics name only the step and the cause, never file content, a claim or the token,
  shown by a canary test (V09, S20); for `codex.v2` it waits on the account fact as M21 does (V03;
  Q3). A81 safe; A22 safe.
- **M53**, each `.v2` Type's `sign-out-hook.v1` implementation. **Freeze as is**: its frozen text
  is the core description (`spec/core/operations.yaml:1452-1457`), which names no client command.
  050's run gains one pre-step, a desk read of both clients' logout for any revocation and its
  scope before the first undo or retirement of a real login (V10). A22 bears on no member's freeze
  through this item (S21). A81 not bearing; A22 safe.
- **M54**, each `.v2` Type's `retire-hook.v1`. **Change before 050**, carrier 050: listed under
  `requires` with a default pointing at the wrapper, signing the client out and emptying its home,
  as D53's standing part has it (`docs/proposals/full-review/decisions.md:82`; V01, V02, V10).
  A81 not bearing; A22 safe.
- **M55**, the account check in each `.v2` instance's setup. **Freeze as is**: TPL-012 and
  PND-011 stand; V02 touches it only through the hook's environment. A81 not bearing; A22 not
  bearing.
- **M56**, how a login that lapses during a turn reaches the engine. **Change before 050**,
  carrier 048a-2: PND-005 gains F84's two halves, a keyed implementation ending the call failed or
  unknown with a diagnostic naming PND-005, a plain one only by a URL request (IFC-016); 050's turn
  implementation follows whichever half matches its item (V28, narrowed by S12). A81 not bearing;
  A22 not bearing.

## F. Each Agent's home

- **M57**, Claude's home, `CLAUDE_CONFIG_DIR` under `CONDUIT_STATE_DIR`. **Change before 050**,
  carrier 050: every `.v2` launch and hook runs the client with `CLAUDE_CONFIG_DIR` inside
  `CONDUIT_STATE_DIR`, read from the environment alone (V02, S11). A81 not bearing; A22 safe.
- **M58**, where Claude keeps its login on macOS. **Freeze as is**, on neutral wording: no frozen
  text names the store; the login is kept where the client keeps it for that directory, and the
  README may name the keychain at a stated client version (V19). A81 not bearing; A22 safe on that
  wording.
- **M59**, Codex's home, `CODEX_HOME` under `CONDUIT_STATE_DIR`, its login in a file. **Change
  before 050**, carrier 050: as M57 with `CODEX_HOME`, and the file store of M51 (V02, V27). A81
  not bearing; A22 not bearing.
- **M60**, the `HOME` given to the client. **Change before 050**, carrier 050: `HOME` stays as
  PRV-023 passes it (`spec/promises/providers.yaml:746-751`); the client-home variables of M57 and
  M59 are set for every `.v2` launch and hook (V02). A81 not bearing; A22 not bearing.
- **M61**, the package refuses a client home outside the instance's state directory (D53).
  **Change before 050**, carrier 050: the wrapper refuses to start a client, a hook or a logout
  where `CONDUIT_STATE_DIR` is unset or the home resolves outside it, with a diagnostic naming the
  cause (V15, V02, S11). G11, not covered: whether the refusal is scoped to `.v2` launches as V26
  scopes the home variables. A81 not bearing; A22 not bearing.
- **M62**, where `CONDUIT_STATE_DIR` comes from. **Change before 050**, carrier 050: the wrapper
  reads it from the environment alone; no item binds it (MAN-018; V02). A81 not bearing; A22 not
  bearing.
- **M63**, a home on Windows or under another runtime provider. **Freeze as is**: decision 25's
  sentence is added to `claude-code.v2`'s description (M16), not to this member (V17). A81 safe;
  A22 not bearing.

## G. The wrappers' members that the full review's orders 16 and 17 add

- **M64**, order 16's stop of the client's whole process tree. **Freeze as is**: no finding
  (Phase 0's flag F2 records 172b's change to PRV-008 and PRV-030). A81 not bearing; A22 not
  bearing.
- **M65**, order 16's report of what survived. **Change before 050**, carrier 050: the survivors
  are named where #17's fields live and, on TERM, in the wrapper's own log (V18). A81 not bearing;
  A22 not bearing.
- **M66**, order 17's failed turns as fields. **Change before 050**, carrier 050: `turn.v1`'s
  schemas stay byte for byte; the class and the turn are not outputs; done when a vector shows the
  fields reach the caller under IFC-025 wherever 050 places them (V18, narrowed by S06; V28). G07,
  not covered: the carrier depends on whether the turn item is keyed. A81 not bearing; A22 not
  bearing.
- **M67**, the class of a failed turn. **Freeze as is**: the classes are named in the README and
  enumerated in no schema or description (V18, V28). A81 not bearing; A22 not bearing.
- **M68**, a failed turn ends the conversation. **Change before 050**, carrier 050, as Q1 (iii)
  decides: the clause is kept with D55's un-refusing struck for `.v2`, or dropped (V17). R4 froze
  it; V17's Lens A holds that the clause would freeze D55's open choice. A81 not bearing; A22 not
  bearing.

## H. Own tools and the `turn` template

- **M69**, `toolsDirectory: {directory: /directory}` on both `.v2` Types. **Change before 050**,
  carrier 050, as `IN_FORCE.md:221` already owes (V01, narrowed: for this member the NAM-024
  reading is weaker, the consequence the same). A81 not bearing; A22 not bearing.
- **M70**, the standard template `turn`. **Freeze as is**: a later release may add the template
  (a new named export, `naming.yaml:420-425`); V30, which would have changed it, was struck as
  known-13. The fixture `agents-0.2.0` follows 050's schema (V13; carrier in
  [section 6](README.md#6-must-state-lists)). A81 not bearing; A22 not bearing.
- **M71**, the client's sandbox and approval prompts off when a directory is named. **Freeze as
  is**: no finding. A81 not bearing; A22 not bearing.

## I. Core contracts and `spec/` text the surface is fixed against

- **M72**, `signIn`'s core shape. **Freeze as is**: the managed connection adds no core member
  (V07). A81 not bearing; A22 not bearing.
- **M73**, `sign-in-login.v1`'s core shape. **Freeze as is** (V07). A81 not bearing; A22 not
  bearing.
- **M74**, the core description of `carried` (`spec/schemas/contracts.yaml:420-424`). **Change
  before 050**, carrier 050: its second sentence says point 8's routes, as `IN_FORCE.md:216`
  already owes. As V07's surviving fix states, 050 rewrites the three stale texts under
  `IN_FORCE.md:216` as already owed (dec-8; known-2 to known-4); V35, which raised it as new, was
  struck. A81 not bearing; A22 not bearing.
- **M75**, IFC-049's message for a sign-in (`spec/interface/implementation.yaml:1382-1386`).
  **Change before 050**, carrier 050: its sentence for `carried: false` says point 8's routes, as
  M74 (V07; dec-8; known-3). A81 not bearing; A22 not bearing.
- **M76**, the limits "Pending actions, sign-in and the network" (`spec/README.md:433-440`).
  **Change before 050**, carrier 050: the two route sentences give way to point 8's words (V07;
  dec-8; known-2), and the limits gain the managed connection's endpoint condition (V07), F95's
  sentence (V22, by M90's named spec fix) and the Codex browser limit (V06). A81 not bearing; A22
  not bearing.
- **M77**, IFC-049's match of a hook's URL and the keys `signInLogin` and `signInReturn`.
  **Freeze as is**: F96 drops a word in IFC-049's `then`, not in the match (V22). A81 not
  bearing; A22 not bearing.

## J. The connection Conduit manages for a Claude sign-in

- **M78**, the managed connection, a member with no design. **Change before 050**, carrier 050:
  conditions with tests, no Type or core member: the endpoint condition (V07, narrowed by S07),
  the forward's lifetime and port (V08) and the negative binding sentence (V22, narrowed by S14),
  which M90's named spec fix carries;
  see [section 6](README.md#6-must-state-lists). A81 not bearing; A22 not bearing.
- **M79**, point 8's routes. **Freeze as is**: the three routes stand; a Claude Agent's paste
  route is the person's own on-node delivery of the return, or the code-page login's code (V12).
  A81 not bearing; A22 not bearing.
- **M80**, the scope of "never reads, holds or relays the credential". **Freeze as is**: point 8
  is a Claude Agent's; PND-013 carries Codex's return (V07, V08; the brief's line 1). A81 not
  bearing; A22 not bearing.

## K. The coordinator's decisions that NAM-024 would freeze

- **M81** to **M85**, decisions 20 to 24 (hosted search as an account service; the turn deadline
  as an input; `effort`; the MCP tool timeout; project instruction files off). **Freeze as is**:
  no finding; R1 and R4 agree. A81 not bearing; A22 not bearing.
- **M86**, decision 25, an Agent's own tools under another runtime provider. **Freeze as is**:
  its sentence is copied into `claude-code.v2`'s description (M16), not changed (V17). A81 not
  bearing; A22 not bearing.
- **M87**, decisions 15 and 16, Codex's sub-agents, and the client version in `codex.v2`'s
  description. **Change before 050**, carrier 050, as the owner answers Q1 (i): the description
  names no client version, and the README records the version read (V17). A81 not bearing; A22
  not bearing.

## L. Rows added on the packet's checks

- **M88**, the `.v1` Types in 0.2.0 and the home their wrapper gives the client. **Freeze as
  is**: the home variables are set only for a `.v2` launch, and a package test shows the `.v1`
  launch's environment unchanged from 0.1.1 (V26). A81 not bearing; A22 not bearing.
- **M89**, the refusal reason for a second `sign-in.v1` of one instance. **Change before 050**,
  carrier 048a-2: one sentence in PND-010's `then` after the fixed-port sentence: a `sign-in.v1`
  for an instance whose earlier `sign-in.v1`, setup or retirement has been dispatched and has not
  ended is not dispatched and reports `conflict`, with a diagnostic naming PND-010 and, where the
  caller may see it (PND-002), the other invocation's pending action; the same in `sign-in.v1`'s
  description; one vector; MC9a registered with that reason before 048a-2 is dispatched. The
  fixed-port sentence stays `unavailable` (V21, V37, merged by S13; `spec/definitions.md:238-239`).
  Under OPS-023 conflict comes before unavailable, so a second sign-in.v1 of the same instance
  reports conflict even where it also shares a fixed port; the fixed-port sentence's unavailable
  then reaches only a sign-in of another instance.
  A81 not bearing; A22 not bearing.
- **M90**, F95, who opens a relayed login URL. **Change before 050**, carrier a named spec fix,
  F95's sentence, landed with 050's rewrite of the limits that `IN_FORCE.md:216` owes: the limits
  and IFC-049's observable gain: "The connection Conduit manages for a Claude sign-in does not bind
  the person who completes the login; the expected account (PND-011) remains the only binding."
  (V22, narrowed by S14). A81 not bearing; A22 not bearing.
- **M91**, F96, the word "session". **Change before 050**, carrier a named spec fix before 048a-2
  is drafted: the request state is bound to the authenticated Interface and the originating
  invocation, "session" is dropped at `implementation.yaml:1393-1394` and `:1435`,
  `pending.yaml:207` and `operations.yaml:1385`, and the removed binding is logged (V22). A81 not
  bearing; A22 not bearing.

## B0-11, not a member

- **B0-11** (not a member), from the blind reader: the engine's NAM-024 install check compares only
  schema defaults and Requirement-input defaults (`engine/crates/core/src/packages.rs:277-331`;
  `engine/crates/core/src/archives.rs:322-345`), so a later release that grows `requires` or adds a
  required property installs without the diagnostic the promise requires. **No verdict form
  applies**: it is a first-build finding about the engine, not a member 0.2.0 freezes; the triage
  marked it N (it changes no verdict and no held order), so it was not verified. It is parked
  ([section 7](README.md#7-the-parked-list)). A81 not bearing; A22 not bearing.

The blind reader's own members B01 to B19 map to M rows (the critic's table): B01 to M07, B02 to
M08, B03 to M20, B04 to M21 and M52, B05 to M44, B06 to M45 and M46, B07 to M24 to M32, B08 to M41,
B09 to M43, B10 to M18, M22 and M49, B11 to M54, B12 to M57 and M58, B13 to M59 and M51, B14 to
M76 and M78, B15 to M50, B16 to M65 and M66, B17 to M89, B18 to M20 (F97), B19 to M03 to M05.
