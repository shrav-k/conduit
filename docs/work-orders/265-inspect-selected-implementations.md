# Work order 265: inspection names each object's selected implementations, so that plan compares what runs (order 245's finding)

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

**The finding.** Order 245 (`conduit plan`) stopped here. Its worker: "K3 rows 18, 19 and 26
require comparing selected implementations before deciding whether to update a template, retain
an instance's pin or require replacement. inspect.v1 returns the declaration and project digest
but omits held and resolved_implementations." SUB-008 keeps a submitted project's contracts and
implementations out of the objects, and no package can supply them at that address (PKG-029). The
submission's digest covers every contract and the manifest, so a moved digest does not say which
changed, and under NAM-009 two submissions with one digest, read from two directories, run
different programs. K3 ([the plan-and-apply reference](../implementation/plan-and-apply/README.md),
step 2) has plan read "resolved definitions, held packages, revisions" through inspection: "No
private snapshot is an acceptable substitute". **The coordinator's decision:** the engine, which
alone knows what it holds, reports what it selected; plan neither guesses nor keeps a copy.

**What `spec/` and the engine hold today.** TPL-004: "The resolved template shows one
implementation for each such operation and where it came from: the template, the type or the
engine"; no output shows either. NAM-009 has a template keep "each implementation item of that
project that it selects ... with a program given as a relative path resolved under SUB-010".
The engine keeps both: each object's `resolved_implementations` maps an operation's qualified
reference to `source`, `selection`, `item` and `scope` (`registration.rs`, `implementations`),
and each scope's address, version and digest in `held.contexts`, which `Held::package_list`
reads to build `packages`. The `inspect.v1` branch of `records.rs` removes both from `declared`
and reports neither. The engine keeps no digest of a `command` item's program (MAN-010: "a
`command` pins nothing"); a `package` item carries its own pin.

**What rows 18, 19 and 26 need, and no more.** Per operation:
- `selection`: which item; another item for one operation is another execution.
- `package`: which release or submission holds it. A qualified reference names neither a version
  nor a submission (SUB-008), and one object can hold two releases of one address through a
  dependency (NAM-014). A release's digest pins its items and any program given as a relative
  path in its archive (MAN-010), so plan compares it and reads items from its cache (K3 O7).
- `item`, but for the engine: the item as the object holds it. For the project's own items no
  cache holds it, and its resolved program is what tells NAM-009's two directories apart.

Not shown: TPL-004's "where it came from", which no row reads; it is worked out: a key of the
template's `declared.spec.implementation` is the template's, `#engine` the engine's, any other
the type's.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2,
every access can be inspected: inspection now shows what serves each operation; 5, every
management action is available to an agent: an agent learns from one inspection what runs,
without the engine's store. Newly shown, to whoever may inspect the object (ACC-033; Pool
administrators and Agents with an inspect Conduit among them): program paths, absolute under a
project's directory or the engine's cache, package identifiers and remote URLs, which test 2
asks for; an item holds no configuration value or secret. Nothing else is stored or recorded
differently; the outputs are kept for repeats, as every inspection's are. The coordinator's
reading.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: a Kubernetes Pod's `status.containerStatuses[].imageID` reports the
digest that runs beside the `image` its spec names. Not taken: plan resolving the held
selection itself from cached archives (a private copy, which cannot reach a submitted project's
items); returning `held` whole (every contract of every scope, in the engine's internal form);
reading records, which a purge removes (REC-004).

**When it runs.** On `main` from #519. Through Conduit's dogfood: one Codex worker turn (4-hour
limit), then a Codex verifier and a Claude reader at once, blind, who run
only what settles a doubt; one fix turn; the gate (DEC-046, DEC-033). By ruling 89, each decision
says what must hold; how is yours. The member's name and form, the readings that a Conduit, a
first-start object and a hand-built entry have none and that `item` is the held item whole, and
the case's name are the coordinator's, open to the owner's strike.

**Shared files.** Orders 255, 258, 262 and 263 change core `requests.rs`, `lifecycle.rs` and
`lib.rs` and the shell; this order changes none. In core `records.rs`, change only the
`inspect.v1` branch, by a few lines; put the code beside `Held::package_list` in
`registration.rs`. `spec/CHANGES.md` gains one line, as 263's does; `engine/STANDING.md` changes
only by the tool. Whichever merges second rebases and runs `promise_standing.py --write` again.

**Stop rule.** Stop, change nothing further, and report where:

- a wording needs a choice that this order and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: core `requests.rs`, `lifecycle.rs` or
  `lib.rs`, a shell or loader source, `engine/crates/conformance/`, a package, a `Cargo.toml` or
  a lockfile;
- an object that registration made keeps an entry other than the engine's with no `selection`
  or `scope`, or a scope with no context: report the object; invent nothing;
- an existing case fails, or `vector_delta.py` shows a change other than the added case: report
  the case and step with the expected and the observed reply; change no other case;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **The spec text.** Change no promise.
   - `inspect.v1`'s description in `spec/core/operations.yaml`, after "present where that Type
     states signIn (CTR-013).", gains:

     ```text
     The optional output `implementations` is present for a template or an instance, other than a Conduit, a Conduit template or an object that first start makes (FST-001). It has one member for each operation for which the object has an implementation selected under TPL-004, its hooks among them, named by the operation's qualified reference. The member's `selection` is the qualified reference of that implementation, and `github.com/shrav-k/conduit/core#engine` for the engine. Its `package` is the item of `packages` that holds the implementation, which tells apart the releases of one address and the submissions at one address, as a qualified reference does not (SUB-008, NAM-009). Its `item`, absent for the engine, is the implementation item as the object holds it under NAM-009, a program given as a relative path shown as it was resolved and is run (SUB-010, MAN-010), so two submissions with one digest, read from two directories, are told apart. An item binds the configuration only by JSON Pointer, so this output shows no configuration value and no secret.
     ```

   - Its `outputSchema`, after `packages`, an optional property:

     ```yaml
           implementations:
             type: object
             propertyNames:
               $ref: github.com/shrav-k/conduit/core#qualified-reference.v1
             additionalProperties:
               type: object
               properties:
                 selection:
                   $ref: github.com/shrav-k/conduit/core#qualified-reference.v1
                 package:
                   $ref: '#/properties/packages/items'
                 item:
                   $ref: github.com/shrav-k/conduit/core#implementation-item.v1
               required:
               - selection
               - package
               additionalProperties: false
     ```

     Where `check_schemas.py` or the schema profile refuses the pointer, write `packages`' item
     schema there again, and list it under `own_choices`.
   - `spec/core/README.md`, after "(NAM-009, PKG-005).":

     ```text
     `implementations`, for a template or an instance, other than a Conduit, a Conduit template or a first-start object, gives for each operation with a selected implementation its qualified reference as `selection`, the item of `packages` that holds it as `package` and, but for the engine, the item as the object holds it, a relative program resolved, as `item` (TPL-004, NAM-009).
     ```

   - `spec/CHANGES.md`, under `## [Unreleased]`:

     ```text
     - Added: `inspect.v1`'s optional output `implementations`: for a template or an instance, other than a Conduit, a Conduit template or a first-start object, the implementation selected for each operation (TPL-004), the package or submission that holds it, and the item as the object holds it under NAM-009, a relative program resolved (order 245's finding; ruling 52).
     ```

2. **The engine** (decision 1). `inspect.v1` fills `implementations` from what the object keeps,
   storing nothing new, so an object kept before this order inspects the same way:
   - one member per entry of its resolved implementations, save as below;
   - an entry the engine implements: `selection` `github.com/shrav-k/conduit/core#engine` and the
     core package's item, and no `item`;
   - any other: its `selection`, the package of its `scope`, built as `Held::package_list` builds
     a `packages` item, and its `item`. One with no `selection` or `scope`, or whose scope has no
     context, gives no member: registration always writes both, so only a hand-built test state
     holds one, such as `records.rs`'s `notice_fixture`, left as it is.

   Each `package` equals an item of the same result's `packages`. `implementations` is absent for
   a Conduit, a Conduit template and an object first start made (each holds no selection).
   Change no other output, record or decision of the inspection; the `inspect-object` decision
   may cite TPL-004 too.
3. **One case**, `registration-implementations-inspected`, in
   `conformance/language/registration/implementations-inspected/` with its `index.yaml` entry:
   portable (from `{"recipe":"empty"}`), promises `LIF-013`, `NAM-009`, `SUB-008`, `SUB-010` and
   `TPL-004`. Take the directory fixture's form from `packages-relative-program-directory` and
   the steps' form from a portable case that registers and inspects, such as
   `secret-reads-nonroot-required`. Write it first, run it on the unchanged engine, and report
   the failing line.
   - First start; then two `register-project` steps, from `file:///projects/one/` and
     `file:///projects/two/`, with one manifest (address `example.org/inspected`, no version,
     one item `server` implementing `#read.v1` with `command: ["./bin/server"]`) and the same
     contract documents (an Operation `read.v1`, a Capability, a Resource Type offering it), so
     one SUB-008 digest D. Project one also holds the template `reader-one`, which selects
     `#server` for `#read.v1`, its instance `one`, owned by `system`, and a Fleet `f` from
     `github.com/shrav-k/conduit/core#fleet`; project two holds the template `reader-two`, alike.
   - Inspecting `reader-one` through root's `management` gives `implementations` with one
     member, `example.org/inspected#read.v1`: `selection` `example.org/inspected#server`,
     `package` `{"address":"example.org/inspected","digest":D}`, and an `item` whose `command`
     is `["/projects/one/bin/server"]`. `reader-two` gives the same, with
     `/projects/two/bin/server` and the same D. `one`, inspected through `system`, gives
     `reader-one`'s member.
   - `f`'s member `github.com/shrav-k/conduit/core#inspect.v1` has `selection`
     `github.com/shrav-k/conduit/core#engine`, the core package's item as `package`, and no
     `item`; inspecting root's Conduit to `management` and the Pool `system` gives no
     `implementations` (`absent` pointers).

   Declare it in `docs/work-orders/265-declared.yaml`, in the form of
   [256's](256-declared.yaml).

## What to read

`inspect.v1` and `implementation-item.v1`; LIF-013, NAM-009, NAM-014, SUB-008, SUB-010,
TPL-004, MAN-010, FST-001 and ACC-038; in `registration.rs`, `implementations`,
`Held::select`, `Held::package_list` and the inheriting loop after `b.templates.push`; in
`records.rs`, the `inspect.v1` branch; in `packages.rs`, `expand_manifest`; the cases
`packages-relative-program-directory` and `registration-implementation-default`; K3's step 2
and rows 18, 19 and 26.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 48d4f36e HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/core/operations.yaml 'The optional output logins holds the logins its object'
j spec/core/operations.yaml 'The optional output `implementations`'
grep -c '^      implementations:$' spec/core/operations.yaml
j spec/core/README.md 'that identifies the contracts of that submission and `version` where its manifest states one (NAM-009, PKG-005).'
grep -cF 'outputs = json!({"id":o["id"],"revision":o["revision"],"packages":' engine/crates/core/src/records.rs
grep -cF 'result[&op] = json!({"source":source,"selection":q,"item":implementation,"scope":ih.scope});' engine/crates/core/src/registration.rs
test -e conformance/language/registration/implementations-inspected; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `0`, `0`, `1`, `1`, `1`, `case 1`, `engine 0` and `0`.

## Scope

Change `spec/core/operations.yaml`, `spec/core/README.md` and `spec/CHANGES.md` as decision 1
says; `engine/crates/core/src/registration.rs`, and `records.rs` only in its `inspect.v1`
branch; `conformance/language/registration/index.yaml`; and `engine/STANDING.md` only by the
tool. Create only the case directory and `docs/work-orders/265-declared.yaml`. Change nothing
else: not a promise, another case, `docs/` (K3 among it), `packages/`, `tools/`, a shell or
loader source, core `requests.rs`, `lifecycle.rs` or `lib.rs`, `engine/crates/conformance/`,
`Cargo.toml` or `Cargo.lock`. Report anything else that needs changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine suite,
once, last.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-implementations-inspected 2>&1 | tail -1)  # 1 cases passed; today no cases matched the requested prefix
j spec/core/operations.yaml 'The optional output `implementations` is present for a template or an instance, other than a Conduit'  # 1; today 0
grep -c '^      implementations:$' spec/core/operations.yaml  # 1; today 0
j spec/core/README.md '`implementations`, for a template or an instance, other than a Conduit'  # 1; today 0
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- Added: `inspect.v1`.*(order 245.s finding; ruling 52)\.$'  # 1; today 0
test -f docs/work-orders/265-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
n=conformance/language/registration/implementations-inspected
git add -N "$n" docs/work-orders/265-declared.yaml  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/265-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/(operations\.yaml|README\.md)|CHANGES\.md)|engine/crates/core/src/(records|registration)\.rs|engine/STANDING\.md|conformance/language/registration/(index\.yaml|implementations-inspected/.+)|docs/work-orders/265-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "$n" docs/work-orders/265-declared.yaml  # stage nothing
```

Report, per decision, what changed and where; the case's failing line on the unchanged engine,
then its passing line; the digest D; and every check's last line.

Stage nothing.

## What follows

Named here and not in this order:

- **Order 245 reads it** in K3's step 2 and compares, for each object other than a Conduit, the
  engine's `implementations` with the file's resolution of the same object, member by member:
  - row 18, a project template: beyond its document and pins, it is updated in place where an
    operation is in one only, a member has another `selection`, or a member whose `package` is
    the project's submission has another `item`, compared as RFC 8785 bytes. A release's
    member needs no `item` comparison: a moved pin is row 18's own clause;
  - row 19, an object whose held pin moved that row 18 does not replace: execution is the same
    only where the operations and each `selection` are, each member of the submission has the
    same `item`, and, for each release member whose pin moved, both cached archives' manifests
    write the same item and any relative program has the same bytes in both (O7). With K3's
    other places equal, it keeps its pins and prints its retained-pin line;
  - row 26: an instance where row 19's comparison fails, its stop line carrying
    `pin=ADDRESS@VERSION`. A Conduit has no member: rows 19 and 26 compare its resolved
    operations, as now.
- **Found while writing:** the project contract documents an object resolved against are still
  not shown, so a moved digest may be an unrelated contract edit. For the owner: either a later
  order reports a digest for each held project document, or 245 takes the safe side (where the
  digest moved, row 18 updates every project template that refers to a project contract and
  row 19 fails for every such instance, so row 26 stops it for `--replace`), which K3's row 19
  does not make.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's turn** (01:13 to 01:52) added `inspect.v1`'s `implementations`: for a template or an
instance, each operation's selection, the package it comes from and the item as held, built
beside `Held::package_list`, with the portable case `registration-implementations-inspected`,
which fails on `main` and passes. The whole engine suite stopped once at
`s2_dependent_call_after_close`, a test this order does not reach, which passed alone.

**Two blind reads at once** (01:53 to 01:56), the owner's answer for A77's window:
`turn-verifier-2.turn.v1` found two musts; the Claude reader passed it with one should, the
verifier's first must in other words. One fix turn (01:57 to 02:16), each must reproduced first,
the coordinator deciding each (open to the owner's strike):
- an item could carry an author's literal for an input marked `isSecret`; such a `value` or
  `default` is now shown as `(secret: not shown)`, and the description says every other literal
  an author wrote is shown to anyone allowed to inspect the object, so secrets belong in secret
  references (SEC-021);
- a Type that records its outputs would have recorded `implementations`; it is kept out of
  recorded outputs and stays in the reply and its repeat (REC-002).

170 core tests and the case pass. The fix turn's whole-suite run stopped at an unrelated CLI test
while the machine's disk was full; from this order on, workers run targeted tests and the gate
alone runs the whole suite. At the merge the gate's names check flagged the masking test's core address,
`example.org/core`, as a misspelling of the core package's; the coordinator renamed it to
`example.org/fixture-engine-package`, which the test does not otherwise read, and the test passes.

The gate's run on the squash is the coordinator's, at the merge.
