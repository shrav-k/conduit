Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

## recommendation
ANSWER
Yes, with a precise meaning of "the language". Make a concise form normative now. spec/ defines it by numbered translation rules into today's canonical documents, and those documents stay the kernel, the wire (register.v1, replace.v1, inspect.v1) and the stored form. Three things are not recommended now:
- making the short form the only form;
- pruning the object model;
- making the change plan normative.

This is the "normative expansion only" option that the prior-art map identified and the first investigation never costed. N2's own designer and its cost skeptic both arrived at the same answer as "stage 1".

WHICH DESIGN
Use N4 (sectioned documents) as the base, with the fixes below. Reasons:
- It is the cheapest normative design: about 22-26 promise touches for the form, roughly half of N1's 31-35 and a third of N3's 60-80.
- The form adds 0 lines to the core.
- It invalidates 0 of the 367 vectors. The cost skeptic confirmed this by scanning 831 files, 3,464 documents and 340 archived documents.
- It is the only one of the four with evidence from a run. The T2 state of S1, translated by hand into 24 canonical documents, was accepted by today's loader and core, and the skeptic reran it.
- It follows existing paradigms (ruling 40): sugar defined by translation, as in Haskell, WebAssembly text, Dhall and SAM, with the long form still legal in the same project, as Compose requires.

Two ideas are taken from the other designs:
- From N1: names that are pure functions of the author's keys, kept stable by replacing break-before-make.
- From N3: the safety rules PRJ-010 (high-trust grants) and PRJ-011 (package templates).

FIXES THE SKEPTICS DEMANDED, ADOPTED
F1. Stable names: replacement is break-before-make under the key's own name. This is Compose's recreate, StatefulSet's delete-then-create and Terraform's default.
- It answers N4's major M1: after T5 or T6 the stored objects carry the same names the translation gives.
- It answers critic gap 6 and N2's M3: framed documents and other projects can name `shared` stably.
- It answers N3 #3: retiring an Agent closes root's turn Conduit to it under CON-008, so ACC-024 no longer makes Agents unreplaceable.
- Exclusive resources (a worktree branch) and "two servers on one root" never coexist.
- Cost: longer windows without access at T5 and T6, for both Agents at once. A name is held while a shutdown is unknown, which fails closed and is resolved by reconcile.

F2. Tombstones, as accepted in ruling 53 but specified so the skeptics' bypasses close:
- Keys: a Conduit is marked by (consumer, local name) and by (consumer, target, capability), so renaming a grant key does not clear the mark. An instance is marked by (kind, name).
- Cascades: a close made by CON-008 leaves a mark naming the retirement that caused it, and is superseded with that retirement's mark.
- Superseding is an authorized act: only root or the actor that made the mark may supersede, and the supersession is recorded with the mark. It is not a precondition anyone may copy from an error message.
- register-replacement.v1 naming a nested instantiation counts as the acknowledgement for that nested Conduit or Resource, and is recorded. This keeps the setup/restore and changes/t4 vectors working.
- Retention: a mark lasts until it is superseded, or purged by root with a record of the purge.
- Disclosure: who closed and which invocation are shown only to root and to the closer. A mark's existence follows the inventory's disclosure tiers.

F3. `as`: amend IFC-014 so that a local name is unique per consumer among Conduits not closed, whatever tools they have. This stops one Conduit's tools being spliced in under another's prefix (N2 #1, N4 #3).

F4. NAM-024: a property added to a Type's schema states no default, as an input added to a Requirement already may not.
- This closes the silent flip of a default by a package (N1 M5, N4 #6) on every route.
- Consequence: ruling 53 #7's reach-flag defaults must ship as `claude-code.v2` and `codex.v2`, not as `.v1` with a default.

F5. In the form:
- `owner` is written on every Resource. With no owner default, a new Resource never silently enters bounds that name a Pool (ACC-029).
- The `operator` shorthand always means the root Interface in v0.
- A one-line grant may not target `management`, and may target `access` only with a `bound.v1` requirement. Those Conduits are written in the full `conduits` section.
- First-start names are reserved as section names.
- A bare name is rewritten into a reference only in fields that accept several kinds.
- A grant has a `from` variant, so package-exported Conduit templates and templates with inputs are reachable.
- The words `entry` and `grant` are not used, because definitions.md reserves both.
- The examples harness routes the 3 kindless examples by file name.
- SCT-009 locates a diagnostic at the definition, and at the field where the rejecting check gives one, so the core still needs 0 lines.
- The as-if rule requires equal objects and records, and diagnostics with the same rule and promise located in the author's text.

F6. Plan rules (tooling, with golden cases that do not bind):
- Before any open or set-up, the plan inspects the registered object's resolved definition and IDs and compares them with the translation and the IDs it expects. This defeats the replace.v1 hijack and squatted names; registration itself is inert.
- An object the translation names but this working copy did not register is foreign: it is refused without `--adopt`.
- Reopening a mark not made in the same apply needs `--reopen`, and recreating one needs `--recreate`.
- A value set by a control is shown and is never reverted without `--take`. An edit at a control's pointer is sent as the control.
- At a version change the plan diffs the whole resolved definition and item, and any widening needs a flag.
- Every declared Conduit and Pool entry of every template used, packages' included, is listed.
- Cascades through resolved references, and Conduits outside the file that a retirement closes, are listed. The plan refuses without a flag where one would not be re-pointed.
- Root-consumed Conduits are re-pointed only by retiring their target.

WHAT IT COSTS AGAINST THE LAYERED PACKAGE
- Spec: about 38-45 touches in all, about 8-10% of 455. About 24-26 are for the form (10 new SCT promises and about 16 amended). About 14-19 are the five additions with F2-F4, which the layered route also needs. The layered route is about 13-18.
- Vectors: about 55-80 new binding cases, against 10-17. 0 existing vectors change because of the form. The additions change about 0-7 existing vectors on either route, depending on the register-replacement rule.
- Code: about the same as the layered route. The CLI compiler moves into the loader as a translator, about 0.7-1.2k lines plus tests. Planning is simpler because there are no successor names.
- No store break. Increments 1-3 stand.
- Readers: implementers read about 7-8 more concepts. Templates and instances have two normative notations. Nothing is removed.

WHAT IT BUYS
- A normative meaning, tested by vectors, for what authors write.
- spec/ itself as the format reference, which answers critic gap 3 and ruling 48's protocol for the fresh author.
- A second implementation must agree (A34).
- No CLI mirror of about 27 promises, so a later change to the core costs about 1.0-1.1 times, not 1.3-1.5.

LINES
On paper S1 costs 20, 7, 0, 2, 1, 1, 2 = 33. Against the baseline's 64 it is better on 5 tasks and the same on 2, so it passes the primary reading. The layered package with the same T7 fix is about 28. The 5 extra lines are the explicit root turn lines and the `owner` lines, which is ruling 52's "as explicit as possible" paid in lines. Normativity itself changes no line count. It fails the strict reading and JSON authoring exactly as every design does, and that turns on decision 2.

THESIS 2, HONESTLY
- Covered: what a set of authored files means when registered. The SCT rules plus the unchanged kernel define it, and translate and equivalence vectors pin it. The loader emits no core Decision, so the translation map carries the SCT rule for each derived document, and the vectors assert those rules.
- Because names are stable, the store equals the translation after every completed apply, with three exceptions: IDs, the pins held by objects that were not replaced (NAM-009, NAM-017), and values set by controls.
- Not covered: how a change is carried out (which requests, in what order, with which flags). That remains the plan's, as ruling 53 #4 accepted "for now". Every request the plan sends has a normative meaning, and the engine enforces expectedId and tombstones against any client.

WHY NOT THE OTHERS NOW
- N2 (one form over a pruned core): about 120-140 promise touches, 152 vectors with changed expectations (about 104 with re-authored steps), a store break, re-evidence of increments 1-3, and an unbudgeted conflict with NAM-025 for core 0.2.0. It also loses expressiveness: templates can no longer carry relationships (S7 adapters), and S4's entries lose their ordering. Its deletions stay v1 candidates, each by its own ruling after the rerun.
- N3 (project document plus normative plan.v1 and a Project object): about 60-80 touches, 6-10.5k lines of Rust and 120-190 vectors. Seven major holes would become binding spec text: tombstone classes, renames, root grants, consent, held IDs, a confused deputy and restoration. Promote the plan later only if a second planner must agree, at about 25-30 promises.
- N1 (sections inside conduit.yaml): it entangles SUB-008 and MAN-004. It makes the translation depend on packages (stage 2). Its normative AUT-018 correspondence adopts other projects' objects. Grants whose consumer is root are inexpressible. Its stable-name insight is adopted as F1.

CHECKLIST
All 15 inventory items are answered:
- The kernel and its 36 promises outside S1 are unchanged and reachable, inline or through `templates` and framed documents.
- The wire and the directory rule are unchanged.
- SUB-008 is untouched, because the form is never placed in the manifest.
- Reuse keeps templates, LIF-009 and INS-020.

## comparison
S1 lines are paper counts by the measurement plan's primary rule: file lines plus command lines that carry values. Baseline A70: 33, 16, 0, 8, 3 (better baseline 2), 2, 2 = 64. Ruling 45 accepts only a run, and the only run behind any normative form is N4's T2 registration.

| | Layered package (E with fixes) | N1 sugar in conduit.yaml | N2 one form, smaller core | N3 project document plus plan.v1 | N4 sectioned documents, as drafted | RECOMMENDED: N4 with the fixes |
|---|---|---|---|---|---|---|
| S1 lines per task, T1-T7 | 16,6,0,2,1,1,1 = 27 (28 with the --reopen fix) | 17,6,0,2,1,1,1 = 28 (34 with stage 2 off) | 20,6,0,2,1,1,1 = 31 | 19,4,0,2,1,1,1 = 28 (25 with D6) | 19,6,0,2,1,1,1 = 30 (33 with its skeptics' fixes) | 20,7,0,2,1,1,2 = 33 (34 without SCT-010) |
| Bar, primary reading | pass, better on 6 | pass, better on 6 | pass, better on 6 | pass, better on 6 | pass, better on 6 | pass, better on 5, same on T3 and T7 |
| Strict reading, against the better T5 baseline | fails (3 of 7) | fails | fails | fails | fails | fails; settled by decision 2, not by the form |
| T1 at the fresh author's 1.7 times factor | about 27 (passes) | about 29 | about 34 (fails) | about 32 | about 32 | about 34 (fails narrowly; risk 1) |
| S2-S10 | The file covers S1's shape. Framed documents beside it cannot follow renamed successors (critic gap 6) | Most, with long-form fallbacks. Grants whose consumer is root cannot be written; entries only partly; declared Conduits cannot follow a replaced target | Nearly everything one for one, with no escape hatch. Templates cannot carry relationships (S7 adapters, S4 ordering through Conduits); no reference survives a replacement | Every kernel construct through `templates`. The escape hatch is removed; Pool entries cannot be granted to; controls are reverted; versions cannot sit side by side | Everything at registration. Names diverge from the live objects after the first replacement (major) | Everything: canonical documents stay legal beside the form, and names stay stable across replacement. Peer mode, enrollment and the expected account are missing from the kernel in every route |
| Safety verdict | major, fixable (critic gaps 1 and 2) | major: 6 major (supersedes is unauthorized, correspondence adopts others' objects, confused deputy, hidden package access, flipped defaults, controls reverted) | major: 7 major (handle not unique, first-start names captured, narrowing reversed, rename bypass, `succeeds` hijack, silent widening, approval) | major: 7 major, and its plan's holes would be binding spec text | major: 2 claims false (thesis 2 after a replacement, the revocation override), 4 medium | every major item above has a fix adopted (F1-F6). Residual: windows without access, the repository as an authority input, root grants that can only be added to |
| Spec promises touched | 11-14 (13-18 with tombstones the engine enforces) | 44-52 (31-35 for the form) | 120-140 (26-31%), plus a ruling on NAM-025 for core 0.2.0 | 60-80 (13-18%), 486-505 promises after | 35-42 (22-26 for the form) | 38-45 (24-26 for the form, 14-19 for the additions with F2-F4) |
| Vectors | 10-17 new; 0-7 changed by tombstones | 50-65 new for the form; 0 changed by the form | 317 cases with inputs converted, 152 with expectations changed (about 104 re-authored), 114 examples, 84 archives repacked | 120-190 new; 5 cases and 11 snapshots, plus tombstone breaks | 40-55 new; 0 of 367 invalidated (scan) | 55-80 new and binding; 0 changed by the form; 0-7 changed by the additions on either route |
| Reference model | CLI 3.5-5.5k lines plus 2.5-3.5k tests; core +0.3-0.6k; nothing rebuilt | +0.7-1.5k over layered; core +0.45-0.9k | 1-3k core lines touched; store break; increments 1-3 re-evidenced; 12-16 work orders | 6-10.5k lines of Rust (1.5-1.8 times layered); core +4.5-7.5k; store break, avoidable | about equal to layered; core 0 for the form | about equal to layered; loader translator 0.7-1.2k plus tests; core 0 for the form plus 0.3-0.6k for the additions; no store break; increments 1-3 stand; 5-6 work orders |
| Reading cost for implementers | 455 promises plus about 5 concepts; a CLI mirrors about 27 promises, so later changes cost 1.3-1.5 times | 474 promises, +7 terms, two syntaxes, a translation that needs packages | about 460, words within 1%, 7 concepts out and 5 in, one form | 486-505 promises, +13-15 concepts, +10-13% words | 466-469 promises, +7-8 concepts | about 466-470 promises, +7-8 concepts, nothing removed; later changes about 1.0-1.1 times |
| Thesis 2 for what authors write | Not tested (golden compiler cases, not binding) | Registration tested; AUT-018 conformance normative (unsafe as written) | Holds only at first registration on an empty engine | Registration and change both normative | Registration only, and false after the first replacement | Registration tested by translate and equivalence vectors. It stays true after replacements, with IDs, held pins and values set by controls apart. Change is tooling made of normative requests |

## grammar
THE SECTIONED DOCUMENT (core#sectioned.v1; promises SCT-001 to SCT-010)

WHERE
- A sectioned document may stand in any top-level documents file of a project or package (MAN-011, SUB-010) except the manifest. It is a YAML document, or a JSON value or array item (SCH-018 amended).
- A map with no `kind` key and at least one section is a sectioned document.
- The strict syntax rules are unchanged: SCH-005, 006, 011, 012, 017, 020 and 021, and UTF-8 with one optional BOM.
- Framed documents stay legal in the same project. Contracts and the manifest are always framed.
- The pins live in conduit.yaml. `conduit init` and `conduit install` write them, and tools never change its `apiVersion`.

TOP-LEVEL KEYS (closed, OBJ-019)
- apiVersion: optional. The default is the manifest's; it is required where there is no manifest.
- templates: {interfaces|agents|resources|pools|conduits: {NAME: TEMPLATE-BODY}}
- interfaces, agents, resources, pools: {NAME: DEFINITION}
- conduits: {NAME: CONDUIT}

NAMES
- NAME follows OBJ-001 (`[a-z](?:[a-z0-9-])*`, at most 63 characters). In addition, it:
  - contains no `--`;
  - is none of system, local, management, access or root;
  - is unique across all sections and the project's framed documents (NAM-027).
- LOCAL (a key of `consumes`) follows OBJ-022, contains no `--`, and is not `t` or `operator`.
- Pool entry keys contain no `--`.
- Every generated name must fit 63 characters; otherwise the document is rejected at the authored key.

DEFINITION of family F: INLINE or REFERENCED, plus COMMON
- INLINE:
  - `type` (required, a fully qualified Type, or `#name` for the project's own);
  - `config?`, `implementation?`, `controls?`;
  - `conduits?`: declared Conduits in the canonical shape, for F other than Pool;
  - `entries?`: Pools only, canonical shape.
  - `from`, `inputs` and `arguments` are rejected.
- REFERENCED: `from` (the four forms of INS-025) and `arguments?`.
- COMMON:
  - `description?` and `annotations?`;
  - standard arguments: a Resource has `owner` (REQUIRED), `placement?` and `runtime?`; an Agent has `placement?` and `runtime?`;
  - `consumes?` (not on a Pool);
  - `operator?`: an OPERATOR, or `null` on a Pool to opt out of SCT-010.

CONSUMES: {LOCAL: GRANT}. A GRANT is closed and takes one of two forms:
- (a) {to, capability, operations, requirements?, description?}
  - `capability` is required and fully qualified; it is never inferred.
  - `operations` is required, may be empty, is fully qualified, and is never grouped.
  - `requirements` is {RNAME: {requirement, arguments?}}.
- (b) {to, from: <Conduit template reference>, arguments?, description?}
- A `to` of `management` is rejected.
- A `to` of `access` is rejected unless the grant holds a `bound.v1` requirement. Such Conduits are written in `conduits`.

OPERATOR: {capability, operations, requirements?, description?}. It is the root Interface's Conduit to this definition's object.

CONDUIT (the `conduits` section; a standalone Conduit with the frame removed)
- Either inline {capability, operations, requirements?} or {from, arguments?}.
- Plus `consumer` and `target` (both required), and `as?`, `description?`, `annotations?`.

REFERENCES
- In `to`, and in the `consumer` and `target` of `conduits`:
  - a bare NAME made by the project's documents becomes {kind: <its family>, name};
  - management and access become Resource, root becomes Interface, and system becomes Pool;
  - anything else is written as OBJ-006 and OBJ-013 require ({kind, name}, or {kind: Pool, name, entry}).
- Fields whose kind is fixed (`owner`, `placement`, `runtime`) take a bare name as today.
- Nothing inside config, arguments, requirement arguments, inline `conduits` or `entries`, or a TEMPLATE-BODY is rewritten (OBJ-021, SEC-001).

TEMPLATE-BODY
- Exactly the canonical template `spec` of the family, with inputs allowed, plus `description?` and `annotations?`.
- A package may hold sectioned documents with `templates` only.

TRANSLATION (SCT-004 to SCT-007; meaning is translation, SCT-003)

| Authored | Translated documents |
|---|---|
| templates.F.N | Template N |
| Inline N | Template N--t (type, config, implementation, controls, conduits or entries), plus instance N {from: N--t, standard arguments} |
| Referenced N | Instance N {from, arguments, standard arguments} |
| N.consumes.L | Conduit template N--L--t {capability, operations, requirements}, plus Conduit N--L {from: N--L--t, consumer: {kind: F, name: N}, target: to, as: L}. Form (b) gives Conduit N--L {from, arguments, consumer, target, as: L} |
| N.operator | Template N--operator--t, plus Conduit N--operator {consumer: {kind: Interface, name: root}, target: {kind: F, name: N}, as: N} |
| A Pool with no operator member (SCT-010) | As if operator were {capability: core#pool-management.v1, operations: [register.v1, register-replacement.v1, inspect.v1, set-up.v1, retire.v1, retire-with-resources.v1, reconcile.v1]}. All are fully qualified and printed in the promise for this spec version |
| conduits.N | Template N--t plus Conduit N, or Conduit N from its template |
| description, annotations | metadata.description, metadata.annotations |

- Every translated document carries the `apiVersion`.

Purity
- The translation depends only on the text, its `apiVersion`, and the names the project's documents make.
- It reads no package, stored object, caller, directory or environment.

As-if rule
- The objects and records equal those of the translated documents.
- Diagnostics carry the same SCT rule and kernel promise, located in the authored text (SCT-009). Secret values are redacted.

Equivalence
- Registering the rendered translation gives the same transition as registering the sectioned document.

S1 EXAMPLE (s1.yaml after T1, 19 lines; T2 adds the marked 6). Each `operations` list is written in full; every line is shown as it is authored.

```yaml
pools:
  s1:
    type: github.com/shrav-k/conduit/core#pool.v1
resources:
  shared:
    type: example.org/s1/filesystem#local-directory.v1
    owner: s1
    config: {root: /lab/shared}
  history:                                                         # T2
    type: example.org/s1/history#file-history.v1                   # T2
    owner: s1                                                      # T2
    config: {root: /lab/history}                                   # T2
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v2
    operator: {capability: example.org/conduit/agent-claude-code#conversation.v1, operations: [example.org/conduit/agent-claude-code#turn.v1]}
    consumes:
      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1]}
      history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # T2
  agent-b:
    type: example.org/conduit/agent-codex#codex.v2
    operator: {capability: example.org/conduit/agent-codex#conversation.v1, operations: [example.org/conduit/agent-codex#turn.v1]}
    consumes:
      project: {to: shared, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}
      history: {to: history, capability: example.org/s1/filesystem#directory.v1, operations: [example.org/s1/filesystem#file.read.v1, example.org/s1/filesystem#directory.list.v1, example.org/s1/filesystem#file.write.v1]}   # T2
```

- It translates to 24 documents (12 templates and 12 instances), the set N4's run registered:
  - s1--t, s1, and the derived s1--operator with its template;
  - shared, history, agent-a, agent-b, each with its --t template;
  - agent-a--operator and agent-b--operator;
  - agent-a--project, agent-a--history, agent-b--project and agent-b--history.
- One of them, as registered:

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata: {name: agent-a--project}
from: agent-a--project--t
consumer: {kind: Agent, name: agent-a}
target: {kind: Resource, name: shared}
as: project
```

- The tool names stay project.* and history.* for the life of the project.
- T4 edits the two `project` lines.
- T5 is `conduit install filesystem-0.2.0.tgz`. `shared` is then replaced break-before-make under the same name.
- T6 edits `config.root`.
- T7 deletes the agent-a `project` line. The restore is the same line again, plus `--reopen agent-a/project`.

S2 EXAMPLE (Linear with its key in 1Password; a filter, a bound, an onward Conduit, root-only get-secret Conduits)

```yaml
pools:
  tools: {type: github.com/shrav-k/conduit/core#pool.v1}
  vault: {type: github.com/shrav-k/conduit/core#pool.v1}
templates:
  conduits:
    linear-all: {capability: example.org/linear#issues.v1, operations: [example.org/linear#issue.list.v1, example.org/linear#issue.get.v1, example.org/linear#issue.search.v1, example.org/linear#issue.create.v1, example.org/linear#issue.update.v1, example.org/linear#comment.create.v1]}
resources:
  op-token:
    type: example.org/keychain#keychain-secret.v1
    owner: vault
    config: {service: conduit-op, account: test-vault}
  linear-key:
    type: example.org/onepassword#op-secret.v1
    owner: vault
    config: {locator: 'op://conduit-test/linear/credential', token: {kind: Resource, name: op-token}}
    consumes:
      token: {to: op-token, capability: github.com/shrav-k/conduit/core#secret-provider.v1, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}
  linear:
    type: example.org/linear#linear.v1
    owner: tools
    config: {token: {kind: Resource, name: linear-key}}
    consumes:
      key: {to: linear-key, capability: github.com/shrav-k/conduit/core#secret-provider.v1, operations: [github.com/shrav-k/conduit/core#get-secret.v1]}
  scoped:
    type: example.org/linear-adapter#team-adapter.v1
    owner: tools
    conduits:
      upstream: {from: linear-all, target: {kind: Resource, name: linear}}
agents:
  reader:
    type: example.org/conduit/agent-claude-code#claude-code.v2
    operator: {capability: example.org/conduit/agent-claude-code#conversation.v1, operations: [example.org/conduit/agent-claude-code#turn.v1]}
    consumes:
      linear: {to: linear, capability: example.org/linear#issues.v1, operations: [example.org/linear#issue.list.v1, example.org/linear#issue.get.v1, example.org/linear#issue.search.v1]}
  writer:
    type: example.org/conduit/agent-codex#codex.v2
    operator: {capability: example.org/conduit/agent-codex#conversation.v1, operations: [example.org/conduit/agent-codex#turn.v1]}
    consumes:
      linear: {to: scoped, capability: example.org/linear#issues.v1, operations: [example.org/linear#issue.list.v1, example.org/linear#issue.get.v1, example.org/linear#issue.search.v1, example.org/linear#issue.create.v1, example.org/linear#issue.update.v1, example.org/linear#comment.create.v1], requirements: {team: {requirement: example.org/linear-adapter#team.v1, arguments: {team: ENG}}}}
      delegate: {to: access, capability: github.com/shrav-k/conduit/core#access-control.v1, operations: [github.com/shrav-k/conduit/core#open.v1], requirements: {bound: {requirement: github.com/shrav-k/conduit/core#bound.v1, arguments: {targets: [{kind: Resource, name: scoped}], operations: [example.org/linear#issue.list.v1]}}}}
```

- Secret references stay in canonical form inside `config` (SEC-001).
- The get-secret Conduits have Resource consumers (SEC-009), and root opens them, since apply runs as root (SEC-008).
- `upstream` is a declared Conduit, so it is opened before the adapter's run (TPL-012).
- Rotation (E13): the locator is fixed (SEC-005), so rotation is `conduit apply --replace linear`, not an edit. The plan:
  - retires `linear`, which closes `reader/linear` and `linear/key`;
  - lists that `scoped` must be replaced too, because `upstream` holds linear's old ID (OBJ-010), and that the `bound` targets name `scoped`;
  - re-registers all of them under the same names.
- The file stays true throughout.
- A grant from an Agent to `management`, or an unbounded grant to `access`, would have to be written in the `conduits:` section.

## decisions
1. THE SHAPE (ruling 53, decision 1).
- (a) Layered: the CLI defines the short form.
- (b) A normative notation in spec/, defined by translation into the unchanged kernel. This is N4 with fixes F1-F6.
- (c) N1: sections inside conduit.yaml, with a normative conformance relation.
- (d) N3: a project document, a normative plan.v1 and a Project object.
- (e) N2: one form over a pruned core.
- Recommended: (b) now. N2's reductions stay v1 candidates. N3's normative plan waits until a second planner must agree.

2. NAMES ACROSS REPLACEMENT.
- (a) Break-before-make under the key's own name. Names are pure functions of keys, and the store equals the translation after each apply.
- (b) Make-before-break with successor names, plus a lineage or "slot" reference in the kernel. That is a new concept, which the skeptics found hijackable.
- (c) Make-before-break, stating that thesis 2 holds only until the first replacement.
- Recommended: (a). It is what Compose, StatefulSet and Terraform do by default.
- Its price: longer windows without access for both Agents at T5 and T6, to be measured in the rerun. An atomic or generation-based make-before-break can come later, by its own ruling.

3. DERIVED DEFAULTS (ruling 53, decision 6; the detail asked for).
- A Pool per project.
  - It would be derived from a `project:` line. It saves 1-2 lines.
  - Risks: adopting a Pool that already exists, and, together with an owner default, entering bounds that name that Pool.
  - Recommended: written, not derived.
- An owner default.
  - It saves 1 line per Resource.
  - Risk: a new Resource silently falls under any bound that names that Pool (ACC-029).
  - Recommended: `owner` written.
- Root's Conduit to each Pool (SCT-010).
  - The seven pool-management operations are printed in the promise for each spec version.
  - It applies to root only, which is unbounded anyway (ACC-022). It can only be added to (ACC-024). It is shown by render, plan and inspect, and `operator: null` opts out.
  - It saves one 400-character line, and prevents mistakes 1 and 2 of both runs (72 lines).
  - Recommended: adopt.
- A turn Conduit per Agent.
  - It would need the Agent Type's contract, so the translation would read packages.
  - It costs 100-200 core lines, and lets a package version change what root holds.
  - Recommended: written, as one `operator` line per Agent.
- S1 totals by choice: 34 with everything written; 33 as recommended; about 29 with everything derived, as in the layered package.

4. VOCABULARY. spec/definitions.md reserves `entry` (a Pool's entry only) and says `grant` is "not used for access".
- Recommended:
  - the key `consumes`, for the Conduits a definition is the consumer of;
  - the term "sectioned definition", for a section's member;
  - the `operator` key kept, meaning root's Conduit to the object.
- The alternative is to amend definitions.md.

5. TOMBSTONES, specified within the accepted addition.
- Keys:
  - (consumer, local name), and also (consumer, target, capability), so renaming a key does not clear a mark;
  - (kind, name) for instances;
  - cascades leave marks tied to their cause.
- Who may supersede:
  - (a) root or the actor that made the mark, recorded with the mark;
  - (b) anyone allowed to open who names the mark;
  - (c) a separate operation of `access`, which a bound can withhold.
  - Recommended: (a). (b) is the bypass the skeptics found.
- register-replacement.v1 naming a nested instantiation counts as its acknowledgement, and is recorded. Recommended: yes. It keeps the restoration vectors.
- Retention: until superseded, or purged by root with a record of the purge.
- Disclosure: the closer and the invocation are shown only to root and to the closer.

6. `as` UNIQUENESS. Amend IFC-014 so that a local name is unique per consumer among Conduits not closed, whatever their tools.
- Recommended: yes. It closes tool splicing on every route.

7. NAM-024 AND THE AGENT TYPES. A property added to a Type's schema states no default.
- Recommended: yes.
- Consequence: ruling 53 #7's reach flags, which default to false, ship as `claude-code.v2` and `codex.v2`, since `.v1` cannot gain a default under NAM-024.

8. HIGH-TRUST GRANTS.
- One-line `consumes` may not target `management`, and may target `access` only with a `bound.v1`. Those Conduits are written in the full `conduits` section.
- Recommended: yes. It costs 0 lines in S1 and follows ruling 52.

9. register.v1 AND THE FORM.
- (a) Now: the wire stays canonical, and Agents send the public translation.
- (b) Later: an additive optional input, about 1-2 promises and 20-40 lines.
- Recommended: (a) now, (b) when the real use's admin Agent needs it.

10. CHANGE STAYS TOOLING (ruling 53 #4, "for now").
- The plan rules F6 go into the CLI's reference, with golden (file, state) -> plan cases that do not bind.
- A normative plan (about 25-30 promises) comes only if a second planner must agree.
- Recommended: confirm.

11. APPROVAL AND COUNTING (decision 2, still open; the language choice does not settle it).
- (a) A saved plan applied by its ID: +1 line per apply, which fails "most".
- (b) One-step commands.
- (c) Sending is the approval: the plan is printed and recorded, and every dangerous act needs a named flag (--reopen, --recreate, --prune, --replace, --take, --adopt), counted when used.
- Recommended: (c) for the rerun, with (b) as the fallback, and the counting rule stated before the run (ruling 45).

12. RULINGS TO CONFIRM OR AMEND.
- Confirm:
  - rulings 8 (IN3 "one shape" and IN9) and 18 (V2) bind the kernel, and an inline definition is notation for IN9's one-off template;
  - names joined from keys by fixed rules are not IN5 interpolation.
- Amend the wording of:
  - ruling 35: the version is known for each document, and a sectioned document takes the manifest's;
  - rulings 31 and 37: the frame becomes optional for templates and instances, and stays canonical.
- Read ruling 53 #5 as pins written by install into conduit.yaml. The sectioned file never holds pins, so it never becomes a second manifest and SUB-008 is untouched.
- Decision 9 becomes: the format reference is spec/ itself, plus the CLI's command page.
- Ruling 47 (pull request 14) is unread and should be checked first.

13. LATER, AFTER THE RERUN, each by its own ruling:
- frames dropped from the kernel;
- anonymous one-off templates in the core (no --t objects);
- whether declared Conduits, replace.v1 and register-replacement.v1 earn their place in v1;
- a project prefix for engines shared by several projects.

## plan
0. BEFORE WRITING spec/ (reversible; read-only for spec/)
- The owner decides items 1-12. Ruling 47 is checked.
- A Rust spike under experiments/ (Rust only, per the owner):
  - a sectioned-document translator over the loader's Node;
  - translate S1's T1 to T7 files, with T5 and T6 replaced break-before-make;
  - register each state through today's loader, core and conformance runner;
  - assert the equivalence property, extending N4's case that registered the 24 translated documents of T2.
  - Run cargo test with kindless maps routed to a stub, to confirm that 0 of 367 vectors change. Move the 3 kindless examples to routing by file name.
  - After order 028, measure the windows without access in T5 and T6 on the shell.
- Draft, as a proposal record under docs/proposals/ and not binding:
  - the SCT promise text;
  - the five additions with F2-F4;
  - the NAM-024 amendment;
  - the CLI plan rules (F6).

1. SPEC BATCH (one work order, Codex), written once in this order
1. definitions.md: the new terms (sectioned document, sectioned definition, section, consumes, operator, translation, generated name). Record the wording of rulings 35, 31 and 37.
2. Kernel additions, each with its vectors:
   - `as` in OBJ-013 and IFC-012, and IFC-014's per-consumer uniqueness of local names;
   - LIF-037 extended to open.v1 and close.v1;
   - the inventory: LIF-013, inspect.v1's output, and ACC-033's disclosure sentence, with no counts;
   - LIF-038 tombstones as decided, with cross-notes in LIF-035, OBJ-002, LIF-009, ACC-006 and REC-004;
   - descriptions: IFC-033 and OBJ-018;
   - NAM-024 for Type schemas;
   - the operation schemas in spec/core/operations.yaml.
3. spec/promises/sections.yaml: SCT-001 to SCT-009, plus SCT-010 if decided. Amend OBJ-006, 012, 013, 017, 019, 020, 022 and 024, INS-001, MAN-003, MAN-011, SUB-004, SUB-005, SUB-010, SCH-018 and NAM-027.
4. spec/schemas/sectioned.yaml, 250-400 lines. The keys of each family are restated, because the profile has no unevaluatedProperties. Then:
   - the core export lists in spec/core/conduit.yaml and the README tables;
   - 10-15 YAML examples (valid, invalid, by rule);
   - an authoring reading path in spec/README.md.
5. tools/check_schemas.py: validate sectioned documents by schema only, 80-200 lines. Fix the harnesses' rule that treats a kindless map as a manifest.
- Gate: every check in AGENTS.md, and cargo test with clippy.

2. BUILD (work orders, Codex; the admin plans and reviews)
- WO-A, core additions and their vectors: 10-17, plus 4-8 tombstone safety rows (rename, cascade, unauthorized supersede, purge, restoration through register-replacement). Report the 0-7 existing vectors that change.
- WO-B, loader translator:
  - the translation map carrying SCT rule IDs, and SCT-009 diagnostics at the definition and field;
  - the runner's `translate` input kind and an `equivalent` option on register-project, modelled on registration-orders;
  - 40-55 vectors: per-rule A/B pairs, names over 63 characters and clashes, high-trust grants rejected, first-start names reserved, redaction, mixed framed and sectioned projects, a package holding only templates, and one exact S1 equivalence case.
  - WO-B runs in parallel with WO-A.
- WO-C, shell:
  - `install` writing pins into conduit.yaml, and `render`;
  - plan and apply:
    - break-before-make classes and drains;
    - checks before open and set-up against the translation and expected IDs;
    - foreign refusal and `--adopt`;
    - the flags;
    - control pointers;
    - the full diff of the resolved definition at version changes;
    - cascades listed;
    - root grants re-pointed only through their target's retirement;
  - `reach`;
  - golden plan cases, not binding.
  - WO-C depends on WO-A.
- WO-D, providers: the Agent Types as `.v2` with reach flags defaulting to false, and true descriptions (4-06, 3-07), published as new package versions.
- Order 028 (running): shell defects 4-12 to 4-16, and `init` recording the store.

3. THE RERUN (ruling 45)
- An experiment plan written in advance:
  - the primary, strict and JSON readings, with the counting rule from decision 11;
  - YAML as the documented form.
- The fresh author receives spec/, the tasks and the CLI's command page.
- The coordinator does a dry run first. Then the fresh Codex author runs.
- Report beside A70 and both earlier runs: lines, requests, windows without access (from records), concepts met, mistakes, and adapters (which must be 0).
- Record the result in the ruling log and in to-review-with-the-owner.md.

4. CONDITIONAL, after the rerun, each by its own ruling
- register.v1 accepting sectioned documents;
- make-before-break through swap.v1 or generations, if the windows matter;
- a normative plan (about 25-30 promises, about 50 vectors), if a second planner must agree;
- the core reductions of decision 13.

## risks
1. THE BAR IS THIN AND ON PAPER.
- The count is 33 against 64: better on 5 tasks, the same on T3 and T7. T5 and T6 win by 1 line each.
- At the fresh author's earlier factor of 1.7, T1 becomes about 34, against the baseline's 33: a fail on T1. The layered package (about 27) is less exposed.
- Only N4's T2 registration has been run. Ruling 45 counts only the rerun.
- Mitigations: a `conduit init` skeleton in YAML; diagnostics that print the line to add; SCT-010 (decision 3).

2. THE STRICT READING AND JSON FAIL ON EVERY ROUTE.
- The strict reading (every command a line, T5 against the better baseline) fails, and so does authoring in JSON.
- The result turns on decision 2/11 and on documenting YAML. The form cannot fix this.

3. WINDOWS WITHOUT ACCESS.
- Break-before-make cuts access for both Agents at T5 and T6, from the close through the retirement, the shutdown, the registration and the set-up.
- At today's 9-17 s per request, that may be tens of seconds.
- The coordinator's T5 had no window. A name stays held while a shutdown is unknown, which fails closed until reconcile.

4. THESIS 2 COVERS REGISTRATION, NOT CHANGE.
- The plan's choices are tooling.
- After a pin bump, objects that were not replaced keep their earlier pins (NAM-009), and values set by controls differ from the file. Equivalence is stated with those apart.

5. TWO NORMATIVE NOTATIONS.
- Templates and instances gain a second notation, and implementers read about 7-8 more concepts. Nothing is removed until the v1 rulings.
- The schema restates each family's keys, so every future template key costs a line in sectioned.yaml.
- The store holds generated --t templates: 12 of the 24 objects in S1's T2 state.

6. NAMES ARE ENGINE-WIDE (OBJ-002).
- Two projects on one engine cannot share a key, and conversion from existing framed deployments needs `--adopt` or re-creation.
- The committed applied record and the author's repository are authority inputs: whoever edits the file and runs apply can widen access.

7. ROOT GRANTS CAN ONLY BE ADDED TO (ACC-024).
- `operator` lines and SCT-010's Conduits can be narrowed or removed only by retiring their target, so the file can describe a state the engine cannot reach.

8. CASCADES.
- Declared Conduits, config references and bound targets hold the IDs they resolved (OBJ-010). Replacing a target forces their holders to be replaced too.
- For Agents, replacement loses the home and the sign-in (ruling 49). Package upgrades at the scale of 10 workers bring 10 sign-ins.
- Package templates' declared Conduits open under root at setup, so the plan must list them.

9. THE ADDITIONS CHANGE EXISTING VECTORS ON EITHER ROUTE.
- Engine-enforced tombstones change about 0-7 existing vectors on either route (setup/restore, changes/t4, s1/whole), depending on the register-replacement acknowledgement rule.
- Tombstones grow without bound in the real use, where names are reused per order: each reuse needs `--recreate` or a fresh name.

10. THE ESTIMATES MAY BE LOW.
- Every skeptic found the designers' costs 15-35% low. The planner, in particular, may land at the upper bound.
- The figures here already include the cost skeptics' corrections, but nothing has been built.

11. UNWRITTEN RULINGS.
- Rulings 49-51 (an account field, pending actions, peer mode) and ruling 52.5 (enrollment) are unwritten in the kernel, so no form can be checked against them.
- If they arrive as new families or standard arguments rather than `config` keys, sectioned.yaml and the SCT rules need amendment.
- Ruling 47 is unread.

12. THE VOCABULARY AND RULING AMENDMENTS (decisions 4 and 12) MUST LAND BEFORE THE SPEC BATCH. Writing `entry` or `grant` into normative text would contradict definitions.md.