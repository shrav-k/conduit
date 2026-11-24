# Work order 302: `list.v1` gives each instance's owner, placement, revision and Running (DEC-114)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the one case this order adds, an expected failure. It needs no network and changes no Rust
source; if a crate is missing from the cache, fetch nothing and report it. Run one cargo command
at a time, and not `tools/gate.py`. Never print a whole large file, case, diff or list: use
`head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-114](../decisions/log.md#dec-114-2026-11-23-the-instance-list-gives-owner-placement-revision-and-running):
"Widen list.v1 (Recommended)": `list.v1` gives, for each instance, its owner, its placement, its
revision and the status of its Running condition. Any client (the CLI, the web UI package of
DEC-109 and DEC-110, the owner's app) can then route `inspect.v1`, `answer.v1` and `sign-in.v1`
to an instance's owner and show whether it runs without inspecting each one.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3,
listing adds no inspection authority and discloses nothing about omitted items (OPS-036 stays as
it is on both counts); 7, the listing request's record is unchanged.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: `kubectl get` shows each object's owner-derived columns, node and readiness
from one list call, and a Kubernetes list carries each item's `resourceVersion`. Not taken: a
count or summary of any kind (OPS-036 forbids it); conditions other than Running.

**When it runs.** On the decision log's branch (DEC-103 to DEC-115 on top of DEC-082 to DEC-102),
or on `main` once both merge. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, the
decision says what must hold; how is yours. The words and the case's name are the coordinator's,
open to the owner's strike (DEC-029).

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width.

1. **`list.v1`'s object item** (`spec/core/operations.yaml`). Four optional properties, each
   given for an instance (Interface, Agent, Resource, Pool, Fleet or Conduit item whose
   `template` is false) and absent for a template:
   - `owner`: the instance's owning Pool or Fleet, as `inspect.v1` and the records give an owner
     (`object.v1`, its kind Pool or Fleet, as `spec/core/results.yaml` constrains a record's
     `owner`); given exactly when the instance has one: a Resource's Pool, an Agent's Fleet.
   - `placement`: the node it is placed on, in the same shape `inspect.v1` gives placement
     (find it under `spec/core/`; reuse it, define nothing new); absent where the instance has no
     placement.
   - `revision`: its current revision, as `inspect.v1` gives it.
   - `running`: an object with the Running condition's `status` and, where it has one,
     `lastHeartbeatTime`, both as `condition.v1` types them; absent where the instance's item is
     not run (PRV-009 and its neighbours say which are).
   The operation's description says so in one sentence, and that the four come from the same
   state `inspect.v1` reads and add no authority.
2. **OPS-036** (`spec/conventions/operations.yaml`): its observable's "Object items have kind,
   name, ID, whether they are templates, and state for instances" gains ", and for instances
   their owner, placement, revision and Running status where they have them (DEC-114)". Its
   `then`, the no-count sentence and the inspection rule stay as they are: a field is given only
   for an item the actor may list, and nothing is given about an omitted one.
3. **One case,** `conformance/conventions/list/instance-details/` (`case.yaml`, and a
   `state.json` if the family's cases use one), id `list-instance-details`, promises OPS-036 and
   ACC-033: through `management`, root lists a Pool with one Resource and a Fleet with one Agent;
   the Resource's item has `owner` naming the Pool, `placement`, `revision`, and the Agent's
   `owner` names the Fleet; a template's item has none of the four. One entry in
   `conformance/conventions/list/index.yaml`, one in `conformance/expected-failures.yaml`
   (sorted by id, its reason naming this order and the engine order that follows), and
   `docs/work-orders/302-declared.yaml` in the shape the other `*-declared.yaml` files use.
4. **CHANGES and counts.** One line in `spec/CHANGES.md` ending "(DEC-114)."; recount any count
   this moves (`check_schemas.py` checks the totals).

## Start check

```sh
k() { f=$1; shift; for p in "$@"; do grep -cF -- "$p" "$f"; done | paste -sd' ' -; }
grep -c '^## DEC-114, ' docs/decisions/log.md  # 1
sed -n '/^  name: list.v1$/,/^---$/p' spec/core/operations.yaml | grep -cE '^ {12}(owner|placement|revision|running):'  # 0
k spec/conventions/operations.yaml 'whether they are templates, and state for instances'  # 1
ls conformance/conventions/list | wc -l | tr -d ' '  # 6
```

## Scope

Change, as the decisions say: `spec/core/operations.yaml`, `spec/conventions/operations.yaml`,
`spec/CHANGES.md`, any README or `definitions.md` count the change moves; the case, its index
entry and its expected-failures entry; `docs/work-orders/302-declared.yaml`; `engine/STANDING.md`
by the tool alone. Not the decision log, `engine/` sources, `packages/` or `tools/`. Where a test
fails because the engine reads the changed spec, report it and change no engine file.

## Done when

Each line prints otherwise before this order and passes after:

```sh
sed -n '/^  name: list.v1$/,/^---$/p' spec/core/operations.yaml | grep -cE '^ {12}(owner|placement|revision|running):'  # 4; 0 before
grep -c 'owner, placement, revision and Running status where they have them (DEC-114)' spec/conventions/operations.yaml  # 1; 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- .*(DEC-114)\.$'  # 1; 0 before
grep -c '"id": "list-instance-details"' conformance/expected-failures.yaml  # 1; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. list-instance-details 2>&1 | grep -c "^EXPECTED FAILURE list-instance-details ")  # 1; 0 before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
n="conformance/conventions/list/instance-details docs/work-orders/302-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/302-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/core
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, the case's first failing step and every check's last line.

## What follows

The engine order: `list.v1` fills the four from the state `inspect.v1` reads, which passes the
case; the CLI's `conduit list` shows them; then the web UI package's instance pane uses them.

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on the decision log's branch (DEC-082
to DEC-115, merged since). Written, built, read and fixed in stage 3 of experiment 028; it is no
order of stage 3.

**Worker-1's turn** (12:06 to 12:18) built the four properties, OPS-036's sentence and the case,
an expected failure until the engine order; the start check matched and every check passed.

**Two blind reads** (12:20 to 12:45). The Codex verifier, `root-verifier-1.turn.v1`, had no must
and one should (the case did not require `running`); the Claude reader (`reader-1`) had no must,
two shoulds and four nits, one of them the order's own uncommitted text, which the coordinator
commits. By the coordinator's ruling `placement` keeps the shape `inspect.v1` gives, with one
sentence on showing the node by name. One fix turn by worker-1 (13:32 to 13:42) applied the rest.
The gate's run on the squash is the coordinator's, at the merge.
