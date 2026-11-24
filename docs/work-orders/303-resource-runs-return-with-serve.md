# Work order 303: local Resource runs come back when `serve` restarts (DEC-115)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the one case this order changes, an expected failure. It needs no network and changes no
Rust source; if a crate is missing from the cache, fetch nothing and report it. Run one cargo
command at a time, and not `tools/gate.py`. Never print a whole large file, case, diff or list:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-115](../decisions/log.md#dec-115-2026-11-23-local-runs-come-back-when-serve-restarts):
what happens to runs after serve restarts -> "Restart all local runs (Recommended)": PRV-026
extends to every Resource run on the engine's own runner, read secrets included; Agents stay as
they are. Today only providers and secret Resources run again, each when a call or a read needs
it (PRV-026, OBS-004); every other Resource that names no `runtime` (PRV-006), the web UI
package's (DEC-109) among them, stays stopped until the owner sets it up again.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
each read for such a run is a request recorded for the caller of the setup that last asked for
it (SEC-011, SEC-015); 2, a closed Conduit or a filter stops the read and the run (SEC-012). At
risk: 6: the engine runs again only what its own stop ended, dispatching no invocation again
(IMP-007), starting no model work (IMP-002) and repairing no difference observed (OBS-004);
ACC-019's exception for later work widens to a Resource's run.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: when Docker's daemon starts, it starts again each container whose restart
policy is `unless-stopped` and that was not stopped (`always` also starts those stopped by hand).
Not taken: a restart setting per Resource (the owner chose all); Agents (the owner's); running
again a run that ends while the engine goes on (PRV-010, OBS-004); a second unrequested attempt
after a failed read (a further setup is the retry).

**When it runs.** On the decision log's branch (DEC-082 to DEC-124), or on `main` once it merges.
Through Conduit: one Codex worker turn, the blind Codex verifier and Claude reader at once, one fix
turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold; how is
yours. The words, the case's changes and their choices (PRV-026 in place, one attempt each restart,
a DL17 part as a restart, read secrets on `local`) are the coordinator's, open to the owner's strike
(DEC-029).

**Shared files.** Orders 299 and 302 change `spec/CHANGES.md`, `spec/core/operations.yaml`,
`conformance/expected-failures.yaml` and `engine/STANDING.md`; 299 also `access.yaml`,
`implementation.yaml`, `implementation-defined.yaml`, both `README.md`, `definitions.md` and the
account: whichever merges later rebases, recounts and reruns `promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. A quoted phrase is matched with its line breaks taken as
spaces, and "X" → "Y" means that the phrase X, which occurs once in its file, becomes Y.

1. **PRV-026** (`spec/promises/providers.yaml`), in place, convention, must, becomes:

   ```text
   when: The engine restarts, or a part of it that held runs of the engine's own runner stops while the engine goes on, under DL17; and a Resource that is not retired had a run on that runner, under PRV-002 or PRV-006, that ended when the engine or that part stopped, and no run of it has been asked for since.
   then: Without waiting for a request, the engine makes one attempt to have its own runner run that Resource again, as PRV-007 and PRV-003 have it run an item, with each secret that the run needs read again under SEC-010 for the caller of the setup that last asked for the Resource's run, decided by the Resource's own Conduit to the secret Resource and the filters on that Conduit as they stand when the read is made; where a read does not succeed, SEC-012 holds. Where the Resource is a provider under PRV-002 or a secret Resource, and the engine is to make a provider call to it or read a secret through it before its run is asked for, the engine runs it so first.
   observable: The engine's own runner reaches each process it starts over stdio, under PRV-003, so the run of each Resource on it ends when the engine, or the part of it that holds the run, stops, under DL17: the server of each provider under PRV-002, and the process of every other Resource that names no `runtime`, under PRV-006; DL16 leaves each to this promise. For each restart the engine makes one attempt at each such run without a request, and another before each provider call or read that needs the run before it is asked for: it runs one again because its own stop ended the run, not because a run is seen to have stopped, under OBS-004, so a run that ends while the engine, and the part of it that holds the run, go on is run again only by its runner's restart policy, under PRV-010, or by a further setup, under PRV-007. Doing so starts no model work, under IMP-002. An Agent's run on that runner is not run again so: a further setup asks for it, under PRV-007. Each read is a request with its own access decision and record, under SEC-011 and SEC-015, for the caller of a setup that has ended; a run that this promise makes is asked for by no setup, so after a further restart the reads are again for the caller of that setup. Nothing is kept across the restart to stand in for a value, under SEC-017. Where a read does not succeed, as where that Conduit is closed or a filter on it now refuses it, SEC-012 follows, and a call that the engine makes of its own accord counts as a report not received. So the secret Resources that such a run reads belong on `local`: until the first report after the restart, PLC-023 holds back each read through one on another node but the machine provider's. A further setup of the Resource leaves a running run as it is, under PRV-007.
   ```

2. **OBS-004** (`spec/conventions/observation.yaml`): its observable's last sentence, from "Once
   the engine has restarted, it runs a provider" to its end, becomes "Once the engine has
   restarted, or a part of it that held runs of its own runner has stopped under DL17, it runs
   again each Resource whose run there ended with it, or with that part, under PRV-026, because
   its own stop ended that run, not because the run is seen to have stopped: a run that ends while
   the engine, and the part of it that holds the run, go on is run again only under its runner's
   restart policy, under PRV-010, or by a further setup." Its `when` and `then` stay.
3. **A Resource's run where a provider's was named.** First each change below and in decision 4.
   Then, where a sentence cites PRV-026, "Resource" takes the place of "provider" in "a provider's
   run", "that provider's run", "the run of a provider", "for a provider under" and "runs a provider
   again": in SEC-010 (twice), SEC-011 (thrice), SEC-017, REC-002, ACC-019's `then`, five rows of
   `spec/definitions.md` and, though it cites no promise, `spec/README.md`'s row for
   `promises/providers.yaml`.
   - PRV-002: "when it first needs it," → "once it has restarted, or that part has stopped,".
     PRV-021: "only when the engine next has a call to make of it, under PRV-026, which for a
     provider that no instance names may be never," → "once the engine has restarted, under
     PRV-026,".
   - DL16 (`spec/implementation-defined.yaml`): "the servers of the runtime, machine and
     package-source providers, and the processes of the secret Resources" → "the processes of the
     Resources"; "when it first needs each" → "once it has restarted". DL17: "the runs that it held
     of providers and of secret Resources" → "the runs of Resources that it held"; "each again
     before the engine next needs it." → "each again.".
   - SEC-012: "for a provider's run under PRV-026, the provider is not run, the provider call that
     needed it" → "for a run under PRV-026, the Resource is not run: a provider call or a read that
     needed a provider or a secret Resource"; "the next such call." → "the next such call or read;
     any other Resource is run again only when a further setup asks for its run, under PRV-007.".
     SEC-014: "a provider's run that ended with the engine" → "a Resource's run on its own runner
     that ended with the engine".
   - ACC-019's observable: "where a provider's run ended" → "where a Resource's run on the engine's
     own runner ended"; "held the run, and the engine first needs the provider again:" → "held the
     run:".
   - IFC-017 (`spec/interface/implementation.yaml`): "a secret Resource that PRV-026 makes again
     before a read" → "a Resource that PRV-026 makes again once the engine has restarted, or that
     part has stopped". `get-secret.v1`'s description (`spec/core/operations.yaml`): "a provider
     whose run ended with it" → "a Resource whose run on its own runner ended with it".
4. **The READMEs and `spec/definitions.md`.** `spec/README.md`: "a provider whose run ended with it,
   for" → "a Resource whose run on its own runner ended with it, for"; "for a provider after" → "for
   a Resource on the engine's own runner after"; "held a provider's run has stopped, it reads" →
   "held a run of its own runner has stopped, it reads"; "of a provider whose run ended with it, a
   secret Resource that its own runner ran among them, before it runs that provider again" → "of
   each Resource whose run there ended with it, before it runs that Resource again without waiting
   for a request"; "while no call needs that provider" → "until PRV-026 runs that server again".
   `spec/core/README.md`: "when it first needs it" → "without waiting for a call"; "a secret
   Resource that its own runner ran, before the first read through it" → "every other Resource that
   its own runner ran". `spec/definitions.md`: "when it first needs to call it" → "without waiting
   for a call"; "before the first read through it (PRV-002," → "in the same way (PRV-002,"; "stops,
   when the engine first needs it" → "stops, without waiting for a call"; "before the first read
   through it (PRV-026)" → "without waiting for a read (PRV-026)"; "held a provider's run has
   stopped, before it runs again a provider whose run ended" → "held a run of its own runner has
   stopped, before it runs again a Resource whose run there ended"; "and a provider whose run ended
   with the engine is run again when the engine first needs it" → "and each Resource whose run on
   the engine's own runner ended with the engine is run again once the engine has restarted"; and,
   after "no run is asked for (PRV-007, TPL-018).", insert "Once the engine has restarted, its own
   runner runs again, asked for by no setup, each Resource that is not retired whose run there ended
   with the engine; an Agent's waits for a further setup (PRV-026)."
5. **The model account** (`experiments/012-model-checking/account.yaml`). The rows of PRV-002,
   PRV-026, SEC-012, SEC-017 and ACC-019 become `text changed, not rerun`, `was:` each one's
   standing, as TPL-012's row has it; each note gains "Order 303 (DEC-115): PRV-026 runs again at a
   restart every Resource on the engine's own runner; text changed, not rerun.". The other rows
   stay, PRV-021's and SEC-010's so marked already.
6. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item (no count moves):

   ```text
   - Changed: PRV-026, once the engine restarts it runs again, without waiting for a request, each Resource that is not retired whose run on its own runner ended with it, its secrets read for the caller of the setup that last asked for that run; an Agent's run waits for a further setup (DEC-115).
   - Changed: OBS-004, PRV-002, PRV-021, SEC-010, SEC-011, SEC-012, SEC-014, SEC-017, REC-002, ACC-019, IFC-017, DL16, DL17 and `get-secret.v1` say a Resource's run where PRV-026 made a provider's again (DEC-115).
   ```

7. **The case.** `conformance/conventions/secret-reads/provider-restart/case.yaml` keeps its `id`,
   files and steps up to the consumer's setup ending succeeded; its `title` becomes "After engine
   restart the store and the consumer run again without a further setup", and `PRV-009` joins its
   `promises` and its `index.yaml` entry's. Its steps from `restore-state` on, in the form they use
   now, hold no set-up and, the case being portable, no `object_facts`:
   - `restore-state`, with `evidence` as other steps have it, expects `restored: true`, the
     builtin runner's `run.v1` for the store as its only intent, and no `get-secret.v1` under
     `/records`.
   - The store's run reported succeeded: `server/discover` dispatched to it. Its result: the
     read's `access-decision` and `invocation-dispatched` records, `serving` `$consumerSetup`,
     `caller` root, `consumer` the consumer, the case's Conduit, the store as `target`,
     `get-secret.v1`.
   - The read's result, a value other than the first run's: its ending records and the builtin
     runner's `run.v1` for the consumer, its `secretBindings` naming that read; the value absent
     from `/reply`, `/records`, `/state` and `/external_intents`.
   - The consumer's run reported succeeded; no intent.
   - `demo.inspect.v1` invoked on the consumer, as in `read-failed`: `/reply` lists every
     condition (arrays are exact), `Running` among them with `status: "True"` and
     `source: urn:conduit:builtin-runner`; no intent.

   Invocations that no record names take `restore-state`'s evidence IDs in order: the store's run,
   the read, the consumer's run. List the case in `conformance/expected-failures.yaml`, in `id`
   order, with the reason "PRV-026 as DEC-115 extends it: the first build runs nothing at a restart;
   DEC-115's engine order makes it pass". Declare each change to the case in
   `docs/work-orders/303-declared.yaml`, in [286's](286-declared.yaml) form, its paths no wider than
   `/title`, `/promises` and the steps from the restart on, then
   run `python3 tools/promise_standing.py --write`.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
o="a provider's run|that provider's run|the provider's run|a provider whose run|runs a provider again|run of a provider that|for a provider under|first needs|next needs|next has a call|first read through it|no call needs"
c=conformance/conventions/secret-reads/provider-restart/case.yaml
q() { for f in spec/*.md spec/*.yaml spec/*/*.md spec/*/*.yaml; do tr -s ' \n' '  ' < "$f" | grep -oE "$o"; done | wc -l | tr -d ' '; }
git --no-optional-locks status --short | wc -l | tr -d ' '  # 0
sed -n '/^## DEC-115, /,/^## DEC-116, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
k spec/promises/providers.yaml "a part of it that held a provider's run stops" "An Agent's run on that runner is not run again so"  # 1 1
q  # 41
grep -cE '^\| (PRV-026|OBS-004) \| .* \| case passes \|$' engine/STANDING.md  # 2
sed -n '/kind: restore-state/,$p' $c | grep -c 'set-up\.v1'  # 1
grep -c '"id": ' conformance/expected-failures.yaml  # 3
```

Where a line prints otherwise, as where order 299 or 302 merged first, work from where it stands
and list it under `own_choices`.

## Scope

Change, as the decisions say, the `spec/` files that hold what they change and
`spec/CHANGES.md`; the account's five rows; the case, its index entry and its expected-failures
entry; `docs/work-orders/303-declared.yaml`; `engine/STANDING.md` by the tool alone. Not the
decision log (DEC-115 stays not done), PLC-023, LIF-036, PKG-030, IMP-002, `engine/` sources,
`packages/` or `tools/`. Where a test fails because the engine reads the changed spec, report it
and change no engine file.

## Done when

Each line prints otherwise before this order and passes after (`j`, `k`, `o`, `c`, `q` as above):

```sh
q  # 0; 41 before
k spec/promises/providers.yaml 'makes one attempt to have its own runner run that Resource' 'For each restart the engine makes one attempt at each such run'  # 1 1; 0 0 before
k spec/conventions/observation.yaml 'it runs again each Resource whose run there ended with it, or with that part'  # 1; 0 before
k spec/promises/secrets.yaml "for a Resource's run again, under PRV-026" 'any other Resource is run again only when a further setup asks for its run'  # 1 1; 0 0 before
k spec/definitions.md 'asked for by no setup, each Resource that is not retired'  # 1; 0 before
for id in PRV-002 PRV-026 SEC-012 SEC-017 ACC-019; do awk -v id="$id" '$0=="- id: "id{p=1;next} p&&/^- id: /{p=0} p&&/^  standing: /' experiments/012-model-checking/account.yaml; done | grep -c 'text changed, not rerun$'  # 5; 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- .*(DEC-115)\.$'  # 2; 0 before
sed -n '/kind: restore-state/,/^- input:$/p' $c | grep -c 'core#run\.v1'  # 1; 0 before
sed -n '/kind: restore-state/,$p' $c | grep -c 'set-up\.v1'  # 0; 1 before
sed -n '/kind: restore-state/,$p' $c | grep -c 'tool: demo\.inspect\.v1$'  # 1; 0 before
sed -n '/"id": "secret-reads-provider-restart"/,/]/p' conformance/conventions/secret-reads/index.yaml | grep -c '"PRV-009"'  # 1; 0 before
grep -c '"id": "secret-reads-provider-restart"' conformance/expected-failures.yaml  # 1; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. secret-reads-provider-restart 2>&1 | grep -c "^EXPECTED FAILURE secret-reads-provider-restart ")  # 1; 0 before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
n=docs/work-orders/303-declared.yaml
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # head equals base; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/303-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
(cd engine && cargo test --offline --release -p conduit-loader --test examples)
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)
git reset -q -- $n  # stage nothing
```

Report each changed sentence, the case's first failing step and every check's last line.

## What follows

DEC-115's engine order: the core asks for these runs at a restart (in the harness, at
`restore-state`) and `serve` makes them, which passes the case (IDs kept in order, or a delta
declared) and removes its entry; the web UI package's Resource then comes back with `serve`
(DEC-109). With it can come a case for a provider run before a call after a failed read, and
`docs/OVERVIEW.md`, not binding, brought up to date for PRV-026.

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on `main` with the decision log
through DEC-131. Written, built, read and fixed in stage 3 of experiment 028; it is no order of
stage 3.

**Worker-2's turn** (13:33 to 13:48) rewrote PRV-026 and OBS-004 and the texts citing them, and
the restart case, an expected failure until the engine order; every check passed.

**Two blind reads** (13:55 to 14:12). The Codex verifier, `root-verifier-2.turn.v1`, passed it
with no finding; the Claude reader (`reader-1`) had no must, three shoulds (what a failed run
attempt looks like, the number and order of attempts, a secret Resource on another node) and two
nits, one the order's own uncommitted text, which the coordinator commits. One fix turn by
worker-2 applied the rest. The gate's run on the squash is the coordinator's, at the merge.
