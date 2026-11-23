# Work order 284: a call's usage limits in its end record, the newest per window in its Agent's and login Resource's status (DEC-077, the spec's first part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the two cases this order adds, both expected failures. It needs no network and changes no
engine code; if a crate is missing from the cache, fetch nothing and report it. Run one cargo
command at a time, and not `tools/gate.py`. Never print a whole large file, case, diff or list:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user, machine or account name, or private repository name.

## What and why

**The owner's answer** ([DEC-077](../decisions/log.md#dec-077-2026-11-22-usage-and-limits-advertised-in-v0)):
"an agent must keep track of and advertise its usage, its a core feature of conduit"; then
"Design B in v0 (Recommended)". The spec comes in two parts, one turn each: this one writes a
call's report, its end record and the status; the second, what a Type states (What follows).

**Today.** IFC-051 keeps a call's `usage` in its end record; a limit appears only as order 260's
`limit` in a failed turn's result. OBS-002 reports every observation as a condition; ruling 68 M1
says numbers that are state "come later". A vendor's window counts use outside Conduit too, so
Conduit passes on the client's report.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7, each
window names who reported it; 3, each report is kept once, in its end record, and the statuses
read it; 4, any client's windows; 6, an inspection starts no call to fill them. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: GitHub's REST API sends `x-ratelimit-remaining` and `x-ratelimit-reset` on
each response; a Kubernetes status keeps an observation with its time. Codex's
`account/rateLimits/read` and Claude Code's `rate_limit_event` are the clients' own reports;
REC-002 and 68 M1 make the record Conduit's. Not taken: totals or a hold in the engine, Claude's
`/usage`, the metrics batch now, limits relayed in the caller's reply, and DEC-066's reported
conditions (a number and its window fit no condition).

**When it runs.** On `main` from `c4fa2197` (#559). Through Conduit: one Codex worker turn, a
blind Codex verifier and Claude reader, one fix turn, the gate (DEC-046, DEC-033). By ruling 89,
each decision says what must hold; how is yours. The split, words, schemas and cases are the
coordinator's, open to the owner's strike. Whichever order merges later rewrites
`engine/STANDING.md` by the tool.

**Stop rule** (the coordinator's D1). Stop, change nothing further, and report only for a
contradiction between promises (give both), a step that needs an engine change, or a file
outside Scope. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, beside any choice no decision governs.

## The decisions

Wrap each insertion at its file's width.

1. **IFC-052,** in `spec/interface/implementation.yaml` after IFC-051: IFC-051's layer, level
   and `when`, for the key `io.github.shrav-k.conduit/limits`. Its `then`:

   ```text
   Where its value conforms to the core schema `limit-report.v1` and names no window `id` twice, the record of the invocation's end holds that value, unchanged, in `limits`. Where it does not, that record holds no part of it, and holds in `limitsDropped` one diagnostic whose `level` is `warning` and whose `ruleId` is the ID of this promise. Nothing else about the call changes.
   ```

   Its `observable` says: the value is the limits the call's client reported, and when, and says
   why where it has no window; any such result may carry it, failed or unknown among them; the
   implementation answers for it and invents nothing; an `id` is its package's fixed word, never
   client text; no string names an account, plan, credits or billing value; the engine checks
   only the shape and that no `id` repeats, and keeps it as received; the warning follows
   IFC-051's and quotes no value, for a repeated `id` at `windows` with no `properties`; outcome,
   result, other records and decisions are as without the key; the consumer's result does not
   carry it, so no caller is relayed it; inspection shows it (IFC-053); the record holds it
   whatever `records` states (CTR-004); and IFC-051's SEC-015 sentence.
2. **IFC-053,** after it, of the same layer and level. `when`: "An Agent, or a Resource whose
   Type states `signIn` (CTR-013), is inspected." `then`:

   ```text
   Its status holds `limits`, in the form of the core schema `limit-status.v1`. For each window `id` that the limits members of its own calls' end records have named (IFC-052), and, for a login Resource, also those of its Agents' calls (CTR-013), it gives the newest observation by the member's `observedAt`: the window's values as reported, its `state`, the member's `observedAt`, and as its source the record's `source`, the instance whose call it was and the invocation. Where no such member exists, `state` is `unknown`, with no window and no time.
   ```

   Its `observable` says: `newest` is the newest member, with its `usageAllowed` and any
   `noWindows`; of two with one `observedAt`, the one kept later stands; a window whose
   `resetsAt` is not after the inspection's time reads `unknown`, keeping its last values and
   time, in place of OBS-003's condition (OBS-002), until a newer member; no purge shortens it,
   as the engine's own state; any other object shows none; the engine sums nothing and compares
   only times; no outcome, access, dispatch, hold or lifecycle state depends on it, and an
   inspection starts no call to fill it (OBS-006).
3. **IFC-051's last sentence** becomes: "A call that ends on its client's usage limit is
   recorded as any other ending: the limit's class, window and time of reset are the
   implementation's own report in its result, and the limits its client saw are the member of
   IFC-052; neither is part of this value."
4. **REC-002:** "holds the usage member that its result carried, or the warning that dropped one,
   under IFC-051," becomes "holds the usage member and the limits member that its result carried,
   or the warning that dropped each, under IFC-051 and IFC-052,". **OBS-002's observable** gains,
   after "under REC-005.": "`limits`, under IFC-053, is the one observation not reported as a
   condition: each window, and its newest report, carries its time in `observedAt` and its source
   in `source`, with the instance and invocation that reported it, and one no longer current
   reads `unknown` there, in place of OBS-003's condition."
5. **The schemas,** in `spec/core/results.yaml` after `usage.v1`, described in its style, each
   naming its promise; no object admits other members.
   - `limit-report.v1`: required `client` (as `usage.v1`'s), `method` (the client's call or
     event, 1 to 128 characters), `observedAt` (date-time), `usageAllowed` (true, false or null:
     whether the vendor allows included usage) and `windows` (at most 16); `noWindows`
     (`no-event`, `read-failed` or `read-timed-out`) present exactly where `windows` is empty
     (`oneOf`). Each window requires `id` (two or three lowercase words, up to 64 characters
     each, joined by `/`, as `codex/primary`), `windowMinutes` (1 or more, or null),
     `usedPercent` (0 or more, unbounded), `resetsAt` (date-time or null), `state` (`allowed`,
     `warning` or `reached`) and `scope` (the model it limits, or null).
   - `limit-status.v1`: required `state` (`observed` or `unknown`) and `windows` (empty where
     `unknown`); `newest` exactly where `observed` (`if`/`then`), requiring `observedAt`,
     `source` (as `record.v1`'s), `instance` (`object.v1`), `invocation` (`id.v1`) and
     `usageAllowed`, with optional `noWindows`. Each window requires the report's window
     members, `state` also allowing `unknown`, and `newest`'s first four.
   - `record.v1`'s `data.properties` gain, after `usageDropped`, `limits` and `limitsDropped`,
     in `usage`'s and `usageDropped`'s form, of `limit-report.v1` and `diagnostic.v1`, and its
     description a sentence naming IFC-052; the file's head lists both schemas.
   - `inspect.v1`'s `status.properties` gain `limits` after `settings`, a block `$ref` to
     `limit-status.v1`, and its description, after "and no purge shortens it.", a sentence of
     IFC-053's `then`.
   - `spec/core/conduit.yaml` exports both after `usage.v1`; `spec/core/README.md` gives each a
     row after `usage.v1`'s, naming its promise. Both READMEs name them, and "a call's limits, an
     instance's limits", where they name `usage.v1` and "a call's usage", and each of their
     counts gains two; on `c4fa2197` they become 147 documents, 77 Schema contracts, 14 in
     `results.yaml`, 48 promises in `implementation.yaml` with slices of 26 tested, and 520 in
     all, 50 of them `interface`.
   - `spec/definitions.md`, row "Conduit's keys", after "never relayed (IFC-051).": "It may
     also carry `limits`, the usage limits its client saw, kept in that record, shown in the
     status of IFC-053 and never relayed (IFC-052)."
6. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item:

   ```text
   - Added: IFC-052, a result may carry the usage limits its client saw in `_meta`, kept in the end record or dropped with a warning; the consumer's result does not carry it (DEC-077).
   - Added: IFC-053, the status of an Agent or a login Resource shows the newest of each window that its calls, or its Agents', reported, `unknown` after the reset; the engine sums and acts on none (DEC-077).
   - Added: `limit-report.v1` and `limit-status.v1`, the forms of IFC-052 and IFC-053, `record.v1`'s `limits` and `limitsDropped`, and the status's `limits` (DEC-077).
   - Changed: IFC-051, a usage-limit ending's limits are IFC-052's member, no part of usage (DEC-077).
   - Changed: REC-002, the end record also holds the limits member or its warning, whatever `records` states (DEC-077).
   - Changed: OBS-002, IFC-053's `limits` is the one observation not reported as a condition (DEC-077).
   ```

7. **IN_FORCE.md, topic 7.** The line ending "kept in the record of its end and deciding nothing
   (IFC-051)." gains, on the same line: "A second is in spec/ ahead of the batch, as DEC-077
   amends 68 M1 for it: the usage limits each call's client saw, kept in its end record and, the
   newest of each window, in the status of the instance called and of an Agent's login Resource,
   deciding nothing (IFC-052, IFC-053)." No bullet is added.
8. **Two cases** in `conformance/conventions/requests/`, each in its `index.yaml`, layers
   `convention` and `interface`, profile `first-build`, from `{"recipe":"empty"}`:
   - `requests-limits-kept` (`limits-kept/`; IFC-052, REC-002), on the files and in the step
     form of `requests-usage-kept`. L, a full report: `usageAllowed` true, `example/short` (300
     minutes, 62, `allowed`) and `example/week` (10080 minutes, 101.5, `reached`, scope
     `example-model-a`). Four plain results carry the key: L beside usage U, complete:
     succeeded, the reply as without the key, and the end record holds L in `limits` and U in
     `usage`; L with no window, `usageAllowed` null and `noWindows` `read-failed`, an error:
     unknown (IFC-010), the record holding it; L with `usedPercent` -1 in its first window:
     succeeded, no `limits`, `limitsDropped` with `ruleId` `IFC-052`, `level` `warning`, at
     `.../windows/0/usedPercent`; L with its first window twice: the same, at `.../windows`.
   - `requests-limits-latest` (`limits-latest/`; IFC-053, OBS-002), on the files of
     `requests-login-resource-one-at-a-time` (Agents `a` and `b`, login Resource `login`), with
     its first ten steps, through opening `call-b`, inspecting `a` and `b` through root's
     `workers` and `login` through `logins` (ACC-033 rows 6A and 6). Before any turn, `a`'s
     `limits` is `{"state":"unknown","windows":[]}`. Turns carry: on `a`, R1 at T1,
     `example/short` 40 (resetting after the last step) and `example/week` 10 (resetting at T5);
     on `b`, R2 at T2 after T1, `example/short` 55; on `a`, R0 at T0 before T1, `example/short`
     20. Before T5, `a` shows 40 and 10 from its first turn, `newest` R1's; `b` shows 55 alone;
     `login` shows 55 from `b` and 10 from `a`, `newest` R2's; each window with `observedAt`,
     `source`, `instance` and `invocation`. After T5, `example/week` reads `unknown` on `a` and
     `login`, still 10 at T1.

   Write each first without the new assertions (records' `limits` and `limitsDropped`, the
   status's `limits`): it passes on the unchanged engine; report that line. Then add them, and
   list both in `conformance/expected-failures.yaml`, in `id` order, each with the reason
   "<its promise>: the first build does not keep or show limits; DEC-077's engine order makes
   it pass and removes this entry". `required_promises`, where the runner allows it, names only
   what the unchanged engine cites. Declare both in `docs/work-orders/284-declared.yaml`, in
   [271's](271-declared.yaml) form, then run `python3 tools/promise_standing.py --write`.

## What to read

The promises named, REC-005, CTR-004 and SEC-015; `usage.v1`, `record.v1`, `inspect.v1`;
CONTRIBUTING rule 8; "Expected failures" in `conformance/README.md`; the two model cases.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor c4fa2197 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
sed -n '/^## DEC-077/,/^## DEC-078/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
git grep -l -e 'IFC-05[23]' -e 'conduit/limits' -e 'limit-report' -e 'limit-status' -- spec conformance engine tools | wc -l | tr -d ' '
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/interface/implementation.yaml 'and are no part of this value.'
j spec/conventions/records.yaml 'or the warning that dropped one, under IFC-051, whatever `records` states.'
j spec/conventions/observation.yaml 'as it names the `source` of a record under REC-005.'
j spec/core/README.md 'exports 145 documents, 75 of them Schema contracts,'
grep -c '"id": ' conformance/expected-failures.yaml
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`, `1`, `0`, then `1` four times, `3` and `engine 0`; report any other
line. Where another order moved a count (a README `0`, or not `3`), add two to `main`'s in the
decisions and Done when.

## Scope

Change, as the decisions say, `spec/interface/implementation.yaml`,
`spec/conventions/records.yaml` and `observation.yaml`, `spec/core/results.yaml`, `conduit.yaml`,
`operations.yaml` (`inspect.v1` alone) and `README.md`, `spec/definitions.md` (one row),
`spec/README.md` (Contents and totals), `spec/CHANGES.md` and `IN_FORCE.md`; create the two case
directories and `docs/work-orders/284-declared.yaml`; add two entries each to the cases'
`index.yaml` and `conformance/expected-failures.yaml`; `engine/STANDING.md` only by the tool.
Nothing else: not the log (DEC-077 stays not done), the engine, `packages/`, `tools/` or
`experiments/` (experiment 012 takes no `interface` row).

## Done when

Each line prints otherwise on `main` today and passes after:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/interface/implementation.yaml 'names no window `id` twice' 'the core schema `limit-status.v1`.' 'IFC-052; neither is part of this value.'  # 1 1 1; 0 0 0 today
j spec/conventions/records.yaml 'or the warning that dropped each, under IFC-051 and IFC-052,'  # 1; 0 today
j spec/conventions/observation.yaml 'is the one observation not reported as a condition'  # 1; 0 today
grep -cE '^  name: limit-(report|status)\.v1$' spec/core/results.yaml  # 2; 0 today
j spec/core/results.yaml 'limitsDropped: {$ref: github.com/shrav-k/conduit/core#diagnostic.v1}'  # 1; 0 today
grep -cE '^  - limit-(report|status)\.v1$' spec/core/conduit.yaml  # 2; 0 today
sed -n '/^  name: inspect.v1$/,/^---$/p' spec/core/operations.yaml | grep -c 'core#limit-status.v1'  # 1; 0 today
k spec/README.md 'slices of 26 promises' '520 in all' 'Draft; 14 contracts |' 'Draft; 147 documents'  # 1 1 1 1; 0 0 0 0 today
grep -cE '^\| `limit-(report|status)\.v1` \| Schema \|' spec/core/README.md  # 2; 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-077)\.$'  # 6; 0 today
j IN_FORCE.md 'as DEC-077 amends 68 M1'  # 1; 0 today
grep -cE '"id": "requests-limits-(kept|latest)"' conformance/expected-failures.yaml  # 2; 0 today
grep -cE '^\| IFC-05[23] \| interface \| must \|.*\| expected failure \|$' engine/STANDING.md  # 2; 0 today
for c in requests-limits-kept requests-limits-latest; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$c" 2>&1 | grep -c "^EXPECTED FAILURE $c "); done  # 1 then 1; 0 and 0 today
```

Guards, which pass today and must still pass; the engine's suite once, last:

```sh
export PYTHONDONTWRITEBYTECODE=1
n="conformance/conventions/requests/limits-kept conformance/conventions/requests/limits-latest docs/work-orders/284-declared.yaml"
git add -N $n
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # head equals base; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/284-declared.yaml; echo "exit $?"  # ends 0 undeclared; 0 errors. then exit 0
test "$(git show HEAD:IN_FORCE.md | grep -cE '^\s*[-*] ')" -eq "$(grep -cE '^\s*[-*] ' IN_FORCE.md)"; echo "bullets $?"  # bullets 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(interface/implementation\.yaml|conventions/(records|observation)\.yaml|definitions\.md|core/(results|conduit|operations)\.yaml|core/README\.md|README\.md|CHANGES\.md)|IN_FORCE\.md|engine/STANDING\.md|conformance/(expected-failures\.yaml|conventions/requests/(index\.yaml|limits-(kept|latest)/[^/]+))|docs/work-orders/284-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes, the two new cases expected failures
git reset -q -- $n  # stage nothing
```

Report each changed sentence, each case's lines before and after its new assertions, and every
check's last line.

## What follows

DEC-077's other orders, in sequence:

- **The spec's second part,** one turn: a Type with a `startsModelWork` operation, or stating
  `signIn`, states `reports` (`usage` and `limits`: `turns`, `read` or `none`) or is rejected at
  install; `limits: read` answers `limits.v1`, a read with no effect that starts no model work
  and never changes the login; a Claude login answers through its status; the status gains
  `not-reported`. It touches about 80 conformance files.
- **The engine,** one route with DEC-066's engine order (a `_meta` report kept with source and
  time, or dropped with a warning), removing both expected failures.
- **agent-codex** (after DEC-077's probe: a read after each turn and `limits.v1`, refresh
  blocked, amending ruling 86 point 5) and **agent-claude-code** (every `rate_limit_event`):
  package releases, at a stage boundary (order 277 item 8, DEC-070). Then **the CLI**, a limits
  table, and **the dispatcher**, pause and reopen from the status.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-1's turn** (13:23 to 13:35) wrote the owner's DEC-077 in spec/, Design B's U1: every
turn's result reports the usage limits its client saw; the engine keeps the newest report for
each window on the Agent and on its login Resource, marks a window unknown after its reset, adds
nothing up and acts on none of it; a login Resource may answer `limits.v1`, which starts no model
work and never refreshes the login; the end record keeps the report whatever the Type's records
setting says; nothing reaches the Agent's model. Both new cases fail at the intended fields until
the engine order.

**Two blind reads at once** (13:35 to 13:42): `turn-verifier-1.turn.v1` found three musts and a
should, the Claude reader a must, three shoulds and a nit. One fix turn (13:43 to 13:47) applied
all nine, its readings listed as its own choices: windows are listed in ascending order of their
IDs; ties in keeping order fall to the record's source and ID; OBS-003 states the limits
exception in its obligation as well; a login's Agents are counted when the call is made, and a
retired Agent's past report still counts; `windowMinutes` takes any positive number.

The engine (U2), both Agent packages (U3, U4), the CLI (U5) and the dispatcher (U6) follow. The
gate's run on the squash is the coordinator's, at the merge.
