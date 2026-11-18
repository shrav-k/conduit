# Work order 048a1 (E7a-1): CTR-013's checks on a Type's sign-in, and the `.v2` fixture's `apiKey`

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share: list in `vectors` the case you add and the case whose fixtures you
change, with the promises each tests and whether it passes. This order has network access only
because the engine's tests bind loopback ports, which the sandbox refuses without it; use it for
nothing else, add no crate, and reach no other machine. Run one cargo command at a time. Never
print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `claude`, `codex` or any
client, and sign in nowhere.

## What and why

**When this order starts.** Order 046a must be merged to `main` first: it brings the rule that a new
case sends only what a client or a provider can send and asserts only what a client can see
([D36](../proposals/full-review/decisions.md#the-pipeline-and-verification), carrying
[FR-350](../proposals/full-review/findings/P07-engine-build.md#fr-350)), and the runner check that
enforces it. Stop, report and change nothing unless `conformance/internal-cases.yaml`,
`engine/crates/conformance/src/portable.rs` and `conformance/conventions/explain/` all exist: only
046a's work creates them. Line numbers below are those of `main` at `9af1c29f`; 046a may move them,
so find each place by the text quoted with it. Order 198 has merged (#339); decision 8 assumes
its F96 line. Orders 046c and 050 may land beside this one; whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool, and reports vector_delta's count on the main it starts
from.

**What it builds.** CTR-013 (`spec/promises/contracts.yaml`) says what a Type that states
`signIn` must hold. The first build checks only the part a JSON Schema can state; this order
builds the rest of the rules on one Type's own contract, and adds one rule. The full review's
fix F97 ([FR-105](../proposals/full-review/findings/L09-packages.md#fr-105)) has CTR-013 refuse
a listed control whose `changes` reaches the account place, as CTR-014 already does for the
own-tools directory: a control that rewrote `/account` would move the expected account between
two account checks (PND-011). The targeted review of the 0.2.0 sign-in surface gave F97 to this
order, with one vector, and the owner accepted that carrier
([ruling 82](../proposals/ruling-82/README.md#the-decisions), point 6, o5; the review's
[section 6, 048a-1](../proposals/review-signin-2026-11-16/README.md#048a-1), item 1; V38 and V22
in [its findings](../proposals/review-signin-2026-11-16/findings.md)).

The same list's item 2 puts here the realignment of the registration-only fixture of the Agent
packages 0.2.0 with the `.v2` schema shape that ruling 82, point 2, records as the coordinator's
reading of the review's section 6, open to the owner's strike: `apiKey` is in `.v2`, an explicit
setting only, and `account` is required with no default, a string or null under the fixture's
`oneOf` beside `apiKey` (the review's V13 and G06; its verdicts on
[M44 to M46 and M70](../proposals/review-signin-2026-11-16/verdicts.md#d-account-and-apikey)).
Today the fixture lists `apiKey` outside `required`, so MAN-017 could never bind it
(R1-09 in V13); the fixture follows 050's schema, not the reverse.

The split that names this order is in [order 117](117-plan-s2-s3.md), lines 89 and 90 as this change leaves it: "CTR-013
in the loader, IFC-016's detection, the signIn fixture Type". This order takes the first and the
third. IFC-016's detection moves to 048a-2 (decision 9): that is the coordinator's change to
order 117's split, which 117's entry records beside it. Of CTR-013, this order builds the checks
on one Type's own contract; its comparisons across Types go to a follow-up order (decision 9).

**What the engine already does, so that you do not redo it.**

- The loader validates every document against `spec/schemas/contracts.yaml`, and a failure is
  `SUB-005`. `type-spec.v1` and `sign-in-login.v1` already hold: `signIn` holds exactly `account`,
  a pointer, and `logins`, a non-empty list; each login's members for its `return` kind; literal
  HTTPS values for `authorization`, `page` and `issuer`; the loopback host, ports and path of a
  redirect; `carried` where it is required and allowed; `issuer` only on a redirect login; and,
  through `dependentSchemas`, the three hooks `sign-in-hook.v1`, `account-hook.v1` and
  `sign-out-hook.v1` under `requires`. The cases are `contracts-sign-in-logins` (27 load steps)
  and steps 5 and 6 of `loader-own-tools`. None of this changes.
- No source of the loader, the core or the shell names CTR-013: its row in `engine/STANDING.md`
  says `none`. So none of CTR-013's rules that a schema cannot state is checked.
- When a package or a project is resolved, `Catalog::resolve` in
  `engine/crates/core/src/packages.rs` checks CTR-014's three rules on a Type that states
  `toolsDirectory` (order 157; their texts are order 177's), in the loop over each scope's
  documents that begins at `if d["kind"] == "Type" {` (`:599`). Rules (a) and (b), the place and
  its path required and the place confined to string or null, are `binding_parts(..., true)`
  (`:1293`); rule (c), no listed control reaching the place, is `own_tools_control_fault`
  (`:1149`), which compares pointers by reference tokens as CTR-006 does. Both return the texts
  that decision 2 reuses as they are. Resolution runs when a project is submitted and when a
  package is installed, so a check added there holds at both.
- `sign-in.v1` is a stub that answers unavailable (DL18 in `engine/CONFORMANCE.md`), no hook
  runs, and a completion whose `resultType` is `input_required` already ends its invocation
  unknown with no diagnostic (`complete` in `engine/crates/core/src/requests.rs`).

**What is new.** The account place's two rules and the control rule, under the rule ID CTR-013;
distinct login names; no URL passing IFC-049's checks for two logins of one Type; F97's sentence
in CTR-013's `then`; one case through the public route of registration; and the fixture's
`required`.

The coordinator ran decisions 2, 3, 5 and 6 once in a scratch copy of the tree at `9af1c29f`,
before 046a: the case's five rejections each gave exactly one CTR-013 diagnostic with the texts
below, the signIn fixture Type registered, the case failed on the unchanged engine, every other
case passed, and `agents-v2-defaults` passed against the realigned fixture with its `case.yaml`
unchanged. That probe's case started from template-family's snapshot, which 046a's portable-case
check refuses; the case below starts empty and builds its state by a first-start step, which the
probe did not run. The coordinator checked only that the IDs template-family's request names
(sender and caller `...0005`, conduit `...0006`, target `...0003`) are those that
`list-from-first-start`'s step 1 expects in its state: root, `root-management-abc01` and
`management`. That was a probe, not a result: your run is the evidence.

**Who decided what.** The carrier of item 1 is the owner's (ruling 82, point 6, o5); the
fixture's shape is the one ruling 82, point 2, records as the coordinator's reading of section 6,
open to the owner's strike. F97's words are the review's must-state text, which the owner
accepted; they differ from FR-105's least change ("`changes`", "by reference tokens"), and the
review's words govern. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): reading "the loader" as the checks a Type gets
when its package or project is resolved, the diagnostic texts, the overlap test of decision 3,
the case and its route, the fixture's shape (ruling 82, point 2), keeping `apiKey`'s
`default: null`, moving IFC-016's detection to 048a-2 (a change to order 117's split), leaving
CTR-013's comparisons across Types to a follow-up order (which order 117 records now, beside
its 048a-1 entry), and the documentation edits. Apply them as written and report them
under `decisions` as the coordinator's. One follow-up stays with the coordinator: spec batch 1's
line for 048a ([the plan](../proposals/spec-batch-1/README.md), "CTR-013 in the loader and
core") is a record outside this order's scope and outside order 117's rows, so it still reads as
if all of CTR-013 came here; the coordinator notes there, in the follow-up order, that CTR-013's
comparisons across Types leave 048a-1 for a later order. The coordinator amended decision 8 and
its checks after order 050b merged first, as 050b's note on this order asks.

## The decisions it applies

1. **CTR-013 gains F97's sentence.** In `spec/promises/contracts.yaml`, CTR-013's `then`
   (`:297-298`), replace these two lines:

   ```text
       and the place is confined to string or null by MAN-017's test. The Type requires the three
       core operations sign-in-hook.v1, account-hook.v1 and sign-out-hook.v1. No URL may pass
   ```

   with these four:

   ```text
       and the place is confined to string or null by MAN-017's test. The Type lists no control
       whose changes holds the account pointer, a pointer inside it or a pointer over it, compared
       as CTR-006 compares pointers. The Type requires the three core operations sign-in-hook.v1,
       account-hook.v1 and sign-out-hook.v1. No URL may pass
   ```

   Nothing else in CTR-013 changes: not its `when`, its `observable`, its `level` or its
   `vectors` list, which keeps its one entry (the coverage index, not that list, ties cases to
   promises). The sentence that follows, "A contract failing these rules is rejected at install
   or submission.", now covers the new rule.

2. **The account place, checked as CTR-014's place is.** In the loop of `Catalog::resolve` over
   each scope's documents, for each Type (the block quoted above, at `:599`), let `account` be
   the value of the Type's `spec.signIn.account`, read as a string, as that block reads
   `spec.toolsDirectory.directory`. Where it is a string:
   - **Rules (a) and (b).** Right after CTR-014's rule (a) or (b) check, call `binding_parts` as
     that check does, with the same one `BindingPart` (the Type's `schema` as `root` and
     `schema`, `secret` false), the tokens of `account` from `pointer_tokens`, `core`, and
     `own_tools` true. An `Err(cause)` gives one error diagnostic whose `ruleId` is `CTR-013`
     and whose message is `{scope}: Type {name} signIn account {account}: {cause}`, written with
     the same `{scope}` and `{name}` as CTR-014's prefix, so the name shows as a JSON string and
     the pointer as written. `{cause}` is `binding_parts`'s text unchanged, its `rule (a)` or
     `rule (b)` label included: CTR-013 now states the same three rules as CTR-014, in the same
     order. MAN-017's own check, `binding` and its secret skip, does not change.
   - **Rule (c).** In the loop over `controls`, after the CTR-014 rule (c) check of each control,
     call `own_tools_control_fault(control, spec, account)` with the control already resolved.
     `Some(cause)` gives one error diagnostic, `ruleId` `CTR-013`, message
     `{scope}: Type {name} signIn account {account}: {cause}`. A control is resolved once for
     its CTR-006, CTR-014 and CTR-013 checks.

   The functions keep their names and texts; CTR-014's diagnostics, and the cases
   `contracts-own-tools-*`, `contracts-control-without-revision` and `inventory-own-tools-*`,
   do not change.

3. **The logins: distinct names, and no URL for two of them.** Add a function that takes the
   Type's `signIn.logins` and returns a list of texts, and call it right after decision 2's rules
   (a) and (b). Each text gives one error diagnostic, `ruleId` `CTR-013`, message
   `{scope}: Type {name} signIn: {text}`. Where `logins` is not an array the function returns
   nothing: the schema reports it under SUB-005. It reads members defensively, since the loop
   runs before the document's schema check (`env.check_document`, `:642`): a login that is not
   an object, or whose `name`, `return` or `authorization` is not a string, takes part in no name
   or overlap text; and a member that the overlap test needs (`clientId`, `redirect` with its
   `host`, `path` and `ports`, or `page`) that is absent or of the wrong type makes the pair not
   overlap. Such documents fail the schema under SUB-005 all the same. Logins are numbered
   from 0, in the list's order; `{a}` and `{b}` below are login names written as JSON strings,
   as `{name}` is. The texts come in this order:
   - **Names.** For each login `j`, in order, whose `name` equals that of an earlier login, let
     `i` be the first such earlier login: `logins {i} and {j} are both named {b}`. Three logins
     of one name give two texts, for (0, 1) and (0, 2).
   - **Overlaps.** Then, for each pair `i < j`, in order of `i` and then of `j`, that overlaps,
     `logins {i} {a} and {j} {b} could both pass IFC-049's checks for one URL: {reason}`. Two
     logins overlap only where their `authorization` values have the same origin and the same
     path, compared as `url::Url::parse` gives them (`origin()` and `path()`, so host case and a
     default port 443 do not count), or, where either does not parse, as equal strings. The core
     already depends on `url`. Then, with `{reason}`:
     - both `return` `device`, whatever their client IDs, since a device URL is `authorization`
       itself, with no query:
       `both are device logins with the same authorization origin and path`;
     - both `redirect`, with equal `clientId`, equal `redirect.host` and `redirect.path` as
       strings, and ports that meet, `any` on either side or one integer in both lists:
       `both are redirect logins with the same authorization origin and path, client ID, host and path, and ports that overlap`;
     - each `page` or `service`, with equal `clientId` and `page` values equal as parsed URLs (or
       as strings where either does not parse), since IFC-049 sends both to the stated page and
       a URL that passes `page`'s checks passes `service`'s:
       `both return to the same page with the same authorization origin and path and client ID`.

     No other pair overlaps: a device URL holds no query and every other login's URL holds
     `client_id`, and a redirect's `redirect_uri` is `http://` where a page's is HTTPS. No text
     holds a URL, a host, a port or a client ID (CTR-013's observable: "reveals no URL value").

   A Type's diagnostics then come in this order: CTR-014's rule (a) or (b), if any; CTR-013's
   rule (a) or (b), if any; the logins' texts; then, for each control in the order of
   `controls`, its CTR-006 diagnostic, its CTR-014 rule (c) and its CTR-013 rule (c), each if
   any. The resolution is rejected as for the loop's other errors (`:655`). No check below tests
   this order, since each of the case's steps gets one diagnostic and the unit test calls only
   the logins' function: the verifier reviews it in the code.

4. **A unit test pins the logins' texts.** In the module's `tests` (`:1504`), add
   `sign_in_login_faults_name_duplicates_and_overlaps`, which calls decision 3's function on
   each list below and asserts the whole list of texts with `assert_eq!`. Let R be
   `{name: browser, return: redirect, authorization: 'https://example.org/authorize', clientId: c1, redirect: {host: 127.0.0.1, ports: [8400, 8401], path: /callback}, carried: true}`,
   D be `{name: device, return: device, authorization: 'https://example.org/device', clientId: c1}`,
   P be `{name: page, return: page, authorization: 'https://example.org/authorize', clientId: c1, page: 'https://example.org/code', carried: false}`
   and S be P with `name: service`, `return: service` and no `carried`. "X with ..." changes only
   the members named.
   - `[R, D]`: no text.
   - `[D, D with name device-2 and clientId c2]`:
     `logins 0 "device" and 1 "device-2" could both pass IFC-049's checks for one URL: both are device logins with the same authorization origin and path`.
   - `[D, D with name device-2 and authorization 'https://EXAMPLE.org:443/device']`: the same
     text.
   - `[R, R with name browser-any and redirect.ports any]`:
     `logins 0 "browser" and 1 "browser-any" could both pass IFC-049's checks for one URL: both are redirect logins with the same authorization origin and path, client ID, host and path, and ports that overlap`.
   - `[R, R with name browser-2 and redirect.ports [8402]]`, `[R, R with name browser-2 and
     redirect.host localhost]` and `[R, R with name browser-2 and clientId c2]`: no text.
   - `[P, S]`:
     `logins 0 "page" and 1 "service" could both pass IFC-049's checks for one URL: both return to the same page with the same authorization origin and path and client ID`.
   - `[R, R with redirect.ports [8402]]` (both named browser): `logins 0 and 1 are both named "browser"`.
   - `[D, R with authorization 'https://example.org/device']`: no text.

   Write YAML's flow mappings above as `json!` values. The function's name and signature are
   your choice; resolution builds its messages through it.

5. **One case, through the public route of registration.** Add
   `conformance/language/contracts/sign-in-resolution/`, with ID `contracts-sign-in-resolution`,
   `layers: [language]`, `promises: [CTR-013]`, title
   `Reject a signIn Type whose account place, controls or logins break CTR-013, and register the signIn fixture Type`,
   `profile: first-build` and `initial` `{"recipe": "empty"}`. Write `case.yaml` as JSON with
   two-space indentation and a final newline, its members `id`, `layers`, `promises`, `title`,
   `profile`, `initial`, `files` and `steps`, in that order (046a's decision 11). Step 1 builds
   the state: `list-from-first-start`'s step 1 input, whole, with `expect`
   `{"reply": {"started": true}, "external_intents": [], "required_promises": ["FST-001"], "state": {"started": true}, "record_facts": []}`
   and no `capture`, as step 1 of 046a's `explain-refusal-tiers` is. Steps 2 to 7 take the
   request and the evidence of `conformance/language/registration/template-family/case.yaml`:
   in each, a copy of that case's step 1 `input.request` and `input.evidence`, except that step N
   (counted from 1) has as `ids` the 24 strings `00000000-0000-4000-8000-` followed by the
   twelve-digit, zero-padded decimal of N × 1000 + 100 + k, for k = 0 to 23 (step 2:
   `...000000002100` to `...000000002123`). Each of these steps' input is
   `kind: register-project` with `file` the step's fixture alias. The case is portable, so it
   meets 046a's rules (decision 10) as they stand: its `initial` is empty; a `register-project`
   step may keep `stateUnchanged` and `required_promises`, and the first-start step `state` and
   `required_promises`; `records` and `record_facts` are allowed; and each step whose reply has
   a reason asserts that reason and, at the first diagnostic, its `ruleId`. Its files:
   - `project/conduit.json`: `{apiVersion, address: example.org/sign-in-resolution}`, the
     `apiVersion` as in template-family's project, alias `manifest`.
   - `project/signed-agent.json`, alias `signed-agent`: **the signIn fixture Type**, two
     documents. An Operation `set-limit.v1` whose `spec` is `{inputSchema: {type: object,
     properties: {revision: {$ref: <core>#revision.v1}, limit: {type: integer}}, required:
     [revision], additionalProperties: false}, changes: {limit: /limit}}`; and a Type
     `signed-agent.v1` whose `spec` is `{family: Agent, schema: {type: object, properties:
     {account: {type: [string, 'null']}, limit: {type: integer}}, required: [account]},
     requires: [<core>#sign-in-hook.v1, <core>#account-hook.v1, <core>#sign-out-hook.v1],
     signIn: {account: /account, logins: [B, V]}, controls: ['#set-limit.v1']}`, where `<core>`
     is the core package's address from `spec/names.yaml`, B is
     `{name: browser, return: redirect, authorization: 'https://example.org/authorize', clientId: placeholder-signed-agent, redirect: {host: 127.0.0.1, ports: [8400, 8401], path: /callback}, carried: true}`
     and V is
     `{name: device, return: device, authorization: 'https://example.org/device', clientId: placeholder-signed-agent}`.
     Order 048a-2 gives this Type its scripted hook items; this order adds none.
   - Five more documents files, each `signed-agent.json` with one change, alias as the file's
     name without `.json`:
     - `project/account-control.json`: an Operation `set-account.v1` is added after `set-limit.v1`,
       written as `set-limit.v1` is with `account: {type: [string, 'null']}` for `limit` and
       `changes: {account: /account}`, and the Type's `controls` are
       `['#set-limit.v1', '#set-account.v1']`. This is the review's one vector.
     - `project/account-optional.json`: the Type's `schema` has no `required`.
     - `project/account-number.json`: the schema's `account` is `{type: [string, number]}`.
     - `project/login-names.json`: login 1 (V) is named `browser`.
     - `project/login-overlap.json`: login 1 is B with `name: browser-any` and `redirect.ports:
       any`.
   - For each of the six documents files K, `fixture-K.json`, alias `fixture-K`:
     `{snapshots: [], project: {files: {conduit.json: manifest, documents.json: K}}}`, as
     template-family's `fixture.json` is.

   After step 1, the registers, steps 2 to 7, in this order: `account-control`,
   `account-optional`, `account-number`, `login-names`, `login-overlap`, `signed-agent`. A
   rejected register allocates its evidence's IDs, hence one ID range per step; the accepted step
   comes last, since it changes the records that a later step would read.
   - Each of steps 2 to 6 expects what template-family's step 1 expects, with
     `reply: {accepted: false, reason: invalid, diagnostics: [{ruleId: CTR-013}]}`, no message
     asserted, `required_promises: [CTR-013]`, `external_intents: []`, `stateUnchanged: true`, and
     its two `records` with the `id`s replaced by the step's first two IDs.
   - Step 7 expects `reply: {accepted: true, diagnostics: [], objects: []}`,
     `external_intents: []`, `required_promises: []`, `stateUnchanged: true` and
     `record_facts: []`: a project of contracts alone makes no object.

   If 046a's runner check refuses the case all the same, stop and report its error: do not
   change the expectations to pass it. No expectation is generated from the engine's output. The coverage index entry holds `id`,
   `case` (`language/contracts/sign-in-resolution/case.yaml`), `promises: [CTR-013]` and
   `safety_rows: []`, in that order. Write it in a `cases` list in `conformance/manifest.yaml`,
   then, from `engine/`, run `cargo run --release -p conduit-conformance --example pieces` with
   no flag, which moves it into `conformance/language/contracts/index.yaml` in ID order, after
   `contracts-sign-in-logins`, and leaves `manifest.yaml` byte for byte as it was.

   The diagnostics' messages, which the case does not assert and your report gives, are then:
   - step 2: `project: Type "signed-agent.v1" signIn account /account: rule (c): control #set-account.v1 reaches the place: input "account" sets "/account", which equals the place`
   - step 3: `project: Type "signed-agent.v1" signIn account /account: rule (a): a branch of the schema does not list /account under required`
   - step 4: `project: Type "signed-agent.v1" signIn account /account: rule (b): in a branch of the schema, the place admits number, where only string and null are allowed`
   - step 5: `project: Type "signed-agent.v1" signIn: logins 0 and 1 are both named "browser"`
   - step 6: `project: Type "signed-agent.v1" signIn: logins 0 "browser" and 1 "browser-any" could both pass IFC-049's checks for one URL: both are redirect logins with the same authorization origin and path, client ID, host and path, and ports that overlap`

6. **The `.v2` fixture requires `apiKey` beside `account`.** The fixture
   `conformance/fixtures/s1/packages/agents-0.2.0/` is generated by
   `engine/crates/conformance/examples/make_archives.rs`, with its archive, `digests.json` and
   the two `-values.json` files of `agents-v2-defaults`. In `release_fixtures`, the `.v2` schema's
   `"required":["account"]` becomes `"required":["account","apiKey"]`, and after the comment
   line `// C12: registration-only fixtures, no signIn or hooks. PKG-031: turn template.` (`:60`)
   add, at the same indent, the line
   `// Ruling 82 point 2: account and apiKey are both required; apiKey defaults to null.`
   Nothing else in either `.v2` schema changes: `account` keeps `{type: [string, 'null']}` with
   no default, `apiKey` keeps its `anyOf` of the core secret reference and null and its
   `default: null`, and the `oneOf` stays as it is. So a template that writes no `apiKey` gets
   null, the sign-in branch; a key is used only where a template writes one; and a template must
   state its `account`. This shape stays as written: it is the coordinator's reading of the
   review's section 6, as ruling 82, point 2, records it, open to the owner's strike. `apiKey`
   defaults to null, `account` is required with no default, and the `oneOf` is
   unchanged. Order 050, written beside this one, must state the same shape for the real
   packages' `.v2` Types. Then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example make_archives`. It must report the
   digest `sha256:12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7` for
   `agents-0.2.0`, and change, of what it writes, only
   `conformance/fixtures/s1/packages/agents-0.2.0/contracts.json`,
   `conformance/fixtures/install-archives/agents-0.2.0.tar.gz`,
   `conformance/fixtures/install-archives/digests.json` and the two `-values.json` files. Write
   that digest in place of the old one in
   `conformance/language/packages/agents-v2-defaults/install.json`. The case's `case.yaml` does
   not change: SCH-009 fills `apiKey`'s default before validation, so every sample keeps its
   verdict and its value. If any sample's verdict or the digest differs, stop and report. The
   dated results and states under `experiments/023-paper-count/` keep the old digest: they are
   evidence of their day. The plan-and-apply reference keeps it too:
   `docs/implementation/plan-and-apply/README.md` (its T1 install line) and its golden cases
   `docs/implementation/plan-and-apply/cases/01-t1.yaml` to `09-t7-restore-unflagged.yaml` name
   it 151 times, and their plan digests hash plan texts that hold it, so a plain replacement would
   leave those digests wrong. Order 052a-1, the first of the two orders that order 117 splits
   052a into (plan and apply), or whichever of 052a-1 and 052a-2 first builds plan against these
   cases, re-pins the archive's digest in them and recomputes what follows from it; 117's entry
   for 052a-1 and 052a-2 says so. Change nothing under `docs/implementation/`.

7. **The declaration.** Write `docs/work-orders/048a1-declared.yaml` with exactly two shapes: an
   `added` shape named `contracts-sign-in-resolution` for that case; and a `fixture` shape named
   `the 0.2.0 fixture's .v2 schemas require apiKey beside account`, `cases:
   [agents-v2-defaults]`, whose `paths` are, in this order,
   `conformance/fixtures/install-archives/agents-0.2.0.tar.gz`,
   `conformance/language/packages/agents-v2-defaults/install.json`,
   `conformance/language/packages/agents-v2-defaults/claude-code.v2-values.json` and
   `conformance/language/packages/agents-v2-defaults/codex.v2-values.json`. The coordinator
   wrote it with this order; check that it matches and change it only to make it so.

8. **The documents that say what the engine checks, and IN_FORCE.md.**
   - `spec/schemas/contracts.yaml`, the header comment: line 58, which ends "that no control of
     a Type stating", stays, and the four lines after it (`:59-62`), from
     `` # `toolsDirectory` has `changes` reaching its place (CTR-014), which needs the contracts of ``
     to `# URL matching two logins, and client IDs shared across addresses.`, become these four:

     ```text
     # `signIn` or `toolsDirectory` has `changes` reaching its place (CTR-013, CTR-014), which
     # needs the contracts of the controls; that a `default` satisfies its schema (SCH-019);
     # compatibility between releases (NAM-024); and CTR-013's other rules: the three hooks,
     # distinct login names, no URL matching two logins, and client IDs shared across addresses.
     ```

     Change nothing else in that file.
   - `spec/schemas/README.md`, the table row that begins
     `` | That the place each pointer of `signIn` and `toolsDirectory` names ``: replace
     `` and that no control of a Type stating `toolsDirectory` has `changes` reaching its place ``
     with `` and that no control of a Type stating `signIn` or `toolsDirectory` has `changes` reaching the place its pointer names ``.
     In the paragraph that begins "CTR-013 adds cross-document checks" (`:299-302`), replace
     "no URL matching two logins\nunder IFC-049, and client IDs shared across addresses" so that
     the paragraph reads, rewrapped at 100 columns:

     ```text
     CTR-013 adds cross-document checks: the required configuration path confined to string or
     null, the three required hook selections, distinct login names, no URL matching two logins
     under IFC-049, no control whose `changes` reaches the account place, and client IDs shared
     across addresses, including held submissions. The account must be attested by the service;
     configuration or a person's account claim does not suffice.
     ```

   - `IN_FORCE.md`, topic 9, the bullet that begins `- *Spec pending*:` and holds
     "F97, by order 048a-1:". Order 050b, which merged first, left in it F97's sentence alone,
     ending "with one loader vector (82 point 6, o5).": delete the whole line. Right after the
     bullet that begins `- **In spec/**: F96:`, which order 198 put there, and so before the
     bullet that begins `- **In spec/**: F95:`, which order 050b put after it, add this bullet,
     one line, at the same level:

     ```text
     - **In spec/**: F97: CTR-013 lets a Type that states `signIn` list no control whose `changes` holds the account pointer, a pointer inside it or a pointer over it, compared as CTR-006 compares pointers, as CTR-014 does for the tools directory; the engine rejects such a Type when its package is installed or its project is submitted, and the case `contracts-sign-in-resolution` shows it (82 point 6, o5).
     ```

     Put no date in it: `check_rulings.py` refuses one. Change no other line of IN_FORCE.md.
   - `engine/STANDING.md`: after decisions 1 to 5, run `python3 tools/promise_standing.py
     --write`, as AGENTS.md asks of a change to a promise and to a case's `promises`. CTR-013's
     row becomes `| CTR-013 | language | must | 3 | 0 | outside | core | case passes |`, and the
     totals change only by what that row changes. If 046a changed CTR-013's count first, the
     count is its count plus one; report it.

9. **What this order does not do.**
   - **IFC-016's detection** stays out, for 048a-2, which raises PND-005, its other half: the
     coordinator's change to order 117's split, which named it here. IFC-016
     says that a URL elicitation outside `sign-in.v1` ends the call unknown and raises PND-005
     only for a Type stating `signIn` with a string account, and that otherwise the call ends
     unknown "with a diagnostic naming the absent signIn or expected account". The ending unknown
     is built; what is missing is that diagnostic, which rides on order 046c's diagnostic for an
     engine-decided outcome, and the raise, which needs 047's pending actions. Neither is a
     check on a Type's contract. Change nothing in `requests.rs`.
   - **CTR-013's comparisons across Types**: client IDs shared across addresses at install,
     registration and replacement, `confirmClientIds`, the REC-002 record, and the `logins`
     outputs of `install.v1` and `inspect.v1`. They compare one Type with what the engine holds,
     another behavior, and go to a later order, this order's follow-up, which the coordinator
     writes after this order; order 117 records it now, in the sentence after its 048a-1 entry
     (the coordinator's decision, open to the owner's strike). Spec batch 1's line for 048a
     ("CTR-013 in the loader and core") is outside this order's scope and is not changed here
     ("Who decided what"). The header comment of decision 8 keeps "client IDs shared across
     addresses" among what the schema does not check, and the README's paragraph keeps it among CTR-013's
     cross-document checks: both stay true until that order lands.
   - **The fixture** keeps no `signIn` and no hooks (spec batch 1's C12) and gains none of the
     inputs that orders 050a and 076 add on their branches (`directory`, `turnDeadlineSeconds`,
     `effort`); order 050 brings the real packages.
   - The expected failure `requests-sign-in-return-kept-nowhere` stays, and so does its reason,
     which names this order and 048a-2: PND-013 and PND-014 are 048a-2's.
   - No other promise, schema, case, fixture or source changes: not `conformance/case.schema.json`,
     `conformance/case-schema/`, `conformance/expected-failures.yaml`, the by-rule outcome table
     `engine/crates/shell/tests/expected/by-rule-outcomes.txt` (no by-rule example states
     `signIn`), `engine/CONFORMANCE.md` or `docs/proposals/`.

## Scope

Change only `spec/promises/contracts.yaml` (CTR-013's `then`), `spec/schemas/contracts.yaml`
(its header comment), `spec/schemas/README.md` (one row and one paragraph),
`engine/crates/core/src/packages.rs`, `engine/crates/conformance/examples/make_archives.rs`,
`conformance/language/contracts/index.yaml`, `IN_FORCE.md` (one bullet of topic 9 and the one
added), `engine/STANDING.md`, written only by the tool, and the files that `make_archives`
writes as decision 6 lists them, with
`conformance/language/packages/agents-v2-defaults/install.json`. Create
`conformance/language/contracts/sign-in-resolution/` with the files decision 5 lists.
`docs/work-orders/048a1-declared.yaml` comes with this order; change it only as decision 7 says.
Not the experiments, `tools/`, the case schema or any other source. `conformance/manifest.yaml` holds the new
entry only until the pieces tool moves it, and ends unchanged.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in_login_faults own_tools_
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance --example make_archives -- --check
cargo run --release -p conduit-conformance -- .. contracts-
cargo run --release -p conduit-conformance -- .. loader-own-tools
cargo run --release -p conduit-conformance -- .. agents-v2-defaults
cargo run --release -p conduit-conformance -- .. inventory-own-tools-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 5 passed and 0 failed: the new test and order 157's and 177's four; if 046a
added a test whose name matches, report the count. The pieces tool prints `Nothing to move.`,
and `make_archives -- --check` ends `41 reproducible archive fixtures verified`. The runner
commands end `6 cases passed`, `1 cases passed`, `1 cases passed` and `3 cases passed`. The
threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decisions 2 and 3. From `engine/`, one at a time:

```bash
cp crates/core/src/packages.rs target/packages.rs.fixed
git show HEAD:engine/crates/core/src/packages.rs > crates/core/src/packages.rs
cargo run --release -p conduit-conformance -- .. contracts-sign-in-resolution
cp target/packages.rs.fixed crates/core/src/packages.rs
cargo run --release -p conduit-conformance -- .. contracts-sign-in-resolution
```

The first runner command must fail, printing the line `FAIL 0 cases passed; 1 failed` followed
by an error at step 2 whose reply is accepted; after the restore the second ends
`1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification time,
so cargo would not rebuild the crate. Git ignores `target/`.

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 -m unittest discover -s tools/tests
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a1-declared.yaml; echo "exit $?"
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/expected-failures.yaml conformance/language/packages/agents-v2-defaults/case.yaml engine/crates/shell/tests/expected experiments docs/implementation
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a1-declared.yaml: 2 shapes.`,
then `Vector delta: 5 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`: the added case and
the four fixtures of `agents-v2-defaults`. `promise_standing.py --check` ends `Errors: 0`.
`check_roadmap.py` ends `Errors: 0`. The unit tests, run with the interpreter that has PyYAML,
end `OK` (123 tests at `9af1c29f`; report the count). The `git status` command prints nothing.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`, and the command's last
line is the doc-tests' `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
followed by its time.

This prints `[['account', 'apiKey'], ['account', 'apiKey']] [False, False] [None, None]`:

```bash
python3 -c "import json; d=json.load(open('conformance/fixtures/s1/packages/agents-0.2.0/contracts.json')); v=[t['spec']['schema'] for t in d if t['metadata']['name'].endswith('.v2')]; print([s['required'] for s in v], ['default' in s['properties']['account'] for s in v], [s['properties']['apiKey'].get('default', 'absent') for s in v])"
```

Each of these prints 1. The `tr` check reads the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
tr -s ' \n' '  ' < spec/promises/contracts.yaml | grep -c 'The Type lists no control whose changes holds the account pointer, a pointer inside it or a pointer over it, compared as CTR-006 compares pointers. The Type requires the three core operations'
grep -c 'sha256:12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7' conformance/language/packages/agents-v2-defaults/install.json
grep -c '"agents-0.2.0.tar.gz": "sha256:12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7"' conformance/fixtures/install-archives/digests.json
grep -c '^- \*\*In spec/\*\*: F97: CTR-013 lets a Type' IN_FORCE.md
grep -A1 '^- \*\*In spec/\*\*: F96:' IN_FORCE.md | grep -c '^- \*\*In spec/\*\*: F97: '
grep -A1 '^- \*\*In spec/\*\*: F97:' IN_FORCE.md | grep -c '^- \*\*In spec/\*\*: F95: '
grep -c '^- \*\*In spec/\*\*: F95: the limits in spec/README.md' IN_FORCE.md
grep -c '^| CTR-013 | language | must | 3 | 0 | outside | core | case passes |$' engine/STANDING.md
grep -c 'no control whose `changes` reaches the account place' spec/schemas/README.md
grep -c '`signIn` or `toolsDirectory` has `changes` reaching its place (CTR-013, CTR-014)' spec/schemas/contracts.yaml
grep -c 'reaching the place its pointer names' spec/schemas/README.md
grep -c 'Ruling 82 point 2: account and apiKey are both required' engine/crates/conformance/examples/make_archives.rs
```

Each of these prints 0:

```bash
grep -c 'F97, by order 048a-1' IN_FORCE.md
grep -c '^- \*Spec pending\*: .*(82 point 6, o5)\.$' IN_FORCE.md
grep -rc '479ef47083c8b151eb9cabf54ee500e9ed1972e4293393dc17bbad2da6e07c71' conformance | grep -v ':0$' | wc -l
```

This prints `1 1 IN_FORCE.md` (the numbers and the name separated by tabs): one line added and one
deleted, so no other line of `IN_FORCE.md` changed:

```bash
git diff --numstat -- IN_FORCE.md
```

This prints 151, the old digest's occurrences in the plan-and-apply reference, which keeps it
for order 052a-1 (decision 6):

```bash
cat docs/implementation/plan-and-apply/README.md docs/implementation/plan-and-apply/cases/*.yaml | grep -o '479ef47083c8b151eb9cabf54ee500e9ed1972e4293393dc17bbad2da6e07c71' | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, and each of the five diagnostics' messages that the case's steps 2 to 6 get
(decision 5 gives them). If a message differs, report it and say why.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a1-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-18, from about 04:08 to 05:00 UTC, on the Mac from `main` `408d5f17` (order 048a-1's
amendment), in two rounds: the worker (1610 s) and a fresh Codex verifier (372 s), then the worker
again (820 s) and a second fresh Codex verifier (310 s). The change is the files of Scope. CTR-013's
`then` gains F97's sentence. `packages.rs` checks, when a Type that states `signIn` is resolved,
that its account path is required and confined to string or null and that no control's `changes`
reaches it, resolving each control once for CTR-006, CTR-014 and CTR-013; that login names are
distinct, citing the first; and that no two logins could pass IFC-049's checks for one URL, in pair
order; with decision 4's unit test. The case `contracts-sign-in-resolution` starts empty, rejects
five projects with decision 5's five messages and registers the signed agent. The `.v2` schemas of
the 0.2.0 fixture require `apiKey` beside `account`, whose archive's digest becomes
`sha256:12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7`; the schema documents,
IN_FORCE.md's F97 bullet and `engine/STANDING.md` (CTR-013 at 3 cases, in `core`) follow. The
worker's own choices: a parsed view of the logins that keeps their positions; ports compared as
numbers, so 8401.0 matches 8401; the citation of IFC-049 built with `concat!`, so that the
standing changes only for CTR-013; and the negative control run against `408d5f17`, since the
worker's HEAD held the change, with its failure placed at step 2 by one-step and two-step scratch
copies, since the runner prints no step number.

The first verifier found three shoulds: a redirect `path` such as `/callback?x` passes the schema's
`^/` and can be reported as overlapping, though IFC-049 compares the path without query; the
documents of decision 8 named the three hooks among what the schema does not check, though its
`dependentSchemas` requires them; and the header presented client IDs shared across addresses as
checked. Round 2 rewrote both texts of decision 8, crediting the schema with the hooks and calling
the client-ID checks pending in the engine, and left the redirect path open, since it needs a
coordinator's decision and a change to the schema's body, outside Scope. The second verifier found
two musts outside Scope: the redirect path above, and a page URL with user information, which the
page pattern admits.

A fresh read-only Opus reader then verified the order (ruling 79, point 2) on the branch based on
`408d5f17`. It ran every check of "Done when", with scratch copies for the commands that write:
fmt, clippy, the five unit tests, `Nothing to move.`, 41 archives, the runner's 6, 1, 1 and 3 cases,
the vectors (2 passed) and the full suite (28 `ok` lines, no failure), all with 4 threads; the
negative control against `408d5f17` (`FAIL 0 cases passed; 1 failed`, uncited `CTR-013`, reply
accepted; `1 cases passed` after the restore); the Python checks (Errors: 0 each; 129 tests, OK);
the vector delta (5 changes, 0 flips, 0 undeclared, exit 0); the spike (9 passed); the `.v2` probe,
the twelve greps that print 1 and the three that print 0, the `1 1` numstat and the 151 count; and
it compared the case and the IN_FORCE.md bullet with decision 5 and the order, and found both
commits' author and committer to be the work-order identity. It found no must and three shoulds:

- S1: round 2 did not apply decision 8's texts as written, and its `pending in the engine`
  sentences in both files reverse decision 9, which keeps client IDs shared across addresses among
  the cross-document checks until the follow-up order lands; only the hooks correction was sound.
- S2: the `concat!` that hides IFC-049 from `promise_standing.py` had no comment saying why.
- S3: the second verifier's two musts, the redirect path with `?` or `#` and the page URL with
  user information, need a home outside this order.

The fixer rebased the branch onto `origin/main` at `9472f5d4` (orders 050d and 047b, after order
046c) with the work-order identity, without conflict; `engine/STANDING.md`, which order 046c also
wrote, keeps both changes and agrees with the tool. It then applied each finding:

- S1: the reader's second option. In `spec/schemas/contracts.yaml` the header's last two lines of
  that sentence read `... and CTR-013's other rules: distinct login names,` and `# no URL matching
  two logins, and client IDs shared across addresses.`, decision 8's text without the hooks, and
  round 2's added paragraph is gone. In `spec/schemas/README.md` the paragraph is the reader's five
  lines, which begin `CTR-013 adds cross-document checks:` and say that the Type schema requires
  the three hook selections.
- S2: the comment explains the split, in the reader's words, except that it says "the interface
  rule" where the reader wrote the rule's ID: `promise_standing.py` counts an ID in a comment, so
  the reader's text moved IFC-049's row to `core` and the source total to 321
  (`promise_standing.py --check` gave Errors: 1). The comment now reads
  `// Split so tools/promise_standing.py does not count the interface rule as built:` and
  `// the engine only cites it in this text; CTR-013 makes the decision.`
- S3: recorded here as open items for the CTR-013 follow-up order, or a schema order: a redirect
  `path` forbids `?` and `#`, and a page URL forbids user information. 048a1 is not held for them.

After the rebase, from the worktree, the fixer ran: in `engine/`, `cargo fmt --check` and `cargo
clippy --all-targets -- -D warnings` (exit 0 each), the five unit tests (`5 passed; 0 failed`),
`pieces` (`Nothing to move.`), `make_archives -- --check` (`41 reproducible archive fixtures
verified`), the runner on `contracts-`, `loader-own-tools`, `agents-v2-defaults` and
`inventory-own-tools-` (6, 1, 1 and 3 cases passed), the vectors (`2 passed; 0 failed`), the full
suite (28 `ok` lines, none failed, exit 0), each with 4 threads, and the negative control with
`origin/main`'s `packages.rs` (`FAIL 0 cases passed; 1 failed`, uncited `CTR-013`, reply accepted;
`1 cases passed` after the `cp` restore); in the spike, `cargo test --release` (`9 passed; 0
failed`, then the doc-tests' `0 passed`); from the root, `check_schemas.py`, `set_names.py --check`,
`check_rulings.py`, `check_docs.py`, `check_roadmap.py`, `check_private_paths.py` and
`promise_standing.py --check` (Errors: 0 each), the tool tests (129, OK), `vector_delta.py
origin/main WORKTREE` with the declaration (`2 shapes`, `5 changes; 0 flips; 0 undeclared; 0
errors`, exit 0, on the main it starts from), the order's `git status` (empty), the twelve greps (1
each), the three (0 each), the numstat (`1 1`), the 151 count, the `.v2` probe (as the order prints
it), and `git diff --check origin/main...HEAD` and `git diff --check` (clean). The gate's run on
this order's squash, which this merge waits on, is the coordinator's at the merge.

At the merge the coordinator applied the second reader's one should in spec/schemas/README.md: the
sentence on the hooks names the schema type-spec.v1, which requires them, not the Type's own
configuration schema. It added a check line to 048a1-declared.yaml, which reached main with this
order's text, so that the gate's vector check reads it. The two schema musts of the second
verifier (a redirect path that forbids `?` and `#`, and a page URL that forbids user information)
stay open for the follow-up order on CTR-013's comparisons across Types, or a schema order. The gate
runs on the squash, and the merge waits on its PASS.
