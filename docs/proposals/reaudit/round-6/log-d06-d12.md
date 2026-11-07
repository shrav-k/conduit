# Round 6: D06 and D12, as the owner revised them

The log of the editor who applied D06 and D12 of [ruling 44](../../ruling-44/README.md), as
the section [After the ruling](../../ruling-44/README.md#after-the-ruling) revises them, on the
branch `round-6-d06-d12` from `eb58b21`. The specification governs; each choice below that goes
beyond the record can be struck by the owner.

## Changes

| Change | File | Promises | Item |
| --- | --- | --- | --- |
| CTR-002 no longer names `implementation` among the members of an Operation's `spec`; its observable says that a contract that holds it is rejected, that a template's selection or its type's default chooses what runs, and that the engine implements, with no selection, each operation that the core package's `engine` lists | `spec/promises/contracts.yaml` | CTR-002 | D06 |
| TPL-003: no key of the `implementation` map is an operation that `engine` implements; a Type's map has the same form (CTR-004 already refers to TPL-003's form); the sentence that an Operation contract may state a default is gone | `spec/promises/templates.yaml` | TPL-003 | D06 |
| TPL-004: the engine, where `engine` lists the operation; otherwise the type's default; otherwise the template is rejected. The observable says where an implementation came from (template, type or engine), cites Kubernetes's API server, says that a Pool type of any package maps none of `pool-management.v1`'s operations, and that a type's default resolves where the type is held | `spec/promises/templates.yaml` | TPL-004 | D06 |
| NAM-018 speaks of a type's default implementation, where it said a default implementation | `spec/promises/naming.yaml` | NAM-018 | D06 |
| CON-022 speaks of an operation that the engine implements, where it said one for which the target selected `engine`, since no template or type selects it now | `spec/promises/conduits.yaml` | CON-022 | D06 |
| The fourteen `implementation: github.com/shrav-k/conduit/core#engine` lines are gone; the notes say that `engine` lists the management verbs in conduit.yaml and that no template or type selects one | `spec/core/operations.yaml` | CTR-002, TPL-003, TPL-004 | D06 |
| The head comment says that what `engine` lists under `implements` is what the engine implements as a rule | `spec/core/conduit.yaml` | TPL-004 | D06 |
| The comment on the core types says that the engine implements the operations they require, where it said that each operation states the engine as its default | `spec/core/types.yaml` | TPL-004 | D06 |
| `operation-spec.v1` has no `implementation`, so the schema rejects an Operation contract that states one; its description says why | `spec/schemas/contracts.yaml` | CTR-002 | D06 |
| `implementation-map.v1` says that no key is an operation that `engine` implements, and the comment lists that check as the engine's (TPL-003) | `spec/schemas/templates.yaml` | TPL-003 | D06 |
| Every Operation contract of the package lost its `implementation`; `local-directory.v1` states `files-server` for the four operations it needs; the description of `directory.list.v1` says that `files-server` and `listing` implement it and that the contract chooses neither | `spec/schemas/examples/valid/files-package/contracts.yaml` | CTR-002, TPL-004, TPL-019 | D06 |
| `mirror.sync.v1` lost its default; `git-mirror.v1` states `#git-mirror-server` | `spec/schemas/examples/valid/project/contracts.yaml` | CTR-002, TPL-004 | D06 |
| New: an Operation contract that states `implementation`, which `operation-spec.v1` rejects | `spec/schemas/examples/invalid/CTR-002-operation-states-implementation.yaml` | CTR-002 | D06 |
| New: a Pool template that selects an implementation for `inspect.v1`, which the tool's new rule rejects | `spec/schemas/examples/invalid-by-rule/TPL-003-template-selects-an-operation-of-the-engine.yaml` | TPL-003 | D06 |
| The six sets that relied on the package's Operation defaults (four of MAN-017, two of PLC-025) now have their type state the default that the contract stated, so that the rule they are named for still rejects them; the TPL-004 set no longer speaks of the operation's own default | `spec/schemas/examples/invalid-by-rule/` | MAN-017, PLC-025, TPL-004 | D06 |
| `implementation-selected` (TPL-004) takes the engine for an operation that the core manifest's `engine` lists and reads no Operation default; new rule `engine-selected` (TPL-003); the reference check no longer expects `spec.implementation` in an Operation | `tools/check_schemas.py` | TPL-003, TPL-004 | D06 |
| `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` take the optional input `expectedId`, `{$ref: github.com/shrav-k/conduit/core#id.v1}`; each description and the notes say that a mismatch reports conflict and changes nothing | `spec/core/operations.yaml` | LIF-037 | D12 |
| New LIF-037: a request for one of those four that carries `expectedId`, whose reference resolves to an object with another ID, is not dispatched and reports conflict, with a diagnostic naming both IDs; nothing is changed. Its observable: the ID names and grants nothing, LIF-031 still applies, the check comes after access and after invalid (OPS-023), a reference that resolves to nothing reports invalid (OPS-029), and the analogy to `preconditions.uid` | `spec/conventions/lifecycle.yaml` | LIF-037 | D12 |
| The rows of selection, implementation, ID and remove; the ID and remove rows cite LIF-035 and LIF-037 | `spec/definitions.md` | CTR-002, TPL-003, TPL-004, LIF-035, LIF-037 | D06, D12 |
| The table of the nine provider and secret operations, the note on management verbs, the row of `engine`, and how a request names things | `spec/core/README.md` | CTR-002, TPL-003, TPL-004, LIF-037 | D06, D12 |
| The TPL-004 row; a new TPL-003 row; the files-package row; the counts of examples/invalid (57) and examples/invalid-by-rule (34, three with a manifest); the paragraph on the resolved form names `expectedId` | `spec/schemas/README.md` | TPL-003, TPL-004, LIF-037 | D06, D12 |
| The history paragraph says that D06 and D12 are applied, where it said they were not yet; the count of lifecycle promises (36) and of all promises (454, 208 of the layer `convention`) | `spec/README.md` | — | D06, D12 |

## Choices beyond the record

1. **The core rule is a rule with no member (D06).** The task offered two ways to bind the core
   package's operations to the engine: keep `implementation` in the core package only, reserved
   there as CTR-009 reserves `reservedToRoot`, or a rule with no member. The rule has less text:
   the core manifest's `engine` already lists, under `implements`, exactly the fourteen operations
   whose contracts stated the engine (MAN-012; ruling 41 made it so), so TPL-004 points at that
   list and needs no reservation promise, no member in `operation-spec.v1`, no check that a
   non-core contract does not hold it, and no fourteen repeated lines. The reserved member would
   have kept a second place that says the same thing as the manifest.
2. **A template or type that selects an implementation for an operation the engine implements
   is rejected (TPL-003).** The record says "implemented by the engine as a rule, with no
   mapping". Before, nothing but TPL-008 (for `set-up.v1` and `retire.v1`) stopped a template
   from selecting another implementation for, say, `inspect.v1`, since the contract's default
   came last in the order. I read "as a rule" as fixed, as Kubernetes's API server serves its
   built-in resources and no custom resource takes one over, and generalized TPL-008's rule to
   all fourteen. The alternative, which keeps a template's override, is to make the engine the
   last step of TPL-004's order instead; the owner may prefer it.
3. **`#listing` stays unselected in the valid package (D06).** The task asked that the default
   `#listing` of `directory.list.v1` move to its Type or template. The Type already chose
   `files-server` over it, so that an instance has one item that is run (TPL-019); moving
   `listing` to the Type, or to the valid template, would make a second run and an invalid
   template, which is what `TPL-019-two-items-that-are-run.yaml` shows. So the Operation's
   default is removed, the Type states `files-server` for each of its four operations, and
   `listing` stays in the manifest, selected by the TPL-019 set and by the MAN-017 set, whose
   type now states it.
4. **A reference that resolves to nothing reports invalid, not conflict (D12).** The task's
   wording gave conflict where "the object its reference resolves to has another ID, or none
   exists", and also asked to follow OPS-023's order of reasons. OPS-029 gives invalid for a
   reference that resolves to nothing, and OPS-023 puts invalid before conflict, so LIF-037
   covers a mismatch alone, and its observable says that a request whose reference resolves to
   nothing reports invalid whatever ID it expected, as Kubernetes answers a precondition on a
   missing object with Not Found (404), not Conflict (409). Making it conflict would change
   OPS-029; that is for the owner.
5. **The diagnostic names the actual ID (D12).** As the task asked. The request has passed the
   access decision by then, so the ID it shows belongs to an object the actor was permitted to
   act on; LIF-037's observable says so.
6. **`unregister.v1` keeps `revision` beside `expectedId`.** Both are preconditions; the revision
   is required and the expected ID optional. Where both fail, the reason is conflict either way.
7. **The new promise is of the layer `convention`,** as LIF-031 and LIF-032 are, beside them in
   the file of lifecycle.
8. **`inspect.v1`, which changes nothing, still reports conflict on a mismatch,** so that an
   actor that inspected an object and then inspects it again by name learns that the name now
   names another, rather than reading the other's status as the first's.

## Coined name

- `expectedId`: the optional input of `set-up.v1`, `retire.v1`, `inspect.v1` and
  `unregister.v1` that carries the ID the actor expects, typed by the core package's `id.v1`.
  It follows the camel case of the core inputs (`nodeSelector`, `packageArguments`) and
  Kubernetes's `preconditions.uid`. The owner may strike or rename it; it is not one of the names
  in `spec/names.yaml`, so `tools/set_names.py` does not manage it.

## For the tracker and the decisions

Not edited here, since another editor owns them: `docs/AUDIT.md` should mark action A69 done
for D06 and D12, with this log, and `docs/proposals/reaudit/round-6/DECISIONS.md` should record
D06 and D12 as applied as revised, citing CTR-002, TPL-003, TPL-004 and LIF-037, and choices 2
and 4 above as points the owner may strike.

## Checks

Run on the commit of this log: the validation of
[CONSOLIDATION.md](../CONSOLIDATION.md#validation) (454 promises ok), `check_docs.py`,
`set_names.py --check`, `check_schemas.py` (Errors: 0) and `git diff --check`, all passing.

## A check of the merge

On 2026-11-07, after this branch was merged into `audit/round-6-codex`, a check of the merge
found one gap and five small points, all fixed: TPL-004's `then` now says that where a promise
speaks of the implementation selected for an operation it means the one the template's map or
TPL-004 gives, the engine among them, so CON-022, ACC-015 and the promises that rely on them
still reach the engine's operations; `spec/README.md` counts 57 and 34 example files; the checker
takes the engine first, as TPL-004 does; TPL-017 and TPL-018 say that no Operation contract
states a default; TPL-003 names the `implementation` map; and the core README's tables cite
LIF-037 for the four actions. Every check passes, with 454 promises.
