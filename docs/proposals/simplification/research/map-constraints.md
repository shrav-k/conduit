Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Conduit language simplification: hard constraints, owner rulings, habits that can go, and what a change would cost to rebuild

Scope: the repository <home> was only read; nothing was changed. Only spec/ binds. The rulings are cited from docs/AUDIT.md (ruling log at lines 536-597, purpose at 431-471, preservation tests at 473-494, layered claim at 711+). Promise IDs come from the YAML files under spec/.

## 0. What the simplification has to fix

- **The costs.** Finding 4-11 in docs/proposals/first-build/log-increment-4.md names the costs, and 4-01 is the change model behind them. I counted the coordinator's final files (docs/proposals/first-build/increment-4/coordinator-run/*/*.yaml): 349 non-blank lines in 35 documents plus 7 manifests, made up of 16 templates and 19 instances.
- **The frame is more than half of it.** The frame alone (apiVersion, kind, metadata, name, `---`, spec) is 42+35+35+35+27+16 = 190 lines, about 54%.
- **Other measured costs:**
  - Lines naming a contract's address: 45.
  - Root's own Conduits to its Pool: about 35 lines, and the first mistake of both authors.
  - Copied digests: 8 lines.
- **Packages ship no templates.** The S1 packages under conformance/fixtures/s1/packages ship contracts only (no `kind: Conduit`, Resource or Agent templates), so every template was written by the author.
- **The repository already names the fix.** FIRST_BUILD.md's thesis table, for when thesis 1 fails, says: "Simplify the documents: fewer concepts, better defaults."
- **Where the bar comes from.** The bar ("less ... on most of S1's tasks and much more on none") is quoted as the owner's only in coordinator-run-2026-11-10.md:29. It is not a row of the ruling log.

## 1. Guarantees that must survive

A note on the "safety" label. The specification has no safety field on its promises, and INDEPENDENT_BUILDS.md (G5) says no list of the safety promises exists yet.
- conformance/manifest.yaml marks cases with `safety_rows`: 255 cases are tagged with 21 rows covering 149 promises.
- 74 cases have no `safety_rows` key at all. These are the access, setup and records vectors (access-s1-exact-access, setup-close, setup-close-in-flight, setup-restore, access-s1-pool-authority, records-broker (E08), records-replace (E11), records-expected-id (E12), records-purge). Yet section 3 of docs/proposals/first-build/increment-2/plan.md calls them "remaining safety traces".
- 38 cases have an empty list.
- So "safety" below means the tagged rows plus the safety traces in section 3 of that plan. Fixing the manifest's tagging is itself a to-do.

Each guarantee below gives the promises that carry it, the rulings and tests behind it, and what it means for a simplification.

**G1. Access is exactly the operations that open Conduits list.** Each request is decided at the target, before dispatch.
- Promises: ACC-001, ACC-002, ACC-003, ACC-008, ACC-009, ACC-016, ACC-036, CON-001 (one consumer, one target, one capability), CON-017, CON-021, IFC-012 (the tools are exactly the listed operations), IFC-014, IFC-033, NAM-023, NAM-026.
- Rulings and tests: preservation test 2; rulings 17 (V3), 28 and 31. Round 6 asks to keep the explicit allow list (design-critique.md:64-86).
- Consequence: no shorthand may grant "the whole capability" by default. Offering a capability must never grant it.
- This gave the build its one clear safety win, which must be kept: upgrading the server added no tool for any Agent. NAM-024 lets new releases only add, and ACC-003 with IFC-012 means nothing is granted unless listed. The baseline's read-only agent gained `append_file` and `copy_file`.

**G2. Every permission is a Conduit.** Ownership permits nothing, the right to open is itself a Conduit to `access`, and delegation only narrows.
- Promises: ACC-005, ACC-006, ACC-012, ACC-022, ACC-024, ACC-028, ACC-029, ACC-030, ACC-032, ACC-033 (one decision table, D18), POOL-001, IFC-021, ACC-017.
- Rulings and tests: rulings 27 to 29 (the ruling 29 row: "every power over a Pool or its Resources is an open Conduit"); preservation tests 2, 5 and 8.
- Consequence: any authority granted for convenience (for example, root over a Pool it just made) must be a real Conduit that can be inspected, closed and recorded. It cannot be an exception in ACC-033.
- Ruling 51 makes this binding twice over. In peer mode, a peer engine admits the owner's root Interface "with Conduits to its management and access", so root's powers must be expressible as Conduits. The stated limits stay as they are: a bound does not narrow filters, and `management` cannot be narrowed to one template, so a broker is used instead (spec/README.md:337-375; ruling 44, D03 and D04).

**G3. Revocation is immediate and independent of both ends.** In v0 one control plane decides every request.
- Promises: ACC-011, ACC-036, ACC-037, ACC-034 (in-flight work finishes, but no new onward requests), ACC-039, CON-007, CON-008, CON-011, CON-013, CON-014, CON-031, IFC-013, IFC-038 (listChanged), IFC-022 and IFC-032 (a removed instance establishes nothing).
- Rulings: 29 (a closed Conduit is never reopened), 40 point 3f option A, 41 item 3a (to narrow, close and open a narrower one; a limit on an open Conduit is A66, later), 42 D6 (a request through a closed Conduit reports refused).
- Forward to v1: A65's done-when says nothing in v0 may assume that every decision is made in one place. The promises v1 would change are ACC-036, ACC-037, PLC-023, ACC-011, ACC-019, ACC-034, ACC-039, IFC-028, CON-008, IFC-012, IFC-013, IFC-038 and OPS-028.
- Consequence: a "change access" or "re-point" verb must close the old Conduit before or together with opening the new one. It must never re-target or widen a Conduit in place.

**G4. Every request is recorded and attributable.**
- Promises: REC-001 (a closed list of events, including access decisions, undispatched ends and revision changes), REC-002 (caller, Conduit, consumer, target, operation, invocation ID, the served ID, idempotency key, time, ending), REC-003 (no change except a purge), REC-004 (only root purges), REC-005, REC-006 (gaps are explicit), SEC-016 (no dispatch if its record cannot be kept), IFC-020, ACC-017, LIF-019, OPS-008 (the invocation index is kept apart from the records).
- Rulings and tests: preservation test 7; ruling 30; LD3 convention 6.
- Consequence: a compound verb (apply, swap or re-target) must still produce one record per access decision and per part, and keep the IDs of what it removed.

**G5. No secret appears in documents, records, diagnostics or results.** Only a Resource reads a secret, only root opens a Conduit that reads one, and the engine reads a secret only before a run or a connection.
- Promises: SEC-001 (a value at a place marked `secret-reference.v1` must be a reference; a literal is rejected), SEC-002, SEC-003, SEC-004, SEC-007, SEC-008, SEC-009, SEC-010, SEC-015, SEC-017, SEC-018, SEC-019, SEC-020 (nothing from the engine's environment), SEC-021 (a variable or header that a Package or Remote marks `isSecret` must bind to a secret reference), SEC-024, PRV-024, IFC-030, IFC-031, IFC-043.
- Rulings: 38 point 1, 39 points 1-4, 41 item 19, 42 D10 and D12.
- Consequence: any new inline or shorthand form (environment, headers, arguments, interpolation) must route values marked `isSecret` through secret references. It must not read values from the engine's or the author's environment.

**G6. A set of documents means the same thing wherever it is loaded (reproducibility).**
- Layered claim: rulings 13 and 14, LD1, and spec/README.md's layer table ("answered by resolving ... and runs nothing").
- Promises: SUB-001 (order has no meaning), SUB-002, SUB-003 (all or nothing), SUB-008 (project digest by RFC 8785), SUB-009 (a replacement resolves against the project handed over with it, D11), INS-013 (an instance holds the full resolved result), INS-018 (no relative path is joined to any base), INS-019, OBJ-009, OBJ-017, OBJ-024 (the specification version of each document is known; ruling 35), NAM-006 (one version and digest per package per project), NAM-009, NAM-010, NAM-011, NAM-014, PKG-003, PKG-004, PKG-005, PKG-006.
- spec/README.md's "Conformance" section: an engine never accepts a document and silently ignores part of it.
- Consequence: any shorthand must have one defined expansion in spec/, with diagnostics that point at what the author wrote. Meaning must not depend on the working directory, environment or hostname (INS-018; DD5 of the declarations record).

**G7. Resolving is deterministic, and the core is pure and cites a promise for every decision.**
- Promises: INS-017 (same template, same arguments, same objects give the same result apart from generated names and IDs), OBJ-003, OBJ-009 (resolved once to IDs), OBJ-010 (a reused name never silently redirects an existing relationship), INS-020 (replacing a template leaves existing instances unchanged), OPS-026.
- Ruling 46 point 1: a pure, deterministic core in which every decision cites a promise. See reference-model/crates/core/src/lib.rs:1-3: "No filesystem, clock, network, process or ambient allocation. The caller supplies ... names, time, machine evidence and unique IDs."
- Round 6 lists "Resolved relationships use immutable IDs" among what to keep.

**G8. Nothing happens implicitly.**
- Promises: INS-016 (instantiating sets up nothing), SUB-006 (registering does nothing else), IMP-001, IMP-002, IMP-003, IMP-007, IMP-008, OBS-001, OBS-004 (a difference between declared and observed state is never repaired without a request), OBS-006, LIF-008 (nothing is recreated automatically), LIF-023, LIF-027 (no rollback), ACC-019 (no work that outlasts its invocation), POOL-007.
- Rulings and tests: preservation test 6; ruling 29; ruling 30; ruling 38 point 6.
- Round 6 (design-critique.md:655): "Automatic apply, rollback, key inference ... Do not add to make examples look shorter."
- Consequence: an `apply` is allowed only as an explicit request that the actor makes each time, reports its parts and runs no reconciler. No setup or turn may start because a document changed.

**G9. Unknown is a first-class outcome, and no effect happens twice.**
- Promises: OPS-001 to OPS-014, OPS-018, OPS-021, OPS-022 (parts make up the outcome), OPS-024, OPS-028, OPS-032, OPS-034, LIF-004, LIF-028.
- Safety rows: S1-plain-ends, S1-unknown-write, S1-reconcile, S1-key-scope.
- Consequence: a compound change must report succeeded, failed or unknown for each part, as OPS-022 does for a setup. It must never infer that an effect failed.

**G10. Changes are checked against the current revision.**
- Promises: REV-001 to REV-015, in particular REV-002 (a stale revision gives conflict) and REV-011 (no revision gives invalid). SUB-009 names the revision.
- Consequence: an update-in-place must name the current revision or provide an equivalent stale check. A tool filling it in silently would defeat the check.

**G11. One forward-only lifecycle, one owner, retire before unregister, and permanent first-start objects.**
- Promises: LIF-001, LIF-005, LIF-006, LIF-008, LIF-009, LIF-015, LIF-017, LIF-022, LIF-034, LIF-035, LIF-037; POOL-001 (exactly one owner, never changed); FST-001 to FST-006; ACC-022; ACC-024.
- Rulings: 29 and 30.

**G12. Identity comes from proof, not location, and network membership grants nothing.**
- Promises: IFC-022, IFC-030, IFC-032, IFC-042, PRV-018, PRV-022, PRV-024, PRV-025, PRV-028, DL13.
- Rulings: 40 point 3e, 43, and 51 point 1.

**G13. Unchanged MCP clients keep working.**
- Promises: IFC-001 to IFC-047, in particular IFC-007 (a plain server gets no Conduit keys), IFC-012 and IFC-014 (the tool set), IFC-033 (tool definitions derived from the contract as the target holds it) and IFC-038.
- Ruling 31, and FIRST_BUILD.md's thesis 3 ("Unchanged MCP clients work behind Conduit").

**G14. A shared thing is owned once and never copied to an Agent.** Providers can be swapped, and the language names no backing tool.
- Preservation tests 3 and 4; ruling 40.
- Promises: PRV-001 to PRV-005, SEC-020.
- A63 requires two providers per provider contract.
- The coordinator's run had 0 provider-specific lines, and that must stay so.

**G15. Restrictions are honest.** An unsupported filter is rejected, never dropped, and every restriction reports whether it is enforced or only mediated.
- Promises: CON-022, ACC-014, ACC-015, IFC-019, IFC-026, ACC-027, ACC-038.
- Preservation test 9.

**G16. Work across several machines stays possible.**
- Promises: PLC-001, PLC-002, PLC-013, PLC-015, PLC-017, PLC-023, PLC-024, PLC-026, PRV-014, PRV-015.
- Ruling 40 point 3b requires robust distributed work. Ruling 51 adds two modes, managed (a node) and peer (its own engine, with relationships across parties as two halves).
- Consequence: nothing may assume one machine, for example a path relative to the author.

## 2. Owner rulings that constrain the design

Ruling 47 is reserved for pull request 14, and its content was not read. In the "Collides with" column, "safety" means one of G1-G16 depends on the ruling. "Shape" means only the owner's choice of form is at stake, and the owner can amend it.

| Ruling | What it fixes | Promises | Collides with |
| --- | --- | --- | --- |
| 4 + scope guard | S1 passing must not narrow the design; round 6 keeps multi-machine, multiple owners, swappable providers and agents managing the system | — | Rules out any simplification that only works for one machine or one owner |
| 8 (IN1-IN7) | Closed input list; structure is never an input (IN2); explicit inputs with their own schema, no shorthand (IN4); no interpolation, expression or merge (IN5); the instance stores only the result (IN6); standard arguments | INS-003, INS-012, INS-013, INS-021, OBJ-016 | IN4 makes each input cost about 3 lines (shape). IN5 and IN6 back G6 and G7 (safety) |
| 8 (IN9) as restated by 18 (V2) | "Every instance comes from a template"; one kind per family; `from` marks an instance | INS-001, INS-002, INS-025 | **Driver 2 of 4-11** (template and instance for everything). Shape: no G-item needs a separate template document; what matters is IN6, OBJ-009, NAM-009 and PKG-005. Needs the owner's amendment. Ruling 17 already rejected two kinds per family as "cumbersome" |
| 13, 14 (LD1, LD3) | The language determines meaning; seven shared conventions; the test that assigns a rule to a layer | layer fields | G6. A new feature must be placed by LD1 |
| 15 | Purpose: establish once, share under restriction; the Conduit is an object; agents can operate the system; nothing implicit; private and shared stay separate | — | Everything |
| 17 | Preservation tests 1-9; a Conduit is pairwise (V3); the seventh convention; agents delegate only within a bound | CON-001, ACC-030, IMP-* | Test 5: every author shortcut must be an operation an Agent can be given through a Conduit, not something only the command line can do. Test 8: no rule may assume one owner |
| 19, 20 | Conduits to live instances are core | CON-012, CON-013, CON-014 | Standalone Conduits must stay. Their tool names are a separate choice (IFC-012) |
| 21 (DR5, first half) and 22 (DD2) | No document rewrites an instance's definition; a submission is a set; **submitting only registers** | OBJ-002, OBJ-011, SUB-001, SUB-003, SUB-006, SUB-007 | **4-01 and driver 3.** Applying a changed document in the style of `kubectl apply` contradicts DD2 and DR5, so it needs the owner |
| 23, 24 | For graphs, schemas and packages, nothing revolutionary; rely on existing paradigms | — | Favors copying existing tools (Compose, kustomize, Helm, Terraform's plan and apply) over inventing |
| 25, 27 | Package identity is address-like; exports are **fully qualified, with no aliases**; one manifest per project pins version and digest; a template records its packages; any number of versions installed side by side; the version is part of a contract's name; a breaking release takes a new address | NAM-002, NAM-005, NAM-006, NAM-009, NAM-011, NAM-024, NAM-025, CTR-001 | Address lines (45). Import aliases need the owner. Package-exported templates avoid most of these lines without aliases |
| 26 | Connections work across package versions | NAM-023 | An upgrade must keep NAM-023 |
| 28, 29 | Every permission is a Conduit; ownership permits nothing; every power over a Pool is a Conduit; one forward-only lifecycle (registered, ready, retired); a closed Conduit is never reopened; no suspended state; a replacement is a new instance, made only on explicit request | ACC-006, ACC-028, ACC-033, POOL-001, LIF-006, LIF-008, LIF-009, LIF-035 | **Driver 4** (root's Conduits to its Pool) and **T7** (restore means a new Conduit). Safety: G2, G3, G11 |
| 30 | Retired from dispatch; first-start objects permanent; declared and observed state compared but never repaired without a request; a closed list of events always recorded; only root purges; model work only through a marked operation | LIF-005, LIF-022, OBS-004, REC-001, REC-004, IMP-001 | G4, G8 |
| 31 | MCP is the wire and an operation is a tool; the engine presents each consumer its permitted operations; a Conduit template holds one capability, its operations and one `requirements` map; camelCase; `apiVersion` | IFC-*, CON-016 | Driver 1 (the frame) is partly ruling 31 and 37 shape |
| 35 | Names belong to a version of the specification; "a document's `apiVersion` says which version it is written for" | OBJ-017, OBJ-024 | The `apiVersion` line repeated on every document. The purpose (an engine can load documents of two versions) could be met per file or per manifest; the owner would need to amend the wording |
| 37 | Core package and schemas accepted as drafted (including the `metadata` and `spec` frame); a request names objects by reference only; names unique across kinds in a project | OBJ-012, OBJ-013, OBJ-018, NAM-027, LIF-031 | Driver 1, shape |
| 38, 39 | A secret is never a literal; only a Resource reads one; only root opens a Conduit that reads one; work that outlasts its invocation is dropped from v0 | SEC-*, ACC-019 | G5 |
| 40 | Innovate only on connection and access; everything else is a provider; multi-machine robust; one control plane in v0, local enforcement (B) likely in v1; proof of the sender across machines | PRV-*, IFC-022 | The simplification itself may only change connection and access. An authoring front end in the style of Compose or Helm is "existing paradigms", not new infrastructure |
| 41 | Removed policies, composite Conduits, `obliges`, re-exports, rename and graphs; to narrow, close and open a narrower one (item 3a); a closed Conduit is removed (item 17); setup and retire hooks; standard forms (SARIF, metav1.Condition, CloudEvents) | OBJ-011, ACC-011, LIF-035 | Bringing back composite or multi-capability Conduits, or rename, needs the owner. Round 6 says keep one capability per Conduit |
| 42, 43 | A closed Conduit refuses (D6); gates; proof forms; the engine's own user is in the domain of trust | IFC-041, IFC-042, PRV-025 | Unaffected by syntax |
| 44 | D03 and D04 limits and the broker; D06; D07 (tools show the target's limits); D11; D12: names stay the only address, plus an optional `expectedId` (LIF-037); D14; the first-build success criteria | LIF-031, LIF-037, IFC-033 | Any verb that names objects follows LIF-031 and LIF-037 |
| 45 | An experiment states what it must show before it runs | — | The simplification is judged by a re-run against the same measurement plan |
| 46 | One Rust codebase: a pure core that cites a promise for every decision, vectors, and a thin shell that is the first slice of the reference engine; S2 to S9 run on the same codebase; each fix goes into the specification with a vector; a feature change goes to the owner | — | The change is made in this codebase, not a fork, and every change in the language needs promises and vectors |
| 48 | Each Agent type turns off by default what an Agent reaches through its own login, and an input of its template turns each on per Agent | Agent package inputs | Per-Agent options must survive even if templates are folded into instances |
| 49 | A sign-in prompt is a URL elicitation, sent to the root Interface in v0 and never to an Agent; the account each home or Resource is for is declared and checked | IFC-007, IFC-016 (not yet written) | The configuration needs a field for the expected account. Pending actions are not promises yet |
| 50 | Enrollment becomes part of the language (design pending); a joined machine becomes a node only once a person confirms; one pending-action mechanism at the Interface, as a control panel | — (not yet written) | Leave room for actions that wait on a person. Nothing may complete without one |
| 51 | Network membership grants nothing; a SPIFFE trust domain; another party's machine is never a node; a relationship across parties is two halves; the pending action is generic; **managed and peer modes** | PRV-018, IFC-022 | Root's powers must stay expressible as Conduits (G2); no implicit "root can do anything here" |
| 52 | Refusals in two tiers; the rule that every connection is as explicit and descriptive as possible without disclosing what the caller may not learn; relative values only where a directory is handed over, and whether they go altogether "goes with the simplification of 4-01 and 4-11" | OPS-023, ACC-037, SUB-010 | Every new verb must report its parts, the IDs it made and removed, and tool-name changes, and refuse in the two-tier form. Explicitness is about what the engine says, not about how much the author must write |

**How the 4-11 drivers and 4-01 map onto these constraints:**

| Driver | Ruled shape in the way (the owner can amend) | Safety underneath (must keep) | Room left open |
| --- | --- | --- | --- |
| 1. The frame (about 54% of lines) | 31, 35 and 37 (`apiVersion` on every document, `metadata.name`, `spec`); OBJ-012, OBJ-013, OBJ-017, OBJ-018, OBJ-019, SUB-005 | Only that each document's kind, name and specification version are known (OBJ-017, OBJ-024, G6) | Version per file or per manifest; a project file keyed by kind; a documented expansion into the current form |
| 2. A template and an instance for everything | IN9 (8), V2 (18), INS-001 | IN6, INS-013, OBJ-009, OBJ-010, INS-020, NAM-009, PKG-005 | An inline, anonymous template inside the instance; packages that export templates (already allowed by INS-025 and NAM-022); one Conduit template whose `operations` is an input (INS-012, CON-016) |
| 3. Change means new documents (4-01) | DD2 and DR5 (21, 22); OBJ-002, OBJ-011, SUB-006, SUB-007; 41 item 3a; NAM-011 and PKG-005 (an instance keeps its package version) | G3 (close before open), G8 (explicit), G9 (parts), G10 (revision), G4 (records), OBJ-010 | An explicit management operation, recorded and decided through `management` under ACC-033, that closes, opens, retires or replaces as parts. Keep the local name so tool names stay stable. It should be atomic so there is no window without access (T4 had about 30 s) |
| 4. Root needs Conduits to its own Pool | ACC-033 rows 1, 4 and 6; FST-002 | G2 (every power is a Conduit), test 8, ruling 51 peer mode | The engine opens, inside the explicit registration, a recorded Conduit from the registering actor to the new Pool; or ACC-033 accepts a Conduit to `management`. Either changes access, so it goes to the owner |
| 5. Tool names follow the Conduit's name | IFC-012's naming rule | IFC-014 (unique), protocol characters | Name tools by a stable local name kept across replacement, or by the target |

## 3. Habits, not constraints

- **The repository already says these parts may be simplified freely.** AUDIT.md:468: "Not differentiating, and so free to simplify: the graph engine, the package system, the schema profile, persistence and export." Rulings 23 and 24 say: existing paradigms.
- **The document frame.** `apiVersion` on every document, nested `metadata.name`, a `spec` wrapper, one object per document, and `---` between them are Kubernetes habits. Only "every document's version is known" (ruling 35) is load-bearing. The coordinator's frame lines: 190 of 349.
- **Authors writing templates.** Packages may export templates, Conduit templates with a fixed operation list among them (INS-025, NAM-022, MAN-007). S1's packages ship none, so the authors wrote 16 templates. A package that ships `read-only` and `read-write` Conduit templates removes about 3 address lines per operation list without breaking NAM-002.
- **One template per access level.** `operations` may already be an input of a Conduit template (INS-012, CON-016).
- **Typing digests by hand.** "Nothing generated by a tool counts as authored" (FIRST_BUILD.md, Measures), so a tool may write the manifest's pins. NAM-006 and PKG-004 only require that the pins exist.
- **The authoring syntax.** YAML or JSON is a free choice (SCH-012, SCH-013), and the fresh author's 646 lines are mostly one-key-per-line JSON. The 130 lines of scripts make up for the command line's failures (4-13, 4-14, 4-15), not the language's.
- **Root writing its own Conduits to Pools.** Which Conduits exist is ruled (ruling 29: they must be Conduits). Who writes them is not.
- **A standalone Conduit for added access.** A declared Conduit has a stable local name (IFC-012), and LIF-009 (`register-replacement.v1`) makes a new one from its declaration. T7's resubmission happened because the restored Conduit was standalone, which is a habit. That a closed Conduit is removed and never reopened (LIF-035, ruling 29) is not a habit.
- **A Pool written for every S1 Resource.** POOL-001 requires an owner, and the purpose relies on Pools. A Pool with one entry, or a default project Pool that comes with its own Conduits, would be a design choice. FST-003 forbids anything beyond the first-start objects at first start, not later.
- **Recreating Agents at T7.** This came from the 180-second turn deadline (4-12), which belongs to the engine and the Agent type, not the language.
- **A front end that compiles to canonical documents.** This fits ruling 40 and the existing-paradigms rulings (Helm and kustomize over Kubernetes). DD5 already allows "a frontend may resolve a relative path before submitting".
  - For it to count as language, its expansion must be written in spec/ so that independent builds (A34) agree.
  - Otherwise it is a tool, and only its input counts as authored.
  - Either way, test 5 requires that what it sends are ordinary operations an Agent could also be given.
- **What is not a habit.**
  - An explicit operation list (round 6 says keep it).
  - One capability per Conduit (ruling 41 removed composites; round 6 says keep it).
  - No expression language (IN5; round 6).
  - One Pool owner for every Resource.
  - Closing and opening in place of narrowing (41 item 3a, until A66).
  - No automatic apply, rollback or reconciler (G8).

## 4. What a language change forces to be rebuilt

Paths are relative to <home> Sizes count non-blank-insensitive `wc -l`.

| Area | Size | Impact |
| --- | --- | --- |
| reference-model/crates/loader | 2,879 src + 2,144 test lines; 64 tests | yaml.rs, json.rs, number.rs, formats.rs and read.rs only parse files and survive any change of document shape. schemas.rs is driven by spec/schemas, so a change of shape is mostly a change to the schemas. authored.rs, rules.rs and project.rs (which files form a project, MAN-001, MAN-002, MAN-011) change with any new project or file form. Its tests check every example's verdict against tools/check_schemas.py's recorded verdicts, so both move together |
| reference-model/crates/core | 10,465 lines; driven by vectors | **registration.rs (2,061)**: template and instance resolution (INS-*), `from` (the Builder's template function, around line 271), references (OBJ-006 to OBJ-010), nested instantiations, Conduit checks (CON-*), the IFC-014 name clash (around 1,906-1,935), replacement (around 1,976). Hit by inline templates, a new frame, update-in-place and re-pointing.<br>**packages.rs (1,116)**: manifest, NAM-*, PKG-*, selections, items, digest. Hit by naming or alias changes and by digests in the manifest.<br>**access.rs (729)**: ACC-033 routing (lines 235-306), bounds, tool list. Hit by root authority over Pools and by tool naming (line 378).<br>**lifecycle.rs (1,133)**: the setup parts TPL-012, CON-004, POOL-013, with tool naming at line 347. Hit by any change verb that opens and closes as parts.<br>**retirement.rs (678)**: LIF-*, CON-008. Changes only if the lifecycle changes, which ruling 29 forbids.<br>**records.rs (522)**: the management verbs register, replace and register-replacement, plus export. Hit by any new verb.<br>**requests.rs (1,709)**: OPS-* outcomes and idempotency. Survives syntax changes; hit by ruling 52's two tiers (OPS-023) and the plumbing of a new verb.<br>**schemas.rs (1,149)**: IFC-033 standalone bundles and defaults. Survives except tool naming (line 1,107).<br>**archives.rs (573)**: survives.<br>**lib.rs (795)**: first start (lines 591-728) changes if root gets Pool Conduits, and new Input variants appear |
| reference-model/crates/conformance | 2,137 + 604 lines | The runner survives. Its fixtures and snapshots do not (see conformance/) |
| reference-model/crates/shell | 6,986 src (about 2,441 of it unit tests) + 3,297 test lines | Survives syntax: runner.rs (1,514, PRV-003), identity.rs (DL13), store.rs, boundary.rs.<br>Changes: author.rs (284) and main.rs (407) for the commands and new verbs; server.rs only for tool naming or listChanged. tests/changes.rs (851) covers the change routes and would be rewritten for a new change model |
| conformance/ | 367 cases (1,291 files) | 481 files hold Conduit documents and 268 pin digests; 240 cases point at shared fixtures; 84 .gz archives.<br>A change of document shape rewrites the document-bearing cases (about 80 of 84 in language/registration, 59 of 61 in language/packages, 29 of 33 in interface/schemas) and regenerates the archives. New archive bytes mean new digests (PKG-004), so manifests and snapshots change in cascade (fixture.json, root-system-state.json).<br>A change of the change model changes S1-duplicate (OBJ-002, OBJ-025), setup-close, setup-restore, records-replace and the 3 cases in conventions/changes |
| spec/ | 455 promises | schemas/: 41 contracts in 8 files (envelope.yaml, document.yaml, instances.yaml and templates.yaml for a new frame). core/: 7 templates, 23 Operation and 7 Capability contracts (a new verb goes in operations.yaml and object-management.v1). examples/: 69 valid and invalid files plus 41 invalid-by-rule. Promises touched: OBJ-*, INS-*, SUB-*, NAM-*, and ACC-033 and IFC-012 depending on the option |
| tools/check_schemas.py | 2,588 lines of Python | Validates the schemas, core and examples, so it is rewritten for any change of shape (and is required by AGENTS.md) |
| packages/agent-claude-code, packages/agent-codex; conformance/fixtures/s1/packages | Each has conduit.yaml, contracts.yaml and a dist archive with digest (994 lines of Rust wrappers that survive) | Documents rewritten, archives repacked, digests changed. If packages are to ship templates (section 3), they gain templates here |

**Rough cost by kind of change:**
- **Smallest: a front end or written expansion that compiles to today's documents.** It needs a new reader or expander in the loader, a spec promise and schema, and new vectors. The core, shell and existing vectors mostly survive. The risk is diagnostics that point at generated text rather than what the author wrote, which conflicts with ruling 52 and SUB-005's attribution.
- **Medium: drop or move the frame natively.** It touches the schemas, check_schemas.py, the loader, metadata and name lookups in the core (registration.rs has about 10, packages.rs about 8), all 481 document-bearing vector files, the examples, the packages, and every digest that follows from them.
- **Medium-large: inline templates.** Mostly registration.rs, LIF-009 and records.rs replacement (`register-replacement` reads "the template as it stands"), plus about 84 registration vectors.
- **Large: an explicit apply or re-point verb.** It needs contracts in core/, an ACC-033 row, records.rs, registration.rs (resolving again), lifecycle.rs and retirement.rs (its parts), requests.rs (the compound outcome), the shell's commands, new vectors, and changes to OBJ-002, OBJ-011, SUB-006 and SUB-007 (rulings 21 and 22).
- **Small-medium: root authority over Pools.** access.rs routing or a generated Conduit, lib.rs first start, and the vectors access-s1-pool-authority, access-entry-authority and first-start-*.
- **Small-medium: tool naming.** Five call sites in the core, the IFC-014 vectors (access-collision-*) and the packages' READMEs.
- **Untouched by any of these:** the runner, identity and gates, archives, strict parsing, outcome and idempotency handling, export of records, and the Rust code of the Agent packages.

**Process constraints:**
- Each change goes into spec/ with a vector (ruling 46 point 4). Removing or adding a feature goes to the owner, and nothing leaves scope by default (A18).
- `spec/names.yaml` is changed only through tools/set_names.py.
- The re-measurement uses the same measurement plan (ruling 52 point 6), with the 21 safety rows and the plan's section 3 traces passing.
- After that comes the cold read (the row "Read" of ROADMAP.md).

## Numbers
| Item | Number |
| --- | --- |
| Baseline / coordinator / fresh author lines | 64 / 332 / 646 (plus 130 lines of scripts) |
| Per task, baseline vs coordinator | T1 33 vs 154; T2 16 vs 73; T3 0 vs 0; T4 8 vs 2; T5 3 vs 62; T6 2 vs 29; T7 2 vs 12 |
| Coordinator's final files (my count) | 349 non-blank lines; 35 documents plus 7 manifests; 16 templates, 19 instances |
| Frame lines (apiVersion 42, kind 35, metadata 35, name 35, `---` 27, spec 16) | 190 of 349 (about 54%) |
| Contract-address lines / root-to-Pool Conduit lines / copied digests (coordinator) | 45 / about 35 / 8 |
| Promises in spec | 455 (203 language, 208 convention, 44 interface) |
| Conformance cases / files / files with documents / files pinning digests / .gz archives | 367 / 1,291 / 481 / 268 / 84 |
| Manifest safety tagging | 255 tagged cases (21 rows, 149 promises); 74 with no key (access, setup and records); 38 empty |
| reference-model lines | loader 2,879 + 2,144 tests; core 10,465; conformance 2,137 + 604; shell 6,986 + 3,297 tests |
| Largest core modules a language change hits | registration.rs 2,061; requests.rs 1,709; lifecycle.rs 1,133; packages.rs 1,116; access.rs 729 |
| Python checker that must change with any document shape | tools/check_schemas.py, 2,588 lines |
| Spec schemas and examples | 41 schema contracts in 8 files; 69 valid and invalid examples plus 41 invalid-by-rule |
| Rulings read for constraints | 4, 8, 13-15, 17-31, 35, 37-46, 48-52 (47 is reserved and was not read) |

## Unverified
- **Ruling 47** is reserved for the owner's open pull request 14. Its content was not found or read, so it may constrain the design.
- **The owner's bar** ("less ... on most of S1's tasks and much more on none") appears only in coordinator-run-2026-11-10.md:29 as the owner's words. It is not a row of the ruling log, and I did not find where the owner said it.
- **The safety set** is my reconstruction. The specification has no safety field. The manifest's `safety_rows` cover 21 rows, and the access, setup and records vectors have no key even though section 3 of the increment-2 plan calls them safety traces. INDEPENDENT_BUILDS.md (G5) says no list of the safety promises exists.
- **Ruled or habit for the frame.** I classed the `metadata`/`spec` frame as accepted with the schemas "as drafted" in ruling 37. I did not find a ruling that fixes `metadata.name` as a principle. Ruling 35's wording ("a document's apiVersion") could be read as requiring the key on every document.
- **Rebuild costs** are estimates from line counts and greps of field names (`from`, `spec`, `metadata`, `local_name`) and from reading the ACC-033 routing and tool-naming code. I did not run cargo test or clippy, did not trace every call path, and did not check whether the stored state's JSON layout would need migrating.
- **How far I read the promises.** I read the convention files in full (access, records, implicit, observation, operations, revisions, lifecycle) and objects, instantiation, submissions, pools, first-start, secrets and interface by their `when` and `then`. I read only selected promises of templates, packages, naming, providers and placement, and none of contracts.yaml, schemas.yaml or manifest.yaml.
- **Records not read in full:** the enrollment proposal and networking-paradigms.md, the sign-in proposal, the consolidation and conventions records (rulings 28-30, which I took from the ruling log rows), and round 6's DECISIONS.md.
- **Not checked:**
  - Whether exporting templates from a package raises issues with NAM-009 or PKG-005 pinning when the package is upgraded.
  - Whether a generated, revision-checked apply could satisfy REV-011 without the author handling revisions.
  - Whether the S1 packages could ship controls to make T6 a live change. They define none today.
- **Pending actions** (rulings 49-51) and the two-tier refusal (ruling 52) are not written as promises yet. Their exact shapes, and so the constraints they put on new verbs, are not final.