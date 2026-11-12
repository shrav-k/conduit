# Full review: a brief for the next coordinator

Not binding. The [specification](../../spec/README.md) alone binds. Written on 2026-11-12, at
`main` `57813f06` or later at the handover, when the admin session (Claude Opus) handed the
coordinator's work to a new session (Claude Fable). The owner asked that session for a full
review of the repository and of the progress so far, and expects it to decide on faults in the
language, change the approach where it is wrong, and alter future experiments to test both. This
brief says how to review and where to look. It does not review, and it decides nothing.

It builds on [the cold readers' instructions](cold-readers.md): use their reading order and
known-open list. For this review, its own classes and record ([what to produce](#what-to-produce))
replace their report form and their destination, the next re-audit round. Unlike a cold reader,
you are also the coordinator: you settle what rulings 37, 46 (point 4) and 71 (point 4) let a
coordinator settle, and put the rest to the owner.

## The state you inherit

- **In flight at the checkpoint**: nothing. Checkpointed, not merged, their branches pushed: 071
  (the conformance manifest in pieces; one must after two rounds), 049a1 (enrollment in the core,
  for the side branch `s3/m2`; five rounds, new musts in each, one must and one should left), 080
  (the shell's runtime-provider calls, for `s3/m2`; three musts in its HTTP bridge after two
  rounds) and the lane's 144 (S4-SRV2; two musts). `main`'s CI is red at times: an SSH-R test fails
  on CI's Linux runner, and the engine shards are then skipped; it is the first fix. The dogfood
  engine is still pinned before 070.
  [The handover](../implementation/HANDOVER.md) gives the queue, the pipeline and the second
  machine.
- **Merged this session**, among others: 042 (the first order whose four rounds ran through
  Conduit's own Agents; 494 vectors), 077 (fenced reconciliation), 070 (the engine 20 to 40 times faster; CI
  shards 1 to 8 minutes), 074, 081 and 082; ruling 77 and later answers to rulings 42, 52, 71, 72
  and 75; the runtime-provider kit, the local-process, local machine, local-directory and
  file-store providers, and SSH-R with its drainer (146, 149 and 150, #260); the provider-contract harness
  (experiment 022); S2's contracts, stub, Conduit-aware implementation and adapter; SCR1;
  S4-SRV1.
- **Waiting for the owner**: [the tracker's list](../AUDIT.md#waiting-for-the-owner); part 2 of
  [ruling 77](../proposals/ruling-77/README.md)'s Windows setup, which can run now (part 1 is done:
  `conduit-run` has its tools and its Codex, signed in by the owner; the coordinator signs nothing
  in); S3's fault means (S3-OF in [S3's plan](../../experiments/014-s3-two-machines/plan.md), A
  recommended); S2's plain server
  (S2-PL in [S2's plan](../../experiments/013-s2-shared-login/plan.md), before S2's runs); and
  S1's bar after the rerun.
- **The second accounts are reserved.** They are used only through Conduit, signed in through
  its Interface, as the test of each Agent's own subscription and of the model as a whole, and
  never as capacity for work orders ([ruling 72's later answers](../proposals/ruling-72/README.md#later-answers)).
  The sign-in chain that test needs is unwritten: 050 (separate homes, hooks, the account check),
  048a, 048b and 051b, then A81 and A82.
- Machine names, accounts and the scripts that run orders are in the private handoff note,
  `.wo/notes/HANDOFF-local.md` beside the main checkout, outside the repository.

## Reading order

1. [AGENTS.md](../../AGENTS.md), and above all [what holds](../../AGENTS.md#what-holds): `spec/`
   alone binds behavior, then IN_FORCE.md, then the ledger, then each ruling's record. The
   roadmap, `docs/implementation/`, `docs/research/`, `docs/archive/` and `experiments/` bind
   nothing.
2. The cold readers' [reading order](cold-readers.md#what-to-read), ending with the promises in
   [contents order](../../spec/README.md#contents).
3. [IN_FORCE.md](../../IN_FORCE.md), by topic. Each line cites rulings by number and point, and
   promises by ID. *In spec/* points at text that exists; *spec pending* is ruled and unwritten,
   and names its batch or order; *Open* is the owner's. A line that differs from the owner's
   words in its ruling is a defect, and the owner's words govern.
4. [The ledger](../proposals/ruling-log.md): find a ruling by number, read the owner's words and
   the status column (which later rulings amend it), then the record's first line. Answers that
   changed no decision are dated under the record's *Later answers* (ruling 67).
5. The state: [the tracker](../AUDIT.md) (who decides, known to be open, the
   [layered claim](../AUDIT.md#layered-claim), [purpose](../AUDIT.md#purpose) and preservation
   tests), [the roadmap](../../ROADMAP.md), [increment 4's log](../proposals/first-build/log-increment-4.md),
   [the review list](../proposals/first-build/to-review-with-the-owner.md),
   [G5's account](../../experiments/012-model-checking/G5.md), and the orders' Records: up to 066
   in [the record](../implementation/WORK_ORDERS.md#the-record), later ones at the foot of each
   order file under `docs/work-orders/`.

## Method

- **Pin a commit.** State it and the date at the top of your report. Recount every number you
  cite: several documents carry counts from earlier commits.
- **Verify against `spec/`, never against a plan.** A plan, a pre-registration, a CLI reference
  or a Record says what someone meant or did; only the promise says what holds. Quote it. A
  passing vector shows its slice, not the whole promise.
- **Class every finding**: *defect* (the text is wrong or cannot be followed), *ambiguity* (two
  sound readings two builders could take), *gap* (a situation no promise decides),
  *contradiction* (two promises, or a procedure and a safety promise, that cannot both hold) or
  *needless complexity* (a concept, file or step the purpose does not need). Mark *unsafe* any by
  which an engine keeping every promise could grant access it should not, leak a secret, lose or
  repeat an effect, or report a wrong result ([the tracker's test](../AUDIT.md#how-to-use-this-document)).
- **Source every finding** with [the cold readers' fields](cold-readers.md#how-to-report-a-finding),
  at your commit, classed as above in place of their wrong, undetermined and misleading.
- **Check that it is new**: [known to be open](../AUDIT.md#known-to-be-open), the cold readers'
  [list](cold-readers.md#what-is-known-to-be-open), the increment logs, G5's findings, the review
  list and [queued below](#queued-and-not-yet-written). Raise a known item again only with a new
  situation.
- **Separate who decides.** The owner rules behavior, scope, purpose and the order of the work,
  and any fix that adds or removes a feature beyond ruling 71's small ones. A coordinator fixes a
  defect, ambiguity or contradiction whose fix adds no behavior (rulings 37; 46, point 4), and a
  small finding that adds a feature (ruling 71, point 4; until the S1 rerun, none that could move
  S1's counts). Log each for the owner to strike; add any that changes what the language says to
  the review list.
- **Prefer existing paradigms** ([ruling 40](../proposals/ruling-40/README.md)). Conduit innovates
  only on connection and access. For each fix, name the standard or system it follows and what
  that system does; a fix that invents infrastructure is wrong. Prefer a provider's contract to
  engine machinery.
- **Make every result explicit** ([ruling 52](../proposals/ruling-52/README.md)): a refusal,
  diagnostic or limit says what happened and why, unless that discloses what the caller may not
  learn. Terse text is a finding.
- **Keep rulings few** ([ruling 67](../proposals/ruling-67/README.md)). Batch the owner's
  questions into one ruling; an answer that changes no decision is a dated note in the existing
  record. Do not ask the owner what a coordinator may settle.
- **Delegate the mechanical parts.** Recounts, cross-reference sweeps and fresh reads suit Codex
  work orders with a fresh read-only verifier ([work orders](../implementation/WORK_ORDERS.md));
  keep the judgment. When the Codex allowance runs out, workers pause and you tell the owner
  (ruling 71's later answer).

## Focus 1: faults in the language

| Area | What to look for | Where |
| --- | --- | --- |
| The catalogue | 519 promises: 226 `language`, 245 `convention`, 48 `interface`. Does each layer pass LD1's test? Are 245 convention promises still "the few behaviors every part must agree on"? One obligation per promise; promises with no vector | [contents](../../spec/README.md#contents), [form of a promise](../../spec/README.md#the-form-of-a-promise), [layer decisions](../AUDIT.md#layer-decisions), [manifest](../../conformance/manifest.yaml) |
| `spec/` against IN_FORCE | Every *spec pending* line: is the text still missing, or did a merged order write it and the line lags (topic 11's order 081 line is one to check)? Every *In spec/* line: does the cited promise say it? | IN_FORCE.md; `git log` for the order; the promise |
| Model checking | G5 accounts for 217 convention-layer safety promises: 114 hold in bounded projections, 7 only under ruling 75 A1, then contradictions MC6-001 and MC6-002, gaps MC4-002, MC4-009 and MC7b-002, ambiguities MC4-001 and MC7b-001, over-broad properties MC3-001 and MC3-002; 53 static, 11 excluded, 10 left to MC9. Did 081 and 082 close what they claimed? Are the 11 exclusions acceptable? 17 promises are not in the catalogue: CTR-014, ENR-001 to ENR-012, PKG-032, PND-008, PND-013 and PRV-030 | [G5's findings](../../experiments/012-model-checking/G5.md#findings), [what G5 still needs](../../experiments/012-model-checking/G5.md#what-g5-still-needs), [catalogue](../implementation/SAFETY_CATALOGUE.md), Records of [081](../work-orders/081-spec-one-run-per-instance.md) and [082](../work-orders/082-spec-records-and-installs.md) |
| Stated limits | Is each limit right for v0, or a fault renamed a limit? A bound does not limit filters; `management` cannot be narrowed below whole operations; the engine's OS user reaches every token and state directory; every restriction reported as mediated (preservation test 9); own tools neither enforced nor mediated; a run beside a later instance | [limits of this version](../../spec/README.md#limits-of-this-version) and its sections |
| Short form and canonical | Is the translation total, and its meaning the canonical one (SCT-003, SCT-015)? `sections.yaml` has 15 promises and no vectors; experiment 011's 2026-11-11 run passed 14 of 14 exact states (the 2026-11-10 run, 0 of 14, predates `as` in the schemas) | [what the short form covers](../../spec/README.md#what-the-short-forms-meaning-covers), [011](../../experiments/011-short-form-spike/README.md), [023](../../experiments/023-paper-count/README.md) |
| Authority and access | Bounds (ACC-029, ACC-030, ACC-054, ACC-055), tombstones and lift (LIF-040, LIF-041, ACC-053), ACC-033's table, two-tier refusals (ACC-056, OPS-023), the Fleet papers' unverified gaps; whether the vector that failed after 042's rebase was a language case | `access.yaml`, `lifecycle.yaml`, [042](../work-orders/042-build-tombstones-and-inventory.md) |
| Retries and the fence | OPS-014's record of an absent effect only once the attempt is fenced, against OPS-024 and OPS-018 (MC2-001); OPS-030; ruling 75 A1's two limits (a server's onward call; a remote server that never answers). Does 077's text close MC2-001 without new machinery? | `operations.yaml`, [ruling 75](../proposals/ruling-75/README.md), [077](../work-orders/077-spec-reconcile-fenced.md) |
| One run per instance | PRV-019, PRV-021, PRV-030, LIF-036, TPL-018: MC4-009's restart race; MC4-002's overlap, a stated limit, with the owner's answer that a shipped runtime provider carries out an owed stop before any start (MC4) | `providers.yaml`, [ruling 42's later answers](../proposals/ruling-42/README.md#later-answers), [PROVIDERS.md](../implementation/PROVIDERS.md) |
| Records and loss | REC-003, REC-006 and DL8 after 082: is a loss the engine cannot tell a limit or a broken promise? What does an engine owe when it cannot store state? | `records.yaml`, [what records cover](../../spec/README.md#what-the-promises-on-records-cover) |

### Queued and not yet written

The admin found these and queued them for its next specification order; none is in a log or an
order yet, though 082's questions are in the tracker's [known-open list](../AUDIT.md#known-to-be-open).
Verify each, then fix it or fold it into your findings.

- PRV-021's owed-stop clause has no split: a start whose own instance owes a stop not carried
  out, or whose own state cannot be read, should end unknown, naming the stops; another
  instance's owed stop, failed, naming it (PRV-019; the kit's order 141).
- PRV-015 names a machine by its ID, and `place.v1`'s output gives a hostname.
- The "labels last reported" wording of PRV-027, LIF-036 and `stop.v1`.
- One live node per machine, acceptances decided one at a time (049a1's decision; ENR-002).
- Order 082's questions: state that cannot be stored; races across addresses.
- Plan and apply shows a changed control value and calls no control (OPS-020;
  [the reference](../implementation/plan-and-apply/README.md#replacement-removal-and-cascades)); S4's T1
  repair needs it.
- The first-start report maps some Rust architectures to the wrong Go `GOARCH` names (PRV-014).

## Focus 2: the approach

- **Work orders and lanes** (rulings 48, 71). From the Records, count rounds, verifier musts,
  admin fixes and times; A77's done-when is unmet, since admin time was not recorded. Do rounds
  converge, or do musts keep coming (049a1, round 5) because orders are too large or the text is
  ambiguous? Do lanes pay for their joins and conflicts (a squash once reverted a merged order,
  restored in pull request 190)?
- **S1's bar** (ruling 71, point 3; ruling 63's counting; review item 9). Both increment 4 authors
  failed it (the coordinator's run wrote 332 lines against the baseline's 64). The bar holds
  findings 4-01 and 4-11, batch 2's merge and the small findings that could move S1's counts. Does
  author lines against [A70](../../experiments/007-s1-baseline/README.md) measure what the
  purpose claims, when the baseline has no inspectable, revocable relationship? Should the rerun
  also report D22's measures and the preservation tests no baseline meets?
- **Plan and apply** (ruling 76). Tooling made of existing requests; besides the control gap,
  [its open points](../implementation/plan-and-apply/README.md#open-points) O1 to O8, with O7
  (no source for package contents) the largest. Is `kubectl apply`'s update (4-01's alternative)
  the better paradigm?
- **Providers as built** (ruling 40). Does each sit behind its contract with no special case in
  the engine? The runtime providers build only inside this repository, through the shared kit;
  do built-ins and packaged locals duplicate work; is SSH-R's drainer of owed stops what PRV-021
  needs, or infrastructure an existing tool supplies?
- **The second machine** (ruling 77): an SSH provider in place of Tailscale, one run user with
  PRV-025's isolation stated unmet. Is a WSL distro on the owner's own computer a fair second
  machine for S3's claims?
- **Dogfooding** (A78, ruling 70). 042 ran through two `codex.v2` Agents of `conduit-core`, with
  turns of hours. Frictions not yet findings in increment 4's log: a short network loss failed a turn as unknown ("session
  cannot restart implicitly") where a plain Codex reconnected; Claude Code wrote in its slot's
  `.git` unprompted (ruling 75's later answers); Codex's terminal children outlived the run's
  process-group stop in the smoke test. What do they say of convention 7 against resilience, and
  of unconfined Agents under the engine's OS user (ruling 75, B3)? Log each as a first-build
  finding.

## Focus 3: experiments to alter

- **S2 to S10** ([scenarios](../../ROADMAP.md#scenarios); each `README.md` and `plan.md` under
  `experiments/013` to `021`), accepted under ruling 75, D. Does each pass condition test a claim
  or only exercise code? Is each measure defined before the run (ruling 45)? Does a simulated
  client stand where a real one is needed (ruling 75's later answers on D4)?
- **Sign-in and isolation**: A81, A82 and orders 050, 048a, 048b and 051b
  ([staged parts](../work-orders/038c-staged/README.md)). Design the test the second accounts are
  kept for: two Agents in separate homes, one on each subscription; what one can reach of the
  other's login under one OS user (PRV-029, DL16); a wrong account undone (PND-005, PND-011).
  Should A5 become this test?
- **Model checking**: rerun MC2 on 077's text; re-register MC4 against 081's PRV-019, PRV-021 and
  PRV-030 and the kit's owed-stop rule; MC9 after batch 2; the 53 static rows to vectors.
- **The fastest falsifier.** Name the one scenario, and the result, that would most quickly show
  the layered claim false. Candidates: S10, where any document edit needed to swap a provider
  fails swappability; resolution from `spec/` alone by a fresh builder (A34, narrowed); S3 under
  a partition, where a convention needs step-by-step behavior to hold; a provider contract that
  needs a special case in the engine. Choose, with reasons, before anything runs.

## What to produce

1. **A dated record**, `docs/proposals/full-review/README.md`, opening "Not binding", with the
   date, the commit, the scope, and what was and was not checked. Findings in a table: an ID
   (`FR-NNN`), class, unsafe or not, promises, `file:line`, quote, situation, counterargument,
   least change, and who decides under which ruling.
2. **The coordinator's fixes**, each logged to strike and ordered as work orders.
3. **Questions for the owner**, as few as possible. Each in plain words, with lettered options, a
   recommendation and its reason, what it changes (`spec/`, IN_FORCE.md, the roadmap) and what
   waits on it.
4. **Changes to experiments**, each naming its row in [the register](../../ROADMAP.md#experiments-and-probes),
   the claim it tests, and its pass condition and measures, fixed before it runs
   ([proposing an experiment](../../ROADMAP.md#proposing-an-experiment)).

Hand the owner a short message that numbers the questions. The answers become one ruling: its
record under `docs/proposals/ruling-NN/`, a ledger row with the owner's words, the lines of
IN_FORCE.md it changes and any action item. An answer that changes no decision is added, dated,
to the existing record.

## Traps found this session

- **Plans are not binding.** The roadmap, PROVIDERS.md, the scenario plans and the plan-and-apply
  reference ("No program has produced these expected plans") say what was meant.
- **Records describe history.** G5.md calls 081 and 082 not yet run; both merged since. A ruling
  is not evidence that its text or code exists.
- **Counts go stale.** The safety catalogue was written from 502 promises, and `spec/` now holds
  519; G5's standings predate 081 and 082. Recount.
- **Private details never enter the repository**: no local paths, user names, addresses, tailnet
  names, IP addresses, fingerprints or account details; accounts by label only (primary, second,
  Linear). `tools/check_private_paths.py` checks paths. Never print the environment.
- **A verifier on Windows misses macOS failures.** SSH-R passed there and failed on the Mac
  (`chmod --`, `find -printf`). Run the suites on the Mac before merging.
- **Model checking describes models.** "Holds" is a bounded projection under an applied reading;
  no claim spans all faults, real cryptography, persistence or the engine's conformance.
- **Worktrees share remote refs.** Fetch, rebase and check ancestry in one command, then squash.
