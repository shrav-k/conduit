# Work order 165: the by-rule examples sent through the engine's public routes

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only because the
sandbox refuses a loopback bind without it, and the engine's existing shell tests, which
`cargo test --release` runs, bind one on 127.0.0.1. The new test binds none. Use the network for
nothing else: add no crate, and reach no other machine. Run one cargo command at a time, and
never print a whole large file or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 25 of [its orders](../proposals/full-review/orders.md))
found that no test sends the specification's by-rule examples to the engine
([FR-357](../proposals/full-review/findings/P07-engine-build.md#fr-357), as its verifier corrected
it). Each of the 41 files under `spec/schemas/examples/invalid-by-rule/` holds a set of documents
that the schemas accept and that the rule its name begins with must reject. The only tests over
them check that the schemas accept them, and that the loader's own check rejects the two SCH-016
files (`engine/crates/loader/tests/examples.rs:249` and `:269`). Nothing shows the engine rejecting
any of them. They are the cheapest test of the first build's second thesis, that the language
fully determines what a set of documents means (`docs/implementation/FIRST_BUILD.md:33`).

This order sends each set to a fresh engine as root, by the operation a client calls, classes
the reply, and keeps the classes as a test that fails when one changes. It is Part E's engine run
of A83, the blind-verdict test, to which the review gave FR-357, the number A83 and the directory
026 ([its changes to the experiments](../proposals/full-review/experiments.md#a83-the-blind-verdict-test);
D77 and D81 of [its decisions](../proposals/full-review/decisions.md)).
[Ruling 78](../proposals/ruling-78/README.md), point 1, puts the blind read itself after the pin;
question 1's sequence places this engine run before it. The order changes no behavior and fixes
nothing. The admin decides what each by-rule file that is not `named` shows, a build defect or a
finding against its promise, and logs it.

## The decisions it applies

1. **The route.** Each set goes to the engine from root's Interface, by `Shell::submit` of the
   `invoke` input that `call_tool` builds from a client's `tools/call`
   (`engine/crates/shell/src/server.rs:448`), without `serving` and the key, and with
   `MissingExecutor`, as the helper `invoke` in `engine/crates/shell/tests/changes.rs:57-59`
   sends it; root's ID is found as that helper finds it. The tool is `management.register.v1`,
   and its arguments are `documents` and `manifest`. A package goes by `management.install.v1`
   with a `file` URL, with the arguments that `boundary::install_arguments` gives
   (`engine/crates/shell/src/boundary.rs:114-139`), as
   `t5_public_install_is_live_and_existing_instances_keep_versions` (`changes.rs:484`) sends
   them. The shell then reads the archive itself (`engine/crates/shell/src/lib.rs:388-424`). Do
   not use the CLI's `register-project` input (`engine/crates/shell/src/main.rs:298`),
   `install-archive` (`boundary.rs:171`), `boundary::install_package`, or the module
   `tests/common`, whose `install` admits archives through `install_package`: no client can send
   those. Read each file with `conduit_loader::read_file`, and send each document as its JSON
   value, in the file's order. No by-rule file holds a package's documents: each goes by
   `register.v1`.
2. **A fresh engine per set.** Each set gets its own store in a temporary directory, opened by
   `Shell::open` with `boundary::context` of the repository root and `Config::default()`, as
   `changes.rs:486-491` opens one; it binds no port. First start is given this fixed machine,
   `{"hostname":"by-rule","labels":{"kubernetes.io/hostname":"by-rule","kubernetes.io/os":"linux","kubernetes.io/arch":"amd64"},"state":"reachable"}`,
   not `local_machine` (`lib.rs:708-732`), whose labels follow the host. A class is then the same
   on the Mac and on CI's Linux. `linux` is the system that `files-server`'s `nodeSelector` asks
   for.
3. **The scaffold, stated per kind** (FR-357's verifier: "a stated scaffold per kind").
   - **A set with its own manifest**, a document with no `kind`: seven files, the four named
     `-manifest-` and those of MAN-018, SEC-024 and TPL-003. That document goes as `manifest` and
     the others as `documents`, an empty list for the four. Nothing is added.
   - **Every other set** (34 files) goes with its documents and this manifest: the `apiVersion`
     that `Specification::names()` gives, `address: example.org/by-rule`, and, only where a
     document's value holds `example.org/files#` (12 files),
     `packages: {example.org/files: {version: 0.3.0, digest: D}}`, where D is the digest of the
     files archive below.
   - **`example.org/files`, for those 12 sets only.** Before the registration, root installs two
     archives, in this order, each by decision 1's route. Each is a gzip tar of regular files at
     its root, as `conformance/fixtures/s1/archives/filesystem-0.2.0.tar.gz` is, built once in the
     test's temporary directory:
     - a stub `example.org/common` 2.0.1 holding two files: `conduit.json`, the manifest
       `{apiVersion, address: example.org/common, version: 2.0.1, exports: [timestamp.v1]}`, and
       `contracts.json`, the one document
       `{apiVersion, kind: Schema, metadata: {name: timestamp.v1}, spec: {type: string, format: date-time}}`.
       The files package lists this package
       (`spec/schemas/examples/valid/files-package/conduit.yaml:24-28`), its `contracts.yaml:50`
       refers to `example.org/common#timestamp.v1`, and no example writes it out;
     - `spec/schemas/examples/valid/files-package`: its `contracts.yaml` byte for byte, and its
       manifest as the loader reads it, as `conduit.json`, with one change: the digest of
       `example.org/common` is the stub archive's.
   - **Nothing else is supplied.** A set may name objects and packages that no example writes out
     (`spec/schemas/README.md:257-258`). A rejection for one of them is class `other`, and the
     results say so.
4. **What is sent, and what is not.**
   - Each file under `invalid-by-rule/` is sent as one registration, and its class is the file's.
     NAM-028's file says that each of its documents must be rejected, and
     `SCH-019-input-default-fails-its-schema.yaml` says that its Operation and its template must
     be; each is still one set.
   - `valid/equal-defaults` is read with `conduit_loader::read_project` and registered as a set
     with its own manifest.
   - `valid/files-package` is installed as in decision 3, after the stub, in its own engine. Its
     outcome is that installation's.
   - Four are excluded by name and never sent. `valid/sectioned-project` and
     `valid/sectioned-package` are in the short form, whose translation awaits the translator
     (`spec/schemas/README.md:110-111`). `valid/first-start` is not a project
     (`spec/schemas/README.md:250`). `valid/project` lists `example.org/harness`, which no example
     writes out, and its items run programs by relative path, which `register.v1` rejects in a
     request with no directory (`spec/core/operations.yaml:127-129`). The review's Part E said
     `valid/project` must be accepted; this exclusion changes that before the run.
   - The test fails if a directory under `valid/` is neither sent nor one of these four, or if a
     file under `invalid-by-rule/` has no line in the table.
5. **The classes** (FR-357's verifier: "some error diagnostic names the rule", the reason that
   OPS-023 gives, and a class for a rejection by the scaffold; DL9 at `engine/CONFORMANCE.md:28`;
   IFC-050; D36). A class is read only from what a client sees: `isError`; the reason in `_meta`,
   under the key made of `shell.context.meta_prefix` followed by `reason`, or, where there is
   none, the outcome, under the prefix followed by `outcome`; and the `ruleId` of each diagnostic
   in `structuredContent.diagnostics` whose `level` is `error`. Each set falls in exactly one
   class:
   - `named`: a by-rule file whose reply has `isError: true` and the reason `invalid`, and one of
     whose error diagnostics, in any position, has as `ruleId` the ID that the file's name begins
     with. `invalid` is the reason `register.v1` gives for every rule these files are named for.
   - `other`: `isError: true`, and not `named`. A valid set that is rejected is `other`.
   - `scaffold`: an installation of decision 3 did not end with `isError: false`, so the set was
     not sent.
   - `accepted`: `isError: false`.
   - `error`: `Shell::submit` returned an error, or a reply that holds `error` in place of a
     result, as the core gives for a tool that is not presented
     (`engine/crates/core/src/requests.rs:584-586`) and `call_tool` passes on as a protocol error
     (`server.rs:465-468`).

   A by-rule file passes only as `named`, and a valid set only as `accepted`.
6. **The test and its table.**
   - `engine/crates/shell/tests/by_rule.rs` holds one test,
     `by_rule_examples_through_public_routes`. It runs every set. It prints the observed table
     whole, then the count of each class. Then, for each set in `other`, `scaffold` or `error`, it
     prints each error diagnostic's message, or the error. In each, it replaces the temporary
     directory's path with `TMP`: first its canonical form, which `install_arguments` puts in the
     archive's URL (`boundary.rs:118`), then the path as created. It asserts that the observed
     table equals `engine/crates/shell/tests/expected/by-rule-outcomes.txt`, and names every set
     that differs.
   - The table opens with lines that begin with `#`, as
     `engine/crates/loader/tests/expected/invalid-verdicts.txt` does. The comparison skips them
     and blank lines. They say what the table is and how it was derived: the command, the date,
     and the commit of `main` it ran on. They also say that a line changes only in the pull
     request that changes what the engine does with that set. Then comes one line per set, sorted
     by name, in the form `NAME CLASS REASON RULES`:
     - NAME is the file's name, or `valid/` followed by the directory's name;
     - REASON is `error` for class `error`; otherwise the reason that decision 5 reads, or the
       outcome where there is no reason, or `-` where there is neither;
     - RULES are the error diagnostics' rule IDs, distinct and sorted, joined by commas with no
       space, or `-`.

     For `scaffold`, REASON and RULES are those of the installation that failed. The table has 43
     lines besides its comments.
   - Write the table from the test's first run, and never change a line to make a result read
     otherwise.
   - The test writes none of the names that `spec/names.yaml` fixes: it reads them through
     `Specification::names()` and `shell.context`, as the other tests do. It reads the tree from
     `CARGO_MANIFEST_DIR`, binds no port, starts no process, and writes only in its temporary
     directory.
7. **Experiment 026 and A83's row** (the roadmap's rules for proposing an experiment; D78, under
   which a probe's result merges only with its row's change).
   - Create `experiments/026-a83-blind-verdicts/README.md`, titled
     `# 026 — A83: blind verdicts from spec/ alone`, with two parts:
     - one paragraph on what A83 asks and its four parts. It links the review's A83 section and
       D81, and ruling 78, point 1, for the parts that run after the pin, each of which is
       pre-registered here before it runs;
     - a section on Part E's engine run. It links this order's file under `docs/work-orders/` as
       the run's pre-registration, states decisions 1 to 5 in a few sentences each, and links the
       test and its table. It says that two things change the review's Part E text before the run:
       the exclusion of `valid/project`, and the reason `invalid` that class `named` asks for,
       from FR-357's verifier. It says why. It gives the review's expectation before the run, as
       an expectation: the files of the eight rules that no engine code named at the review's
       commit (CTR-009, PLC-028, PRV-001, PRV-004, PRV-016, SEC-001, SEC-022 and SEC-024) are not
       `named`, and SEC-002 has no example.
   - Create `experiments/026-a83-blind-verdicts/results-DATE.md`, where DATE is the day of the run,
     written as 2026-11-15 is. It holds:
     - the commit of `main`, and the command, beside every count;
     - the count of each class among the 41 files, and the outcome of the two valid sets;
     - the files in each class. For each file in `other`, `scaffold` or `error`, it gives the
       reason and the rules. It adds one clause on what the diagnostic names, and on whether the
       rejection is the scaffold's doing: something the set names that no example writes out, or
       something that decision 3 added;
     - apart, the class of each file of the ten rule IDs that FR-357's verifier found with no
       case: the eight above, PKG-030 and PLC-025;
     - SEC-002's missing example, as a finding for the admin to log;
     - the limits: root only; no HTTP transport and no token; one registration per file; decision
       3's scaffold; the four exclusions; and nothing of Part E's readers.

     It quotes no message text that holds a path.
   - In `experiments/README.md`, add this row after 023's, with DATE as above:

   ```markdown
   | [026 — A83: blind verdicts from spec/ alone](026-a83-blind-verdicts/README.md) | A83: whether a reader with only spec/ gives the engine's verdicts, in four parts. Part E's engine run sends the specification's by-rule examples through the engine's public routes. | Part E's engine run on DATE by work order 165; [results](026-a83-blind-verdicts/results-DATE.md). The other parts run after the pin. |
   ```

   - In `ROADMAP.md`, add this row to the register after A82's, with DATE as above. In the order
     of the work, add `A83` at the end of step 3's Items.

   ```markdown
   | A83 | Blind verdicts from `spec/` alone: does a reader with only `spec/` and the first-build profile give the engine's verdicts, name the deciding promise and place each promise in the layer that LD1's test gives; and is each of the specification's by-rule examples, sent through the engine's public routes, rejected under the rule it is named for? Four parts, scored apart: V, verdicts; R, resolving by hand; L, layers; E, the specification's examples | Part E's engine run: the first build. Parts V, R and L, and Part E's readers: SO-A and SO-B merged, then the S1 rerun's pin ([ruling 78](docs/proposals/ruling-78/README.md), point 1) | Part E: A38 and G4. The whole: A45, A34, A59's next read and G7; only a failing result goes to the owner, with its numbers | Each part's dated results in [experiment 026](experiments/026-a83-blind-verdicts/README.md), pre-registered there before it runs: Part E's engine run as work order 165 states it, and the other parts as [the full review's A83 section](docs/proposals/full-review/experiments.md#a83-the-blind-verdict-test) states them, with the review's D81 | in progress: Part E's engine run on DATE, on branch `wo/165-by-rule-examples-public-routes` (work order 165), [results](experiments/026-a83-blind-verdicts/results-DATE.md); the other parts run after the pin | The full review: [FR-357](docs/proposals/full-review/findings/P07-engine-build.md#fr-357), [FR-436](docs/proposals/full-review/findings/E05-falsifier.md#fr-436), [FR-437](docs/proposals/full-review/findings/E05-falsifier.md#fr-437) and D81 of [its decisions](docs/proposals/full-review/decisions.md) |
   ```

8. **What does not change.** The engine's source and its other tests, `spec/`, `conformance/`,
   `IN_FORCE.md`, `docs/AUDIT.md` and increment 4's log. Fix nothing that the run shows: report it.

## Scope

Create `engine/crates/shell/tests/by_rule.rs`,
`engine/crates/shell/tests/expected/by-rule-outcomes.txt`,
`experiments/026-a83-blind-verdicts/README.md` and
`experiments/026-a83-blind-verdicts/results-DATE.md`. Change `experiments/README.md` (one row) and
`ROADMAP.md` (A83's row, and step 3's Items). Change nothing else.

## Done when

From `engine/`, each of these passes, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --test by_rule
cargo test --release
```

The third command reports exactly 1 passed. A passing test's output is hidden, so add
`-- --nocapture` to that command when you need to read what the test prints. Then show that the
test can fail:
- change one line's class in the table, run the third command, and see it fail naming that file;
- delete one line, and see it fail naming that file;
- restore the table, and see it pass.

Report the lines that failed. From the repository root, the first command prints 43 and the second
prints 0:

```bash
grep -vc -e '^#' -e '^$' engine/crates/shell/tests/expected/by-rule-outcomes.txt
grep -cE 'register-project|install-archive|install_package|local_machine|mod common' engine/crates/shell/tests/by_rule.rs
```

Each of these passes:

```bash
python3 tools/check_roadmap.py
python3 tools/check_docs.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
git diff --check
```

`check_private_paths.py` and `git diff --check` read tracked files only. The admin therefore
stages the new files, runs `python3 tools/check_private_paths.py` and `git diff --cached --check`,
and shows the engine build, the four engine shards and the doctests passing on the pull request.
Do not stage anything. In your report, give each choice you made as your own, the count of each
class, and every check's last line.
