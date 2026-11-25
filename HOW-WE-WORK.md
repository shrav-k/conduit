# How we work

Status: draft, not yet in force. Until [AGENTS.md "What holds"](AGENTS.md#what-holds) names this
file, the rules hold where they are stated today, and this file adds none. The owner chose a
separate file for the rules of the work, at the root, ruling 92's rules first
([DEC-001](docs/decisions/log.md#dec-001-2026-11-20-the-decision-framework),
[DEC-028](docs/decisions/log.md#dec-028-2026-11-20-ruling-92s-rules)).

The form: one rule per line, current rules only, at most 12 KB in all. Each line ends with the
decision it comes from (`DEC-NNN`, or `ruling N pt M`) and one label saying how it is enforced:

- `checked: tools/…`, a check in the gate;
- `refused: <operation>`, a Conduit operation that refuses the breach;
- `by review`, the read of each order, until a check or a refusal replaces it.

A rule names no date and no order number. Conduit's own behavior is not here: it is in
[`spec/`](spec/README.md).

## 1. The rules of the work

1. **Purpose gate.** Every order names the [preservation tests](docs/AUDIT.md#preservation-tests) it serves; an order that adds a per-Agent copy of a shared thing fails test 3 and is refused; an order over 20 KB is a sign that the engine should do the work, not a document. (DEC-028, ruling 92 pt 1) `by review`
2. **No private code.** The engine is Rust, and a package may be written in any language; other code in the tree is Rust, and the Python checkers in `tools/` and the Workflow tool's JavaScript are the named exceptions. Code in the tree is tested against the real binary, private values live only in configuration outside the tree, and a gap in the engine is a finding and an order, never a script. (DEC-025, DEC-028, DEC-111, ruling 92 pt 2) `by review`
3. **"Should this exist", once, before dispatch.** Each order answers which existing tool does this and, if none, which promise makes it Conduit's; "none" is a finding against the contract, not a licence to build. (DEC-028, DEC-003, ruling 92 pt 3) `by review`
4. **Token discipline.** An order gets one read, of its diff, after the local gate passes, and no read rounds on order texts; a must is a failing case or test, not a prose finding; every run records its tokens; the target is a fresh reader's orientation under 10k tokens. (DEC-028, ruling 92 pt 4) `by review`
5. **Milestone check-ins are scenarios**: S2 live, A78's stage 2 through Conduits, a provider called; between them the owner sees one ratio each week, of lines under `spec/` and `engine/` to lines under `docs/work-orders/` and `docs/proposals/`, and if it falls, orders stop until it recovers. (DEC-028, ruling 92 pt 5) `by review`
6. **The coordinator's decisions are not the owner's.** A decision the owner did not make is a Record line marked as the coordinator's, which the owner may strike until the owner confirms it, and never a log entry. (DEC-029, DEC-028, ruling 92 pt 6) `by review`
7. **One place per fact.** A fact is stated in one place and cited, not restated, everywhere else. (DEC-028, ruling 92 pt 7) `by review`
8. **A scenario passes only when the access it exists to show was exercised**: a refusal, a close, a shared thing never copied. (DEC-028, ruling 92 pt 8) `by review`

## 2. Who decides and who does

From [DEC-029](docs/decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does).

## 3. Branches, gates and merges

From [DEC-030](docs/decisions/log.md#dec-030-2026-11-20-how-an-order-runs-and-merges).

- A worker runs the whole engine suite at most once, at the end; a verifier or reader runs only
  what settles a doubt; the gate runs everything before each merge. `by review`
- An order that changes a modelled promise's text marks its row of 012's account "text changed,
  not rerun". `checked: tools/check_model_account.py`
- A commit passes the gate only when its author and its committer are each `Conduit work
  order <work-orders@conduit.invalid>` or a human contributor that [CONTRIBUTORS](CONTRIBUTORS)
  lists at the merge base. (DEC-089) `checked: tools/gate.py`
- Agents and the coordinator commit as the work-order identity; a listed human commits under
  their own name and opens their branch's pull request, and its rebase and squash keep them as
  author; only the owner approves a line of `CONTRIBUTORS`. (DEC-089) `by review`

## 4. Proof: scenarios, experiments and measurement

From [DEC-026](docs/decisions/log.md#dec-026-2026-11-20-proof-cases-experiments-measurement).

## 5. The dogfood

From [DEC-027](docs/decisions/log.md#dec-027-2026-11-20-conduit-builds-conduit-directly-through-conduit).
Rules about the owner's own accounts and machines are private, held by the coordinator.

## 6. Model access and allowances

From [DEC-019](docs/decisions/log.md#dec-019-2026-11-20-model-access-subscriptions-now-a-gateway-route-later).

## 7. Scope

From [DEC-004](docs/decisions/log.md#dec-004-2026-11-20-the-scope-guard-and-v0s-scope).

## 8. Records: where each fact lives

1. **Git is the archive.** A retired file leaves the tree, and a link to it points at its last commit. (DEC-031) `checked: tools/check_docs.py`
2. **The archive's index** ([docs/archive/README.md](docs/archive/README.md)) keeps one line per retired item. (DEC-031) `by review`
3. **Tracking stays in the repository**, as rows of [ROADMAP.md](ROADMAP.md), with no GitHub issues for now; the scheme for tracking is discussed when the repository becomes a GitHub organization. (DEC-031) `by review`
4. **Changelog fragments.** A change to `spec/` writes its lines in its own fragment under `spec/changes/`, in the form [spec/CHANGES.md](spec/CHANGES.md) states; a release moves them into that file. (DEC-177) `checked: tools/check_decisions.py`
5. **No stored counts.** The READMEs of `spec/`, `spec/core/` and `spec/schemas/` state no count; the checkers print the totals. (DEC-177) `checked: tools/check_schemas.py`

## 9. Publication and organization

From [DEC-005](docs/decisions/log.md#dec-005-2026-11-20-v0s-gate-publication-and-stewardship) and
[DEC-021](docs/decisions/log.md#dec-021-2026-11-20-how-packages-are-organized).

## Waiting for the owner

In [the tracker](docs/AUDIT.md#waiting-for-the-owner).
