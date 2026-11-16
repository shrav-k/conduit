# Work order 191: the '(log)' marks closed, and the repository's settings written down

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none,
and it runs no cargo command. Never print a whole large file or log: read
`docs/implementation/HANDOVER.md`, `docs/AUDIT.md`, `docs/proposals/metrics/general.md` and the
other long pages by line ranges or with `grep`. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 (order 37 of [its orders](../proposals/full-review/orders.md)) found
facts that only the coordinator's private notes held, and settings that only GitHub holds. This
order writes each into the tree, so that a coordinator taking over, or the owner, needs nothing
private to know them. It carries
[FR-532](../proposals/full-review/findings/G6-outside-the-tree.md#fr-532) as its verifier corrected
it, [FR-538](../proposals/full-review/findings/G6-outside-the-tree.md#fr-538),
[FR-533](../proposals/full-review/findings/G6-outside-the-tree.md#fr-533), rows 7 and 8 of
[FR-525](../proposals/full-review/findings/G6-outside-the-tree.md#fr-525), and the sentence for
`CONTRIBUTING.md` of [FR-534](../proposals/full-review/findings/G6-outside-the-tree.md#fr-534).
[Ruling 52](../proposals/ruling-52/README.md) asks every result to be as explicit and descriptive
as possible; here each fact says where it was read, and what is not known says so. Every edit is
the coordinator's, and none changes behavior: nothing under `spec/`, no case and no code changes.
The repository's settings are the owner's: this order says what they are and what the review asks
the owner to turn on, and it turns nothing on.

- **The '(log)' marks.** The handover marks with "(log)" a fact found only in the coordinator's
  private notes (lines 9 and 10). FR-532 counted eleven at the reviewed commit. Orders 155 and
  161 recorded A78's two lists, and order 173 the Windows sandbox line, the load rule and 081's
  failed dispatches; [order 182](182-stale-state-lines.md) left the other six to this order. On
  `main` at `0ed26306`, `grep -c '(log)' docs/implementation/HANDOVER.md` prints 7: the six and
  the definition. Each of the six is in the tree already, or the handover's own words supply it:
  - "Ask the owner before part 2" (line 129): the tree already records that part 2 waits for the
    owner, as FR-532's verifier notes (the handover's open items and the tracker), and
    [ruling 77's "What follows"](../proposals/ruling-77/README.md#what-follows) gives the setup's
    sudo steps to the owner. FR-532 asks for one clause in the guide.
  - The second machine's capacity (lines 130 and 131): ruling 77 says only that WSL keeps its
    default; FR-532 asks for one sentence in the guide.
  - The first-start report's architecture names (line 164): order 155 logged them as finding
    4-28, which leaves out one name the handover lists. The shell passes `wasm32` and `wasm64`
    through (`engine/crates/shell/src/lib.rs` lines 720 to 728), where the local machine
    provider's table gives Go's `wasm` for both (`packages/provider-local-machine/src/lib.rs`
    line 30).
  - 049a1's one live node per machine (lines 214 and 215): the Record on the order's pushed
    branch, `wo/049a1-build-enrollment-core`, quotes the decision, at commit `769fbcbe`, lines 692
    to 702 of the order's file there.
  - The side branch `s3/m2` (line 232): point 4 of [order 120](120-plan-providers.md), under
    [ruling 76's later answer](../proposals/ruling-76/README.md#later-answers) of 2026-11-11.
  - The keys printed on 2026-11-11 (line 250): D14 of
    [the review's decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification)
    closes it with one dated line that cites [ruling 75](../proposals/ruling-75/README.md)'s note
    on the API keys: no commit of any branch, tag or pull-request ref held them, and the owner
    accepted leaving them as they are. Nothing is asked again.
- **Checks run by hand** (FR-525, row 7; FR-533).
  [Order 042's Record](042-build-tombstones-and-inventory.md#record) says that 491 vectors
  passed, and the handover says 494. The tree at its merge, `57813f06`, holds 494 cases; 491
  matches what it held before order 077's three cases reached `main` (443 at `575a1bcd^`, and
  042's 48). [Order 077](077-spec-reconcile-fenced.md#record) merged on the Mac's runs while its
  pull request's CI skipped the engine's shards. No CI run is cited by address anywhere in the
  tree, and the handover says of the CI fix (#266) only "Six CI runs in a row passed". The
  coordinator read the runs on GitHub on 2026-11-15; decisions 2 and 5 name them.
- **GitHub's settings** (FR-525, row 8; FR-533; FR-534). The section "Settings this repository
  relies on" that [order 151](151-private-names-and-secret-scan.md) wrote in `CONTRIBUTING.md`
  names `main`'s protection and the variable `CONDUIT_PRIVATE_NAMES`, but not the protection's
  rules, nor whether the variable is set. The coordinator read the settings on 2026-11-15 with
  the read-only commands under "Done when":
  - `main` requires a pull request, with no approving review, binds administrators too, and
    refuses force-pushes and deletion, as [ruling 74](../proposals/ruling-74/README.md), point 1,
    asks. No status check is required, and no ruleset is defined. When the protection was first
    applied is recorded nowhere the coordinator can read.
  - GitHub's secret scanning and its push protection are on.
  - The repository has no Actions variable or secret, so `CONDUIT_PRIVATE_NAMES` is not set.
    CI's `docs` job prints the step's environment in its log (run 43805746528 shows the
    variable's line, empty), GitHub masks a secret there but not a variable, and the repository
    and its logs are public. Set as a variable, it would publish the names it guards.
  - On 2026-11-11 CI was paused (the Records of orders 104 and 105 say so). The runs of the six
    commits that reached `main` from `a7c176d8` (#139) to `147f0582` (#144) were cancelled or
    never started; none of the 54 from `3bc816c5` (#145) to `defee9ba` (#198, order 068) has a
    run; and no pull request's run started in that time. CI ran again from `1b717c40`, order
    121's merge.
- **D11's step for the owner.** D11 asks the owner, outside the ruling batch, to make `docs` and
  `rust` required on `main` and to turn on secret scanning with push protection, once the CI fix
  has held over three runs and the engine's test shards wait on no other job. Since
  [order 152](152-ci-engine-shards-stand-alone.md) the shards need only the engine's build. The
  fix held once #268 joined it: #266's first three runs on `main` passed, but between them #267's
  pull request run failed once in `rust (packages/provider-ssh-runtime)`, and #268 made two more
  of that package's tests retry; from #268 on, 46 runs on `main` in a row passed. The next, on
  `0ed26306`, failed in `macos (packages/provider-local-process)`, one of the macOS jobs that
  [order 187](187-ci-macos-leg.md) added to `rust` after D11 was written. Whether `rust` is
  required before those jobs have held is the owner's, and is asked with the step. The second
  half of D11 is already on. The `secrets` job, which order 151 added after D11 was written, is
  named beside the two as the coordinator's addition, which the owner may drop.
- **The metrics proposal's cost numbers** (FR-538) cite a benchmark and a probe store that were
  kept in a session's scratch space, outside the repository.

You have no network: write the facts about GitHub as this order gives them. The admin reads them
again before merge.

This order leaves these, each for the reason given:

- Rows for the decisions of 049a1 and `s3/m2` on the owner's review list: FR-532 asks that they
  be added once, with the other decisions that the handover lists as logged since (row 15 of its
  queue). This order says only where each is recorded.
- The base commit of order 070's timings (FR-525, row 7): nothing in the tree gives it.
- A dated table of each second-machine setup step's check (FR-538;
  [FR-526](../proposals/full-review/findings/G6-outside-the-tree.md#fr-526), part 5): the
  results are in the coordinator's private note, which no worker reads, and the guide keeps them
  outside the repository (order 184).
- The tag on commit `0a9a3232` (D87): a step the coordinator takes itself, with no file.
- How CI reads `CONDUIT_PRIVATE_NAMES`: reading it from a secret would change
  `.github/workflows/checks.yml` and needs a secret that only the owner can set, so it goes to
  the owner first.
- The failure of `macos (packages/provider-local-process)` on `main`: a test fix, not a document.
- The differential build, in no workflow (FR-533): [order 188](188-differential-vectors-nightly.md).
- FR-538's points on the lane coordinator's context and on the adopted standards: the finding
  asks nothing of the tree for them.

Orders 188 and 189 are written and change `docs/implementation/METHOD_AS_RUN.md` and the
workflows, not the files of this order. Order 192 may change `docs/AUDIT.md`'s action items (A40's
status), not its "Waiting for the owner" table; whichever merges later is rebased by the admin,
who keeps both. Build on each file as it is on `main` when you start.

## The decisions it applies

1. **How each edit is made.**
   - Quoted old text is as `main` holds it at `0ed26306`, at the lines named. If an order merged
     since has moved it, find the same text. If the text itself has changed, apply the nearest
     sound reading and say so in your report.
   - A blockquote gives the new text exactly, except DATE, which is the day you write it, as
     `date +%F` prints it. Where a decision says that words in it are a link, make them a
     Markdown link to the target named, written relative to the file that holds it. Every target
     named below is a link from this order's own directory. Each "run N" in a blockquote is a
     link to https://github.com/shrav-k/conduit/actions/runs/N, N being the run's number.
   - Keep every Markdown line that you write or rewrap at most 100 characters. Rewrap only the
     sentence, bullet or paragraph that a decision changes, at its own indent. A table row stays
     on one line.
   - Nothing else in any file changes. Add no "(log)" mark.
2. **The handover**, `docs/implementation/HANDOVER.md` (FR-532; FR-525, row 7; FR-533).
   - Lines 9 and 10: the sentence "a fact found only in the coordinator's private notes is
     marked "(log)" and is still to be recorded." becomes:

     > a fact found only in the coordinator's private notes is marked "(log)" until it is
     > recorded, and since work order 191 no fact is so marked.

     Line 9 is 116 characters on `main`: rewrap lines 8 to 11 so that each is at most 100.
   - Line 129: "Ask the owner before part 2 (log)." becomes "Ask the owner before part 2, since
     ruling 77 gives the setup's steps to the owner." The words from "ruling 77" on are a link to
     [ruling 77's "What follows"](../proposals/ruling-77/README.md#what-follows).
   - Lines 130 and 131, the bullet "Capacity", become:

     > - **Capacity**: WSL keeps its default 15 GB (M2-7). Five orders reached 12 GB; before 070,
     >   shell test binaries took 5 to 8 GB each and ran it out of memory once. The coordinator
     >   saw these figures up to 2026-11-12, and no measurement of them is in the repository (the
     >   guide).

     "the guide" is a link to
     [the guide's "How the computer joins"](../guides/second-machine.md#how-the-computer-joins).
   - Line 153, in the paragraph on #266: "Six CI runs in a row passed." becomes:

     > The pull request's CI run, run 43584337827 on its head `01c53ed9`, passed six attempts in a
     > row. CI's first three runs on `main` from the merge on passed every job: run 43615068193 on
     > `255b3206`, run 43660743667 on `2b133486` (#267) and run 43662044232 on `8873bedf` (#268).
     > Between them, #267's pull request run, run 43659191448, failed once in
     > `rust (packages/provider-ssh-runtime)` and passed on its second attempt, and #268 made two
     > more of that package's tests retry a start that follows a held lock.

   - Line 164, row 8 of the queue. In its State cell, "mirror order 128's mapping (PRV-014)
     (log)" becomes "mirror order 128's mapping (PRV-014). Logged as finding 4-28", where
     "finding 4-28" is a link to [increment 4's log](../proposals/first-build/log-increment-4.md).
     Its Next step cell, "A small order, or a fix under 71 point 4", becomes "The review's order
     66, after the pin (F78), as 4-28's settlement says", where "order 66" is a link to
     [the review's orders](../proposals/full-review/orders.md) and "F78" to
     [its decisions](../proposals/full-review/decisions.md#providers-runs-and-machines). The rest
     of the row does not change.
   - Lines 214 and 215 become:

     > - 049a1: one live node per machine; acceptances decided one at a time; a second refused
     >   with a conflict naming the node. The Record on the order's pushed branch,
     >   `wo/049a1-build-enrollment-core` at `769fbcbe`, quotes the decision in lines 692 to 702
     >   of the order's file there.

     You may check the quote with
     `git show 769fbcbe:docs/work-orders/049a1-build-enrollment-core.md | sed -n 692,702p`.
   - Line 232 becomes:

     > - The side branch `s3/m2` for the Windows engine work, merged after the rerun's pin:
     >   point 4 of order 120's plan, under ruling 76's later answer of 2026-11-11.

     "order 120's plan" is a link to [order 120](120-plan-providers.md), and "ruling 76's later
     answer" to [ruling 76's later answers](../proposals/ruling-76/README.md#later-answers).
   - Lines 249 and 250: "On 2026-11-11 an agent printed a session's environment, which held keys
     (log)." becomes:

     > On 2026-11-11 an agent printed a session's environment, which held API keys. The
     > exposure was closed by ruling 75's note of the same day: the coordinator found the keys
     > in no commit of any branch, tag or pull-request ref, and the owner accepted leaving them
     > as they are (D14).

     "ruling 75's note" is a link to [ruling 75's record](../proposals/ruling-75/README.md), and
     "D14" to
     [the review's decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification).
     The bullet's next sentences, from "Every brief that may run a command", do not change.
3. **The second-machine guide**, `docs/guides/second-machine.md` (FR-532).
   - In the paragraph of lines 21 to 26, after "(step 2).", add "The coordinator asks the owner
     before starting any of the three." The paragraph already says "Ruling 77's setup is the
     owner's". "Tell the coordinator where a step differs." stays last.
   - After the bullet "On demand" (lines 48 to 51), add this bullet:

     > - **Capacity:** WSL keeps its default memory limit, 15 GB on this computer; M2-7 needed
     >   no ruling (ruling 77, its other questions). Five orders reached 12 GB, and before order
     >   070 the shell's test binaries took 5 to 8 GB each and ran it out of memory once. The
     >   coordinator saw these figures up to 2026-11-12, and no measurement of them is in the
     >   repository. The method as run gives the load rule.

     "ruling 77, its other questions" is a link to
     [ruling 77's other questions](../proposals/ruling-77/README.md#the-other-questions), and "the
     load rule" to [the method as run's "Load"](../implementation/METHOD_AS_RUN.md#load).
4. **Increment 4's log**, `docs/proposals/first-build/log-increment-4.md` (FR-532). In row
   4-28's Finding cell, "big-endian ARM gets `arm` and `arm64` for `armbe` and `arm64be`."
   becomes "big-endian ARM gets `arm` and `arm64` for `armbe` and `arm64be`; WebAssembly's
   `wasm32` and `wasm64` stay as they are for `wasm`." Nothing else in the row or the log
   changes.
5. **The Records of 042 and 077** (FR-525, row 7; FR-533). At the end of each Record, after a
   blank line, add one paragraph.
   - In `docs/work-orders/042-build-tombstones-and-inventory.md`:

     > Added on DATE by work order 191 (FR-525, row 7), the evidence at merge. `main` at
     > `57813f06` (#264) holds 494 cases: from the repository root,
     > `git ls-tree -r --name-only 57813f06 conformance | grep -c '/case.yaml$'` prints 494.
     > The 491 above matches the tree before order 077's three cases reached `main`: 443 cases at
     > `575a1bcd^`, and this order's 48. The handover reports that all 494 pass on the normal and
     > the differential builds after the rebase onto 077, the differential run taking about 2.4
     > hours on the Mac; the commands and last lines of those runs are not in the tree. The pull
     > request's CI run, run 43312121448 on its head `e33c121f`, and CI's run on `main` after the
     > merge, run 43315266140, passed every job, among them the engine's four test shards, which
     > run the vectors on the normal build. No CI job ran the differential build then.

     "FR-525, row 7" is a link to
     [FR-525](../proposals/full-review/findings/G6-outside-the-tree.md#fr-525), and "The
     handover" to [the handover's queue](../implementation/HANDOVER.md#the-queue).
   - In `docs/work-orders/077-spec-reconcile-fenced.md`:

     > Added on DATE by work order 191 (FR-525, row 7), the evidence at merge. `main` at
     > `575a1bcd` (#263) holds 446 cases: from the repository root,
     > `git ls-tree -r --name-only 575a1bcd conformance | grep -c '/case.yaml$'` prints 446.
     > The pull request's one CI run, run 43279956068 on its head `edb0160e`, failed on both its
     > attempts in `rust (packages/provider-ssh-runtime)`, so the engine's four test shards and
     > its doctests were skipped. The order merged on the Mac's runs of the engine's suite and the
     > 446 vectors; their commands and last lines are not in the tree. CI's run on `main` after
     > the merge, run 43282660601, passed every job, the four engine shards among them.

     "FR-525, row 7" is the same link as in 042's paragraph.

   Run the two `git ls-tree` commands, and `git ls-tree -r --name-only 575a1bcd^ conformance |
   grep -c '/case.yaml$'`, which prints 443, and give what each printed in your report.
6. **`CONTRIBUTING.md`** (FR-533; FR-534; FR-525, row 8). Lines 271 to 279, the body of
   "Settings this repository relies on", become:

   > These settings are held by GitHub, not by the tree, and they are the owner's: no pull
   > request changes them. Here they are as read on 2026-11-15 with read-only `gh api` and
   > `gh run` calls: on the repository, its `branches/main/protection`, its `rulesets`, its
   > `actions/variables` and `actions/secrets`, the last two only counted so that no value is
   > printed, and its CI runs.
   >
   > - **`main` is protected** under ruling 74, point 1: every change arrives through a pull
   >   request, with no approving review required; the rule binds administrators too; and
   >   force-pushes and deletion are refused. No ruleset is defined. When the protection was
   >   first applied is not recorded.
   > - **No status check is required** by a branch setting. The admin merges a pull request only
   >   after its `docs`, `rust` and `secrets` checks pass; `rust` passes only when every Rust job
   >   does, the macOS jobs of order 187 among them. The full review's decision D11 asks the
   >   owner to make `docs` and `rust` required once the CI fix (#266) has held over three runs
   >   and the engine's test shards wait on no other job. Since order 152 the shards wait only on
   >   the engine's build. CI's first three runs on `main` after #266 passed, but between them
   >   #267's pull request run failed once in `rust (packages/provider-ssh-runtime)`, and #268
   >   made two more of that package's tests retry. From #268 on, 46 runs on `main` in a row
   >   passed; the next, run 43806163460 on `0ed26306`, failed in
   >   `macos (packages/provider-local-process)`, a job that order 187 added after D11. The
   >   coordinator adds `secrets`, which order 151 created after D11. The step and its timing are
   >   the owner's, and the tracker lists them.
   > - **GitHub's secret scanning and its push protection are on**, the rest of D11. CI's own
   >   `secrets` job also runs a pinned gitleaks release on the commits each pull request or push
   >   to `main` adds; its configuration in .gitleaks.toml extends the default secret-detection
   >   rules.
   > - **`CONDUIT_PRIVATE_NAMES` is not set**: the repository has no Actions variable or secret.
   >   CI's `docs` job passes a repository variable of that name to
   >   `tools/check_private_paths.py`, beside the checker's seven built-in patterns:
   >   home-directory paths, session scratch paths, Codex login files, tailnet DNS names, tailnet
   >   IPv4 addresses, SSH key fingerprints and Windows home paths. The variable is meant to
   >   list, separated by commas, the private names that no pattern finds: the user, host,
   >   machine and account names that never enter the tree. Set as a variable, it would publish
   >   them: the job's log prints the step's environment, GitHub masks a secret there but not a
   >   variable, and this repository's logs are public. The admin's own run of the checker
   >   before each merge takes the names from the coordinator's private note.
   > - **CI was paused once.** On 2026-11-11 the `checks` workflow was paused. The runs of the
   >   six commits that reached `main` from `a7c176d8` (#139) to `147f0582` (#144) were
   >   cancelled or never started, none of the 54 from `3bc816c5` (#145) to `defee9ba` (#198,
   >   order 068) has a run, and no pull request's run started in that time. CI ran again from
   >   `1b717c40`, order 121's merge.

   The links: "ruling 74, point 1" to [ruling 74](../proposals/ruling-74/README.md); "decision
   D11" to
   [the review's decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification);
   "CI's first three runs on `main` after #266" to
   [the handover's queue](../implementation/HANDOVER.md#the-queue); "the tracker" to
   [what waits for the owner](../AUDIT.md#waiting-for-the-owner); ".gitleaks.toml" to
   [the file](../../.gitleaks.toml), as now; and "The admin's own run of the checker before each
   merge" to
   [the method as run, section 7](../implementation/METHOD_AS_RUN.md#7-merge-and-the-record).
   The heading does not change.
7. **The tracker**, `docs/AUDIT.md` (D11). At the end of the table under "Waiting for the
   owner", after the row on readiness G8 (line 177), add one row. Its first cell:

   > Required status checks on `main` (D11): make `docs` and `rust` required, and `secrets` with
   > them, so that GitHub refuses to merge a pull request before they pass, administrators
   > included. `secrets` is the coordinator's addition, which the owner may drop. Secret
   > scanning and its push protection, the rest of D11, are on. D11's two conditions, and CI's
   > runs as read on 2026-11-15, among them a failure in a macOS job that `rust` has gathered
   > since order 187, are in the settings section. Until then the admin waits for the three
   > checks itself.

   Its second cell: "The settings this repository relies on; D11", where the first words are a
   link to
   [`CONTRIBUTING.md`'s section](../../CONTRIBUTING.md#settings-this-repository-relies-on) and
   "D11" to
   [the review's decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification).
8. **The metrics proposal**, `docs/proposals/metrics/general.md` (FR-538). Before line 1366,
   "MEASUREMENTS AND SAVED SOURCES (the coordinator's session notes, not in the repository)", add
   this paragraph, with a blank line after it:

   > Added on DATE by work order 191 (FR-538): the benchmark's source and output and the probe
   > store named below stayed in a session's scratch space, and none is in the repository. So no
   > reader of the tree can check the numbers that cite them: the benchmark's rerun in point 1 of
   > "WHAT MUST COME FIRST, AND THE COST" and in point 7 of "ENGINE WORK BEFORE THE METRICS
   > BATCH", the table under "Cost evidence", and the sizes of the 2026-11-10 probe store's
   > `install.v1` entries in point (a) of the engine prerequisites and in point 1 of "ENGINE WORK
   > BEFORE THE METRICS BATCH". The full review's D16 has the store's cost measured before any
   > store order.

   "FR-538" is a link to
   [FR-538](../proposals/full-review/findings/G6-outside-the-tree.md#fr-538), and "D16" to
   [the review's decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification).

## Scope

Change only these eight files, and in each only what its decision names:

- `docs/implementation/HANDOVER.md` (decision 2);
- `docs/guides/second-machine.md` (decision 3);
- `docs/proposals/first-build/log-increment-4.md`, row 4-28 (decision 4);
- `docs/work-orders/042-build-tombstones-and-inventory.md` and
  `docs/work-orders/077-spec-reconcile-fenced.md`, one paragraph each at the end of the Record
  (decision 5);
- `CONTRIBUTING.md`, the body of "Settings this repository relies on" (decision 6);
- `docs/AUDIT.md`, one row (decision 7);
- `docs/proposals/metrics/general.md`, one paragraph (decision 8).

Nothing under `spec/`, `conformance/`, `engine/`, `packages/`, `experiments/`, `tools/` or
`.github/` changes, nor `IN_FORCE.md`, the ledger or any ruling's record. No vector changes, so
the order writes no declaration.

## Done when

From the repository root, each prints what its comment gives:

```sh
grep -c '(log)' docs/implementation/HANDOVER.md                                   # 1
grep -oh 'actions/runs/[0-9]*' docs/implementation/HANDOVER.md docs/work-orders/042-build-tombstones-and-inventory.md docs/work-orders/077-spec-reconcile-fenced.md | sort -u | wc -l   # 9
tr -s ' \n' '  ' < docs/work-orders/042-build-tombstones-and-inventory.md | grep -c 'holds 494 cases'   # 1
tr -s ' \n' '  ' < docs/work-orders/077-spec-reconcile-fenced.md | grep -c 'holds 446 cases'   # 1
grep -c 'CONDUIT_PRIVATE_NAMES' CONTRIBUTING.md                                   # 1 or more
tr -s ' \n' '  ' < CONTRIBUTING.md | grep -c 'GitHub masks a secret there but not a variable'   # 1
tr -s ' \n' '  ' < CONTRIBUTING.md | grep -c 'from `a7c176d8` (#139) to `147f0582` (#144)'   # 1
grep -c '^| Required status checks on' docs/AUDIT.md                              # 1
grep -c 'wasm64' docs/proposals/first-build/log-increment-4.md                    # 1
tr -s ' \n' '  ' < docs/guides/second-machine.md | grep -c 'asks the owner before starting any of the three'   # 1
grep -c 'Capacity:' docs/guides/second-machine.md                                 # 1
grep -c 'work order 191' docs/proposals/metrics/general.md                        # 1
git rev-list --first-parent --count 9ee84fe2..147f0582                            # 6
git rev-list --first-parent --count 147f0582..defee9ba                            # 54
git diff -U0 | grep -E '^\+[^+|]' | awk 'length > 101'                             # prints nothing
python3 tools/check_docs.py                                                       # Errors: 0
python3 tools/check_rulings.py
python3 tools/check_roadmap.py
python3 tools/check_private_paths.py                                              # Errors: 0
git diff --check
git status --short                                                                # the eight files of the scope
```

The first and sixth counts must differ on `main`. Run them against `HEAD`, and give both results
in your report:

```sh
git show HEAD:docs/implementation/HANDOVER.md | grep -c '(log)'                   # 7
git show HEAD:CONTRIBUTING.md | tr -s ' \n' '  ' | grep -c 'GitHub masks a secret there but not a variable'   # 0
```

Do not stage anything. In your report, mark each choice you made yourself as your own, and give
the last line of every check.

Before merge, the admin reads GitHub again from the repository root with these read-only
commands, which print no secret or variable value:

```sh
gh api 'repos/{owner}/{repo}' --jq '[.visibility, .security_and_analysis.secret_scanning.status, .security_and_analysis.secret_scanning_push_protection.status] | join(" ")'   # public enabled enabled
gh api 'repos/{owner}/{repo}/branches/main/protection' --jq '"approvals=\(.required_pull_request_reviews.required_approving_review_count) admins=\(.enforce_admins.enabled) force=\(.allow_force_pushes.enabled) deletions=\(.allow_deletions.enabled) checks=\(.required_status_checks)"'   # approvals=0 admins=true force=false deletions=false checks=null
gh api 'repos/{owner}/{repo}/rulesets' --jq 'length'                    # 0
gh api 'repos/{owner}/{repo}/actions/variables' --jq '.total_count'     # 0
gh api 'repos/{owner}/{repo}/actions/secrets' --jq '.total_count'       # 0
for r in 43584337827 43615068193 43660743667 43662044232 43312121448 43315266140 43279956068 43282660601 43806163460; do gh run view $r --json conclusion --jq .conclusion; done   # success, except failure for 43279956068 and 43806163460
gh api 'repos/{owner}/{repo}/actions/runs/43659191448/attempts/1' --jq .conclusion   # failure
for c in $(git rev-list --reverse --first-parent 2b133486..0ed26306); do gh run list --workflow checks.yml --commit $c --event push --json conclusion --jq '.[0].conclusion'; done | uniq -c   # 46 success, then 1 failure
gh run list --workflow checks.yml --branch main --event push --limit 5 --json headSha,conclusion --jq '.[] | "\(.headSha[0:8]) \(.conclusion)"'   # main's latest runs, for decision 6's sentence on the macOS job
gh run view --job 133096008581 --log | grep -c 'CONDUIT_PRIVATE_NAMES:'   # 1: run 43805746528's docs job prints the line
for c in $(git rev-list --reverse --first-parent 9ee84fe2..147f0582); do gh run list --workflow checks.yml --commit $c --event push --json status,conclusion --jq '.[0] | "\(.status) \(.conclusion)"'; done | uniq -c   # 5 completed cancelled, then 1 queued
for c in $(git rev-list --first-parent 147f0582..defee9ba); do gh run list --workflow checks.yml --commit $c --json databaseId --jq length; done | sort | uniq -c   # 54 0
gh run list --workflow checks.yml --created '2026-11-11T20:29:00Z..2026-11-12T03:39:00Z' --limit 100 --json databaseId --jq length   # 0
```

If an answer differs, the admin corrects decisions 6 and 7's text to match and says so in the
Record. The admin then shows CI's `docs`, `rust` and `secrets` checks passing on the pull
request, and a fresh read-only Opus reader reads the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).
