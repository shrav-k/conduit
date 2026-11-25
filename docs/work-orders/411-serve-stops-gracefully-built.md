# Work order 411: serve stops gracefully: it takes no new request, lets those in progress end within the grace period, then cuts the rest (DEC-176, built in the engine)

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
[DEC-176](../decisions/log.md#dec-176-2026-11-24-serve-stops-gracefully), "Yes, graceful stop
(Recommended)": a stop of serve takes no new request and lets those in progress finish within a
bound; a spec order and an engine order; the restart script retires.
[DEC-178](../decisions/log.md#dec-178-2026-11-24-serve-runs-under-launchd-after-the-graceful-stop),
"Yes, after the graceful stop (Recommended)": once DEC-176 is built, a private launchd job runs
serve. [Order 410](410-serve-stops-gracefully.md) is the spec half: OPS-037, DL20, the definition
`grace period`, the `stop` input of the conformance runner and the two cases
`requests-stop-takes-no-new-request` and `requests-stop-grace-period-ends`, both expected failures
"until order 411 makes it pass". This order is the engine half: the core takes the stop as
OPS-037 has it, the shell's `serve` asks for it on a signal and keeps its grace period, both cases
pass and their entries go.

**Today** (410's build, checkpoint `b08d6eb0`), no engine source names OPS-037, `engine-stop` or a
grace period. In `engine/crates/shell/src/main.rs`, `serve` makes one `CancellationToken`,
`shutdown`, which a task cancels on Ctrl-C or SIGTERM (`tokio::signal::ctrl_c()` and
`SignalKind::terminate()`). Startup setup (`set_up_instances`) polls it and, once cancelled, sends
a `cancel` for the setup in progress and starts no other. In `server.rs`, `serve_with_ready`
passes the same token to `axum::serve(...).with_graceful_shutdown`, which closes both listeners
at once, and to `listener_router`'s `StreamableHttpServerConfig::cancellation_token`; the
`Handler`'s `call_tool` loop, on that token, sends a `cancel` for its invocation and answers the
caller `internal("request cancelled")`; `listen`, `initialize`'s notification task and
`end_sign_in_wait_at` end on it. After the listeners close, `serve_with_ready` submits
`{"kind":"recover"}`, which ends anything still in progress unknown under IMP-007, and `main.rs`
submits another and drops the runner, whose `shutdown` ends every run (DL16, `Config.stop_grace`,
10 seconds from TERM to KILL). In the core, `Input` has no `Stop`, so each case first fails at its
step 15, the first `stop` step, with `unsupported core input`; `engine/STANDING.md` gives OPS-037
`2 | 2 | outside | none | expected failure`. `engine/CONFORMANCE.md` lists DL20 among the choices
still required ("Not built; order 411 builds OPS-037."). The coordinator restarts the dogfood's
`serve` with a restart script outside this repository, which waits for no call.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1), as order
410 gives them: 6, a call cut at the end of the grace period ends unknown, never failed, and is
not dispatched again without a repeat (IMP-007, OPS-012); a setup or retirement cut ends as a
cancellation ends it (OPS-027); 2, every request a stopping engine still takes is decided as any
other (ACC-001), and one it turns away gets no access decision; 7, each request turned away and
each call cut is recorded naming OPS-037, and the stop's two records name `system`. At risk: the
dogfood, whose restarts today cut every Agent's turn at once; with this order a restart lets a
turn end within the grace period, and cuts it unknown after, so a longer turn still ends unknown
(What follows).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes:
OPS-037 is a must since order 410, and both its cases wait on this order. By existing practice,
Go's `http.Server.Shutdown` stops taking work and waits for the active connections; Kubernetes
sends SIGTERM and waits `terminationGracePeriodSeconds`, 30 seconds by default, before SIGKILL;
systemd waits `TimeoutStopSec`; a server draining behind a load balancer answers `503` to new
work while it finishes the old (RFC 9110, section 15.6.4), which OPS-037's unavailable is; and
many servers end their wait at a second Ctrl-C. Not taken: closing the listeners at the signal
(OPS-037 answers each request unavailable, which needs them open); waiting past the grace
period; a stop operation through `management` (DL20 leaves the means to each engine, and order
410 declined it); a grace period stored in the store, which waits for DEC-139's startup file;
asking a provider for a report meanwhile (DL20's PRV-014 example); serving reads during the stop.

**When it runs.** It stacks on order 410 and runs on 410's build (checkpoint `b08d6eb0`, `main`
with 410 built and queued to merge) until 410 merges, then on `main`. Through Conduit, on pair 3
or 4 (DEC-180): one Codex worker turn, the blind Codex verifier and Claude reader at once, one
fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold;
how is yours. Every word, name and rule below that DEC-176 and OPS-037 do not state is the
coordinator's, open to the owner's strike (DEC-029), among them: the core's `stop` input as the
one way the stop reaches the core, with the reply forms order 410's cases give; the stop kept in
a `State` member that `recover` clears; the stopped record in the transition that ends the last
invocation in progress; the cut's reply and its use of `cancel`'s ending; the means `token` for
the library's token; a second signal ending the grace period at once; `Config.grace_period`,
30 seconds by default, and `serve --grace-period-seconds N`, 0 allowed; `/records` answered as
before; no further startup setup after the ask; the test names; and the documents' words. It
fits one turn if the worker reuses `cancel`'s ending and settles a cut retirement as a stop
report would; were it not to, it splits in two halves
from the start, each with its own turn, reads, fix turn and gate: 411a, the core (decisions 1 to
6, decision 9's core tests, decision 10's DL8 and profile lines, the two expected-failure
entries), and 411b, on 411a's build, the shell (decisions 7 and 8, decision 9's shell tests, the
rest of decision 10). *In two halves*, after the Done when, gives each half's differences. The
cases pass after 411a alone, since the conformance runner drives the core.

**Shared files.** Order 313 changes the shell's `runner.rs`, `store.rs`, `lib.rs` and `tests.rs`,
`tests/runner.rs` and `engine/CONFORMANCE.md`'s DL16 rows; order 320 (on its branch) adds
`admission_ask.rs` and changes core `requests.rs`, `lib.rs` and `engine/CONFORMANCE.md`: a call
waiting for a login's report is decided and not yet dispatched, so where 320 has merged, a stop
ends each such waiting call unavailable at once under OPS-037 (decision 4). Whichever merges
later rebases, keeps every insertion in a shared row and reruns `promise_standing.py --write`.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), or a file outside Scope
that must change. Where this order's wording contradicts a promise it does not change, the promise
governs: apply it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first OPS-037 whole, OPS-023, OPS-027, OPS-012, OPS-013, OPS-014, OPS-022, IMP-007, REC-001,
REC-007, IFC-020, IFC-049, IFC-050, LIF-013, LIF-021, TPL-009 and PRV-026; DL16 and DL20 in
`spec/implementation-defined.yaml`, the row "grace period" in `spec/definitions.md`;
`conformance/case-schema/inputs/stop.json`; the two cases in
`conformance/conventions/requests/stop-*/`; order 410 whole, with its Record; in
`engine/CONFORMANCE.md` the rows DL8, DL16 and DL20. In the core, `Input`, `State` and its
`with_installed_packages`, `step_body` and `reply_context` in `lib.rs`; `invoke_inner` to its access
decision, `reject_diagnostics`, `cancel`, `finish`, `recover` and `record` in `requests.rs`;
`lifecycle::cancel` and `cascade::cancel`; `explain::outcome_cause`. In the shell, `main.rs`'s
`serve` branch and `set_up_instances`; in `server.rs`, `Handler`, `Pending`, `call_tool`,
`end_sign_in_wait_at`, `listener_router`, `serve` and `serve_with_ready`; `Config` and the input
kinds that need evidence in `lib.rs`. *The work in progress* is OPS-037's; *the ask* is the
first stop asked for; *the end* is the moment the engine stops, at once when nothing is in
progress, else when the last invocation in progress ends or the grace period ends.

1. **The `stop` input** (OPS-037, REC-001). The core gains `Input::Stop`, read as
   `stop.json` gives it: `evidence`, `means` (a string, optional) and `graceEnded` (true, optional).
   - The ask (no stop under way): one `engine-stop` record, `data` `{target: system's ID,
     phase: "asked", means}`, `means` left out where none is given, with no `caller` in `data` or
     beside it; reply `{"stopping": true}`; no intent; the state records that the engine is
     stopping. Where nothing is in progress, the same transition also keeps the stopped record of
     decision 5 with `cut` false.
   - A further `stop` without `graceEnded` while stopping keeps no record, changes nothing and
     replies `{"stopping": true}`.
   - `graceEnded` true while stopping and not yet stopped cuts the work in progress (decision 6).
     With no stop asked before, it is the ask and its end in one transition: the asked record
     first. Once stopped, any `stop` changes nothing and replies `{"stopped": true, "cut": []}`.
2. **What a stopping engine turns away** (OPS-037, OPS-023, IFC-050, REC-001). From the ask, each
   request that is not part of the work in progress is turned away, unless it is a repeat that
   OPS-009, OPS-012, OPS-032 or OPS-034 answers without a dispatch, which is answered as before
   (one that OPS-012 or OPS-024 would dispatch as a new attempt is turned away). Turned away: not
   dispatched; no access decision and no `access-decision` record; reply `isError` true, `_meta`
   with `io.github.shrav-k.conduit/reason` `unavailable` and no `outcome`, and one error
   diagnostic, `ruleId` OPS-037, whose text names the stop and says that the request may be made
   again once the engine has started again; one `request-ended` record with `reason` unavailable
   and `ruleId` OPS-037; no intent. It is decided where OPS-023 puts it, first for a request that
   can be read, and only for one: a request that cannot be read (`request.readable` false) is
   answered as today whether or not its identity is established, invalid under IFC-036 in
   `invoke_inner`'s first check where it is not, and invalid under OPS-023 from
   `access::evaluate`'s first check where it is. So the turn-away stands in `invoke_inner` after
   IFC-036's check, guarded by `request.readable`, and before `access::evaluate`; a readable
   request whose authority cannot be read is turned away under OPS-037, not ACC-037, as OPS-023
   now orders them. Nothing else about a turned-away request is examined or reported. Every input
   kind that begins a new request is turned away alike.
3. **What a stopping engine takes** (OPS-037, IFC-020, IFC-049, PND-004). It takes and decides as
   any other: an onward request whose `serving` names an invocation in progress (IFC-020); any
   other request from an instance while an invocation dispatched to it is in progress; an
   `answer.v1` for a pending action that an invocation in progress waits for; and a `respond`
   carrying the requestState of an invocation in progress (IFC-049). The parts of an invocation
   in progress (a hook, a read of a secret, a provider call, a sign-in's parts) are dispatched as
   before, and `complete`, `setup-report`, `cancel`, `sign-in-wait-ended`, `builtin-run-stopped`
   and `builtin-secret-provider-exited` are read as before. `list-tools` is answered as before.
4. **A decided call not yet dispatched** (OPS-037, REC-007). Where the build holds a request decided
   before the ask that is waiting for anything but its own parts, the ask ends it unavailable
   under OPS-037 at once, as decision 2's reply and record, its reply in `request_replies`. The
   coordinator finds none in this tree (the Start check's line on `admission`): report what you
   find and build this only where you find one. Where order 320 has merged, its waiting calls are
   such calls.
5. **The end** (OPS-037, REC-001, LIF-013). The engine stops once nothing is in progress, at the
   ask or in the transition that ends the last invocation in progress, which keeps, after its own
   records, one `engine-stop` record, `data` `{target: system's ID, phase: "stopped", cut: false}`,
   with no `caller`. In progress means each invocation entry whose `state` is `in-progress`, parts
   and hooks included. Both records name `system`, so inspecting `system` lists them under LIF-013.
   The stop's transitions are commits as any other, so a record that REC-007 holds to be kept
   late is kept at them as at any commit; no other path is built.
6. **The cut** (OPS-037, OPS-027, OPS-022, LIF-021, OPS-012, OPS-013, IMP-007, TPL-009,
   IFC-050). At `graceEnded`, each invocation still in progress that is no other's part ends in
   that one transition: none is left pending, and the engine waits for no answer.
   - **Intents.** The cut sends the intents a `cancel` of it sends today: its cancellation, a
     setup's or retirement's `cancel-setup-parts` and its children's cancellations, and each stop
     that TPL-009 still asks for of a retirement so cancelled; a stop already outstanding is not
     sent again. Its parts end as that cancellation ends them.
   - **A call that is not a setup or a retirement** ends unknown, as `cancel` ends one today.
   - **A setup** ends as `lifecycle::cancel` ends it today, which settles it at once under OPS-022.
   - **A retirement** does not end as a cancellation does: `retirement::cancel` leaves one pending
     where its stop is outstanding or newly asked for (`phase` `stop`, its reply
     `{"pending": true}`, settled under OPS-022 by the later stop report, as
     `cancellation_during_stop_settles_under_ops022_on_later_report` shows). Under the stop, once
     the stop intent is sent or was sent before, the retirement's stop part is added as ended
     `unknown`, with its effect, and the retirement is settled at once, as
     `retirement::settle` settles one on a stop report: its outcome is OPS-022's for those parts,
     and its object's `shutdown` is what `settle` gives an unknown stop. A container's retirement
     (`retire-with-resources.v1` and its siblings) is cut as `cascade::cancel` cuts it, each
     member's retirement settled so, and it ends with LIF-021's outcome in the same transition.
     A stop report or any answer that arrives later for a cut invocation, a retirement's
     included, changes no outcome, state or record (OPS-013): it is read as one for an ended
     invocation is read today.
   - **The diagnostic** (IFC-050). Each cut invocation's result carries one error diagnostic,
     `ruleId` OPS-037, in place of OPS-027's, and its `invocation-ended` and `request-ended`
     records hold `ruleId` OPS-037, so a reader tells it from a cancellation and from an end at
     a restart. Its text says that the engine stopped before the call ended. Where the outcome
     is unknown it also says that the effect may have occurred and directs the caller, once the
     engine has started again, to repeat with the same idempotency key where the call carried
     one (OPS-012), and otherwise to reconcile under OPS-014 after checking whether the effect
     occurred, rather than make a new request; it may add that a repeat may still report
     unknown. It names the key only as "the same idempotency key", never its value. Build it in
     `explain.rs` beside OPS-027's text, from `explain::outcome_cause`.
   - **The reply** is `{"stopped": true, "cut": [...]}`, each cut call's result in the order of
     `state.invocations`, and each is also in `request_replies`, so each caller still waiting is
     answered. Then the stopped record of decision 5, with `cut` true where any was cut. A cut
     invocation is not dispatched again without a repeat, and after a restart a repeat is
     answered under OPS-012, as for any invocation ended unknown.
7. **The state of the stop** (DL2, IMP-007). The core keeps the stop in one `State` member,
   `#[serde(default, skip_serializing_if = …)]`, so a state with no stop is written byte for byte
   as today; `with_installed_packages`'s destructuring names it. Its form is yours: it holds at
   least whether the stop is asked or done. `recover` clears it and keeps no record for it, so an
   engine killed during the grace period opens and serves as IMP-007 and REC-007 have it, and
   `requests-stop-grace-period-ends`' last step finds `records` empty. The shell reads whether
   the engine has stopped from the state, through one public function of the core or the member
   itself; how is yours.
8. **The shell** (OPS-037, DL20, DL16). In `main.rs` and `server.rs`, with one function for the
   stop that startup and serving share:
   - **The ask.** SIGTERM or SIGINT submits `{"kind":"stop","means":"SIGTERM"}` (or `"SIGINT"`).
     The library's `serve` and `serve_with_ready` keep their signatures; their `shutdown` token,
     once cancelled, is the ask, with `means` `token`, so a test that cancels it with nothing in
     progress stops at once, as today. Asked during startup setup, the setup in progress goes on,
     no further startup setup starts, and the listeners never open.
   - **During the grace period** the listeners stay open, and each request goes to the core as
     before, which takes it or turns it away (decisions 2 and 3); `initialize`, the protocol's
     other messages and `/records` are answered as before. `call_tool` no longer cancels on the
     ask: a caller's own cancellation and the response's end cancel as before. `listen`,
     `initialize`'s notification task and `end_sign_in_wait_at` end at the end, not at the ask.
     The engine does nothing of its own accord meanwhile.
   - **The grace period** is `Config.grace_period`, a `Duration`, 30 seconds by default, set by
     `serve --grace-period-seconds N` (whole seconds, 0 allowed, which cuts at once); the flag is
     refused on every other command and refused with a malformed value before the store opens,
     as `--https-listen`'s is. It runs from the ask; at its end, or at a further SIGTERM or
     SIGINT, whichever is first, the shell submits `{"kind":"stop","graceEnded":true}`. A further
     signal keeps no record.
   - **The end.** Once the core has stopped, the shell answers each caller still connected with
     its reply, then closes the listeners, then ends the runner's runs as DL16 states and exits 0,
     as today. Only then does the runner's `shutdown` start, so the runs the work in progress
     needs, an Agent's among them, go on through the grace period. The `recover` submits that
     follow stay, and find nothing in progress. `stop` joins the input kinds that need evidence.
9. **Tests.** Write them first, run them on the otherwise unchanged code and report each failing
   line. No other test's name contains a new test's name. Bound each wait at 60 seconds, but a
   grace period a test sets. A case's steps are counted from 1, as the runner prints them.
   - In a new `engine/crates/core/src/stop_tests.rs` (`mod stop_tests;` in `lib.rs`, under
     `#[cfg(test)]`), from the state of `requests-stop-takes-no-new-request` through step 14 (A
     dispatched), built as `limits_tests.rs` builds a case's state with `limits_case`, or as
     `login_resource_tests::run_case` does:
     - `stop_turns_away_new_requests_unavailable_first` (decisions 1 and 2): the ask without
       `means` gives the asked record without `means`; then root's `inspect.v1` of `system`, an
       `inspect.v1` naming an object that does not exist (refused or not found before the ask),
       and the Agent's `project.file.write.v1` each report as decision 2 gives, with no
       `access-decision` record, OPS-037's diagnostic alone and its text containing "started
       again"; two unreadable requests (`readable` false), one from root, whose identity is
       established, and one from an identity that is not, give after the ask the reply,
       records and decisions they give before it, invalid under OPS-023 and IFC-036, with no
       OPS-037 diagnostic; `list-tools` for root gives the same reply as before the ask; a second
       ask keeps no record. Today the ask is `unsupported core input`.
     - `stop_takes_the_work_in_progress_and_its_repeats` (decision 3): in a state with root's
       `call-a.turn.v1` dispatched to Agent `a`, built as `limits_tests.rs` builds one with
       `root_call` (where `a` reaches no other operation, give it one in the test's copy of the
       project, and report how), after the ask `a`'s own call is dispatched as before the ask
       would dispatch it, and another instance's call is turned away; a keyed repeat of a call that
       completed before the ask is answered from its kept result, with no dispatch; a `respond`
       for an invocation in progress is read as before. Today the ask fails.
     - `stop_ends_when_the_last_call_ends` (decisions 1 and 5): with A and a second call in
       progress, completing the first keeps no stopped record; completing the last keeps it, after
       that call's records, with `cut` false; in a state with nothing in progress the ask keeps the
       asked record, then the stopped one; once stopped, `stop` with `graceEnded` replies
       `{"stopped": true, "cut": []}`. Today the ask fails.
     - `stop_grace_end_cuts_calls_as_cancel_ends_them` (decision 6, calls and setups): in copies
       of one state, a `cancel` of a plain call in progress and the stop's cut give the same
       intents and outcome, unknown, but for `ruleId` OPS-037 and the diagnostic; for a setup in
       progress (from a case with a setup dispatched, as `requests-cancellation` or a setup case
       has one; name it) the outcome is a cancellation's, not unknown. Two unknown calls are cut,
       one carrying an idempotency key and one without: the first's diagnostic text says that
       the engine stopped before the call ended, that the effect may have occurred, and to repeat
       with the same idempotency key once the engine has started again, and does not contain the
       key's value; the second's says the first two and to reconcile after checking the effect
       rather than make a new request. Each result has `isError` true, outcome unknown and no
       `reason`. Today the ask fails.
     - `stop_grace_end_settles_retirements_without_a_report` (decision 6, retirements), from
       `retirement.rs`'s test fixture or a case with a retirement in progress, in three states:
       a retirement whose stop is already outstanding (`phase` `stop`, as
       `cancellation_during_stop_settles_under_ops022_on_later_report` sets it); one with a run
       and no stop yet, whose cut asks for the stop; and a container's retirement with a member
       retirement in progress, as `cascade.rs`'s `cascade_fixture` makes one. In each, the cut's
       transition holds a final result for the retirement, not `{"pending": true}`, in the
       reply's `cut` and in `request_replies`, with OPS-022's outcome (LIF-021's for the
       container), `ruleId` OPS-037 and the diagnostic; the intents hold the stop where TPL-009
       asks for one and no stop twice; no invocation stays in progress; and the stopped record
       follows. Then a `setup-report` of the stop with outcome `succeeded` for the cut
       retirement is refused or changes nothing: no outcome, object, invocation or record differs
       from the cut's. Today the ask fails.
     - `stop_is_forgotten_at_restart` (decision 7): the state serialized before the ask equals
       the one after `recover` byte for byte, but for what `recover` itself changes; a stop asked
       and never ended, then `recover`, leaves no stop member and keeps no record for it, and
       a request after it is decided as before the ask. Today the ask fails.
   - In `engine/crates/shell/src/server_tests.rs`, over MCP, as
     `pending_http_completion_and_client_cancellation` serves a shell with a pending call (a
     shell from a case through a step, as `fixed_case_shell` makes one, and a scripted executor
     that holds the dispatch):
     - `serve_stop_answers_new_calls_unavailable_and_the_call_in_progress`: with a call pending,
       cancel the token: `serve_with_ready` has not returned; root's new `tools/call` reports
       unavailable naming OPS-037, and `tools/list` answers as before; the executor completes the
       pending call: its caller receives its result, and `serve_with_ready` returns within 10
       seconds; the records end with `engine-stop` asked, means `token`, then stopped, `cut`
       false. Today the cancellation answers the pending caller `request cancelled`.
     - `serve_stop_takes_an_agents_own_calls`: with a call pending on an Agent, after the
       cancellation the Agent's own `tools/call`, with its token, is dispatched, and another
       instance's is turned away; that Agent call completes, then the first: `serve_with_ready`
       returns. Today the Agent's call fails.
     - `serve_grace_period_end_answers_the_cut_call_unknown`: with `Config.grace_period` 1
       second and a pending call the executor never completes, after the cancellation the caller
       receives, within 10 seconds, a result with outcome unknown and an OPS-037 diagnostic
       whose text says that the engine stopped before the call ended and that the effect may
       have occurred; the
       executor saw one cancellation; `serve_with_ready` returns; the stopped record holds `cut`
       true; the invocation is `unknown` in the state. Today the caller gets `request cancelled`.
   - In `engine/crates/shell/tests/runner.rs`, beside
     `serve_signal_during_setup_ends_run_and_starts_no_later_part`, through the binary, with
     `scripted_server`'s modes (add a `release` mode, whose call answers once a file named
     `release` appears in its root; how is yours):
     - `serve_sigterm_lets_the_call_in_progress_end`: `serve --grace-period-seconds 600` set up
       in `release` mode; a `conduit call` reading through `history` waits, in a thread; once it
       is dispatched, SIGTERM; a second `conduit call` of root's `inspect.v1` reports unavailable
       naming OPS-037; write `release`: the first call's reply succeeds; serve exits 0 within 60
       seconds; reopened, the store's records hold the two `engine-stop` records, means
       `SIGTERM`, `cut` false, and `inspect` of `system` lists them; every run group is gone.
       Today the first call is cut at the signal.
     - `serve_grace_end_cuts_the_hanging_call_by_timer_or_second_signal`: in `hang` mode, twice:
       with `--grace-period-seconds 2` and one SIGTERM, and with `--grace-period-seconds 600`,
       SIGTERM, then SIGINT. Each time the waiting call's reply has outcome unknown and an OPS-037
       diagnostic whose text says that the engine stopped before the call ended, serve exits 0
       within 60 seconds, its records hold one asked and one stopped record with `cut` true, and
       the reopened state holds the invocation `unknown`. Then `serve --grace-period-seconds x`
       and `init --grace-period-seconds 1` are refused and leave the store unopened. Today the
       reply is not OPS-037's.
   - An existing test changes only where it stops `serve` with a call in progress and asserts the
     immediate cut, or where the grace period makes it slower than its bound: it may set
     `Config.grace_period` or the flag. `serve_signal_during_setup_ends_run_and_starts_no_later_part`
     is expected to stay as it is. Report each test you change, and why.
10. **The documents.** Wrap each insertion at its paragraph's width.
    - `engine/CONFORMANCE.md`, row "DL8: records": "Seven event types:" → "Eight event types:";
      after "pending-action-answered (order 047c, an answer to a pending action, apart from the
      request's records under PND-004 and REC-002)" add ", engine-stop (order 411, the two
      records of a stop under OPS-037, against `system`)". Row "DL16: executor coverage", after
      "with one shared Config.stop_grace (10 seconds by default) for all runs.": "From order
      411, under `serve` the runner's shutdown begins only once the engine has stopped under
      OPS-037 (DL20), so the runs the work in progress needs go on through the grace period."
      Remove the row "| DL20 | Not built; …" from the choices still required, and add after the
      row "DL15" of the first table: "| DL20: the stop of the engine | From order 411, SIGTERM or
      SIGINT asks `serve` to stop under OPS-037, as cancelling the token passed to the library's
      `serve` does (means `token`). The grace period is `Config.grace_period`, 30 seconds by
      default, set by `serve --grace-period-seconds N`; 0 cuts at once. A further SIGTERM or
      SIGINT ends the grace period at once. Meanwhile the engine does nothing of its own accord.
      The listeners stay open until the end, answering each request not taken unavailable;
      `initialize`, `tools/list` and `/records` are answered as before. Once stopped, the runner
      ends its runs as DL16 states. | `requests-stop-takes-no-new-request`,
      `requests-stop-grace-period-ends`, the six tests of the core's `stop_tests.rs`,
      `serve_stop_answers_new_calls_unavailable_and_the_call_in_progress`,
      `serve_stop_takes_an_agents_own_calls`, `serve_grace_period_end_answers_the_cut_call_unknown`,
      `serve_sigterm_lets_the_call_in_progress_end`,
      `serve_grace_end_cuts_the_hanging_call_by_timer_or_second_signal` |".
    - `conformance/profiles/first-build.yaml`: `"engine-stop"` after `"pending-action-answered"`
      in `record_types`.
    - `engine/crates/shell/src/help.rs`: serve's usage gains ` [--grace-period-seconds N]` after
      `[--https-roots FILE]`; "SIGTERM or Ctrl-C ends runs." → "SIGTERM or Ctrl-C stops it
      gracefully: no new request is taken, those in progress may end within the grace period,
      then runs end; a second signal ends the grace period at once."; under "Options for serve
      alone:", "  --grace-period-seconds N: how long a stop lets calls in progress end (default
      30)."
    - `engine/CLI.md`: the serve usage line gains ` [--grace-period-seconds N]`; after the
      paragraph ending "when that variable is unset or empty.", a paragraph: "SIGTERM or SIGINT
      asks `conduit serve` to stop (OPS-037). From then on it takes no new request: each reports
      unavailable, naming OPS-037, and may be made again once serve has started again. Calls in
      progress, and the calls they make, go on for the grace period, `--grace-period-seconds N`
      (30 by default); then each still in progress is cancelled and its caller answered, a setup
      or a retirement as a cancellation ends it, any other call with the outcome unknown. A
      second signal ends the grace period at once. Serve then ends its runs and exits; inspecting
      `system` lists the two records of the stop."
    - `engine/README.md`: "SIGINT or SIGTERM stops the server." → "SIGINT or SIGTERM stops the
      server gracefully, as the command guide describes (OPS-037).", with "the command guide"
      linked to `engine/CLI.md`'s section "Engine context".

    Then remove the two entries of `conformance/expected-failures.yaml`, and only after that
    run `python3 tools/promise_standing.py --write`, which reads the expected failures to give
    OPS-037's row; `conformance/internal-cases.yaml` keeps both cases.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
for d in 176 178; do sed -n "/^## DEC-$d, /,/^## /p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1
grep -c '^- id: OPS-037$' spec/conventions/operations.yaml; grep -c '^- id: DL20$' spec/implementation-defined.yaml  # 1, then 1
grep -cE '"id": "requests-stop-(takes-no-new-request|grace-period-ends)"' conformance/expected-failures.yaml  # 2
grep -cE '"requests-stop-(takes-no-new-request|grace-period-ends)",' conformance/internal-cases.yaml  # 2
grep -c '"stop",' engine/crates/conformance/src/portable.rs  # 2
grep -cE '^\| OPS-037 \| convention \| must \| 2 \| 2 \| outside \| none \| expected failure \|$' engine/STANDING.md  # 1
grep -rlE 'OPS-037|engine-stop|graceEnded|grace_period|grace-period' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '  # 0
ls engine/crates/core/src | grep -cE '^(stop|stop_tests|admission.*)\.rs$'  # 0
k engine/crates/core/src/lib.rs 'pub enum Input {' 'Input::Recover { evidence } => requests::recover(&mut transition, &evidence, context)?,' '| Input::Recover { evidence }' 'mod limits_tests;'  # 1 1 1 1
k engine/crates/core/src/requests.rs 'pub(crate) fn cancel(' 'pub(crate) fn recover(t: &mut Transition, evidence: &Evidence, c: &Context) -> Result<()> {' 'fn invoke_inner(' 'fn reject_diagnostics(' 'if !request.readable && access::established(&t.state, &request.identity).is_err() {'  # 1 1 1 1 1
k engine/crates/core/src/limits_tests.rs 'fn limits_case(' 'fn root_call('  # 1 1
k engine/crates/shell/src/server.rs 'return Err(internal("request cancelled"));' '_=self.shutdown.cancelled()=>{} } }=>' '.with_graceful_shutdown(shutdown.clone().cancelled_owned())' 'pub async fn serve_with_ready('  # 1 1 1 1
k engine/crates/shell/src/main.rs 'tokio::select! { _ = tokio::signal::ctrl_c() => {}, _ = terminate.recv() => {} }' 'signal.cancel();' '// Release startup tickets if a signal interrupted setup before listening.' '"--call-timeout-seconds" => {'  # 1 1 1 1
k engine/crates/shell/src/lib.rs 'stop_grace: Duration::from_secs(10),' 'pub call_timeout: Duration,' '| "recover" | "builtin-run-stopped"'  # 1 1 1
k engine/crates/shell/src/server_tests.rs 'fn fixed_case_shell(path: &Path, case: &Path, through: usize) -> Shell {' 'fn pending_http_completion_and_client_cancellation() {' 'fn token(shell: &Shell, name: &str) -> String {'  # 1 1 1
k engine/crates/shell/tests/runner.rs 'fn scripted_server() {' 'if mode == "hang" || mode == "cancel" {' 'fn serve_signal_during_setup_ends_run_and_starts_no_later_part() {'  # 1 1 1
grep -rlE 'fn (stop_turns_away_new_requests_unavailable_first|stop_takes_the_work_in_progress_and_its_repeats|stop_ends_when_the_last_call_ends|stop_grace_end_cuts_calls_as_cancel_ends_them|stop_grace_end_settles_retirements_without_a_report|stop_is_forgotten_at_restart|serve_stop_answers_new_calls_unavailable_and_the_call_in_progress|serve_stop_takes_an_agents_own_calls|serve_grace_period_end_answers_the_cut_call_unknown|serve_sigterm_lets_the_call_in_progress_end|serve_grace_end_cuts_the_hanging_call_by_timer_or_second_signal)\(' engine/crates | wc -l | tr -d ' '  # 0
k engine/CONFORMANCE.md '| DL20 | Not built; order 411 builds OPS-037.' 'Seven event types:' '| DL15 | Generated names retain at most the first 57 characters' 'with one shared Config.stop_grace (10 seconds by default) for all runs.' 'From order 411,'  # 1 1 1 1 0
k conformance/profiles/first-build.yaml '"pending-action-answered"' '"engine-stop"'  # 1 0
k engine/crates/shell/src/help.rs 'SIGTERM or Ctrl-C ends runs.' '"[--https-url URL] [--https-listen ADDRESS] [--https-roots FILE]",'  # 1 1
k engine/CLI.md 'conduit serve --store DIR [--https-url URL] [--https-listen ADDRESS] [--https-roots FILE]' 'when that variable is unset or empty.'  # 1 1
k engine/README.md 'print token files or recover the running store. SIGINT or SIGTERM stops the server.'  # 1
k engine/crates/core/src/retirement.rs 'fn settle(' 'fn cancellation_during_stop_settles_under_ops022_on_later_report() {' 'if v["retirement"]["phase"] == "stop" {' 't.state.invocations[i]["retirement"]["phase"] = json!("stop");'  # 1 1 1 1
k engine/crates/core/src/cascade.rs 'pub(crate) fn cancel(' 'fn cascade_fixture(members: &[&str]) -> (State, Context) {'  # 1 1
k engine/crates/core/src/access.rs 'if !request.readable {' 'return Ok(Verdict::reason("invalid", "OPS-023"));'  # 1 1
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"  # engine 0
```

Where a line prints otherwise, stop, change nothing and report it. A second to seventh printed
line other than stated means order 410 is not in the tree as built; an eighth other than `1`,
that its standing moved; a ninth other than `0`, that an engine source already names the stop; a
tenth other than `0`, that `stop.rs` exists or order 320's `admission_ask.rs` has merged (then
read decision 4 and the Shared files, report it and go on); an eleventh to eighteenth other than
stated, that the code moved since this order was written; a nineteenth other than `0`, that a
new test's name is taken; a twentieth to twenty-fourth, that a document already changed; a
twenty-fifth to twenty-seventh, that the retirement, container or readability code that
decisions 2 and 6 rely on moved.

The first line, and the Done when's line that counts files outside Scope, count the tree as it is
dispatched, with this order committed to its base. On a review copy where this order's file is
still untracked, each prints 1 more, for that file alone, and other uncommitted files there add
theirs. The coordinator commits the final order to the dispatch base and reruns both lines there
before dispatch; a worker who sees anything but 0 stops as above.

## Scope

By the coordinator's rulings during the build, three more files:
`engine/crates/conformance/src/checks.rs`, whose list of record event types gains `engine-stop` and
nothing else; `engine/crates/shell/src/runner.rs`, so that a cut retirement's provider stop reaches
the runtime provider (OPS-037, TPL-009, PRV-005), OPS-013's settled results kept; and
`engine/crates/shell/tests/s2.rs`, whose legacy assertion of a SIGTERM's exit status follows
decision 8.

Change, in `engine/crates/core/src/`: a new `stop.rs` (the stop's transitions, decisions 1 to 6)
and `stop_tests.rs` (its tests); `lib.rs` (`Input::Stop`, the `State` member and its
destructuring, `step_body`, `reply_context`, the `mod` lines); `requests.rs` (the turn-away in
`invoke_inner`, the stopped record after an end, the cut through `cancel`'s ending);
`retirement.rs` and `cascade.rs` for the cut's settlement of a retirement, its own and a
container's, without a stop report (decision 6), and where a later report for a cut retirement
must change nothing; `lifecycle.rs` and `sign_in.rs` only where a cut setup or sign-in must carry
OPS-037 in place of OPS-027; `pending.rs` only to recognise an `answer.v1`
that work in progress waits for; `explain.rs` only for OPS-037's text; `record_fault.rs` only if
`stop` must join `covers`; `differential.rs` only for the `State` member. In
`engine/crates/shell/src/`: `main.rs` (the signals, startup setup, the flag), `server.rs` (the
stop), `lib.rs` (`Config.grace_period`, the evidence kinds), `help.rs` and `server_tests.rs` (its
three tests); `engine/crates/shell/tests/runner.rs` (its two tests and the `release` mode). Also
the two entries of `conformance/expected-failures.yaml`, `conformance/profiles/first-build.yaml`'s
one line, `engine/CONFORMANCE.md`, `engine/CLI.md` and `engine/README.md` as decision 10 says, and
`engine/STANDING.md` by the tool alone. Change an existing test only under decision 9, in
`tests/runner.rs`, `tests/serve.rs`, `tests/cli.rs`, `tests/agents.rs` or `src/server_tests.rs`.
Change nothing else: not `spec/`, a case, `conformance/internal-cases.yaml`, the conformance crate,
the loader, `docs/` (DEC-176 and DEC-178 stay not done; the coordinator marks DEC-176 at the merge),
`packages/`, `tools/`, a `Cargo.toml` or `Cargo.lock`. No promise changes, so no changelog line is
added. Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after, with `j` and `k` as in the Start check;
the second, guards, passes today and after. Run each existing test you changed by name, once,
too. Run the core's tests, the shell's `--lib`, then the whole vectors test, once each, last;
never the whole engine suite.

```sh
ok='^test result: ok\. 1 passed'
for t in stop_turns_away_new_requests_unavailable_first stop_takes_the_work_in_progress_and_its_repeats stop_ends_when_the_last_call_ends stop_grace_end_cuts_calls_as_cancel_ends_them stop_grace_end_settles_retirements_without_a_report stop_is_forgotten_at_restart; do (cd engine && cargo test --offline --release -p conduit-core --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 six times; 0 today
for t in serve_stop_answers_new_calls_unavailable_and_the_call_in_progress serve_stop_takes_an_agents_own_calls serve_grace_period_end_answers_the_cut_call_unknown; do (cd engine && cargo test --offline --release -p conduit-shell --lib "$t" 2>&1 | grep -c "$ok"); done  # 1 three times; 0 today
for t in serve_sigterm_lets_the_call_in_progress_end serve_grace_end_cuts_the_hanging_call_by_timer_or_second_signal; do (cd engine && cargo test --offline --release -p conduit-shell --test runner "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; 0 today
for c in takes-no-new-request grace-period-ends; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. requests-stop-$c 2>&1 | grep -c "^PASS requests-stop-$c "); done  # 1 twice; 0 twice today
grep -cE '"id": "requests-stop-(takes-no-new-request|grace-period-ends)"' conformance/expected-failures.yaml  # 0; 2 today
grep -cE '^\| OPS-037 \| convention \| must \| 2 \| 0 \| outside \| (core|core, shell) \| case passes \|$' engine/STANDING.md  # 1; 0 today
k engine/CONFORMANCE.md '| DL20 | Not built; order 411 builds OPS-037.' 'Eight event types:' 'engine-stop (order 411,' '| DL20: the stop of the engine |' 'From order 411,'  # 0 1 1 1 2; 1 0 0 0 0 today
k conformance/profiles/first-build.yaml '"engine-stop"'  # 1; 0 today
k engine/crates/shell/src/help.rs 'SIGTERM or Ctrl-C ends runs.' '[--https-roots FILE] [--grace-period-seconds N]' '--grace-period-seconds N: how long a stop lets calls in progress end (default 30).'  # 0 1 1; 1 0 0 today
k engine/CLI.md '[--https-roots FILE] [--grace-period-seconds N]' 'asks `conduit serve` to stop (OPS-037).'  # 1 1; 0 0 today
k engine/README.md 'SIGINT or SIGTERM stops the server gracefully, as' 'describes (OPS-037).'  # 1 1; 0 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
s() { (cd engine && cargo test --offline --release -p conduit-shell $1 "$2" 2>&1 | grep -c "^test result: ok\. $3 passed"); }
s --lib sign_in_wait_end_is_submitted_at_expiry 1  # 1: the wait still ends on the token
s --lib pending_http_completion_and_client_cancellation 1  # 1: a caller's own cancellation as before
s --lib memory_equals_disk_after 5  # 1: a state with no stop is written as before
s '--test runner' serve_signal_during_setup_ends_run_and_starts_no_later_part 1  # 1
s '--test cli' root_defaults_allow_startup_setup_and_inspection_while_serving 1  # 1: SIGTERM with nothing in progress
s '--test cli' help_describes_every_command_without_store_access 1  # 1
for t in serve_prints_one_ready_line_once_it_answers records_while_serving_match_records_after_stop; do s '--test serve' "$t" 1; done  # 1 twice
for p in requests-cancellation requests-restart-interrupted explain-outcome-causes requests-keyed-hook-dispatch requests-limits-kept; do (cd engine && cargo run --offline --release -q -p conduit-conformance -- .. "$p" 2>&1 | tail -1); done  # "N cases passed", each N the same before and after: report both
(cd engine && cargo test --offline --release -p conduit-conformance --test expected_failures 2>&1 | grep -c '^test result: ok\.')  # 1
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-shell --all-targets -- -D warnings)
n=$(git ls-files -o --exclude-standard -- engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_schemas check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/core/src/(stop|stop_tests|lib|requests|lifecycle|cascade|retirement|sign_in|pending|explain|record_fault|differential)\.rs|engine/crates/shell/src/(main|server|lib|help|server_tests)\.rs|engine/crates/shell/tests/(runner|serve|cli|agents|s2)\.rs|engine/crates/shell/src/runner\.rs|engine/crates/conformance/src/checks\.rs|conformance/expected-failures\.yaml|conformance/profiles/first-build\.yaml|engine/(CONFORMANCE|STANDING|CLI|README)\.md)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && cargo test --offline --release -p conduit-shell --lib 2>&1 | grep -c '^test result: ok\.')  # 1: the shell's own tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Both cases pass and their expected-failure entries go; no case file changes, so the vector delta
is empty.

Your report gives, per decision, what changed and where; each new test's failing line on the
unchanged code, then its passing line; each case's line before and after; what decision 4 found;
each existing test you changed, and why; your own choices; and every check's last line. Stage
nothing.

**In two halves.** Where the order is dispatched as 411a and 411b, each half changes only the
files of its own decisions within Scope and runs this Start check and Done when with these
differences.

- 411a, the core: decisions 1 to 7; of decision 9, the six core tests; of decision 10, the DL8
  row, the profile line, the two entries and the standing. Its Done when drops the shell's test
  loops, the DL16 and DL20 parts of the CONFORMANCE line, which reads `1 1 1 0 0` (`1 0 0 0 0`
  today), and the help, CLI and README lines; the shell guards stay; its STANDING line reads
  `| core |`. Until 411b, a signal stops `serve` at once, as today.
- 411b, the shell, on 411a's build: decision 8, the shell's tests of decision 9, the rest of
  decision 10. Its Start check prints these lines otherwise, and only these: its fifth `0`; its
  eighth `0` (the row now reads `2 | 0`); its ninth more than `0`; its tenth `2` (`stop.rs` and
  `stop_tests.rs`); its eleventh and twelfth `1 1 1 1` and `1 1 1 1 1`, or what 411a changed;
  its nineteenth `1` (`stop_tests.rs`, which holds 411a's six core tests; no other file may
  match); its twentieth `1 0 1 1 0`; its twenty-first `1 1`; and its twenty-fifth and
  twenty-sixth what 411a left in `retirement.rs` and `cascade.rs`. Report them as they print and
  go on. The coordinator runs this adjusted Start check whole on 411a's merged build before
  dispatching 411b, and states each line's value in 411b's brief. Its Done when's core, case,
  entries, standing and profile lines are guards.

## What follows

- **The restart script.** The coordinator's restart-serve script, outside this tree, waits for
  no call; with this order a SIGTERM stops serve gracefully, so the script is no longer needed.
  The coordinator marks DEC-176 done at this order's merge.
- **The launchd job** (DEC-178): a private job runs serve, restarting it on a crash or a reboot.
  launchd's `ExitTimeOut` defaults to 20 seconds before SIGKILL, so the job sets it above the
  grace period plus the runner's 10 seconds from TERM to KILL and its 3 seconds' wait (DL16),
  or a turn in its grace period is killed and ends as IMP-007 has it.
- **The dogfood's grace period.** An Agent's turn may outlast 30 seconds; the coordinator chooses
  the dogfood's `--grace-period-seconds` before the launchd job, or asks the owner.
- **The startup file** (DEC-139): once it exists, the grace period may move there, and DL20's row
  changes.
- **Order 320**, where it merges after this order: its waiting calls end at the ask (decision 4).

## Record

Run on 2026-11-24 and 11-25 through Conduit itself, in Pacific time, stacked on order 410 (the
stop's spec half). Drafted by the drafter, `root-drafter.turn.v1`, on the Team seat, checked
adversarially by verifier-3 (four musts and a should) and revised; built and read on the team
seats (DEC-179, DEC-180).

**Worker-3's turns**, on the Business seat: 411a (the engine takes the stop; both of 410's cases
pass), then 411b (the shell's signals, the grace period's setting, five tests). Three scope
rulings by the coordinator during the build, each named in Scope: the conformance runner's list
of record event types gains `engine-stop`; the shell's runner delivers a cut retirement's
provider stop to the runtime provider, which it had dropped (OPS-037, TPL-009, PRV-005); and
`s2.rs`'s legacy assertion of a SIGTERM's exit status follows decision 8.

**Two blind reads.** The Codex verifier, `root-verifier-3.turn.v1`, asked for two musts: a cut
`unregister.v1` reported succeeded and lost OPS-037's diagnostic; and three files lay outside the
written Scope (the ruled ones, which the coordinator then named in Scope). The Claude reader,
`root-reader-a.turn.v1`, on the Team seat, asked for the same scope must, five shoulds (among
them that serve wrote nothing to stderr when asked to stop) and two nits. One fix turn by
worker-3 answered the rest. The gate's run on the squash is the coordinator's, at the merge.
