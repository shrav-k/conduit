# Work order 273: inspection lists each package an object reaches through its project's contracts, so that plan sees such a pin move (order 267's finding)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` the one case you add, with the promises it tests.
This order needs no network: it reaches no host and fetches no crate. Every crate it builds is in
the local cache already: run cargo offline, and if a build asks for a crate that is not there,
stop and report, and fetch nothing. Run one cargo command at a time. Never print a whole large
file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Do not run `conduit` outside a test,
`npm`, `node`, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user or machine name, or private repository name.

## What and why

**The finding.** Order 267's writer: "`Held::package_list` lists the packages that the object's
own document and its selected items name, not one reached only through a project contract, so a
moved pin of such a package shows only as a moved submission digest with every document
unchanged." So `conduit plan` (order 245) takes the safe side there: K3's row 18 updates, row 19
fails and row 26 stops for `--replace`
([the plan-and-apply reference](../implementation/plan-and-apply/README.md)). Row 18 names "a
package whose pin moved"; plan reads only what inspection shows (K3's step 2).

**What `spec/` and the engine hold today.** NAM-009: a template "keeps the address, version and
digest of each package it refers to", and inspecting it "lists those packages under `packages`".
`inspect.v1` shows the list registration stored. `Held::package_list` builds it from the scopes
of the object's own references (`packages::selections_document`, each resolved by
`Held::select`) and of its selected implementations. A Type, Capability, Operation or Schema of
the submission that names another package's contract is checked at submission (the same
`selections_document` and `resolve_name` in `packages.rs`), and `held.contexts` keeps that
package's address, version and digest, but the list leaves it out. A package's own pins are in its
archive, so its digest moves with them (PKG-004).

**The coordinator's decision**, open to the owner's strike: NAM-009's "each package it refers to"
and PKG-007's "uses a package" include a package reached through a contract of the object's
submission; the list adds it, and only it. Change no promise.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, every
access can be inspected: inspection shows each package the object's meaning depends on; 5, every
management action is available to an agent: one inspection shows which pin moved. Newly shown, to
whoever may inspect the object (ACC-033): a package's address, version and digest, which the
object already holds. Stored and recorded differently: a newly registered object's `packages`,
and the record of an inspection of it, gain that item. Refused differently: an instance of a
template that reaches a release not installed (PKG-007). None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a lockfile such as `Cargo.lock` lists each package the build reaches, not
only those the root names, so a moved pin shows in its own entry. Not taken: building the list
at inspection from `held` (the shown list would differ from the kept one, against LIF-013's
check); walking every package's contracts too (its digest already covers its pins); keeping 245's
safe side for every object.

**When it runs.** On `main` once order 267 has merged. Through Conduit's dogfood: one Codex worker
turn (4-hour limit), a blind Codex verifier and Claude reader, one fix turn, the gate (DEC-046,
DEC-033). By ruling 89, each decision says what must hold; how is yours. The words, the walk's
bound (the submission's contracts only) and the case's name are the coordinator's.

**Shared files.** Order 270, not merged, changes `inspect.v1`'s description, `spec/core/README.md`,
`spec/CHANGES.md` and `engine/STANDING.md` at other lines. Whichever merges second rebases and runs
`promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: core `records.rs`, `packages.rs`,
  `requests.rs`, `lifecycle.rs` or `lib.rs`, a shell or loader source,
  `engine/crates/conformance/`, a package, a `Cargo.toml` or a lockfile;
- an existing case or test fails, or `vector_delta.py` shows a change other than the added case:
  where the only change is decision 2's item in an expected `packages`, add that item, list the
  case or test under `own_choices`, and declare a case's change in `273-declared.yaml` as
  `kind: expectation` with paths under that `packages`, such as `/steps/*/expect/*/packages/*`
  (`vector_delta.py` compares a list item by item); otherwise report the case and step with
  the expected and the observed reply, and change no other case;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text.** Change no promise.
   - `inspect.v1`'s description in `spec/core/operations.yaml`: "`packages` lists the packages
     the object holds: a package's item has" becomes:

     ```text
     `packages` lists the packages the object holds: each package it refers to, directly or through a contract of its submitted project that it reaches (NAM-022), so that a moved pin of a package that only such a contract names shows in that package's item. A package that only another package's contracts name is not listed, since that other package's digest covers its pin (PKG-004). A package's item has
     ```

   - `spec/core/README.md`: "`packages` is a list. A package's item has" becomes "`packages` is
     a list of each package the object refers to, directly or through a contract of its submitted
     project that it reaches (NAM-022). A package's item has".
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Changed: `inspect.v1`'s `packages` lists each package an object reaches through a contract of its submitted project, so a moved pin of a package that only such a contract names shows in that package's item, not only as a moved submission digest, and an instance of a template that reaches such a package needs that release installed (order 267's finding; NAM-009, PKG-007; ruling 52).
     ```

2. **The engine** (decision 1). In `registration.rs`, `Held::package_list` also lists each scope
   reached through the submission's contracts:
   - where a reference resolves, by `Held::select`, into the scope `project` to a document that
     `packages::contract` accepts, take that document's references by
     `packages::selections_document`, as the submission's resolution in `packages.rs` takes them,
     and resolve each from `project` by `Held::select`, each document once; list the scope of
     each reference so resolved, the core's among them, and walk on from each such document of
     `project`;
   - walk no document of another scope: a package's own references stay unlisted;
   - keep today's sources: the object's own references and its selected implementations.

   Change nothing else. Its three callers in `registration.rs` stay as they are, and so do
   `inspect.v1`'s branch of `records.rs`, 265's `implementations` and 267's `documents`. An
   object kept before this order keeps its list. The list's other readers stay unchanged in code:
   a template's instantiation now needs the reached release installed (PKG-007, the check in
   `Builder::template` that reads `packages`), and a replacement whose list changes counts as
   changed.
3. **One case**, `registration-reached-package-inspected`, in
   `conformance/language/registration/reached-package-inspected/` with its `index.yaml` entry:
   portable (from `{"recipe":"empty"}`), promises `NAM-009` and `PKG-007`. Take its form from
   267's `registration-contract-documents-inspected`, and its snapshots from
   `registration-retained-package-build-metadata`'s fixtures. Write it first, run it on the
   unchanged engine, and report each failing line.
   - First start; then two `register-project` steps, from `file:///projects/one/` and
     `file:///projects/two/`. Each fixture carries a `preinstalled-snapshot` of
     `example.org/reached`, exporting a Schema `note.v1` (`{"type":"string"}`): version `1.0.0`
     with digest `sha256:` and 64 `a` for project one, `2.0.0` with 64 `b` for project two; and,
     in both, one of `example.org/unreached` `1.0.0`, 64 `c`, exporting a Schema `spare.v1` alike.
     Both projects have address `example.org/reaching`, no version, and pin their `reached`
     release and `unreached` `1.0.0`; one item `server` implementing `#read.v1` with
     `command: ["./bin/server"]`; and the same contracts: an Operation `read.v1` whose
     `inputSchema` is
     `{"type":"object","properties":{"note":{"$ref":"example.org/reached#note.v1"}}}`, a
     Capability `reading.v1` listing it, a Resource Type `reader.v1` offering that, and a Schema
     `spare.v1` that nothing else names,
     `{"type":"object","properties":{"spare":{"$ref":"example.org/unreached#spare.v1"}}}`, so a
     list of every pin or every scope fails the case. Project one
     holds the template `reader-one`, which selects `#server` for `#read.v1`, and its instance
     `one`, owned by `system`; project two holds `reader-two`, alike.
   - Inspecting `reader-one` through root's `management`: `packages` has exactly two items, the
     submission's (`example.org/reaching`, digest D1) and
     `{"address":"example.org/reached","version":"1.0.0","digest":"sha256:aaa…"}`. `reader-two`
     gives D2 and `2.0.0` with the `b` digest; D1 and D2 differ, and the submission's item has
     the same `documents` in both. `one`, inspected through `system`, gives `reader-one`'s two
     items. No list has `example.org/unreached`. Today each list has one item.
   - A third `register-project`, from a fixture with no snapshot, manifest address
     `example.org/later`, whose one document is a Resource `three` from `reader-one` owned by
     `system`: rejected with a PKG-007 diagnostic naming `example.org/reached` and `1.0.0`. Today
     it is accepted.

   Take D1, D2 and the documents' digests apart from the engine, as order 267's decision 3 takes
   them. Declare the case in `docs/work-orders/273-declared.yaml`, in the form of
   [256's](256-declared.yaml), and run `python3 tools/promise_standing.py --write`.

## What to read

`inspect.v1`'s description and `packages`; NAM-009, NAM-010, NAM-022, PKG-004, PKG-007 and
SUB-008; in `registration.rs`, `Held`, `Held::select`, `Held::package_list` and its three
callers, and `Builder::template`, whose PKG-007 check reads a template's `packages`; in
`packages.rs`, `selections_document`, `selections_spec`, `resolve_name`, `contract`, and where
the resolution calls `selections_document` for each document; the cases
`registration-contract-documents-inspected` and `registration-retained-package-build-metadata`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 5b83f31a HEAD; echo "main $?"
test -d conformance/language/registration/contract-documents-inspected; echo "267 $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
grep -c '^pub fn document_digest(' engine/crates/core/src/packages.rs
grep -c '^    fn package_list($' engine/crates/core/src/registration.rs
j spec/core/operations.yaml '`packages` lists the packages the object holds: a package'
j spec/core/operations.yaml 'directly or through a contract of its submitted project'
j spec/core/README.md '`packages` is a list. A package'
test -e conformance/language/registration/reached-package-inspected; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `267 0`, `1`, `1`, `1`, `0`, `1`, `case 1`, `engine 0` and `0`.

## Scope

Change `spec/core/operations.yaml`, `spec/core/README.md` and `spec/CHANGES.md` as decision 1
says; `engine/crates/core/src/registration.rs` (its tests among it);
`conformance/language/registration/index.yaml`; and `engine/STANDING.md` only by the tool. Create
only the case directory and `docs/work-orders/273-declared.yaml`. The stop rule's narrow update
aside, change nothing else: not a promise, another case, `docs/` (K3 among it), `packages/`,
`tools/`, a shell or loader source, another core file, `engine/crates/conformance/`, `Cargo.toml`
or `Cargo.lock`. Report anything else that needs changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-reached-package-inspected 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
j spec/core/operations.yaml 'each package it refers to, directly or through a contract of its submitted project that it reaches (NAM-022)'  # 1; today 0
j spec/core/operations.yaml "since that other package's digest covers its pin (PKG-004)."  # 1; today 0
j spec/core/README.md '`packages` is a list of each package the object refers to, directly or through'  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Changed: `inspect.v1`.s `packages` lists each package .*(order 267.s finding; NAM-009, PKG-007; ruling 52)\.$'  # 1; today 0
test -f docs/work-orders/273-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/language/registration/reached-package-inspected
git add -N "$n" docs/work-orders/273-declared.yaml  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/273-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/(operations\.yaml|README\.md)|CHANGES\.md)|engine/crates/core/src/registration\.rs|engine/STANDING\.md|conformance/language/registration/(index\.yaml|reached-package-inspected/.+)|docs/work-orders/273-declared\.yaml)$' | wc -l | tr -d ' '  # 0, but for a case the stop rule's update names
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "$n" docs/work-orders/273-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; each failing line of the case on the unchanged
engine, then its passing line; D1 and D2 as you took them; any case or test the stop rule's update
changed; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- **Order 245's safe side for this case can then be dropped**, in a later plan change, not this
  order. Today, where a document an object reaches names a package missing from its `packages` and
  the manifest moved, plan updates by row 18, fails row 19 and so stops by row 26. For an object
  registered after this order that package has its own item, so plan compares its pin as any
  package's: row 18 updates a project template that refers to it, and rows 19 and 26 decide an
  instance by it. An object kept before this order keeps its shorter list until it is replaced, so
  that change keeps the safe side only where an unchanged reached document names a package that
  `packages` lacks, which then only such an object shows.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` after order 267 merged.

**Worker-2's turn** (07:11 to 07:19) made `Held::package_list` follow each reference that
resolves to a contract document of the submitted project, through that document's own references,
each document once, stopping at package boundaries, so `inspect.v1`'s `packages` lists every
package an object reaches, with address, version and digest, and an instance of a template that
reaches a release not installed is refused (PKG-007). It stopped before the vectors test,
rightly: the changelog line this text prescribed had no decision citation, which the decisions
guard requires. The coordinator added `; ruling 52` to the line and to its Done-when pattern
above; a second turn (07:20 to 07:25) made that change and ran the vectors test. The new case
`registration-reached-package-inspected`, 181 core tests and the vectors test pass.

**Two blind reads at once** (07:26 to 07:28): `turn-verifier-2.turn.v1` passed it with no
findings; the Claude reader passed it with two nits. One accepted: a short turn (07:28 to 07:31)
makes the indirect refusal also name the template and the project contract that reaches the
release, after the existing words. The other nit was the coordinator's untracked copy of this
text, committed with the order.

The gate's run on the squash is the coordinator's, at the merge.
