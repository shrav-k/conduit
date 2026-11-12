# Work orders: the admin session and Codex

Not binding. The specification in [spec/](../../spec/README.md) alone binds. This page says how
the coordinator's session and Codex sessions divide the work on Conduit, as
[ruling 48](../proposals/ruling-48/README.md) asks, what the first rounds taught, and the record
of merged orders. Who decides and who merges, lanes included (ruling 71), is in
[IN_FORCE.md, section 15](../../IN_FORCE.md#15-how-the-work-is-governed).

## Roles

| Who | Does | Does not |
| --- | --- | --- |
| The owner | Rules on behavior, on what Conduit is for and on the order of the work. Can strike any decision of the coordinator | |
| The admin session, the coordinator (Claude Code) | Plans the work and writes each order: what it applies, its scope, its done-when, its report. Makes the decisions an order applies, under rulings 37, 38 and 46, point 4, each logged and strikeable. Reviews what comes back, runs the checks, commits, and opens and merges the pull request. Puts to the owner what is the owner's | Spend its own session on work that an order can state |
| A Codex worker | Carries out one order in a working tree of its own: a build increment, an experiment, a sweep of the promises, a verification of another order's result, or a fresh read | Commit, push or open a pull request. Decide what the order leaves to the admin: it reports instead |

Codex gets work whose result the admin can check: a done-when that the checks show, counts that
cite their evidence, or findings that cite the text. The admin keeps whatever chooses: a decision
on a finding, a measure or its counting rule, what a result means, and what goes to the owner.

## An order

An order is a file `docs/work-orders/NNN-name.md`, written before it runs, as ruling 45 asks of
an experiment. It holds:

1. **What and why**: the goal, in two sentences, and the records it comes from.
2. **The decisions it applies**, each with its source. The worker does not reopen them. Where one
   cannot be applied as written, it applies the nearest sound reading and says so in its report.
3. **What to read**: the files and the promises, by path and ID.
4. **Scope**: the files it may change. Anything else it needs changed goes in its report.
5. **Done when**: the checks that must pass, each a command.
6. **The report**: a JSON Schema, given to Codex with `--output-schema`, so that what comes back
   is a value the admin reads, not prose. It says at least what changed, what could not be done,
   which choices the worker made itself, and each check's last line. Most orders share
   [one schema](../work-orders/README.md#shared-and-staged-files).
7. **Rules**: those of [AGENTS.md](../../AGENTS.md#if-you-were-given-a-work-order), which every
   Codex session in this repository reads, and any of the order's own.
8. **Record**: the admin's account of the run, written in the order's file at merge.
   [The record](#the-record) holds those of 001 to 036k and 060 to 066; a later merged order keeps
   its Record in its own file under `docs/work-orders/` until a later condensation (A80) carries
   it here.

## Dispatch

One order, one working tree on a branch `wo/NNN-name` from `main`, outside the repository's own
checkout, made with `git worktree add --no-track`: two orders started at once raced for Git's
configuration without it. The admin starts Codex headless in it:

```sh
CARGO_NET_OFFLINE=true codex exec \
  --ignore-user-config --strict-config --disable apps \
  -s workspace-write --add-dir "$HOME/.cargo" \
  -C <tree> -c 'model_reasoning_effort="high"' \
  --output-schema <order>.schema.json -o <report>.json --json \
  - < docs/work-orders/NNN-name.md > <events>.jsonl
```

Each part answers a fault the first round found:

- `--ignore-user-config` keeps out the owner's Codex configuration and its MCP servers. Codex
  still signs in with the owner's subscription (ruling 48, point 2).
- `--strict-config` stops at a key Codex does not know. A70 lost a task to an override that Codex
  accepted without a warning and ignored (mistake 2 of its results).
- `--disable apps` turns off the apps of the ChatGPT login (ruling 48, point 1).
- `workspace-write` lets Codex write in its tree and nowhere else, with no network. Cargo's cache
  is the one other place it writes, and with `CARGO_NET_OFFLINE` every crate comes from it. An
  order whose tests bind a loopback port has the network on, since the sandbox without it refuses
  the bind (order 010's second round).
- The order goes in on standard input (`-`), so that Codex never waits on an open input, as one
  of A70's runs did for five minutes.
- `-o` and `--output-schema` write the report to a file; `--json` keeps every event, which the
  admin reads only when a report needs explaining. Codex's events do not show every command it
  runs (A70, surprise 7), so a claim that matters is checked against the files.

## Checking what comes back

1. **A verifier**: a second, fresh Codex session in a read-only sandbox, with the order, the diff
   and a schema of its own ([the standing verifier order](../work-orders/verify.md)). It tries to
   refute each change against the decisions and the texts they cite. A reviewer reading cold found
   the first round's most consequential faults: a contradiction across nine promises, and a wrong
   claim about Codex in the coordinator's own notes.
2. **The admin** runs every check itself, reads the diff and the verifier's findings, scans for
   private paths and names, and decides what to fix, to reorder or to put to the owner.
3. **One more round at most**: the admin sends the worker back once with the verifier's findings
   it accepts. If that does not settle them, the admin finishes the work itself.

## Fresh reads

A fresh read is an order whose worker sees only what the order names: a working tree of the
build base, or of `spec/` alone, and no history of rulings. Ruling 48, point 4 gives Codex the
fresh author of increment 4. A fresh reader's report lists where it had to guess.

## What this round taught

From A70, increment 1, the review of both, and the orders below:

- **Brief in decisions, not in hopes.** The order states the decisions; the report states the
  worker's own choices apart, so that the admin sees every choice made.
- **A skeptic per angle pays for itself.** Every review of the first round changed a result: the
  loader's three reviewers found four defects, A70's auditor corrected six counts, and the last
  review found two of the coordinator's own errors. Verifiers found must-level faults in most
  build orders and none in the results orders (005, 008 and 011; 011 had a nit).
- **Confirm a connection with a real call.** Codex's debug output hides MCP tools, and its
  configuration can fail silently: A4, A71 and increment 3 confirm each Agent's connection by a
  tool call that the server records.
- **Keep the owner's machine out.** No order edits the owner's agent configuration or reads a
  credential. A worker's own state, such as Codex's sessions, is the agent's.
- **Close the input, pin the flags, write the report to a file.** The dispatch above is the sum
  of the first round's harness faults.
- **The admin runs what a sandbox cannot.** Real clients, and the harness against them, run from
  the admin session; when such a run shows a harness fault, the admin fixes it, says so in the
  order's Record, and has the worker test the fix (orders 002 and 010).
- **Pre-register before the harness, amend before the run.** Order 002's review found that
  experiment 008's pre-registration named tools otherwise than IFC-012 and left its matrix open;
  it was amended, with a dated note, before any run.
- **Count the coordinator's own choices as risks.** Counting rules fixed after a result is seen
  are the easiest to challenge, so the order that sets them says so.
- **Verify the admin's own decisions.** Order 020's "no exp" contradicted DL13; order 009's
  tarball did not carry the licence the admin had claimed it did.
- **Check each finding against the promise that decides it.** Order 012's admin required logging
  any undecided case rather than inventing its answer; each reported case was decided.
- **Keep durable work outside session scratch space.** Order 019's first run was lost before it
  was committed; its Record reports the rerun, not a recovered first result.
- **Never print a whole large file.** Order 028 printed a fixture whole and passed the model's
  output limit after 44 minutes with no change; use `head`, `tail`, `grep` or a count.
- **Parallel edits need a joining round.** Orders 021 and 022 conflicted in six files; 022 needed
  four rounds and a second fresh verifier after the joined end-to-end test.
- **Read a grant against the requested check.** Order 024 misread its npm grant; the admin ran
  `CONDUIT_TEST_NPM=1` and recorded that the suite passed.

The two passages the condensation plan keeps verbatim are in [order 027's Record](#order-027)
(Codex's `collaboration.*` tools) and [order 028's decision 5](#order-028) (S1's packages are
packages like any other).

## Running the orders through Conduit

The orders go through Conduit itself (A78), as
[IN_FORCE.md, section 15](../../IN_FORCE.md#15-how-the-work-is-governed) states (ruling 61, B12):
each worker an Agent of the `conduit-core` Fleet, and fixed worker and tree slots set up once by
root. Root dispatches orders 1 to 3, which need authority through Fleets (041b, merged) and
workers that run code (ruling 70; order 050a's `.v2` Agents, installed from order 076's branch
until 050 publishes them): order 042, on 2026-11-11, was the first. A coordinator Agent that root
appoints dispatches orders 4 and 5, once appointment is built (the second specification batch,
after the S1 rerun). Each order's working tree is a directory
Resource, and the queue of orders and their reports is the message history. Each worker is an Agent
that the engine runs with a Conduit to its own tree that lists reads and writes, and one to `main`
that lists reads. The admin opens a worker's Conduits when it dispatches the order and closes them
when it merges. That is S1 in daily use, so each friction is logged as a finding of the first build.
When S2 runs, opening the pull request moves behind a Conduit to GitHub too, and the token for it
stays with the engine rather than going to any worker.

## The record

One row per merged order, 001 to 036k and 060 to 066, as the condensation plan's E30 set under
ruling 58, point 4; A77 reads its evidence here. Later merged orders keep their Record in their
own file under `docs/work-orders/` until a later condensation (A80) carries them here. The admin's
own time was not recorded. Every fact in a row comes from the order's Record, kept verbatim below
the row; orders 001 to 004 had no Record of their own, and their original rows, less the bullets
that repeated the title, stand as their record. "Not recorded" marks a
missing fact; a fault with no stated severity is not counted as must, should or nit; a stated
second round does not supply its unrecorded time.

Below each Record, the order's decisions are condensed to a line or a phrase each, in the order's
own bolded words where it had them. They say what the order asked, not what ran or passed. Their
full text is in [this page's last full version](https://github.com/shrav-k/conduit/blob/d8440de34f0e7c5e564dee298e424d2956e8ac40/docs/implementation/WORK_ORDERS.md#the-record).
What they settled holds in `spec/`, the
[settlements register](../proposals/first-build/README.md#settlements-register), the
[log of increment 4](../proposals/first-build/log-increment-4.md),
[CONFORMANCE.md](../../engine/CONFORMANCE.md) and the packages' READMEs, as each line says.

An order keeps its Record in its own file, to avoid parallel edits here. When a condensation
carries it here, the admin writes its row serially, checks that the row and its section below carry its Record and
decisions, then removes the order and its own schema. The shared report schema, the verifier,
staged patches and declared vector changes stay in [the directory](../work-orders/README.md).

| Order | Worker minutes; rounds | Verifier minutes; must / should / nit | Tests or vectors at merge | Findings | Admin changes, observations and own decisions |
| --- | --- | --- | --- | --- | --- |
| [001](#order-001) | 21; rounds not recorded | 5; 1 / not recorded / not recorded | Regression case; pass count not recorded | SCH-015 point settled | [Preserved record and decisions](#order-001) |
| [002](#order-002) | 17 + 25; 2 rounds | 5; 5 / 1 / not recorded | One real run per client; pass count not recorded | Stub findings fixed | [Preserved record and decisions](#order-002) |
| [003](#order-003) | 17 + 5; 2 rounds | 4; 2 / 1 / not recorded | not recorded | Six points and two candidate findings decided | [Preserved record and decisions](#order-003) |
| [004](#order-004) | 17; 2 rounds, second time not recorded | 5; 2 / not recorded / not recorded | 13 vectors pass | Two leading byte order marks settled | [Preserved record and decisions](#order-004) |
| [005](#order-005) | 11; 1 round | 3; 0 / 0 / 0 | not recorded | Four candidate findings | [Preserved record and decisions](#order-005) |
| [006](#order-006) | 18; 2 rounds, second time not recorded | 4; 4 / not recorded / not recorded | 46 vectors pass | 2-03 settled | [Preserved record and decisions](#order-006) |
| [007](#order-007) | 8; 1 round | 2; 0 / 0 / 0 | 20 tests; admin ran tests and 12 runs | not recorded | [Preserved record and decisions](#order-007) |
| [008](#order-008) | 7; 1 round | 2; 0 / 0 / 0 | not recorded | not recorded | [Preserved record and decisions](#order-008) |
| [009](#order-009) | 23; 2 rounds, second time not recorded | 5; 3 / not recorded / not recorded | 107 vectors; fixes each have a failing-before vector | Licence claim corrected; tarball removal assigned to 013 | [Preserved record and decisions](#order-009) |
| [010](#order-010) | 13; 2 rounds, second time not recorded | 4; not recorded / 2 / not recorded | All tests pass in admin session; nine scenarios completed | Two tests fixed; Codex helper corrected | [Preserved record and decisions](#order-010) |
| [011](#order-011) | 10; 1 round | 3; not recorded / not recorded / 1 | not recorded | One candidate finding on Codex's header helper | [Preserved record and decisions](#order-011) |
| [012](#order-012) | 25; 2 rounds, second time not recorded | 7; 9 / 1 / not recorded | 190 vectors; fixes each have a failing-before vector | Each finding checked against its deciding promise | [Preserved record and decisions](#order-012) |
| [013](#order-013) | 14; 2 rounds, second time not recorded | 6; 4 / not recorded / not recorded | 230 vectors pass | Four findings fixed | [Preserved record and decisions](#order-013) |
| [014](#order-014) | 22; 2 rounds, second time not recorded | 6; 5 / not recorded / not recorded | 248 vectors; six fixes with failing-before vectors | Two access holes and four other faults fixed | [Preserved record and decisions](#order-014) |
| [015](#order-015) | 26; 2 rounds, second time not recorded | 6; 6 / not recorded / not recorded | 306 vectors; fixes with failing-before vectors | 2-07 settled; decision 8a narrowed | [Preserved record and decisions](#order-015) |
| [016](#order-016) | 23; 2 rounds, second time not recorded | 7; 8 / not recorded / not recorded | 345 vectors; fixes with failing-before vectors | Eight findings fixed | [Preserved record and decisions](#order-016) |
| [017](#order-017) | 41; 2 rounds, second time not recorded | 6; 3 / not recorded / not recorded | 361 vectors; fixes with failing-before vectors | 2-08 settled against disclosure; OPS-029 amended | [Preserved record and decisions](#order-017) |
| [018](#order-018) | 8; rounds not recorded | 3; 1 / not recorded / not recorded | not recorded | Neither protocol error proves no effect; decision 8a settled | [Preserved record and decisions](#order-018) |
| [019](#order-019) | Lost run time not recorded; rerun 16; then second round, time not recorded | 5; 3 / 1 / not recorded | 58-step S1 vector; 75 record expectations fail when removed | 217 promises covered, 91 given reasons; increment 2 done | [Preserved record and decisions](#order-019) |
| [020](#order-020) | 37; 2 rounds, second time not recorded | 7; not recorded / not recorded / not recorded (3 defects) | 105 Rust tests pass; every vector passes; other checks clean | 3-01: admin's no-exp decision contradicted DL13 | [Preserved record and decisions](#order-020) |
| [021](#order-021) | 49; 2 rounds, second time not recorded | 7; not recorded / not recorded / not recorded (5 defects) | 122 Rust tests with CONDUIT_TEST_NPM=1 pass; other checks clean | Node 24 declaration qualified on Node 21.5.0 | [Preserved record and decisions](#order-021) |
| [022](#order-022) | 48; 4 rounds, later times not recorded | 5; second verifier time not recorded; not recorded / not recorded / not recorded (3 + 4 defects) | 134 Rust tests with CONDUIT_TEST_NPM=1 pass; other checks clean | Merge conflicted in six files; end-to-end S1 test | [Preserved record and decisions](#order-022) |
| [023](#order-023) | 32; 2 rounds, second time not recorded | 9; not recorded / not recorded / not recorded (4 defects, 2 smaller faults) | 136 Rust tests and six in each package pass; other checks clean | 3-02: model required with an empty default; no real client ran | [Preserved record and decisions](#order-023) |
| [024](#order-024) | 32; rounds not recorded | 5; 0 / 0 / 0 | 136 Rust tests with CONDUIT_TEST_NPM=1 and six in each package pass; other checks clean | 3-03 to 3-05 settled | [Preserved record and decisions](#order-024) |
| [025](#order-025) | 61 + 40; 2 rounds | 5; second verifier time not recorded; not recorded / not recorded / not recorded (4 defects, 1 weak test; 3 further faults) | 147 tests with CONDUIT_TEST_NPM=1 pass; other checks pass | 4-03, 4-04, 4-05 | [Preserved record and decisions](#order-025) |
| [026](#order-026) | 9; 2 rounds, second time not recorded | 2; not recorded / not recorded / not recorded (1 stale statement) | Checks pass; worker Rust run could not bind loopback | 25 findings: 16 in scope, nine outside | [Preserved record and decisions](#order-026) |
| [027](#order-027) | 43; 2 rounds, second time not recorded | 4; not recorded / not recorded / not recorded (1 fault) | 153 tests with CONDUIT_TEST_NPM=1 and six in agent-codex pass; other checks pass | 4-06 stays open in this Record; see later package settlement | [Preserved record and decisions](#order-027) |
| [028](#order-028) | 44 lost + 50 rerun; then second round, time not recorded | 5; not recorded / not recorded / not recorded (2 faults) | 163 tests with CONDUIT_TEST_NPM=1 pass; other checks pass | No specification form for an ended condition (LIF-011, OBS-002) | [Preserved record and decisions](#order-028) |
| [029](#order-029) | 14; 2 rounds, second time not recorded | 4; not recorded / not recorded / not recorded (3 stale statements) | Links resolve after rebase; Rust loopback blocked | Three outside-scope worker findings | [Preserved record and decisions](#order-029) |
| [030](#order-030) | 22; 2 rounds, second time not recorded | 7; not recorded / not recorded / not recorded (2 faults) | 163 Rust tests, five repository checks and tool unit tests pass | WO030-F1: safety-vector classification undecided | [Preserved record and decisions](#order-030) |
| [031](#order-031) | 14; rounds not recorded | 4; 0 / 0 / 0 | 9 spike tests pass; verifier reproduced results | 031-01: exact translation refused because of as | [Preserved record and decisions](#order-031) |
| [032](#order-032) | 30; rounds not recorded | 7; not recorded / not recorded / not recorded (retired-ID check and broken sentence) | 163 Rust tests pass, three ignored; regression test | 032-G1, 032-G2, 032-G3 | [Preserved record and decisions](#order-032) |
| [033](#order-033) | 21; rounds not recorded | 5; not recorded / not recorded / not recorded (core wording and tracked bytecode) | 163 engine tests pass, three ignored; nine in experiment 011 | Pure-core wording restored; bytecode removed | [Preserved record and decisions](#order-033) |
| [034](#order-034) | 22; rounds not recorded | 7; not recorded / not recorded / not recorded (3 directive statuses, 1 history row) | 17 extractions compared equal to C0; pass counts not recorded | 034-F1: A79 already assigned; lean vectors under A39 | [Preserved record and decisions](#order-034) |
| [035](#order-035) | 10; rounds not recorded | 5; not recorded / not recorded / 1 | not recorded | 035-01: A78 trigger corrected to ruling 61 B12 | [Preserved record and decisions](#order-035) |
| [036a](#order-036a) | 22; 2 rounds, second time not recorded | 10; not recorded / not recorded / not recorded (4 faults) | 163 engine tests pass, three ignored | 036a-01, 036a-02, 036a-03 | [Preserved record and decisions](#order-036a) |
| [036b](#order-036b) | 29; 2 rounds, second time not recorded | 7; not recorded / not recorded / not recorded (4 faults) | 163 engine tests pass, three ignored; staged text passes schema and document checks in scratch tree | 166 owner-input steps in 161 cases affected | [Preserved record and decisions](#order-036b) |
| [036k](#order-036k) | 8; rounds not recorded | 6; not recorded / not recorded / not recorded (YAML 1.1 fallback) | 18 tests written; pass count not recorded; regression test | YAML fallback corrected to YAML 1.2 | [Preserved record and decisions](#order-036k) |
| [060](#order-060) | 16; rounds not recorded | 6; not recorded / not recorded / not recorded (1 missed choice) | not recorded | WO060-01: ledger-link patch; M75 choice restored | [Preserved record and decisions](#order-060) |
| [061](#order-061) | Guarded dispatch time not recorded; rerun 21; rounds not recorded | 5; not recorded / not recorded / not recorded (1 missed choice, 2 links) | Shell loopback tests blocked; pass counts not recorded | F3 rejected option and reason restored | [Preserved record and decisions](#order-061) |
| [062](#order-062) | Guarded dispatch time not recorded; rerun 14; then second round, time not recorded | 6; not recorded / not recorded / not recorded (circular G9 and missed references) | not recorded | G9 circular condition removed; 399 references repointed | [Preserved record and decisions](#order-062) |
| [063](#order-063) | 20; rounds not recorded | 5; not recorded / not recorded / not recorded (3 C0 links, instruction, 2 historical references) | Shell loopback tests blocked; pass counts not recorded | 33 findings preserved in settlements register | [Preserved record and decisions](#order-063) |
| [064](#order-064) | 14; 2 rounds, second time not recorded | 5; not recorded / not recorded / not recorded (4 passages and row references) | 31 extracted passages compared with C0 | 47 row references repointed | [Preserved record and decisions](#order-064) |
| [065](#order-065) | 12; rounds not recorded | 5; not recorded / not recorded / not recorded (2 statements) | not recorded | Coverage count 100, not 76; node selectors retained | [Preserved record and decisions](#order-065) |
| [066](#order-066) | 12; rounds not recorded | 2; 0 / 0 / 0 | Six package tests pass | not recorded | [Preserved record and decisions](#order-066) |

### Order 001

Work order 001: the review's fixes, and the tokens of a pattern. Its original record, as it had no
Record of its own:

- **What**: The coordinator's fixes from the review of increment 1 and A70, and ruling 48, point 3
- **Worker**: 21 minutes; every decision applied as written; 36 files; one point left to the admin
- **Verifier**: 5 minutes; one `must`: `check_schemas.py` crashed on a repeat count past Python's limit
- **What the admin changed**: Fixed the crash, with a regression case; settled the point left to it, SCH-015's rule for a reference to the wrong kind
- **State**: done on 2026-11-09

Decisions, the coordinator's under ruling 46, point 4, unless they say ruling 48:

- **A. A rejection by a schema cites SUB-005 everywhere** (completes 1-01): CON-020, POOL-009,
  POOL-012, MAN-012, MAN-007, CTR-008, OBJ-015, OBJ-019, TPL-001 and every like wording; a package
  document a schema rejects at installation is reported under SUB-005, the request invalid under
  PKG-023 (MAN-011); the diagnostic carries `keywordLocation`, and `absoluteKeywordLocation` where
  known, in its SARIF `properties`. Now finding 1-14.
- **B. A byte order mark in JSON is accepted and ignored** (corrects 1-05), as RFC 8259, section
  8.1 allows; UTF-8 is the profile's own rule, with I-JSON as precedent. Now finding 1-15.
- **C. SUB-011 gets a floor and a wider reach** (corrects 1-11): every engine accepts 64 levels
  of nesting, the loader 128; limits on a file, a submission and a package's archive fall under
  it; exhausted storage stays DL10's. Now finding 1-16.
- **D. A pattern keeps to JSON Schema's interoperable tokens** (ruling 48, point 3; settles
  1-04): SCH-016 amended, the specification's patterns rewritten with no verdict changed. Now
  finding 1-17.
- **E. The note on Codex's token is wrong**: Codex 0.167.0 reads headers from
  `mcp_servers.<id>.http_headers_helper`, as Claude Code does from `headersHelper`, so no adapter
  is counted in advance. Its 401 question and its note on a missing plain tool are answered in
  [noted for later increments](../proposals/first-build/README.md#noted-for-later-increments);
  experiments 008 and 009 showed Codex reaching an engine at a loopback address.
- **F. What an Agent reaches by its own means is outside the documents**; confining it is a
  runtime provider's (PRV-006), and every restriction is reported as mediated (ACC-027). Now in
  `spec/README.md`, as later rulings amended it.
- **G. What a pin holds**: a package file's hash does not pin its registry-resolved
  dependencies, and a `command` pins nothing (MAN-010). Now finding 1-18.
- **H. The log**: findings 1-14 on, now in the settlements register.

### Order 002

Work order 002: the harness of experiment 008, a stub of the presented server and a runner. Its
original record, as it had no Record of its own:

- **What**: The harness of experiment 008 (A71)
- **Worker**: 17 minutes, then 25 on a second round; a stub on the official Rust SDK and a runner, with tests that need no agent
- **Verifier**: 5 minutes; five `must` and one `should` in the stub, all fixed on the second round
- **What the admin changed**: Amended the pre-registration before any run: tools named as IFC-012 names them, and the matrix cut to 68 runs. Sent the worker back to use the owner's login rather than a fresh configuration and an API key. Added Codex's approval of the stub's server, which the second round missed. Ran one real run per client before the matrix
- **State**: done on 2026-11-09

Decisions, the method of [experiment 008](../../experiments/008-real-client/README.md): (1) **one
Cargo project in experiments/008-real-client/**, on the official Rust SDK (`rmcp`); (2) **the
stub** names tools as IFC-012 does, takes every `_meta` key from `spec/names.yaml`, and accepts
protocol revisions 2025-06-18, 2025-11-25 and 2026-07-28, logging each session's; (3) **the
scenario** is fixed in code, a Conduit `files` listing `read` and `write` and `notes` listing
`read`, closed at first; (4) **the runner** starts each client headless outside the repository
with the pre-registration's isolation, its standard input closed, and never edits the owner's
configuration; (5) **tests that need no agent**; (6) **a finding of the first build**: the nearest
sound reading, listed under `findings`.

### Order 003

Work order 003: a study for increment 2, the core on S1. Its original record, as it had no Record of
its own:

- **Worker**: 17 minutes; the plan, the drafts of S1 and a test; six decisions for the admin, two candidate findings, nine proposed orders
- **Verifier**: 4 minutes; two `must`, a promise's strength misstated and a safety case's refusal misdefined, and one `should`, a weak case
- **What the admin changed**: Sent the worker back once with the findings, which it fixed in 5 minutes; [decided](../proposals/first-build/increment-2/decisions.md) the six points and the two findings
- **State**: done on 2026-11-09

Decisions: (1) the order is a reading and decides nothing, setting out options with a
recommendation; (2) nothing under `spec/` changes; (3) **`plan.md`**, the worker's proposal: S1 as
documents, with an existing tool for the history (ruling 40), the promises and safety cases S1
touches, a format for the vectors (A39), a pure core with time and IDs passed in, where the text
does not decide, and a sequence of orders; (4) **`s1/`**, the drafts; (5) **one test** that the
loader accepts them. The admin's [decisions](../proposals/first-build/increment-2/decisions.md)
on the plan hold.

### Order 004

Work order 004 (I2-01): the vectors' format, the core's crates, and first start. Its original
record, as it had no Record of its own:

- **Worker**: 17 minutes, then a second round; `conduit-core`, `conduit-conformance`, the format and 11 vectors; one point left to the admin
- **Verifier**: 5 minutes; two `must`: unauthorized requests reached a diagnostic before their refusal, and records took the named object for the target
- **What the admin changed**: Settled the point left to it (two leading byte order marks are not YAML) and sent the worker back once with it and the findings; 13 vectors pass
- **State**: done on 2026-11-09

Decisions:

1. **The vectors' format**: YAML cases under `conformance/`, with its README (not binding),
   `case.schema.json` and the profile `conformance/profiles/first-build.yaml`.
2. **Two new crates**: `conduit-core`, pure, with no file, clock, network or process access, and
   `conduit-conformance`, which runs the cases.
3. **The core's interface**: state and input in; new state, decisions, records, intents for the
   shell and a reply out. Each decision names its promises, an implementation choice its DL item;
   time, IDs and machine reports are inputs.
4. **First start** makes exactly what FST-001 and FST-002 name, calls no implementation (FST-003),
   keeps provenance (FST-005) and makes `local` (PLC-008, PLC-026, PRV-014); a repeat makes no new
   identity (DL6).
5. **Vectors**: first start, FST-006, LIF-034, the repeat, and increment 1's four loader-only
   settlements.
6. **A finding**: where a promise does not decide, stop that case, do not invent its answer, and
   log it "For the admin". Later orders apply this decision.

### Order 005

Work order 005: the results of experiment 008 (A71). Its Record, written by the admin when it merged
the order, on 2026-11-09:

- **Worker**: 11 minutes. Every run classified or listed as not run as planned; dated results
  and trimmed evidence; four candidate findings for the admin.
- **Verifier**: 3 minutes. No fault found.
- **Admin**: checked the classification of C5 against the raw records of two runs, and
  recorded that review in the results, which the worker had left as pending. No second round.

Decisions, now the method of [its results](../../experiments/008-real-client/results-2026-11-09.md):
(1) **wire, client and model apart** (E17), never inferring what the model saw from the wire;
(2) **the next action** by the pre-registration's categories only, each citing its event; (3)
**the questions the pre-registration asks**, per client and case, with runs; (4) **runs that did
not run as planned** listed with their cause, not classified; (5) the admin's three smoke runs and
Codex's approval of the stub's server stated as deviations; (6) **findings of the first build** as
candidates (IFC-012, IFC-013, IFC-025, IFC-038, IFC-042, PRV-022), `spec/` unchanged.

### Order 006

Work order 006 (I2-02): authored schemas, the tool definitions, and E05. Its Record, written by the
admin when it merged the order, on 2026-11-09:

- **Worker**: 18 minutes, then a second round. The core's evaluation of authored schemas and its
  standalone tool definitions; 28 vectors for E05 and the publication checks; IFC-033 amended
  by decision 6; one point left to the admin (2-03).
- **Verifier**: 4 minutes. Four `must`: a narrowed control's references left dangling; the
  pattern adapter changed what a reference to a declared property means; defaults not filled
  under `additionalProperties` and `patternProperties`; and the runner's check that state is
  unchanged could not fail, so it hid every other fault of that kind.
- **Admin**: settled 2-03 (different unconditional defaults for one place are rejected under
  SCH-009) and sent the worker back once with it and the four findings. Each fix has a vector
  that failed before it; 46 vectors pass, and none of the first 41 failed once the check could.

Decisions:

1. **Reuse the loader's validators**: no new dialect.
2. **Publication** as IFC-033 says: a standalone schema, each contract copied once under its key,
   local references rewritten, `$ref` siblings kept, finite on recursion, with no network and no
   resolver.
3. **E05's vectors** compare the verdicts of the authored schema and of the published definition,
   for `inputSchema` and `outputSchema`.
4. **The root `$defs` key** (decision 6, settling F2): an authored root `$defs` key of a contract
   copy's form is rejected under IFC-033. Now finding 2-02.
5. **A finding**: as order 004's decision 6.

### Order 007

Work order 007: experiment 008's follow-up, a later turn. Its Record, written by the admin when it
merged the order, on 2026-11-09:

- **Worker**: 8 minutes. Two cases with two turns each, the follow-up's 12 runs behind
  `--follow-up`, the original 68 unchanged; 20 tests that need no agent.
- **Verifier**: 2 minutes. No fault found.
- **Admin**: ran the tests, read the resume command, and ran the 12 runs. No second round.

Decisions: (1) **two new cases**, C1-later and C5-later; (2) **a second turn in the same
session**, Claude Code on streamed input and Codex by `codex exec resume`, every other flag as
order 002 left it; (3) **the follow-up's matrix**, 12 runs behind `--follow-up`, the original 68
unchanged; (4) **what is recorded**, per turn; (5) **tests that need no agent**.

### Order 008

Work order 008: the results of experiment 008's follow-up, a later turn. Its Record, written by the
admin when it merged the order, on 2026-11-09:

- **Worker**: 7 minutes. 12 runs, 24 turns classified, none excluded; results and 37 trimmed
  evidence files.
- **Verifier**: 2 minutes. No fault found.
- **Admin**: read the second turn of all 12 runs against the classification, and recorded its
  review in the results. No second round.

Decisions: (1) order 005's decisions 1, 2, 4 and 6 apply, for each turn apart; (2) **the
question of the follow-up**: in the second turn, did the model use the tool opened in the first
(C5-later), and stop offering or still call the one closed (C1-later); (3) the later-turn results
and trimmed evidence, with no absolute path or credential, linked from the pre-registration, the
first results and A71's row.

### Order 009

Work order 009 (I2-03, first part): packages and projects, from snapshots. Its Record, written by
the admin when it merged the order, on 2026-11-09:

- **Worker**: 23 minutes, then a second round. An immutable catalog of snapshots, the resolving
  of a project's pins, exports and names, SUB-008's digest, and S1's fixtures with computed
  digests; 58 vectors, 104 in all, then 107.
- **Verifier**: 5 minutes. Three `must`: a binding through a parent that may be null was
  accepted; a template's override of an implementation was matched by its spelling rather
  than by the operation it resolves to; and normalizing `..` in a relative program could
  change which program it names.
- **Admin**: sent the worker back once with the three findings; each fix has a vector that
  failed before it. Kept the published tarball of the filesystem server (36 KB) as a fixture,
  so that its pin's hash is real. **Corrected the same day**: the admin had written that the
  tarball carries its licence; it does not (its `package.json` says "SEE LICENSE IN LICENSE",
  and no such file is inside). Order 013 removes it, and keeps only its hash.

Decisions:

1. **A catalog of installed packages** is given to the core as snapshots, never fetched, and is
   immutable once given.
2. **Resolving a project**: pins against the catalog, exports and names (PKG, MAN, NAM),
   SCH-015's qualified references, and the core package's contracts, which no package replaces
   (NAM-020, FST-005); a missing or mismatched pin is rejected and nothing is fetched.
3. **The digest of a submitted project** as SUB-008 says, from the manifest as submitted.
4. **S1's packages and project** move into `conformance/fixtures/s1/`, with computed digests.

Also: (5) vectors, an accepted and a rejected case per rule, none claiming an installation; (6) a
finding, as order 004's decision 6.

### Order 010

Work order 010: the harness of experiment 009 (A4), wrappers that run each client as an Agent. Its
Record, written by the admin when it merged the order, on 2026-11-09:

- **Worker**: 13 minutes, then a second round. Two wrappers in Rust, one per client, and a
  runner of nine scenarios, reusing experiment 008's stub.
- **Verifier**: 4 minutes. Two `should`: a test of attribution that would pass with the
  credentials swapped, and a test of redaction that never reached the redaction.
- **Admin**: ran the harness against the real clients. Codex failed twice, because it runs its
  header helper without the run's environment and rejects `env_vars` for an HTTP server; the
  admin had the Codex wrapper write the identity directory into the helper's command line, and
  the third attempt completed all nine scenarios. It sent the worker back once to test that
  change and fix the two tests. The worker's sandbox, run without network access, refused the
  tests that bind a loopback port, so the admin ran them: all pass.

Decisions, now the method of [experiment 009's harness](../../experiments/009-native-launch/HARNESS.md):
(1) **one Cargo project in experiments/009-native-launch/**, reusing experiment 008's stub; (2)
**two wrappers**, each an MCP server over stdio whose one tool runs one turn of its client, with
experiment 008's isolation, Claude Code's `SendMessage`, `ListAgents` and `RemoteTrigger` denied
and its claude.ai connectors off, the token read by the header helper from
`CONDUIT_IDENTITY_DIR`, and `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` removed; (3) **a runner**
that starts each wrapper with only PRV-022's and PRV-023's variables, a private empty working
directory and its own identity directory, for two turns with a Conduit opened and another closed
between them, and both Agents at once; (4) **tests that need no agent**; (5) where a promise is
not clear enough, the nearest sound reading, under `findings`.

### Order 011

Work order 011: the results of experiment 009 (A4), native launch and identity. Its Record, written
by the admin when it merged the order, on 2026-11-09:

- **Worker**: 10 minutes. Every point of the pre-registration answered or said to be unshown,
  with runs cited; all three attempts reported; one candidate finding, on Codex's header helper.
- **Verifier**: 3 minutes. One `nit`: the evidence guide said usage was omitted, though Codex's
  token counts were kept.
- **Admin**: fixed the nit, and carried the finding and the points left unshown into what
  increment 3 needs. No second round.

Decisions: (1) **one answer per point of "What it must show"**, per client, citing runs, never
from the flags alone; (2) **the two first attempts are results too**: Codex runs its header helper
with a filtered environment and rejects `env_vars` for an HTTP server, so its wrapper writes the
identity directory into the helper's command line, a candidate finding for increment 3's Codex
Agent type; (3) **write** the results and trimmed evidence, every path, user name and credential
removed, and mark A4 done; (4) **limits**.

### Order 012

Work order 012 (I2-04): registration and the resolving of instances. Its Record, written by the
admin when it merged the order, on 2026-11-09:

- **Worker**: 25 minutes, then a second round. Atomic registration and the resolving of
  instances; S1 registers as nine templates, six named instances and four nested Conduits, with
  no effect, in any order; 52 vectors, then 190 in all.
- **Verifier**: 7 minutes. Nine `must` and one `should`, among them a Type with no schema
  rejected, a requirement's arguments left unvalidated, a retired Resource selectable, and
  unresolved names in nested declarations left unreported.
- **Admin**: sent the worker back once, telling it to check each finding against the promise
  that decides it and to log, not invent, any the promises did not decide. Each was decided
  (CTR-004, PLC-005, NAM-019, PKG-007, OBJ-009, POOL-014, OBJ-008, OBJ-013, INS-023 and others),
  and each fix has a vector that failed before it.

Decisions:

1. **Registration is atomic** (SUB-003): a rejected submission publishes nothing and starts
   nothing.
2. **Resolving an instance**: template, arguments with defaults filled, references stored by ID and
   never by name, owner, placement and nested objects; generated names as DL15's choice.
3. **No effect**: registration makes no setup, no run and no call.
4. **Order has no meaning** (SUB-001).

Also: (5) vectors for S1 and each rule, accepted and rejected, in several orders; (6) a finding,
as order 004's decision 6.

### Order 013

Work order 013 (I2-03, second part): installing a package from its archive. Its Record, written by
the admin when it merged the order, on 2026-11-09:

- **Worker**: 14 minutes, then a second round. Installation from an archive's bytes, given as an
  input, under PKG-004, PKG-022, PKG-026 and MAN-013, with DL7's dialect in the profile; S1
  resolves after installation as from its snapshot; the third-party tarball removed and its
  hash kept as recorded data; archives the repository makes by a generator in Rust.
- **Verifier**: 6 minutes. Four `must`: an archive could install at an address a submitted
  project still holds; a link through a regular file was read as a directory; the limit of 40
  link expansions allowed 39; and an escape's diagnostic did not name its member.
- **Admin**: sent the worker back once; each finding was decided by its promises (PKG-029,
  MAN-016, PKG-022, MAN-013, PKG-023 and the profile's DL7) and fixed with a vector that failed
  before it. 230 vectors pass.

Decisions:

1. **The archive is an input**: its bytes, its expected hash and the completion of fetching them;
   the core fetches nothing.
2. **Installing** verifies the digest (PKG-026), unpacks as PKG-022 and MAN-013 say in DL7's form,
   checks address and version, reads documents as MAN-011 says, dot-names skipped, and reports as
   PKG-023 says.
3. **Vectors with real archives** that the repository makes reproducibly.
4. **No third party's bytes in the repository**: the filesystem server's tarball goes, as it
   carries no licence text; its SHA-256, from `npm pack` of the published version, stays as data.
5. **A finding**: as order 004's decision 6.

### Order 014

Work order 014 (I2-05): the access decision and the tools an Agent is shown. Its Record, written by
the admin when it merged the order, on 2026-11-09:

- **Worker**: 22 minutes, then a second round. The pure access decision and each instance's
  exact tool list; 10 vectors, 240 in all, then 248.
- **Verifier**: 6 minutes. Five `must`, two of them holes in access itself: a Pool entry's
  reference resolved to the Pool, so authority over management alone reached the entry's
  Resource; and an operation was allowed that the target's held Capability did not include.
  Also: root could declare `local` lost; tool-name collisions counted operations never shown as
  tools; and a collision's diagnostic named neither the tool nor the other Conduit.
- **Admin**: sent the worker back once, and had it search, after the first fix, for any other
  reference that resolves to an owner rather than to what it names. It found one: a bare name
  ignored the kind its schema fixes and could select an owner of the same name (OBJ-006). Each
  of the six was decided by its promises and fixed with a vector that failed before it.

Decisions:

1. **The access decision is pure**: from the state, the instance a request was established for
   (or none, or one whose existence cannot be read, ACC-037), the tool and its arguments, it routes
   under ACC-033 and applies each bound; refused and invalid follow OPS-023's precedence, and a
   refusal discloses no named object (IFC-025).
2. **The tools an instance is shown** are exactly IFC-012's, defined as IFC-033 says; a tool no
   contract maps is never shown (CON-017); the list changes when a Conduit opens or closes
   (IFC-013, IFC-038).

Also: (3) vectors S1-exact-access, S1-pool-authority and S1-plain-filter, a guessed tool refused
with no invocation, and each bound at and past its limit; (4) a finding, as order 004's decision
6.

### Order 015

Work order 015 (I2-06): how a request ends, its repeats, and plain results. Its Record, written by
the admin when it merged the order, on 2026-11-09:

- **Worker**: 26 minutes, then a second round. How a request ends, repeats from the invocation
  index, plain results with decision 8a, unknown outcomes, reconciliation and cancellation, and
  F1 by decision 5; 37 vectors, then 306 in all. It stopped at one gap (2-07): no contract
  declares an effectful plain operation safe to repeat.
- **Verifier**: 6 minutes. Six `must`: malformed metadata read as an outcome; capability errors
  that the promises make failed read as unknown; interrupted calls left in progress after a
  restart; conflict reported before invalidity; and records without the idempotency key or the
  inputs S1 asks them to keep.
- **Admin**: settled 2-07 by narrowing decision 8a (after -32020, only an operation with no
  effect is sent again), and sent the worker back once with it and the six findings, each
  decided by its promises and fixed with a vector that failed before. Marked the two cases of
  -32602 and -32020 settled in the tracker.

Decisions:

1. **The ending of a request** under OPS-001 and OPS-005, with IFC-025's envelope and OPS-023's
   order; inputs normalized with defaults filled before a repeat is compared (OPS-009, OPS-012).
2. **Repeats**: the invocation index is keyed by caller, sender, target, operation and filled
   contents; a repeat of a success is answered from what was kept and never dispatched again;
   other contents report conflict (OPS-032, OPS-034).
3. **F1, by decision 5**: a succeeded `plain` invocation's content is given again on a repeat
   while DL10 keeps outputs, then a repeat reports conflict (OPS-034). Now finding 2-04.
4. **Plain results**, by [decision 8a](../proposals/first-build/increment-2/decisions.md) on
   order 018's check: `-32602` from a `plain` server is `failed` only for an operation with no
   effect, not a control and starting no model work, `unknown` otherwise; after `-32020` a call is
   sent again only for an operation with no effect or declared safe to repeat; a rejection before
   sending has a reason and no outcome (OPS-005). Now findings 2-05 and 2-06, narrowed by 2-07.
5. **Unknown outcomes and reconciliation**: an unknown write is never dispatched again, after a
   repeat or a restart (OPS-012, OPS-013, OPS-018, IMP-007); reconciliation by an authorized actor
   with the evidence it checked sets the outcome (OPS-014, OPS-030), never a timeout.
6. **Records cannot erase invocations**: the invocation index is state of its own.

Also: (7) vectors for every row and variant; (8) a finding, as order 004's decision 6.

### Order 016

Work order 016 (I2-07): setup, and the opening and closing of Conduits. Its Record, written by the
admin when it merged the order, on 2026-11-09:

- **Worker**: 23 minutes, then a second round. Setup in its parts, intents to run and their
  completions, and the opening, closing, removal and explicit replacement of Conduits, with T7's
  case: closing A's Conduit leaves B's working; 22 vectors, then 345 in all.
- **Verifier**: 7 minutes. Eight `must`, among them Pool entries set up by name rather than by
  their dependencies, a refused hook leaving its setup in progress, cancellation or retirement
  losing an effect's unknown outcome, and a relative program sent to a runtime provider.
- **Admin**: sent the worker back once, insisting that an effect whose outcome is unknown is
  never reported failed and that nothing the promises forbid is dispatched. Each finding was
  decided by its promises (POOL-013, OPS-022, OPS-027, TPL-012, LIF-009, PRV-007, PRV-005 and
  others) and fixed with a vector that failed before it.

Decisions:

1. **Setup** in its parts, in the promises' order, the sender's authority checked for each open; a
   failed part leaves those before it, with no rollback; a second setup makes no new identity and
   opens nothing already open.
2. **Intents and completions**: running an implementation is an intent to the shell (PRV-003,
   PRV-007, PRV-022); the core never starts a process.
3. **Opening and closing** under CON-004: a closed Conduit's tools leave its consumer's list
   (IFC-013); a late result ends as ACC-034 and IFC-025 say; a closed Conduit is removed (LIF-035),
   and restoring one is an explicit registration of its replacement (LIF-009).

Also: (4) vectors for S1's base setup and the rows named; (5) a finding, as order 004's decision
6.

### Order 017

Work order 017 (I2-08): records, inspection, and the bounds of repair. Its Record, written by the
admin when it merged the order, on 2026-11-09:

- **Worker**: 41 minutes, then a second round. Every record validated against `record.v1`,
  inspection, retention and purge that leave the invocation index whole, repair's bounds, and
  D19's counts (a keyed unknown write's index entry kept 373,365 bytes after a purge, as REC-004
  and OPS-008 require); 11 vectors, then 361 in all. It stopped at 2-08: which Pool's authority
  inspects an object that no longer exists.
- **Verifier**: 6 minutes. Three `must`: a successful keyed registration repeated could report
  conflict; template inspection could give a status its schema forbids; and a refused keyed
  request could lose its key in the record.
- **Admin**: settled 2-08 by the least disclosure (only management's authority reaches an object
  that does not exist; a Pool's alone is refused), and sent the worker back once with it and the
  three findings, each fixed with a vector that failed before. It then found that OPS-029 had
  decided 2-08's case the other way, on purpose, and kept its settlement as a choice against
  disclosure, amending OPS-029 and saying so in the log: this is the decision of increment 2 the
  owner may most want to look at. `conformance/` now holds about 8 MB of cases, many of them full
  states; order 019 is asked to report how to keep them lean.

Decisions:

1. **Every record the promises require**, validated against `record.v1`, the access decision's
   record before any dispatch it allows.
2. **Inspection** as LIF-013 says, compared by decision 7's projections, which never hide an
   omission.
3. **Retention and purge** leave the invocation index whole; a gap a purge leaves is shown.
4. **Repair's bounds**: a follow-up by expected ID, a template replaced with its revision checked,
   and E08's broker case, with no implicit effect.
5. **Counting** for D19: the bytes of records and of the invocation index, apart.

Also: (6) vectors for records, inspection and repair; (7) a finding, as order 004's decision 6.

### Order 018

Work order 018: the check of two protocol errors, -32602 and -32020. Its Record, written by the
admin when it merged the order, on 2026-11-09:

- **Worker**: 8 minutes. The protocol's text and the official SDKs' source, cited: neither
  `-32602` nor `-32020` proves that a server has not acted, and the SDKs let a handler answer
  either after acting. No such path in the filesystem server at 2.1.0 or 2.2.0.
- **Verifier**: 3 minutes. One `must`: the report gave `failed` to the engine's own rejection
  before dispatch, which has a reason and no outcome (OPS-005).
- **Admin**: fixed that sentence, and decided the two readings as decision 8a of
  [increment 2's decisions](../proposals/first-build/increment-2/decisions.md), which order
  015 applies.

Decisions: (1) only the new file, nothing under `spec/`; (2) **the protocol's text** on -32602
and -32020 in each named revision; (3) **what servers do**: whether a handler in the official
SDKs can answer with either code after acting; (4) **the reference servers**: whether the
filesystem server at 2.1.0 and 2.2.0 does; (5) **write** the check, with options and a
recommendation, deciding nothing. It is now
[the evidence for decision 8a](../proposals/first-build/increment-2/decisions.md#evidence-for-decision-8a).

### Order 019

Work order 019 (I2-09): S1 whole on the core, and what increment 3 receives. Its Record, written by
the admin when it merged the order, on 2026-11-10:

- **Worker**: a first run of 2026-11-09 was lost with the session's scratch space, before it was
  committed; the order ran again, 16 minutes, then a second round. S1 runs whole on the core as
  a 58-step vector; every promise S1 touches has vectors (217) or a stated reason (91); the
  first build's conformance document and the handoff to increment 3 are written; increment 2's
  done-when is reported item by item.
- **Verifier**: 5 minutes. Three `must` in the handoff and the conformance document, and one
  `should`: steps of the whole-S1 vector that did not check their records.
- **Admin**: sent the worker back once; each of the 75 record expectations added now fails the
  vector when removed. Marked increment 2 done.

Decisions: (1) **S1 whole**, one vector or a short chain from first start through T1, T2 and T7,
with scripted completions for real servers; (2) **coverage**: for every promise S1 touches, its
vectors or why none; (3) **the first build's conformance document**, now
[CONFORMANCE.md](../../engine/CONFORMANCE.md); (4) **what increment 3 receives**, now summarized
in [the core's boundary](../../engine/README.md#the-cores-boundary); (5) **the done-when of
increment 2**, item by item; (6) **lean vectors**: why `conformance/` holds about 8 MB of cases,
and how to keep them small, rewriting nothing; (7) a finding, as order 004's decision 6.

### Order 020

Work order 020 (I3-01): the shell's store, its loop and its identities. Its Record:

Run on 2026-11-10 by the dispatch of [work orders](../implementation/WORK_ORDERS.md): the
worker in 37 minutes, then a fresh verifier in 7. The verifier found three defects, each real:
a waiting caller could be given an earlier success instead of the core's refusal, or wait
forever; outputs held under a pending repeat escaped the retention cap; and tokens carried no
`exp`. The third was the admin's: this order said "no exp", against DL13 and JWT-SVID
section 3.3. The admin settled it for an `exp` 100 years ahead, logged as finding 3-01 in the
[log of increment 3](../proposals/first-build/README.md#settlements-register), and sent the worker back
once with all three. The second round added `request_replies` to the core's `Transition`, so
the shell delivers the core's own reply, made `expire-outputs` reach outputs held in previous
snapshots, and issued and checked the `exp`. The core's decisions did not change; every vector
still passes. The admin reran every check: 105 Rust tests pass, and clippy, fmt and the five
repository checks are clean.

Decisions, each choice now in [CONFORMANCE.md](../../engine/CONFORMANCE.md):

1. **A new crate**, `conduit-shell` with the binary `conduit`, deciding nothing the core decides;
   where it would, it stops with an embedding error.
2. **The store**: the core's whole `State` committed as one file replaced atomically, one commit
   per transition, with a format version (DL2).
3. **The loop**: one writer at a time; state and records are committed **before** any intent runs
   or any final reply returns (DL2, REC-001); a pending reply never reaches a caller as a
   success; an unserved intent is never dropped silently.
4. **Evidence** from the shell: clock, random UUIDs, revision values and suffix candidates.
5. **Recovery**: nothing dispatched before a crash is sent again (IMP-007), shown by crash tests.
6. **Retention of outputs (DL10)**, as the owner asked in the
   [first review](../proposals/first-build/to-review-with-the-owner.md#the-first-review-2026-11-10):
   expiry through the core, oldest first, past a size cap or an age; invocation state never goes.
7. **Identities (DL13)**: one signing key per store, never in state, records or logs; per instance
   a JWT-SVID with `jwks.json` and `ca.crt` in a per-run directory; verification asks whether the
   instance still exists (IFC-022, IFC-032); nothing of an identity in state, records or output
   (PRV-024, SEC-018). The order said "no `exp`"; finding 3-01 replaced it.
8. **The command line**: `conduit init`, `register`, `inspect` and `records`.

Also: (9) tests of the loop end to end on S1; (10) **the log of increment 3**, where a case the
core cannot decide is a finding and unfinished shell work is not.

### Order 021

Work order 021 (I3-02): the engine's own runner, and the unchanged filesystem server. Its Record:

Run on 2026-11-10: the worker in 49 minutes, in parallel with order 022, then a fresh verifier
in 7. The verifier found five defects, each real, all in how the runner composes a server's
command line and environment from its Package: bare program names resolved inside the package
rather than through `PATH`, symlinks with `..` refused, a fixed positional argument without
`valueHint` refused, Package environment values and defaults never applied, and server.json
variables in argument values never substituted. The admin sent the worker back once with all
five and two further decisions: the names checker gains an exact exception for the test switch
`CONDUIT_TEST_NPM`, rather than the switch being spelled by concatenation to get past it; and
the filesystem server 2.1.0 declares Node 24 or later but was qualified on Node 21.5.0, which
`CONFORMANCE.md` and the log of increment 3 now state as a limit (the runner does not check a
Package's declared engines). The admin reran every check: with `CONDUIT_TEST_NPM=1`, 122 Rust
tests pass, including S1's setup, reads, writes and history against the real server and its
refusal of a path outside the root, and clippy, fmt and the five repository checks are clean.

Decisions, each choice now in [CONFORMANCE.md](../../engine/CONFORMANCE.md). Order 020's
`Executor` answers each intent with an `Execution`; work that takes time answers `Deferred` and
completes later through `Shell::complete` or `Shell::setup_report`, never holding the lock.
(1) **Intents served**: `builtin-runner`, `dispatch`, `refresh-tools-and-call`, `cancel` and
`cancel-setup-parts`, with `provider-call` and `connect` unserved in the first build; (2) **items
the runner runs**: an npm `package` item, its SHA-256 checked first, which pins the artifact and
not its dependency graph (MAN-010), or a `command` item, any other `registryType` failing (DL16);
(3) **the arguments and the environment**, as MAN-010 binds them and exactly PRV-023's; (4) **the
process**: a private working directory and process group per run, no run outliving the engine,
the condition for `http` on loopback (PRV-022), and no restart policy (PRV-010); (5) **MCP over
stdio**, with **no** `roots` capability, so no client widens the server's directory; (6)
**dispatch**: no Conduit key, identity, configuration or revision crosses to a `plain` server
(IFC-007), and `refresh-tools-and-call` sends a read once, never a write (IFC-046); (7) **S1 for
real**, with `@modelcontextprotocol/server-filesystem` 2.1.0, when `CONDUIT_TEST_NPM` is set; (8)
findings and limits, as order 020's decision 10.

### Order 022

Work order 022 (I3-03): the server consumers connect to. Its Record:

Run on 2026-11-10, in parallel with order 021: the worker in 48 minutes, then a fresh verifier
in 5. The verifier found three defects, each real: a closed response stream did not cancel its
invocation (OPS-027), the inventory of unfinished operations skipped custom controls, and a
malformed `tools/call` with no identity was recorded as a refusal instead of getting the
protocol's error and no record (IFC-036). The admin sent the worker back with all three. Order
021 merged meanwhile, and the merge of main into this order's tree conflicted in six files; a
third round resolved them, joined `conduit serve` to the runner, and added the end-to-end test
the order asked for, run as a process on S1 with the real filesystem server: A refused a write,
B writing and reading back, no token and a forged token refused, T7's close of A's Conduit and
its explicit restoration through `conduit call` with B unaffected and sent nothing, and no run
left after the server stops. A second fresh verifier then read the joined result and found four
more: a session mismatch with an established identity returned 401 with no record (REC-001),
rejection records dropped the caller's idempotency and serving metadata (REC-002), a session
that overflowed its notification queue stopped hearing changes while staying open (IFC-038),
and loopback hosts the HTTP layer refuses were accepted at first start. A fourth round fixed
all four, the last by refusing every loopback host but `127.0.0.1`, `localhost` and `::1`.
The admin reran every check: with `CONDUIT_TEST_NPM=1`, 134 Rust tests pass, and clippy, fmt
and the five repository checks are clean.

Decisions, each choice now in [CONFORMANCE.md](../../engine/CONFORMANCE.md), with order 021's
note on the `Executor`: (1) **`conduit serve`**, Streamable HTTP at `/mcp` on loopback only, at
the address in every token's audience, stopping cleanly on a signal and ending every run; (2)
**who a request comes from (IFC-022, IFC-042)**: a verified bearer token, or with none identity
`none`; one that does not verify gets HTTP 401 and reaches no tool; identity comes from nothing
else; a foreign `Origin` gets 403, against DNS rebinding; (3) **the tools (IFC-012, IFC-033,
IFC-038)**: `tools/list` is exactly the core's `list-tools`, `list_changed` goes to that
consumer's sessions only, and no `ttlMs` is set (DL10, DL12); (4) **a call (ACC-001, ACC-036,
IFC-025)**: authority is checked on every call, never from a cached permit; a tool not presented
is a protocol error with `refused` in `error.data`; a client's cancellation becomes the core's
(OPS-027); every request has its record before its reply (REC-001); (5) **the management route**,
`conduit call` as the Interface `root` (FST-001, FST-002), and `conduit tools`; (6) **what is not
finished (H03)**: an operation an S1 identity would be shown that the core cannot carry out is a
finding, and management-only ones answer `-32603`; (7) tests with `rmcp` clients as A and B; (8)
findings and limits, as order 020's decision 10.

### Order 023

Work order 023 (I3-04): the Agent types for Claude Code and Codex, as packages of their own. Its
Record:

Run on 2026-11-10: the worker in 32 minutes, then a fresh verifier in 9. The worker reported
one finding: the order asked for an optional `model` input bound to a variable, which MAN-017
does not allow; the admin accepted its settlement, a required string with an empty default,
logged as finding 3-02. The verifier found four defects and two smaller faults, each real:
`--package` read only `conduit.yaml` and not a linked manifest; a turn whose message was `-`
reached Codex as its marker for standard input; Claude Code's repeated start-of-session tool
list was reported as current, which experiment 009 had warned against; a cancelled turn
queued behind another could still start model work; and a test assumed Cargo's default
target directory. The admin sent the worker back once with all six. The admin also updated
`AGENTS.md` and `CONTRIBUTING.md`, which this order's scope left out, to say the names check
covers `packages/`. The admin reran every check: 136 Rust tests in `reference-model/` and six
in each package pass, and clippy, fmt and the five repository checks are clean. No real client
ran; the [closeout plan](../proposals/first-build/increment-3/closeout-results-2026-11-10.md#how-to-run-the-open-checks) is for the admin.

Decisions, each now in the packages' READMEs,
[Claude Code](../../packages/agent-claude-code/README.md) and
[Codex](../../packages/agent-codex/README.md): (1) **two packages**, each a Rust crate with no path
dependency outside its directory; (2) **the Type**, `claude-code.v1` and `codex.v1`, a `command`
item running the wrapper, one operation `turn` marked `startsModelWork` (TPL-019) whose records
keep neither inputs nor outputs, and setup that starts no model work (its `model` input became
finding 3-02); (3) **the wrapper**, an MCP server over stdio presenting only `turn`, the engine its
client's only MCP server, the client in the run's private working directory (DL16); (4) **the
owner's configuration kept out**, and the API key variables removed (ruling 48, point 2); (5)
**the Agent's own reach, off by default** (ruling 48, point 1), Codex's read-only shell still
reading outside the engine, a stated route (ACC-027), not a solved problem; (6) **the token from
its file**, by each client's header helper, never in a variable, an argument or a log; (7) **one
approval**, the engine's decision the only one per call; (8) **access changes at a turn's
boundary**; (9) **build and pin**: a reproducible archive with its SHA-256, pinned copies under
`conformance/fixtures/agents/`, and `tools/set_names.py` covering `packages/`; (10) tests with a
scripted client; (11) the closeout plan, not run; (12) findings and limits, as order 020's
decision 10.

### Order 024

Work order 024 (I3-05): what the first run with the real clients found. Its Record:

Run on 2026-11-10: the worker in 32 minutes, then a fresh verifier in 5, which found nothing
to fix. The worker logged findings 3-03 to 3-05 as settled. It did not run the suite with
`CONDUIT_TEST_NPM=1`, reading this order's network grant as not covering npm; the admin ran
it. The admin also rewrapped the spec paragraphs the worker had left with long lines, without
changing their content (each YAML file loads to the same value). The admin reran every
check: 136 Rust tests in `reference-model/` with `CONDUIT_TEST_NPM=1` and six in each package
pass, and clippy, fmt and the five repository checks are clean. The admin reruns the closeout
with the real clients next.

Decisions:

1. **Finding 3-03, in the specification**: an operation that declares no outputs is presented
   with **no** `outputSchema`, and its results carry **no** `structuredContent` (IFC-025,
   IFC-033), because the protocol makes the schema optional, a client that sees one may prefer the
   structured result, and an empty map would hide the content. Item 11 of
   [the review list](../proposals/first-build/to-review-with-the-owner.md).
2. **Finding 3-04, Claude Code's wrapper**: `--setting-sources ""`.
3. **Finding 3-05, Codex's wrapper**: every feature 3-05 names disabled, and
   `project_doc_max_bytes=0`; `accountServices` enables `apps` alone.
4. **The Agent's own machine tools are off by default**: the admin extended ruling 48, point 1,
   under ruling 46, point 4, to a client's shell, code execution, browser, other Agents, hooks and
   plugins. Item 12 of the review list.

Also: (5) each package 0.1.1; (6) the closeout plan updated.

### Order 025

Work order 025 (I4-01): the operations S1's tasks of change need. Its Record:

Run on 2026-11-10: the worker in 61 minutes, then a fresh verifier in 5, which found four
defects and one weak test: unregistering a template asked for a stop meant for instances; a
further retirement could start a run for a hook that is not dispatched again (TPL-018); a
retirement on a lost node dropped its unfinished hook (PLC-027); `conduit submit` dropped the
project directory, so relative fetches and programs could not get SUB-010's semantics; and T4's
notice assertions passed with no notice sent. The admin settled each, the fourth by having
`submit` refuse relative values until the specification decides how `register.v1` carries a
directory (finding 4-03). The worker's second round took 40 minutes. A second fresh verifier
found three more faults on retire hooks: a run prepared before choosing the hook's own
implementation, a successful hook run left out of the retirement's parts, and an unknown run
state recorded as an effectful part instead of the hook's part unavailable (TPL-018, OPS-022).
None is on S1's path, which declares no retire hook; the admin merged with them logged as
finding 4-04, for a follow-up order. The admin reran every check: 147 tests with
`CONDUIT_TEST_NPM=1`, clippy, fmt and the five repository checks pass. The worker also reported
an existing runner test that can read a PID file before it is written (finding 4-05).

Decisions:

1. **`install.v1`, `retire.v1` and `unregister.v1` become public operations**: an installed
   package is usable without a restart; `retire.v1` closes the Conduits that target the instance
   at once, and those it consumes once its shutdown is confirmed; a case the core cannot decide is
   a finding, not settled.
2. **Commands for the author**, each a request of the serving engine as root: `conduit install`,
   `submit`, `replace`, `retire` and `unregister`.
3. **The filesystem package 0.2.0**, pinning server-filesystem 2.2.0.
4. **Vectors and tests**, each failing before: T4's route (IFC-012, LIF-009, IFC-038, INS-020),
   T5's (PKG-020, IFC-012, PKG-005) and retirement (CON-025).
5. **`CONFORMANCE.md`** lists what is supported and what remains.

### Order 026

Work order 026: the entry documents, read cold and brought up to date. Its Record:

Run on 2026-11-10: the worker in 9 minutes, then a fresh verifier in 2, which found one stale
statement left in scope. The worker's cold reading reported 25 findings: 16 in scope, fixed,
and 9 in documents outside it. The admin widened the scope for a second round to those nine,
with limits: in `spec/` only its README's non-normative status and contents, in `docs/AUDIT.md`
only current-status summaries and never a row of the ruling log. The admin checked that no
promise and no ruling-log row changed. The checks pass; the worker's Rust run in its sandbox
could not bind a loopback port, and no code changed.

Decisions: (1) **read cold first**, each place that misleads, loses or stops a newcomer a
finding; (2) **bring them up to date** from the records, never from memory, each state dated;
(3) **make the way in clearer** with a "Start here"; (4) **write the cold readers' instructions**,
now [cold-readers.md](../guides/cold-readers.md).

### Order 027

Work order 027 (I4-02): what the coordinator's run and order 025's review left open. Its Record:

Run on 2026-11-10: the worker in 43 minutes, then a fresh verifier in 4, which found one fault:
the missing-object diagnostic chose its input by name, not by operation. The worker's second
round fixed it. The admin reran every check: 153 tests in `reference-model/` with
`CONDUIT_TEST_NPM=1` and 6 in `packages/agent-codex`, clippy, fmt and the repository checks pass.
Decision 6 did not hold with the real client: with `multi_agent`, `multi_agent_v2` and
`multi_agent_v2_dynamic_tools` disabled, and in further trials `collaboration_modes`,
`in_app_local_automation` and `agents.max_depth=0`, Codex 0.167.0 still listed its six
`collaboration.*` tools. Finding 4-06 stays open. The trials also showed `sleep_tool` controls
`clock.sleep`, and that a trial without the wrapper's `web_search="disabled"` lists `web__run`.

The later [package settlement](../../packages/agent-codex/README.md) records finding 4-06 as
settled by the admin; the Record above says it was open at that point.

Decisions:

1. **Finding 4-04, the retire hook**: choose the hook's own implementation before preparing any
   run; record a run started for the hook as a part of the retirement, with its effect; where the
   run's state is unknown, do not dispatch the hook, and record its part unavailable, with no
   effectful part invented (TPL-018, OPS-022). Item 19 of
   [the review list](../proposals/first-build/to-review-with-the-owner.md).
2. **Finding 4-05, the runner test**: wait for the PID file's contents, not its existence.
3. **Finding 4-07, a store that cannot be repaired**: `conduit serve` starts even where a
   registered instance's setup fails, so that the commands can repair it.
4. **Finding 4-09, inspection**: `conduit inspect` asks the serving engine, and lists every Conduit
   an instance consumes.
5. **Finding 4-10, the diagnostic**: a request naming a missing object (OPS-029) says so, with
   its kind and name.
6. **Finding 4-06, Codex's own agent tools**: disable each feature that offers the six
   `collaboration.*` tools. Item 16 of the review list.

### Order 028

Work order 028 (I4-03): commands an author can rely on. Its Record:

A first run on 2026-11-10 ended after 44 minutes with no change made: the worker printed a large
fixture whole and passed the model's output limit. The admin ran the order again with a note
against printing whole fixtures. The second run took 50 minutes, then a fresh verifier 5, which
found two faults: a run stopped by the runner still showed `RunRunning`, and failures after a
session's initialization bypassed the descriptive error. The worker's second round fixed both.
The worker reported one finding, that the specification has no form for a condition that has
ended (LIF-011, OBS-002); the CLI shows such conditions apart with their end, in its own output.
The admin reran every check: 163 tests with `CONDUIT_TEST_NPM=1`, clippy, fmt and the
repository checks pass.

Decisions: (1) **exit status (4-13)**, non-zero when any printed result did not succeed;
(2) **discovery (4-14)**, `--help` for every command and errors that say what would fix them;
(3) **timeouts (4-15)**, retrying a session's initialization, never a request already sent;
(4) **stale conditions (4-16)**, an ended condition shown as past, `spec/` unchanged. Decision 5,
verbatim:

5. **S1's packages are packages like any other.** `conduit register` stops preinstalling S1's
   filesystem and history snapshots: the engine knows nothing of S1. Every test, fixture and
   README that relied on it installs them with `install` (or the library's equivalent) first. The
   conformance vectors that read S1's snapshots from fixtures keep doing so, as fixtures.

### Order 029

Work order 029: the documents after rulings 49 to 54. Its Record:

Run on 2026-11-10: the worker in 14 minutes, then a fresh verifier in 4, which found three stale
statements. The admin widened the scope for a second round to those, three outside-scope findings
of the worker's own, the removal of the rule that ruling 57 revoked, and the mention of rulings 55
to 59. The worker's links to the newest ruling records failed in its tree, which predated them, and
resolved after the rebase; its Rust run could not bind loopback ports in its sandbox, and no code
changed.

Decisions: (1) **`spec/README.md`, its status only**: rulings 49 to 54 accepted and not yet
written; (2) **the Codex Agent's stated routes**: finding 4-06 settled as a route to more parallel
model work within the Agent's own access, not around it, and finding 3-07 kept; (3) **indexes**;
(4) **a staleness pass** for rulings 49 to 54 and orders 025 to 028.

### Order 030

Work order 030: the checks and code dependencies the condensation needs. Its Record:

Run on 2026-11-10: the worker in 22 minutes, then a fresh verifier in 7, which found two faults:
Markdown autolinks bypassed the permalink check, and the copied safety traces ended with an extra
blank line. The new checks also found eight existing problems: six GitHub line-range anchors in the
archive, which the second round accepts when the lines exist, and two stale paths in tool comments.
The worker reported finding WO030-F1: the manifest's `safety_rows` tags take 21 values, of which
only four are trace names of S1's proposed safety traces, so what counts as a safety vector is not
yet decided. The admin reran the Rust suite outside the sandbox: 163 tests pass; the five
repository checks and the tools' unit tests pass.

Decisions: (1) **`tools/check_schemas.py`, a new step**: seven fields per promise, allowed layer
and level, unique IDs, no retired ID reused; (2) **`tools/check_docs.py`, permalinks**, checked
at their commit; (3) **a path-mention check**; (4) **the frozen S1 set**,
`conformance/s1-promises.yaml`, the 308 promise IDs of increment 2's plan; (5) **S1's proposed
safety traces**, copied verbatim to [s1-safety-cases.md](../../conformance/s1-safety-cases.md),
not as the definition of `safety_rows`: the manifest uses about 21 tag values, only four of them
trace names, so what counts as a safety vector is not decided (finding WO030-F1).

### Order 031

Work order 031: a spike of the short form's translation, on S1. Its Record:

Run on 2026-11-10: the worker in 14 minutes, then a fresh verifier in 4, which found nothing to fix
and reproduced the results. The exact translation is refused by today's kernel because it emits
`as`, one of ruling 53's additions not yet in the specification (finding 031-01); with `as` dropped,
all fourteen states register. In flow style S1 costs 55 lines on paper against the baseline's 64,
better on two tasks and worse on T1 and T7; in block style 97. The admin reran the spike's tests:
9 pass.

Decisions, the method in the experiment's
[README](../../experiments/011-short-form-spike/README.md): (1) grants refer to Conduit templates
for reuse (`from: read`), and operation names stay fully qualified (ruling 25); (2) a reference
names its kind (ruling 19 is kept); (3) the frame is optional, the version from the manifest;
(4) names stay stable: a replacement retires the old object and makes the new one under the same
name; (5) no `fleet:` key, since rulings 59 to 62 are not in today's core; (6) **a new
experiment**, its counting rule stated before the results; (7) **a translator**, a spike; (8)
**S1's states in the short form**, in flow and block style; (9) **check each state** through
today's loader and core; (10) **count** against the baseline and the coordinator's run, a
spike's paper count not a run (ruling 45); (11) **the register**: row A79.

### Order 032

Work order 032: the specification without its change narrative. Its Record:

Run on 2026-11-10: the worker in 30 minutes, then a fresh verifier in 7. The worker condensed 153
YAML head comments and the five Markdown files: `spec/` went from 219,540 words to 183,415, its head
comments from 47,053 to 19,907. Every YAML file loads to the same data, all 455 promises and 15 DL
records are unchanged, and the 74 retired IDs named at C0 stay named. It kept two passages of
`spec/promises/secrets.yaml`'s head that no promise holds, as the order asked: that a file store
ships in the standard library and a store reading the engine's own variables is not provided
(finding 032-G1), and what a store package's contract states (032-G2); they describe current
content and stay until a change to secrets writes them as promises. The verifier found that
`tools/check_schemas.py` did not read the new register form "OLD: nothing replaces it", so 20
retired IDs had lost their reuse check (032-G3), and one broken sentence in `spec/definitions.md`.
The admin fixed both, with a regression test, which brought the recognized retired IDs back to 72,
and reran the Rust suite outside the sandbox: 163 tests pass, 3 ignored.

Decisions: (1) only `spec/`'s comments and five Markdown files change; (2) **`spec/README.md`**:
"History" becomes "Where the reasons are"; (3) **every YAML head comment under `spec/`** keeps
current content, open issues, "Standards followed" and a register of removed IDs, and carries no
change narrative; (4) **gaps, not deletions**: a dropped sentence stating a behavior no promise
holds is a finding, kept until the admin decides; (5) **gates, each run by script**: equal YAML
data, unchanged promise text, every retired ID still named; (6) no digest or fixture regenerated.

### Order 033

Work order 033: `reference-model/` becomes `engine/`. Its Record:

Run on 2026-11-10: the worker in 21 minutes, then a fresh verifier in 5. The directory moved with its
73 files; every path, link, tool and test that named it follows, and the live documents call it the
engine. The verifier found that the readiness conditions of A42 and A59 had become "the engine and
the vectors agree with the promises" where ruling 46's pure core is meant; the admin restored
"core". It also found three Python bytecode files tracked since order 032's merge, holding local
paths: the admin removed them from the tree and ignored `__pycache__/`. Continuous integration
merged while the order ran, so the admin renamed its paths too. Records under `docs/proposals/`,
`docs/archive/` and `docs/work-orders/` keep their words, with links repointed. The admin reran the
suites outside the sandbox: 163 tests pass in `engine/`, 3 ignored, and 9 in experiment 011.

Decisions: (1) no promise, schema, expected outcome, fixture or digest changes; (2) **move the
directory** with `git mv`, crate and binary names kept; (3) **every path that names it** follows;
(4) **the live documents' word**: "the engine", or "the reference engine" in ruling 47's sense,
and "core" where ruling 46's pure core is meant; (5) **records keep their words**; (6) **find what
is left**; (7) [ruling 64](../proposals/ruling-64/README.md), point 4: the rename, because ruling
47 makes v0 the reference engine, which that directory holds.

### Order 034

Work order 034: the ledger of rulings, the condensed tracker, and records that say whether they
hold. Its Record:

Run on 2026-11-10 and 2026-11-11, in parallel with orders 033 and 035: the worker in 22 minutes,
then a fresh verifier in 7. The worker wrote the ledger of 67 rulings with its appendices,
condensed `docs/AUDIT.md` from 24,882 words to 8,414, gave 49 records their status lines, and added
`tools/check_rulings.py` with its tests; 17 extractions compared equal to their C0 sources. The
verifier found that three directive statuses named work orders instead of the action item that
carries them, and one history row mislabelled round 2's prefixes. The admin added action item A80
for the rest of the condensation plan, marked ruling 64's point 4 done (order 033 merged) and ruling
67's directives done with this order and 035, made the check require an action item on a
directive, and corrected the row. The worker also found that the plan's A79 had since gone to the
short-form spike (finding 034-F1); the lean-vector work stays under A39. The admin applied order
033's rename to this order's text and added the new check to continuous integration at merge.

Decisions: (1) the admin's draft ledger is verified row by row; (2) the plan's "ruling 57", for
the acceptance of earlier logged decisions, is ruling 58, point 1, and **C0** is
`f1cb473493fbb7b8667686d6fdf639ae96579ec3`; (3) **the ledger** in ruling 67's form, the owner's
words verbatim and whole when 40 words or fewer, otherwise "In the record", never excerpted;
(4) **`docs/AUDIT.md`** condensed, every linked heading kept; (5) **status lines** on every
ruling record; (6) **the check**, `tools/check_rulings.py`.

### Order 035

Work order 035: the contributor rules, the readers' guides, the roadmap and the indexes. Its Record:

Run on 2026-11-10, in parallel with orders 033 and 034: the worker in 10 minutes, then a fresh
verifier in 5, which passed it with one nit: the protocol watch list copied into the cold readers'
guide referred to verdicts and rows of its source report, so the admin named that report. The
worker reported that `docs/implementation/WORK_ORDERS.md` still gave A78's old trigger (finding
035-01); the admin corrected it to ruling 61's (B12). The ledger is named as a path until order
034 merges; the rename of order 033 was applied to this order's text at merge.

Decisions: (1) ruling numbers as order 034 reads them; (2) **E19, `CONTRIBUTING.md`**, with the
head-comment rule (ruling 58, point 6) and ruling 67's rule for a new ruling; (3) **E3, the cold
readers' guide**; (4) **E24's one-line notes**; (5) **E33, `ROADMAP.md`**, including A78's trigger
as ruling 61 (B12) set it, and that the independent builds inform the owner's judgment without
gating v0 (ruling 47); (6) **`docs/README.md`** becomes a map; (7) **ruling 63's counting rules**
go into the measurement plan.

### Order 036a

Work order 036a: spec batch 1, the Fleet family and the access additions. Its Record:

Run on 2026-11-11: the worker in 22 minutes, then a fresh verifier in 10, then a second round. The
worker added 15 promises (FLEET-001 to 006, ACC-044, ACC-053 to 055, ACC-057, LIF-040, LIF-041,
OPS-036, FST-007; the plan's "16" counted one twice, finding 036a-01) and amended 44, with DL1, DL6
and DL13, the definitions, `spec/README.md`'s contents and limits, and IN_FORCE.md's lines. It
listed the vector cases the new text contradicts, about 144, for the build orders (findings 036a-02
and 036a-03): ownerless Agents, registrations without root's defaults, and Agents routed through
`management`. The verifier found that root's default Conduits, opened at registration, contradicted
SUB-006, INS-016 and the definition of registration; that `list.v1`'s package visibility cited a
retired promise; that ACC-055 limited an actor where ruling 66 limits the holder, which may be a
Resource; and that the definition of inspect missed the inventory. The second round fixed all four.
The admin reran the engine's suite outside the sandbox: 163 tests pass, 3 ignored.

Decisions: (1) **write 036a as the plan states it**: the Fleet family (FLEET-001 to 006), the
access additions (ACC-044, ACC-053 to 055, ACC-057, ACC-033 whole), LIF-040 and LIF-041;
(2) **the plan's drafts are drafts**: where one and a ruling's record differ, the record governs;
(3) **IDs** from the plan's C1, checked against the registers of removed IDs; (4) **what this
order does not write**, left to 036b, 037, 038a, 038b and batch 2; (5) **IN_FORCE.md**: a *spec
pending* line written in full becomes a pointer to its promises, one written in part says what
remains; (6) **the vectors and the engine do not change**, and contradicted vectors are listed.

### Order 036b

Work order 036b: spec batch 1, the core package and schemas for 036a. Its Record:

Run on 2026-11-11: the worker in 29 minutes, then a fresh verifier in 7, then a second round. The
worker landed 15 new exports of the core package (among them `fleet.v1`, `fleet-management.v1`,
`list.v1`, `retire-with-agents.v1`, `lift.v1`, the templates of the plan's C8 without `answer` and
`sign-in`, and the schemas inventory, tombstone, provenance, fleet-reference and trust-domain) and
staged, as exact patches, what the engine would reject today, for orders 039, 040 and 041a, with
handoffs for the enrollment orders to 038b; applied together in a scratch tree, the staged text
passes the schema and document checks. It listed the vector steps the staged contracts will change
(166 owner-input steps in 161 cases) for the build orders. The verifier found that the reconcile
contract dropped ruling 66's allowance for a Fleet's Agents, that a tombstone did not require its
owner, one stale marker in IN_FORCE.md, and trailing white space in the staged patches; the second
round fixed all four, keeping the patches exact as `.patch` files exempt from the white-space check.
The admin reran the engine's suite outside the sandbox: 163 tests pass, 3 ignored.

Decisions: (1) the plan's drafts are drafts, and IN_FORCE.md changes as order 036a's decision 5
says; (2) **write 036b as the plan's work-order section states it**: `fleet.v1`,
`fleet-management.v1`, `list.v1`, `retire-with-agents.v1`, `lift.v1`, C8's templates without
`answer` and `sign-in`, and the schemas it names; (3) **staging (C25)**: what the engine would
reject today goes verbatim into [036b-staged/](../work-orders/036b-staged/README.md); (4)
**citations**: every cited promise exists; (5) only edits that keep the engine's tests green land.

### Order 036k

Work order 036k: a report of what changed in the vectors. Its Record:

Run on 2026-11-11: the worker in 8 minutes, then a fresh verifier in 6. The worker wrote
`tools/vector_delta.py` with 18 tests: it pairs cases by `id` and steps by position, reports flips
of reason, outcome, acceptance, tool and `stateUnchanged`, every other change by JSON Pointer, and
fixture changes by hash, and fails on any flip or undeclared change. Today's tree shows no change;
order 033 changed three fixture paths and no expectation. The verifier found that its YAML fallback
used YAML 1.1's scalars, so `no` and `off` both read as false and a changed tool description passed
unseen; the admin gave it YAML 1.2's core schema, as the engine and `check_schemas.py` read, with a
regression test.

Decisions, the tool's use now in
[reviewing vector changes](../../conformance/README.md#reviewing-vector-changes):
(1) **`tools/vector_delta.py`** reports every **flip** of `reason`, `outcome`, acceptance, `tool`
or `stateUnchanged`, and every other change; (2) **declared shapes**: it exits 1 on any flip or
undeclared change; (3) **tests**; (4) **today's tree**; (5) no expectation may be regenerated
from engine output: a changed outcome must be meant.

### Order 060

Work order 060: the re-audit's working papers leave the tree. Its Record:

Run on 2026-11-11: the worker in 16 minutes, then a fresh verifier in 6. The worker removed the 57
files of removal list A, keeping 39,919 of the re-audit's 255,446 words: the round records, their
decisions and the "Choices that set behavior" extracted from the editors' logs (E35, 68 lines), with
every removed path linked at C0 and listed in the archive's index. It stopped short of three links
in the ledger, which the order had put out of its scope, and prepared a patch that changes only
their targets (finding WO060-01). The verifier found one choice the extraction missed, M75's dropped
wording and its reason; the admin added it and applied the ledger patch, mending two targets the
patch had joined without a slash.

Decision 1, which orders 061 to 066 share: the plan predates later work, so extract only what
orders 034 and 035 did not; the plan's line numbers are those of **C0**.

Decision 2, **extract first**: E4 into round 6's "For the owner"; E5's condensations of the
re-audit's records; E35's "Choices that set behavior", one line per choice that names a rejected
alternative, a candidate for later or a reading a promise does not state.

Decisions 3 to 6, which orders 061 to 066 share:

3. **Then remove** only the files the list names, each unchanged since C0, or stop and report.
   From order 061 on, a file changed only in links or path mentions that earlier orders repointed
   may go; any other change stops that file, and the worker goes on with the others.
4. **Repoint every link** into a removed file: to the new home of moved text; to a live
   destination for a sentence on future work, listed for the admin; otherwise to its C0 permalink.
   From order 061 on, path mentions too, and links in the ledger and in records, whose targets may
   change and words may not; from order 064 on, mentions by file name, section name and row ID.
5. **The archive's index**: one row per removed directory or group.
6. **Report** the words removed and kept, the extractions, every link repointed by class, and a
   `git grep` of each removed path showing only C0 permalinks and archive rows.

### Order 061

Work order 061: the logs of rulings 38, 41, 42 and 43 leave the tree. Its Record:

Run on 2026-11-11. The first dispatch stopped at the order's guard, as written: 27 of the 30 files
to remove had changed since C0, only in links that orders 033 and 060 repointed; the admin let
files whose only changes are repointed links go, and dispatched it again. The worker then took 21
minutes and a fresh verifier 5. It removed 30 files, the logs of rulings 38, 41, 42 and 43, for a net
100,954 fewer words, with ruling 41's planner's choices, its open decisions condensed in place and
each ruling's "Choices that set behavior" kept. The verifier found one choice missed (F3's rejected
option (b) and its reason, in ruling 42's integration log) and two links that pointed at an
extraction where the history meant the removed text itself; the admin added the choice and linked
both to C0. The shell's loopback tests could not run in the worker's sandbox; no code changed.

Decisions 1 and 3 to 6 are as [order 060](#order-060) sets them out. Decision 2, **extract first**:
ruling 41's planner's choices P1 to P20 (E16); `ruling-41/open-decisions.md` condensed in place,
"For the owner" verbatim (E17); E35 for rulings 41, 42 and 43; MX-21 of ruling 43 into the ledger's
appendix.

### Order 062

Work order 062: the batch records' logs leave the tree, and the older records condense. Its Record:

Run on 2026-11-11. The first dispatch stopped at the order's guard, as written, on five files whose
only changes since C0 were links repointed by earlier orders; the admin let such files go and
dispatched it again. The worker then took 14 minutes and a fresh verifier 6, then a second round.
It removed eight files, the logs of `core-and-schemas/` and `docs-audit/`, condensed the secrets
record and the older records in place by deleting sections only, and moved the independent builds'
assessment of 2026-11-06 verbatim into ruling 39's record, replacing it with the state now, the
coordinator's until G11: about 42,700 fewer words. The verifier found that the new G9 required both
builds to run before the gate let them start, and that bare file names and section references to
removed text had been missed; the second round removed the circular condition and linked 399
references to their C0 text, the records' words unchanged.

Decisions 1 and 3 to 6 are as [order 060](#order-060) sets them out. Decision 2, **extract first**:
E35 for `core-and-schemas/` and `docs-audit/`; E20's spot-check of their 22 "left undone" items; the
secrets record condensed in place, each drafted SEC-n mapped to `spec/promises/secrets.yaml` or the
dropped list (E18); E20 for thirteen older records; the independent builds' assessment of 2026-11-06
moved verbatim into ruling 39's record and replaced by the state now, the coordinator's until G11
(E31; ruling 58, point 7).

### Order 063

Work order 063: the first build's closed increments condense. Its Record:

Run on 2026-11-11: the worker in 20 minutes, then a fresh verifier in 5. The worker removed 16 files,
49,269 words, keeping 22,853 in the first build's records: the logs of increments 1 to 3 became one
settlements register (33 findings, with D19's caveat, the sentinel method, the evidence of vector
strength and the rejected alternatives verbatim), the closeout gained "How to run the open checks",
`engine/README.md` gained "The core's boundary", rebuilt from the code, and `conformance/README.md`
its planned work. One path stays inside an invalid example's JSON value, which is data and not a
link. The verifier found three C0 links whose line numbers had become part of the file name, an
instruction for future findings that pointed at the closed register instead of the open increment's
log, and two historical references to the handoff sent to its rebuilt summary; the admin fixed all
of them. The shell's loopback tests could not run in the sandbox; no code changed.

Decisions 1 and 3 to 6 are as [order 060](#order-060) sets them out. Decision 1 adds that lean
vectors go under A39 (034-F1). Decision 2, **extract first** (E6 to E10, E12, E13): the [settlements
register](../proposals/first-build/README.md#settlements-register), D19's caveat verbatim; "How to
run the open checks", the sentinel method verbatim; "The core's boundary", rebuilt from the code;
`conformance/README.md`'s "Planned work".

### Order 064

Work order 064: the language-shape, simplification and Project papers leave the tree. Its Record:

Run on 2026-11-11: the worker in 14 minutes, then a fresh verifier in 5, then a second round. The
worker removed 31 papers, the simplification's research, the short form's designs and inventories,
and the Project investigations' papers, a net 270,808 fewer words, after extracting E21 into the
simplification's record, E22 into the short form's synthesis and what the Projects brief lacked of
E23. The verifier found four passages copied verbatim that had lost a space, and row IDs, such as
PRJ-010 and AUT-018, still pointing at definitions only the removed papers held; the second round
restored the spaces, compared all 31 extracted passages with C0 by script, and linked 47 row
references to their C0 definitions. The short form's synthesis and critic stay until spec order 037
lands.

Decisions 1 and 3 to 6 are as [order 060](#order-060) sets them out. Decision 1 keeps the short
form's `synthesis.md` and `critic.md` for spec order 037. Decision 2, **extract first**: E21 into
the simplification's record ("Rejected, and why" verbatim, the load-bearing guarantees, the
v1-candidate reductions with their costs); E22 into the short form's synthesis; E23's provenance
quotes and synthesis decisions into the [Projects brief](../proposals/projects/brief.md), where
missing.

### Order 065

Work order 065: the research notes and the archive condense. Its Record:

Run on 2026-11-11: the worker in 12 minutes, then a fresh verifier in 5. The worker removed 35 files,
a net 108,381 fewer words, from `docs/research/` and `docs/archive/`, with the migration checker and
its lines: the archive's index now says what each removed group was and what it is source material
for, carries FORMATS' paragraph, the collaborator proposals' "not adopted, and why" verbatim and the
prior art still applied; the plan-audit history keeps what E26 keeps; `docs/LANGUAGE_DEFECTS.md`
keeps its open rows in full and a crosswalk line for each other. The verifier found two statements
to correct: the count of promises the manifest's cases cite, which counted ranges by their ends
only (100, not 76), and "labels and selectors were not adopted", which read against PLC-007's node
selectors; the admin corrected both.

Decisions 1 and 3 to 6 are as [order 060](#order-060) sets them out. Decision 2, **extract first**
(E25 to E27): the archive's index, with the collaborator proposals' "Not adopted, and why" verbatim;
the plan-audit history condensed; `docs/LANGUAGE_DEFECTS.md` keeping its open rows in full and a
crosswalk line for each other; `docs/research/SIMPLIFICATION.md` as written.

### Order 066

Work order 066: the experiments keep their method and results, and shed raw records. Its Record:

Run on 2026-11-11: the worker in 12 minutes, then a fresh verifier in 2, which found nothing to fix.
The worker removed 942 files, 148,612 words: the raw run records of experiments 005, 007, 008 and
009, and the harness crates of 008 and 009, which the engine's shell and the Agent packages
superseded, so they leave continuous integration's matrix. Each experiment keeps its README, dated
results, HARNESS.md and evidence READMEs, 38 files and 46,100 words, with links into what left
pointing at C0; `experiments/README.md` gained "Evidence for v0" and the raw-records rule. Experiment
011 was not touched. One package test fixture's `source` note now names its C0 path; the admin ran
that package's tests: 6 pass.

Decisions 1 and 3 to 6 are as [order 060](#order-060) sets them out. Decision 2, **extract first**
(E28, E29): each experiment keeps its README, dated results, HARNESS.md, evidence READMEs and
counted artifacts; `experiments/README.md` gains "Evidence for v0" (A75) and the raw-records rule;
experiment 011 and experiment 010's dated results stay untouched; a file a live proposal cites goes
only with its citation repointed to C0.
