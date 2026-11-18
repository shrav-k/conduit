# Work order 201: MC9a's pre-registration, pending actions and sign-in as written

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since this order adds and changes no case. This is
a paper order: it changes one document. It has no network access and needs none. It needs no
cargo command; run none. Never print a whole large file: read
`experiments/012-model-checking/README.md` and the orders it names with `grep -n`, `sed -n`,
`head`, `tail` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Do not run
`claude`, `codex` or any client, and sign in nowhere.

## What and why

The full review of 2026-11-14 puts a sign-in model ahead of the sign-in build: row 2 of
[FR-446](../proposals/full-review/findings/G1a-model-checking-register.md#fr-446), "MC9a: pending
actions and sign-in as written", as its verifier corrected it, is order 24 of
[the review's orders](../proposals/full-review/orders.md), with
[FR-448](../proposals/full-review/findings/G1a-model-checking-register.md#fr-448)'s PND-008 and
PND-013 and [FR-433](../proposals/full-review/findings/E04-signin-isolation.md#fr-433)'s two
sign-ins of one instance at once. Step 3 of
[the review's list for experiment 012](../proposals/full-review/experiments.md#experiment-012-the-runs-each-registered-before-it-runs)
registers MC9a "when order 048a-2 is drafted". MC7b's pre-registration left pending actions to
MC9, and G5 holds thirteen promise rows `left to MC9`: the ten pending-action rows, with PND-008,
PND-013 and PND-014.

[The targeted review's section 6](../proposals/review-signin-2026-11-16/README.md#048a-2), 048a-2
item 1, and [ruling 82](../proposals/ruling-82/README.md), point 6, o4, refuse a second
`sign-in.v1` of one instance as `conflict`, and the 048a-2 line of
[IN_FORCE.md](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach)
asks that "MC9a is registered with that reason before 048a-2 is dispatched". Order 048a-2,
written beside this one as `docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md`, writes
that sentence into PND-010's `then` and checks, before it starts, that
`experiments/012-model-checking/README.md` holds a heading that begins `## MC9a`.

This order writes MC9a's pre-registration into that README, in the form of MC6's and MC7b's: the
texts and commits it reads, its nodes and scripts, its fault model and configurations, its
accounting, each property's quoted slice and predicate, its mutants, its expected tables, the
findings it registers, and its limits. It registers only. It writes no model code, runs nothing
and records no result: the order that writes `src/pending.rs` and runs it comes later, under order
24 of the review.

**Who decided what.** Every decision below is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the model's two parts, its scripts and bounds, its
configurations, its properties, mutants and expected verdicts, the reading of "with every fault
switched off" as the configuration Base, the reading of experiment 012's rule in the review's
`experiments.md` (a mutant that refutes its property on that single schedule checks it in one
step) that a clause its mutant refutes in Base is reported "one step", to a vector, and not as
holds, the readings that the findings MC9a-001 to MC9a-003 register, the status line's new
clause, and leaving `ROADMAP.md` unchanged (decision 5). Apply them as written and report them
under `decisions` as the coordinator's.

**Amended 2026-11-18.** Round 2 changed decision 4's block, to answer round 1's verifier: new
witnesses 23 to 26, the mutant RetainHold, a computed setup action's ledger identity by the setup
ending that computed it, G's route to open the candidate through an `access` Conduit, and the two
future vectors it names, `requests-sign-in-during-sign-in-conflict` and
`requests-sign-in-during-retirement-conflict`. The readers' findings of the same day changed it
again: the first added MC9a-002's reading that an `O` found by a sign-in after A's retirement has
been dispatched raises no action (PND-003), and P01's matching exception; the second made K11
require success where the check or polling finds the expected account, limited R13's passed return
to the sign-in-hook.v1 call that started the login, and limited P01's required raise to a turn
decided while A is ready. The third tied R13's passed return to a `sign-in-hook.v1` retry of the
part that started the login, added that retry to the ledger, and put the requestState of H's call
outside the model. The coordinator adopts those
changes, each open to the owner's strike as the rest; the README's section, not the block below, is
the registration, and its verdict rows are 23.

**Read first:** `AGENTS.md`; this order; in `experiments/012-model-checking/README.md`, lines 1 to
24 and the headings (`grep -n '^#'`); order 048a-2's decisions 1, 2, 4, 6 and 7. You need not read
`spec/`: the block below quotes it, and a done-when check compares each quotation with its source.

## The decisions it applies

1. **Order 048a-2 is on the base, as written, and MC9a is not yet registered.** Before anything
   else, from the repository root, run these commands. Each prints what its comment says. If one
   prints anything else, change nothing and report which: the coordinator dispatches this order
   only from a base on which each prints that.

```sh
F=docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md
git status --porcelain | grep -c .                              # 0
git cat-file -t f72c2bc3                                        # commit
git merge-base --is-ancestor f72c2bc3 HEAD; echo $?             # 0
git log -1 --format=%H HEAD -- $F | grep -c .                   # 1
grep -c '^## Record' $F                                         # 0
grep -c '^1\. \*\*PND-005 gains F84.s two halves\.\*\*' $F      # 1
grep -c '^2\. \*\*PND-010.s `then` gains the conflict\.\*\*' $F # 1
grep -c '^4\. \*\*The sign-in kind, stored\.\*\*' $F            # 1
grep -c '^6\. \*\*`sign-in.v1`.s invalid and conflict, before the stub\.\*\*' $F   # 1
grep -c '^7\. \*\*Retirement answers it void\.\*\*' $F          # 1
tr -s ' \n' '  ' < $F | grep -c 'A sign-in.v1 for an instance whose earlier sign-in.v1, setup or retirement has been dispatched and has not ended is not dispatched and reports conflict, with a diagnostic naming PND-010 and, where the caller may see it (PND-002), the other invocation.s pending action.'   # 1
tr -s ' \n' '  ' < $F | grep -c 'A contract reports a missing or lapsed login in one of two forms: an implementation that carries Conduit.s keys ends the call failed or unknown, as it can tell, with an error diagnostic naming PND-005; a plain one reports it only by a URL request, under IFC-016.'   # 1
grep -c '^## MC9a' experiments/012-model-checking/README.md     # 0
```

2. **Two values the block takes from the base.** In the block of decision 4 and the line of
   decision 3, replace every `DATE` with the output of `date -u +%F`, and every `C048A2` with the
   output of this command, eight hexadecimal characters:

```sh
git log -1 --format=%H HEAD -- docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md | cut -c1-8
```

   Replace nothing else. Do not rewrap the block: its lines of prose are at most 100 characters,
   and each link sits whole on one line, since `check_docs.py` checks a link only when its text and
   its target share a line. Its links are written as they serve from
   `experiments/012-model-checking/`.

3. **The status line.** In `experiments/012-model-checking/README.md`, line 3, the line that
   begins `Status: executed 2026-11-11`, replace its last words

```text
MC2 rerun 2026-11-15 under work order 160.
```

   with

```text
MC2 rerun 2026-11-15 under work order 160; MC9a registered DATE under work order 201, not run.
```

   Nothing else on that line, or above the new section, changes.

4. **The registration is a new section at the end of the README.** After the file's last line,
   "and no file outside this work order's scope needs a change for these two fixes.", add one blank
   line and then this block, word for word, ending with a single newline:

````markdown
## MC9a — pending actions and sign-in as written: pre-registration

Registered DATE by work order 201, before any MC9a code exists and before any run. Nothing
here is a result. The reference model will be `src/pending.rs`, two protocols on the existing
harness, importing no engine code; a later work order writes it and runs it. The expected tables
are immutable: an observation that differs is recorded beside them, and a bound or model change
found necessary before the first run is registered as a dated revision before that run, as
MC7a's were.

MC9a is row 2 of
[FR-446](../../docs/proposals/full-review/findings/G1a-model-checking-register.md#fr-446),
as its verifier corrected it, with PND-008 and PND-013 from
[FR-448](../../docs/proposals/full-review/findings/G1a-model-checking-register.md#fr-448)
and the two sign-ins of one instance at once of
[FR-433](../../docs/proposals/full-review/findings/E04-signin-isolation.md#fr-433).
It registers order 048a-2's `conflict` for a second `sign-in.v1` of one instance, as
[the targeted review's section 6](../../docs/proposals/review-signin-2026-11-16/README.md#048a-2),
048a-2 item 1, and [ruling 82](../../docs/proposals/ruling-82/README.md), point 6, o4, ask,
before 048a-2 is dispatched. Its accounting is the thirteen rows that G5 leaves to MC9: the ten
pending-action rows, PND-008, PND-013 and PND-014. MC9b, for grants, keep, withhold and
appointment, waits for batch 2's text.

### MC9a: the texts it reads, and their commits

- `spec/` at `f72c2bc3`: every promise slice quoted below, from `spec/promises/pending.yaml`,
  `spec/conventions/operations.yaml` and `spec/interface/implementation.yaml`.
- [Order 048a-2](../../docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md)
  at `C048A2`, as PND-010 and PND-005 will read once it merges: its decision 2, the sentence it
  adds to PND-010's `then`; its decision 1, F84's two halves in PND-005's `observable`; its
  decision 4, a sign-in action stored with the invocation that raised it and its expiry, and
  answered by no `answer.v1`; its decision 6, a conflict's diagnostic naming the instance's
  unanswered sign-in action only where the caller's view of it is not none; and its decision 7,
  retirement answering that action void.
- Orders 047, 047b and 047c as written at `f72c2bc3` (047 merged at `3aafbd93`; 047b and 047c
  written and not yet built): 047's setup action, computed and never stored, its ID the setup
  invocation's, raised while a step's time is before its expiry and not at it; 047b's scope, from
  the Interface's open inspection Conduits, `list.v1`'s `pending`, the `unanswered` key on an
  Interface's own results only when nonzero, and nothing raised at an Agent; 047c's `answer.v1`
  on an Interface's own behalf, its raised-at test without the time test, refused before invalid,
  conflict after access for an answered or expired action, `retried` by a further setup, a
  dismissal or expiry that stays answered, and answered state that survives a purge.

A promise whose text changes after these commits makes its row "text changed, not rerun" until
MC9a is registered again against the new text (D29).

### MC9a nodes, scripts and bounds

Each part has three nodes. Q sends the requests of the Interfaces and of the Agent, each labelled
with its caller; E is the engine; H implements the part's instance: its hooks, its turn and its
run's stop, and, for sign-in, the login program and the account in the instance's home. E saves
instances, lifecycle, Conduits, invocations with their dispatch and end, stored actions and their
answers, the hold, `SignInUndone`, consumed requestStates and records. Only a return's value is
volatile, from the retry that carries it to the `sign-in-hook.v1` retry that passes it on. Crash
erases volatile state; recovery restores saved state, dispatches nothing again, and ends unknown
every invocation in progress at the crash, sending its caller that result (IMP-007). H saves its
account and its login programs; the person's acts at the login page are H's local choices. Q
answers each requestState it receives at most once, and sends a stage only once the answers of
the stage before have come. Q sends a bracket's requests one at a time, in either order, a local
choice of Q; ReorderQ's fault reorders them once they are on the link. Each request in the
scripts carries its own idempotency key; only CopyQ's duplicate repeats a request. A copy
delivered after its invocation has ended is answered as OPS-009 says: from the retained outcome
where it succeeded, by OPS-034's conflict where nothing is retained, by OPS-012 where it is
unknown, and as a new attempt under OPS-024 where it failed. A new attempt is a dispatch for X10
and U06, and U06 reads an answer as that of the ending it answered.

Expiry is time-based: an action with no answer at or past its expiry reads expired. The external
ledger records each request's decision, with its reason, its diagnostics' rule IDs and any action
ID it names; each dispatch and end with its outcome; each part and its result; each action
raised or answered; each pass of E's clock past an action's expiry, naming the action, whether or
not the action has an answer; an `expired` answer is recorded only for an action with no answer
at that moment; each `list.v1` result with its actions and its `unanswered` count; each
requestState issued, accepted or rejected; each return passed on or refused; each emission's
content, as symbols; each Conduit opened or closed; each purge; and each crash and recovery. An
action that an ending raises or computes is logged before that ending's result. Every predicate
reads only the ledger, and recomputes what the text requires from the stimuli
and the earlier entries there. None reads node data, a queue, a fault counter,
or a value that the model wrote for the property's own use, as the full review's
[step for experiment 012](../../docs/proposals/full-review/experiments.md#experiment-012-the-runs-each-registered-before-it-runs)
forbids. The test replays every first refutation after erasing every node and queue.

**Sign-in.** Agent A of Fleet F. Its Type states `signIn` with the expected account `E` at
`/account`, and one login L: `redirect`, `ports: any`, `carried: true`. Its `sign-in-hook.v1`
implementation carries Conduit's keys. R is root's Interface; root's Conduit to F, from
`root-fleet`, lists `set-up.v1`, `retire.v1`, `sign-in.v1`, `list.v1` and `answer.v1`.
Interface X holds an open inspection Conduit to F, an open Conduit to F listing `list.v1` and
`sign-in.v1`, and an open Conduit to A listing A's turn, an operation that starts model work; its
scope holds F and A.
Interface Y holds an open Conduit to F listing `list.v1` and `sign-in.v1`, and no inspection
Conduit. A's home starts with no account. H reports an account as the home stores it: `e`, the
expected account in another ASCII case, or `O`, another account. Each stage waits for the
answers to the stage before; requests in brackets are sent together:

1. R sets A up. Its `account-hook.v1` part reports no account, so sign-in action s1 is raised;
   its setup hook succeeds, and A is ready.
2. X asks for A's turn; then Y asks `sign-in.v1` for A.
3. [R: `sign-in.v1` for A; X: `sign-in.v1` for A]. The one dispatched, W, runs
   `account-hook.v1`, which reports no account, and then `sign-in-hook.v1`: H starts L and
   answers with its URL, which E relays to W's caller with requestState rs1. A second
   invocation's URL, where one is dispatched here, is never completed, so Q sends no retry for it.
4. The person completes L as `e` or as `O`, a local choice. W's caller retries with return v1 and
   rs1; E checks rs1, then PND-013's checks, and passes v1 on in a `sign-in-hook.v1` retry. H's
   program takes it, and the home's account becomes the choice. Before the account appears, E
   may answer with a further input request and rs2, at most once in a run, in any invocation, so
   that requestStates are at most four in a Correct run; the caller then retries with v2, a
   second paste. E answers the v2 retry only when polling ends the invocation, or at DL18's end
   in Wait, and issues no further requestState for it. Polling ends W:
   `e` succeeds and answers A's unanswered sign-in action signed-in; `O` dispatches
   `sign-out-hook.v1`, which succeeds or, at most once in a run, fails, and W ends failed.
5. If W did not succeed, R asks `sign-in.v1` for A again, W2, which runs as W does with return v3
   in place of v1, and the person completes L as `e`. Where the sign-out failed, W2's check
   before `sign-in-hook.v1` finds `O`, and polling counts it as no account yet.
6. X asks for A's turn. H ends it succeeded or, at most once in a run, failed with an error
   diagnostic naming PND-005, a keyed lapse, which leaves the home with no account and raises s2.
7. [R: `retire.v1` for A; X: `sign-in.v1` for A]. A sign-in dispatched here finds `e` and
   succeeds at once or, after the lapse, finds no account and relays a URL that no one completes.
   X's `sign-in.v1` decided after A's retirement has ended gets no reason from the text, so the
   model ends that branch at its decision, without a decision X10 checks.

E's clock passes at most one expiry in a run, of any sign-in action raised, as a local choice of
E at any point from that action's raising to the end of the script. A sign-in action raised after
that expiry, by a held request or by W2's check finding `O`, is s3; at most three sign-in actions
exist.

**Actions.** Agent B of Fleet F, whose Type states no `signIn`, and the machine provider M,
owned by `system`, with the machine action m1 raised in the initial state. R, X and Y are as in
sign-in, without X's Conduit to A, with root's Conduit to M as the core gives it, and `answer.v1`
in place of `sign-in.v1` in X's and Y's Conduits to F. Agent G of F
holds an open Conduit to F listing `inspect.v1` and `list.v1`. Stages:

1. R sets B up; its setup hook fails with no effect, so setup action u1 is computed.
2. G asks `list.v1`; then G asks to open a Conduit to F listing `answer.v1`. G's open is decided
   by PND-009 alone, and witness 2 requires its refusal to name PND-009.
3. [X: `list.v1`; Y: `list.v1`]; then R asks `list.v1`.
4. Y answers u1 `dismissed`; then X answers u1 `acknowledged`.
5. R closes X's inspection Conduit; then X asks `list.v1`; then X answers u1 `dismissed`.
6. R sets B up again: its dispatch answers u1 `retried`, and its hook fails again, so setup
   action u2 is computed.
7. R answers u2 `dismissed`.
8. R purges the records made before this stage; then R answers u2 `dismissed`; then R asks
   `list.v1`.

E's clock passes u2's expiry at most once in a run, as a local choice of E that may not happen,
at any point from u2's computing to the end of the script, and passes no other expiry in this
part.

Each run is bounded by at most 8 messages on each directed link, 400,000 unique states and 15
seconds, with two checker threads for a full run and one for a first refutation. Sign-in has one
Agent, one Fleet, three Interfaces, at most three sign-in actions, three returns (v1, v2 and v3),
at most four requestStates, three accounts (none, `e` and `O`) and at most two login programs
waiting at once. The program and requestState bounds count what the Correct script reaches; a
mutant may exceed them, and the model puts no cap on them. Actions has two
instances and an Agent, one Fleet, three Interfaces, three actions and one purge. Scripts and
local choices are finite; no retry, wire copy or clock is unbounded.

### MC9a configurations and fault windows

| Configuration | Parts | Enabled fault |
| --- | --- | --- |
| Base | both | none; the clock, the person's choices, a failed sign-out and a lapse are local choices, not faults |
| LossQ | both | one loss, on Q to E or E to Q |
| ReorderQ | both | one reorder, on Q to E, among a stage's bracketed requests |
| CopyQ | both | one duplication, on Q to E or E to Q |
| Crash | both | one crash of E and its recovery |
| Wait | sign-in | one timer of E: DL18's end, which ends unknown the oldest `sign-in.v1` in progress |

Every enabled harness fault has its harness witness in its own full run. All other budgets are
zero: faults on the links between E and H are MC2's and MC6's questions of dispatch; a cut, its
repair and a lost node are MC4's; a crash of H is not modelled, since no slice here names an
implementation's restart; and no timer runs outside Wait, since DL18's end is the only timer
these slices give. Nothing retries a lost request or answer of its own accord; a lossy branch can
stop, and safety applies to its prefix. Base is the configuration “with every fault switched off”
of the full review's step for experiment 012.

### MC9a catalogue accounting (13 promises)

Each target promise occurs once. "Modelled" means the stated bounded projection and property,
not every clause of the promise.

| Promise | Account | Property / reason |
| --- | --- | --- |
| PND-001 | modelled | P01, for the sign-in action; U06 carries the setup action |
| PND-002 | modelled | S02 |
| PND-003 | modelled | A03 |
| PND-009 | modelled | A09 |
| PND-012 | modelled | D12: answered state survives a purge. That a record names the action's ID and holds no prompt or return is a static decision, to be shown by vectors |
| PND-005 | modelled | H05 (the hold) and P01 (raising) |
| PND-010 | modelled | X10, with the sentence of order 048a-2's decision 2 |
| PND-011 | modelled | K11 |
| PND-013 | modelled | R13 |
| PND-014 | modelled | V14, what is emitted. What stays in the engine's memory is not in the ledger and is not modelled; the case `requests-sign-in-return-kept-nowhere` names it |
| PND-006 | modelled | U06 |
| PND-007 | not modelled | A static decision, to be shown by vectors: a notice is raised within one retirement step, by order 047d, not yet written, and its answer by `answer.v1` follows A09 and A03 as the setup action's does |
| PND-008 | modelled | S02: machine actions are raised at root alone. Their answers by `admit-machine.v1` and `refuse-machine.v1` are the enrollment model's |

Totals: **12 modelled; 1 not modelled, a static decision to be shown by vectors**. Supporting,
outside the count: PND-004 (interface layer: evidence and withheld codes), IFC-016, IFC-049 (the
requestState), IMP-007 (an invocation in progress across a restart ends unknown), OPS-009,
OPS-012, OPS-024, OPS-032 and OPS-034 (repeats), OPS-023 (the order of reasons), REV-014, REC-004
(purge), ACC-033 (the target that authorizes inspection), DL18 and DL19.

### MC9a property text and preconditions

Each property reads the slices quoted with it, from the commit named before them; `must not` is
stated where the level gives the negation. Each applies to the stated finite script.

**X10 (PND-010, must).** At `f72c2bc3`, PND-010's `then`: “The engine implements it only for the
Interface acting on its own behalf, authorized through the Agent's owning Fleet or the Resource's
owning Pool (ACC-033). A caller other than root signs in only an Agent of a Fleet in its scope;
Resources remain root's.” At `C048A2`, order 048a-2's decision 2, the sentence PND-010's `then`
will hold: “A sign-in.v1 for an instance whose earlier sign-in.v1, setup or retirement has been
dispatched and has not ended is not dispatched and reports conflict, with a diagnostic naming
PND-010 and, where the caller may see it (PND-002), the other invocation's pending action.” Its
decision 6 reads “the other invocation's pending action” as the instance's unanswered sign-in
action. Supporting, at `f72c2bc3`, OPS-032's `then`: “It is a repeat. Nothing is dispatched, and
it reports conflict.” Predicate, over every decision on a `sign-in.v1` for A: (a) it is
dispatched only where its caller is R, or an Interface whose scope, from the inspection Conduits
that the ledger shows open at the decision, holds F; (b) where (a) permits it and the ledger
holds a dispatch of a `sign-in.v1`, `set-up.v1` or `retire.v1` of A with no end before the
decision, it is not dispatched and reports conflict, with a diagnostic naming PND-010 that names
A's sign-in action exactly where one is unanswered at the decision and the caller is R or its
scope holds A, or, where it is a copy of that `sign-in.v1`'s request, with OPS-032's diagnostic
and that invocation's ID, naming no action; (c) no decision reports conflict naming PND-010 where
no such dispatch is open. A setup or retirement asked for while a `sign-in.v1` of A is in progress
is outside X10 (MC9a-002).

**K11 (PND-011, must).** At `f72c2bc3`, PND-011's `then`: “Only the expected account, compared
ignoring ASCII case, completes sign-in.” and “During polling after its URL elicitation, the
expected account succeeds with output account and answers the action signed-in; the account that
the check before sign-in-hook.v1 found counts as no account yet; any other account is undone by
a sign-out-hook.v1 part, sign-in.v1 ends failed naming both accounts, and the action stays
pending with the attempt as evidence.” Its `observable`: “If sign-out does not succeed, the
engine reports SignInUndone=False with its cause”. Predicate: (a) every `sign-in.v1` of A that
ends succeeded, and every answer signed-in, comes after an `account-hook.v1` result within that
invocation whose account equals `E` ignoring ASCII case; (b) within a `sign-in.v1`, a polling
result naming an account that is neither `E` ignoring case nor the account that its check before
`sign-in-hook.v1` found is followed, before the invocation ends, by a `sign-out-hook.v1` part,
and the invocation ends failed naming both accounts, answering no sign-in action of A; no
`sign-out-hook.v1` part is dispatched for `e` or for that first check's account; (c) after a
`sign-out-hook.v1` part that does not succeed, `SignInUndone=False` is reported before its
invocation ends.

**R13 (PND-013, must).** At `f72c2bc3`, PND-013's `then`: “IFC-049's requestState checks come
first: a retry they reject carries no return anywhere.” and “second, no return has been passed on
for this invocation, else already-returned;” Its `observable`: “After DL18 no return is passed on
and no hook runs; IFC-049 rejects the expired requestState before these checks.” Supporting,
IFC-049's `observable`: “rejects tampering, replay of a consumed state or another principal. Each
retry consumes its state;” and PND-014's `observable`: “a restart loses the value.” Predicate:
(a) a retry is accepted by requestState's checks only where the ledger shows its requestState
issued for that invocation to that caller and not consumed by an earlier accepted retry, and the
invocation has not ended; (b) a return is passed on only in an accepted retry, at most once for
each invocation, and a later return in a retry of it that requestState's checks accept is refused
`already-returned`; (c) no return is passed on after its invocation's end, whether DL18's end or
a restart ended it; (d) a retry that requestState's checks reject passes nothing on and reports
no PND-013 reason.

**V14 (PND-014, must not).** At `f72c2bc3`, PND-014's `then`: “The value appears in a result,
structuredContent, response _meta, notification, diagnostic, log or record, is among the values
REC-002 keeps, reaches an Agent”. Predicate: no emission of E carries v1, v2 or v3 other than the
`_meta` of E's `sign-in-hook.v1` retry to H: no result or diagnostic to Q, no record, and nothing
delivered to an Agent.

**H05 (PND-005, must).** At `f72c2bc3`, PND-005's `then`: “The instance is then held until
sign-in.v1 answers that it is signed in (PND-010, PND-011): an operation that starts model work
on an Agent, and any request to a Resource other than the parts of sign-in.v1, of a setup and of
its retirement, is not dispatched and reports unavailable with the ID of the instance's
unanswered sign-in action in” the `pendingAction` key, “with a diagnostic naming PND-005 and that
the instance awaits sign-in to its expected account, without naming that account (PND-004). The
hold is the instance's, not the action's: an expiry lifts nothing.” Predicate: from the raising
of a sign-in action of A to the next `sign-in.v1` of A that ends succeeded, no turn of A is
dispatched; each turn decided in that span reports unavailable, naming in `pendingAction` a
sign-in action of A unanswered at its decision, with a diagnostic naming PND-005 among its
diagnostics and naming neither `E` nor `e`; an expiry in the span ends nothing of it (the reading
of MC9a-003). A turn held under both PND-005 and PND-011's `SignInUndone=False` reports
unavailable with both diagnostics, PND-005's (with `pendingAction`) and PND-011's.

**P01 (PND-001 and PND-005, must).** At `f72c2bc3`, PND-001's `observable`: “An instance has at
most one unanswered sign-in and one unanswered setup action.” PND-005's `then`: “A sign-in action
is raised for the instance, with expiry, unless one is unanswered (PND-001).” Its `observable`:
“After expiry a new sign-in is raised on the next held request or setup, never by expiry alone.”
At `C048A2`, order 048a-2's decision 1: “A contract reports a missing or lapsed login in one of
two forms: an implementation that carries Conduit's keys ends the call failed or unknown, as it
can tell, with an error diagnostic naming PND-005; a plain one reports it only by a URL request,
under IFC-016.” Predicate: the ledger never holds two sign-in actions of A that are raised and
neither answered nor expired; each is raised by setup's account check reporting no account or
another, by an `account-hook.v1` part within `sign-in.v1` reporting another account, by a held
request after an expiry, or by a keyed call of A ending failed or unknown with an error
diagnostic naming PND-005, and never by an expiry alone; and the keyed lapse of stage 6 raises
one where none is unanswered.

**S02 (PND-002 and PND-008, must).** At `f72c2bc3`, PND-002's `then`: “It is raised at root and
every Interface whose scope contains a subject: the Pool or Fleet itself, the owner of an
instance, or an owner of either end of a Conduit. Scope is computed from the Interface's open
inspection Conduits (definitions.md). It is never raised at an Agent.” Its `observable`:
“Agents' list and inspect results carry no pending actions.” and, of the `unanswered` key, “the
number raised there, only when nonzero.” PND-008's `then`: “These actions are raised at root
alone: system belongs to no scope (PND-001, PND-002).” Predicate: each `list.v1` result lists
exactly the actions that are unanswered and unexpired at its decision and raised at its caller:
at R, every one; at an Interface, each whose subject its scope holds, the scope computed from the
inspection Conduits that the ledger shows open at that decision; at G, none; and m1 at R alone.
Each result of an Interface's own request carries `unanswered`, equal to the number of actions
unanswered and unexpired at, and raised at the caller when, the result is emitted, the ledger
entry that carries the result (for a dispatched request, its end), counting each action that this
ending or decision raises or computes, as order 047b counts over the state after the step, and
carries none where that number is zero; no result to G carries it.

**A09 (PND-009, must).** At `f72c2bc3`, PND-009's `then`: “It is dispatched only on the
Interface's own behalf, through an open Conduit listing it to the target that authorizes
inspection of the subject (ACC-033)” and “A Conduit whose consumer is not an Interface cannot be
opened listing answer.v1; that open reports refused.” Its `observable`: “An action not raised at
the caller reports refused before invalid, revealing nothing of whether it exists.” Predicate: an
`answer.v1` is dispatched only for a caller at which its action is raised, by the raised-at test
without the time test (047c), through an open Conduit to F listing `answer.v1`; one whose action
is not raised at its caller reports refused, with no diagnostic naming the action, whatever else
is wrong with it; and G's open of a Conduit listing `answer.v1` reports refused and opens nothing.

**A03 (PND-003, must).** At `f72c2bc3`, PND-003's `then`: “The answer is decided under ordinary
access using the Conduits the actor holds.” Its `observable`: “Once it passes the answer is
expired, with no actor, and nothing is dispatched. An operation answering an answered, expired or
void action reports conflict after the access decision.” Supporting, OPS-009: a copy of an ended
`answer.v1` request is a repeat, answered with no dispatch. Predicate: each action has at most one
answer in the ledger, `expired` counting as one; an `answer.v1` decided after its action's
answer, permitted by access, not invalid and not a repeat that OPS-009 answers, reports conflict,
and a repeat that OPS-009 answers records no answer; and an expiry dispatches nothing.

**D12 (PND-012, must).** At `f72c2bc3`, PND-012's `observable`: “Pending state and answered state
used to prevent repeated answers survive a purge of records under REC-004”. Predicate: an
`answer.v1` decided after a purge, for an action answered or expired before it, and permitted by
access, reports conflict, and the ledger holds no answer of that action after the purge.

**U06 (PND-006, must).** At `f72c2bc3`, PND-006's `when`: “An instance is not retired and its
latest set-up.v1 invocation ended failed or unknown, including one whose outcome a reconciliation
has since set under LIF-028.” Its `then`: “One setup action is computed, with that invocation's
ID, the instance as subject”. Its `observable`: “Dispatch of a further set-up.v1 answers retried”
and “A dismissal or expiry for that invocation stays answered and is not recomputed as
unanswered. A later unsuccessful setup makes a new action.” Predicate: each `list.v1` result at R
lists a setup action of B exactly where B's latest `set-up.v1` in the ledger ended failed or
unknown and that action has no answer, its ID that invocation's and its subject B; the dispatch
of a further `set-up.v1` answers `retried` the setup action unanswered at that dispatch; and
after u2 is dismissed or expires, no result lists u2.

### MC9a reach predicates

Sign-in, numbered in source order:

0 `setup-raises`: stage 1 raises s1. 1 `held`: a turn reports unavailable naming a sign-in action.
2 `caller-refused`: Y's `sign-in.v1` is not dispatched. 3 `conflict-sign-in`: a `sign-in.v1`
reports conflict under PND-010 while another is in progress. 4 `conflict-retirement`: a
`sign-in.v1` reports conflict under PND-010 while A's retirement is in progress.
5 `retire-during-sign-in`: A's retirement is dispatched while a `sign-in.v1` of A is in progress
(MC9a-002). 6 `return-passed`. 7 `already-returned`. 8 `wrong-account-undone`: `O` found by
polling is undone, and the invocation ends failed. 9 `undo-failed`: `SignInUndone=False` is
reported. 10 `first-account-kept`: W2's polling finds the account its first check found and
dispatches no sign-out. 11 `signed-in`: a `sign-in.v1` succeeds on `e` and answers an action
signed-in. 12 `expiry-then-signed-in`: a sign-in action expires before the next `sign-in.v1` of A
succeeds. 13 `signed-in-then-expiry`: an action answered signed-in reaches its expiry, which
changes nothing. 14 `new-after-expiry`: a held request after an expiry raises s3.
15 `lapse-raises`: the keyed lapse raises s2. 16 `void`: the retirement answers an unanswered
sign-in action void. 17 `restart-ends-sign-in`: a `sign-in.v1` in progress at a crash ends
unknown at recovery. 18 `restart-overlap`: after a restart ended a `sign-in.v1`, another is
dispatched while H's login program from the first still waits (MC9a-001). 19 `repeat-conflict`:
a copy of a `sign-in.v1` request in progress reports conflict under OPS-032.
20 `replay-rejected`: a copy of a retry is rejected by requestState's checks. 21 `wait-ended`:
DL18's end ends a `sign-in.v1` unknown. 22 `quiescent`: the script has ended and every queue is
empty; a `sign-in.v1` whose URL no one completes may remain in progress.

Actions, numbered in source order:

0 `setup-action`: R's `list.v1` lists u1. 1 `agent-sees-none`: G's `list.v1` lists nothing and
carries no `unanswered`. 2 `agent-open-refused`. 3 `scoped-listed`: X's `list.v1` lists u1, with
`unanswered` 1. 4 `unscoped-empty`: Y's `list.v1` lists nothing and carries no `unanswered`.
5 `machine-at-root`: R's `list.v1` lists m1, and X's does not. 6 `unscoped-refused`: Y's
answer reports refused. 7 `invalid-answer`: X's `acknowledged` reports invalid.
8 `scope-closed`: after the close, X's `list.v1` lists nothing, and its answer reports refused.
9 `retried`: the further setup answers u1 `retried`. 10 `answer-then-expiry`: u2 is dismissed and
then reaches its expiry, which changes nothing. 11 `expiry-then-answer`: u2 expires, and R's
answer then reports conflict. 12 `purge-then-conflict`: R's answer after the purge reports
conflict. 13 `repeat-answered`: a copy of an ended `answer.v1` request is answered with no
dispatch. 14 `quiescent`.

### MC9a mutants

Each changes one rule of the correct model:

- NoConflict: a `sign-in.v1` of A is dispatched while another, a setup or a retirement of A is in
  progress.
- Unavailable: that `sign-in.v1` reports unavailable naming PND-010, in place of conflict.
- CopyAsNew: a copy of a `sign-in.v1` request in progress is dispatched as a new invocation.
- ConflictForever: recovery ends a `sign-in.v1` in progress unknown, as IMP-007 says, but keeps it
  as in progress for PND-010's test.
- WrongAccountKept: an `O` found by polling ends the `sign-in.v1` succeeded and answers the action
  signed-in, with no sign-out.
- UndoInitial: polling undoes the account that the check before `sign-in-hook.v1` found.
- ReturnTwice: PND-013's second check is skipped.
- ReplayState: a retry does not consume its requestState.
- SavedReturn: a return's value is saved, and recovery passes it on in a `sign-in-hook.v1` retry.
- LeakDiagnostic: a refused return's diagnostic holds its value.
- ExpiryLifts: the expiry of A's sign-in action lifts A's hold.
- VolatileHold: the hold is volatile, so recovery forgets it.
- RaiseTwice: a held request raises a new sign-in action while one is unanswered.
- WrongScope: `list.v1` computes an Interface's scope from the Conduits open when the action was
  raised.
- AgentSees: an Agent's `list.v1` lists the actions that its scope would hold.
- VolatileScope: `list.v1`'s scope index is volatile and empty after recovery, for every
  Interface other than R.
- SkipRaisedAt: `answer.v1` skips the raised-at test.
- AnswerTwice: a copy of an `answer.v1` request that was dispatched and succeeded (OPS-009's
  case) is dispatched again and records its answer with no answered test.
- AnswerExpired: `answer.v1` skips the expiry test.
- VolatileAnswers: every stored answer other than an expiry is volatile.
- ForgetOnPurge: a purge erases every stored answer other than an expiry.
- RecomputeDismissed: `list.v1` lists a dismissed setup action as unanswered while its invocation
  is still B's latest failed setup.

### MC9a expected verdicts, fixed before execution

H means holds; F means refuted. The safety order for sign-in is X10 K11 R13 V14 H05 P01, and for
actions S02 A09 A03 D12 U06; only a part's own properties are registered for it. A Correct row
expects exactly its configuration's reach below, and a witness absent there is expected not
reached; every enabled harness witness is expected reached.

| Part / configuration | Expected reach |
| --- | --- |
| Sign-in / Base | 0 to 16, 22 |
| Sign-in / LossQ | as Base |
| Sign-in / ReorderQ | as Base |
| Sign-in / CopyQ | as Base, with 19 and 20 |
| Sign-in / Crash | as Base, with 17 and 18 |
| Sign-in / Wait | as Base, with 21 |
| Actions / Base | 0 to 12, 14 |
| Actions / LossQ | as Base |
| Actions / ReorderQ | as Base |
| Actions / CopyQ | as Base, with 13 |
| Actions / Crash | as Base |

Each mutant runs in its configuration and, where that is not Base, in Base too: Base is the
configuration with every fault switched off. A mutant's reach is its configuration's row with the
changes listed.

| Variant / configuration | Expected safety | Expected in Base | Reach changes |
| --- | --- | --- | --- |
| Correct / every sign-in configuration | HHHHHH | HHHHHH | none |
| Correct / every actions configuration | HHHHH | HHHHH | none |
| NoConflict / Base | FHHHHH | the same run | remove 3 4 |
| Unavailable / Base | FHHHHH | the same run | remove 3 4 |
| CopyAsNew / CopyQ | FHHHHH | HHHHHH | remove 19 |
| ConflictForever / Crash | FHHHHH | HHHHHH | remove 18 |
| WrongAccountKept / Base | HFHHHH | the same run | remove 8 9 10 |
| UndoInitial / Base | HFHHHH | the same run | remove 10 |
| ReturnTwice / Base | HHFHHH | the same run | remove 7 |
| ReplayState / CopyQ | HHFHHH | HHHHHH | remove 20 |
| SavedReturn / Crash | HHFHHH | HHHHHH | none |
| LeakDiagnostic / Base | HHHFHH | the same run | none |
| ExpiryLifts / Base | HHHHFH | the same run | remove 14 |
| VolatileHold / Crash | HHHHFH | HHHHHH | none |
| RaiseTwice / Base | HHHHHF | the same run | none |
| WrongScope / Base | FHHHH | the same run | remove 8 |
| AgentSees / Base | FHHHH | the same run | remove 1 |
| VolatileScope / Crash | FHHHH | HHHHH | none |
| SkipRaisedAt / Base | HFHHH | the same run | remove 6 8 9 |
| AnswerTwice / CopyQ | HHFHH | HHHHH | remove 13 |
| AnswerExpired / Base | HHFHH | the same run | remove 11 |
| VolatileAnswers / Crash | FHFFF | HHHHH | none |
| ForgetOnPurge / Base | HHFFH | the same run | none |
| RecomputeDismissed / Base | FHHHF | the same run | none |

Five refutations beyond a mutant's own property are registered here: VolatileAnswers and
ForgetOnPurge refute D12 as well as A03, since R's only repeated answer follows the purge;
VolatileAnswers refutes S02 and U06 too, since a crash between stage 8's answer and its
`list.v1` loses u2's dismissal, and that `list.v1` then lists u2; and RecomputeDismissed refutes
S02 as well as U06, since S02 reads the same `list.v1` results. Any
other difference is recorded beside these unchanged tables and explained, as MC6's and MC7a's
were. Every full run is tested against these tables, and one more single-thread breadth-first
search for each mutant prints its first target refutation, replayed after erasing node data.

### MC9a: what holds, and what is one step

A clause that its mutant refutes in Base needs no fault: it is reported "one step", to be shown by
a vector, and not as holds. A clause that its mutant refutes only with a fault on is reported as
holds where every Correct row holds. So the registered expectation is:

| Property | One step, to a vector | Holds, with a fault |
| --- | --- | --- |
| X10 | the conflict and its reason (NoConflict, Unavailable): order 048a-2's vector | a copy of a request in progress (CopyAsNew); a restart (ConflictForever) |
| K11 | WrongAccountKept, UndoInitial | none |
| R13 | ReturnTwice | a replayed requestState (ReplayState); a restart (SavedReturn) |
| V14 | LeakDiagnostic | none |
| H05 | ExpiryLifts | a restart (VolatileHold) |
| P01 | RaiseTwice | none |
| S02 | WrongScope, AgentSees | a restart (VolatileScope); VolatileAnswers refutes it only beside A03 |
| A09 | SkipRaisedAt | none |
| A03 | AnswerExpired | a copy (AnswerTwice); a restart (VolatileAnswers, which refutes D12, S02 and U06 beside it) |
| D12 | ForgetOnPurge | none; VolatileAnswers refutes it only beside A03 |
| U06 | RecomputeDismissed | none; VolatileAnswers refutes it only beside A03 |

The account names, for each "one step" clause, the vector or the order that writes it when MC9a's
results merge.

### MC9a findings registered before execution

**MC9a-001 — possible gap, PND-010 as order 048a-2 writes it, with IMP-007.** A `sign-in.v1` in
progress when the engine restarts ends unknown, so PND-010's conflict no longer refuses a new
`sign-in.v1` of that instance, while the implementation's login program from the ended invocation
may still wait: two logins can then run in one home, the case FR-433 names, after a restart. No
promise stops the first program. `restart-overlap` searches for it in Crash. No property is
invented for it: PND-011's account check alone decides completion, and K11 checks it there.

**MC9a-002 — one-sided exclusion, PND-010 as order 048a-2 writes it, and REV-014.** The new
sentence refuses a `sign-in.v1` while a setup or retirement of the instance is in progress, but
REV-014's `when` names a request to set up, retire or change an instance while another setup,
retirement or change of it is in progress, and a `sign-in.v1` is none of these. So a retirement
or a setup asked for while a `sign-in.v1` is in progress is dispatched, and the setup's account
check and sign-out (PND-011) may run while that `sign-in.v1` polls. The model applies the text as
written: stage 7 dispatches the retirement, and `retire-during-sign-in` searches for it. The
finding is reported, not resolved by an added refusal.

**MC9a-003 — ambiguity, PND-005 and PND-010.** PND-005 holds the instance until `sign-in.v1`
answers that it is signed in, and says that an expiry lifts nothing; PND-010's success answers
any sign-in action signed-in. Where the action expired before a `sign-in.v1` succeeds, no action
is left for that success to answer, and the words do not say whether the success lifts the hold.
The model reads the hold as lifted by the next `sign-in.v1` of the instance that ends succeeded,
whatever its action's state, since the hold is the instance's, not the action's.
`expiry-then-signed-in` searches for it. The reading is reported, not settled.

### MC9a limits

These are bounded projections of the quoted slices: they say nothing of the engine, a client, a
browser, a real login, cryptography or unbounded executions. One fault at a time, on Q's links
alone; faults between E and H, cuts, lost nodes and a crash of H are outside. No fairness,
eventual completion or eventual delivery is asserted, and a `sign-in.v1` whose URL no one
completes may stay in progress. X10's setup clause, a `sign-in.v1` while a setup is in progress,
has no witness in this script and is not credited. A09's refusal ahead of invalid and conflict
has no witness in this script. PND-011's hold under `SignInUndone=False`
coincides in this script with PND-005's hold, which the still unanswered action keeps, so the
script cannot tell them apart and K11 credits only the report. Evidence items and the codes PND-004
withholds, the URL and IFC-049's checks and message, `signInLogin`, fixed ports and the device
login, a Resource's sign-in, peer delivery, notices (PND-007), enrollment's answers,
reconciliation, `inspect.v1`'s `status.pending`, and DL18's and DL19's durations are outside it.
An expiry is a local choice of E's clock, not elapsed time.
````

5. **`ROADMAP.md` does not change.** Experiment 012's row lists the models that have run, and
   names MC9a among those to come, "each registered before it runs"; it lists no registered model.
   MC9a's run is still to come, so the row stays as it is.

6. **What this order does not do.** It writes no code under `experiments/012-model-checking/src/`
   and changes no `Cargo.toml`, `Cargo.lock`, `account.yaml` or `G5.md`: PND-005's and PND-010's
   rows stay `left to MC9`, as order 048a-2's start check requires, and the coordinator updates
   the account when MC9a's results merge. It changes no promise, case, ruling, `IN_FORCE.md`,
   `ROADMAP.md`, order file or the work orders' README.

## Scope

Change only `experiments/012-model-checking/README.md`: its line 3 (decision 3) and the new
section at its end (decision 4). Report anything else that needs changing.

## Done when

From the repository root, each passes:

```sh
python3 tools/check_docs.py             # last line: Errors: 0
python3 tools/check_private_paths.py    # last line: Errors: 0
git diff --check                        # prints nothing
git diff --name-only                    # exactly experiments/012-model-checking/README.md
git diff --numstat | awk '{print $2}'   # 1
```

Then each of these prints what its comment says. Those marked "on the base" print something else
on the base without this change, so each fails with the change removed.

```sh
F=experiments/012-model-checking/README.md
S='/^## MC9a — pending actions and sign-in as written: pre-registration$/,0'
grep -c '^## MC9a' $F                                             # 1; on the base, 0
grep '^## ' $F | tail -1                                          # ## MC9a — pending actions and sign-in as written: pre-registration
awk "$S" $F | grep -c '^### MC9a'                                 # 11
awk "$S" $F | grep -c -e 'DATE' -e 'C048A2'                       # 0
awk "$S" $F | grep -v -e '](' -e '^|' | /usr/bin/python3 -c 'import sys; print(sum(len(l.rstrip("\n")) > 100 for l in sys.stdin))'   # 0
sed -n 3p $F | grep -c '; MC9a registered 20[0-9-]* under work order 201, not run\.$'   # 1; on the base, 0
awk "$S" $F | grep -c '^| [A-Za-z]* / [A-Za-z]* | [HF]* | '      # 23
awk "$S" $F | grep -c '^| Correct / every '                       # 2
awk "$S" $F | grep -c '^\*\*[A-Z][0-9][0-9] (PND-'                # 11
awk "$S" $F | grep -c '^\*\*MC9a-00[1-3] — '                      # 3
awk "$S" $F | grep -c '^| PND-0[01][0-9] | '                      # 13
awk "$S" $F | grep -c '^| NoConflict / Base | FHHHHH | the same run | remove 3 4 |$'    # 1
awk "$S" $F | grep -c '^| Unavailable / Base | FHHHHH | the same run | remove 3 4 |$'   # 1
tr -s ' \n' '  ' < $F | grep -c 'is not dispatched and reports conflict, with a diagnostic naming PND-010'   # 1; on the base, 0
tr -s ' \n' '  ' < $F | grep -c 'Totals: \*\*12 modelled; 1 not modelled, a static decision to be shown by vectors\*\*'   # 1
C=$(git log -1 --format=%H HEAD -- docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md | cut -c1-8)
echo $C | grep -c '^[0-9a-f]\{8\}$'                               # 1
awk "$S" $F | grep -c "\`$C\`"                                    # 3
awk "$S" $F | grep -c "^  at \`$C\`, as PND-010 and PND-005 will read once it merges"   # 1
git diff -U0 | grep '^+' | /usr/bin/python3 -c 'import sys; print(sum(chr(64) in l for l in sys.stdin))'   # 0
```

And each quotation in curly quotes in the new section is found, whitespace aside, in its source:
`spec/promises/pending.yaml`, `spec/conventions/operations.yaml`,
`spec/interface/implementation.yaml` and `docs/proposals/full-review/experiments.md` at
`f72c2bc3`, or order 048a-2's file at the commit decision 2 names. This prints `34 0`: 34
quotations, none missing.

```sh
/usr/bin/python3 - <<'EOF'
import re, subprocess
def show(rev, path):
    return subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True,
                          check=True).stdout
order = "docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md"
c = subprocess.run(["git", "log", "-1", "--format=%H", "HEAD", "--", order],
                   capture_output=True, text=True, check=True).stdout.strip()
paths = ["spec/promises/pending.yaml", "spec/conventions/operations.yaml",
         "spec/interface/implementation.yaml", "docs/proposals/full-review/experiments.md"]
corpus = " ".join([show("f72c2bc3", p) for p in paths] + [show(c, order)])
norm = lambda s: " ".join(s.split())
corpus = norm(corpus)
text = open("experiments/012-model-checking/README.md", encoding="utf-8").read()
section = text[text.index("## MC9a "):]
quotes = re.findall("“([^”]*)”", section)
print(len(quotes), len([q for q in quotes if norm(q) not in corpus]))
EOF
```

`check_docs.py` checks that each new link and anchor resolves, the link to order 048a-2's file
among them. Do not stage anything. Report each choice you make as your own, and every check's
last line.

The admin then has a fresh, read-only Opus reader read the diff before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2), and the merge waits for the local gate
([ruling 80](../proposals/ruling-80/README.md), point 10). This order merges before order 048a-2
is dispatched.

## Record

Run on 2026-11-18, from about 12:36 to 13:12 UTC, on the Mac from `main` `f414e438` (the merge
that added orders 048a-2 and 201), in two rounds: the worker (156 s) and a fresh Codex verifier
(611 s), then the worker again (646 s) and a second fresh Codex verifier (698 s). The
worker found the 13 base checks of decision 1 as written, took `DATE` as `2026-11-18` and `C048A2`
as `f414e438`, changed line 3 as decision 3 says, and added decision 4's block word for word; its
own choice was to copy the block from the checked-in order and compare the README with it byte for
byte. The first verifier found the checkpoint identical to the block after the substitution, and
the 34 quotations, the mechanical checks and the 13 account rows as written, and found 8 musts and
5 shoulds in the prescribed content: X10 refused the ordinary stage 4 retry and a copy of an ended
request; K11 failed a correct run interrupted after a wrong polling result; G had no route through
`access` to reach PND-009's refusal; stage 7 dropped the schedule in which retirement removes A;
the final sign-in could not find a preserved `O`; ReplayState and AnswerExpired could still reach
witnesses 20 and 11; ForgetOnPurge's refutation of S02 was not registered; A03 counted answers by
public ID across a redispatch; MC9a-003's witness did not observe the hold lift; the portable vector
was credited for conflicts it does not exercise; quiescence was not in the ledger; and Y's refusal
named no reason. Round 2, at the admin's prompt, rewrote the block for all 13: X10 applies PND-010
to fresh requests and repeats eligible for a new attempt, R13 checks continuations and OPS-009,
OPS-012 and OPS-034 the other copies; K11 asks for the failure on ordinary completion and an unknown
ending on interruption; G opens through an `access` Conduit with a bound naming F and `answer.v1`;
stage 7 keeps the removal and checks X's refusal through its Fleet-only route; the final sign-in
reads the preserved account; witnesses 20 and 11 were narrowed; ForgetOnPurge became `FHFFH`; a
computed setup action's ledger identity is the ending that computed it; witness 12 and H05 observe
the stage 6 turn dispatched, with the mutant RetainHold; the vector is credited for setup, and two
future vectors are named; quiescence is a harness-only check; and witnesses 23 to 26 were added.
The worker's own choices were those witnesses, the stage 6 turn as the release probe, the vectors'
names and the count of 23 verdict rows, which it reported under `could_not` against the order's
22. The second verifier found 6 musts, 5 shoulds and a nit: the count; Wait's window; K11's
queued hook after DL18; H05's release while A is still registered; P01 with no raise required on
the final sign-in's `O`; a sign-in action left unanswered on retired A; Q's stage 4 after W has
ended; K11's success clause; the return's destination during restart overlap; the conflict
diagnostic's invocation ID; witness 13 under AnswerTwice; and the limits' faults.

A fresh read-only Opus reader then verified the order (ruling 79, point 2) on the branch. It ran
the order's greps, the quotation script (`34 0`), `check_docs` and `check_private_paths` (Errors: 0
each) and `git diff --check origin/main...HEAD` (clean), compared the block with the section, and
read DL18, PND-003, PND-010, LIF-004, LIF-005, LIF-015, LIF-024, LIF-035, OPS-029 and CON-024 at
`f72c2bc3`. Every check passed but the verdict rows, 23 against 22. It found four musts and seven
shoulds:

- M1: the order's check of verdict rows expects 22, and round 2's RetainHold makes 23.
- M2: Wait's timer ended the oldest `sign-in.v1` in progress, though DL18 is the URL wait, which
  ends when polling finds an account; so Wait's expected reach of witness 26 was wrong.
- M3: under Crash, E can crash in stage 1 after s1 is raised and before setup succeeds, leaving A
  registered; H05's release then requires a turn that CON-024 refuses.
- M4: a copy of Y's refused stage 4 answer reaches witness 13, which AnswerTwice does not change.
- S1: P01 allowed, and did not require, the raise when a sign-in's initial check finds `O`.
- S2: in stage 7, an `O` found after retirement is dispatched could raise an action on retired A.
- S3: K11 (d) let a hook already on the link run after DL18, which PND-010 forbids.
- S4: X10 did not check that PND-010's diagnostic names no invocation ID (order 048a-2, decision 6).
- S5: stage 4 gave Q no step when W ended before Q held rs1.
- S6: the two future vectors' names were the worker's own and owned by no order, and a link stood
  alone on its line.
- S7: the limits put every fault on Q's links, though Crash and Wait act on E.

The fixer found the branch already on `origin/main` at `f414e438`, its two commits with the
work-order identity, and applied each finding, rewrapping the changed paragraphs at 100 columns:

- M1: the order's check expects `# 23`, and a dated note after "Who decided what" records round
  2's changes to decision 4's block, with the vectors' names, and the coordinator's adoption of
  them, open to the owner's strike. Dropping RetainHold was the other choice; the coordinator
  confirmed the adoption at the merge.
- M2: Wait's row reads "DL18's end, which ends unknown the oldest `sign-in.v1` that has relayed its
  URL and whose polling has found no account yet"; Wait's reach is "as Base, with 21 and 24"; K11
  (d) and witness 26 read "recovery, or DL18 before polling finds an account". In witness 26 that
  DL18 branch cannot occur, so Crash alone reaches it, as the reader said.
- M3: stage 1 gains "If setup ends unknown, A stays registered and the script goes on; a turn to A
  then reports unavailable under CON-024."; H05's span clause covers turns "decided in that span
  while the ledger shows A ready", its release "the next turn in stage 6, decided while A is ready
  and access permits it", and its oracle readiness; witness 12's turn is "with A ready".
- M4: witness 13 is the reader's text word for word.
- S1 and S2: P01 gains the reader's clause and witness 24 the raise; stage 7 and P01 add that no
  action is raised once A's retirement has been dispatched, and P01 that none of A is unanswered
  after it; MC9a-002 gains the interleaving and the reading, from PND-003. The words "unless A's
  retirement has been dispatched" are the fixer's: without them, S1's clause and S2's check
  contradict each other in that interleaving.
- S3, S4, S5 and S7: the reader's texts, word for word; S4 after a comma.
- S6: the names stay, adopted in M1's note, and the link is folded into "When MC9a's results merge
  under", as the text of a link to full review order 24. The reader's other choice was not taken:
  full review order 24 names the model, not these vectors, so "which full review order 24 names"
  would be wrong.

The second verifier's shoulds on K11's success clause and on the return's destination during
restart overlap were not among the reader's findings; this round leaves them to the coordinator.
The fixer then ran, from the worktree's root on the files of this commit before it was made, every
check of "Done when", with `git diff` against `origin/main`: `check_docs` and `check_private_paths`
(Errors: 0 each), `git diff --check origin/main` (clean), the README's deletions (1), and each
grep as its comment says, with the verdict rows at 23; the quotation script printed `34 0`. The
names changed are the README and this order, which the amendment and this Record change. MC9a
remains unrun.

A second fresh Opus reader found no must and nine shoulds: the amendment's wording (S1), K11's
success clause and R13's return destination, the second verifier's two open shoulds (S2, S3), DL18
in K11 (d) and witness 26 (S4), MC9a-001 after recovery (S5), P01's raise limited to A ready (S6),
the revision paragraph (S7), the Record's durations (S8) and Q's dropped request (S9). A fixer
applied each as its fix states, taking the first option where it offered two, and the coordinator
confirmed the amendment. Every check of "Done when" passed as before, the quotation script printing
`34 0`.

A third fresh Opus reader found no must and three shoulds: R13 (b)'s antecedent and the requestState
the model does not hold (S1), the amendment and revision paragraph naming the second reader (S2),
and
the Record's missing round (S3). A fixer applied each as its fix states. A fourth reader found the
readers still miscounted and the ledger's late-result clause displaced; the coordinator applied both
fixes at the merge, the revision paragraph now naming the readers without a count. Every check of
"Done when" passed as before, the quotation script printing `34 0`.
