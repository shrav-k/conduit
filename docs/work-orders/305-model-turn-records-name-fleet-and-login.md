# Work order 305: each model turn's end record names its Fleet and login Resource (DEC-117, the spec)

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

**The owner's words.** [DEC-117](../decisions/log.md#dec-117-2026-11-23-every-model-turns-record-names-its-fleet-and-login):
attribution, "Yes, in v0 (Recommended)": the end record of every call that starts model work
names the Agent's Fleet and its login Resource beside the Agent; the CLI lists those records for
a login or a Fleet with each one's reported usage and adds nothing up; any total is a UI
package's. DEC-094 stands: "Conduit specifies what must be reported and what contracts must be
upheld. The engine then trusts this reporting", and the engine computes nothing itself. So does
DEC-116: a model account is its login Resources, one per account per machine (DEC-121), so the
login Resource a turn names is the account it spent on that machine.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
everything an agent does through Conduit is attributable: each model turn's end record names the
Fleet that answers for the Agent and the login whose account the turn used; 6, naming them
dispatches, holds and decides nothing, and the record of a request that ends before dispatch is
unchanged; 3, the login Resource is named by its ID, owned once by its Pool, and nothing of it is
copied into the record. At risk: who sees a turn's record. Since the record names them,
inspecting the login Resource (its owning Pool, ACC-033 row 6) or the Fleet (`management`, row
7) lists each turn's whole end record under LIF-013: its caller, Conduit and consumer, outcome
and usage, and its inputs and outputs where the operation's `records` keeps them (REC-002,
CTR-004). Whoever may inspect the Agent through its Fleet (row 6A) sees these already; the
login's Pool and `management`, which row 6A keeps from the Agent of a live Fleet, do not. Today
that Pool sees, in the login's `limits`, only the newest observation per window, with the
instance and invocation that reported it (IFC-053); registering an Agent whose login place
refers to the login needs that Pool too, unless it is in the same submission (ACC-033).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: a Kubernetes audit event's `objectRef` names the object's namespace beside
its name, and a CloudTrail event's `userIdentity` names the role that issued a session
(`sessionContext.sessionIssuer`) beside the session; Amazon Bedrock's model invocation log gives
each call's identity and token counts and leaves totals to other tools. Not taken: the
`object.v1` form, since a record names every other object by its ID; the members on the call's
other records, or on a request that ends before dispatch, which starts no model work; the login's
Pool; any total per login or Fleet (DEC-117 leaves it to a UI package; IFC-051 and IFC-053 sum
nothing).

**When it runs.** On the decision log's branch (DEC-082 to DEC-124 on top of `main`), or on
`main` once it merges. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The words, the case's name and these choices are the
coordinator's, open to the owner's strike (DEC-029): the members are IDs named `fleet` and
`loginResource`, since the spec's `login` is a Type's sign-in login; only the invocation's end
record holds them, the one that holds `usage` (IFC-051); only where the target is an Agent; the
login Resource is the one the login place referred to when the call was made, as IFC-053 has
it, in an end recorded late or unknown too; and the record is listed by inspecting either
under LIF-013, with the authority ACC-033 gives, which widens who sees it as At risk says and
which the owner may narrow. The case inspects `login` alone: a Fleet's records also hold every
set-up made through it, so their exact list would be long, and the Fleet's half of the listing
is left to a later case.

**Shared files.** Orders 299, 302, 303 and 304 also change `spec/CHANGES.md`; 299, 302 and 303
`conformance/expected-failures.yaml` and `engine/STANDING.md`; 299, 303 and 304
`spec/interface/implementation.yaml`; 299 and 303 `spec/definitions.md`; 299
`spec/core/results.yaml`; and 303 a sentence of REC-002 that this order leaves as it is:
whichever merges later rebases, recounts and reruns `promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width.

1. **REC-002** (`spec/conventions/records.yaml`). Its `then` ends, after "the time, and how it
   ended.":

   ```text
   The record of an invocation's end, where the operation's contract is marked `startsModelWork: true` and the target is an Agent, also holds the Agent's owning Fleet and, where the Agent's Type states `loginResource`, the login Resource that its login place referred to when the call was made (CTR-013).
   ```

   Its observable, after "under IFC-051 and IFC-052, whatever `records` states.":

   ```text
   The Agent's Fleet and login Resource are `fleet` and `loginResource` in `record.v1`, each an ID, as the record names the caller and the target. Both are held whatever `records` states and however the invocation ended, in an end recorded late under REC-007 or as unknown at a restart under IMP-007 too; `loginResource` is absent where the Agent's Type states no `loginResource`. No other record holds the members `fleet` and `loginResource`: neither the other records of the same request, nor the record of a request that ends without being dispatched, which starts no model work, nor the end record of a call whose target is not an Agent. The record names that Fleet and that login Resource, so inspecting either finds it, under LIF-013, by the authority ACC-033 gives for inspecting it. The engine sums or compares no usage of the records that name a Fleet or a login Resource: each record shows its own member, under IFC-051.
   ```

2. **`record.v1`** (`spec/core/results.yaml`). `data` gains, after `target`,
   `fleet: {$ref: github.com/shrav-k/conduit/core#id.v1}` and
   `loginResource: {$ref: github.com/shrav-k/conduit/core#id.v1}`, neither required. Its
   description ends, after "(IFC-052).": "The end record of a call of an operation marked
   startsModelWork: true whose target is an Agent names in `fleet` the Agent's owning Fleet and,
   where the Agent's Type states loginResource, in `loginResource` the login Resource its login
   place referred to when the call was made (REC-002)."
3. **IFC-051's observable** (`spec/interface/implementation.yaml`): "The record names the target
   whose call it was (REC-002)." becomes "The record names the target whose call it was and,
   where that target is an Agent and the call starts model work, its owning Fleet and, where its
   Type states `loginResource`, its login Resource (REC-002)." Its "the engine sums, compares and
   acts on none of its numbers" stays, and IFC-053 does not change: "The engine sums nothing"
   stays.
4. **`spec/definitions.md`,** the `record` row: after "names in `subject` the instance whose run
   it is;", insert "the end record of a call that starts model work on an Agent names its owning
   Fleet in `fleet` and, where its Type states `loginResource`, its login Resource in
   `loginResource`;".
5. **One case,** `conformance/conventions/requests/turn-names-fleet-and-login/`, id
   `requests-turn-names-fleet-and-login`, title "A model turn's end record names the Agent's
   Fleet and its login Resource", layers `convention` and `interface`, promises REC-002, IFC-051
   and LIF-013, profile `first-build`, in the form of `requests-limits-latest`:
   - Its one file of its own is `documents.json`, a copy of
     `requests-login-resource-one-at-a-time`'s with one more Type, `own-agent.v1`, family Agent,
     stating no `loginResource`, offering `#turns.v1` through the same implementation; an Agent
     `c` of it in `workers`; and a Conduit `call-c` from `turns`, root to `c`. Its `fixture` and
     `manifest` point at `../login-resource-one-at-a-time/`, as `requests-limits-latest`'s do.
   - Steps: register the project; set up `a` and `c` and open `call-a` and `call-c` as
     `requests-limits-latest` does; root calls `call-a.turn.v1` and the implementation completes
     it succeeded with a usage member; root calls `call-c.turn.v1`, completed the same way; root
     inspects `login` with `logins.inspect.v1`.
   - It expects, with a step's records asserted under `records`, an exact ordered list as the
     runner reads arrays (not `record_facts`): at `call-a`'s completion, the `invocation-ended`
     record holds `target` `a`, `fleet` the ID of `workers`, `loginResource` the ID of `login`
     and the usage as sent; the complete step's `request-ended` record, and the invoke step's
     `access-decision` and `invocation-dispatched` records, hold neither `fleet` nor
     `loginResource` (`absent` on each); `c`'s end record holds `fleet` and no `loginResource`.
     The inspection's `structuredContent.records` is an exact list too: every record that names
     `login`, in source order, as the engine returns it today, with `a`'s end record added at
     its place; it holds no record of `c`'s call. Each entry may hold only what tells it apart,
     such as `type` and `data.invocation`.
   - With no `fleet` or `loginResource` expected anywhere and `a`'s end record left out of that
     list, the case passes on today's engine (run it so before its expected-failures entry is
     in), so every other expectation is checked before the engine order. Report that run's last
     line and the list's record types in order.

   One entry in `conformance/conventions/requests/index.yaml` and one in
   `conformance/expected-failures.yaml`, each in `id` order, the latter with the reason "REC-002,
   order 305: no model turn's end record names its Fleet or login Resource yet; DEC-117's engine
   order makes it pass and removes this entry". Declare it in `docs/work-orders/305-declared.yaml`,
   in [297's](297-declared.yaml) form, then run `python3 tools/promise_standing.py --write`.
6. **CHANGES and counts.** In `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

   ```text
   - Changed: REC-002, IFC-051 and `record.v1`, the end record of a call that starts model work on an Agent names its Fleet in `fleet` and, where its Type states one, its login Resource in `loginResource`, and the engine adds no usage up (DEC-117).
   ```

   No count moves: the order adds no promise or schema, and REC-002, IFC-051 and LIF-013 already
   have cases; `check_schemas.py` checks the totals.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
sed -n '/^## DEC-117, /,/^## DEC-118, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
sed -n '/^  name: record.v1$/,/^---$/p' spec/core/results.yaml | grep -cE '^        (target|usage|fleet|loginResource):'  # 2
k spec/conventions/records.yaml 'the time, and how it ended.' 'under IFC-051 and IFC-052, whatever `records` states.'  # 1 1
k spec/interface/implementation.yaml 'The record names the target whose call it was (REC-002).' 'the engine sums, compares and acts on none of its numbers' 'The engine sums nothing and compares no allowance'  # 1 1 1
k spec/definitions.md 'names in `subject` the instance whose run it is; and the record of a change'  # 1
sed -n '/^- id: REC-002$/,/^- id: REC-003$/p' experiments/012-model-checking/account.yaml | grep -c '^  standing: static$'  # 1
grep -c '"id": ' conformance/expected-failures.yaml  # 3
ls conformance/conventions/requests | grep -cE '^(limits-latest|login-resource-one-at-a-time|turn-names-fleet-and-login)$'  # 2
grep -cE '"\.\./login-resource-one-at-a-time/(fixture|conduit)\.json"' conformance/conventions/requests/limits-latest/case.yaml  # 2
k engine/crates/conformance/src/expect.rs 'arrays are exact ordered sets/sequences' '{pointer}/{key}: missing required field'  # 1 1
```

## Scope

Change, as the decisions say: `spec/conventions/records.yaml`, `spec/core/results.yaml`,
`spec/interface/implementation.yaml`, `spec/definitions.md` and `spec/CHANGES.md`; the case, its
index entry and its expected-failures entry; `docs/work-orders/305-declared.yaml`;
`engine/STANDING.md` by the tool alone. Not the decision log (DEC-117 stays not done),
`experiments/012-model-checking/account.yaml` (REC-002 is static there), `engine/` sources,
`packages/` or `tools/`. Where a test fails because the engine reads the changed spec, report it
and change no engine file.

## Done when

Each line prints otherwise before this order and passes after, with `j` and `k` as above:

```sh
k spec/conventions/records.yaml 'the login Resource that its login place referred to when the call was made (CTR-013).' 'are `fleet` and `loginResource` in `record.v1`, each an ID' 'The engine sums or compares no usage of the records that name a Fleet or a login Resource'  # 1 1 1; 0 0 0 before
sed -n '/^  name: record.v1$/,/^---$/p' spec/core/results.yaml | grep -cE '^        (target|usage|fleet|loginResource):'  # 4; 2 before
k spec/core/results.yaml 'referred to when the call was made (REC-002).'  # 1; 0 before
k spec/interface/implementation.yaml 'The record names the target whose call it was (REC-002).' 'where its Type states `loginResource`, its login Resource (REC-002).'  # 0 1; 1 0 before
grep -cF 'names its owning Fleet in `fleet`' spec/definitions.md  # 1; 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- .*(DEC-117)\.$'  # 1; 0 before
cat conformance/expected-failures.yaml conformance/conventions/requests/index.yaml | grep -c '"id": "requests-turn-names-fleet-and-login"'  # 2; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-turn-names-fleet-and-login 2>&1 | grep -c "^EXPECTED FAILURE requests-turn-names-fleet-and-login ")  # 1; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-turn-names-fleet-and-login 2>&1 | grep -cE ': /records/[0-9]+/data/(fleet|loginResource): missing required field')  # 1; 0 before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
n="conformance/conventions/requests/turn-names-fleet-and-login docs/work-orders/305-declared.yaml"
git add -N $n
k spec/interface/implementation.yaml 'the engine sums, compares and acts on none of its numbers' 'The engine sums nothing and compares no allowance'  # 1 1
k spec/core/results.yaml 'A reader adds neither part to its whole.'  # 1
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/305-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/core
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, the case's first failing step, the inspection's record types in
order, the last line of the run without the new members, and every check's last line.

## What follows

DEC-117's engine order: the engine keeps the Fleet and the login Resource from each such call's
dispatch and writes `fleet` and `loginResource` in its end record, a late or unknown end
included, which passes the case. Then the CLI's listing: `conduit` lists a login Resource's or a
Fleet's records, read by `inspect.v1` under LIF-013, with each one's reported usage and no total
(DEC-117); any total is a UI package's (DEC-101). DEC-117 is done when the listing merges.

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on `main` with the decision log
through DEC-135. Written, built, read and fixed in stage 3 of experiment 028; it is no order of
stage 3.

**Worker-2's turn** (15:01 to 15:17) added `fleet` and `loginResource` to the model turn's end
record, REC-002, `record.v1`, IFC-051 and the definitions, with the case an expected failure
until the engine order; every check passed.

**Two blind reads** (15:20 to 15:40). The Codex verifier, `root-verifier-2.turn.v1`, passed it
with no finding; the Claude reader (`reader-1`) had no must, one should (which member IFC-051's
sentence meant) and two nits. One fix turn by worker-2 applied them. The gate's run on the squash
is the coordinator's, at the merge.
