# Work order 192: the register's rows for model checking, mutation testing and the native probes

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It changes no code, so run no cargo command. Never print a whole large file or log: read
`ROADMAP.md`, `docs/AUDIT.md` and the findings in parts, by line range or `grep`. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 39 of [its orders](../proposals/full-review/orders.md))
found the register's rows for model checking, mutation testing and the native probes stale, and
some of them wrong. Experiment 012's row is done for the harness alone, though its models ran once
on text that later specification orders, 077, 081 and 082 among them, have changed since
([FR-450](../proposals/full-review/findings/G1a-model-checking-register.md#fr-450)). A40's status
counts a set of rows the account no longer has
([FR-451](../proposals/full-review/findings/G1a-model-checking-register.md#fr-451)). A44's
done-when, "No unexplained surviving mutant", says neither what explains a mutant nor which tests
may catch one ([FR-452](../proposals/full-review/findings/G1a-model-checking-register.md#fr-452)).
Several of the native probes' rows state things that are stale or wrong
([FR-489](../proposals/full-review/findings/G3-native-probes.md#fr-489)). A6 waits for engine runs
for two traces that already exist
([FR-485](../proposals/full-review/findings/G3-native-probes.md#fr-485)). A7 credits to A70's T3 a
sentence that comes from ruling 48, and asks a question that rulings 70, 75 and 77 have answered in
part ([FR-481](../proposals/full-review/findings/G3-native-probes.md#fr-481)). A10's method cannot
run, since both Agent Types keep the client's hooks off, and what it asked is already recorded
([FR-484](../proposals/full-review/findings/G3-native-probes.md#fr-484)). A22 does not test the
session that ruling 75, B3 puts the engine in
([FR-486](../proposals/full-review/findings/G3-native-probes.md#fr-486)). A73 still waits for
providers that have started
([FR-487](../proposals/full-review/findings/G3-native-probes.md#fr-487)). A82 says its return is
carried where, all on the Mac, the package's own listener takes it, and leaves out the placement
that ruling 75, C1 asks
([FR-430](../proposals/full-review/findings/E04-signin-isolation.md#fr-430)).

This order rewrites those rows as the findings give them, with their verifiers' corrections, which
govern. It applies [ruling 78](../proposals/ruling-78/README.md), points 1, 3.2 and 3.3, the
coordinator's D29, D31, D33, D34, D39, D60, D61, D66, D67 and D79
([decisions](../proposals/full-review/decisions.md)), and
[the review's changes to the experiments](../proposals/full-review/experiments.md). Since the
review, [order 153](153-model-account-checked.md) checks the model-checking account with
`tools/check_model_account.py`, and [order 160](160-mc2-rerun-077.md) ran MC2 again.
[Order 183](183-v0-security-claim-drafted.md) drafted v0's security claim in
`engine/CONFORMANCE.md`, corrected the readiness assessment's G8 cell and closed A20 on the claim.
[Order 190](190-desk-checks-terms-1password.md) set A73's State and wrote its stage 1 note, and
left the rest of A73's row to this order. So FR-489's points 2, 3 and 4 are done, and this order
changes none of that.

Each changed row states, before anything runs, what it must show and how it is measured, as the
roadmap's [rules for proposing an experiment](../../ROADMAP.md#proposing-an-experiment) ask
(ruling 45). This order runs nothing, makes no directory, and adds no row and no A-number. It
decides nothing that is the owner's: the owner's approval, sudo and leave stay conditions in the
rows. No promise, case, vector or code changes, so `CONTRIBUTING.md` rule 8 (order 163) asks for
no case, and no declared file is needed (order 174). Line numbers are those of `main` at
`0ed26306`. If `main` has moved when you start, find each row by its first cell and each line by
its words, and report the lines you found.

## The decisions it applies

1. **Form.** Only the cells this order names change. Every other cell, row and line of the three
   files stays byte for byte, and each table row stays on one line. Each quoted text below is the
   cell's exact new text: join its lines with single spaces. In the quotes, links are written
   relative to this order's file, so that they resolve here. Write each so that it names the same
   file from the file it goes in: in `ROADMAP.md`, a link that begins with two parent steps loses
   them, and one that begins with a single parent step into proposals begins with docs instead;
   in `docs/AUDIT.md`, two parent steps become one, and one parent step into proposals becomes
   none. `check_docs.py` checks the result. Keep the straight apostrophes and quotation marks.
2. **Experiment 012's row** (`ROADMAP.md` line 76; FR-450 as its verifier corrected it; D29, D31,
   D33 and D34; ruling 78, point 3.2). The Item and Needs cells stay.
   - Question it settles:
     > The harness for A40 and the models built on it: do the rules that the convention-layer
     > safety promises state compose, in small configurations, under loss, reordering,
     > duplication, a cut and its repair, a crash and a lost machine? A claim about the text, not
     > about an engine or a provider. Each model is registered before it runs, naming the commit
     > of `spec/`, or the ruled reading, that it reads
   - Feeds:
     > A40 and G5; each specification order that changes a modelled promise, which names in its
     > done-when the model order that reruns it, or marks the row "text changed, not rerun"
     > (D29); S3's added ordering cells, after the MC2 rerun (D66), and S4's E18, whose fixture
     > waits for MC6's answer (D67)
   - Done when:
     > Each model's registered verdict table is reproduced by the ordinary test run, every search
     > inside its state ceiling and deadline: a truncated search gives no verdict. Each safety
     > property holds in every registered configuration and has a mutant that refutes it, and
     > every enabled fault and every safety-relevant outcome has a witness; a collateral
     > refutation is registered before the run or explained beside the table, and a refutation on
     > a correct model is classed, as a finding against the promises or an over-broad property,
     > before it goes to the coordinator (D33). "Holds" is kept for a property that relates
     > different steps and can fail (D31). `python3 tools/check_model_account.py` passes. Every
     > finding against the promises ends in a regression vector or a stated limit (D34).
     > Measures, per model and configuration: each property's verdict against the registered
     > table, the first trace's length in actions for each refutation, unique states, depth and
     > seconds; per promise row, the property that carries it, the commit or reading it ran
     > against and whether a mutant targets the row's own clause; per finding, its regression
     > vector or limit; and the count of rows at "text changed, not rerun". What A40 needs of the
     > runs is [ruling 78](../proposals/ruling-78/README.md), point 3.2's
   - State. Run `python3 tools/check_model_account.py` on the tree you start from. On `main` at
     `0ed26306` its first line gives 245 rows, and its standings list `text changed, not rerun:
     28`. If it prints other counts, write those and the commit you started from in place of
     these, and report them.
     > in progress: the harness and its own model ran on 2026-11-11
     > ([MC1's dated results](../../experiments/012-model-checking/README.md#mc1-dated-results-2026-11-11),
     > work order 104, in the proof lane of ruling 71), and the models MC2 to MC7b the same day
     > (orders 105 and 107 to 115), before later specification orders, 077, 081 and 082 among
     > them, changed text they read. MC2 ran again on ruling 78's reading on 2026-11-15
     > ([its dated results](../../experiments/012-model-checking/README.md#mc2-rerun-dated-results-2026-11-15),
     > work order 160). [G5's account](../../experiments/012-model-checking/G5.md) (order 116) is
     > checked by `tools/check_model_account.py` since order 153, and order 181 classed the
     > promises the catalogue lacked: on `main` at `0ed26306` the account holds 245 rows, 28 of
     > them "text changed, not rerun". Next, each registered before it runs: MC2 again with the
     > specification order that writes ruling 78, point 6, and steps 3 to 7 of
     > [the full review's list](../proposals/full-review/experiments.md#experiment-012-the-runs-each-registered-before-it-runs),
     > MC9a first, when order 048a-2 is drafted
   - Source:
     > Rulings 45, 71 and 75 E1, as ruling 78, point 3.2 amends it; A40; the full review's
     > [FR-450](../proposals/full-review/findings/G1a-model-checking-register.md#fr-450)
3. **Two rows of the order of the work.**
   - Step 6's State (line 31; FR-451's line for it, and FR-452):
     > now: A40's harness and models,
     > [experiment 012](../../experiments/012-model-checking/README.md), ran on 2026-11-11, and
     > are run again in the order that its row in the register gives; MC2 ran again on
     > 2026-11-15. A44's first pass runs on the build that the S1 rerun pins
     > ([FR-452](../proposals/full-review/findings/G1a-model-checking-register.md#fr-452))
   - The In parallel row's Needs (line 26): replace "except A5 and A6, whose rows say what they
     now need" with "except A5 and A7, whose rows say what they need". After decision 5, A6 needs
     no engine, and A7's attempts run Agents through the engine.
4. **A40 and A44 in the tracker** (`docs/AUDIT.md` lines 477 and 476; FR-451, its Status only,
   and FR-452, each as its verifier corrected it; D39). A40's Action, Source and Done when stay:
   ruling 78, point 3.2 wrote its done-when.
   - A40's Status:
     > in progress: [experiment 012](../../experiments/012-model-checking/README.md)'s harness
     > (order 104) and models MC2 to MC7b (orders 105 and 107 to 115) ran on 2026-11-11, before
     > later specification orders, 077, 081 and 082 among them, changed text they read; MC2 ran
     > again on ruling 78's reading on 2026-11-15 (order 160). Its
     > [G5 account](../../experiments/012-model-checking/G5.md) (order 116) gives each row its
     > standing, and `tools/check_model_account.py` (order 153) fails while a row whose promise
     > changed after its model ran still claims that run; such a row reads "text changed, not
     > rerun". The reruns are ordered in experiment 012's row of the
     > [register](../../ROADMAP.md#experiments-and-probes). The tool was evaluated in experiment
     > 004
   - A44's Action:
     > Mutation-test the core against the conformance vectors, by promise: the mutants that
     > cargo-mutants generates, scoped by file, function or diff, each attributed to the promise
     > IDs that its enclosing function cites, and hand-written mutants for the convention-layer
     > safety promises that the core cites. Only a conformance vector counts as catching a mutant;
     > catches by the workspace's other tests are reported apart.
   - A44's Source:
     > G6; [resolver probes](../../conformance/README.md#evidence-of-vector-strength-so-far),
     > [regression mutations](../../conformance/README.md#evidence-of-vector-strength-so-far),
     > [record-omission probes](../../conformance/README.md#evidence-of-vector-strength-so-far);
     > the full review's
     > [FR-452](../proposals/full-review/findings/G1a-model-checking-register.md#fr-452) and D39
   - A44's Done when:
     > No unexplained surviving mutant: each survivor is in one of three classes, fixed before the
     > first pass: no difference observable at the conformance boundary; an engine choice that
     > `engine/CONFORMANCE.md` or a DL item lists; or a promise the vectors do not pin, with its
     > case written or the order that writes it named. Every convention-layer safety promise that
     > the core cites has a mutant that a case naming the promise catches.
   - A44's Status:
     > open: the first pass runs on the build that the S1 rerun pins, scoped by file or diff under
     > a machine-time ceiling registered before it runs, with its dated results in
     > `experiments/031-a44-mutation/` and the measures of
     > [the full review's A44 row](../proposals/full-review/experiments.md#register-rows); from
     > the pin, each engine order's Record lists the mutants in its own diff, each caught or
     > classed (D39). A44 is not on v0's list and delays nothing (ruling 47). The evidence so far
     > is increment 2's:
     > [mutation results and limits](../../conformance/README.md#evidence-of-vector-strength-so-far).

   The Done when keeps the words "No unexplained surviving mutant", which G6 of the readiness gate
   (`docs/implementation/INDEPENDENT_BUILDS.md` line 117) also uses, and defines them, so G6's row
   does not change. Naming cargo-mutants is the coordinator's choice. It is the existing mutation
   tool for Rust, and FR-452's verifier gives its mutants, each attributed by the citations in its
   enclosing function, as cheaper than hand-made mutants at each of about 728 citation sites. The
   directory number 031 is the review's assignment to A44; its name is the coordinator's.
5. **A6** (`ROADMAP.md` line 63; FR-485 as its verifier corrected it; D67; ruling 78, point 1).
   Every cell but the Item changes.
   - Question it settles:
     > A lost response to a real client: after a response is lost, does an unchanged client send
     > the same `tools/call` again without a new step of its model, and what does the model do
     > next? The claim: a native client does not repeat a call on its own; a second call is its
     > model's and, carrying no key, is new work (OPS-011), so an effect behind Conduit is applied
     > twice only when the model asks twice
   - Needs:
     > No engine: a new, minimal stub in Rust of the presented server, as experiment 008's was,
     > whose harness left the tree with order 066. Two cases: L1, the stub applies the effect and
     > closes the connection with no response; L2, it never answers, until the client gives up or
     > 300 seconds pass. The primary accounts, headless, as the allowance allows, with the owner
     > told before the runs
   - Feeds:
     > D02 and E09 of round 6, in S4: the client axis of its unkeyed cells; and whether S4's one
     > lost-response replay on a second account, which ruling 75, D4 also names as A6's
     > real-client trace, adds more than the engine's records. That replay stays as ruled unless
     > the owner says otherwise; see A71
   - Done when:
     > Twelve runs, two clients by two cases by three repetitions, with dated results in
     > [experiment 008](../../experiments/008-real-client/README.md), pre-registered there before
     > they run. Pass: in all twelve, no `tools/call` is sent again without a new step of the
     > model; a resend is a finding against the Agent package and adds a native-client row to S4's
     > unkeyed cells. Measures: `tools/call` requests per call the model issued; stream
     > resumptions (a GET with Last-Event-ID) and session re-initializations, counted apart;
     > whether a request's `_meta` carries an idempotency key; seconds until the client gives up;
     > the model's next action in experiment 008's classes; effects in the stub's ledger. If no
     > client resends, the owner is told that S4's replay adds only the engine's records
   - State:
     > later: after the S1 rerun's pin ([ruling 78](../proposals/ruling-78/README.md), point 1).
     > Of the four traces its first wording asked for, discovery and invocation exist (A71; the
     > engine served two Agents with different lists in
     > [increment 3's closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#results-of-the-third-run));
     > refresh, as the shared key's rotation, stays S2's own case, and the lost response through
     > the engine S4's. A6 closes on the twelve runs (D67 of
     > [the full review's decisions](../proposals/full-review/decisions.md))
   - Source:
     > Probe E3 of the plan audit; the full review's
     > [FR-485](../proposals/full-review/findings/G3-native-probes.md#fr-485) and D67
6. **A7** (line 64; FR-481 as its verifier corrected it; FR-430's point on A7; FR-489, points 1
   and 7; D60; ruling 78, point 3.3). Every cell but the Item changes. The old Question's
   sentence that Claude Code's messaging and scheduling tools need no permission, credited to
   A70's T3, goes with it: it came from ruling 48, point 1, and experiment 009 records message
   refusal as untested (FR-489, point 1). The claim's statement 2 joins 3, 5, 6 and 7 because
   FR-481's verifier added attempt 11, a Pool's credential. Attempt 9 waits for part 2 of ruling
   77's setup (its lockdown), as FR-481 and the review's A7 section say; the run user already
   exists. The directory number 030 is the review's assignment to A7; its name is the
   coordinator's.
   - Question it settles:
     > Routes around the engine, and v0's security claim: which routes around the engine exist for
     > an Agent with its client's own tools off, for an Agent whose template names a directory,
     > for a plain process of the run's operating-system user, and on a second machine? Is each
     > one a statement of
     > [v0's drafted security claim](../../engine/CONFORMANCE.md#v0s-security-claim-drafted), and,
     > where the language reports reach, does inspection show it? It tests the claim's statements
     > 2, 3, 5, 6 and 7 through the attempts that
     > [the full review's A7 section](../proposals/full-review/experiments.md#a7-the-attempts)
     > numbers 1 to 11
   - Needs:
     > No engine change. The engine on `main` with the `.v1` Agent packages for attempts 1 to 5
     > and 7, and the dogfood engine with the `.v2` packages of orders 050a and 076 for attempts 8
     > and 8b. The primary accounts; canaries and file metadata only, never a real credential;
     > clients started under `env -i`. The owner's leave, at a time when no live session reads the
     > owner's settings, for attempt 5's user-level sentinels. Attempt 6 waits for order 050,
     > attempt 9 for part 2 of ruling 77's setup on the Windows computer, its lockdown, attempt 10
     > for a confining runtime provider, and attempt 11 for a secret provider and S2's measure
     > D20. A5's reach matrix supplies the same-user rows for logins: reading, using or replacing
     > another Agent's login, and acting as root's Interface
   - Feeds:
     > v0's security claim (rulings 47 and 78, point 3.3): each route found becomes one of its
     > statements; A5's reach matrix; measure D20 of S2 and S3; the Agent packages' release checks
   - Done when:
     > Dated results in `experiments/030-a7-routes/`, pre-registered there before each attempt
     > runs, and the claim's sections "Routes around the engine found so far" and "Not shown yet"
     > agree with them. With the tools off, an attempt passes when no canary outside the Agent's
     > Conduits is reached and a baseline outside the engine shows that the canary can be
     > reached; an attempt without a working baseline is "not shown", never a pass, and a route
     > found is a finding of the first build against the Agent package and a new statement of the
     > claim. With the tools on, and for a plain process of the same user, each attempt is
     > expected to succeed, and passes when the claim already states it; with the tools on,
     > inspection must also show `ownTools` with `directoryNamed: true` and `enforcement:
     > neither` (LIF-013). Measures, per regime, client and attempt: attempted, reached, refused
     > and by whom (the client, the engine, the server or the operating system), or not shown;
     > the client's version; whether an engine record exists for each reach, an absent record
     > never showing absent reach. Every result opens with what one operating-system user cannot
     > show (D60). The attempts with the tools off then become the Agent packages' release checks,
     > run again at each new client version, and A7 closes once the claim is published
   - State:
     > open: its method ran with the tools off, for files, in
     > [increment 3's closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#routes-around-the-engine-acc-027),
     > and the dogfood's smoke test saw two routes with the tools on, rows 4-22 and 4-23 of
     > [increment 4's log](../proposals/first-build/log-increment-4.md), which the claim states.
     > The other attempts run as observation ([ruling 78](../proposals/ruling-78/README.md), point
     > 3.3) and are not yet scheduled. A20, which waited for it, closed on the drafted claim
   - Source:
     > Probe E4 and disagreement D5 of the plan audit; ruling 47; the full review's
     > [FR-481](../proposals/full-review/findings/G3-native-probes.md#fr-481)
7. **A10 closes** (line 65; D79; FR-484 and FR-483, as their verifiers corrected them; FR-489,
   point 5). The Question, Needs and Done when stay. Its State must link dated results, as
   `tools/check_roadmap.py` asks of a done row (order 180).
   - Feeds:
     > Automatic wake, which rulings 15 and 17 hold out of v0 and the ledger lists as later
     > scope; nothing observed asked for it
     > ([FR-483](../proposals/full-review/findings/G3-native-probes.md#fr-483), as its verifier
     > corrected it)
   - State:
     > done on 2026-11-14, by D79 of
     > [the full review's decisions](../proposals/full-review/decisions.md), on two recorded
     > exchanges between two Agents, each turn asked for explicitly through root's Interface: T2
     > of
     > [increment 3's closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#results-of-the-third-run)
     > on 2026-11-10, and order 042's nine turns on 2026-11-11 and 12, each with root as caller
     > ([dated results](../../experiments/028-a78-dogfood/results-2026-11-12.md)). Its notice
     > through hooks could not run as worded: both Agent Types keep the client's hooks off. The
     > native routes to wake, Claude Code's inbound cross-session messages and a background job
     > that outlives its turn, are A7's attempts 2 and 8b
   - Source:
     > Probe E5 of the plan audit; the full review's
     > [FR-484](../proposals/full-review/findings/G3-native-probes.md#fr-484) and D79
8. **A22** (line 67; FR-486 as its verifier corrected it; D61). Every cell but the Item changes.
   - Question it settles:
     > The keychain with no person present, in the sessions the engine will run in: can a
     > keychain item be written and read unattended (a) by a LaunchAgent in the owner's graphical
     > session, which is optional; (b) by a process of the dedicated macOS user of ruling 75, B3,
     > with no graphical login, started by launchd; and (c) by that user over SSH? For that user,
     > where does a Claude Code login land, a keychain item or a file, and does the client report
     > it signed in at its next start with no person present? Yes or no only, never the credential
   - Needs:
     > No engine and no second account: experiment 005's probe, which is in the tree, and a
     > launchd property list. The owner's approval under 005's safety rules; the owner's sudo for
     > (b) and (c); the dedicated user's own keychain, holding nothing else, and never the owner's
     > login keychain; one sign-in that the owner makes, for the Claude login case
   - Feeds:
     > The keychain store of A63 and the wording of its contract; whether ruling 75, B3 works for
     > Claude Agents as ruled; the custody of credentials
   - Done when:
     > Dated results in [experiment 005](../../experiments/005-macos-keychain/README.md) for each
     > session that runs, pre-registered there before it runs, and cited by A63's row. Cases per
     > session: 005's trusted read, self-created item and locked read, on a throwaway keychain
     > with interaction disabled; the state of the session's default keychain; a rebuild from
     > unchanged source, and one with a stable signing identity; for (b) and (c), the Claude login
     > case. Measures, per session and case: read, refused with its code, or waited; milliseconds;
     > the launchd manager's name; the session's attributes. The reading is fixed before: if (b)
     > cannot read an unlocked keychain without the user's password, the keychain store's contract
     > says "graphical login session only" and that user's secrets use the file store; if a Claude
     > login cannot be kept and read unattended for that user, the owner hears before batch 2 that
     > B3 as ruled covers Codex slots only, or needs that user logged in
   - State:
     > in progress: [Experiment 005](../../experiments/005-macos-keychain/README.md) recorded
     > results on 2026-11-06, in the user's logged-in session. Sessions (b) and (c) run when the
     > dedicated user is created, before any Claude Agent becomes a dogfood slot and before batch
     > 2 (D61 of [the full review's decisions](../proposals/full-review/decisions.md), which
     > proposes creating that user at order 050's sitting; the owner decides). Session (a) is
     > optional
   - Source:
     > Probe E10 of the plan audit; the full review's
     > [FR-486](../proposals/full-review/findings/G3-native-probes.md#fr-486) and D61
9. **A73** (line 68; FR-487). Order 190 set its State, which stays.
   - Question it settles:
     > 1Password with no person present: can an item be read with no prompt answered, by a
     > service account and through the desktop app's session? Stage 1 answers from 1Password's
     > published documentation; stage 2, a run on a throwaway account and vault, runs only if a
     > 1Password store is ordered
   - Needs:
     > Stage 1: nothing. Stage 2: an order for a 1Password store;
     > [Experiment 005's safety rules](../../experiments/005-macos-keychain/README.md#safety); a
     > throwaway account and vault and the owner's approval; the service account's token kept in
     > another store, which an unconfined Agent of the engine's operating-system user can read
     > (ruling 75, C1), passed by file and never printed; and the owner's approval before the
     > desktop app's case, which runs only if stage 1 showed that a read can succeed while the
     > app is locked
   - Feeds:
     > The 1Password store of A63, and through it S10's second secret store, under the owner's
     > answer Q2: "1Password after A73, or else the file store"
   - Done when:
     > Stage 1's dated note; stage 2's dated results if it runs, pre-registered in
     > [experiment 025](../../experiments/025-1password/README.md) with each measure defined
     > before it runs: reads with no prompt out of n, latency, and the error for a revoked token,
     > a missing item and no network. A desktop case that does not run is recorded from the
     > documentation as a store that needs a person present
   - Source:
     > Ruling 45 (X1 of pull request 10, from `allocmemory`), which put it under A63; the full
     > review's [FR-487](../proposals/full-review/findings/G3-native-probes.md#fr-487)
10. **A82** (line 74; FR-430 as its verifier corrected it; D60). Its Feeds and State stay. D60
    applies because A82 is a sign-in test run under one operating-system user.
    - Question it settles:
      > Does a Resource's browser login, Linear's OAuth, complete through Conduit's Interface by
      > both routes that PND-013 allows, with the token kept by the provider package and the
      > account checked? Leg (a): `conduit`, the engine and the package all on the Mac, the
      > redirect taken by the package's own listener. Leg (b): `conduit` run on the Mac over SSH
      > from the Windows computer, the browser there and the final URL pasted, so that the return
      > is carried to the Interface that received the sign-in (ruling 75, C2 and C4)
    - Needs:
      > The primary Linear account and its test workspace; a Linear OAuth application the owner
      > registers (S2-LOG-O); the provider package S2-LOG-P; orders 048a, 048b and 051b; for leg
      > (b), an SSH session from the Windows computer to the Mac. During the run, no unconfined
      > Agent and no plain work-order worker runs under the package's operating-system user
      > (ruling 75, C1), or the result states that placement as unmet: on the Mac it stays unmet
      > until a confining runtime exists, and the dedicated macOS user of ruling 75, B3 does not
      > change it
    - Done when:
      > A dated trace of one login per leg, in
      > [experiment 013](../../experiments/013-s2-shared-login/README.md), the account checked
      > with one read, no code or token recorded (S2-LOG); the result opens with what one
      > operating-system user cannot show (D60)
    - Source:
      > Ruling 72; the full review's
      > [FR-430](../proposals/full-review/findings/E04-signin-isolation.md#fr-430)
11. **The undefined label in the providers' plan.** In `docs/implementation/PROVIDERS.md`, item 7
    of lane SP's queue (line 62), delete "; MX0 at the admin's word", so that the item ends "SV1
    after 071." The label appears nowhere else in the tree outside the review's record, and the
    commit that wrote it defines it nowhere
    ([FR-449](../proposals/full-review/findings/G1a-model-checking-register.md#fr-449); "Define
    or drop" in the review's experiment 012 section). The coordinator drops it: the work the
    review gives model checking and mutation testing is in experiment 012's row and A44's.
    Nothing else in the file changes.
12. **What does not change.**
    - No row is added and no A-number is given. FR-430's A83, one Linux user per Agent, is not
      added (D50), and A83 is the blind-verdict test (D77).
    - No directory is made. 030 and 031 are named only, as the roadmap already counts them.
    - FR-489, point 8: ACC-027's S1 reason in `conformance/manifest.yaml` (line 2007) stays.
      Ruling 78, point 14 has a later order change ACC-027's observable with one vector, which
      replaces that reason; this order changes nothing the conformance runner reads.
    - S2's plan keeps its S2-LOG row (`experiments/013-s2-shared-login/plan.md` line 96), which
      names one route, and its table "Logins through the Interface" (line 229). Report that the
      row needs decision 10's two legs and its placement line, and that FR-430 replaces the
      table's first-use column with A5, for S2's dated amendments (D64).
    - The experiment program's method row "Mutation testing of the model against the vectors"
      (line 130) stays: "the model" is an older word for the core, and AGENTS.md asks that older
      words not be renamed in bulk.
    - `IN_FORCE.md`, `docs/implementation/HANDOVER.md`, `docs/implementation/INDEPENDENT_BUILDS.md`,
      `engine/CONFORMANCE.md`, and everything under `spec/`, `conformance/`, `experiments/`,
      `engine/` and `packages/` stay as they are.

## Scope

Change only `ROADMAP.md` (the nine rows of decisions 2, 3 and 5 to 10), `docs/AUDIT.md` (the rows
of A40 and A44, decision 4) and `docs/implementation/PROVIDERS.md` (the line of decision 11).

## Done when

From the repository root, one at a time, each passes:

```sh
python3 tools/check_roadmap.py        # Errors: 0
python3 tools/check_docs.py           # Errors: 0
python3 tools/check_model_account.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 tools/vector_delta.py HEAD    # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.
git diff --check
git diff --numstat                    # exactly three lines: 9 9 ROADMAP.md; 2 2 docs/AUDIT.md; 1 1 docs/implementation/PROVIDERS.md
git grep -n -w MX0 -- . ':!docs/proposals/full-review' ':!docs/work-orders/192-*'   # prints nothing
git grep -n 'scheduling tools need no permission' -- ROADMAP.md   # prints nothing
grep '^| A6 |' ROADMAP.md | grep -c '| later: after the S1 rerun'                # 1
grep '^| A7 |' ROADMAP.md | grep -c 'experiments/030-a7-routes/'                # 1
grep '^| A10 |' ROADMAP.md | grep -c '| done on 2026-11-14'                      # 1
grep '^| A22 |' ROADMAP.md | grep -c 'dedicated macOS user'                      # 1
grep '^| A73 |' ROADMAP.md | grep -c 'The first providers started'              # 0
grep '^| A82 |' ROADMAP.md | grep -c 'own listener'                              # 1
grep '^| \[Experiment 012\]' ROADMAP.md | grep -c '| in progress: the harness'   # 1
grep -c -e 'mutation testing (A44) later' -e 'except A5 and A6' ROADMAP.md      # 0
grep '^| A40 |' docs/AUDIT.md | grep -c 'order 160'                              # 1
grep '^| A44 |' docs/AUDIT.md | grep -c 'experiments/031-a44-mutation/'         # 1
```

On `main` at `0ed26306`, `check_roadmap.py`'s first line ends "51 not finished", and its third
line is "Done rows in the register: 8, 8 of them linking dated results.". After your change, the
first line must end "50 not finished", since A10 is done, and the third line must be unchanged,
since A10 becomes done and experiment 012 in progress. If the tree you start from prints other
counts before your change, report them: your change must lower "not finished" by exactly one and
leave the done rows' count as it was. Where `python3` lacks PyYAML, run `vector_delta.py` with
`/usr/bin/python3`. The review's row names `check_roadmap.py`, the docs checks and the two
`git grep` commands; the `git grep` for MX0 leaves out the review's record and this order, which
name the label.

The checks must also be able to fail. Run these against `main`'s files; each prints the value
shown, the opposite of the checks above:

```sh
git show HEAD:ROADMAP.md | grep '^| A10 |' | grep -c '| done on 2026-11-14'                  # 0
git show HEAD:ROADMAP.md | grep '^| A7 |' | grep -c 'experiments/030-a7-routes/'             # 0
git show HEAD:ROADMAP.md | grep '^| \[Experiment 012\]' | grep -c '| in progress: the harness'   # 0
git show HEAD:ROADMAP.md | grep -c 'scheduling tools need no permission'                     # 1
git show HEAD:docs/AUDIT.md | grep '^| A44 |' | grep -c 'experiments/031-a44-mutation/'     # 0
git show HEAD:docs/implementation/PROVIDERS.md | grep -c -w MX0                              # 1
```

Do not stage anything. Report the counts that `check_model_account.py` printed and the counts you
wrote, the lines where you found each row if `main` had moved, S2's plan's S2-LOG row and table as
decision 12 asks, each choice you made yourself, and every check's last line. A fresh Codex
verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).
