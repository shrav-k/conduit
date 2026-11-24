# Work order 307: a usage report carries paid usage, and a status keeps each usage signal (DEC-096, the usage guard's first spec order)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the two cases this order adds, one passing and one an expected failure. It needs no network
and changes no Rust source; if a crate is missing from the cache, fetch nothing and report it.
Run one cargo command at a time, and not `tools/gate.py`. Never print a whole large file, case,
diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in
nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public:
no absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** DEC-093: "flags for allowing for usage credits etc, this should be a core
feature of conduit". DEC-094: "Conduit specifies what must be reported and what contracts must be
upheld. The engine then trusts this reporting".
[DEC-096](../decisions/log.md#dec-096-2026-11-23-a-usage-report-may-carry-paid-usage-state),
"Accept (Recommended)": a report may carry whether paid usage is possible and in use, each
mapped only from a field the client gave, with no amount, balance, limit, plan or currency.
DEC-120: "I don't want to modify how the models operate just police their usage", so this order
reads only what a client already reports, in the core schemas the engine checks (DEC-116). Today
a report has no paid state, needs every window's percentage and IDs of letters alone, and a later
null `usageAllowed` in `newest` hides an earlier `false`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
each signal names the record source, instance and invocation that reported it; 6, an inspection
starts no call to fill it, and nothing depends on it (IFC-053); at risk, 4, no vendor assumed:
every new member is optional or null where a client gives no field, and the spec names none
(DEC-105).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: the IETF's draft RateLimit fields report each named quota's window,
remaining units and reset; a Kubernetes Node keeps each condition apart with its own heartbeat
time, and a patch omitting one leaves it standing, as a null leaves a signal. Not taken: amounts
(DEC-096); a signal per window; clearing a signal by reset or age (its reader judges, DEC-094);
totals.

**When it runs.** On the decision log's branch (DEC-082 to DEC-124 on top of `main`), or on
`main` once it merges. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The words, the cases' names and these choices are the
coordinator's, open to the owner's strike (DEC-029): `paidUsage`'s reset time, its three members
all required; null percentages; digits and hyphens in window IDs; the count of left-out windows,
at most 64; signals no time, purge or later null clears, never `unknown` (OBS-002); nulls in
`newest`. This first guard order writes the limits the others read and waits on no open answer.

**Shared files.** Orders 298, 299 and 302 to 305 change some of the same files: whichever merges
later rebases, recounts and reruns `promise_standing.py --write`.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width. In the YAML, indentation is the file's and a `#` line
stands for what stays as it is, or the lines it names.

1. **`limit-report.v1`** (`spec/core/results.yaml`):

   ```yaml
   spec:
     type: object
     properties:
       # client, method, observedAt and usageAllowed, then:
       paidUsage:
         type: object
         properties:
           possible: {type: [boolean, 'null']}
           inUse: {type: [boolean, 'null']}
           resetsAt: {type: [string, 'null'], format: date-time}
         required: [possible, inUse, resetsAt]
         additionalProperties: false
       # noWindows, then:
       windowsOmitted: {type: integer, minimum: 1, maximum: 64}
       windows:
         # each item as it is, but for two members:
             id: {type: string, pattern: '^[a-z]{1,64}/[a-z0-9][a-z0-9-]{0,63}(/[a-z0-9][a-z0-9-]{0,63})?$'}
             usedPercent: {type: [number, 'null'], minimum: 0}
     # required and additionalProperties, then:
     oneOf:
     - properties:
         windows: {maxItems: 0}
       oneOf:
       - required: [noWindows]
       - required: [windowsOmitted]
     # the second branch
   ```

   Its description: after "says neither." insert "`paidUsage`, where given, says whether paid
   usage beyond the included usage is possible and in use, each true, false or null, and when
   that state resets, or null; it holds no amount."; "its used percentage," becomes "its used
   percentage or null,"; "A report with no windows gives the reason in `noWindows`." becomes
   "`windowsOmitted` counts the windows the client gave that the package has no fixed ID for,
   which `windows` leaves out. A report with no windows gives the reason in `noWindows` or, where
   it left out every window the client gave, `windowsOmitted`, never both."
2. **`limit-status.v1`**, the same file:

   ```yaml
   spec:
     $defs:
       # instance, then:
       signal:
         type: object
         properties:
           value: {type: boolean}
           # newest's observedAt, source, instance and invocation lines
         required: [value, observedAt, source, instance, invocation]
         additionalProperties: false
       paidSignal:
         # signal's lines, with resetsAt: {type: [string, 'null'], format: date-time} after
         # value in properties and in required
     type: object
     properties:
       # state and newest, then:
       signals:
         type: object
         properties:
           usageAllowed: {$ref: '#/$defs/signal'}
           paidUsage:
             type: object
             properties:
               possible: {$ref: '#/$defs/paidSignal'}
               inUse: {$ref: '#/$defs/paidSignal'}
             minProperties: 1
             additionalProperties: false
         minProperties: 1
         additionalProperties: false
       # windows
     # required, additionalProperties, if and then, then:
     else:
       # properties, then:
       not:
         anyOf:
         - required: [newest]
         - required: [signals]
   ```

   `newest` takes `limit-report.v1`'s `paidUsage` block after `usageAllowed` and its
   `windowsOmitted` line after `noWindows`, neither required, and each `windows` item its new `id`
   and `usedPercent`. Its description: "included usage and any reason it named no windows."
   becomes "included usage, and any paid usage, reason it named no windows and count of windows
   left out that it gave."; after "at or after its reset time." insert "`signals` gives, for
   `usageAllowed` and paid usage's `possible` and `inUse`, the newest value other than null a
   report gave, with its time, record source, instance and invocation, and for paid usage that
   report's reset time; a signal no report gave other than null is absent, and so is `signals`
   where none was given."; "with no windows and no `newest`." becomes "with no windows, no
   `newest` and no `signals`."
3. **IFC-052's observable** (`spec/interface/implementation.yaml`). After "never text from the
   client." insert: "`paidUsage` says whether the client reported paid usage beyond the included
   usage as possible and in use, and when that state resets: each value maps a field the client
   gave, and one that no field gives is null, never inferred. A window whose percentage the client
   did not give has a null `usedPercent`, and one the package has no fixed ID for is left out of
   `windows` and counted in `windowsOmitted`." After "billing value in any string" insert ", and
   nothing in `paidUsage` names an amount, balance, limit, plan or currency".
4. **IFC-053.** Its `then`, after "in ascending order of Unicode code points.": "For each of three
   signals, `usageAllowed` and `paidUsage`'s `possible` and `inUse`, `signals` gives the newest
   observation by the member's `observedAt` among the members that gave it a value other than
   null: that value, the member's `observedAt`, for `possible` and `inUse` the member's
   `paidUsage.resetsAt`, and the same source as a window."; its last sentence ends "with no
   window, no signal and no time." Its observable: "with its `usageAllowed` and any `noWindows`."
   becomes "with its `usageAllowed`, null or not, and any `paidUsage`, `noWindows` and
   `windowsOmitted` it gave."; "For `newest` and for each window," becomes "For `newest`, each
   window and each signal,"; after "No purge shortens it, as the engine's own state." insert "A
   member that gives a signal null, or no `paidUsage`, leaves that signal as it stands, and no
   time or purge changes it: only a newer member that gives it a value other than null does. A
   signal that no member gave other than null is absent, and `signals` is absent where none is
   given." **OBS-002's observable** (`spec/conventions/observation.yaml`), after "in place of
   OBS-003's condition.": "Each usage signal in `limits` carries its time and source in the same
   way and keeps its value, never reading `unknown` under OBS-003: it is shown with that time, not
   as current, and its reader judges whether it still holds."
5. **`inspect.v1`'s description** (`spec/core/operations.yaml`): after "in `limit-status.v1` form
   (IFC-053)." insert "In `limits`, `signals` holds the newest value other than null of each usage
   signal those calls reported."
6. **CHANGES and counts.** In `spec/CHANGES.md`, under `## [Unreleased]`, after its last item:

   ```text
   - Changed: IFC-052 and `limit-report.v1`, a report may carry `paidUsage`, with no amount, a null `usedPercent`, `windowsOmitted` and window IDs with digits and hyphens (DEC-094, DEC-096).
   - Changed: IFC-053, OBS-002, `limit-status.v1` and `inspect.v1`'s description, a status keeps in `signals` each usage signal's newest value other than null (DEC-094, DEC-096).
   ```

   No count moves, and the account's rows stay (OBS-002's is `static`, with no digest).
7. **Two cases** in `conformance/conventions/requests/`, each in its `index.yaml` in `id` order,
   layers `convention` and `interface`, profile `first-build`, from `{"recipe":"empty"}`, with
   neutral names such as `example-client` and `example/all/300m`:
   - `requests-limits-paid-usage` (`limits-paid-usage/`; IFC-052, REC-002), on the files and in
     the step form of `requests-limits-kept`: three plain results carry the key, each turn
     succeeding with its reply as without the key. P: `usageAllowed` false, `paidUsage`
     `{possible: true, inUse: null, resetsAt: <a time>}`, `windowsOmitted` 1, windows
     `example/all/300m` (300 minutes, `usedPercent` null, `reached`) and
     `example/model-a/10080m` (10080 minutes, 100, `reached`, scope `example-model-a`): its end
     record holds P unchanged in `limits`, with no `limitsDropped`. Q: `usageAllowed` null,
     `paidUsage` all null, no window, `windowsOmitted` 2, no `noWindows`: the same. R: P with
     `amount` 5 in `paidUsage`: no `limits`; `limitsDropped` with `ruleId` IFC-052, `level`
     `warning`, at `/_meta/io.github.shrav-k.conduit~1limits/paidUsage`, `keywordLocation`
     `/properties/paidUsage/additionalProperties`. Write it before decision 1: it fails at P's
     record. After decision 1 it passes on the unchanged engine, which checks it against the
     spec's own schema; if not, stop. Report both lines.
   - `requests-limits-signals` (`limits-signals/`; IFC-053), on the files of
     `requests-login-resource-one-at-a-time`, with `requests-limits-latest`'s steps through
     opening `call-b` and inspecting `a` before any turn (`{"state":"unknown","windows":[]}`, no
     `signals`). Turns, every reset after the last step: on `a`, S1 at T1, `usageAllowed` false,
     `paidUsage` `{possible: true, inUse: null, resetsAt: X}`, `example/all/300m` with
     `usedPercent` null; on `b`, S2 at T2 after T1, `usageAllowed` true, no `paidUsage`,
     `windowsOmitted` 1, `example/all/300m` at 20; on `a`, S3 at T3 after T2, `usageAllowed`
     null, `paidUsage` `{possible: null, inUse: false, resetsAt: null}`, no window, `noWindows`
     `no-event`. Then `a` shows `newest` S3's with its `paidUsage`; in `signals`, `usageAllowed`
     false and `possible` true with `resetsAt` X from S1, `inUse` false from S3; the window from
     S1. `b`: `newest` S2's with `windowsOmitted` 1, `usageAllowed` true, no `signals.paidUsage`.
     `login`: `newest` S3's, `usageAllowed` true from S2, `possible` true from S1, `inUse` false
     from S3, the window at 20 from S2. Each signal names its `observedAt`, `source`, `instance`
     and `invocation`. After decision 1, and before its expected-failures entry is in, write it
     without `signals`, `paidUsage` and `windowsOmitted` in the statuses: it passes on the
     unchanged engine; report that line.

   List `requests-limits-signals` alone in `conformance/expected-failures.yaml`, in `id` order,
   with the reason "IFC-053, order 307: the first build keeps no usage signal, paid usage or
   left-out count in a status; the engine's limits order makes it pass". `required_promises`,
   where the runner allows it, names only what the unchanged engine cites. Declare both in
   `docs/work-orders/307-declared.yaml`, in [297's](297-declared.yaml) form, then run
   `python3 tools/promise_standing.py --write`.

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
sed -n '/^## DEC-096, /,/^## DEC-097, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
cat spec/core/results.yaml spec/core/operations.yaml spec/interface/implementation.yaml spec/conventions/observation.yaml | grep -cE 'paidUsage|windowsOmitted|signals'  # 0
k spec/core/results.yaml '[a-z]{1,64}/[a-z]{1,64}(/[a-z]{1,64})?' 'usedPercent: {type: number, minimum: 0}' 'says neither.' 'its used percentage,' 'A report with no windows gives the reason in `noWindows`.' 'included usage and any reason it named no windows.' 'at or after its reset time.' 'with no windows and no `newest`.'  # 2 2 1 1 1 1 1 1
k spec/interface/implementation.yaml 'never text from the client.' 'billing value in any string.' 'code points. Where no such member' 'with its `usageAllowed` and any `noWindows`.' 'For `newest` and for each window,' 'No purge shortens it, as the engine'  # 1 1 1 1 1 1
j spec/conventions/observation.yaml "in place of OBS-003's condition."  # 1
j spec/core/operations.yaml 'in `limit-status.v1` form (IFC-053).'  # 1
grep -cE '^\| IFC-05[23] \| interface \| must \| 1 \| 0 \|' engine/STANDING.md  # 2
grep -c '^- id: IFC-' experiments/012-model-checking/account.yaml  # 0
```

Where a line moved, work from where it stands and list it under `own_choices`.

## Scope

Change, as the decisions say: in `spec/core/results.yaml`, `limit-report.v1` and
`limit-status.v1`; `inspect.v1`'s description; IFC-052, IFC-053 and OBS-002; `spec/CHANGES.md`;
the two cases, their index entries and the one expected-failures entry;
`docs/work-orders/307-declared.yaml`; `engine/STANDING.md` by the tool alone. Not the decision
log (DEC-094 and DEC-096 stay not done), `spec/definitions.md`, either README, `IN_FORCE.md`,
`experiments/`, `engine/` sources, `packages/` or `tools/`. Where a test fails because the engine
reads the changed spec, report it and change no engine file.

## Done when

Each line prints otherwise before this order and passes after, with `j` and `k` as above:

```sh
sed -n '/^  name: limit-report.v1$/,/^---$/p' spec/core/results.yaml | grep -cE '^    (paidUsage:$|windowsOmitted: |- required: \[windowsOmitted\]$)'  # 3; 0 before
sed -n '/^  name: limit-status.v1$/,/^---$/p' spec/core/results.yaml | grep -cE '^    (signal|paidSignal|signals):$|^        (paidUsage:$|windowsOmitted: )|^      - required: \[signals\]$'  # 7; 0 before
k spec/core/results.yaml '[a-z]{1,64}/[a-z0-9][a-z0-9-]{0,63}(/[a-z0-9][a-z0-9-]{0,63})?' "usedPercent: {type: [number, 'null'], minimum: 0}" 'it holds no amount.' 'its used percentage or null,' 'never both.' 'left out that it gave.' 'where none was given.' 'no `newest` and no `signals`.'  # 2 2 1 1 1 1 1 1; 0s before
k spec/interface/implementation.yaml 'no field gives is null, never inferred.' 'nothing in `paidUsage` names an amount' 'gave it a value other than null:' 'no window, no signal and no time.' '`noWindows` and `windowsOmitted` it gave.' 'each window and each signal,' 'no time or purge changes it:'  # 1 1 1 1 1 1 1; 0s before
j spec/conventions/observation.yaml 'its reader judges whether it still holds.'  # 1; 0 before
j spec/core/operations.yaml 'In `limits`, `signals` holds the newest value other than null'  # 1; 0 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- .*(DEC-094, DEC-096)\.$'  # 2; 0 before
cat conformance/expected-failures.yaml conformance/conventions/requests/index.yaml | grep -cE '"id": "requests-limits-(paid-usage|signals)"'  # 3; 0 before
grep -cE '^\| IFC-05(2 \| interface \| must \| 2 \| 0|3 \| interface \| must \| 2 \| 1) \|' engine/STANDING.md  # 2; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-limits-paid-usage 2>&1 | grep -c "^PASS requests-limits-paid-usage ")  # 1; 0 before
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-limits-signals 2>&1 | grep -c "^EXPECTED FAILURE requests-limits-signals ")  # 1; 0 before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
r=conformance/conventions/requests; n="$r/limits-paid-usage $r/limits-signals docs/work-orders/307-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/307-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff -U0 -- spec | grep '^+' | grep -ciE 'claude|codex|anthropic|openai|chatgpt'  # 0: no vendor named (DEC-105)
git diff --check  # prints nothing
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/core
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case line asked for above and every check's last line.

## What follows

The guard's other spec orders, whose admission hook reads `signals`; the engine's limits order,
removing this order's entry; the packages' mappings (DEC-124). No team-seat Agent takes a turn
until the guard merges (DEC-102).

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on `main` with the decision log
through DEC-157: the usage guard's first spec order (DEC-166 puts the guard's orders first).
Written, built, read and fixed in stage 3 of experiment 028; it is no order of stage 3.

**Worker-1's turn** (15:44 to 16:00) added paid usage, null percentages and left-out windows to
`limit-report.v1` and IFC-052, and the newest value of each signal to `limit-status.v1`; the paid
usage case passes on today's engine and the signals case is the expected failure.

**Two blind reads** (16:05 to 16:35). The Codex verifier, `root-verifier-1.turn.v1`, passed it
with no finding; the Claude reader (`reader-1`) had no must and two shoulds (what `possible` and
`inUse` mean; what a null `usedPercent` does to a window's earlier number). One fix turn by
worker-1 applied them. The gate's run on the squash is the coordinator's, at the merge.
