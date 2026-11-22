# Work order 267: inspection lists each contract document of a submitted project by its digest, so that plan sees which contract changed (order 245's finding)

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

**The finding.** Order 245 (`conduit plan`) stopped on K3 rows 18, 19 and 26
([the plan-and-apply reference](../implementation/plan-and-apply/README.md)). Order 265 gave it
each object's selected implementations. Its writer found what remains: "A submitted project's own
contract documents are still not shown, and its digest covers all of them, so a moved digest
can't say which contract changed." Row 18 updates a project template that refers "to a contract
document or an implementation item of the project that changed"; row 19 keeps a pin only where
meaning and execution would not differ; row 26 replaces the instance where row 19 fails. K3's
step 2: "No private snapshot is an acceptable substitute". **The coordinator's decision**, open to
the owner's strike: the engine reports what it holds; plan neither keeps a copy nor forces
`--replace` on every instance that uses a project contract.

**What `spec/` and the engine hold today.** SUB-008's digest is the SHA-256, written as PKG-004
writes a digest, of the RFC 8785 form of `{documents, manifest}`: the project's contract documents,
ordered by `kind` then `metadata.name` by their UTF-8 bytes, and the manifest as handed over.
`inspect.v1` shows that digest on the submitted project's item of `packages`, and nothing that
says which part moved. Each object's `held` keeps the whole project: `contexts.project` holds its
address, version and digest, and `indexes.project` its documents by name (NAM-027: one name, one
document), the same values `packages::project_digest` hashed. The `inspect.v1` branch of
`records.rs` removes `held` and shows none of them.

**What rows 18, 19 and 26 need, and no more.** For each contract document of the submission the
object holds: its `kind`, its `name` and the digest of its RFC 8785 form, which plan takes the
same way from the author's file. RFC 8785 is compositional: a document's form alone is the bytes
it has inside SUB-008's list, so the two digests use one canonicalization. Every document, not
only those the object reaches: plan works out the reach from the file (What follows), and where
every document is unchanged and the submission digest moved, the manifest moved. Not shown: any
document's content, and a digest of the manifest, which that deduction gives.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, every
access can be inspected: inspection now shows what the object resolved against; 5, every management
action is available to an agent: one inspection tells it which contract changed. Newly shown, to
whoever may inspect the object (ACC-033): the kind and name of each contract of its submission and a
digest. A contract holds no configuration value or secret. Nothing is stored or recorded
differently. The coordinator's reading.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, by
existing practice: an OCI image manifest names each layer by its own digest beside the image's
digest, so two images show which layers differ. Not taken: showing each document whole (more than
any row reads); listing only the documents the object reaches (the engine would define reach, which
the file gives plan); the safe side 265 offered (every instance using a project contract stopped for
`--replace` on any edit).

**When it runs.** On `main` once order 265 has merged; it changes the same `inspect.v1` branch.
Through Conduit's dogfood: one Codex worker turn (4-hour limit), a blind Codex verifier and Claude
reader, one fix turn, the gate (DEC-046, DEC-033). By ruling 89, each decision says what must
hold; how is yours. The member's name and form, listing every contract document, the public
function, and the case's name are the coordinator's, open to the owner's strike.

**Shared files.** In core `records.rs`, change only the `inspect.v1` branch, by a few lines; put
the code beside `Held::package_list` in `registration.rs`; in `packages.rs`, add one function
beside `project_digest`. `spec/CHANGES.md` gains one line; `engine/STANDING.md` changes only by the
tool. Whichever merges second rebases and runs `promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: core `requests.rs`, `lifecycle.rs` or
  `lib.rs`, a shell or loader source, `engine/crates/conformance/` beyond decision 2, a package,
  a `Cargo.toml` or a lockfile;
- an object that registration made has its submission's item in `packages` but no
  `indexes.project` in `held`, or a digest the engine gives differs from the one you take apart
  from it (decision 3): report the object or the document; invent nothing;
- an existing case fails, or `vector_delta.py` shows a change other than the added case: report
  the case and step with the expected and the observed reply; change no other case;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text.** Change no promise.
   - `inspect.v1`'s description in `spec/core/operations.yaml`, after "and `version` where its
     manifest states one.", gains:

     ```text
     That item also has `documents`: for each contract document of that submission, in the order SUB-008 lists them, its `kind`, its `name`, and as `digest` the SHA-256, written as PKG-004 writes a digest, of the RFC 8785 form of that document, the form it has in SUB-008's list, so an author takes the same digest from the document as read from its file, or as translated from a sectioned file. A moved submission digest so says which documents changed and, where none did, that the manifest did. No document's content is shown, and no other item has `documents`.
     ```

     In 265's sentence on `implementations`, "Its `package` is the item of `packages` that holds
     the implementation," becomes "Its `package` is the item of `packages` that holds the
     implementation, without its `documents`,".
   - Its `outputSchema`: the first branch of `packages`' `anyOf` gains, after `digest`:

     ```yaml
                   documents:
                     type: array
                     items:
                       type: object
                       properties:
                         kind:
                           enum:
                           - Capability
                           - Operation
                           - Requirement
                           - Schema
                           - Type
                         name:
                           $ref: github.com/shrav-k/conduit/core#contract-name.v1
                         digest:
                           $ref: github.com/shrav-k/conduit/core#digest.v1
                       required:
                       - kind
                       - name
                       - digest
                       additionalProperties: false
     ```

     Indent it as `digest` is. Where `check_schemas.py` refuses `contract-name.v1`, use
     `export-name.v1` and list it under `own_choices`.
   - `spec/core/README.md`: "and `version` where its manifest states one (NAM-009, PKG-005)."
     becomes "and `version` where its manifest states one, and `documents`, the kind, name and
     digest of each of that submission's contract documents, taken as SUB-008 takes the
     submission's (NAM-009, PKG-005, SUB-008)."; in 265's sentence, "that holds it as `package`"
     becomes "that holds it, without its `documents`, as `package`".
   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Added: `inspect.v1`'s `packages`: the submitted project's item lists `documents`, the kind, name and digest of each contract document of that submission, the digest taken by SUB-008's canonical form, so a moved submission digest says which contract changed; no content is shown (order 245's finding; ruling 52).
     ```

2. **The engine** (decision 1). `inspect.v1` fills `documents` from what the object keeps,
   storing nothing new, so an object kept before this order inspects the same way:
   - in `packages.rs`, beside `project_digest`, `pub fn document_digest(document: &Value) ->
     Result<String>`: `bytes_digest` of `serde_json_canonicalizer::to_vec(document)`, the
     canonicalization `project_digest` uses; plan calls the same function on the file's document;
   - the item of the result's `packages` whose `address` and `digest` equal its `held`'s
     `contexts.project` gains `documents`: each document of `held`'s `indexes.project` that
     `packages::contract` accepts, ordered as `project_digest` orders them, as
     `{"kind","name","digest"}`;
   - no other item gains it, nor 265's `package` of an `implementations` member; an object with no
     `held`, or whose `held` has no `contexts.project` with a digest, gives none, as first start's
     objects and hand-built test states such as `records.rs`'s `notice_fixture`;
   - the conformance runner's LIF-013 check in its `records.rs` compares each item of the
     output's `packages`, without `documents`, with the stored item.

   Change no other output, record or decision of the inspection; the `inspect-object` decision
   may cite SUB-008 too.
3. **One case**, `registration-contract-documents-inspected`, in
   `conformance/language/registration/contract-documents-inspected/` with its `index.yaml`
   entry: portable (from `{"recipe":"empty"}`), promises `NAM-009` and `SUB-008`. Take
   its form from 265's `registration-implementations-inspected`. Write it first, run it on the
   unchanged engine, and report the failing line.
   - First start; then two `register-project` steps, from `file:///projects/one/` and
     `file:///projects/two/`, with one manifest (address `example.org/documented`, no version,
     one item `server` implementing `#read.v1` with `command: ["./bin/server"]`) and the contract
     documents an Operation `read.v1`, a Capability `reading.v1` listing it, a Resource Type
     `reader.v1` offering it, and a Schema `note.v1` that nothing refers to, whose
     `metadata.description` is `first` in project one and `second` in project two. Project one
     also holds the template `reader-one`, which selects `#server` for `#read.v1`, and its
     instance `one`, owned by `system`; project two holds the template `reader-two`, alike.
   - Inspecting `reader-one` through root's `management`: the item of `packages` whose address is
     `example.org/documented` has digest D1 and `documents` exactly
     `[{"kind":"Capability","name":"reading.v1","digest":C},{"kind":"Operation","name":"read.v1","digest":O},{"kind":"Schema","name":"note.v1","digest":N1},{"kind":"Type","name":"reader.v1","digest":T}]`,
     and its `implementations` member `example.org/documented#read.v1` has a `package` with no
     `documents`. `reader-two` gives D2 and the same list with N2 for `note.v1`; D1 and D2
     differ, as N1 and N2 do. `one`, inspected through `system`, gives `reader-one`'s item.
   - Inspecting root's Conduit to `management` gives `packages` with no `documents` (an `absent`
     pointer).

   Take C, O, N1, N2, T, D1 and D2 apart from the engine, from the case's own files: for a
   document with ASCII member names and no fractional number, Python's
   `json.dumps(d, sort_keys=True, separators=(",", ":"), ensure_ascii=False)` is its RFC 8785
   form; hash it with `hashlib.sha256` and write `sha256:` and the hex. D1 and D2 are SUB-008's
   map taken the same way. Declare the case in `docs/work-orders/267-declared.yaml`, in the form
   of [256's](256-declared.yaml), and run `python3 tools/promise_standing.py --write`.

## What to read

`inspect.v1`, with 265's `implementations`; SUB-008, SUB-009, SUB-010, NAM-009, NAM-022,
NAM-027 and PKG-004; in `packages.rs`, `project_digest`, `bytes_digest` and `index`; in
`registration.rs`, `Held`, `Held::package_list` and where `prepare` builds `indexes`; in
`records.rs`, the `inspect.v1` branch; the case `registration-implementations-inspected`; K3's
step 2 and rows 18, 19 and 26.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor d7c81f61 HEAD; echo "main $?"
test -d conformance/language/registration/implementations-inspected; echo "265 $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/core/operations.yaml 'and the item of the submitted project whose contracts the object holds has'
j spec/core/operations.yaml 'Its `package` is the item of `packages` that holds the implementation,'
j spec/core/operations.yaml 'That item also has `documents`'
grep -c '^              documents:$' spec/core/operations.yaml
j spec/core/README.md 'and `version` where its manifest states one (NAM-009, PKG-005).'
grep -c '^pub fn project_digest(' engine/crates/core/src/packages.rs
grep -c '^pub fn document_digest(' engine/crates/core/src/packages.rs
test -e conformance/language/registration/contract-documents-inspected; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `265 0`, `1`, `1`, `0`, `0`, `1`, `1`, `0`, `case 1`, `engine 0` and `0`.

## Scope

Change `spec/core/operations.yaml`, `spec/core/README.md` and `spec/CHANGES.md` as decision 1
says; `engine/crates/core/src/registration.rs`, `packages.rs` only by the new function, and
`records.rs` only in its `inspect.v1` branch; the conformance runner's `records.rs` only as
decision 2 says; `conformance/language/registration/index.yaml`;
and `engine/STANDING.md` only by the tool. Create only the case directory and
`docs/work-orders/267-declared.yaml`. Change nothing else: not a promise, another case, `docs/`
(K3 among it), `packages/`, `tools/`, a shell or loader source, core `requests.rs`,
`lifecycle.rs` or `lib.rs`, another conformance runner file, `Cargo.toml` or `Cargo.lock`. Report
anything else that needs changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-contract-documents-inspected 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
j spec/core/operations.yaml 'That item also has `documents`: for each contract document of that submission'  # 1; today 0
grep -c '^              documents:$' spec/core/operations.yaml  # 1; today 0
j spec/core/operations.yaml 'that holds the implementation, without its `documents`,'  # 1; today 0
j spec/core/README.md 'and `documents`, the kind, name and digest of each'  # 1; today 0
j spec/core/README.md 'that holds it, without its `documents`, as `package`'  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Added: `inspect.v1`.s `packages`: .*(order 245.s finding; ruling 52)\.$'  # 1; today 0
grep -c '^pub fn document_digest(' engine/crates/core/src/packages.rs  # 1; today 0
test -f docs/work-orders/267-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/language/registration/contract-documents-inspected
git add -N "$n" docs/work-orders/267-declared.yaml  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-conformance --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/267-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/(operations\.yaml|README\.md)|CHANGES\.md)|engine/crates/(core/src/(records|registration|packages)|conformance/src/records)\.rs|engine/STANDING\.md|conformance/language/registration/(index\.yaml|contract-documents-inspected/.+)|docs/work-orders/267-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "$n" docs/work-orders/267-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; the case's failing line on the unchanged engine,
then its passing line; C, O, N1, N2, T, D1 and D2 as you took them; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- **Order 245 reads it** in K3's step 2. For each object whose `packages` holds a submission's
  item, plan takes each contract document of the file's project by `packages::document_digest`,
  on the document as read or translated, and compares with the item's `documents`:
  - row 18, a project template: a contract of the project changed for it where a document it
    reaches is not held with the same kind, name and digest. It reaches the project documents its
    own references name, and those each of them names in turn (NAM-022), walked in the file's
    project: while each document met is held unchanged, its references are the held ones, so the
    walk needs no held content. An edit to a document it does not reach leaves it as it is;
  - row 19, an object whose held submission digest is not the file's, that row 18 does not
    replace: its meaning is the same where no document it reaches changed; with 265's comparison
    of `implementations` and the pins in `packages`, it keeps its pins and prints its retained-pin
    line. Where a document it reaches names a package not in its `packages` (below) and the
    manifest moved, 245 takes the safe side: row 18 updates; row 19 fails, so row 26 stops;
  - row 26: an instance that reaches a changed document through its template fails row 19 and is
    replaced under `--replace NAME`; its stop line's form is 245's.
- **Found while writing:** `Held::package_list` lists the packages that the object's own
  document and its selected items name, not one reached only through a project contract, so a
  moved pin of such a package shows only as a moved submission digest with every document
  unchanged. Listing it is a later order's.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main` with order 265 stacked while it waited in the merge queue.

**Worker-2's first turn** (03:18 to 03:19) stopped before any edit: keeping `documents` out of
recorded outputs, as order 265 keeps `implementations` out, needs `requests.rs`'s record
projection, which the scope excluded. The coordinator widened the scope by that projection alone
(open to the owner's strike). **Its second turn** (03:19 to 03:27) built the order: the submitted
project's item of `packages` gains `documents`, each held contract document's kind, name and the
SHA-256 of its RFC 8785 form, by the canonicalizer the submission digest uses; replies and
retained repeats keep it, recorded outputs omit it. Both inspection cases and 171 core tests pass.

**Two blind reads at once** (03:28 to 03:30): `turn-verifier-2.turn.v1` recomputed all seven
digests by hand and found one must; the Claude reader passed it with one nit. One fix turn (03:31
to 03:34):
- the output schema accepted `documents` under an implementation's `package`; it now rejects it
  there and keeps it on the submitted-project item;
- an object holding a project with no held index now inspects without `documents` instead of
  failing the request.

The gate's run on the squash is the coordinator's, at the merge.
