# Work order 050n: the Agent packages 0.2.0, joined and merged, the last 050 order

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` each case you add and each case whose files or
expectations you change, with the promises each tests and whether it passes. This order has
network access only because the engine's and the packages' tests bind loopback ports, which the
sandbox refuses without it; use it for nothing else, add no crate, and run cargo offline, with
`CARGO_NET_OFFLINE=true` set on each cargo command's own line. Run one cargo command at a time.
Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print or copy an environment
variable's value, never read or print any process's environment or arguments, never run a command
that lists every process on the machine, such as `ps -e`, `ps aux` or `top`, and never signal a
process that a test of this order did not start. Never read or print a key, a token or any
credential. Never run the real `claude` or `codex` programs, not even for their help or version,
never read their program files, and sign in nowhere: every test uses the packages' scripted
clients, as every test on your branch does.

## What and why

**What it does.** The coordinator split order 050 into orders 050b to 050o
([order 117](117-plan-s2-s3.md)'s 050 entry, the coordinator's, open to the owner's strike).
Orders 050c to 050h built the `.v2` Agent Types on one branch; at 050h's tip the work split into a
Claude lane (orders 050i, 050j and 050k, on `claude-code.v2`) and a Codex lane (orders 050l and
050m, on `codex.v2`), each a checkpoint with its Record in its own order file, none merged alone.
This order is the last of them: it carries both lanes to `main` as one release, the Agent
packages 0.2.0, which NAM-024 lets publish once. It is the only 050 order that changes `engine/`
and `conformance/`. It:

- moves the engine's pinned copies of the two packages from 0.1.1 to 0.2.0, so that the engine's
  test `live_s1_scripted_agents_call_engine_with_distinct_run_identities`, which fails on the lanes
  by design (order 050a's Record), passes again;
- aligns the S1 fixture's `.v2` Types with the packages' (order 048a-1's must-state item 2);
- adds the cases that the lanes left to it: loader vectors that accept both `.v2` Types as
  shaped and a template binding `/account` (the targeted review's section 6, item 1 of both 050
  lists, as orders 050f, 050j, 050l and 050m left it), a refusal under TPL-005 that order 050l
  left unshown, and the vector that shows a failed turn's fields reaching the caller under IFC-025
  (item 7 of both lists, as order 050e left it);
- takes the two shoulds that the lanes carried here: the sign-in hook's failures that do not yet
  name PND-010 (order 050k's second verifier, carried here by its Record), and the Codex
  README's sentence on a system proxy (order 050m's second verifier, left open by its Record);
  and brings Codex's login limit within DL18's wait, as the Claude package's already is;
- brings `engine/CONFORMANCE.md` and the accounts guide, `docs/guides/accounts.md`, to 0.2.0, the
  guide's half of the browser-opens limit included (order 050b left it here);
- checks each `.v2` description sentence by sentence against Q1's answer (item 6 of both lists),
  and reports, changing none.

**Why it waited.** The owner's [ruling 82](../proposals/ruling-82/README.md), point 3, merges the
Agent packages 0.2.0 only after order 048b-join, onto an engine that runs the account check, and
point 1 lets `codex.v2` state `signIn` only if A5's live check passes
([experiment 027](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check),
stage 1, step 1). The owner's [ruling 86](../proposals/ruling-86/README.md), points 3 and 4, has
orders 207 and 209 land first, so that both READMEs' statements of the refusal of a named login
before any part and of the going on after a failed check hold on `main`; and order 208's probe
had to report no write before the Codex lane relied on the account check's route. Under the
owner's [ruling 89](../proposals/ruling-89/README.md), the live check runs through `conduit sign-in`,
so after order 051b-2 as well as 048b-join, with the check's package keeping the Type's name
`codex.v2` at a scratch address
([027's amendment of 2026-11-19](../../experiments/027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs),
items 1 and 2); A77's pairing is paused until this order merges (point 6); and the S1 rerun's pin
comes at the first dogfood milestone, after this order and 051b-2 (point 7). This text was written
ahead of those merges, from the lane tips `f1ede6a5` (Claude) and `34f4e662` (Codex), and the
coordinator refreshes it at dispatch (below).

**This text is for a pass of the live check.** On a fail, `codex.v2` states no `signIn`, Codex
Agents do not sign in yet, and the question returns to the owner with FR-118 (ruling 82, point
1). The coordinator then does not dispatch this text: it rewrites the start (the Claude lane
alone, with `codex.v2` as order 050h left it, `requires` holding `retire-hook.v1` alone, and no
Q1 (v)) and the decisions that touch `codex.v2`'s sign-in, and the Codex lane is left out of
0.2.0, dropped from the branch, never reverted on `main`; the coordinator's commit that settles
order 050h's logout exit (below) then moves to the Claude lane, so the rewritten start still reads
it. Where this order was built ahead of the
result (below) and the check fails, its branch does not merge: the coordinator rewrites this text
as above and dispatches it again. Building ahead bets the build on a pass, the coordinator's
choice, open to the owner's strike.

## Where this order starts

Before dispatch the coordinator makes the branch `wo/050n-agent-packages-joined` and sets your
tree to its tip, `HEAD`:

1. **The base**, one of two, either of which serves:
   - `main` at a commit that holds order 048b-join's merge, and so the merges of orders 209 to
     048a-5gh before it; or
   - stacked, under the owner's lever "Pipeline the chain" as the coordinator applies it
     ([ruling 89](../proposals/ruling-89/README.md), point 2, open to the owner's strike): 048b-join's
     round-1 branch, or 051b-2's where 051b-2 was dispatched on it, with each order from 209 to
     048a-5gh merged or on its own round-1 branch below it. This lets the build run before the live
     check, so that after the owner's sitting only the result's merge, a rebase and the gate remain
     (a readiness finding in the coordinator's notes, private).

   Order 051b-2 need not be on the base, since nothing here runs `conduit sign-in`; it merges
   before this order does, since the live check runs through it.
2. It rebases the Codex lane onto that base.
3. It applies onto that tip, in order, the Claude lane's commits after the lanes' split, from
   `7bf4cb47` (excluded, order 050h's last commit on that lane) to the Claude lane's tip. They
   touch only `packages/agent-claude-code/` and the files of orders 050i to 050k, and those order
   files on `main` hold the same text that the lane's do, before their Records.
4. Every commit it writes or rewrites has the author and committer
   `Conduit work order <work-orders@conduit.invalid>`, and the fetch, the rebase and the check
   that the base is an ancestor run in one command (the coordinator's squash guard).

`HEAD` then holds both lanes' packages as their tips left them, every Record of orders 050a to
050m, the A77 records of orders 050j and 050k (`050j-a77.yaml`, `050k-a77.yaml`), and the
engine's pinned copies still at 0.1.1. On a stacked base, your tree's copy of this order's file,
and of the chain's files, may be older than the texts on `main`, or, for this file, missing; where
they differ, the texts on `main` govern: read them with `git show origin/main:docs/work-orders/FILE`,
`FILE` being the file's name. The lanes' commits are checkpoints; the coordinator squashes this
branch into one commit, "Agent packages 0.2.0", when it merges.

### What the coordinator rechecks at dispatch

The coordinator refreshes this text before dispatch, in a change of its own, where any of these
differs from what it says:

- **The predecessors of the build.** Orders 050b (#346), 207 and 208 merged, each with its
  Record; orders 209 to 048b-join merged, or on the stacked base above. The coordinator sets
  `STACKED` in the start check to each order of the chain that has not merged at dispatch, as
  `ORDER=COMMIT` with that order's round-1 tip, or to `none`; until it does, the check stops the
  worker.
- **The predecessors of the merge.** Orders 209 to 048b-join and 051b-2 merged; the
  live check's dated result merged with A5's row, as 027's pre-registration says, reading pass. The
  owner's slot sign-ins are not this order's predecessors.
- **The pass.** The Codex step's result reads pass, under 027's "What the outcomes change", run as
  its amendment of 2026-11-19 has it. Where the result has merged at dispatch, the coordinator
  writes, verbatim, as `PASS_LINE` in the start check below, a line of the result that records
  the Codex step's pass and that only the Codex step's result holds (one that names `codex.v2`
  and the package's address, as the amendment's item 1 has the step record them), never a line
  that also occurs in the Claude sign-in's section; the check reads the result on `main`. The
  value is the whole physical line of the wrapped Markdown file, since `grep -x` matches whole
  lines; choose a line with no single quote, or write each `'` in it as `'"'"'`. Where
  it dispatches this order ahead of the result, it writes `AHEAD` there, and checks the pass before
  the merge instead; until it writes either, the check stops the worker. Before the merge it
  compares `packages/agent-codex/` at the Codex lane tip with the package commit the result
  records, from which the check's package differs only in its address (the amendment's item 1):
  if the lane changed the package after the build the check used, it says so and judges whether
  the pass stands for the tree joined here, open to the owner's strike (ruling 82, point 1). This
  order's own changes to that package (decisions 2 and 3) touch failure texts, a README and how
  long an unfinished login waits, not the route the check ran; the coordinator records at the
  merge whether the pass stands for them, by the same judgment. The Claude scratch sign-in that
  follows in the same sitting, and in the same result file, holds nothing here (ruling 89,
  point 5): its fail or fault goes to the owner before any Claude slot signs in, and does not hold
  this order.
- **The lane tips.** If either lane moved past `f1ede6a5` or `34f4e662`, it reads the new
  commits' Records for anything carried here, and sets the two SHAs of the start check.
- **What 209 to 048b-join built.** Each package README's account of the engine's sign-in (its
  refusal before any part, its relay, its going on after a failed check, DL18's end) is still what
  the engine on the base does; where it is not, the coordinator adds the change as a decision, in
  the promises' words. On a stacked base it checks this again after the rebase before the merge.
- **Order 050h's open limit.** Order 050h left to the coordinator, before this order or the first
  real retirement, what each client's logout exits with in a home that holds no login (050h's
  "Who decided what"). The coordinator records its settlement in order 050h's Record before
  dispatch, as a commit on the Codex lane, which moves that lane's tip and so the start check's
  SHA (the lane tips, above), in a paragraph that begins "The coordinator settled the logout's
  exit in a home with no login"; the start check reads it. Where the settlement changes a hook,
  it adds a decision here.
- **Order 050c's Linux run.** Order 050c's Record holds the coordinator's Linux run of
  `v2_client_home_refusals` (2026-11-18), as its S2 asked before this order's gate; the start
  check reads it.
- **Shoulds left open.** Order 050h's second verifier's three shoulds, order 050i's second
  verifier's three (the sign-out test's check that the conversation's process stopped before
  logout, an account case whose accepted directory is removed before the call, and the deadline's
  precedence over an oversized output), order 050l's second verifier's other three (identities
  kept after an incomplete scoped stop, the temporary-directory condition, and the email's
  provenance under CTR-013), and order 050c's path-resolution window and re-created marker files
  stay with the coordinator, outside this order, unless it adds them here. So do two README lines
  that the readiness review listed: the Codex README's launch baseline, Codex 0.167.0, which
  stays, since Codex's turns are still launched against it (decision 8's `CONFORMANCE.md`
  wording); and the Claude README's caveat on the spelling of `loggedIn`, which could cite Claude
  Code 2.1.318 only from a read of the program, which no worker of this order makes.

## What to read

- [AGENTS.md](../../AGENTS.md), its checks above all, and
  [conformance/README.md](../../conformance/README.md#reviewing-vector-changes),
  "Reviewing vector changes", for `tools/vector_delta.py` and its declarations, and its
  [Portable cases](../../conformance/README.md#portable-cases), for the form of decision 6's
  cases.
- The Records of orders [050j](050j-claude-sign-in-redirect-login.md),
  [050k](050k-claude-code-page-login.md), [050l](050l-codex-account-hook.md) and
  [050m](050m-codex-sign-in-hook.md), at the foot of each file in your tree, for what each lane
  built and carried here; and in orders [050e](050e-agents-v2-failed-turn-fields.md),
  [050f](050f-agents-v2-inputs.md), [050h](050h-agents-v2-retire-hook.md),
  [050j](050j-claude-sign-in-redirect-login.md), [050l](050l-codex-account-hook.md) and
  [050m](050m-codex-sign-in-hook.md), each "What this order does not do", for what each left to
  this order (`grep -n '050n'` finds each place).
- [The targeted review's section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists),
  the lists "048a-1", "050, `claude-code.v2`" and "050, `codex.v2`": items 1, 6 and 7 of both 050
  lists, and 048a-1's item 2. Item 3 of both lists is replaced by ruling 86, point 1: the items
  stay plain in 0.2.0.
- [Q1](../proposals/review-signin-2026-11-16/README.md#q1-the-v2-descriptions-as-020-freezes-them)
  and the line of [IN_FORCE.md](../../IN_FORCE.md) that begins "*Spec pending, order 050*: the
  two `.v2` descriptions hold six sentences", which gives the owner's answer.
- The promises CTR-013, MAN-017, TPL-005, TPL-019, PKG-020, IFC-025, PND-010 and NAM-024, in
  `spec/`, and the case files they name.
- [engine/CONFORMANCE.md](../../engine/CONFORMANCE.md), section I3-04 and the lines that name the
  Agent packages' version; [docs/guides/accounts.md](../guides/accounts.md);
  `engine/crates/conformance/examples/make_archives.rs`; `engine/crates/shell/tests/agents.rs`;
  `engine/examples/s1/`; `conformance/fixtures/agents/`; and the cases
  `conformance/language/registration/live-agents/` and
  `conformance/language/packages/agents-v2-defaults/`.
- [Order 050g's results](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md),
  its two outcome lines, for the accounts guide's vendor line.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor d4343f71 HEAD; echo "floor $?"
git cat-file -e origin/main:docs/work-orders/050n-agent-packages-joined.md; echo "order $?"
test -e docs/work-orders/050n-declared.yaml; echo "declared $?"
git status --short | wc -l
STACKED='SET AT DISPATCH'
test "$STACKED" != 'SET AT DISPATCH'; echo "stacked set $?"
echo "$STACKED" | tr ' ' '\n' | grep -v '^none$' | while read -r S; do git merge-base --is-ancestor "${S#*=}" HEAD; echo "${S%%=*} $?"; done
grep -c 'fn sign_in_relays_through_the_interface' engine/crates/shell/src/server_tests.rs
for P in 050b 207 208 050i 050j 050k 050l 050m; do printf '%s ' $P; cat docs/work-orders/$P-*.md | grep -c '^## Record'; done
grep -c '^The coordinator ran S2 on the Linux node' docs/work-orders/050c-*.md
grep -c "^The coordinator settled the logout's exit in a home with no login" docs/work-orders/050h-*.md
ls docs/work-orders/050j-a77.yaml docs/work-orders/050k-a77.yaml | wc -l
PASS_LINE='SET AT DISPATCH'
test "$PASS_LINE" != 'SET AT DISPATCH'; echo "pass set $?"
test "$PASS_LINE" = AHEAD && echo ahead || git ls-tree --name-only origin/main experiments/027-a5-sign-in/ | grep '/results-.*-1-step-1\.md$' | while read -r F; do git show "origin/main:$F" | grep -qxF -- "$PASS_LINE" && echo "$F"; done | wc -l
git diff --quiet f1ede6a5 HEAD -- packages/agent-claude-code; echo "claude lane $?"
git diff --quiet 34f4e662 HEAD -- packages/agent-codex; echo "codex lane $?"
grep -c '^version: 0.2.0$' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
grep -c '^version: 0.1.1$' conformance/fixtures/agents/agent-codex/conduit.yaml conformance/fixtures/agents/agent-claude-code/conduit.yaml
/usr/bin/python3 -c 'import sys, yaml
for p in sys.argv[1:]:
    for d in yaml.safe_load_all(open(p)):
        if d and d["kind"] == "Type" and d["metadata"]["name"].endswith(".v2"):
            s = d["spec"]
            print(d["metadata"]["name"], len(s.get("requires", [])), [x["name"] for x in s.get("signIn", {}).get("logins", [])])' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
```

They print `floor 0`, `order 0`, `declared 1` (you write it), `0`; `stacked set 0`, then one
line `ORDER 0` for each order `STACKED` names (none where it is `none`); `1` or more (048b-join's
work on the base); `1` for each of the eight orders (`050b 1` to `050m 1`); `1` (050c's Linux
run); `1` (050h's settlement); `2`; `pass set 0`, then `ahead` where the coordinator dispatched this order ahead of the
live check's result, or otherwise `1` (the result that holds the coordinator's pass line);
`claude lane 0` and `codex lane 0` (if your clone lacks those commits the line prints `128`; then
the dispatch note says the coordinator checked the join, and you go on); `1` for each package's
`conduit.yaml` and for each fixture copy's; and `codex.v2 4 ['browser', 'device']` and
`claude-code.v2 4 ['browser', 'code-page']`. Report `HEAD`'s commit and which orders `STACKED`
names.

## The decisions it applies

Behavior is stated here, with the tests that show it. Write exactly only the texts this order
quotes: diagnostics, and the one README sentence of decision 3. Everything else is yours to word,
in the terms of `spec/definitions.md`, and to report.

1. **The packages stay as the lanes left them, but for decisions 2 and 3.** The two packages'
   sources are not made one again: the Claude lane put its hooks in `src/sign_in.rs` and the
   Codex lane in `src/lib.rs`, and each `main.rs` lists its own tools. Orders 050i and 050j said
   that this order "joins the two sources again"; the coordinator reads that as joining the two
   lanes into one tree, which the start above does, since making one source of two tested hook
   sets would change tested code with no finding behind it. Do not change either package's
   `conduit.yaml`, `contracts.yaml`, `Cargo.toml`, `Cargo.lock` or version: their copies become
   the engine's pins (decision 4). Report any difference you find between the two packages that
   looks unintended, and change nothing for it.

2. **Every start failure of a sign-in hook names PND-010.** `sign-in-hook.v1` asks for a
   diagnostic naming PND-010 and why where a hook cannot start a login, and one naming PND-013 and
   the cause where a return fails. Today the Claude hook fails without naming PND-010 where the
   configuration was refused or the home check refuses (order 050k's second verifier, finding 1,
   which order 050k's Record carried here as the coordinator's option (b)), and the Codex hook
   does the same on its home and readiness checks and where the stated logins cannot be read. In
   each package, for the failures where the hook cannot start a login (PND-010's case): the
   readiness and `.v1` checks, the refused configuration, the home check, stated logins that
   cannot be read, and a login's start or stop failing:
   - every such failure text that the sign-in hook returns holds `PND-010`; a text that already
     does stays as it is;
   - a text that does not is the same text followed by ` (PND-010)`, appended in the sign-in hook
     alone and last, after any `; NOTE` that a stop's note adds, so that the shared refusals that a turn and the account, sign-out and retire hooks
     return keep their texts, unchanged;
   - texts naming PND-013, and the retry path's texts, stay as they are; protocol errors
     (`invalid_params`, a missing capability, a cancelled hook) are not failure texts and stay as
     they are;
   - nothing else changes: no step, no order of the checks, no new text.

   Tests, in each package's `tests/wrapper.rs`, each new one with a name that begins with
   `v2_sign_in`: through the sign-in hook, a refused configuration and a client home outside the
   state directory each fail with the shared text followed by ` (PND-010)`; through a turn and
   through the account hook, the same two states fail with the shared text alone; and every
   start-path failure text the existing sign-in tests drive holds `PND-010`. Each package
   README's sign-in section says that every failure of the hook to start a login names PND-010,
   the shared refusals with ` (PND-010)` added, and each README's list or table of the hook's
   texts shows the new texts. Extending the change to the Codex hook is the
   coordinator's.

3. **Two Codex changes: the proxy sentence and the login limit.**
   - In `packages/agent-codex/README.md`, in the paragraph on a configured proxy and the refresh
     request, the sentence "The endpoint names no server, and TLS to it cannot be verified."
     states as certain what holds only where no proxy intercepts TLS (order 050m's second
     verifier). Replace it with IN_FORCE.md's words, exactly: "A system proxy that receives the
     request can read the token only if it intercepts TLS with a certificate the system trusts,
     as it could for Codex's own refresh." If the old sentence is not there, change nothing and
     report.
   - The Codex hook stops a started login that the person has not finished after `LOGIN_LIMIT`,
     900 seconds, while the engine's wait for the person, DL18's, ends after ten minutes. Between
     the two, the first Agent's `codex login` still holds the fixed port 1455, so another Codex
     Agent's sign-in fails its port check under PND-010 for up to five minutes after an abandoned
     one (a readiness finding in the coordinator's notes, private). Set `LOGIN_LIMIT` to 600
     seconds, as the Claude package's already is, and bring the README's statements of that limit
     (its "15 minutes" and "900 seconds") to it. A test in each package's `tests/wrapper.rs`,
     named as decision 2's are, shows that the package's login limit is at most 600 seconds.
     Nothing else in the login changes.

4. **The engine's pins move to 0.2.0.** Each package's `conduit.yaml` and `contracts.yaml` are
   copied byte for byte to `conformance/fixtures/agents/<package>/`; each copy's
   `contracts.tar.gz` holds exactly those two files, written by `make_archives` as USTAR with a
   fixed mode, uid and gid 0 and mtime 0, gzipped with mtime 0, so that the same bytes give the
   same digest and its `--check` covers both archives; each copy's README says it is the
   package at 0.2.0, written by this order, and keeps what it says of the archive. The digest of
   each archive, with version 0.2.0, is what `engine/examples/s1/conduit.yaml` and
   `conformance/language/registration/live-agents/fixture.json` pin, and
   `engine/crates/shell/tests/agents.rs` pins version 0.2.0 where it pinned 0.1.1. S1's templates
   keep the `.v1` Types, which 0.2.0 still exports unchanged (order 050a's byte check), so that the
   live S1 test and its case show the same project as before; the `.v2` Types are shown by
   decision 6's cases. The case `registration-live-agents` passes, and so does the engine test
   `live_s1_scripted_agents_call_engine_with_distinct_run_identities`. Where
   `engine/examples/s1/README.md` names a version, it names 0.2.0. Choosing to keep S1 on `.v1`
   is the coordinator's.

5. **The S1 fixture's `.v2` Types follow the packages'.** `make_archives.rs` builds the
   `agents-0.2.0` release with hand-written `.v2` Types ("registration-only fixtures, no signIn or
   hooks"). Under 048a-1's item 2 the fixture follows the packages, not the reverse: build its
   `.v2` Types from the packages' copies under `conformance/fixtures/agents/` so that each `.v2`
   Type's `schema` equals the package's, and so do its `requires` and `signIn` where the fixture's
   cases still pass with them. Where a member cannot be carried without the fixture gaining
   implementations it does not have, carry the schema alone and report which member and why.
   Regenerate the fixtures with `make_archives` (`conformance/fixtures/s1/packages/agents-0.2.0/`,
   `conformance/fixtures/install-archives/` and
   `conformance/language/packages/agents-v2-defaults/`), so that its `--check` passes. The case
   `agents-v2-defaults` passes against the new schemas: each sample keeps the verdict it had, or,
   where the real schema changes one, you report the sample, its old and new verdict and the schema
   member that changes it, and declare it (decision 9).

6. **The cases this order adds.** Each a portable case under
   [conformance/README.md](../../conformance/README.md#portable-cases)'s "Portable cases", using
   `install-archive`, `register-project` and `invoke`, in the place of the cases beside it (their
   directory, with an `index.yaml` entry where it has one), its `promises` listing what it tests:
   - **Both `.v2` Types accepted as shaped** (item 1 of both lists): registering each package's
     0.2.0 copy, from `conformance/fixtures/agents/`, is accepted with no diagnostic, its `.v2`
     Type stating `signIn` with the account place `/account`, its stated logins and the four
     hooks in `requires`, each with a default; the case cites CTR-013 and PKG-020.
   - **A template binding `/account` passes MAN-017** (item 1): a project of one template of each
     `.v2` Type whose input is bound to `/account`, as order 210's text binds it in its
     codex-worker template (an input `account`, a string with `minLength` 1, to
     `/config/account`), is accepted; the case cites MAN-017.
   - **TPL-005 for `account-hook.v1`** (order 050l's decision 9, left unshown): every item of the
     0.2.0 Codex package either implements `account-hook.v1` or is refused under MAN-017 first,
     and selecting a second run item gives TPL-019, so the case uses a synthetic package, written
     by `make_archives` into `conformance/fixtures/install-archives/`: the 0.2.0 Codex package's
     `contracts.yaml` under its own address, with the same exports. It has one item,
     `wrapper-v2`, a copy of the package's `wrapper-v2` (`plain`, `tools`, `command`,
     `environment`, `implements`) with `account-hook.v1` removed from both `implements` and
     `tools`. The template is of that package's `codex.v2` and selects `#wrapper-v2` for every
     operation, `account-hook.v1` included. If installation refuses the Type's `account-hook`
     default, drop that default and keep the template's selection. The case, or the shell
     fallback, asserts ruleId TPL-005 with a diagnostic that names `account-hook.v1` and the
     item, and no other refusal: no MAN-017 and no TPL-019. If the synthetic package cannot be
     made so, write this case as a shell test, as the fallback below allows, and report why; the
     choice of this case is the coordinator's.
   - **A failed turn's fields reach the caller** (item 7 of both lists): a plain implementation of
     `turn.v1` that ends a turn failed with two text blocks in `content`, the failure text and one
     JSON object `{"failedTurn": {...}}` with the fields the packages' READMEs list, gives the
     caller a result with `isError` true, outcome failed, no `structuredContent`, and that
     `content` unchanged after the engine's own text block; the case cites IFC-025. The fields'
     values are fixed words and numbers, as the READMEs give them. If the case format cannot
     script a plain implementation's failed result, write the vector as a test of the shell
     crate, citing IFC-025 in a comment, and report why. The same fallback holds for the
     TPL-005 case, citing TPL-005.

   Each new case passes. None goes on `conformance/expected-failures.yaml`.

7. **The descriptions, checked and not changed** (item 6 of both lists). Split each `.v2` Type's
   `metadata.description` into sentences, and give, in your report, for each sentence, which of
   Q1's six it carries ((i) no client version; (ii) the other-runtime sentence, with "On the
   engine's own runner"; (iii) "ends the conversation"; (iv) D53's retire-hook sentence; (v), in
   `codex.v2`, the browser login named as the unasked default and the device login listed; (vi),
   in `claude-code.v2`, the code-page login listed) or "none" where it carries none; then each of
   the six that no sentence carries, and any sentence that names a client version or a store for
   the login. Change no description: the coordinator decides on what you report before the
   merge.

8. **The documents.**
   - `engine/CONFORMANCE.md`: I3-04 and the present-state bullet under "What the first build is",
     which names the Agent packages' version and calls the `.v2` Types unmerged, say that `main`
     holds the packages 0.2.0, which export the `.v1` Types unchanged and the `.v2` Types with each
     Agent's own home, `signIn`, and the account, sign-out, sign-in and retire hooks, all on plain
     items (ruling 86, point 1), read against Claude Code 2.1.318, with Codex's turns launched
     against its 0.167.0 baseline and its sign-in, account and logout reads at 0.167.1, with their
     sign-in and account hooks built against scripted clients and run against real clients only in
     the live check's sitting: A5's check against a real Codex, and the Claude scratch sign-in
     against a real Claude Code
     ([027's amendment of 2026-11-19](../../experiments/027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs),
     item 6), while the `.v2` Types' turns have also run against the real clients in the dogfood
     ([ruling 70](../proposals/ruling-70/README.md)); the coordinator adds each one's result at the
     merge. Rows on the engine's own
     sign-in stay as the engine's orders left them,
     and dated increment records, such as I4-02's line that the Codex package remains 0.1.1, stay
     as they are.
   - `docs/guides/accounts.md`: replace its stale state (its paragraph "State on 2026-11-12", which
     says neither sign-in nor each Agent's home is built) with what holds once this merges, and
     bring each section to 0.2.0. It must say: each `.v2` Agent keeps its client's login in its own
     home inside its state directory, and the person's own logins stay as they are; the person
     signs an Agent in with `conduit sign-in` (order 051b-2, merged before this order) or through
     root's Interface, and does so only for an Agent whose sign-in action is pending, never as a
     check, since for a Codex Agent a check that fails goes on to the sign-in hook, and Codex's
     login first signs that home out; after the engine restarts, `conduit pending` shows any Agent
     whose account check failed at its setup; a Codex Agent left idle until its access token lapsed
     may fail its check, since the check blocks the token's refresh, and is then asked to sign in
     again, though a turn would have refreshed it (ruling 86, point 5); no login can be named in
     0.2.0, so each Agent signs in with its client's default login, and the device code and
     Claude's code-page login wait for later releases; on the engine's own runner Codex's browser
     login may open the default browser of the engine user's graphical session, so the person
     closes that window and keeps that browser free of an OpenAI session (review item 6 of the
     Codex list; the specification's limit, which order 050b wrote); a Claude sign-in completes
     only where the person's browser reaches the login's loopback port on the engine's machine
     until order 050o's managed connection is built; the account check alone decides who signed in
     (PND-011); each client's logout at retirement revokes that home's own token at the vendor
     (order 050g's two outcome lines), and a login on a machine that is lost, which no logout
     reaches, stays until it is revoked at the vendor (D53); and a failed logout at retirement is
     dispatched again only after reconciliation records it failed (order 050h's Record, R2's
     wording). Keep the guide's own voice: it speaks to the person and binds nothing.
   - Each package README: nothing beyond decisions 2 and 3, unless a sentence names a state that
     this merge ends (such as "on this branch"); then reword that sentence alone so it stays true
     on `main`, and report each.

9. **The records the tools keep.** Run `python3 tools/promise_standing.py --write` so that
   `engine/STANDING.md` agrees with the new cases. Write `docs/work-orders/050n-declared.yaml`,
   declaring every case and fixture change, as `tools/vector_delta.py --help` describes, so that
   `python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/050n-declared.yaml`
   exits 0; report its summary line, the count of changes it reports.

10. **What this order does not do.** No change to `spec/`, to any promise, to `IN_FORCE.md`, to
    `ROADMAP.md`, to the experiments, or to any other order's file, the Records and A77 records of
    orders 050a to 050m included; no change to the engine's sources beyond the test and the
    example of decisions 4 and 5; no keyed item (ruling 86, point 1); no `apiKey` (ruling 83); no
    new conversation operation, which is order 203's, as 0.2.1; and no archive installed anywhere
    but a temporary store a test makes and removes. On a dispatch ahead of the live check's
    result, such a store, holding scripted clients and signing nothing in, is not an installation
    in the sense of the owner's gate (ruling 82, point 1; 027's gate as its amendment of
    2026-11-19, item 1, narrows it), though the 0.2.0 Codex archive it holds states `signIn` in
    `codex.v2`. This extends orders 050l's and 050m's reading, which covered registration alone, to
    the setup and turns of the `.v1` Types in such a store, as the live S1 test and its case run
    them; the reading is the coordinator's, open to the owner's strike.

## Who decided what

That 0.2.0 merges only after order 048b-join, and that `codex.v2` states `signIn` only on the live
check's pass, is the owner's (ruling 82, points 1 and 3); that the items stay plain in 0.2.0, and
that orders 207 and 209 land first, is the owner's (ruling 86, points 1, 3 and 4); that the
packages are subscription-only is the owner's (ruling 83); that A77's pairing waits until this
order merges, that the S1 rerun's pin comes at the first dogfood milestone, and that the Claude
scratch sign-in joins the live check's sitting, are the owner's (ruling 89, points 5 to 7). The
must-state items are the targeted review's, which the owner accepted with ruling 82. Everything
else is the coordinator's, each open to the owner's strike ([ruling
48](../proposals/ruling-48/README.md)): the split of 050 and this order's place last; this text
written ahead and refreshed at dispatch; the join as the start describes it, on a stacked base and
ahead of the live check's result where the coordinator so dispatches it (ruling 89, point 2, as the
coordinator applies it); the reading that this order's own changes to the Codex package leave the
pass standing, made at the merge; the reading of "joins the two sources again" (decision 1); option
(b) for order 050k's carried should and its extension to the Codex hook (decision 2); IN_FORCE.md's
sentence for the proxy should, and Codex's login limit at 600 seconds (decision 3); S1 kept on
`.v1` (decision 4); carrying a schema alone where a fixture member cannot follow (decision 5); the
choice of the four cases (decision 6); reporting the descriptions without changing them (decision
7); the guide's content, its sentences on when to sign in and on an idle Codex Agent included
(decision 8); and the reading that, on a dispatch ahead of the result, a temporary store that a
test makes and removes is not an installation under the gate (decision 10). Apply them as written and report them under `decisions` as the coordinator's.

## Scope

You may change only:

- `packages/agent-claude-code/src/sign_in.rs`, `packages/agent-claude-code/tests/wrapper.rs`,
  `packages/agent-claude-code/README.md`;
- `packages/agent-codex/src/lib.rs`, `packages/agent-codex/tests/wrapper.rs`,
  `packages/agent-codex/README.md`;
- `conformance/fixtures/agents/`, `conformance/fixtures/s1/packages/agents-0.2.0/`,
  `conformance/fixtures/install-archives/`, `conformance/language/packages/agents-v2-defaults/`,
  `conformance/language/registration/live-agents/`, and the new cases' directories with their
  directories' `index.yaml`;
- `engine/crates/conformance/examples/make_archives.rs`, `engine/crates/shell/tests/agents.rs`,
  a shell test of decision 6's fallback, `engine/examples/s1/conduit.yaml`,
  `engine/examples/s1/README.md`, `engine/CONFORMANCE.md` and `engine/STANDING.md` (by its tool);
- `docs/guides/accounts.md`, and `docs/work-orders/050n-declared.yaml`, new.

Anything else you find that needs changing goes in your report.

## Done when

From the repository root, each passes, with `Errors: 0` or its tool's success line:

```sh
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_schemas.py
python3 tools/check_roadmap.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/050n-declared.yaml
git diff --check
```

In each of `packages/agent-claude-code/` and `packages/agent-codex/`, one command at a time, each
with `CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4` on its line: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo test --release` (report each target's passed
and failed counts) and `cargo run --release --bin package` (report the digest; it is a host
build's, and it is not a pin).

In `engine/`, one command at a time, each with `CARGO_NET_OFFLINE=true` on its line:

```sh
cargo run --release -p conduit-conformance --example make_archives -- --check 2>&1 | tail -n 1
cargo test --release -p conduit-shell --test agents
cargo test --release -p conduit-conformance
cargo clippy --all-targets -- -D warnings
cargo test --release
```

The last is the whole suite, run once, after the others pass; it must pass with no failure, the
test `live_s1_scripted_agents_call_engine_with_distinct_run_identities` among the passed.

Then, from the repository root, each prints what is beside it:

```sh
for NAME in agent-codex agent-claude-code; do for F in conduit.yaml contracts.yaml; do cmp -s packages/$NAME/$F conformance/fixtures/agents/$NAME/$F; echo "$NAME $F $?"; tar -xOzf conformance/fixtures/agents/$NAME/contracts.tar.gz $F | cmp -s - conformance/fixtures/agents/$NAME/$F; echo "$NAME archive $F $?"; done; done   # each 0
for NAME in agent-codex agent-claude-code; do tar -tzf conformance/fixtures/agents/$NAME/contracts.tar.gz | sort | tr '\n' ' '; echo; done   # conduit.yaml contracts.yaml, twice
for NAME in agent-codex agent-claude-code; do D=$(shasum -a 256 conformance/fixtures/agents/$NAME/contracts.tar.gz | cut -c1-64); grep -c "sha256:$D" conformance/language/registration/live-agents/fixture.json engine/examples/s1/conduit.yaml; done   # each 1 or more
grep -c '0\.1\.1' engine/crates/shell/tests/agents.rs engine/examples/s1/conduit.yaml   # 0 and 0
git diff --quiet HEAD -- packages/agent-codex/conduit.yaml packages/agent-codex/contracts.yaml packages/agent-claude-code/conduit.yaml packages/agent-claude-code/contracts.yaml; echo "contracts $?"   # contracts 0
git diff --quiet HEAD -- spec IN_FORCE.md ROADMAP.md experiments; echo "untouched $?"   # untouched 0
git diff --name-only HEAD -- docs/work-orders | grep -v '^docs/work-orders/050n-declared.yaml$' | wc -l   # 0
git status --short | grep -c '^?? docs/work-orders/050n-declared.yaml$'   # 1
grep -c 'PND-010' packages/agent-claude-code/tests/wrapper.rs packages/agent-codex/tests/wrapper.rs   # each more than at HEAD
tr '\n' ' ' < packages/agent-codex/README.md | tr -s ' ' | grep -c 'A system proxy that receives the request can read the token only if it intercepts TLS with a certificate the system trusts, as it could for Codex'"'"'s own refresh\.'   # 1
tr '\n' ' ' < packages/agent-codex/README.md | tr -s ' ' | grep -c 'TLS to it cannot be verified'   # 0
grep -c '0\.1\.1' conformance/fixtures/agents/agent-codex/README.md conformance/fixtures/agents/agent-claude-code/README.md   # 0 and 0
grep -cE '900 seconds|15 minutes' packages/agent-codex/README.md   # 0
/usr/bin/python3 -c 'import json, yaml
f = {d["metadata"]["name"]: d for d in json.load(open("conformance/fixtures/s1/packages/agents-0.2.0/contracts.json")) if d["kind"] == "Type"}
for p in ("agent-codex", "agent-claude-code"):
    for d in yaml.safe_load_all(open("packages/%s/contracts.yaml" % p)):
        if d and d["kind"] == "Type" and d["metadata"]["name"].endswith(".v2"):
            n = d["metadata"]["name"]; print(n, f[n]["spec"]["schema"] == d["spec"]["schema"])'   # codex.v2 True and claude-code.v2 True
for P in CTR-013 PKG-020 MAN-017 TPL-005 IFC-025; do printf '%s ' $P; git ls-files --others --exclude-standard conformance engine | xargs grep -l -- "$P" | wc -l; done   # each 1 or more
git ls-files --others --exclude-standard conformance engine | xargs grep -l -- TPL-005 | xargs grep -c -- 'account-hook.v1'   # each 1 or more
grep -c 'State on 2026-11-12' docs/guides/accounts.md   # 0
grep -c 'Agent packages on `main` are 0\.1\.1' engine/CONFORMANCE.md; tr '\n' ' ' < engine/CONFORMANCE.md | tr -s ' ' | grep -c 'remain on the unmerged branches of orders 050a'; grep -c '0\.1\.1' conformance/language/registration/live-agents/fixture.json   # 0, 0 and 0
```

The promise loop counts new files; where decision 6's fallback put a case in an existing shell
test file instead, report that file and its `grep -c` for the promise. In `engine/`, with
`CARGO_NET_OFFLINE=true` on its line, `cargo fmt --check` also passes.

The mutation of decision 2: copy `packages/agent-codex/src/lib.rs` to `target/lib.rs.fixed`,
remove the ` (PND-010)` suffix that the sign-in hook appends, and run
`CARGO_NET_OFFLINE=true cargo test --release --test wrapper v2_sign_in` in `packages/agent-codex/`;
it must fail at the new home or configuration assertion. Restore with
`cp target/lib.rs.fixed src/lib.rs` (not `mv`, so that cargo rebuilds), rerun the same command,
which passes, and check that `cmp target/lib.rs.fixed src/lib.rs` prints nothing. Do the same in
`packages/agent-claude-code/` with `src/sign_in.rs`.

`git status --short` lists only paths in Scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, give the last line of every check and each count above,
and give decision 7's table and decision 5's members not carried. Quote no login literal, no URL
and no path under a home directory. Tests that bind loopback ports may fail in your sandbox; if
they do, say so, and the admin reruns them. Where the sandbox denies PID-scoped `ps`, a test
that stops a client's process tree, such as `v2_client_tree_stop_leaves_no_survivor`, may fail
with the stop's note: name each such test and its counts, and the reader reruns them on the Mac.

This order runs one Codex round, a worker and a fresh Codex verifier ([ruling
87](../proposals/ruling-87/README.md), point 1). It is not paired: A77's pairing is paused until
this order merges ([ruling 89](../proposals/ruling-89/README.md), point 6), and its record says so.
A fresh read-only Opus reader then reads the result, warm and targeted, citing your last
whole-suite line ([ruling 79](../proposals/ruling-79/README.md), point 2; ruling 89, point 2).
Where the coordinator dispatched this order on a stacked base, the result is read there, with
`tools/gate.py --base` set to the base's commit, the predecessor's round-1 tip under the lanes'
commits, and it merges only once every predecessor of the merge above has merged and the pass is
checked; the coordinator then rebases this branch onto `main` with `git rebase --onto origin/main`
from that same commit, so that only the lanes' commits and this order's are replayed, resolves any
conflict where a predecessor's verifier or fixer changed what this order changes
(`engine/CONFORMANCE.md`, `engine/STANDING.md`, `make_archives.rs`, `agents.rs` and the fixtures),
and reruns `make_archives`'s check, `vector_delta.py` and the engine's whole suite on the rebased
tree. The coordinator runs `tools/gate.py` on the squash before the merge ([ruling
80](../proposals/ruling-80/README.md)), with no other cargo running on the Mac and no failure
expected, checks every commit's author and committer, and fills the `merged` and `ci` fields of
orders 050j's and 050k's A77 records after the merge's checks on `main`. Once this merges, the
coordinator turns IN_FORCE.md's *Spec pending, order 050* lines into pointers to the packages,
updates A5's and A78's rows, tells the owner whether anything else the S1 rerun's pin waits for has
not merged (ruling 89, point 7), and puts order 203 on this order's tree. This order's Record is
written at its foot at the merge.
