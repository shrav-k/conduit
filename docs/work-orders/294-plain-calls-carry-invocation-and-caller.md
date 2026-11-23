# Work order 294: a plain item may list operations whose calls carry their invocation and caller, so an Agent's wrapper can keep each turn for `history.v1` (DEC-067, the spec)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the three cases this order adds, each an expected failure. It needs no network and changes
no Rust source; if a crate is missing from the cache, fetch nothing and report it. Run one cargo
command at a time, and not `tools/gate.py`. Never print a whole large file, case, diff or list:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's answer** ([DEC-067](../decisions/log.md#dec-067-2026-11-22-an-agent-speaks-to-the-owner-only-in-replies-made-readable)):
"Replies only, made readable (Recommended)": an Agent speaks only in the reply to a turn someone
asked for, and both Agent packages offer `history.v1`, each turn's message, final text,
invocation, caller and times, with no effect and no model work.
[DEC-079](../decisions/log.md#dec-079-2026-11-22-access-requests-health-and-history-now) builds it
now, in v0, and keeps tool calls, tool results, file contents and usage limits out of it.

**Why a spec order first** (read on `main` at `b832ec4a`). `history.v1` needs no contract in
`spec/`: no engine reads it, it is each package's own Operation, as `turn.v1` is, and stating no
`effect` and no model work makes it read-only (IFC-033). But both packages' turn items are plain
(ruling 86, point 1), so a turn's call carries neither its invocation nor its caller (IFC-007),
and `keyed` lists hooks alone (PKG-020). The engine's records keep no message or text (CTR-004,
REC-002), and the wrapper, which holds the text, cannot name the invocation or the caller. So
this order and an engine order come before the package orders that DEC-067's status names. The
owner may strike both: a history without invocation and caller needs neither.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
everything an agent does through Conduit is attributable: a plain server learns which invocation
it serves and for whom; 4, any plain item may opt in, so nothing assumes a vendor's agent; 6, the
call starts nothing more. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a gateway tells a server not written for it which request it serves and for
whom, and the server may ignore it, as Envoy's `x-request-id` and oauth2-proxy's
`X-Forwarded-User` do. Not taken: the two keys on every plain call, with no new key, since a plain
server not written for Conduit would learn the caller without its package asking; keying the turn,
which would leave a failed turn's reply only the engine's block (IFC-025), lose the `failedTurn`
fields that DEC-046's pause reads and change how a lapsed login is reported (PND-005); the
engine's records; a core `history.v1` contract; the Conduit, configuration or idempotency key on
the call, which DEC-067 does not name.

**When it runs.** On `main` from `b832ec4a` (#565). Through Conduit: one Codex worker turn, the
blind Codex verifier and Claude reader at once, one fix turn, the gate (DEC-046, DEC-071,
DEC-033). By ruling 89, each decision says what must hold; how is yours. The member's name, the
words and the cases are the coordinator's, open to the owner's strike (DEC-029). Shared files:
`spec/CHANGES.md`, `spec/README.md`, `spec/definitions.md`, `spec/interface/implementation.yaml`,
the loader's `invalid-verdicts.txt`, the requests `index.yaml`, `conformance/expected-failures.yaml`
and `engine/STANDING.md` (orders 284 and 293), and the kit's `src/schemas.json` (order 291):
whichever order merges later rebases and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step that needs a change outside Scope (give its failing line), or a file outside
Scope that must change. Where this order's wording contradicts a promise it does not change, the
promise governs: apply it and list it under `own_choices`, beside every choice no decision
governs.

## The decisions

Wrap each insertion at its file's width.

1. **PKG-020,** in `spec/promises/packages.yaml`. In its `then`, "and its other calls to the
   implementation do not." becomes "and its other calls carry none of them, except as
   `attributed` gives.", followed by:

   ```text
   It may also state `attributed`, a list of operations, each a selection in its `implements` and a key of its `tools` that names none of those five hooks: the engine's calls for an operation that `attributed` lists carry `io.github.shrav-k.conduit/invocation` and `io.github.shrav-k.conduit/caller`, as IFC-006 gives them, and no other key of Conduit's, and are plain in every other way.
   ```

   Its `observable`: after "or names an operation other than those five hooks.", the sentence
   "So are an item that states `attributed` and is not marked `plain` or is reached by `remote`,
   and a selection in `attributed` that is absent from `implements` or from the keys of `tools`,
   or that names one of those hooks."; "A plain server is sent none of Conduit's keys except on a
   call for a hook that `keyed` lists," gains "and its invocation and caller alone on a call that
   `attributed` lists," before "so it learns"; "and on such a call through IFC-006's keys"
   becomes "and on a keyed call through IFC-006's keys"; and, after "Every other call to it is
   plain, under IFC-007 and IFC-010.", a sentence saying a call that `attributed` lists declares
   no extension and needs no `server/discover`, its result, `content` and repeat are a plain
   call's (IFC-010, IFC-025, OPS-012), and each attempt of one invocation carries the same ID
   (IFC-006), so a wrapper that lists its turn can tell its turns apart and say who asked
   (DEC-067).
2. **IFC-007 and IFC-037,** in `spec/interface/implementation.yaml`. IFC-007's `then` becomes:

   ```text
   The call carries one of Conduit's keys, other than `io.github.shrav-k.conduit/invocation` and `io.github.shrav-k.conduit/caller` where its item's `attributed` lists the operation (PKG-020).
   ```

   In its `observable`, "such as `io.modelcontextprotocol/protocolVersion`." gains, before its
   full stop, ", and `io.github.shrav-k.conduit/invocation` and `io.github.shrav-k.conduit/caller`
   where `attributed` lists the operation". In IFC-037's `observable`, "is sent none of Conduit's
   keys" gains ", but its invocation and caller where its item's `attributed` lists the
   operation,".
3. **MAN-009 and MAN-010,** in `spec/promises/manifest.yaml`. In MAN-009's `then`, "`plain`,
   `keyed`, `tools`" becomes "`plain`, `keyed`, `attributed`, `tools`", and in its `observable`
   "`plain`, `keyed` and `tools`" becomes "`plain`, `keyed`, `attributed` and `tools`". In
   MAN-010's `then`, "`keyed` is the list of PKG-020." becomes "`keyed` and `attributed` are the
   lists of PKG-020."
4. **The schema,** `implementation-item.v1` in `spec/schemas/manifest.yaml`: `attributed` after
   `keyed`, in `keyed`'s form, under `properties` and `dependentSchemas`, and `attributed: false`
   after `keyed: false`. Its description names `attributed` beside `keyed`: what it lists, that
   PKG-020's rules for it are not in the schema, and that an item that is not plain gives none.
   Add `spec/schemas/examples/invalid/PKG-020-manifest-attributed-not-plain.yaml`, in the form of
   `PKG-020-manifest-keyed-not-plain.yaml`, an item not marked plain that lists `'#turn.v1'` under
   `attributed`, and its line `PKG-020-manifest-attributed-not-plain.yaml schema`, in name order,
   in the loader's `engine/crates/loader/tests/expected/invalid-verdicts.txt`. The
   `examples/invalid` row of `spec/README.md` gives the number of files there (77 with this one,
   more if order 293 merges first). The kit's `packages/runtime-provider-kit/src/schemas.json`
   copies `implementation-item.v1`: it gains `attributed` where it holds `keyed`, by the drift
   test's own conversion, as order 274's Record did, and nothing else. In
   `spec/schemas/README.md`, the row on `keyed`'s selections covers `attributed`'s rules too, and
   "holds no `tools` and no `keyed`, and one reached by `remote` no `keyed`" becomes "holds no
   `tools`, no `keyed` and no `attributed`, and one reached by `remote` neither list".
5. **`spec/definitions.md`.** The row "plain, `keyed`" gains, before its full stop, ", and its
   invocation and caller alone on a call for an operation that its item lists under
   `attributed`". In the row `plain`, "under `keyed` carries Conduit's keys;" gains, before its
   semicolon, ", and one for an operation that it lists under `attributed` its invocation and
   caller alone". In the row "implementation item", "`keyed` (PKG-019, PKG-020)" becomes
   "`keyed` and `attributed` (PKG-019, PKG-020)". After the row `` `keyed` ``, a row:

   ```text
   | `attributed` | The key of an implementation item that is marked `plain: true` and reached by `command` or `package`, which lists operations, none of them a hook that `keyed` may list, whose calls carry the invocation and the caller and no other key of Conduit's, and are plain in every other way (PKG-020, IFC-007, IFC-010, IFC-025, OPS-012). An Agent's wrapper lists its turn, so it can keep each turn with the invocation and the caller that asked (DEC-067). |
   ```

6. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item:

   ```text
   - Changed: PKG-020, a plain item reached by `command` or `package` may list in `attributed` the operations whose calls carry their invocation and caller, plain in every other way (DEC-067).
   - Changed: IFC-007 and IFC-037, a plain call carries no key of Conduit's but those two, where its item's `attributed` lists the operation (DEC-067).
   - Changed: MAN-009 and MAN-010 admit `attributed`, the list of PKG-020 (DEC-067).
   - Added: `attributed` in `implementation-item.v1`, only on a plain item reached by `command` or `package` (DEC-067).
   ```

7. **Three cases** in `conformance/conventions/requests/`, each in its `index.yaml`, profile
   `first-build`, from `{"recipe":"empty"}`:
   - `requests-attributed-plain-call` (`attributed-plain-call/`; PKG-020, IFC-007, IFC-025), on
     the documents and fixture of `explain-failed-turn-content`, with a manifest whose item is
     reached by `command` as `requests-keyed-hook-dispatch`'s is and lists `#turn.v1` under
     `attributed`. Two turns, one succeeding and one failing: each dispatch's `_meta` holds the
     protocol's keys, that turn's own `invocation` and the caller's ID, and no other key of
     Conduit's (no `operation`, `instance`, `config`, `filters`, `revision`, `conduit` or
     `idempotencyKey`, and no extension declared), asserted as `requests-keyed-hook-dispatch`
     asserts a plain call; each reply is as it would be without the list (IFC-010, IFC-025).
   - `requests-attributed-not-in-implements` and `requests-attributed-a-hook` (PKG-020), in the
     form of `requests-keyed-not-in-implements` and `requests-keyed-not-a-hook`: a `register.v1`
     request reports invalid, as in those cases, for a selection that is a key of `tools` and
     absent from `implements`, and for a core hook in both. By the owner's rule (ruling 52,
     DEC-010), its diagnostic's `ruleId` is PKG-020, its `message.text` names the item and the
     selection and says what to do (list it in `implements` and `tools`; a hook belongs in
     `keyed`), and its `locations` give the selection's pointer. Each case asserts all three, in
     `fleet-register-name-clash`'s form.

   Run each on the unchanged engine and report its first failing step. List all three in
   `conformance/expected-failures.yaml`, in `id` order, each with the reason "<its first promise>:
   the first build does not read `attributed`; DEC-067's engine order makes it pass and removes
   this entry". `required_promises`, where the runner allows it, names only what the unchanged
   engine cites. Declare them in `docs/work-orders/294-declared.yaml`, in
   [271's](271-declared.yaml) form, then run `python3 tools/promise_standing.py --write`.

## What to read

The promises and cases the decisions name; IFC-001 and IFC-006; the kit README's drift test;
CONTRIBUTING rule 8; "Expected failures" in `conformance/README.md`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor b832ec4a HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
sed -n '/^## DEC-067/,/^## DEC-068/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
git grep -l -e '`attributed`' -e '"attributed"' -e 'attributed:' -- spec conformance engine/crates packages | wc -l | tr -d ' '
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/promises/packages.yaml "and its other calls to the implementation do not." "and on such a call through IFC-006's keys."
k spec/interface/implementation.yaml "The call carries Conduit's keys." "is sent none of Conduit's keys, and the engine"
k spec/promises/manifest.yaml '`plain`, `keyed`, `tools`,' '`keyed` is the list of PKG-020.'
k spec/schemas/manifest.yaml "may list in \`keyed\` the hooks whose calls carry Conduit's keys;"
k spec/definitions.md "except on a call for a hook that its item lists under \`keyed\`." "under \`keyed\` carries Conduit's keys;"
echo "$(grep -c '^    keyed:$' spec/schemas/manifest.yaml) $(grep -c '"id": ' conformance/expected-failures.yaml) $(grep -c '"keyed"' packages/runtime-provider-kit/src/schemas.json)"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`, `1`, `0`, `1 1` three times, `1`, `1 1`, `2 3 3` and `engine 0`;
report any other line. Where another order moved the count of expected failures, report it.

## Scope

Change, as the decisions say, `spec/promises/packages.yaml` (PKG-020),
`spec/interface/implementation.yaml` (IFC-007, IFC-037), `spec/promises/manifest.yaml`
(MAN-009, MAN-010), `spec/schemas/manifest.yaml`, `spec/schemas/README.md`,
`spec/definitions.md`, `spec/README.md` (one count), `spec/CHANGES.md`, the loader's
`invalid-verdicts.txt` (one line) and the kit's `src/schemas.json`; create the schema example, the three case directories and
`docs/work-orders/294-declared.yaml`; add three entries each to the requests `index.yaml` and
`conformance/expected-failures.yaml`; `engine/STANDING.md` only by the tool. Nothing else: not
another promise, the log (DEC-067 stays not done), `IN_FORCE.md`, the engine's or the kit's
other files, `tools/` or `experiments/`. Where a core test or vector fails because the engine
reads the changed schema, report it and change no engine file.

## Done when

Each line prints otherwise on `main` today and passes after:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/promises/packages.yaml 'It may also state `attributed`, a list of operations,' 'and a selection in `attributed` that is absent from' "and on a keyed call through IFC-006's keys"  # 1 1 1; 0 0 0 today
k spec/interface/implementation.yaml "where its item's \`attributed\` lists the operation (PKG-020)." "but its invocation and caller where its item's \`attributed\`"  # 1 1; 0 0 today
k spec/promises/manifest.yaml '`keyed`, `attributed`, `tools`,' '`keyed`, `attributed` and `tools`' '`keyed` and `attributed` are the lists of PKG-020.'  # 1 1 1; 0 0 0 today
echo "$(grep -c '^    attributed:$' spec/schemas/manifest.yaml) $(grep -c '^      attributed: false$' spec/schemas/manifest.yaml) $(grep -c '"attributed"' packages/runtime-provider-kit/src/schemas.json)"  # 2 1 3; 0 0 0 today
e=PKG-020-manifest-attributed-not-plain.yaml; echo "$(ls spec/schemas/examples/invalid | grep -cFx "$e") $(grep -cFx "$e schema" engine/crates/loader/tests/expected/invalid-verdicts.txt)"  # 1 1; 0 0 today
k spec/schemas/README.md 'no `keyed` and no `attributed`, and one reached by `remote` neither list'  # 1; 0 today
k spec/definitions.md '`keyed` and `attributed` (PKG-019, PKG-020)' 'that it lists under `attributed` its invocation and caller alone'; grep -c '^| `attributed` |' spec/definitions.md  # 1 1 then 1; 0 0 then 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-067)\.$'  # 4; 0 today
grep -cE '"id": "requests-attributed-(plain-call|not-in-implements|a-hook)"' conformance/expected-failures.yaml  # 3; 0 today
for c in requests-attributed-plain-call requests-attributed-not-in-implements requests-attributed-a-hook; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$c" 2>&1 | grep -c "^EXPECTED FAILURE $c "); done  # 1 three times; 0 today
```

Guards, which pass today and must still pass. Run the core's tests and then the whole vectors
test once, last; never the whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
r=conformance/conventions/requests; n="$r/attributed-plain-call $r/attributed-not-in-implements $r/attributed-a-hook spec/schemas/examples/invalid/PKG-020-manifest-attributed-not-plain.yaml docs/work-orders/294-declared.yaml"
git add -N $n
for p in requests-keyed- explain-failed-turn-content packages-plain-missing-tool; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # each passes, as today
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # head equals base; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/294-declared.yaml; echo "exit $?"  # ends 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(promises/(packages|manifest)\.yaml|interface/implementation\.yaml|schemas/(manifest\.yaml|README\.md|examples/invalid/PKG-020-manifest-attributed-not-plain\.yaml)|definitions\.md|README\.md|CHANGES\.md)|engine/(STANDING\.md|crates/loader/tests/expected/invalid-verdicts\.txt)|packages/runtime-provider-kit/src/schemas\.json|conformance/(expected-failures\.yaml|conventions/requests/(index\.yaml|attributed-[a-z-]+/[^/]+))|docs/work-orders/294-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # the verdict fixture
(cd packages/runtime-provider-kit && cargo test --offline --release --test schema_drift)  # the kit's copy
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case's first failing step on the unchanged engine, and every
check's last line. Stage nothing.

## What follows

DEC-067's other orders, in sequence: the engine order (S), which sends the two keys and applies
PKG-020's new rules with decision 7's diagnostics, removing the three expected failures; 295 and
296, a release of each Agent package whose turn is listed under `attributed` and which offers a
read-only `history.v1` that holds nothing DEC-079 excludes; and the role-template update.
DEC-067's last clause already holds (PND-005).

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (15:39 to 15:56) wrote the spec for DEC-067's ground: a plain item may list
operations as `attributed`, whose calls carry their invocation and caller, so an Agent's wrapper
can keep each turn for `history.v1`, which holds none of what DEC-079 excludes. MAN-009, MAN-010,
PKG-020, IFC-004, IFC-037 and the glossary changed; the kit's schema copy follows. The three new
cases fail on the unchanged engine as expected failures; the loader examples, the kit's drift
test, 189 core tests and the whole vectors test pass.

**Two blind reads.** The Claude reader (15:57 to 16:01) had no must and three shoulds. The Codex
verifier's first read failed at once on a session the coordinator's earlier stop had left
unable to restart; after a serve restart, `turn-verifier-2.turn.v1` read it blind (16:15 to
16:19): four musts, on the positive vector's turn contract and its pinned protocol revision,
IFC-004's two revision rules for an attributed call, and IFC-047's observable. One fix turn
(16:19 to 16:30) applied all twelve findings, each must shown first. The gate's run on the
squash is the coordinator's, at the merge.
