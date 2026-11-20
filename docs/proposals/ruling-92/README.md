# Ruling 92: eight rules for the work going forward, D62 reversed, two streams of orders in parallel, and the rulings consolidation next

Status: in force; directive (A89: the rulings consolidation, point 10, beside point 9's two streams of orders) (ruled by the owner on 2026-11-20, as the coordinator's notes date the answers, at about 02:40 PST). Holds now: [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed), D62's row in [the full review's decisions](../full-review/decisions.md), and the row of A89 in [the tracker's action items](../../AUDIT.md#p2--prune-and-re-baseline-under-the-scope-guard). This record says why.

On 2026-11-20 the owner asked for a triage of the repository against what Conduit is for, after
finding that the coordinator had built shell tooling outside the engine. A Fable session wrote the
triage (ruling 79); its section 6 proposes eight rules for the work going forward, and the
coordinator's notes (private) hold it. The coordinator then put four questions to the owner through
the session's question tool, each with options and a recommendation. The owner's words, as the notes
hold them, follow verbatim.

## The owner's words

At about 02:00 PST:

> "Why does ruling 15 say that a "checkout" is something a pool should own? We also need to plan to
> go through all rulings to consolidate and update them to be consistent with the current version
> of conduit"

At about 02:40 PST, after the Fable triage (AskUserQuestion):

> 1. Next build priority -> "Both in parallel" (S2 on one lane, the dogfood's engine gaps on the
>    other).
> 2. The going-forward rules as one ruling -> "Adopt all eight (Recommended)" — the option read:
>    "One short ruling. It also reverses D62, so code I write is Rust in the repository." The eight:
>    purpose gate; no private code; "should this exist" before dispatch; token discipline; milestone
>    check-ins as scenarios; the coordinator's decisions are not rulings; one place per fact; a
>    scenario passes only when its access was exercised.
> 3. Private folder cleanup -> "Yes, clean up (Recommended)" — the option read: "Keep the stage-2
>    store and its signed-in Agents, the current worktrees, and the notes the triage cites. Delete
>    the rest."
> 4. Rulings consolidation -> "Start after this turn (Recommended)" — "On Opus, lean."

The parentheticals and the list after "The eight:" are the coordinator's notes on what the options
referred to; only the quoted labels and option texts are what the owner read and chose.

Answer 3 concerns the coordinator's private folder, outside the repository, and decides nothing
here.

## The decisions

Points 1 to 8 are the eight rules, each restated from the triage's section 6.

1. **Purpose gate.** Every order names the [preservation tests](../../AUDIT.md#preservation-tests)
   it serves. An order that adds a per-Agent copy of a shared thing fails test 3 and is refused. An
   order over 20 KB is a sign that the engine should do the work, not a document.
2. **No private code.** Code the coordinator writes is Rust in the tree, tested against the real
   binary; private values live only in configuration outside the tree. The Python checkers in
   `tools/` are the named exception, and a gap in the engine is a finding and an order, never a
   script. The full review's D62, that the runners stay scripts outside the tree, is reversed.
3. **"Should this exist", once, before dispatch.** Each order answers: which existing tool does
   this, and if none, which promise makes it Conduit's. "None" is a finding against the contract,
   not a licence to build (ruling 40).
4. **Token discipline.** An order gets one read, of its diff, after the local gate passes, and no
   read rounds on order texts. A must is a failing case or test, not a prose finding, and every run
   records its tokens. The target: a fresh reader's orientation under 10k tokens.
5. **Milestone check-ins are scenarios**: S2 live, A78's stage 2 through Conduits, a provider
   called. Between them the owner sees one ratio each week: lines under `spec/` and `engine/`
   against lines under `docs/work-orders/` and `docs/proposals/`. If it falls, orders stop until it
   recovers.
6. **The coordinator's decisions are not rulings.** A decision the owner did not make is a Record
   line marked *Coordinator's*, which the owner may strike, and not a numbered ruling with a
   directory.
7. **One place per fact.** A fact is stated in one place and cited, not restated, everywhere else.
8. **A scenario passes only when the access it exists to show was exercised**: a refusal, a close,
   a shared thing never copied.
9. **Two streams of orders run in parallel next** (answer 1): S2 in one, the engine gaps that the
   dogfood found in the other.
10. **The rulings consolidation starts next** (answer 4), on Opus and lean: every ruling is gone
    through and made consistent with the current version of Conduit.

*Open*, the owner's: whether the Workflow tool's JavaScript is a further exception to point 2. The
triage names it as one only if the owner agrees, and the question did not put it.

## The coordinator's readings

Each is *Coordinator's*, open to the owner's strike (point 6).

- **Point 4 and the reads.** The one read is the fresh Opus read after the gate (rulings 79 and 85).
  The Codex round keeps its worker and drops its verifier, so ruling 87's point 1 loses the
  verifier and its point 2, which judges the verifier's report, falls. A must is made a failing
  case or test, the Opus fixer makes it pass and the gate reruns, with no reread: ruling 85's
  point 2 and ruling 79's read of the coordinator's fixes go. A should or nit is applied by the
  fixer or dropped. Ruling 85's point 1 stands: an order whose read finds no must merges on the
  gate, and one with a must waits for the coordinator. A77's paired reads keep their registration,
  since reads are that experiment's subject.
- **Point 2 and what is already running.** The stopgap turn on the shell harness that the owner
  approved before this sitting stands, as temporary, until its Rust replacement lands.
- **Point 7's homes.** Which document is the home of which fact is the consolidation's work
  (point 10). The triage's allocation (`spec/` binds, IN_FORCE.md cites IDs and never restates, the
  ledger holds the owner's words, an order's record is its merge, status lives in the tracker) is
  its input, and changes no line now.
- **Point 5's ratio.** The triage measured it at 0.58 on `main` at `b897d316`, the first reading.
  It falls when a week's reading is below the week before.
- **Point 9's streams.** The option's "lane" is read as a stream of orders, not as a lane
  coordinator: the admin session dispatches both streams, and until the S1 rerun's pin no lane
  coordinator runs beside it, as ruling 78's point 1 holds.

## What it amends

- **D62** of the full review: reversed (point 2).
- **Ruling 79**: the coordinator's fixes are not reread (point 4).
- **Ruling 85**: point 2's reread of shoulds goes (point 4).
- **Ruling 87**: point 1's round has no verifier, and point 2 falls (point 4).

## What it does not change

- Ruling 85's point 1, the merge without the coordinator after a read with no must.
- Ruling 89's dogfood milestones; point 5 sets check-ins, not milestones.
- Ruling 3: exploratory code is Rust, as point 2 now has all of the coordinator's code.
- Ruling 78's point 1: one coordinator until the S1 rerun's pin, with no lane beside it; point 9's
  two streams are both the admin session's (the coordinator's reading).

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md), topic 15: the line on who reads an order is replaced, and a
  lane coordinator's merge waits on the fresh read, not a verifier; one line holds points 1 to 8
  and the open question, one holds points 9 and 10, and D62 joins the line of what the rulings
  settle of the full review.
- [The ledger](../ruling-log.md) gains row 92, and the rows of 79, 85 and 87 their amendments.
- [The tracker](../../AUDIT.md#p2--prune-and-re-baseline-under-the-scope-guard) gains A89, the
  rulings consolidation (point 10), and [the roadmap](../../../ROADMAP.md#the-order-of-the-work)
  lists it in the work in parallel; step 5's row marks S2 next (point 9).
- [Work orders](../../implementation/WORK_ORDERS.md#roles): the coordinator's row and the section
  on dispatch and checking still describe the Codex verifier and the rereads. A follow-up edit, in
  its own pull request, brings them in line with point 4.
