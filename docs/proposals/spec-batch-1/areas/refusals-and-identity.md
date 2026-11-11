# Spec batch 1, area: refusals-and-identity

Not binding. An area planner's draft for [the batch 1 plan](../README.md); where the plan differs, the plan governs.

## Scope

Ruling 52, points 1 and 2: the two tiers of a refusal and the rule of explicitness. This changes OPS-023 and what a result, a refusal, a diagnostic and a notice carry. Ruling 49: the expected account, checked after a sign-in, and a home for each Agent, with what goes into spec/ separated from what stays in the Agent packages. The Agent Types .v2 provider work: claude-code.v2 and codex.v2, with reach flags false by default, true descriptions of what each client reaches around the engine (findings 4-06 and 3-07), an API key as a sign-in choice (finding 4-02), and ruling 63.3's standard `turn` Conduit template, all folded into one .v2 so that no .v3 follows. Repository: /Users/shravankumaran/Projects/Conduit/conduit (read only; nothing was modified).

## Promise and core changes

Every promise ID below is tentative. The batch editor assigns IDs once across all areas. Do not use OPS-035, which /Users/shravankumaran/Projects/Conduit/conduit/docs/proposals/projects/brief.md reserves for group B's act-for.

A. REFUSALS AND EXPLICITNESS (ruling 52, points 1 and 2)

A1. NEW ACC-040, "What a refusal says" (conventions/access.yaml, convention, must). One place holds the tiers and one row for each refusing rule, so it is written once.
- when: A request reports refused. The engine may decide it, or an implementation may give the reason under IFC-026.
- then: The result carries one error diagnostic.
  - Its `ruleId` is the ID of the promise that refused the request.
  - Its `properties.refusal` follows a new core schema, refusal.v1.
- Tier 1, given to every caller, an Agent's model among them:
  - the operation, by its qualified name;
  - what the request named, as the caller named it: the tool, the Conduit's local name, and each object its inputs name, by the reference written;
  - the cause that the refusing promise's row states. For ACC-001 the cause is that no open Conduit from the caller to that target lists the operation.
  - Tier 1 states nothing that depends on whether a named object exists, who owns it, or what state it is in.
- Tier 2 is given where the caller would be permitted `inspect.v1` of the target that authorizes the request under ACC-033, and of each object the request names, through an open Conduit to which no filter applies. It is always given to the root Interface. It adds:
  - that target;
  - the Conduits from the caller to it, each with its name, local name, state and operations, as inspecting the target shows them. That covers open Conduits, Conduits registered and not yet open, and closed ones whose tombstone (ruling 53's LIF-038 mark) is kept;
  - or, instead, that a named object does not exist.
- observable:
  - A caller without that authority gets the same diagnostic whether the named object exists or not, apart from the reference it wrote, which is echoed.
  - The rows (the cause each refusing rule states):
    - ACC-001: the sentence above.
    - ACC-033: the request is authorized against the target its row names. Tier 1 names that target only where the request itself names it (`management`, `access`, the instance for a control, a Pool written as `owner` in the submitted document). Otherwise it describes the target by its row, for example "the Pool that owns the Resource named".
    - ACC-032: reserved to the root Interface, acting on its own behalf.
    - SEC-007: get-secret.v1 is never dispatched for a request; the engine alone reads a secret, under SEC-010.
    - SEC-008: only the root Interface, on its own behalf, opens such a Conduit, and only for a Resource or Agent consumer.
    - ACC-024: as amended by ruling 59 A3.
    - ACC-039: the consumer is retired.
    - ACC-019, ACC-034, IFC-028: the served invocation has ended, its Conduit is closed, or it is not one the sender serves.
    - IFC-021: a Resource makes requests only for an invocation it serves.
    - IFC-032 and IFC-022: no instance was established.
    - LIF-034: the object is a first-start object.
    - ACC-029, ACC-030 and ACC-008: the bound or the filter, also named in `restriction`.
    - IFC-026: the implementation refused, with its restriction named.
    - Rules added by rulings 53, 59, 62 and 66 add their own rows.
  - IFC-025's text block, and the `message` of IFC-012's error, repeat the diagnostic's message. A client that gives its model only `content`, or only the error's `message`, therefore passes the explanation on. Experiment 008, case C1, showed both clients passing the -32602 message to the model.
  - The record states the rule and the tier, but not the tier-2 list.

A2. NEW IFC-048, "What the engine says" (interface/implementation.yaml, interface, must). This is ruling 52's point 2, placed where results and diagnostics are stated.
- when: The engine gives a consumer or an Interface a result, a diagnostic, a condition or a notice.
- then: It says what happened and why, in full:
  - the operation and what the request named;
  - the reason or the outcome;
  - the ID of the promise that decided it;
  - the cause, in words.
  - It leaves out only what ACC-040's tiers and the inventory's disclosure rule (ruling 53, LIF-013) withhold from that caller.
- observable:
  - Every result that ends with a reason carries an error diagnostic whose `ruleId` names the deciding promise: ACC-040's for refused, as today for invalid and conflict, and the deciding rule's for unavailable (CON-024, ACC-014, ACC-037, SEC-012 and others).
  - A failed or unknown outcome that the engine itself decided carries a diagnostic naming the cause, for example under IFC-010, OPS-027 or a deadline. An implementation's own content is passed as IFC-025 states.
  - The first text block states the reason or the outcome, then each error diagnostic's message.
  - A condition's `message` states its cause.
  - A pending-action notice states its kind, subject, evidence and the answer asked.
  - The wording is the engine's own. Vectors check `ruleId`, the reason, and the fields of refusal.v1.

A3. CHANGED promises:
- OPS-023:
  - The order of reasons stays.
  - The sentence "its result says nothing else about the objects it names" becomes: "it carries ACC-040's diagnostic and nothing about any other reason that applies".
  - The ACC-037 branch now carries one diagnostic, with `ruleId` ACC-037, that names no object. Today it carries none.
- OPS-005: in the -32602 case, the reason and the explanation are carried in the protocol's error.
- OPS-029, ACC-001 and the observable of ACC-033: "says nothing of whether the thing exists" becomes ACC-040's tier rule. The rest of ACC-033's observable is unchanged.
- IFC-012:
  - The error's `message` says that no tool of that name is presented, and gives ACC-040's statement.
  - Its `data` holds `io.github.shrav-k.conduit/reason` and the refusal diagnostic.
  - Tier 1 must be identical for a tool name never presented and for one whose Conduit was closed.
- IFC-016: tasks requests that do not proceed are answered in the same form.
- IFC-025:
  - Remove "A result that reports refused carries none, and says nothing there of the objects its request names".
  - A refused result carries ACC-040's diagnostic, and an unavailable result carries its deciding rule's diagnostic.
  - The rule for the text block follows IFC-048.
- IFC-032: in its observable, a refusal of an unidentified caller says that no instance was established. A 401 may carry `error_description`, as RFC 6750 allows.
- LIF-037: the observable's disclosure sentence follows ACC-040. Coordinate with ruling 53's extension of LIF-037 to open.v1 and close.v1.
- REC-001 (a coordinator choice): the record of a request that ended with a reason states the deciding promise's ID, and for a refusal the tier given.
- Also update:
  - the header comment of /Users/shravankumaran/Projects/Conduit/conduit/spec/core/operations.yaml, around line 193;
  - the matching sentences of /Users/shravankumaran/Projects/Conduit/conduit/spec/core/README.md, around lines 246, 386 and 446-451.

A4. CORE PACKAGE AND SCHEMAS:
- NEW Schema refusal.v1, in spec/core/results.yaml:
  - `operation`;
  - `named`: `{tool, conduit, objects[]}`;
  - tier-2 members `target`, `conduits[]` (each `{name, as, state: open|registered|closed, operations[]}`) and `exists`.
- diagnostic.v1's description: for a refusal, `properties.refusal` holds refusal.v1. This is SARIF's property bag, as schema rejections already use it.
- Export refusal.v1 in spec/core/conduit.yaml, add it to the README tables, and add examples under spec/schemas/examples.

A5. definitions.md:
- A new row, "refusal": two tiers (ACC-040).
- Updated rows:
  - "diagnostic", "result" and "reason": every reason carries a diagnostic (IFC-048);
  - "access decision": the ACC-037 diagnostic;
  - "permitted": a pointer to ACC-040.

B. THE EXPECTED ACCOUNT, CHECKED (ruling 49, point 3)

B1. NEW CTR-013 (promises/contracts.yaml, language, must).
- when: A Type contract states `signIn.account`, a JSON Pointer into its configuration.
- then: The contract is rejected at install or at submit if:
  - the place is not required with every member on the way to it;
  - the place is not confined to a string or null, by MAN-017's test;
  - or the Type does not list core#account-hook.v1 and core#sign-out-hook.v1 under `requires`.
- observable:
  - The place names the account that the instance's sign-ins are for. The same rule serves an Agent's home and a kind-3 Resource, so Resources are not foreclosed.
  - The value at that place is the "account expected" in the evidence of a sign-in pending action (ruling 56, decision 7).

B2. type-spec.v1 (spec/schemas/contracts.yaml) gains an optional member: `signIn: {account: pointer.v1}`, which is required inside `signIn` and closed. The loader's strict schema follows.

B3. NEW core Operations (spec/core/operations.yaml, conduit.yaml exports, README tables):
- account-hook.v1:
  - no inputs; outputs `{account: string|null}`;
  - states no `effect` and starts no model work;
  - reports the account the client is signed in to, read by the implementation from its own client. The engine reads none of the home.
- sign-out-hook.v1:
  - no inputs; its `effect` removes the instance's sign-in from where its client keeps it.
- Both are dispatched through no Conduit, like setup-hook.v1 and retire-hook.v1, and no consumer can call them.

B4. NEW account-check promise(s), placed in the file that the pending-action area creates for sign-ins (convention, must).
- when: A sign-in pending action for an instance whose Type states `signIn` is completed by the person, as the implementation's completion notice says.
- then: The engine dispatches account-hook.v1 to the instance, then:
  - Where the hook succeeds with an account equal, ignoring ASCII case, to the value at the `signIn.account` place, the action's answer is `signed-in`, and the request the sign-in held is dispatched.
  - In every other case (another account, null, failed or unknown), the engine dispatches sign-out-hook.v1. The answer is `refused`, with the expected account and the account reported as evidence, shown where the action was raised. A new sign-in is raised when the implementation asks again. No held request is dispatched.
  - Where sign-out-hook.v1 does not succeed, the instance shows a condition, for example `SignInUndone=False` with its cause. Nothing that starts model work is dispatched to it until a reconciliation records the sign-out. This fails closed.
- Also in these promises: an implementation's URL elicitation is not raised as a sign-in for an instance whose Type states no `signIn`, or whose place holds null. Its request reports unavailable, with a diagnostic naming the missing expected account. This is the strict reading of "Checked" (see ruling_points).
- observable:
  - The engine compares only what the hook reports with the configuration.
  - Records hold both accounts.
  - The hooks are dispatched for the Interface that answered the action, which is root in v0 and a scoped Interface under ruling 66.3. The editor confirms this.

B5. definitions.md: new rows "expected account" and "account check"; the hook row gains the two hooks.

C. A HOME FOR EACH AGENT (ruling 49, "What follows")

C1. NEW PRV-029, "An instance's state directory" (promises/providers.yaml, convention, must).
- when: The engine's own runner starts the process of a run of an instance.
- then: It gives the run a directory of the instance's own:
  - made, empty and private to the engine's operating-system user (mode 0700), before the instance's first run;
  - the same directory for every later run, including after the engine restarts;
  - never a directory that another instance had: its path holds the instance ID;
  - removed with its contents once the instance is removed (LIF-035) or unregistered;
  - named in the variable CONDUIT_STATE_DIR.
- The engine reads nothing in it, records nothing of it and passes it to no other run.
- observable:
  - This is systemd's StateDirectory= paradigm.
  - Another run under the same operating-system user can still read it (DL16; mediated, ACC-027).
  - A runtime provider other than the engine's own runner gives such a directory only where its contract states it does. A Type whose implementation needs one says so, and under a runner that gives none its setup fails with an explicit diagnostic.

C2. CHANGED:
- MAN-018: also rejects an item that binds CONDUIT_STATE_DIR. Otherwise a configuration could point an Agent at another Agent's login.
- PRV-023: the observable names the variable.
- LIF-035: a cross-note that removal removes the state directory.
- DL16 in implementation-defined.yaml: where the directory lives in the store.
- names.yaml: new key `stateDirVariable: CONDUIT_STATE_DIR`, coined for the owner to strike, as ruling 43's names were. tools/set_names.py learns the key.

C3. definitions.md: a new row, "state directory". "Home" stays a word of the packages.

C4. What stays in the packages and not in spec/:
- CLAUDE_CONFIG_DIR and CODEX_HOME are subdirectories of CONDUIT_STATE_DIR.
- Codex's file credential store. It needs `-c cli_auth_credentials_store="file"` on `codex login` and on every launch, because `--ignore-user-config` skips the home's config.toml.
- The login flows: Claude Code by `claude auth login` with its own browser callback on the engine's machine, never the pasted code; Codex by `codex login --device-auth`, with its user code passed as evidence.
- The two hooks, and a retire hook that signs out before the directory is removed. On macOS, Claude Code keys its Keychain entry to CLAUDE_CONFIG_DIR, and removing the directory leaves that entry behind.

D. AGENT PACKAGES 0.2.0 (provider work, not binding; packages/agent-claude-code and packages/agent-codex)

D1. Schema of claude-code.v2:
- `accountServices` and `peerMessaging`: `{type: boolean, default: false}`;
- `model`: `{type: string, default: ''}`;
- `account`: `anyOf [string minLength 1, null]`. Required, with no default, if the owner keeps the recommendation in ruling_points O1;
- `apiKey`: `anyOf [core#secret-reference.v1, null]`, default null;
- every property required; `additionalProperties: false`;
- `signIn: {account: /account}`;
- `requires`: account-hook.v1, sign-out-hook.v1 and retire-hook.v1, with defaults pointing at the wrapper;
- `environment` gains `AGENT_API_KEY: /apiKey` (a secret, under SEC-010) and `AGENT_ACCOUNT: /account`, a login hint used where a client takes one.

codex.v2 is the same, but drops `peerMessaging`, which changes no Codex argument. A flag that does nothing breaks the rule of explicitness.

D2. Wrapper behavior:
- At setup or on a lapse, the wrapper checks the client's own status. If signed out, it starts the client's login and raises a URL elicitation, through the path the pending-action area defines.
- The account hook parses `claude auth status`, and `codex login status` or, if that names no account, the email claim of the id_token in the home's auth.json (see O3).
- The sign-out hook runs `claude auth logout` or `codex logout` against the Agent's home only.
- Ambient ANTHROPIC_API_KEY and OPENAI_API_KEY are still removed.

D3. API key (finding 4-02):
- When AGENT_API_KEY is set, the wrapper gives it only to the client: ANTHROPIC_API_KEY for Claude Code, CODEX_API_KEY for `codex exec` (to be confirmed in a trial). It never stores the key with `--with-api-key`, and raises no sign-in.
- The Agent needs a declared Conduit listing get-secret.v1, opened by root (SEC-008). A scoped Interface cannot open it (ruling 66.3); the failure is explicit.
- SEC-001, SEC-008, SEC-010 and MAN-017 are used unchanged.

D4. Draft descriptions. These are short, because ruling 53's tool descriptions repeat them.
- claude-code.v2: "Claude Code run by the engine as an Agent in a home of its own, signed in by the account named in `account` (checked) or by an API key from a secret Resource. It reaches other objects only through its Conduits. By default its built-in tools, shell, browser, hooks, plugins, account connectors and peer messaging are off; accountServices turns on the signed-in account's claude.ai connectors, and peerMessaging SendMessage, ListAgents and RemoteTrigger, both outside any Conduit. It runs as the engine's OS user: access is mediated, not OS isolation."
- codex.v2: "Codex run by the engine as an Agent in a home of its own, signed in by device flow to the account named in `account` (checked) or by an API key from a secret Resource. It reaches other objects only through its Conduits. Its shell, browser, hooks, plugins and apps are off, its sandbox is read-only (apply_patch writes are refused) and web search is off; accountServices turns on the account's apps, outside any Conduit. Its collaboration tools still start sub-agents that run more model work in parallel, with this Agent's access and no more (finding 4-06). Its home starts empty, so no instruction file of the owner's reaches it (finding 3-07). It runs as the engine's OS user: access is mediated, not OS isolation."
- The full statement of reach goes in the README and CONFORMANCE.md.

D5. Ruling 63.3: each package exports a Conduit template `turn` (capability conversation.v1, operation turn.v1). conversation.v1 and turn.v1 are unchanged. 0.2.0 exports v2 only, and v1 stays at 0.1.1.

D6. Documents:
- Rewrite the READMEs. Remove "the owner's subscription login is shared".
- /Users/shravankumaran/Projects/Conduit/conduit/reference-model/CONFORMANCE.md:
  - the DL16 row gains the state directory;
  - the statement of 3-07 is closed once the marker trial passes;
  - 4-06 is kept.
- /Users/shravankumaran/Projects/Conduit/conduit/docs/guides/accounts.md, updated.
- conformance/fixtures/s1/packages/agents 0.2.0 exports claude-code.v2 and codex.v2 drafts with the real packages' schema and no `signIn`. These are registration-only, so that the short-form S1 equivalence case and the rerun's dry run register the same author text. Coordinate with WO-B.

E. UNCHANGED:
- SUB-010: ruling 52, point 3, keeps it.
- NAM-024's amendment belongs to the short-form area.

## Vectors

NEW. The case directories are proposals.

Refusals (conformance/conventions/refusals/):
1. tier-one-closed-tool:
   - An Agent with no inspect authority calls the tool of a closed Conduit, and then a tool name never presented.
   - Both calls get -32602. The `message` names the tool, the local name and the operation, and gives the ACC-001 cause.
   - `data` holds reason refused and a diagnostic with `ruleId` ACC-001, with no `conduits` and no `exists`.
   - The two calls get identical tier 1, apart from the tool name.
2. tier-one-missing-same: a caller with only a Pool's authority asks to inspect or set up an existing Resource of another Pool, and then a missing name. The diagnostics are equal apart from the echoed reference (`ruleId` ACC-033), and existence is not disclosed. This is ruling 52's "same answer".
3. tier-two-root-submit (finding 4-08):
   - Root registers a Resource into an existing Pool to which it has no Conduit listing register.v1.
   - The result is refused, `ruleId` ACC-033, naming register.v1 and the Pool as written in `owner`.
   - Tier 2 lists root's Conduits to that Pool: none, then a registered Conduit that is not open, with its operations.
4. tier-two-inspector:
   - An Interface holds management with inspect.v1 and not set-up.v1. Its refused set-up of an Agent lists its Conduit to management: open, [inspect.v1].
   - The same request naming a missing Agent is refused with `exists: false`.
5. tier-two-closed-mark: root calls a tool of a Conduit it closed. Tier 2 lists the closed Conduit from its tombstone, with its operations. This needs ruling 53's LIF-038.
6. tier-two-withheld-for-named-object: a caller that may inspect a Pool but not its Resource gets tier 1 only, and the Pool that owns the Resource is not named (reading R2).
7. rule-statements: one step for each refusing rule (SEC-007, SEC-008, ACC-032, ACC-024, ACC-039, ACC-019, ACC-034, IFC-021, IFC-032, LIF-034). Each step's `ruleId` and refusal fields are checked, and its cause is not the ACC-001 sentence.
8. restriction:
   - A bound refusal under ACC-030 and ACC-029, and a filter refusal under ACC-008, carry `restriction` plus the diagnostic.
   - An implementation's IFC-026 refusal carries its restriction too.
9. unavailable-authority-state: under ACC-037, one diagnostic with `ruleId` ACC-037 that names no object.
10. text-block:
    - A refused tool result's first text block begins with "refused" and holds the diagnostic's message.
    - `structuredContent.diagnostics` holds refusal.v1.
11. records: the record of a refused request states the rule and the tier, and holds no tier-2 list (REC-001; if O-choice R9 is kept).
12. conformance/interface/refusals/tasks-closed: a `tasks/get` through a closed Conduit gets -32602 in the same form (IFC-016).
13. conventions/requests/explicit-reasons: one step for each reason. Each carries an error diagnostic with its `ruleId` (IFC-048).
14. conventions/requests/explicit-engine-outcomes: an unknown outcome from a dropped connection (OPS-027) and a failed outcome under IFC-010 each carry a diagnostic naming the cause.

Sign-in (conformance/conventions/sign-in/, using a scripted Agent-Type fixture with `signIn` and both hooks):
15. account-match: on completion, account-hook.v1 is dispatched. With an equal account the answer is signed-in, the held call is dispatched, and records are kept.
16. account-mismatch: sign-out-hook.v1 is dispatched. The answer is refused, with the expected and the reported account in the evidence. The held call is not dispatched; a new sign-in is raised.
17. account-hook-unknown: a hook that fails, is unknown or returns null leads to sign-out and refused. This fails closed.
18. sign-out-unknown: the instance shows `SignInUndone=False`, and a turn reports unavailable naming it until a reconciliation.
19. case-insensitive: "Owner@Example.com" matches "owner@example.com" (reading R8).
20. no-expected-account: a URL elicitation from an instance whose place holds null is not raised. Its call is unavailable, with a diagnostic naming the missing account (O2).
21. evidence: the pending sign-in's evidence carries the expected account from config. Shared with the pending-action area.
22. conformance/language/sign-in/type-sign-in-place, under CTR-013, is rejected in each of these cases:
    - the pointer leads to a place that is not required;
    - the place is not confined to a string or null;
    - `signIn` is stated without the two hooks.

State directory: these are shell tests, not portable vectors, unless WO-H puts it in the run intent.
23. reference-model shell tests:
    - the directory is made with mode 0700;
    - it is kept across runs and across restarts;
    - it is removed when the instance is removed and when it is unregistered;
    - its path holds the instance ID, so it is never reused;
    - the engine does not read or record it;
    - a binding of CONDUIT_STATE_DIR is rejected under MAN-018. This last one is a portable language vector: conformance/language/manifest/state-dir-binding.

EXISTING vectors that change. I verified each by reading its case.yaml. In every one the reason or outcome stays the same; only the assertions change.
- 25 refused steps in 7 cases assert `/reply/diagnostics` absent. Under the change they carry the refusal diagnostic:
  - conformance/language/first-start/protected, steps 2-4. Root registers into `system` and is refused. These steps now carry tier 2, `ruleId` ACC-033, naming register.v1 and Pool `system`. There is still no FST-006 diagnostic, so the title's ordering still holds.
  - conventions/access/s1-pool-authority, steps 1 and 4;
  - conventions/access/bounds, 9 steps, of which 2 already carry `restriction`;
  - conventions/access/current-tools, steps 3 and 4;
  - conventions/access/identity, steps 1 and 2 (IFC-032 and IFC-022);
  - conventions/access/s1-exact-access, 3 steps;
  - conventions/access/s1-plain-filter, 4 steps.
- conventions/records/expected-id, steps 5 and 7: the content text is exactly "refused" and `structuredContent` is absent. They become the tier-1 statement with refusal.v1, and stay identical for a missing and an existing Resource. Step 2 asserts diagnostics absent on a -32602.
- 3 unavailable steps assert no diagnostics: conventions/access/identity (ACC-037) and conventions/access/s1-plain-filter (ACC-014). Under IFC-048 they gain one.
- 58 steps in 37 cases assert that the first text block is exactly the reason or outcome word. 24 of these are reasons: conflict 12, invalid 9, refused 2, unavailable 1. They include requests/key-scope, reason-order, reason-reconsidered, reconcile-false, reconcile-true, reconcile-invalid, reconcile-revision-*, records-inputs-invalid and plain-content. The other 34 are outcomes, failed 7 and unknown 27, in requests/aware-*, read-*, write-*, cancellation, cancel-confirmed, unknown-write, invalid-outputs and similar cases.
  - These change because the text block now holds the explanation.
  - Recommendation: the runner gains a text-prefix assertion, so that vectors check the leading reason or outcome word plus `ruleId`, and not the engine's wording.
  - The 34 outcome steps change only if R6 is kept.
- These keep passing as projections and may be extended:
  - interface/schemas/e05-closed, e05-missing-contract and e05-secret-omitted;
  - conventions/s1/whole (three -32602 refusals);
  - conventions/requests/reason-order (OPS-023's order unchanged).
- These keep passing with no change, since they have no assertion of absence:
  - the other refused steps, in access/entry-authority, fixed-kind-references, routing, root-local-lost and capability-skew;
  - first-start/lifecycle and lifecycle-pool-target;
  - records/broker, broker-filtered, keyed-refused and purge-nonroot;
  - setup/close-in-flight and review-hook-refused*.
- conformance/manifest.yaml gains the new promises.

## Build

All under /Users/shravankumaran/Projects/Conduit/conduit. Sizes are rough estimates.

SPEC (one section of the single spec-batch order, written once):
- New: ACC-040, IFC-048, PRV-029, CTR-013, 1-2 account-check promises, two core Operations, and refusal.v1. About 300 lines.
- Changed: OPS-023, OPS-005, OPS-029, ACC-001, ACC-033, IFC-012, IFC-016, IFC-025, IFC-032, LIF-037, LIF-035, MAN-018, PRV-023 and REC-001. About 150 lines net.
- Also: type-spec.v1, DL16, names.yaml, conduit.yaml exports, the core README tables, about 8 definitions rows, spec/README status, and 5-8 schema examples. tools/set_names.py learns stateDirVariable, about 20 lines.

WORK ORDERS (Codex; the coordinator numbers them; 033 is the next free number):
- WO-H, the state directory. Small, and can start as soon as the spec lands.
  - reference-model/crates/shell/src/runner.rs and store.rs: make the directory, pass CONDUIT_STATE_DIR, remove it on removal or unregistration, and record and retry a failed delete. About 120 lines plus 150 of tests.
  - Core and loader check for MAN-018, about 20 lines; one language vector.
- WO-R, refusals and explicitness.
  - crates/core/src/access.rs: Verdict already carries the ruleId of every refusal (IFC-032, IFC-022, IFC-028, ACC-019, ACC-034, ACC-039, IFC-021, ACC-001, SEC-007, ACC-033, ACC-032, LIF-034, ACC-024, SEC-008, ACC-029, ACC-030). It gains what was named, the tier decision (inspect authority under ACC-033 for the target and the named objects, root always) and the tier-2 list. About 250-300 lines.
  - requests.rs and lib.rs (RequestReply): diagnostics on every reason and on engine-decided outcomes, and the text block. About 120 lines.
  - records.rs: the rule field, about 20 lines.
  - crates/shell/src/server.rs: the -32602 message and data, and the content block. About 80 lines.
  - author.rs and main.rs: the CLI prints tier 2 for root, which closes 4-08. About 60 lines.
  - crates/conformance: refusal and text-prefix assertions. About 100 lines.
  - About 14 new cases (about 45 steps), and edits to about 45 existing cases (about 85 steps).
  - Tier 2's tombstone listing lands after WO-A, ruling 53's tombstones and inventory.
- WO-S, the account check, after the pending-action and URL-elicitation order.
  - Core: the hook dispatch path, reusing setup and retire hooks, the comparison, the answer, the SignInUndone condition, and fail-closed gating of model work. About 300-400 lines.
  - Loader and core: CTR-013, about 80 lines.
  - Shell: dispatch of the two hooks to plain wrappers through the manifest's `tools` map. About 60 lines.
  - About 8 cases, with a scripted Agent-Type fixture.
- WO-D, the Agent packages 0.2.0, after WO-H, WO-S and the elicitation order.
  - packages/agent-claude-code and packages/agent-codex, which share near-identical code:
    - contracts.yaml rewritten, about 120 lines each;
    - each wrapper +250-350 lines (home, login and elicitation, two hooks, retire sign-out, API key);
    - tests +250 each, with scripted clients for every path;
    - READMEs rewritten; dist archives rebuilt and pinned by digest.
  - conformance/fixtures/s1/packages/agents 0.2.0, unless WO-B takes it.
  - CONFORMANCE.md and docs/guides/accounts.md.

REAL-CLIENT TRIALS (run by the coordinator, recorded in the increment log):
- A marker trial for 3-07: markers in the owner's ~/.codex/AGENTS.md, ~/.claude/CLAUDE.md and ~/.claude.json must not reach either Agent. If any does, set HOME to the state directory as well, and run again.
- 4-06: check the six collaboration tools and the sub-agents' reach again.
- One sign-in per client to the right account through the engine. Record what each client's status command reports (see O3).
- The retire hook signs out; on macOS, the Keychain entry is gone.
- An API-key run for each client, if the owner supplies keys.
- One refused call through a filter, to show that the explanation reaches each model in a tool result. Experiment 008 showed this only for -32602.
- The wrong-account refusal with real clients is A5's, and needs the owner's second accounts.

GATE: every check in AGENTS.md, plus `cargo test --release` and clippy in reference-model and in each package.

## Ruling points

IMPLEMENTED, AND WHERE:
- 52.1, tier 1: ACC-040 (new); OPS-023, OPS-005, ACC-001, IFC-012, IFC-016, IFC-025 and IFC-032 (changed); refusal.v1 and diagnostic.v1.
- 52.1, "a target that does not exist gets the same answer": ACC-040, OPS-029, the observable of ACC-033, LIF-037; vectors 1 and 2.
- 52.1, tier 2, with the root Interface always: ACC-040, through ACC-033's inspect rows, the inventory and the tombstones; vectors 3-6. This matches the Fleet brief's "root's refusals are always tier 2".
- 52.2, the explicitness rule: IFC-048 (new); IFC-025, diagnostic.v1, the ACC-037 branch of OPS-023, REC-001 and definitions; vectors 9-14.
- 52.3: no change (SUB-010).
- 49.3, expected account checked, refused and undone: CTR-013, type-spec.v1 `signIn`, account-hook.v1 and sign-out-hook.v1, and the account-check promise; vectors 15-22.
- 49, "What follows", homes made and removed with the Agent, with the engine reading none of them: PRV-029, MAN-018, PRV-023, LIF-035, DL16 and names.yaml in spec. CLAUDE_CONFIG_DIR, CODEX_HOME and the file store in the packages.
- 49, the Codex device flow and Claude's callback, never a pasted code: the packages, through the elicitation area's path.
- 49, settles 3-07: the empty per-Agent home, the true description and the marker trial.
- 53.7 and 55 A5: claude-code.v2 and codex.v2 in packages 0.2.0, with flags false by default.
- 4-02: `apiKey`, a nullable secret reference, folded into .v2, as critic point 14 asked, so that no .v3 follows.
- 4-06: stated in the codex.v2 description, the README and CONFORMANCE.md.
- 63.3: the packages ship a `turn` Conduit template.
- 56.7: the expected account in the sign-in evidence comes from `signIn.account`.
- 66.3: the check is the same whichever Interface completes the sign-in.
- 55 A3 and 54: homes are keyed by instance ID, so a replaced Agent, even under a stable name, gets a new home and a new sign-in.

FOR THE OWNER (decisions):
- O1. Whether `account` is required in .v2 config. This is the faithful reading of 49.3, "the configuration names the account".
  - Cost: one line per Agent. In S1, T1 then stays at experiment 011's 34 lines against the baseline's 33, because the two lines of reach flags that the false defaults save are spent on account lines. Without it, T1 would be 32.
  - Recommendation: required.
  - Alternatives: optional and unchecked when absent, which weakens 49.3; or a file-level default, which needs the derived defaults that are still open.
- O2. Strict reading of "Checked": a sign-in that cannot be checked is not raised. That covers a Type with no `signIn`, or a null account. In v0 this blocks kind-3 sign-ins from plain servers.
- O3. Codex may not report the signed-in account. If `codex login status` names none, the wrapper must read the email claim of the id_token in its own auth.json, an undocumented format. If the trial finds neither works, the Codex check cannot be made as ruled. The owner then chooses between accepting that read, API keys only for Codex, or an unchecked sign-in with a mark.
- O4. The wrong-account refusal with real clients needs the owner's second accounts (A5). Before the rerun, only the scripted vectors prove refusal and undo. Recommendation: A5 does not gate the rerun.

READINGS (the coordinator settles; the owner may strike):
- R1. Tier 2's "closed" Conduits can only come from tombstones, because a closed Conduit is removed at once (LIF-035; definitions, "close").
- R2. Tier 2 needs inspect authority over the authorizing target and over every object the answer names. Otherwise a caller that may inspect a Pool but not its Resource would learn the Resource's owner.
- R3. Each refusing rule states its own true cause. The ACC-001 sentence is used only where it is true; it would be false for SEC-007, for example.
- R4. "An Agent's turns are refused" while a sign-in is pending reads as the reason unavailable (CON-024), with a diagnostic naming the action. Under OPS-005, refused means not permitted.
- R5. The unavailable result of ACC-037 carries a diagnostic that names no object.
- R6. Explicitness covers failed and unknown outcomes the engine decides, which also helps 4-12. Cost: 34 vector steps edited.
- R7. codex.v2 drops `peerMessaging`.
- R8. Accounts are compared ignoring ASCII case.
- R9. Records state the deciding promise's ID.
- R10. The name CONDUIT_STATE_DIR is coined for the owner to strike.

Nothing in this area was found to be impossible to implement as ruled, except possibly O3, which waits on the trial.

## Dependencies

MUST LAND FIRST, OR IN THE SAME SPEC BATCH:
1. The pending-action mechanism and the path for URL elicitations: the changes to IFC-007 and IFC-016, the kind, subject, evidence, expiry and answer, and the lapse rule. These belong to the area of rulings 49, 50, 51 and 56, and WO-S and WO-D are built on them. The point where the wrapper raises its sign-in is decided with that area. My recommendation is the setup hook, so that root signs an Agent in at setup and not inside a turn.
2. Ruling 53's tombstones (LIF-038) and the inventory (LIF-013, the output of inspect.v1, and ACC-033's sentence on disclosure). Tier 2's list of closed Conduits and its disclosure rule depend on them, so WO-R's tier-2 part follows WO-A.
3. ACC-033's rows as rulings 59 and 60 amend them: an Agent is inspected and set up through its Fleet. Tier 2 says "may inspect" through ACC-033, so it follows those rows without being rewritten. The refusing rules that rulings 53, 59 (A3: ACC-024 and the cascades), 62 (C1, C2) and 66.4 add each take a row in ACC-040, so ACC-040 is finalized after them.
4. NAM-024's amendment (ruling 55 A5, the short-form area), which is the reason the .v2 Types exist.
5. Package-exported Conduit templates and `from:` (ruling 63.3, the short-form area), for the .v2 `turn` template.
6. Ruling 53 #5's tool descriptions (IFC-033, OBJ-018), which carry the .v2 descriptions to callers. This is why the descriptions are kept short.
7. Ruling 66.3: pending actions at scoped Interfaces, which affects who answers. ID allocation across batch 1: ACC-040+, IFC-048, PRV-029 and CTR-013, avoiding OPS-035.

ORDER OF THE WORK:
1. The spec batch.
2. WO-H, the state directory.
3. WO-R, refusals and explicitness, after WO-A.
4. The pending-action and elicitation order, from the other area.
5. WO-S, the account check.
6. WO-D, the Agent packages, then the real-client trials.
7. The archives published by digest.
8. The rerun.

WHAT THIS BLOCKS:
- The S1 rerun. It needs:
  - the .v2 packages;
  - a home and a sign-in for each real Agent, which means two person actions at root at T1;
  - refusals that explain themselves to the fresh author.
- The measurement plan: it must state how sign-ins and the account lines are counted (ruling 63).
- A5, which runs on this flow.
- WO-B's S1 equivalence case, through fixture 0.2.0.
- Closing findings 4-02, 4-08 and 3-07, and the statement of 4-06.
- docs/guides/accounts.md.

BATCH 2 (ruling 61, Fleet authority): not foreclosed.
- Act-for refusals become new rows of ACC-040.
- The inspect-based tier follows whatever rows group B adds to ACC-033.
- Completing a sign-in is independent of who answers.
