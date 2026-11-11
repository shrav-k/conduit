Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# What S2 to S10 and the owner's real use need from the language: a checklist for a short form (ruling 4's scope guard)

## Key points
1. Ruling 4's scope guard means a short form is checked against S2-S10 and the owner's real use, not only against S1. Round 6 names four things that must survive: multi-machine work, multiple owners, provider swaps, and agents managing the system (preservation tests 5 and 8).
2. S2 (Linear, key in 1Password) needs:
   - two secret Resources of different store Types, the 1Password store reading its own service-account token from a keychain or file secret;
   - two root-only grants listing get-secret.v1 (SEC-008), and secret references with isSecret bindings (SEC-001, SEC-021);
   - grants with different operation lists, a filter with per-grant arguments (which needs a Conduit-aware adapter, D01 and D05), and an Agent's grant to `access` with a bound (E07);
   - a declared Conduit for onward "dependent calls".
3. S3 needs:
   - a machine provider owned by `system`, a runtime provider per remote machine, `placement` on instances or `nodeSelector` on Types and items, and shared-host.v1;
   - for ruling 51's peer mode, an Interface standing for another engine's actor and a trust domain. The language has neither today.
4. S4 needs Pool entries that refer to each other and may be optional, setup and retire hooks (`requires`), effect marks on operations, and an operator grant that lists reconcile.v1. Its retry is set-up again; nothing rolls back (LIF-027).
5. S5 needs the template's `controls` map with input narrowing (TPL-010), config defaults (E16) and grants that list a control. It forces the question of who owns a setting once a live control changes it: the file or the object (critic gap 5).
6. S6 and S8 have no round 6 evidence cases. S6 needs:
   - removing a line to mean an explicit close or retire, never pruning without a flag;
   - tombstones enforced by the engine and keyed per grant, so a later apply cannot reopen;
   - retire hooks and task-capable operations.
   S8 needs filters with per-grant arguments carried onward through a declared Conduit, `supports` on both implementations, and `records` flags set to false.
7. S7 is mostly contract and package authoring: Type, Capability, Operation, Requirement, a manifest, plain and Conduit-aware implementation items, `nodeSelector`, exported templates, and isSecret bindings. A short form limited to projects leaves two normative syntaxes.
8. S9 is mostly engine configuration (trust domain, hostname, the list set on the runner, the store) and requests: set-up again on an instance that is ready, reconcile, replace, declare-lost, unregister. A desired-state short form must keep three states apart: the file, what is registered, and what is observed. It must never recreate or repair on its own (OBS-004, LIF-008).
9. S10 and E13 (rotation) are cascades of replacements, because runtime, locator and placement are fixed for life (PRV-006, SEC-005, PLC-009). A new secret means a new service Resource and re-pointed grants for both Agents. A new runtime means replacing every instance, and a replaced Agent loses its home and login, so it must sign in again (ruling 49). A normative form must say whether the cascade is part of the language's meaning.
10. The real use, 10 workers:
    - 20 objects and about 50 grants, with each worker needing its own working tree, `main` read-only, the queue, a turn grant from the admin, and the admin's and verifier's read grants on its tree;
    - about 340-420 lines today on paper, against a floor of about 1 line per worker.
    - The language today cannot instantiate an Agent and its own Resource together (a Pool makes only Resources; an Agent template makes only Conduits), and cannot declare grants whose consumer is the admin inside the worker's template (CON-002).
11. Reuse at scale is constrained by ruling 8: no interpolation (IN5), and a Type is never an input (IN2), so Codex and Claude Code workers need two templates. StatefulSet-style replicas, Compose `extends` and Terraform for_each are the existing patterns. Names derived by interpolation would need the owner.
12. Package upgrades at scale replace every worker (INS-020), and each replacement brings ten new sign-ins (experiment 010: a Codex login cannot be copied between Agents). Agent-driven creation of workers needs either broad `management` trust or a broker (D04, preservation test 5).
13. The S1-only formats (design E, synthesis) cannot express filters, grants from Agents to `access` or `management`, Agent-to-Agent grants, Interfaces other than root, placement, more than one Pool, secret references, runtime, hooks, controls, records flags, expected accounts, or composites at scale. Their escape hatch cannot follow names compiled across replacements.
14. Several constructs that S3, S9, S10 and the real use rely on have no written promises yet, so no short form can be checked against them now: enrollment (decision 5 of ruling 52 open), pending actions and sign-ins with the expected account (rulings 49 and 50), peer mode (ruling 51), the five additions of ruling 53, the two-tier refusal (52.1), and complete filter contracts (D05).

## Findings
Repository: <home> I only read it and changed nothing. Sources: ROADMAP.md (scenarios table lines 131-142; round 6's cases lines 160-181), docs/proposals/reaudit/round-6/design-critique.md (the first useful journey lines 88-107; D01-D22; the E-cases lines 663-704), docs/proposals/multi-machine/README.md (its own scenarios at lines 273-417, called MM-S1 to MM-S9 here so they are not confused with the roadmap's S-numbers; "What a user writes" in section 7, lines 930-977), docs/proposals/secrets/README.md (the one-page proposal, lines 538-716), spec/promises/secrets.yaml, docs/proposals/ruling-48/49/50/51/52/53/README.md, docs/proposals/sign-in/README.md, docs/proposals/enrollment/README.md, docs/proposals/owner-direction/2026-11-10.md, experiments/010-subscription-terms/results-2026-11-10.md, and docs/proposals/simplification/research/{critic.md, design-E-layered.md, map-constraints.md, map-floor.md, synthesis.md}. Promise IDs are from spec/. Every line count below was estimated on paper. None comes from a run.

## 0. The rule this checks, and how to read it

Ruling 4 (docs/AUDIT.md:550): "Claude and Codex collaborating on one project on one machine is the starting acceptance scenario; success there must not narrow the design below its broad scope." The scope guard (docs/archive/plan-audit-history.md:136-147) adds that passing a scenario removes nothing, and that the model is checked against S3-S9 before any pruning. Round 6 (design-critique.md:656-661) names four things that must be preserved: multi-machine work, multiple owners, provider substitution and agent-driven management. Preservation test 5 (docs/AUDIT.md:487): "Every management action is available to an agent under a grant, and delegation only narrows." Test 8: "No rule assumes exactly one owner: a human's access is an Interface holding grants."

The constructs fall into four kinds. Only kind A is what the "short form" would replace, but a normative short form has to say how it relates to the other three.
- A. Project authoring: instances, grants, ownership, placement and per-instance settings.
- B. Contract and package authoring: Types, Capabilities, Operations, Requirements, manifests and implementation items.
- C. Requests: open and close, set-up, retire, replace, reconcile, controls with a revision, idempotency keys, expectedId, install, declare-lost, purge, and enroll (proposed).
- D. Engine configuration, outside documents: trust domain, hostname, the list set on the runner, and the store.

## 1. The construct inventory (labels used below)

Kind A, documents:
- K1 Pool. Owns Resources; there is exactly one owner, fixed for life (POOL-001). Its entries are nested Resource instantiations (POOL-002), may refer to each other, and the setup orders them (POOL-006, POOL-013). An entry may be marked optional, and a template input supplies an entry's placement and runtime (POOL-009). Agents have no owner (the instance schema in spec/schemas/instances.yaml), so a Pool can never make an Agent.
- K2 Resource instance: `from`, `owner`, `placement`, `runtime`, `arguments`.
- K3 Agent instance: type and config, which includes ruling 48's reach flags `accountServices` and `peerMessaging` and `model` (packages/agent-claude-code/contracts.yaml); `placement`; `runtime`.
- K4 Interfaces other than root (test 8). An operator Interface is optional in S1 but needed for multiple owners.
- K5 A standalone grant (Conduit): one consumer, one target, one capability, and an explicit list of operations (CON-001, ACC-003). The pairs the scenarios need are:
  - Agent to Resource
  - Interface to Agent, for turns
  - Agent to Agent, for turns
  - Interface to Pool, with pool-management operations
  - Resource to secret Resource, listing get-secret.v1 (only root opens this, SEC-008)
  - Agent to `access` with a bound (ACC-029, ACC-030)
  - Agent to `management`, which is broad trust (ACC-030)
  - Agent to a broker Resource (the broker row of definitions.md; D04)
- K6 A declared Conduit: the template's `conduits` map. Its consumer is always the new instance (CON-002). Its target or arguments can come from an input (INS-012), and it is opened as part of the setup (TPL-012).
- K7 Requirements on a grant:
  - a filter with arguments (CTR-005), which the target's implementation must list in `supports` or the grant is not made (CON-022); it applies to the call and to what the call returns (IFC-019), and it is carried onward (ACC-017);
  - bound.v1 {pools, targets, operations}, on `access` only (CON-028);
  - shared-host.v1 (PLC-013, CON-006).
- K8 Settings and controls: a config literal; input defaults and Type-schema defaults (INS-019, SCH-009); the template's `controls` map, which selects controls and narrows their inputs (TPL-010); and the narrowed schema that the tool shows (IFC-033).
- K9 Templates with inputs, for reuse. Inputs may set only config, a declared Conduit's target or arguments, an entry's placement, runtime or arguments, a Conduit template's `operations`, or a requirement's arguments (INS-012). There is no interpolation or expression (ruling 8, IN5; OBJ-019). Structure, including `type`, is never an input (IN2). A template is replaced with a project context (SUB-009). Existing instances keep what they resolved (INS-020).
- K10 The implementation map and hooks: Type defaults and template overrides (TPL-003, TPL-004); setup and retire hooks through the Type's `requires` (TPL-012, TPL-017, TPL-018).
- K11 Placement: `placement: <node>` on the instance, fixed for life (PLC-001, PLC-009), or a label selector `nodeSelector` on the Type or implementation item, never on an instance (PLC-007; multi-machine section 7). A value such as a path is tied to its node (PLC-020).
- K12 `runtime: <runtime-provider Resource>`, fixed for life (PRV-006). Without it, the engine's own runner runs the instance, on `local` only (PRV-019).
- K13 Provider Resources:
  - a runtime provider;
  - a machine provider: at most one, owned by `system` (PRV-013);
  - a package source: owned by `system`, claims `schemes`, and lists no control over them (PRV-016);
  - a secret Resource: its locator is its config, and it has no controls (SEC-005).
- K14 Secret references at places marked `secret-reference.v1` (SEC-001). A literal or a bare name there is rejected. A variable or header marked `isSecret` must be bound to such a place (SEC-021). Values are read before a run or a connection (SEC-010, SEC-013), and a header carrying a secret goes only to https (SEC-024).
- K15 Manifest pins, version and digest (NAM-006), and `fetch` (MAN-006).
- K17 The Type's `records` flags for inputs and outputs.
- K18 The expected account of an Agent's home or of a Resource. Ruling 49 accepted it. It is not yet written.
- K19 The additions ruling 53 accepted, not yet written:
  - `as`, a local name for a standalone Conduit;
  - descriptive tool descriptions;
  - an inventory in inspect.v1;
  - tombstones enforced by the engine;
  - expectedId on open.v1 and close.v1.

Kind B, contracts:
- K16 A Type has `family`, `schema`, `offers`, `controls`, `requires`, `implementation`, `nodeSelector`, `records`, `conditions` and `description`.
- An Operation has `inputSchema`, `outputSchema`, `effect`, `startsModelWork`, `deduplicates`, MCP hints, and `changes` for a control.
- Capability, Requirement (its `inputs`) and Schema.
- An implementation item has `implements`, `command` or `package` or `remote`, `plain`, `tools`, `supports`, `nodeSelector`, and the bindings `environment`, `packageArguments` and `headers` (MAN-010).

Kind C, requests:
- R1 idempotency key (OPS-009, OPS-012)
- R2 revision (REV-002)
- R3 expectedId (LIF-037; open and close under ruling 53)
- R4 reconcile.v1 (OPS-014)
- R5 set-up (also again on an instance that is ready), retire, retire-with-resources, unregister, replace, register-replacement (LIF-009)
- R6 open and close
- R7 install
- R8 declare-lost (PLC-017)
- R9 purge
- R10 enroll (ruling 50; its design is held on decision 5 of ruling 52)
- R11 tasks and cancel (IFC-016)
- R12 pending actions for a person: sign-in and enrollment, raised at the root Interface (rulings 49, 50, 51.5)

## 2. Authoring sketches, in neutral terms

### S2. One service login shared by two Agents with different restrictions
Roadmap: "Requirements, Conduits, shared authentication, dependent calls". The owner's direction makes it Linear, with its key in 1Password through a service account limited to a test vault (owner-direction/2026-11-10.md).

What the author declares:
1. Pinned packages:
   - a Linear service package: a Type with a secret-reference place `token`; operations to list, get and search issues, and to create, update and comment;
   - an implementation item: either a remote https server with an Authorization header marked `isSecret`, or a stdio server taking an environment variable;
   - a 1Password store package: a secret-provider Type with an `op://vault/item/field` locator and a secret-reference place for its own service-account token;
   - a bootstrap store, keychain or file, for that token;
   - the two Agent packages.
2. Ownership: at least one Pool. A separate Pool for credentials is likely, so that authority over secrets is apart from authority over tools.
3. Secret Resources: `op-token` (keychain locator) and `linear-key` (1Password locator, whose token place refers to `op-token`). The store reads its own credential under SEC-010, as the header comment of spec/promises/secrets.yaml says.
4. A service Resource `linear` with config `token: -> linear-key`.
5. Two root-only grants listing get-secret.v1: `linear -> linear-key` and `linear-key -> op-token` (SEC-008, which only root opens).
6. Two Agents, a reader (Claude Code) and a writer (Codex). Reach flags are off, and each names its expected account (K18).
7. The grants:
   - reader -> linear [list, get, search];
   - writer -> linear [the same, plus create, update, comment].
   - A plain server is enough for operation lists alone (D01 table). Any narrower limit, such as one team or a deadline (E02), is a filter with arguments. It needs a Conduit-aware adapter whose implementation `supports` it (D01, D05).
8. Dependent calls: an adapter Resource that serves an Agent by making onward requests through its own declared Conduit to `linear`. Those requests carry the caller and the filters (ACC-016 to ACC-018). The term traces to the collaborator review C2 (docs/archive/collaborator-proposals/REVIEW.md:35).
9. Delegation (E07): writer -> `access` with a bound {targets: linear, operations: list}. This shows that a bound does not narrow filters (ACC-030; spec/README.md, "What access does not narrow").
10. Turn grants from the operator to both Agents.

Changes S2 must survive:
- E13: rotate the key in 1Password while both Agents work. The running `linear` process keeps the old value (SEC-014), so `linear` is replaced and both grants are re-pointed, with a failure partway through restoring them. No secret may reach an Agent's run.
- E02: a repeat after a filter's deadline.
- E17: what a real client shows after a revocation.

Constructs needed: K1 (more than one Pool), K2, K3, K5 (Agent to Resource, Resource to secret root-only, Agent to access with a bound, Interface to Agent), K6, K7 (filter with per-grant arguments, bound), K13 (two store Types), K14, K15, K16 (for whoever packages Linear), K18, K19 (`as`, so tool names survive a replacement), and R1, R3, R5, R6.

### S3. S1 and S2 across two machines, with a partition and a lost machine
Ruling 51 adds two modes, managed and peer. The owner's second machine is Windows with WSL.

What the author declares:
- Managed mode:
  - A machine provider Resource, such as Tailscale, owned by `system` (PRV-013; ruling 50: "tailscale should be a resource in the pool").
  - Machines are added by enrollment, which is a request (R10) whose labels the enrollment names (ruling 52.4). It is not a document.
  - A runtime provider Resource for the remote machine. The multi-machine paper's example is a docker-context with an ssh host, owned by the team and placed on `local` (section 7). The engine's own runner runs only on `local` (PRV-019).
  - Instances with `placement: m2` and `runtime: m2-docker`, or with no placement, leaving the machine provider to place them by the label selector on their Type or item (PLC-002, PLC-007, PRV-015).
  - A grant with shared-host.v1 where both ends must share a machine, for example an Agent editing a directory through its own shell (PLC-013).
  - Paths stay tied to their node (PLC-020).
  - The secret store placed where the engine reaches it reliably (MM-S2).
- Peer mode (ruling 51's caveat): two engines are introduced, each with its own trust domain (51.2). The owner's root Interface is admitted on the peer as an Interface with grants to the peer's `management` and `access`. A relationship across parties is two halves, each opened by its own engine (51.4). The language has nothing for this yet: no way to declare an Interface that stands for another engine's actor, and no way to name a trust domain.
- Partition and loss (E15; MM-S5 to MM-S9) need no authoring. They are behavior: unavailable, unknown, retired and stopped stay distinct (PLC-023, PLC-024, CON-008).
- The multi-machine paper says documents are unchanged between v0 and v1 (section 7).

Constructs needed: K11 (`placement` and `nodeSelector`), K12, K13 (machine provider, per-machine runtime providers), K7 (shared-host), K4 and K5 (an Interface on a peer), engine configuration D (trust domain), R8, R10, R12.

### S4. A setup that fails partway, is retried, and loses a response
What the author declares:
- A Pool template whose entries depend on each other by reference, for example secret, then service, then a webhook Resource (POOL-006, POOL-013), with some entries optional.
- A Resource Type whose `requires` selects a setup hook and a retire hook with effects, such as registering a webhook in Linear (TPL-012, TPL-017, TPL-018).
- Declared Conduits, a run and a `remote` connection, each one part of the setup.
- Operation contracts that mark `effect`, `startsModelWork` and `deduplicates` (OPS-012).
- The operator's grant listing set-up, inspect, reconcile, retire and retire-with-resources to the target ACC-033 names. Design E's defect (1) was a derived operator grant that omitted reconcile.v1 (design-E-layered.md:593).

The cases:
- E01: a plain server makes part of an effect, then fails.
- E09: two clients repeat after a lost response, one keeping its idempotency key and one sending none.
- E18: the owning Pool is retired, and its Resource is inspected and reconciled.
- E19: records are purged, then an old keyed request is repeated.
- E20: a hook's reply is lost, and its late onward request is attributed to the latest caller (OPS-028).

What must hold:
- A retry is set-up again. Parts that already succeeded are left as they are (pools.yaml:206), and the hook's idempotency key is the same in every setup (OPS-028).
- Nothing rolls back (LIF-027).

Constructs needed: K1 (entries, references, optional), K10 (hooks), K6, K16 (effect marks), K5 (operator grant that includes reconcile), and R1, R4, R5, R9, R11.

### S5. A control changed under contention
What the author declares:
- A Resource whose Type lists a control, such as `set-read-limit.v1`, which changes `/limit` (CTR-006).
- A template whose `controls` map selects it, narrows its input to at most 50 under a contract that allows 100, and leaves out another control (TPL-010; E10).
- An initial config value and defaults (E16: an omitted input, null, `{}`, a nested default, an array element, a default reached only through a `$ref` at the root).
- Two actors, for example the operator Interface and an Agent, each holding a grant that lists the control.

What happens: both send against revision r1. One succeeds and the other gets conflict (REV-002). An omitted input stays as stored (REV-006). A lost receipt is a repeat with the same key (OPS-009). E03 and E04 are contract-time rejections of `changes` that overlap.

The stress for a short form: once a control changes a setting, who owns it, the file or the live object? The critic found that the layered planner treats a control applied outside the file as drift to be replaced (critic.md, gap 5).

Constructs needed: K8 (controls map with narrowing, config, defaults), K5 (grant listing a control), K4, K16 (`changes`), and R1 and R2.

### S6. Access revoked and a Resource retired while requests are in flight
No round 6 case covers S6.

What the author declares:
- Agents A and B, each with a grant to a Resource R, say a search or Linear adapter.
- R has a declared Conduit to a backing Resource for its onward requests, a long operation that may run as a task (IFC-016), and a retire hook (TPL-018).

The sequence:
1. A's grant is closed in the middle of A's call. The call runs to its end, and onward requests made after the close are refused (ACC-034).
2. R is retired while B's call is in flight. Grants to R close at once (CON-008), and after that only the requests of the consumer's own retirement are permitted (ACC-039).
3. Tools are withdrawn (IFC-013, with listChanged under IFC-038), and every new request is decided again (ACC-036).
4. The author's file no longer lists A's grant or R.

The stress for a short form:
- Removing a line has to mean an explicit close or retire, never pruning without a flag (ruling 53.4).
- What is closed must stay closed against a later apply, through tombstones enforced by the engine (ruling 53.3) and keyed by the grant (critic gap 2).
- Refusals must explain themselves (ruling 52.1).

Constructs needed: K5, K6, K10 (retire hook), K16 (task-capable operation), K19 (tombstones, expectedId on close), and R3, R5, R6, R11.

### S7. An outside author packages an existing MCP server and a small code plugin
What the outside author declares, all kind B:
- A package manifest: address, version, exports.
- A Type: a schema with a secret-reference place for the server's token; `offers`; `nodeSelector` (for example linux and amd64, the label selector that replaced "location needs" under ruling 41, 8a); and `records`.
- A Capability and Operation contracts, with effect marks and hints.
- A Requirement contract for the plugin's filter, with complete semantics (D05).
- Two implementation items:
  - the existing server, `plain`: a `package` item (npm or OCI with a digest, PRV-012), stdio, a `tools` map, and an environment variable or package argument marked `isSecret` bound to the token place (SEC-021). A repeated `-e` argument is the MAN-010 limit;
  - the plugin, Conduit-aware, whose item lists the filter in `supports`.
- Exported templates, so that consumers do not write them. Today the S1 packages export none (map-constraints.md, section 0).
- A later release follows NAM-024: additions only.

The consumer then installs it (a pin), declares an instance and declares grants.

The qualification on one user task (D01): plain server, small adapter, Conduit-aware implementation.

The stress for a short form: does it also cover contracts and packages? If not, the language has two normative syntaxes.

Constructs needed: K15, K16 (all of it), K14, K11 (`nodeSelector`), K9 (exported templates), and R7.

### S8. Search over storage returning only what the caller may see
No round 6 case covers S8.

What the author declares:
- A storage Resource `docs`.
- A search Resource whose index is derived from `docs`. It serves queries by onward reads through its own declared Conduit to `docs`, carrying the caller's filters (ACC-017).
- Grants:
  - Agent A -> search with the filter visible-prefix(/public);
  - Agent B -> search with visible-prefix(/);
  - a third grant with a deadline filter (E02: a repeat after the deadline is dispatched again and refused, and no stored output is delivered; OPS-009, OPS-034).
- The search implementation, and the docs implementation that the filter is carried to, both list the filter in `supports` (CON-022).
- The search Type sets `records` outputs to false, so hidden content stays out of records.

The filter applies to what is returned (IFC-019), and that includes derived content.

Constructs needed: K7 (filters with per-grant arguments), K6, K16 (Requirement contract, `supports`), K17, K5, and R1.

### S9. First start, later start, and explicit repair
No round 6 case covers S9.

- First start: the engine makes `system`, `local`, `management`, `access` and `root`, with root's two grants (FST-001 to FST-006). The author's configuration is engine configuration, outside documents:
  - the trust domain, named at first start (51.2);
  - the hostname (DL5);
  - the list set on the runner (PRV-023, DL16);
  - the store.
- Foundational declarations: the providers (machine provider and package sources owned by `system`, runtime providers, secret stores with their own credentials); operator Interfaces and their grants to Pools; and the sign-ins pending for each Agent's home (R12).
- Later start: nothing in the documents changes.
  - Provider runs are made again when first needed (PRV-026).
  - Remote connections read their secrets again (SEC-023).
  - Invocations that were in progress end unknown (IMP-007).
  - Runs that ended stay ended. Nothing repairs itself (OBS-004).
- Explicit repair:
  - set-up again on an instance that is ready, which asks for the run (TPL-012, PRV-007);
  - reconcile (OPS-014);
  - replace or register-replacement (LIF-009);
  - declare-lost (PLC-017);
  - unregister what is retired (LIF-017, LIF-036).

The stress for a short form: a short form with desired-state semantics must keep three states apart: what the file says, what is registered, and what is observed. It must never recreate on its own (LIF-008). The derived defaults (a Pool per project, the operator grant, turn grants) are ruling 53 decision 6, which is open.

Constructs needed: K13, K4, K5, K18, D, and R4, R5, R8, R12.

### S10. S1 and S2 with every provider swapped
The owner's second providers: 1Password as the store, Docker as the runtime, Git as the package source.

What the author declares:
- A new secret Resource of another store Type with that Type's locator form (SEC-005: fixed).
- A Docker runtime provider Resource. Instances must then use `package` OCI items with digests, because a relative command runs only on the engine's own runner (PRV-007; D16).
- A Git package-source provider owned by `system` that claims a scheme (PRV-013, PRV-016), with `fetch` changed in the manifests.

Because runtime (PRV-006), locator (SEC-005) and placement (PLC-009) are fixed for life, each swap is a cascade of replacements:
- a new secret, then a new service Resource, then both Agents' grants re-pointed;
- a new runtime, then every instance replaced. Replacing an Agent loses its home and login (critic gap 11; ruling 49), so every replaced Agent needs a fresh sign-in.

The cases: E06 (two providers claiming one scheme are refused), E14 (list every provider-specific edit), D15 and D16 (what a swap preserves, defined before the run), and D09 (rotation on the swapped store).

The stress for a short form: a provider choice, per instance or as a project default, and a stated meaning for the cascade a swap causes.

Constructs needed: K12, K13 (all four kinds), K14, K15, K19 (`as`, tombstones, expectedId), and R5, R6, R12.

## 3. The owner's real use: Conduit building Conduit, with 10 agents

Sources: A78 (ROADMAP.md:71), ruling 48, owner-direction/2026-11-10.md. The worker shape is "each worker an Agent with a Conduit to its own working tree and one to `main`, and the queue of orders its message history".

Declared once:
- Pins for the Codex and Claude Code Agent packages, a git working-tree Type, and a queue or history Type.
- A Pool `conduit-dev`.
- A Resource `main`, the checkout, whose path is tied to its node.
- A Resource `orders`, the queue.
- An Agent `admin` (Claude Code), with:
  - main [read, list, write], since it merges;
  - orders [read, post];
  - the authority to create workers and grants: either `management` plus `access` with a bound over `conduit-dev`, which is broad trust (ACC-030), or a broker that registers only the approved worker template (D04);
  - turn grants to each worker.
- The root Interface's turn grant to `admin`.

Declared per worker i (1 to 10), with parameters name, client (Codex or Claude Code), expected account, model, branch, and machine (`local` or the WSL node under ruling 51's managed mode):
- Resource `tree-i`: a working tree of `main` on branch wo/NNN. Its setup hook adds it, and its retire hook removes it only if clean (L86).
- Agent `worker-i`, with its reach flags off and its own home and sign-in. Experiment 010: "A Codex login cannot be copied into each Agent's home", so ten sign-ins arrive as pending actions.
- Grants:
  - worker-i -> tree-i [read, list, write];
  - worker-i -> main [read, list];
  - worker-i -> orders [read, post]. Seeing only its own orders needs either a filter, and so a Conduit-aware queue, or one queue per worker;
  - admin -> worker-i [turn];
  - admin -> tree-i [read, list], and a verifier -> tree-i [read].

Each order then runs through a cycle: create the worker and its tree, run its turns, merge, then retire both. Names are reused across orders, so expectedId (D12, E12) and tombstones keyed per grant matter.

Totals: 20 objects and about 50 grants. Several packages are worked on at once, so the project also needs:
- local contracts replaced along with their templates (SUB-009; D11, E11);
- locally built archives installed from `file:` URLs;
- versions side by side (rulings 25 and 27).

Estimated cost today, on paper:
- Even with a worker template whose declared Conduits take the tree as an input (INS-012 allows a declared Conduit's target as an input), each worker still needs about 34 to 42 lines: a Resource instance (about 9), an Agent instance (about 9), and 2 or 3 standalone grants (about 8 each). The admin's grants cannot be declared in the worker's template, because a declared Conduit's consumer is always the new instance (CON-002).
- That is about 340 to 420 lines for 10 workers, plus about 100 shared.
- The floor is about 1 line of differing facts per worker, plus about 15 shared lines.

What this stresses about reuse:
1. No template makes an Agent and its own Resource together. A Pool's entries make only Resources, and an Agent template makes only Conduits.
2. Grants whose consumer is another object, such as the admin's to each worker and tree, become N standalone grants (CON-001: one target each). Sugar like "grant admin read on every tree" must still expand to N explicit grants that can be inspected.
3. No interpolation (IN5). Names, branches and paths are written per worker, or derived by the type's implementation.
4. A Type is never an input (IN2), so Codex workers and Claude Code workers need two templates.
5. Upgrading an Agent package replaces every worker (INS-020: frozen instances), which loses ten homes and brings ten new sign-ins. The alternative is to keep old versions side by side until each order ends.
6. Workers churn per order, under agent-driven management (test 5): broad trust or a broker.
7. Some workers may run on the WSL node, which needs placement and runtime per worker.
8. The per-worker view of the queue needs a filter or a queue per worker.

Existing patterns for "N replicas, each with its own volume, with stable names": Kubernetes StatefulSet with volumeClaimTemplates, Compose `extends` and YAML anchors, and Terraform modules with for_each. for_each with names derived by interpolation would run into IN5, so it would need the owner.

## 4. The matrix: which scenario needs which construct

- K1 Pools: more than one, with entries and references between them. S2, S4, S9, real use.
- K3 per-Agent settings: reach flags, model, account (K18). S2, S9, S10, real use.
- K4 Interfaces other than root: S3 peer, S5, S9, test 8.
- K5 grant pairs beyond Agent to Resource:
  - Agent to Agent turn: real use;
  - Agent to access with a bound: S2 (E07), real use;
  - Agent to management or a broker: real use (D04);
  - Resource to secret, root-only: S2, S10;
  - Interface to Pool with reconcile: S4, S9.
- K6 declared Conduits, for onward requests from a Resource: S2, S4, S6, S8.
- K7 filters with arguments per grant: S2, S8, real use. Bound: S2, real use. Shared-host: S3.
- K8 controls, narrowing and defaults: S5.
- K9 templates and reuse at scale: S7 (exported templates), real use.
- K10 hooks: S4, S6, real use (setup and retire of a worktree).
- K11 and K12 placement, label selectors and runtime: S3, S10, real use (WSL).
- K13 providers of all four kinds: S2, S3, S9, S10.
- K14 secret references and isSecret bindings: S2, S7, S10.
- K15 and K16 contracts and packages: S7, and S2 for whoever packages Linear.
- K17 records flags: S8.
- K18 expected account: S2, S9, S10, real use.
- K19 `as`, expectedId on open and close, tombstones, inventory: S2, S6, S10, real use.

Requests:
- Idempotency key: S2, S4, S5, S8.
- Revision: S5.
- Reconcile: S4, S9.
- Set-up again: S4, S9.
- Replace and re-point cascades: S2 (E13), S10, real use.
- Declare-lost: S3, S9.
- Enroll: S3.
- Pending actions: S3, S9, S10, real use.

## 5. Where the S1-only formats fall short

Design E's file "cannot express filters, Conduits to `access` or `management` for Agents, Agent-to-Agent Conduits, Interfaces other than root, placement, or more than one Pool" (design-E-layered.md:575). The critic adds secret references, and notes that its escape hatch of framed documents cannot follow compiled names across replacements (critic.md, gap 6).

The sketches above add these, which no format proposed so far covers:
- runtime per instance, and a provider choice per project;
- label selectors on Types and items (K16);
- Pool entries with references and optional entries;
- hooks;
- controls with narrowing, and how they relate to a desired-state file;
- requirement arguments per grant;
- records flags;
- the expected account;
- composites of an Agent and its own Resources at scale;
- grants whose consumer is not the declared object;
- peer engines and trust domains;
- the cascade of replacements on a rotation or a swap;
- set-up again as repair.

## 6. Questions the scenarios put to any normative short form (not decided here)

1. Coverage: projects only (kinds A and K9) with contracts left canonical, which makes two normative syntaxes; or contracts and packages too (S7).
2. Desired state against requests:
   - a file value at a control's pointer against a live control (S5);
   - removing a line against an explicit retire or close (S6);
   - later start and drift (S9). The engine must never reconcile on its own (OBS-004, LIF-008), which is ruling 53.4.
3. Cascades: rotation (E13), provider swaps (S10) and package upgrades at scale (real use). Each is a replacement plus re-pointed grants, with windows without access and lost logins. Is the cascade part of the language's meaning (thesis 2), or a tool's plan?
4. Every derived default (operator grant, turn grants, a Pool per project) needs an override for S2, S3, S9 and real use: multiple Pools, a non-root operator, turn grants consumed by Agents.
5. Reuse without expressions (IN2, IN5) at N=10, the real use.
6. Constructs whose promises are not written yet cannot be checked against any form:
   - enrollment (52.5 open);
   - pending actions and sign-ins (49, 50);
   - the expected account;
   - peer mode (51);
   - the restart verb (A67, not adopted);
   - ruling 53's five additions;
   - the two-tier refusal (52.1);
   - complete filter contracts (D05).

## Unverified
- Every line count, the estimate of 34-42 lines per worker and 340-420 for 10 workers, and the floor of about 1 line per worker, is my paper estimate in the coordinator's counting style. None was run, and ruling 45 counts only runs.
- The authentication modes of Linear's MCP server (a remote https server with OAuth or an Authorization header carrying an API key, against a community stdio server reading an environment variable) were not checked against Linear's current documentation. The S2 sketch covers both routes in neutral terms.
- How 1Password service accounts behave unattended is A73, which has not run.
- The meaning of "dependent calls" in S2's roadmap row is inferred from the collaborator review C2 ("Resource grants with applicable authenticated-caller restrictions"). The roadmap does not define it.
- The concrete content of S6, S8 and S9 is my own instantiation of their one-line roadmap rows. Round 6 gives them no evidence cases.
- I read "explicit repair" in S9 as set-up again on an instance that is ready, plus reconcile, replace, declare-lost and unregister. That rests on LIF-004 ("one that was ready is still ready"), pools.yaml:206 ("a further setup leaves a Resource that is ready as it is") and PRV-007. No promise names a repair verb.
- The working-tree Type, its setup and retire hooks, the queue Type and the per-worker filter do not exist as packages. docs/research/RESOURCES.md, which is not binding and predates the spec, mentions a git-worktree type.
- Whether the admin is an Agent or the root Interface in A78 is not settled. Ruling 48 says only "Conduits that the admin session declares".
- Peer mode's authoring needs (an Interface for another engine's actor, a trust domain in documents) are my reading of ruling 51. No proposal or promise defines them yet.
- I did not read the five design papers or synthesis.md in full; I took design E's limits and the critic's gap 6 as quoted. I also did not check whether a Pool template's entries can take the template of an Agent's own Resource through inputs; INS-012 and POOL-009 suggest only placement, runtime and arguments can be inputs.