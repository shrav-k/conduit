Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# The S1 cost drivers: which spec promises create each one, what each protects, and which guarantees must survive a simpler language

## 1. Short answer

- **Where the lines go.** About half of the coordinator's document lines are the per-document frame: `apiVersion`, `kind`, `metadata`, `name` and `---` make up 164 of the 328 lines in its final files. A quarter is plumbing for the root Interface that has no counterpart in the baseline and protects nothing for root (81 lines). That plumbing is the root Interface's Conduits to its own Pool (38 lines), its Conduits to each Agent for turns (30 lines; finding 4-11 does not list these) and a Pool template plus its instance (13 lines). 27% is change done by writing new documents (T5–T7: 88 lines plus 15 lines of manifests). 10% is manifests (34 lines), of which 8 are copied digests and 10–14 are manifests the spec does not require.
- **What is load-bearing.** The safety guarantees behind drivers (b) and (d) are real, but narrower than the rules that carry them. Eight must survive any simplification:
  1. Stored references hold IDs, never names.
  2. An open Conduit's content never changes. Narrowing is close + open, and every open passes `access` with its bound check.
  3. Nothing implicit re-issues a revoked grant.
  4. The code and configuration running under an already-opened grant never change unless that grant is decided again.
  5. What runs is the installed digest.
  6. Operation lists are explicit and fixed.
  7. Revision preconditions protect against concurrent actors.
  8. Every request is authorized by a named open Conduit and recorded.
- **What is incidental.** These cost lines and protect nothing:
  - the per-document frame;
  - the mandatory template behind every instance (IN9);
  - template input declarations for one-off values;
  - a standalone Conduit's tool prefix being its global object name (the recorded reason was rename, which ruling 41 removed);
  - a closed Conduit being removed, so restoring it needs a document;
  - LIF-009 rebuilding a replacement from "the template as it now stands";
  - root needing explicit Conduits to its own Pools and Agents. Root already holds that authority through ACC-022 plus `root-management`, and ACC-024 makes those Conduits unclosable, so they add no revocability;
  - hand-copied digests and unneeded manifests.
- **Paper estimates against the bar** (§5):
  - Tooling and packages only, no language change: about 136 lines against 64. Fails.
  - Targeted change operations on today's document form: about 61–70. Borderline; still more on T2 and T6.
  - Those operations plus a compact authoring form and an explicit, plan-based apply: about 25–34. Passes on paper.
  - To meet the bar, both the frame and in-place change must move.

## 2. Where the coordinator's lines went

The attribution is reconstructed from the final files in `docs/proposals/first-build/increment-4/coordinator-run/`, excluding `project-t1fix`, whose documents were merged into `project/`. That gives 328 non-blank lines. The run's own count adds the 2-line T1-fix manifest and T4's 2 edits, for 332. The files agree with the run on 33 documents, 26 separators and 8 digests.

| Purpose (non-overlapping) | Lines | Share | Driver |
| --- | --- | --- | --- |
| Setting up the shared things and access in T1 and T2 (Resource, Agent and Conduit templates and instances) | 125 | 38% | a, d |
| Change done as new documents (T5 `upgrade.yaml` 57, T6 `replace.yaml` 24, T7 `restore.yaml` 7) | 88 | 27% | b, d |
| Root's Conduits to the Pool (`pool-admin` + `root-workspace`, 20; `pool-register` + `root-workspace-register` + manifest, 18) | 38 | 12% | c |
| Manifests (11 + 8 + 5 + 5 + 5) | 34 | 10% | e |
| Root's turn Conduits to the Agents (`claude-turn`, `codex-turn`, `turn-a`, `turn-b`) | 30 | 9% | f (not in 4-11) |
| Pool template + instance | 13 | 4% | c |

The following categories cut across the table:

- **Frame:** 39 `apiVersion` + 33 `kind` + 33 `metadata:` + 33 `name:` + 26 `---` = 164 lines (50%).
- **Template/instance markers:** 15 `spec:` + 18 `from:` lines.
- **Input declarations for one-off values:** 12 lines.
- **Contract-address lines:** 45. These make lines wide, not more numerous.

Manifests the spec did not require:
- T2-fix (2 lines) names only the core package. NAM-007 exempts the core package from being listed.
- T6 and T7 (5 + 5) hold only instances of local templates, and NAM-010 resolves those through the template's held pins.
- The shell's `submit` sends a manifest only when one is present (`reference-model/crates/shell/src/author.rs`: `if let Some(manifest)`).

Concrete evidence of 4-01's "name lies": in the final `coordinator-run/project/templates.yaml`, the template `read` lists `file.write.v1`, and `read-write` lists only read and list. That is the result of T4 replacing both shared templates.

## 3. The load-bearing guarantees: keep them, whatever the mechanism

| # | Guarantee | Carried today by | Why (record) | What would violate it |
| --- | --- | --- | --- | --- |
| G1 | Reusing a name never redirects an existing relationship | OBJ-009, OBJ-010 | DD1 (ruling 21, `docs/proposals/declarations/README.md`): "Templates hold names; instances hold IDs. A reference resolves once"; round 6 keeps it: "Reusing a readable name does not silently redirect an existing relationship" | Resolving references by name at use, or re-resolving on apply |
| G2 | What an open Conduit permits never changes while it is open. Changing it is close + open, and the open is decided by `access` with its bound | ACC-011 ("To narrow access, the Conduit is closed and a narrower one ... is opened"), ACC-006, ACC-030, CC3/LIF-006 | RR14: "A grant is revoked whole. Narrowing one means revoking it and issuing another." It also keeps records simple: REC-002 names "the Conduit used", whose content is then fixed | Editing a Conduit's operations, target or filters in place without an `access` decision |
| G3 | Nothing implicit re-issues or reopens a revoked grant | LIF-008, LIF-009 (explicit request only), CON-004 (setup reports conflict), DO2 | RD2: "Nothing does: not a restart, a repeated setup or an import ... Today the text allows (b), which quietly undoes a revocation." DO2 (ruling 30): "continuous repair would undo revocations." Ruling 29: "a replacement is a new instance made only on an explicit request" | An apply or setup that recreates or reopens a Conduit someone closed |
| G4 | The code and configuration that run under an already-opened grant never change unless that grant is decided again | OBJ-011 second clause ("no document rewrites the definition of an instance"), INS-013, INS-020, PKG-005, NAM-010 | DR5/L109 ("An instance's definition is never rewritten through the registry"). ACC-030 says `replace.v1` trusts its consumer to choose implementation items "of the instances made from the template afterwards". SEC-008: only the root Interface opens a Conduit listing `get-secret.v1`. Without G4, a holder of `management` `replace`/`register` could run its own program under a Resource's secret-reading Conduit | Upgrading or restarting an instance in place under its existing Conduits, unless the request is authorized as if it re-opened each consumed Conduit (and is root-only where one lists `get-secret.v1`) |
| G5 | What runs is the archive whose digest was installed, and a registered template keeps meaning what it meant | NAM-006, NAM-009, PKG-004, PKG-026, PKG-006 | Naming record (ruling 27): "A template registered last month must keep meaning what it meant then"; principles review 29 keeps the archive digest; D17 warns against "version ranges or implicit installation to shorten an example" | Version ranges, implicit fetching, unpinned resolution |
| G6 | Operation lists are explicit and are not widened by an upgrade | CON-017, IFC-012, PKG-020 (`tools` map) | NAM-024 lets a later release add "a further operation of a capability". This is why 2.2.0's `append_file` and `copy_file` reached no Agent in T5, where the baseline's read-only agent gained them | An "all operations" shorthand that is expanded again at upgrade |
| G7 | Stale knowledge does not overwrite a newer change | REV-001 to REV-005, REV-011, SUB-009 | Round 6 preserve list: "Stale knowledge is rejected rather than silently overwriting a newer setting"; agents can operate the system (preservation test 5) | Last-writer-wins updates where several actors change one object |
| G8 | Every request is decided by one named open Conduit and recorded | ACC-001, ACC-002, ACC-033, REC-001, REC-002 | Rulings 27–29: "every permission is an open Conduit"; preservation tests 2 and 7; RC1/RC2 (ruling 30) | Authority by identity with no Conduit; derived Conduits that are not real, inspectable objects |
| G9 | Required Conduits are open before an instance's run starts | TPL-012, CON-004 | RD3 (ruling 19): "The requested access is then an object that can be inspected before anything runs"; principles review 8g (runs that read secrets at start) | Dropping declared Conduits for standalone ones everywhere |

Two further points:
- **Revocation takes effect at once** (OL1: "A revocation has to take effect immediately"). Any swap must keep that.
- **Fail-closed checks at open** (RD6, CON-022) must run again whenever what a Conduit points at changes.

## 4. The drivers

### (a) The document frame, and a template and an instance for everything

**Promises that create it:**
- OBJ-012: a template "has `apiVersion`, `kind`, `metadata` and `spec`, and everything the template declares is under `spec`".
- OBJ-013: an instance "has `apiVersion`, `kind` and `from` as top-level keys".
- OBJ-017: every document "carries `apiVersion`".
- OBJ-020: every document has a `kind`. OBJ-018: `name` is required in a template.
- INS-001: "It is a template if it has no `from`, and an instance if it has `from`. There is no other marker."
- INS-002: "Every instance is made from a template."
- Record IN9 (ruling 8, `docs/proposals/instantiation/README.md`): "Every instance comes from a template. A complete instance cannot be created directly; a one-off is a template with no inputs, instantiated once."
- INS-021, with IN4 ("There is no shorthand that infers an input's type from the field it lands on"): an input needs a `schema` and a `to`.

**What it protects:**
- **The frame:** tooling uniformity and nothing else. Ruling 31's D2 says "The envelope is already Kubernetes' and the fixtures already use camelCase"; its D11 says "One loader and one schema profile then cover everything an author writes". OBJ-024 needs a language version per document, but one per project would serve.
- **The split:** ruling 17 rejected two kinds per family "since a template and its instance are two variants of one kind and the template flows into the instance". Ruling 18 then made `from` the only marker. What the split actually buys:
  - **Reuse and publication.** Packages hold only templates and contracts (MAN-011); package templates are reached by qualified `from` without being registered (INS-025, NAM-013).
  - **Registration order independence** (DD1).
- **What it does not buy:**
  - **The instance snapshot.** That belongs to the instance itself: INS-013 says "The instance holds the complete resolved definition ... The arguments are not kept as a separate layer." After instantiation a template's only roles are the `from` shown in inspection and LIF-009's replacement source.
  - **Control through the closed input list (IN1).** ACC-030 states that `management` "cannot be narrowed to one template: a Conduit to it that lists `register.v1` or `replace.v1` trusts its consumer to register ... an Agent made from any template among them." So the closed input list gives no access guarantee today. Narrow provisioning goes through a broker (D04).

**Load-bearing?** Neither, for one-off objects. Templates still matter for packages and for a future approved-template broker (v1).

**Options with the same guarantees and fewer lines:**
1. **Packages export ready templates.** No spec change: the filesystem package exports a Resource template with input `root` and Conduit profiles, and each Agent package exports a Conduit template for `turn.v1`. This saves the template documents but not the frame of each instance (§5, P0).
2. **A compact project form**, as Docker Compose writes a project:
   - one `apiVersion` per project;
   - objects as maps keyed by name under family keys;
   - an instance with its definition inline;
   - `owner` defaulted;
   - the capability inferred when the target's type offers exactly one capability that holds the listed operations, otherwise rejected naming the candidates;
   - operations written relative to that capability's package.

   It translates deterministically to today's documents, and the translator can emit an anonymous template per inline object, so IN9 still holds at the document level. Made a normative syntax, it changes OBJ-012/013/017/020, SUB-005 (diagnostic locations must point into the compact file, as ruling 52 requires), MAN-001/003, NAM-027 (needs its own cross-kind check) and ruling 31's D2. Kept as tooling first, it changes nothing in `spec/` and nothing the independent builds must agree on.
3. **Normative inline instances.** This reverses INS-001, INS-002 and IN9 (ruling 18), and OBJ-003's naming from the template. LIF-009 replacement would come from the held inline definition, which also removes 4-01's shared-template coupling. The template checks of TPL-003/004/005, MAN-017, CON-017/022 and IFC-014 run when the instance is registered. No load-bearing guarantee is lost: registering still needs `management` (ACC-033).

### (b) Objects never change

**Promises that create it:**
- OBJ-002: "A submission only adds objects and never changes one."
- SUB-007: "It never replaces a template"; "A template is replaced only by the operation that SUB-009 names."
- OBJ-011: "No document and no operation changes its name ... and no document rewrites the definition of an instance."
- INS-020: "Each of those instances keeps the definition and configuration it resolved when it was made."
- LIF-008 (must not): "Setup, anything else that happens to the instance it belongs to, or anything other than an explicit request under LIF-009, makes another instance in its place."
- LIF-009: a replacement is made "from the nested instantiation as the instance it belongs to holds it ... from the template that the `from` of the nested instantiation names, as that template stands when the request is made."
- LIF-035: "A Conduit is so removed when it is closed."
- SUB-009: `replace.v1` "names the template's current revision" and "never replaces several together".
- REV-002 and REV-011: a request with a stale revision reports conflict, one with none reports invalid.
- PKG-005: "The instance holds, for each package its template uses, the version and digest that the template holds under NAM-009."
- NAM-010: references resolve "through the packages the template holds, not through any manifest in force at that time."

**What each protects:**

| Rule | Protects | Load-bearing? |
| --- | --- | --- |
| OBJ-009/010 resolved IDs | G1 | Yes |
| OBJ-011, a name is fixed | Kubernetes' `metadata.name` (principles review 10, which removed rename) | Cheap to keep. With G1 it is not needed for safety |
| OBJ-011, no document rewrites an instance; INS-020; PKG-005 | G4 (DR5/L109, ACC-030 "afterwards", SEC-008) and G5 | Yes, as a guarantee. An explicit upgrade that decides the instance's grants again keeps it |
| OBJ-002, SUB-007 | DD2 (ruling 22): "submitting only registers"; DR9 rejects colliding or changing documents (L112, L113) | Partly. No overwrite by accident is load-bearing. An explicit update request with a revision precondition gives the same |
| LIF-008, LIF-009 explicitness | G3 (RD2, CC3, ruling 29) | Yes |
| LIF-009 rebuilding from "the template as it now stands" | Nothing found in the records | No. It causes 4-01: every holder of a shared template takes the change at its next replacement, and the names lie |
| LIF-035/ACC-011, a closed Conduit is removed | Lifecycle simplicity (principles review 17: "Cost: a closed Conduit is removed when it closes") | No. It forced T7's 7-line resubmission |
| REV-*, a revision is mandatory | G7 | The check is load-bearing. Who writes the revision is not: a tool can supply the one it read at plan time |
| DO2, no reconciler | G3 and preservation test 6 | Yes |

**Options** (the owner's two options under 4-01):

1. **Explicit apply**, as `terraform plan/apply` or `kubectl apply`. This is consistent with DO2, which forbids only change without a request. It is also the paradigm DD2 itself cited ("how Docker Compose and Terraform treat a configuration"), and both of those tools update on re-apply. It stays safe only under these conditions:
   1. The plan is a diff against the stored definitions, carried out by authorized operations, each of which leaves a record.
   2. A Conduit that a request closed since the last apply is a conflict. It is never silently reopened; reopening needs an explicit flag or request. This is Kubernetes server-side apply's field-manager conflict, and it keeps G3.
   3. Any change to what a Conduit permits is an atomic close + open under the same local name, with the open checked against the bound (G2).
   4. Any change to an instance's code or configuration under open grants is an upgrade or restart operation authorized as in G4.
   5. Revision preconditions come from plan time (Terraform rejects a stale plan).
   6. Results are reported step by step (ruling 52, point 2; finding 4-13).

   Apply can be a frontend composition: the shell's `submit` already composes register, open and set-up.
2. **Targeted operations:**
   - **Swap a consumer's Conduit by local name**, to new operations, a new template or a new target. The engine switches at its single decision point (ACC-002, ACC-036), so there is no window. Today T4 left about 30 s without access per Agent, and T6 about 80 s.
   - **Upgrade or restart a Resource in place**, keeping its ID and Conduits (A67's `kubectl rollout restart`, D09). Every open Conduit to and from it is checked again (CON-021, CON-022, IFC-014, IFC-033), failing closed.
   - **Reopen a closed Conduit as a new one** from its last definition.

   Apply needs these operations anyway: without them T5 and T6 still make new Agents or rename tools.

**What breaks:**
- **Promises:**
  - OBJ-002 (3 conformance files), SUB-007 (1), OBJ-011 (1), INS-020 (7), LIF-009 (8; 40 references in `spec/`), PKG-005 (3), and REV-011 if the revision becomes tool-supplied.
  - Rulings: ruling 22/DD2, and round 6's D11 ("not an apply of several").
- **New failure cases:**
  - partial applies;
  - stale plans;
  - a restart while invocations, hooks and secret reads are in flight (D09);
  - records need the instance's revision or package digest at dispatch so that behaviour is attributed to code; REC-002 lacks it today.
- **Keep for v1.** A65's leased, versioned copies of the Conduits on each machine stay simple only if Conduits remain immutable values and change happens as a swap of the set. Under ruling 51 a relationship across parties is two halves, so a swap is per half and per engine.
- **Round 6 asked for this evidence.** D08: "If the only usable upgrade path repeatedly recreates unrelated agents or relationships, that is evidence for a targeted language change. Establish it with a complete trace first." The two runs are that trace: new Resources, standalone Conduits and, for the fresh author, recreated Agents.

### (c) The root Interface needs Conduits to its own Pool

**Promises that create it:**
- ACC-033 row (1): registering a Resource into an existing Pool needs "`management` and that Pool, each of the two Conduits listing `register.v1`".
- ACC-033 row (4): setting up or retiring a Resource needs "the Pool that owns it". Rows (6), (8) and (14) are Pool-scoped too.
- ACC-033 observable: "at any moment one target authorizes a request".
- FST-002: root has "one Conduit to `management` and one to `access`", and nothing else.
- FST-003 (must not): no other object exists after first start.

**What it protects:**
- OL2 (ruling 30): "One authority for an object's whole life: its owning Pool for a Resource", modelled on Azure resource groups and Kubernetes namespaces.
- Ruling 29: "every power over a Pool or its Resources is an open Conduit".
- A1 (re-audit): "Registering a Resource into a Pool that already exists also needs a Conduit to that Pool."
- RR11: only root has access at first start.

These protect Pool-scoped delegation for Agents and, later, teams (preservation tests 5 and 8). There they are load-bearing.

**For root they protect nothing.** Root already holds the authority: `root-management` lists `register.v1`, and ACC-022 gives root an unbounded Conduit to `access`, so root can register and open any Pool Conduit for itself. ACC-024 then makes every Conduit whose consumer is root unclosable, so the explicit Conduit adds a record line and no revocability. The same holds for root's turn Conduits to the Agents (§4f).

**Options:**
1. **Tool-derived.** When root submits a new Pool or Agent, the command line adds the root→Pool and root→Agent Conduit documents and reports them (ruling 52). No change to `spec/`.
2. **A default Pool at first start**, like the Kubernetes `default` namespace. Root's template declares a Conduit to it listing `pool-management.v1`, and `owner` defaults to it. This removes the Pool documents, the `owner` lines and the root→Pool Conduits for a single developer. It changes FST-001/002/003/005/006 (FST-003 is cited by 6 conformance files), OL5's list of permanent objects, and OBJ-013/POOL-001's default for `owner`.
3. **Root's `management` Conduit also authorizes the Pool rows**, as a cluster-wide grant (a Kubernetes ClusterRoleBinding, an Azure subscription scope). This changes ACC-033's single-target rule for root only; ACC-033 is cited by 78 conformance files. It must not be generalized to every holder of `management`: that would let a delegate holding `set-up.v1` or `retire.v1` on `management` act on every Resource in every Pool, which is an escalation.

**Ruling 51 caveat.** In peer mode the owner's root is an ordinary Interface on the peer engine, so options 2 and 3 apply per engine, and option 1 works anywhere.

**Diagnosability.** Ruling 52's two-tier refusal makes this mistake self-explaining. It cost the coordinator about 37 lines and a discarded store, and it was the fresh author's first mistake too.

### (d) Declared and standalone Conduits, and tool names

**Promises that create it:**
- IFC-012: "The local name of a declared Conduit is the name its consumer's template gives it, and that of a Conduit instantiated on its own is the Conduit's own name."
- OBJ-002: a Conduit's name is unique among all Conduits, so two Agents cannot both have a standalone `project`.
- IFC-014: a Conduit whose tool name would equal another tool presented to the same consumer is not instantiated.
- CON-004: a closed declared Conduit "reports conflict until a replacement is registered under LIF-009".
- CON-012: a standalone Conduit leaves "the consumer's definition ... not changed".
- LIF-009: the replacement holds the target already resolved, so a declared Conduit cannot be pointed at a new Resource.

**What it protects:**
- **IFC-014:** each tool name maps to exactly one Conduit per consumer, which keeps attribution. Load-bearing.
- **Declared Conduits:** G9, the access an instance needs is open before its run starts. Load-bearing.
- **Standalone Conduits:** granting access to a live instance. Ruling 19: "issuing a Conduit to a live instance must be a core part of the language." Load-bearing.
- **A standalone Conduit's own name as its tool prefix:** not load-bearing. `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-ab.md`, point 2, chose it because the alternative "would need the engine to keep that name after a rename", and rename was removed by ruling 41.
- **The spec already values stable tool names.** LIF-009's observable: "The local name of a declared Conduit does not change, so the tool name ... is the one it had."

**Option:** a standard argument giving a Conduit instance its local name, defaulting to the Conduit's own name. IFC-014's check per consumer already prevents collisions, and the swap operation in (b) is keyed by consumer and local name. This changes OBJ-013 (a new standard argument) and one sentence of IFC-012. One edge needs a rule of precedence: a standalone Conduit can take the local name of a closed declared one and then block that declared one's `register-replacement.v1` (noted by round 4, reader 3). The Agent's frozen definition either records the swap (its revision advances, a narrow exception to OBJ-011) or inspection reports the current Conduit under each local name (order 027's fix of 4-09).

### (e) The manifest and pinning by digest

**Promises that create it:**
- NAM-006: "each package the project uses by address, with exactly one version and that version's digest".
- NAM-007: every address a document refers to is listed in the manifest.
- PKG-004: the digest is "the SHA-256 of the package's archive file exactly as distributed".
- PKG-006: a registration is rejected unless every listed package is installed.
- NAM-009: a template keeps each package's address, version and digest. PKG-005: so does each instance.
- PKG-026: installation rejects an archive whose digest differs.

**What it protects:**
- G5.
- NB (ruling 27): one file of exact pins, as Go's `go.mod` and Cargo do: "Pins are already exact, so a second file would only repeat them."
- Principles review 12: install before register.

The stored pins and the digest check at installation are load-bearing. The author typing the digests is not: they are copied from the output of `install`, and so are the repeated manifests in later submissions.

**Option:** `conduit install` writes or updates the project's pins, as `go get` and `cargo add` do. Nothing in `spec/` changes, and under the measures nothing a tool generates counts as authored. A language option, omitting the digest where one installation of that version exists, is weaker: the documents would no longer say which bytes they expect. It is not needed.

### (f) Other drivers of author lines

- **Root's turn Conduits to the Agents** (30 lines, 20% of T1). Every request must go through a Conduit (G8), but these are unclosable (ACC-024) and root can open them for itself. Either a tool derives them, or each Agent package exports a Conduit template for `turn.v1`.
- **Agent configuration that is required but always the default.** Both Agent types require `accountServices` and `peerMessaging`, though ruling 48 says reach is off by default. A schema default of `false` in the package saves a line per Agent.
- **Input declarations for one-off values** (12 lines). IN4 governs reusable templates; a one-off can write `config` directly.
- **`owner` on every Resource** (4 lines). A default Pool removes it.
- **`apiVersion` on every document.** One per project would save about 33 lines (OBJ-017, OBJ-024).
- **Contract addresses** (45 lines). These make lines wide, not more numerous; an operation could be written relative to its capability's package.
- **Requests and time without access.** T4 took 8 requests and left about 30 s per Agent without access; an atomic swap needs 2 requests and leaves no window.
- **Explicit operation lists must stay (G6).** Any "all operations" shorthand must be expanded once, when the Conduit is instantiated, and never again at an upgrade.
- **Costs in the fresh author's run that are not the language:**
  - JSON with one key per line;
  - 130 lines of scripts, driven by the commands' behaviour (4-13, 4-14, 4-15);
  - 128 lines in T7, from recreating both Agents after the turn deadline (4-12).

  These still count against the bar.

## 5. Three paper estimates against the baseline (lines; baseline T1 33, T2 16, T3 0, T4 8, T5 3, T6 2, T7 2, total 64)

| Option set | T1 | T2 | T3 | T4 | T5 | T6 | T7 | Total | Against the bar |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| P0. No change to `spec/`: package templates, pins written by `install`, root Conduits derived by the tool, YAML | ~42 | ~24 | 0 | ~15 | ~24 | ~24 | ~7 | ~136 | Fails: much more on T5 and T6 |
| P1. P0 + targeted operations: local name, atomic swap, upgrade or restart in place, reopen as new, a default Pool; today's document form | ~36 | ~25 | 0 | 0 (2 requests) | 0–2 | 0–7 | 0 | ~61–70 | Borderline: less on T4, T5, T7; more on T2 and possibly T6 |
| P2. P1 + a compact project form with inline definitions and an explicit apply | ~15–21 | ~5–6 | 0 | 2 | 0–2 | 1 | 0–2 | ~25–34 | Passes on paper: less on 5–6 of 7, much more on none |

Example of T1 in a P2-style form (illustrative only, not specified). The pins are written by `install`; root's turn Conduits are derived or add 3 lines:

```yaml
apiVersion: github.com/shrav-k/conduit/v0
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    config: {root: <lab>/shared}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v1
    conduits:
      project: {target: shared, operations: [file.read.v1, directory.list.v1]}
  agent-b:
    type: example.org/conduit/agent-codex#codex.v1
    conduits:
      project: {target: shared, operations: [file.read.v1, directory.list.v1, file.write.v1]}
```

How the later tasks would run under P2:
- **T4:** edit two lines; apply swaps both Conduits under the same local names.
- **T5:** `install` rewrites the pin; apply upgrades `shared` in place.
- **T6:** edit `root:`; apply replaces `shared` and swaps both Conduits, or restarts it in place.
- **T7:** a close request; apply reports the closed Conduit as a conflict and does not reopen it; restoring it is an explicit reopen.

## 6. Rulings each option would reverse (decisions for the owner)

- **Compact form, normative:** ruling 31 D2 (the Kubernetes envelope). As tooling: none.
- **Inline instances:** ruling 18 and IN9 (ruling 8).
- **Apply:** ruling 22/DD2 ("submitting only registers"), round 6 D11 ("not an apply of several").
- **Upgrade or restart in place:** PKG-005's "fixed", OBJ-011's second clause. A67 is already listed for the owner.
- **Swap of a declared Conduit's target or template:** LIF-009's source.
- **Default Pool:** FST-001 to FST-003 and OL5 (ruling 30).
- **Root covering Pools:** OL2 (ruling 30) for root only.
- **Local name argument:** nothing ruled; the choice in `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-ab.md`, point 2, now has no reason.
- **Manifest written by a tool:** none.

## 7. Checks against v1 and the later rulings

- **A65** (local enforcement under leases): favours immutable Conduits changed by swapping the set. Mutating Conduits in place would need a version per Conduit.
- **Ruling 51:** derived defaults are per engine; a cross-party relationship is swapped per half.
- **Teams (preservation test 8):** keep Pool scoping for everyone but root.
- **Rulings 49 and 50:** root as the control panel supports conveniences centred on root.
- **Ruling 40:** apply follows Terraform and Kubernetes, the compact form follows Compose, and pins follow Go and Cargo; none invents infrastructure.
- **Ruling 52:**
  - Every derived object, default or inference must appear in results and inspection with the rule that made it.
  - An apply reports each step.

## Key files

- spec/promises/objects.yaml (OBJ-002, OBJ-011, OBJ-012, OBJ-013, OBJ-017)
- spec/promises/instantiation.yaml (INS-001, INS-013, INS-020, INS-025)
- spec/promises/submissions.yaml (SUB-007, SUB-009)
- spec/conventions/lifecycle.yaml (LIF-008, LIF-009, LIF-035)
- spec/conventions/access.yaml (ACC-022, ACC-024, ACC-030, ACC-033)
- spec/interface/implementation.yaml (IFC-012, IFC-014)
- spec/promises/first-start.yaml and spec/core/templates.yaml (FST-*, root templates)
- docs/proposals/declarations/README.md (DD1, DD2, DR5)
- docs/proposals/relationships/README.md (RD2, RD3, RR14)
- docs/proposals/consolidation/README.md (CC3)
- docs/proposals/conventions/README.md (OL1, OL2, DO2)
- docs/proposals/instantiation/README.md (IN1–IN10)
- docs/proposals/principles-review/README.md (items 10, 12, 17, 29)
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-ab.md (point 2)
- docs/proposals/reaudit/round-6/design-critique.md (D04, D08, D09, D11)

## Numbers
| Item | Number |
| --- | --- |
| Baseline (A70), lines per task | T1 33, T2 16, T3 0, T4 8, T5 3, T6 2, T7 2; 64 in all |
| Coordinator, lines per task | T1 154, T2 73, T3 0, T4 2, T5 62, T6 29, T7 12; 332 in all |
| Coordinator's final files, reconstructed | 328 non-blank lines, 33 documents, 26 separators, 8 digest lines |
| Frame: `apiVersion` 39, `kind` 33, `metadata` 33, `name` 33, `---` 26 | 164 lines (50%) |
| Template/instance markers: `spec:` 15, `from:` 18 | 33 lines (10%) |
| Change done as new documents (T5 57, T6 24, T7 7), plus their manifests (15) | 88 + 15 = 103 lines (31%) |
| Root's Conduits to the Pool | 38 lines (12%) |
| Root's turn Conduits to the Agents (not listed in 4-11) | 30 lines (9%; 20% of T1) |
| Pool template + instance | 13 lines (4%) |
| Manifests in all; of which digests; of which not required by the spec | 34; 8; 10–14 |
| Input declarations for one-off values | 12 lines |
| Contract-address lines (width, not count) | 45 |
| Coordinator's T4 | 8 requests; about 30 s without access per Agent |
| Coordinator's T6 | 8 requests; about 80 s without access for both Agents |
| Fresh author | 646 JSON lines in 46 files, plus 130 lines of scripts; T7 128 lines (Agents recreated) |
| P0, no change to `spec/` | about 136 lines; fails |
| P1, targeted operations, today's document form | about 61–70 lines; borderline |
| P2, P1 + compact form + explicit apply | about 25–34 lines; passes on paper |
| Conformance files citing each promise (grep) | ACC-033 78, IFC-012 16, NAM-009 10, CON-004 9, LIF-009 8, INS-020 7, FST-003 6, IFC-014 6, OBJ-002 3, PKG-005 3, OBJ-011 1, SUB-007 1 |
| References in `spec/` | ACC-033 75, IFC-012 57, LIF-009 40, SUB-009 32, `register-replacement` 28, OBJ-002 20, OBJ-011 14, PKG-005 12 |

## Unverified
- **All P0, P1 and P2 line counts are paper estimates.** No design was written out in full or run. The P2 compact form is illustrative and not specified. Its counts depend on YAML style, on whether the pins and root's turn Conduits are written by a tool, and on how a task such as T6 is routed (replacing the Resource or restarting it in place).
- **The attribution in §2 is reconstructed from the final files**, not from per-task diffs: 328 lines against the run's 332, the difference being the T1-fix manifest and T4's edits. It is approximate.
- **That 10–14 manifest lines were not required is inferred, not tested.** It comes from NAM-007, MAN-015, NAM-010 and a reading of `author.rs` (`if let Some(manifest)`). No `submit` without a manifest was run, and the commands page the authors were given may have told them to include one.
- **The safety analysis of an upgrade or restart in place is not complete.** In-flight invocations, hooks, secret reads at the new run, and stops of unknown outcome (D09) were not worked through. The claim that records should carry the instance's revision or package digest at dispatch is my inference; REC-002 does not have it today.
- **Atomic swap.** Whether the shell can switch Conduits under one decision without a window was not checked in code. Whether Claude Code and Codex keep a working session when a tool list changes under the same names was not tested; increment 4 only showed that tool lists changed at the next turn.
- **Not every spec promise was read.** I read the promises relevant to the five drivers and the records they cite. The blast radius in conformance files and spec references comes from grep, not from reading each vector.
- **Ruling 47's content is unknown.** It is reserved for pull request 14.
- **Judgements left to the owner:** whether "much more" applies to P1's T2 (about 1.5×) or T6 (up to about 3.5×), and whether a compact form that exists only as tooling satisfies the language thesis.
- **Records not found:** none states a safety reason for LIF-009 rebuilding a replacement "from the template as it now stands", or for a closed Conduit being removed rather than kept. Their incidental status rests on that absence and on principles review 17 stating only a cost.