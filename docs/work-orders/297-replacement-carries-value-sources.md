# Work order 297: a LIF-009 replacement carries each held value's source and checks it against the template as it now stands (DEC-076, the engine's second part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; list under
`vectors` the two cases you add, with the promises each tests. It needs no network: run cargo
offline; if a crate is missing, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, case, diff or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential, even one a test made. Do not run `conduit` outside a test, `codex`, `claude`,
the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user or
machine name, account, or private repository name; every value in a case is fake.

## What and why

**The owner's answer** ([DEC-076](../decisions/log.md#dec-076-2026-11-22-installation-values-are-valuev1-resources)):
the durable form of private values -> "value.v1 Resources (Recommended)". Order 283 wrote it into
the spec; order 288 built the engine's first part and declared two limits in its decision 6. This
order lifts the second limit. The first stays, because no control stores a value on this base
(decision 1).

**Today** (read on `main` at `a28e7fef` with order 288's branch, `wo/288-value-engine`):

- **No control stores a value.** A succeeded control leaves `config` and `revision` as they were:
  neither `requests.rs` nor `revisions.rs` writes them, `engine/CONFORMANCE.md` calls control
  dispatch unfinished, and REV-007 stands `no case, no reason`. INS-013's removal of a source map
  entry by a control under REV-007 has nothing to act on, so this order leaves it to the order
  that builds control storage (What follows).
- **A replacement loses the source.** `registration::replacement` expands the held nested
  instantiation with no value origins, so the new instance has no source map. `prepare` keeps in
  the holder only `_value_sources`, each entry's `{kind: Resource, id}`, and drops the rest of the
  origin: node constraint, document, entry pointer and name as written. With no origins, none of
  288's `values` checks runs on a held scalar under a template that `replace.v1` changed, and a
  refused replacement's diagnostic holds `ruleId` and `message` alone, with no `locations`.
- INS-017 stands `reason, no case`. Order 283 added no cases; order 288 added five.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6,
nothing repeats an effect implicitly: a replacement resolves no name and reads no value Resource,
so retiring one or registering another under its name changes no replacement (OBJ-010); 7,
everything is attributable: each copy names its one source, and a refusal names the document and
entry that supplied it. At risk: 3, a shared thing is owned once and never copied to an agent:
each replacement holds one more copy, as 283 decided, and its source map names the one owner.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Helm keeps each release's values with the release, and `helm rollback`
renders from those stored values, not from where they came from; a compiler keeps source spans
through expansion, so an error found later names the place that was written (rustc's "in this
macro invocation"). Not taken: resolving the name again at replacement, which changes an instance
by a name that may now mean another object (OBJ-010, test 6); holding the scalar as a plain
argument, which hides its source and skips OBJ-009 under a changed template (INS-013); refusing a
replacement because its value Resource was since retired, which ties a value's lifecycle to
instances it no longer feeds (OBJ-010).

**When it runs.** It stacks on `wo/288-value-engine` until order 288 merges, then on `main` from
that merge; the start check stops if 288's work is absent. Through Conduit: one Codex worker turn
(a 4-hour limit; aim for under 2 hours), a blind Codex verifier and Claude reader (DEC-071), one
fix turn, the gate (DEC-046, DEC-033). By ruling 89, each decision says what must hold; how is
yours. The case IDs and steps, the `ruleId` of each check and the test names are the
coordinator's, open to the owner's strike.

**Shared files.** Core `registration.rs`, `registration_values.rs`, `requests.rs`, `lifecycle.rs`
and `records.rs`, the registration `index.yaml`, `engine/CONFORMANCE.md` and `engine/STANDING.md`,
with the CLI's order: whichever merges later rebases and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only where two promises contradict (give
both), a step needs a change outside the scope (give its failing line), or a file outside Scope
must change. Where this order's wording contradicts a promise it does not change, the promise
governs: build to the promise and list it under `own_choices`, with every other choice no decision
governs.

## The decisions

1. **Controls (REV-007): nothing.** No control stores a value on this base, so none removes a
   source map entry. 288's sentence on control dispatch in `engine/CONFORMANCE.md` stays.
2. **The holder keeps each source's origin** (INS-013). Where registration fills a held nested
   instantiation's argument from a `values` entry, written at that entry or forwarded by the
   holder's own input, the holder keeps, at that argument's source map pointer (such as
   `/entries/worker/arguments/home`), the value Resource's ID, its resolved node constraint or
   none, the original document's name, the entry's pointer in it (`/values/account` in the
   Pool's document, `/spec/entries/worker/values/home` in the template that writes it) and the
   reference as written. Only registration writes it. It is not in `declared`, and no reply,
   record or diagnostic shows it except as decision 4 says; `inspect.v1` gives the holder's
   `values` as 288 does.
3. **A replacement carries the source** (LIF-009, INS-013, OBJ-010). A `register-replacement.v1`
   for a Pool's entry or a declared Conduit takes each held scalar as today, and the new
   instance's source map gives each place that scalar fills in its own definition the ID its
   holder keeps for that argument, as registration would. It resolves no source name and reads
   no source Resource: the result is the same whether that Resource was since retired,
   unregistered or another registered under its name. The holder's map and kept origins do not
   change; only its slot names the new instance. A held argument no value supplied is taken as
   today, with no source.
4. **Its checks** (INS-013, OBJ-009, INS-003, INS-005, INS-014, OBJ-021), before the instance is
   made, against the current template and its Type, for each held argument with a source: (1) the
   current template declares the input, else INS-003; (2) a held node constraint is the
   replacement's placement, else OBJ-009, so an instance with no placement takes none; (3) no
   reference schema governs the input or any place the scalar reaches, forwarded destinations
   included, checked before it could resolve as a reference, else OBJ-009, so a value never
   stands for an object or a secret (SEC-001) after a template changes; (4) the scalar satisfies
   the input's schema, else INS-005, and the Type's schema as any argument must (INS-014).
   - A failing check: the request is not dispatched, reports invalid (LIF-009, OBJ-023),
     registers nothing and changes no holder. By the owner's rule (ruling 52) it gives one
     diagnostic with the check's `ruleId`. Its `message.text` has 288's form,
     `<document>: <entry pointer> names {kind: Resource, name: <name>}: <check>`; the check says
     what failed: for (1) that the current template declares no input of that name, for (2) both
     nodes. Its `locations` give the entry's place, `/objects/<document><entry pointer>`, and,
     where the check fails at a receiving input or destination, that place in the current
     template. Neither the reply nor any record holds the scalar.
   - A replacement's other checks, and their order, are as today (POOL-003, PRV-015, LIF-041
     after the checks giving invalid).
5. **INS-017 with held sources.** Two replacements from one held nested instantiation, with the
   same held sources and node constraints while the same objects exist, are equal field by field
   once generated names and IDs are set aside, their source maps included. Where only one held
   source differs (another value Resource with the same scalar), their configurations are equal
   and their source maps differ at that place alone.
6. **`engine/CONFORMANCE.md`,** at its width. In the paragraph beginning "From order 288,", the
   three sentences from "A LIF-009 replacement takes the scalar" to "work for the second part."
   give way to a paragraph beginning "From order 297," which says that a replacement carries each
   held source from origins its holder keeps outside `declared`, resolving no name and reading no
   value Resource; that it checks them against the current template and Type and names the
   original entry when it refuses; and that INS-017 holds with held sources (INS-013, LIF-009,
   INS-017).
7. **Two portable cases and four core tests.** The cases start from `{"recipe":"empty"}` with
   the documents of 288's `registration-values-resolved`; write them first and run them on the
   base, reporting each first failing step. Retire a value Resource as
   `registration-values-unresolved` does, retire `team`'s entry Resource through root's Conduit to
   `team` as `fleets-retire-login-resource` retires `login` through `logins`, replace a template
   as `change-t4` does, and request an entry's replacement as
   `setup-review-replacement-reference` does. Each refusal states its `ruleId`, `message.text`
   and `locations`; each replacement step has `absent` `{text, under: ["/reply", "/records"]}`
   for each scalar.
   - `registration-replacement-sources` (LIF-009, INS-013, INS-017, OBJ-010): capture `acct`'s
     and `home`'s IDs. Retire `acct`; if a portable route unregisters it, as
     `fleets-retire-login-resource` unregisters through `management.unregister.v1`, unregister it
     and register another `acct` with another scalar. Retire the entry's Resource and request its
     replacement, which succeeds. Inspecting the replacement gives the held scalars under
     `config`, the new scalar `absent` under `/reply`, and `values` mapping `/config/account` and
     `/config/home` to the captured IDs; inspecting `team` gives its `values` as before. Retire the
     replacement and request another: inspecting it gives the same `config` and `values`.
   - `registration-replacement-checks` (LIF-009, INS-013, OBJ-009, INS-003, INS-005, OBJ-023):
     with the entry's Resource retired, four times replace `worker-t` from its original with one
     change and request the replacement, which reports invalid with `stateUnchanged`. Steps (i)
     to (iii) hand over 288's `worker.v1` unchanged. (i) No input `home`: INS-003 naming
     `team-t`, `/spec/entries/worker/values/home` and `{kind: Resource, name: home}`.
     (ii) `account`'s schema `{type: integer}`: INS-005 naming `team`, `/values/account` and
     `{kind: Resource, name: acct}`, with `worker-t`'s input in `locations`. (iii) `account`'s
     schema `resource-reference.v1`, `acct`'s scalar being the name of a plain Resource the case
     registers: OBJ-009. (iv) `replace.v1` hands over, with `worker-t`, a `worker.v1` that also
     declares an optional `ref` governed by `resource-reference.v1` (SUB-009), and `account`'s
     `to` also gives `/config/ref`: OBJ-009 naming that destination. Then replace `worker-t`
     with its original: the replacement succeeds.
   - Core tests, each named `value_replacement_<what>`: check (2) refusing, since no portable case
     places a replacement on a node other than `local`; a declared Conduit's replacement carrying
     its source; and decision 5's two comparisons.

   If a step cannot be written portably, leave it out and list it under `own_choices`. Index the
   cases in `conformance/language/registration/index.yaml`, declare them in
   `docs/work-orders/297-declared.yaml` in the form of [271's](271-declared.yaml), and run
   `python3 tools/promise_standing.py --write`.

## What to read

INS-003, INS-005, INS-013, INS-014, INS-017, LIF-009, OBJ-009, OBJ-010, OBJ-021, OBJ-023 and
REV-007; order 288's decisions 4 to 6 and its five cases; in `registration.rs`, `replacement` and
`prepare`'s copy into the holder; `ValueOrigin` in `registration_values.rs`; the replacement paths
in `requests.rs` and `lifecycle.rs`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor a28e7fef HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
sed -n '/^## DEC-076/,/^## DEC-077/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
j spec/promises/instantiation.yaml 'It takes the held scalar and carries the same source Resource ID'
j spec/promises/instantiation.yaml 'their held value sources and node constraints are the same too.'
grep -c 'From order 288,' engine/CONFORMANCE.md  # order 288's work
grep -c 'A LIF-009 replacement takes the scalar held' engine/CONFORMANCE.md
grep -c 'Declare-lost.v1 and control dispatch remain unfinished' engine/CONFORMANCE.md
grep -c '^| REV-007 | convention | must | 0 | 0 | outside | shell | no case, no reason |$' engine/STANDING.md
grep -c '^| INS-017 | language | must | 0 | 0 | reason | core | reason, no case |$' engine/STANDING.md
test -f conformance/language/registration/values-resolved/case.yaml; echo "288 $?"
test -e conformance/language/registration/replacement-sources; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, eight lines `1`, `288 0`, `case 1`, `engine 0` and `0`.

## Scope

Change `engine/crates/core/src/` for decisions 2 to 5 and their tests; `engine/CONFORMANCE.md`
(decision 6); the registration `index.yaml`; `engine/STANDING.md` by the tool. Create only the two
case directories and `docs/work-orders/297-declared.yaml`. No promise changes, so
`spec/CHANGES.md` gains no line. Change nothing else: not a file under `spec/`, another case,
`conformance/manifest.yaml` (INS-017's S1 row keeps its reason), the shell, the loader,
`engine/crates/conformance/`, `packages/`, `tools/`, `docs/decisions/` or a Cargo file. Report
anything else.

## Done when

The first group prints otherwise on the base today and passes after; the second, guards, passes
today and must still pass. Run the core's tests and then the whole vectors test once, last; never
the whole engine suite.

```sh
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. registration-replacement- 2>&1 | tail -1)  # 2 cases passed; today FAIL no cases matched the requested prefix
(cd engine && cargo test --offline --release -p conduit-core --lib value_replacement_ 2>&1 | sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p')  # 4 or more; today 0
grep -c 'From order 297,' engine/CONFORMANCE.md  # 1; today 0
grep -c 'A LIF-009 replacement takes the scalar held' engine/CONFORMANCE.md  # 0; today 1
grep -c '^| INS-017 | language | must | 1 | 0 | reason | core | case passes |$' engine/STANDING.md  # 1; today 0
test -f docs/work-orders/297-declared.yaml; echo "declared $?"  # declared 0; today declared 1
```

```sh
new=$(git status --short -uall | sed -n 's/^?? //p'); git add -N $new  # so the checks see them
for p in registration-values- registration-value-resource-checks access-value-resource-root-only setup-restore setup-review-replacement-reference change-t4 change-revisions-returned e4-tombstone-register-replacement e4-bound-removal-lift-register-replacement; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # each passes, as today
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do PYTHONDONTWRITEBYTECODE=1 python3 tools/$t.py | tail -1; done  # Errors: 0, four times
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/297-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/[a-z_]+\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/language/registration/(index\.yaml|replacement-(sources|checks)/.+)|docs/work-orders/297-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $new  # stage nothing
```

Report, per decision, what changed and where; each case's failing step on the base, then its
passing line; each refusal's message; each core test's name; and every check's last line. Stage
nothing.

## What follows

- **The CLI's** (M, stacked on 288 beside this): `conduit value set`, `list` and `unset`; `set`
  shows the value as typed, echoes it back and saves only on a yes, and takes no secret, which
  only a secret reference carries; the plan's value lines and its stop naming
  `conduit value set <name>`, with no value (K3); records redacted by value name on export.
- **Found while writing:** control storage under REV-007 is unbuilt; the order that builds it
  removes each source map entry at or below a place a control writes, keeping a place whose value
  stays the same (INS-013). INS-017's S1 row keeps its reason: registration's own equal
  instantiations are still not compared.
- Then the dogfood's and the short form's, as order 283 lists them.

## Record

Run on 2026-11-23 through Conduit itself, the first order of stage 3 of experiment 028, by its
pair 1, in Pacific time, on `main`.

**Worker-1's turn** (09:33 to 10:01) built DEC-076's second engine part: a LIF-009 replacement
carries each held value's source Resource, node constraint, document, entry pointer and reference
as written (INS-013), and INS-017 compares held sources. Both new cases and seven focused tests
failed on the unchanged engine and pass, with 211 core tests and the whole vectors test.

**Two blind reads** (10:01 to 10:09). The Codex verifier, `root-verifier-1.turn.v1`, passed it
with no finding. The Claude reader (`reader-1`, while stage 3's reader is held) had one must (a
node refusal's second location named the replacement that was refused and never made), two
shoulds (each refusal says what to do; CONFORMANCE's INS-017 line said more than the engine shows)
and a nit. One fix turn (10:09 to 10:20) applied all four, each failing first; the cases, 211 core
tests and the vectors test pass. The gate's run on the squash is the coordinator's, at the merge.
