# Work order 244: the short form's rejections at the authored place (SCT-014), its equivalence (SCT-015), and the sectioned examples of `spec/` through the engine

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` each case you add or change. This order needs no
network: it reaches no host and fetches no crate, and its only sockets are the loopback ports
the engine's own tests bind. Every crate it builds is in the local cache already; if a build
asks for one that is not, stop and report, and fetch nothing. Run one cargo command at a time.
Never print a whole large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a
count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential. Do not run `conduit` outside a
test, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Every file
you write is public: write no absolute local path, no user or machine name and no private
repository name.

## What and why

**The gap.** Order 242 made the loader translate the short form, so a sectioned project or
package reaches the core as framed documents, and its own rejections name their SCT rule at the
authored place. A check that runs after translation still reports against a document the
author never wrote (`/objects/worker--files`), and `conduit submit` prints it as JSON. The
invalid `-sectioned-` examples are only schema-checked (`loader/tests/examples.rs`),
`shell/tests/by_rule.rs` excludes both sectioned directories, SCT-014 and SCT-015 have no
passing case, and
[DEC-007](../decisions/log.md#dec-007-2026-11-20-documents-instantiation-packages-and-the-short-form)
is "not done". This is batch 1's 043b
([the plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders))
without `manages`, which order 217 cut, with 052b's "diagnostics as file:line:col with the SCT
origin".

**What `spec/` settles.** [SCT-014](../../spec/promises/sections.yaml) points every such
rejection into the authored text, with `properties.translation` `{rule, document}` for a
translated document; its observable gives the mapping. SCT-015: registering the sectioned
project gives the objects, records and reason of registering its translation, and diagnostics
of the same `ruleId` and `level`. SCT-003: the checks apply unchanged and no provenance is
stored, so the map lives with whoever translated.

**Departures from batch 1's 043b** (the coordinator's readings, open to the owner's strike).
The `invalid/` examples named `SCT-*` are rejected under SUB-005, as SCT-001 and SCT-014 say of
a schema rejection; the name gives the promise its `sectioned.v1` clause encodes, as for every
file there. SUB-005 holds at installation too (PKG-023), so install is in scope.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92, point 1): it
serves 2: an access the short form makes stays inspectable from the text, since each
rejection names the `consumes` item, `operator` or member that made the Conduit. None is at
risk: nothing registered, sent or recorded changes (SCT-015's cases).

**Should this exist** (point 3). Yes, as existing practice: a Source Map (ECMA-426) maps a
generated position back to its source; compilers print `file:line:column: message` (GNU Coding
Standards) and note a first definition beside a second; SARIF 2.1.0 keeps a second place in
`relatedLocations`; GNU make and ld write `archive(member)`. Equivalence is shown by differential
cases. Not taken: the map stored in the engine, or the core taught the short form (SCT-003); a
`render` command (dropped; the plan prints the translation).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit), a Codex
verifier and a Claude reader, who run only what settles a doubt, and the gate, which runs
everything (DEC-030, DEC-033). By ruling 89, each decision says what must hold; how is yours.
The line's form, the case prefixes and the test names are the coordinator's, open to the owner's
strike.

**Stop rule.** Change nothing under `spec/` or in `docs/decisions/log.md`. Stop, change nothing
further, and report where:

- the start check prints otherwise than it says, above all where order 242 has not merged;
- a promise needs a choice `spec/` does not make: report the promise, the choice and the
  options; do not choose;
- a sectioned registration differs from its translation's because of the core (where the
  translator causes it, fix the translator and report it);
- a correct result needs a change outside the scope: a core verdict, `spec/`, the shell's
  `main.rs` or `help.rs`, `experiments/`, `packages/`, a `Cargo.toml` or a lockfile;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The map** (SCT-003, SCT-014). Each document the loader hands over from a project or a
   package keeps where it came from: its file, its document's number and position; for a
   generated document, also the definition, item or member that made it, by its authored
   pointer, with the SCT rule and its generated name. Where 242 keeps such a map, use it;
   where not, add it in the translator. One loader function takes a SARIF diagnostic about the
   handed-over set and returns it located (decision 2), by SCT-014's observable, where
   `N--operator` and `N--operator--t` map under `/F/N/operator`, a `conduits` member under
   `/conduits/N`, a supplied Fleet owner to its file's `/fleet`, a Fleet that several files
   supply to each file's `/fleet`, and any other pointer to its nearest authored ancestor. The
   function finds the document from the logical location `/objects/NAME` (registration's
   form) or from `properties.document` and a pointer. Where a rejection of decision 3 names no
   document, the core adds `/objects/NAME`, changing no `ruleId`, `level`, message or verdict.
   The map is never stored, sent or recorded.
2. **The located form.** `locations[0]` has `physicalLocation`, with `artifactLocation.uri`
   the authored file as the loader names it and `region` `{startLine, startColumn}` of the
   authored node, and `logicalLocations[0].fullyQualifiedName`, the authored pointer.
   `properties.file` and `properties.document` are the authored file and document, as the
   loader's have them. A generated document's diagnostic carries `properties.translation`; a
   syntax rejection with no emitted document carries none. `keywordLocation` stays. A second
   place goes in `relatedLocations`, in the same form: for NAM-027 the earlier document, in
   file-name then document order, the later in `locations`, as a compiler reports a
   redefinition; for a shared Fleet, the other files' `/fleet`. `ruleId`, `level` and message stay, and locating
   adds no value (SEC-003). One loader function writes a loader `Diagnostic` in this form, in
   place of `loader_diagnostic` in `core/src/archives.rs` and `diagnostic` in
   `conformance/src/lib.rs`, with their output unchanged for a framed document.
3. **What is located.** Every rejection of a sectioned document or of a generated one, on every
   path that translates:
   - the loader's, of a generated document: SUB-005 at the authored field, its
     `keywordLocation` into the canonical schema;
   - the core's, of a translated registration: NAM-027 at both authored places; OBJ-002 at the
     definition or member; CON-021 at the item's or member's `capability`; IFC-014 at the
     authored `operator` where the Conduit is `N--operator`, at `/F/N/consumes/L` where it is
     `N--L`, and at the written `as` or the `conduits` member otherwise; any other at least at
     its definition, item or member;
   - install's own, of a package's documents: SUB-005, NAM-027, and MAN-011 at the key,
     `fleet` or the instance section, as 242 names it.

   The shell's `submit` and `replace` and the runner's registration steps locate the reply by
   the map of the project they read; install locates its own in the core, in the call.
   `conduit register`, whose code is in `main.rs`, stays unlocated here; it is named under What
   follows.
4. **The command line.** For each diagnostic of a rejected load or reply, `submit`, `replace`
   and `install` print one line on standard error, then one per related place:

   ```text
   PATH:LINE:COLUMN: LEVEL: MESSAGE [RULE]
   PATH:LINE:COLUMN: LEVEL: MESSAGE [RULE; SCT-NNN NAME]
   PATH:LINE:COLUMN: note: also here [RULE]
   ```

   The second form is a translated document's, with `properties.translation`'s rule and
   document. `PATH` is the file as the person named it: the directory argument as given, joined
   with the file's name, never a path the shell resolved; a member of an archive is
   `ARCHIVE(MEMBER)`. A diagnostic with no place prints `PATH: LEVEL: ...` with the directory or
   archive. `LEVEL` is the diagnostic's `level`. The final error line stays; where it held the
   diagnostics' text, it gives their count instead, such as
   `submit: cannot load project DIR: 2 diagnostics above`. The JSON reply on standard output
   carries the located diagnostics, so an agent reads the same places. One shell function
   prints these lines, for `conduit plan` and `conduit apply` (orders 245 and 247) to call too.
5. **SCT-014's cases**, with IDs starting `sections-located-`, beside 242's SCT cases. Each
   asserts decision 2's form with `diagnostics` and `diagnostic_facts`, or a new `expect`
   piece where those cannot. One each, at least: SUB-005 of a generated document with its `keywordLocation`; NAM-027 between
   a generated and a framed document, both places; OBJ-002 against an existing object; CON-021
   at an inline `consumes` item's `capability`; IFC-014 at an `operator`; a supplied Fleet
   owner at `/fleet`; a Fleet that two files supply; a template-section body; and a syntax
   rejection with no `properties.translation`. Nine or more.
6. **SCT-015's cases**, with IDs starting `sections-equivalent-`. Each registers a sectioned
   project and its translation, written by hand in the case's files, from one initial state,
   and compares the objects apart from generated IDs, the records, the reason, and the
   diagnostics' `ruleId` and `level`. Use 242's `equivalent` option; where it compares less,
   extend it; where 242 left none, add it to the registration step. At least: every section
   and both forms at once, accepted; sectioned and framed documents in one project; two files
   that state one `fleet`; and a CON-021 rejection whose places differ. Four or more.
7. **The examples through the engine.**
   - `loader/tests/examples.rs`: the skip goes. Every directory under `valid/` is read with
     `read_project`; for a sectioned one, every document handed over is framed and passes the
     schemas, and each generated one bears SCT-013's name. Each `invalid/` file named `-sectioned-` is read
     as a project, beside a manifest that gives the version, and is rejected under SUB-005 by
     the schema step, its `keywordLocation` in `sectioned.v1`, at the authored key;
     `tests/expected/invalid-verdicts.txt` lists each as `schema`, its header dated.
   - `shell/tests/by_rule.rs`: `sectioned-project` and `sectioned-package` leave `excluded` and
     are sent: the project registers through `register.v1` from `read_project`, its pins
     installed from archives and its digests set to theirs, as `files-package` does; the package
     installs after its dependencies, from the fixture archives with their digests, whichever
     package order 240 leaves there; where none holds a dependency, the test packs one holding
     the contracts the templates name, as `archives()` packs `example.org/common`, and sets the
     pinned digest to the packed archive's. `tests/expected/by-rule-outcomes.txt` gains a line
     `accepted succeeded -` for each, its header re-derived.
   - `tools/check_schemas.py`: its line that sectioned examples "await the translator" says
     instead that the engine's tests translate and register them, naming the two files.
8. **The command line's tests**, in a new `engine/crates/shell/tests/sections.rs`, on the real
   binary with `serve` on a temporary store, as `tests/cli.rs` runs it:
   - `submit_prints_authored_places`: a sectioned project rejected under CON-021 prints the
     second line form at the `capability` the author wrote, with its line and column; and one
     rejected under NAM-027 prints the first place and a `note:` line at the second;
   - `install_prints_archive_member_places`: an archive the test packs, of a sectioned package
     that install rejects about a generated document, prints `ARCHIVE(MEMBER):LINE:COLUMN:`;
   - `located_lines_echo_no_secret_value`: a generated document rejected at a place that
     `secret-reference.v1` governs, holding a literal: neither the reply nor any line holds it.
9. **Coverage.** `conformance/s1-promises.yaml` gains the 14 SCT IDs not yet there, SCT-001 to
   SCT-009, SCT-011 and SCT-013 to SCT-016, its header reading "the 322 live IDs: the 308 named
   in increment 2, section 2, and the 14 SCT IDs that batch 1 planned for 043b"; `s1_coverage`
   in `conformance/manifest.yaml` gains a row for each with its cases, 242's and these; the S1
   inventory paragraph of `conformance/README.md` follows, with a dated count; and
   `python3 tools/promise_standing.py --write` rewrites `engine/STANDING.md`, where every SCT
   row then reads `case passes`. `docs/work-orders/244-declared.yaml` declares each added or
   changed case, decision 1's moved locations among them. The coordinator, not you, then sets
   DEC-007's Status line to done with the pull request's number.

## What to read

- SCT-001, SCT-003, SCT-011, SCT-013, SCT-014 and SCT-015; SUB-005; NAM-027; SEC-003;
  `diagnostic.v1` in `spec/core/results.yaml`.
- Order 242's text and what it built: the translator, the runner's `translate` step and
  `equivalent` option, and its SCT cases.
- In `engine/crates/`: the loader's `diagnostic.rs`, `project.rs` and `tests/examples.rs`; the
  core's `registration.rs` (`diagnostic`; OBJ-002, CON-021, IFC-014), `packages.rs` (`diag`)
  and `archives.rs` (`loader_diagnostic`, MAN-011); the runner's `lib.rs`, `packages.rs`,
  `steps/` and `expect.rs`; the shell's `author.rs`, `boundary.rs`, `tests/by_rule.rs` and
  `tests/cli.rs`; and `conformance/README.md`'s S1 inventory section.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
grep -E '^\| SCT-[0-9]{3} \|' engine/STANDING.md | grep -vc '| case passes |$'
grep -E '^\| SCT-[0-9]{3} \|' engine/STANDING.md | grep -v '| case passes |$' | cut -d' ' -f2 | tr '\n' ' '; echo
grep -rlw 'translate' conformance/case-schema | wc -l | tr -d ' '
grep -c 'Short-form translation is built in 043a/043b' engine/crates/loader/tests/examples.rs
grep -c -- '-sectioned-' engine/crates/loader/tests/expected/invalid-verdicts.txt
grep -cE '^valid/sectioned-' engine/crates/shell/tests/expected/by-rule-outcomes.txt
test -e engine/crates/shell/tests/sections.rs; echo "sections $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

On `main` at `cc3605a3`, before order 242, they print `main 0`, `14`, the 14 IDs, `0`, `1`,
`0`, `0`, `sections 1`, `engine 0` and `0`: stop there. Once order 242 has merged, the
second line prints `0`, `1` or `2`, the third only IDs among SCT-014 and SCT-015 (any other
ID is 242's gap: stop and report it), the fourth `1` or more and the fifth `0`; the rest print
as before.
`engine` printing otherwise means a crate is missing from the cache: stop, and fetch nothing.

## Scope

Change `engine/crates/loader/` (`src/`, `tests/`); `engine/crates/core/src/registration.rs`,
`packages.rs` and `archives.rs` for decisions 1 to 3's locations alone;
`engine/crates/conformance/src/` and its `examples/` where a fixture archive is generated;
`engine/crates/shell/src/author.rs`, `boundary.rs` and one new module (with its `mod` line in
`src/lib.rs`); `engine/crates/shell/tests/by_rule.rs`, `tests/expected/by-rule-outcomes.txt` and
the new `tests/sections.rs`; under `conformance/`, the new cases and their fixtures, their
entries in the index pieces, existing cases whose diagnostics gain a location, case-schema
pieces, `manifest.yaml`, `s1-promises.yaml` and `README.md`'s S1 inventory paragraph;
`engine/STANDING.md` by the tool; the one line of `tools/check_schemas.py`; and
`docs/work-orders/244-declared.yaml`. Change nothing else; report anything else that needs
changing.

## Done when

Write the cases and tests first, run them on 242's code and report each failing line; then
change the code until all pass. Each line marked "fails today" prints otherwise on `main` at
`cc3605a3`.

```sh
(cd engine && cargo run --release -q -p conduit-conformance -- .. sections-located- | grep -c '^PASS ')  # 9 or more; fails today (0)
(cd engine && cargo run --release -q -p conduit-conformance -- .. sections-equivalent- | grep -c '^PASS ')  # 4 or more; fails today (0)
(cd engine && cargo test --release -p conduit-shell --test sections 2>&1 | grep -c '^test result: ok\. 3 passed')  # 1; fails today (0)
test "$(grep -c -- '-sectioned-.* schema$' engine/crates/loader/tests/expected/invalid-verdicts.txt)" = "$(ls spec/schemas/examples/invalid | grep -c -- '-sectioned-')"; echo "verdicts $?"  # verdicts 0; fails today (verdicts 1)
grep -cE '^valid/sectioned-[a-z]+ accepted succeeded -$' engine/crates/shell/tests/expected/by-rule-outcomes.txt  # 2; fails today (0)
grep -cE '^\| SCT-[0-9]{3} \|.*\| cases \|.*\| case passes \|$' engine/STANDING.md  # 14; fails today (0)
grep -c '^- SCT-' conformance/s1-promises.yaml  # 14; fails today (0)
grep -c 'await the translator' tools/check_schemas.py  # 0; fails today (1)
```

These pass, as on `main`; run the whole engine suite once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-loader --test examples)  # every test passes
(cd engine && cargo test --release -p conduit-shell --test by_rule)  # every test passes
(cd engine && cargo run --release -q -p conduit-conformance -- .. >/dev/null); echo "cases $?"  # cases 0
python3 tools/promise_standing.py --check  # exit 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/244-declared.yaml; echo "exit $?"  # exit 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD | grep -E '^(spec/|docs/decisions/|experiments/|packages/)|Cargo\.(toml|lock)$|shell/src/(main|help)\.rs$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --release)  # once, at the end; every test passes
```

Your report gives: per decision, what changed and who calls the locating function; each new case
and test with its failing line on 242's code, then its passing line; each core diagnostic that
gained a location and each case whose expectation moved; each translator fault, with its case;
your own choices, under `own_choices`; and every check's last line.

Stage nothing. A verifier follows; the coordinator then runs `tools/gate.py`, has a fresh
Claude reader read the diff (ruling 92, point 4), and merges.

## What follows

Named here and not in this order:

- Orders 245 and 247 print rejections with decision 4's function; order 246's reference gives
  the line's form.
- `spec/README.md`'s row for `promises/sections.yaml` says "no vectors yet": for a spec order or
  order 250.
- `conduit register` calls decision 4's function: order 245 or 250.

## Record

Run on 2026-11-21 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-1 stopped at the start** (14:55 to 15:24), as the stop rule says. On today's engine, a
rejected submission's diagnostic quoted a literal written at a place that `secret-reference.v1`
governs, a SEC-003 breach. The order allowed changes to locations only.

Root's decision, open to the owner's strike, was that the SEC-003 fix lands here, first and at
its source. Every diagnostic about such a value withholds it, in framed and sectioned documents
alike, over the wire, at the CLI and at install, keeping its rule ID, level, verdict, document
and pointer.

**Worker-1 then built the order** (15:25 to 16:19): SCT-014's authored locations, SCT-015's
equivalence and the sectioned examples through the engine. The whole engine suite ran once
(509 passed, 0 failed, 9 ignored), and conformance gave 613 passed and 3 expected failures.
Mid-turn, a shell mistake truncated eleven files. Worker-1 restored them from HEAD and applied
its edits again, and the verifier found no lost content.

**`turn-verifier-1.turn.v1`** (16:19 to 16:26) found five musts, which worker-1 fixed (16:27 to
17:01, the suite 510 passed):
- a remaining disclosure through a long reference chain, closed by redaction that follows every
  reference validation follows and fails closed;
- three lost authored locations: a configuration field, a standard argument and a nested member;
- a missing Fleet-owner case.

**The Claude reader** (17:02 to 17:07) passed it with one should, which root treated as
required: whole-document conflict diagnostics printed `arguments` and `config` verbatim when the
governing template could not be resolved. Worker-1 now withholds them (17:08 to 17:16), with a
sectioned and framed regression. 234 core and conformance tests pass.

Left for the owner: the loader's YAML syntax diagnostics quote the scalar that fails to parse
(`yaml.rs`, about lines 579 to 601). Either those messages drop it, or SEC-003 does not reach a
document that fails to parse.

At the rebase onto orders 240 and 252, the gate's engine row failed once. Order 240 had replaced the `sectioned-package` example, so it pins `example.org/ticket-queue`, but this order's public-route test still installed S1's filesystem for it. Worker-1 (18:33 to 18:35) made the test build a contracts-only `example.org/ticket-queue` package and install it first; the example registers as before.

The gate's run on the squash is the coordinator's, at the merge.
