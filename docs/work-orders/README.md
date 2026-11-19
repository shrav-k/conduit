# Work orders

Not binding. The admin writes each order before dispatch, as
[work orders](../implementation/WORK_ORDERS.md) describes under
[ruling 48](../proposals/ruling-48/README.md). Merged orders 001 to 036k and 060 to 066
have left the tree; their results, decisions and observations are in
[the record](../implementation/WORK_ORDERS.md#the-record).

## Orders still in the directory

One row for each order file in this directory, in file-name order. Orders 048a7, 048a8 and 050o,
which the coordinator's cuts of 048a-6 and 050 plan (order 117), are not yet written; they have
rows without links where their names sort, each saying what it comes after. On 2026-11-19 the
owner joined the last four parts of 048a-5 in pairs, as orders 048a5ef and 048a5gh, and moved
048a7, 048a8 and 051b3, the carried return, off 0.2.0's path and the dogfood path, to after the
dogfood, beside the keyed release (ruling 88).

| Order | What |
| --- | --- |
| [037](037-spec-short-form.md) | The short form in the specification |
| [038a](038a-spec-pending-and-sign-in.md) | Refusals, pending actions and sign-in |
| [038b](038b-spec-enrollment.md) | enrollment in the specification |
| [038c](038c-spec-sign-in-return.md) | the return of a browser login in the specification |
| [039](039-build-local-names-and-expected-id.md) | Local names and expectedId in the build |
| [040](040-build-fleet-and-first-start.md) | Fleet and first start in the build |
| [041a](041a-build-defaults-and-owners.md) | root's defaults, Agents' owners, and the vectors' migration |
| [041b](041b-build-fleet-authority.md) | authority through Fleets |
| [042](042-build-tombstones-and-inventory.md) | tombstones, lift, the inventory, provenance and reconcile limits |
| [044](044-build-package-fixtures.md) | package fixtures for the .v2 Types, and no default added to a published Type |
| [045](045-build-state-directory.md) | each instance's state directory |
| [046a](046a-build-refusal-tiers-and-diagnostics.md) | one deciding rule, and a diagnostic on every reason |
| [046b](046b-build-tier-two-and-closed-conduits.md) | tier 2 for a caller that may inspect the target, and closed Conduits from tombstones; after 042, 046a and 046c (order 117's split, with 046a's move of tier 2 for callers other than root, the coordinator's, open to the owner's strike) |
| [046c](046c-build-outcome-diagnostics.md) | the outcomes the engine decides name their promise |
| [047](047-build-setup-action.md) | a failed setup's action, computed and raised at root's inspection |
| [047b](047b-build-actions-raised-by-scope.md) | where pending actions are raised: scoped Interfaces, disclosure, list.v1's pending output and the unanswered key; after 047 (the coordinator's split, open to the owner's strike) |
| [047c](047c-build-stored-answers.md) | `answer.v1` with stored answers: dismissed, retried and retired, conflict, and answered state that survives a purge; after 047b (the coordinator's split, open to the owner's strike) |
| [047d](047d-build-retirement-notices.md) | the notice kind (PND-007): one per Fleet whose Conduit a Resource's retirement closes, stored with an engine ID, named in the retirement's `pendingAction` key, raised at that Fleet's scope and answered `acknowledged`; after 047c (the coordinator's split, open to the owner's strike) |
| [048a1](048a1-build-sign-in-type-checks.md) | CTR-013's checks on a Type's sign-in, and the .v2 fixture's apiKey |
| [048a2](048a2-build-sign-in-lapse-and-conflict.md) | a lapsed login raises the instance's sign-in action (F84's two halves, IFC-016's detection), and a second sign-in of one instance reports conflict (PND-010); after 047, 047b, 047c and 048a-1, the first part of 048a-2 (the coordinator's split, open to the owner's strike) |
| [048a3](048a3-build-sign-in-hold.md) | the hold of an instance awaiting sign-in (PND-005's `then`): held requests unavailable with `pendingAction` and a PND-005 diagnostic, an expiry lifting nothing, and a new action on the next held request; after 048a-2 (the coordinator's split, open to the owner's strike) |
| [048a4](048a4-build-sign-in-setup-account.md) | setup's account check in TPL-012's order (PND-011's setup half): no account or another raises the sign-in action, another is undone by a sign-out part, and a failed undo reports SignInUndone=False and holds the instance under PND-011, beside PND-005's hold, until an account check clears it; after 048a-3 (the coordinator's split, open to the owner's strike) |
| [048a5](048a5-build-sign-in-refusals-before-parts.md) | `sign-in.v1`'s refusals before any part, after 048a-2's invalid and before its conflict: a request without URL elicitation gets -32021 under IFC-049, a return on a first request reports invalid under PND-013, and an unknown `signInLogin` invalid under PND-010, with the shell forwarding `clientCapabilities` and `signInLogin`; after 048a-4, the first of 048a-5's seven parts (the coordinator's split, open to the owner's strike) |
| [048a5b](048a5b-build-sign-in-account-first.md) | `sign-in.v1` dispatched, its account check first: the expected account, compared ignoring ASCII case, succeeds with output `account`, answers the sign-in action signed-in, lifts PND-005's hold and clears `SignInUndone`; a failed or unknown check completes nothing; no account or another ends failed for now, naming PND-010 and order 048a-5e, another raising the action unless one is unanswered; C11's stub removed; after 048a-5, the second of 048a-5's seven parts (the coordinator's split, open to the owner's strike) |
| [048a5c](048a5c-build-sign-in-fixed-port.md) | PND-010's fixed-port refusal: a `sign-in.v1` that may start a login on a fixed port that another instance's sign-in in progress may start reports unavailable, naming the shared ports and only a visible pending action, after 048a-2's conflict, with the asked-for login kept on the invocation; after 048a-5b (the coordinator's split, open to the owner's strike) |
| [048a5d](048a5d-build-sign-in-hook-part.md) | the `sign-in-hook.v1` part, after an account check that finds no account or another or that ends failed or unknown (ruling 86): the asked-for login's name passed to a keyed hook (IFC-006), the hook's `input_required` answer taken by `sign-in.v1`, plain or keyed alike, and never read as IFC-016's lapse trigger (IFC-010), an answer that is not one URL elicitation ending it failed under IFC-049, and a URL elicitation meeting the interim ending of orders 048a-5b and 209, moved there, with the implementation's requestState kept with the part; after 048a-5c, 048b-relay2, 207 and 209, the fourth of 048a-5's parts (the coordinator's split, open to the owner's strike) |
| [048a5d2](048a5d2-build-sign-in-url-match.md) | IFC-049's URL match: a `sign-in-hook.v1` URL matched to exactly one stated login and the asked-for one, a plain hook's included (ruling 86, point 4), a failing URL ending `sign-in.v1` failed naming IFC-049, the check, the field and, where the check concerns one, the login, holding no URL value; after 048a-5d, the fifth of 048a-5's seven parts (the coordinator's cut of 048a-5d, recorded with its refresh, open to the owner's strike) |
| [048a5ef](048a5ef-build-sign-in-relay-and-polling.md) | `sign-in.v1` relays its URL action and polls: the engine's message and the input-required output that 048b-relay1 relays, with `signInLogin`'s members in it and in the final result, `acceptsReturn` from the hook call's own `plain`, and DL18's expiry, the hook part kept in progress, a plain hook's included; and each accepted retry polling the account, signed in on the expected one, asked again while there is none, the first check's account or a poll that failed or ended unknown, failed on a decline or cancel; after 048a-5d2, the sixth of 048a-5's seven parts, joining 048a-5e and 048a-5f under ruling 88, which waives FR-264's one behavior per order for it (the coordinator's split, open to the owner's strike) |
| [048a5gh](048a5gh-build-sign-in-undo-and-expiry.md) | another account found while polling undone by a sign-out part, `sign-in.v1` failed naming both, the attempt kept as the unanswered action's evidence, and SignInUndone=False where the undo fails or a cancel or restart cuts it off; and DL18's end: a sign-in with no retry ends unknown and no hook runs later, on the input the shell submits at each relayed output's expiry; after 048a-5ef, the last of 048a-5's seven parts, joining 048a-5g and 048a-5h under ruling 88, which waives FR-264's one behavior per order for it (the coordinator's split, open to the owner's strike) |
| [048a6](048a6-build-sign-in-return-not-taken.md) | a sign-in return where none is taken: `signInReturn` on a request whose operation is not `sign-in.v1` reports invalid under PND-013 and is kept nowhere (PND-014), with the shell forwarding it, lifting the expected failure `requests-sign-in-return-kept-nowhere`; after 048a-5 alone, the first of the returns' three parts (the coordinator's split, open to the owner's strike) |
| 048a7 | an accepted return passed on: on a retry, PND-013's first three checks, a page login's code passed to `sign-in-hook.v1` in its retry, and `signInLogin`'s `returned`; after 048a-5gh and 048a-6, deferred after the dogfood, beside the keyed release (ruling 88) (planned; the coordinator's split, open to the owner's strike) |
| 048a8 | a redirect return: PND-013's fourth check, the value rebuilt and passed to `sign-in-hook.v1` in 048a-7's retry, and `error` in `signInLogin`; after 048a-7, deferred after the dogfood, beside the keyed release (ruling 88) (planned; the coordinator's split, open to the owner's strike) |
| [048b-join](048b-join-build-sign-in-through-the-interface.md) | the join of the engine's sign-in with the shell's relay, shown end to end over MCP: a `sign-in.v1` started at the engine reaches the person through the Interface as an input-required result, and the Interface's accepted retries poll the account until the expected account signs the instance in, through a keyed hook and a plain one, with a declined retry ending failed; the URL relay and polling only, its return leg moving after the dogfood with 048a-7 and 048a-8 (ruling 88); after 048a-5gh and 048b-relay2, the last parts on the dogfood path of the cuts of 048a-2's entry and 048b-relay (the coordinator's split, open to the owner's strike) |
| [048b-relay1](048b-relay1-build-input-required-relay.md) | the server relays the core's input request to the Interface that asked, as an `InputRequiredResult` with a requestState the shell issues and holds, and checks each retry that echoes one (IFC-049); the core's input-required output and the retry's input declared; the first part of 048b-relay, after nothing (the coordinator's split, open to the owner's strike) |
| [048b-relay2](048b-relay2-build-url-elicitation-and-part-retry.md) | calls for requests and hooks to an instance whose Type states signIn declare URL elicitation (IFC-045), written by the core in `intent`, shown by `requests-sign-in-lapse-and-conflict` gaining IFC-045; and a part's own `InputRequiredResult` reaches the core as a result and is sent again as the core writes it (`retry_intent`), with that implementation's requestState and the responses, `runner.rs` unchanged and pinned by a test; after 048a-2 merged and 048b-relay1, and before 048a-5d (the coordinator's split, open to the owner's strike) |
| [049a1](049a1-build-enrollment-core.md) | enrollment in the core |
| [050a](050a-agents-v2-own-tools.md) | the .v2 Agent Types, with the Agent's own tools |
| [050b](050b-spec-claude-sign-in-connection.md) | the connection Conduit manages for a Claude sign-in, F95, the three stale route texts, and Codex's browser-opens limit, in the specification; after ruling 82 and order 199 (the coordinator's split, open to the owner's strike) |
| [050c](050c-agents-v2-client-home.md) | each .v2 Agent's own client home inside its state directory; after the stack's rebase onto main and order 045 (the coordinator's split, open to the owner's strike) |
| [050d](050d-agents-v2-process-tree.md) | the .v2 wrappers stop their client's whole process tree (the full review's order 16); after 050c (the coordinator's split, open to the owner's strike) |
| [050e](050e-agents-v2-failed-turn-fields.md) | failed turns as fields, with a network class, and a lapsed login in a turn (the full review's order 17); after 050d, and 048a-2 written (the coordinator's split, open to the owner's strike) |
| [050f](050f-agents-v2-inputs.md) | the .v2 inputs: account, toolsDirectory, and the description sentences (i) and (ii), with no apiKey, and the wrappers' `.env` refusal reworded without its apiKey reason (ruling 83); after 050e, and 048a-1 and order 202's work merged (the coordinator's split, open to the owner's strike) |
| [050g](050g-logout-revocation-read.md) | desk reads: what each client's logout revokes, and the scope of each; after nothing (the coordinator's split, open to the owner's strike) |
| [050h](050h-agents-v2-retire-hook.md) | the retire hook: each .v2 Agent signs its client out of its own home and empties the home at retirement (D53), with Q1 (iv)'s description sentence; after 050f, and 050g merged; its tip is where the two lanes split (the coordinator's split, open to the owner's strike) |
| [050i](050i-claude-account-sign-out-hooks.md) | claude-code.v2's account and sign-out hooks: the `.v2` item serves `account`, the account that `claude auth status` names in the Agent's own home, and `sign-out`, the client's own logout there with the home kept, neither yet required by the Type; the first order of the Claude lane, after 050h, and 048a-4 written (the coordinator's split, open to the owner's strike) |
| [050j](050j-claude-sign-in-redirect-login.md) | claude-code.v2 states signIn: its account place `/account`, its redirect login `browser` with `carried: false` and literals read from Claude Code 2.1.318, and the four hooks required, each served by the `.v2` item; the `sign-in` hook runs the client's own login with the wrapper as its browser and relays the URL handed over, and nothing the client prints; with order 050i's last two shoulds; the second order of the Claude lane, after 050i, and 048a-1 merged (the coordinator's split, open to the owner's strike) |
| [050k](050k-claude-code-page-login.md) | claude-code.v2 states its code-page login, `code-page`, `return: page` with `carried: false` and its page read from Claude Code 2.1.318, beside the browser login and not choosable in 0.2.0 (ruling 86, point 1), and its description lists it (Q1 (vi)); the sign-in hook's fixed failure texts name PND-010, from 050j's second verifier; the third and last order of the Claude lane, after 050j (the coordinator's split, open to the owner's strike) |
| [050l](050l-codex-account-hook.md) | codex.v2's account hook asks Codex itself through account/read, with refreshToken false and a deadline, never opening the login file and failing closed, and its sign-out hook; the first order of the Codex lane, after 050h, the coordinator's reading that order 197 confirms the route, and 048a-4 written (the coordinator's split, open to the owner's strike) |
| [050m](050m-codex-sign-in-hook.md) | codex.v2 states signIn on the Codex lane: its browser login, its device login stated and refused with a diagnostic, and its sign-in hook, plain in 0.2.0 (ruling 86, point 1), with 050l's two last shoulds and the account check's refresh blocked by an override under `.invalid` on the check's child alone (ruling 86, point 5); 050n joins it only if the live check passes; the second order of the Codex lane, after 050l, 048a-1 merged, and order 208's probe reporting no write (the coordinator's split, open to the owner's strike) |
| [050n](050n-agent-packages-joined.md) | the Agent packages 0.2.0, joined and merged, the last 050 order: the Claude lane joined with the Codex lane on the live check's pass; the engine's pins and the S1 fixture's `.v2` Types moved to 0.2.0, so that the live S1 engine test, which fails on the lanes by design, passes again; loader vectors for both `.v2` Types, a template binding `/account`, TPL-005 for `account-hook.v1` and a failed turn's fields under IFC-025; every failure of a sign-in hook to start a login naming PND-010; Codex's login limit within DL18's wait; the accounts guide and `engine/CONFORMANCE.md` at 0.2.0; written ahead, built on 048b-join's work, merged or stacked on its round-1 branch, and merged after 051b-2 and the live check's results with a pass (the coordinator's split, open to the owner's strike) |
| 050o | the managed connection for a Claude sign-in, built in the Interface; after 051b-3 merged, and so after the dogfood (ruling 88), and 050b merged (planned; the coordinator's split, open to the owner's strike) |
| [051b](051b-cli-pending-and-answer.md) | `conduit pending` and `conduit answer`: root's pending actions listed and answered from the command line, with the unanswered hint; after 047c alone, the first of 051b's three parts (the coordinator's split, open to the owner's strike) |
| [051b2](051b2-cli-sign-in.md) | `conduit sign-in`: `sign-in.v1` sent through root's Conduit to the owner declaring URL elicitation, with `--login` for one stated login; the engine's message and the full URL shown, the browser opened only on the person's consent, and each retry (accept on Enter, decline, cancel) carried until signed in, failed or the wait's end (DL18); the client of A5's live check and of a dogfood slot's sign-in on the Mac; after 048b-join and 051b, the last sign-in order on the dogfood path, before A5's live check and 050n (the coordinator's split, open to the owner's strike) |
| [051b3](051b3-cli-pasted-return.md) | the carried return from the command line (D53, ruling 75 C4's case): `conduit sign-in` asks for the paste only where `signInLogin`'s `acceptsReturn` is true and standard input is a terminal, reads it with echo off and sends it as `signInReturn` `{value, via: paste}`, asks again only after a refused return, and opens no loopback listener, since it cannot tell which machine a node is (PND-013); `conduit call` refuses arguments that name `signInReturn` and sends nothing; the targeted review's section 6 sentence (the paste is the `carried: true` route and serves no Claude Agent's sign-in) in `engine/README.md`; after 051b-2 and 048a-8, deferred after the dogfood, beside the keyed release (ruling 88), the last of 051b's three parts (the coordinator's split, open to the owner's strike) |
| [067](067-condense-work-orders.md) | Move merged orders into their record |
| [068](068-test-gate-and-ci.md) | the test gate, CI split across runners, and a check for private paths |
| [069](069-spec-cli-plan-apply.md) | the CLI reference for plan and apply |
| [069b](069b-spec-plan-apply-ruling-76.md) | the plan-and-apply reference under ruling 76 |
| [070](070-build-state-copies.md) | stop copying the whole state on every commit |
| [071](071-conformance-manifest-pieces.md) | the coverage index in pieces, the runner's seams, and parallel vectors |
| [072](072-shell-list-operation.md) | list.v1 and the stubbed operations through the shell |
| [073](073-paper-count-flow-style.md) | a paper count of S1 in flow style, after the bar's rule was fixed |
| [074](074-shell-session-idle-limit.md) | an MCP session outlives the calls it carries |
| [075](075-spec-own-tools-reach.md) | an Agent's own tools in the specification |
| [076](076-agents-v2-folder-grant.md) | in the `.v2` Agent Types, the folder is the grant |
| [077](077-spec-reconcile-fenced.md) | failed is reconciled only once fenced, and the safety catalogue's findings |
| [078](078-agents-codex-path-item.md) | the Codex Agent item that finds its wrapper by name |
| [079](079-shell-https-address.md) | the engine's https address, on a second loopback listener |
| [080](080-shell-runtime-provider-calls.md) | the shell calls runtime providers and reaches their runs over TLS |
| [081](081-spec-one-run-per-instance.md) | one run per instance ID, a replacement's overlap, two readings settled |
| [082](082-spec-records-and-installs.md) | a record lost outside the engine, and installs of one address and version one at a time |
| [100](100-safety-catalogue.md) | the safety catalogue |
| [101](101-pre-a-s2-s3.md) | the pre-registrations of S2 and S3 |
| [102](102-pre-b-s4-s5-s6-s9.md) | the pre-registrations of S4, S5, S6 and S9 |
| [103](103-pre-c-s7-s8-s10.md) | the pre-registrations of S7, S8 and S10 |
| [104](104-mc1-model-checking-harness.md) | the model-checking harness |
| [105](105-mc2-operation-identity.md) | operation identity, unknown outcomes and revisions, model-checked |
| [107](107-mc3-access-over-time.md) | access over time, model-checked |
| [108](108-mc4-placement-and-lost-machine.md) | placement and a lost machine, model-checked across two machines |
| [109](109-mc5a-gates-and-identity.md) | gates and identity, model-checked |
| [110](110-mc5b-secrets.md) | reading and confining secrets, model-checked |
| [111](111-mc6-lifecycle.md) | setup, retirement, removal and Pools, model-checked |
| [112](112-mc7a-controls-and-outcomes.md) | controls, revisions and outcomes, model-checked |
| [113](113-mc7b-records-and-packages.md) | records, observation and installing packages, model-checked |
| [114](114-mc6-finish.md) | close the four checks MC6's final verifier left open |
| [115](115-mc7b-finish.md) | close the two checks MC7b's final verifier left open |
| [116](116-mc8-g5-account.md) | G5's account of the safety promises |
| [117](117-plan-s2-s3.md) | the order-level plans of S2 and S3 |
| [118](118-plan-s4-s5-s6-s9.md) | the order-level plans of S4, S5, S6 and S9 |
| [119](119-plan-s7-s8-s10.md) | the order-level plans of S7, S8 and S10 |
| [120](120-plan-providers.md) | the order-level plan of the providers, A63 |
| [121](121-plans-follow-up.md) | the split reviews and the admin's answers, applied to the plans |
| [122](122-a63-rk1-runtime-kit-server.md) | the runtime-provider kit, its MCP server and backend interfaces |
| [123](123-a63-h1-provider-contract-harness.md) | the harness for provider-contract vectors |
| [124](124-a63-rk1-finish.md) | close what RK1's final verifier left open |
| [125](125-a63-h1-finish.md) | close what H1's final verifier left open |
| [126](126-a63-rk2-runtime-kit-gate.md) | the runtime-provider kit's gate |
| [127](127-a63-rp1-local-process.md) | the local-process runtime provider |
| [128](128-a63-mp1-local-machine.md) | the local machine provider |
| [129](129-a63-rk-owed-stop-first.md) | no run starts on a machine that owes a stop |
| [130](130-a63-rk-owed-stop-first-finish.md) | close the window after admission |
| [131](131-a63-ps-local-directory.md) | the local-directory package source |
| [132](132-a63-fs-file-store.md) | the file secret store |
| [133](133-s2-f-linear-restrictions.md) | S2's restriction contracts, as a contracts-only package |
| [134](134-plans-ruling-77.md) | ruling 77 in the plans, and S3's fault method |
| [135](135-s2-linear-stub.md) | a local stub of the Linear API, for S2 |
| [136](136-s2-l1-linear-aware.md) | S2's Conduit-aware Linear implementation |
| [137](137-ssh-r-runtime.md) | the SSH runtime provider, built and unit-tested |
| [138](138-a63-rk-backend-reasons.md) | a runtime's own reason for a failed staging or start |
| [139](139-s2-l2-linear-adapter.md) | S2's small Linear adapter |
| [140](140-ssh-r-finish.md) | the launch fence, MAN-017, the kit's reasons and the setup |
| [141](141-a63-rk-own-owed-stop.md) | a start after its own owed stop, under order 081 |
| [142](142-scr1-scripted-client.md) | the core scripted MCP Agent client |
| [143](143-s4-srv1-effect.md) | S4's partial-effect and deduplicating effect servers |
| [144](144-s4-srv2-hook.md) | S4's setup hook and its ledger reader |
| [145](145-ssh-r-durable-dir.md) | a durable directory, outside the engine's lifecycle |
| [146](146-ssh-r-drainer.md) | the stop-drainer that outlives the provider |
| [147](147-ssh-r-key-sweep.md) | a sweep of retired providers' keys |
| [148](148-scr1-finish.md) | case-folded names, invalid-script reports and redaction |
| [149](149-ssh-r-drainer-finish.md) | the launch generation set by the provider |
| [150](150-ssh-r-one-drainer.md) | one drainer, reading the newest configuration |
| [151](151-private-names-and-secret-scan.md) | Private names and secrets checked before anything private is committed (the full review's order 2) |
| [152](152-ci-engine-shards-stand-alone.md) | The engine's CI stands alone, and CI builds every crate (the full review's order 3) |
| [153](153-model-account-checked.md) | the model-checking account, checked by a tool |
| [154](154-plan-apply-reference-corrected.md) | the plan-and-apply reference corrected against spec/ |
| [155](155-increment-4-log.md) | increment 4's log: the dogfood frictions and the Records' findings |
| [156](156-runner-stopped-after-end.md) | the runner reports a run stopped only once it has seen it end |
| [157](157-own-tools-reported.md) | an Agent's own tools reported as its Type declares them (the full review's order 15) |
| [158](158-fresh-author-wording.md) | wording the fresh author reads at the pin: flags recorded with the plan, and CTR-013's citation corrected |
| [159](159-a5-stage-0-client-facts.md) | the two client facts order 050 needs (A5's stage 0) |
| [160](160-mc2-rerun-077.md) | MC2 rerun on order 077's text, as ruling 78 leaves it (the full review's order 9) |
| [161](161-a78-stage-1-evidence.md) | A78's stage 1: its evidence and the engine's records of order 042 enter the tree (the full review's order 10) |
| [162](162-dispatch-deadline-cancels.md) | a dispatch deadline cancels its call before the call is reported missing (the full review's order 14) |
| [163](163-a24-at-entry.md) | A24 at entry: a new promise lands with a case that can fail (the full review's order 18) |
| [164](164-planner-difference-table.md) | the planner's difference table, and package contents by digest (the full review's order 23) |
| [165](165-by-rule-examples-public-routes.md) | the by-rule examples sent through the engine's public routes (the full review's order 25) |
| [166](166-conformance-pieces-messages.md) | the conformance pieces' last messages (follow-up to 071; after 163) |
| [167](167-a5-probe-signals-tested.md) | A5's probe: signals and the shim tested, and a signal never cuts a logout (follow-up to 159) |
| [168](168-tidy-after-153-155.md) | the points orders 153 to 155 left: G5.md, the tracker, and one difference recorded |
| [169](169-model-account-reads-index-pieces.md) | the model-checking account's tool reads the coverage index's pieces |
| [170](170-a78-pre-registration.md) | A78's pre-registration: the claim, the pass conditions and the measures for orders 2 and 3 (the full review's order 12) |
| [173](173-method-as-run.md) | the work-order method as run (the full review's order 27) |
| [174](174-vector-changes-declared.md) | vector changes checked against the order's declaration (the full review's order 29) |
| [175](175-promise-totals-checked.md) | the promise totals checked (the full review's order 32) |
| [176](176-core-purity-enforced.md) | the core's purity enforced (the full review's order 33) |
| [177](177-ctr-014-diagnostics-ctr-006-at-resolution.md) | CTR-014's diagnostics name what reaches the place, and CTR-006 checked at resolution (follow-up to 157 and 176) |
| [180](180-roadmap-checker-results-linked.md) | the roadmap checker: a finished row links its results (the full review's order 40) |
| [181](181-uncatalogued-promises-classified.md) | the uncatalogued promises classified (the full review's order 41) |
| [182](182-stale-state-lines.md) | stale state lines in the handover, tracker, roadmap and engine pages (the full review's order 36) |
| [183](183-v0-security-claim-drafted.md) | v0's security claim, drafted (the full review's order 38) |
| [184](184-second-machine-docs-ruling-77.md) | the second machine's documents agree with ruling 77 (the full review's order 42) |
| [185](185-promise-standing-generated.md) | each promise's standing, generated (the full review's order 43) |
| [186](186-scenario-plans-to-tree.md) | the scenario plans brought to the tree (the full review's order 35) |
| [187](187-ci-macos-leg.md) | a macOS leg in CI (the full review's order 30) |
| [188](188-differential-vectors-nightly.md) | the differential vectors, nightly (the full review's order 31) |
| [189](189-package-tests-repeated-linux.md) | changed package tests run five more times on Linux (the full review's order 44) |
| [190](190-desk-checks-terms-1password.md) | two desk checks: each vendor's terms for two subscriptions; 1Password's unattended reads (the full review's order 45) |
| [191](191-log-marks-closed-settings-written.md) | the '(log)' marks closed, and the repository's settings written down (the full review's order 37) |
| [192](192-register-rows-mc-mutation-native.md) | register rows: model checking, mutation testing, native probes (the full review's order 39) |
| [201](201-mc9a-pending-actions-sign-in-registered.md) | MC9a's pre-registration, with `conflict` for a second `sign-in.v1` of one instance; merges before 048a-2 is dispatched (the coordinator's decision, open to the owner's strike) |
| [202](202-agents-v2-fixture-subscription-only.md) | the 0.2.0 fixture's `.v2` Types subscription-only under ruling 83: no `apiKey` and no `oneOf`, `account` required, a string or null, and the case `agents-v2-defaults` with them; after ruling 83's record merged, and merges before 050f is dispatched (ruling 83's What follows; the coordinator's order, open to the owner's strike) |
| [203](203-new-conversation.md) | a new conversation without restarting the engine: `new-conversation.v1` in each Agent package, released as 0.2.1 (the full review's order 62, numbered 203 since 062 is a merged order's number); after 050n merged, or from its verified branch by a dispatch note (the coordinator's placement, open to the owner's strike) |
| [204](204-mc9a-model-run.md) | MC9a's model built and run on the Linux node as order 201 registered it, with its dated amendment of 2026-11-19 (ruling 84's note in row A22, which MC9a does not read), and its results recorded beside the registration (the full review's order 24; the coordinator's placement and bounds, open to the owner's strike) |
| [204b](204b-mc9a-state-space-measured.md) | MC9a's state space measured on the Linux node before its registration is revised: order 204's model, with its readers' fixes R204-4 to R204-7, and its Record carried from its unmerged branch, the model's checking tests ignored until the revision, and a census that reads no property counting each sign-in row's states, the choices that multiply them, the ledger's order and the harness's speed at that size, recorded beside the registration; the revision and the run proper follow (the coordinator's, open to the owner's strike) |
| [205](205-gateway-probe.md) | the gateway probe pre-registered: experiment 032 and its register row A85, with A84's row as the full review assigned it; the first of three parts, the harness in Rust (205b) and the run on the Linux node (205c) following, each written after the one before it merges (the coordinator's split, open to the owner's strike) |
| [205b](205b-gateway-probe-harness.md) | the gateway probe's harness: a Rust crate in experiment 032 using the standard library alone, with its tests and `HARNESS.md`, its self-test, its check of the namespace for the installs and a rehearsal with a stand-in client run on the Linux node, and no client installed or run; after order 205 and the README's amendment of 2026-11-19 (a PID namespace with its own `/proc`, Codex's auth-command validation quoted, the installs and `--version` runs kept from the real home), the second of order 205's three parts (the coordinator's split, open to the owner's strike) |
| [206](206-sign-in-login-path-and-page.md) | a redirect login's `path` refuses `?` and `#`, and a page or service login's `page` refuses user information, in `sign-in-login.v1` and CTR-013, with three examples and three steps of `contracts-sign-in-logins` (the two schema points order 048a-1's Record left open); CTR-013's comparisons across Types follow in three later orders it names (the coordinator's, open to the owner's strike) |
| [207](207-keyed-hooks-and-sign-in-spec-fixes.md) | ruling 86 in the specification: D, keys per operation, as the route for keying, an item marked `plain` listing under `keyed` the hooks whose calls carry Conduit's keys while its other calls stay plain (narrowed by the coordinator to the five core hooks, on items reached by `command` or `package`); IFC-010 relays a URL request from the `sign-in-hook.v1` part of `sign-in.v1`; PND-010 refuses before any part a `signInLogin` aimed at a plain hook; and a failed or unknown account check goes on to `sign-in-hook.v1` as no account, counts as no account yet while polling, and, by the coordinator's reading, lets a setup go on; four cases, three of them expected failures until order 209 builds the engine half; after ruling 86's record merged (the coordinator's text, open to the owner's strike) |
| [208](208-codex-account-read-refresh-blocked-probe.md) | the refresh-blocked probe, pre-registered in experiment 027 and run on the Linux node: whether Codex 0.167.1's `account/read`, asked with `refreshToken` false and with `CODEX_REFRESH_TOKEN_URL_OVERRIDE` under `.invalid` on its process alone, ever writes the home's auth.json, under proactive refresh and under recovery after a 401, with a fabricated login and plain-http stubs on loopback inside a network namespace; any write goes to the owner; before order 050m relies on ruling 86's route (the coordinator's reading of ruling 86, and its placement, open to the owner's strike) |
| [209](209-build-keyed-hooks-and-sign-in-fixes.md) | ruling 86 in the engine: D's engine half, so that a later keyed sign-in release is package-only: `keyed` checked under PKG-020, `plain` read per call on the hook path, a hook that a plain item lists under `keyed` sent with Conduit's keys once `server/discover` lists the extension and left undispatched, unavailable under IFC-037, where it does not; PND-010's refusal before any part of a `signInLogin` aimed at a plain hook; and a failed or unknown account check counted as no account, in `sign-in.v1` and, by the coordinator's reading, in setup, going on to its setup hook; a keyed part's URL-mode `input_required` result left to IFC-010, not read as a terminal outcome; three cases, one step of `requests-sign-in-account-first` changed, and order 207's three expected failures removed; off the chain, refreshed after order 207 merged (the coordinator's text, open to the owner's strike) |
| [210](210-a78-stage-2-fleet-documents.md) | A78's stage 2: the `conduit-core` Fleet's documents for the store initialized again after 0.2.0, two pairs of a Codex worker and a Codex verifier and one Claude reader on Claude Opus, each naming a directory and the owner's primary account of its client, written before order 050n merges and registering nothing, in `experiments/028-a78-dogfood/stage-2/fleet/` with a section of 028's README; beside the sign-in chain (the coordinator's, open to the owner's strike) |
| [211](211-a78-stage-2-registration.md) | A78's stage 2 registered in experiment 028 before its first turn: on the store initialized again after order 050n, five Agents of order 210's documents that the owner signs in once, each with a home of its own, run Conduit's work orders as turns, one at a time until an amendment lets turns run at once, with a Claude reader on Claude Opus through Conduit; stage 1's seven conditions per order and two more (no sign-in asked again, every read fresh), its measures, five stop rules and what it cannot show, and A78's roadmap row; the first dogfood milestone's run, the Claude reader joining once D58's guard is in the harness (ruling 89); after order 210, the ruling-84 correction of 028 and the coordinator's reconciliation of IN_FORCE.md's topic 15 (the coordinator's, open to the owner's strike) |
| [212](212-docs-entry-at-milestone-1.md) | the entry documents at milestone 1, step A of the owner's review of the README and the docs: a README for visitors first, of about 110 lines with a quick start, a docs map routed by reader, one dated state in the tracker that every entry document links, the accounts guide rewritten from the sign-in orders, the Overview's named wrong lines fixed with its full re-read left for later, AGENTS.md trimmed near 150 lines with its check commands and worker rules unchanged, CONTRIBUTING.md the one home of what each check needs, a minimal edit of spec/README.md's non-normative lines, the work-order index repaired and split into live and merged orders, the archive rows owed, the archive of merged orders' index created empty, and the rule that one row names each order file; no file moves (step B is a later order); after 048b-join, 050n and 051b-2 merge with their Records and the live check's results merge and a turn has run through Conduit on 0.2.0, and after ruling 90's record (the owner's choices of 2026-11-19, and the coordinator's, open to the owner's strike) |

## Shared and staged files

- [The shared report schema](004-i2-01-vectors-and-first-start.schema.json), used by every
  remaining order; its original name is kept.
- [The standing verifier order](verify.md) and [its report schema](verify.schema.json).
- [036b-staged/](036b-staged/README.md): exact patches and handoffs for later orders, including
  their vector impact. This directory stays until its consumers apply the patches.

An order that changes a case or a case's fixture writes its declaration here as
`NNN-declared.yaml`, and its pull request commits it: CI checks every vector change against
the declarations that a pull request adds or changes
([reviewing vector changes](../../conformance/README.md#reviewing-vector-changes)).
Declarations stay with the orders that use them.
An order keeps its Record at the foot of its file after it merges, until a condensation
(A80) carries it into [the record](../implementation/WORK_ORDERS.md#the-record): the admin then
writes its row there serially, and removes the order and its own schema.
