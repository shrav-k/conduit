# Work order 198: F96: the request state bound to the Interface and the invocation, and to no session

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order has network
access only because the engine's tests bind loopback ports, which the sandbox refuses without it;
use it for nothing else, and reach no other machine. Do not run `claude`, `codex` or
`client-facts`, sign in nowhere, and outside your working tree read nothing under your home
directory: never a login file, a keychain item or a session's transcript. Run one cargo command at
a time. Never print a whole large file or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential.

## What and why

IFC-049 binds the `requestState` of a URL action to "the authenticated Interface, session and
originating invocation", and PND-010, IFC-049 again and the description of `sign-in.v1` let a
sign-in's return travel only in that "session and invocation". No row of
[spec/definitions.md](../../spec/definitions.md) defines a session, and its row connection says
that in the baseline revision of the Model Context Protocol the transport "keeps no session"; SEC-013
says the same. IFC-022 establishes the instance "for that request alone and not from an earlier
request". So the word can be read two ways. Read as the transport's session, a person who spends
two minutes in a browser and whose client then retries on a new connection is refused, and the
sign-in ends unknown at DL18. Read as nothing checkable, the binding is to the Interface and the
invocation alone. The task path of the same promise carries no `requestState` and so no session
binding at all, which shows the word is not needed
([FR-039](../proposals/full-review/findings/L04-interface.md#fr-039), with its verifier's
corrections, which govern).

The fix is F96 of the coordinator's decisions
([Secrets, pending actions and sign-in](../proposals/full-review/decisions.md#secrets-pending-actions-and-sign-in)):
the request state is bound to the Interface that the request establishes and to the originating
invocation, and the undefined word leaves the four places that hold it. Dropping it removes a
binding the specification stated, not only a word, so the removal is logged. The targeted review
of the sign-in surface placed it as a named spec fix before order 048a-2 is drafted, since MC9a
then models PND-010 and IFC-049's binding
([its carriers outside the held orders](../proposals/review-signin-2026-11-16/README.md#carriers-outside-the-held-orders);
M91 in [its verdicts](../proposals/review-signin-2026-11-16/verdicts.md); V22 in
[its findings](../proposals/review-signin-2026-11-16/findings.md)), and the owner accepted that
carrier in [ruling 82](../proposals/ruling-82/README.md#the-decisions), point 6, o5. IN_FORCE.md,
topic 9, holds it as spec pending.

**Who decided what.** The carrier and its place before 048a-2 are the owner's (82 point 6, o5).
F96 is the coordinator's fix, and the exact words below, which follow FR-039's least change, are
the coordinator's. So is decision 1's "value" for "handle", the wording point FR-039's verifier
raised: `handle` is a term of `spec/definitions.md` with another meaning (IFC-011). Apply them as
written and report them under `decisions` as the coordinator's. The review's record and
IN_FORCE.md's pending line say "the authenticated Interface"; F96's row and FR-039's least change
say "the Interface that the request establishes", and IFC-022 is the promise by which a request
establishes it. The words below follow F96's row, since "authenticated" is in no row of
spec/definitions.md; the binding is the same.

**What does not change.** No behavior is added; one binding is removed. The first build has no
`requestState` (sign-in is order 048a-2's), so nothing under `engine/` changes. Its binding of a
transport's session IDs to the consumer (`engine/CONFORMANCE.md`) is not a binding of the request
state and stays. No case, vector or fixture changes, so this order has no declaration
(`NNN-declared.yaml`; see [the work orders' README](README.md#shared-and-staged-files)). The
safety catalogue quotes the `then` of PND-010 and of IFC-049, and this order changes only their
`observable`, so the catalogue stays. PND-010 is `left to MC9` in the model-checking account with
no digest, and IFC-049, of the interface layer, has no row there, so the account stays. The
promises' text changes, so AGENTS.md asks for `tools/promise_standing.py --write` in the same
change (decision 6); `engine/STANDING.md` does not hold promise text, so the tool is expected to
leave it as it is.

**Before you start.** From the repository root, this prints 1:
`grep -c 'F96, by a named spec fix before 048a-2 is drafted' IN_FORCE.md`, and this prints 4:
`git grep -i -w session -- spec/interface/implementation.yaml spec/promises/pending.yaml spec/core/operations.yaml | wc -l`.
If either does not, stop and report it. Run `python3 tools/promise_standing.py --check` once
before any change and keep its `Standings:` line for your report. Orders 197 and 199 may be
written beside this one; if either adds a row to the owner's review list before you finish,
decision 5's number is the next free one, as it says.

**Size: S.** Four sentences in three files of `spec/`, one bullet of `IN_FORCE.md` split in two,
one section of the owner's review list, and the standing table rewritten by its tool, unchanged.

Read first:

- FR-039 and its verifier's corrections, and F96's row in the coordinator's decisions;
- IFC-049 (`spec/interface/implementation.yaml`, from `- id: IFC-049`) and IFC-022 there;
- PND-010's `observable` in `spec/promises/pending.yaml`;
- the description of `sign-in.v1` in `spec/core/operations.yaml`;
- the rows connection, Interface, invocation and handle in `spec/definitions.md`;
- IN_FORCE.md, topic 9, the bullet that begins "*Spec pending*: F97, by order 048a-1";
- [the owner's review list](../proposals/first-build/to-review-with-the-owner.md), from "## The
  scenario plans' amendments, from the full review" to its end;
- in AGENTS.md, what `promise_standing.py` needs.

## The decisions it applies

Line numbers are those of `main` at `96ce049e`. If a line has moved, find it by its words. Each
replacement below is exact. Change no other word of any promise, and rewrap only where a decision
gives the new lines.

1. **IFC-049's request state** (`spec/interface/implementation.yaml`, its `observable`). Lines 1393
   and 1394 read:

   ```text
       through its own integrity-protected requestState, an opaque handle bound to the
       authenticated Interface, session and originating invocation, expiring with DL18. The
   ```

   They become these three lines, at the same four-space indent. Line 1395, "client echoes it
   exactly; ...", follows them unchanged:

   ```text
       through its own integrity-protected requestState, an opaque value bound to the Interface
       that the request establishes under IFC-022 and to the originating invocation, expiring
       with DL18. The
   ```

   So the sentence reads: "The engine correlates retries through its own integrity-protected
   requestState, an opaque value bound to the Interface that the request establishes under IFC-022
   and to the originating invocation, expiring with DL18." The sentence after it, which rejects
   "tampering, replay of a consumed state or another principal", stays: a retry that establishes
   another instance under IFC-022 is that other principal.
2. **IFC-049's return** (same `observable`). Line 1435,
   `    the Interface that received the sign-in, in its bound session and invocation. No Agent`,
   becomes `    the Interface that received the sign-in, in its bound invocation. No Agent`. Line
   1434 and line 1436 stay.
3. **PND-010's return** (`spec/promises/pending.yaml`, its `observable`). Line 207,
   `    received the sign-in: only its retry in the session and invocation bound by requestState`,
   becomes `    received the sign-in: only its retry in the invocation bound by requestState`. Line
   208, "(IFC-049) carries a return under PND-013. ...", stays.
4. **`sign-in.v1`'s description** (`spec/core/operations.yaml`). Line 1385,
   `    (IFC-049). Only the receiving Interface's tools/call retry in its bound session and`,
   becomes `    (IFC-049). Only the receiving Interface's tools/call retry in its bound`. Line 1386,
   "invocation may carry signInReturn under PND-013; ...", stays. Nothing else in the contract
   changes.
5. **The removed binding is logged** in the owner's review list,
   `docs/proposals/first-build/to-review-with-the-owner.md`, the page that lists the decisions
   that change what the language says, as order 186 added its rows there. Between the table that
   ends with row 38 ("The coordinator's lines in S10's amendments since acceptance") and the
   heading "## What the owner ruled, for reference", add this block, with one blank line before it
   and one after it. Write each Markdown line as one line; the table row is one line. Row 39 is
   the next number on `main` at `96ce049e`; if the list's last row on your tree is not 38, use
   the next free number in both the row and decision 6's line, and report it.

   ```markdown
   ## From the targeted review of the sign-in surface

   [Ruling 82](../ruling-82/README.md), point 6, o5, accepted the targeted review's carriers for the full review's fixes F95 to F97, which are the coordinator's ([decisions](../full-review/decisions.md#secrets-pending-actions-and-sign-in)). F96 removes a binding that the specification stated, and F96's row says the removal is logged, as FR-039's verifier asked ([FR-039](../full-review/findings/L04-interface.md#fr-039)). It is logged here, with the other decisions that change what the language says.

   | # | Decision | Why it matters | Where |
   | --- | --- | --- | --- |
   | 39 | **A URL action's requestState is bound to the Interface that the request establishes and to the originating invocation, and to no session** (fix F96, work order 198) | It removes a binding that IFC-049, PND-010 and `sign-in.v1` stated. No definition gave the session, and the baseline transport keeps none, so an engine that read it as the transport's session would refuse a sign-in's retry that comes back on a new connection, and the sign-in would end unknown at DL18 | [Order 198](../../work-orders/198-f96-request-state-binding.md); [FR-039](../full-review/findings/L04-interface.md#fr-039); IFC-049, PND-010, `sign-in.v1` |
   ```

   Change nothing else on the page. In the block as you write it, drop the three leading spaces
   that this list gives it.
6. **IN_FORCE.md**, topic 9 (line 228), the bullet that begins "- *Spec pending*: F97, by order
   048a-1:". Delete from it exactly this sentence and the one space after it, so that "with one
   loader vector." is followed by " F95, by a named spec fix":

   ```text
   F96, by a named spec fix before 048a-2 is drafted: the request state is bound to the authenticated Interface and the originating invocation, the word "session" leaves its four places in IFC-049, PND-010 and `sign-in.v1`, and the removed binding is logged.
   ```

   The rest of that bullet stays, F97's and F95's sentences and "(82 point 6, o5)" included.
   Right after it, at the same level, add this bullet, one line:

   ```text
   - **In spec/**: F96: IFC-049 binds its requestState to the Interface that the request establishes under IFC-022 and to the originating invocation, and a sign-in's return travels only on a retry in that invocation (PND-010, `sign-in.v1`); none of them names a session. The binding to a session that this removed is item 39 of the owner's review list, `docs/proposals/first-build/to-review-with-the-owner.md` (82 point 6, o5).
   ```

   Put no date in it: `check_rulings.py` refuses one. Change no other line of IN_FORCE.md; the
   bullet of topic 9 that speaks of "the tunnel or SSH session" is ruling 80's, about another
   connection, and stays, as does topic 15's line that names F95 to F97.
7. **The standing table.** After decisions 1 to 4, run `python3 tools/promise_standing.py --write`
   from the repository root, as AGENTS.md asks of a change to a promise, then
   `python3 tools/promise_standing.py --check`. The table holds no promise text, so
   `engine/STANDING.md` is expected to stay byte for byte as it is, and the `Standings:` line to
   equal the one you kept before your change. If the tool changes the file, keep its change and
   report the diff's size and why.

## Scope

Change only `spec/interface/implementation.yaml` (IFC-049's `observable`, decisions 1 and 2),
`spec/promises/pending.yaml` (PND-010's `observable`, decision 3), `spec/core/operations.yaml`
(`sign-in.v1`'s description, decision 4), `docs/proposals/first-build/to-review-with-the-owner.md`
(decision 5) and `IN_FORCE.md` (topic 9, decision 6), and let `tools/promise_standing.py --write`
alone touch `engine/STANDING.md` (decision 7). Nothing else under `engine/`, and nothing under
`conformance/`, `packages/`, `tools/` or `experiments/`. Leave alone: F96's row in
`docs/proposals/full-review/decisions.md`, the review's record, ruling 82's record, the safety
catalogue, the model-checking account, `spec/definitions.md`, `spec/README.md`,
`spec/core/README.md`, `engine/CONFORMANCE.md`, `docs/implementation/HANDOVER.md`, and every other
promise, PND-013 and SEC-013 among them. Add no declaration.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
git diff --check
```

What they must show:

- `check_schemas.py`'s step 8 still compares "520 promises in 27 files, 226 of the layer
  `language`, 246 of the layer `convention` and 48 of the layer `interface`; 0 errors.", and it
  ends `Errors: 0`. If an order that merged after `96ce049e` changed the totals, report what you
  see.
- `check_model_account.py` prints `Rows: 246 (safety 232, not safety 14, uncatalogued 0)` and ends
  `Errors: 0`.
- `promise_standing.py --check` ends `engine/STANDING.md is current; promise rows: 520.` and
  `Errors: 0`, with the same `Standings:` line as before your change.
- Each other check ends `Errors: 0`, and `git diff --check` prints nothing.

Each of these prints nothing:

```bash
git grep -n -i -w session -- spec/interface/implementation.yaml spec/promises/pending.yaml spec/core/operations.yaml
git grep -n 'opaque handle' -- spec
git status --short -- engine conformance packages tools experiments | grep -v -x ' M engine/STANDING.md'
```

The grep drops engine/STANDING.md only, which decision 7's tool alone may change; report it if it
appears.

Each of these prints 1. The first four read their file with its line breaks as spaces, so a
rewrap does not hide a phrase:

```bash
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'requestState, an opaque value bound to the Interface that the request establishes under IFC-022 and to the originating invocation, expiring with DL18\. The client echoes it exactly;'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'only from the Interface that received the sign-in, in its bound invocation\. No Agent receives'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'only its retry in the invocation bound by requestState (IFC-049) carries a return under PND-013\.'
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c "Only the receiving Interface's tools/call retry in its bound invocation may carry signInReturn under PND-013;"
grep -c '^- \*\*In spec/\*\*: F96: IFC-049 binds its requestState' IN_FORCE.md
grep -c 'F97, by order 048a-1: CTR-013 lets a Type' IN_FORCE.md
grep -c '^## From the targeted review of the sign-in surface$' docs/proposals/first-build/to-review-with-the-owner.md
grep -c "^| [0-9][0-9]* | \*\*A URL action's requestState is bound" docs/proposals/first-build/to-review-with-the-owner.md
grep -c 'is item 39 of the owner' IN_FORCE.md
```

The last prints 1. If decision 5's row took another number, grep for that number in place of 39,
and report it.

Each of these prints 0:

```bash
grep -c 'F96, by a named spec fix' IN_FORCE.md
grep -c 'the removed binding is logged' IN_FORCE.md
```

This prints 7, four fewer than on `main` at `96ce049e` (11): the word stays only where it means
something else, in `spec/README.md`, `spec/definitions.md`, `spec/implementation-defined.yaml`,
`spec/promises/providers.yaml` and `spec/promises/secrets.yaml`:

```bash
git grep -o -i -w session -- spec | wc -l
```

`git diff --numstat -- spec IN_FORCE.md docs/proposals/first-build` prints exactly these five
lines, tab-separated, in this order:

```text
2	1	IN_FORCE.md
8	0	docs/proposals/first-build/to-review-with-the-owner.md
1	1	spec/core/operations.yaml
4	3	spec/interface/implementation.yaml
1	1	spec/promises/pending.yaml
```

`grep -n -A1 'S10.s amendments since acceptance' docs/proposals/first-build/to-review-with-the-owner.md`
shows a blank line next. Then
`grep -B2 '^## What the owner ruled, for reference$' docs/proposals/first-build/to-review-with-the-owner.md`
shows row 39 and a blank line above the heading.

`git status --short` lists the five files in scope and nothing else, `engine/STANDING.md` not
among them unless decision 7's tool changed it.

From `engine/`, run `CONDUIT_VECTOR_THREADS=4 cargo test --release`; it passes. The tests read
`spec/`, and the 4 keeps the runner within memory, as order 071 found.

Do not stage anything. In your report:

- mark decisions 1 to 6's words as the coordinator's, and any choice you made yourself as your
  own;
- give the `Standings:` line of `promise_standing.py --check` from before your change and after
  it, and say whether `--write` changed `engine/STANDING.md`;
- give the number decision 5's row took;
- give the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only, and this order adds no
file. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The merge waits on the admin's local
gate, `python3 tools/gate.py`, on the squash, not on GitHub's CI, which still runs on the pull
request and does not block ([ruling 80](../proposals/ruling-80/README.md#the-decisions), point 10).

## Record

Run on 2026-11-17, from about 06:27 to 06:38 UTC, on the Mac from `main` `9af1c29f` (#338, which
wrote this order), in one round: the worker (533 s) and a fresh Codex verifier (139 s). The change
is the five files of Scope, with the numstat of "Done when": IFC-049's `observable` takes decisions
1 and 2, with decision 1's three-line rewrap that ends "with DL18. The"; PND-010's takes decision 3;
`sign-in.v1`'s description takes decision 4; the owner's review list takes decision 5's block, its
row numbered 39, since the list's last row was still 38, between that row and the heading "What the
owner ruled, for reference", with a blank line before and after; and IN_FORCE.md loses F96's pending
sentence and one space and gains the `In spec/` bullet after it (decision 6).
`promise_standing.py --write` left `engine/STANDING.md` byte for byte as it was, and the
`Standings:` line was the same before and after:
`case passes 293; expected failure 5; reason, no case 74; no case, no reason 148.` The worker
applied decisions 1 to 6 as the coordinator wrote them and made one choice of its own, adding
`CARGO_NET_OFFLINE=true` to the cargo command. Every check of "Done when" passed on its tree but the
engine's tests: the 13 HTTP tests of the shell library failed on binding `127.0.0.1:0`, which the
sandbox refused (41 passed, 13 failed; the portable vectors passed). The verifier found nothing, and
did not rerun cargo.

A fresh read-only Opus reader then verified the order (ruling 79, point 2) on the branch's
checkpoint. It ran the preconditions on `origin/main` (F96's sentence once in IN_FORCE.md; the word
`session` 4 times in the three files), `check_schemas` (step 8: 520 promises in 27 files, 226
`language`, 246 `convention`, 48 `interface`), `set_names --check`, `check_rulings`, `check_docs`,
`check_model_account` (`Rows: 246 (safety 232, not safety 14, uncatalogued 0)`) and
`check_private_paths` (Errors: 0 each), `promise_standing --check` on `main` and on the branch (the
same `Standings:` line; `engine/STANDING.md is current; promise rows: 520.`), `--write` on a scratch
clone (`engine/STANDING.md` byte-identical), `git diff --check origin/main...HEAD` (clean), the
three greps that print nothing, the nine that print 1 (row 39), the two that print 0, the count of
`session` in `spec` (7, 11 on `main`), the numstat (the five lines in order) and the placement of
row 39. It compared decision 5's block and decision 6's edit with the order's text byte for byte
(exact), scanned the added lines and the commit's metadata (no at sign, absolute path or private
path; the work-order identity), and on a scratch clone of the checkpoint ran, from `engine/`,
`CONDUIT_VECTOR_THREADS=4 CARGO_NET_OFFLINE=true cargo test --release`, which exited 0 with every
test result ok, so that check of "Done when" is now met. It found no must and no should. It noted
that, since the harness commits the worker's output as a checkpoint, the literal `git diff --numstat
-- ...` and `git status --short` of "Done when" print nothing on the branch, and give the five lines
and the five files against `origin/main...HEAD`.

The fixer found `origin/main` still at `9af1c29f`, so the rebase with the work-order identity left
the branch as it was, and with no finding to apply it changed no file but this Record. It then ran,
from the worktree's root, every check of "Done when" against `origin/main...HEAD`: `check_schemas`
(step 8 as above), `set_names --check`, `check_rulings`, `check_docs`, `check_model_account` (the
same `Rows:` line), `promise_standing --check` (the same `Standings:` line; current, 520 rows) and
`check_private_paths`, each Errors: 0; `git diff --check origin/main...HEAD` and `git diff --check`
(clean); the three greps that print nothing (nothing), the nine that print 1 (1 each), the two that
print 0 (0 each), the count of `session` (7), the numstat (the five lines in order), the placement
of row 39 (a blank line after row 38, row 39 and a blank line above the heading) and the changed
files (the five of Scope); and, from `engine/`,
`CONDUIT_VECTOR_THREADS=4 CARGO_NET_OFFLINE=true cargo test --release`, which had printed 15 test
results, each ok, when this commit was made; a second fresh reader's full run of the same command on
this commit, in a scratch clone, exited 0 with every test result ok. The gate's run on this order's
squash, which this merge waits on, is the coordinator's at the merge.

At the merge the coordinator applied the second reader's two shoulds, both in this Record: the base
is named by the pull request that wrote the order, and the test run is reported as finished. The
gate runs on the squash, and the merge waits on its PASS.
