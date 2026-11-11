# The shape of the language: a normative short form, and Projects

Not binding. The [specification](../../../spec/README.md) alone binds. The coordinator's summary of
two investigations the owner asked for in ruling 53 and after it, run on 2026-11-10 with many
agents, each with designs, skeptics, a synthesis and a critic. The working papers are in
[short-form/](short-form/synthesis.md) and [project/](project/synthesis.md). Every count is a paper
count; ruling 45 accepts only a run.

## Where they agree

- **The kernel stays.** Today's canonical documents, the engine and its 455 promises, and the
  reference model of increments 1 to 3 stand. Neither investigation found a pruned core worth its
  cost now (about 120 to 140 promises, a store break, and S7 and S4 lose expressiveness). The
  reductions remain v1 candidates, each by its own ruling after the rerun.
- **What an author writes must be much shorter**, and it can be without weakening anything: every
  grant stays a real Conduit listing its operations, every object stays immutable, and every change
  is an explicit request.
- **A Project is the missing unit**, and the owner's direction is broadly right: a flat family, each
  object homed in one project, a computed inventory, Pools shared to other projects, and authority
  over a project held as Conduits to it.

## 1. The short form becomes part of the language

**Recommended:** a sectioned document, defined in `spec/` by numbered translation rules into the
unchanged canonical documents, as the Haskell report defines its sugar. Canonical documents stay
legal beside it, the wire and the stored form do not change, and none of the 367 existing vectors
changes ([short-form synthesis](short-form/synthesis.md)). One by-hand translation of S1's T2 state
was already accepted by today's loader and core.

```yaml
pools:
  s1: {type: github.com/shrav-k/conduit/core#pool.v1}
resources:
  shared: {type: example.org/s1/filesystem#local-directory.v1, owner: s1, config: {root: /lab/shared}}
agents:
  agent-a:
    type: example.org/conduit/agent-claude-code#claude-code.v2
    operator: {...turn.v1}
    consumes:
      project: {to: shared, from: read}
```

What it buys over the layered route of the first investigation: the meaning of what authors write
is binding and tested by vectors (thesis 2); `spec/` itself is the format reference a fresh author
reads (ruling 48); a second implementation must agree (A34). What it costs: about 38 to 45 promises
in all, against 13 to 18, of which 14 to 19 are ruling 53's additions that both routes need.

The critic's corrections, taken into account here:

- **The pass is thin and depends on style.** On paper S1 is 33 lines against the baseline's 64,
  better on five tasks and the same on two. Written as idiomatic block YAML with every operation
  fully qualified in every grant, T1 rises to about 38 and the bar fails. The remedy within ruling
  25 is reuse: a Conduit template names its operations once (`read`, `read-write`), and each grant
  refers to it (`from: read`), so no line carries a list of qualified names.
- **The comparison with the layered route was biased.** The safety fixes (stable names, tombstones,
  `as` uniqueness, no defaults added to a published Type, plan rules) apply to both routes. The real
  differences are the four above and the spec cost.
- **Rulings it amends**, not confirms: 8 and 18 (an inline definition makes a template and an
  instance at once), 19 (a bare name where a field accepts several kinds), and the wording of 31,
  35 and 37 (the frame becomes optional). The alternative keeps references with their kind
  (`to: {kind: Resource, name: shared}`), which costs width and no lines.

## 2. Projects: the skeleton now, authority in v1

The [project synthesis](project/synthesis.md) recommends a sixth family and works out the owner's
direction with the skeptics' fixes:

- **Each object has one home**, a Project or the engine, recorded when it is registered and never
  changed. A Project references every object under it through a computed inventory, not a stored
  list, which would drift.
- **Some objects stay at the engine level**: Interfaces (people are principals, not members of a
  project, so one project's managers cannot retire someone whose authority spans others), nodes,
  `system` and its providers, the first-start objects, packages and records.
- **Pools are shared for use, not administration.** A Pool has one home; it is offered to other
  projects by explicit audience entries naming the projects, the Resources and a ceiling of
  operations, as GitHub shares an organization's secrets with selected repositories. A skills Pool
  for every project and an auth Pool for one are both audiences. Secrets stay root's (SEC-008): an
  auth Pool lives in a root-only project and only its service is shared.
- **Authority over a project is a Conduit to it.** An Interface with authority over three projects
  holds three Conduits, each revocable by its project. Root and `management` enclose every project.
  A Conduit that crosses projects needs authority on both sides.
- **Flat, no nesting**: inheritance would let a parent's grant act inside a project that cannot see
  or close it.

Its honest size is about 100 promises, more than the short form. Its critic found that it reverses
three rulings without naming them: ruling 30's one authority for an object's whole life, ruling 44's
v0 limits on narrowing provisioning, and the v0 scope of rulings 15, 49 and 50 ("teams: not in v0";
granular Interfaces "a lot of the work for v1"; pending actions at the root Interface in v0).
**The coordinator therefore recommends staging it**:

| Stage | What | When |
| --- | --- | --- |
| A | The family, homes, names unique within a project with qualified references across projects, the inventory, tombstones per project, retiring a project with its members | v0, after the S1 rerun: the owner's real use and A78 (work orders through Conduit) need several projects on one engine |
| B | Authority per project: Conduits to a Project, per-project bounds, two-sided authority across projects, revocation of an actor's grants | v1, with granular Interfaces |
| C | Audiences: Pools shared to projects | v1 |
| D | Propose and approve: approvals as pending actions answered by people | v1, once a person's identity sits behind a provider (an engine-served approval page would be new infrastructure, against ruling 40) |

Ruling 53's five additions are written now in a form ready for scopes (each key carries a scope that
defaults to the engine), so nothing is written twice and the rerun does not wait on Projects.

## 3. Approvals in v0 (ruling 53, decision 2)

The project critic noted that propose-and-approve answers a different question from the one still
open: how an author confirms a change without a terminal, and how that counts. For v0 the
recommendation is that **the command is the approval**: `conduit apply` prints and records the plan
and acts; every dangerous act needs a named flag (`--reopen`, `--recreate`, `--prune`, `--replace`,
`--adopt`), counted when used; narrowing never waits. Pending actions at the root Interface carry
sign-ins and enrollment (rulings 49 and 50). Approval by people other than root is stage D.

## Decisions for the owner

1. **The short form is part of the language**, defined by translation into the unchanged kernel.
   **Recommended: yes**, instead of a CLI-only layer.
2. **The rulings the form amends**: 8 and 18 (inline definitions), 19 (bare names), and the wording
   of 31, 35 and 37 (frames optional). **Recommended: amend 8, 18, 31, 35 and 37; keep 19**, so a
   reference names its kind (no lines, only width).
3. **Grants refer to Conduit templates for reuse** (`from: read`), and operation names stay fully
   qualified where they are written. **Recommended: yes.**
4. **Names stay stable across a replacement**: an upgrade or a new directory retires the old object
   before making the new one under the same name, as Compose and Terraform do. **Recommended: yes**,
   at the price of a longer window without access at T5 and T6, measured in the rerun.
5. **Derived defaults** (ruling 53, decision 6): root's Conduit to each Pool is derived, shown and
   recorded; an Agent's turn Conduit is one written `operator` line; a Resource's owner is written.
   **Recommended: yes.**
6. **Projects**: adopt the skeleton of section 2 (sixth family, one home, inventory, audiences for
   use, two-sided authority, flat, Interfaces at the engine level), staged A in v0 after the rerun
   and B to D in v1. **Recommended: yes.** The alternative is all four stages in v0, which amends
   rulings 15, 30, 44, 49 and 50.
7. **Approvals in v0**: the command is the approval, with named flags for dangerous acts.
   **Recommended: yes**; people approving through their own Interfaces is stage D.
8. **Who may supersede a tombstone**: through an operation of `access` that a bound can withhold, so
   agents keep every management action under a grant (preservation test 5). **Recommended: yes**,
   over "root or whoever closed it".
9. **A published Type gains no default**: the Agent Types' reach flags ship as `claude-code.v2` and
   `codex.v2`. **Recommended: yes.**

## What would happen next

1. A Rust spike under `experiments/` that translates S1's sectioned files for T1 to T7 and registers
   each state through today's loader and core, so the form's claims are checked before a promise is
   written.
2. One specification batch: the sectioned form, ruling 53's five additions in scope-ready form, the
   amended rulings' text, NAM-024; then work orders for the loader's translator, the shell's plan and
   apply, and the Agent Types `.v2`.
3. The coordinator's dry run, then the fresh author's rerun under a plan written in advance,
   including how block style and flags are counted.
4. Projects stage A, as its own batch, after the rerun.
