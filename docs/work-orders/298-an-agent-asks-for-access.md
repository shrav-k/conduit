# Work order 298: an Agent asks root for access (DEC-064, the spec's first part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the four cases this order adds, each an expected failure. It needs no network; if a crate
is missing from the cache, fetch nothing and report it. Run one cargo command at a time, and not
`tools/gate.py`. Print no whole large file, case, diff or list: use a count or `sed -n`. Never
run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere. Prefix each `python3` with
`PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

## What and why

**The owner's answers.** [DEC-064](../decisions/log.md#dec-064-2026-11-22-an-agent-may-ask-for-access):
an Agent may ask for an operation it cannot call, raising one pending action that permits
nothing; an ask for `get-secret.v1` or an operation reserved to root is refused and raises
nothing. DEC-079: built now. DEC-085: an ask for `install.v1`, or naming `management` or
`access`, is refused too. DEC-084: no reason input.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2,
an ask grants nothing; 5, an Agent asks within its asking Conduit's bound, no right to open; 6,
raising permits and resumes nothing; 7, each ask is the Agent's own request. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Teleport Access Requests, where a role one may not request is
refused at once; Kubernetes's CertificateSigningRequest. Not taken (the coordinator's design
notes): a tool on every Agent with no Conduit (IFC-012); the ask on the Agent's Fleet, which
no bound scopes; asking by default.

**When it runs.** On `main` from `d7e63ebd` (#573). Through Conduit: one Codex worker turn, the
blind Codex verifier and Claude reader at once, one fix turn, the gate (DEC-046, DEC-071,
DEC-033). By ruling 89, how is yours; the refusal text is exact. The coordinator's, open to the
owner's strike (DEC-029): Agents alone asking, on `access`; `succeeded` for an ask raising
nothing; withdrawing; a decline holding to expiry; asks keyed on names; ACC-001's sentence;
names, words, cases. Orders 293, 294, 295 and 297 share files: the later merge rebases,
recounts and reruns `promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must
change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, beside every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. `core#x` is `github.com/shrav-k/conduit/core#x`; "A" →
"B" replaces A with B.

1. **The contracts,** after `lift.v1` in `spec/core/operations.yaml`, in the file's form, each
   `deduplicates: true`. `request-access.v1`: inputs `operation` (string, minLength 1,
   required), `target` (`core#target-reference.v1`), `conduit` (string, minLength 1); outputs
   `state` (`raised`, `unanswered`, `declined` or `held`; required), `action` (`core#id.v1`),
   `expiry` (date-time), `tools` (strings), `diagnostics` (`core#diagnostic.v1`; required);
   effect "Raises one access-request pending action, which permits nothing." Its description:
   who may ask (ACC-058); exactly one of `target`, the target a Conduit listing the operation
   would have (ACC-033's first fitting row's), and `conduit`, a local name of the caller's whose
   target it takes; each state.
   `withdraw-access-request.v1`: input `action` (`core#id.v1`, required); effect "Answers the
   caller's own access-request action withdrawn." `open.v1`: "A Conduit to `access` that
   applies no bound is unlimited" → "A right to open that applies no bound is unlimited"; it
   adds "A Conduit that lists either ask of ACC-058 is opened only as an Agent's asking
   Conduit (ACC-058)."
2. **Where it is offered.** `access-control.v1` lists both and `access.v1` requires both; their
   descriptions, and the capability's header line, say an Agent asks and withdraws through them
   and a Conduit to `access` listing no other of its operations is an asking Conduit, no right
   to open (ACC-058). `spec/core/conduit.yaml` exports both and `access-request`; `engine`
   implements both. After `fleet-manage-own`, the Conduit template `access-request` of
   `core#access-control.v1` lists exactly the two, with no bound and a header line.
   `root-access`: "lists every operation of `access-control.v1`" → "lists `open.v1`, `close.v1`,
   `reconcile.v1` and `lift.v1`, not the two asks of ACC-058,". `bound.v1`'s description adds
   "On an asking Conduit it bounds the asks (ACC-058)."; the header's "has no bound and is
   unlimited" → "has no bound and, as a right to open, is unlimited". For MAN-012's shell
   test, `UNFINISHED_ENGINE_OPERATIONS` in `engine/crates/shell/src/server.rs` gains
   both with `"DEC-064"`, and its check in `tests.rs` accepts `DEC-` and three digits too.
3. **ACC-058,** at the end of `spec/conventions/access.yaml`, convention, must. `when`: an actor
   asks for either operation, or to open a Conduit listing either. It says:
   - Either is dispatched only for an Agent on its own behalf, through its own asking Conduit,
     one to `access` listing no other of its operations; no other Conduit listing either is
     opened (refused under ACC-058).
   - An ask for an operation of the table, or with target `management` or `access` (by
     `target` or `conduit`), is refused under ACC-058 and raises nothing, decided on the
     operation's qualified reference in the core package alone, as PRV-004 matches, and on the
     target as named. One with both or neither of `target` and `conduit`, or a `conduit` not
     the caller's, reports invalid under ACC-058.
   - Where the asking Conduit applies `bound.v1`, an ask outside it, as ACC-029 states for an
     open, is refused under ACC-029.
   - Otherwise the ask succeeds and raises one pending action of the kind `access-request`,
     which permits nothing (PND-001), unless the caller is the consumer of an open Conduit to the
     target listing the operation (`held`), or an action for the same ask (Agent, operation and
     target as named) is unanswered (`unanswered`) or declined and not expired (`declined`).
     Apart from the bound, a result depends only on those names and the caller's own Conduits
     and asks, so a missing, a hidden and a replaced target give the same result. Its one
     `info` diagnostic, ruleId ACC-058, says what happened and what to do.
   - `withdraw-access-request.v1` answers its action `withdrawn` only for the Agent that raised
     it; another ID is refused under ACC-058 before invalid.
   - An asking Conduit is no right to open, close or lift, whatever its bound (ACC-006, ACC-012,
     ACC-053); only an actor whose own Conduit to `access` has no bound opens one with none.
   - ACC-001's cause, for an Agent holding the ask's tool, adds "You may ask root for access
     with TOOL, naming the operation by its qualified reference and its target."
   - The refusal's message is exact (IFC-050 leaves wording to the engine): "OPERATION
     cannot be asked for (ACC-058): WHY Nothing was raised. TODO", OPERATION as sent; its first
     text block, `refused: ` and that message. For a refused target: "an ask cannot name TARGET
     as its target (ACC-058): access to it registers any program or opens Conduits, which root
     gives only in its own documents. Nothing was raised. N". `observable` holds the table
     written out, with L(X) "No Conduit whose consumer is an Agent may list it (X).", N "If your
     task needs it, say so in your reply.", Y "say so in your reply", I "only an Interface
     acting on its own behalf", G "If a login has lapsed, say so in your reply; root signs in.",
     P "If your task needs a package, say so in your reply.", E "the engine alone calls it, by
     a provider call", D "the engine alone dispatches it, as a part of a" and U "If your task
     needs that, ask for set-up.v1 or retire.v1 with the instance's owning Pool or Fleet as the
     target.":

     | Operations | WHY | TODO |
     | --- | --- | --- |
     | declare-lost, purge, enroll-machine, admit-machine, refuse-machine | it is reserved to root acting on its own behalf. L(ACC-032) | Only root does this. N |
     | answer | I answers a pending action. L(PND-009) | An Interface answers pending actions. If your task needs one answered, Y. |
     | sign-in | I signs in an instance. L(PND-010) | G |
     | run, run.v2, stop, inspect-run | E (PRV-005). L(PRV-004) | It happens within set-up.v1 or retire.v1 of the instance whose run it is. U |
     | list-machines | the engine alone calls it, of its own accord (PRV-005). L(PRV-004) | There is nothing to ask for. |
     | place | E, while it decides a registration (PRV-005). L(PRV-004) | If your task needs something registered, Y. |
     | fetch-package | E, while it decides an installation (PRV-005). L(PRV-004) | P |
     | setup-hook, retire-hook | D setup or a retirement, through no Conduit (TPL-017, TPL-018). | U |
     | sign-in-hook, account-hook, sign-out-hook | D sign-in or a setup, through no Conduit (PND-011). | G |
     | enroll, accept-machine, remove-machine | E within root's enrollment (PRV-005). | Only root enrolls machines. N |
     | get-secret | no instance's request for it is ever dispatched (SEC-007); the engine alone reads a secret, into a run, through a Conduit only root opens (SEC-008). | If your task needs a secret, Y. |
     | install | root gives it only in its own documents. | P |

     Names are the core's, `.v1` unless versioned.

4. **The rest of `spec/conventions/access.yaml`.** ACC-022: "ACC-032, PRV-004" → "ACC-032,
   ACC-058, PRV-004"; "A Conduit to `access` that has no bound is unlimited" → "A right to open
   that has no bound is unlimited". ACC-029: "has no bound and is unlimited:" → "has no bound
   and, as a right to open, is unlimited:", that sentence followed by "On an asking Conduit
   (ACC-058) a bound says what its consumer may ask for, and no bound lets it ask for anything
   ACC-058 does not refuse." ACC-030: "is one that the actor's own Conduit to `access` lists"
   gains ", the two of ACC-058 apart". ACC-033: "lift a tombstone;" gains " ask for access or
   withdraw an ask;"; row 10 gains ", or asking for access or withdrawing an ask" before its
   colon, and ACC-058. ACC-056: "ACC-057, CTR-013" → "ACC-057, ACC-058, CTR-013"; "and ACC-032,
   PND-009" → "and ACC-032, ACC-058, PND-009"; "covers every target (ACC-029)." → "covers every
   target (ACC-029), but an asking Conduit's missing bound covers none (ACC-058)."; after
   ACC-057's row, "| ACC-058 | Only an Agent asks or withdraws its own ask, through its own
   asking Conduit; nothing is asked for that no Agent's Conduit may list, get-secret.v1 or
   install.v1, nor on `management` or `access`; nothing was raised. |". The other rows stay.
5. **Elsewhere.** SCT-008's `when`: "core `bound.v1` requirement." → "core `bound.v1`
   requirement, unless it is an inline item listing only `request-access.v1`,
   `withdraw-access-request.v1` or both (ACC-058)." `spec/core/README.md`: both in the verbs
   table after `lift.v1` (`access-control.v1`, deduplicates, ACC-058) and in "Every export",
   with `access-request`; it and `spec/README.md` recount, promise totals too.
   `spec/definitions.md`: after `right to open`, a row `asking Conduit` (an Agent's Conduit to
   `access` listing one or both of the two and no other of its operations; its bound says what
   may be asked; no right to open; ACC-058); the `core package` rows name its template, the
   `access` row adds the asks, and the `bound` row says "as a right to open" as ACC-029 does.
   Done when gives the words.
   `experiments/012-model-checking/account.yaml`: after ACC-056's row, `- id: ACC-058`, `class:
   uncatalogued`. `spec/CHANGES.md`, after `## [Unreleased]`'s last item:

   ```text
   - Added: `request-access.v1`, `withdraw-access-request.v1` and the template `access-request`, by which an Agent asks root for access (DEC-064, DEC-079, DEC-084).
   - Added: ACC-058, an Agent asks through its own Conduit to `access`; an ask for 23 operations, `install.v1`, `management` or `access` is refused with a fixed message, raising nothing (DEC-064, DEC-079, DEC-085).
   - Changed: ACC-022, ACC-029, ACC-030, ACC-033, ACC-056 and SCT-008, an asking Conduit is no right to open and needs no bound inline (DEC-064, DEC-079).
   ```

6. **Four cases,** `first-build`, from `{"recipe":"empty"}`, the first as
   `sectioned-consumes-bound`, the others on `explain-agent-results`'s setup, written first and
   run on the unchanged engine, each first failing step reported; a refusal asserts `ruleId`,
   tier 1 and any message decision 3 fixes, with its text block.
   - `sectioned-consumes-asking` (SCT-008, ACC-058): an Agent's inline item `ask` to `access`
     listing only the two, no bound, translates; with `core#open.v1` too it is rejected.
   - `access-ask-offered` (ACC-058, ACC-030, IFC-012): root opens `ask` from
     `core#access-request` for `agent-a`, whose `ask.` tools are then the two alone; root's opens
     of one consumed by root, and of one listing `core#open.v1` too, are refused under ACC-058.
   - `access-ask-refused` (ACC-058, ACC-056): agent-a asks for one operation per table row, for
     `inspect.v1` on `management`, and with `conduit: ask`: each refused, raising nothing.
   - `access-ask-states` (ACC-058, ACC-029): with `conduit: project`, agent-a asks for
     `file.read.v1` (`held`) and `file.write.v1` (`raised`, with `pendingAction`), that again
     (`unanswered`, same action); on a missing Resource twice (`raised`, `unanswered`); for
     `example.org/other#purge.v1` (`raised`); with `target` and `conduit` (invalid, ACC-058).
     agent-b, whose asking Conduit's `bound.v1` names one target and operation, gets `raised`
     within it, ACC-029's refusal outside and ACC-058's on withdrawing agent-a's action. Root
     lists one `access-request` action per `raised`.

   Index them; list them in `conformance/expected-failures.yaml` by `id`, the reason "<its first
   promise>: the first build does not offer request-access.v1; DEC-064's engine orders make it
   pass"; declare them in `298-declared.yaml` as [288's](288-declared.yaml); run
   `promise_standing.py --write`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor d7e63ebd HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
git grep -l -e 'request-access\.v1' -e 'withdraw-access' -e 'ACC-058' -- spec conformance engine/crates | wc -l | tr -d ' '
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/conventions/access.yaml 'ACC-032, PRV-004' '(10) Opening or closing a Conduit, or lifting' 'ACC-057, CTR-013'
k spec/README.md 'Draft; 155 documents' 'Draft; 35 promises; vectors' '521 in all'
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`, `0`, then `1`s and `engine 0`. Where a line prints otherwise, its
text has moved: work from where it stands, and list it under `own_choices`.

## Scope

As the decisions say: seven files of `spec/core/`, `spec/conventions/access.yaml`,
`spec/promises/sections.yaml`, `spec/README.md`, `definitions.md` and `CHANGES.md`, one account
row, decision 2's shell lines; the cases, their index and expected-failure entries and
`docs/work-orders/298-declared.yaml`; `engine/STANDING.md` by the tool. Nothing else, and no
other promise. Where a test or vector fails because the engine reads the changed spec, report it.

## Done when

Each line prints otherwise on `main` today and passes after, with `j` and `k` as above:

```sh
p='(request-access|withdraw-access-request)\.v1$'; s=spec/core
echo "$(grep -cE "^  name: $p" $s/operations.yaml) $(grep -cE "core#$p" $s/capabilities.yaml) $(grep -cE "core#$p" $s/types.yaml) $(grep -cE "^  - ($p|access-request$)" $s/conduit.yaml) $(grep -cE "core#$p" $s/conduit.yaml) $(grep -c '^  name: access-request$' $s/templates.yaml)"  # 2 2 2 3 2 1; 0s today
k $s/operations.yaml 'A right to open that applies' 'either ask of ACC-058 is opened only'; k $s/templates.yaml 'not the two asks of ACC-058'; k $s/requirements.yaml 'it bounds the asks (ACC-058).'  # 1 1, 1, 1; 0s today
grep -c '^- id: ACC-058$' spec/conventions/access.yaml experiments/012-model-checking/account.yaml  # :1 twice; :0 today
k spec/conventions/access.yaml 'may list it (PND-009).' 'only root opens (SEC-008).' 'ACC-032, ACC-058, PRV-004' 'the two of ACC-058 apart' 'withdrawing an ask: `access`' 'missing bound covers none' '| ACC-058 |'  # seven 1s; 0s today
k spec/promises/sections.yaml '`withdraw-access-request.v1` or both (ACC-058).'; k $s/README.md 'The 40 Operation contracts' 'The 20 templates' 'exports 158 documents'; grep -cE '^\| `(request-access\.v1|withdraw-access-request\.v1|access-request)` \|' $s/README.md  # 1, 1 1 1, 5; 0s today
k spec/README.md 'Draft; 158 documents' 'Draft; 40 contracts' 'Draft; 20 templates'  # 1 1 1; 0s today
k spec/definitions.md '| asking Conduit |' 'objects, a value Resource and an asking Conduit;' 'of a value Resource and of an asking Conduit (OPS-019, ACC-029, ACC-058,' "lift tombstones, and take an Agent's asks (ACC-006, ACC-012, ACC-053, ACC-058)" 'and, as a right to open, is unlimited, as the root'  # five 1s; 0s today
echo "$(sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c 'DEC-064, DEC-079') $(grep -c '"DEC-064")' engine/crates/shell/src/server.rs) $(grep -cE '"id": "(sectioned-consumes-asking|access-ask-(offered|refused|states))"' conformance/expected-failures.yaml)"  # 3 2 4; 0 0 0 today
for c in sectioned-consumes-asking access-ask-offered access-ask-refused access-ask-states; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$c" 2>&1 | grep -c "^EXPECTED FAILURE $c "); done  # four 1s; 0s today
```

Guards, which pass today and after; the vectors test once, last, never the whole suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
a=conformance/conventions/access; n="conformance/language/sectioned/consumes-asking $a/ask-offered $a/ask-refused $a/ask-states docs/work-orders/298-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # head equals base; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/298-declared.yaml; echo "exit $?"  # ends 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(core/|conventions/access|promises/sections|README|definitions|CHANGES)|experiments/012-model-checking/account|engine/(STANDING|crates/shell/src/(server|tests))|conformance/|docs/work-orders/298-declared)' | wc -l | tr -d ' '  # 0
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo test --offline --release -p conduit-shell --lib every_engine_operation)
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case's first failing step and every check's last line.

## What follows

From the fix turns (the coordinator's rulings, open to the owner's strike): this order defines the
asking Agent as the access-request action's resolved subject and keeps the named target, resolved or
not, in the action's evidence; its cases assert that an ask raises one action at root and permits
nothing, not root's answering. Order 299 defines the answers `granted`, `superseded`, `declined`,
`withdrawn`, `expired` and `void`, their PND-003 rows, and `request-access.v1`'s `declined` result.
A later order adds reply-diagnostic substring assertions to the conformance runner, then the
bound refusal's wording and its assertions; until then IFC-050 leaves that wording to the engine.

Order 299; CON-032, if accepted; the engine, CLI and package orders.

## Record

Run on 2026-11-23 through Conduit itself, in stage 3 of experiment 028, by its pair 2, in Pacific
time, stacked on the decision log's entries DEC-082 to DEC-102.

**Worker-2's turn** (10:27 to 10:32) wrote the spec's first part of DEC-064: `request-access.v1`,
`withdraw-access-request.v1`, the asking-Conduit template, ACC-058 with the refusal list (the
owner's DEC-085 included) and four portable cases that fail until the engine order. All guards,
the loader tests, 204 core tests, clippy and the whole vectors test passed.

**Two blind reads** (10:33 to 10:37). The Codex verifier had one must (a case expected an ask for
an undefined operation to raise an action); the Claude reader (`reader-1`, while stage 3's reader
is held) had two musts (ACC-058 overrode an unchanged IFC-050; an ask could match two refusals)
and four shoulds and a nit. The fix turn (10:37 to 10:5x) applied six and asked two scope
questions; the coordinator ruled, open to the owner's strike, that the action's subject is the
asking Agent with the target as evidence, root's answers being order 299's, and that the bound
refusal's exact wording waits for a later order; the turn then finished (to 11:05). The guards and
the vectors test pass. The gate's run on the squash is the coordinator's, at the merge.
