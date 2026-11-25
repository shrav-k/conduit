# Work order 410: an engine asked to stop takes no new request and lets those in progress end within a grace period (DEC-176)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the two cases this order adds, both expected failures. It needs no network and changes no
source of the loader, core or shell: its one Rust change is two lines of the conformance runner
(decision 6). If a crate is missing from the cache, fetch nothing and report it. Run one cargo
command at a time, and not `tools/gate.py`. Never print a whole large file, case, diff or list:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-176](../decisions/log.md#dec-176-2026-11-24-serve-stops-gracefully),
"Yes, graceful stop (Recommended)": a stop of serve takes no new request and lets those in
progress finish within a bound; a spec order and an engine order; the restart script retires.
Today SIGINT or SIGTERM cancels each call in progress at once with an internal error (`request
cancelled`), and the engine's own runner then ends every run (DL16's rows of
`engine/CONFORMANCE.md`); no promise says what a stop does. IMP-007 ends an invocation in
progress across a restart unknown; PRV-026 ([order 303](303-resource-runs-return-with-serve.md))
runs each Resource on the engine's own runner again at the next start.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 6, a
call cut at the end of the grace period ends unknown, never failed, and is not dispatched again
without a repeat (IMP-007, OPS-012); 2, every request a stopping engine still takes is decided as
any other (ACC-001); 7, each request turned away and each call cut is recorded naming OPS-037,
and the stop is recorded against `system`. None at risk: a request turned away gets no access
decision, as one that ACC-037 decides gets none, and is recorded all the same.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
Kubernetes sends SIGTERM and waits `terminationGracePeriodSeconds` before SIGKILL, systemd waits
`TimeoutStopSec`, and Go's `http.Server.Shutdown` closes its listeners and waits for active
connections; not taken are the word "bound" (`access` has it), a `preStop` for instances (the
stop retires nothing), a stop operation through `management` (DL20 leaves the means to each
engine), serving reads during the stop, and waiting past the grace period.

**When it runs.** On `main`, through Conduit, on pair 3 or 4 (DEC-180): one Codex worker turn,
the blind Codex verifier and Claude reader at once, one fix turn, the gate (DEC-046, DEC-071,
DEC-033). By ruling 89, each decision says what must hold; how is yours. Every word, name and
rule below that DEC-176 does not state is the coordinator's, open to the owner's strike
(DEC-029), among them: OPS-037 (OPS-035 was never used in `spec/`); "grace period"; which
requests a stopping engine still takes, a model's own calls and IFC-049's retries among them;
unavailable first in OPS-023's order, with no access decision; a decided request not yet
dispatched, outside the work in progress, ending at once; the two records of the stop; the
`stop` input, its reply carrying each cut call's result, and the cases; and order 411's grace
period, set by configuration and a `serve` flag until DEC-139's startup file exists.

**Shared files.** Other orders change `spec/CHANGES.md`, `spec/README.md`'s totals,
`conformance/expected-failures.yaml`, the catalogue's counts, the account and
`engine/STANDING.md`: whichever merges later rebases, recounts and reruns
`promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. A quoted phrase is matched with its line breaks taken as
spaces, and "X" → "Y" means that the phrase X, which occurs once in its file, becomes Y.

1. **OPS-037,** in `spec/conventions/operations.yaml`, after OPS-036:

   ```text
   layer: convention; level: must
   when: The engine is asked to stop, by a means that DL20 of the implementation-defined list names, such as the signal SIGTERM.
   then: From then on it takes no new request: each request that arrives is not dispatched and reports unavailable, save a repeat answered without a dispatch and a request that is part of the work in progress, as the observable says. Each invocation in progress goes on. Once none is in progress, or once the grace period that DL20 states has passed since the stop was asked for, whichever is first, the engine stops. An invocation still in progress at the end of the grace period is cancelled, as OPS-027 cancels one, and ends then with the outcome unknown, or, for a setup or a retirement, with the outcome that OPS-027 gives it: OPS-022's, or LIF-021's for `retire-with-resources.v1`. The engine keeps a record when it is asked to stop and another when it stops.
   observable: A stopping engine takes, and decides as any other, each request that is part of the work in progress: an onward request that names an invocation in progress in `io.github.shrav-k.conduit/serving`, under IFC-020; any other request from an instance while an invocation dispatched to it is in progress, since a model's own calls may name none; an `answer.v1` for a pending action that an invocation in progress waits for; and a retry carrying the requestState of an invocation in progress, under IFC-049. The parts of an invocation in progress, such as a hook, a read of a secret or a provider call, are dispatched as before. A repeat that OPS-009, OPS-012, OPS-032 or OPS-034 answers without a dispatch is answered as before; one that OPS-012 or OPS-024 would dispatch as a new attempt is new work. Every other request is turned away, `inspect.v1` among them: no access decision is made for it, OPS-023 puts this reason first for a request that can be read, its result carries an error diagnostic naming this promise and saying that the request may be made again once the engine has started again, under IFC-050, and REC-001 records it as ended without being dispatched, with this promise's ID. A request decided before the stop, not yet dispatched and not part of the work in progress, such as one held waiting, ends unavailable under this promise at once, as REC-007 has one found so at a restart end. `initialize`, `tools/list` and the protocol's other messages are answered as before. At the end of the grace period the engine sends each cancellation, and each stop that TPL-009 still asks for of a retirement so cancelled, and waits for no answer: the attempt has none, under OPS-012, and one that arrives later changes no outcome, under OPS-013. Each caller still waiting is answered with the outcome and an error diagnostic naming this promise, whose ID the end record holds in `ruleId`, so that a reader tells it from an end at a restart under IMP-007. A cut invocation is not dispatched again without a repeat, under IMP-007; once the engine has started again a repeat is answered under OPS-012, and a new attempt answered failed leaves it unknown, as OPS-012 and OPS-024 have it, until a reconciliation under OPS-014. Only once it stops does the engine end the runs of its own runner, as DL16 states, so the runs that the work in progress needs, an Agent's among them, go on through the grace period; PRV-026 runs again at the next start each Resource whose run so ended. A run that another runtime provider holds does not end with the engine. Before it stops, the engine keeps each record that REC-007 holds to be kept late, where a record can then be kept. Both records of the stop name `system`, so inspecting it lists them under LIF-013, and hold no caller: the first states the means of the stop, the second whether an invocation was cut. An engine that stops otherwise, as when killed during the grace period, restarts as IMP-007 and REC-007 have it.
   ```

2. **Its citations.**
   - OPS-023's `then`: "otherwise unavailable where the access decision cannot be made under
     ACC-037," → "otherwise unavailable where the engine is stopping and does not take the
     request under OPS-037, else unavailable where the access decision cannot be made under
     ACC-037,"; "Where the access decision cannot be made, nothing else about the request is
     examined or reported;" → "Where the engine does not take the request, or its access decision
     cannot be made, nothing else about it is examined or reported;". Its observable, after "is
     reported only where no other reason applies.": "A request that a stopping engine does not
     take reports unavailable under OPS-037 whatever else is wrong with it, provided it can be
     read, and its result carries no diagnostic but OPS-037's."
   - OPS-012's observable: "under OPS-027 or when a call passes the time that DL10 allows it, is
     no answer." → "under OPS-027, when a call passes the time that DL10 allows it or when the
     grace period of a stop ends under OPS-037, is no answer."
   - IMP-007's observable, after "An invocation that was in progress across a restart has the
     outcome unknown.": "One that the end of the grace period of a stop cut, under OPS-037, ended
     unknown then, and is likewise not dispatched again without a repeat; a request that a
     stopping engine did not take ends unavailable under OPS-037, not unknown."
   - REC-001's `when`: "PRV-013)." → "PRV-013); the engine is asked to stop, or stops
     (OPS-037)." REC-007's observable: "Records held to be kept late are lost if the engine stops
     before a record can be kept;" → "Records held to be kept late are kept before the engine
     stops under OPS-037, where a record can then be kept, and are lost if it stops before a
     record can be kept;".
   - PRV-026's observable, after "An Agent's run on that runner is not run again so: a further
     setup asks for it, under PRV-007.": "A stop under OPS-037 ends the runs on that runner only
     once no invocation is in progress or its grace period has passed, and each Resource's run it
     ends is run again so once the engine has started again."
3. **The implementation-defined list** (`spec/implementation-defined.yaml`).
   - DL3: "No promise limits how long an invocation is in progress:" → "No promise limits how long
     an invocation is in progress, but OPS-037 at a stop of the engine:".
   - DL16: "when the engine stops and when it starts again," → "when the engine stops, which
     under OPS-037 is only once no invocation is in progress or its grace period has passed, and
     how long it then gives them to end before it ends them by force, as systemd's
     `TimeoutStopSec` does for a unit, and when it starts again,".
   - DL17, after "although the engine as a whole goes on.": "A stop under OPS-037 is the whole
     engine's: from the time it is asked for no part takes a new request, and the grace period of
     DL20 is one for all its parts; whether a part that stops alone first lets the invocations in
     progress on it end is stated here."
   - After DL19, `id: DL20`, `subject: The stop of the engine`, `document:` "The means by which the
     engine is asked to stop under OPS-037, such as the signals SIGTERM and SIGINT, or a command;
     the grace period, which is finite, and how it is set, as Kubernetes'
     `terminationGracePeriodSeconds` and systemd's `TimeoutStopSec` are; what a further ask does
     while a stop is under way, such as ending the grace period at once; and what the engine does
     of its own accord meanwhile, such as asking the machine provider for its report under
     PRV-014. What becomes of the processes of its own runner once it stops is DL16."
4. **Definitions, counts and CHANGES.** `spec/definitions.md`: after the row "in progress", "|
   grace period | The time that a stopping engine gives the invocations in progress to end; at its
   end it cuts those still in progress, as OPS-037 has them end, and stops. Each engine states it
   under DL20 (OPS-037). |"; in "How a request ends", "be read; otherwise refused," → "be read;
   otherwise unavailable where a stopping engine does not take it (OPS-037); otherwise refused,";
   in the row "access decision", "No access decision is made for a provider call (PRV-005)." →
   "No access decision is made for a provider call (PRV-005), nor for a request that a stopping
   engine does not take (OPS-037)."; in the row "unavailable", "or the state of the Conduits
   cannot be established." → "the state of the Conduits cannot be established, or the engine is
   stopping.". `spec/README.md`: "Draft; 31 promises; vectors test slices of 30 promises" →
   "Draft; 32 promises; vectors test slices of 30 promises"; "522 in all: 226 of the layer
   `language`, 246 of the layer `convention`" → "523 in all: 226 of the layer `language`, 247 of
   the layer `convention`"; "Draft; 17 items" → "Draft; 18 items". `spec/CHANGES.md`, under
   `## [Unreleased]`, after its last item:

   ```text
   - Added: OPS-037, an engine asked to stop takes no new request, which reports unavailable, lets each invocation in progress end within a grace period, cuts those still in progress at its end, and records the stop (DEC-176).
   - Added: DL20, the means of asking the engine to stop and the grace period of OPS-037 (DEC-176).
   - Added: `grace period` in spec/definitions.md (DEC-176).
   - Changed: OPS-023, OPS-012, IMP-007, REC-001, REC-007, PRV-026, DL3, DL16 and DL17 for OPS-037's stop, and in spec/definitions.md "How a request ends" and the rows `access decision` and `unavailable` (DEC-176).
   ```

5. **The catalogue and the account.** `docs/implementation/SAFETY_CATALOGUE.md`: after OPS-036's
   row, "| OPS-037 | convention | must | effect | `then`: “ends then with the outcome unknown” —
   ending a call cut at the end of the grace period as failed lets a repeat apply a partly applied
   effect again. | — |"; operations.yaml's "Read: 32 promises." → "Read: 33 promises."; in its
   head, after order 286's sentence, "Work order 410 added the row of OPS-037, which it wrote.",
   and the counts become 523 read, 415 safety, effect 126, convention 234 (both places) and 130 in
   the seven convention files. `experiments/012-model-checking/account.yaml`: after OPS-036's row,
   one with `id: OPS-037`, `class: safety`, `standing: not modelled` and `note: Written by work
   order 410; not yet modelled.`; the notes of IMP-007, OPS-012 and PRV-026, already `text changed,
   not rerun`, each gain "Order 410 (DEC-176) cites the stop of OPS-037; text changed, not
   rerun."; REC-001's note, whose row stays `static`, gains "Order 410 (DEC-176) adds the two
   records of the stop to its when; R1 was not rerun.".
6. **The runner's input and the conformance document.** `conformance/case-schema/inputs/stop.json`,
   in `restore-state.json`'s form: `kind` const `stop`; `evidence`, an object; `means`, a string
   of at least one character; `graceEnded`, const `true`; `kind` and `evidence` required. In
   `engine/crates/conformance/src/portable.rs`, `"stop",` after `"restore-state",` in
   `check_case`'s allowed list and in its test's list of allowed kinds; in `conformance/README.md`,
   "`restore-state`, `list-tools`," → "`restore-state`, `stop`, `list-tools`,". In
   `engine/CONFORMANCE.md`'s "Choices still required", after DL19's row: "| DL20 | Not built;
   order 411 builds OPS-037. SIGINT or SIGTERM stops `serve` at once: each call in progress is
   cancelled under OPS-027 and its caller answered with an internal error, no request is answered
   unavailable under OPS-037, and no record of the stop is kept. |".
7. **Two cases** in `conformance/conventions/requests/`, each in its `index.yaml` in `id` order,
   layers `convention` and `interface`, profile `first-build`, from `{"recipe":"empty"}`, on the
   files and through step 14 of `explain-outcome-causes`, in
   `conformance/conventions/explain/outcome-causes/`: the S1 project set up and the Agent's
   `project.file.read.v1`, call A, dispatched. Then a `stop` step with `means` `SIGTERM`: reply
   `{"stopping": true}`, no intent, and one record of `type` `engine-stop` with `data.phase`
   `asked` and `data.means` `SIGTERM`. Every `engine-stop` record holds `data.target` `system`'s
   ID and no `caller`.
   - `requests-stop-takes-no-new-request` (`stop-takes-no-new-request/`; OPS-037, OPS-023,
     REC-001): root's `management.inspect.v1` of the filesystem Resource, then the Agent's
     `project.file.write.v1` (no invocation in progress is dispatched to the Agent): each reports
     unavailable, its first diagnostic's `ruleId` OPS-037, with one `request-ended` record holding
     `reason` unavailable and `ruleId` OPS-037, no `access-decision` record and no intent. Then A's
     completion with a result that succeeds, as `requests-limits-kept` completes a call: A's reply
     and end records as without the stop, then an `engine-stop` record with `data.phase` `stopped`
     and `data.cut` false; no intent.
   - `requests-stop-grace-period-ends` (`stop-grace-period-ends/`; OPS-037, OPS-012, IMP-007): a
     `stop` step with `graceEnded` true. Its reply, `{"stopped": true, "cut": [...]}`, holds A's
     result, which answers A's caller: as a `cancel` step's reply in
     `requests-keyed-hook-dispatch` (`isError` true, `_meta` outcome unknown and A's invocation),
     with an error in `structuredContent.diagnostics` whose `ruleId` is OPS-037. Its intent is A's
     cancellation, as that step's; its records, A's `invocation-ended` with `outcome` unknown and
     `ruleId` OPS-037, then an `engine-stop` with `data.phase` `stopped` and `data.cut` true. Then
     a `restore-state` step, as in `requests-restart-interrupted`: reply `{"restored": true}`, no
     intent, A's state unknown.

   The engine's own runner ends its runs in the shell, outside the core, so no step's intents
   hold them. Each case first fails at its first `stop` step, which the first build's core does
   not read; report that line. List both in `conformance/expected-failures.yaml`, in `id` order,
   with the reason "OPS-037, order 410 (DEC-176): the first build takes no stop input and stops
   serve at once; order 411 makes it pass and removes this entry". Declare both in
   `docs/work-orders/410-declared.yaml`, in [307's](307-declared.yaml) form, then run
   `python3 tools/promise_standing.py --write`.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
sed -n '/^## DEC-176, /,/^## DEC-177, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
cat spec/*.md spec/*.yaml spec/*/*.md spec/*/*.yaml | grep -ciE 'OPS-037|DL20|grace period'  # 0
grep -c '^- id: OPS-' spec/conventions/operations.yaml; grep -c '^- id: DL' spec/implementation-defined.yaml  # 31, then 17
k spec/conventions/operations.yaml 'under OPS-027 or when a call passes the time that DL10 allows it, is no answer.' 'otherwise unavailable where the access decision cannot be made under ACC-037,' 'Where the access decision cannot be made, nothing else about the request is examined or reported;' 'is reported only where no other reason applies.'  # 1 1 1 1
k spec/conventions/implicit.yaml 'An invocation that was in progress across a restart has the outcome unknown.'  # 1
k spec/conventions/records.yaml 'PRV-013).' 'Records held to be kept late are lost if the engine stops before a record can be kept;'  # 1 1
k spec/promises/providers.yaml "An Agent's run on that runner is not run again so: a further setup asks for it, under PRV-007."  # 1
k spec/implementation-defined.yaml 'No promise limits how long an invocation is in progress:' 'when the engine stops and when it starts again,' 'although the engine as a whole goes on.'  # 1 1 1
k spec/definitions.md 'be read; otherwise refused,' 'No access decision is made for a provider call (PRV-005).' 'or the state of the Conduits cannot be established.'  # 1 1 1
k spec/README.md 'Draft; 31 promises; vectors test slices of 30 promises' '522 in all: 226 of the layer `language`, 246 of the layer `convention` and 50 of the layer `interface`' 'Draft; 17 items'  # 1 1 1
k docs/implementation/SAFETY_CATALOGUE.md 'Promises read: 522; safety: 414;' 'effect 125,' 'convention 233, interface 45.' 'seven convention files: 129' 'convention layer anywhere: 233' 'Read: 32 promises. Source: [spec/conventions/operations.yaml]'  # 1 1 1 1 1 1
grep -A2 -E '^- id: (IMP-007|OPS-012|PRV-026)$' experiments/012-model-checking/account.yaml | grep -c '^  standing: text changed, not rerun$'  # 3
k experiments/012-model-checking/account.yaml 'R1 checks recording at creation and was not rerun against late recording.'  # 1
grep -c '"id": ' conformance/expected-failures.yaml  # 13
grep -c '"restore-state",' engine/crates/conformance/src/portable.rs  # 2
k conformance/README.md '`restore-state`, `list-tools`,'  # 1
```

Where a line prints otherwise, as where another order merged first, work from where it stands
and list it under `own_choices`.

## Scope

Change, as the decisions say: `spec/conventions/operations.yaml`, `implicit.yaml` and
`records.yaml`, `spec/promises/providers.yaml`, `spec/implementation-defined.yaml`,
`spec/definitions.md`, `spec/README.md`'s three counts and `spec/CHANGES.md`; the catalogue and
the account; `engine/CONFORMANCE.md`'s one row; `stop.json`, the two lines of `portable.rs` and
the one in `conformance/README.md`; the cases, their index entries and expected-failures entries;
`docs/work-orders/410-declared.yaml`; `engine/STANDING.md` by the tool alone. Not the decision
log (DEC-176 stays not done), OPS-027, TPL-009, `record.v1`, `IN_FORCE.md` (its "seventeen
implementation-defined entries" are those the rulings gave, and stay), `engine/CLI.md`,
`engine/README.md`, the profile, other `engine/` sources, `packages/` or `tools/`. Where a test
fails because the engine reads the changed spec, report it and change no engine file.

## Done when

Each line prints its value after this order, and a 0 or the old count before, with `j` and `k`
as above:

```sh
grep -c '^- id: OPS-' spec/conventions/operations.yaml; grep -c '^- id: DL' spec/implementation-defined.yaml  # 32, then 18
k spec/conventions/operations.yaml 'ends then with the outcome unknown' 'or when the grace period of a stop ends under OPS-037' 'where the engine is stopping and does not take the request' 'its result carries no diagnostic but OPS-037'  # 1 1 1 1
k spec/conventions/implicit.yaml 'ended unknown then, and is likewise not dispatched again'  # 1
k spec/conventions/records.yaml 'the engine is asked to stop, or stops (OPS-037).' 'are kept before the engine stops under OPS-037'  # 1 1
k spec/promises/providers.yaml 'A stop under OPS-037 ends the runs on that runner only once'  # 1
k spec/implementation-defined.yaml 'but OPS-037 at a stop of the engine:' 'which under OPS-037 is only once' 'A stop under OPS-037 is the whole engine' 'subject: The stop of the engine'  # 1 1 1 1
k spec/definitions.md '| grace period |' 'where a stopping engine does not take it (OPS-037);' 'nor for a request that a stopping engine does not take' 'or the engine is stopping.'  # 1 1 1 1
k spec/README.md 'Draft; 32 promises; vectors test slices of 30 promises' '523 in all: 226 of the layer `language`, 247 of the layer `convention` and 50 of the layer `interface`' 'Draft; 18 items'  # 1 1 1
git diff -U0 -- spec/CHANGES.md | grep -c '^+- .*(DEC-176)\.$'  # 4
k docs/implementation/SAFETY_CATALOGUE.md 'Promises read: 523; safety: 415;' 'effect 126,' 'convention 234, interface 45.' 'seven convention files: 130' 'convention layer anywhere: 234' 'Read: 33 promises.' '| OPS-037 | convention | must | effect |'  # 1 1 1 1 1 1 1
grep -A2 '^- id: OPS-037$' experiments/012-model-checking/account.yaml | grep -c '^  standing: not modelled$'  # 1
k experiments/012-model-checking/account.yaml 'Order 410 (DEC-176) cites the stop of OPS-037' 'Order 410 (DEC-176) adds the two records of the stop'  # 3 1
grep -c '"stop",' engine/crates/conformance/src/portable.rs  # 2
ls conformance/case-schema/inputs | grep -cx 'stop.json'  # 1
k conformance/README.md '`restore-state`, `stop`, `list-tools`,'  # 1
grep -c '^| DL20 |' engine/CONFORMANCE.md  # 1
cat conformance/expected-failures.yaml conformance/conventions/requests/index.yaml | grep -cE '"id": "requests-stop-(takes-no-new-request|grace-period-ends)"'  # 4
grep -cE '^\| OPS-037 \| convention \| must \| 2 \| 2 \| outside \| none \| expected failure \|$' engine/STANDING.md  # 1
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-stop- 2>&1 | grep -cE "^EXPECTED FAILURE requests-stop-(takes-no-new-request|grace-period-ends) ")  # 2
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
r=conformance/conventions/requests; n="$r/stop-takes-no-new-request $r/stop-grace-period-ends conformance/case-schema/inputs/stop.json docs/work-orders/410-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/410-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff -U0 -- spec | grep '^+' | grep -ciE 'claude|codex|anthropic|openai|chatgpt'  # 0: no vendor named (DEC-105)
git diff --check  # prints nothing
(cd engine && cargo fmt --check -p conduit-conformance); echo "fmt $?"  # fmt 0
(cd engine && cargo test --offline --release -p conduit-conformance --lib portable)  # the portable checks
(cd engine && cargo test --offline --release -p conduit-loader --test rules)  # reads implementation-defined.yaml
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case's first failing step and every check's last line.

## What follows

Order 411 builds OPS-037 in `serve`, with its grace period, `engine-stop` in DL8's vocabulary and
the profile, DL20's row among the implemented choices, both cases passing, a shell test of an
Agent's turn making its own calls during the grace period, and the stop's text in the shell,
`engine/CLI.md` and `engine/README.md`; then the restart script retires (DEC-176) and a launchd
job runs serve (DEC-178).

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start. Built and read
on the team seats (DEC-179, DEC-180).

**Worker-4's turns**, on the Business seat. The first applied decisions 1 to 6 and stopped at
decision 7: its restore step had to assert state, which the portable runner refuses outside a
state-building step. The coordinator ruled the precedent the worker found: the two new cases are
listed in `conformance/internal-cases.yaml` as `requests-restart-interrupted` is. The second turn
finished decision 7; both cases first fail at step 15, as the order expects, until order 411
builds the stop.

**Two blind reads.** The Codex verifier, `root-verifier-4.turn.v1`, asked for one must: IMP-007's
new sentence ended every cut invocation unknown, against OPS-037's setup and retirement
exceptions. The Claude reader, `root-reader-a.turn.v1`, on the Team seat, asked for a must (the
grace-period step forbade the request-ended record every end of a tool call writes), two shoulds
and a nit. One fix turn by worker-4 answered all five, with a regression that failed first. The
gate's run on the squash is the coordinator's, at the merge.
