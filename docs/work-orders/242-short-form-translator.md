# Work order 242: the short-form translator: the loader reads a project's or package's sectioned documents as the framed documents that SCT's rules make

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`, with one
`vectors` item per case you add. This order needs no network: it reaches no host and fetches no
crate. Every crate it builds is in the local cache already; if a build asks for one that is not,
stop and report, and fetch nothing. Run one cargo command at a time. Never print a whole large
file, diff, log or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Do not run `conduit` outside a test, `codex`, `claude`,
the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`, and run the tools that need PyYAML with `/usr/bin/python3`. Every
file you write is public: no absolute local path, user or machine name, or private repository name.

## What and why

**The gap.** `spec/README.md` tells an author to begin with the short form
([sections.yaml](../../spec/promises/sections.yaml), [`sectioned.v1`](../../spec/schemas/sectioned.yaml)).
The loader checks every map of a documents file against `document.v1`, so a map with no `kind`
fails under SUB-005, and `loader/tests/examples.rs` skips the sectioned examples. No SCT promise
has a case
([DEC-007](../decisions/log.md#dec-007-2026-11-20-documents-instantiation-packages-and-the-short-form),
"accepted, not done"). The S1 rerun waits for this chain. This is its first loader order, batch
1's 043a
([the plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders)),
written against today's `spec/`: order 217 cut `manages:` (SCT-012), so a scope is a `consumes`
item whose `from` names a core scope template, such as `core#pool-view`.

**What `spec/` settles.** The short form is part of the language, defined by translation into the
unchanged framed documents, which stay the kernel, the wire and the stored form
([ruling 55](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-55/README.md)
A1; SCT-001, SCT-003). The wire input `documents` of `register.v1` takes framed documents only
(SUB-005). A package's sectioned documents hold only `templates` and `apiVersion` (MAN-011).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, as
existing practice: Docker Compose reads a service's short syntax (`ports: ["8080:80"]`) as its long
form, and kustomize renders framed Kubernetes manifests from a shorter file; neither changes the
API's form. SCT-001 to SCT-016 make it Conduit's. Not taken: translating in the
core or on the wire (SUB-005 keeps the wire framed); a `render` command (`spec/` names none);
porting [experiment 011](../../experiments/011-short-form-spike/README.md)'s spike, which rejects
with strings, not rule IDs at authored places, and predates order 217's cuts: it is prior art,
not a source.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves 2,
since every Conduit a sectioned document makes is a standalone framed Conduit, inspected and
closed as any other, and 6, since translation sets up, opens, retires and repeats nothing
(SCT-003, SCT-015). None at risk: a Resource keeps its one Pool owner (test 3).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (4-hour limit; the whole engine
suite once, at the end), a Codex verifier and a Claude reader, who run only what settles a doubt,
and the gate (DEC-030, DEC-033). Work in this order: decisions 1 to 6 with decision 9's loader
test, then decision 7, then the cases promise by promise; if the turn runs short, report the
promises left. By ruling 89, each decision says what must hold; how is yours.
The step, option, expectation, case and test names, and decision 3's readings, are the
coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- an SCT rule needs a choice that `spec/` and decisions 1 to 5 do not make (a rule ID, a generated
  name, a place): report the rule, the choice and the options; do not choose;
- a correct result needs a change outside the scope;
- an existing case's verdict or expectation changes;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **Where translation happens** (SCT-001, SCT-003). When the loader reads a project, by
   `read_project` or `read_submitted_files` (and so when the core installs an archive), a map with
   no `kind` in a documents file is a sectioned document; the manifest never is (MAN-001). Each is
   checked against `sectioned.v1`, then translated; the generated documents are checked against
   `document.v1` as framed ones are, and handed over in `Project.documents` with the framed
   documents read beside them, as one set. The manifest is unchanged. `read_file`, `read_text`,
   `read_bytes` and the runner's `read` step translate nothing. Nothing changes on the wire: a
   `register.v1` call whose `documents` holds a sectioned value stays invalid under SUB-005.
2. **The translation**, exactly as the rules state, each generated document carrying SCT-002's
   version, and `description` and `annotations` going where each rule says:
   - `templates.<family>.N`: template N, its body the family's canonical `spec` (SCT-005);
   - an inline definition N: template `N--t` and instance N from it; a referenced one: instance N
     with its `from` and `arguments`; in both, standard arguments go to the instance (SCT-006);
   - `consumes: {L: item}` on N: Conduit `N--L`, consumer `{kind: F, name: N}`, `target` equal to
     `to`, `as: L`; an inline item also makes template `N--L--t` (SCT-007);
   - `operator` on N: Conduit `N--operator`, consumer `{kind: Interface, name: root}`, target N,
     `as: N`; inline, also `N--operator--t` (SCT-009);
   - `conduits.N`: Conduit N with its consumer, target and optional `as`; inline, also `N--t`
     (SCT-016);
   - `fleet: X`: Fleet X from the core template `fleet`, made once per project, and X as the owner
     of every Agent instance in that file, sectioned or framed (SCT-011).

   Nothing inside `config`, `arguments`, `implementation`, `controls`, canonical declared
   `conduits`, `entries` or requirement arguments is rewritten (SCT-004). Two definitions of one
   name are both handed over, for NAM-027 to reject; only SCT-011's Fleet is made once.
3. **Rejections.** The `ruleId` is the promise that rejects (SCT-014), and each names the authored
   file, document and JSON Pointer:
   - a `sectioned.v1` fault: SUB-005 with `keywordLocation`, as `document.v1`'s faults are; an
     unknown version is one (OBJ-017, by `api-version.v1`). A map with `metadata` or `spec` and no
     `kind` gives one such diagnostic, whose message says that a framed document needs `kind`;
   - no version in the document or the manifest: SCT-002;
   - a `consumes` item to `management`, or to `access` without an explicitly written core
     `bound.v1` requirement (a referenced item to `access` always): SCT-008 at the item, saying to
     write it under `conduits`;
   - a second `fleet` in one file, naming both; an Agent whose written owner is another Fleet,
     naming the file's `fleet` and that owner: SCT-011;
   - a generated name over 63 characters: SCT-013, at the authored key;
   - at install, a package's sectioned document holding `fleet` or any instance section, even
     empty: MAN-011, naming the file and the key, the request invalid (PKG-023).

   An Agent left with no owner is not a translation rejection: its instance has no `owner`, and
   the canonical check rejects it under its own rule. Every fault of a project is reported, and
   nothing is handed over (SUB-003). A canonical check of a translated document keeps its own rule
   and place here; mapping it into the authored text, and SARIF's `properties.translation`, are
   order 244's.
4. **Purity** (SCT-004). The translation depends only on the authored documents and their
   versions: it reads no package, contract, stored object, file name, directory or environment.
   The same text under other file names, file orders or key orders gives equal documents (SCH-007).
5. **What a generated document carries**: its SCT rule, and the authored file, document and
   pointer of the definition, item or member that made it, for the loader's callers; never stored
   and never on the wire (SCT-003). A framed Agent given its owner by SCT-011 carries SCT-011 and
   its file's `/fleet`. A Fleet made from several files carries each file's `/fleet`. Order 244
   maps diagnostics through it.
6. **The loader's public items** that `experiments/011-short-form-spike` uses keep their
   signatures: `read_file`, `read_text`, `read_project`, `read_submitted_files`, `Specification`,
   `Syntax`, and `Project`'s `manifest` and `documents`, with each document's `root`. CI tests
   011 on every pull request; the gate does not.
7. **The runner** ([the pieces rule](../../conformance/README.md#the-coverage-index-and-the-case-schema-in-pieces)):
   - a `translate` step, `{kind: translate, files: [ALIAS, ...]}`, read as one project as `load`
     reads it. Its reply holds `accepted` and `diagnostics` as `load`'s does, and on acceptance
     two expectations: `framed`, the alias of a file whose documents equal the documents handed
     over as a set, each by SCH-007, none more and none less; and `translations`,
     `[{kind, name, rule}]` for each generated document, in byte order of kind, then name (a
     framed document read beside has none, unless SCT-011 gave it its owner: then `rule: SCT-011`);
   - an `equivalent` option on `register-project`: the alias of a second context fixture whose
     project holds framed documents. The step registers both from the same state with the same
     evidence and request, and fails, naming the first difference, unless reply, state and records
     are equal, diagnostics compared by `ruleId` and `level` alone (SCT-015);
   - their pieces under `conformance/case-schema/`, the step in a new module under `src/steps/`
     with one dispatch line, the expectations in `src/expect.rs`, and one paragraph in
     `conformance/README.md`. Touch `case.schema.json` only in its one open list, if needed.

   Write each expected framed document from the SCT rules, never from the translator's output.
8. **The cases**, under `conformance/language/sectioned/`, IDs `sectioned-*`, one `index.yaml`
   piece, each tagged with the promises its steps assert: at least 30 cases, rejections that share
   files as steps of one case. Accepted translations assert `framed` and `translations`. They show:
   - SCT-001: one Pool makes `N--t` and N (also with `equivalent`); `apiVersion` alone; an unknown
     key, at `/grants`; a frame without `kind`; a JSON array of a framed and a sectioned item; a
     manifest with a section key, still a manifest, rejected by `manifest.v1`; `register.v1` with
     a sectioned value, invalid under SUB-005;
   - SCT-002: the version from the manifest; stated, with no manifest; neither; unknown;
   - SCT-003: a framed template used by a sectioned `from`, and a framed Conduit whose consumer is
     a sectioned Agent (with `equivalent`); one bad document rejecting the set (SUB-003); a
     generated name equal to a framed document's (NAM-027);
   - SCT-004: files and keys reordered and a file renamed; flow and block style
     (ruling 63 point 2); a `{kind, name}` map in `config`, a secret reference and `arguments`, unchanged;
   - SCT-005: all five families, `inputs` kept, `description` to `metadata`; `templates.fleets`;
   - SCT-006: inline in each of the four sections (with `equivalent`); `from` in INS-025's forms
     with `arguments`; `type` with `from`; neither; inline `arguments` or `inputs`; a Resource
     with no owner; `description` on the instance alone;
   - SCT-007: inline; `from` a core scope template (`core#pool-view`), a package's (`#read` of
     `example.org/s1/filesystem`, translated only, since translation reads no package) and a local
     one; a bare `to`; mixed forms; `consumes` on a Pool;
   - SCT-008: to `management`; to `access` with no bound, and through `from`; to `access` with an
     explicit `bound.v1`, accepted;
   - SCT-009: inline; referenced; on a Pool, null included;
   - SCT-011: the key (with `equivalent`), a framed Agent in its file given the owner; one Fleet
     from two files; two in one file; another owner; an Agent in another file left with none; at
     install, a templates-only sectioned package that installs and exports its template, and one
     holding `fleet` (MAN-011) and one with an empty `agents: {}` (MAN-011), as new archives from
     `examples/make_archives.rs` with their digests;
   - SCT-013: `--`, a reserved name, a `consumes` key `t` or `operator`, a generated name too long;
   - SCT-016: inline and `from`, `as` kept and defaulting to N; to `management`, accepted.

   Declare them in `docs/work-orders/242-declared.yaml` (`kind: added`,
   [the format](../../conformance/README.md#reviewing-vector-changes)); nothing else should need
   a declaration.
9. **Tests outside the runner**, by these names:
   - `every_valid_sectioned_example_reads_as_its_translation`, in `loader/tests/examples.rs`:
     `read_project` reads `valid/sectioned-project` and `valid/sectioned-package` (as they stand
     when you start; order 240 may replace the second) and hands over the documents their
     translation makes, kinds and names asserted. The "043a/043b" comment goes; the invalid SCT
     examples and `shell/tests/by_rule.rs` stay as they are, for order 244.
   - `equivalent_fails_on_a_differing_framed_project`, a unit test of the runner: a framed fixture
     that differs by one operation fails the step, naming the difference.
   - `short_form_directory_registers_through_the_cli`, in a new `shell/tests/short_form.rs`:
     after initializing a store with `conduit init --store DIR`, `conduit register` of a
     short-form directory of core contracts alone exits 0, and the store holds its generated names.

   Add a section to `engine/CONFORMANCE.md` saying where translation happens and what order 244
   leaves, and run `tools/promise_standing.py --write`.

## What to read

- `spec/`: the SCT promises and `sectioned.v1`; MAN-001, MAN-003, MAN-011, SUB-001, SUB-003,
  SUB-005, INS-025, NAM-027, FLEET-002; `core/templates.yaml` and `core/requirements.yaml`.
- The loader's `src/` and `tests/examples.rs`; the core's `archives.rs`; the runner's `src/steps/`,
  `packages.rs`, `expect.rs` and `examples/make_archives.rs`; `conformance/README.md`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
grep -c '043a/043b' engine/crates/loader/tests/examples.rs
grep -rl 'sectioned' engine/crates/loader/src engine/crates/core/src engine/crates/conformance/src | wc -l | tr -d ' '
test -e conformance/language/sectioned; echo "cases $?"
test -e conformance/case-schema/inputs/translate.json; echo "translate $?"
grep -c '^- id: SCT-' spec/promises/sections.yaml
grep -cE '^\| SCT-0[0-9]{2} \| language \| must \| 0 \| 0 \| outside \| none \| no case, no reason \|$' engine/STANDING.md
grep -c 'manages' spec/schemas/sectioned.yaml
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
(cd experiments/011-short-form-spike && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "spike $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `0`, `cases 1`, `translate 1`, `14`, `14`, `0`, `engine 0`, `spike 0`
and `0`. Orders 240 and 241 may merge first; neither changes a line here. If order 243 has
merged, use the store initialization it keeps. `engine` or `spike` printing otherwise means a
crate is missing from the cache: stop and report, and fetch nothing.

## Scope

Change `engine/crates/loader/`; `engine/crates/core/src/archives.rs`, for decision 3's MAN-011
rule alone; `engine/crates/conformance/` (`src/`, `examples/make_archives.rs`); the new
`engine/crates/shell/tests/short_form.rs`; under `conformance/`, new pieces in `case-schema/` and
an `equivalent` property in `case-schema/inputs/register-project.json`, `case.schema.json`'s open
list if needed, the new `language/sectioned/`, new archives with their digests and README lines
in `fixtures/install-archives/`, and one paragraph of `README.md`;
`engine/CONFORMANCE.md` (one section); `engine/STANDING.md` (by the tool); and the new
`docs/work-orders/242-declared.yaml`. Change nothing else: not `spec/`, `conformance/manifest.yaml`,
`s1-promises.yaml` or `expected-failures.yaml`, the shell's `src/`, `experiments/`, `packages/`,
`tools/`, and no `Cargo.toml` or `Cargo.lock`.

## Done when

Write decision 9's tests first and run them on the unchanged code: report each failing line. Then
change the code until all pass. Each line marked "fails today" prints otherwise on `main` at
`cc3605a3`; all pass after.

```sh
(cd engine && cargo run --release -p conduit-conformance -- .. sectioned- 2>&1 | grep -c '^PASS sectioned-')  # 30 or more; fails today (0)
grep -E '^\| SCT-0(0[1-9]|11|13|16) ' engine/STANDING.md | grep -c '| case passes |$'  # 12; fails today (0)
(cd engine && cargo test --release -p conduit-loader --test examples every_valid_sectioned_example_reads_as_its_translation 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; fails today (0)
(cd engine && cargo test --release -p conduit-conformance --lib equivalent_fails_on_a_differing_framed_project 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; fails today (0)
(cd engine && cargo test --release -p conduit-shell --test short_form 2>&1 | grep -c '^test result: ok\. 1 passed')  # 1; fails today (0)
grep -c '043a/043b' engine/crates/loader/tests/examples.rs  # 0; fails today (1)
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/242-declared.yaml >/dev/null; echo "delta $?"  # delta 0; fails today (2)
```

These pass, as on `main`; run the whole engine suite once, last:

```sh
(cd engine && cargo run --release -p conduit-conformance -- .. sectioned- 2>&1 | grep -c '^EXPECTED FAILURE')  # 0
(cd engine && cargo run --release -p conduit-conformance --example make_archives -- --check)
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
/usr/bin/python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/set_names.py --check
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short | grep -vE ' (engine/crates/(loader/|conformance/|core/src/archives\.rs|shell/tests/short_form\.rs)|engine/[CS][A-Z]+\.md|conformance/|docs/work-orders/242-declared\.yaml)' | wc -l | tr -d ' '  # 0
(cd experiments/011-short-form-spike && cargo test --release)  # every test passes
(cd engine && cargo test --release)  # every test passes
```

Your report gives, per decision, what changed and the rule and place of each kind of rejection;
each case in `vectors`; each new test's failing line on the unchanged code, then its passing line;
each existing test you changed, and why; your own choices under `own_choices`; and every check's
last line.

Stage nothing.

## What follows

Order 244: the rest of SCT-014 and SCT-015, the spec's sectioned examples through the engine,
the S1 inventory's SCT rows and DEC-007. Orders 245 and 250 read through this loader.

## Record

Not yet run.
