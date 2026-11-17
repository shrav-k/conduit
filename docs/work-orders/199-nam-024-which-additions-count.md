# Work order 199: NAM-024 says which additions count

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since this order adds and changes no case. This
order has network access only because the engine's tests bind loopback ports, which the sandbox
refuses without it; use it for nothing else, and reach no other machine. Run one cargo command at
a time. Never print a whole large file or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `claude`, `codex` or
`client-facts`, and outside your working tree read nothing under your home directory except
Cargo's and rustup's own files; never a login file or a keychain item.

## What and why

This is the named spec fix that [ruling 82](../proposals/ruling-82/README.md#the-decisions),
point 4, places before order 050 is written. NAM-024 (`spec/promises/naming.yaml`, lines 414 to
439) lets a later release that exports a contract under the same name "add optional things, such
as an optional input of an operation, a further operation of a capability or an optional property
of a Type's `schema`", and forbids it to "remove or change anything the earlier one held". It
does not say whether a login appended to a Type's `signIn.logins` is one of those optional things.
The targeted review of the sign-in and home surface read it both ways
([its record](../proposals/review-signin-2026-11-16/README.md#1-the-open-line-clause-by-clause),
o6): V11's Lens B held that a later login is not safe to defer under NAM-024 as written, since it
changes which login an unasked sign-in may start; V33, merged into V11, held that it defers,
since NAM-024's list is "such as" and a further login widens what a sign-in may start as a further
operation widens a capability; G02's Lens B added that NAM-024's own example is an element
appended to a required list
([V11](../proposals/review-signin-2026-11-16/findings.md#v11-codexv2-states-its-device-login-now-with-an-explicit-refusal-v33-merged)).
G02's two verifiers split the same way
([G02](../proposals/review-signin-2026-11-16/findings.md#g02-one-reading-of-nam-024-for-a-login-added-later)),
so the record put the reading to the owner as
[Q5](../proposals/review-signin-2026-11-16/README.md#q5-may-a-later-v2-release-add-a-login-o6-m23-m33-m43),
with the skeptic's reading (C01, S04) as its option a, in the words of
[G02](../proposals/review-signin-2026-11-16/findings.md#g02-one-reading-of-nam-024-for-a-login-added-later)'s
"How the record takes it": "an addition that changes what an unchanged request may do (a further
login; a member added to an existing login) is stated at 0.2.0 or never in `.v2`; an addition an
unchanged request never exercises (a new operation with its implementation default) is an
optional thing". The owner chose option a (ruling 82, answer 4).

Ruling 82 point 4 holds that reading, and says "NAM-024's text gains words saying so"; that the
words go into NAM-024, and that their carrier is this named spec fix landed before 050 is written,
not 048a-1, is the coordinator's reading of the owner's answer, open to the owner's strike.
[IN_FORCE.md](../../IN_FORCE.md) holds it as spec pending (topic 5, line 101). This order writes
it: NAM-024's `then` and `observable` gain the words, the `login` row of `spec/definitions.md`
gains one sentence, and IN_FORCE.md's line becomes an **In spec/** line. Order 050 then states
`codex.v2`'s device login, where `codex.v2` states `signIn`, and `claude-code.v2`'s code-page
login on these words (ruling 82 points 4 and 5); that is 050's, not this order's.

Ruling 82 point 4 answers the review's parked line 5 ("NAM-024 could say in words which additions
count: a login, an offers entry") for logins and members of a login only. Its offers entry, whether
a capability added to a Type's `offers` in a later release is one of NAM-024's optional things,
stays parked for the owner ([the parked
list](../proposals/review-signin-2026-11-16/README.md#7-the-parked-list); the tracker's row under
"Waiting for the owner"). This order must not settle it: the new words name no `offers` entry and
classify only what ruling 82 point 4 names.

What does not change, and why:

- **No case and no vector.** The change adds no promise ID, so the rule that a new promise lands
  with a case that can fail ([CONTRIBUTING.md](../../CONTRIBUTING.md#writing-promises), rule 8)
  does not apply, and no existing case asserts anything the new words change: the eleven cases
  that cite NAM-024 under `conformance/language/packages/` compare defaults or add schema
  properties and operations, and none adds a login. So there is no declaration file and no entry
  in `conformance/expected-failures.yaml`.
- **The engine's install check.** NAM-024's `observable` already says that installing a release
  whose same-named contract breaks the rule reports invalid with a NAM-024 diagnostic; with these
  words a release that adds a login under the same name breaks it. The first build compares only
  defaults (`engine/crates/core/src/packages.rs`), which the review parked as line 4, B0-11
  (R1; ruling 46 point 4). Ruling 82 point 4 names that check as parked and needing no change for
  these words; it stays parked, with any case that would show it, and is not this order's.
- **`engine/STANDING.md`.** It is generated from each promise's ID, layer and level, the coverage
  index, the expected failures, the S1 rows and the engine's sources, none of which changes. As
  AGENTS.md asks for any change to a promise, run `python3 tools/promise_standing.py --write`
  anyway (decision 6); it must leave the file as it is.
- **The model-checking account.** NAM-024 is a language-layer promise and has no row in
  `experiments/012-model-checking/account.yaml`, which holds the convention layer's.
- **Other texts.** NAM-025 ("apart from the additions NAM-024 allows") reads the new words as they
  stand. CTR-013, PND-010 and PND-013 are cited, not changed. The safety catalogue's NAM-024 row
  quotes an earlier commit, as its header says, and the sentence it quotes stays word for word.

## The decisions it applies

Line numbers are those of `main` at `96ce049e`. Folded `>-` text keeps its four-space indent and
lines under 97 characters; rewrap only as far as a change needs. Everything a decision does not
name stays word for word ([CONTRIBUTING.md](../../CONTRIBUTING.md#writing-promises), "Writing
promises", rules 1 and 7).

1. **NAM-024's `then`**, `spec/promises/naming.yaml`, lines 420 to 425, becomes exactly this,
   wrapped as shown:

   ```yaml
     then: >-
       The later contract may add optional things, such as an optional input of an operation, a
       further operation of a capability or an optional property of a Type's `schema`. A further
       operation of a capability is one of them, and so is the default implementation of it that
       a Type whose earlier contract offers that capability adds to its `implementation`. A login
       added to a Type's `signIn.logins`, and a member added to a login that the earlier contract
       states, are not: a Type's logins, and the members of each, are stated in the first release
       at the address that exports the Type under its name, or in none. A property that it adds to
       a Type's `schema`, at any depth, and an input that it adds to a Requirement contract, state
       no `default`. It does not remove or change anything the earlier one held.
   ```

   Its first sentence, and its last two, are the present text unchanged.
   - *Coordinator's reading.* "The first release of the address", in ruling 82 point 4, is read
     as the first release at the address that exports the Type under that name: `.v2` first
     appears in 0.2.0, not in the address's first release. The record words the same rule as
     "stated at 0.2.0 or never in `.v2`" (o6; G02), which is this reading. "Or never" is read as
     never under that name: a Type with another version segment is another contract (NAM-015),
     and may state other logins. A Type that states no `signIn` in its first release and gains
     one later adds logins, so the words forbid that too; the review's V01 already holds that
     `signIn` enters `.v2` in 0.2.0 or never, since it adds hooks under `requires`.
   - *Coordinator's reading.* "Its implementation default", in ruling 82 point 4, is read as the
     entry for the new operation in the `implementation` of a Type that already offers the
     capability, since no Operation contract states a default implementation (CTR-002) and a
     Type's `implementation` holds its defaults (CTR-004); a Type that newly offers the
     capability is the parked offers entry, and the words say nothing of it.
2. **NAM-024's `observable`**, the same promise, lines 426 to 439. Its first two sentences
   ("Comparing the two contracts shows only such additions. Any other change is made as a new
   contract, under a name with another version segment, as NAM-015 states.") stay. After them,
   insert exactly these three sentences:

   > A request that was dispatched under the earlier release, made again with the same target
   > and inputs, names no further operation, since a request that names an operation that the
   > version the target holds does not include reaches no implementation, under NAM-026; so a
   > further operation changes nothing such a request may do. A login, or a member of a login,
   > added later would change what such a request may do: a `sign-in.v1` that names no login may
   > start any of its Type's logins, the hook choosing among them under PND-010, and a member such
   > as `issuer` adds a check of that login's carried return under PND-013. So a login, or a
   > member of one, that the first release exporting a Type under its name does not state is
   > stated only under a name with another version segment.

   Everything after them, from "An input of a requirement that states a `default`" to "and
   changed pointer.", stays word for word. Rewrap the `observable` from the inserted text to the
   end of the line on which the kept text resumes, and no further where the kept lines fit.
3. **The rest of NAM-024 stays**: its `id`, `layer`, `level`, `when` and `vectors: []`. No other
   promise in `spec/` changes. The words name no `offers` entry, no capability added to a Type,
   and no other kind of addition than those decisions 1 and 2 name; do not add an example, on
   either side, of anything else. The ruling's general test, an addition that changes what an
   unchanged request may do, is not written into `then` as a rule: as a rule it would classify
   every other kind of addition, an entry added to `offers` among them, which ruling 82 point 4
   leaves parked for the owner. The `observable` carries the test only as the reason for the two
   classes that `then` names.
4. **`spec/definitions.md`, the `login` row** (line 272). After its one sentence, which ends
   "pinned with the contract (CTR-013).", add, in the same cell: "A Type's logins, and the members
   of each, are stated in the first release of its package that exports the Type under its name,
   or in none: a later release adds neither under that name (NAM-024)." Keep every other byte of
   the file.
5. **IN_FORCE.md, topic 5, line 101.** Replace the whole line, the one that begins "- *Spec
   pending, a named spec fix landed before order 050 is written*: NAM-024 says in words which
   additions count.", with exactly this one line, so that no other line of the file moves:

   ```markdown
   - **In spec/**: NAM-024 says which additions count. A login added to a Type's `signIn.logins`, or a member added to a login, is stated in the first release of its package that exports the Type under its name or in none, and a further operation of a capability, with its default implementation, stays an optional thing; so a login that the Agent packages 0.2.0 do not state never joins `.v2`. NAM-024 does not say whether an entry added to a Type's `offers` is an optional thing: that stays parked for the owner (82 point 4; writing the answer into NAM-024, and its carrier, are the coordinator's reading of the owner's answer, open to the owner's strike).
   ```

6. **The standing table.** Once decisions 1 and 2 are done, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks for any change to a promise;
   never edit `engine/STANDING.md` by hand. It must leave the file unchanged. If it changes the
   file, keep the change, report every changed row, and say so as a finding.

## Scope

Change only `spec/promises/naming.yaml` (NAM-024's `then` and `observable`), `spec/definitions.md`
(the `login` row) and `IN_FORCE.md` (line 101). Run `tools/promise_standing.py --write`, which must
change nothing. Nothing under `conformance/`, `engine/` (beyond what that tool writes),
`packages/`, `tools/` or `experiments/`; no declaration file; and not `docs/AUDIT.md`, `ROADMAP.md`,
the handover, the review's record, the ruling's record or the safety catalogue. Report anything
else that seems to need changing.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_private_paths.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
git diff --check
```

Each of the `tools/` checks ends with a line that ends `Errors: 0`; `git diff --check` prints
nothing. `check_model_account.py` prints the same counts as on your starting tree.

From `engine/`, after the change to `spec/`, as AGENTS.md asks:

```bash
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

Every `test result:` line it prints reads `ok` with `0 failed`. Report the last one. The threads
are 4 because at 24 the runner ran out of memory (order 071's Record).

Each of these prints 1. They read the file with its line breaks as spaces, so a rewrap hides
nothing:

```bash
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'and so is the default implementation of it that a Type whose earlier contract offers that capability adds to its `implementation`\.'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'are stated in the first release at the address that exports the Type under its name, or in none\.'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'the hook choosing among them under PND-010'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'is stated only under a name with another version segment\. An input of a requirement'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'state no `default`\. It does not remove or change anything the earlier one held\.'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'signIn.logins`, and a member added to a login that the earlier contract states, are not: a Type'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'A further operation of a capability is one of them, and so is'
tr -s ' \n' '  ' < spec/promises/naming.yaml | grep -c 'and inputs, names no further operation, since a request that names an operation'
grep -cF "(CTR-013). A Type's logins, and the members of each, are stated in the first release of its package that exports the Type under its name, or in none: a later release adds neither under that name (NAM-024). |" spec/definitions.md
grep -cxF -f <(awk '/^   ```markdown$/{f=1;next} /^   ```$/{f=0} f' docs/work-orders/199-nam-024-which-additions-count.md | sed 's/^   //') IN_FORCE.md   # decision 5's line, whole
```

This prints `True True True`. It reads NAM-024 with its line breaks as single spaces and finds
that its `then` is exactly decision 1's text, that its `observable` begins with its two kept
sentences, decision 2's text and the kept "An input of a requirement", and that the sentence the
safety catalogue's NAM-024 row quotes stands whole in `then`:

```bash
/usr/bin/python3 -c '
import yaml
p = {x["id"]: x for x in yaml.safe_load(open("spec/promises/naming.yaml", encoding="utf-8"))}["NAM-024"]
then = " ".join(p["then"].split()); obs = " ".join(p["observable"].split())
d1 = ("The later contract may add optional things, such as an optional input of an "
      "operation, a further operation of a capability or an optional property of a Type\x27s "
      "`schema`. A further operation of a capability is one of them, and so is the default "
      "implementation of it that a Type whose earlier contract offers that capability adds "
      "to its `implementation`. A login added to a Type\x27s `signIn.logins`, and a member "
      "added to a login that the earlier contract states, are not: a Type\x27s logins, and the "
      "members of each, are stated in the first release at the address that exports the "
      "Type under its name, or in none. A property that it adds to a Type\x27s `schema`, at "
      "any depth, and an input that it adds to a Requirement contract, state no `default`. "
      "It does not remove or change anything the earlier one held.")
d2 = ("A request that was dispatched under the earlier release, made again with the same "
      "target and inputs, names no further operation, since a request that names an "
      "operation that the version the target holds does not include reaches no "
      "implementation, under NAM-026; so a further operation changes nothing such a request "
      "may do. A login, or a member of a login, added later would change what such a "
      "request may do: a `sign-in.v1` that names no login may start any of its Type\x27s "
      "logins, the hook choosing among them under PND-010, and a member such as `issuer` "
      "adds a check of that login\x27s carried return under PND-013. So a login, or a member "
      "of one, that the first release exporting a Type under its name does not state is "
      "stated only under a name with another version segment.")
kept = ("Comparing the two contracts shows only such additions. Any other change is made as a "
        "new contract, under a name with another version segment, as NAM-015 states. ")
q = ("A property that it adds to a Type\x27s `schema`, at any depth, and an input that it "
     "adds to a Requirement contract, state no `default`")
print(then == d1, obs.startswith(kept + d2 + " An input of a requirement"), q in then)
'
```

Each of these prints 0:

```bash
awk '/^- id: NAM-024$/,/^  vectors:/' spec/promises/naming.yaml | grep -c '`offers`'
grep -c 'Spec pending, a named spec fix landed before order 050 is written' IN_FORCE.md
git diff -U0 spec/promises/naming.yaml | grep -E '^[^-+ ]{2} -[0-9]' | grep -vcE '^[^-+ ]{2} -4[23][0-9][, ]'
```

The first of them finds no `offers` entry named in NAM-024; the verb "offers", in decision 1's
"whose earlier contract offers that capability", is not one. The last reads each hunk header of
the change and finds that every hunk starts within NAM-024, lines 420 to 439.

`git diff --numstat IN_FORCE.md` prints `1`, `1` and `IN_FORCE.md`, separated by tabs: one line
replaced, none moved. `git diff --numstat spec/definitions.md` prints `1`, `1` and
`spec/definitions.md`, separated by tabs.

`git status --short` lists exactly `IN_FORCE.md`, `spec/definitions.md` and
`spec/promises/naming.yaml`, each modified, and `engine/STANDING.md` is not among them. Do not
stage anything. Report each choice you made yourself and every check's last line.

Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
([ruling 79](../proposals/ruling-79/README.md), point 2), the reader against ruling 82 point 4
above all: that the words change NAM-024's meaning only as that point says, and settle nothing on
an `offers` entry. It merges on a pass of `tools/gate.py`; CI runs on the pull request as a second
opinion that the merge does not wait for ([ruling 80](../proposals/ruling-80/README.md)). It merges
before order 050 is written.
