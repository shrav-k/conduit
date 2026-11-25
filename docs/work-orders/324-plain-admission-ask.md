# Work order 324: the engine sends a login's admission ask to a plain item as a plain call (PKG-020, IFC-007, IFC-010, DEC-094, DEC-124, the usage guard's plain-ask engine order)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network, and every crate it builds is
cached: run cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one
cargo command at a time, and not `tools/gate.py` or the whole engine suite. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere: every implementation a test meets is scripted. Open,
copy or read no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.**
[DEC-094](../decisions/log.md#dec-094-2026-11-23-usage-is-self-reported-by-packages-and-the-engine-trusts-the-report):
"the codex package itself should define how to calculate usage statistics and levers. Conduit
specifies what must be reported and what contracts must be upheld. The engine then trusts this
reporting".
[DEC-124](../decisions/log.md#dec-124-2026-11-23-the-levers-come-on-new-type-versions), "New
Type versions (Recommended)": `codex-login.v2` and `account-home.v2` carry the levers, the
admission hook and its control.
[DEC-123](../decisions/log.md#dec-123-2026-11-23-a-call-waits-in-the-request-for-a-fresh-usage-report),
"Wait in the request (Recommended)": with no current report, a call that starts model work waits
for the login's one ask and is then dispatched or held on its answer.
[DEC-168](../decisions/log.md#dec-168-2026-11-23-a-report-asks-records-name-what-caused-it),
"Record the ask's cause": the ask's records name the call whose wait caused it. So the package
answers the ask from its own login wrapper, and the engine must reach that wrapper as PKG-020
says it reaches a server that was not written for Conduit: plainly. Orders 317 and 319 to 321
built the keeping, the ask, the hold and the warning; this order makes the ask reach a plain
item's hook, which the guard's package orders, 322 and 323, need.

**Today** (order 321's build: `main` with the decision log through DEC-157, the builds of orders
307 and 314 to 321 on top, and the log's DEC-158 to DEC-171), `admission_ask::wait` starts a
login's ask as an invocation with `"plain":false` and `"discovery":"pending"`, and its one intent
is `requests::discovery_intent`, a `server/discover`, whatever the login's item states.
`complete_discovery` then sends the ask keyed (IFC-006) where the result lists Conduit's
extension at `v0`, and otherwise withholds it, unavailable under IFC-037
(`hook-without-extension`); `settle_ended` gives such an ask the cause `not-dispatched`, and
PND-016 holds every call that waited on it, and every call on the login for the 60 seconds after.
A plain item is a server not written for Conduit (PKG-020) and lists no extension of Conduit's:
the login wrappers of agent-codex and agent-claude-code answer no `server/discover` at all. So
on a login whose item is plain, every call that starts model work would be held, and no turn
would run. The engine's other hooks are already right: `dispatch_hook` makes a hook of a plain
item plain unless the item's `keyed` lists it (`requests::keyed_hook`), records its dispatch
(`record_hook_dispatch`) and sends its `tools/call` at once; and order 209's section of
`engine/CONFORMANCE.md` says "Every other call to the item, every request among them, stays plain
(IFC-007, IFC-010)", which the ask does not keep. Both packages' login items are plain (ruling 86,
point 1), and the guard's package orders put `admission-hook.v1` on a plain item: `login-v2` of
`codex-login.v2` (order 322) and of `account-home.v2` (order 323). PKG-020 lets `keyed` list
only `setup-hook.v1`, `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` and
`sign-out-hook.v1`, so an item cannot ask for the admission hook keyed: on a plain item the ask
must be plain. Every admission case's login item, `resource`, is not marked plain, so the
admission cases and order 319's tests take the keyed path and stay as they are.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4,
nothing assumes a particular vendor's agent: a login's package answers the ask from a server
that need not know Conduit's extension, as its other hooks do; 6, the ask starts no model work
and is sent once, with no discovery or retry of its own, and its answer is read as any plain
call's (IFC-010); 7, the ask stays attributable: its records still name its caller and, in
`causedBy`, the call whose wait started it (DEC-168), though the call itself carries no key of
Conduit's; 1, nothing of it reaches an Agent's implementation. At risk, none in use: no package
states `admission` (the Start check's fifteenth printed line), so no store but a test's holds a
guarded login, and DEC-102 keeps the team seats from turns until the guard's orders merge.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
PKG-020 and IFC-007 are musts, today's ask breaks them for a plain item, and without this order
the guard's first login Types (DEC-124) would hold every call on their logins. By existing
practice, a Kubernetes API server calls a validating admission webhook with one `AdmissionReview`
request over HTTPS that any HTTP server can answer, with no discovery request before it; and
the engine's own `dispatch_hook` sends the sign-in, account, sign-out and retire hooks of a plain
item that `keyed` does not list as plain calls, recorded and sent in one transition (order 209,
ruling 86 point 2). Not taken: a `server/discover` before a plain ask; letting `keyed` list
`admission-hook.v1` (a change of PKG-020, and the packages would then need Conduit's extension);
`attributed` for the ask (the engine reads `attributed` for no call yet; the order that builds it
covers the ask with every other call); a new case (the core tests below show it; order 322's
engine test shows it through the shell and a packed wrapper); any change to the ask's input, its
deadline, its waiters or the hold.

**When it runs.** It stacks on orders 307 and 314 to 321 and runs on 321's build, commit
`45d3480555f0a3d3dc90fd58438641e7cb89c8ee`, until they merge, then on `main`. It comes before
order 322b, whose `codex_login_v2_admission_ask_reaches_the_hook` needs it, and before the
dogfood guards a login on order 323's `account-home.v2`. Through Conduit: one Codex worker turn,
the blind Codex verifier and Claude reader at once, one fix turn, the gate (DEC-046, DEC-071,
DEC-033). By ruling 89, each decision says what must hold; how is yours. It fits one turn: one
branch where the ask starts, one section and two core tests. The coordinator's, open to the
owner's strike (DEC-029): the ask plain exactly where `dispatch_hook` would make a hook plain,
by the item's `plain` and `keyed_hook`; its dispatch recorded in the transition that starts it;
`not-dispatched` left a keyed ask's cause alone; no case; the section's words; and the test
names.

**Shared files.** Orders 322 and 323 change `engine/crates/shell/tests/agents.rs` and
`make_archives.rs`, not the core. DEC-149's orders 311 to 313 change core `requests.rs`. Whichever
merges later rebases and reruns `promise_standing.py --check`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first PKG-020, IFC-006, IFC-007, IFC-010, IFC-037, IFC-045, IFC-046, IFC-052, IFC-054,
PND-016, OPS-007 and DL21; `admission-hook.v1` in `spec/core/operations.yaml`; in
`engine/CONFORMANCE.md` the sections "Keys per hook and the sign-in fixes" (order 209's) and of
orders 319 to 321; orders 319 and 320 whole. In the core, `admission_ask.rs`'s `wait`,
`settle_ended`, `deadline` and `recovered`; in `requests.rs`, `keyed_hook`, `dispatch_hook`,
`record_hook_dispatch`, `record_content`, `discovery_intent`, `dispatch_intent`,
`complete_discovery` and the completion, its `repairing` path for -32020 and its plain reading
from `let plain = v["plain"] == true;`; `admission::Report::take`; in the shell, `runner.rs`'s
`dispatch_later` and `discover_later`, which send an intent's call as written and discover only
for an intent whose method is `server/discover`; and order 319's and 320's helpers
in `admission_tests.rs` (`GuardScenario` with `admission_call`, `admission_input`, `finish`,
`discover_ask`, `login_status`; `waiting`, `dispatched_ids`, `assert_admission_hold`,
`no_report_fields`, `report_one`, `limits_member`, `result`) and `login_resource_tests::run_case`.
*The ask* is order 319's. *The login's item* is the implementation the login's publication object
resolves for `admission-hook.v1` (`resolved_implementations`); it is *plain* where it is marked
`plain`.

1. **A plain item's ask is a plain call** (PKG-020, IFC-007, IFC-045, PND-016, DEC-168). Where
   the login's item is plain, the ask is a plain invocation, as `dispatch_hook` makes a hook of a
   plain item that `keyed` does not list: `plain` true and no `discovery`, dispatched in the
   transition that starts it. That transition holds the ask's `invocation-dispatched` record, as
   `record_hook_dispatch` writes one, naming its caller and, in `causedBy`, the call whose wait
   started it, with no `serving`; and one intent for it, its `tools/call` as `dispatch_intent`
   builds a plain call: named by the item's `tools` for `admission-hook.v1`, its arguments
   `{levers, limits, previous}` as today, its `_meta` holding no key of Conduit's and declaring
   no extension of Conduit's. That is IFC-007's rule for an item whose `attributed` does not list
   `admission-hook.v1`; where it does, PKG-020 gives the call `invocation` and `caller`, but the
   engine reads `attributed` for no call yet, and building it is not this order's (What
   follows), so the ask carries no key of Conduit's whatever the item states. No
   `server/discover` is sent for it. Decide it by the item's `plain`
   and `requests::keyed_hook`, as `dispatch_hook` does, so that one rule says which hook calls are
   plain; share `record_hook_dispatch` rather than copy it. Where the item is not plain, the ask
   is sent as today: discovery first, then keyed (IFC-006, IFC-037). Nothing else of the ask
   changes: its caller, its input, its decision `start-admission-ask`, its entry in
   `admission_asks`, its waiters, the waiting reply and DL21's timer stay as order 319 built them.
2. **Its answer is read as a plain call's** (IFC-010, IFC-046, IFC-052, IFC-054, OPS-007,
   PND-016). A plain ask's completion takes the path of any plain call's in `requests.rs`, as it
   stands, with no step for discovery; this order changes nothing of that path. So: a first
   -32020 does not end the ask, but has its implementation's `tools/list` read again and the
   call sent once more (IFC-046's repair, `repair-parameter-headers-once`), the two being one
   call read from the second response, since `admission-hook.v1` states no `effect`, is no
   control and starts no model work; a complete result whose `isError` is not true succeeds
   where its `structuredContent` passes `admission-hook.v1`'s `outputSchema` and ends unknown
   where it does not (OPS-007); a complete result whose `isError` is true, or a protocol error
   that ends the call, after any resend IFC-046 makes, fails it, for the same reason (IFC-010);
   a result whose `resultType` is `input_required` or a value the engine does not recognize, and
   no answer, end it unknown (IFC-010). A succeeded answer's `admission` is kept as order 317
   keeps a report, a `limits` member in its `_meta` is kept in its end record (IFC-052) and does not end that report (IFC-054), and the
   calls waiting on it are settled as orders 319 to 321 built it, on a report or for want of one.
   A plain ask that gives no report gives the cause `failed`, `unknown` or `deadline`;
   `not-dispatched` stays the cause of a keyed ask that discovery withheld. At DL21 the ask's one
   intent is its `cancel`, and a restart ends it unknown under IMP-007, as today.
3. **The words.** At the end of `engine/CONFORMANCE.md`, wrapping at the width of the sections
   above it, a section `## A login's admission ask to a plain item (order 324)` whose one
   paragraph reads: "From order 324, the admission ask to a login whose implementation item for
   `admission-hook.v1` is marked `plain` is a plain call, as PKG-020 makes every call to a plain
   item that its `keyed` does not list, and `keyed` lists no admission hook: it is recorded and
   sent in the transition that starts it, with no `server/discover` before it, as the item's
   tool for `admission-hook.v1`, with the same input, no key of Conduit's and no extension
   declared (IFC-007). Where the item's `attributed` lists `admission-hook.v1`, PKG-020 gives
   the ask `invocation` and `caller`; the engine reads `attributed` for no call yet, so this
   build sends the ask with no key of Conduit's whatever the item states, and that support is
   deferred to the order that builds `attributed`. Its records still name its caller and the
   call whose wait started it (PND-016). Its answer is read as IFC-010 reads a plain call's:
   a first -32020 has the tool list read again and the call sent once more, as one call
   (IFC-046); a complete error result, or a protocol error that ends the call after any such
   resend, fails it, since `admission-hook.v1` states no effect; a result whose `resultType` is
   `input_required` or unrecognized, no answer, and an answer whose `admission` fails
   `admission-report.v1` (OPS-007), end it unknown; a succeeded answer is kept, with its limits,
   as order 317 keeps a report (IFC-052, IFC-054). An ask to an item not marked `plain` is sent
   after discovery, with Conduit's keys, as before."
4. **Tests.** Write them first, run them on the otherwise unchanged code and report each line.
   They go in `engine/crates/core/src/admission_tests.rs`, beside orders 319's and 320's, with
   their helpers and helpers of yours. *The plain project* is `guarded()` with its manifest's
   item `resource` marked `plain: true` and given `tools` naming its five selections:
   `account-hook.v1` `account`, `sign-in-hook.v1` `sign-in`, `sign-out-hook.v1` `sign-out`,
   `admission-hook.v1` `admission` and `#set-admission.v1` `set-admission`. *The plain report
   state* is the report state of order 317 with the plain project: the first 13 steps of
   `requests-admission-report-kept`, as `run_case` runs them, then the login's account check
   (`$accountcheck`), sent plain, answered at 13:00:00 with
   `{"isError":false,"content":[{"type":"text","text":"example-account"}],"structuredContent":{"account":"example-account"}}`,
   ending succeeded; where those steps do not run so with the plain item, set the state up as
   you see fit and report how. *A* and *B* are root's `call-a.turn.v1` and `call-b.turn.v1`; *K*
   is the ask the first waiting call starts; R1 is `report_one()`. Times are on 2026-11-21 in
   UTC.
   - `admission_ask_sent_plain_to_a_plain_item` (PKG-020, IFC-007, PND-016, DEC-168; decisions 1
     and 2): in the plain report state, A at 13:01:00 waits on K (`waiting`), and that
     transition has exactly one external intent, K's `dispatch` to the login, whose call has
     `method` `tools/call`, `params.name` `admission`, `params.arguments`
     `{"levers":{"billing":"included","paidUsage":"refuse","whenUnknown":"refuse"},"limits":{"state":"unknown","windows":[]},"previous":null}`
     and a `params._meta`, where there is one, with no key beginning `io.github.shrav-k.conduit/`
     and no extension of Conduit's under `io.modelcontextprotocol/clientCapabilities`, as
     `sign_in_plain_hook_through_the_interface_signs_in` checks a plain hook's; K's invocation
     has `plain` true and no
     `discovery`; the transition holds one `invocation-dispatched` record of K, whose `caller` is
     root's ID and `causedBy` A, with no `serving`, one `start-admission-ask` decision naming K
     and A, and no `read-implementation-discovery` decision. B at 13:01:00 joins K with no
     intent. K completed at 13:01:00 with
     `{"isError":false,"content":[{"type":"text","text":"admitted"}],"structuredContent":{"admission":R1},"_meta":{"io.github.shrav-k.conduit/limits":limits_member()}}`
     ends succeeded, its end record holding `admission` R1 and `limits` that member; the login's
     status has `state` `current` and `report` R1; and A and B are dispatched in that transition,
     in that order. In a fresh plain report state whose item also states
     `keyed: ['github.com/shrav-k/conduit/core#sign-in-hook.v1']`, A's ask is sent the same.
     Today the one intent is K's `server/discover`, and K's `plain` is false.
   - `admission_plain_ask_answers_read_as_plain` (IFC-010, IFC-046, IFC-054, OPS-007, PND-016,
     DL21; decision 2): in fresh plain report states, A waits on K at 13:01:00, and K
     - answered `{"isError":true,"content":[{"type":"text","text":"read failed"}]}` at 13:01:00
       ends failed, its end record holding no `admission`, and A is held with
       `no_report_fields(K, "failed", "2026-11-21T13:02:00Z")`;
     - answered with the protocol error -32603 at 13:01:00 ends failed, and A is held as in the
       first;
     - answered with the protocol error -32020 at 13:01:00 stays in progress, with A still
       waiting on it, no record of its end, one intent, K's `refresh-tools-and-call`, and one
       decision `repair-parameter-headers-once`; answered then with R1 as in
       `admission_ask_sent_plain_to_a_plain_item`, it ends succeeded and A is dispatched;
     - answered at 13:01:00 with `{"isError":false,"structuredContent":{"admission":R1}}`, R1
       lacking `singleUse`, ends unknown, and A is held with
       `no_report_fields(K, "unknown", "2026-11-21T13:02:00Z")`;
     - answered at 13:01:00 with `{"resultType":"unrecognized"}` ends unknown, and A is held as
       in the previous;
     - ended by `admission-ask-deadline` at 13:01:30 leaves one intent, K's `cancel`, and A is
       held with `no_report_fields(K, "deadline", "2026-11-21T13:02:30Z")`.

     Check each hold with `assert_admission_hold`, and that no transition of the test holds an
     intent whose method is `server/discover`; the -32020 bullet shows that IFC-046's repair,
     which this order does not change, reaches a plain ask. Today K's first answer is read as its
     discovery, which lists no extension, and A is held with `noReport` `not-dispatched`.
   - An existing test changes only where it asserts that an ask to a plain item is sent after
     discovery. None is expected: `admission_ask_started_once_for_waiting_calls` and the others
     ask a login whose item is not plain, and stay as they are, the keyed path's guards. Report
     any test that fails before you change it.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 094 123 124 168; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1 1 1
cat spec/promises/packages.yaml spec/interface/implementation.yaml spec/promises/pending.yaml | grep -cE '^- id: (PKG-020|IFC-007|IFC-010|IFC-052|IFC-054|PND-016)$'  # 6
k spec/promises/packages.yaml "the engine's calls for a hook that \`keyed\` lists carry Conduit's keys, and its other calls carry none of them," '`setup-hook.v1`, `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` or `sign-out-hook.v1`:'  # 1 1
k spec/interface/implementation.yaml "An engine sends a call for a request or a hook to an implementation marked \`plain\`, other than a call for a hook that its item's \`keyed\` lists (PKG-020)." "An implementation marked \`plain\` answers a \`tools/call\` request, other than one for a hook that its item's \`keyed\` lists (PKG-020)."  # 1 1
awk '/^  name: admission-hook.v1$/,/^---$/' spec/core/operations.yaml | grep -cE '^  (effect|startsModelWork):'  # 0
grep -o '"resource": {[^}]*}' conformance/language/contracts/admission-shape/conduit.json | grep -c '"plain"'  # 0
grep -cE '^\| (PKG-020 \| language \| must \| 12 \| 3 \| cases|IFC-007 \| interface \| must not \| 5 \| 1 \| cases|PND-016 \| convention \| must \| 7 \| 1 \| outside) \|' engine/STANDING.md  # 3
k engine/crates/core/src/admission_ask.rs '"state":"in-progress","plain":false,' 't.external_intents.push(requests::discovery_intent(&ask, c));' '"levers":login["config"].pointer(place),' '"previous":admission::newest(&t.state, login_id)' 'if invocation.is_none_or(|v| v["discovery"] == "pending") {' 'pub(crate) fn deadline('  # 1 1 1 1 1 1
k engine/crates/core/src/requests.rs 'pub(crate) fn keyed_hook(item: &Value, operation: &str) -> bool {' 'fn record_hook_dispatch(' 'pub(crate) fn discovery_intent(v: &Value, c: &Context) -> Value {' 'pub(crate) fn dispatch_intent(v: &Value, url_elicitation: bool, c: &Context) -> Value {' 'let mut rule = Some(if plain { "IFC-010" } else { "OPS-007" });' 'fn complete_discovery('  # 1 1 1 1 1 1
k engine/crates/core/src/admission.rs '|| invocation["discovery"] == "pending"'  # 1
k engine/crates/core/src/admission_tests.rs 'fn guarded() -> packages::Project {' 'fn discover_ask(&mut self, ask: &str, time: &str) -> Transition {' 'fn no_report_fields(ask: Value, cause: &str, until: &str) -> Value {' 'fn admission_ask_started_once_for_waiting_calls() {' 'admission_ask_sent_plain' 'admission_plain_ask'  # 1 1 1 1 0 0
k engine/crates/core/src/explain.rs 'pub(crate) fn admission_warning(tool: &str, form: &Value) -> String {'  # 1
k engine/CONFORMANCE.md "## A login's admission warning and its control (order 321)" "## A login's admission ask to a plain item (order 324)" 'Every other call to the item, every request among them, stays plain (IFC-007, IFC-010).'  # 1 0 1
grep -rlE 'admission-hook|loginTypes' packages | wc -l | tr -d ' '  # 0
for p in agent-codex agent-claude-code; do j packages/$p/conduit.yaml "command: [bin/$p, --login-resource] plain: true"; done | paste -sd' ' -  # 1 1
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it. A second printed line other
than stated means a decision moved; a third to sixth, that a promise this order rests on moved;
a seventh other than `0`, that an admission case's login item is plain, and an existing test
may need this order's change; an eighth, that the standing rows moved; a ninth to fourteenth,
that the code or order 321's build is not as this order was written against (the ninth's first
`1` is the literal this order removes); a fifteenth other than `0`, that a package states
`admission`, and the Preservation tests' "At risk" sentence no longer holds; and a sixteenth
other than `1 1`, that a login item is no longer plain.

The first line, and the Done when's line that counts files outside Scope, count the tree as it is
dispatched, with this order committed to its base. On a review copy where this order's file is
still untracked, each prints 1 more, for that file alone, and other uncommitted files there add
theirs. The coordinator commits the final order to the dispatch base and reruns both lines there
before dispatch; a worker who sees anything but 0 stops as above.

## Scope

Change, for decisions 1 to 4, in `engine/crates/core/src/`: `admission_ask.rs` (the ask's start),
`requests.rs` only to share `record_hook_dispatch` or the rule `dispatch_hook` uses for a plain
hook, `admission.rs` only where keeping a plain ask's report needs it (none is expected), and
`admission_tests.rs` (the two tests and their helpers); and `engine/CONFORMANCE.md`'s new
section. Change an existing test only under decision 4. Change nothing else: not `spec/`, a case,
`conformance/expected-failures.yaml`, `engine/STANDING.md`, `docs/` (DEC-094, DEC-123, DEC-124
and DEC-168 stay not done here), the shell, the loader, the conformance crate, `packages/`,
`tools/`, a `Cargo.toml` or `Cargo.lock`. No promise changes, so `spec/CHANGES.md` gains no line.
Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `j` and `k` as in the Start check;
the second, guards, passes today and after. Run the core's tests and then the whole vectors
test, once each, last; never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in admission_ask_sent_plain_to_a_plain_item admission_plain_ask_answers_read_as_plain; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; 0 twice today
k engine/crates/core/src/admission_ask.rs '"state":"in-progress","plain":false,'  # 0; 1 today: order 322's Start check reads it
k engine/CONFORMANCE.md "## A login's admission ask to a plain item (order 324)" 'From order 324,'  # 1 1; 0 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p "$1" --lib "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
k engine/crates/core/src/admission_ask.rs '"levers":login["config"].pointer(place),' '"previous":admission::newest(&t.state, login_id)'  # 1 1: order 322's Start check reads them
for t in admission_ask_started_once_for_waiting_calls admission_ask_without_a_report_starts_none_for_60_seconds admission_ask_deadline_ends_it_unknown admission_held_for_want_of_a_report admission_levers_changed_while_asking admission_wait_cancelled_restarted_or_repeated admission_warned_call_is_sent_as_without_the_guard keyed_hooks_read_plain_per_call; do s conduit-core "$t" 1; done  # 1 eight times
s conduit-core login_resource_ 10  # 1
for t in admission_asks_kept_across_a_reopen admission_ask_ends_at_its_deadline sign_in_plain_hook_through_the_interface_signs_in; do s conduit-shell "$t" 1; done  # 1 three times
s conduit-shell memory_equals_disk_after 5  # 1
for p in requests-admission- requests-keyed- requests-sign-in- requests-login-resource- contracts-admission-shape registration-admission-checks; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "6 cases passed; 1 expected failures", then "N cases passed", N in order: 4 22 2 1 1; where one prints otherwise before your change, report it and compare after with before
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(admission_ask|admission|admission_tests|requests)\.rs|engine/CONFORMANCE\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

No case's result changes: every admission case asks a login whose item is not plain, and
`requests-admission-single-use` still first fails at step 35.

Your report gives, per decision, what changed and where; each new test's line on the unchanged
code, then its passing line; how you set up the plain report state; each existing test you
changed, and why; your own choices; and every check's last line. Stage nothing.

## What follows

- **Order 322b**, `codex-login.v2` and its `login-v2` item, on this order's build: its
  `codex_login_v2_admission_ask_reaches_the_hook` shows the same through the shell's runner and
  the packed wrapper, a plain `tools/call` of `admission` and no `server/discover`. Its Start
  check's plain-ask line (`'"state":"in-progress","plain":false,'`, `0`) holds once this order
  merges.
- **Order 323**, `account-home.v2`, whose `login-v2` item is plain too: its hook is reached only
  on this order's build, and its *When it runs* should say so before a login is guarded on it.
- **`attributed` for the ask**: PKG-020 lets a plain item's `attributed` list
  `admission-hook.v1`, giving its calls `invocation` and `caller`; the engine reads `attributed`
  for no call yet, and the order that builds it covers the ask.
- DEC-094, DEC-123, DEC-124 and DEC-168 stay not done until the guard's orders and the sitting
  are done, and no team-seat Agent takes a turn before then (DEC-102).

## Record

Run on 2026-11-25 through Conduit itself, in Pacific time, on the usage guard's stack through order
321. Drafted by the drafter, `root-drafter.turn.v1`, on the Team seat, for orders 322 and 323's
second halves; checked adversarially by verifier-4 (one must, one should) and revised; built and
read on the team seats (DEC-179, DEC-180).

**Worker-4's turn**, on the Business seat, made the engine send a login's admission ask plain to a
plain item (PKG-020), keeping the one repair of -32020, with two tests that failed first.

**One read**, since the order changes engine code and the engine's own conformance account only
(DEC-189). The Codex verifier, `root-verifier-4.turn.v1`, passed it with no finding. The gate's
run on the squash is the coordinator's, at the merge.
