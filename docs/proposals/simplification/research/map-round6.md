Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Simplifications already proposed for Conduit (round 6, ruling 44, ruling 41): what each proposed, what happened to it, and whether it touches S1's measured costs

## 1. Bottom line

- **Round 6 put forward about a dozen candidate simplifications.** Codex reader 1 wrote design-critique.md on 2026-11-07. Ruling 44 (the owner, 2026-11-07: "I accept all your recommendations…") accepted the coordinator's disposition of all of them. An independent check then led the coordinator to revise D06 and D12, and the owner accepted both revisions ("Agree on both", ruling-44/README.md:83-108).
  - **Applied:** D06 (revised), D07, D11, D12 (revised, as LIF-037), D14, D18 (restated only, no behaviour change) and D21.
  - **No change:** D13 and D17.
  - **Moved to the experiment programme:** D01, D02, D05, D08, D09, D10, D15, D16, D19 and D20.
  - **D03 and D04:** option a, meaning the limits are documented and nothing is redesigned.
  - **D22:** became the first build (A68).
- **None of the applied round-6 simplifications touches the five cost drivers S1 measured:** document frames, a template plus an instance for everything, change only through new documents, root's Conduits to its own Pool, and tool names that follow Conduit names.
  - I checked this directly. No author file in either run (`docs/proposals/first-build/increment-4/coordinator-run/project*`, `fresh-author-run/t*`) contains `implementation`, `controls`, `expectedId`, `inputSchema` or a local `#name` reference. So D06, D07, D12, D14 and D11's local-contract case save 0 of S1's lines.
  - D18 changed ACC-033's form, not its answers. Its `then` is 418 words now, against 406 before (spec/conventions/access.yaml:652).
  - The promise count went from 452 at round 6, to 454 after D06 and D12, to 455 now.
- **FIRST_BUILD.md:32 and A68 (AUDIT.md:818) expect this to be recorded.** They ask that the measurements be reported "beside what they show for the simplifications that round 6 proposed". The answer is that they show approximately nothing for S1's authoring cost.
- **The earlier big cut was ruling 41**, which applied the principles review of 2026-11-06. It removed 67 promises and added 59, taking the count from 434 to 426 (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md, "Counts").
  - It removed graphs, runtimes in templates, policies, composite Conduits, requirement composition, `obliges`, re-exports, rename, the retrieval tool, the default location and location facts, and `narrows`.
  - Several of these cuts kept S1's documents shorter than they would have been: no graphs, no runtimes, no placement lines.
  - Three of them moved cost into exactly S1's change tasks:
    - **3a (policies removed):** there is no narrowing in place, so T4 became close + register-replacement + open for each Agent.
    - **10 (rename removed):** names are fixed, so templates misdescribe what they grant after T4, and standalone Conduits need distinct names, which gives distinct tool names.
    - **17 (removal after retirement):** a closed Conduit is removed, so T7 had to resubmit it, and T6's mistake 3 (finding 4-10) followed.
- **No record under docs/proposals/ proposes cutting document frames or the template/instance pairing.**
  - The consolidation audit went the other way: CS10/CC8 (consolidation/README.md:117, 177) made first-start objects instances of core templates, so that every instance comes from a template.
  - The principles review §5 kept "the Kubernetes envelope, names and references" as aligned with existing paradigms (principles-review/README.md:861).
  - The instance schemas require `from` (spec/schemas/instances.yaml:80-127).
- **Root's Conduits to its own Pool** are touched only by D18's deferred alternative, "one uniform management target", which was never written up.
- **Changing what already exists (finding 4-01)** is touched only by three open items: D08's trigger, D09/A67's restart verb, and A66 (policies again).

## 2. Round 6, D01 to D22: what each proposed and its fate

Sources:
- the critique: docs/proposals/reaudit/round-6/design-critique.md
- the decisions: docs/proposals/reaudit/round-6/DECISIONS.md:103-126
- the ruling: docs/proposals/ruling-44/README.md
- the D06 and D12 log: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-d06-d12.md

"Coordinator / owner" means the coordinator decided under rulings 37 and 38, and the owner accepted it in ruling 44 on 2026-11-07.

| Item | What it proposed | Expected saving, as stated | Decision, by whom | Where it went / status | Touches S1's costs? |
| --- | --- | --- | --- | --- | --- |
| D01 | A qualification table for a plain server, a small adapter and a Conduit-aware implementation, with adapter work counted; reusable implementations for the first restriction patterns | None claimed; it sizes integration cost | Experiment programme (coordinator / owner) | S2, S7, A63 (ROADMAP.md:176, 180); open | No |
| D02 | Four retry cases: key-keeping clients and keyless clients, against plain and deduplicating implementations | None | Experiment programme | S4; open | No |
| D03 | Show effective authority in two parts, with a negative example; ask whether filter-preserving delegation is needed | Clarity only | Option a, applied (owner, ruling 44 point 2) | spec/README "what access does not narrow"; ACC-029/030 observables | No |
| D04 | Narrow peer provisioning through a broker rather than a policy language | Avoids a new policy language | Option a, applied (owner, ruling 44 point 2) | ACC-030. "If it is clumsy, a focused constraint, such as a list of allowed templates on access to management, comes back to the owner." E08 ran only as a core vector with a simulated broker (log-increment-2.md:948-958); no author has measured it | No |
| D05 | Make the first requirements complete contracts, with vectors shared by two implementations | None | Experiment programme | S2; open | No |
| D06 | Remove Operation-level default implementations; keep Type defaults and template selections | "Deletes a semantic fallback": predicting what runs needs 2 places, not 3. The critique's own cost: more Type mappings; reopens C42 | **Accepted as revised** (owner, "Agree on both", 2026-11-07), after the check found that every Pool Type would have to map 7 operations and that the failure would remain under TPL-004 | Applied as A69 (AUDIT.md:803): a package's own Operation contracts state no default; the engine implements the core operations by rule. CTR-002, TPL-003, TPL-004; 14 core `implementation: …#engine` lines removed. The owner may strike two editor's choices: a template may not override an engine operation, and a reference that resolves to nothing stays invalid | No. S1 authors select no implementation; this is provider-author territory |
| D07 | Discovery shows the effective control constraints. Alternative: remove per-template narrowing of control inputs | Fewer invalid calls; no lines | Accepted, applied (owner) | IFC-033 presents `allOf`; IFC-012 presents no tool for an excluded control, and a call to it reports invalid in the protocol's unknown-tool form. The removal alternative was **not taken**, so templates keep their narrowing | No. S1 uses no controls |
| D08 | A change-and-repair chapter (live setting, template upgrade, nested Resource replacement, access restored); a frontend may show "a pure preview of the required explicit actions". "If the only usable upgrade path repeatedly recreates unrelated agents or relationships, that is evidence for a targeted language change. Establish it with a complete trace first." "Do not add automatic reconciliation, rollback or background model work to hide the cost." (critique :282-303) | Measurement only | Experiment programme | Became S1's T4 to T7 (FIRST_BUILD.md:131). **The trace now exists in both runs.** The anticipated language change is open as finding 4-01. The preview tooling has not been built. Nested replacement through a Pool entry has not been measured: S1 uses top-level Resources (increment-2/plan.md:146) | **Yes, as the measurement that exposed the immutability cost, not as a fix** |
| D09 | Rotate a secret by explicit replacement first; bring A67's restart verb to the owner only if that is disruptive | Measurement | Experiment programme (S2, S10) | A67 open (AUDIT.md:831) | Indirectly. A restart that keeps identity and Conduits is the nearest proposed in-place mechanism |
| D10 | Make `ruleId`, invocation identity and status enough to tell the cases of a conflict apart | None | Experiment programme (S4, S5) | Open. Related to finding 4-12: `reconcile.v1` takes one `occurred` value for a whole turn | No |
| D11 | `replace.v1` takes a fresh project context instead of the original project's `#name` context | Removes unregistering and re-registering to load an edited local contract | Accepted, applied (owner, ruling 44 point 3) | SUB-009, NAM-022, PKG-029; `replace.v1` gains `contracts` and `manifest`. Follow-on R-11: `replace.v1` now trusts its consumer with the contracts and implementation items it hands over (ACC-030) | Used by both authors' replace routes (coordinator T4, fresh author T5), but S1 has no local contracts. INS-020 still freezes instances, so it changes templates only. **Saves 0 lines** |
| D12 | Allow resolved `{kind,id}` in actor requests | Removes an actor/onward asymmetry | **Accepted as revised**: names stay the only address and LIF-031 stands; set-up, retire, inspect and unregister take an optional `expectedId`, and a mismatch is a conflict (owner, "Agree on both") | LIF-037 (lifecycle.yaml:852); A69 done. It adds a mechanism rather than deleting one | No |
| D13 | Compare the schema-default insertion model with explicit whole-input defaults | "Neither is an automatic win" | **No change** (coordinator / owner) | E16 in S5; open | No |
| D14 | Allow the full schema profile at ordinary Operation roots; controls keep the limited root | Removes wrapping that adds structure without meaning | Accepted, applied (owner, ruling 44 point 4; reverses ruling 41 item 16a) | CTR-012. Follow-on rule C-03 in SCH-009; a root `$defs` key collision is still open (DECISIONS.md:160) | No. Affects contract authors only |
| D15 | Record provider costs | None | Experiment programme (A63) | Open | No |
| D16 | Define what a provider swap preserves before S10 runs | None | Experiment programme (S10) | Open | No. T6 is analogous but is not a provider swap |
| D17 | Keep installation explicit, but let tooling prepare dependency-ordered installation requests with pinned artifacts. "Do not add version ranges, implicit dependency installation or aliases simply to shorten the first example." "A package author can absorb repetition without requiring every end user to become one." | Usability through tooling | **No change** to the rules (coordinator / owner) | The tooling half is not built: digests are copied by hand | **Partly.** It bears on manifests, digests and contract addresses (see §6), but the decision was no change |
| D18 | State ACC-033 as one decision table; tools name the required authorization target where the caller may learn it. "If evidence later favors one uniform management target, consider that as a separate behavior redesign" | Fewer reading errors | Accepted and applied as a restatement, with no answer changed (owner) | ACC-033 is a 14-row list. The explanatory half returns as ruling 52 point 1, two-tier refusals; settled in direction, promises not yet written. **The uniform-target alternative was never proposed** | Only by diagnosis: the coordinator found mistake 2 by reading ACC-033. The root's Pool Conduits come from rows (1) and (4) and are unchanged |
| D19 | Measure invocation state | None | Experiment programme (A38, A14) | Open | No |
| D20 | Put the trust model beside the first example | None | Experiment programme (S2, S3, A64) | Open | No |
| D21 | Reading paths per audience. Low-priority options: remove the `from: '#name'` synonym; a formatter that favours one reference form | Reading burden | Accepted and applied as reading paths only; the synonym removal and the formatter were not taken | spec/README reading paths | No lines saved. The fresh author completed S1 without reading ruling history, which meets criterion 1 |
| D22 | Judge by cross-rule evidence and usefulness, with author measurements; "the first end-to-end test should be allowed to falsify a simplification proposal in this report" | n/a | Accepted (owner, ruling 44 point 1) | First build (A68); increment 4 measured S1 | This is the bar that both Conduit runs fail |

## 3. Other parts of the critique

- **"What should be preserved"** (critique :64-86): Conduit as a separate object; the five families; ownership not granting access; immutable resolved IDs (OBJ-009, OBJ-011, INS-020); explicit registration, setup and effects; unknown as an outcome; revisions; providers outside the language; rejection of unsupported filters; no expression language. Also "keep one capability per Conduit" and "one run per instance". Two of these are themselves S1 cost drivers:
  - immutable resolved definitions;
  - explicit separate setup and registration.
- **"What to cut, retain or investigate"** (critique :639-661). How each row ended up:

  | Row | Fate |
  | --- | --- |
  | Operation-level defaults | D06, applied as revised |
  | Template control narrowing | D07: exposure applied, removal not taken |
  | Original-project-only replacement context | D11, applied |
  | Ban on resolved IDs in actor requests | D12: ban kept, `expectedId` added |
  | Recursive default insertion | D13: measure, open |
  | Operation-root restriction | D14, applied |
  | Mutable package-source claims | Made immutable by finding 6-1-05 (PRV-016) |
  | Permanent unkeyed invocation state | D19: measure, open |
  | Five families, ownership, lifecycle, unknown | Retained |
  | One run per instance, one capability per Conduit, no expression language | Retained pending contrary evidence; S1 gives none |
  | Automatic apply, rollback, key inference, general policies or graphs | "Do not add to make examples look shorter. Tools may prepare explicit actions; effects and uncertainty remain visible" |

  The last row is in direct tension with finding 4-01's first option ("resubmitting a changed document updates the object, as `kubectl apply` does"). It is the critique's recommendation, not a separate ruling. DECISIONS.md records no row for it, but the purpose's "no background reconciler or automatic wake" (AUDIT.md:464) does bind.
- **The four open points** (DECISIONS.md:128-135):
  - a Type's default for a hook it does not require: applied (TPL-004, CTR-004);
  - a repeated Package argument: waits on S7/A63;
  - the cause in a record of a machine's loss: waits on A39;
  - OPS-028's late onward request: E20 in S4, A65.

  None of them is a simplification.
- **The evidence cases E01 to E20** are in ROADMAP.md:164-181.
  - E05, E08, E11 and E12 ran as core vectors in increment 2 (log-increment-2.md:948-958, 1100).
  - ROADMAP.md:169 still says "None has been run", which is stale.
- **The baseline comparison and success criteria** (critique :692-704) became A70 and ruling 46 point 3.
  - The critique's way out, that "a language that adds setup work can still win if changing and auditing relationships becomes materially easier", did not hold on lines. T5, T6 and T7 also cost more lines.
  - It held on safety and continuity: no restarts, no tool leak at T5, and T3 answered in two commands.

## 4. Ruling 41: what was cut before, and why

Sources: principles-review/README.md §2, ruling-41/README.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md "Decisions checklist" and "Counts". The owner's ruling on 2026-11-06 was "Accept all". The principles were P1 (swappable behind a contract), P2 (no architectural invention) and P3 (not overcomplicated).

| Item | What was cut | Why (precedent) | Cost the review admitted | Effect on S1 |
| --- | --- | --- | --- | --- |
| 1 | Setup and retire graphs (about 26 GRA promises, 5 schemas); replaced by setup and retire hooks plus dependency order | Terraform create/delete; Kubernetes PostStart/PreStop; Compose | Composing operations needs a small MCP server rather than YAML | Positive: no graph lines in S1 |
| 2 | Runtimes declared in templates, held handles, `restoresRuntimes`, OBS-005, OBS-007; replaced by the runtime provider (run, stop, status) | CRI, Nomad, OCI | Several long-lived processes run as one | Positive: no runtime lines |
| 3a | Policies (`apply-policy.v1`, `remove-policy.v1`, ACC-026/031/035/042/043) | Policy decision points belong outside (AuthZEN); policies locked actors out of plain servers | "An open Conduit can no longer be narrowed in place: narrowing becomes opening a narrower Conduit and closing the wider one". Recorded as A66, a candidate later feature (AUDIT.md:830) | **Moves cost into T4.** Making B read-only is close, register-replacement and open: 8 requests in the coordinator's run, with about 30 s without access for each Agent |
| 3b | No outside decision point in v0 | P3 | — | Neutral |
| 5 | Composite Conduit templates (CON-003/019/029) | Kubernetes RoleBinding binds one role; reuse lives in templates | Two declared Conduits can be decided apart. One Conduit spanning capabilities was explicitly not recommended | Neutral; also removed one segment from tool names |
| 6c | `retrieve-result.v1` | Idempotency-Key, AIP-155 | — | Neutral |
| 7 | Requirement composition | Contracts are atomic | — | Neutral; S1 has no filters |
| 8 | Default location, location facts, locations registered by hand | Kubernetes scheduler and labels | — | Positive: no placement lines |
| 9 | Re-exports | Kubernetes API groups | — | Neutral |
| 10 | Rename (OBJ-005, OBJ-026 to OBJ-028, CON-026) | Kubernetes `metadata.name` "Cannot be updated" | "Renaming becomes registering a new object and retiring the old". Renaming had changed tool names (CON-026) | **Adds cost.** After T4 the template names describe access they no longer grant (4-01). Standalone Conduits need unique names, so tools appear as `history-a.*` and `project-b.*` (IFC-012) |
| 12 | Registration with packages missing (INS-022 and deferral clauses) | `terraform init`, Helm | One explicit install first | One `install` request in T5 |
| 14, 15, C20 | `obliges`; clamping in controls; `narrows` | OAuth resource server; Terraform | — | Neutral |
| 17 | Two-step removal; closed Conduits are now removed at close (LIF-035) | Kubernetes finalizers | "A closed Conduit is removed when it closes" | **Adds cost.** T7 resubmits the Conduit document (12 lines, 5 of them the manifest). T6 mistake 3: unregistering an already-removed Conduit reports invalid (4-10) |

Section 6 of the review (considered, not proposed): C29 (RFC 9396) and C39 (a cascade flag on `retire.v1`). Neither is relevant to S1.

## 5. Earlier simplification records outside round 6 and ruling 41

- **The vocabulary simplification pass** (docs/research/vocabulary/SIMPLIFICATION.md) was adopted in rulings 17 and 18. It changed names only and merged no behaviour.
- **The consolidation audit** (rulings 25 to 29):
  - CS1, CS4 and CS10 unified where things go in a document.
  - First-start objects became instances of core templates, which entrenches "every instance has a template".
  - CC10 called the split between type and template "principled" and noted the overlap between type defaults and template selection, which is D06's ancestor.
- **Standing direction** (AUDIT.md:468): "Not differentiating, and so free to simplify: the graph engine, the package system, the schema profile, persistence and export". From rulings 23 and 24: do nothing revolutionary, and rely on existing frameworks.

## 6. S1's measured cost drivers against every proposal

Figures are from coordinator-run-2026-11-10.md:82-96 unless marked "my recount" (made from coordinator-run/project*).

**1. Each document's frame (`apiVersion`, `kind`, `metadata`, `name`, plus `---`)**
- About 130 lines plus 26 separators, roughly half of 332. My recount: 147 frame lines including the manifests' `apiVersion`, 27 separators, 35 documents.
- Proposals that touch it: none. Principles review §5 retained the Kubernetes envelope. D21 offers only a formatter and synonym removal.
- **Unaddressed.**

**2. A template and an instance for everything**
- My recount: 16 templates and 19 instances, each template used once.
- Proposals: none. The consolidation audit (CS10) moved toward more template use.
- Existing mechanism that is not used: packages may export templates ("a qualified reference, for a template that a package exports", spec/promises/instantiation.yaml:383). This is D17's "a package author can absorb repetition". But S1's filesystem package exports contracts only (conformance/fixtures/s1/packages/filesystem/conduit.yaml).
- **Unaddressed.**

**3. Change is new documents (finding 4-01)**
- The governing promises:
  - OBJ-002 and SUB-007: a submission only adds;
  - OBJ-011: no document rewrites an instance's definition;
  - INS-020 and PKG-005: instances keep their resolved definitions and package versions;
  - LIF-009: a replacement is made from the nested instantiation as it is held, with its target fixed.
- What it cost: T5 62 lines, T6 29, T7 12 in the coordinator's run; the fresh author recreated both Agents.
- What touches it:
  - D08 measured it, and its trigger is now met;
  - D11 covers templates only;
  - D09/A67's restart verb is open;
  - A66, policies again, is open and covers narrowing only. T4 also widens A's access.
- Finding 4-01 names two candidate options, both open for the owner:
  - (a) resubmit-to-update, as `kubectl apply` does;
  - (b) operations that point an Agent's nested Conduit at another template or Resource, and move a Resource to another package version.
- Constraints on any fix: the critique's "no automatic apply/rollback", D08's "do not add automatic reconciliation", and the purpose's "no background reconciler".

**4. Root's Conduits to its own Pool**
- The source is ACC-033 row (1): registering into a Pool needs both `management` and the Pool, each with `register.v1`. Row (4): setting up or retiring a Resource is authorized by its owning Pool.
- What it cost: 35 lines and two mistakes in the coordinator's run. It was also the fresh author's first mistake.
- What touches it: D18 (form only; the uniform-target idea deferred and never written), and ruling 52 point 1 (the refusal explains itself, but no lines are removed).
- Any fix must respect preservation tests 2 and 8: a human's access is an Interface holding grants that can be inspected and revoked.
- **Unaddressed.**

**5. Tool names follow Conduit names**
- IFC-012 (implementation.yaml:476): a tool is named by the Conduit's local name and the operation. Only a nested replacement under LIF-009 keeps a declared Conduit's local name.
- What touches it: nothing in round 6. Ruling 41 items 5 and 10 removed the composite segment and the renames that changed names.
- **Unaddressed.**

**Further drivers visible in the files, not listed in finding 4-11:**
- **A manifest with every submission, repeating its pins.** My recount: 37 manifest lines across the coordinator's 7 submissions, including the 8 copied digests; 5 of T7's 12 lines. Add the 45 lines that name a contract address. D17 rejects aliases "simply to shorten the first example", and its preparation tooling is not built.
- **T5's new Conduit templates (`read-2`, `read-write-2`, 16 lines).** These followed from T4's route, which left the template names misdescribing what they grant. NAM-017 (naming.yaml:324) says package versions are not compared when a Conduit is opened, so the version bump alone did not require them. This is my inference.

## 7. Round-6 proposals still open and relevant now, most relevant first

1. **D08's "targeted language change"**, with D08's tooling idea (a pure preview of the explicit actions a change needs). Its stated trigger is met: T5 and T6 recreated Resources and relationships, and the fresh author recreated both Agents. It is the round-6 hook for the decision on 4-01, which is now with the owner. Finding 4-11 is open "with 4-01 and round 6's simplifications (A68), after the fresh author's run", and that run has happened.
2. **D18's deferred "one uniform management target"**, for root's Pool Conduits. It was never proposed. Ruling 52 point 1's two-tier refusal is settled in direction, but its promises are not written.
3. **D09 / A67's restart verb** (keep identity and Conduits while replacing the run). It is the nearest existing carrier for 4-01 option (b)'s "move a Resource to another package version". Alongside it, **A66 (policies again)** is the only recorded proposal for narrowing an open Conduit in place (T4's B side). Both are "a proposal for the owner, if one is wanted".
4. **D17's tooling half and its packaging remark.** Tooling would prepare pinned installation requests and manifests. Templates shipped in packages would let the end user write only instances.
5. **D03/D04's "if clumsy, a focused constraint comes back".** No author has measured it yet: E08 ran only as a core vector.
6. **Lower relevance to S1:**
   - D13, the default-insertion comparison (S5);
   - D21's `from: '#name'` synonym removal and formatter;
   - the retained-pending-evidence rows (one capability per Conduit, one run per instance), on which S1 gives no contrary evidence;
   - the strikeable editor's choices on D06 and D12.
7. **Not relevant to S1's authoring cost:** D01, D02, D05, D10, D15, D16, D19 and D20, which measure S2 onwards, and three of the four open points.

**Constraints these records place on any new simplification:**
- Preserve:
  - the five families;
  - the Conduit as an object that can be inspected and revoked;
  - immutable resolved IDs: no silent retargeting (OBJ-010);
  - explicit effects, with unknown kept visible;
  - one capability per Conduit, unless there is scenario evidence against it.
- Do not add:
  - automatic apply, rollback or background reconciliation;
  - version ranges, implicit installation or aliases just to shorten examples.
- Follow the rulings:
  - ruling 40: adopt an existing paradigm, not an invention;
  - ruling 52: every connection explicit and descriptive.

## Numbers
| Quantity | Value | Source |
| --- | --- | --- |
| Round-6 design critiques | 22 (D01-D22), 4 open points, 20 evidence cases | design-critique.md |
| Applied | D03, D04 (option a); D06 and D12 (as revised); D07, D11, D14, D18, D21, D22 | DECISIONS.md:103-126 |
| No change | D13, D17 | DECISIONS.md:117, 121 |
| Experiment programme | D01, D02, D05, D08, D09, D10, D15, D16, D19, D20 | DECISIONS.md |
| S1 author lines saved by the applied round-6 simplifications | 0 (no S1 author file uses `implementation`, `controls`, `expectedId`, `inputSchema` or local `#` references) | grep over both runs' files |
| Promises: before ruling 41 / after it / round 6 / after D06 and D12 / now | 434 / 426 / 452 / 454 / 455 | PLAN.md Counts; https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-d06-d12.md; my count of `- id:` lines |
| Ruling 41: promises removed / added | 67 / 59 | https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md |
| ACC-033 `then`, words, before D18 / now | 406 / 418 | critique :512; my count |
| Lines written: baseline / coordinator / fresh author | 64 / 332 / 646 JSON + 130 script | results files |
| Coordinator lines T1..T7 | 154, 73, 0, 2, 62, 29, 12 | coordinator-run:42 |
| Baseline lines T1..T7 | 33, 16, 0, 8, 3, 2, 2 | 007 results:20 |
| Frame lines + separators (coordinator) | about 130 + 26 (coordinator); 147 + 27 over 35 documents (my recount) | coordinator-run:84; my recount |
| Templates / instances written (coordinator) | 16 / 19 (my recount) | coordinator-run/project* |
| Root's Pool Conduit lines (coordinator) | 35; mistakes 1 and 2; also the fresh author's first mistake | coordinator-run:90-93; fresh-author-run:55 |
| Manifest lines across submissions (coordinator) | 37, including 8 copied digests (my recount); T7: 5 of 12 lines | coordinator-run/project-*/conduit.yaml |
| Contract-address lines (coordinator) | 45 | coordinator-run:47 |
| T4 cost under the policy-less route | 8 requests; about 29-30 s without access for each Agent | coordinator-run:42-52 |
| S1 cost drivers addressed by any round-6 proposal | 0 of 5 (frames, template+instance, immutability, root's Pool Conduits, tool names); immutability is only measured (D08) | this analysis |

## Unverified
- I did not read the principles review §4 (secrets, reduced), ruling 41's group logs (log-G1 to log-G7, log-integration.md), round 6's editor logs other than the D06/D12 log and the parts of log-integration.md I grepped, or reader-1.md beyond its simplification note. A minor simplification decided only in those logs could be missing here.
- The grep for later handling of D06, D07, D08, D11, D12, D13, D14, D17 and D18 matched many files under ruling-42, ruling-43, core-and-schemas and multi-machine. Those use D-numbers from other schemes, for example ruling 41's open questions D1-D20. I excluded them by date and context, not by reading every hit.
- "Saves 0 lines" for D06, D07, D12 and D14 rests on a grep of both runs' author files for the features they changed. It does not measure provider work: D06 may have shortened the shipped packages' contracts.
- My recounts of frame lines, templates, instances and manifest lines are rough greps over coordinator-run/project*. Some documents appear in two directories (project and project-t1fix), so they differ slightly from the coordinator's "about 130 lines, 33 documents".
- Whether T5's new Conduit templates were required is my inference from NAM-017 and from T4's route. The coordinator's log does not give the reason.
- I did not check whether the owner has since struck either editor's choice on D06 or D12. DECISIONS.md and A69 still list them as strikeable.
- I ran nothing: no engine, no vectors and no checks. Every result cited is from the dated records.
- The owner's bar for v0 ("less of an author's work than the baseline on most of S1's tasks and much more on none") appears in the repository only in coordinator-run-2026-11-10.md:28. I found no ruling record that states it.