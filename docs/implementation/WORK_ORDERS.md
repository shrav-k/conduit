# Work orders: the admin session and Codex

Not binding. The specification in [spec/](../../spec/README.md) alone binds. This page says how
the work on Conduit is divided between the coordinator's session and Codex sessions, as
[ruling 48](../proposals/ruling-48/README.md) asks, and what the first round with real agents
taught about doing it without a person present.

## Roles

| Who | Does | Does not |
| --- | --- | --- |
| The owner | Rules: on behavior, on what Conduit is for, on the order of the work. Can strike any decision of the coordinator | |
| The admin session, the coordinator (Claude Code) | Plans the work and writes each order: what it applies, its scope, its done-when, its report. Makes the decisions an order applies, under rulings 37, 38 and 46, point 4, each logged and strikeable. Reviews what comes back, runs the checks, commits, opens and merges the pull request. Puts to the owner what is the owner's | Spend its own session on work that an order can state |
| A Codex worker | Carries out one order in a working tree of its own: a build increment, an experiment, a sweep of the promises, a verification of another order's result, or a fresh read | Commit, push or open a pull request. Decide what the order leaves to the admin: it reports instead |

What goes to Codex is work whose result the admin can check: a done-when that the checks show,
counts that cite their evidence, or findings that cite the text. What stays with the admin is
whatever chooses: a decision on a finding, a measure or its counting rule, what a result
means, and what goes to the owner.

## An order

An order is a file `docs/work-orders/NNN-name.md`, written before it runs, as ruling 45's rule
asks of an experiment. It holds:

1. **What and why**: the goal, in two sentences, and the records it comes from.
2. **The decisions it applies**, each with its source. A worker applies them and does not
   reopen them. Where one cannot be applied as written, it applies the nearest sound reading
   and says so in its report.
3. **What to read**: the files and the promises, by path and ID.
4. **Scope**: the files it may change. Anything else it needs changed goes in its report.
5. **Done when**: the checks that must pass, each a command.
6. **The report**: a JSON Schema, given to Codex with `--output-schema`, so that what comes
   back is a value the admin reads, not prose. Every order's report says, at least, what it
   changed, what it could not do, which choices it made itself, and each check's last line.
7. **Rules**: those of [AGENTS.md](../../AGENTS.md#if-you-were-given-a-work-order), which every
   Codex session in this repository reads, and any of the order's own.

## Dispatch

One order, one working tree on a branch `wo/NNN-name` from `main`, outside the repository's
own checkout. The admin starts Codex headless in it:

```sh
CARGO_NET_OFFLINE=true codex exec \
  --ignore-user-config --strict-config --disable apps \
  -s workspace-write --add-dir "$HOME/.cargo" \
  -C <tree> -c 'model_reasoning_effort="high"' \
  --output-schema <order>.schema.json -o <report>.json --json \
  - < docs/work-orders/NNN-name.md > <events>.jsonl
```

Each part is there for a reason that this round found:

- `--ignore-user-config` keeps the owner's own Codex configuration out, and its MCP servers.
  Codex still signs in with the owner's subscription (ruling 48, point 2).
- `--strict-config` stops at a key that Codex does not know. A70 lost a task to an override
  that Codex accepted without a warning and ignored (mistake 2 of its results).
- `--disable apps` turns off the apps of the ChatGPT login (ruling 48, point 1).
- `workspace-write` lets Codex write in its tree and nowhere else, with no network. Cargo's
  own cache is the one other place it writes, and with `CARGO_NET_OFFLINE` every crate comes
  from that cache.
- The order goes in on standard input (`-`), so that Codex never waits on an open input: one
  of A70's runs did, for five minutes.
- `-o` and `--output-schema` give the report as a file; `--json` keeps every event, which the
  admin reads only when a report needs explaining. Codex's events do not show every command it
  runs (A70, surprise 7), so a claim that matters is checked against the files, not the
  events.

## Checking what comes back

1. **A verifier**: a second, fresh Codex session, in a read-only sandbox, with the order, the
   diff and a schema of its own. It tries to refute each change against the decisions and the
   texts they cite. A reviewer reading cold found the most consequential faults of this round:
   a contradiction across nine promises, and a wrong claim about Codex in the coordinator's own
   notes.
2. **The admin**: runs every check itself, reads the diff and the verifier's findings, scans
   for private paths and names, and decides what to fix, to reorder or to put to the owner.
3. **One more round at most**: the admin sends the worker back once with the verifier's
   findings it accepts. If that does not settle them, the admin finishes the work itself.

## Fresh reads

A fresh read is an order whose worker sees only what the order names: a working tree of the
build base, or of `spec/` alone, and no history of rulings. Ruling 48, point 4 gives Codex
the fresh author of increment 4. A fresh reader's report lists where it had to guess.

## What this round taught

From A70, increment 1 and the review of both:

- **Brief in decisions, not in hopes.** Each order states the decisions, and the worker's
  report states its own choices apart, so that the admin sees every choice made.
- **A skeptic per angle pays for itself.** Every review of this round changed a result: the
  loader's three reviewers found four defects, A70's auditor corrected six counts, and the
  last review found two of the coordinator's own errors.
- **Confirm a connection with a real call.** Codex's debug output hides MCP tools, and its
  configuration can fail silently: A4, A71 and increment 3 confirm each Agent's connection by
  a tool call that the server records.
- **Keep the owner's machine out.** No order edits the owner's agent configuration or reads a
  credential. A worker's own state, such as Codex's sessions, is the agent's.
- **Close the input, pin the flags, write the report to a file.** The dispatch above is the
  sum of this round's harness faults. Two orders started at the same moment once raced for
  Git's configuration while making their working trees; `git worktree add --no-track` avoids
  it.
- **A test that binds a loopback port needs the network flag.** Codex's sandbox without
  network access refuses to bind one, so an order whose tests run a local server has it on;
  order 010's second round met this.
- **The admin runs what a sandbox cannot.** Real clients, and the harness against them, run
  from the admin session; when such a run shows a harness fault, the admin fixes it, says so in
  the order's record, and has the worker test the fix (orders 002 and 010).
- **Pre-register before the harness, amend before the run.** Order 002's review found that the
  pre-registration of experiment 008 named tools otherwise than IFC-012 and left its matrix
  open; it was amended, with a dated note, before any run.
- **Count the coordinator's own choices as risks.** Counting rules fixed after a result is
  seen are the easiest to challenge, so the order that sets them says so.

- **Keep durable work outside session scratch space.** Order 019's first run was lost before
  it was committed; its Record reports the rerun, not a recovered first result.
- **Never print a whole large file.** Order 028 printed a fixture whole and passed the model's
  output limit after 44 minutes with no change; use `head`, `tail`, `grep` or a count.
- **Parallel edits need a joining round.** Orders 021 and 022 conflicted in six files; 022
  needed four rounds and a second fresh verifier after the joined end-to-end test.
- **Read a grant against the requested check.** Order 024 misread its npm grant; the admin ran
  `CONDUIT_TEST_NPM=1` and recorded that the suite passed.
- **Verify the admin's own decisions.** Order 020's "no exp" contradicted DL13; order 009's
  tarball did not carry the licence the admin had claimed it did.
- **A verifier checks build and results orders alike.** Verifiers found must-level faults in
  most build orders and none in the results orders (005, 008 and 011; 011 had a nit).
- **Check each finding against the promise that decides it.** Order 012's admin required
  logging any undecided case rather than inventing its answer; each reported case was decided.

Order 028, decision 5, verbatim:

5. **S1's packages are packages like any other.** `conduit register` stops preinstalling S1's
   filesystem and history snapshots: the engine knows nothing of S1. Every test, fixture and
   README that relied on it installs them with `install` (or the library's equivalent) first. The
   conformance vectors that read S1's snapshots from fixtures keep doing so, as fixtures.

Order 027, the real-client observation in its Record, verbatim:

Decision 6 did not hold with the real client: with `multi_agent`, `multi_agent_v2` and
`multi_agent_v2_dynamic_tools` disabled, and in further trials `collaboration_modes`,
`in_app_local_automation` and `agents.max_depth=0`, Codex 0.167.0 still listed its six
`collaboration.*` tools. Finding 4-06 stays open. The trials also showed `sleep_tool` controls
`clock.sleep`, and that a trial without the wrapper's `web_search="disabled"` lists `web__run`.

The later [package settlement](../../packages/agent-codex/README.md) records finding 4-06
as settled by the admin; the preserved Record below still says it was open at that point.

## Running the orders through Conduit

Once Fleet authority is built (the second specification batch, after the S1 rerun; ruling 61,
B12), the orders are to go through Conduit itself (A78): each worker is an Agent of the
`conduit-core` Fleet, root sets up fixed worker and tree slots once, and a coordinator Agent that
root appoints dispatches later orders. Each order's working tree is a directory Resource, the queue of orders and their reports is the message history,
and each worker is an Agent that the engine runs with a Conduit to its own tree that lists
reads and writes, and one to `main` that lists reads. The admin opens a worker's Conduits when
it dispatches the order and closes them when it merges. That is S1 in daily use, so each
friction is a finding of the first build, logged as one. When S2 runs, opening the pull
request moves behind a Conduit to GitHub too, and the token for it stays with the engine
rather than going to any worker.

## The record

Merged orders 001 to 036k and 060 to 066 have one row each below. The admin's own time was
not recorded. Orders 001 to 004 had no per-file Record: their original rows here are preserved
below as their record. Every later result comes only from that order's Record, preserved in
full below its row. Missing facts are marked "not recorded"; unlabelled faults are not assigned
a must, should or nit severity. A stated second round does not supply its unrecorded duration.
The separate decisions are the order's instructions, not evidence of what ran or passed.

An order in flight keeps its Record in its own file to avoid parallel edits here. At merge,
the admin writes its row serially, checks that the row and its linked detail carry its Record
and decisions, then removes the order and its own schema. The shared report schema, verifier,
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

Work order 001: the review's fixes, and the tokens of a pattern

Original record (no per-file Record was written):

- **Order**: 001
- **What**: The coordinator's fixes from the review of increment 1 and A70, and ruling 48, point 3
- **Worker**: 21 minutes; every decision applied as written; 36 files; one point left to the admin
- **Verifier**: 5 minutes; one `must`: `check_schemas.py` crashed on a repeat count past Python's limit
- **What the admin changed**: Fixed the crash, with a regression case; settled the point left to it, SCH-015's rule for a reference to the wrong kind
- **State**: done on 2026-11-09

Own decisions (as written; the Record above states corrections):

Each is the coordinator's decision under ruling 46, point 4, unless it says ruling 48. Do not
reopen them.

**A. A rejection by a schema cites SUB-005 everywhere (completes finding 1-01).** SUB-005 now says that a document or manifest a Schema contract rejects is rejected under SUB-005. At least nine promises still name another rule for a case that a schema decides: CON-020, POOL-009, POOL-012, MAN-012, MAN-007, CTR-008, OBJ-015, OBJ-019 and TPL-001. Search all of `spec/` for every other "rejected under X" (and like wording) where a Schema contract makes the rejection, and list each in your report. Reword each so the requirement stays and the rule it is reported under is SUB-005, for instance "is rejected by `declared-conduit.v1`, under SUB-005"; keep any rule that a schema does not decide. Extend SUB-005 so that a document of a package rejected by a schema when the package is installed is reported under SUB-005 too, with the request reporting invalid under PKG-023, consistently with MAN-011. Add to SUB-005's observable that the diagnostic carries the failing keyword's location as JSON Schema 2020-12's output format names it (`keywordLocation`, and `absoluteKeywordLocation` where known) in the `properties` of its SARIF result; check `diagnostic.v1` under `spec/`, and if it does not allow that, allow it there in SARIF's own form. Have the loader's `Diagnostic` carry that location for a schema rejection, with a test. In the log, say that the thesis "the core cites a promise for every decision" is met for the rules a schema encodes through SUB-005 and the keyword's location.
**B. A byte order mark in JSON is accepted and ignored (corrects finding 1-05).** RFC 8259, section 8.1, lets a parser ignore one, and files saved on some systems begin with one. Change the profile's Encoding row so that a file in either syntax may begin with one, and so that UTF-8 is stated as the profile's own rule, citing I-JSON as its precedent, which removes the clash with the Value model row's "Two of its rules, and no other". Change SUB-005's wording if it repeats the old rule. The loader and `check_schemas.py` skip a leading byte order mark in a JSON file. Add tests for both.
**C. SUB-011 gets a floor and a wider reach (corrects finding 1-11).** Every engine accepts values nested at least 64 levels deep; the loader's limit stays 128. Say it in SUB-011 or in DL10, and keep every example and vector within it. Limits on a file's size, on the number of documents or files in a submission, and on a package's archive also fall under SUB-011; at installation the request reports invalid under PKG-023 with that diagnostic. Narrow "a limit that the conformance document does not state is not applied" so that it does not reach the exhaustion of storage, which DL10 leaves to the engine.
**D. A pattern keeps to JSON Schema's interoperable tokens (ruling 48, point 3; settles 1-04).** Amend SCH-016: a `pattern`, and a key of `patternProperties`, uses only the tokens that JSON Schema 2020-12, core, section 6.4, asks schema authors to keep to, quoted from the standard, plus a backslash before a character that has a meaning in a pattern, so that it stands for itself. It is still read as ECMA-262 in Unicode mode. A pattern that uses anything else is rejected as finding 1-03 settled. Then: - Rewrite every pattern in the specification's own schemas (under `spec/schemas/`, `spec/core/` and anywhere else under `spec/`) that uses anything else, so that it keeps to the tokens. The review found lookaheads in `address.v1`, `qualified-reference.v1`, `destination.v1` and three places in `templates.yaml`; rewrite each with `allOf` and `not` and a second pattern where that says the same. It found `[^\s]` in `server-package.v1` and `server-remote.v1`; write out ECMA-262's white space and line terminators in the class instead, exactly. Search for every other `\d`, `\w`, `\s`, `\b`, `.`, lookaround, back-reference, named group or `\p{..}`. - No verdict may change. Add a test to the loader that matches each old pattern and its rewrite, both with `regress`, against every string value in the examples and against generated strings (a few thousand per pattern, including the edge cases of each), and requires the same answer. - The loader checks that an authored pattern keeps to the tokens, replacing its checks of ECMA-262's later editions; keep `regress` for matching. `check_schemas.py`'s rule `ecma-pattern` checks the same token list, by a small tokenizer, and keeps `ecma()` for the meaning of `$` and of any class. - Vectors: keep `invalid-by-rule/SCH-016-pattern-that-is-not-ecma-262.yaml`, and add `invalid-by-rule/SCH-016-pattern-outside-the-interoperable-tokens.yaml`, whose pattern is valid ECMA-262 but uses a lookahead. Both the loader and the tool reject it. - In the log, 1-04 is settled by ruling 48, point 3.
**E. The note on Codex's token is wrong.** In `docs/proposals/first-build/README.md`, "Noted for later increments" says that Codex takes an MCP server's bearer token only from a variable, so increment 3 needs an adapter. Codex 0.167.0 also has `mcp_servers.<id>.http_headers_helper`, a local command that prints a JSON object of headers, as Claude Code has `headersHelper`. Both can read `token` from `CONDUIT_IDENTITY_DIR` by a one-line helper, and no adapter is counted in advance. Say what is still to confirm, by A4 or A71: whether Codex runs the helper again after a 401, and whether its "locally connected" servers include an engine at a loopback address. Add one more note for increment 3: when a `plain` server's `tools/list` lacks a tool that its item's `tools` map names, whether the request reports `failed` or `unavailable` is to be settled there, with a vector.
**F. What an Agent reaches by its own means is outside the documents.** In `spec/README.md`, where it says what is deliberately outside the specification, add: an Agent's own means, its built-in tools, its shell and sandbox, its network, and the services of the account it signs in with, are outside the documents; inspecting an instance shows its reach through Conduit only; confining an Agent is a runtime provider's, under PRV-006, and every restriction in this version is reported as mediated, under ACC-027. One or two sentences.
**G. What a pin holds.** In `spec/README.md`'s limits, or in MAN-010's observable if it fits better there, state that the hash of a package file does not pin what its registry resolves for the package's dependencies, and that a `command` pins nothing.
**H. The log.** In `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-1.md`, add a section "After the review of 2026-11-09", with one finding per item A to D and G, numbered from 1-14 on, each with its class, its promises, what changed, its vector, and "The owner can strike it" (D: "Settled by ruling 48, point 3"). Update the table, the summary and the README's table of logs. Keep every count in `spec/README.md`, `spec/schemas/README.md`, `docs/AUDIT.md` and `docs/implementation/INDEPENDENT_BUILDS.md` right, as `check_schemas.py` reports them.

### Order 002

Work order 002: the harness of experiment 008, a stub of the presented server and a runner

Original record (no per-file Record was written):

- **Order**: 002
- **What**: The harness of experiment 008 (A71)
- **Worker**: 17 minutes, then 25 on a second round; a stub on the official Rust SDK and a runner, with tests that need no agent
- **Verifier**: 5 minutes; five `must` and one `should` in the stub, all fixed on the second round
- **What the admin changed**: Amended the pre-registration before any run: tools named as IFC-012 names them, and the matrix cut to 68 runs. Sent the worker back to use the owner's login rather than a fresh configuration and an API key. Added Codex's approval of the stub's server, which the second round missed. Ran one real run per client before the matrix
- **State**: done on 2026-11-09

Own decisions (as written; the Record above states corrections):

1. **One Cargo project in [experiments/008-real-client/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client)**, with `Cargo.lock`, a `rust-toolchain.toml` pinning 1.97.1 as `reference-model/` does, and build output ignored. Use existing crates: the official Rust SDK of the Model Context Protocol (`rmcp`) if it lets the stub set every field these promises need (`_meta`, `isError`, `cacheScope`, `listChanged`, notifications on an open stream, the protocol revisions below); otherwise an HTTP server crate and JSON-RPC by hand. Say which, and why.
2. **The stub** does what the pre-registration's "The stub" says. Name the tools as IFC-012 does (the Conduit's local name and the operation's export name, joined by a dot). Take every `_meta` key from `spec/names.yaml`, never written by hand. Accept the protocol revisions that these clients may offer, 2025-06-18, 2025-11-25 and 2026-07-28, as far as the crate allows, and log which one each session negotiates. Where a promise cannot be followed for a revision, the stub follows the revision and your report says so.
3. **The scenario** is fixed in code: a Conduit `files` listing `read` and `write` on a directory of a few dummy files the stub serves from memory, and a Conduit `notes` listing `read`, closed at first. `control.next` steps through the states each case needs. Write the states for C1 to C5 as a table in a `HARNESS.md` in the directory.
4. **The runner** holds the matrix and the prompts of the pre-registration, fixed in code, and for each run: starts the stub on a free loopback port, writes the token file, starts the client headless in a fresh lab directory under an output directory it is given (never inside the repository), with the isolation of the pre-registration's method, step 3, and records the stub's log, the client's event stream, its standard error and the model's final text, one directory per run. It never edits a file of the owner's configuration, and gives each client no tool but the stub's: for Claude Code, check whether `--tools ""` removes its built-in tools, and allow only the stub's server; for Codex, a read-only sandbox. It closes each client's standard input. It has `--dry-run`, which prints every command without running one, and `--only <cell>`.
5. **Tests that need no agent**: a scripted client drives the stub over HTTP through every case and checks each answer against the promise it follows, citing the ID in the test's name or a comment: admission by token, and 401 on the first request when asked; the list per state; `listChanged` and the notification on `control.next`; `cacheScope`; the `_meta` keys and `isError` for succeeded, failed, unknown and refused; the answer to a tool no longer presented. And a test that the runner's `--dry-run` produces the whole matrix.
6. **A finding of the first build**: where a promise is not clear enough to build the stub from, follow the nearest sound reading and list it in your report under `findings`, with the IDs.

### Order 003

Work order 003: a study for increment 2, the core on S1

Original record (no per-file Record was written):

- **Order**: 003
- **What**: A study for increment 2, the core on S1
- **Worker**: 17 minutes; the plan, the drafts of S1 and a test; six decisions for the admin, two candidate findings, nine proposed orders
- **Verifier**: 4 minutes; two `must`, a promise's strength misstated and a safety case's refusal misdefined, and one `should`, a weak case
- **What the admin changed**: Sent the worker back once with the findings, which it fixed in 5 minutes; [decided](../proposals/first-build/increment-2/decisions.md) the six points and the two findings
- **State**: done on 2026-11-09

Own decisions (as written; the Record above states corrections):

1. This order is that reading. It decides nothing: where a choice is needed, it sets out the options with a recommendation, for the admin.

2. Change nothing under `spec/`: what the text needs goes in section 6. Report anything else that needs changing.

3. **`plan.md`**, not binding, marked as the worker's proposal awaiting the admin's decisions, with these sections:

   1. *S1 as documents.* The project that does T1 and T2: two Agents, one directory that one reaches through a Conduit listing reads and the other through one listing reads and writes, and a message history both post to and read. The packages it needs: the filesystem MCP server as a `plain` item with the contracts its tools map to, the message history, and the Agent types, which increment 2 needs only as far as registering and resolving them. Follow ruling 40: reuse an existing tool for the history, as A70 did, rather than invent a service; set out the options and recommend one.

   2. *The promises S1 touches*, grouped by the slices of increment 2 (first start; registration and resolving; setup and opening; the access decision; how a request ends; records): each ID with one line on what the core must do for S1, and whether the loader already covers it.

   3. *The safety cases S1 touches*, E05 first, each as a proposed vector: its inputs, the decision expected, and the promise that requires it.

   4. *A proposed format for the vectors* (A39): their layout under `conformance/`, how a vector states its documents, its requests and the diagnostics and outcomes it expects (by `ruleId`, with the keyword's location for a schema's rejection, under SUB-005), and how the core runs them. Include the four settlements of increment 1 whose only vectors are the loader's tests.

   5. *A proposed shape of the core*: its crates beside `conduit-loader`, its pure interface (state and inputs in, decisions and records out, each decision citing its promise), and how time, IDs and anything else not deterministic are passed in.

   6. *Where the text does not decide*: each gap, ambiguity or contradiction met while reading for S1, with the IDs, the options and a recommendation.

   7. *A proposed sequence of work orders* for increment 2, each sized for one Codex run, with its scope and its done-when.

4. **`s1/`**, the draft documents of section 1: the project's files as a project directory, and each package as a directory of its own, written in the terms and forms of `spec/`.

5. **One test**, `reference-model/crates/loader/tests/s1_drafts.rs`, that reads each draft directory with the loader and requires it accepted, so the drafts are valid as far as the loader goes. Any draft that the loader cannot judge alone (a rule of the core) is noted in the plan.

### Order 004

Work order 004 (I2-01): the vectors' format, the core's crates, and first start

Original record (no per-file Record was written):

- **Order**: 004
- **What**: I2-01: the vectors' format, the core's crates, and first start
- **Worker**: 17 minutes, then a second round; `conduit-core`, `conduit-conformance`, the format and 11 vectors; one point left to the admin
- **Verifier**: 5 minutes; two `must`: unauthorized requests reached a diagnostic before their refusal, and records took the named object for the target
- **What the admin changed**: Settled the point left to it (two leading byte order marks are not YAML) and sent the worker back once with it and the findings; 13 vectors pass
- **State**: done on 2026-11-09

Own decisions (as written; the Record above states corrections):

1. **The vectors' format** is the study's section 4, as decision 3 adopts it: YAML cases under `conformance/`, each with a stable ID, its layers, its promise IDs, a title, the profile it assumes, an initial state, its files, a sequence of inputs and what it expects. Write `conformance/README.md` (not binding: the format is a convention of the runner, not part of the language), a JSON Schema for a case, `conformance/case.schema.json`, and the first build's profile, `conformance/profiles/first-build.yaml`, holding decision 4's choices on the implementation-defined list. A case's documents stay ordinary Conduit files.
2. **Two new crates** in the workspace under `reference-model/crates/`: `conduit-core`, pure, with no file, clock, network or process access, and `conduit-conformance`, which reads the cases and runs them against the loader and the core. Keep the loader as it is, apart from what the runner needs of its public interface.
3. **The core's interface** is section 5's, in its first form: a state, an input, and a transition that gives the new state, the decisions, the records, the intents for the shell and a reply. Each decision names the promises that require it, and an implementation choice names its DL item in a field of its own, never as a promise. Time, IDs and machine reports enter as inputs.
4. **First start**: from an empty state and a first-start input carrying the time, the ID allocations and the local machine's report, the core makes exactly the objects FST-001 and FST-002 name, with root's two declared Conduits open and all ready, makes nothing else and calls no implementation (FST-003), keeps their provenance (FST-005), makes `local` as PLC-008, PLC-026 and PRV-014 say, and records what it did, with the event types of decision 7. A second first start on a state that has them makes no new identity (decision 4, DL6).
5. **Vectors**, under `conformance/`: the first-start case; FST-006, a submitted instantiation of each protected core template rejected; LIF-034, retiring, unregistering or declaring `local` lost refused; the repeated first start. And the four settlements of increment 1 whose only vectors are the loader's tests: write each as a vector that the runner runs through the loader (the log of increment 1 and the study's section 4 name them).
6. **A finding**: where a promise does not decide what the core must do, stop that case, do not invent its answer, and log it in a new `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md`, in the form of [the log of increment 1](../proposals/first-build/README.md#settlements-register), with "For the admin" as its state. Add the log to the table in [the first build's findings](../proposals/first-build/README.md).

### Order 005

Work order 005: the results of experiment 008 (A71)

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 11 minutes. Every run classified or listed as not run as planned; dated results
  and trimmed evidence; four candidate findings for the admin.
- **Verifier**: 3 minutes. No fault found.
- **Admin**: checked the classification of C5 against the raw records of two runs, and
  recorded that review in the results, which the worker had left as pending. No second round.

Own decisions (as written; the Record above states corrections):

1. **Wire, client and model apart**, as E17 asks: for each run, what the stub's log shows reached the client (`stub.jsonl`), what the client's events show it gave its model (`events.jsonl`, `client-exposed.jsonl`), and what the model did next (`final.txt` and the events). Never infer what the model saw from the wire alone.
2. **The next action** of each run, by the pre-registration's categories and no others: the model retried the same call; told the user the call was refused, failed or unknown, in those words or the same meaning; called another tool; reported success that did not happen; or something else, quoted. Each classification cites the event or line that shows it.
3. **The questions the pre-registration asks**, answered per client and case, each with the runs that show it: whether the client's tool list followed the server's without a restart, when a Conduit closed (C1) and when one opened (C5), under each cache and `ttlMs` variant; what the model was told on a refusal, a failure and an unknown outcome, and whether any of Conduit's `_meta` keys reached it; whether each client read its token from the file by its helper, and what it did after a 401. Record the protocol revision each client negotiated and how each client named the tools to its model.
4. **Runs that did not run as planned** (a client error, a timeout, a harness fault) are listed with their cause and are not classified as model behaviour.
5. **Before the matrix**, the admin ran three smoke runs to check the harness, and added Codex's approval of the stub's server after order 002; say so under the results' deviations.
6. **Findings of the first build**: where the results bear on a promise (IFC-012, IFC-013, IFC-025, IFC-038, IFC-042, PRV-022 or the third thesis), say how, as a candidate for the admin, without changing `spec/`.

### Order 006

Work order 006 (I2-02): authored schemas, the tool definitions, and E05

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **Reuse the loader's validators.** The core evaluates an authored schema with the profile's rules (formats asserted, patterns as SCH-016 says, defaults as SCH-009 and SCH-019 say), reusing the loader's validator and its pattern engine. No new dialect.
2. **Publication**: the core gives the tool definition of an operation of an open Conduit as IFC-033 says: a standalone schema, each contract it reaches copied once under its key, every local reference rewritten so it resolves within the definition, siblings of a `$ref` kept, finite on recursion, with no network and no package resolver.
3. **E05's vectors**, every row of the study's table and the two publication checks after it: each validates its samples twice, against the authored schema in its contract's context and against the published definition with no resolver, and compares the verdicts, not the wording. Cover `inputSchema` and `outputSchema`.
4. **The root `$defs` key** (decision 6, settling the study's F2): an authored schema whose root `$defs` holds a key of the form IFC-033 gives a contract's copy is rejected under IFC-033 when its document is installed or submitted. Amend IFC-033 to say so, with the diagnostic it gives, add a vector, and log the fix in `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md` as "Settled by the admin, decision 6", with "The owner can strike it". If the change belongs in `check_schemas.py` too, so that an example shows it, make it there with an example.
5. **A finding**: as order 004's decision 6 says. Stop a case the text does not decide; log it "For the admin".

### Order 007

Work order 007: experiment 008's follow-up, a later turn

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 8 minutes. Two cases with two turns each, the follow-up's 12 runs behind
  `--follow-up`, the original 68 unchanged; 20 tests that need no agent.
- **Verifier**: 2 minutes. No fault found.
- **Admin**: ran the tests, read the resume command, and ran the 12 runs. No second round.

Own decisions (as written; the Record above states corrections):

1. **Two new cases**, C1-later and C5-later, as the follow-up states, with their prompts fixed in code: one prompt for the first turn and one for the second. The stub's states are those of C1 and C5.
2. **A second turn in the same session**: for Claude Code, on its streamed input (`--input-format stream-json`) in the same process, the second message sent only after the first turn's result; for Codex, by resuming the same session (`codex exec resume`), which needs the first run without `--ephemeral`, and with the same isolation flags otherwise. Every other flag and the removal of the API key variables stay as order 002 left them, including Codex's approval of the stub's server.
3. **The follow-up's matrix**, 12 runs, selected by a new option such as `--follow-up`, so the original 68 stay as they are; `--dry-run` with it prints exactly the 12.
4. **What is recorded** per turn: the stub's log, the client's events, and the model's text, marked by turn.
5. **Tests that need no agent**: the runner's planning of both turns for each client, and the dry run's 12.

### Order 008

Work order 008: the results of experiment 008's follow-up, a later turn

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 7 minutes. 12 runs, 24 turns classified, none excluded; results and 37 trimmed
  evidence files.
- **Verifier**: 2 minutes. No fault found.
- **Admin**: read the second turn of all 12 runs against the classification, and recorded its
  review in the results. No second round.

Own decisions (as written; the Record above states corrections):

1. Order 005's decisions 1, 2, 4 and 6 apply unchanged, for each turn apart: what reached the
   client, what the client gave its model, and what the model did next.

2. **The question of the follow-up**, answered per client and case, each answer citing runs: in the second turn, did the model see and use the tool that opened during the first (C5-later), and did it stop offering, or still call, the tool that closed (C1-later)? Say what the stub's log shows each client fetched between the turns, and what each client's events show its model was given in the second turn.

3. Write `experiments/008-real-client/results-2026-11-09-later-turn.md`, with its trimmed evidence under [experiments/008-real-client/evidence-2026-11-09-later-turn/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn), every absolute path replaced and no credential of any kind; link it from the pre-registration's "Results" and from the first results' summary; and add to the roadmap's A71 row and the register's 008 row that the follow-up ran, with a link.

### Order 009

Work order 009 (I2-03, first part): packages and projects, from snapshots

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **A catalog of installed packages**, given to the core as an input: each a snapshot of an address, a version, a digest and its loaded documents, marked as a snapshot, never fetched. The core keeps it immutable once given.
2. **Resolving a project**: its manifest's pins against the catalog, its exports and names under the PKG, MAN and NAM promises the study lists, the qualified references of SCH-015, and the core package's contracts, which no package may replace (NAM-020, FST-005). A missing or a mismatched pin is rejected, under the promise that requires it, and nothing is fetched.
3. **The digest of a submitted project** as SUB-008 says, computed from the manifest as submitted, never from paths the runner expanded or its fixtures' metadata.
4. **S1's packages and project**: move the drafts of the filesystem server's package, the history's and the two Agent types', with S1's project, into `conformance/fixtures/s1/`, with their digests computed rather than illustrative, and show by vectors that they resolve.
5. **Vectors** for each rule above, an accepted and a rejected case for each, every decision citing its promise, and none that claims an installation: a snapshot is not one.
6. **A finding**: as order 004's decision 6 says.

### Order 010

Work order 010: the harness of experiment 009 (A4), wrappers that run each client as an Agent

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **One Cargo project in [experiments/009-native-launch/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch)**, with `Cargo.lock`, the pinned toolchain of `reference-model/`, and build output ignored. Reuse experiment 008's stub as a path dependency on its library rather than copying it; change 008 only if a small public interface is needed, and say so.
2. **Two wrappers**, each a program that, started with its standard input and output as an MCP server, serves one tool that runs one turn of its client and returns the client's final text with the events it reports. Claude Code's wrapper keeps one process on streamed input (`--input-format stream-json`) and sends each turn's message after the last result; Codex's resumes the same session for each turn (`codex exec resume <id>`). Each starts its client with the isolation of experiment 008 (Claude Code: `--setting-sources`, `--strict-mcp-config`, `--mcp-config`, `--tools ""` and an allow of the stub's server only; Codex: `--ignore-user-config`, `--strict-config`, `--disable apps`, a read-only sandbox and the stub's server approved), plus, for Claude Code, settings that deny `SendMessage`, `ListAgents` and `RemoteTrigger`, turn off the claude.ai connectors and refuse messages from other sessions: find each setting's exact name in Claude Code's documentation or `claude --help`, and say which you could not confirm. Each client reads its token by its header helper from `CONDUIT_IDENTITY_DIR`, and each wrapper removes `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` from its client's environment.
3. **A runner** that starts each wrapper as a run would be started: only the variables PRV-022 and PRV-023 name, a private empty working directory, and an identity directory with its own token naming its own instance. It asks for two turns, with experiment 008's stub opening one Conduit and closing another between them, and records per run the stub's log, the wrapper's exchange, the client's start report (its tools, its MCP servers, its authentication) and the model's text. It also runs both Agents at once, against one stub, with two tokens. Three runs of each; `--dry-run` prints them all; the output directory is an argument with no default.
4. **Tests that need no agent**: the wrappers' MCP interface driven by a scripted client with a fake client program standing in for `claude` and `codex`, the environment each run is given, and the dry run.
5. **Where a promise is not clear enough to build the launch from**, follow the nearest sound reading and list it under `findings`.

### Order 011

Work order 011: the results of experiment 009 (A4), native launch and identity

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 10 minutes. Every point of the pre-registration answered or said to be unshown,
  with runs cited; all three attempts reported; one candidate finding, on Codex's header helper.
- **Verifier**: 3 minutes. One `nit`: the evidence guide said usage was omitted, though Codex's
  token counts were kept.
- **Admin**: fixed the nit, and carried the finding and the points left unshown into what
  increment 3 needs. No second round.

Own decisions (as written; the Record above states corrections):

1. **One answer per point of "What it must show"**, per client, each citing runs, from the records and never from the flags alone: the wrapper (its files and lines), the launch with only the variables of PRV-022 and PRV-023, the owner's configuration kept out, the login's reach off, two Agents apart (the stub's attribution of every request in the pair runs), the change of access between turns, and the login each client reports. Where a client reports nothing that shows a point, as Codex does not report its tools in its events, say so, and say what would show it.
2. **The two first attempts are results too.** The admin ran the matrix three times: the first two failed for Codex, and the admin changed the harness between them. Report each attempt, what failed, what the records show of the cause, and what changed. Codex runs its header helper with a filtered environment, rejects `env_vars` for an HTTP server, and so its wrapper writes the identity directory into the helper's command line; this is a finding for increment 3's Codex Agent type, and goes under the candidate findings with its runs.
3. **Write** `experiments/009-native-launch/results-2026-11-09.md` and trimmed evidence under [experiments/009-native-launch/evidence-2026-11-09/](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/009-native-launch/evidence-2026-11-09), every absolute path, user name and home directory replaced, and no token, key or credential of any kind (the identity directories hold tokens: never copy them). Link the results from the pre-registration's status and "Results", the [register](../../experiments/README.md)'s row for 009 and the roadmap's A4 row, whose state becomes "done on 2026-11-09" with a link.
4. **Limits**: the pre-registration's, and any found in the run, such as what the records cannot show.

### Order 012

Work order 012 (I2-04): registration and the resolving of instances

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **Registration is atomic** (SUB-003): a submission registers every object it holds or none, and a rejected one publishes nothing and starts nothing.
2. **Resolving an instance** under the promises the study lists: its template, its arguments with defaults filled, its references stored by ID and never by name, its owner, its placement, and the nested objects of its declared slots, with generated names reported as DL15's choice in the profile says.
3. **No effect**: registration makes no setup, no run and no call; those are I2-07's.
4. **Order has no meaning** (SUB-001): S1's documents register to the same objects and diagnostics in any order of files and of documents.
5. **Vectors**: S1's project registers, with the objects the study counts (15 named and four nested) or the count the promises give if that differs, which you report; each rule of decision 2 with an accepted and a rejected case; a rejected variant of S1 that publishes nothing; the same result in several orders. Every decision cites its promise.
6. **A finding**: as order 004's decision 6 says.

### Order 013

Work order 013 (I2-03, second part): installing a package from its archive

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **The archive is an input**: its bytes, its expected hash and the completion of fetching them are given to the core; the core fetches nothing.
2. **Installing** verifies the archive's digest (PKG-026), unpacks it as PKG-022 and MAN-013 say, in the form DL7's choice in the profile names, checks that its manifest states the address and the version that were asked for, reads its documents as the loader reads a project (MAN-011, dot-names skipped), and reports the outcome as PKG-023 says. A package installed this way enters the catalog as installed, not as a snapshot.
3. **Vectors with real archives**: small gzip-compressed tar archives kept under `conformance/`, made reproducibly and with their hashes computed: S1's filesystem package installed and then resolved as order 009's snapshot was; a wrong digest; a manifest naming another address or version; an instance document in an archive (MAN-011); a `._` entry that is skipped; and an entry that would unpack outside its directory, rejected. Every decision cites its promise.
4. **No third party's bytes in the repository.** Remove `conformance/fixtures/s1/artifacts/server-filesystem-2.1.0.tgz`, the published package of the filesystem server, which order 009 kept as a fixture: it carries no licence text. The three cases that hash it (`s1`, `s1-digest-mismatch` and `s1-missing` under `conformance/language/packages/`) take its SHA-256 as recorded data instead, with the fixture README saying how it was computed and from what (`npm pack` of the published version) and that the core never fetches it. Every archive the installation vectors use is one this repository makes.
5. **A finding**: as order 004's decision 6 says.

### Order 014

Work order 014 (I2-05): the access decision and the tools an Agent is shown

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **The access decision is pure**: given the state, the instance that a request was established for (or none, or one whose existence cannot be read, under ACC-037), the tool it names and its arguments, the core decides as the ACC promises say, routing under ACC-033, applying each restriction's bounds, and citing the promise of every outcome. Refused and invalid follow the precedence that OPS-023 gives, and a refusal discloses no named object (IFC-025).
2. **The tools an instance is shown** are exactly those of IFC-012: one per operation of each open Conduit, named by the Conduit's local name and the operation's export name, defined as IFC-033 says through order 006's publication, and nothing else. A tool the server offers that no contract maps is never shown (CON-017), and the tool list changes when a Conduit opens or closes (IFC-013, IFC-038).
3. **Vectors**: S1-exact-access, S1-pool-authority and S1-plain-filter as the study writes them; A and B's lists with A's Conduit listing reads and B's reads and writes; a guessed tool refused with no invocation; a request from no instance, and from one whose existence cannot be read; and each restriction's bound at its limit and past it. Every decision cites its promise.
4. **A finding**: as order 004's decision 6 says.

### Order 015

Work order 015 (I2-06): how a request ends, its repeats, and plain results

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **The ending of a request** under OPS-001 and OPS-005, with the result envelope of IFC-025, reasons in the order OPS-023 gives, and inputs normalized with defaults filled before a repeat is compared (OPS-009, OPS-012).
2. **Repeats**: the invocation index keyed as OPS-009 and OPS-012 say, by caller, sender, target, operation and filled contents; a repeat of a succeeded invocation is answered from what was kept and never dispatched again; one with other contents reports conflict (OPS-032, OPS-034); one still in progress is answered as the promises say.
3. **F1, by decision 5**: the content of a succeeded `plain` invocation is kept and given again on a repeat, as declared outputs are, for as long as DL10 keeps outputs; after that a repeat reports conflict under OPS-034. Amend OPS-009, IFC-025 and DL10 to say so, add the vector, and log the fix in `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md` as "Settled by the admin, decision 5", with "The owner can strike it".
4. **Plain results**, with the table of IFC-010 and IFC-046 corrected by [decision 8a](../proposals/first-build/increment-2/decisions.md), which settles the check of [order 018](../proposals/first-build/increment-2/decisions.md#evidence-for-decision-8a): `-32602` from a `plain` server is treated as any other error of such a server, `failed` for an operation that has no effect, is not a control and starts no model work, and `unknown` otherwise; and the engine sends a call again after `-32020` only for an operation that has no effect or that its contract declares safe to repeat, and otherwise ends it `unknown`. A rejection the engine makes before it sends a call keeps its reason and has no outcome (OPS-005). Amend IFC-010 and IFC-046, and OPS-003 or OPS-018 if they repeat the old reading, with vectors, and log each as "Settled by the admin, decision 8a", with "The owner can strike it". Then cover every result S1-plain-ends lists (success, `isError`, no result, `input_required`, `-32602`, `-32020` and `-32021`), reads and writes apart.
5. **Unknown outcomes and reconciliation**: an unknown write is never dispatched again by the core, not after a repeat and not after a restart from stored state (OPS-012, OPS-013, OPS-018, IMP-007); reconciliation by an authorized actor with the evidence it checked sets the outcome (OPS-014, OPS-030), never a timeout. Cancellation as its promises say.
6. **Records cannot erase invocations**: the invocation index is state of its own, not records, so that purging records leaves it whole.
7. **Vectors**: the four rows of section 3 named above, every variant they list, and F1's; every decision cites its promise.
8. **A finding**: as order 004's decision 6 says.

### Order 016

Work order 016 (I2-07): setup, and the opening and closing of Conduits

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **Setup** in its parts, in the order its promises give, each part an invocation where the promises say so, with the sender's authority for each open checked as it is made; a part that fails leaves the parts before it, with no rollback, as the promises say; a second setup makes no new identity and opens nothing already open.
2. **Intents and completions**: running an implementation is an intent to the shell (PRV-003, PRV-007, PRV-022), and its report is an input; the core never starts a process.
3. **Opening and closing** under CON-004 and the CON and LIF promises: a closed Conduit's tools leave its consumer's list (IFC-013, through order 014); a result that arrives after its Conduit closed ends as ACC-034 and IFC-025 say; a closed Conduit is removed (LIF-035), and restoring one is an explicit registration of its replacement (LIF-009), never implicit.
4. **Vectors**: S1's base setup, and the six rows of section 3 named above, every variant they list; every part's invocation and caller links checked; every decision cites its promise.
5. **A finding**: as order 004's decision 6 says.

### Order 017

Work order 017 (I2-08): records, inspection, and the bounds of repair

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

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

Own decisions (as written; the Record above states corrections):

1. **Every record the promises require**, with every field `record.v1` asks for, validated against it, from every transition of orders 004 to 016, the access decision's record before any dispatch it allows; the event types of decision 7, documented in the profile.
2. **Inspection** as LIF-013 and the promises the study lists say, compared by decision 7's projections, which never hide an omission.
3. **Retention and purge**: purging records leaves the invocation index whole (order 015), and a gap that a purge leaves is shown as one.
4. **Repair's bounds**: a follow-up by expected ID, a template replaced with its revision checked, and E08's narrow broker case, with no implicit effect, as section 3 says.
5. **Counting** for D19: the bytes of records and of the invocation index, apart, and their growth per request, reported by the runner.
6. **Vectors**: every row of section 3 on records, inspection and repair; every decision cites its promise.
7. **A finding**: as order 004's decision 6 says.

### Order 018

Work order 018: the check of two protocol errors, -32602 and -32020

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 8 minutes. The protocol's text and the official SDKs' source, cited: neither
  `-32602` nor `-32020` proves that a server has not acted, and the SDKs let a handler answer
  either after acting. No such path in the filesystem server at 2.1.0 or 2.2.0.
- **Verifier**: 3 minutes. One `must`: the report gave `failed` to the engine's own rejection
  before dispatch, which has a reason and no outcome (OPS-005).
- **Admin**: fixed that sentence, and decided the two readings as decision 8a of
  [increment 2's decisions](../proposals/first-build/increment-2/decisions.md), which order
  015 applies.

Own decisions (as written; the Record above states corrections):

1. Only the new file. Change nothing under `spec/`.

2. **The protocol's text**: in each revision this specification names (read IFC-001, IFC-003 and DL12 for which), quote exactly what it says of -32602 and of -32020: when a server sends each, and whether it says the server has not acted. Cite each revision and section.

3. **What servers do**: in the official SDKs a server is built on (TypeScript, Python and Rust's `rmcp`, at least), find where each code is sent, and whether a tool's handler, after it has acted, can make the server answer with it (a handler that throws the protocol's own error with that code, for instance). Cite files and lines of a tagged release.

4. **The reference servers**: whether the filesystem server that S1 uses, at the versions experiment 008 and A70 ran (2.1.0 and 2.2.0), ever answers either code after writing.

5. **Write** `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/check-plain-errors.md`, not binding: for each reading, what the protocol says, what its servers do, whether the reading holds, and if it does not, the options for IFC-010 or IFC-046 with a recommendation for the admin. Decide nothing.

### Order 019

Work order 019 (I2-09): S1 whole on the core, and what increment 3 receives

Record, verbatim:

Written by the admin when it merged the order, on 2026-11-10.

- **Worker**: a first run of 2026-11-09 was lost with the session's scratch space, before it was
  committed; the order ran again, 16 minutes, then a second round. S1 runs whole on the core as
  a 58-step vector; every promise S1 touches has vectors (217) or a stated reason (91); the
  first build's conformance document and the handoff to increment 3 are written; increment 2's
  done-when is reported item by item.
- **Verifier**: 5 minutes. Three `must` in the handoff and the conformance document, and one
  `should`: steps of the whole-S1 vector that did not check their records.
- **Admin**: sent the worker back once; each of the 75 record expectations added now fails the
  vector when removed. Marked increment 2 done.

Own decisions (as written; the Record above states corrections):

1. **S1 whole**: one vector, or a short chain of them, that takes S1 from first start through registration, setup and the opening of its four declared Conduits, then T1's and T2's requests (A reads the directory, B reads and writes it, both post to and read the history), then T7's (A's Conduit to the directory closed, B still working, A's restored by explicit replacement), each request ending and recorded as orders 015 and 017 say, with the scripted completions of the shell standing in for real servers.
2. **Coverage**: the conformance manifest lists, for every promise that the study's section 2 says S1 touches, the vectors that show it, or says why none does; a promise S1 touches with no vector and no reason is a gap to fill or to log.
3. **The first build's conformance document**: `reference-model/CONFORMANCE.md`, stating each choice the first build makes on the implementation-defined list (the profile's, and those orders 004 to 017 made), each with its DL item and where it is shown.
4. **What increment 3 receives**: `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md`, not binding: the core's interface as the shell will call it, every intent the core emits and every completion it expects, with their shapes; the tool contract the shell presents; and what increment 3 must still build or show, from "What increment 3 needs" and experiments 008 and 009, each as an item the shell's orders can take.
5. **The done-when of increment 2**, checked and reported item by item: S1's documents register and resolve; every decision cites a promise; the safety vectors pass; each case the core could not decide is a logged finding, settled or left for the admin.
6. **Lean vectors**: `conformance/` holds about 8 MB of cases, many of them whole states of up to 90 KB. Report what makes them large, and propose how to keep them small and readable, such as shared fixtures for a common state and projections in place of whole states, as decision 7 asks of records. Do not rewrite them in this order; say what a later order would change.
7. **A finding**: as order 004's decision 6 says.

### Order 020

Work order 020 (I3-01): the shell's store, its loop and its identities

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. **A new crate**, `reference-model/crates/shell`, package `conduit-shell`, with a library and a binary named `conduit`. It depends on `conduit-core` and `conduit-loader` and never decides anything the core decides: access, outcomes, records and lifecycle stay in the core. Where the shell would need to decide one, it stops with an embedding error and you report it.
2. **The store** is a directory the user names. The core's whole `State`, which includes its retained records, is committed as one file replaced atomically: written to a temporary file in the same directory, flushed with `fsync`, renamed over the old one, and the directory flushed. One commit per transition. The store's directory is created with mode 0700 and its files 0600. A store holds a format version, and the shell refuses a store whose version it does not know. Record it as the DL2 choice for storage.
3. **The loop.** One writer at a time: a transition reads the latest committed state, calls `conduit_core::step`, and commits the new state and records **before** it carries out any intent or returns any final reply (DL2, REC-001). Intents are carried out outside the lock by an `Executor` trait, one method per intent kind of the handoff, so that orders 021 and 022 plug in. Each completion comes back as its own input through the same lock: `complete` by invocation and attempt, `setup-report` by invocation. A pending reply waits, inside the shell, until a later transition ends the invocation, and it never reaches a caller as a success. An intent kind that no executor serves ends as the core says when the shell reports it failed or missing; it is never dropped silently. This order supplies a scripted executor for tests only.
4. **Evidence** comes from the shell: the time from the system clock in RFC 3339 UTC, IDs as random UUIDs from the operating system's generator, and revision values and five-character suffix candidates as the core's contract asks. Supply enough for one transition; if the core reports exhaustion, retry the same input once with a larger supply and then stop with an embedding error. The state you retry from is the committed one, unchanged.
5. **Recovery.** When the shell opens a store, it gives the core one `recover` input before it accepts anything else, and commits the result. Nothing that was dispatched before the crash is sent again (IMP-007). Show it with crash tests: a child process that the test kills (a) after the commit and before the dispatch, and (b) after the dispatch and before the completion is committed. Each test reopens the store and shows that the invocation ended unknown, has one record of its end, and was not dispatched twice.
6. **Retention of outputs (DL10).** The owner asked, in the [first review](../proposals/first-build/to-review-with-the-owner.md#the-first-review-2026-11-10), for a cap on what item 3 keeps: the content of `plain` calls kept for repeats. The shell expires retained outputs through the core's `expire-outputs` input, never by editing state itself, when their total size passes a cap or their age passes a limit, oldest first. Defaults: 64 MiB in total and 7 days, both settable in the shell's configuration. Expiry never drops invocation state. Add the policy to `CONFORMANCE.md` as the shell's DL10 choice, replacing the line that says no production policy exists, and show it with a test that passes the cap.
7. **Identities (DL13).** The shell holds one signing key per store, an ECDSA P-256 key made at first start, kept in the store with mode 0600 and never in the core's state, in a record or in a log. For an instance it issues what PRV-022 names: `token`, a JWT-SVID signed with ES256 whose `sub` is the instance's SPIFFE ID, `spiffe://<engine-id>/instance/<instance UUID>`, whose `aud` is the engine's address, with `iat`, no `exp` (a credential lasts as long as its instance, under DL13) and a `jti`; `engine-id`, the engine's ID, a trust domain made at first start (`<random 26 lowercase letters and digits>.conduit.local`); `jwks.json`, the JWK Set of the public key; and `ca.crt`, a self-signed root certificate in PEM for the same key. Since the first build serves `http` on loopback (DL13, PRV-022), `ca.crt` checks nothing; say so in `CONFORMANCE.md`. The shell writes them into a directory it creates per run, mode 0700, files 0600, under the store, and gives `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` for it. The names come from the run contract the intent cites, as the handoff says, never from literals. Verifying a token checks the signature, the algorithm, `aud`, the SPIFFE ID's form and trust domain, and asks the core's state whether the instance still exists (IFC-022, IFC-032); the result is the `identity` value the core's `Invoke` takes, `{kind: instance, id}`, or `{kind: none}` with no token, or `{kind: unreadable}` with a token that does not verify. A test shows that no part of an issued identity appears in the committed state, the records or the shell's output (PRV-024, SEC-018).
8. **The command line.** `conduit init --store DIR` (first start, with the machine's host name and no labels unless given), `conduit register --store DIR PROJECT_DIR` (the project path of the handoff, with the S1 packages preinstalled from `conformance/fixtures/s1/` as the runner does), `conduit inspect --store DIR` and `conduit records --store DIR` (export, as JSON lines). Registration's setup will stop at its first runner intent while no runner exists; that is expected here, and the command says which intent waits. `conduit serve` comes in order 022.
9. **Tests** run the loop end to end on S1 with the scripted executor standing in for the runner and the servers, as the `s1-whole` vector's completions do, and check that the records the shell exports match those the vector expects, apart from times and IDs.
10. **The log of increment 3**: create `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md` in the form of the log of increment 2, and log there any finding under ruling 46, numbered 3-01 on. A case the core cannot decide is a finding; unfinished shell work is not.

### Order 021

Work order 021 (I3-02): the engine's own runner, and the unchanged filesystem server

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

Order 020's `Executor` answers each intent with an `Execution`. An intent whose work takes time
answers `Deferred` and returns its completion later through `Shell::complete` or
`Shell::setup_report`, never by holding the shell's lock while it waits. The engine's address
is `Config::engine_url`.

1. **Intents served**: `builtin-runner` (`run.v1` and `inspect-run.v1`), `dispatch`, `refresh-tools-and-call`, `cancel` and `cancel-setup-parts`, each exactly as the handoff gives its shape and its returning input. `provider-call` and `connect` stay unserved in the first build, which has no runtime provider and no remote item; they end as order 020's loop ends an unserved intent, and `CONFORMANCE.md` says so.
2. **Items the runner runs.** A `package` item with `registryType` `npm`: the runner fetches the pinned version with `npm pack`, checks the file's SHA-256 against the item's `fileSha256` before anything else and refuses to run on a mismatch, unpacks it into a cache in the store keyed by that hash, installs its dependencies there with `npm install --omit=dev --ignore-scripts`, and runs the package's `bin` with `node`. Say in `CONFORMANCE.md` that the hash pins the artifact and not its dependency graph (MAN-010). A `command` item: its program, as a relative path, is taken from the installed package's files, written out under the store; an absolute program is run as given. Any other `registryType` makes the run fail, with a diagnostic that names it, as the DL16 choice of supported registry types.
3. **The arguments and the environment.** `packageArguments` are bound as MAN-010 gives them, positional by `valueHint`; for S1 that binds the filesystem server's directory to the held root, and nothing else is passed. The process gets the environment of PRV-023 exactly: the six variables where the engine has them and their value does not begin with `()`, the variables a list in the shell's configuration names (empty by default), the run's `environment` and secrets, and `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` from order 020's identity for that instance, whose values win. A test shows a variable of the engine's own environment, such as one named `GITHUB_TOKEN`, does not reach the process.
4. **The process.** Working directory: a private, empty directory per run under the store, mode 0700, which is not the shared directory. Each run gets its own process group; the runner ends every group when the shell stops, and no run outlives the engine, which is the condition under which PRV-022 lets `CONDUIT_URL` be `http` on loopback. Restart policy (PRV-010): none in the first build. A run that stops unasked is reported `stopped` by the next `inspect-run.v1`, and its outstanding attempts end as the core says when the runner reports them missing. After the shell restarts, every run from before is `stopped`. Each of these is a DL16 choice, written in `CONFORMANCE.md`.
5. **MCP over stdio.** One client session per run, opened at `run.v1`, with `rmcp` pinned as experiment 009 pins it. The runner declares **no** `roots` capability, so that the filesystem server keeps the directory it was given and no client can widen it through MCP Roots; a test with the real server shows a path outside the root is refused by the server itself, and the record shows the engine's own decision came first. `run.v1` reports `succeeded` only after the session's `initialize` has answered; a run that starts and never answers reports `failed` after a deadline (DL10, default 30 seconds, settable).
6. **Dispatch.** A `dispatch` sends exactly the `call` the intent holds, `tools/call` with the operation's arguments only, to the session of the selected implementation, and returns `complete` with the same invocation and attempt: `result`, `protocol-error` with the code and data, or `missing` when the process ended or the deadline passed (DL10, default 120 seconds, settable). No Conduit key, identity, configuration or revision crosses to a `plain` server (IFC-007); a test inspects the bytes on the pipe. `refresh-tools-and-call` refreshes the tool list once and sends the read once, within its attempt, never generalized to a write (IFC-046). `cancel` sends `notifications/cancelled` for that attempt's request ID. `cancel-setup-parts` ends every run part that setup started and starts none.
7. **S1 for real.** `conduit register` on S1's project, as order 020 wired it, now completes setup with the real filesystem server `@modelcontextprotocol/server-filesystem` 2.1.0 for both directories, whose `fileSha256` the fixture pins. A test, run only when the variable `CONDUIT_TEST_NPM` is set, does that against temporary directories, makes one read and one write through the core as S1's B, and checks the bytes on disk and the records. Run it and report its result; the default `cargo test` stays offline.
8. **Findings and limits**: as order 020's decision 10, in the log of increment 3.

### Order 022

Work order 022 (I3-03): the server consumers connect to

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

Order 020's `Executor` answers each intent with an `Execution`. An intent whose work takes time
answers `Deferred` and returns its completion later through `Shell::complete` or
`Shell::setup_report`, never by holding the shell's lock while it waits. The engine's address
is `Config::engine_url`.

1. **`conduit serve --store DIR`** opens the store (recovering as order 020 does), starts the runner of order 021 and serves the engine's address: Streamable HTTP at path `/mcp`, bound to a loopback address only, on the port the store's configuration names. The address is the one order 020 put in every token's audience; if order 020 did not fix it at first start, fix it there now, and refuse to serve at any other. It also runs S1's setup, as `register` left it, if setup is outstanding. It stops cleanly on a signal, ending every run.
2. **Who a request comes from (IFC-022, IFC-042).** A request with `Authorization: Bearer` and a token that order 020 verifies is from that instance. A request with no token is identity `none`; one whose token does not verify, names another audience, or names an instance that no longer exists is answered with HTTP 401 and the `WWW-Authenticate` header, and reaches no tool. The shell never takes identity from anything else: not the client's name, not a header it sets, not the machine. Requests whose `Origin` is present and is not the engine's own origin are refused with 403, against DNS rebinding.
3. **The tools (IFC-012, IFC-033, IFC-038).** `tools/list` answers exactly the core's `list-tools` for that identity, with each tool's standalone schema, its annotations, and the private cache scope and `listChanged` the handoff names; nothing is added, renamed or hidden by the shell. The server declares `tools.listChanged`. After every committed transition, the shell calls `access::tool_list_changes` and sends `notifications/tools/list_changed` to that consumer's open sessions only, never to another consumer's. State in `CONFORMANCE.md` that no `ttlMs` is set (DL10, DL12) and that the engine does not infer a client's model catalog from what it sends.
4. **A call (ACC-001, ACC-036, IFC-025).** `tools/call` becomes the core's `invoke` with the established identity, the tool, the arguments and the serving key the request carries in its `_meta`; current authority is checked on every call by the core, never from a cached permit. A final reply is the core's MCP result unchanged. A pending reply is held, inside the shell, until the invocation ends, and the HTTP request stays open meanwhile. A tool not presented to that identity is a protocol error with `refused` in `error.data`, as the core answers, never a successful result. `notifications/cancelled` from the client becomes the core's `cancel` for that invocation (OPS-027). Every request has its record before its reply leaves (REC-001); a test reads them back.
5. **The management route.** `conduit call --store DIR TOOL [ARGUMENTS-JSON]` sends one `tools/call` to the running server as the Interface `root`, through which the person who runs the engine acts (FST-001, FST-002), by a token order 020's identity issues for `root` and keeps in the store with mode 0600, and prints the result. `conduit tools --store DIR [--as INSTANCE]` lists tools. These are what the admin uses to close and restore a Conduit in T7, so test both: closing A's Conduit to the directory, as T7 does, is seen in A's next `tools/list`, B's list does not change, B is sent no notification and B keeps working, and A's Conduit restored by explicit replacement is seen in A's next list.
6. **What is not finished (H03).** Before serving, the shell compares every operation that `list-tools` can present with what the core can execute, from the handoff's last section. If any identity in S1 would be presented an operation the core cannot carry out, stop and report it as a finding; do not hide it in the shell. Operations presented only to the management identity that the core cannot carry out yet are answered with JSON-RPC internal error `-32603` and a message that names the operation as unfinished, with nothing committed. List them in `CONFORMANCE.md` as limits of the first build.
7. **Tests** start `conduit serve` on S1 with the scripted executor or, where `CONDUIT_TEST_NPM` is set, the real filesystem server, and connect with an `rmcp` client as A and as B by the tokens the runner's identity directories hold: A lists read, list and the history tools and is refused a write; B writes and reads it back; a request with no token and one with a forged token are refused as decision 2 says; T7's close and explicit restoration through `conduit call` behave as decision 5 says. Run the `CONDUIT_TEST_NPM` tests and report them.
8. **Findings and limits**: as order 020's decision 10, in the log of increment 3.

### Order 023

Work order 023 (I3-04): the Agent types for Claude Code and Codex, as packages of their own

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. **Two packages**, `packages/agent-claude-code/` and `packages/agent-codex/`, each a Rust crate of its own (its own `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` pinned as `reference-model/` pins it, and ignored build output), with no path dependency on anything outside its directory: what both need is copied, not shared. Each directory also holds the package's `conduit.yaml` and `contracts.yaml`, a `README.md` that names its maintainer as the project's owner and the client versions it was tested with (Claude Code 2.1.318, Codex 0.167.0, or what `--version` reports here), and a test of its own.
2. **The Type.** Each exports one Agent Type, `claude-code.v1` and `codex.v1`, whose schema keeps the fixture's `accountServices` and `peerMessaging` booleans and adds an optional `model` string. Its implementation item is a `command` whose program is the package's wrapper binary at a path relative to the archive's root, with `environment` binding each of the three inputs to a variable by a JSON Pointer (MAN-010). It offers one operation, `turn`, whose input is a message string and whose output is the reply text, marked `startsModelWork` (TPL-019), with `records` that keep neither inputs nor outputs. Setup starts no model work: the wrapper starts its client only at the first `turn`.
3. **The wrapper** is an MCP server over stdio, as experiment 009's is, presenting only `turn`. It reads `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` from its environment (PRV-022), and configures its client with the engine's server as its only MCP server. One conversation per Agent for the wrapper's life: Claude Code as one streamed session (`--input-format stream-json`), Codex resumed by thread in a new process per turn (H06). The client runs in the private, empty working directory that the runner gives the run (DL16), never in the shared directory, and the client's configuration that the wrapper writes goes there.
4. **The owner's configuration kept out** (need 3, H07): Claude Code with `--setting-sources` naming no user source, `--settings` and `--mcp-config` from the wrapper's own files, and `--strict-mcp-config`; Codex with `--ignore-user-config`, `--strict-config` and `--ignore-rules`. `ANTHROPIC_API_KEY` and `OPENAI_API_KEY` are removed from the client's environment, so the login is the owner's shared one (ruling 48, point 2).
5. **The Agent's own reach, off by default** (need 4, H08, ruling 48 point 1). With `accountServices` false: Claude Code's claude.ai connectors off and Codex's apps disabled (`--disable apps`). With `peerMessaging` false: Claude Code's `SendMessage`, `ListAgents` and `RemoteTrigger` disallowed and messages from other sessions refused. Each input set true turns its part on for that Agent alone. The client's own built-in tools that reach the machine are off for Claude Code (`--tools ""`, with the engine's server's tools allowed), and for Codex its sandbox is `read-only` in the private directory, with web search disabled. Codex's read-only shell can still read files outside the engine: say so in the package's README, as a route around the engine the closeout must report (ACC-027), not as a solved problem.
6. **The token from its file** (need 6, H09). Claude Code reads it by `headersHelper`, Codex by `http_headers_helper`, each running the wrapper binary in a helper mode that prints the `Authorization` header from `token` in the identity directory. Codex's helper is given the directory on its command line, since Codex runs it without the run's environment and rejects `env_vars` for an HTTP server (experiment 009). The token is never in a variable, an argument or a log.
7. **One approval** (need 7): an allow rule for the engine server's tools in Claude Code's settings, and `default_tools_approval_mode = "approve"` for that server in Codex, so that the engine's decision is the only one per call.
8. **Access changes at a turn's boundary** (need 2, H06). When the wrapper's client is sent `notifications/tools/list_changed`, the change reaches the model at its next turn, as experiment 008 found. Each `turn` result's structured content reports the engine's tool list the client saw at that turn's start, as the client's own events show it, apart from the engine's list, so the closeout can compare them.
9. **Build and pin.** A script-free step: `cargo run --release --bin package` in each crate (or one documented `cargo` command) builds the wrapper for the host and writes a gzip archive in the form of the S1 fixture's archives (USTAR, sorted names, zero owner and time, the binary at mode 0755, the two YAML files at 0644) with its SHA-256. The shell's `conduit register` gains `--package ARCHIVE#SHA256`, which installs an archive through the core's `install-archive` before registration, checking the digest, so the runner can run its `command` item. Add a project for the live S1 at `reference-model/examples/s1/`: S1's fixture project with its two Agent templates using these Types, each with both reach inputs false. Copy each package's two YAML files into `conformance/fixtures/agents/<package>/` as the pinned copy, with a README saying where they come from, and add one vector that registers the live project against them; touch no other vector. The packages' YAML holds the names of `spec/names.yaml`, so extend `tools/set_names.py` to rewrite and check `packages/` too, as it does `conformance/`.
10. **Tests without the real clients**: a scripted client binary, as experiment 009's, stands in for each, and tests check the exact arguments and environment the wrapper gives it for each combination of the two inputs, that the helper prints the header from the file and nothing else, that no key or token reaches an argument or a log, and that the wrapper presents only `turn`. One test, in the shell's crate, runs S1's setup with both Agents on the scripted clients, calls `turn` on each through `conduit call`, and shows that each scripted client reached the engine with its own identity.
11. **What only the real clients can show** (need 10): write `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md`, not binding, as the admin's plan for the closeout: each check of need 10 and of increment 3's done-when that needs a real client, with the sentinel or evidence that would show it (such as an instruction file, a hook and a user setting each planted where the client would load it if it could), the command to run and what a pass and a fail look like. Do not run it.
12. **Findings and limits**: as order 020's decision 10, in the log of increment 3.

### Order 024

Work order 024 (I3-05): what the first run with the real clients found

Record, verbatim:

Run on 2026-11-10: the worker in 32 minutes, then a fresh verifier in 5, which found nothing
to fix. The worker logged findings 3-03 to 3-05 as settled. It did not run the suite with
`CONDUIT_TEST_NPM=1`, reading this order's network grant as not covering npm; the admin ran
it. The admin also rewrapped the spec paragraphs the worker had left with long lines, without
changing their content (each YAML file loads to the same value). The admin reran every
check: 136 Rust tests in `reference-model/` with `CONDUIT_TEST_NPM=1` and six in each package
pass, and clippy, fmt and the five repository checks are clean. The admin reruns the closeout
with the real clients next.

Own decisions (as written; the Record above states corrections):

1. **Finding 3-03, in the specification.** An operation that declares no outputs is presented as a tool with **no** `outputSchema`, and a result of it carries **no** `structuredContent`; its `content` is as IFC-025 gives it, a `plain` result's content unchanged. An operation that declares outputs keeps its `outputSchema`, and its results their `structuredContent`, as now. Change IFC-025 and IFC-033 (and any other promise that says an operation with no outputs has an empty map, such as the passage in `spec/conventions/operations.yaml` on a repeat), giving the reason in each promise's `observable`: the protocol makes the schema optional, a client that sees one may prefer the structured result, and an empty map would hide the content. Change the core so it presents and answers that way, and every vector that expects the old form; add one vector that fails before the change and passes after, for a `plain` read whose text is the result's only content. Log 3-03 in the log of increment 3 as settled, with the spec's change.
2. **Finding 3-04, Claude Code's wrapper**: `--setting-sources ""` in place of `project`. Nothing else the wrapper relies on may come from a setting source: the engine's server, the header helper and the allow rule must reach Claude Code through `--mcp-config` and `--settings` alone. Test the arguments.
3. **Finding 3-05, Codex's wrapper**: disable every feature named in finding 3-05, by `--disable`, and set `-c project_doc_max_bytes=0`. `accountServices` true enables `apps` alone, as now; nothing in this order turns the others back on. Write in each package's README the client's own tools that stay on (for Codex, those the trial still listed, such as `apply_patch` in its read-only sandbox, and its request for user input), as routes the closeout must report. Test the arguments.
4. **The Agent's own machine tools are off by default.** Ruling 48's point 1 turned off the Agent's own account services and messaging. The admin settles, under ruling 46, point 4, that the same holds for a client's own tools that reach the machine, a shell, code execution, a browser, another Agent, hooks and plugins, which Claude Code's wrapper already does with `--tools ""` and Codex's now does by decision 3. Say so in each Type's description and README, and in `CONFORMANCE.md`.
5. **Pins and versions.** Bump each package's version to 0.1.1, rebuild its pinned copy under `conformance/fixtures/agents/`, and update the live project and the vector that registers it.
6. **The closeout plan**: update `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md` for decisions 1 to 4: a sentinel in an ancestor directory and its baseline; Codex asked to read a file outside the engine; the read's text reaching the model.

### Order 025

Work order 025 (I4-01): the operations S1's tasks of change need

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. **`install.v1`, `retire.v1` and `unregister.v1` become public operations** as `spec/core/operations.yaml` and the promises describe them, requested of the serving engine like any other management operation, with their records. For `install.v1`, the engine fetches `file` URLs itself; an `https` fetch may be left unfinished, reported invalid with a diagnostic that says so, and listed in `CONFORMANCE.md`. A package installed while the engine serves is usable by the next request without a restart. `retire.v1` stops the run of an item that is run, through the runner, and closes the Conduits of which the instance is the target at once and those of which it is the consumer when its shutdown is confirmed; a retired Agent's run ends and its consumer stream is ended. Where the core already decides a case and the shell only lacked the path, keep the core's decision; where the core cannot decide a case from the promises, it is a finding: report it, do not settle it.
2. **Commands for the author**, each a request of the serving engine as the root Interface, as `conduit call` is, printing the result as `call` does: - `conduit install --store S ARCHIVE#SHA256`: `install.v1` with the address and version read from the archive's manifest, the digest given and an absolute `file:` URL for `fetch`. - `conduit submit --store S DIR`: `register.v1` of the documents in a directory, read as MAN-001 and SUB-010 read a project directory, with that project's manifest and contracts. It then sets up the new instances and opens the new Conduits whose consumer is the root Interface, as `conduit register` does at first registration, each as a request with its record. - `conduit replace --store S DIR KIND/NAME`: `replace.v1` of the template of that kind and name, its new document read from the project directory `DIR` with that project's manifest and contracts, at the template's current revision, which the command reads by `inspect.v1`. - `conduit retire --store S KIND/NAME` and `conduit unregister --store S KIND/NAME`, the latter at the current revision. The commands work only while `conduit serve` holds the store; without it each says so and exits non-zero. `conduit register` stays the command of first registration.
3. **The filesystem package 0.2.0.** Beside `example.org/s1/filesystem` 0.1.0, add 0.2.0: the same contracts and the same `tools` map, pinning `@modelcontextprotocol/server-filesystem` 2.2.0 by its `fileSha256`, which you compute from the npm archive you fetch. Its archive is not a preinstalled snapshot: it is installed by `conduit install`. Lay out its files beside 0.1.0's in `conformance/fixtures/s1/`, with a way to rebuild its archive and digest that the README states. The history package stays on filesystem 0.1.0.
4. **Vectors and tests**, each failing before this order and passing after: - T4's route: `replace.v1` of a Conduit template that a nested Conduit was made from, then `close.v1`, `register-replacement.v1` and `open.v1` of that nested Conduit. The consumer's list then has exactly the operations the template now lists, under the same tool names (IFC-012, LIF-009), the `list_changed` notice reaches that consumer alone (IFC-038), and another consumer whose nested Conduit was made from the same template before the replace keeps its list until its own replacement (INS-020). - T5's route: `install.v1` of 0.2.0; a new Resource whose template resolves filesystem 0.2.0; a Conduit to it that lists the three operations. Its consumer is offered exactly three tools, and `append_file` and `copy_file` never appear in any consumer's list (PKG-020, IFC-012); an instance made on 0.1.0 keeps 0.1.0 (PKG-005). With `CONDUIT_TEST_NPM=1`, the 2.2.0 server runs and a read through it succeeds. - Retirement: retiring a Resource closes the Conduits that target it at once; a `register-replacement.v1` whose target is retired reports invalid (CON-025); `unregister.v1` of an instance that is not retired reports conflict, and of a retired one removes it while its records stay. - Each command of decision 2, in the shell's tests, against a serving store.
5. **`CONFORMANCE.md`** lists the operations now supported, what remains unfinished, and every choice this order leaves to you, and the reference model's README shows the commands.

### Order 026

Work order 026: the entry documents, read cold and brought up to date

Record, verbatim:

Run on 2026-11-10: the worker in 9 minutes, then a fresh verifier in 2, which found one stale
statement left in scope. The worker's cold reading reported 25 findings: 16 in scope, fixed,
and 9 in documents outside it. The admin widened the scope for a second round to those nine,
with limits: in `spec/` only its README's non-normative status and contents, in `docs/AUDIT.md`
only current-status summaries and never a row of the ruling log. The admin checked that no
promise and no ruling-log row changed. The checks pass; the worker's Rust run in its sandbox
could not bind a loopback port, and no code changed.

Own decisions (as written; the Record above states corrections):

1. **Read cold first.** Before changing anything, read the entry documents as a newcomer would: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, `docs/OVERVIEW.md`, `ROADMAP.md` down to its register, `reference-model/README.md`, `docs/guides/README.md`, and `spec/README.md` down to its contents. For each place where a newcomer would be misled, lost or stopped, note it as a finding: the file and line, what is wrong (stale, unclear, missing, contradicting another document) and the fix. Keep them in your report whether or not you fix them.

2. **Bring them up to date.** Fix every stale statement in your scope from the records it summarizes, never from memory: the roadmap's step 3, the logs under `docs/proposals/first-build/`, `reference-model/README.md` and `CONFORMANCE.md`, the ruling log in `docs/AUDIT.md` (rulings 48 and 49 above all), `docs/work-orders/README.md`, and `experiments/README.md`. Each state you write carries its date. Keep each document's role: the top README says what Conduit is and what exists, with links; `AGENTS.md` instructs agents and workers, and its rules and checks do not change in substance; `CONTRIBUTING.md` instructs people.

3. **Make the way in clearer.** Add to the top README a short "Start here" for a person new to the project: three or four links, in order, with one line on each, for a reader who wants to understand the language, one who wants to run what exists, and the owner's [setup guides](../guides/README.md). Do not restructure the documents beyond that, and keep the house style: plain sentences, the terms of `spec/definitions.md`, no marketing.

4. **Write the cold readers' instructions**, as `docs/guides/cold-readers.md`, for the owner's collaborators, who are to read the text as the first build leaves it (the "Read" row of the roadmap's order of the work). Build them on [for the next reader](../AUDIT.md#for-the-next-reader) and link to it rather than repeat it: what to read and in what order, what is known to be open, how to report a finding (where it goes and its form, as round 6's records show), what not to do (change `spec/` directly, read the reference clones, publish anything), and how long a read takes, if the records say. Add it to the table of `docs/guides/README.md`.

### Order 027

Work order 027 (I4-02): what the coordinator's run and order 025's review left open

Record, verbatim:

Run on 2026-11-10: the worker in 43 minutes, then a fresh verifier in 4, which found one fault:
the missing-object diagnostic chose its input by name, not by operation. The worker's second
round fixed it. The admin reran every check: 153 tests in `reference-model/` with
`CONDUIT_TEST_NPM=1` and 6 in `packages/agent-codex`, clippy, fmt and the repository checks pass.
Decision 6 did not hold with the real client: with `multi_agent`, `multi_agent_v2` and
`multi_agent_v2_dynamic_tools` disabled, and in further trials `collaboration_modes`,
`in_app_local_automation` and `agents.max_depth=0`, Codex 0.167.0 still listed its six
`collaboration.*` tools. Finding 4-06 stays open. The trials also showed `sleep_tool` controls
`clock.sleep`, and that a trial without the wrapper's `web_search="disabled"` lists `web__run`.

Own decisions (as written; the Record above states corrections):

1. **Finding 4-04, the retire hook.** Choose the hook's own implementation before preparing any run, so that a remote hook is connected to as TPL-018 says and an unrelated run is never started for it; record a run started for the hook as a part of the retirement with its effect; and where the run's state is unknown, do not dispatch the hook and record its part unavailable, with no effectful part invented (TPL-018, OPS-022). A test for each, failing before and passing after.
2. **Finding 4-05, the runner test.** Make the test wait for the PID file's contents, not its existence. Change nothing else in the runner for it.
3. **Finding 4-07, a store that cannot be repaired.** `conduit serve` starts even where setting up a registered instance fails: the failure is that instance's state and its records, as for any setup that fails, and serve listens, so that `submit` and the other commands can repair what is missing. Test it: register a project whose Resource's setup fails for want of root's Conduit to its Pool, then serve, submit that Conduit, set the Resource up through it, and see it ready.
4. **Finding 4-09, inspection.** (a) `conduit inspect` works while the engine serves, by asking the serving engine as the root Interface, as the author's commands do; without a serving engine it behaves as now. (b) Inspecting an instance lists every Conduit of which it is the consumer, the standalone ones with the declared ones, each with its name, target and whether it is open. If the core package's output for `inspect.v1` has no place for this, do not change `spec/`: report it as a finding and add it to the CLI's output alone.
5. **Finding 4-10, the diagnostic.** Where a request names an object that does not exist (OPS-029), the diagnostic's text says that the object named does not exist, naming its kind and name as given, and that a closed Conduit and a removed instance no longer exist.
6. **Finding 4-06, Codex's own agent tools.** Codex 0.167.0 offered `collaboration.spawn_agent`, `collaboration.send_message`, `collaboration.list_agents`, `collaboration.wait_agent`, `collaboration.interrupt_agent` and `collaboration.followup_task` with `multi_agent` disabled. Find, from `codex features list` run with `--ignore-user-config` and from the binary's own strings if needed, which features offer them (likely `multi_agent_v2` or a related one), and disable each in `packages/agent-codex` as the other machine tools are disabled, with a test of the arguments. You cannot run Codex as a client; the admin checks it with the real client afterwards. Keep the package's version at 0.1.1 and say in its README that the admin's check is pending.

### Order 028

Work order 028 (I4-03): commands an author can rely on

Record, verbatim:

A first run on 2026-11-10 ended after 44 minutes with no change made: the worker printed a large
fixture whole and passed the model's output limit. The admin ran the order again with a note
against printing whole fixtures. The second run took 50 minutes, then a fresh verifier 5, which
found two faults: a run stopped by the runner still showed `RunRunning`, and failures after a
session's initialization bypassed the descriptive error. The worker's second round fixed both.
The worker reported one finding, that the specification has no form for a condition that has
ended (LIF-011, OBS-002); the CLI shows such conditions apart with their end, in its own output.
The admin reran every check: 163 tests with `CONDUIT_TEST_NPM=1`, clippy, fmt and the
repository checks pass.

Own decisions (as written; the Record above states corrections):

1. **Exit status (4-13).** Every command exits non-zero when any result it prints did not succeed, `isError` or an outcome other than `succeeded`, and zero only when every one did. `submit` prints, after its results, one line per step it took (registration, each open, each setup) with that step's outcome, so that a partial success says exactly which steps succeeded.
2. **Discovery (4-14).** `conduit --help` and `conduit COMMAND --help` describe every command, its arguments and an example. `tools --as NAME` accepts `NAME` or `KIND/NAME`, and where the instance exists but has no presented tools yet, or does not exist, says which, and what to do. Every error a command prints says what was asked, what failed and, where the engine knows it, what would fix it. Update both Agent packages' READMEs to the `install` command in place of `register --package`.
3. **Timeouts (4-15).** Find why the commands' client intermittently times out initializing its session with the serving engine (the fresh author saw it with calls in parallel and in sequence). Fix the cause if it is the engine's or the client's; if it is not found, the error names the step and the time waited, and the client retries initialization, never a request already sent. Add a test that runs several commands at once against one serving engine.
4. **Stale conditions (4-16).** Inspection shows a condition that no longer holds as past (with when it ended), not beside the current state as if it held. Do not change `spec/`; if the specification's status form gives no place for this, report it as a finding.
5. **S1's packages are packages like any other.** `conduit register` stops preinstalling S1's filesystem and history snapshots: the engine knows nothing of S1. Every test, fixture and README that relied on it installs them with `install` (or the library's equivalent) first. The conformance vectors that read S1's snapshots from fixtures keep doing so, as fixtures.

### Order 029

Work order 029: the documents after rulings 49 to 54

Record, verbatim:

Run on 2026-11-10: the worker in 14 minutes, then a fresh verifier in 4, which found three stale
statements. The admin widened the scope for a second round to those, three outside-scope findings
of the worker's own, the removal of the rule that ruling 57 revoked, and the mention of rulings 55
to 59. The worker's links to the newest ruling records failed in its tree, which predated them, and
resolved after the rebase; its Rust run could not bind loopback ports in its sandbox, and no code
changed.

Own decisions (as written; the Record above states corrections):

1. **`spec/README.md`, its status only.** Say, dated 2026-11-10, that rulings 49 to 54 are accepted and not yet written into the specification, each in one line with a link, and that the language's shape is being settled first so that each is written once. Change no promise, schema or other file under `spec/`.

2. **The Codex Agent's stated routes.** In `packages/agent-codex/README.md` and in `reference-model/CONFORMANCE.md`, state finding 4-06 as settled (`docs/proposals/first-build/log-increment-4.md`): Codex 0.167.0 keeps six `collaboration.*` tools under every switch tried; a sub-agent it spawns, in the admin's trial, had no shell, could not read outside its sandbox and was refused a write by the read-only sandbox, and its requests of the engine carry the parent Agent's credential; so it is a route to more parallel model work within the Agent's own access, not around it. Keep the statement of finding 3-07 (Codex reads its home's `AGENTS.md`) as it is.

3. **Indexes.** In `docs/README.md`, list the proposal records added since order 026: `sign-in/`, `enrollment/` (with `networking-paradigms.md` and `verification-methods.md`), `simplification/`, `language-shape/`, `owner-direction/` (both records), and rulings 49 to 54, each with one line on what it holds, in the document's existing style. In `docs/proposals/first-build/README.md`, make increment 4's row state its findings and its two runs from the log. In `docs/work-orders/README.md`, list orders 027, 028 and this one.

4. **A staleness pass.** Read `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/OVERVIEW.md`, `reference-model/README.md` and `docs/guides/` for statements made stale by rulings 49 to 54 or work orders 025 to 028 (for example the commands `install`, `submit`, `replace`, `retire`, `unregister`, `--help`, and that `register` no longer preinstalls S1's packages). Fix them from the records, dated, keeping each document's role. Report each change as a finding.

### Order 030

Work order 030: the checks and code dependencies the condensation needs

Record, verbatim:

Run on 2026-11-10: the worker in 22 minutes, then a fresh verifier in 7, which found two faults:
Markdown autolinks bypassed the permalink check, and the copied safety traces ended with an extra
blank line. The new checks also found eight existing problems: six GitHub line-range anchors in the
archive, which the second round accepts when the lines exist, and two stale paths in tool comments.
The worker reported finding WO030-F1: the manifest's `safety_rows` tags take 21 values, of which
only four are trace names of S1's proposed safety traces, so what counts as a safety vector is not
yet decided. The admin reran the Rust suite outside the sandbox: 163 tests pass; the five
repository checks and the tools' unit tests pass.

Own decisions (as written; the Record above states corrections):

1. **`tools/check_schemas.py`, a new step** (the plan's E15, without its third check): every promise under `spec/promises/`, `spec/conventions/` and `spec/interface/` has exactly the seven fields, its layer and level take the allowed values, IDs are unique across the three, and no live ID is a retired ID named in a head comment's register of removed IDs or in `spec/README.md`'s list of promise files that rulings removed. The rules come from `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md` lines 136 to 151. If current text fails, report each failure and do not fix it.

2. **`tools/check_docs.py`, permalinks.** A link of the form `https://github.com/shrav-k/conduit/(blob|tree)/<sha>/<path>#<anchor>` is checked: the path with `git cat-file -e <sha>:<path>`, the anchor against the headings of `git show <sha>:<path>`, using the same anchor rules as for local links. Outside a Git tree with that history, the check is skipped with a notice. Other external links stay unchecked.

3. **A path-mention check**, in `tools/check_docs.py` or a new step: repository-relative paths named in `tools/*.py`, `conformance/manifest.yaml` and `reference-model/` sources must exist. Report what fails today; fix only what this order's items 4 and 5 move.

4. **The frozen S1 set** (the plan's E14): `conformance/s1-promises.yaml` holds the 308 promise IDs of `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md` section 2, verbatim. Point `reference-model/crates/conformance/src/lib.rs` (around lines 145 to 147 and 206 to 260), `conformance/manifest.yaml` (around line 6883) and `conformance/README.md` (around line 399) at it. Remove the loader test `reference-model/crates/loader/tests/s1_drafts.rs`, or point it at `reference-model/examples/s1`; repoint `conformance/fixtures/s1/README.md` line 7.

5. **S1's proposed safety traces** (the plan's E36, as the critic corrected it): copy `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md` section 3 (around lines 515 to 594) verbatim to `conformance/s1-safety-cases.md`, headed as **the safety traces proposed for S1 in increment 2**. Do not say it defines the manifest's `safety_rows` tags: the manifest uses about 21 tag values, and only four are trace names of that section, so no one has yet decided what "safety vector" means. Add no check on `safety_rows`. Record the mismatch, with the counts, in your report as a finding; the admin carries it to the open items.

### Order 031

Work order 031: a spike of the short form's translation, on S1

Record, verbatim:

Run on 2026-11-10: the worker in 14 minutes, then a fresh verifier in 4, which found nothing to fix
and reproduced the results. The exact translation is refused by today's kernel because it emits
`as`, one of ruling 53's additions not yet in the specification (finding 031-01); with `as` dropped,
all fourteen states register. In flow style S1 costs 55 lines on paper against the baseline's 64,
better on two tasks and worse on T1 and T7; in block style 97. The admin reran the spike's tests:
9 pass.

Own decisions (as written; the Record above states corrections):

1. grants refer to Conduit templates for reuse (`from: read`), and operation names stay fully qualified wherever they are written (ruling 25);

2. a reference names its kind (ruling 19 is kept): write `to: {kind: Resource, name: shared}`;

3. the frame is optional; the version comes from the manifest;

4. names stay stable: a replacement retires the old object and makes the new one under the same name.

5. The Fleet family of rulings 59 to 62 does not exist in today's core, so this spike writes no `fleet:` key; it records how many lines `fleet: s1` would add.

6. **A new experiment**, `experiments/011-short-form-spike/`, in Rust (its own `Cargo.toml`, `Cargo.lock`, a pinned toolchain, ignored build output), depending by path on the loader and core crates of `reference-model/`. Its README states, before the results, what it must show and how each line is counted: non-blank lines an author writes, separators included, comments excluded, each command line that carries a value the author chose counted apart (the [measurement plan](../proposals/first-build/increment-4/measurement-plan.md)).

7. **A translator** from the sectioned document to canonical documents, implementing the translation table of the grammar section as amended above. It is a spike, not the reference implementation: keep it small, and say in the README what it leaves out.

8. **S1's states in the short form.** Write the author's files for S1 after each of T1, T2, T4, T5, T6 and T7 (the tasks are in `docs/implementation/FIRST_BUILD.md`), in two styles: flow style, as the synthesis wrote it, and idiomatic block YAML, one key per line. Use the shipped packages' addresses and digests from `conformance/fixtures/s1/`.

9. **Check each state**: translate it, and register the result on a fresh state through today's loader and core, as the conformance runner does for a submitted project. Each must be accepted. For T5 and T6, also check that the translation of the new state differs from the old only by the objects a stable-name replacement would retire and make.

10. **Count**, per task and per style, the lines an author writes to go from the previous state, and compare with the baseline (33, 16, 0, 8, 3, 2, 2) and the coordinator's run (154, 73, 0, 2, 62, 29, 12). Put the dated results in `experiments/011-short-form-spike/results-2026-11-10.md`, with every limit stated: a spike's paper count is not a run (ruling 45).

11. **The register.** Add the experiment's row to the roadmap's register as A79, with its state, and its line in `experiments/README.md`.

### Order 032

Work order 032: the specification without its change narrative

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. Change: `spec/` comments and the five Markdown files named, only. Report anything else.

2. **`spec/README.md`**: its "History" section becomes "Where the reasons are": one line each linking the records that hold the reasons (the plan's E34 list), and `docs/proposals/ruling-log.md` written as a future link only if it exists when you finish (it may not; then link the ruling log in `docs/AUDIT.md`). The other history paragraphs of the five Markdown files above are condensed the same way.

3. **Every YAML head comment under `spec/`** keeps: the sentences describing current content; open issues; "Standards followed" where present (for example `providers.yaml`, `secrets.yaml`); and a **register of removed IDs**, one line each, "Removed: OLD → where it went (ruling NN)" or "OLD: nothing replaces it (ruling NN)", covering every promise and DL ID that the head names at C0 and that is no longer live, keeping every "not reused" statement. Add one standing line: "This file carries no change narrative; the reasons are in the records listed in spec/README.md, 'Where the reasons are'." Drop the rest of the narrative. A path into a proposal that survives becomes the C0 permalink (`https://github.com/shrav-k/conduit/blob/<C0>/<path>`).

4. **Gaps, not deletions.** A dropped sentence that states a behavior no promise holds is reported as a finding (a specification gap), and kept until the admin decides.

5. **Gates, each run by script and reported**:

   - for every YAML file, `yaml.safe_load` before equals after (JSON files untouched);

   - every promise's text is unchanged, compared by script;

   - every ID named in a C0 head comment that is not a live ID is named in the new registers or in `spec/README.md`'s list of promise files that rulings removed, and the same for DL IDs;

   - "Standards followed" sentences are present;

   - the words before and after, per file and in total.

6. If any digest or fixture that the shell or the vectors read changes, stop and report: do not regenerate anything.

### Order 033

Work order 033: `reference-model/` becomes `engine/`

Record, verbatim:

Run on 2026-11-10: the worker in 21 minutes, then a fresh verifier in 5. The directory moved with its
73 files; every path, link, tool and test that named it follows, and the live documents call it the
engine. The verifier found that the readiness conditions of A42 and A59 had become "the engine and
the vectors agree with the promises" where ruling 46's pure core is meant; the admin restored
"core". It also found three Python bytecode files tracked since order 032's merge, holding local
paths: the admin removed them from the tree and ignored `__pycache__/`. Continuous integration
merged while the order ran, so the admin renamed its paths too. Records under `docs/proposals/`,
`docs/archive/` and `docs/work-orders/` keep their words, with links repointed. The admin reran the
suites outside the sandbox: 163 tests pass in `engine/`, 3 ignored, and 9 in experiment 011.

Own decisions (as written; the Record above states corrections):

1. Do not change a promise, a schema, a vector's expected outcome, a fixture's bytes or a digest. If any digest or fixture would change, stop and report.

2. **Move the directory** with `git mv reference-model engine`. Its crates, their names and the binary's name stay as they are. Delete nothing; `engine/target/` stays ignored.

3. **Every path that names it**, in every tracked file, becomes `engine/`: Markdown link targets, relative paths in YAML and TOML (for example `conformance/language/registration/live-agents/case.yaml` and `experiments/011-short-form-spike/Cargo.toml`), `.github/workflows/`, and the tools (`tools/set_names.py`'s default roots, `tools/check_docs.py`'s path-mention check and its pattern, `tools/check_schemas.py`'s text, `tools/tests/`). Code under the new `engine/` that names its own directory changes too.

4. **The live documents' word.** In `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `ROADMAP.md`, `docs/README.md`, `docs/OVERVIEW.md`, `docs/AUDIT.md` (its tracker prose, not its ruling log), `docs/guides/`, `docs/implementation/`, `conformance/` READMEs, `spec/README.md` and the new `engine/`'s own documents, text that calls this directory or its code "the reference model" says "the engine" (or "the reference engine" where ruling 47's sense matters). Where a sentence means ruling 46's pure core, keep "core". Keep the sense of every sentence.

5. **Records keep their words.** Under `docs/proposals/`, `docs/archive/`, `docs/work-orders/` and every experiment's dated results, change only link targets and paths that a check reads; leave their prose, link text and quotations as written. Never change an owner's quoted words or a ruling's decisions (ruling 64 itself says "`reference-model/` is renamed `engine/`": keep it).

6. **Find what is left.** After the edits, `git grep -n "reference-model"` and `git grep -n -i "reference model"` must show only lines in the records of item 4, or lines that speak of the old name on purpose. Report the count of each, and list every live-document line that remains with the reason.

7. [Ruling 64](../proposals/ruling-64/README.md), point 4: `reference-model/` is renamed `engine/`, because [ruling 47](../proposals/ruling-47/README.md) makes v0 the reference engine, which that directory holds. No code, promise, vector or crate name changes: only the directory's name, every path that names it, and the live documents' word for it.

### Order 034

Work order 034: the ledger of rulings, the condensed tracker, and records that say whether they hold

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. A draft of the ledger's table, written from a reading of every ruling, follows this order under "From the admin session". It is a starting point: verify every row against the record and the AUDIT row it summarizes (the gist, the date, the owner's words character for character, the status against each later record's list of what it amends), correct what is wrong, and report each correction. Remove its bracketed marks such as `[10]`; they pointed at questions now carried in `docs/AUDIT.md`.

2. The plan was written beforerulings 57 to 67 were numbered. Where it says "ruling 57" for the acceptance of earlier logged decisions, read ruling 58, point 1; its "WO-031" is this order. **C0**, the commit every "last version" link of the condensation points to, is `f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

3. **The ledger, `docs/proposals/ruling-log.md`** (new), in ruling 67's form, point 3:

   - First line under the title: "History. What holds now is [IN_FORCE.md](../../IN_FORCE.md); the specification alone binds behavior." Then ruling 67's precedence, in one paragraph.

   - One table: `| # | Date | Ruling | The owner's words | Status |`, one row per ruling, 1 to 67. *Ruling* is a gist of 25 words or fewer linking the record (or "no record" for rulings 1 to 6, 9 to 12, 16, 26 and 34 to 36, whose only record is their row). *The owner's words* are verbatim and whole when they run to 40 words or fewer, copied from the record or the AUDIT row that quotes them; otherwise "In the record." Never excerpt. For rulings 1 to 37, where no words are marked as quoted, write "(the coordinator's summary)". *Status* uses only ruling 67's words: in force, done, directive (A-NN), open (points), amended by N (points), replaced by N, withdrawn; combine with semicolons where points differ. Take each "amended by" from the later ruling's own list of what it amends and from the tables of later amendments, and only from those; the admin's draft in the section below is a starting point to verify, not to copy.

   - Appendices, headed as history, verbatim, as E1 lists them: the preamble (AUDIT's ruling-log preamble), "Rulings 1 to 37 as given" (AUDIT's rows 1 to 37), the multi-system options under ruling 5, "Rulings before the plan audit", the tables of later amendments with rows added for rulings 45 to 67 from each record's list of what it amends, the round-5 changes, and the record-specific ID prefixes. Above the amendments tables: "The ledger's status column says what is current; these tables are kept as written."

4. **`docs/AUDIT.md`**, condensed as E2 says, with these changes from ruling 67:

   - Its "Ruling log" section becomes three lines: where the ledger is, where IN_FORCE.md is, and the precedence. Keep the headings `## Ruling log` and `### Later amendments` as stubs, so links to them resolve.

   - "Waiting for the owner" holds only what is waiting today: the metrics proposal (`docs/proposals/metrics/README.md`); what counts as a safety vector, which ruling 47's gate ("Every safety vector passes") needs (finding WO030-F1 of work order 030's record); A20, after the bypass probe A7; ruling 62's C6 to C12, after the S1 rerun; items 15 to 19 of the review list and increment 4's open decisions (ruling 58, point 1); four earlier rulings whose status is unclear (ruling 5's option (b), ruling 7's PL8 on fork-from-checkpoint, ruling 46's fallback of two Claude Code instances against ruling 47's gate, and ruling 1's reference clones after increments 1 to 3); and the readiness gate's G8 and G11. Rulings 59 to 66 settled the Projects rows, ruling 63 the counting rules and ruling 65 the public name: remove those rows. Do not add items of your own; report them.

   - "Known to be open" as E2 lists it, plus: findings 032-G1 and 032-G2 (the two passages of `spec/promises/secrets.yaml`'s head that no promise holds, in work order 032's record) and finding 4-18 (the core's speed, in increment 4's log).

   - Every heading that a tracked file links stays, as a stub if needed.

5. **Status lines.** Every `docs/proposals/ruling-NN/README.md` opens, under its title, with one line in ruling 67's form: `Status: <the ledger's status>. Holds now: <IN_FORCE.md's topic, by link, and the promises, where a promise holds it>. This record says why.` Replace the record's existing status sentence with it, keeping the date and "ruled by the owner" in it. A record that holds several rulings (for example `placement/`, `naming/`, `conventions/`, `schemas-and-interface/`, `reaudit/`) gets one line naming each. Change nothing else in a record: never the owner's words or the decisions.

6. **The check**, in `tools/check_docs.py` or a new `tools/check_rulings.py` run beside it, with a unit test under `tools/tests/`: one ledger row per ruling from 1 to the highest `ruling-NN` directory; each status cell uses only ruling 67's words; every `ruling-NN/README.md` has a status line as its first paragraph; no line of IN_FORCE.md's decisions holds a date. If you add a new tool, add it to AGENTS.md's list of checks and to `.github/workflows/checks.yml` if that file exists.

### Order 035

Work order 035: the contributor rules, the readers' guides, the roadmap and the indexes

Record, verbatim:

Run on 2026-11-10, in parallel with orders 033 and 034: the worker in 10 minutes, then a fresh
verifier in 5, which passed it with one nit: the protocol watch list copied into the cold readers'
guide referred to verdicts and rows of its source report, so the admin named that report. The
worker reported that `docs/implementation/WORK_ORDERS.md` still gave A78's old trigger (finding
035-01); the admin corrected it to ruling 61's (B12). The ledger is named as a path until order
034 merges; the rename of order 033 was applied to this order's text at merge.

Own decisions (as written; the Record above states corrections):

1. The plan was written beforerulings 57 to 67 were numbered. Where it says "ruling 57" for the acceptance of earlier logged decisions, read ruling 58, point 1; where it says "Accepted by ruling 57 (point 2)", write "Accepted by ruling 58 (point 1)". **C0** is `f1cb473493fbb7b8667686d6fdf639ae96579ec3`.

2. **E19, `CONTRIBUTING.md`**: as the plan lists, including the section "Writing promises", the removal procedure, and the rule that a specification head comment holds what the file holds, its standing, reading notes, the standards it follows, open issues and its register of removed IDs, and no change narrative (ruling 58, point 6; the register form is `Removed: OLD → NEW (ruling NN).` or `OLD: nothing replaces it (ruling NN).`). Add ruling 67's rule for a new ruling: one change writes its record, its ledger row in `docs/proposals/ruling-log.md`, the status lines and rows of the rulings it amends, IN_FORCE.md's lines, and any action item; an answer that changes no decision is added to the existing record, dated.

3. **E3, `docs/guides/cold-readers.md`**, as the plan lists.

4. **E24's one-line notes**, as the plan lists, with the ruling numbers corrected as above.

5. **E33, `ROADMAP.md`**: step 3's state in two lines; A77's and A71's states corrected from their records; A79 cited. Also: A78's row, whose trigger "once increment 3 runs" ruling 61 (B12) replaced: work orders go through Conduit after Fleet authority (batch 2), each worker an Agent of the `conduit-core` Fleet with a coordinator Agent appointed by root. And the sentence of "the order of the work" that has S1 and S2 also pass "in both independent builds": ruling 47 made the independent builds inform the owner's judgment without gating v0; say so, citing ruling 47.

6. **`docs/README.md`** becomes a map: what to read first (AGENTS.md's list), where decisions are (IN_FORCE.md, the ledger, the records), where the state of the work is (`docs/AUDIT.md`), and an index of the records that remain under `docs/proposals/`, one line each, including `ruling-67/`, `metrics/` and the ledger. Drop rows for what has left the tree. The ledger does not exist in your tree yet (order 034 writes it): name it as the code path `docs/proposals/ruling-log.md`, not as a link, here and in item 1; the admin links it at merge.

7. **The counting rules of ruling 63** go into `docs/proposals/first-build/increment-4/measurement-plan.md`, beside the rules it already applies, in ruling 63's words, with a link to the record.

### Order 036a

Work order 036a: spec batch 1, the Fleet family and the access additions

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. **Write 036a as the plan states it**, in its internal order: the rows of `spec/definitions.md`; the objects and kinds (OBJ-001, 002, 013, 020 with `as`, `owner` and Fleet; FLEET-001 to 006; CON-024, CON-027, CTR-004, TPL-001, TPL-002, PLC-006); access (ACC-024; ACC-033 written whole with the rows the plan lists; ACC-044, 053, 054, 055, 057; ACC-005, 006, 012, 022, 029; OPS-008, 014, 019, 029 and OPS-036; REV-010, REV-014); the lifecycle (LIF-040, LIF-041 and the amended LIF, CON, REC, FST, PRV and DL entries); the interface (IFC-012, 014, 025, 033); and `spec/README.md`'s contents and limits. New promise files follow the house form; `spec/README.md` lists any new file.

2. **The plan's drafts are drafts.** Write each promise in the specification's form and terms (seven fields, the writing rules in `CONTRIBUTING.md`, the words of `spec/definitions.md`), with the meaning the plan gives it. Where a draft and a ruling's record differ, the record governs; report the difference. Where the plan applies a reading the owner may strike, write it and list it in your report.

3. **IDs.** Use the IDs of the plan's C1. Check each against every head comment's register of removed IDs and `spec/README.md`'s list before using it; `tools/check_schemas.py` fails a reused ID.

4. **What this order does not write.** The core package's contracts and schemas (order 036b), the short form (037), refusals, pending actions and sign-in (038a), enrollment (038b), and anything of ruling 61 (batch 2). Where a promise here must cite one of them, write the citation the plan gives and list it; if `tools/check_schemas.py` then fails because the cited contract does not exist yet, stage that sentence as the plan's C25 describes and report it.

5. **IN_FORCE.md.** Each *spec pending* line this order writes in full becomes a pointer to the promises that now hold it, in that file's form; a line written only in part says which part remains and which order writes it.

6. **The vectors and the engine do not change in this order.** Run the engine's suite to show it still passes on the changed text, and report every vector whose expectation the new text contradicts (by path and step), for the build orders.

### Order 036b

Work order 036b: spec batch 1, the core package and schemas for 036a

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. The plan's drafts are drafts: write each item in the specification's form and terms, with the meaning the plan gives it; where a draft and a ruling's record differ, the record governs, and you report the difference. Each *spec pending* line of [IN_FORCE.md](../../IN_FORCE.md) that this order writes in full becomes a pointer to what now holds it; a line written in part says which part remains and which order writes it.

2. **Write 036b as the plan's work-order section states it**: `fleet.v1`, `fleet-management.v1`, `list.v1`, `retire-with-agents.v1`, `lift.v1` (input `id`); the ten templates of conflict C8, without `answer` and `sign-in` (order 038a adds them); the schemas inventory (with scope and administrators), tombstone (C18), provenance, fleet-reference and trust-domain; the Fleet, `as` and owner branches of `instance.v1`, `template.v1` and `type-spec.v1`; `expectedId` on open.v1 and close.v1; the first-start example; and the contracts the plan names as changed.

3. **Staging (C25).** What the engine would reject today goes, verbatim and in final form, into `docs/work-orders/036b-staged/<order>.md`, one file per build order that lands it (039, 040, 041a, 049a, 049b), each edit with its target file and the exact text. Then apply all staged text in a scratch copy of the tree (outside the repository), run `tools/check_schemas.py` and `tools/check_docs.py` there, and report both last lines.

4. **Citations.** Every promise ID a contract or schema cites must exist after this order (check_schemas step 5).

5. The engine reads `spec/core/` and `spec/schemas/` at run time, so this order lands only the edits that keep the engine's tests green and stages the rest verbatim for the build orders that will land them (the plan's conflict C25).

### Order 036k

Work order 036k: a report of what changed in the vectors

Record, verbatim:

Run on 2026-11-11: the worker in 8 minutes, then a fresh verifier in 6. The worker wrote
`tools/vector_delta.py` with 18 tests: it pairs cases by `id` and steps by position, reports flips
of reason, outcome, acceptance, tool and `stateUnchanged`, every other change by JSON Pointer, and
fixture changes by hash, and fails on any flip or undeclared change. Today's tree shows no change;
order 033 changed three fixture paths and no expectation. The verifier found that its YAML fallback
used YAML 1.1's scalars, so `no` and `off` both read as false and a changed tool description passed
unseen; the admin gave it YAML 1.2's core schema, as the engine and `check_schemas.py` read, with a
regression test.

Own decisions (as written; the Record above states corrections):

1. **`tools/vector_delta.py`**, Python 3 with the standard library (and PyYAML only if the case files need it; they are JSON written as YAML, and some hold characters PyYAML refuses, so prefer `json`). It compares the vector cases under `conformance/` between two Git revisions (default: `origin/main` and the working tree) and reports, per case file and per step:

   - a case added, removed or renamed (by its `id`);

   - every **flip**: a step whose expected reply changes its `reason`, its `outcome`, whether it is accepted, its `tool`, or `stateUnchanged`, printed as before and after;

   - every other change to an expectation, summarized by its JSON path;

   - changes to inputs and files that a case reads (fixtures), summarized by path. Read `conformance/README.md` ("Inputs and files", "Assertions") for what a case holds.

2. **Declared shapes.** An order may pass a file of declared shapes (`--declared FILE`, YAML or JSON): a list of entries, each naming a kind of change the order means (for example "a step's reason goes from refused to conflict in these cases", or "these JSON paths gain these keys in every S1 registration case"), with case globs. The report marks each change as declared or undeclared, and the tool exits with status 1 when any flip or undeclared change remains, so a verifier can run it as a check. Design the declaration format simply and document it in the tool's help and in `conformance/README.md`.

3. **Tests** in `tools/tests/`, on small cases written in the test: an added case, a flip of each kind, a declared and an undeclared change, a renamed case, and an unreadable file reported rather than crashing.

4. **Today's tree**: run it between `origin/main` and the working tree (no change: empty report, status 0) and between the commit before order 033 and after it (paths only: report what it finds), and put both last lines in your report.

5. No expectation may be regenerated from engine output: a changed outcome must be meant.

### Order 060

Work order 060: the re-audit's working papers leave the tree

Record, verbatim:

Run on 2026-11-11: the worker in 16 minutes, then a fresh verifier in 6. The worker removed the 57
files of removal list A, keeping 39,919 of the re-audit's 255,446 words: the round records, their
decisions and the "Choices that set behavior" extracted from the editors' logs (E35, 68 lines), with
every removed path linked at C0 and listed in the archive's index. It stopped short of three links
in the ledger, which the order had put out of its scope, and prepared a patch that changes only
their targets (finding WO060-01). The verifier found one choice the extraction missed, M75's dropped
wording and its reason; the admin added it and applied the ledger patch, mending two targets the
patch had joined without a slash.

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

2. **Extract first**, as removal list A and E4, E5 and E35 say: E4 into `round-6/DECISIONS.md`'s "For the owner" (anchor kept); E5's condensations of `reaudit/README.md`, `round-5/README.md`, `round-5/log-fixes.md` and `round-6/log-integration.md`; E35's "Choices that set behavior" sections in `reaudit/README.md` and `round-6/README.md`, one line per choice that names a rejected alternative, a candidate for later, or a reading a promise does not state, copied verbatim where it fits, with a count per log.

3. **Then remove** the files removal list A names, and only those. Before deleting each, check it is unchanged since C0 (`git diff --quiet f1cb473 HEAD -- <path>`); if one changed, stop and report.

4. **Repoint every link** into a removed file or section, in three classes as the plan's sequence says: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/` for a directory).

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group (what it was, what it is source material for, its last version at C0).

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.

### Order 061

Work order 061: the logs of rulings 38, 41, 42 and 43 leave the tree

Record, verbatim:

Run on 2026-11-11. The first dispatch stopped at the order's guard, as written: 27 of the 30 files
to remove had changed since C0, only in links that orders 033 and 060 repointed; the admin let
files whose only changes are repointed links go, and dispatched it again. The worker then took 21
minutes and a fresh verifier 5. It removed 30 files, the logs of rulings 38, 41, 42 and 43, for a net
100,954 fewer words, with ruling 41's planner's choices, its open decisions condensed in place and
each ruling's "Choices that set behavior" kept. The verifier found one choice missed (F3's rejected
option (b) and its reason, in ruling 42's integration log) and two links that pointed at an
extraction where the history meant the removed text itself; the admin added the choice and linked
both to C0. The shell's loopback tests could not run in the worker's sandbox; no code changed.

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

2. **Extract first**: E16 (ruling 41's planner's choices P1 to P20 into `ruling-41/README.md`, anchor `#planners-choices`); E17 (`ruling-41/open-decisions.md` condensed in place, "For the owner" kept verbatim with its anchors; "Left for round 5" dropped only for items you find settled, the rest reported for the tracker's "Known to be open"); E35's "Choices that set behavior" sections in the READMEs of rulings 41, 42 and 43 from their logs, one line per choice that names a rejected alternative, a candidate for later, or a reading a promise does not state, copied verbatim where it fits, with a count per log; MX-21 of ruling 43 into the ledger's appendix. Check that E19's parts from `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md` are already in `CONTRIBUTING.md` (order 035) and add only what is missing.

3. **Then remove** the files the list names, and only those. Before deleting each, check that since C0 it changed only in link targets or path mentions, which orders 033, 034, 035 and 060 repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else changed, stop and report that file, and go on with the others.

4. **Repoint every link and path mention** into a removed file or section, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change, their words may not.

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.

### Order 062

Work order 062: the batch records' logs leave the tree, and the older records condense

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

2. **Extract first**: E35's "Choices that set behavior" in `core-and-schemas/README.md` and `docs-audit/README.md` from their logs; E20's spot-check of the 22 "left undone" items of `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-documents.md` and [`log-promises.md`](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-promises.md), any unsettled one reported for the tracker's "Known to be open"; E18, `secrets/README.md` condensed in place at the same path, dropping only the sections the plan names, and "[Promises that would follow](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)" only after mapping each drafted SEC-n to `spec/promises/secrets.yaml` or principles-review's dropped list (any draft sentence carried by neither is reported); E20 for the thirteen older records, the vocabulary-audit, multi-machine and the READMEs the plan lists, deleting sections only, with the plan's keep lists; E31, the "State on 2026-11-06" of `docs/implementation/INDEPENDENT_BUILDS.md` moved verbatim, row by row, into `ruling-39/README.md` as "The assessment point 8 confirmed", and replaced in INDEPENDENT_BUILDS by the state now for G8, G9 and G10, marked as the coordinator's until G11 (ruling 58, point 7).

3. **Then remove** the files the list names, and only those. Before deleting each, check that since C0 it changed only in link targets or path mentions, which orders 033, 034, 035 and 060 repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else changed, stop and report that file, and go on with the others.

4. **Repoint every link and path mention** into a removed file or section, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change, their words may not.

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.

### Order 063

Work order 063: the first build's closed increments condense

Record, verbatim:

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

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order). Since the plan: `reference-model/` is `engine/` (order 033); work order 030 already did E14 and E36 and the `s1_drafts.rs` choice; ruling 63 settled the rerun's counting rules, which order 035 put in the measurement plan, so E11 adds only what ruling 63 did not settle and marks the rest settled; the plan's A79 is now the short-form spike, so its lean-vector work goes under A39 (finding 034-F1).

2. **Extract first**, as the plan's E6 to E10, E12 and E13 say: the settlements register in `docs/proposals/first-build/README.md` (one row per finding of increments 1 to 3: ID, decision, promises, vector, rejected alternative and why; D19 with its caveat verbatim; the two open rows; the two "noted for later" items); E7 into `increment-2/decisions.md`; E8's section "How to run the open checks" in the closeout results, with the sentinel method verbatim; E9 into increment 4's log and A68's done-when; E10's corrected mapping in `to-review-with-the-owner.md`; E12, "The core's boundary" in `engine/README.md`, rebuilt from the current code and not copied; E13's "Planned work" in `conformance/README.md`, with "Evidence of vector strength so far" verbatim. Check that what https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md's sections 4 to 7 hold is carried by `conformance/README.md`, `increment-2/decisions.md` or the code; report what is not, for the tracker's "Known to be open".

3. **Then remove** the files the list names, and only those. Before deleting each, check that since C0 it changed only in link targets or path mentions, which orders 033, 034, 035 and 060 repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else changed, stop and report that file, and go on with the others.

4. **Repoint every link and path mention** into a removed file or section, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change, their words may not.

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.

### Order 064

Work order 064: the language-shape, simplification and Project papers leave the tree

Record, verbatim:

Run on 2026-11-11: the worker in 14 minutes, then a fresh verifier in 5, then a second round. The
worker removed 31 papers, the simplification's research, the short form's designs and inventories,
and the Project investigations' papers, a net 270,808 fewer words, after extracting E21 into the
simplification's record, E22 into the short form's synthesis and what the Projects brief lacked of
E23. The verifier found four passages copied verbatim that had lost a space, and row IDs, such as
PRJ-010 and AUT-018, still pointing at definitions only the removed papers held; the second round
restored the spaces, compared all 31 extracted passages with C0 by script, and linked 47 row
references to their C0 definitions. The short form's synthesis and critic stay until spec order 037
lands.

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order). Since the plan: the brief E23 describes was written as `docs/proposals/projects/brief.md` (not under `https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project`); ruling 63 settled the rerun's counting rules (order 035 put them in the measurement plan), so E11 adds only what ruling 63 did not settle; order 063 did E13's planned work in `conformance/README.md`, so E36's to-do from `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-constraints.md` section 1 goes there only if it is missing. Keep `language-shape/short-form/synthesis.md` and `critic.md`: spec order 037 still works from them (the plan's "later, after the spec batches").

2. **Extract first**: E21 into `docs/proposals/simplification/README.md` ("Rejected, and why", copied verbatim from the synthesis, so that you do not select; the load-bearing guarantees; the v1-candidate reductions with their measured costs); E22's closing section "Carried from the papers" in `short-form/synthesis.md`; E23: check that `docs/proposals/projects/brief.md` carries its provenance quotes (verbatim, with their sources), every synthesis decision 1 to 20 with the options and why each rejected one lost, and the other points E23 lists, and add, verbatim and with the source, only what is missing; E9's and E11's parts not yet carried; E36's to-do if missing.

3. **Then remove**: `https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research` (all 13 files); `language-shape/short-form/`'s design-N1 to N4 and its four inventories; and `https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project` (all 10 files). Remove nothing else. Before deleting each, check that since C0 it changed only in link targets or path mentions, which earlier orders (033 to 063) repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else changed, stop and report that file, and go on with the others.

4. **Repoint every link and mention** of a removed file or section, found by path, by bare file name, by section name and by row ID, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change, their words may not.

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.

### Order 065

Work order 065: the research notes and the archive condense

Record, verbatim:

Run on 2026-11-11: the worker in 12 minutes, then a fresh verifier in 5. The worker removed 35 files,
a net 108,381 fewer words, from `docs/research/` and `docs/archive/`, with the migration checker and
its lines: the archive's index now says what each removed group was and what it is source material
for, carries FORMATS' paragraph, the collaborator proposals' "not adopted, and why" verbatim and the
prior art still applied; the plan-audit history keeps what E26 keeps; `docs/LANGUAGE_DEFECTS.md`
keeps its open rows in full and a crosswalk line for each other. The verifier found two statements
to correct: the count of promises the manifest's cases cite, which counted ranges by their ends
only (100, not 76), and "labels and selectors were not adopted", which read against PLC-007's node
selectors; the admin corrected both.

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

2. **Extract first**, as E25, E26 and E27 say: the archive's index rewritten with a row for each removed group, what it was and what it is source material for, FORMATS' paragraph, the collaborator proposals' "Not adopted, and why" verbatim, "Prior art still applied" with C0 links, and the attribution (E25); `docs/archive/plan-audit-history.md` condensed to what E26 keeps, its multi-system options linked to the ledger's appendix, which order 034 already holds; and `docs/LANGUAGE_DEFECTS.md` as E27 says, its 30 open type-layer rows and 16 runtime rows kept in full and every other row a crosswalk line, with "None has a vector" checked against the manifest. `docs/research/SIMPLIFICATION.md` stays as written. Where an extraction's destination was already written by an earlier order, check it and add only what is missing.

3. **Then remove** the files removal list E names under `docs/research/` and `docs/archive/` (including `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/tools/check_doc_migration.py` with the migrations, its CONTRIBUTING block and its row in `docs/README.md`, and the reviews only because the ledger already holds "Rulings before the plan audit"), and only those. Before deleting each, check that since C0 it changed only in link targets or path mentions, which earlier orders (033 to 063) repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else changed, stop and report that file, and go on with the others.

4. **Repoint every link and mention** of a removed file or section, found by path, by bare file name, by section name and by row ID, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change, their words may not.

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.

### Order 066

Work order 066: the experiments keep their method and results, and shed raw records

Record, verbatim:

Run on 2026-11-11: the worker in 12 minutes, then a fresh verifier in 2, which found nothing to fix.
The worker removed 942 files, 148,612 words: the raw run records of experiments 005, 007, 008 and
009, and the harness crates of 008 and 009, which the engine's shell and the Agent packages
superseded, so they leave continuous integration's matrix. Each experiment keeps its README, dated
results, HARNESS.md and evidence READMEs, 38 files and 46,100 words, with links into what left
pointing at C0; `experiments/README.md` gained "Evidence for v0" and the raw-records rule. Experiment
011 was not touched. One package test fixture's `source` note now names its C0 path; the admin ran
that package's tests: 6 pass.

Own decisions (as written; the Record above states corrections):

1. The plan predates later work.Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger, the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to extract into them is present, and extract only what is missing. The plan's line numbers refer to **C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

2. **Extract first**, as E28 and E29 and the section `experiments` say: each experiment keeps its README, dated results, HARNESS.md where a harness ran, its evidence READMEs and its counted artifacts; links into removed raw files become C0 permalinks; the stale lines the plan names are fixed; `experiments/README.md` gains "Evidence for v0" (A75) and the raw-records rule, and its 001 row moves to the archive's index. **Do not touch `experiments/011-short-form-spike/`** (work order 039 is changing it) or experiment 010's dated results. A file a live proposal cites as evidence (for example `docs/proposals/metrics/README.md` citing experiment 008's later-turn runs) may go only with its citation repointed to C0.

3. **Then remove** the raw records and superseded code the section `experiments` names, and only those (each experiment directory that loses its code also leaves continuous integration's matrix, which finds `Cargo.toml` files by itself; say which). Before deleting each, check that since C0 it changed only in link targets or path mentions, which earlier orders (033 to 063) repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else changed, stop and report that file, and go on with the others.

4. **Repoint every link and mention** of a removed file or section, found by path, by bare file name, by section name and by row ID, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says where future work goes (list each for the admin); otherwise to its C0 permalink, `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>` (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change, their words may not.

5. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.

6. **Report** the words removed and kept, the extraction counts, every link repointed by class, and `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and archive rows.
