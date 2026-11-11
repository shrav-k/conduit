# Decisions in force

Not binding. The specification alone binds.

This file says what has been decided beyond `spec/` and still holds. `spec/` says what Conduit does. The ledger, `docs/proposals/ruling-log.md`, says what was ruled, when, in the owner's words, and each ruling's status. A ruling's record says why (ruling 67, point 1). Where a line here and `spec/` differ, `spec/` governs. Where a line here and the owner's words in a ruling differ, the owner's words govern until the line is fixed.

How to read a line:

- A citation such as "59 A3" means ruling 59, point A3. Promise IDs are the IDs used in `spec/`.
- A line headed **In spec/** points to the promises that already hold a decision. It does not restate them.
- *Spec pending* marks a decision that is ruled but not yet written in `spec/`. Its batch is named where a ruling or the [Fleet brief](docs/proposals/projects/brief.md) sets one. Where the line names a promise, that promise says otherwise today. Builds and vectors follow `spec/` until the change is written.
- *Open* marks a question the owner holds open, which no session settles.
- *Coordinator's* marks a decision the coordinator logged that the owner may still strike.

## 1. Purpose, scope and principles

- The language is the core, and the engine keeps its promises. Whatever makes the language more concise, concrete and useful should be done (6); sufficiency must be provable (10), and the critical concepts survive the rework (13).
- The relationship is the product. An agent decides for itself, so the language declares what surrounds it: what it may reach, under which limits, and who answers for what it did (15).
- Establish once, share under restriction. A Pool owns a login, a checkout or a memory, which agents of any vendor use, each under its own limits, and the credential is never handed over (15; POOL-001, ACC-003, SEC-007, SEC-019).
- The Conduit is an object. Each agent's access to each target can be inspected and revoked independently of both ends. To narrow an access, close the Conduit and open a narrower one (15, as amended by 39 point 7 and 41 item 3a; CON-001, ACC-011).
- Agents operate the system through the same contract a person uses: they set up, start a peer, and delegate a narrower slice (15; preservation test 5; OPS-019, ACC-030). Rulings limit this in v0. Only root registers Agents (59 A1) and admits machines (56 point 11). Only root drives, sets up or retires an Agent that acts for a Fleet (61 B6). Enrollment and secrets stay with root (66 point 3).
- Nothing implicit spends tokens or repeats an effect. An unknown outcome stays unknown, and nothing runs a background reconciler or wakes on its own (15, 17; preservation test 6). The seventh convention holds this, and topic 7 points to it.
- Private and shared stay separate. An agent's subscription and context are its own, and anything shared goes through a Resource (15, 48 point 1, 53 point 7).
- The first user is one developer running agents from several vendors on one machine (15). The starting acceptance scenario is Claude and Codex collaborating on one project on one machine: S1 and S2, a shared history and one service login shared under per-Agent restrictions (4).
- Teams are not in v0, but no rule may assume exactly one owner. A person's access is an Interface holding Conduits (15; preservation test 8; ACC-023). Where a ruling gives a power to root alone in v0, later versions are to open it to others: sign-in (49 point 2), approval (55 A4) and acting for a Fleet (61 B4). 62 C6 is open.
- A restriction reports whether it is enforced or only mediated. Enforcement is opted into by choosing, through the standard argument `runtime`, a runtime provider that confines its consumer. In v0 every restriction is mediated (15, 17 test 9, 37 definitions, 41 item 2d; ACC-027, PRV-006).
- Conduit innovates only on its documents, connection, access, accountability and the shape of its contracts. Everything else is either a provider behind a contract, swappable by the user, or an existing standard. Each rule names the practice it follows, and a design that invents infrastructure is wrong (23, 24, 40 points 1 and 2; spec/README.md, "Existing standards first").
- Graphs, the package system, the schema profile, persistence and export do not set Conduit apart, so they may be simplified freely (15).
- Work across machines is part of what Conduit is for, and it must be robust. Providers own the mechanics. Conduit owns what placement, a lost machine and a failed request mean for access (5, 40 point 3b, 42, 43). Topic 10 holds the details.
- The scope guard. The acceptance scenario is a first proof, not the scope. Passing it removes nothing, and "never exercised" is evidence about order only. Deferring or dropping anything needs an explicit owner ruling (A18). A shortcut taken to reach the scenario is logged with the general behavior it stands in for, and the object model is checked against scenarios unlike the acceptance scenario (S3 to S9) before anything is pruned (4).
- Swappable by the user, as far as possible: "we should go as far as we can in the 'swappable by user' direction, leaving decisions to the user on architecture and existing tools as long as they meet requirements", and "We shouldn't overcomplicate this" (the owner, 40 point 2). A choice left to each engine is written so that it can become a provider contract later without breaking anything: in a standard format, with no assumption of one backing system (40).
- The nine preservation tests ([docs/AUDIT.md](docs/AUDIT.md#preservation-tests)) are accepted. Every rework decision is checked against them, so that the purpose and the agent-first differentiation survive the rework (14, 17).

## 2. The specification: form, layers and conformance

- **In spec/**:
  - What binds, its parts and its three layers: spec/README.md, "What this is" and "Contents" (13, 17 NS1, 31, 32 G1, 37, 40). The layer decisions LD1 to LD5, with LD1's layer test, have no other record than [the layered claim](docs/AUDIT.md#layered-claim) (14).
  - The seven shared conventions: spec/conventions/ (14 LD3, 17).
  - The implementation interface as the one shared wire, so one engine's internal services need not work with another's: IFC-001, IFC-002 (14).
  - The form of a promise, stable IDs that are never reused, and the register of removed promise files: spec/README.md (17 NS2, 20, 41 P2).
  - The three conformance levels, each naming its version: spec/README.md, "Conformance" (24 PD5, 38 point 12a).
  - The seventeen implementation-defined entries, DL1 to DL10, DL12, DL13 and DL15 to DL19: spec/implementation-defined.yaml (31 L1, 32 U10, 41, 42 D8, 43 Q2, 49, 51, round 5's M69 as accepted by 58 point 1).
- A vector is a given-when-then case under conformance/. It binds a conformance claim, not meaning (17 NS4, 32 G1).
- Where each kind of decision is written: the language and the conventions in `spec/`, with vectors; a type contract with its type's package; and the conformance document with the reference engine (14 LD5, 42 N18, 47).
- Specification files carry no change narrative (58 point 6). The specification's one description stays first (45 point 1).
- The owner's decisions before the plan audit (M1 to M3, the decisions before public distribution, I1 to I7 and C1 to C3) are kept, with their status, in the ledger's appendix.
- Decisions that are ruled but not yet written in `spec/` are listed only here, as *spec pending* lines (67 point 7).

## 3. Vocabulary and the project's names

- **In spec/**: the terms in force, and each old word with its replacement, in spec/definitions.md (16, 17 V1 and V3 to V8, 18, 27 to 29, 32 W1 to W3, 37 definitions, 42 N1 to N20, 45 point 2). The names the specification assigns, defined once in spec/names.yaml and changed only by a new version of the specification: OBJ-024, NAM-020, IFC-029 (33 F3, 34, 35).
- Each area is renamed once, as it is converted. Older documents keep their words and are not mass-renamed. A word that one type or the conformance document owns is settled there (16, 17).
- `python3 tools/set_names.py --check` verifies the names, and the same tool rewrites them. No domain is needed before an engine is built (34, 38 point 5).
- The public name is ConduitLang, spelled `conduitlang` everywhere: the GitHub organization `conduitlang`, with `conduit-lang` reserved, the domain `conduitlang.dev` once it is bought, the npm scope `@conduitlang` and the crate prefix `conduitlang-`. The language stays Conduit (65).
- Identifiers stay `github.com/shrav-k/conduit/...` until the organization exists. Then one command moves them, under ruling 35's rule, before any outside user adopts them. A project-controlled domain is preferred (47, 64 point 2, 65).
- **In spec/**: Fleet and what it is not, owner, default Conduit, provenance, scope, frame, local name, inventory, tombstone, lift and trust domain: spec/definitions.md (53 point 3, 54 point 3, 59 A5, 60, 66). "Project" keeps its meaning; "Fleet" names the Agent-owning family.
- **In spec/**: a pending action has an answer, never an outcome: PND-001, PND-003 and spec/definitions.md (56 point 8).

## 4. Documents, templates, instantiation and submission

- **In spec/**:
  - Families and kinds, one kind per family, templates and instances, and the document's form: OBJ-012 to OBJ-014, OBJ-017 to OBJ-020 (18, 28 CS1, 31 D2, 33 F26, 37 C7 and C8).
  - Instantiation, defaults and destinations: INS-001 to INS-021, INS-025 (8 IN1 to IN7, 21 DR1 to DR4, 28 CS2 and CS7, 37 P4, A3 and D17).
  - Standard arguments, `runtime` among them: OBJ-013 (8, 20, 41 item 31b).
  - References and what a request names: OBJ-006, OBJ-008 to OBJ-010, OBJ-021, LIF-031, LIF-037 (18, 19, 21 DD1, 32 G2, 37 X2, C13, C21, D14 and D16, 38 point 10, 44 D12).
  - Names, fixed for an object's life and unique across kinds within a project: OBJ-001 to OBJ-003, OBJ-011, NAM-027 (21 DR7 and DR8, 24 PR8, 37 X5, 41 items 10 and 30).
  - Submission as a set, which only registers: SUB-001 to SUB-011 (21 DR9 and DR10, 22, 27, 44 D11, 52 point 3).
- *Open*: whether relative values in a submission go altogether is decided with the simplification of findings 4-01 and 4-11. Until then SUB-010 stands, and relative values are resolved only where an engine is handed a directory (52 point 3).
- *Spec pending, batch 1*:
  - **In spec/**: the Agent's required, fixed Fleet owner: OBJ-013, FLEET-002, FLEET-003 (54 point 1, 59 A1, 60).
  - **In spec/**: Pool and Fleet names and the reserved names: FLEET-006 (59 A3).
  - **In spec/**: standalone Conduit local names and `expectedId` on opening and closing: OBJ-013, IFC-012, IFC-014, LIF-037 (53 point 3). The `instance.v1` and open/close input edits are in the core package, landed by order 039.
  - **In spec/**: SCT-001 to SCT-016 (SCT-010 unused), spec/promises/sections.yaml. The short form is part of the language. It is a sectioned document that `spec/` defines by numbered translation rules into the canonical documents. These stay unchanged and legal, and they remain the kernel, the wire and the stored form (55 A1).
  - **In spec/**: SCT-002, SCT-006, OBJ-012, OBJ-017, INS-001 and INS-002. An inline definition in the short form makes a template and its instance together, which amends rulings 8 and 18 (INS-001, INS-002, SCT-006). The frame is optional, with the version taken from the manifest, which amends the wording of rulings 31, 35 and 37 and changes OBJ-017. A reference still names its kind, and ruling 19 is kept: the name alone stays allowed where the field accepts exactly one kind (55 A6).
  - **In spec/**: SCT-011. `fleet: <name>` at the top of a file makes the Fleet, which owns every Agent in the file. An Agent written elsewhere states `owner:` (59 A6, 60).
  - **In spec/**: SCT-007, SCT-009, SCT-016 and PKG-031. Grants refer to Conduit templates for reuse (`from: read`). Operation selections follow NAM-018 and NAM-022: qualified references, or `#name` for the project's or package's own operations, with NAM-022's exceptions; never aliases or bare names (25, 53 point 8, 55 A2).
  - **In spec/**: SCT-004, SCT-013 and spec/README.md. Names stay stable across a replacement. An upgrade or a new directory retires the old object before it makes the new one under the same name, accepting a longer window without access, which the rerun measures (55 A3).
  - **In spec/**: spec/README.md, "What the short form's meaning covers". Plan and apply are made of today's requests. The plan is printed and recorded, and nothing runs automatically. In v0 the command is the approval. Each dangerous act takes a named flag, counted when used: `--reopen`, `--recreate`, `--prune`, `--replace`, `--adopt`, the cascade flag, and `--withhold`, which comes with batch 2. Narrowing never waits (53 point 4, 55 A4, 59 A3, 61 B11).
  - **In spec/**: spec/README.md and the sectioned examples. Examples and guides are written in flow style, and block style stays legal. `conduit init` writes a flow-style skeleton (63 point 2).
  - **In spec/**: SCT-012 writes an Interface's scope and an Agent's own-Fleet scope (66 point 1). Fleet-held Pools remain for batch 2 (topic 9).

## 5. Packages, the manifest, contracts, the core package and schemas

- **In spec/**:
  - Addresses, references to exports, pins, versions across a Conduit, and one address per major line: naming.yaml, NAM-001 to NAM-028 (24, 25, 26, 27 NA to NF, 32 G8 and G10, 37 C6, D9, D18 and P6, 41 item 11, 53 point 8).
  - Installing, digests and sources: packages.yaml, PKG-001 to PKG-030, and PRV-017 (24 PD2, PD3 and PR1 to PR8, 27 NB and NF, 32 U11, 37 C35, 38 point 8, 41 items 9, 12, 21 and 29).
  - The manifest and implementation items: manifest.yaml, MAN-001 to MAN-018 (37 C43 to C56, 41 item 25 and change C14, 42 N6 and N7).
  - Contracts: contracts.yaml, CTR-001 to CTR-012 (31 D11, 33 F21, 37 C5, C9 to C12, C19 and D27, 41 item 16, 44 D06, D14 and open point 1).
  - The core package, version 0.1.0, built in at a reserved address: spec/core/ (28 CS10, 37 C17, C18, C22, C23, C30 and C41, 42 N10).
  - The schema profile and the document schemas: schemas.yaml, SCH-001 to SCH-021, spec/schema-profile.md and spec/schemas/ (23 SD1 to SD3 and SR1 to SR5, 33 F23, 37 C1 to C4, D26 and A6, 38 point 12h, 41 item 20, 48 point 3).
- Providers and the Claude and Codex Agent types are self-contained Rust crates under `packages/` through v0's validation. The published package format depends on neither the language nor the repository layout, and an outside author may write a package in any language (40, 47).
- Packages are distributed as versioned release assets over HTTPS, with a small index on the project's site. No optional package is installed by default. A user may pick a prepared flow and fetch its set of packages together (47).
- *Spec pending, batch 1*:
  - **In spec/**: PKG-031. Packages ship the standard Conduit templates of their capabilities, such as `read` and `read-write` for the filesystem package (63 point 3).
  - **In spec/**: spec/README.md, the author's reading path. `conduit install` writes package pins into the author's file (53 point 5).
  - **In spec/**: NAM-024. A property added to a published Type's schema states no default, so the Agent Types' reach flags ship as `claude-code.v2` and `codex.v2`. NAM-024 covers both Type schema properties at any depth and Requirement inputs (55 A5).
  - **In spec/** in part: `fleet-management.v1` is in [capabilities.yaml](spec/core/capabilities.yaml), and `retire-with-agents.v1`, `list.v1` and `lift.v1` are in [operations.yaml](spec/core/operations.yaml) (55 A7, 59, 60). The exact `fleet.v1`, `fleet` and Fleet schema edits are in the core package, landed by order 040; required Agent owners are in the core package, landed by order 041a.
- *Spec pending, batch 2*: the core package gains `make.v1`, the keep and withhold operations, and acting for a Fleet (61 B3, B8, B10 and B11).

## 6. Conduits and access

- **In spec/**:
  - A pairwise Conduit and its template, with one capability and no composites: CON-001, CON-016 to CON-018, CON-027 (17 V3, 31 D7, 33 F20, 37 D25 and A5, 41 item 5).
  - A grant is a Conduit, and every permission, the right to open included, is a Conduit: 27 CC1, 28 CC2; ACC-006, ACC-030.
  - No managed access without an open Conduit, checked at the target. Restrictions only narrow, and a restriction that cannot be evaluated makes the operation unavailable: ACC-001 to ACC-003, ACC-008, ACC-009, ACC-014 (17, 19 RR1, RR6 and RD8).
  - Declared Conduits and standalone Conduits: CON-002, CON-004, CON-012 to CON-014, CON-021 to CON-024, LIF-029 (19 RD3, RD6 and RR4, 20, 29, 32 G6 and G9, 41 item 1).
  - Closing, which never reopens: CON-007, CON-008, CON-011, CON-031, LIF-035, ACC-034, ACC-039, OPS-029 (19 RD1, RR5 and RR15, 29, 38 point 9, 41 items 6a and 17, 42 D6).
  - Nothing gives back access that was taken away: LIF-006, LIF-008 (19 RD2).
  - One authority over an object's whole life, and the decision table: ACC-033 (28 CC13, 30 OL2, 32 A1 and A2, 33 F14, 37 C18, 44 D18). Ownership permits nothing: ACC-028 (29).
  - Delegation through a bounded right to open: ACC-006, ACC-029, ACC-030, CON-028 (17, 19 RD7, 28 CC2, 29, 32 U8, 33 F12 and F13, 37 C32 and X3, 44 D03 and D04).
  - Root: FST-002, ACC-022 to ACC-024, ACC-032 (19 RR11 and RD5, 28 CS12).
  - Onward requests: ACC-016 to ACC-018, IFC-020, IFC-021, IFC-028 (19 RR9, 32 U2, 33 F7, 44 open point 4).
  - Requirements and filters: CTR-005, CON-018, CON-022, IFC-018, IFC-019 (32 U1, 41 items 7 and 14 and change C20).
- Policies are not in v0, and v0 has no outside policy decision point. A limit placed on an open Conduit is a candidate for a later version (A66) (41 items 3a and 3b).
- Narrow provisioning goes through a trusted operation of a Resource, a broker. S1 and S2 measure whether that is good enough; if it is clumsy, a focused constraint, such as a list of allowed templates on access to `management`, comes back to the owner (44 point 2, D03 and D04). Until then, a bound that names the templates it covers is for teams, later (41 item 28).
- Consent on the receiving side is v1, as a new version of `bound.v1` with a consumers item (62 C3).
- **In spec/**:
  - One lifecycle authority for each family, including the owning Fleet for an Agent: ACC-033, CON-024, OPS-014, REV-010 (30 OL2, 59 A1, 60).
  - **In spec/**: root's default Conduits and the closing exception: FLEET-005, FST-002, FST-005, ACC-012, ACC-024 (59 A3). The default and first-start template edits are in the core package, landed by order 040; the other standard templates are in [templates.yaml](spec/core/templates.yaml).
  - **In spec/**: tombstones at register and open, and lifting through bounded access: LIF-040, LIF-041, ACC-053, LIF-009, LIF-035 (53 point 3, 55 A7). The contract `lift.v1` is in [operations.yaml](spec/core/operations.yaml), and `inventory.v1` and `tombstone.v1` are in [results.yaml](spec/core/results.yaml); the Fleet kind and Agent tombstone owner additions are in the core package, landed by order 040.
  - **In spec/**: opening provenance, retained in inventory and tombstones: ACC-044, LIF-013, LIF-040, REC-004 (54 point 3). The list on close, review and marks remain in batch 2; `provenance.v1`, `inventory.v1` and `tombstone.v1` are in [results.yaml](spec/core/results.yaml).
  - **In spec/**: bound items retain resolved IDs: ACC-029 (59 A7). Printing affected bounds in replacement plans remains for CLI order 052b; the guidance is in spec/README.md.
- *Spec pending, batch 2*:
  - An actor other than root creates a Conduit with `make.v1`, an operation of `access`. It makes the Conduit from a registered template, within the actor's bound, and every condition on opening applies to making. This amends 44 point 2 for Conduits only (61 B8).
  - Grant review, as 61 B9 and B10 replace 54 points 2 to 4. Closing a Conduit through which grants were made closes none of those grants. The close lists them, and each is marked, by computation, as granted under a closed authority; later grants under a closed authority are marked too. One pending action asks about each grant, with the answer "closed" or "kept" and no expiry. Keeping is an operation of `access` that a bound can withhold. The grant's own actor never uses it, nor does the holder of the authority the grant was made under, nor anyone acting through a Fleet.
  - `conduit close <grant> --withhold` also refuses that Fleet re-making the same consumer and target until the lift removes it. A Fleet never holds withhold or lift. A withhold only refuses, and permits nothing (61 B11).
- Access limits:
  - **In spec/**: only root opens or registers root-consumed Conduits on its own behalf: ACC-054, ACC-006 (62 C1). `make.v1` remains for batch 2 (61 B8).
  - **In spec/**: bounded reconciliation through `access` and sender-limited reconciliation through Pools and Fleets: ACC-055, ACC-033, OPS-008, OPS-014 and `reconcile.v1` in [operations.yaml](spec/core/operations.yaml) (62 C2, 66 point 4).

## 7. Operations, outcomes, revisions, records and observation

- **In spec/**:
  - Operations, outcomes, the four reasons and their order: OPS-001 to OPS-007, OPS-019, OPS-020, OPS-023 (20 OD1, OR1 and OR2, 28 CS13 and CC5, 32 Q5, 38 point 12d; ACC-037).
  - Idempotency keys and repeats: OPS-008 to OPS-011, OPS-024, OPS-025, OPS-027, OPS-028, OPS-032, OPS-034 (20 OR3, 32 Q1 and Q2, 37 P10, 38 point 12c, 41 items 6b, 6c and 22, P8 and P20, 42 N15, 44 6-1-02).
  - Unknown outcomes and reconcile: OPS-012 to OPS-014, OPS-031 (20 OD2, 32 U4, 37 C39, 41 item 15 and change C33).
  - Revisions and controls: REV-001 to REV-015, CTR-006, TPL-010, TPL-011, TPL-013 (20 OD4, OD5, OR7 and OR9, 31 D6, 32 G11, 33 F10, 37 C10, C20 and P14, 38 point 12b, 41 items 15 and 23 and change C21, 44 D07 and 6-1-03).
  - Model work, the seventh convention, with later work dropped from v0: IMP-001 to IMP-003, IMP-007, IMP-008, ACC-019 (15, 17, 30 NI1 and NI2, 32 U3, 33 F8, 38 point 6).
  - Observation without a reconciler, conditions, and inspection: OBS-001 to OBS-006, LIF-011, LIF-013 (15, 29, 30 DO1 and DO2, 37 C38, 41 item 24b and P19, 42 N13 and N14).
  - Records, record.v1 and purging: REC-001 to REC-006 (19 RR10, 30 RC1 to RC3, 32 Q6 and U9, 33 F4 and r5, 37 C34 and P2, 41 item 24c, 42 N12).
  - Diagnostics, a reduced SARIF result whose `ruleId` is a promise ID: IFC-025 (41 item 24a and P16).
- **In spec/** (ACC-056, IFC-050, OPS-023; 52 points 1 and 2):
  - Refusals come in two tiers, for any caller, an Agent's model among them. Every refusal names the operation and the target, as the caller named them, and says that no open Conduit from the caller to that target lists the operation; a target that does not exist gets the same answer where the caller has no authority over it. A caller that may inspect the target is also shown which Conduits from it to the target exist, open or closed, and what each lists, or that the target does not exist; root is always shown them. This is held by ACC-056, OPS-023 and refusal.v1 (52 point 1, 59 A4).
  - Every result, refusal, diagnostic and notice says what happened and why, in full, unless saying it would disclose what the caller may not learn (52 point 2).
  - **In spec/**: full inventory under object inspection and filtered discovery without counts: LIF-013, OPS-036 (53 point 3, 59 A4). The `inventory.v1` result contract is in [results.yaml](spec/core/results.yaml), and `inspect.v1` and `list.v1`, including their outputs, are in [operations.yaml](spec/core/operations.yaml); Fleet kind additions are in the core package, landed by order 040.
- *Spec pending, with the vectors*: a portable field for the cause of a machine's loss (44, open point 3).

- *Spec pending, the metrics batch after batch 2*: metrics (68 point 1, as 69 amends it). Each Agent and Resource emits the metrics its package declares (name, unit, kind, attribute keys, OpenTelemetry's names where they exist); every attribute, account and model included, is the package's own report, and the package answers for its numbers; the engine attaches only which object emitted a point, and so its Fleet or Pool, and emits its own counts (requests, refusals, durations, turns) the same way under the core package. A Fleet's or Pool's metrics are its members' points merged, summing only points of the same name and identical attributes and never dropping an attribute; gauges are not summed; token types are separate metrics. An Interface reads the metrics of every Fleet and Pool it may inspect; filtering, grouping and charts are the viewer's (CLI tables in v0, export to existing tools); Conduit defines no query language and no saved views (69). Tokens only and no cost in v0; usage is not relayed to the calling Agent in v0; numbers decide nothing (68).

## 8. Lifecycle, Pools and first start

- **In spec/**:
  - One forward-only lifecycle, with the states registered, ready and retired, and no suspended state: LIF-001 to LIF-006, LIF-012 (29).
  - Retirement, shutdown and removal: LIF-005, LIF-014, LIF-017, LIF-024 to LIF-026, LIF-035, LIF-036, PRV-021 (30 OL1 and OL3, 32 G7 and G12, 33 r3, 41 item 17, 42 D11, round 5's M41 as accepted by 58 point 1).
  - Replacement only by an explicit `register-replacement.v1`, and a further setup that leaves finished parts as they are: LIF-009, LIF-029, CON-011, OPS-021, OPS-022 (29, 32 A3, 37 C37, 41 item 1c and P9).
  - Pools, their entries and their retirement: POOL-001 to POOL-017, LIF-020, LIF-021 (21 DD3 and DR12, 27, 28 CS5, 29, 30 OL4, 31 D9, 32 G3 and G12, 37 C14, 41 P14, 42 R5).
  - First start, a fixed set of permanent objects: FST-001 to FST-006, LIF-022, LIF-034 (24 PD4 and PR8, 28 CS10, 30 OL5, 37 C28).
- **In spec/**: root's default Conduits: FLEET-005, FST-002, ACC-024, under topic 6 (59 A3).
- **In spec/**: the unique, fixed SPIFFE trust domain at first start and in status: FST-007, PRV-022, DL6, DL13 (51 point 2). The `status.v1` output is in [operations.yaml](spec/core/operations.yaml) and `trust-domain.v1` in [results.yaml](spec/core/results.yaml); the first-start example edit is in place, landed by order 040.
- *Coordinator's, not yet accepted*: where the run's state is unknown, a retire hook's part is unavailable, not unknown (review list, item 19; work order 027).

## 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach

- **In spec/**: the Fleet family: FLEET-001 to FLEET-006, OBJ-001, OBJ-013, OBJ-020, CTR-004, TPL-001, TPL-002, PLC-006 (59, 60). Core operation contracts are in [operations.yaml](spec/core/operations.yaml) and `fleet-management.v1` in [capabilities.yaml](spec/core/capabilities.yaml); Fleet documents and schema branches are in the core package, landed by order 040, and required Agent owners, landed by order 041a.
  - **In spec/**: family and kind, core template, Type and capability: FLEET-001, OBJ-020 (60). The CLI spelling `conduit fleet ...` remains for order 051a.
  - **In spec/**: fixed Agent ownership: FLEET-002, FLEET-003 (54 point 1, 59 A1).
  - **In spec/**: ownership permits nothing; a Fleet runs nothing, has no placement, contains no Pool and is no machine group: FLEET-004, FLEET-001, spec/definitions.md (60).
  - **In spec/**: retirement conflicts and cascades: LIF-020, LIF-021, REV-014, IFC-025 (59 A2). The flag opening, using and closing a one-time cascade Conduit remains for CLI order 051a (59 A3).
  - **In spec/**: only root registers Agents on its own behalf: ACC-057 (59 A1).
- *Spec pending, batch 2*: a Fleet's authority (61 B1 to B7 and B13). Fleet authority ships in v0, as the second specification batch, after the S1 rerun; batch 1, written before the rerun, holds the Fleet's structure (59, 61 B1).
  - A Fleet may administer a Pool other than `system`, never listing `register`, `register-replacement` or `retire-with-resources`. It may hold a bounded right to grant, never unbounded, never listing lift, keep or withhold, and its bound naming none of the excluded operations nor acting for a Fleet nor `get-secret`. Root, or any actor whose own right covers them, gives these (B2).
  - An actor uses what a Fleet holds by forwarding. It asks the Fleet through its own act-for Conduit, and the engine, as the Fleet, makes one onward request through the Conduit named. Two linked records result (B3).
  - Any Interface that root appoints may act for a Fleet, with its own token, and may bridge several Fleets, marked as bridging them. A Fleet's own Agents act only for their own Fleet. Only root opens or makes an act-for Conduit, and closing one follows the ordinary rule; and only root drives, sets up or retires an Agent that acts for a Fleet. Appointing an Agent that others already hold Conduits to is refused, and the refusal names them (B4 to B6).
  - A grant made through a Fleet goes to the Fleet itself, to one of its Agents that acts for no Fleet, or to an instance it sets up. It targets a Pool other than `system`, or a Resource. It never lists what starts model work, `get-secret`, an operation reserved to root, `register`, `register-replacement` or `retire-with-resources`. A Fleet closes only the grants it sent, and reconciles only its own invocations and its Agents' (B7).
- **In spec/**: tool descriptions name the target and its Type and carry the Type's description: IFC-033 (53 point 3).
- **In spec/** in part: Pool inspection shows every administrator (LIF-013; 62 C5), through `inspect.v1` in [operations.yaml](spec/core/operations.yaml) and `inventory.v1` in [results.yaml](spec/core/results.yaml). The notice pending action is PND-007 and CON-008; shared-Pool guidance remains for order 055; no rule of one administrator is added.
- *Open, after the S1 rerun*: 62 C6 to C12. These are people other than the owner acting for Fleets, registration through a Fleet, names per Fleet, a Fleet setting what an Interface may use, records filtered by Fleet, root's tool count, and reach beyond one hop.
- **In spec/** in part: scope is computed from open inspection Conduits and shown in inspection (spec/definitions.md, LIF-013; 66). The `manages:` notation is in SCT-012; the standard scope templates are in [templates.yaml](spec/core/templates.yaml), and `inventory.v1` is in [results.yaml](spec/core/results.yaml). Sign-in and pending actions remain for 038a; Fleet-held Pools remain for batch 2.
  - **In spec/**: SCT-012 covers an Interface and a Fleet's own Agent for its own Fleet. Fleet holders for Pools remain spec pending for batch 2. An Interface, or a Fleet's own Agent for its own Fleet, or a Fleet for Pools, writes `manages: {fleets: [...], pools: [...]}` with the role manage or view. It expands at apply into ordinary Conduits, made from standard templates and opened by root, and inspection shows them back as a scope. There is no wildcard, and `system`, `management` and `access` are never members.
  - Manage covers listing, inspecting, setting up and reconciling the scope's Agents and Resources, administering the Pools its Fleets hold, and acting for its Fleets within ruling 61. It never covers creating anything, retiring members, the cascades, `system`, use or grants of its own, turns, appointing, keeping or withholding, or secrets. Root may write explicit turns for the owner's own consoles.
  - There is no Scope object and no grouping shorthand. An Interface is the named set. Scopes do not replace what a Fleet holds.
- **In spec/** in part: pending actions, PND-001 to PND-007 and PND-009 to PND-012, IFC-049, LIF-013 and DL19 (50 point 3, 51 point 5, 56, 66 point 3). Enrollment kinds remain for 038b; grant review and grant answers remain for batch 2 (61 B9); peer delivery waits on R3.
  - Every action waiting on a person is one mechanism, raised at an Interface, which is a control panel. It carries a kind, a subject, typed evidence, an expiry and an answer. Each evidence item has a kind (`code`, `fingerprint`, `attestation` or `fact`), a value, where to find it and what it proves. Only the engine shows the evidence, never through a model's output.
  - A pending action is raised at root and at every Interface whose scope holds its subject. What an Interface may answer is limited by what it holds: it completes sign-ins only for its scoped Fleets' Agents (PND-010). Grant closing and keeping answers remain for batch 2; the ruled limits are closing a grant always and keeping one only with keep. Enrollment and anything touching a secret stay with root. This amends rulings 49 and 50 (66 point 3).
  - Approval by people other than root comes later (55 A4).
- **In spec/**: sign-in, PND-005, PND-010, PND-011, CTR-013, IFC-007, IFC-016, IFC-045, IFC-049; state directory PRV-029, MAN-018, DL16 and names.yaml (49, 56 point 7, 66 point 3). Agent client-specific homes and login choices remain for provider order 050.
  - Sign-in is part of the language. The engine passes a URL elicitation, and no other kind, to a person's Interface, never to an Agent. A client's own login reaches the person in the same shape.
  - The account that each Agent's home or Resource is for is declared in configuration. A sign-in to another account is undone under PND-011; the attempt fails. A lapsed login raises PND-005, and model work reports unavailable naming that action.
  - Evidence follows PND-004 and PND-005: expected account and device user code, with code values only in the engine panel and recorded answer, never wire results.
  - The Claude Code and Codex Agent types give each Agent its own home, made and removed with it, with its own subscription login; Codex stores its login in a file there, and the engine reads none of it. Codex signs in by its device flow, and Claude Code by its own browser callback on the engine's machine, never by a pasted code (48 point 2, 49).
  - Other input requests are not relayed; URL actions within sign-in use IFC-049, and mid-call input_required ends unknown under IFC-016 (41 item 27, 49).
- What Agents reach (48 point 1, 53 point 7, 55 A5, 61 B13):
  - What an Agent reaches through its own login is off by default, and a template input turns it on for one Agent. An account service that Agents should share becomes a Resource behind a Conduit.
  - *Spec pending, batch 2*: an Agent that an actor can take turns on is never given a `get-secret` Conduit, and no actor is given a turn on an Agent that reads a secret. Inspection and plans show, one hop deep, what each turn-holder other than root reaches.

## 10. Placement, nodes, machines, identity and enrollment

- **In spec/**:
  - Placement: only Agents and Resources carry one. It is a standard argument, fixed for the instance's life, and interpreted only at the placement: PLC-001, PLC-002, PLC-005, PLC-006, PLC-009, PLC-020, PRV-015 (7 PL3, PL7 and PL8, 8 for PL2, 41 item 8c, round 5's M22 as accepted by 58 point 1).
  - Nodes registered from what the machine provider reports, and `local`: PLC-008, PLC-011, PLC-015, PLC-026, PRV-013, PRV-014 (7 PL1, 28 CC13, 37 D31, 40 point 3b, 41 item 8d, P7 and change C58, 42 N10).
  - Placement needs as inline `nodeSelector`s, and `shared-host.v1`: PLC-007, PLC-013, PLC-025 (7 PL4 and PL5, 19 RR16, 37 C33, 41 items 8a and 8b and P12, 42 N8 and N9).
  - Unreachable, gone and declared lost: PLC-017, PLC-019, PLC-023, PLC-024, PLC-027, LIF-010 (7 PL8, 32 A2, 33 F16, 41 item 8e, 42 N11).
  - One control plane, failover, and at most one part acting on an invocation: ACC-036, ACC-037, DL17 (7, 40 point 3f, 43 Q2).
  - Identity, gates, and the engine's own user inside the domain of trust: IFC-022, IFC-041, IFC-042, PRV-018, PRV-022, PRV-025, PRV-028, DL13, DL16 (32 U10, 33 F2, 40 point 3e, 41 item 26a, 42 D17, 43 D5, D20, Q1 and Q5, 51 point 1).
- v0 must not rule out local enforcement. A robust local enforcement is likely a v1 requirement, and A65 records v1's shape (40 point 3f, 43 Q4).
- No support for remote machines is claimed before a test across two machines. S3 runs on real machines, and model checking under loss, reordering and partition is required for v0 (5, 47; A40, A74).
- Ruling 5's order after placement, (b) separated components and serialized messages under a simulated network, then real processes, then two machines, is replaced by model checking of the safety promises (A40) and S3 on real machines (47, 68 point 4).
- Fork-from-checkpoint when an Agent moves to another machine (7 PL8) is type-contract material: an Agent Type may offer it, and the language promises nothing (68 point 4).
- **In spec/** in part: [network limits](spec/README.md#pending-actions-sign-in-and-the-network) (51 points 1, 4, 6, 7, 9 and 10). Managed enrollment remains for 038b; peer engines (51 caveat, 62 C4) are written in batch 2, with appointment, keep and withhold; the Interface a peer admits counts as root for ACC-054, ACC-024 and 59 A1's "only root registers Agents", and not for the operations reserved to root (68 point 3).
  - A machine is eligible to be a node because its engine's owner controls it, not because of where it is. Another party's machine is only ever a peer.
  - Both modes are supported. A managed machine is enrolled as a node. A peer runs an engine of its own that decides requests into its own objects. Where both engines are the owner's, the peer may admit the owner's root Interface with Conduits to its `management` and `access`. The peer's root template then names that Interface for its defaults, appointment, keep, withhold and pending actions.
  - **In spec/**: [network limits](spec/README.md#pending-actions-sign-in-and-the-network), including two halves: a relationship across parties is two halves, each opened and closed by its own engine, over `https` or an outbound connector behind a provider, never a shared overlay. There are no capability tokens in v0. Across parties, v1 uses OAuth tokens bound to the receiving engine.
  - **In spec/**: [network limits](spec/README.md#pending-actions-sign-in-and-the-network) hold the caller-chain and protocol choices (51 points 9 and 10).
- *Spec pending, needed before S3*: enrollment (50, 51 point 8, 52 point 4, 56). This changes PLC-025 and PLC-026.
  - Enrollment is an operation of the machine provider's capability. Conduit opens it, and a person completes it, through the provider's link that carries no secret, where the provider has one.
  - Only root starts and admits enrollments, and machines stay outside Fleets. A joined machine becomes a node only once a person confirms it.
  - The provider declares its verification method. Whatever the method, the engine enforces a minimum. Admission binds the machine's stable `id`, which is required, opaque, scoped to the provider and derived from the machine's key where the provider offers one, and the enrollment, and at least one evidence item comes from the machine's key or from attestation. Customizing only raises the minimum.
  - The evidence is recorded with the answer. In v0 a person compares a fingerprint of 6 to 8 characters; a QR code is a later option, and for peers in v1, the fingerprint of the trust root, with PAKE codes or short authentication strings to be studied then. The person admits a machine by typing the fingerprint read on it, and the engine compares (68 point 3, R2). A machine with no screen needs attestation, or the explicit flag `--accept-weaker-check`, which is recorded.
  - A node is bound to the provider's machine `id` as well as its hostname. The enrollment names any labels beyond the descriptive ones, and the provider binds them.
  - The join secret appears only in the action shown at root, never in a record or in a model's context. Refusing an enrollment removes the machine at the provider. Declaring a node lost asks the person first.
  - Tailscale is a candidate machine provider, and its evidence is a short fingerprint of the node key, with hostname, os, arch, labels and id; A74 confirms that both ends can show the node key. A Windows computer running WSL2 reports `linux`. Setting up a second machine by hand is for testing only (A74).

## 11. The implementation interface, providers, hooks and items

- **In spec/**:
  - An implementation is an MCP server, with one tool per operation. The baseline protocol revision is the one IFC-005 names, and only a new version of the specification raises it: IFC-001 to IFC-005 (31 I1).
  - Conduit's keys travel as one MCP extension with `revision: v0`, and a request without it reports unavailable: IFC-006, IFC-008, IFC-009, IFC-037, IFC-038, IFC-047 (31 I2, 32, 33 F1, 41 items 1b and 18, 42 N16 and N17).
  - A plain server's answers mapped to outcomes, and how an implementation answers: IFC-009, IFC-010, IFC-025 to IFC-027, IFC-045 (31 I3, 33 F1 and F6, 37 D21 and P9 to P11, 41 item 6b, 44 6-1-01).
  - The server presented to each consumer: IFC-012 to IFC-014, IFC-023, IFC-033, IFC-038 to IFC-040 (31 I6, 32 Q7, 33 F5, 37 D3, 41 changes C51 and C52, 44 D07 and 6-1-04).
  - Handles, and instances kept independent: IFC-011, PKG-016 (24 PR7, 31 I4 and I5).
  - Templates, the `implementation` map, run items and hooks: TPL-001 to TPL-004, TPL-012, TPL-017 to TPL-019, OPS-028 (14, 31 D1, D8 and D10, 38 point 12g, 40 point 3c, 41 items 1 and 1a, P3 and P20).
  - Providers as packages implementing core provider capabilities, with the runtime provider in the shape of CRI: PRV-001 to PRV-028 (40 points 1, 2 and 3a, 41 items 2, 2a, 2b and 31, P4 to P6, P13 and P18, 42 N3 to N5, N20 and D8, 43 Q3, 44 6-1-05).
  - Implementation items: MAN-009, MAN-010, MAN-017, IFC-041 to IFC-044, SEC-024 (37 C51, 41 items 25a to 25d and P15, 42 N6 and N7, 43 D5).
- Identity, isolation, record storage and the engine's state may become swappable providers, under topic 1's rule of swappability; v0's provider contracts are runtime, machines, package source and secrets (40 point 2). In v1, fetching a package by digest is a feature that a runtime provider declares (43 D20).
- *Spec pending, batch 1*: a tool's description names its target and the target Type's description (53 point 3).
- *Open*: binding one occurrence of a repeated argument waits for a real package that needs it (44, open point 2).
- *Open*: the error rule for a call to an unknown tool is revisited when MCP settles SEP-2145 (38 point 11; 41 item 6b settled the other half).

## 12. Secrets

- **In spec/**: secrets.yaml, SEC-001 to SEC-024, and PRV-011 and PRV-026. It holds references in place of literals, one Type per store offering only `secret-provider.v1`, the one read path through the engine, who may open a reading Conduit, recording without the value, and no store in the engine (38 point 1, 39 points 2 to 4, 41 section 4 and items 2c, 4a to 4c, 13a, 13b and 19, 42 N1, D10 and D12, 43 Q3).
- The limits of the guarantee are stated as limits, not promises: the domain of trust (spec/README.md, "What the guarantee for secrets covers"; 41, 42 N19, 43 Q5).
- The macOS keychain is among the first packaged secret providers, starting with S2 (46 point 5).
- What is specific to one store goes in that store's contract (41). A file store ships with the reference engine, and a store's own credential is itself a secret Resource (41 items 13a and 13b; A63).
- *Spec pending, batch 2*: the rule on secrets for an Agent that others can drive, under topic 9 (61 B13).

## 13. Proof, the build, experiments and measurement

- Sufficiency must be provable to a user of the open-source protocol. The methods are the promise register, a reference model, vectors, mutation testing, model checking, independent reading and builds (10).
- The first build is one Rust codebase. Its pure, deterministic core cites a promise for every decision and is driven by vectors. A thin shell around it grows into the reference engine, and S2 to S9 run on the same codebase. It uses existing crates, and its vectors live in conformance/ (10, 44 D22, 46 point 1, 47).
- v0 is the reference engine (47). To pass, it must meet all of the following:
  - S1 to S10 run end to end, with their vectors passing.
  - The engine runs Claude Code and Codex as Agents through S1's and S2's tasks, with a record for each request. Simulated clients alone do not pass v0, and ruling 46's fallback of two Claude Code instances is void for v0 (68 point 4).
  - Every safety vector passes. A safety vector is one whose failure would be an unsafe finding as the tracker defines it; each carries a field naming the safety promise it tests, and each of S1's proposed safety traces has at least one or is listed as missing; the manifest's group tags are not safety tags (68 point 2). *Spec pending, with the vectors*: the field and the tagging.
  - The author measurements, against the baseline (A70), take less of an author's work than the baseline on most of S1's tasks and much more on none.
  - Model checking shows the safety promises hold under loss, reordering and partition (A40).
  - The subscription terms allow the engine to run the native clients headless on a subscription, or v0 names the alternative it uses (A8).
  - The observational probes do not block. A7's findings go into v0's public security claim.
- The experiment program has ten scenarios, as later rulings amended it, with native probes run alongside the language work (9, 40). Every experiment pre-registers what it must show and how each measure is counted (45). A provider contract is shown swappable by two implementations on different tools that pass the same vectors (40; A63).
- Each run measures what an author must do (44 D22, 46 point 3, 48 point 4, 52 point 6, 53 point 9, 63 point 1).
  - It counts files edited, concepts met, grants made by hand, adapters written, and recovery steps that need a person.
  - The success criteria are fixed before the run. Those the owner accepted: a competent reader completes the checked S1 path without reading ruling history or editing a provider; S2's restrictions differ without copying the shared credential; closing one Agent's access leaves the other working; the user resolves uncertainty without guessing that an effect failed. Observed time and mistakes are recorded before any numeric target is set (46 point 3).
  - Every interval without access is listed (52 point 6, 55 A3).
  - A person's actions at the panel, such as a sign-in, are listed apart from the author's lines, as root's Conduits are, with the strict count beside; `account:` lines count as authored lines (68 point 3, R1).
  - A fresh Codex author gets the specification, the tasks and the CLI's format reference.
  - Pins that `install` writes, shipped packages and standard templates count as tool or provider work. Digests copied by hand count as lines and are listed apart, and root's Conduits are listed apart from grants to Agents.
  - A command counts as a line only if it carries a value the author chose. A line removed and restored within one task counts once. The block-style and strict readings are reported beside the primary one.
  - S1's Conduit keeps its local name `project`, for comparison with the baseline (60).
- Independent builds inform the owner's judgment and do not gate it (11, 12, 14 LD4, 38 point 4, 47).
  - They cover the language, the conventions and the interface. There are two, by different vendors, one a Codex session and one a Claude session, and neither checks the other's work; a second full engine is optional later. Builders are fresh sessions that have only the specification and the vectors, from an explicit build brief, each on a separate branch with isolation enforced by the checkout (`build-base`), and a neutral party compares the results (10, 12, 14 LD4).
  - The session that wrote the language builds no independent engine; the first build is written beside the language (11, 46).
  - They wait for G1 to G11 and the owner's sign-off. Convergence means no unsafe finding in the latest round of readers, with the model and the vectors agreeing with the promises.
- Exploratory code needs no permission, and all code is Rust (3). The one exception is the Python checkers in `tools/` (the editors' fix B4 of round 2, accepted by 39 point 6; 34).

## 14. Publication and organization

- Publication starts only after v0 passes and the owner, judging the complete evidence, finds that Conduit works and is worth publishing (47; A75). Dated results, the method as run, the prompts and the counted artifacts stay in the tree as v0's evidence. Raw runs leave it, with links to them (58 point 5).
- Stewardship is independent and open source, and the license is Apache-2.0. The owner has final authority over the language's direction, designates official packages and appoints their maintainers (47).
- The repositories (47; A76):
  - The main repository owns the specification, the package format and its contracts, the conformance cases, the engine and its CLI, the roadmap, and the site.
  - After the organization moves, each official integration has its own repository, Claude and Codex first, with a pinned copy kept in the main repository for its vectors. A `.github` repository holds the shared guidance.
  - Community packages belong to their authors.
  - An official package names its maintainer and support contact, states which engine versions it supports, and tests installation and use against a released engine.
- The first public preview needs one released engine and CLI, and one official integration released on its own (47).
- If an official package's maintenance lapses, the owner appoints a successor or marks it unmaintained. A maintainer ships routine fixes; a change to a shared contract goes through the main repository. The check of package authoring includes an implementation not written in Rust (S7). One website serves engine users and package authors equally (47).
- Continuous integration runs only on free runners (64 point 3). The organization's own items wait until the organization exists (64 point 5, 65).

## 15. How the work is governed

- The two reference clones that ruling 1 kept are removed when convenient, by the owner, on the owner's machine (1, 68 point 4).
- The external audit report is the basis. docs/AUDIT.md holds the action items and their priorities, and ROADMAP.md shows the order of the work and each experiment's state (1, 45).
- Who decides (37, 38 point 13, 46 point 4, 48):
  - The owner rules where settling a finding would change what Conduit is for, or would add or remove a feature.
  - The coordinator, the admin session, settles the rest. It plans, decides, reviews and merges, and logs each decision for the owner to strike.
  - Codex sessions carry out work orders. Fresh reads by Codex test what a reader without context can do.
- Every decision the coordinator, an editor or a work order logged before ruling 57 is accepted. The exceptions are items 15 to 19 of the review list and increment 4's open decisions, which stay open to the owner until reviewed (39 point 6, 42, 58 point 1).
- *Coordinator's*: decisions logged since ruling 57 stay open to the owner to strike. Ruling 67, which the owner delegated, the owner confirmed (68 point 5).
- The work orders run through Conduit (A78) once Fleet authority is built. Root registers and sets up fixed worker and tree slots once, and dispatches orders 1 to 3. A coordinator Agent of the conduit-core Fleet, appointed by root, dispatches orders 4 and 5. Each Fleet has one queue, and nothing retires a slot. The coordinator Agent grants each order's tree through its Fleet's Conduits and drives workers through turns root opened (48, 61 B12).
- Each piece of work has one branch and one pull request, committed and pushed at each milestone, squash-merged once its checks pass (36, 38 point 14, 46, 48). A proposed shape is checked against the consolidation list before it is offered (25, 39).
- Accepted decisions may still be challenged by Codex or any later reader (39 point 6). Decisions that changed an earlier ruling become recorded amendments (58 point 1).
- The standard for records is relevance, not a perfect record. A document that does not bind may be edited, condensed or removed once every piece of relevant Conduit information in it is preserved in `spec/` or a live document. Git keeps every version, and an index lists the removed files (57, 58 points 6 and 9).
- When a batch, an increment or an investigation closes, its working papers condense into the registers and the records. A settlements register replaces the logs of closed increments, and the work-order record replaces merged orders (58 points 3, 4 and 10).
- Rulings are kept as four documents, one question each (67):
  - `spec/` binds behavior. This file holds every other decision in force. The ledger holds what was ruled. A record holds why.
  - A record opens with its status line. The owner's words and the decisions as ruled are never rewritten, and a change goes in the later ruling's record.
  - A ruling number is given only when the owner decides something new. A confirmation or clarification goes into the record and the ledger row of the ruling it clarifies.
  - One change carries a ruling everywhere: its record, its ledger row, the rows and status lines of the rulings it amends, the lines of this file, and any action item.
  - No register re-keys the decision rows that the specification cites (58 point 2), and this file uses no identifiers of its own.
  - AGENTS.md keeps only what every session needs, near 150 lines, and names this file by link.
  - A check in `tools/` verifies the ledger, the status lines and that this file holds no dates.

## 16. Waiting for the owner

- What waits for the owner is listed in [docs/AUDIT.md](docs/AUDIT.md#waiting-for-the-owner), with the open lines above. No session settles those questions.
