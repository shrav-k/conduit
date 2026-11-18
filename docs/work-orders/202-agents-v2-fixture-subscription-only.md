# Work order 202: the 0.2.0 fixture's `.v2` Types, subscription-only under ruling 83

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share: list in `vectors` the case whose expectations and fixtures you change,
with the promises it tests and whether it passes. This order has network access only because the
engine's tests bind loopback ports, which the sandbox refuses without it; use it for nothing
else, add no crate, and reach no other machine. Run one cargo command at a time. Never print a
whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never
run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Do not run `claude`, `codex` or any client,
and sign in nowhere.

## What and why

**When this order starts.** The record of the owner's ruling 83 must be merged to `main` first.
It brings `docs/proposals/ruling-83/README.md`, the ruling's row in the ledger, and the change to
`IN_FORCE.md` that replaces ruling 82, point 2's line on `apiKey` in `.v2`. Where the coordinator
dispatches this order from that change's verified branch before it merges, a dispatch note says so;
the note relaxes only "merged", never the checks below. This order merges before 050f is
dispatched (ruling 83's What follows). Order 048a-1 (merged, #352) brings the fixture's present
shape. Before you change anything, check from the repository root that you find what both bring, and stop, change nothing and report if any check prints otherwise:

```bash
test -f docs/proposals/ruling-83/README.md; echo "record $?"
grep -c '^| 83 | 2026-11-18 |' docs/proposals/ruling-log.md
grep -c '`.v2` holds `apiKey`, as an explicit setting only' IN_FORCE.md
grep -c '^## Record' docs/work-orders/048a1-build-sign-in-type-checks.md
grep -c 'Ruling 82 point 2: account and apiKey are both required; apiKey defaults to null.' engine/crates/conformance/examples/make_archives.rs
grep -c 'sha256:12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7' conformance/language/packages/agents-v2-defaults/install.json
```

They print `record 0`, `1`, `0`, `1`, `1` and `1`. No other written order changes
`make_archives.rs`'s `.v2` Types or the case `agents-v2-defaults`; if one lands first and the
checks above still print as stated, keep both changes and report it.

**What it changes.** The owner decided on 2026-11-18 (ruling 83) that the Agent packages 0.2.0
are subscription-only: `apiKey` is not in `.v2`; `account` stays required with no default, a
string or null; the `oneOf` of `account` and `apiKey` goes; and API access comes later as a
declared gateway route. The registration-only fixture of the packages 0.2.0, which order 044
shaped and order 048a-1 (its decision 6) made require `apiKey` under ruling 82, point 2, still
holds the old shape in both `.v2` schemas: `apiKey` among the properties, an `anyOf` of the core
package's `secret-reference.v1` and null with `default: null`; `required` `["account","apiKey"]`;
and a `oneOf` of a string account with a null key and a null account with a key. This order removes `apiKey` and the `oneOf` from both schemas,
and brings the case `agents-v2-defaults`, which installs the fixture and evaluates seven samples
against each `.v2` schema, into line. One behavior: the fixture's `.v2` configuration accepts an
account that is a string or null and refuses any key.

**What already holds, so that you do not redo it.** In `release_fixtures` of
`engine/crates/conformance/examples/make_archives.rs`, `account` is `{"type":["string","null"]}`
with no default, the schema sets `"additionalProperties":false`, `peerMessaging` is on
`claude-code.v2` only, the fixture states no `signIn` and no hooks (spec batch 1's C12), and the
seven sample values that the two `-values.json` files carry are listed after the loop. None of
that changes. The sample values themselves stay as they are, the two that set a key included:
under the new schema `additionalProperties` refuses them, which is what the case should show.

**The coordinator's probe.** The coordinator applied decisions 1 to 3 once, in a scratch clone of
`main` at `e4ceb4c8`, before ruling 83's record existed. `make_archives` reported the digest of
decision 2 and wrote only the five files decision 2 lists; the case failed with its old
expectations at step 3 (`/reply/samples/0/value/apiKey: missing required field`) and passed
with decision 3's; the negative control below failed and passed as it says; `fmt`, `clippy`,
`make_archives -- --check`, the vectors with 4 threads (2 passed), the spike (9 passed) and the
Python checks it ran (`check_schemas.py`, `set_names.py --check`, `check_rulings.py`,
`check_roadmap.py`, `check_model_account.py`, `promise_standing.py --check`) passed;
`engine/STANDING.md` did not change; and `vector_delta.py` against the declaration gave 19
changes, 0 undeclared. It did not run the full suite. That was a probe, not a result: your run is
the evidence.

**Who decided what.** The owner decided what the `.v2` schemas hold (ruling 83). Everything else
here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): keeping the seven samples, the two with a key
included; the case's new title; removing the core package's name lookup that served only
`apiKey`'s `$ref`; the comment's words; the declaration's shapes; merging before 050f is
dispatched; leaving `IN_FORCE.md` to ruling 83's own change; and leaving the documents that
decision 5 lists for later changes. Apply them as written and report them under `decisions` as the
coordinator's.

**Read first:**

1. [Order 048a1](048a1-build-sign-in-type-checks.md), its decisions 6 and 7 and its Record, and
   [order 044](044-build-package-fixtures.md), which gave the fixture its `oneOf`.
2. `docs/proposals/ruling-83/README.md`, and ruling 82's record, point 2, which it replaces for
   `apiKey`.
3. SCH-009 in `spec/promises/schemas.yaml`: a default is filled before the value is validated.
4. `release_fixtures` in `engine/crates/conformance/examples/make_archives.rs`, and the case
   `conformance/language/packages/agents-v2-defaults/case.yaml`, its steps 3 and 4.
5. `conformance/README.md`, its section "Reviewing vector changes".

**Size: S**: about ten changed lines of Rust in one example, about twenty lines of one case, and
the files that the example writes.

## The decisions it applies

1. **The `.v2` schemas hold no `apiKey` and no `oneOf`.** In `release_fixtures` of
   `engine/crates/conformance/examples/make_archives.rs` (line numbers of `main` at `e4ceb4c8`;
   find each place by its text):
   - Delete these four lines (`:56-59`). `core` served only `apiKey`'s `$ref`, so without them
     `clippy` would refuse an unused variable:

     ```text
         let names = conduit_loader::read_file(&root.join("spec/names.yaml")).unwrap();
         let core = conduit_loader::Names::from_node(&names[0].root)
             .unwrap()
             .core_package;
     ```

   - Replace the comment line (`:61`)
     `    // Ruling 82 point 2: account and apiKey are both required; apiKey defaults to null.`
     with, at the same indent,
     `    // Ruling 83: subscription only. No apiKey; account is required, a string or null, no default.`
     The line before it, `// C12: registration-only fixtures, no signIn or hooks. PKG-031: turn template.`,
     stays.
   - In the `properties` literal (`:66-67`), replace these two lines:

     ```text
                 "account":{"type":["string","null"]},
                 "apiKey":{"anyOf":[{"$ref":format!("{}#secret-reference.v1", core)},{"type":"null"}],"default":null}
     ```

     with this one:

     ```text
                 "account":{"type":["string","null"]}
     ```

   - In the Type's literal (`:73-74`), replace these two lines:

     ```text
                 "family":"Agent","schema":{"type":"object","properties":properties,"required":["account","apiKey"],"additionalProperties":false,
                     "oneOf":[{"properties":{"account":{"type":"string"},"apiKey":{"type":"null"}}},{"properties":{"account":{"type":"null"},"apiKey":{"type":"object"}},"required":["apiKey"]}]},
     ```

     with this one:

     ```text
                 "family":"Agent","schema":{"type":"object","properties":properties,"required":["account"],"additionalProperties":false},
     ```

   Nothing else in the file changes: not `accountServices`, `model` or `peerMessaging`, not
   `offers`, `requires` or `records`, and not the seven sample values after the loop. Run
   `cargo fmt` only to check; the lines above are already formatted.

2. **The fixture, regenerated.** From `engine/`, run
   `cargo run --release -p conduit-conformance --example make_archives`. It must print
   `agents-0.2.0: sha256:8f151f36064055c6e19599e36e6bd26281d6f84f88c6f8d0b9ec79d5543008bc` and
   end `41 reproducible archive fixtures written`, and change, of what it writes, only
   `conformance/fixtures/s1/packages/agents-0.2.0/contracts.json`,
   `conformance/fixtures/install-archives/agents-0.2.0.tar.gz`,
   `conformance/fixtures/install-archives/digests.json`,
   `conformance/language/packages/agents-v2-defaults/claude-code.v2-values.json` and
   `conformance/language/packages/agents-v2-defaults/codex.v2-values.json`. Then, in
   `conformance/language/packages/agents-v2-defaults/install.json`, replace
   `sha256:12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7` with
   `sha256:8f151f36064055c6e19599e36e6bd26281d6f84f88c6f8d0b9ec79d5543008bc`, and change nothing
   else in it. If the digest differs, or another file changes, stop and report.

3. **The case's expectations follow.** In
   `conformance/language/packages/agents-v2-defaults/case.yaml`, which is JSON with two-space
   indentation and a final newline, keep every member and its order, and change only:
   - `title`, from `Installed Agent v2 defaults and exclusive account or API key configurations`
     to `Installed Agent v2 defaults, a required account of string or null, and no API key`.
   - In steps 3 and 4 (the two `schema-evaluation` steps, `claude-code.v2` then `codex.v2`), in
     `expect.reply.samples`, counting samples from 1:
     - delete the member `"apiKey": null` from the `value` of samples 1, 3, 4, 6 and 7, where
       SCH-009 filled it from the default that no longer exists, and the comma that the deletion
       leaves dangling on the line before it where it was the last member;
     - keep the member `apiKey`, the object `{"kind": "Resource", "name": "api-key"}`, in the
       `value` of samples 2 and 5, as the samples supply it;
     - set sample 2's `valid` from `true` to `false`: a null account with a key now fails, since
       `apiKey` is not a property and the schema refuses additional properties;
     - set sample 4's `valid` from `false` to `true`: a null account alone now passes, since the
       `oneOf` that required a key beside it is gone.

   So the verdicts become, for `claude-code.v2`, valid, invalid, invalid, valid, invalid, valid,
   valid, and for `codex.v2` the same but sample 7 invalid, as it was (`peerMessaging` is not a
   property of `codex.v2`). No other value changes: every other member of every `value`,
   `stateUnchanged`, `required_promises`, the steps 1 and 2, `promises`, `layers` and `files`
   stay as they are. No expectation is generated from the engine's output: write them as this
   decision says, and if the case then fails, stop and report the runner's error.

4. **The declaration.** `docs/work-orders/202-declared.yaml` comes with this order, with five
   shapes for `agents-v2-defaults`: the ten removals of a null `apiKey`, sample 2's two `valid`
   changes, sample 4's two, the title, and the four fixture files that the case names
   (`agents-0.2.0.tar.gz`, `install.json` and the two `-values.json` files). Check that it
   matches your change and change it only to make it so. The fixture's `contracts.json` under
   `conformance/fixtures/s1/packages/agents-0.2.0/` is no file alias of a case, so
   `vector_delta.py` does not report it, as for order 048a-1.

5. **What this order does not change.**
   - **`IN_FORCE.md`**: ruling 83's own change rewrites its line on `apiKey` (the start checks
     above). This order changes no line of it.
   - **`engine/STANDING.md`**: no promise, no case's `promises`, no expected failure, no S1 row
     and no promise ID in a source changes, so `promise_standing.py --check` gives `Errors: 0`
     with the file as it is. Do not run it with `--write`. If it reports an error, stop and report.
   - **The plan-and-apply reference** under `docs/implementation/plan-and-apply/` keeps the older
     digest `479ef470...` and its `apiKey: null` lines, which order 048a-1 also left: order 052a-1
     re-pins the archive's digest there (order 117's entry). Under this order's schema
     `apiKey: null` is refused, so a re-pin alone would leave the golden cases' `.v2`
     configurations invalid against the fixture: the coordinator amends order 117's entry for
     052a-1 and 052a-2 under ruling 83, so that the re-pin also drops `apiKey: null` from the
     golden cases' `.v2` configurations and plan texts and rewords the README's "as for S1's
     apiKey" example. The dated evidence under `experiments/` keeps what it holds.
   - **`spec/`**: the example `spec/schemas/examples/valid/sectioned-project/s1.yaml` still
     writes `apiKey: null` in both Agents' `config`. It checks authored syntax only, and
     `engine/crates/shell/tests/by_rule.rs` excludes it from the examples it sends, so nothing
     fails; the coordinator brings it into line with ruling 83 in a change of its own.
   - **The real packages** on the branch of order 050 and the planned orders 050f to 050n: the
     coordinator restates them under ruling 83. This order merges before 050f is dispatched
     (ruling 83's What follows), since 050f states the packages' `.v2` schemas against this
     fixture.
   - No `signIn`, no hook and none of the inputs that orders 050a and 076 add (spec batch 1's
     C12), no other case, fixture, source, schema or tool.

## Scope

Change only `engine/crates/conformance/examples/make_archives.rs` (decision 1), the five files that
`make_archives` writes as decision 2 lists them,
`conformance/language/packages/agents-v2-defaults/install.json` (its digest) and
`conformance/language/packages/agents-v2-defaults/case.yaml` (decision 3).
`docs/work-orders/202-declared.yaml` comes with this order; change it only as decision 4 says.
Not `IN_FORCE.md`, `engine/STANDING.md`, `spec/`, `tools/`, `packages/`, `experiments/`,
`docs/implementation/`, `docs/proposals/`, the case schema, `conformance/manifest.yaml`, any index,
any other case or any engine source: report any defect you find there as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --release -p conduit-conformance --example make_archives -- --check
cargo run --release -p conduit-conformance -- .. agents-v2-defaults
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

`make_archives -- --check` ends `41 reproducible archive fixtures verified`. The runner command
ends `1 cases passed`. The vectors report `2 passed; 0 failed`. The full suite reports no failed
test; report its `ok` lines' count. The threads are 4 because at 24 the runner ran out of memory
(order 071's Record).

Next, show that the case fails on the old fixture. From `engine/`, one at a time:

```bash
git show HEAD:conformance/language/packages/agents-v2-defaults/claude-code.v2-values.json > ../conformance/language/packages/agents-v2-defaults/claude-code.v2-values.json
cargo run --release -p conduit-conformance -- .. agents-v2-defaults
cargo run --release -p conduit-conformance --example make_archives
cargo run --release -p conduit-conformance -- .. agents-v2-defaults
cargo run --release -p conduit-conformance --example make_archives -- --check
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed` and an error line
that, cut before `; reply:`, ends `step 3: /reply/samples/1/valid: expected false, found true`;
report it. The runner matches an expected `value` as a subset of the reply's, so the null
`apiKey` that the old schema fills does not fail sample 1: the verdicts carry the test, and decision
3's deletions keep the expectations true. `make_archives` then writes the file again from
decision 1's source, after which the second runner command ends `1 cases passed` and the check ends
`41 reproducible archive fixtures verified`.

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_model_account.py
python3 -m unittest discover -s tools/tests
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/202-declared.yaml; echo "exit $?"
git status --short -- IN_FORCE.md engine/STANDING.md engine/crates/core engine/crates/loader engine/crates/shell spec tools packages experiments docs/implementation docs/proposals conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/language/packages/index.yaml
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/202-declared.yaml: 5 shapes.`,
then `Vector delta: 19 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`: ten removals,
four `valid` changes, the title and four fixture files. `check_schemas.py`, `check_rulings.py`,
`check_roadmap.py`, `check_model_account.py` and `promise_standing.py --check` each end
`Errors: 0`. The unit tests, run with the interpreter that has PyYAML, end `OK`; report the count.
The `git status` command prints nothing.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`.

From the repository root, this prints
`[['account'], ['account']] [False, False] [False, False] [False, False]`: each `.v2` schema
requires `account` alone, and holds no `apiKey`, no `oneOf` and no default for `account`:

```bash
python3 -c "import json; d=json.load(open('conformance/fixtures/s1/packages/agents-0.2.0/contracts.json')); v=[t['spec']['schema'] for t in d if t['metadata']['name'].endswith('.v2')]; print([s['required'] for s in v], ['apiKey' in s['properties'] for s in v], ['oneOf' in s for s in v], ['default' in s['properties']['account'] for s in v])"
```

This prints
`[[True, False, False, True, False, True, True], [True, False, False, True, False, True, False]] [0, 0] [2, 2]`:
the verdicts of steps 3 and 4, then in each step the samples whose value holds a null `apiKey`,
then those whose value holds a key:

```bash
python3 -c "import json; c=json.load(open('conformance/language/packages/agents-v2-defaults/case.yaml')); s=[c['steps'][i]['expect']['reply']['samples'] for i in (2, 3)]; print([[x['valid'] for x in t] for t in s], [sum(1 for x in t if 'apiKey' in x['value'] and x['value']['apiKey'] is None) for t in s], [sum(1 for x in t if isinstance(x['value'].get('apiKey'), dict)) for t in s])"
```

Each of these prints 1:

```bash
grep -c '"title": "Installed Agent v2 defaults, a required account of string or null, and no API key",' conformance/language/packages/agents-v2-defaults/case.yaml
grep -c 'sha256:8f151f36064055c6e19599e36e6bd26281d6f84f88c6f8d0b9ec79d5543008bc' conformance/language/packages/agents-v2-defaults/install.json
grep -c '"agents-0.2.0.tar.gz": "sha256:8f151f36064055c6e19599e36e6bd26281d6f84f88c6f8d0b9ec79d5543008bc"' conformance/fixtures/install-archives/digests.json
grep -c 'Ruling 83: subscription only. No apiKey; account is required, a string or null, no default.' engine/crates/conformance/examples/make_archives.rs
grep -c '"required":\["account"\],"additionalProperties":false},' engine/crates/conformance/examples/make_archives.rs
```

Each of these prints 0:

```bash
grep -c 'Ruling 82 point 2' engine/crates/conformance/examples/make_archives.rs
grep -c '"oneOf"' engine/crates/conformance/examples/make_archives.rs
grep -c 'secret-reference.v1\|core_package' engine/crates/conformance/examples/make_archives.rs
grep -c 'apiKey' conformance/fixtures/s1/packages/agents-0.2.0/contracts.json
grep -rl '12eca5b0c8e74fa0054af053691fe9560d037c161727f71fbc5cf6dcd5699fb7' conformance engine | wc -l
```

This prints 3, decision 1's comment line and the two sample values that supply a key, which
stay:

```bash
grep -c 'apiKey' engine/crates/conformance/examples/make_archives.rs
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, and give the last line of every check, each exit status
and count above, and the negative control's error line.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `202-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.

## Record

Run on 2026-11-18, from about 17:06 to 17:51 UTC, on the Mac from `main` `578835bd` (the merge of
orders 048a-4 and 202's text, #362), in two rounds: the worker (1320 s) and a fresh Codex verifier
(283 s), then the worker again (818 s) and a second fresh Codex verifier (245 s). Round 1 changed
the eight files of Scope as decisions 1 to 4 say: `make_archives.rs` writes both `.v2` schemas with
`account` alone required, a string or null with no default, and no `apiKey` and no `oneOf`; the five
generated files and `install.json` carry `sha256:8f151f36...`; the case takes the new title, the ten
removals of a null `apiKey` and the four `valid` changes; and the declaration's five shapes matched
its 19 changes. The worker's own choices: cargo ran offline, with the compiler cache off after it
met a sandbox write error, and `/usr/bin/python3` ran the tools that need PyYAML; the negative
control's `git show HEAD:` read the base, since the checkpoint came after the worker.

The first verifier found no must and three shoulds: the case could pass while an installed Type
still accepted a null `apiKey`, since steps 3 and 4 evaluate the values file's schema and match
values as required-field projections; order 117's entry for 052a-1 and 052a-2 calls for the digest's
re-pin without the golden configurations' cleanup; and the 050f row of the index and order 117's
plan still direct an explicit `apiKey`. The admin's round 2 asked for the fixes in the order's
scope. The worker added to step 2, the install, projections of both installed `.v2` schemas
(`account`'s type, `required` and `additionalProperties: false`) and an `absent` list of six
pointers (`apiKey`, `oneOf` and `account`'s default in each), and two shapes to `202-declared.yaml`,
and left the other two to the coordinator as outside the scope. Its own choices: the negative
control and the checks of the old source and digest read `578835bd`, since `HEAD` held the new
fixture after the checkpoint, and the two shapes declare the leaf paths that `vector_delta.py`
reports. With a null-only `apiKey` added to either installed Type, and the install's digest changed
to match, the case failed; both files were then restored byte for byte. The second verifier found no
must and four shoulds: an installed schema could still admit `apiKey: null` through
`patternProperties`; steps 3 and 4 could pass if the evaluator added an `apiKey: null` that a value
omits; and the first verifier's two follow-ups to order 117 and the index, again the coordinator's.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on the branch before
its rebase. It ran `cargo fmt --check` and clippy (clean), `make_archives -- --check` (41 verified,
the digest `8f151f36...`), the runner (`1 cases passed`), the vectors with 4 threads (2 passed; 0
failed), the full suite (28 `ok` lines, 339 passed, 0 failed), the spike (9 passed), the eight
Python checks (`Errors: 0` where named), the tool tests (129, OK), the order's probes and greps
(as stated), the protected paths' status (empty), `git diff --check`, a privacy scan of the added
lines, the commits' identity and a trial merge with `main` at `3c761dd9` (order 047d; clean). On a
scratch copy with a fresh build it ran the negative control against the base `578835bd`: the
first runner exited 1 with `step 3: /reply/samples/1/valid: expected false, found true`, and the
rest passed as stated. A first attempt from a copied target directory ran a binary that wrote the
worktree's fixtures again, with identical bytes; the worktree stayed clean. It found one must and
one should:

- R1: "Done-when unmet as worded": `vector_delta.py` gives 7 shapes and 33 changes, not the order's
  5 shapes and 19 changes, and against the order's declaration as merged on `main` it gives 14
  undeclared and exits 1. Round 2's additions to step 2 and the declaration go beyond decision 3
  ("the steps 1 and 2 ... stay as they are") and decision 4 ("change it only to make it so").
- R2: after the checkpoint commits, two checks of "Done when" that name `HEAD` test nothing: the
  negative control's `git show HEAD:` restores the new values file, and
  `vector_delta.py HEAD WORKTREE` reports 0 changes.

The fixer rebased the branch onto `origin/main` at `1e38ff92` (order 048a-2, which merged
meanwhile; before it, onto `3c761dd9`) with the work-order identity, without conflict, since
neither order shares a file with this one, and took each finding:

- R1: the reader's option (b). Round 2's assertions stay, as the coordinator's choice: the admin's
  round 2 amended decisions 3 and 4, adding step 2's projections of both installed `.v2` schemas,
  its `absent` list of six pointers and the declaration's two shapes for them. The merge's check
  is `vector_delta.py origin/main WORKTREE --declared docs/work-orders/202-declared.yaml`, which
  printed `7 shapes.` and `Vector delta: 33 changes; 0 flips; 0 undeclared; 0 errors.`, exit 0.
  With the 5-shape declaration from `main` it gives 14 undeclared, exit 1; from `origin/main` to
  round 1's checkpoint with that declaration it gives `5 shapes.` and
  `Vector delta: 19 changes; 0 flips; 0 undeclared; 0 errors.`, exit 0, as the order states. The
  order's text is unchanged.
- R2: both checks ran against a base, not `HEAD`: the negative control took the values file from
  `578835bd`, and `vector_delta.py` read `origin/main` at `1e38ff92`, which holds the old fixture
  and the order's text. The order's text is unchanged; where the squash lands first, the base is
  `origin/main~1` and the head `origin/main`.

The second verifier's two shoulds on the installed schemas (`patternProperties`, and an
`apiKey: null` that the evaluator adds) and both verifiers' follow-ups to order 117's entries for
050f, 052a-1 and 052a-2 and the index's 050f row are left to the coordinator; the reader raised
none of them.

On the rebased branch, with this Record written, the fixer ran from `engine/`, one at a time and
with `RUST_TEST_THREADS=4` and `CONDUIT_VECTOR_THREADS=4`: `cargo fmt --check` and clippy (exit
0), `make_archives -- --check` (`41 reproducible archive fixtures verified`), the runner
(`1 cases passed`), the vectors (`test result: ok. 2 passed; 0 failed`) and the full suite (exit
0, 28 `ok` lines, 349 passed, none failed; the ten more than the reader's come with orders 047d
and 048a-2). The negative control, with the values file from `578835bd`, failed first with
`FAIL 0 cases passed; 1 failed` and, cut before `; reply:`, the line ending
`step 3: /reply/samples/1/valid: expected false, found true`; `make_archives` then wrote the 41
fixtures again, the runner ended `1 cases passed`, the check ended
`41 reproducible archive fixtures verified`, and `git status` showed no fixture changed. From the
root: `check_schemas.py`, `set_names.py --check`, `check_rulings.py`, `check_docs.py`,
`check_roadmap.py`, `check_model_account.py`, `check_private_paths.py` and
`promise_standing.py --check` (`Errors: 0` each), the tool tests with `/usr/bin/python3` (129,
OK), the two probes (as stated), the five greps that print 1, the five that print 0 and the one
that prints 3 (as stated), the protected paths' status and their diff from `origin/main` (both
empty), and `git diff --check origin/main...HEAD` (clean). The spike's `tests/spike.rs` reported
`test result: ok. 9 passed; 0 failed`. The commits' authors and committers are the work-order
identity alone. The gate's run on this order's squash, which the merge waits on, is the
coordinator's at the merge.
