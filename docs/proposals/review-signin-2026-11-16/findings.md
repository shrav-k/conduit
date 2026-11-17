# The findings that survived verification

Section 4 of the [record](README.md). Not binding. Each entry is a surviving item of Phase 2 in its
surviving form: where a verifier narrowed it, the narrowed claim and the narrowed fix, which went
forward. Each gives its members and risks, its evidence at T or on a branch by hash, its failure
scenario, must or should, the least fix, the existing system it follows, its carrier, its verifiers'
outcomes, and how the skeptic's flags apply to it (Phase 3). The verdicts these findings carry are
in [verdicts.md](verdicts.md).

Of the 38 V items, 34 survived and 4 were struck. Of the 34, the skeptic's flags take three out of
the record (V04, V05 and V14, [below](#dropped-by-the-skeptics-flags)) and merge two into others
(V33 into V11, V37 into V21), so 29 entries follow. Six gap items of the completeness critic (G01 to
G06) were verified in the reserve; G03 was refuted and the other five survived ([gap
findings](#gap-findings-verified-in-the-reserve)). The level is the one the item carries after Lens
B, or after the skeptic's flag where one changes it (V11).

The claims and fixes keep the items' own words, which use the readers' numbers: "the owner's Q1"
in V01 and V13 is the first reader's question 1, the record's [Q2](README.md#q2-apikey-in-020);
P-R1-a to P-R4-4 are the readers' named probes; R1-01 and the like are the readers' findings.

## Surviving findings

### V01: `signIn`, the hooks under `requires` and `toolsDirectory` enter in 0.2.0 or never in `.v2`

- **Kind and level:** finding; must. Sources: R1-01, R2-02, B0-01, B0-06.
- **Members:** M07, M08, M15, M20, M22, M44, M45, M54, M69.
- **Risks:** K9, K10, K11.
- **Claim:** signIn with its required account place (CTR-013 :296-298), apiKey bound as a required
  default-free place (MAN-017 :363-397; NAM-024 :422-424), and any entry under requires, the three
  sign-in hooks and retire-hook.v1 among them (type-spec :212-223; TPL-004 :98-104; NAM-024
  :424-425), cannot be added to .v2 after 0.2.0 without a new version segment, so 0.2.0 must hold
  them or .v2 never will; the branches hold none of them (requires: [], no signIn, no account, no
  apiKey). For toolsDirectory the same conclusion rests only on the reading that NAM-024's 'optional
  things' are its three examples; IN_FORCE.md:221 already makes the .v2 statement of toolsDirectory
  spec pending for 050, so the practical consequence is the same. B0-06's retire-hook part holds:
  D53's retire-hook sentence stands (dec-34), TPL-004 uses a type's retire-hook default only where
  the type lists it under requires, FR-431 (E04 :341-346) records the keychain item outside the
  directory and the SSH runtime's kept remote directory, and PRV-029 :979-981 says another runtime
  provider supplies a directory only where its contract states it.
- **Evidence:** `spec/promises/naming.yaml:420-425`; `spec/promises/contracts.yaml:296-298`;
  `spec/schemas/contracts.yaml:212-223`; `spec/promises/manifest.yaml:363-397`;
  `spec/promises/templates.yaml:98-110`; `559ecd0b:packages/agent-codex/contracts.yaml:105-118`;
  `559ecd0b:packages/agent-claude-code/contracts.yaml:100-116`; `IN_FORCE.md:215`;
  `docs/proposals/full-review/decisions.md:82`; `docs/proposals/ruling-49/README.md:32-34`;
  `spec/conventions/lifecycle.yaml:741-742`; `spec/promises/providers.yaml:979-981`;
  `docs/proposals/full-review/findings/E04-signin-isolation.md:341-346`.
- **Scenario:** 0.2.0 is published from the branches (requires: [], no signIn, no toolsDirectory); a
  0.3.0 later adds signIn: CTR-013 needs the account place required, NAM-024 lets a later release
  add only an optional default-free property, and the added requires entries change the held list,
  so the addition is a new contract (.v3) and every .v2 template is re-registered and re-signed-in.
  (R2-02: a 0.2.1 adding requires and defaults makes every registered .v2 template dispatch hooks it
  did not select and need a required account it does not write, so PKG-023 rejects the install under
  NAM-024. B0-06: 0.2.0 ships without retire-hook.v1 in requires; a Claude Agent is retired; its
  keychain login stays live under the engine's user; a later release cannot add the hook to .v2.)
- **Least fix:** 050 states, on both .v2 Types before any 0.2.0 archive is distributed: signIn with
  its required account place (CTR-013, contracts.yaml:296-297); sign-in-hook.v1, account-hook.v1,
  sign-out-hook.v1 and retire-hook.v1 under requires, each with a default in the implementation map
  (TPL-004, templates.yaml:104-110; D53's standing retire-hook part, decisions.md:82); and
  toolsDirectory {directory: /directory} (CTR-014; IN_FORCE.md:221; 075:55-58). apiKey goes in as a
  required nullable place only if the owner's Q1 puts it in 0.2.0 (MAN-017, manifest.yaml:363-373
  rejects a pointer to a non-required place, so it cannot be bound if added later as optional).
  Where codex.v2 states no signIn (V03's fail branch), requires holds retire-hook.v1 with its
  default alone. Dropped as already provided or beyond the verdict: the 048a-1 fixture sentence
  (048a-1 already carries CTR-013 in the loader and the signIn fixture Type under dec-31; the
  rejection of a signIn Type lacking the hooks is CTR-013's own and the type-spec's
  dependentSchemas, schemas/contracts.yaml:212-223; a loader cannot reject a Type for lacking signIn
  or toolsDirectory) and the dogfood store's re-initialization (known-14 already plans it; NAM-024's
  install check fires only for a later release at the same address, naming.yaml:434-439).
- **Follows:** NAM-024 as ruling 55 set it (a compatible release only adds); CTR-013 and the
  type-spec's dependentSchemas; TPL-004's hook defaults; FR-431's retire hook.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (toolsDirectory rests on one reading only; the consequence is the
  same, since IN_FORCE.md:221 owes it). Lens B narrowed (the fix above; the 048a-1 fixture sentence
  and the store's re-initialization dropped as already provided).
- **Skeptic:** C02: claude-code.v2 states signIn now; codex.v2 under Q3's pass, and on its fail
  requires holds retire-hook.v1 alone; apiKey as Q2 answers.

### V02: The `.v2` wrappers give the client the engine's home

- **Kind and level:** finding; must. Sources: R2-01, R3-10, B0-05.
- **Members:** M57, M59, M60, M61, M62, M53, M54, M55, M16, M17, M50.
- **Risks:** K1, K3, K7.
- **Claim:** The .v2 wrappers on every branch give the client the engine's HOME and no client-home
  variable, so the hooks 050 builds on that base act on the owner's everyday login. (R3-10: a login
  the client starts inside a turn lands in the owner's default home and a logout there signs the
  owner out. B0-05: a sign-out hook, a wrong-account undo or a retire hook run with that environment
  acts on the owner's everyday login.)
- **Evidence:** `559ecd0b:packages/agent-codex/src/lib.rs:23`;
  `559ecd0b:packages/agent-codex/src/lib.rs:47-55`;
  `559ecd0b:packages/agent-codex/src/lib.rs:456-459`;
  `559ecd0b:packages/agent-claude-code/src/lib.rs:23`;
  `559ecd0b:packages/agent-claude-code/src/lib.rs:452-466`;
  `559ecd0b:packages/agent-codex/README.md:40-41`;
  `559ecd0b:packages/agent-claude-code/README.md:39-40`; `spec/promises/providers.yaml:746-751`;
  `spec/promises/providers.yaml:972-984`; `spec/promises/manifest.yaml:424-427`;
  `docs/proposals/full-review/decisions.md:82`;
  `docs/proposals/full-review/findings/E04-signin-isolation.md:86`; `docs/guides/accounts.md:23-25`;
  `engine/CONFORMANCE.md:561-564`.
- **Scenario:** 050 adds sign-out-hook.v1 and retire-hook.v1 to lib.rs as it stands; at a store
  re-initialization every slot's setup runs the account check, finds the owner's everyday account in
  the shared home, and the undo runs claude auth logout or codex logout against it; the owner, the
  coordinator's session and the plain Codex workers are signed out, silently, until the next use.
  (R3-10: a codex.v2 Agent with a directory runs codex login or codex logout in its shell during a
  turn; with the engine's HOME and no CODEX_HOME the client resolves the owner's everyday home.
  B0-05: the wrapper is started by a runner that gives no CONDUIT_STATE_DIR; the hook runs codex
  logout with HOME only.)
- **Least fix:** 050 sets CLAUDE_CONFIG_DIR and CODEX_HOME to subdirectories of CONDUIT_STATE_DIR
  for every .v2 launch and hook, reads CONDUIT_STATE_DIR from the environment alone, refuses to
  start a client or hook where it is unset or the home resolves outside it (D53), with an explicit
  diagnostic, and tests it with a planted marker in a fake default home untouched after an undo, a
  retirement and a turn (R3-10's P-R3-f: a fake client whose login writes a marker to the home it
  resolves).
- **Follows:** Ruling 49's home of its own; PRV-029's state directory; MAN-018's reservation of the
  variable; the clients' own per-directory homes; D53's refusal.
- **Carrier:** 050.
- **Verifiers:** Lens A confirmed. Lens B confirmed.
- **Skeptic:** S11: merged with V15 and V26 into one must-state line per client. C09: the client's
  home changes, not HOME.

### V03: `codex.v2` cannot state `signIn` before the Codex account fact is in

- **Kind and level:** finding; must. Sources: R4-01, R1-11, B0-02.
- **Members:** M21, M52, M45, M51.
- **Risks:** K10.
- **Claim:** codex.v2 cannot be frozen with signIn as is: CTR-013's observable
  (contracts.yaml:305-309, binding under 171a) lets a Type state signIn only where account-hook.v1
  reports a service-attested account, and no source at T establishes that fact for Codex 0.167.1.
  Stage 0's status names no account (results-2026-11-16.md:70-72); 195's part A read the status only
  and, by order 195's own rule, followed neither the login-file nor the ID-token code
  (195.md:352-353 'stage 1's route, not part A's'; 0b-source :482, :496); the only registered trial
  is stage 1, by the package's hook, on the first build with sign-in (README :15, :193-194; D80,
  decisions.md:119; ROADMAP.md:62). The scenario follows (PND-005 pending.yaml:137-157; polling to
  DL18 ends unknown, :193-197, implementation-defined.yaml:247-254; CTR-013:308-309; NAM-024
  naming.yaml:424-428). Three narrowings. (1) A 195-style source read, claim names as identifiers
  only, can show at most that the client parses and keeps a named account claim from the ID token,
  so that a hook could read one where present; whether the service's token for the owner's account
  carries it is a run fact, which the fix itself concedes by adding a run. (2) That run is decided
  as stage 1's and the package's (dec-29, D80 'tried only here, by the package'; README :193-194
  'never the coordinator'): the fix's P-R4-2, a decode printed by the owner outside the package,
  reopens dec-29 and is a parked note at most; the B0 route, the package's hook in its own scratch
  home before any install, is consistent with D80 but is not stage 1 as registered (README :15, one
  checked sign-in on the first build) and needs pre-registration in the README (:19-20) as stage 1's
  first step. (3) On a fail, 'Codex is configured by apiKey' is O3's second answer, which known-5
  leaves to the owner among O3's remaining answers; apiKey's presence in 0.2.0 is not decided
  (known-22) and the API key is never the default (dec-15). Decided section D item 9 directs that a
  0.2.0 waiting on stage 1 be stated as a candidate owner question, not a verdict; the finding
  stands as the finding behind that question for M21 and M52, with 'must' kept because NAM-024
  forbids removing signIn once a release holds it (naming.yaml:424-425, checked at install
  :434-437).
- **Evidence:** `spec/promises/contracts.yaml:303-310`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:69-72`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:478-496`;
  `experiments/027-a5-sign-in/README.md:15`; `experiments/027-a5-sign-in/README.md:190-195`;
  `docs/proposals/full-review/decisions.md:119`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:370`;
  `spec/promises/naming.yaml:424-425`; `spec/schemas/contracts.yaml:212-223`; `ROADMAP.md:62`.
- **Scenario:** 0.2.0 publishes codex.v2 with signIn and the three hooks; stage 1's hook finds no
  email claim in the ID token; every Codex Agent's setup raises PND-005 and holds it, sign-in.v1
  polls to DL18 and ends unknown; CTR-013 then requires the Type to state no signIn, which NAM-024
  forbids removing, so codex.v3 and a fresh registration of every Codex slot follow.
- **Least fix:** One gate, not two. Before 050 states signIn on codex.v2, the account fact is taken
  on a real signed-in home only: either P-R4-2 (the owner alone decodes the account claim kept in
  the owner's own everyday Codex login file and prints one true/false: whether a claim names an
  account equal, ignoring ASCII case, to the signed-in account; no value recorded, no agent reads
  the file), or B0 (the package's account hook run in a scratch home signed in through Conduit's
  Interface flow, stage 1 as registered, before any 0.2.0 archive stating signIn on codex.v2 is
  installed anywhere). Pass as the fix states. On a fail codex.v2 states no signIn and Codex is
  configured by apiKey (O3's second answer), or the owner chooses between O3's two remaining
  answers; the third is barred (known-5). The 195-style public-source read (P-R4-1 / P-R1-a) is
  dropped from the gate: it may run as a pre-read that costs no owner time, but it decides nothing
  and has no fail branch.
- **Follows:** CTR-013's own rule ('A Type unable to learn it states no signIn'); OIDC Core section
  5.1, the email claim of an ID token; A5's stage plan and O3.
- **Carrier:** 050, after the owner answers Q3; the probe is pre-registered in experiment 027's
  README (G06).
- **Verifiers:** Lens A confirmed. Lens B confirmed. Third check (reserve): Lens A narrowed, to the
  claim above; Lens B narrowed, to the fix above (one gate, not two; the public-source read decides
  nothing).
- **Skeptic:** S03: the finding behind Q3; P-R4-2 is an option in Q3 only (b); the record parks it
  instead (parked line 2; [section 8](README.md#8-how-it-ran), deviation 13), so Q3 has no such
  option; on the probe's fail FR-118 returns to the owner. C02, C03.

### V13: `apiKey` is in 0.2.0 or never in `.v2`, and two decided items collide on its route

- **Kind and level:** finding; must. Sources: R4-03, R1-03, B0-09, R1-09.
- **Members:** M45, M46, M47, M44, M70.
- **Risks:** K2, K7.
- **Claim:** apiKey can be in 0.2.0 or never in .v2, and its only planned route, a secret read by an
  Agent that takes turns, meets 61 B13 and SEC-008 as written; two decided items cannot both be met
  as written and no source resolves them. (B0-09: apiKey and its exclusivity with account must enter
  together: a later oneOf would make `account: null` alone invalid where 0.2.0 accepted it. R1-09:
  the fixture agents-0.2.0 leaves apiKey out of required, so MAN-017 could not bind it; the fixture
  is registration-only and must be realigned to 050's schema, not the reverse.)
- **Evidence:** `spec/promises/naming.yaml:420-425`; `spec/promises/manifest.yaml:363-397`;
  `docs/proposals/first-build/log-increment-4.md:13`; `IN_FORCE.md:222`;
  `spec/promises/secrets.yaml:192-213`; `spec/promises/secrets.yaml:230-250`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:191-194`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:206-208`;
  `conformance/fixtures/s1/packages/agents-0.2.0/contracts.json:122-144`;
  `engine/crates/conformance/examples/make_archives.rs:62-73`;
  `conformance/language/packages/agents-v2-defaults/case.yaml:175-193`;
  `spec/promises/contracts.yaml:304-305`;
  `spec/schemas/examples/valid/sectioned-project/s1.yaml:11`.
- **Scenario:** 0.2.0 publishes without apiKey to meet B13; the owner later wants the API-key
  alternative accepted in 4-02; an added property states no default and MAN-017 requires it, so
  every existing .v2 template fails validation unless rewritten; the alternative is .v3. Or 0.2.0
  publishes apiKey as a secret reference; a .v2 Agent with turns reads a secret through its own
  Conduit, against IN_FORCE.md:222. (B0-09: 0.2.0 has account only; 0.3.0 adds apiKey with the
  oneOf; a template with account: null registered under 0.2.0 fails validation under 0.3.0. R1-09:
  050 copies the fixture's schema; a template binding AGENT_API_KEY: /apiKey is rejected under
  MAN-017 because /apiKey is not required.)
- **Least fix:** The owner answers before 050 (R1's Q1 / R4's question 4) whether .v2 offers apiKey
  and by which route the key reaches the client; 050 states apiKey exactly as the answer allows, or
  none; where it is in, account and apiKey are both in required with the fixture's oneOf (account
  string with apiKey null, or account null with apiKey an object), the key goes only to the client's
  environment under SEC-010 and is never stored, and the fixture follows 050's schema in a small
  conformance fix; where it is out, no oneOf and account is a non-empty string. Tier note: waits on
  an owner's answer, not a fact, so 2.
- **Follows:** MAN-017's binding rule and SEC-010; the fixture's shape; RFC 8252's and both vendors'
  API-key automation routes.
- **Carrier:** 050, after the owner answers Q2; the fixture realignment with 048a-1 (G06).
- **Verifiers:** Lens A confirmed. Lens B confirmed.
- **Skeptic:** S19: the finding behind Q2. C02.

### V09: The account hook's diagnostics hold no credential

- **Kind and level:** finding; must. Sources: R2-05.
- **Members:** M52, M07, M08.
- **Risks:** K2.
- **Claim:** Codex's account hook, as 050 is to write it, reads the ID token in the home's login
  file when status names no account (refusals-and-identity.md:187, :370; 027 README :190-195); an
  error path that echoes the file, a claim or a library error carrying the offending input puts a
  credential into the hook's diagnostic, which reaches sign-in.v1's result (IFC-025) and any log,
  and reaches a record only through the sign-in action's evidence ('attempts with their results',
  PND-005 :148-150; answer records hold full evidence, record.v1 :171-172) if the engine puts the
  diagnostic's text there, which no line states; REC-002 and record.v1 otherwise keep outcome,
  reason and ruleId, and the .v2 Types record no inputs or outputs. No committed secret scan exists
  at T (tools/check_private_paths.py matches paths, the Codex login file's name, tailnet values, a
  fingerprint and emails, no token), so D56's scrub cannot be relied on. The least fix stands:
  diagnostics name the step and cause only; a package test plants canary token and claims in a fake
  home and drives every error path.
- **Evidence:** `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:187`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:370`;
  `experiments/027-a5-sign-in/README.md:190-195`; `spec/conventions/records.yaml:50-53`;
  `docs/proposals/full-review/decisions.md:90`.
- **Scenario:** The hook parses the login file; a malformed token makes the JSON or base64 library's
  error carry the offending input; the hook returns that message as its diagnostic; the engine
  records it under REC-002; D56's export is committed scrubbed by a pattern that does not match a
  JWT fragment.
- **Least fix:** The hook's diagnostics name only the step and the cause, never file content, a
  claim or the token; a package test plants a canary token and canary claims in a fake home and
  drives every error path, asserting no canary in any result, diagnostic or log.
- **Follows:** sign-in-hook.v1's and SEC-019's wording, applied to the account hook; 050a's rule
  that no raw client text reaches a result.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (the record and export legs rest on the action's evidence, which no
  line states). Lens B confirmed.
- **Skeptic:** S20: merged with V25 into one must-state line per client, with both canary tests.

### V10: Whether the clients' logout revokes beyond one home

- **Kind and level:** finding; must. Sources: R2-06.
- **Members:** M53, M54.
- **Risks:** K1, K6.
- **Claim:** Whether claude auth logout or codex logout revokes beyond the one home is in no source
  at T (M53's row; 0b-source :478-486 followed no login-file, logout or revocation path; stage 0 ran
  no logout in a default home, 027 README :164-169; the vendor pages read say nothing on one account
  in several homes, exp 010 :165-181), while the accounts guide asserts 'signing an Agent in never
  signs you out' (:23-25). If a logout revokes account-wide, PND-011's undo (:222-224, setup
  :236-239) and the retire hook (F98) sign the owner and every worker on that account out. The least
  fix: a source and documentation read of both clients' logout for any revocation call and its scope
  before stage 1's first undo; 0.2.0's hook description names no client command
  (operations.yaml:1452-1457 already reads 'remove the instance client login from where its client
  keeps it'), so the read gates no freeze. The fallback 'the hook deletes the home's credential
  locally' is per client: for Claude on macOS the login may be a keychain entry keyed to
  CLAUDE_CONFIG_DIR that removing the home leaves behind (refusals-and-identity.md:169), and A22
  (known-11) has not yet said where the login lands, so the Claude fallback names the keychain item
  or waits on A22.
- **Evidence:** `spec/core/operations.yaml:1452-1457`; `spec/promises/pending.yaml:222-225`;
  `experiments/010-subscription-terms/results-2026-11-16.md:165-181`;
  `docs/guides/accounts.md:23-25`.
- **Scenario:** A slot's setup finds the second account where the primary is expected and undoes it
  with codex logout; the client revokes the account's refresh tokens at the vendor; the owner's
  everyday Codex and every worker on that account fail at their next call, and status, reading a
  local file, still says logged in.
- **Least fix:** M53 and M54 freeze as is. 050's run states one pre-step: before its first
  retirement or undo of a real login, a desk read of Codex's source and documentation, as order 195
  read them, and of Claude Code's documentation, for any revocation call made by codex logout or
  claude auth logout and its scope; pass is no revocation, or revocation scoped to that home's own
  token. On a fail, 050's hook removes the home's credential without the client command, and the
  vendor line D53 already owes the accounts guide says what stays at the vendor. No text of 0.2.0
  changes under either outcome.
- **Follows:** Order 195's part A method (a read, no client run); F98's stated limit on what stays
  at the vendor.
- **Carrier:** 050's run, as a pre-step before the first undo or retirement of a real login.
- **Verifiers:** Lens A narrowed (the fallback is per client; Claude's login may be a keychain
  item). Lens B narrowed (M53 and M54 freeze as is; the change is to 050's run).
- **Skeptic:** S21: A22 bears on no member's freeze through this item. C10.

### V15: D53's refusal of a client home outside the state directory

- **Kind and level:** change reason; must. Sources: R1:M61.
- **Members:** M61.
- **Risks:** K1.
- **Claim:** R1's change with no finding behind it: D53's refusal (the package refuses a client home
  outside the instance's state directory) is decided and absent on the branch; it is wrapper
  behavior with a test, not frozen text. R2 changes the same member on finding R2-01 (V02).
- **Evidence:** `docs/proposals/full-review/decisions.md:82`;
  `docs/proposals/full-review/findings/E04-signin-isolation.md:86`;
  `spec/promises/providers.yaml:972-984`; `559ecd0b:packages/agent-codex/src/lib.rs:23`.
- **Scenario:** An undo or a retire hook runs with a home outside CONDUIT_STATE_DIR and signs the
  owner out of the everyday login (FR-422's hazard, D53).
- **Least fix:** 050's wrappers refuse to start a client, a hook or a logout whose home is outside
  CONDUIT_STATE_DIR, with a diagnostic naming the cause; done when a package test plants a default
  home, asks for an undo, a retirement and a turn against it, and finds it untouched.
- **Follows:** D53 as decided; PRV-029; ruling 52's explicit diagnostics.
- **Carrier:** 050.
- **Verifiers:** Lens A confirmed (the test plants a synthetic home, never a real login file). Lens
  B confirmed.
- **Skeptic:** S11: merged with V02.

### V06: Codex's browser login opens the engine machine's default browser

- **Kind and level:** finding; must. Sources: R3-01, B0-04.
- **Members:** M50, M35, M49, M43.
- **Risks:** K3.
- **Claim:** On the engine's own runner, Codex 0.167.1's browser login attempts webbrowser::open on
  its authorization URL with no terminal, headless or browser-variable condition and discards the
  result (0b-source :199-216), and stage 0 saw a tab open past the BROWSER/PATH shim (results
  :73-76); 195 found no control in the source read or its documentation that suppresses the browser
  while keeping the printed URL and the loopback wait (0b-source :258-263, :380-384), and did not
  read the opener library (:229-231, :504-505), so 'no control in the client' is 'none found at
  0.167.1'. A sign-in can therefore complete in the engine machine's default browser before the
  person's Interface accepts (IFC-049 :1398-1400, accept is consent), with PND-011's account check
  (:212-240) the only guard, as the A5 README :153-160 already states for the probe. Nothing cited
  binds the package's client here: IFC-049 :1424 and CTR-013 :312-313 bind the engine, which starts
  no login and opens no URL; so stage 0's (b) failure can be met with sign-in in the core by a
  stated limit, guidance and probe P-R3-a, and does not of itself raise FR-118. The fix's B0-04
  (declare both logins; device login by default) is clause o6's question and goes to the owner.
- **Evidence:** `experiments/027-a5-sign-in/results-2026-11-16.md:73-76`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:199-216`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:258-263`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:498-500`;
  `experiments/027-a5-sign-in/README.md:153-160`; `spec/interface/implementation.yaml:1398-1400`;
  `spec/promises/pending.yaml:212-225`; `docs/proposals/full-review/decisions.md:276`;
  `IN_FORCE.md:218`.
- **Scenario:** The hook starts codex login on the engine's own runner as the owner's user; the
  source calls webbrowser::open unconditionally and discards the result (0b-source :209-214), the
  shim on BROWSER and PATH catches nothing (stage 0, shim_called false) and a tab opens in the
  owner's default browser at the login page. If that browser holds the owner's ChatGPT session, one
  click completes the login with no message from the engine; the redirect goes to Codex's loopback
  listener on the same machine and the account check (PND-011) is the only guard. The relayed URL
  the person was shown through the Interface is the same authorization URL, so the Interface's
  consent was never given for that click.
- **Least fix:** 050 states the limit in the two unfrozen texts it already writes, not in codex.v2's
  frozen description: the limits section of spec/README.md that 050 rewrites under 80 point 8
  (IN_FORCE.md:216) says that on the engine's own runner Codex's browser login may open the default
  browser of the engine user's graphical session, where one exists; the accounts guide (which D53
  already has 050 edit, decisions.md:82) tells the person to close that window and to keep that
  session's default browser free of an OpenAI session. The hook still relays the printed URL under
  IFC-049 (implementation.yaml:1414-1424); the account check (PND-011, pending.yaml:219-225) stays
  the only guard, unchanged. No probe gates the freeze: wording the limit conditionally on a
  graphical session makes P-R3-a (a watched start under 75 B3's dedicated user) a later narrowing of
  the guide's words, not a change to any member, runnable when that user exists (D61). Dropped:
  B0-04's device-first default, which inverts dec-3 C5 ('Codex uses its browser login everywhere',
  ruling-75/README.md:63-65) and 038c's plan (return.md:51-53) and so cannot be a finding's fix,
  only a candidate owner question; and 'declare both Codex logins', which is M43's verdict under o6.
  The answer to dec-35 (a) for stage 0's (b) failure stands as the item has it: met with sign-in in
  the core, FR-118 not raised.
- **Follows:** Stage 0's browser guard; IFC-049's relay with PND-011's account check as the guard;
  ruling 52's explicitness.
- **Carrier:** 050 (the limits section and the accounts guide).
- **Verifiers:** Lens A narrowed ('none found at 0.167.1'; the device-first default is the owner's).
  Lens B narrowed (the limit in unfrozen texts; B0-04's device-first default and 'declare both
  logins' dropped).
- **Skeptic:** C03, C09. B0-04's device-first default is parked (S16).

### V07: The managed connection's endpoint condition

- **Kind and level:** finding; must. Sources: R3-03.
- **Members:** M78, M79, M80, M31, M74, M76, M72, M73.
- **Risks:** K2, K4.
- **Claim:** Point 8's 'never reads, holds or relays' (ruling-80 :151-154) is met only where no
  engine, Interface or provider process decrypts or terminates the channel that carries the return
  in plaintext; a relay of an already-encrypted stream between the person's client and the node's
  server is not such an endpoint. 050 states this as a condition per route (forward, remote
  desktop), scoped to logins stating carried: false, with what is recorded (opened and closed, for
  which sign-in, never what passed), the three texts (M74 to M76) saying so, and the canary test.
  The fix's condition (3) (a connection only from the engine's machine toward a node it reaches; a
  person's SSH session from another machine is the person's own) is dropped: it is a design
  restriction, not a condition the claim needs, and it would remove point 8's SSH route for a
  person's machine that is no node, which dec-8 says stands (ruling-80 :352-353).
- **Evidence:** `docs/proposals/ruling-80/README.md:147-156`;
  `docs/proposals/ruling-80/README.md:185-197`; `docs/proposals/ruling-80/README.md:345-353`;
  `experiments/010-subscription-terms/results-2026-11-16.md:143-144`;
  `spec/promises/pending.yaml:309-330`; `spec/interface/implementation.yaml:1425-1429`;
  `spec/README.md:434-435`; `packages/provider-ssh-runtime/contracts.yaml:93-94`;
  `docs/proposals/ruling-40/README.md:14`.
- **Scenario:** 050 builds the forward inside the Interface: conduit opens a local listener on the
  login's port on the person's machine and copies bytes over an SSH library channel to the node's
  loopback. The redirect's HTTP request, code and state in its query, passes through conduit's
  memory in plaintext: Conduit holds and relays the credential, AN1's 'intermediate', and PND-014
  does not even apply, since no signInReturn was carried. The same holds for a remote-desktop relay
  that Conduit's process terminates. By contrast, a forward or desktop session whose endpoints are
  the person's own ssh or desktop client and the node's sshd or desktop server, which Conduit opens,
  limits and closes, never gives Conduit's processes the plaintext.
- **Least fix:** 050 states two conditions and one test, not five: (1) scope, logins stating
  carried: false, nothing changing for carried: true (PND-013 :251-252); (2) endpoints, the channel
  that carries the return terminates in the ssh or remote-desktop client program Conduit runs and in
  the node's own sshd or desktop server, and no process running Conduit's code, engine, Interface,
  runtime provider or its kit's gate, decrypts, terminates or copies it. Test P-R3-d as written: a
  canary redirect over the managed connection appears in no engine, Interface or provider log,
  record or diagnostic, and the forwarded port's sockets belong to the ssh or desktop processes, not
  a Conduit process. The three stale texts (M74 to M76) are rewritten by 050 under IN_FORCE.md:216
  as already owed; this finding adds nothing to them.
- **Follows:** The person's existing tools, SSH port forwarding and remote desktop (ruling 40);
  PND-014's keep-nowhere rule.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (the endpoint is of the channel carrying the return; condition (3)
  dropped as design). Lens B narrowed (conditions (3) to (5) dropped; the wording of (2) corrected).
- **Skeptic:** S07: condition (2) names no software: the channel that carries the return terminates
  at each end in a program that is not Conduit's, the system's own server for the route point 8
  names on the node and that route's client on the person's side; no process running Conduit's code
  decrypts, terminates or copies it. The test stands. C07.

### V08: The managed forward's lifetime and port

- **Kind and level:** finding; must. Sources: R3-04.
- **Members:** M78, M27, M37.
- **Risks:** K3, K4, K5.
- **Claim:** For a Claude Agent's sign-in (carried: false, M78, M27), a managed forward that opens
  before IFC-049 relays this sign-in's URL, outlives sign-in.v1's end (signed-in, failed or DL18
  unknown), is reused by another sign-in, or binds another local port than the URL's redirect_uri
  can carry one Agent's redirect to another Agent's node over a connection Conduit opened, or make a
  held port fail silently; 050 states those limits as conditions with the scripted fixed-port test.
  The code reaches another Agent's listener only where that Agent's client still waits (its sign-in
  ended unknown or failed with the client alive), not after a completed sign-in. The Codex (M37)
  scenario is dropped: point 8's managed connection is a Claude Agent's, and Codex's carried: true
  return goes through PND-013's Interface listener, which already opens for one login, answers only
  a matching path, Host and state and stops after that request or at DL18.
- **Evidence:** `spec/promises/pending.yaml:179-183`; `spec/promises/pending.yaml:300-306`;
  `spec/interface/implementation.yaml:1418-1420`; `spec/implementation-defined.yaml:247-254`;
  `docs/proposals/ruling-80/README.md:345-348`.
- **Scenario (as raised; the narrowed claim above drops the Codex case, M37):** Agent A on node 1
  and Agent B on node 2 are Codex Agents, whose login states one fixed port (M37). A's sign-in
  completes and its forward stays open; B's sign-in starts later, so PND-010's fixed-port rule does
  not refuse it. The person's browser redirects to the local port, still forwarded to node 1: B's
  code reaches A's machine over a connection Conduit opened. PKCE stops redemption, but the code
  crossed to the wrong program through Conduit's connection, and B's sign-in ends unknown at DL18
  with no explanation. Where the local port is held by the owner's own client login, a forward that
  silently picks another port would never match the redirect_uri in the URL.
- **Least fix:** Members M78 and M27 only; M37 drops out. 050 states, for a Claude Agent's managed
  forward: it opens only after IFC-049 relayed this sign-in's URL, for the port and path that URL
  names, to that node's loopback only; it closes when sign-in.v1 ends, signed-in, failed or at DL18;
  where the local port cannot be bound, the Interface says so naming the port and opens no forward
  on another port. Test: a scripted sign-in with a fake client on a fixed port shows the forward
  absent before the relay, present during the wait and gone after the end.
- **Follows:** PND-013's listener rule (opens for one login, answers one matching request, stops at
  DL18), applied to the forward.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (the Codex scenario dropped; the code reaches another listener only
  while that client waits). Lens B narrowed (three parts beyond the verdict dropped).
- **Skeptic:** C07.

### V12: Claude's code-page login

- **Kind and level:** finding; must. Sources: R3-02, R1-05.
- **Members:** M33, M79, M23, M76, M75.
- **Risks:** K3, K4, K8.
- **Claim:** Whether claude-code.v2 states Claude's code-page login (M33), which stage 0 saw
  (results :44, :67-68) and no source decides, is a choice at 0.2.0 under the strict reading of
  NAM-024 :420-425 (R1 item 1), and is a candidate owner question; if stated, with carried: false,
  its page URL and clientId read from the client, and the conditions the item gives (only the hook's
  process and the person's session on the node hold the code; the hook gives it to the waiting
  client as program input; the channel lives only while that sign-in waits; no record or diagnostic
  holds the code; the redirect login is relayed unless signInLogin asks). The claim that point 8's
  paste route 'has no mechanism' and is 'dead for claude-code.v2' without it does not hold: with the
  redirect login the person over SSH on the node can give the redirect's return themselves to the
  client's loopback listener, as a request to the relayed redirect_uri's port and path, which is the
  delivery PND-013 :270-273 and sign-in-hook.v1 :1421-1424 describe, Conduit managing only the
  session and holding nothing; so the 'if not stated' branch (record logs that the paste route is
  not available) is dropped, as it would also contradict dec-8's standing routes (ruling-80
  :352-353).
- **Evidence:** `docs/proposals/ruling-80/README.md:154-156`;
  `docs/proposals/ruling-80/README.md:350-353`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:44`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:67-68`; `spec/promises/pending.yaml:251-252`;
  `spec/promises/pending.yaml:297-299`; `spec/core/operations.yaml:1421-1424`;
  `spec/schemas/contracts.yaml:443-458`; `spec/promises/naming.yaml:420-425`;
  `spec/promises/contracts.yaml:300-302`.
- **Scenario (as raised; the narrowed claim above drops the dead paste route):** 050 states only
  Claude's redirect login. The owner, on the Windows computer, opens the relayed URL in a browser
  there; Claude's code page shows a code; the hook's client on the Mac node waits on its loopback
  listener with no terminal. The code cannot be pasted into the Interface (that is a carried return,
  refused by PND-013's first check for carried: false, and forbidden for Claude by point 8), and no
  stated login lets the hook take it as program input. The person's only routes are remote desktop
  or a forward; point 8's 'pasting the code' is dead for claude-code.v2 for the life of .v2, because
  a later release cannot add the page login without a .v3.
- **Least fix:** No owner question. M33 changes before 050: claude-code.v2 states Claude's code-page
  login, return: page, carried: false, its page URL and clientId read from the client at a stated
  version, as the mechanism of point 8's paste route. 050 states the on-node channel as conditions:
  the code reaches the hook's waiting client only on the node, from the person's own SSH session or
  terminal there, through no engine or Interface process; the channel exists only while that sign-in
  waits; no record or diagnostic holds the code (as PND-014 holds for a carried value); the hook
  relays the redirect login unless signInLogin asks for the page login. Open to the owner's strike
  in the record's accept-or-strike list.
- **Follows:** The client's own code-page flow, which stage 0 observed; RFC 7636 PKCE; PND-014's
  keep-nowhere rule.
- **Carrier:** 050, reaching the owner as
  [Q5](README.md#q5-may-a-later-v2-release-add-a-login-o6-m23-m33-m43), whether it can join `.v2`
  later, and its description sentence as Q1 (vi).
- **Verifiers:** Lens A narrowed ('no mechanism' does not hold: the person can deliver the
  redirect's return on the node; the 'if not stated' branch dropped). Lens B narrowed (no owner
  question needed; the conditions reduce to PND-014's pattern and V07).
- **Skeptic:** S05: claude-code.v2 states the code-page login as a second route for point 8's paste,
  open to the owner's strike; conditions: no engine or Interface process holds the code, nothing
  records it (PND-014's pattern), the hook relays the redirect login unless signInLogin asks, a
  fake-client test; the channel design and the 'if not stated' branch dropped.

### V11: `codex.v2` states its device login now, with an explicit refusal (V33 merged)

- **Kind and level:** finding; should (must after Lens B; should under S04). Sources: R3-07, R1-04,
  B0-08, R4:M43.
- **Members:** M43, M23.
- **Risks:** K8, K11.
- **Claim:** If codex.v2 states its device login at 0.2.0, 050 must state with it an explicit
  refusal: the hook asked for it fails with a diagnostic naming PND-010 and the unbuilt path, never
  chooses it unasked, and the description names the browser login as the default; because until
  F83's code path and 051c's panel exist the device login has no code path (FR-066; IFC-049
  :1384-1385; PND-010 :202-206), and a relayed verification URL would end unknown at DL18 behind a
  message saying the code is in the panel. Whether it must be stated now rests on the strict reading
  of NAM-024 :420-425 (a further login is not among the 'optional things' a later release may add),
  which is R1 item 1's open question, not settled by the lines; under the other reading, R4's defer
  (V33) is the alternative, so the choice is a candidate owner question, not a confirmed must.
- **Evidence:** `spec/promises/naming.yaml:420-425`; `spec/promises/pending.yaml:202-206`;
  `docs/proposals/full-review/findings/L06-secrets-pending.md:120-144`;
  `docs/proposals/full-review/decisions.md:264`; `docs/proposals/ruling-75/README.md:63-65`;
  `IN_FORCE.md:206`; `IN_FORCE.md:211`; `IN_FORCE.md:218`;
  `spec/interface/implementation.yaml:1384-1385`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:266`;
  `docs/work-orders/038c-staged/return.md:51-53`.
- **Scenario:** 050 omits the device login. The owner on the Windows computer later wants 75 C4's
  device code for a Codex Agent; it cannot be added to codex.v2 without a .v3, a new registration
  and a fresh sign-in of every Codex Agent. Stated now without an explicit refusal: a person asks
  for it, the hook relays the verification page, IFC-049's message says the code is in the panel, no
  code reaches any panel (FR-066), and the sign-in ends unknown at DL18 with a misleading message.
  (B0-08: 0.2.0 states only the browser login; 0.3.0 adds device; a sign-in with no signInLogin may
  now start a device login where before it could only start the browser's.)
- **Least fix:** 050 states the device login with its literals read from the client at a stated
  version and states that, until F83's code path and 051c's panel are built, the hook asked for it
  fails with a diagnostic naming PND-010 and the unbuilt path, and never chooses it unasked (R3-07,
  R1-04); the description names the default login (B0-08). R4 defers this member instead (V33).
- **Follows:** RFC 8628, the device authorization grant; PND-010's 'fails with a diagnostic naming
  PND-010 and why it could not'; ruling 75 C5.
- **Carrier:** 050, reaching the owner as
  [Q5](README.md#q5-may-a-later-v2-release-add-a-login-o6-m23-m33-m43), and its description sentence
  as Q1 (v).
- **Verifiers:** Lens A narrowed (the refusal confirmed; 'now or never' is one reading of NAM-024).
  Lens B confirmed (defer is not safe under NAM-024 as written). V33, its opposite (Lens A narrowed:
  the defer holds on the open reading), falls under S04 and C01.
- **Skeptic:** S04 and C01: one item on M43 under the record's NAM-024 reading, level should; V33
  falls; o6 reaches the owner as Q1's sentence (v). Since G02's verifiers split, the record puts
  the reading to the owner as Q5, with S04's reading as its option a; under its b, V33's defer
  holds.
- **Merged, V33 (defer reason, falls):** M43 defers: a device login appended to codex.v2's logins
  removes or changes nothing the earlier contract held (naming.yaml:424-425); NAM-024's list of
  additions is 'such as', not exhaustive (:420-422); no template or document selects a login; a
  device return states no port, so CTR-013's distinctness rule (contracts.yaml:298-300) needs only
  its verification-page URL to be distinct from the browser login's authorization URL under
  IFC-049's match (implementation.yaml:1414-1418), and PND-010's fixed-port refusal
  (pending.yaml:179-183) gains no case; it is started only where signInLogin names it (IFC-049
  :1425-1426) or where the hook, without a name, chooses it (PND-010 :202-204), and the hook is the
  package's own code shipped in the same release, so the unasked choice stays the package's (browser
  unless asked, as M43 has it). Two of the item's reasons do not hold as given: 'could not complete
  under IFC-049' is FR-066's gap (no user-code path from hook to engine, known-8) and 78 point 9's
  panel (051c), not an IFC-049 rule, since IFC-049 relays a device URL and says the code is in the
  panel (:1384-1385); and 'would freeze a verification page and client ID that 195 did not read'
  holds equally of the browser login, whose literals 050 also reads from the client
  (0b-source:510-511; M30), so it is not a reason to defer. R1, R3 and B0's 'none of the named
  additions' reads 'such as' as exhaustive; their 'changes which login an unasked sign-in may start'
  is a widening of the same kind as a further operation of a capability, and PND-010 leaves that
  choice to the hook.

### V16: `codex.v2`'s sign-in hook carries Conduit's keys

- **Kind and level:** finding; should. Sources: R2-03, B0-07.
- **Members:** M49, M18, M19, M41, M43.
- **Risks:** K3, K11.
- **Claim:** codex.v2's sign-in hook must carry Conduit's keys: a plain hook is refused every
  carried return by PND-013's first check and gets no login name, so a Codex login across machines
  never completes and a named login cannot be asked for. (B0-07: both branch items are plain: true;
  a plain hook receives no signInLogin name and no signInReturn, so Codex's two logins and any
  carried return need an item that carries Conduit's keys.)
- **Evidence:** `spec/promises/pending.yaml:251-252`; `spec/promises/pending.yaml:202-206`;
  `spec/interface/implementation.yaml:169-186`; `spec/interface/implementation.yaml:1428-1429`;
  `559ecd0b:packages/agent-codex/conduit.yaml:29`; `559ecd0b:packages/agent-codex/conduit.yaml:41`;
  `559ecd0b:packages/agent-claude-code/conduit.yaml:30`;
  `docs/proposals/full-review/decisions.md:264`; `IN_FORCE.md:218`.
- **Scenario:** 050 keeps the hooks on the plain wrapper-v2; the owner signs a Codex Agent in from
  the other machine; the Interface carries the redirect back; PND-013 refuses it not-accepted; the
  person is told to paste, and the paste is refused the same way; the login never completes and A81
  cannot run. (B0-07: root asks for Codex's device login with signInLogin; the plain hook gets no
  name, starts the browser login, and its URL fails IFC-049's match.)
- **Least fix:** The item that implements codex.v2's sign-in-hook.v1 carries Conduit's keys
  (IFC-006, IFC-037); claude-code.v2's does too where it states more than one login; turn.v1 may
  keep its plain item (B0-07). R1 holds that the plain flag is a manifest member NAM-024 does not
  freeze, so only the default's name is a 0.2.0 matter.
- **Follows:** IFC-006's keyed calls; PND-010 and PND-013 as written.
- **Carrier:** 050.
- **Verifiers:** Lens A confirmed (should; no Lens B).
- **Skeptic:** C04, C05.

### V17: What the `.v2` descriptions would freeze

- **Kind and level:** finding; should. Sources: R1-06, R2-08, R4-07, R4-06.
- **Members:** M16, M17, M68, M86, M87, M63.
- **Risks:** K8.
- **Claim:** The .v2 descriptions freeze three things 050 should decide: a client version ('Codex
  0.167.0'), the poison clause 'ends the conversation' that D55 deferred changing, and an asymmetric
  runtime sentence; 050 rewrites both descriptions anyway and dec-36 sends the result to the owner.
  (R2-08/R4-07: a client version literal in a description NAM-024 freezes is wrong at publication
  and uncorrectable within .v2. R4-06: claude-code.v2's description states the tools run as the
  engine's user with no clause for another runtime provider, while decision 25's sentence is in
  codex.v2 only; a later Claude PATH item would make the frozen description false.)
- **Evidence:** `559ecd0b:packages/agent-codex/contracts.yaml:93-95`;
  `559ecd0b:packages/agent-codex/contracts.yaml:100-102`;
  `559ecd0b:packages/agent-codex/contracts.yaml:91-93`;
  `559ecd0b:packages/agent-claude-code/contracts.yaml:84-86`;
  `559ecd0b:packages/agent-claude-code/contracts.yaml:97-99`;
  `docs/proposals/full-review/decisions.md:89`; `spec/promises/naming.yaml:424-427`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:69`;
  `docs/proposals/first-build/to-review-with-the-owner.md:58-59`;
  `docs/proposals/first-build/to-review-with-the-owner.md:68`.
- **Scenario:** Codex releases 0.168 and drops its collaboration tools; the frozen description still
  names 0.167.0's behavior, and IFC-033 copies it into every tool description an Agent reads; or
  after the rerun the coordinator un-refuses failed conversations as D55 planned, and the frozen
  'ends the conversation' is false for .v2's life; or a later 0.2.x adds a Claude PATH item and the
  frozen claude-code.v2 sentence 'run as the engine's operating-system user' is false on the other
  node.
- **Least fix:** 050's descriptions name no client version (the README records the version read),
  carry decision 25's sentence in both Types with 'On the engine's own runner' before the
  engine-user sentence, and either keep the poison clause with D55's un-refusing struck for .v2 or
  drop the clause; the owner sees them under dec-36. R4 freezes M68 as is (the clause is right and
  order 62 gives the explicit new conversation), a disagreement for the verifier.
- **Follows:** CTR-014's rule that a description states what the tools are and reach; the package
  README, which records client versions and is not frozen.
- **Carrier:** 050, after the owner answers Q1.
- **Verifiers:** Lens A confirmed (should; no Lens B).
- **Skeptic:** S15: one dec-36 question, Q1, lists every description sentence for accept or strike.

### V18: Where order 17's fields and order 16's survivors live

- **Kind and level:** finding; should. Sources: R1-07, R2-12, R4-05, B0-12.
- **Members:** M05, M56, M65, M66, M67, M18.
- **Risks:** K2, K8, K11.
- **Claim:** #17's failure fields and #16's survivor names cannot be turn.v1 outputs: its
  outputSchema is closed (559ecd0b codex contracts.yaml:48-75, additionalProperties: false at :73
  and :75), shared with .v1 and under 050a's byte check (:471-474); today a failed turn is text with
  isError true and no structuredContent (main.rs:55-57); and no source names where they live
  (orders.md:28-29). Where they live is bounded by IFC-025 (implementation.yaml:675-685, :699-701,
  :711-717), not open: a result the engine gives a consumer with isError true carries none of the
  operation's outputs, diagnostics in structuredContent apart; for a plain implementation its
  content is passed on unchanged; and the specification already states that an isError result need
  not conform to outputSchema, so R1's P-R1-e is a reading, not a probe. 050 must therefore name the
  carrier among what IFC-025 passes: the failed result's content text, stated as such, for a plain
  wrapper; diagnostic.v1 entries where the engine passes an implementation's diagnostics; or, for a
  keyed wrapper, a stated _meta key, which no promise today has the engine pass on (IFC-009
  :204-222, IFC-026 :752-773 name only outcome, reason and restriction). The description names the
  fields and the README the classes, neither enumerating them (B0-12 holds). A plain wrapper's lapse
  reaches PND-005 only through the URL-elicitation trigger (pending.yaml:143-144; IFC-016 :438-442),
  not through 'its contract reports a missing or lapsed login' (:144-145), which F84, left whole,
  gives a keyed implementation only.
- **Evidence:** `docs/proposals/full-review/orders.md:28-29`;
  `559ecd0b:packages/agent-codex/contracts.yaml:48-75`;
  `559ecd0b:packages/agent-codex/src/main.rs:51-58`;
  `559ecd0b:packages/agent-claude-code/src/lib.rs:386-414`;
  `spec/interface/implementation.yaml:204-247`; `spec/promises/pending.yaml:141-145`;
  `spec/interface/implementation.yaml:438-442`;
  `docs/work-orders/050a-agents-v2-own-tools.md:471-474`;
  `docs/proposals/full-review/findings/P06-dogfooding.md:280-300`;
  `spec/conventions/operations.yaml:1-1`.
- **Scenario:** #17 adds class and turn to turn.v1's outputSchema: 050a's byte check fails, .v1's
  wrapper never emits them, and a later class added to a description enumerating classes is a .v3;
  or a plain wrapper writes them as text the engine never reads.
- **Least fix:** Fields in the failed result's structuredContent (or _meta for a keyed wrapper;
  text, stated as such, where it stays plain) with no client text; turn.v1's schemas unchanged byte
  for byte; the description names the fields, the README the classes, neither enumerates them;
  survivors named where #17's fields live, and on TERM in the wrapper's own log; a vector shows the
  fields reach the caller with IFC-010's outcome (R1's P-R1-e checks that an isError result's
  structuredContent is not validated against the outputSchema; on a fail, _meta).
- **Follows:** MCP's error result and IFC-025; 050a decision 10's byte check; OPS-007, which
  validates successful results only.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (the proposed places do not follow from IFC-025; the outputSchema
  exemption is already text; a plain wrapper reaches PND-005 through the URL-elicitation trigger).
- **Skeptic:** S06: the positive half dropped; done when a vector shows the fields reach the caller
  under IFC-025 wherever 050 places them.

### V19: A22 and the words on Claude's credential store; Claude's port

- **Kind and level:** finding; should. Sources: R2-07, B0-10.
- **Members:** M58, M16, M57, M27.
- **Risks:** K1, K9.
- **Claim:** If 050 writes the macOS keychain into claude-code.v2's description or any other text
  0.2.0 freezes, A22 case (b) gates that text (ROADMAP.md:67 asks, for the dedicated user, whether a
  Claude login lands in a keychain item or a file; D61 runs it before any Claude Agent is a slot),
  because NAM-024 (naming.yaml:424-425) would then forbid correcting it. With the description saying
  only that the retire hook signs the client out and empties the home, and that the login is kept
  where the client keeps it for that directory (the README may name the keychain at a stated client
  version), A22 bears on no member's freeze. For M27, whether Claude's redirect port is the same
  across starts is unrecorded: stage 0's two runs each saw a port and recorded no value
  (results-2026-11-16.md:5-8, :42; the JSON results hold no port key). 050 states `any` where the
  port varies and the fixed list where it does not, read from the client at a stated version, and a
  two-start probe recording only same-or-different is the least way to learn which; `any` is not
  free, since PND-010's concurrent refusal (pending.yaml:179-183) covers fixed ports only, so `any`
  leaves FR-433's any-port case to M89 (V21).
- **Evidence:** `docs/proposals/sign-in/README.md:27-30`; `ROADMAP.md:67`;
  `docs/proposals/full-review/decisions.md:95`; `spec/promises/providers.yaml:982-984`;
  `spec/promises/naming.yaml:424-425`; `spec/schemas/contracts.yaml:402-415`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:42`.
- **Scenario:** 050 writes 'the login is a keychain item keyed to CLAUDE_CONFIG_DIR' into
  claude-code.v2's description; A22 case (b) shows the client under launchd writes a file instead;
  the description is wrong for the life of .v2 and the retire hook's emptying is described wrongly.
  (B0-10: a fixed Claude port stated now that a later client version changes needs a .v3.)
- **Least fix:** The description says the login is kept where the client keeps it for that directory
  and that the retire hook signs out before the directory goes; the README may name the keychain at
  a stated client version; 050 reads whether the client's redirect port is fixed and states any
  otherwise (B0: stage 0's probe run twice, recording only whether the port is the same, no value).
  With this wording A22 bears on no member's freeze.
- **Follows:** PRV-029's observable, which leaves credential storage to the Type's contract without
  naming a form; D61's placement of A22.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (the keychain half holds as a conditional; the ports half reduced
  to the unrecorded stability and its probe).
- **Skeptic:** S15, C10.

### V20: What stops Codex's opener is unread in its library

- **Kind and level:** finding; should. Sources: R4-08.
- **Members:** M50.
- **Risks:** K3.
- **Claim:** What the tree leaves unknown is whether any control that the webbrowser 1.2.4 library
  itself reads stops Codex 0.167.1's browser while its authorization URL prints and its loopback
  listener waits: a variable other than BROWSER, a program other than open or xdg-open on PATH, or
  anything on its Linux path. 195's not-found covers Codex's own code only (0b-source :229-252,
  :260-263), and order 195's rule kept the library's source out of part A (195.md:120-121 'no source
  of a library that Codex depends on'; 0b-source :225-227, :505). Stage 0's one watched run already
  shows that on macOS neither BROWSER nor an open or xdg-open on PATH caught Codex's opener (results
  :74-75 shim_called false with a tab open; README :154-156 states what the shim catches), so 050
  cannot name a BROWSER shim for Codex as README :206-207 does for Claude; that part of the item's
  claim is established, not unknown. Part B as pre-registered passes only arguments after `codex
  login` (README :352-356), so a watched run of a variable or PATH program needs a changed probe, a
  later order with tests and an amendment, as the README says. The proposed library read is outside
  195's part A rule and needs its own order, not a re-run of 195; 'o3 is answerable now' by stating
  the limit is the item's proposal for the review's o3 answer, not a fact of the tree.
- **Evidence:** `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:225-263`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:498-500`;
  `experiments/027-a5-sign-in/README.md:352-359`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:73-76`.
- **Scenario:** 050 is written naming a BROWSER shim for Codex as for Claude; the library ignores
  it, as stage 0 showed; at every Codex sign-in a browser opens on the Agent's machine with whatever
  session that browser holds, beside the relayed URL.
- **Least fix:** P-R4-4, a 195-style read of the webbrowser crate's macOS and Linux paths at the
  version Codex pins (no client run, no owner time); part B's watched run only where it names a PATH
  program or variable; else 050 states the limit (V06). Waits on that fact: pass, the library execs
  a program found on PATH or honors a variable, and the hook uses a no-op opener; fail, the limit's
  words are o3's answer.
- **Follows:** Order 195's part A method.
- **Carrier:** None: the read may run at no owner cost and gates nothing; no order is named (S09).
- **Verifiers:** Lens A narrowed. Third check (reserve): Lens A narrowed, to the claim above.
- **Skeptic:** S09: tier 2; P-R4-4 may run at no owner cost and narrows the guide's limit where it
  passes; it gates nothing; no order named.

### V21: The refusal of a second `sign-in.v1` of one instance (V37 merged)

- **Kind and level:** finding; should. Sources: R2-11, R4-09, B0:B17.
- **Members:** M89.
- **Risks:** K5.
- **Claim:** Three logged positions on o4 (F85 conflict, decisions.md:266; D53 'refuses ... naming
  PND-010' with no reason word, :82; FR-433 unavailable, E04:381-401), none with a carrier named
  beyond '048a', which 117:84-86 splits. Any same-instance refusal, conflict or unavailable, closes
  FR-433's any-port case and sits in OPS-023's order. REV-014's `when` covers setups, retirements
  and changes of an instance (revisions.yaml:227-233; a control is a change), not sign-in.v1, so 'as
  REV-014 answers' is F85's analogy and not an existing rule; and the one existing refusal of a
  second sign-in of one instance, the fixed-port case, is unavailable (pending.yaml:179-183;
  operations.yaml:1381-1383). The reason stays the review's to pick. Whichever it picks, the carrier
  is 048a-2: one sentence in PND-010's `then` after the fixed-port sentence (a sign-in.v1 for an
  instance whose earlier sign-in.v1, setup or retirement has been dispatched and has not ended is
  not dispatched and reports <reason>, with a diagnostic naming PND-010 and, where the caller may
  see it (PND-002), the other invocation's pending action), the same in sign-in.v1's description,
  one vector (D53), and MC9a registered with that reason before 048a-2 is dispatched (orders.md:36;
  D18, decisions.md:32). If the reason is conflict, OPS-023 reports it before 'unavailable for any
  other cause', so the fixed-port unavailable sentence then reaches only two instances of one Type;
  the record should say so.
- **Evidence:** `spec/promises/pending.yaml:179-183`; `spec/conventions/revisions.yaml:227-243`;
  `spec/conventions/operations.yaml:505-515`; `docs/proposals/full-review/decisions.md:266`;
  `docs/proposals/full-review/decisions.md:82`;
  `docs/proposals/full-review/findings/E04-signin-isolation.md:381-401`;
  `docs/proposals/full-review/orders.md:36`;
  `docs/work-orders/171b-sign-in-return-and-holds.md:123-124`.
- **Scenario:** Two sign-in.v1 invocations for one Claude Agent, whose login states ports: any, run
  together; one completes as the wrong account and is undone while the other completes as the right
  one; the undo removes the right login; the wait ends unknown with no explanation. (R4-09: 048a-2
  is written to D53's 'unavailable naming PND-010' while F85's conflict stands logged; two builders
  differ, MC9a models one and the engine builds the other.)
- **Least fix:** 048a-2 writes into PND-010's then, after the fixed-port sentence: a sign-in.v1 for
  an instance whose earlier sign-in.v1, setup or retirement has been dispatched and has not ended is
  not dispatched and reports conflict, as REV-014 answers a second act on one instance, with a
  diagnostic naming PND-010 and, where the caller may see it (PND-002), the other invocation's
  pending action; the same in sign-in.v1's description; one vector (D53); MC9a registered with that
  reason. B0 (V37) proposes unavailable naming PND-010 instead, R1 freezes the text; the verifier
  sees both.
- **Follows:** PND-010's existing refusal; the reasons' definitions (spec/definitions.md:238-239);
  OPS-023's order.
- **Carrier:** 048a-2.
- **Verifiers:** Lens A narrowed (OPS-023 and REV-014 pick no reason; the carrier holds).
- **Skeptic:** S13: merged with V37; 048a-2 writes the refusal as conflict; the fixed-port sentence
  stays unavailable; B0's unavailable is the option not taken.
- **Merged, V37 (change reason, Lens A narrowed):** What holds: PND-010's then
  (spec/promises/pending.yaml:179-183) refuses a second sign-in.v1, unavailable, only where an
  in-progress invocation may start a login stating a fixed port that this request's login also
  states, and names no instance; for any-port, device and service logins two sign-in.v1 invocations
  of one instance may run together and one's undo may remove the other's login, failing closed with
  no explanation to the caller (E04-signin-isolation.md:387, :397). M89 is engine-side, so the
  choice touches no .v2 contract. One sentence and one vector in PND-010's then are owed, with one
  reason, and the carrier the split supports is 048a-2 (117-plan-s2-s3.md:84-86 puts the loader work
  in 048a-1 and 'sign-in as a kind of pending action' in 048a-2; E04:399 says 048a-2; D53 and F85
  say only '048a'). What does not hold: OPS-023 (spec/conventions/operations.yaml:505-515) does not
  pick the reason. It ranks reasons once more than one applies (conflict before 'unavailable for any
  other cause'); a same-instance rule written as conflict or as unavailable each sits in that order,
  so 'pick the reason consistent with OPS-023's order' is satisfied by both. REV-014
  (revisions.yaml:227-234) does not decide it either: its when covers a setup, retirement or change,
  and sign-in.v1 is none of these, so F85's 'as REV-014' is an analogy, as FR-433's verifier's 'as
  REV-012' is. The source that bears on the choice is the reasons' definitions,
  spec/definitions.md:238-239: conflict is a clash 'with something that exists, such as ... another
  setup, retirement or change of the same instance that has not ended (REV-014) ... or an invocation
  that it repeats and that has not ended (OPS-032)'; unavailable is 'could not be checked or carried
  out now'. A second sign-in.v1 of an instance whose sign-in.v1 is in progress follows the conflict
  pattern (an unended act on the same object); the existing fixed-port refusal follows the
  unavailable pattern (a node port not free now). Under OPS-023 a conflict sentence and the
  unavailable fixed-port sentence coexist: where both apply to one request, conflict is reported.
  The record settles o4; this is the reading of the source.

### V22: F95 to F97 and their carriers

- **Kind and level:** finding; should. Sources: R4-10, R3-08, R3-09.
- **Members:** M90, M91, M78, M77, M20.
- **Risks:** K3.
- **Claim:** F95 to F97 have no carrier (known-7; 171b.md:125-126), each is a short text change, and
  none bears on a 0.2.0 member's freeze: the branch Types state no signIn and list no control
  (559ecd0b, both contracts.yaml), so F97's CTR-013 rule, a mirror of CTR-014's control clause
  (contracts.yaml:342-344), bites at install only on a later Type; F95 and F96 are engine-side (M90,
  M91). Carriers, one each: F97's sentence and loader check with 048a-1 ('CTR-013 in the loader',
  117:84-85); F96 as a named spec fix before 048a-2 is drafted, since MC9a then models PND-010 and
  IFC-049's binding (orders.md:36), the first build already binds transport session IDs to the
  consumer (engine/CONFORMANCE.md:95-97), and 050 brings 'SSH session' (IN_FORCE.md:216) into the
  same promises; F95's sentence with 050's rewrite of the limits and IFC-049's observable, which 050
  already owns (IN_FORCE.md:216). R3-08's addition is reduced to its negative half and loses the
  word F96 strikes: 'The connection Conduit manages for a Claude sign-in does not bind the person
  who completes the login; the expected account (PND-011) remains the only binding.'
- **Evidence:** `docs/proposals/full-review/decisions.md:276-278`;
  `spec/interface/implementation.yaml:1393-1394`; `spec/interface/implementation.yaml:1424`;
  `spec/interface/implementation.yaml:1435`; `spec/promises/pending.yaml:207`;
  `spec/core/operations.yaml:1385`; `spec/definitions.md:289`;
  `spec/promises/contracts.yaml:290-302`; `spec/promises/contracts.yaml:343-345`;
  `docs/work-orders/117-plan-s2-s3.md:84-88`; `IN_FORCE.md:216`;
  `docs/proposals/full-review/findings/GL1-protocol.md:109-130`;
  `docs/proposals/full-review/findings/L04-interface.md:172-196`.
- **Scenario:** The five orders are written to their must-state lists and none carries F95 to F97;
  o5 stays open after ruling 82, and the next full review raises the same three fixes a third time.
  (R3-09: the Interface's transport reconnects during the two minutes of a login; a build that binds
  'session' to the transport rejects the retry as another principal and the sign-in ends unknown at
  DL18. R3-08: a reader takes the SSH authentication of a managed connection as binding the person
  who completes the login.)
- **Least fix:** F97's CTR-013 sentence and loader check go to 048a-1; F96's words (requestState
  bound to the authenticated Interface and the originating invocation, 'session' dropped, the
  removed binding logged) to 048b-join or a named spec fix before or with 050; F95's sentence to
  050's rewrite of the limits and IFC-049's observable, with R3-08's added sentence: 'A connection
  Conduit manages for a sign-in authenticates the person's session to the node, not the person who
  completes the login.' Carriers differ among the three readers (R3: named spec fixes; R4: 048a-1,
  048b-join, 050; R1: 048a-2 or SO-H); the verifier names one.
- **Follows:** CTR-014's control clause (F97); the protocol's requestState binding (F96); the limits
  section's form (F95, FR-541's least change).
- **Carrier:** F97: 048a-1. F96: a named spec fix before 048a-2 is drafted. F95: 050.
- **Verifiers:** Lens A narrowed (R3-08's sentence reduced to its negative half, without the word
  F96 strikes).
- **Skeptic:** S14: F95's sentence is the negative half alone. C06.

### V23: A Codex login started outside `sign-in.v1` holds the port

- **Kind and level:** finding; should. Sources: R3-05.
- **Members:** M37, M49.
- **Risks:** K5.
- **Claim:** Codex's source at rust-v0.167.1 sets the login server's port from a named constant and
  binds it (0b-source :207-208, :211); whether a failed bind falls back is not shown, :194 printing
  `server.actual_port` being consistent with either. A `codex login` started outside sign-in.v1 on
  the node (the owner's everyday login, a coordinator worker's) holds that port unseen by PND-010,
  whose concurrency refusal reads only in-progress sign-in.v1 invocations (pending.yaml:179-183);
  the hook then either fails under :202-203 with a diagnostic naming PND-010 and 'why it could not',
  whose words 050 writes, or, only where the binary falls back to another port, IFC-049 fails the
  URL at 'one stated port' (implementation.yaml:1418-1419). The scenario's 'IFC-049's diagnostic and
  no mention of the busy port' holds for the fallback branch alone; on a failed bind the person sees
  the hook's own failure, which PND-010 already requires to say why, so 050's addition is that the
  hook reads the client's bind failure as the cause and names it, holding no URL value. M37's 'not
  recorded' is incomplete only in that the coordinator's non-binding proposal of 2026-11-10
  (docs/proposals/sign-in/README.md:2-3, :39) records, from Codex's documentation of that day, one
  loopback callback port; it is not the literal of 0.167.1, which 050 reads from the client (038c
  return.md:53-55; 195.md:141-143). Two Codex Agents' sign-ins at once are already refused
  unavailable by PND-010, since every Agent of the Type states the same ports, so the guide's line
  is for logins outside Conduit. Probe P-R3-c runs the client twice and, because Codex 0.167.1 opens
  a browser past the shim (results :73-76), falls under stage 0's guards and the owner's watch
  (ruling 81 point 3; README :374-381), with pre-registration in the README; it settles only the
  literal's form, fixed list or any, not whether 050 may state signIn.
- **Evidence:** `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:194`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:207-211`;
  `spec/promises/pending.yaml:179-183`; `spec/promises/pending.yaml:202-203`;
  `docs/proposals/ruling-75/README.md:17-18`.
- **Scenario (as raised; the narrowed claim above drops all but the fallback branch):** The owner's
  everyday codex login, or a coordinator worker's, waits on the node's loopback port when sign-in.v1
  starts a Codex Agent's login. PND-010's rule sees no in-progress sign-in.v1 and dispatches the
  hook; the client cannot bind, or binds another port if the binary falls back, so the printed
  redirect_uri no longer matches the stated ports and IFC-049 fails the URL. The person sees a
  failed sign-in with IFC-049's diagnostic and no mention of the busy port.
- **Least fix:** 050 reads Codex's port from the client and states it as a fixed list; the hook,
  where the client reports it cannot bind, fails with a diagnostic naming PND-010 and the held port,
  holding no URL value; the guide says one Codex sign-in at a time per machine. Probe P-R3-c (two
  codex login starts in two scratch homes at once, cancelled before any sign-in; records fixed or
  fallback as a word, never a port value) settles fixed against fallback before 050 states the list;
  tier 3 because the literal's form waits on that fact.
- **Follows:** PND-010's fixed-port refusal and diagnostic, extended to the hook's bind failure; RFC
  8252 section 7.3 on loopback ports.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed. Third check (reserve): Lens A narrowed, to the claim above.
- **Skeptic:** S10: tier 2; 050 states the fixed list read from the client; the hook names a bind
  failure; P-R3-c optional, no gate.

### V24: `issuer` for Codex's redirect login

- **Kind and level:** finding; should. Sources: R3-06.
- **Members:** M42, M32.
- **Risks:** K4, K8.
- **Claim:** Stating issuer for Codex's redirect login, read from the client's DEFAULT_ISSUER at the
  stated version (results-2026-11-16-0b-source.md:207), costs nothing where the server sends no iss
  (PND-013: 'The issuer check applies only where both are present', pending.yaml:259-260) and is the
  only way Conduit's own return check refuses a mismatched iss; PND-013 says in its own words that
  without issuer 'Conduit stops no mix-up and the program relies on its own defenses' (:291-293),
  and whether Codex's server checks iss itself was not read (0b-source :505). Under the plain
  reading of NAM-024 (naming.yaml:420-425: additions are optional inputs, operations and schema
  properties; nothing the earlier release held changes) with CTR-013 making the logins part of the
  Type contract under the package digest (contracts.yaml:300-302), a member added later to an
  existing login is a change to a login 0.2.0 held, so issuer is stated now or not within .v2;
  whether a later login addition is an allowed optional thing is R1 item 1's open question
  (members.md M43), so this is a reading, not a settled rule. The scenario does not follow as
  written: Codex's traced path has one issuer, IFC-049 checks the relayed URL's origin against the
  stated authorization (implementation.yaml:1415), and a return must carry the relayed state
  (pending.yaml:258-259), so a look-alike server's code is not a mix-up of the RFC 9207 kind, and
  Codex redeems at its own issuer, not 'the wrong server'. For Claude's carried: false login nothing
  turns on it (PND-013's first check, :251-252).
- **Evidence:** `spec/promises/pending.yaml:259-260`; `spec/promises/pending.yaml:291-293`;
  `experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:207`;
  `spec/schemas/contracts.yaml:425-428`; `spec/promises/naming.yaml:420-425`.
- **Scenario (as raised; the narrowed claim above drops the mix-up):** 050 states Codex's login
  without issuer. A carried return with an iss parameter from a look-alike authorization server
  passes PND-013's checks, since the issuer check applies only where both are present; the engine
  gives the code to Codex's listener, which redeems it at the wrong server. A later release cannot
  add issuer to the login without a .v3.
- **Least fix:** 050 states issuer for Codex's redirect login with the value its source names as
  DEFAULT_ISSUER, read from the client at the stated version; stating it is harmless where the
  server sends no iss (PND-013: 'only where both are present'). For Claude's carried: false login
  nothing turns on it.
- **Follows:** RFC 9207 and RFC 9700 section 4.4, as PND-013 already cites them.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed ('now or never' holds under the plain reading; the mix-up scenario
  overstated).
- **Skeptic:** C01: it cites the record's reading; it holds under either reading (G02).

### V25: Claude gives two URLs at one login start

- **Kind and level:** finding; should. Sources: R3-11.
- **Members:** M50, M49, M33.
- **Risks:** K2, K3.
- **Claim:** At one Claude login start two authorization URLs appear, a redirect URL handed to the
  BROWSER program and a page URL printed, and both pass every IFC-049 check, so both hold state and
  code_challenge (results-2026-11-16.md:41-45, 67-68; implementation.yaml:1418-1422). 050's hook
  must take its URL from the login it started (for redirect, the argument its BROWSER program
  received) rather than scanning the client's output, and must write no client output into a
  diagnostic, log or record: brief line 2 allows state nowhere, and IFC-049 :1387-1389 and PND-012
  :129-130 already bar the relayed URL from tool content, structuredContent and records. The
  wrong-URL half of the scenario is a failed sign-in, not a leak: for a keyed hook given
  signInLogin, IFC-049 :1414 and :1422-1423 refuse a URL that is not the login asked for and end
  sign-in.v1 failed with a diagnostic holding no URL value, code or state; for a plain hook no login
  is asked for (PND-010, pending.yaml:204-205), so there is nothing to mismatch. PND-014
  (pending.yaml:309-330) binds signInReturn and URL values taken from a return, not the hook's own
  URL.
- **Evidence:** `experiments/027-a5-sign-in/results-2026-11-16.md:44-45`;
  `experiments/027-a5-sign-in/results-2026-11-16.md:67-68`;
  `spec/interface/implementation.yaml:1414`; `spec/interface/implementation.yaml:1422-1424`;
  `spec/promises/pending.yaml:309-330`.
- **Scenario (as raised; the narrowed claim above drops the wrong-URL leak):** The hook reads the
  client's stdout to find a URL, finds the page URL first and relays it although the person asked
  for the redirect login; or it logs the client's output on a failure, and a diagnostic now holds a
  URL with state and code_challenge, which the record keeps. Brief line 2 allows state nowhere.
- **Least fix:** 050's hook takes the URL from the argument its browser program received, relays the
  one that matches the login asked for, and writes no client output into a diagnostic; a package
  test with a fake client that prints a canary URL beside the one it passes to BROWSER shows the
  canary in no result, diagnostic, record or log (P-R3-e).
- **Follows:** IFC-049's 'holding no URL value, code or state'; PND-014's keep-nowhere rule.
- **Carrier:** 050.
- **Verifiers:** Lens A narrowed (the wrong-URL half is a detected failure that leaks nothing; the
  diagnostic half holds).
- **Skeptic:** S20: merged with V09; 'from the argument its browser program received' is how, and is
  dropped from the must-state line.

### V26: The home change must not reach the `.v1` launch

- **Kind and level:** finding; should. Sources: R2-09.
- **Members:** M88, M03.
- **Risks:** K1.
- **Claim:** If 050's home change reaches the .v1 launch, every .v1 Agent, whose Type states no
  signIn, gets an empty home it can never sign in through Conduit.
- **Evidence:** `559ecd0b:packages/agent-codex/conduit.yaml:10-19`;
  `559ecd0b:packages/agent-codex/contracts.yaml:1-27`; `spec/promises/contracts.yaml:308-309`;
  `559ecd0b:packages/agent-codex/src/lib.rs:86`;
  `docs/work-orders/050a-agents-v2-own-tools.md:77-78`.
- **Scenario:** lib.rs sets CODEX_HOME for every launch; a .v1 Agent installed from 0.2.0 starts in
  an empty home; codex exec reports not logged in; the Type states no signIn so no action is raised;
  every turn fails with class authentication for good.
- **Least fix:** The home variables are set only where the launch is .v2
  (AGENT_TURN_DEADLINE_SECONDS present), as MCP_TOOL_TIMEOUT already is; a package test shows the
  .v1 launch's environment unchanged from 0.1.1.
- **Follows:** 050a decision 10 (.v1 byte for byte), extended to the .v1 launch's environment.
- **Carrier:** 050.
- **Verifiers:** Lens A confirmed (should; no Lens B).
- **Skeptic:** S11: merged with V02; 'as MCP_TOOL_TIMEOUT already is' is how, and is dropped from
  the must-state line.

### V27: Codex's credential store

- **Kind and level:** finding; should. Sources: R2-10.
- **Members:** M51, M59.
- **Risks:** K1, K7.
- **Claim:** The branch launches Codex with --ignore-user-config and no credential-store setting; if
  0.167.1's default store is a keyring entry not keyed per CODEX_HOME, two Codex Agents' logins
  collide in one entry.
- **Evidence:** `559ecd0b:packages/agent-codex/src/lib.rs:303`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:167`; `IN_FORCE.md:211`.
- **Scenario:** Two Codex slots sign in on one machine; the second login overwrites the first's
  keyring entry; the first slot's next turn runs on the second's account between account checks
  (K7's replace route), unseen until setup or sign-in checks again.
- **Least fix:** 050 passes -c cli_auth_credentials_store="file" on codex login and every launch and
  states it in the README; a source read of the default and the keyring key (no client run; pass:
  default file, or an entry keyed per CODEX_HOME) decides whether the setting is mandatory or belt
  and braces. The change is made either way, so tier 2.
- **Follows:** The batch-1 plan's credential-store setting as written; order 195's method for the
  read.
- **Carrier:** 050.
- **Verifiers:** Lens A confirmed (should; no Lens B).
- **Skeptic:** None.

### V28: A login that lapses during a turn

- **Kind and level:** finding; should. Sources: R2-04.
- **Members:** M56, M18, M19, M66, M67.
- **Risks:** K2, K11.
- **Claim:** With F84 unwritten (decisions.md:265; 171b.md:68-85) and the .v2 wrappers plain, a
  login that lapses during a turn reaches the engine only as the wrapper's isError text
  (main.rs:55-57) whose class string 'authentication' (lib.rs:393-403) the engine does not read;
  IFC-010 (implementation.yaml:234-240) ends that turn.v1 call unknown, no PND-005 is raised, and
  the wrapper's own rule (lib.rs:495-497, 513) refuses every later turn of that session, so the
  lapse is seen only at the next setup or sign-in account check. New against known-15: the IFC-010
  ending and the wrapper's permanent refusal. Not new: that the trigger has no form the engine can
  see, that a plain wrapper's only route is a URL elicitation under IFC-016 (:438-442), and the
  tension with IN_FORCE.md:209, which 171b.md:84-85 already records as left standing. The carrier
  for the spec half is 048a-2 writing F84's two halves into PND-005. Whether 050's wrapper is keyed
  is clause o1's answer, not this finding's; the finding holds under either answer, the plain half
  being the URL request IFC-016 names.
- **Evidence:** `spec/promises/pending.yaml:144-145`; `spec/interface/implementation.yaml:234-240`;
  `559ecd0b:packages/agent-codex/src/lib.rs:393-395`;
  `559ecd0b:packages/agent-codex/src/main.rs:55-57`; `docs/proposals/full-review/decisions.md:265`;
  `docs/work-orders/171b-sign-in-return-and-holds.md:68-85`; `IN_FORCE.md:209`.
- **Scenario:** A Codex Agent's login lapses mid-turn; codex exec prints an authentication error;
  the wrapper answers isError text 'client failure: authentication'; IFC-010 records the turn
  unknown; no sign-in action is raised, the caller gets no action ID to answer, and the next turn is
  refused by the wrapper's own rule, with the owner never asked to sign in.
- **Least fix:** 048a-2 writes F84's two halves into PND-005 (a keyed implementation ends the call
  failed or unknown with a diagnostic naming PND-005; a plain one only by a URL request, IFC-016);
  050's wrapper, keyed, ends such a turn failed with a diagnostic naming PND-005, with a scripted
  lapse as a vector. R1 holds that the plain half is already in text (PND-005's third trigger,
  IFC-016) and freezes M56.
- **Follows:** F84 as logged (FR-067); IFC-009's outcome and diagnostic keys; IFC-016's URL request;
  ruling 52.
- **Carrier:** 048a-2 (PND-005's text); 050 (the wrapper's half).
- **Verifiers:** Lens A narrowed (most of it repeats known-15; new: IFC-010's unknown ending and the
  wrapper's lasting refusal; it holds whether the wrapper is keyed or plain).
- **Skeptic:** S12: 048a-2 writes F84's two halves; 050's turn implementation follows whichever half
  matches its item. C05.

### V31: An engine before 048a-2 enforces none of `signIn`'s behavior

- **Kind and level:** finding; should. Sources: R4-04.
- **Members:** M20, M21, M44.
- **Risks:** K3, K10.
- **Claim:** If 0.2.0 is installed on an engine built before 048a-2, the engine enforces the shape
  of signIn but none of its behavior: the loader already rejects a malformed signIn
  (engine/STANDING.md:122 records CTR-013's case conformance/language/contracts/sign-in-logins/ as
  passing: the closed schema, the four return kinds and the three hooks under requires,
  spec/schemas/contracts.yaml:169-178, 212-223), and what 048a-1 adds is the rest of CTR-013
  (117:84); but engine/crates/core/src has no occurrence of account-hook, signIn or PND-005, so
  setup dispatches no account check (TPL-012, templates.yaml:214-216, is spec only), PND-005 is
  never raised, and sign-in.v1 answers unavailable, 'not built yet, order 048a'
  (requests.rs:1046-1058; engine/CONFORMANCE.md:70). A fresh-home slot could then be signed in only
  by hand outside the Interface, which leaves ruling 49 point 3's check unmade and contravenes
  ruling 72's rule that every test login goes through Conduit's Interface login flow
  (ruling-72:72-74), not ruling 49 point 1, which binds the engine's relay and is not engaged by a
  sign-in the engine never sees. IN_FORCE.md:302 already places the owner's one sign-in after the
  six sign-in orders. The item bears on dec-31's [not decided] edge (050 against 048b-join) and may
  be an owner question; its fix (050 written in parallel after the record, 0.2.0 merged after
  048b-join) meets both 117:93 and ruling 81 :95-97.
- **Evidence:** `engine/crates/core/src/requests.rs:1046-1058`;
  `docs/work-orders/117-plan-s2-s3.md:84-93`; `spec/promises/templates.yaml:214-216`;
  `spec/promises/pending.yaml:137-157`; `IN_FORCE.md:302`;
  `docs/proposals/ruling-72/README.md:72-74`; `docs/proposals/ruling-81/README.md:95-97`.
- **Scenario (as raised; the narrowed claim above drops ruling 49 point 1):** 0.2.0 merges once 050
  is ready, ahead of 048b-join (ruling 80 point 7 read as 'next'); the dogfood store is
  re-initialized; each slot's Type states signIn with a string account, yet setup dispatches no
  account hook (nothing in engine/crates/core names account-hook or signIn at T) and sign-in.v1
  answers 'not built yet, order 048a'; the owner signs each slot in by running the client in its
  home, outside the Interface, against ruling 49 point 1 and ruling 72's rule, and turns run on an
  unchecked account until 048a-2 lands.
- **Least fix:** 050 is written after the record in parallel with 048a-1 and 048a-2, and 0.2.0
  merges only after 048b-join (117:93's edge kept); the owner's sign-in of each slot follows 051b,
  as ruling 78 point 1 part 2 already has it. This answers the [not decided] item on ruling 80 point
  7's 'next' and may be an owner question.
- **Follows:** Order 117's split as written; ruling 78 point 1 part 2.
- **Carrier:** None: it is the finding behind Q4, the order of the work, which is the owner's.
- **Verifiers:** Lens A narrowed (the loader checks signIn's shape at T; a by-hand sign-in
  contravenes ruling 72, not ruling 49 point 1).
- **Skeptic:** S08: the finding behind Q4; its fix is Q4's recommended option, not a verdict.

### V32: M06 defers

- **Kind and level:** defer reason; should. Sources: R4:M06.
- **Members:** M06.
- **Risks:** K8.
- **Claim:** M06 defers under NAM-024's 'a further operation of a capability' (naming.yaml:420-422).
  NAM-024's bar on a default names only 'a property that it adds to a Type's schema' and 'an input
  that it adds to a Requirement contract' (:421-424), and its observable's 'A default requires a new
  version segment' sits in SCH-009's context of schema defaults (:433-436); an entry added to a
  Type's implementation map for the new operation removes or changes nothing held (:424-425) and is
  an addition the comparison shows (:427). R1's reading that the map cannot gain an entry is not in
  NAM-024's words. TPL-004 (templates.yaml:84-86, 95-98) then needs that default in every Type
  offering conversation.v1, so the release that adds the operation adds the entry in all four Types,
  .v1 included; the .v1 contracts change in that release, which 050a decision 10's byte-for-byte
  rule (a 0.2.0 decision) does not govern. PKG-031's clause (packages.yaml:431-433) binds only a
  template the package exports; no branch exports turn (M70), so it applies only if 050 adds one. A
  new capability on the .v2 Types is likewise an addition. Nothing 0.2.0 holds blocks the later
  addition. Residual for the record: NAM-024's list is 'such as', so that an implementation entry is
  an optional thing is a reading, stated in one line.
- **Evidence:** `spec/promises/naming.yaml:420-425`; `spec/promises/templates.yaml:76-110`;
  `spec/promises/packages.yaml:423-440`; `docs/proposals/full-review/orders.md:74`;
  `docs/proposals/ruling-81/README.md:95-97`; `spec/promises/contracts.yaml:110-124`.
- **Scenario:** If the defer holds: 0.3.0 adds the operation to conversation.v1 and a default to
  each offering Type's implementation map in the same release, and NAM-024 accepts the install;
  nothing in 0.2.0 blocks it. If it does not hold: V14's scenario (a .v2 template at 0.3.0 has no
  implementation for the new operation and TPL-004 rejects it; .v2 cannot state a default without a
  .v3).
- **Least fix:** If confirmed, M06 defers and V14 falls; if refuted, V14's fix stands (fix #62's
  carrier before 050: an optional input of turn.v1, or a NAM-024 clause for a default added with its
  operation).
- **Follows:** NAM-024's own list of additions; TPL-004.
- **Carrier:** None: a defer.
- **Verifiers:** Lens A narrowed (the defer holds, with its conditions made explicit).
- **Skeptic:** S02: M06 defers; V14 dropped.

### V34: M48 defers; M47 only where `apiKey` is in

- **Kind and level:** defer reason; should. Sources: R2:M47, R2:M48.
- **Members:** M47, M48.
- **Risks:** K7.
- **Claim:** M48 defers: AGENT_ACCOUNT: /account is a binding of a manifest item
  (refusals-and-identity.md:181), outside NAM-024's when, which reaches a contract a release exports
  (naming.yaml:417-419); it points at /account, which 0.2.0 holds required and confined to string or
  null (M44), so MAN-017 (manifest.yaml:363-372) accepts it when a later release adds it, and
  PND-011 (pending.yaml:219-224) compares the attested account itself, so the hint changes no
  meaning. M47 does not defer on the reason given: MAN-017 rejects an item whose environment points
  at a place the Type's schema does not require (manifest.yaml:365-367; observable :376-378,
  'neither is a pointer into a member that is not itself required'), and NAM-024 lets a later
  release add only an optional property with no default (naming.yaml:420-424), so AGENT_API_KEY:
  /apiKey can be added later only if 0.2.0 already holds apiKey as a required, nullable place (the
  area plan's shape, refusals-and-identity.md:177: required, anyOf [secret-reference.v1, null]).
  M47's defer is therefore conditional on V13 putting apiKey in 0.2.0's schema; if V13 leaves it
  out, M47 is not addable within .v2, as the item's own 'if not' scenario says. CTR-013's 'as for
  API-key configuration' (contracts.yaml:304-305) describes a null account and settles nothing about
  apiKey's presence.
- **Evidence:** `spec/promises/naming.yaml:414-425`; `spec/promises/contracts.yaml:304-305`;
  `spec/promises/manifest.yaml:363-397`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:181`;
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:191-194`; `IN_FORCE.md:222`;
  `spec/promises/pending.yaml:219-224`.
- **Scenario:** If the defer holds: 050 may omit the bindings and a later release adds them with no
  NAM-024 cost, provided apiKey's schema presence is settled by V13. If not: a binding needs a
  required place (MAN-017), which cannot be added later, so omitting apiKey's binding now ties to
  omitting the property for good.
- **Least fix:** If confirmed, M47 and M48 defer and V13's apiKey question is about the schema
  property alone; if refuted, M47 follows V13's answer and the binding is written with 050.
- **Follows:** MAN-017's binding rule; NAM-024's when.
- **Carrier:** None: a defer; M47 follows Q2.
- **Verifiers:** Lens A narrowed (holds for M48; for M47 only where 0.2.0 holds apiKey as a required
  place).
- **Skeptic:** C02.

### V38: F97: CTR-013 gains CTR-014's control sentence

- **Kind and level:** change reason; should. Sources: B0:B18.
- **Members:** M20.
- **Risks:** none for 0.2.0.
- **Claim:** B0's change of B18 (F97, on M20) with no finding behind it: CTR-014 states the rule for
  toolsDirectory (no control whose changes reaches the directory place) and CTR-013 lacks it; a
  control that rewrote account would move the expected account between checks; neither .v2 Type
  lists a control, so 0.2.0 is unaffected whichever way it is written. B0's words: CTR-013's then
  gains CTR-014's sentence adapted, 'The Type lists no control whose changes holds the account
  pointer, a pointer inside it or a pointer over it, compared as CTR-006 compares pointers', with
  one loader vector, as a named small spec fix. R1 and R4 freeze M20 and send F97 to 048a-1 (R4-10,
  V22).
- **Evidence:** `docs/proposals/full-review/decisions.md:278`;
  `spec/promises/contracts.yaml:343-345`; `spec/promises/contracts.yaml:290-302`;
  `559ecd0b:packages/agent-claude-code/contracts.yaml:100-116`.
- **Scenario:** A later Type at another address lists a control whose changes holds /account; the
  loader accepts it; the control rewrites the expected account between the account check and the
  next sign-in.
- **Least fix:** CTR-013's then gains the adapted sentence with one loader vector, carried by 048a-1
  (CTR-013 in the loader) or a named spec fix; no 0.2.0 member changes.
- **Follows:** CTR-014's control sentence (FR-105's least change).
- **Carrier:** 048a-1.
- **Verifiers:** Lens A confirmed (should; no Lens B).
- **Skeptic:** C06: carrier 048a-1; 'or a named spec fix' dropped.

## Gap findings verified in the reserve

The completeness critic named eleven gaps. The reserve verified the six it ranked costliest
(G01 to G06) under Lens A, and G01 and G02, the must-level ones, under Lens B. G03 was refuted
(below, with the struck items); G07 to G11 were not covered, and the record names each where it
bears ([section 8](README.md#8-how-it-ran)).

### G01: whether a new sign-in rotates the tokens of other homes on one account

- **Kind and level:** risk coverage; should after Lens B (the critic ranked it must).
- **Members:** M53, M54, M57, M59. **Risks:** K6.
- **Claim:** K6's second half is read by no survivor: whether a new sign-in of a slot on the
  primary account rotates or revokes the tokens of the owner's everyday login, the coordinator's
  workers or the other slots. V10's desk read covers only what the clients' logout revokes;
  experiment 010's pages are silent on one account signed in independently in several homes.
- **Evidence:** `experiments/010-subscription-terms/results-2026-11-16.md:165-181`;
  `docs/proposals/ruling-75/README.md:85-86`; `docs/proposals/full-review/decisions.md:119`;
  `docs/guides/accounts.md:23-25`; `spec/core/operations.yaml:1452-1457`.
- **Scenario:** at 050's re-initialization of the store the owner signs four slots into the
  primary account one after another; if the service invalidates older refresh tokens on a new
  login, the owner's everyday client and the coordinator's workers fail at their next call while
  status, reading a local file, still says logged in.
- **Least fix, as proposed:** extend V10's pre-step with the same desk read for whether a new
  login in another home invalidates existing sessions' tokens. No 0.2.0 text changes under either
  outcome.
- **Verifiers:** Lens A confirmed. Lens B refuted the change: A5's registered pass condition P7
  (`docs/proposals/full-review/experiments.md:109`) already reads it at stage 1, after each
  sign-in, undo and retirement, with a fail going to the owner; the four members freeze as is
  whatever the service does; and a desk read could not show what the service does with older
  tokens. Under run.md's survival rule the item goes forward as should and changes no verdict; it
  adds no must-state item.

### G02: one reading of NAM-024 for a login added later

- **Kind and level:** an unsettled disagreement; must.
- **Members:** M43, M23, M33, M42.
- **Claim:** whether a login appended to a Type's logins later is an allowed optional addition is
  settled by no verifier, and V11 (Lens B confirmed, defer not safe) and V33 (Lens A narrowed, the
  defer holds) answer it both ways; V12 and V24 rest on the same reading. The record cannot write
  M43's verdict, M23's list or o6's line without one reading.
- **Evidence:** `spec/promises/naming.yaml:420-425`; `spec/promises/contracts.yaml:290-291,
  300-302`; `spec/promises/pending.yaml:202-206`.
- **Scenario:** the record states the device login now with V11's refusal and also defers M43 per
  V33, and 050's writer reads two answers; or the record defers and the owner later wants 75 C5's
  device code, which under V11's reading `codex.v2` cannot gain without a `.v3`.
- **Least fix, as the critic proposed it (Lens A confirmed it):** one line in the record adopting
  the strict reading (a login is an element of the required list `signIn.logins`, pinned by
  digest, so a later login changes what the earlier release held), with V33's open-list reading as
  the strongest case against, and one merged owner question on the device login, the page login
  and `issuer`.
- **Lens B narrowed it to the opposite reading:** NAM-024's "may add optional things"
  (`naming.yaml:420-422`) covers a login appended to `signIn.logins`, as it covers an operation
  appended to a capability's list, while altering a login the earlier release stated (adding
  `issuer`) is a change `:424-425` forbids; under that line M43 and M33 defer (o6: later, within
  `codex.v2`), M32 and M42 are decided at 0.2.0 under either reading, and no owner question is
  spent, since the reading is the review's and o6 reaches the owner in the accept-or-strike list.
  Its reasons: NAM-024's own example is an element appended to a required list; every release has
  a new digest; a sign-in without `signInLogin` starts the login the hook chooses, a named login
  is used only where asked for, and a device login states no port.
- **How the record takes it:** the two lenses split, so neither reading is confirmed, and the
  record puts the reading to the owner as
  [Q5](README.md#q5-may-a-later-v2-release-add-a-login-o6-m23-m33-m43). Its option a and
  recommendation is the skeptic's reading (C01, S04): an addition that changes what an unchanged
  request may do (a further login; a member added to an existing login) is stated at 0.2.0 or
  never in `.v2`; an addition an unchanged request never exercises (a new operation with its
  implementation default) is an optional thing; under it V11 holds at should, V33 falls, and M43
  and M33 change. Its option b is Lens B's form: M43 and M33 defer. M32 and M42 are as both
  lenses have them. [Section 8](README.md#8-how-it-ran) records this.

### G04: 051b's paste is not a Claude Agent's paste route

- **Kind and level:** carrier; should. **Members:** M79, M78, M31.
- **Claim:** dec-34's standing 051b part ("the command line offers the paste when it runs on the
  node's machine and the browser is elsewhere") is PND-013's carried return, accepted only where
  the login states `carried: true` (`pending.yaml:251-252`), so it serves Codex; point 8's paste
  route for a Claude Agent holds the code only on the node, through no engine or Interface process
  (V12, V07). No survivor said so.
- **Evidence:** `docs/proposals/full-review/decisions.md:82`;
  `spec/promises/pending.yaml:251-252, 300-304`; `docs/proposals/ruling-80/README.md:154-156`;
  `docs/proposals/ruling-75/README.md:60-62`; `docs/work-orders/117-plan-s2-s3.md:88-89`.
- **Scenario:** 051b's writer has `conduit sign-in` on the node accept a pasted Claude code and
  hand it to the waiting client; a Conduit process then holds the credential, against point 8 and
  V07's condition.
- **Least fix:** the record's one line for 051b: nothing from this review changes it; dec-34's
  paste is the `carried: true` route under PND-013 and serves no Claude Agent's sign-in.
- **Verifiers:** Lens A confirmed. **Carrier:** 051b's must-state line.

### G05: the fail branch of the answer to (a)

- **Kind and level:** handed answer; should. **Members:** M21, M52.
- **Claim:** V03 makes the yes for stage 0's failure (a) conditional on the account fact and names
  the fail's consequence for `codex.v2`, but says nothing of FR-118; R1 and R4 reason that FR-118
  does not return under any outcome. The brief's output contract says a named probe's fail, where
  the yes is conditional, returns FR-118 to the owner once the probe has run.
- **Evidence:** the brief's output contract 2(a); decided item dec-35;
  `docs/proposals/full-review/questions.md:357`.
- **Least fix:** the record's (a) line: yes for failure (b); for failure (a), yes only if V03's
  probe passes; on its fail FR-118 returns to the owner, with the recommendation that `codex.v2`
  state no `signIn` and the owner's choice among O3's two remaining answers, and R1's and R4's
  reasoning as the case for keeping sign-in in the core regardless.
- **Verifiers:** Lens A confirmed. **Carrier:** the record's (a) line
  ([section 2](README.md#2-the-three-questions-the-owner-left-to-the-review)).

### G06: probes and a fixture fix that no held order carries

- **Kind and level:** carrier; should. **Members:** M21, M37, M27, M50, M70.
- **Claim:** V03's scratch-home route is a stage-1 variant needing pre-registration in experiment
  027's README, which pre-registers each later stage before it runs
  (`experiments/027-a5-sign-in/README.md:15, 19-20`); V23's P-R3-c and V19's two-start probe run the
  real clients and need the same README (`:352-359`); V20's P-R4-4 lies outside order 195's part A
  rule; V13's fixture realignment names no order. None of the five held orders carries them
  (`docs/work-orders/117-plan-s2-s3.md:84-93`).
- **Scenario:** 050's writer has no order to pre-register the probes and runs them unregistered,
  against the README and ruling 81 (`docs/proposals/ruling-81/README.md:98-99`), or 0.2.0 waits
  with no one assigned.
- **Least fix:** one carrier for each: a small pre-registration of experiment 027's README, as
  order 195 was, for the stage-1 scratch-home run, P-R3-c and the Claude two-start probe, with
  stage 0's guards and the owner told first; P-R4-4 as a read order or dropped; the fixture
  realignment with 048a-1, which carries the `signIn` fixture Type, or a named conformance fix.
- **Verifiers:** Lens A confirmed. **How the record takes it:** with S09 and S10, which make
  P-R4-4 and P-R3-c optional and name no order for P-R4-4; see
  [section 6](README.md#carriers-outside-the-held-orders).

## Dropped by the skeptic's flags

These survived Phase 2 but leave the record's findings under the skeptic's flags, as the
coordinator's decision to apply every flag has it.

- **V04** (should; M41, M49): freezing `carried: true` before a probe shows Codex's listener takes
  PND-013's rebuilt request is not safe. Lens A narrowed; Lens B refuted (every A81 outcome leaves
  `carried: true` valid; a fail ends `returned: undelivered` and polling continues). Dropped by
  S01 and S18: A81 bears on no member's freeze.
- **V05** (should, a not-safe mark; M41, B02, B08): the not-safe mark on M41 for A81. Lens A
  narrowed (a fail would put `carried: false` to the owner, which NAM-024 forecloses once frozen);
  Lens B refuted, as for V04; the workflow marked it `makes_gate: false`. Dropped by S01; G09, not
  covered, asked the record to say which lens governs, and the record follows Lens B, as the
  critic's handed table and the skeptic do ([section 8](README.md#8-how-it-ran)).
- **V14** (should; M06, M02, M04, M05, M07, M08): #62's carrier is unstated before 050. Lens A
  narrowed; Lens B refuted (NAM-024's no-default clause names only schema properties and
  Requirement inputs; a carrier exists after 0.2.0). Dropped by S02: M06 defers under V32.

## Struck

Struck in Phase 2: Lens A refuted each, so it ends.

- **V29** (M44, M48): the accounts guide's "or a label for each" cannot work. The guide's label is
  what the owner may give the coordinator in place of the emails, not a configured value; the rest
  restates decided text. At most a parked line.
- **V30** (M70): neither real package exports the standard template `turn`. It repeats known-13
  with no new fact.
- **V35** (M74 to M76, M78): the three engine-side texts are stale under ruling 80 point 8. It
  repeats known-2 to known-4 with no new fact; point 8 is not in `spec/` at T. M74 to M76 change as
  V07's surviving fix states, on dec-8 and those known items (G10).
- **V36** (M50): R2's change of M50 waits on stage 0b's part B. Part B does not run
  (`experiments/027-a5-sign-in/results-2026-11-16-0b-source.md:498-500`), and the item repeats
  known-12; V06 and V20 carry M50.
- **G03** (M56, M66, M67, M18): no test drives a canary through the turn's failure path. The
  branch already holds that test (`559ecd0b:packages/agent-codex/tests/wrapper.rs:771-848`), the
  client's stderr reaches nothing (`559ecd0b:packages/agent-codex/src/lib.rs:464`), and no failure
  cause interpolates client text. Residual, M66's: order 17 keeps the sentinel test when it
  replaces the text with fields.
